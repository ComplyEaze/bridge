#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0
//
// Proposes the next release version from the labels of the pull requests
// merged since the last release, and, with --apply, writes it to every file
// that must carry it. See "Choosing the version" in docs/release-process.md.
//
//   node scripts/next-version.mjs                 propose only (read-only)
//   node scripts/next-version.mjs --since TAG     compare against TAG instead
//   node scripts/next-version.mjs --to REF        compare up to REF (default origin/master)
//   node scripts/next-version.mjs --level minor   override the proposed level
//   node scripts/next-version.mjs --apply         write the proposed version
//
// A pull request is classified by its own labels together with the labels of
// the issues it closes, the highest kind winning: a chore that closes a
// `breaking` issue is breaking. Any pull request left unclassified makes the proposal
// refuse, naming each one, unless --level is given: a guessed level would be
// silent, and the level is the one judgement this script cannot make.
import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(import.meta.dirname, "..");

// Highest kind wins. Labels are the repository's existing ones, plus
// `breaking`, which a maintainer adds to a pull request that removes or
// changes behaviour an existing user relies on.
export const KINDS = [
  ["breaking", ["breaking"]],
  ["feature", ["type:feature", "enhancement"]],
  ["fix", ["type:bug", "type:rectify", "bug"]],
  ["maintenance", ["type:chore", "dependencies", "github_actions", "infra"]],
  ["docs", ["documentation"]],
];

export function classify(labels) {
  for (const [kind, names] of KINDS) {
    if (labels.some((label) => names.includes(label))) return kind;
  }
  return null;
}

// SemVer 2.0.0 defines MAJOR/MINOR/PATCH only for 1.0.0 and later; under
// 0.y.z "anything MAY change at any time" (rule 4), and its FAQ suggests
// incrementing the minor version for each release. This project's own
// convention, within that freedom: before 1.0.0 a breaking change or a
// feature bumps the minor version and a fix-only release the patch; from
// 1.0.0 a breaking change bumps the major version, as rule 8 requires.
export function levelFor(kinds, current) {
  const has = (kind) => kinds.includes(kind);
  const [major] = parse(current);
  if (has("breaking")) return major === 0 ? "minor" : "major";
  if (has("feature")) return "minor";
  if (has("fix") || has("maintenance")) return "patch";
  return null;
}

export function parse(version) {
  const match = /^(\d+)\.(\d+)\.(\d+)$/.exec(version);
  if (!match) throw new Error(`not a plain MAJOR.MINOR.PATCH version: ${version}`);
  return match.slice(1).map(Number);
}

export function bump(version, level) {
  const [major, minor, patch] = parse(version);
  if (level === "major") return `${major + 1}.0.0`;
  if (level === "minor") return `${major}.${minor + 1}.0`;
  if (level === "patch") return `${major}.${minor}.${patch + 1}`;
  throw new Error(`unknown level: ${level}`);
}

// Every file that must carry the release version. The preview admission check
// reads the first two; the bundle smoke check compares the server's
// CARGO_PKG_VERSION (Cargo.toml) with the manifest; check-license-metadata
// requires all five to agree.
export const VERSION_FILES = {
  "package.json": {
    read: (text) => JSON.parse(text).version,
    pattern: /^(  "version": ")([^"]+)(",)$/m,
  },
  "packaging/mcpb/manifest.json": {
    read: (text) => JSON.parse(text).version,
    pattern: /^(  "version": ")([^"]+)(",)$/m,
  },
  "src-tauri/Cargo.toml": {
    read: (text) => /^version = "([^"]+)"$/m.exec(text)?.[1],
    pattern: /^(version = ")([^"]+)(")$/m,
  },
  "src-tauri/tauri.conf.json": {
    read: (text) => JSON.parse(text).version,
    pattern: /^(  "version": ")([^"]+)(",)$/m,
  },
  "src-tauri/Cargo.lock": {
    read: (text) => /\[\[package\]\]\nname = "bridge"\nversion = "([^"]+)"\n/.exec(text)?.[1],
    pattern: /(\[\[package\]\]\nname = "bridge"\nversion = ")([^"]+)("\n)/,
  },
};

export function readVersions(base = root) {
  return Object.fromEntries(
    Object.entries(VERSION_FILES).map(([file, spec]) => [file, spec.read(readFileSync(resolve(base, file), "utf8")) ?? null]),
  );
}

