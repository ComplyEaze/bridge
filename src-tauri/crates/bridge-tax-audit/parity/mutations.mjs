// SPDX-License-Identifier: Apache-2.0
// Run the crate's recorded mutations and keep their results.
//
// Each mutation in `parity/mutations.json` edits one source file (`from` -> `to`). It is "killed" when
// some test of the crate fails on the edited source, "survives" when none does, and "did not apply"
// when its `from` text is no longer in its file exactly once (update or retire it). The list keeps
// every mutation an author or a reviewer has written for this crate, with who wrote it.
//
//     node parity/mutations.mjs                         # every mutation, killers first
//     node parity/mutations.mjs M08 B12                 # by id
//     node parity/mutations.mjs --changed-since origin/master --jobs 3
//     node parity/mutations.mjs --changed-since origin/master --verify   # no build: the merge check
//     node parity/mutations.mjs --check-anchors        # no build: exit 2 on an anchor not exactly once at HEAD
//     node parity/mutations.mjs --full --shard 2/8 --results shard.json  # nightly, one shard
//     node parity/mutations.mjs --shard 2/8 --results shard.json --killers-from parity/mutation-results.json
//                                                       # a pull request head's proof run, one shard
//     node parity/mutations.mjs --merge shard*.json --results all.json --report report.md
//
// Run from anywhere with Node 22 or later and the pinned toolchain exported (RUSTC, RUSTDOC and PATH; see
// docs/release-process.md).
//
// ## What a verdict rests on
//
// Mutations are never applied to the working tree. Each worker is a copy of the COMMITTED bytes of
// HEAD's tracked `src-tauri` files (plus the root `rust-toolchain.toml`) under
// `src-tauri/target/mutants/w<N>/`, with its own `CARGO_TARGET_DIR`; a mutation is applied to the
// copy's bytes and undone there. Only files whose bytes differ are re-written, so a worker's build
// cache stays warm between runs. A run (not `--list` or `--verify`, which read only
// committed trees and the list) refuses to start while tracked `src-tauri` files or
// `rust-toolchain.toml` have uncommitted changes, so what it proves is what HEAD holds, and while
// another run holds the same worker directory. It writes only the results file, the worker
// directory and, with `--report`, the report.
//
// Before any mutation, every worker builds the unmutated copy and the full suite runs on it once.
// If a test already fails there, or the copy does not build, the run refuses: a test that fails
// without the mutation would otherwise "kill" every mutation.
//
// Each cargo run is two steps: `cargo test --no-run` (bounded by `--build-timeout`), then the tests
// (bounded by `--timeout`), so the build's time never counts against the test timeout. The run
// refuses unless `--timeout` is at least 3 times the unmutated suite's measured test time. A mutated
// test run that still exceeds it is taken as a hang the suite would not let through, and counts as
// killed everywhere: in the run, in `--verify` and in the nightly report. That is a bound, not a
// proof: a machine slowed more than threefold mid-run could turn a survivor into a "timeout", and
// the report lists every timeout as a warning. A build that exceeds `--build-timeout` does not
// count. A test binary that dies of a crash signal (SIGABRT, as a stack overflow aborts, SIGSEGV,
// SIGBUS, SIGILL, SIGFPE) without naming a failed test counts as killed by that target, recorded as
// `<target>::<crashed>`; one killed from outside (SIGKILL, SIGTERM) proves nothing.
//
// Killers first: when the results file records the tests that killed a mutation last time, those
// tests run first (`--exact`, only their targets). If one fails or hangs, the mutation is killed
// ("killers-first" mode, and its margin is then a lower bound). Otherwise the FULL suite runs, so any
// verdict other than killed comes from the full suite. `--full` skips the killers-first step (the
// nightly run). `--killers-from FILE` supplies killer lists from other records (a proof run's shards
// start with an empty results file): those records are read for their killers only, never counted
// or written as results. A killer subset is not run on the unmutated copy on its own, only the full
// suite is, so a test that fails only when run alone (order or shared state) would read as a kill.
//
// SIGINT or SIGTERM stops the cargo runs in flight (their own process groups) and exits; records
// already written stay. A run killed outright (SIGKILL) can leave its cargo running and a mutation
// in its worker copy; the next run's copy refresh undoes the mutation.
//
// ## A shard that cannot start
//
// Before any mutation the unmutated copy must build and pass. When it does not, the run exits 2 and
// prints the compiler's first lines. With `--shard` it also writes, in place of results, one document
// `{"refused": {"shard": "K/N", "problems": [...], "output": [...]}}` to `--results`, so `--merge`
// reports "Shards that refused to run" with the reason, and lists the ids that shard would have run
// apart from ids no shard ran. Both kinds of id still go on the nightly issue's failing line: a change
// must re-prove what no shard proved. Exit 1 is a mutation not killed; the nightly shard job fails on
// any other non-zero exit and leaves the judging of exit 1 to the merge. An uncaught exception is exit 2,
// never 1: before the first mutation of a shard it is written as a refusal ("crashed: <type>", with the
// exception and its innermost frame); after, the records already made stay and the shard still fails.
//
// ## The results file
//
// `parity/mutation-results.json` is committed. It holds one record per mutation, one line each:
// the verdict, the killing tests qualified by their target (`tests/edge_books.rs::name`,
// `src/lib.rs::book::tests::name`), the mode, the commit and time, a hash of the mutation's own
// definition, and `crate_tree`, a hash of this crate's committed tree without the results file.
//
// `parity/accepted-survivors.json` lists the mutations no test is expected to kill, each with the
// hash of its definition, why, and where that is documented. A `survived` verdict counts as passing
// only for a listed id whose current definition has that hash; an edited mutation, or one not
// listed, must be killed. An entry that accepts nothing (its id retired or its definition changed)
// fails `--verify` and the nightly until it is updated or removed, and a listed mutation that is now
// killed is reported, so its entry can go.
//
// The results file is self-attested: CI never builds, so it trusts that a record was made by this runner.
// Only the nightly run re-proves the records independently.
//
// ## The guarantee
//
// At merge, `--verify --changed-since <base>` (in CI, no build) requires:
// - every mutation in the list applies exactly once to the merged tree;
// - no record for a mutation no longer in the list (retiring a mutation deletes its record); and
// - every mutation SELECTED by the change has a killed record made on exactly the merged crate
//   tree. A mutation is selected when
//   - its target file changed;
//   - a file holding one of its recorded killing tests changed, or a file that file pulls in with
//     `include_str!`/`include_bytes!`, or `tests/common` for an integration-test killer;
//   - its own definition changed, it has no killed record, or a killer's file is unknown;
//   - ANY crate file other than Rust source under `src/` and `tests/` changed (fixtures, goldens,
//     rules, parity scripts, the Markdown tests read, `Cargo.toml`, `build.rs`): then the whole
//     list is selected, because tests read such files at run time in ways no file name reveals.
//     The runner's own files and `parity/mutations.json` are the exception: no crate test reads
//     them (a unit test fails if any crate source names them), and a change to the list is judged
//     entry by entry against the merge base's list: an id that is new there, or whose definition
//     changed, is selected whatever record the branch carries (so a record made on another tree, or
//     carried over under a renamed id, must be made again on the merged tree), and a removed id
//     leaves a record the orphan check refuses; or
//   - a nightly tracking issue is open and lists it as failing (`--nightly-issues`), and the
//     change touches the crate: so the fix for a nightly failure can merge, and nothing else can
//     until the failing mutations are proven killed again (or retired). Every open issue must
//     carry that line, and the ids of all of them are required together; if any open issue lacks
//     it, every crate change is refused until a maintainer closes that issue.
//
// CI hashes the pull request's MERGE commit, so when master has changed any crate file since the
// branch was proven, update the branch from master and re-run the selection before pushing.
//
// What the guarantee does NOT cover. The nightly run re-proves the whole list against the full suite
// on master, so each of these carries up to about a day of exposure:
// - Inputs outside this crate. Selection and `crate_tree` see only this crate's own files, so a
//   change to any of these selects nothing: the path dependencies `bridge-tally-protocol` and
//   `bridge-tally-primitives`; that protocol crate's `tests/fixtures/native`, read by
//   `tests/tally_text_rule.rs`; the workspace `src-tauri/Cargo.toml`, whose `[workspace.lints]`
//   this crate inherits; `src-tauri/Cargo.lock`; and the root `rust-toolchain.toml`.
// - A change to crate source that is neither a mutation's target nor a killer's file but still
//   weakens a killer: a test helper in another module, or a `mod` line that stops compiling a
//   test module.
// - A mutation killed by a hang: its record names no killing test, so only a change to its target
//   file or a non-source input selects it.

import { execFileSync, spawn } from "node:child_process";
import { createHash } from "node:crypto";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

// ------------------------------------------------------------------------------------ constants
/** The paths one run works on, from the crate directory: a throwaway repository in a test, this
 * repository otherwise. */
