// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { chmodSync, copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import test from "node:test";

import {
  VERSION_FILES, bump, checkArguments, classify, draftNotes, latestReleaseTag, levelFor, propose, pullNumbers, readVersions, run, writeVersions,
} from "./next-version.mjs";

const repository = resolve(import.meta.dirname, "..");
const pull = (number, labels, issueLabels = []) => ({ number, title: `change ${number}`, labels, issueLabels });

test("the highest kind among a pull request's labels wins", () => {
  assert.equal(classify(["type:bug", "breaking"]), "breaking");
  assert.equal(classify(["area:tally", "type:feature"]), "feature");
  assert.equal(classify(["type:rectify"]), "fix");
  assert.equal(classify(["dependencies"]), "maintenance");
  assert.equal(classify(["documentation"]), "docs");
  assert.equal(classify(["area:tally", "severity:p2"]), null);
});

test("before 1.0.0 a breaking change or a feature bumps the minor version", () => {
  assert.equal(levelFor(["breaking"], "0.3.0"), "minor");
  assert.equal(levelFor(["feature", "fix"], "0.3.0"), "minor");
  assert.equal(levelFor(["fix"], "0.3.0"), "patch");
  assert.equal(levelFor(["maintenance"], "0.3.0"), "patch");
  assert.equal(levelFor(["docs"], "0.3.0"), null);
});

test("from 1.0.0 a breaking change bumps the major version", () => {
  assert.equal(levelFor(["breaking"], "1.2.3"), "major");
  assert.equal(levelFor(["feature"], "1.2.3"), "minor");
  assert.equal(bump("1.2.3", "major"), "2.0.0");
  assert.equal(bump("0.3.0", "minor"), "0.4.0");
  assert.equal(bump("0.3.0", "patch"), "0.3.1");
  assert.throws(() => bump("0.3", "patch"), /not a plain/);
});

test("an unclassified pull request refuses the proposal unless a level is given", () => {
  const pulls = [pull(1, ["type:feature"]), pull(2, [])];
  const refused = propose({ current: "0.3.0", pulls });
  assert.equal(refused.ok, false);
  assert.deepEqual(refused.unclassified.map((p) => p.number), [2]);
  const overridden = propose({ current: "0.3.0", pulls, level: "patch" });
  assert.equal(overridden.ok, true);
  assert.equal(overridden.next, "0.3.1");
  assert.equal(overridden.proposedLevel, "minor");
  assert.equal(overridden.overridden, true);
});

test("the highest kind among a pull request's labels and its closed issues' wins", () => {
  const result = propose({ current: "0.3.0", pulls: [pull(5, ["type:chore"], ["breaking"])] });
  assert.equal(result.classified[0].kind, "breaking");
  assert.equal(result.next, "0.4.0");
});

test("--level cannot release nothing, and lowering the level is flagged", () => {
  const none = propose({ current: "0.3.0", pulls: [], level: "patch" });
  assert.equal(none.ok, false);
  assert.match(none.reason, /nothing to release/);
  const lowered = propose({ current: "0.3.0", pulls: [pull(6, ["type:feature"])], level: "patch" });
  assert.equal(lowered.lowered, true);
  const same = propose({ current: "0.3.0", pulls: [pull(6, ["type:feature"])], level: "minor" });
  assert.equal(same.lowered, false);
});

test("a pull request with no label of its own takes the closed issue's", () => {
  const result = propose({ current: "0.3.0", pulls: [pull(3, [], ["area:tally", "type:bug"])] });
  assert.equal(result.ok, true);
  assert.equal(result.next, "0.3.1");
});

test("documentation alone proposes no release", () => {
  const result = propose({ current: "0.3.0", pulls: [pull(4, ["documentation"])] });
  assert.equal(result.ok, false);
  assert.match(result.reason, /nothing to release/);
});

test("the pull request number is the last one in a squash subject, and a direct push is kept", () => {
  const { numbers, missing } = pullNumbers([
    "Name a ledger stored with a trailing CR LF (#626) (#708)",
    "E2b: port high_value_register (#713)",
    "A direct push with no number",
    "",
  ]);
  assert.deepEqual(numbers, [708, 713]);
  assert.deepEqual(missing, ["A direct push with no number"]);
});

test("the latest release tag is chosen by version, not by name order", () => {
  assert.equal(latestReleaseTag(["v0.1.0", "mcp-preview-0.2.0", "mcp-preview-0.10.0", "mcp-preview-0.9.1", "other"]), "mcp-preview-0.10.0");
  assert.equal(latestReleaseTag(["other"]), null);
});