export function writeVersions(next, base = root) {
  // Check every file before writing any, so a refusal leaves none half-bumped.
  const updates = Object.entries(VERSION_FILES).map(([file, spec]) => {
    const path = resolve(base, file);
    const text = readFileSync(path, "utf8");
    const count = (text.match(new RegExp(spec.pattern.source, `${spec.pattern.flags}g`)) ?? []).length;
    if (count !== 1) throw new Error(`${file}: expected exactly one version line, found ${count}`);
    return [path, text.replace(spec.pattern, (_, before, _old, after) => `${before}${next}${after}`)];
  });
  const readme = resolve(base, "README.md");
  const readmeText = readFileSync(readme, "utf8");
  const sentence = /(current development source is version `)([^`]+)(`)/;
  const sentences = (readmeText.match(new RegExp(sentence.source, "g")) ?? []).length;
  if (sentences !== 1) throw new Error(`README.md: expected exactly one current-version sentence, found ${sentences}`);
  updates.push([readme, readmeText.replace(sentence, (_, a, _old, b) => `${a}${next}${b}`)]);
  for (const [path, text] of updates) writeFileSync(path, text);
}

export function propose({ current, pulls, level }) {
  const classified = pulls.map((pull) => ({
    ...pull,
    kind: classify([...pull.labels, ...(pull.issueLabels ?? [])]),
  }));
  const unclassified = classified.filter((pull) => !pull.kind);
  const kinds = [...new Set(classified.map((pull) => pull.kind).filter(Boolean))];
  const proposedLevel = levelFor(kinds, current);
  if (!level && unclassified.length) {
    return { ok: false, current, classified, unclassified, reason: "unclassified pull requests; label them or pass --level" };
  }
  if (!classified.length) return { ok: false, current, classified, unclassified, reason: "nothing to release: no merged pull requests" };
  const chosen = level ?? proposedLevel;
  if (!chosen) return { ok: false, current, classified, unclassified, reason: "nothing to release: documentation only" };
  const rank = { patch: 0, minor: 1, major: 2 };
  const lowered = Boolean(level && proposedLevel && rank[level] < rank[proposedLevel]);
  return { ok: true, current, next: bump(current, chosen), level: chosen, proposedLevel, overridden: Boolean(level), lowered, classified, unclassified };
}

export function draftNotes(classified) {
  const titles = { breaking: "Breaking changes", feature: "New and changed", fix: "Fixes", maintenance: "Maintenance", docs: "Documentation", null: "Unclassified" };
  const lines = ["<!-- Draft from pull request titles. Rewrite it in plain words (docs/release-process.md). -->"];
  for (const kind of ["breaking", "feature", "fix", "maintenance", "docs", null]) {
    const group = classified.filter((pull) => pull.kind === kind);
    if (!group.length) continue;
    lines.push("", `**${titles[kind]}**`, "");
    for (const pull of group) lines.push(`- ${pull.title} (#${pull.number})`);
  }
  return lines.join("\n");
}