export function layout(root) {
  const workspace = path.dirname(path.dirname(root)); // src-tauri
  const repo = path.dirname(workspace);
  const crate = path.relative(repo, root).split(path.sep).join("/"); // src-tauri/crates/bridge-tax-audit
  return {
    root,
    workspace,
    repo,
    crate,
    list: path.join(root, "parity", "mutations.json"),
    results: path.join(root, "parity", "mutation-results.json"),
    resultsRel: `${crate}/parity/mutation-results.json`,
    survivors: path.join(root, "parity", "accepted-survivors.json"),
    workers: path.join(workspace, "target", "mutants"),
  };
}

export const ROOT = path.resolve(fs.realpathSync(path.dirname(fileURLToPath(import.meta.url))), "..");
export const DEFAULT = layout(ROOT);
export const PACKAGE = "bridge-tax-audit";
export const COPIED = ["src-tauri", "rust-toolchain.toml"]; // repo-relative: what a worker copy holds
export const DEFAULT_TIMEOUT = 45 * 60;
export const DEFAULT_BUILD_TIMEOUT = 60 * 60;
export const KILL_GRACE = 30; // seconds a cargo group gets after SIGTERM before SIGKILL
export const MARGIN = 3; // --timeout must be at least this many times the unmutated suite's test time

export const KILLED = "killed";
export const SURVIVED = "survived";
export const DID_NOT_APPLY = "did_not_apply";
export const COMPILE_ERROR = "compile_error";
export const TIMEOUT = "timeout";
export const BUILD_TIMEOUT = "build_timeout";
export const FAILED_NO_TEST = "failed_no_test";
export const PASSING = new Set([KILLED, TIMEOUT]); // a test-run timeout is a hang the suite would not let through
export const CRASHED = "<crashed>"; // the test name recorded when a test binary dies without naming a failure

// Crate-relative files that never change what a test does: no crate source names them (a unit
// test holds that), and a change to the list is judged entry by entry (see the header). The
// local parity example is built by `cargo test` and run by no test; a change that stops it
// compiling fails the pull request's own test and clippy runs.
export const INERT = [
  "parity/mutations.mjs",
  "parity/mutations.test.mjs",
  "parity/mutation-results.json",
  "parity/mutations.json",
  "examples/local_parity.rs",
];
// The machine-readable line a failed report ends with, which the nightly puts in its issue.
export const FAILING_MARK = "mutation-nightly-failing:";
// The header ci.yml writes before each open issue's body in the --nightly-issues file.
export const ISSUE_MARK = "mutation-nightly-issue";
export const REPORT_ROWS = 100; // rows per report section; GitHub caps an issue body at 65,536 characters
// A shard that could not start writes this instead of results: a typed reason and the first lines of
// the compiler's output, so the merge can tell "refused to run" from "ran and found nothing".
export const REFUSED_KEY = "refused";
export const REFUSAL_LINES = 40; // compiler lines kept in a refusal
export const REFUSAL_LINE_CHARS = 300; // characters kept of each

/** Run-wide state: a stop request, the cargo groups in flight, and whether any mutation has a verdict
 * (a crash then keeps the records). */
export const state = { stop: false, active: new Set(), madeVerdict: false, killing: Promise.resolve() };

export class Stopped extends Error {
  constructor() {
    super("stopped");
    this.name = "Stopped";
  }
}

// ------------------------------------------------------------------------------------ small helpers
const cmp = (a, b) => (a < b ? -1 : a > b ? 1 : 0);
const sorted = (items, key = (x) => x) => [...items].sort((a, b) => cmp(key(a), key(b)));
const isObject = (v) => v !== null && typeof v === "object" && !Array.isArray(v);
/** An object that holds only the keys it was given, whatever they are called. */
const dict = (entries = {}) => Object.assign(Object.create(null), entries);
const has = (obj, key) => obj != null && Object.prototype.hasOwnProperty.call(obj, key);
const escapeRegExp = (s) => s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
const sha16 = (text) => createHash("sha256").update(text, "utf8").digest("hex").slice(0, 16);
const pyRepr = (v) => (typeof v === "string" ? `'${v.replaceAll("\\", "\\\\").replaceAll("'", "\\'")}'` : String(v));
const pyList = (items) => `[${items.map(pyRepr).join(", ")}]`;
const out = (...parts) => console.log(parts.join(" "));
const err = (...parts) => console.error(parts.join(" "));

/** Python's str.splitlines: the line breaks it knows, and no empty last line. */
export function splitlines(text) {
  const lines = text.split(/\r\n|[\n\r\v\f\x1c-\x1e\x85\u2028\u2029]/);
  if (lines.length && lines[lines.length - 1] === "") lines.pop();
  return lines;
}

/** json.dumps(value, sort_keys=True, ensure_ascii=False) with Python's default separators: the form the
 * mutation hash and the results file are written in. */
export function pyDumps(value) {
  if (value === null || value === undefined) return "null";
  if (typeof value === "string") return JSON.stringify(value);
  if (typeof value === "number" || typeof value === "boolean") return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(pyDumps).join(", ")}]`;
  return `{${sorted(Object.keys(value))
    .map((k) => `${JSON.stringify(k)}: ${pyDumps(value[k])}`)
    .join(", ")}}`;
}

function readJson(file) {
  return JSON.parse(fs.readFileSync(file, "utf8"));
}

// ------------------------------------------------------------------------------------ pure parts
/** A mutation's own identity: its file, `from` and `to`. An edited mutation is unproven. */
export function mutationHash(m) {
  return sha16(pyDumps({ file: m.file, from: m.from, to: m.to }));
}

/** Whether a record proves its mutation killed (a test-run timeout included). */
export function proven(rec) {
  return rec != null && PASSING.has(rec.verdict);
}

/** Whether `m` is listed in the accepted-survivors file for exactly this definition. An entry
 * for an edited mutation (another hash) accepts nothing. */
export function acceptedSurvivor(m, accepted) {
  const entry = has(accepted, m.id) ? accepted[m.id] : undefined;
  return entry != null && entry.mutation === mutationHash(m);
}

/** Whether a record is a verdict the guarantee accepts: killed, or a survivor listed for this
 * exact definition in the accepted-survivors file. */
export function passes(m, rec, accepted = null) {
  return proven(rec) || (rec != null && rec.verdict === SURVIVED && acceptedSurvivor(m, accepted));
}

/** Entries of the accepted-survivors file that accept nothing: an id not in the list, or a
 * hash that is not the mutation's current definition. Each must be updated or removed. */
export function survivorProblems(mutations, accepted) {
  const byId = new Map(mutations.map((m) => [m.id, m]));
  const ids = sorted(Object.keys(accepted));
  return [
    ...ids.filter((i) => !byId.has(i)).map((i) => `${i}: accepted as a survivor but no longer in the list`),
    ...ids
      .filter((i) => byId.has(i) && !acceptedSurvivor(byId.get(i), accepted))
      .map((i) => `${i}: accepted as a survivor for another definition`),
  ];
}

const RUNNING = /^\s*Running (?:unittests )?(\S+) \(/;
const DOCTESTS = /^\s*Doc-tests (\S+)/;
const FAILED = /^test (.+?) \.\.\. FAILED$/;
const COMPILE = /^error\[E\d+\]|^error: could not compile /;
// A test binary that died of its own fault; SIGKILL/SIGTERM (an OOM killer, an operator) are not.
const CRASH_LINE = /^\s*process didn't exit successfully: .*\(signal: \d+, (SIGABRT|SIGSEGV|SIGBUS|SIGILL|SIGFPE)\b/;

/** [failed tests, each qualified by the target cargo reported it under, compile error?], read
 * from `cargo test` output with stderr merged into stdout, so each `Running` line precedes its
 * binary's results and cargo's report of how that binary exited. A doc test is qualified as
 * `doc`. A binary that died of a crash signal (SIGABRT, as a stack overflow aborts, SIGSEGV,
 * SIGBUS, SIGILL, SIGFPE) without naming a failed test is recorded as `<target>::<crashed>`;
 * one killed from outside (SIGKILL, SIGTERM) names nothing and proves nothing. */
export function parseTestOutput(text) {
  const failed = [];
  let target = "?";
  let named = false; // whether the current target has named a failed test
  let compileError = false;
  for (const line of splitlines(text)) {
    let m;
    if ((m = RUNNING.exec(line))) {
      target = m[1];
      named = false;
    } else if ((m = DOCTESTS.exec(line))) {
      target = "doc";
      named = false;
    } else if ((m = FAILED.exec(line))) {
      failed.push(`${target}::${m[1]}`);
      named = true;
    } else if (CRASH_LINE.test(line) && !named) {
      failed.push(`${target}::${CRASHED}`);
      named = true;
    } else if (COMPILE.test(line)) {
      compileError = true;
    }
  }
  return [sorted(new Set(failed)), compileError];
}

function isFile(p) {
  try {
    return fs.statSync(p).isFile();
  } catch {
    return false;
  }
}

/** Whether the crate file `file` defines a function named `fn`. */
function defines(crate, file, fn) {
  let text;
  try {
    text = new TextDecoder("utf-8", { fatal: true }).decode(fs.readFileSync(path.join(crate, file)));
  } catch {
    return false;
  }
  return new RegExp(`\\bfn\\s+${escapeRegExp(fn)}\\s*[(<]`).test(text);
}

/** The crate-relative source file that holds a qualified killing test, or null when unknown.
 * `tests/x.rs::name` lives in tests/x.rs; `src/lib.rs::a::b::name` in the file of the longest
 * module path that exists (src/a/b.rs, src/a/b/mod.rs, then src/a.rs ...), so an inline
 * `mod tests` maps to its parent's file and a `tests.rs` file to itself; `doc::src/x.rs - item
 * (line N)` in src/x.rs (cargo may print it workspace-relative). The file must still exist and
 * define `fn <name>`: a renamed file, a `#[path]` remap or a macro-made test is unknown, and an
 * unknown killer file selects its mutation on every change. A crashed library binary names no
 * test, so its file is unknown too. */
export function killerFile(killer, crate = ROOT) {
  const at = killer.indexOf("::");
  const target = at < 0 ? killer : killer.slice(0, at);
  const name = at < 0 ? "" : killer.slice(at + 2);
  if (target === "doc") {
    let file = name.split(" - ")[0].trim();
    const prefix = path.relative(path.dirname(path.dirname(crate)), crate).split(path.sep).join("/") + "/";
    if (file.startsWith(prefix)) file = file.slice(prefix.length);
    return file.endsWith(".rs") && isFile(path.join(crate, file)) ? file : null;
  }
  if (name === CRASHED) {
    return target.startsWith("tests/") && isFile(path.join(crate, target)) ? target : null;
  }
  const parts0 = name.split("::");
  const fn = parts0[parts0.length - 1];
  if (target.startsWith("tests/")) return defines(crate, target, fn) ? target : null;
  if (target === "src/lib.rs") {
    let parts = parts0.slice(0, -1); // the module path; the last part is the test fn
    while (parts.length) {
      // `book::tests` is src/book/tests.rs if that file exists, else src/book.rs
      for (const candidate of [`src/${parts.join("/")}.rs`, `src/${parts.join("/")}/mod.rs`]) {
        if (isFile(path.join(crate, candidate))) return defines(crate, candidate, fn) ? candidate : null;
      }
      parts = parts.slice(0, -1);
    }
    return defines(crate, "src/lib.rs", fn) ? "src/lib.rs" : null;
  }
  return null;
}

const INCLUDE = /include_(?:str|bytes)!\(\s*"([^"]+)"\s*\)/g;

