// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import test from "node:test";

import { ACK_DIR, MAX_REASON, SURFACE_PATH, checkAck, parseAck, parseNameStatus, parsePins, historicPaths, withdrawnPins } from "./check-surface-ack.mjs";

// ---- the pure core ----

const pin = (path, reason) => (reason === undefined ? { path } : { path, reason });
const BASE = [pin("a.txt"), pin("b.txt"), pin("c/d.txt")];
const ack = (n, paths, extra = []) => ({
  path: `${ACK_DIR}pr-${n}.txt`,
  content: `${[...paths, "reviewer: octocat", ...extra].join("\n")}\n`,
});
const check = (o = {}) =>
  checkAck({
    basePins: BASE,
    headPins: BASE,
    changed: [],
    acks: { added: [], modified: [], deleted: [] },
    prNumber: 7,
    ...o,
    ...(o.acks ? { acks: { added: [], modified: [], deleted: [], ...o.acks } } : {}),
  });
const mod = (path, status = "M") => ({ status, path });
const failsWith = (result, pattern) => {
  assert.equal(result.ok, false, "expected a failure");
  assert.match(result.reasons.join("\n"), pattern);
  return result;
};

test("no pinned change and no ack passes, and unpinned changes are ignored", () => {
  const r = check({ changed: [mod("src/other.rs"), mod(`${ACK_DIR}pr-1.txt`, "A")] });
  assert.deepEqual([r.ok, r.reasons, r.touched], [true, [], []]);
});

test("a pinned file modified without an ack fails and names the file", () => {
  const r = failsWith(check({ changed: [mod("a.txt")] }), /exactly one ack must be added; found 0/);
  assert.deepEqual(r.touched, ["a.txt"]);
});

test("an exact ack passes; the first reason is the first failure", () => {
  const changed = [mod("a.txt"), mod("c/d.txt")];
  assert.equal(check({ changed, acks: { added: [ack(7, ["a.txt", "c/d.txt"])] } }).ok, true);
  const bad = check({ changed, acks: { added: [ack(9, ["a.txt"])] } });
  assert.match(bad.reasons[0], /expected .*pr-7\.txt/);
  assert.ok(bad.reasons.length >= 2);
});

test("an ack whose path set differs from the touched set fails, naming missing and extra paths", () => {
  const changed = [mod("a.txt")];
  failsWith(check({ changed, acks: { added: [ack(7, ["a.txt", "b.txt"])] } }), /not changed pinned paths: b\.txt/);
  failsWith(check({ changed: [mod("a.txt"), mod("b.txt")], acks: { added: [ack(7, ["a.txt"])] } }), /missing changed pinned path\(s\): b\.txt/);
  failsWith(check({ changed, acks: { added: [ack(7, ["a.txt", "zzz-unpinned.txt"])] } }), /zzz-unpinned\.txt/);
});

test("a stray ack when nothing pinned changed is an error", () => {
  failsWith(check({ changed: [mod("x.rs")], acks: { added: [ack(7, [])] } }), /no pinned path changed/);
});

test("another ack modified or deleted alongside a pinned change fails", () => {
  const base = { changed: [mod("a.txt")] };
  const mine = ack(7, ["a.txt"]);
  failsWith(check({ ...base, acks: { added: [mine], modified: [`${ACK_DIR}pr-3.txt`] } }), /pr-3\.txt.*modified/);
  failsWith(check({ ...base, acks: { added: [mine], deleted: [`${ACK_DIR}pr-3.txt`] } }), /pr-3\.txt.*deleted/);
  failsWith(check({ changed: [], acks: { modified: [`${ACK_DIR}pr-3.txt`] } }), /modified/);
  failsWith(check({ ...base, acks: { added: [mine, ack(8, ["a.txt"])] } }), /found 2/);
});

test("cleanup-only deletion of old acks passes", () => {
  const r = check({ changed: [], acks: { deleted: [`${ACK_DIR}pr-3.txt`, `${ACK_DIR}pr-4.txt`] } });
  assert.equal(r.ok, true);
});

test("a rename counts both the old and the new name", () => {
  const changed = [{ status: "R", oldPath: "a.txt", path: "unpinned-new.txt" }];
  assert.deepEqual(check({ changed }).touched, ["a.txt"]);
  const intoPin = [{ status: "R", oldPath: "unpinned-old.txt", path: "b.txt" }];
  assert.deepEqual(check({ changed: intoPin }).touched, ["b.txt"]);
  const both = [{ status: "R", oldPath: "a.txt", path: "b.txt" }];
  assert.equal(check({ changed: both, acks: { added: [ack(7, ["a.txt", "b.txt"])] } }).ok, true);
  failsWith(check({ changed: both, acks: { added: [ack(7, ["b.txt"])] } }), /missing changed pinned path\(s\): a\.txt/);
});

test("added, deleted, copied, type-changed and mode-only changes all count as touches", () => {
  for (const status of ["A", "D", "C", "T", "M"]) {
    const r = check({ changed: [mod("b.txt", status)] });
    assert.deepEqual(r.touched, ["b.txt"], `status ${status}`);
    assert.equal(r.ok, false);
  }
});

test("an added pin needs a non-empty reason of at most 500 characters, even if its file did not change", () => {
  const headPins = [...BASE, pin("e.txt")];
  const acks = { added: [ack(7, ["e.txt"])] };
  const r = failsWith(check({ headPins, acks }), /e\.txt: a pin added by this pull request needs a non-empty "reason"/);
  assert.deepEqual(r.touched, ["e.txt"]);
  failsWith(check({ headPins: [...BASE, pin("e.txt", "   ")], acks }), /non-empty "reason"/);
  failsWith(check({ headPins: [...BASE, pin("e.txt", "x".repeat(MAX_REASON + 1))], acks }), /over 500/);
  assert.equal(check({ headPins: [...BASE, pin("e.txt", "x".repeat(MAX_REASON))], acks }).ok, true);
});

test("a pin-list-only change (added pin, file unchanged) requires an ack", () => {
  const headPins = [...BASE, pin("e.txt", "needed for the gate")];
  failsWith(check({ headPins }), /exactly one ack must be added; found 0/);
  assert.equal(check({ headPins, acks: { added: [ack(7, ["e.txt"])] } }).ok, true);
});