test("draft notes group titles by kind and mark themselves as a draft", () => {
  const notes = draftNotes(propose({ current: "0.3.0", pulls: [pull(5, ["type:feature"]), pull(6, ["type:bug"])] }).classified);
  assert.match(notes, /^<!-- Draft/);
  assert.ok(notes.indexOf("**New and changed**") < notes.indexOf("**Fixes**"));
  assert.match(notes, /- change 5 \(#5\)/);
});

test("writing a version changes exactly the five version files and the README sentence", () => {
  const base = mkdtempSync(join(tmpdir(), "next-version-"));
  try {
    for (const file of [...Object.keys(VERSION_FILES), "README.md"]) {
      mkdirSync(dirname(join(base, file)), { recursive: true });
      copyFileSync(join(repository, file), join(base, file));
    }
    const before = readVersions(base);
    assert.equal(new Set(Object.values(before)).size, 1, "the repository's own version files agree");
    const next = bump(Object.values(before)[0], "minor");
    writeVersions(next, base);
    assert.deepEqual(Object.values(readVersions(base)), Object.keys(VERSION_FILES).map(() => next));
    assert.match(readFileSync(join(base, "README.md"), "utf8"), new RegExp(`current development source is version \`${next.replaceAll(".", "\\.")}\``));
    const old = Object.values(before)[0];
    const original = readFileSync(join(repository, "src-tauri/Cargo.lock"), "utf8");
    const expected = original.replace(`name = "bridge"\nversion = "${old}"\n`, `name = "bridge"\nversion = "${next}"\n`);
    assert.notEqual(expected, original);
    assert.equal(readFileSync(join(base, "src-tauri/Cargo.lock"), "utf8"), expected, "only the bridge entry's version changed");
  } finally {
    rmSync(base, { recursive: true, force: true });
  }
});

test("a file with two candidate version lines is refused, not half-written", () => {
  const base = mkdtempSync(join(tmpdir(), "next-version-"));
  try {
    for (const file of [...Object.keys(VERSION_FILES), "README.md"]) {
      mkdirSync(dirname(join(base, file)), { recursive: true });
      copyFileSync(join(repository, file), join(base, file));
    }
    const cargo = join(base, "src-tauri/Cargo.toml");
    writeFileSync(cargo, `${readFileSync(cargo, "utf8")}\nversion = "9.9.9"\n`);
    const packageBefore = readFileSync(join(base, "package.json"), "utf8");
    assert.throws(() => writeVersions("0.9.0", base), /Cargo\.toml: expected exactly one version line, found 2/);
    assert.equal(readFileSync(join(base, "package.json"), "utf8"), packageBefore, "no file was written");
  } finally {
    rmSync(base, { recursive: true, force: true });
  }
});

test("a README with two current-version sentences is refused before any file is written", () => {
  const base = mkdtempSync(join(tmpdir(), "next-version-"));
  try {
    for (const file of [...Object.keys(VERSION_FILES), "README.md"]) {
      mkdirSync(dirname(join(base, file)), { recursive: true });
      copyFileSync(join(repository, file), join(base, file));
    }
    const readme = join(base, "README.md");
    writeFileSync(readme, `${readFileSync(readme, "utf8")}\ncurrent development source is version \`0.0.1\`\n`);
    const packageBefore = readFileSync(join(base, "package.json"), "utf8");
    assert.throws(() => writeVersions("0.9.0", base), /README\.md: expected exactly one current-version sentence, found 2/);
    assert.equal(readFileSync(join(base, "package.json"), "utf8"), packageBefore, "no file was written");
  } finally {
    rmSync(base, { recursive: true, force: true });
  }
});

test("a command that stalls or fails is reported as one line naming it", () => {
  assert.throws(() => run(process.execPath, ["-e", "setTimeout(() => {}, 5000)"], { timeoutMs: 200 }), (error) => error.kind === "timeout" && /timed out after 0\.2s/.test(error.message));
  assert.throws(() => run(process.execPath, ["-e", "console.error('boom\\nmore'); process.exit(3)"]), (error) => error.kind === "failed" && /failed: boom$/.test(error.message));
  assert.throws(() => run("next-version-no-such-command", []), (error) => error.kind === "missing");
});

test("unknown flags, stray arguments and the --flag=value form are refused", () => {
  assert.doesNotThrow(() => checkArguments(["--since", "v1.0.0", "--to", "HEAD", "--level", "minor", "--apply"]));
  assert.doesNotThrow(() => checkArguments([]));
  assert.throws(() => checkArguments(["--lvel", "minor"]), /unknown flag --lvel/);
  assert.throws(() => checkArguments(["--level=minor"]), /write --level minor, with a space/);
  assert.throws(() => checkArguments(["--apply=true"]), /the --apply=VALUE form is not read/);
  assert.throws(() => checkArguments(["minor"]), /unexpected argument minor/);
  assert.throws(() => checkArguments(["--level", "minor", "extra"]), /unexpected argument extra/);
  assert.throws(() => checkArguments(["--level", "patch", "--level", "major"]), /--level is given twice/);
});

// The script is copied into a throwaway repository with a bare origin and a stub `gh`, because
// its guards read git state, the version files and the network, all of which this fakes.
const gh = `#!/bin/sh
if [ "$1 $2" = "pr view" ]; then
  case "$3" in
    626) echo "GraphQL: Could not resolve to a PullRequest with the number of 626." >&2; exit 1 ;;
    *) printf '{"number":%s,"title":"change %s","labels":[{"name":"type:bug"}],"closingIssuesReferences":[]}\\n' "$3" "$3" ;;
  esac
else
  echo "unexpected gh $*" >&2; exit 97
fi
`;

function inRelease(callback) {
  const base = mkdtempSync(join(tmpdir(), "next-version-cli-"));
  try {
    const work = join(base, "work");
    const origin = join(base, "origin.git");
    const bin = join(base, "bin");
    mkdirSync(bin);
    writeFileSync(join(bin, "gh"), gh);
    chmodSync(join(bin, "gh"), 0o755);
    for (const file of [...Object.keys(VERSION_FILES), "README.md", "scripts/next-version.mjs"]) {
      mkdirSync(dirname(join(work, file)), { recursive: true });
      copyFileSync(join(repository, file), join(work, file));
    }
    const inherited = Object.fromEntries(Object.entries(process.env).filter(([name]) => !name.startsWith("GIT_")));
    const env = { ...inherited, GIT_CONFIG_GLOBAL: "/dev/null", GIT_CONFIG_NOSYSTEM: "1", PATH: `${bin}:${process.env.PATH}`, GIT_AUTHOR_NAME: "t", GIT_AUTHOR_EMAIL: "t@example.invalid", GIT_COMMITTER_NAME: "t", GIT_COMMITTER_EMAIL: "t@example.invalid" };
    const git = (...args) => execFileSync("git", ["-c", "commit.gpgsign=false", ...args], { cwd: work, env, stdio: "pipe" });
    const commit = (subject) => git("commit", "--allow-empty", "-q", "-m", subject);
    const current = readVersions(work)["package.json"];
    execFileSync("git", ["init", "--bare", "-q", origin], { env });
    git("init", "-q", "-b", "master");
    git("add", "-A");
    commit("Start");
    git("tag", `mcp-preview-${current}`);
    git("remote", "add", "origin", origin);
    const next = (...args) => spawnSync(process.execPath, ["scripts/next-version.mjs", ...args], { cwd: work, env, encoding: "utf8" });
    return callback({ git, commit, next, current, work });
  } finally {
    rmSync(base, { recursive: true, force: true });
  }
}
const skipOnWindows = { skip: process.platform === "win32" && "the gh stub is a POSIX script" };

test("the default comparison stops at origin/master, so a working branch's commits are not counted", skipOnWindows, () => inRelease(({ git, commit, next }) => {
  commit("Fix a thing (#11)");
  git("push", "-q", "origin", "master", "--tags");
  git("checkout", "-q", "-b", "lane/work");
  commit("wip, no pull request number");
  const result = next();
  assert.equal(result.status, 0, result.stdout + result.stderr);
  assert.match(result.stdout, /compared up to origin\/master/);
  assert.match(result.stdout, /1 commits since then/);
  assert.doesNotMatch(result.stdout, /#\?/);
  const explicit = next("--to", "HEAD");
  assert.equal(explicit.status, 1, "an explicit --to HEAD still counts the branch's commit");
  assert.match(explicit.stdout, /#\? wip, no pull request number/);
}));

test("a subject ending in an issue number, and a bad flag, end the run with one clear line", skipOnWindows, () => inRelease(({ git, commit, next, current }) => {
  commit("Fix another thing (#626)");
  git("push", "-q", "origin", "master", "--tags");
  const issue = next();
  assert.equal(issue.status, 1);
  assert.match(issue.stderr, /^next-version: #626 could not be read as a pull request\. If it is an issue number/);
  assert.doesNotMatch(issue.stderr, /\n\s+at /, "no stack trace");
  // --level is the escape the message names, so it has to work on this input.
  const overridden = next("--level", "patch");
  assert.equal(overridden.status, 0, overridden.stdout + overridden.stderr);
  assert.match(overridden.stdout, /unclassified \(1\):\n  #626 /);
  assert.match(overridden.stdout, /overridden to patch: /);
  const withSince = next("--level", "minor", "--since", `mcp-preview-${current}`);
  assert.equal(withSince.status, 0, withSince.stdout + withSince.stderr);
  assert.match(withSince.stdout, /overridden to minor: /);
  const flag = next("--level=minor");
  assert.equal(flag.status, 1);
  assert.match(flag.stderr, /^next-version: write --level minor, with a space/);
}));

test("the default range refuses a stale origin/master and version files that differ from it", skipOnWindows, () => inRelease(({ git, commit, next, current, work }) => {
  commit("Fix a thing (#11)");
  git("push", "-q", "origin", "master", "--tags");
  assert.equal(next().status, 0);

  git("update-ref", "refs/remotes/origin/master", "HEAD~1");
  const stale = next();
  assert.equal(stale.status, 1);
  assert.match(stale.stderr, /origin\/master here is not origin's current master; run git fetch origin/);
  git("fetch", "-q", "origin");

  writeVersions("0.0.7", work);
  git("add", "-A");
  commit("Bump the version (#12)");
  git("push", "-q", "origin", "master");
  git("reset", "-q", "--hard", "HEAD~1");
  const bumped = next();
  assert.equal(bumped.status, 1);
  assert.match(bumped.stderr, new RegExp(`the version files here say ${current.replaceAll(".", "\\.")} but origin/master says 0\\.0\\.7`));
}));

test("the guards in main refuse a stale, ahead or inconsistent release state", skipOnWindows, () => inRelease(({ git, commit, next, current, work }) => {
  commit("Fix a thing (#11)");
  git("push", "-q", "origin", "master", "--tags");
  assert.equal(next().status, 0);

  git("tag", "mcp-preview-9.9.9");
  const unreleased = next();
  assert.equal(unreleased.status, 1);
  assert.match(unreleased.stderr, /the version files say .* but the last release tag, mcp-preview-9\.9\.9, is 9\.9\.9/);
  git("tag", "-d", "mcp-preview-9.9.9");

  git("push", "-q", "origin", "HEAD:refs/tags/mcp-preview-9.9.9");
  const stale = next();
  assert.equal(stale.status, 1);
  assert.match(stale.stderr, /the newest release tag on origin is mcp-preview-9\.9\.9, but the local one is .*; run git fetch --tags origin/);
  git("push", "-q", "origin", ":refs/tags/mcp-preview-9.9.9");

  git("push", "-q", "origin", "HEAD:refs/tags/mcp-preview-0.0.1", `:refs/tags/mcp-preview-${current}`);
  const ahead = next();
  assert.equal(ahead.status, 1);
  assert.match(ahead.stderr, new RegExp(`the local release tag mcp-preview-${current.replaceAll(".", "\\.")} is newer than origin's mcp-preview-0\\.0\\.1; push it`));
  assert.doesNotMatch(ahead.stderr, /git fetch/, "fetching cannot fix a tag that origin lacks");

  git("checkout", "-q", "--orphan", "elsewhere");
  commit("An unrelated root");
  git("tag", "v0.0.1");
  const missing = next("--since", "no-such-tag");
  assert.equal(missing.status, 1);
  assert.match(missing.stderr, /no-such-tag does not exist here/);
  assert.doesNotMatch(missing.stderr, /not an ancestor/);
  const dashed = next("--since", "-x");
  assert.match(dashed.stderr, /--since needs a value that does not start with "-"/);
  const unrelated = next("--since", "v0.0.1");
  assert.equal(unrelated.status, 1);
  assert.match(unrelated.stderr, /v0\.0\.1 is not an ancestor of origin\/master/);

  writeFileSync(join(work, "package.json"), readFileSync(join(work, "package.json"), "utf8").replace(`"version": "${current}"`, '"version": "0.0.7"'));
  const disagree = next();
  assert.equal(disagree.status, 1);
  assert.match(disagree.stderr, /version files disagree/);
}));