/** Crate-relative files `sourceFile` pulls in with `include_str!`/`include_bytes!`, followed
 * transitively; each path resolves against the including file's directory, as rustc does. */
export function included(sourceFile, crate = ROOT) {
  const seen = new Set();
  const todo = [sourceFile];
  while (todo.length) {
    const f = todo.pop();
    let text;
    try {
      text = new TextDecoder("utf-8", { fatal: true }).decode(fs.readFileSync(path.join(crate, f)));
    } catch {
      continue;
    }
    for (const match of text.matchAll(INCLUDE)) {
      const parts = [];
      for (const p of [...f.split("/").slice(0, -1), ...match[1].split("/")]) {
        if (p === "..") {
          if (parts.length) parts.pop();
        } else if (p !== "." && p !== "") {
          parts.push(p);
        }
      }
      const file = parts.join("/");
      if (!seen.has(file)) {
        seen.add(file);
        todo.push(file);
      }
    }
  }
  return seen;
}

/** Crate-relative Rust source that the file-level selection rules understand. */
export function isSource(file) {
  return (
    file.endsWith(".rs") &&
    (file.startsWith("src/") || file.startsWith("tests/")) &&
    !file.startsWith("tests/fixtures/")
  );
}

/** Map {id => [reasons]} for every mutation the change selects; see the header. `changed` is
 * crate-relative paths. */
export function select(mutations, results, changed, crate = ROOT, accepted = null, base = null) {
  changed = changed.filter((c) => !INERT.includes(c));
  const other = sorted(changed.filter((c) => !isSource(c)));
  let changedSet = new Set(changed.filter(isSource));
  if ([...changedSet].some((c) => c.startsWith("tests/common/"))) {
    // every integration test binary may include tests/common
    const dir = path.join(crate, "tests");
    for (const entry of fs.readdirSync(dir)) if (entry.endsWith(".rs") && isFile(path.join(dir, entry))) changedSet.add(`tests/${entry}`);
  }
  const picked = new Map();
  for (const m of mutations) {
    const reasons = [];
    const rec = has(results, m.id) ? results[m.id] : undefined;
    if (other.length) {
      const more = other.length > 3 ? ` and ${other.length - 3} more` : "";
      reasons.push(`non-source input changed: ${other.slice(0, 3).join(", ")}${more}`);
    }
    if (changedSet.has(m.file)) reasons.push(`target ${m.file} changed`);
    if (base !== null && (has(base, m.id) ? base[m.id] : undefined) !== mutationHash(m)) reasons.push("new or edited since the base");
    if (!passes(m, rec, accepted)) {
      reasons.push("no killed record");
    } else if (rec.mutation !== mutationHash(m)) {
      reasons.push("definition changed");
    } else {
      const files = new Set();
      for (const k of rec.killers ?? []) {
        const f = killerFile(k, crate);
        if (f === null) {
          reasons.push("a killer's file is unknown");
        } else {
          files.add(f);
          for (const inc of included(f, crate)) files.add(inc);
        }
      }
      const hit = sorted([...files].filter((f) => changedSet.has(f)));
      if (hit.length) reasons.push(`killer file changed: ${hit.join(", ")}`);
    }
    if (reasons.length) picked.set(m.id, [...new Set(reasons)]);
  }
  return picked;
}

/** Whether `rec` proves `m` killed (or an accepted survivor) on exactly the crate tree `tree`,
 * for this definition. */
export function fresh(m, rec, tree, accepted = null) {
  return passes(m, rec, accepted) && rec.crate_tree === tree && rec.mutation === mutationHash(m);
}

/** What stops a merge: each chosen mutation without a fresh record, and each record for a
 * mutation no longer in the list. */
export function verify(chosen, results, tree, order, accepted = null) {
  const problems = chosen
    .filter((m) => !fresh(m, has(results, m.id) ? results[m.id] : undefined, tree, accepted))
    .map((m) => `${m.id}: not proven on crate tree ${tree}`);
  const inList = new Set(order);
  for (const i of sorted(Object.keys(results).filter((k) => !inList.has(k)))) {
    problems.push(`${i}: a record for a mutation no longer in the list (delete it)`);
  }
  return problems;
}

/** The mutation ids the open nightly issues list as failing, from the line `report` writes.
 * `text` holds each issue's body after a `<!-- mutation-nightly-issue N -->` header, as ci.yml
 * writes it. null when ANY issue lacks the line, and when non-empty text holds no header at all
 * (the header drifted from ci.yml's): an issue that cannot be read must not be hidden by
 * another's line. [] when there is no issue. */
export function failingIds(text) {
  if (!text.trim()) return [];
  let blocks = text.split(new RegExp(`<!--\\s*${escapeRegExp(ISSUE_MARK)}\\s+\\d+\\s*-->`));
  if (blocks.length === 1) return null;
  blocks = blocks.slice(1); // anything before the first header belongs to no issue
  const ids = new Set();
  for (const block of blocks) {
    const marks = [...block.matchAll(new RegExp(`<!--\\s*${escapeRegExp(FAILING_MARK)}([^>]*?)-->`, "g"))];
    if (!marks.length) return null;
    for (const mark of marks) for (const i of mark[1].split(/\s+/).filter(Boolean)) ids.add(i);
  }
  return sorted(ids);
}

/** Shard k of n (1-based), whole files per shard, balanced greedily by mutation count. */
export function shard(mutations, k, n) {
  const counts = new Map();
  for (const m of mutations) counts.set(m.file, (counts.get(m.file) ?? 0) + 1);
  const loads = new Array(n).fill(0);
  const owner = new Map();
  for (const f of [...counts.keys()].sort((a, b) => counts.get(b) - counts.get(a) || cmp(a, b))) {
    let i = 0;
    for (let j = 1; j < n; j++) if (loads[j] < loads[i]) i = j;
    owner.set(f, i);
    loads[i] += counts.get(f);
  }
  return mutations.filter((m) => owner.get(m.file) === k - 1);
}

/** Throw unless `refusal` is what a refusing shard writes: a "K/N" shard with 1 <= K <= N, a list of
 * problems and a list of compiler lines, all text. A corrupt document is then "not read", with the
 * rest of the merge intact, rather than a crash with no report. */
export function refusedShard(refusal) {
  let ok = isObject(refusal);
  if (ok) {
    const m = /^(\d+)\/(\d+)$/.exec(typeof refusal.shard === "string" ? refusal.shard : "");
    ok = m !== null && Number(m[1]) >= 1 && Number(m[1]) <= Number(m[2]);
    for (const key of ["problems", "output"]) {
      const lines = refusal[key];
      ok = ok && Array.isArray(lines) && lines.every((x) => typeof x === "string");
    }
  }
  if (!ok) throw new ValueError("not a shard refusal");
}