test("a removed pin needs a removed-pin line and is not listed as a path", () => {
  const headPins = BASE.filter((p) => p.path !== "b.txt");
  const r = check({ headPins });
  assert.deepEqual([r.touched, r.removed], [["b.txt"], ["b.txt"]]);
  failsWith(check({ headPins, acks: { added: [ack(7, [])] } }), /need a "removed-pin:" line: b\.txt/);
  assert.equal(check({ headPins, acks: { added: [ack(7, [], ["removed-pin: b.txt"])] } }).ok, true);
  // the removed file was also deleted in the diff: still only a removed-pin line
  assert.equal(check({ headPins, changed: [mod("b.txt", "D")], acks: { added: [ack(7, [], ["removed-pin: b.txt"])] } }).ok, true);
  failsWith(check({ headPins, acks: { added: [ack(7, ["b.txt"], ["removed-pin: b.txt"])] } }), /not changed pinned paths: b\.txt/);
  failsWith(check({ changed: [mod("a.txt")], acks: { added: [ack(7, ["a.txt"], ["removed-pin: c/d.txt"])] } }), /were not removed: c\/d\.txt/);
});

test("a 64-hex token anywhere in the ack is rejected", () => {
  const hex = "0123456789abcdef".repeat(4);
  const changed = [mod("a.txt")];
  failsWith(check({ changed, acks: { added: [ack(7, ["a.txt"], [`# ${hex}`])] } }), /64-hex/);
  failsWith(check({ changed, acks: { added: [ack(7, ["a.txt"], [`reviewer: ${hex.toUpperCase()}`])] } }), /64-hex/);
});

test("the ack must be named for the supplied pull request number", () => {
  const changed = [mod("a.txt")];
  failsWith(check({ changed, acks: { added: [ack(8, ["a.txt"])] } }), /expected .*pr-7\.txt/);
  failsWith(check({ changed, prNumber: NaN, acks: { added: [ack(7, ["a.txt"])] } }), /number is not known/);
  failsWith(check({ changed, acks: { added: [{ path: `${ACK_DIR}my-branch.txt`, content: ack(7, ["a.txt"]).content }] } }), /expected/);
});

test("nothing is special-cased: .gitattributes and the checker count only because they are pinned", () => {
  const paths = [".gitattributes", "scripts/check-surface-ack.mjs"];
  // not in a pin list: an edit is not a touch and needs no ack
  for (const path of paths) assert.deepEqual([check({ changed: [mod(path)] }).ok, check({ changed: [mod(path)] }).touched], [true, []]);
  // pinned (at the base, or only at the head): an edit needs an ack that lists it
  const pinnedBase = [...BASE, ...paths.map((p) => pin(p))].sort((a, b) => Buffer.compare(Buffer.from(a.path), Buffer.from(b.path)));
  for (const path of paths) {
    const both = { basePins: pinnedBase, headPins: pinnedBase };
    const r = failsWith(check({ ...both, changed: [mod(path)] }), /exactly one ack must be added; found 0/);
    assert.deepEqual(r.touched, [path]);
    assert.equal(check({ ...both, changed: [mod(path)], acks: { added: [ack(7, [path])] } }).ok, true);
    failsWith(check({ ...both, changed: [mod(path)], acks: { added: [ack(7, ["a.txt"])] } }), /missing changed pinned path\(s\)/);
  }
});

test("a nested .gitattributes is refused, with or without a pinned change; the root one is an ordinary pin", () => {
  failsWith(check({ changed: [mod("src/.gitattributes", "A")] }), /src\/\.gitattributes: a nested \.gitattributes can change/);
  failsWith(check({ changed: [{ status: "R", path: "docs/x.txt", oldPath: "src/.gitattributes" }] }), /nested \.gitattributes/);
  failsWith(check({ changed: [mod("a.txt"), mod("src/.gitattributes", "A")], acks: { added: [ack(7, ["a.txt"])] } }), /nested \.gitattributes/);
  const root = check({ basePins: [pin(".gitattributes")], headPins: [pin(".gitattributes")], changed: [mod(".gitattributes")], acks: { added: [ack(7, [".gitattributes"])] } });
  assert.deepEqual([root.ok, root.reasons], [true, []]);
});

test("a rename or copy counts the destination, and the source only when the source is itself pinned", () => {
  const copyToUnpinned = check({ changed: [{ status: "C", oldPath: "a.txt", path: "new.txt" }] });
  assert.deepEqual(copyToUnpinned.touched, ["a.txt"], "a pinned source counts");
  const copyFromUnpinned = check({ changed: [{ status: "C", oldPath: "unpinned.txt", path: "b.txt" }] });
  assert.deepEqual(copyFromUnpinned.touched, ["b.txt"], "a pinned destination counts; an unpinned source does not");
  const neither = check({ changed: [{ status: "C", oldPath: "unpinned.txt", path: "new.txt" }] });
  assert.deepEqual([neither.ok, neither.touched], [true, []]);
});

test("an ack renamed out of the ack directory is refused with or without a pinned change", () => {
  const renamedOut = { renamedOut: [`${ACK_DIR}pr-3.txt`] };
  failsWith(check({ changed: [], acks: renamedOut }), /pr-3\.txt.*renamed out/);
  failsWith(check({ changed: [mod("a.txt")], acks: { added: [ack(7, ["a.txt"])], ...renamedOut } }), /pr-3\.txt.*renamed out/);
});

test("the ack format is strict: reviewer line, sorted unique paths, no other content", () => {
  const changed = [mod("a.txt"), mod("b.txt")];
  const run = (content) => check({ changed, acks: { added: [{ path: `${ACK_DIR}pr-7.txt`, content }] } });
  assert.equal(run("\na.txt\n\nb.txt\nreviewer: octocat\n\n").ok, true);
  failsWith(run("a.txt\nb.txt\n"), /exactly one reviewer line, found 0/);
  failsWith(run("a.txt\nb.txt\nreviewer: a\nreviewer: b\n"), /found 2/);
  failsWith(run("b.txt\na.txt\nreviewer: octocat\n"), /not sorted and unique/);
  failsWith(run("a.txt\na.txt\nb.txt\nreviewer: octocat\n"), /not sorted and unique/);
  failsWith(run("a.txt\nb.txt \nreviewer: octocat\n"), /line 2: not a repository path/);
  failsWith(run("a.txt\r\nb.txt\r\nreviewer: octocat\r\n"), /line 1: not a repository path/);
  failsWith(run("a.txt\nb.txt\nreviewer: not a login\n"), /reviewer is not a GitHub login/);
  failsWith(run("a.txt\n/abs/b.txt\nreviewer: octocat\n"), /line 2/);
  failsWith(run("a.txt\n../b.txt\nreviewer: octocat\n"), /line 2/);
  assert.deepEqual(parseAck("reviewer: dependabot[bot]\n").problems, []);
});

