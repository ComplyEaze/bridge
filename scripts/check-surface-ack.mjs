// SPDX-License-Identifier: Apache-2.0
// A pull request that changes a pinned file, or adds or removes a pin, must add exactly one
// acknowledgement, docs/tally/compatibility/acks/pr-<N>.txt, listing the changed pinned paths.
// Only the pin lists decide what is pinned (`.gitattributes` and this script are pinned like any
// other file); scripts/merge-gate.sh applies the same rules from the GitHub API.
// The pure core is checkAck(); the CLI reads git and applies it per pull request.
//
//   node scripts/check-surface-ack.mjs --mode pull_request|merge_group|push|workflow_dispatch
//        [--report-only] [--pr N] [--base REV]        (run from the repository root)
//
// Exit 0: pass, or --report-only (prints "WOULD FAIL: ..."). Exit 1: a rule failed, or git could
// not answer (fail closed). Exit 2: bad command-line usage.
// A `push` event (master) is the one mode --report-only does not soften: every first-parent commit
// the push landed (`before..HEAD`, `before` from the event payload or --base) is checked like a pull
// request, each attributed by the `(#N)` in its subject (each squash commit is one pull request),
// and an unacknowledged pinned change turns the master run red. A push whose `before` is missing,
// new (all zeros) or not an ancestor of HEAD (a force-push) cannot be verified and fails. That
// is the after-the-fact tripwire the stored hashes used to give; merge-gate.sh reads the base when
// it runs but the merge binds only the head, so a base that moved in between is caught here.
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

export const SURFACE_PATH = "docs/tally/compatibility/compatibility-surface.json";
export const ACK_DIR = "docs/tally/compatibility/acks/";
export const MAX_REASON = 500;
const ACK_NAME = /^docs\/tally\/compatibility\/acks\/pr-([1-9][0-9]{0,8})\.txt$/;
const HEX64 = /[0-9a-f]{64}/i;
// A GitHub login: 1-39 characters, alphanumeric or hyphen, no hyphen at either end, optional [bot].
// The same expression is used by scripts/merge-gate.sh.
const LOGIN = /^[A-Za-z0-9](?:[A-Za-z0-9-]{0,37}[A-Za-z0-9])?(\[bot\])?$/;
const REV = /^[A-Za-z0-9_][A-Za-z0-9_.\/^~@{}-]*$/;

// Every ordering here is by UTF-8 bytes (Rust's str order, `LC_ALL=C sort`), never UTF-16 code units.
const byBytes = (a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b));
const sorted = (items) => [...new Set(items)].sort(byBytes);
const strictlyAscending = (items) => items.every((p, i) => i === 0 || byBytes(items[i - 1], p) < 0);
// Rust counts characters and refuses control characters (Cc) in a reason; so do the gate and this.
const reasonLength = (r) => [...r].length;
const CONTROL = /[\u0000-\u001f\u007f-\u009f]/;
const validRepoPath = (p) =>
  typeof p === "string" && p.length > 0 && p === p.trim() && !/[\u0000-\u001f\u007f\\]/.test(p) && !p.startsWith("/") &&
  p.split("/").every((seg) => seg !== "" && seg !== "." && seg !== "..");

// Parse one ack body. Strict: every line is a repository path, `reviewer: <login>`,
// `removed-pin: <path>`, or empty. Never trims, so trailing spaces and CR are refused.
export function parseAck(content) {
  const out = { paths: [], removed: [], reviewers: [], problems: [] };
  if (HEX64.test(content)) out.problems.push("contains a 64-hex token (an ack lists paths, never hashes)");
  content.split("\n").forEach((line, i) => {
    if (line === "") return;
    const at = `line ${i + 1}`;
    let m;
    if ((m = /^reviewer: (.*)$/.exec(line))) {
      if (LOGIN.test(m[1])) out.reviewers.push(m[1]);
      else out.problems.push(`${at}: reviewer is not a GitHub login`);
    } else if ((m = /^removed-pin: (.*)$/.exec(line))) {
      if (validRepoPath(m[1])) out.removed.push(m[1]);
      else out.problems.push(`${at}: removed-pin is not a repository path`);
    } else if (validRepoPath(line)) out.paths.push(line);
    else out.problems.push(`${at}: not a repository path, "reviewer: <login>" or "removed-pin: <path>"`);
  });
  if (out.reviewers.length !== 1) out.problems.push(`needs exactly one reviewer line, found ${out.reviewers.length}`);
  if (!strictlyAscending(out.paths)) out.problems.push("path lines are not sorted and unique");
  if (!strictlyAscending(out.removed)) out.problems.push("removed-pin lines are not sorted and unique");
  return out;
}