/** Throw unless `doc` is what a shard that ran writes: an object of records, each an object. Valid
 * JSON of any other shape (null, a number, a string, a list, a record that is not an object) is then
 * "not read", with the rest of the merge intact, rather than a crash or a file merged as if it held
 * nothing (#1041). */
export function shardResults(doc) {
  if (!(isObject(doc) && Object.values(doc).every(isObject))) throw new ValueError("not shard results");
}

export class ValueError extends Error {
  constructor(message) {
    super(message);
    this.name = "ValueError";
  }
}

/** [Markdown, failed?] for a whole-list run. It fails when any mutation was not run or not
 * killed, when a shard's results could not be read, or when the committed results file is stale
 * (a record for a mutation not in the list, or none, or one made for a different definition).
 * A failed report's first lines name every failing id, for the nightly issue; when a shard could
 * not be read, every id in the list, since what it held is unknown. Each section lists at most
 * REPORT_ROWS rows, so the report fits an issue body. A shard that refused to run (`refused`, by
 * "k/n") fails the run and is named with its reason and compiler lines; the ids it would have run
 * are listed apart from ids no shard ran, and go on the failing line like every id not run. */
export function report(mutations, merged, committed, unreadable = [], accepted = null, refused = null) {
  const order = mutations.map((m) => m.id);
  const byId = new Map(mutations.map((m) => [m.id, m]));
  refused = refused ?? {};
  const blocked = new Set();
  for (const key of Object.keys(refused)) {
    const [k, n] = key.split("/").map(Number);
    for (const m of shard(mutations, k, n)) blocked.add(m.id);
  }
  const ran = (i) => has(merged, i);
  const missing = order.filter((i) => !ran(i) && !blocked.has(i));
  const missingRefused = order.filter((i) => !ran(i) && blocked.has(i));
  const bad = order.filter((i) => ran(i) && !passes(byId.get(i), merged[i], accepted)).map((i) => [i, merged[i].verdict]);
  const survivors = order.filter((i) => ran(i) && !proven(merged[i]) && passes(byId.get(i), merged[i], accepted));
  const nowKilled = order.filter((i) => proven(has(merged, i) ? merged[i] : undefined) && acceptedSurvivor(byId.get(i), accepted));
  const staleSurvivors = survivorProblems(mutations, accepted ?? {});
  const staleIds = order.filter((i) => !has(committed, i) || committed[i].mutation !== mutationHash(byId.get(i)));
  const stale = staleIds.map((i) => (!has(committed, i) ? `${i}: no committed record` : `${i}: committed record is for another definition`));
  for (const i of Object.keys(committed)) if (!byId.has(i)) stale.push(`${i}: committed record for a mutation no longer in the list`);
  const timeouts = order.filter((i) => ran(i) && merged[i].verdict === TIMEOUT);
  const thin = order.filter((i) => ran(i) && merged[i].verdict === KILLED && merged[i].killers.length < 2).map((i) => [i, merged[i].killers]);
  const failed = Boolean(missing.length || missingRefused.length || Object.keys(refused).length || bad.length || stale.length || unreadable.length || staleSurvivors.length);
  const lines = [`# Tax-audit mutations: ${failed ? "FAILED" : "all killed"}`, ""];
  if (failed) {
    const ids = unreadable.length
      ? order
      : sorted(new Set([...missing, ...missingRefused, ...bad.map(([i]) => i), ...staleIds]), (i) => order.indexOf(i));
    lines.push(`<!-- ${FAILING_MARK} ${ids.join(" ")} -->`, "");
  }
  lines.push(`${order.length} in the list; ${Object.keys(merged).length} run; ${order.filter((i) => proven(has(merged, i) ? merged[i] : undefined)).length} killed.`, "");
  const refusals = sorted(Object.keys(refused)).map((k) => {
    const r = refused[k];
    const reason = (r.problems ?? []).join("; ") || "no reason recorded";
    return `\`${k}\`: ${reason}` + (r.output && r.output.length ? ` (${r.output[0]})` : "");
  });
  const sections = [
    ["Shard results not read", unreadable.map((u) => `\`${u}\``)],
    ["Shards that refused to run", refusals],
    ["Not killed", bad.map(([i, v]) => `\`${i}\` (${byId.get(i).file}): ${v}`)],
    ["Not run, their shard refused", missingRefused.map((i) => `\`${i}\``)],
    ["Not run", missing.map((i) => `\`${i}\``)],
    ["Stale committed records", stale],
    ["Stale accepted-survivor entries", staleSurvivors],
    ["Accepted survivors", survivors.map((i) => `\`${i}\`: ${accepted[i].reason ?? ""}`)],
    ["Accepted as survivors but now killed: remove the entry (warning)", nowKilled.map((i) => `\`${i}\``)],
    ["Killed by a hang, not a failing test (warning)", timeouts.map((i) => `\`${i}\``)],
    ["Killed by fewer than 2 tests (warning)", thin.map(([i, k]) => `\`${i}\`: ${k.join(", ") || "none"}`)],
  ];
  for (const [title, rows] of sections) {
    if (!rows.length) continue;
    const more = rows.length > REPORT_ROWS ? [`... and ${rows.length - REPORT_ROWS} more`] : [];
    lines.push(`## ${title} (${rows.length})`, "", ...rows.slice(0, REPORT_ROWS).map((r) => `- ${r}`), ...more, "");
  }
  return [lines.join("\n"), failed];
}

/** One record per line, in list order, so two pull requests touching different mutations
 * rarely touch the same line (adjacent lines still conflict: keep both records). */
export function renderResults(records, order) {
  const inOrder = new Set(order);
  const ids = [...order.filter((i) => has(records, i)), ...sorted(Object.keys(records).filter((i) => !inOrder.has(i)))];
  const lines = ids.map((i) => `  ${JSON.stringify(i)}: ${pyDumps(records[i])}`);
  return "{\n" + lines.join(",\n") + (lines.length ? "\n" : "") + "}\n";
}

/** Replace `file` whole, so no reader and no killed run ever leaves half a file. */
export function writeAtomic(file, text) {
  const tmp = path.join(path.dirname(file), `.${path.basename(file)}.${process.pid}.tmp`);
  fs.writeFileSync(tmp, text, "utf8");
  fs.renameSync(tmp, file);
}

const ID = /^[A-Za-z0-9_.-]+$/;

/** Entries the runner cannot judge safely: an id outside [A-Za-z0-9_.-] (the failing-ids line
 * splits on whitespace), or a `file` that is not exactly a tracked crate path in git's spelling
 * (`src/./x.rs` would open the right file but never match a changed path, so a change to it
 * would select nothing). */
export function listProblems(mutations, tracked) {
  const problems = [];
  for (const m of mutations) {
    const i = m.id;
    if (typeof i !== "string" || !ID.test(i)) problems.push(`${pyRepr(i)}: an id may hold only A-Z, a-z, 0-9, '_', '.' and '-'`);
    if (!tracked.has(m.file)) problems.push(`${i}: file ${pyRepr(m.file)} is not a tracked crate path, spelled as git spells it`);
  }
  return problems;
}

/** Why the list cannot be judged at all (exit 2), or null: an id used more than once, or an entry
 * `listProblems` refuses. */
export function loadRefusal(mutations, tracked) {
  const seen = new Map();
  for (const m of mutations) seen.set(m.id, (seen.get(m.id) ?? 0) + 1);
  const repeated = sorted([...seen].filter(([, n]) => n > 1).map(([i]) => i));
  if (repeated.length) return `refusing: mutation ids used more than once: ${pyList(repeated)}`;
  const malformed = listProblems(mutations, tracked);
  return malformed.length ? "refusing: mutation entries the runner cannot judge:\n  " + malformed.join("\n  ") : null;
}

const BIG = 1 << 30;

function gitRaw(args, { cwd, input } = {}) {
  return execFileSync("git", args, { cwd, input, maxBuffer: BIG, stdio: ["pipe", "pipe", "pipe"] });
}

/** git's stdout as text; throws when git fails. */
export function git(args, repo = DEFAULT.repo) {
  return gitRaw(args, { cwd: repo }).toString("utf8");
}

/** Crate-relative paths of the files HEAD tracks under the crate, as git spells them. */
export function trackedFiles(repo = DEFAULT.repo, crate = DEFAULT.crate) {
  const listing = gitRaw(["ls-tree", "-r", "-z", "--name-only", "HEAD", "--", crate], { cwd: repo });
  return new Set(listing.toString("utf8").split("\0").filter(Boolean).map((n) => n.slice(crate.length + 1)));
}

/** The `git status --porcelain` lines that stop a run: any tracked change but the results file. */
export function dirty(porcelain, resultsRel = DEFAULT.resultsRel) {
  return splitlines(porcelain).filter((line) => line.trim() && !line.endsWith(resultsRel));
}

/** Every mutation's `from` text appears exactly once in its file: in the working tree, or in
 * `files` ({crate-relative path: Buffer}) when given. */