test("removed-pin lines must be sorted and unique, like the path lines", () => {
  const headPins = [pin("a.txt"), pin("b.txt"), pin("c/d.txt")].filter((p) => p.path === "b.txt");
  const run = (lines) => check({ headPins, acks: { added: [ack(7, [], lines)] } });
  assert.equal(run(["removed-pin: a.txt", "removed-pin: c/d.txt"]).ok, true);
  failsWith(run(["removed-pin: c/d.txt", "removed-pin: a.txt"]), /removed-pin lines are not sorted and unique/);
  failsWith(run(["removed-pin: a.txt", "removed-pin: a.txt", "removed-pin: c/d.txt"]), /removed-pin lines are not sorted and unique/);
});

test("the reviewer login is a GitHub login: 1-39 characters, no hyphen at either end, optional [bot]", () => {
  const ok = ["a", "octocat", "a-b", "a--b", "x".repeat(39), "dependabot[bot]", "a1"];
  const bad = ["", "-a", "a-", "-", "x".repeat(40), "a b", "a_b", "a[bot", "a[bot][bot]", "[bot]", "a\u00e9", "a-[bot]"];
  for (const login of ok) assert.deepEqual(parseAck(`reviewer: ${login}\n`).problems, [], login);
  for (const login of bad) assert.match(parseAck(`reviewer: ${login}\n`).problems.join(), /not a GitHub login/, JSON.stringify(login));
});

test("ordering is by UTF-8 bytes, not UTF-16 code units", () => {
  // U+E000 is one UTF-16 unit above the surrogates; U+10000 is two units starting at D800. UTF-16
  // order puts U+10000 first, UTF-8 (and Rust, and LC_ALL=C sort) puts it last.
  const lo = "\ue000.txt";
  const hi = "\u{10000}.txt";
  assert.ok(hi < lo, "the two orders really differ");
  const pins = [pin(lo), pin(hi)];
  const changed = [mod(lo), mod(hi)];
  assert.equal(check({ basePins: pins, headPins: pins, changed, acks: { added: [ack(7, [lo, hi])] } }).ok, true);
  failsWith(check({ basePins: pins, headPins: pins, changed, acks: { added: [ack(7, [hi, lo])] } }), /not sorted and unique/);
  assert.deepEqual(check({ basePins: pins, headPins: pins, changed }).touched, [lo, hi]);
  const v3 = (files) => JSON.stringify({ schema_version: 3, files });
  assert.deepEqual(parsePins(v3([{ path: lo }, { path: hi }])), [{ path: lo }, { path: hi }]);
  assert.throws(() => parsePins(v3([{ path: hi }, { path: lo }])), /sorted and unique/);
});

test("the reason cap counts characters, and a reason may not hold a control character", () => {
  const acks = { added: [ack(7, ["e.txt"])] };
  const withReason = (reason) => check({ headPins: [...BASE, pin("e.txt", reason)], acks });
  assert.equal(withReason("\u{1F600}".repeat(MAX_REASON)).ok, true, "500 astral characters are 500 characters");
  failsWith(withReason("\u{1F600}".repeat(MAX_REASON + 1)), /501 characters, over 500/);
  failsWith(withReason("bad\u0007reason"), /control character/);
  failsWith(withReason("bad\u009freason"), /control character/);
});

test("a path with DEL or another control character is refused", () => {
  assert.match(parseAck("a\u007f.txt\nreviewer: octocat\n").problems.join(), /not a repository path/);
  assert.throws(() => parsePins(JSON.stringify({ schema_version: 3, files: [{ path: "a\u007f" }] })), /malformed/);
});

test("pin lists: schema 3 only at HEAD, schema 2 paths accepted at the base, anything malformed refused", () => {
  const v3 = (files) => JSON.stringify({ schema_version: 3, files });
  assert.deepEqual(parsePins(v3([{ path: "a" }, { path: "b", reason: "why" }])), [{ path: "a" }, { path: "b", reason: "why" }]);
  const v2 = JSON.stringify({ schema_version: 2, files: [{ path: "a", sha256: "0".repeat(64) }, { path: "b", sha256: "1".repeat(64) }] });
  assert.deepEqual(parsePins(v2, { allowSchema2: true }), [{ path: "a" }, { path: "b" }]);
  assert.throws(() => parsePins(v2), /schema_version 2 is not accepted.*merge master and migrate the pin list, see docs\/release-process\.md/);
  assert.throws(() => parsePins(JSON.stringify({ schema_version: 1, files: [] })), (e) => !/merge master/.test(e.message));
  assert.throws(() => parsePins(v3([{ path: "a", sha256: "0".repeat(64) }])), /row 1 is malformed/);
  assert.throws(() => parsePins(v3([{ path: "b" }, { path: "a" }])), /sorted and unique/);
  assert.throws(() => parsePins(v3([{ path: "a" }, { path: "a" }])), /sorted and unique/);
  assert.throws(() => parsePins(v3([{ path: "/abs" }])), /malformed/);
  assert.throws(() => parsePins(v3([{ path: "a", reason: 5 }])), /malformed/);
  assert.throws(() => parsePins(JSON.stringify({ schema_version: 3, files: [], extra: 1 })), /exactly the keys/);
  assert.throws(() => parsePins("not json"), /not valid JSON/);
});

test("name-status -z parsing keeps both names of renames and refuses truncated records", () => {
  const text = ["M", "a.txt", "R100", "old.txt", "new.txt", "C75", "src", "copy", "T", "t.txt", "D", "gone.txt", ""].join("\0");
  assert.deepEqual(parseNameStatus(text), [
    { status: "M", path: "a.txt" },
    { status: "R", oldPath: "old.txt", path: "new.txt" },
    { status: "C", oldPath: "src", path: "copy" },
    { status: "T", path: "t.txt" },
    { status: "D", path: "gone.txt" },
  ]);
  assert.throws(() => parseNameStatus(["R100", "old.txt", ""].join("\0")), /truncated/);
  assert.throws(() => parseNameStatus("Q\0a\0"), /unreadable/);
});

// ---- the CLI against a real temporary git repository ----

const script = new URL("./check-surface-ack.mjs", import.meta.url).pathname;
const tmpDirs = [];
test.after(() => tmpDirs.forEach((d) => rmSync(d, { recursive: true, force: true })));