// basePins/headPins: [{path, reason?}]. changed: [{status, path, oldPath?}] (git name-status).
// acks: files under ACK_DIR in the diff: {added: [{path, content}], modified: [path], deleted: [path],
// renamedOut: [path]} (renamedOut: an ack renamed to a name outside ACK_DIR, which is a delete plus
// an add of a file that is not an ack: refused whether or not a pinned path changed).
// A path is a touch only because it is in a pin list (base or head). The one exception is a nested
// `.gitattributes`, which is refused (it can change a pinned file's bytes with no pinned path in the diff).
export function checkAck({ basePins, headPins, changed, acks, prNumber }) {
  const reasons = [];
  const baseSet = new Set(basePins.map((p) => p.path));
  const headSet = new Set(headPins.map((p) => p.path));
  const pinned = new Set([...baseSet, ...headSet]);
  const changedPins = new Set();
  for (const c of changed) {
    // Both names of a rename or copy; a copy source counts only if it is itself pinned.
    for (const p of [c.path, c.oldPath]) if (p && pinned.has(p)) changedPins.add(p);
  }
  const addedPins = headPins.filter((p) => !baseSet.has(p.path));
  const removed = sorted([...baseSet].filter((p) => !headSet.has(p)));
  const touched = sorted([...changedPins, ...addedPins.map((p) => p.path), ...removed]);
  const expectedPaths = touched.filter((p) => !removed.includes(p));
  const result = (r) => ({ ok: r.length === 0, reasons: r, touched, removed });
  const renamedOut = acks.renamedOut ?? [];
  // The digest hashes working-tree bytes, and a `.gitattributes` below the root (eol, ident,
  // working-tree-encoding) changes those bytes without any pinned path appearing in a diff. The
  // root one is pinned, so it is covered like any other pin; a nested one is refused outright.
  for (const c of changed) {
    for (const p of [c.path, c.oldPath]) {
      if (p && p !== ".gitattributes" && p.endsWith("/.gitattributes")) {
        reasons.push(`${p}: a nested .gitattributes can change the bytes of a pinned file without a pinned path changing; none is allowed (see docs/release-process.md)`);
      }
    }
  }
  for (const p of renamedOut) reasons.push(`${p}: an existing ack was renamed out of the ack directory (acks are append-only)`);

  if (touched.length === 0) {
    // Cleanup exemption: deleting old acks alone is fine. Adding or editing one is not.
    for (const a of acks.added) reasons.push(`${a.path}: an ack was added but no pinned path changed`);
    for (const p of acks.modified) reasons.push(`${p}: an existing ack was modified (acks are append-only)`);
    return result(reasons);
  }

  if (!Number.isInteger(prNumber) || prNumber < 1) reasons.push(`the pull request number is not known (${prNumber})`);
  for (const p of acks.modified) reasons.push(`${p}: an existing ack was modified (acks are append-only)`);
  for (const p of acks.deleted) reasons.push(`${p}: an existing ack was deleted alongside a pinned change`);
  if (acks.added.length !== 1) {
    reasons.push(`pinned paths changed (${touched.join(", ")}) so exactly one ack must be added; found ${acks.added.length}`);
  } else {
    const [ack] = acks.added;
    const wanted = `${ACK_DIR}pr-${prNumber}.txt`;
    if (ack.path !== wanted) reasons.push(`the ack is ${ack.path}, expected ${wanted}`);
    const parsed = parseAck(ack.content);
    for (const p of parsed.problems) reasons.push(`${ack.path}: ${p}`);
    const listed = new Set(parsed.paths);
    const missing = expectedPaths.filter((p) => !listed.has(p));
    const extra = parsed.paths.filter((p) => !expectedPaths.includes(p));
    if (missing.length) reasons.push(`${ack.path}: missing changed pinned path(s): ${missing.join(", ")}`);
    if (extra.length) reasons.push(`${ack.path}: lists path(s) that are not changed pinned paths: ${extra.join(", ")}`);
    const undeclared = removed.filter((p) => !parsed.removed.includes(p));
    const invented = parsed.removed.filter((p) => !removed.includes(p));
    if (undeclared.length) reasons.push(`${ack.path}: removed pin(s) need a "removed-pin:" line: ${undeclared.join(", ")}`);
    if (invented.length) reasons.push(`${ack.path}: "removed-pin:" names path(s) that were not removed: ${invented.join(", ")}`);
  }
  for (const p of addedPins) {
    const r = p.reason;
    if (typeof r !== "string" || r.trim() === "") reasons.push(`${p.path}: a pin added by this pull request needs a non-empty "reason"`);
    else if (reasonLength(r) > MAX_REASON) reasons.push(`${p.path}: the pin reason is ${reasonLength(r)} characters, over ${MAX_REASON}`);
    else if (CONTROL.test(r)) reasons.push(`${p.path}: the pin reason contains a control character`);
  }
  return result(reasons);
}