export function applyCheck(mutations, crate = ROOT, files = null) {
  const bad = [];
  for (const m of mutations) {
    const file = path.join(crate, m.file);
    let data = null;
    if (files === null) data = isFile(file) ? fs.readFileSync(file) : null;
    else data = has(files, m.file) ? files[m.file] : null;
    const n = data !== null ? countBytes(data, Buffer.from(m.from, "utf8")) : 0;
    if (n !== 1) bad.push(`${m.id}: \`from\` found ${n} times in ${m.file}`);
  }
  return bad;
}

/** bytes.count: non-overlapping occurrences (an empty needle matches between every byte). */
function countBytes(data, needle) {
  if (needle.length === 0) return data.length + 1;
  let n = 0;
  for (let at = data.indexOf(needle); at >= 0; at = data.indexOf(needle, at + needle.length)) n++;
  return n;
}

/** A hash of this crate's committed tree at `ref`, without the results file. */
export function crateTree(ref = "HEAD", repo = DEFAULT.repo, crate = DEFAULT.crate) {
  const results = `${crate}/parity/mutation-results.json`;
  const rows = splitlines(git(["ls-tree", "-r", ref, "--", crate], repo)).filter((r) => !r.endsWith("\t" + results));
  return sha16(rows.join("\n"));
}

/** {id: definition hash} of the mutation list at the merge base of `base` and HEAD ({} when the
 * base has no list): what a change to the list is judged against. */
export function baseDefinitions(base, repo = DEFAULT.repo, crate = DEFAULT.crate) {
  const mb = git(["merge-base", base, "HEAD"], repo).trim();
  let shown;
  try {
    shown = gitRaw(["show", `${mb}:${crate}/parity/mutations.json`], { cwd: repo });
  } catch {
    return {};
  }
  return dict(Object.fromEntries(JSON.parse(shown.toString("utf8")).map((m) => [m.id, mutationHash(m)])));
}

/** Crate-relative paths changed between the merge base of `base` and HEAD's committed tree. */
export function changedSince(base, repo = DEFAULT.repo, crate = DEFAULT.crate) {
  const mb = git(["merge-base", base, "HEAD"], repo).trim();
  // --no-renames: a renamed file is listed under its old name too, which a killer may name.
  // -z: git would otherwise quote a path with non-ASCII bytes, and the quoted form would not
  // match the crate prefix below, so the change would silently select nothing.
  const listing = gitRaw(["diff", "--name-only", "-z", "--no-renames", mb, "HEAD", "--", crate], { cwd: repo });
  return listing
    .toString("utf8")
    .split("\0")
    .filter((n) => n && n.startsWith(crate + "/"))
    .map((n) => n.slice(crate.length + 1));
}

/** Refuse the run with `message` (exit 2) from deep inside a helper. */
export class Refusal extends Error {
  constructor(message) {
    super(message);
    this.name = "Refusal";
  }
}

/** {repo-relative path: committed bytes} for every tracked file under `paths` at `ref`. A tracked
 * symlink or submodule refuses the run: the copy would not reproduce it. */
export function committedFiles(ref = "HEAD", repo = DEFAULT.repo, paths = COPIED) {
  const listing = gitRaw(["ls-tree", "-r", "-z", ref, "--", ...paths], { cwd: repo });
  const entries = [];
  for (const row of listing.toString("utf8").split("\0").filter(Boolean)) {
    const tab = row.indexOf("\t");
    const [mode, kind, sha] = row.slice(0, tab).split(" ");
    const file = row.slice(tab + 1);
    if (kind !== "blob" || mode === "120000") throw new Refusal(`refusing: ${file} is a ${mode === "120000" ? "symlink" : kind}`);
    entries.push([file, sha]);
  }
  const data = gitRaw(["cat-file", "--batch"], { cwd: repo, input: entries.map(([, sha]) => `${sha}\n`).join("") });
  const files = {};
  let pos = 0;
  for (const [file, sha] of entries) {
    const end = data.indexOf(0x0a, pos);
    const header = data.subarray(pos, end).toString("utf8").split(" ");
    if (header[0] !== sha) throw new Refusal(`refusing: git cat-file answered ${header[0]} for ${sha}`);
    const size = Number(header[2]);
    files[file] = Buffer.from(data.subarray(end + 1, end + 1 + size));
    pos = end + 1 + size + 1;
  }
  return files;
}

// ------------------------------------------------------------------------------------ workers
/** Send `sig` to the process group `proc` started; false when the group is already gone. Once its
 * leader is reaped, kill answers ESRCH; on macOS, while the leader is an unreaped zombie and no
 * other member is left, it answers EPERM. EPERM from a leader still running is raised. */
export function signalGroup(proc, sig) {
  try {
    process.kill(-proc.pid, sig);
    return true;
  } catch (e) {
    if (e.code === "ESRCH") return false;
    if (e.code === "EPERM") {
      if (proc.exitCode !== null || proc.signalCode !== null) return false;
      // Node reaps a child only on its event loop: a leader that has exited may still be an unreaped zombie.
      let stat = "";
      try {
        stat = execFileSync("ps", ["-o", "stat=", "-p", String(proc.pid)], { encoding: "utf8" }).trim();
      } catch {}
      if (stat.startsWith("Z")) return false;
    }
    throw e;
  }
}

function waitExit(proc, seconds) {
  if (proc.exitCode !== null || proc.signalCode !== null) return Promise.resolve(true);
  return new Promise((resolve) => {
    const timer = setTimeout(() => resolve(false), seconds * 1000);
    proc.once("exit", () => {
      clearTimeout(timer);
      resolve(true);
    });
  });
}

/** End the process group `proc` started (its own session), and nothing else. */
export async function killGroup(proc, grace = KILL_GRACE) {
  if (!signalGroup(proc, "SIGTERM")) return;
  if (!(await waitExit(proc, grace))) {
    if (signalGroup(proc, "SIGKILL")) await waitExit(proc, grace);
  } else {
    signalGroup(proc, "SIGKILL"); // anything of the group the leader outlived
  }
}

export function killActive() {
  return Promise.all([...state.active].map((p) => killGroup(p)));
}

let outputSeq = 0;

export class Worker {
  constructor(index, root = DEFAULT.workers, lay = DEFAULT) {
    this.dir = path.join(root, `w${index}`);
    this.tree = path.join(this.dir, "src-tauri");
    this.target = path.join(this.dir, "target");
    this.crate = path.join(this.tree, path.relative(lay.workspace, lay.root));
  }

  /** Make the copy hold exactly `files` ({repo-relative path: Buffer}), re-writing only what
   * differs, and delete any other file under the copied src-tauri. */
  sync(files) {
    const wanted = new Set();
    for (const [rel, data] of Object.entries(files)) {
      const dst = path.join(this.dir, rel);
      wanted.add(dst);
      let same = false;
      try {
        same = fs.statSync(dst).isFile() && fs.readFileSync(dst).equals(data);
      } catch {
        same = false;
      }
      if (!same) {
        fs.mkdirSync(path.dirname(dst), { recursive: true });
        fs.writeFileSync(dst, data);
      }
    }
    const walk = (dir) => {
      for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
        const p = path.join(dir, entry.name);
        if (entry.isDirectory()) walk(p);
        else if (!wanted.has(p)) fs.unlinkSync(p);
      }
    };
    if (fs.existsSync(this.tree)) walk(this.tree);
  }

  /** [exit code, or null on a timeout; merged output] of one `cargo test` in this copy. A stop, or
   * any error, ends the process group this call started before it propagates. */
  async run(args, timeout) {
    if (state.stop) throw new Stopped();
    const env = { ...process.env, CARGO_TARGET_DIR: this.target, CARGO_TERM_COLOR: "never" };
    fs.mkdirSync(this.dir, { recursive: true });
    // One file for both streams: the same open file keeps cargo's stderr lines and the tests'
    // stdout lines in the order they were written.
    const outFile = path.join(this.dir, `.cargo-output.${process.pid}.${outputSeq++}.txt`);
    const fd = fs.openSync(outFile, "w");
    let proc = null;
    try {
      proc = spawn("cargo", ["test", "--locked", "--no-fail-fast", "-p", PACKAGE, ...args], {
        cwd: this.crate,
        env,
        detached: true,
        stdio: ["ignore", fd, fd],
      });
      const exited = new Promise((resolve, reject) => {
        proc.once("error", reject);
        proc.once("exit", (code, signal) => resolve([code, signal]));
      });
      state.active.add(proc);
      if (state.stop) throw new Stopped(); // a stop that came between the check above and registering
      let timedOut = false;
      const timer = setTimeout(() => {
        timedOut = true;
        killGroup(proc).catch(() => {});
      }, timeout * 1000);
      let code;
      let signal;
      try {
        [code, signal] = await exited;
      } finally {
        clearTimeout(timer);
      }
      if (state.stop) throw new Stopped(); // the run ended because the stop killed it: no verdict
      const rc = timedOut ? null : (code ?? -(os.constants.signals[signal] ?? 1));
      return [rc, fs.readFileSync(outFile, "utf8")];
    } catch (e) {
      if (proc !== null && proc.pid !== undefined) await killGroup(proc).catch(() => {});
      throw e;
    } finally {
      if (proc !== null) state.active.delete(proc);
      fs.closeSync(fd);
      fs.rmSync(outFile, { force: true });
    }
  }
}