function makeRepo() {
  const dir = mkdtempSync(join(tmpdir(), "surface-ack-"));
  tmpDirs.push(dir);
  const sh = (...args) =>
    execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@example.invalid", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", ...args], {
      cwd: dir,
      encoding: "utf8",
    }).trim();
  sh("init", "-q", "-b", "main");
  const write = (rel, text) => {
    mkdirSync(dirname(join(dir, rel)), { recursive: true });
    writeFileSync(join(dir, rel), text);
  };
  const commit = (msg) => {
    sh("add", "-A");
    sh("commit", "-q", "-m", msg);
    return sh("rev-parse", "HEAD");
  };
  const surface = (rows, version = 3) => write(SURFACE_PATH, JSON.stringify({ schema_version: version, files: rows }, null, 2));
  const ackFile = (n, paths) => write(`${ACK_DIR}pr-${n}.txt`, `${paths.join("\n")}\nreviewer: octocat\n`);
  return { dir, sh, write, commit, surface, ackFile };
}

function cli(dir, args, env = {}) {
  const clean = { ...process.env };
  for (const k of ["PR_NUMBER", "PR_HEAD_SHA", "MERGE_GROUP_BASE_SHA", "GITHUB_SHA", "GITHUB_REF", "GITHUB_EVENT_PATH", "GITHUB_EVENT_NAME"]) delete clean[k];
  return spawnSync(process.execPath, [script, ...args], { cwd: dir, env: { ...clean, ...env }, encoding: "utf8" });
}

// main: M0 (a, b pinned; other unpinned); branches cut from M0; then main moves on (M1).
function prRepo({ version = 3 } = {}) {
  const r = makeRepo();
  const rows = [{ path: "a.txt" }, { path: "b.txt" }];
  r.surface(version === 2 ? rows.map((x) => ({ ...x, sha256: "0".repeat(64) })) : rows, version);
  for (const f of ["a.txt", "b.txt", "other.txt"]) r.write(f, `${f}\n`);
  r.commit("base");
  return r;
}
function branch(r, name, edit) {
  r.sh("switch", "-q", "-c", name, "main");
  edit();
  const tip = r.commit(name);
  r.sh("switch", "-q", "main");
  return tip;
}
function mergeBranch(r, name) {
  r.sh("switch", "-q", "-C", `merge-${name}`, "main");
  r.sh("merge", "--no-ff", "-q", "-m", `merge ${name}`, name);
}

test("pull_request mode: HEAD^1 is the base, HEAD^2 must be the PR head, and the ack rules apply", () => {
  const r = prRepo();
  const good = branch(r, "good", () => (r.write("a.txt", "changed\n"), r.ackFile(7, ["a.txt"])));
  const noAck = branch(r, "no-ack", () => r.write("b.txt", "changed\n"));
  const wrongAck = branch(r, "wrong-ack", () => (r.write("b.txt", "changed\n"), r.ackFile(9, ["b.txt"])));
  const clean = branch(r, "clean", () => r.write("other.txt", "changed\n"));
  r.write("master-only.txt", "master moved on\n");
  r.commit("master moves (adds no ack and touches no pin)");

  mergeBranch(r, "good");
  const ok = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "7", PR_HEAD_SHA: good });
  assert.equal(ok.status, 0, ok.stdout + ok.stderr);
  assert.match(ok.stdout, /touched pinned files \(1: a\.txt\)/);
  assert.match(ok.stdout, /surface ack check ok/);
  assert.equal(cli(r.dir, ["--mode", "pull_request", "--pr", "7"], { PR_HEAD_SHA: good }).status, 0, "--pr instead of PR_NUMBER");

  const wrongHead = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "7", PR_HEAD_SHA: noAck });
  assert.equal(wrongHead.status, 1);
  assert.match(wrongHead.stdout, /HEAD\^2 .* is not the pull request head/);
  const wrongNumber = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "8", PR_HEAD_SHA: good });
  assert.equal(wrongNumber.status, 1);
  assert.match(wrongNumber.stdout, /expected .*pr-8\.txt/);
  const noNumber = cli(r.dir, ["--mode", "pull_request"], { PR_HEAD_SHA: good });
  assert.equal(noNumber.status, 1, "an unknown PR number fails closed");

  mergeBranch(r, "no-ack");
  const missing = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "8", PR_HEAD_SHA: noAck });
  assert.equal(missing.status, 1);
  assert.match(missing.stdout, /touched pinned files \(1: b\.txt\)/);
  assert.match(missing.stdout, /surface ack check FAILED: .*exactly one ack must be added; found 0/);
  const reportOnly = cli(r.dir, ["--mode", "pull_request", "--report-only"], { PR_NUMBER: "8", PR_HEAD_SHA: noAck });
  assert.equal(reportOnly.status, 0);
  assert.match(reportOnly.stdout, /WOULD FAIL: .*exactly one ack/);
  assert.match(reportOnly.stdout, /^::warning title=Surface acknowledgement \(report-only\)::.*exactly one ack/m);

  mergeBranch(r, "wrong-ack");
  const named = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "8", PR_HEAD_SHA: wrongAck });
  assert.equal(named.status, 1);
  assert.match(named.stdout, /expected .*pr-8\.txt/);

  mergeBranch(r, "clean");
  const none = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "9", PR_HEAD_SHA: clean });
  assert.equal(none.status, 0, none.stdout);
  assert.match(none.stdout, /touched pinned files \(none\)/);

  r.sh("switch", "-q", "--detach", "main");
  const notMerge = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "9" });
  assert.equal(notMerge.status, 1, "a non-merge HEAD has no HEAD^2");
  assert.equal(cli(r.dir, ["--mode", "pull_request", "--report-only"], { PR_NUMBER: "9" }).status, 0);
  const explicit = cli(r.dir, ["--mode", "pull_request", "--pr", "9", "--base", "HEAD~1"]);
  assert.equal(explicit.status, 0, "--base checks base..HEAD without the HEAD^2 assertion");
  assert.match(explicit.stdout, /touched pinned files \(none\)/);
});

test("pull_request mode: a schema 2 base and schema 3 head can be checked (the cut-over)", () => {
  const r = prRepo({ version: 2 });
  const tip = branch(r, "cutover", () => {
    r.surface([{ path: "a.txt" }, { path: "b.txt" }, { path: "n.txt", reason: "new pin" }]);
    r.write("n.txt", "n\n");
    r.write("a.txt", "changed\n");
    r.ackFile(21, ["a.txt", "n.txt"]);
  });
  mergeBranch(r, "cutover");
  const ok = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "21", PR_HEAD_SHA: tip });
  assert.equal(ok.status, 0, ok.stdout + ok.stderr);
  assert.match(ok.stdout, /2: a\.txt, n\.txt/);

  const r2 = prRepo();
  const bad = branch(r2, "still-v2", () => r2.surface([{ path: "a.txt", sha256: "0".repeat(64) }], 2));
  mergeBranch(r2, "still-v2");
  const refused = cli(r2.dir, ["--mode", "pull_request"], { PR_NUMBER: "22", PR_HEAD_SHA: bad });
  assert.equal(refused.status, 1);
  assert.match(refused.stdout, /schema_version 2 is not accepted/);
});