// ---- pin list and git plumbing ----

export function parsePins(text, { allowSchema2 = false } = {}) {
  let doc;
  try {
    doc = JSON.parse(text);
  } catch {
    throw new Error("the pin list is not valid JSON");
  }
  if (!doc || typeof doc !== "object" || Array.isArray(doc)) throw new Error("the pin list is not a JSON object");
  const v2 = doc.schema_version === 2;
  if (doc.schema_version !== 3 && !(allowSchema2 && v2)) {
    const stale = v2 && !allowSchema2 ? "; this branch is stale: merge master and migrate the pin list, see docs/release-process.md" : "";
    throw new Error(`pin list schema_version ${JSON.stringify(doc.schema_version)} is not accepted (schema 3 required${allowSchema2 ? ", 2 allowed at the base" : ""}${stale})`);
  }
  if (Object.keys(doc).sort().join() !== "files,schema_version" || !Array.isArray(doc.files)) {
    throw new Error("the pin list must have exactly the keys files and schema_version");
  }
  const allowed = v2 ? ["path", "sha256"] : ["path", "reason"];
  const rows = doc.files.map((row, i) => {
    const ok = row && typeof row === "object" && !Array.isArray(row) && Object.keys(row).every((k) => allowed.includes(k));
    if (!ok || !validRepoPath(row.path) || (row.reason !== undefined && typeof row.reason !== "string")) {
      throw new Error(`pin list row ${i + 1} is malformed`);
    }
    return row.reason === undefined || v2 ? { path: row.path } : { path: row.path, reason: row.reason };
  });
  if (!strictlyAscending(rows.map((r) => r.path))) throw new Error("pin list paths are not sorted and unique");
  return rows;
}

// `git diff --name-status -z` output: STATUS NUL path NUL (renames and copies: STATUS NUL old NUL new NUL).
export function parseNameStatus(text) {
  const tok = text.split("\0");
  if (tok[tok.length - 1] === "") tok.pop();
  const out = [];
  for (let i = 0; i < tok.length; ) {
    const status = tok[i++];
    if (!/^[ACDMRTUX]\d*$/.test(status ?? "")) throw new Error(`unreadable git name-status record ${JSON.stringify(status)}`);
    if (status[0] === "R" || status[0] === "C") {
      const oldPath = tok[i++];
      out.push({ status: status[0], oldPath, path: tok[i++] });
    } else out.push({ status: status[0], path: tok[i++] });
  }
  if (out.some((c) => !c.path || (["R", "C"].includes(c.status) && !c.oldPath))) throw new Error("truncated git name-status output");
  return out;
}

const git = (...args) =>
  execFileSync("git", args, { encoding: "utf8", maxBuffer: 256 << 20, stdio: ["ignore", "pipe", "pipe"] });
const rev = (r) => {
  if (!REV.test(r ?? "")) throw new Error(`refusing revision ${JSON.stringify(r)}`);
  return r;
};
const failure = (label, error) => ({
  ok: false,
  label,
  touched: [],
  reasons: [`${label}: ${String(error.stderr || error.message).trim().split("\n")[0]}`],
});