/** [verdict, killers] of one build-then-test step over `targets` (every target when empty),
 * restricted to the exact test `names` when some are given. */
export async function phase(worker, targets, names, timeout, buildTimeout) {
  let [rc, text] = await worker.run(["--no-run", ...targets], buildTimeout);
  if (rc === null) return [BUILD_TIMEOUT, []];
  if (rc !== 0 || parseTestOutput(text)[1]) return [COMPILE_ERROR, []];
  [rc, text] = await worker.run(names.length ? [...targets, "--", "--exact", ...names] : [...targets], timeout);
  if (rc === null) return [TIMEOUT, []];
  const [failed, compileError] = parseTestOutput(text);
  if (compileError) return [COMPILE_ERROR, []]; // a doc test compiles at test time
  if (failed.length) return [KILLED, failed];
  return [rc === 0 ? SURVIVED : FAILED_NO_TEST, []];
}

/** [cargo target flags, exact test names] to run exactly these killers, or null when one of them
 * cannot be run on its own (a doc test, a crashed binary, an unknown target). */
export function killerArgs(killers) {
  const targets = new Set();
  const names = new Set();
  for (const k of killers) {
    const at = k.indexOf("::");
    const target = at < 0 ? k : k.slice(0, at);
    const name = at < 0 ? "" : k.slice(at + 2);
    if (name === CRASHED) return null;
    if (target === "src/lib.rs") targets.add("--lib");
    else if (target.startsWith("tests/") && target.endsWith(".rs")) targets.add(`--test=${path.posix.basename(target, ".rs")}`);
    else return null;
    names.add(name);
  }
  if (!targets.size) return null;
  return [sorted(targets), sorted(names)];
}

const isoSeconds = () => new Date().toISOString().replace(/\.\d{3}Z$/, "+00:00");

export async function runOne(worker, m, record, full, timeout, buildTimeout = DEFAULT_BUILD_TIMEOUT) {
  const file = path.join(worker.crate, m.file);
  const original = fs.readFileSync(file);
  const from = Buffer.from(m.from, "utf8");
  const to = Buffer.from(m.to, "utf8");
  const base = { mutation: mutationHash(m), time: isoSeconds() };
  const n = countBytes(original, from);
  if (n !== 1) return { ...base, verdict: DID_NOT_APPLY, killers: [], mode: "none", note: `${n} matches` };
  const at = original.indexOf(from);
  fs.writeFileSync(file, Buffer.concat([original.subarray(0, at), to, original.subarray(at + from.length)]));
  try {
    if (!full && proven(record)) {
      const spec = killerArgs(record.killers ?? []);
      if (spec) {
        const [verdict, failed] = await phase(worker, spec[0], spec[1], timeout, buildTimeout);
        if (PASSING.has(verdict)) return { ...base, verdict, killers: failed, mode: "killers-first" };
      }
    }
    const [verdict, failed] = await phase(worker, [], [], timeout, buildTimeout);
    return { ...base, verdict, killers: failed, mode: "full" };
  } finally {
    fs.writeFileSync(file, original);
  }
}

/** The compiler's own lines from cargo's output, from the first `error` on: at most REFUSAL_LINES of
 * them, each cut to REFUSAL_LINE_CHARS, so a refusal says why without holding a log. */
export function compilerLines(text) {
  const lines = splitlines(text);
  const start = lines.findIndex((line) => line.startsWith("error"));
  if (start < 0) return [];
  return lines.slice(start, start + REFUSAL_LINES).map((line) => line.slice(0, REFUSAL_LINE_CHARS));
}

/** Problems with the unmutated copies: a build that fails or hangs, a test that does not pass,
 * or a `timeout` under MARGIN times the unmutated suite's measured test time (a slow run would
 * then be counted as a hang). Every worker builds; the full suite runs once, on the first.
 * When `details` is given, the compiler's first lines of the first failed build go into it. */
export async function baseline(workers, timeout, buildTimeout, details = null) {
  const problems = [];
  const name = (w) => path.basename(w.dir);
  for (const w of workers) {
    const [rc, text] = await w.run(["--no-run"], buildTimeout);
    if (rc === null || rc !== 0 || parseTestOutput(text)[1]) {
      problems.push(`${name(w)}: ${rc === null ? BUILD_TIMEOUT : COMPILE_ERROR}`);
      if (details !== null && !details.length) details.push(...compilerLines(text));
    }
  }
  if (problems.length) return problems;
  const w = workers[0];
  const start = performance.now();
  const [rc, text] = await w.run([], timeout);
  const took = (performance.now() - start) / 1000;
  const [failed, compileError] = parseTestOutput(text);
  if (rc === null) problems.push(`${name(w)}: the unmutated suite exceeded --timeout ${timeout}s`);
  else if (compileError) {
    problems.push(`${name(w)}: ${COMPILE_ERROR}`);
    if (details !== null) details.push(...compilerLines(text));
  } else if (failed.length || rc !== 0) problems.push(`${name(w)}: fails unmutated: ${failed.join(", ") || `exit ${rc}`}`);
  else if (timeout < MARGIN * took) problems.push(`--timeout ${timeout}s is under ${MARGIN}x the unmutated suite's ${took.toFixed(0)}s`);
  return problems;
}

const GATE_STALE_MS = 10_000; // a takeover gate that names no owner and is older than this was left by a run that died inside it
let claimSeq = 0;

function pidAlive(pid) {
  if (!Number.isInteger(pid) || pid <= 0) return false;
  try {
    process.kill(pid, 0);
    return true;
  } catch (e) {
    return e.code === "EPERM";
  }
}

/** Whether a takeover gate was left behind. A gate names its owner's pid in a file inside it, so it is stale
 * exactly when that run is gone, however long a live owner has been stalled (suspended, or asleep with its
 * machine). A gate with no owner yet is one a run has just made, or one that run died before naming: only the
 * second is older than GATE_STALE_MS. A crashed run's pid that a later process reuses keeps its gate until someone removes
 * it, as a claim already stays with a reused pid (pidAlive cannot tell the two processes apart). */
function gateIsStale(gate) {
  const owner = Number.parseInt(fs.readFileSync(path.join(gate, "owner"), "utf8"), 10);
  if (Number.isInteger(owner) && owner > 0) return !pidAlive(owner);
  throw new Error("no owner named");
}

function claimedBy(lock) {
  try {
    return Number.parseInt(fs.readFileSync(lock, "utf8"), 10);
  } catch {
    return null; // no claim
  }
}

/** An exclusive claim on the worker directory for this run, or null when another live run holds it:
 * two runs sharing copies would apply and undo each other's mutations.
 *
 * The claim is a file named `.lock` that holds the holder's pid. It is created by hard-linking a
 * finished file of its own, so it appears whole or not at all: no run ever reads a claim that has no
 * pid yet, and a second claimant cannot overwrite it (link fails when the name exists). A claim whose
 * holder is gone is taken over, and only inside a short-lived gate (`.lock.takeover`, made with mkdir,
 * which is atomic): while one run holds the gate, no other run removes a claim, so a run that has just
 * made a new claim cannot have it removed by a run that read the old one. */
export function lockWorkdir(workdir) {
  fs.mkdirSync(workdir, { recursive: true });
  const lock = path.join(workdir, ".lock");
  const gate = path.join(workdir, ".lock.takeover");
  const mine = path.join(workdir, `.lock.${process.pid}.${claimSeq++}.tmp`);
  fs.writeFileSync(mine, `${process.pid}\n`);
  const pause = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
  try {
    for (let attempt = 0; attempt < 200; attempt++) {
      try {
        fs.linkSync(mine, lock);
        const release = () => {
          if (claimedBy(lock) === process.pid) fs.rmSync(lock, { force: true });
        };
        process.once("exit", release);
        return {
          release: () => {
            process.removeListener("exit", release);
            release();
          },
        };
      } catch (e) {
        if (e.code !== "EEXIST") throw e;
      }
      const holder = claimedBy(lock);
      if (holder !== null && pidAlive(holder)) return null;
      // A holder that died without releasing (or a claim that holds no pid): take over under the gate.
      try {
        fs.mkdirSync(gate);
      } catch (e) {
        if (e.code !== "EEXIST") throw e;
        try {
          let stale;
          try {
            stale = gateIsStale(gate);
          } catch {
            // No owner named yet (or the gate just went): old enough means its maker died before naming itself.
            stale = Date.now() - fs.statSync(gate).mtimeMs > GATE_STALE_MS;
          }
          if (stale) fs.rmSync(gate, { recursive: true, force: true });
        } catch {
          // another run removed it first
        }
        pause(20);
        continue;
      }
      let named = false;
      try {
        try {
          fs.writeFileSync(path.join(gate, "owner"), `${process.pid}\n`);
          named = true;
        } catch (e) {
          // This run was suspended between making the gate and naming itself for longer than GATE_STALE_MS, and
          // another run took the unnamed gate as abandoned: it is no longer ours to use or to remove.
          if (e.code === "ENOENT") continue;
          throw e;
        }
        const again = claimedBy(lock);
        if (again !== null && !pidAlive(again)) fs.rmSync(lock, { force: true });
      } finally {
        if (named) fs.rmSync(gate, { recursive: true, force: true });
      }
    }
    return null;
  } finally {
    fs.rmSync(mine, { force: true });
  }
}