test("pull_request mode: an unset PR_HEAD_SHA fails closed unless --base is given", () => {
  const r = prRepo();
  const tip = branch(r, "good", () => (r.write("a.txt", "changed\n"), r.ackFile(7, ["a.txt"])));
  mergeBranch(r, "good");
  const unset = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "7" });
  assert.equal(unset.status, 1, unset.stdout);
  assert.match(unset.stdout, /PR_HEAD_SHA is not set/);
  const blank = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "7", PR_HEAD_SHA: "  " });
  assert.equal(blank.status, 1);
  assert.equal(cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "7", PR_HEAD_SHA: tip.toUpperCase() }).status, 0, "case is not significant");
  assert.equal(cli(r.dir, ["--mode", "pull_request", "--report-only"], { PR_NUMBER: "7" }).status, 0, "report-only still exits 0");
  assert.equal(cli(r.dir, ["--mode", "pull_request", "--base", "HEAD^1"], { PR_NUMBER: "7" }).status, 0, "an explicit --base needs no PR_HEAD_SHA");
});

test("pull_request mode: an edit to .gitattributes needs an ack only because it is pinned", () => {
  const r = makeRepo();
  r.surface([{ path: ".gitattributes", reason: "eol rules move every digest" }, { path: "a.txt" }]);
  for (const f of ["a.txt", "other.txt"]) r.write(f, `${f}\n`);
  r.write(".gitattributes", "* text=auto\n");
  r.commit("base");
  const bare = branch(r, "attr-no-ack", () => r.write(".gitattributes", "* text eol=lf\n"));
  const acked = branch(r, "attr-acked", () => (r.write(".gitattributes", "* text eol=lf\n"), r.ackFile(7, [".gitattributes"])));
  const wrong = branch(r, "attr-wrong-ack", () => (r.write(".gitattributes", "* text eol=lf\n"), r.ackFile(7, ["a.txt"])));
  mergeBranch(r, "attr-no-ack");
  const missing = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "7", PR_HEAD_SHA: bare });
  assert.equal(missing.status, 1);
  assert.match(missing.stdout, /touched pinned files \(1: \.gitattributes\)/);
  mergeBranch(r, "attr-acked");
  assert.equal(cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "7", PR_HEAD_SHA: acked }).status, 0);
  mergeBranch(r, "attr-wrong-ack");
  const wrongAck = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "7", PR_HEAD_SHA: wrong });
  assert.equal(wrongAck.status, 1);
  assert.match(wrongAck.stdout, /missing changed pinned path\(s\): \.gitattributes/);

  // the same edit with the file NOT pinned is an ordinary change: nothing is special-cased
  const u = makeRepo();
  u.surface([{ path: "a.txt" }]);
  for (const f of ["a.txt"]) u.write(f, `${f}\n`);
  u.write(".gitattributes", "* text=auto\n");
  u.commit("base");
  const plain = branch(u, "attr-unpinned", () => u.write(".gitattributes", "* text eol=lf\n"));
  mergeBranch(u, "attr-unpinned");
  const fine = cli(u.dir, ["--mode", "pull_request"], { PR_NUMBER: "7", PR_HEAD_SHA: plain });
  assert.equal(fine.status, 0, fine.stdout);
  assert.match(fine.stdout, /touched pinned files \(none\)/);
});

test("pull_request mode: renaming an ack out of the ack directory is refused, pinned change or not", () => {
  const r = prRepo();
  r.ackFile(3, ["a.txt"]);
  r.commit("an old ack");
  const moveOut = () => r.sh("mv", `${ACK_DIR}pr-3.txt`, "moved-ack.txt");
  const plain = branch(r, "move-out", moveOut);
  const withPin = branch(r, "move-out-and-edit", () => (moveOut(), r.write("b.txt", "changed\n"), r.ackFile(8, ["b.txt"])));
  const deleteOnly = branch(r, "cleanup", () => r.sh("rm", "-q", `${ACK_DIR}pr-3.txt`));
  mergeBranch(r, "move-out");
  const empty = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "9", PR_HEAD_SHA: plain });
  assert.equal(empty.status, 1, empty.stdout);
  assert.match(empty.stdout, /pr-3\.txt: an existing ack was renamed out of the ack directory/);
  mergeBranch(r, "move-out-and-edit");
  const pinned = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "8", PR_HEAD_SHA: withPin });
  assert.equal(pinned.status, 1, pinned.stdout);
  assert.match(pinned.stdout, /renamed out of the ack directory/);
  mergeBranch(r, "cleanup");
  const cleanup = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "9", PR_HEAD_SHA: deleteOnly });
  assert.equal(cleanup.status, 0, "a plain deletion is the exempt cleanup");
});

test("pull_request mode: a copy of a pinned file is read as a copy, and the pinned source is a touch", () => {
  const r = makeRepo();
  r.surface([{ path: "a.txt" }, { path: "b.txt" }]);
  const body = Array.from({ length: 30 }, (_, i) => `line ${i} of a file long enough for copy detection`).join("\n");
  for (const f of ["a.txt", "b.txt"]) r.write(f, `${body}\n`);
  r.commit("base");
  const tip = branch(r, "copy", () => {
    r.write("a.txt", `${body}\nan edit\n`);
    r.write("copy-of-a.txt", `${body}\nan edit\n`);
    r.ackFile(7, ["a.txt"]);
  });
  mergeBranch(r, "copy");
  const res = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "7", PR_HEAD_SHA: tip });
  assert.equal(res.status, 0, res.stdout);
  assert.match(res.stdout, /touched pinned files \(1: a\.txt\)/);
  const names = execFileSync("git", ["diff", "--name-status", "--find-renames", "--find-copies", "HEAD^1", "HEAD"], { cwd: r.dir, encoding: "utf8" });
  assert.match(names, /^C\d+\ta\.txt\tcopy-of-a\.txt$/m, "git really reported a copy, so the C path was exercised");
});

// A queue-shaped range: two squash commits on top of the base, each its own pull request.
function queueRepo({ firstSubject = "Change a (#11)", secondAck = true, firstAck = true } = {}) {
  const r = prRepo();
  const base = r.sh("rev-parse", "HEAD");
  r.write("a.txt", "changed\n");
  if (firstAck) r.ackFile(11, ["a.txt"]);
  const first = r.commit(firstSubject);
  r.write("b.txt", "changed\n");
  if (secondAck) r.ackFile(12, ["b.txt"]);
  const second = r.commit("Change b (#12)");
  const env = { MERGE_GROUP_BASE_SHA: base, GITHUB_SHA: second, GITHUB_REF: `refs/heads/gh-readonly-queue/main/pr-12-${second}` };
  return { r, first, second, env };
}