function collectAcks(changed, headRev) {
  const acks = { added: [], modified: [], deleted: [], renamedOut: [] };
  const inDir = (p) => p?.startsWith(ACK_DIR);
  for (const c of changed) {
    if (c.status === "R" && inDir(c.oldPath)) (inDir(c.path) ? acks.deleted : acks.renamedOut).push(c.oldPath);
    if (!inDir(c.path)) continue;
    if (c.status === "D") acks.deleted.push(c.path);
    else if (["A", "R", "C"].includes(c.status)) acks.added.push({ path: c.path, content: git("show", `${headRev}:${c.path}`) });
    else acks.modified.push(c.path);
  }
  return acks;
}

function evaluate(label, baseRev, headRev, prNumber) {
  try {
    const at = (r) => git("show", `${rev(r)}:${SURFACE_PATH}`);
    const basePins = parsePins(at(baseRev), { allowSchema2: true });
    const headPins = parsePins(at(headRev));
    const changed = parseNameStatus(git("diff", "--name-status", "-z", "--find-renames", "--find-copies", rev(baseRev), rev(headRev), "--"));
    const acks = collectAcks(changed, headRev);
    return { label, ...checkAck({ basePins, headPins, changed, acks, prNumber }) };
  } catch (error) {
    return failure(label, error);
  }
}

function pullRequest(opts, env) {
  try {
    let base = opts.base;
    if (!base) {
      // The workflow always sets PR_HEAD_SHA; without it nothing proves HEAD^2 is the pull request
      // head (and so that HEAD^1 is the base), so refuse rather than skip the assertion.
      const expected = (env.PR_HEAD_SHA ?? "").trim().toLowerCase();
      if (!expected) throw new Error("PR_HEAD_SHA is not set, so HEAD^2 cannot be shown to be the pull request head");
      const second = git("rev-parse", "--verify", "HEAD^2").trim();
      if (second !== expected) throw new Error(`HEAD^2 (${second}) is not the pull request head ${env.PR_HEAD_SHA}`);
      base = "HEAD^1";
    }
    return [evaluate("pull_request", base, "HEAD", Number(opts.pr ?? env.PR_NUMBER))];
  } catch (error) {
    return [failure("pull_request", error)];
  }
}