// ------------------------------------------------------------------------------------ main
export function loadResults(file) {
  return isFile(file) ? dict(readJson(file)) : dict();
}

const OPTIONS = {
  "--changed-since": { key: "changedSince", value: "BASE" },
  "--verify": { key: "verify", flag: true },
  "--nightly-issues": { key: "nightlyIssues", value: "FILE" },
  "--list": { key: "list", flag: true },
  "--full": { key: "full", flag: true },
  "--jobs": { key: "jobs", value: "N", int: true },
  "--shard": { key: "shard", value: "K/N" },
  "--timeout": { key: "timeout", value: "SECONDS", int: true },
  "--build-timeout": { key: "buildTimeout", value: "SECONDS", int: true },
  "--results": { key: "results", value: "FILE" },
  "--workdir": { key: "workdir", value: "DIR" },
  "--merge": { key: "merge", many: true },
  "--report": { key: "report", value: "MD" },
  "--check-anchors": { key: "checkAnchors", flag: true },
  "--killers-from": { key: "killersFrom", value: "FILE" },
};

export const USAGE = `usage: mutations.mjs [ids ...] [--changed-since BASE] [--verify] [--nightly-issues FILE] [--list]
                    [--full] [--jobs N] [--shard K/N] [--timeout SECONDS] [--build-timeout SECONDS]
                    [--results FILE] [--workdir DIR] [--merge FILE ...] [--report MD] [--check-anchors]
                    [--killers-from FILE]`;

/** Read the command line (the options of the Python runner this replaces). Throws a ValueError with
 * the usage when it cannot. */
export function parseArgs(argv, lay = DEFAULT) {
  const args = {
    ids: [],
    changedSince: null,
    verify: false,
    nightlyIssues: null,
    list: false,
    full: false,
    jobs: 1,
    shard: null,
    timeout: DEFAULT_TIMEOUT,
    buildTimeout: DEFAULT_BUILD_TIMEOUT,
    results: null,
    workdir: lay.workers,
    merge: null,
    report: null,
    checkAnchors: false,
    killersFrom: null,
    help: false,
  };
  const fail = (message) => {
    throw new ValueError(`${USAGE}\nmutations.mjs: error: ${message}`);
  };
  for (let i = 0; i < argv.length; i++) {
    let token = argv[i];
    if (token === "-h" || token === "--help") {
      args.help = true;
      continue;
    }
    if (!token.startsWith("--") || token === "--") {
      if (token.startsWith("-") && token !== "-" && token !== "--") fail(`unrecognized arguments: ${token}`);
      args.ids.push(token);
      continue;
    }
    let inline = null;
    const eq = token.indexOf("=");
    if (eq > 0) {
      inline = token.slice(eq + 1);
      token = token.slice(0, eq);
    }
    const opt = OPTIONS[token];
    if (!opt) fail(`unrecognized arguments: ${token}`);
    if (opt.flag) {
      if (inline !== null) fail(`argument ${token}: ignored explicit argument '${inline}'`);
      args[opt.key] = true;
    } else if (opt.many) {
      const files = inline !== null ? [inline] : [];
      while (inline === null && i + 1 < argv.length && !argv[i + 1].startsWith("--")) files.push(argv[++i]);
      if (!files.length) fail(`argument ${token}: expected at least one argument`);
      args[opt.key] = files;
    } else {
      let value = inline;
      if (value === null) {
        if (i + 1 >= argv.length) fail(`argument ${token}: expected one argument`);
        value = argv[++i];
      }
      if (opt.int) {
        if (!/^[+-]?\d+$/.test(value.trim())) fail(`argument ${token}: invalid int value: ${pyRepr(value)}`);
        value = Number.parseInt(value, 10);
      }
      args[opt.key] = value;
    }
  }
  return args;
}

/** Python exits 1 on an uncaught exception, which here would mean "a mutation was not killed": any
 * uncaught error is exit 2, and (before the first mutation of a shard) a refusal written to --results. */
export async function main(argv = process.argv.slice(2), lay = DEFAULT) {
  let args;
  try {
    args = parseArgs(argv, lay);
  } catch (e) {
    if (e instanceof ValueError) {
      err(e.message);
      return 2;
    }
    throw e;
  }
  if (args.help) {
    out(USAGE);
    return 0;
  }
  state.stop = false;
  state.madeVerdict = false;
  try {
    return await run(args, lay);
  } catch (e) {
    if (e instanceof Refusal) {
      err(e.message);
      return 2;
    }
    const name = e instanceof Error ? e.name : typeof e;
    const message = e instanceof Error ? e.message : String(e);
    const crash = `crashed: ${name}`;
    const frame = /\n\s+at (?:(.+?) \()?(?:file:\/\/)?([^()\n]+?):(\d+):\d+\)?/.exec(e instanceof Error ? (e.stack ?? "") : "");
    const detail = [`${name}: ${message}`.slice(0, REFUSAL_LINE_CHARS), `at ${frame ? `${path.basename(frame[2])}:${frame[3]} in ${frame[1] ?? "<module>"}` : "unknown"}`];
    err(`${crash}: ${detail[0]}\n  ${detail[1]}`);
    if (args.shard && args.results && !state.madeVerdict) {
      // nothing was run: a refusal
      try {
        writeAtomic(args.results, JSON.stringify({ [REFUSED_KEY]: { shard: args.shard, problems: [crash], output: detail } }, null, 1) + "\n");
      } catch (werr) {
        err(`the refusal could not be written: ${werr.name}`);
      }
    }
    return 2;
  }
}

/** `--check-anchors`: the list, the tracked paths and the files all read from HEAD's committed
 * bytes, never the working tree, so the answer is about the head that will be run. Exit 2 when an
 * anchor is not exactly once (as for a list the runner cannot judge), else 0. No build, no copy. */
export function checkAnchors(args, lay = DEFAULT) {
  const others = [
    ["ids", args.ids.length > 0],
    ["--verify", args.verify],
    ["--list", args.list],
    ["--merge", args.merge !== null],
    ["--changed-since", args.changedSince !== null],
    ["--results", args.results !== null],
    ["--nightly-issues", args.nightlyIssues !== null],
    ["--report", args.report !== null],
    ["--full", args.full],
    ["--shard", args.shard !== null],
    ["--killers-from", args.killersFrom !== null],
  ]
    .filter(([, set]) => set)
    .map(([name]) => name);
  if (others.length) {
    err(`refusing: --check-anchors takes no other mode or option: ${pyList(others)}`);
    return 2;
  }
  const head = git(["rev-parse", "HEAD"], lay.repo).trim();
  const files = {};
  for (const [p, data] of Object.entries(committedFiles(head, lay.repo, [lay.crate]))) files[p.slice(lay.crate.length + 1)] = data;
  const mutations = JSON.parse(files[path.relative(lay.root, lay.list).split(path.sep).join("/")].toString("utf8"));
  const refusal = loadRefusal(mutations, new Set(Object.keys(files)));
  if (refusal) {
    err(refusal);
    return 2;
  }
  const stale = applyCheck(mutations, lay.root, files);
  for (const line of stale.slice(0, REPORT_ROWS)) out(`STALE ${line}`);
  if (stale.length > REPORT_ROWS) out(`... and ${stale.length - REPORT_ROWS} more`);
  out(`${stale.length} of ${mutations.length} anchors are not exactly once in HEAD's bytes (${head})`);
  return stale.length ? 2 : 0;
}