test("merge_group mode checks each first-parent commit against its own ack", () => {
  const { r, env } = queueRepo();
  const ok = cli(r.dir, ["--mode", "merge_group"], env);
  assert.equal(ok.status, 0, ok.stdout + ok.stderr);
  assert.equal((ok.stdout.match(/touched pinned files \(1: /g) ?? []).length, 2, "one summary line per commit");
});

test("merge_group mode fails a commit whose own ack is missing, even if a sibling's ack would cover it", () => {
  const { r, env } = queueRepo({ secondAck: false });
  const res = cli(r.dir, ["--mode", "merge_group"], env);
  assert.equal(res.status, 1);
  assert.match(res.stdout, /exactly one ack must be added; found 0/);
  const report = cli(r.dir, ["--mode", "merge_group", "--report-only"], env);
  assert.equal(report.status, 0);
  assert.match(report.stdout, /WOULD FAIL/);
  const first = queueRepo({ firstAck: false });
  assert.equal(cli(first.r.dir, ["--mode", "merge_group"], first.env).status, 1);
});

test("merge_group mode fails closed when a commit cannot be attributed to exactly one pull request", () => {
  const noNumber = queueRepo({ firstSubject: "Change a with no number" });
  const res = cli(noNumber.r.dir, ["--mode", "merge_group"], noNumber.env);
  assert.equal(res.status, 1);
  assert.match(res.stdout, /cannot attribute the commit to exactly one pull request/);

  const q = queueRepo();
  const noRef = cli(q.r.dir, ["--mode", "merge_group"], { ...q.env, GITHUB_REF: "refs/heads/main" });
  assert.equal(noRef.status, 1, "the last entry needs the queue ref");
  const disagree = cli(q.r.dir, ["--mode", "merge_group"], { ...q.env, GITHUB_REF: `refs/heads/gh-readonly-queue/main/pr-99-${q.second}` });
  assert.equal(disagree.status, 1, "queue ref and subject naming different pull requests");
  const empty = cli(q.r.dir, ["--mode", "merge_group"], { ...q.env, MERGE_GROUP_BASE_SHA: q.second });
  assert.equal(empty.status, 1, "an empty range verifies nothing");
  const noBase = cli(q.r.dir, ["--mode", "merge_group"], { GITHUB_SHA: q.second, GITHUB_REF: q.env.GITHUB_REF });
  assert.equal(noBase.status, 1);
});

test("workflow_dispatch only validates that the acks directory is well formed", () => {
  const r = prRepo();
  r.write("a.txt", "changed\n");
  r.commit("pinned change with no ack at all");
  assert.equal(cli(r.dir, ["--mode", "workflow_dispatch"]).status, 0);
  assert.equal(cli(r.dir, ["--mode", "push"], { GITHUB_EVENT_NAME: "workflow_dispatch" }).status, 0, "a push-mode run started by workflow_dispatch is not a landed pull request");
  r.write(`${ACK_DIR}pr-8.txt`, `b.txt\nreviewer: octocat\n# ${"ab".repeat(32)}\n`);
  r.commit("hex ack");
  const hex = cli(r.dir, ["--mode", "workflow_dispatch"]);
  assert.equal(hex.status, 1);
  assert.match(hex.stdout, /pr-8\.txt: contains a 64-hex token/);
  assert.equal(cli(r.dir, ["--mode", "workflow_dispatch", "--report-only"]).status, 0);
  r.write(`${ACK_DIR}pr-8.txt`, "b.txt\nreviewer: octocat\n");
  r.write(`${ACK_DIR}my-branch.txt`, "b.txt\nreviewer: octocat\n");
  r.commit("badly named ack");
  assert.match(cli(r.dir, ["--mode", "workflow_dispatch"]).stdout, /my-branch\.txt: not named pr-<N>\.txt/);
  assert.equal(cli(makeRepo().dir, ["--mode", "workflow_dispatch"]).status, 1, "a directory that is not a git repository or has no commits fails closed");
});

test("push mode checks every first-parent commit the push landed, like a pull request, and is never report-only", () => {
  const r = prRepo();
  const tip = () => r.sh("rev-parse", "HEAD");
  // `before` is the tip before the commits under test; it comes from --base or the event payload.
  const push = (before, extra = [], env = {}) => cli(r.dir, ["--mode", "push", "--base", before, ...extra], env);
  let before = tip();
  // A pinned change with its ack, named by the squash subject.
  r.write("a.txt", "changed\n");
  r.ackFile(7, ["a.txt"]);
  r.commit("Change a (#7)");
  const ok = push(before);
  assert.equal(ok.status, 0, ok.stdout);
  assert.match(ok.stdout, /\[push [0-9a-f]{12}\] touched pinned files \(1: a\.txt\)/);
  // The base race: another pull request pinned b.txt after this one was gated, and this one
  // edited b.txt with no ack. Every check at gate time was green; the master run must be red.
  before = tip();
  r.write("b.txt", "changed\n");
  r.commit("Change b (#9)");
  const race = push(before);
  assert.equal(race.status, 1);
  assert.match(race.stdout, /exactly one ack must be added; found 0/);
  assert.equal(push(before, ["--report-only"]).status, 1, "report-only does not soften a landed commit");
  assert.doesNotMatch(push(before, ["--report-only"]).stdout, /::warning/);
  // Several commits in one push: the unacknowledged one is not the tip, and is still found.
  r.write("b.txt", "again\n");
  r.ackFile(10, ["b.txt"]);
  r.commit("Change b with its ack (#10)");
  const several = push(before);
  assert.equal(several.status, 1, "an unacknowledged commit below the tip is not hidden by an acknowledged tip");
  assert.match(several.stdout, /exactly one ack must be added; found 0/);
  // A commit that cannot be attributed to a pull request, with a pinned change, fails closed.
  before = tip();
  r.write("b.txt", "third\n");
  r.commit("direct push with no number");
  assert.match(push(before).stdout, /pull request number is not known/);
  // An unpinned change needs neither an ack nor a number.
  before = tip();
  r.write("other.txt", "changed\n");
  r.commit("unpinned change");
  assert.equal(push(before).status, 0);
  // A landed commit with a stray ack is red too.
  before = tip();
  r.write(`${ACK_DIR}pr-12.txt`, "b.txt\nreviewer: octocat\n");
  r.commit("stray ack (#12)");
  assert.match(push(before).stdout, /ack was added but no pinned path changed/);
  // `before` comes from the event payload when --base is not given.
  const event = join(tmpdir(), `surface-ack-event-${process.pid}.json`);
  tmpDirs.push(event);
  writeFileSync(event, JSON.stringify({ before }));
  assert.match(cli(r.dir, ["--mode", "push"], { GITHUB_EVENT_PATH: event }).stdout, /ack was added but no pinned path changed/);
  // A missing, unreadable, new-branch (all zeros) or non-ancestor `before` cannot be verified.
  for (const bad of [undefined, "0".repeat(40), "f".repeat(40), "not-a-sha"]) {
    writeFileSync(event, JSON.stringify(bad === undefined ? {} : { before: bad }));
    const refused = cli(r.dir, ["--mode", "push"], { GITHUB_EVENT_PATH: event });
    assert.equal(refused.status, 1, `before ${bad}`);
    assert.match(refused.stdout, /no usable "before"|not an ancestor/);
  }
  assert.equal(cli(r.dir, ["--mode", "push"]).status, 1, "no payload and no --base");
  // A force-push: `before` is a commit that is not an ancestor of HEAD.
  r.sh("switch", "-q", "-c", "rewritten", "HEAD~2");
  r.write("other.txt", "rewritten\n");
  r.commit("rewritten (#13)");
  const forced = push(before);
  assert.equal(forced.status, 1);
  assert.match(forced.stdout, /not an ancestor of HEAD/);
  // A repository with one commit has no parent: fails closed, as does a directory with no commits.
  const one = makeRepo();
  one.surface([{ path: "a.txt" }]);
  one.write("a.txt", "a\n");
  const first = one.commit("only (#1)");
  assert.equal(cli(one.dir, ["--mode", "push", "--base", first]).status, 1);
  assert.equal(cli(makeRepo().dir, ["--mode", "push"]).status, 1);
});

test("bad usage exits 2", () => {
  const dir = tmpdir();
  for (const args of [[], ["--mode", "nope"], ["--mode"], ["--mode", "push", "--surprise"]]) {
    assert.equal(cli(dir, args).status, 2, JSON.stringify(args));
  }
});

// ---- the branch history check: a pin the branch added and then lost ----

const OWN_REASON = "decides what is posted";
const WITHDRAWN = /pinned by a commit of this pull request but not in the pin list at the head; restore the pin, or declare the withdrawal with a removed-pin line/;

// main: M0 pins a and b. The branch adds its own pin n, changes a, and acknowledges both. Main moves
// on. The branch then merges main resolving the pin list by taking main's, which drops n, and `fix`
// edits the working tree for that merge commit (the acknowledgement, usually). The pull request is
// then merged into main, so HEAD^2 is the branch tip.
function ownPinLost({ fix = () => {}, unionList = false } = {}) {
  const r = prRepo();
  r.sh("switch", "-q", "-c", "own", "main");
  r.write("n.txt", "n\n");
  r.write("a.txt", "changed\n");
  r.surface([{ path: "a.txt" }, { path: "b.txt" }, { path: "n.txt", reason: OWN_REASON }]);
  r.ackFile(7, ["a.txt", "n.txt"]);
  r.commit("own pin");
  r.sh("switch", "-q", "main");
  r.write("master-only.txt", "master moved on\n");
  r.commit("master moves on");
  r.sh("switch", "-q", "own");
  r.sh("merge", "--no-commit", "--no-ff", "main");
  if (!unionList) r.sh("checkout", "main", "--", SURFACE_PATH);
  fix(r);
  r.commit("merge main");
  const tip = r.sh("rev-parse", "HEAD");
  r.sh("switch", "-q", "main");
  mergeBranch(r, "own");
  return { r, tip };
}
const run = ({ r, tip }, n = "7") => cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: n, PR_HEAD_SHA: tip });