function mergeGroup(opts, env) {
  try {
    const base = rev(opts.base ?? env.MERGE_GROUP_BASE_SHA);
    const head = rev(env.GITHUB_SHA);
    const commits = git("rev-list", "--first-parent", "--reverse", `${base}..${head}`, "--").split("\n").filter(Boolean);
    if (commits.length === 0) throw new Error(`no commits in ${base}..${head}, nothing can be verified`);
    const queueRef = /^refs\/heads\/gh-readonly-queue\/.+\/pr-([1-9][0-9]*)-[0-9a-f]+$/.exec(env.GITHUB_REF ?? "");
    return commits.map((sha, i) => {
      const label = `merge_group ${sha.slice(0, 12)}`;
      try {
        const subject = git("log", "-1", "--format=%s", sha, "--").trim();
        const trailing = /\(#([1-9][0-9]*)\)$/.exec(subject);
        const last = i === commits.length - 1;
        const n = last ? queueRef?.[1] : trailing?.[1];
        if (!n || (last && trailing && trailing[1] !== n)) {
          throw new Error(`cannot attribute the commit to exactly one pull request (subject ${JSON.stringify(subject)}); a group of several pull requests is unverified`);
        }
        return evaluate(label, `${sha}^1`, sha, Number(n));
      } catch (error) {
        return failure(label, error);
      }
    });
  } catch (error) {
    return [failure("merge_group", error)];
  }
}

// push (master): every first-parent commit from the event's `before` to HEAD is one squashed pull
// request, named by `(#N)` (or "Merge pull request #N") in its subject. Same rules as a pull
// request, and never report-only. Checking only the tip would let several commits land in one push
// with only the last one looked at.
function pushedCommits(opts, env) {
  try {
    let before = opts.base;
    if (!before) {
      try {
        before = JSON.parse(readFileSync(env.GITHUB_EVENT_PATH, "utf8")).before;
      } catch {
        before = undefined;
      }
    }
    if (!/^[0-9a-f]{40,64}$/.test(before ?? "") || /^0+$/.test(before)) {
      throw new Error(`the push has no usable "before" commit (${JSON.stringify(before ?? null)}): a new branch or a missing event payload cannot be verified`);
    }
    try {
      git("merge-base", "--is-ancestor", before, "HEAD");
    } catch {
      throw new Error(`before ${before} is not an ancestor of HEAD (a force-push, rewritten history or an unknown commit): cannot verify what landed`);
    }
    const commits = git("rev-list", "--first-parent", "--reverse", `${before}..HEAD`, "--").split("\n").filter(Boolean);
    if (commits.length === 0) throw new Error(`no commits in ${before}..HEAD, nothing can be verified`);
    return commits.map((sha) => {
      const label = `push ${sha.slice(0, 12)}`;
      try {
        const subject = git("log", "-1", "--format=%s", sha, "--").trim();
        const named = /\(#([1-9][0-9]*)\)$/.exec(subject) ?? /^Merge pull request #([1-9][0-9]*)\b/.exec(subject);
        return { ...evaluate(label, `${sha}^1`, sha, named ? Number(named[1]) : NaN), strict: true };
      } catch (error) {
        return { ...failure(label, error), strict: true };
      }
    });
  } catch (error) {
    return [{ ...failure("push", error), strict: true }];
  }
}

// workflow_dispatch (and the second half of push): the diff is already merged, so check the acks
// directory is well formed.
function acksDirectory() {
  try {
    const names = git("ls-tree", "-r", "--name-only", "-z", "HEAD", "--", ACK_DIR).split("\0").filter(Boolean);
    const reasons = [];
    for (const name of names) {
      if (!ACK_NAME.test(name)) reasons.push(`${name}: not named pr-<N>.txt`);
      else for (const p of parseAck(git("show", `HEAD:${name}`)).problems) reasons.push(`${name}: ${p}`);
    }
    return [{ label: "acks", ok: reasons.length === 0, touched: [], reasons }];
  } catch (error) {
    return [failure("acks", error)];
  }
}

export function parseArgs(argv) {
  const opts = { mode: null, reportOnly: false, pr: undefined, base: undefined };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === "--report-only") opts.reportOnly = true;
    else if (["--mode", "--pr", "--base"].includes(a) && argv[i + 1] !== undefined) opts[a.slice(2)] = argv[++i];
    else throw new Error(`unknown or incomplete argument ${JSON.stringify(a)}`);
  }
  if (!["pull_request", "merge_group", "push", "workflow_dispatch"].includes(opts.mode)) {
    throw new Error("--mode must be pull_request, merge_group, push or workflow_dispatch");
  }
  return opts;
}

// Workflow-command data: a newline or "%" would end or alter the command.
const annotationSafe = (text) => text.replace(/%/g, "%25").replace(/\r/g, "%0D").replace(/\n/g, "%0A");

export function main(argv, env) {
  let opts;
  try {
    opts = parseArgs(argv);
  } catch (error) {
    console.error(`${error.message}\nusage: check-surface-ack.mjs --mode pull_request|merge_group|push [--report-only] [--pr N] [--base REV]`);
    return 2;
  }
  const results =
    { pull_request: pullRequest, merge_group: mergeGroup }[opts.mode]?.(opts, env) ??
    (opts.mode === "push" && env.GITHUB_EVENT_NAME !== "workflow_dispatch" ? [...pushedCommits(opts, env), ...acksDirectory()] : acksDirectory());
  for (const r of results) {
    const list = r.touched.length ? `${r.touched.length}: ${r.touched.join(", ")}` : "none";
    console.log(`[${r.label}] touched pinned files (${list})`);
  }
  const failed = results.filter((r) => !r.ok);
  if (failed.length === 0) {
    console.log("surface ack check ok");
    return 0;
  }
  const reasons = failed.flatMap((r) => r.reasons);
  const blocking = !opts.reportOnly || failed.some((r) => r.strict);
  console.log(`${blocking ? "surface ack check FAILED" : "WOULD FAIL"}: ${reasons.join("; ")}`);
  // Report-only exits 0, which a green check cannot tell from a pass (a git failure looks the same),
  // so surface it as a workflow annotation as well.
  if (!blocking) console.log(`::warning title=Surface acknowledgement (report-only)::${annotationSafe(reasons.join("; "))}`);
  return blocking ? 1 : 0;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  process.exitCode = main(process.argv.slice(2), process.env);
}