export async function run(args, lay = DEFAULT) {
  if (args.checkAnchors) return checkAnchors(args, lay); // before the working-tree list is read
  // --killers-from names records read ONLY for their killer lists (a proof run's killers-first pass);
  // they are never results: the run's own results file starts as it is, and only what this run
  // measured is written. A missing file would quietly make every mutation run the full suite.
  if (args.killersFrom !== null) {
    const clash = [
      ["--full", args.full],
      ["--merge", args.merge !== null],
      ["--verify", args.verify],
      ["--list", args.list],
    ]
      .filter(([, set]) => set)
      .map(([name]) => name);
    if (clash.length) {
      err(`refusing: --killers-from is for a run of mutations without --full: ${pyList(clash)}`);
      return 2;
    }
    if (!isFile(args.killersFrom)) {
      err(`refusing: --killers-from ${args.killersFrom} is not a file, so no mutation would run its killers first`);
      return 2;
    }
  }

  const mutations = readJson(lay.list);
  const order = mutations.map((m) => m.id);
  const refusal = loadRefusal(mutations, trackedFiles(lay.repo, lay.crate));
  if (refusal) {
    err(refusal);
    return 2;
  }
  const unknown = sorted(args.ids.filter((i) => !order.includes(i)));
  if (unknown.length) {
    err(`refusing: no such mutation ids: ${pyList(unknown)}`);
    return 2;
  }

  // Always on: every mutation in the list applies exactly once.
  const stale = applyCheck(mutations, lay.root);
  const accepted = loadResults(lay.survivors);
  for (const line of stale) out(`STALE ${line}`);

  if (args.merge) {
    if (args.results === null) {
      err("refusing: --merge writes --results; name that file (the committed one is never the default here)");
      return 2;
    }
    const merged = dict();
    const unreadable = [];
    const refused = dict();
    for (const file of args.merge) {
      try {
        const doc = readJson(file);
        if (isObject(doc) && has(doc, REFUSED_KEY)) {
          // a shard that could not start: a reason, never results
          refusedShard(doc[REFUSED_KEY]);
          refused[doc[REFUSED_KEY].shard] = doc[REFUSED_KEY];
        } else {
          shardResults(doc);
          Object.assign(merged, doc);
        }
      } catch (e) {
        if (!(e instanceof ValueError || e instanceof SyntaxError || e.code !== undefined)) throw e;
        unreadable.push(`${file}: ${readFailure(e)}`);
      }
    }
    const [text, failed] = report(mutations, merged, loadResults(lay.results), unreadable, accepted, refused);
    writeAtomic(args.results, renderResults(merged, order));
    if (args.report) writeAtomic(args.report, text + "\n");
    out(text);
    return failed ? 1 : 0;
  }

  const resultsPath = args.results ?? lay.results;
  const results = loadResults(resultsPath);
  const head = git(["rev-parse", "HEAD"], lay.repo).trim(); // read once: every record names this commit's tree
  const tree = crateTree(head, lay.repo, lay.crate);
  let reasons = new Map();
  let changed = [];
  if (args.changedSince) {
    changed = changedSince(args.changedSince, lay.repo, lay.crate);
    reasons = select(mutations, results, changed, lay.root, accepted, baseDefinitions(args.changedSince, lay.repo, lay.crate));
  }
  if (args.nightlyIssues && changed.length) {
    const required = failingIds(fs.readFileSync(args.nightlyIssues, "utf8"));
    if (required === null) {
      err("refusing: an open mutation-nightly issue has no failing-ids line, so no change can show it re-proves what failed; a maintainer closes that issue by hand to let the fix through");
      return 1;
    }
    for (const i of required) {
      if (order.includes(i)) reasons.set(i, [...(reasons.get(i) ?? []), "failing on the nightly run"]);
    }
  }
  let chosen = mutations.filter((m) => (!args.ids.length || args.ids.includes(m.id)) && (!args.changedSince || reasons.has(m.id)));
  if (args.shard) {
    const [k, n] = args.shard.split("/").map((x) => {
      if (!/^\s*[+-]?\d+\s*$/.test(x)) throw new ValueError(`invalid literal for int(): ${pyRepr(x)}`);
      return Number.parseInt(x, 10);
    });
    chosen = shard(chosen, k, n);
  }
  const record = (m) => (has(results, m.id) ? results[m.id] : undefined);

  if (args.list || args.verify) {
    for (const m of chosen) {
      const why = (reasons.get(m.id) ?? ["listed"]).join("; ");
      out(`${m.id}: ${why}` + (fresh(m, record(m), tree, accepted) ? "" : "  [NOT PROVEN ON THIS TREE]"));
    }
    const problems = [...verify(chosen, results, tree, order, accepted), ...survivorProblems(mutations, accepted)];
    const unproven = chosen.filter((m) => !fresh(m, record(m), tree, accepted)).length;
    for (const p of problems) if (!p.includes("not proven on crate tree")) out(p);
    out(`\n${chosen.length} selected, ${chosen.length - unproven} proven on crate tree ${tree}`);
    if (args.verify) {
      if (problems.length) {
        err(
          "Update the branch from master, re-run the selection with " +
            "`node parity/mutations.mjs --changed-since BASE`, delete any record for a retired " +
            `mutation, then commit ${lay.resultsRel}.`,
        );
      }
      return problems.length || stale.length ? 1 : 0;
    }
    return 0;
  }

  const blocking = dirty(git(["status", "--porcelain", "--untracked-files=no", "--", ...COPIED], lay.repo), lay.resultsRel);
  if (blocking.length) {
    err("refusing: uncommitted changes to tracked files the copies hold; a verdict must describe a committed tree:\n" + blocking.join("\n"));
    return 2;
  }
  const lock = lockWorkdir(args.workdir);
  if (lock === null) {
    err(`refusing: another run holds ${args.workdir}`);
    return 2;
  }
  try {
    const killers = args.killersFrom === null ? dict() : loadResults(args.killersFrom);
    return await runMutations({ args, lay, mutations, order, accepted, results, killers, resultsPath, head, tree, chosen, stale });
  } finally {
    lock.release();
  }
}

/** The kind of failure a shard file had, named as the Python runner named it. */
function readFailure(e) {
  if (e instanceof SyntaxError) return "JSONDecodeError";
  if (e instanceof ValueError) return "ValueError";
  return e.code === "ENOENT" ? "FileNotFoundError" : e.code === "EISDIR" ? "IsADirectoryError" : e.code === "EACCES" ? "PermissionError" : "OSError";
}

async function runMutations({ args, lay, mutations, order, accepted, results, killers, resultsPath, head, tree, chosen, stale }) {
  const files = committedFiles(head, lay.repo);
  const jobs = Math.max(1, Math.min(args.jobs, chosen.length || 1));
  const workers = Array.from({ length: jobs }, (_, i) => new Worker(i, args.workdir, lay));
  const onSignal = () => {
    state.stop = true;
    state.killing = killActive();
  };
  process.on("SIGINT", onSignal);
  process.on("SIGTERM", onSignal);
  try {
    const details = [];
    let problems;
    try {
      for (const w of workers) w.sync(files);
      problems = await baseline(workers, args.timeout, args.buildTimeout, details);
    } catch (e) {
      if (e instanceof Stopped) {
        await state.killing;
        err("stopped before any mutation");
        return 130;
      }
      throw e;
    }
    if (problems.length) {
      err("refusing: the unmutated copy does not pass; fix that first:\n  " + problems.join("\n  "));
      if (details.length) err("the compiler said:\n  " + details.join("\n  "));
      if (args.shard && args.results) {
        // the merge reads this as a refusal, not as results: nothing was run
        writeAtomic(resultsPath, JSON.stringify({ [REFUSED_KEY]: { shard: args.shard, problems, output: details } }, null, 1) + "\n");
      }
      return 2;
    }
    const done = new Map();
    let next = 0;
    let failure = null;
    const lane = async (w) => {
      while (!state.stop) {
        const m = chosen[next++];
        if (m === undefined) return;
        let rec;
        try {
          // This run's own record first; else a --killers-from record, used only for its killer list.
          const prior = has(results, m.id) ? results[m.id] : has(killers, m.id) ? killers[m.id] : undefined;
          rec = await runOne(w, m, prior, args.full, args.timeout, args.buildTimeout);
        } catch (e) {
          if (e instanceof Stopped) return;
          throw e;
        }
        rec.commit = head;
        rec.crate_tree = tree;
        done.set(m.id, rec);
        state.madeVerdict = true;
        const kill = rec.killers.length ? ` by ${rec.killers.length}: ${rec.killers.join(", ")}` : "";
        out(`${m.id} ${rec.verdict} (${rec.mode})${kill}`);
        results[m.id] = rec;
        writeAtomic(resultsPath, renderResults(Object.fromEntries(order.filter((i) => has(results, i)).map((i) => [i, results[i]])), order));
      }
    };
    const settled = await Promise.allSettled(workers.map((w) => lane(w).catch((e) => { failure ??= e; state.stop = true; throw e; })));
    await state.killing;
    if (failure !== null) throw failure;
    if (settled.some((s) => s.status === "rejected")) throw settled.find((s) => s.status === "rejected").reason;
    if (state.stop) {
      err(`\nstopped after ${done.size} of ${chosen.length}; the results file keeps those`);
      return 130;
    }
    const bad = chosen.filter((m) => !passes(m, done.get(m.id), accepted));
    const thin = chosen.filter((m) => done.get(m.id).verdict === KILLED && done.get(m.id).killers.length < 2);
    out(`\n${chosen.length - bad.length} of ${chosen.length} killed`);
    for (const m of bad) out(`  ${m.id} (${m.by}: ${m.what}): ${done.get(m.id).verdict}`);
    if (thin.length) {
      out(`\nwarning: ${thin.length} killed by fewer than 2 tests (a lower bound in killers-first mode):`);
      for (const m of thin) out(`  ${m.id} (${done.get(m.id).mode}): ${done.get(m.id).killers.join(", ")}`);
    }
    return bad.length || stale.length ? 1 : 0;
  } finally {
    process.removeListener("SIGINT", onSignal);
    process.removeListener("SIGTERM", onSignal);
  }
}

if (process.argv[1] && fs.realpathSync(process.argv[1]) === fs.realpathSync(fileURLToPath(import.meta.url))) {
  process.exitCode = await main();
}