test("history check: a pin the branch added and lost fails while the acknowledgement still names it", () => {
  const out = run(ownPinLost());
  assert.equal(out.status, 1, out.stdout);
  assert.match(out.stdout, /touched pinned files \(2: a\.txt, n\.txt\)/);
  assert.match(out.stdout, /lists path\(s\) that are not changed pinned paths: n\.txt/);
});

test("history check: a lost pin fails even when the acknowledgement is edited to match", () => {
  const out = run(ownPinLost({ fix: (r) => r.ackFile(7, ["a.txt"]) }));
  assert.equal(out.status, 1, out.stdout);
  assert.match(out.stdout, WITHDRAWN);
  assert.match(out.stdout, /n\.txt: pinned by a commit of this pull request/);
});

test("history check: a lost pin fails when the acknowledgement is deleted, and nothing else is pinned", () => {
  const out = run(
    ownPinLost({
      fix: (r) => {
        r.sh("rm", "-q", "-f", `${ACK_DIR}pr-7.txt`);
        r.sh("checkout", "main", "--", "a.txt");
      },
    }),
  );
  assert.equal(out.status, 1, out.stdout);
  assert.match(out.stdout, /touched pinned files \(1: n\.txt\)/);
  assert.match(out.stdout, /exactly one ack must be added; found 0/);
});

test("history check: a withdrawn pin passes when the acknowledgement declares it with a removed-pin line", () => {
  const out = run(ownPinLost({ fix: (r) => r.write(`${ACK_DIR}pr-7.txt`, "a.txt\nreviewer: octocat\nremoved-pin: n.txt\n") }));
  assert.equal(out.status, 0, out.stdout);
  assert.match(out.stdout, /touched pinned files \(2: a\.txt, n\.txt\)/);
  assert.match(out.stdout, /surface ack check ok/);
});

test("history check: a union resolution that keeps the pin passes", () => {
  const out = run(ownPinLost({ unionList: true }));
  assert.equal(out.status, 0, out.stdout);
  assert.match(out.stdout, /surface ack check ok/);
});

test("history check: a removed-pin line for a pin that was never withdrawn is still refused", () => {
  const out = run(ownPinLost({ unionList: true, fix: (r) => r.write(`${ACK_DIR}pr-7.txt`, "a.txt\nn.txt\nreviewer: octocat\nremoved-pin: zzz.txt\n") }));
  assert.equal(out.status, 1, out.stdout);
  assert.match(out.stdout, /"removed-pin:" names path\(s\) that were not removed: zzz\.txt/);
});

test("history check: a shallow clone fails closed", () => {
  const { r, tip } = ownPinLost({ unionList: true });
  const clone = mkdtempSync(join(tmpdir(), "surface-ack-shallow-"));
  tmpDirs.push(clone);
  execFileSync("git", ["clone", "-q", "--depth", "2", `file://${r.dir}`, clone, "--branch", "merge-own"], { encoding: "utf8" });
  const out = cli(clone, ["--mode", "pull_request"], { PR_NUMBER: "7", PR_HEAD_SHA: tip });
  assert.equal(out.status, 1, out.stdout);
  assert.match(out.stdout, /shallow clone/);
});