// A failed or slow command is reported as one line naming it, never as a stack trace, and never
// as a hang: `gh` waits on the network and a maintainer cannot tell a stall from a slow answer.
export function run(command, args, { timeoutMs = 60_000, cwd = root } = {}) {
  try {
    return execFileSync(command, args, { cwd, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"], timeout: timeoutMs, maxBuffer: 64 * 1024 * 1024 }).trim();
  } catch (error) {
    const shown = `${command} ${args.slice(0, 3).join(" ")}`;
    const failure = (kind, message) => Object.assign(new Error(message), { kind, status: error.status });
    if (error.code === "ETIMEDOUT") throw failure("timeout", `${shown} timed out after ${timeoutMs / 1000}s; check the network and authentication, then run it again`);
    if (error.code === "ENOBUFS") throw failure("failed", `${shown} printed more output than the ${64} MB limit`);
    if (error.code === "ENOENT") throw failure("missing", `${command} is not installed or not on PATH`);
    const detail = String(error.stderr ?? "").trim().split("\n")[0];
    throw failure("failed", `${shown} failed${detail ? `: ${detail}` : ""}`);
  }
}

const ALLOWED_FLAGS = { "--since": true, "--to": true, "--level": true, "--apply": false };

// An unknown flag, a positional argument, or `--level=minor` used to be ignored, which turns a
// mistyped --level into a silent default. Each is refused, naming the accepted form.
export function checkArguments(args) {
  const seen = new Set();
  for (let at = 0; at < args.length; at += 1) {
    const token = args[at];
    const [name] = token.split("=", 1);
    if (!token.startsWith("--")) throw new Error(`unexpected argument ${token}; the flags are ${Object.keys(ALLOWED_FLAGS).join(", ")}`);
    if (!(name in ALLOWED_FLAGS)) throw new Error(`unknown flag ${name}; the flags are ${Object.keys(ALLOWED_FLAGS).join(", ")}`);
    if (token.includes("=")) throw new Error(`write ${name} ${token.slice(name.length + 1) || "VALUE"}, with a space: the ${name}=VALUE form is not read`);
    if (seen.has(name)) throw new Error(`${name} is given twice; only one value would be used`);
    seen.add(name);
    if (ALLOWED_FLAGS[name]) at += 1;
  }
}

function readVersionsAt(ref) {
  return Object.fromEntries(Object.entries(VERSION_FILES).map(([file, spec]) => [file, spec.read(run("git", ["show", `${ref}:${file}`])) ?? null]));
}

// `rev-parse --verify --quiet` exits 1 with no output for an unknown name; any other failure is
// git itself failing and is reported as that.
function requireCommit(ref, advice) {
  try {
    run("git", ["rev-parse", "--verify", "--quiet", `${ref}^{commit}`]);
  } catch (error) {
    if (error.kind === "failed" && error.status === 1) throw new Error(`${ref} does not exist here; ${advice}`);
    throw error;
  }
}

export function latestReleaseTag(tags) {
  const versioned = tags
    .map((tag) => ({ tag, match: /^(?:mcp-preview-|mcp-v|v)(\d+\.\d+\.\d+)$/.exec(tag) }))
    .filter(({ match }) => match)
    .map(({ tag, match }) => ({ tag, version: parse(match[1]) }));
  versioned.sort((a, b) => a.version[0] - b.version[0] || a.version[1] - b.version[1] || a.version[2] - b.version[2]);
  return versioned.at(-1)?.tag ?? null;
}

// Squash merges end each subject with "(#N)", the pull request; a subject
// that carries an issue number first, "... (#626) (#708)", ends with the pull
// request, so the last number is the one taken.
export function pullNumbers(subjects) {
  const numbers = [];
  const missing = [];
  for (const subject of subjects.filter(Boolean)) {
    const match = /\(#(\d+)\)\s*$/.exec(subject);
    if (match) numbers.push(Number(match[1]));
    else missing.push(subject);
  }
  return { numbers, missing };
}

function pullsSince(tag, to, level) {
  const { numbers, missing } = pullNumbers(run("git", ["log", "--format=%s", `${tag}..${to}`]).split("\n"));
  const pulls = [];
  for (const number of numbers) {
    let pull;
    try {
      pull = JSON.parse(run("gh", ["pr", "view", String(number), "--json", "number,title,labels,closingIssuesReferences"]));
    } catch (error) {
      if (error.kind !== "failed") throw error;
      // --level is the documented way past a pull request that cannot be classified, so it also
      // covers one that cannot be read; it is listed as unclassified, never dropped.
      if (level) {
        pulls.push({ number, title: "(not readable as a pull request)", labels: [], issueLabels: [] });
        continue;
      }
      throw new Error(`#${number} could not be read as a pull request. If it is an issue number, the commit subject ends with it instead of its pull request number; pass --level to override. (${error.message})`);
    }
    const issueLabels = [];
    for (const issue of pull.closingIssuesReferences ?? []) {
      issueLabels.push(...JSON.parse(run("gh", ["issue", "view", String(issue.number), "--json", "labels"])).labels.map((label) => label.name));
    }
    pulls.push({ number: pull.number, title: pull.title, labels: pull.labels.map((label) => label.name), issueLabels });
  }
  // A commit without a pull request number (a direct push) cannot be
  // classified by label; it is reported as unclassified rather than dropped.
  for (const subject of missing) pulls.push({ number: "?", title: subject, labels: [], issueLabels: [] });
  return pulls;
}

function argument(name) {
  const index = process.argv.indexOf(name);
  if (index === -1) return undefined;
  const value = process.argv[index + 1];
  if (value === undefined || value.startsWith("-")) throw new Error(`${name} needs a value that does not start with "-"`);
  return value;
}

async function main() {
  checkArguments(process.argv.slice(2));
  const versions = readVersions();
  const distinct = [...new Set(Object.values(versions))];
  if (distinct.length !== 1 || !distinct[0]) {
    throw new Error(`version files disagree: ${JSON.stringify(versions)}`);
  }
  const current = distinct[0];
  const level = argument("--level");
  if (level && !["major", "minor", "patch"].includes(level)) throw new Error("--level must be major, minor or patch");
  // HEAD would count the commits of an unmerged working branch as unclassified direct pushes.
  const to = argument("--to") ?? "origin/master";
  requireCommit(to, "run git fetch origin, or pass --to REF (the default is origin/master)");
  if (!argument("--to")) {
    // The range ends at a remote-tracking ref, so a stale one would silently undercount, and the
    // version files read here must be the ones that range ends with, or a bump would repeat.
    const tip = run("git", ["ls-remote", "origin", "refs/heads/master"]).split(/\s/)[0];
    if (tip && tip !== run("git", ["rev-parse", "origin/master"])) throw new Error("origin/master here is not origin's current master; run git fetch origin");
    const atTo = [...new Set(Object.values(readVersionsAt(to)))];
    if (atTo.length !== 1 || atTo[0] !== current) throw new Error(`the version files here say ${current} but ${to} says ${atTo.join(" and ")}; merge or rebase onto ${to} first, or pass --to HEAD`);
  }
  const since = argument("--since") ?? latestReleaseTag(run("git", ["tag", "--list"]).split("\n"));
  if (!since) throw new Error("no release tag found; run git fetch --tags origin, or pass --since TAG");
  // After a version pull request merges and before its tag exists, the files
  // already say the next version; proposing again would bump it twice.
  const tagged = /(\d+\.\d+\.\d+)$/.exec(since)?.[1];
  if (!argument("--since") && tagged && tagged !== current) {
    throw new Error(`the version files say ${current} but the last release tag, ${since}, is ${tagged}; tag ${current} first, or pass --since TAG`);
  }
  if (!argument("--since")) {
    // A stale clone would silently compare against an older release.
    const remote = latestReleaseTag(run("git", ["ls-remote", "--tags", "--refs", "origin"]).split("\n").map((line) => line.split("refs/tags/")[1] ?? ""));
    if (remote && remote !== since) {
      const newest = latestReleaseTag([remote, since]);
      throw new Error(newest === remote
        ? `the newest release tag on origin is ${remote}, but the local one is ${since}; run git fetch --tags origin`
        : `the local release tag ${since} is newer than origin's ${remote}; push it (git push origin ${since}) or pass --since TAG`);
    }
  }
  // git log A..B does not fail when A is not an ancestor of B; it silently
  // returns a different set of commits.
  requireCommit(since, "run git fetch --tags origin, or pass --since TAG");
  try {
    run("git", ["merge-base", "--is-ancestor", since, to]);
  } catch (error) {
    if (error.kind === "failed" && error.status === 1) throw new Error(`${since} is not an ancestor of ${to}; the pull requests since it cannot be listed`);
    throw error;
  }
  const result = propose({ current, pulls: pullsSince(since, to, level), level });

  console.log(`current version ${current}; last release tag ${since}; compared up to ${to}`);
  console.log(`${result.classified.length} commits since then`);
  if (result.unclassified.length) {
    console.log(`unclassified (${result.unclassified.length}):`);
    for (const pull of result.unclassified) console.log(`  #${pull.number} ${pull.title}`);
  }
  if (!result.ok) {
    console.error(result.reason);
    process.exitCode = 1;
    return;
  }
  console.log(`proposed level ${result.proposedLevel ?? "none"}${result.overridden ? `, overridden to ${result.level}` : ""}: ${current} -> ${result.next}`);
  if (result.lowered) console.log(`WARNING: --level ${result.level} is lower than the proposed ${result.proposedLevel}; the labels say this release changes more than that.`);
  console.log(`\n${draftNotes(result.classified)}\n`);
  if (process.argv.includes("--apply")) {
    writeVersions(result.next);
    console.log(`wrote ${result.next} to ${Object.keys(VERSION_FILES).join(", ")} and README.md`);
    console.log("package.json, src-tauri/Cargo.toml and src-tauri/Cargo.lock are pinned: the pull request that commits them needs an acknowledgement file (see docs/release-process.md).");
  }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  try {
    await main();
  } catch (error) {
    console.error(`next-version: ${error.message}`);
    process.exitCode = 1;
  }
}