test("history check: a branch commit whose pin list cannot be parsed fails closed", () => {
  const r = prRepo();
  r.sh("switch", "-q", "-c", "broken", "main");
  r.write(SURFACE_PATH, "{ not json");
  r.commit("break the list");
  r.surface([{ path: "a.txt" }, { path: "b.txt" }]);
  r.write("other.txt", "changed\n");
  const tip = r.commit("repair it");
  r.sh("switch", "-q", "main");
  mergeBranch(r, "broken");
  const out = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "7", PR_HEAD_SHA: tip });
  assert.equal(out.status, 1, out.stdout);
  assert.match(out.stdout, /cannot be parsed/);
});

test("history check, pure: a merge is compared with all of its parents, not the first only", () => {
  const set = (...paths) => new Set(paths);
  const lists = new Map([
    ["p1", { blob: "b1", paths: set("a") }],
    ["p2", { blob: "b2", paths: set("a", "z") }],
    ["m", { blob: "b3", paths: set("a", "z") }],
    ["n", { blob: "b4", paths: set("a") }],
  ]);
  const pinsOf = (sha) => lists.get(sha);
  const commits = [
    { sha: "m", parents: ["p1", "p2"] },
    { sha: "n", parents: ["m"] },
  ];
  // z reached the branch through the second parent; the first-parent view would call it the branch's own.
  assert.deepEqual(withdrawnPins({ commits, pinsOf, basePaths: set("a"), headPaths: set("a") }), []);
  // The same path added by a commit whose every parent lacks it is the branch's own, and is withdrawn.
  lists.set("c", { blob: "b5", paths: set("a", "z") });
  lists.set("p0", { blob: "b6", paths: set("a") });
  assert.deepEqual(withdrawnPins({ commits: [{ sha: "c", parents: ["p0"] }, ...commits.slice(1)], pinsOf, basePaths: set("a"), headPaths: set("a") }), ["z"]);
  // A pin the base or the head holds is never withdrawn.
  assert.deepEqual(withdrawnPins({ commits: [{ sha: "c", parents: ["p0"] }], pinsOf, basePaths: set("a", "z"), headPaths: set("a") }), []);
  assert.deepEqual(withdrawnPins({ commits: [{ sha: "c", parents: ["p0"] }], pinsOf, basePaths: set("a"), headPaths: set("a", "z") }), []);
  // A commit whose list is the same blob as a parent's adds nothing, and an unreadable list throws.
  assert.deepEqual(withdrawnPins({ commits: [{ sha: "n", parents: ["n"] }], pinsOf, basePaths: set(), headPaths: set() }), []);
  assert.throws(() => withdrawnPins({ commits: [{ sha: "x", parents: [] }], pinsOf: () => { throw new Error("unreadable"); }, basePaths: set(), headPaths: set() }), /unreadable/);
});

// ---- historic pin lists are read for their paths only ----

// main: M0 pins a and b. The branch's first commit pins a new file n with the list out of order
// (a, n, b); its second commit sorts it, or drops n. The strict rules apply to the list at the head
// and at the base; a list in an older commit of the branch is read for its paths only.
function outOfOrderThen({ keep, ackText = "n.txt\nreviewer: octocat\n" }) {
  const r = prRepo();
  r.sh("switch", "-q", "-c", "own", "main");
  r.write("n.txt", "n\n");
  r.surface([{ path: "a.txt" }, { path: "n.txt", reason: OWN_REASON }, { path: "b.txt" }]);
  r.commit("pin n, list out of order");
  r.surface(keep ? [{ path: "a.txt" }, { path: "b.txt" }, { path: "n.txt", reason: OWN_REASON }] : [{ path: "a.txt" }, { path: "b.txt" }]);
  r.write(`${ACK_DIR}pr-7.txt`, ackText);
  const tip = r.commit(keep ? "sort the list" : "drop the pin");
  r.sh("switch", "-q", "main");
  mergeBranch(r, "own");
  return { r, tip };
}

test("history check: an out-of-order list in an older branch commit passes once the head list is sorted", () => {
  const out = run(outOfOrderThen({ keep: true }));
  assert.equal(out.status, 0, out.stdout);
  assert.match(out.stdout, /surface ack check ok/);
});

test("history check: the same out-of-order history fails when the pin is then dropped, and passes when declared", () => {
  const out = run(outOfOrderThen({ keep: false }));
  assert.equal(out.status, 1, out.stdout);
  assert.doesNotMatch(out.stdout, /cannot be parsed/);
  assert.match(out.stdout, /touched pinned files \(1: n\.txt\)/);
  const declared = run(outOfOrderThen({ keep: false, ackText: "reviewer: octocat\nremoved-pin: n.txt\n" }));
  assert.equal(declared.status, 0, declared.stdout);
});

test("history check: a historic list that is not readable at all still fails closed", () => {
  const r = prRepo();
  r.sh("switch", "-q", "-c", "own", "main");
  r.write(SURFACE_PATH, JSON.stringify({ schema_version: 3, files: [{ path: 3 }] }));
  r.commit("a list whose rows are not paths");
  r.surface([{ path: "a.txt" }, { path: "b.txt" }]);
  r.write("other.txt", "changed\n");
  const tip = r.commit("repair it");
  r.sh("switch", "-q", "main");
  mergeBranch(r, "own");
  const out = cli(r.dir, ["--mode", "pull_request"], { PR_NUMBER: "7", PR_HEAD_SHA: tip });
  assert.equal(out.status, 1, out.stdout);
  assert.match(out.stdout, /cannot be parsed/);
});

test("historicPaths reads paths only: unsorted, duplicated and extra-key rows are fine, unreadable lists are not", () => {
  const rows = (...r) => JSON.stringify({ files: r });
  assert.deepEqual([...historicPaths(rows({ path: "b" }, { path: "a", extra: 1 }, { path: "a" }))].sort(), ["a", "b"]);
  assert.deepEqual([...historicPaths(JSON.stringify({ schema_version: 2, files: [{ path: "a", sha256: "0".repeat(64) }] }))], ["a"]);
  for (const bad of ["{ not json", "[]", "{}", JSON.stringify({ files: "x" }), rows({ path: 3 }), rows({}), rows(null), rows({ path: "" })]) {
    assert.throws(() => historicPaths(bad), /pin list/, bad);
  }
});
