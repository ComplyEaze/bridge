// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";
import { checkReleaseDateChange, parseReleaseDate, ReleaseDateRefusal, RELEASE_DATE_FILE } from "./check-release-date-changed.mjs";

// The unit cases drive the decision on the file's text; the repository cases run the real script against real commits,
// so the way it reads the file from git (an absent file is told apart from a git failure) is exercised too.
const script = join(dirname(fileURLToPath(import.meta.url)), "check-release-date-changed.mjs");

function git(cwd, ...args) {
  return execFileSync("git", ["-c", "user.name=t", "-c", "user.email=dev@example.invalid", "-c", "commit.gpgsign=false", "-c", "core.autocrlf=false", ...args], { cwd, encoding: "utf8" }).trim();
}

// sides: { base, head } are the file's bytes at each commit, or null for no file there.
function run(sides, argsOverride) {
  const dir = mkdtempSync(join(tmpdir(), "release-date-"));
  try {
    git(dir, "init", "-q");
    const commit = (text, message) => {
      mkdirSync(join(dir, "src-tauri"), { recursive: true });
      if (text === null) rmSync(join(dir, RELEASE_DATE_FILE), { force: true });
      else writeFileSync(join(dir, RELEASE_DATE_FILE), text);
      writeFileSync(join(dir, "marker.txt"), message);
      git(dir, "add", "-A");
      git(dir, "commit", "-q", "-m", message);
      return git(dir, "rev-parse", "HEAD");
    };
    const base = commit(sides.base, "base");
    const head = commit(sides.head, "head");
    const result = spawnSync(process.execPath, [script, ...(argsOverride ? argsOverride(base, head) : [base, head])], { cwd: dir, encoding: "utf8" });
    return { status: result.status, out: result.stdout, err: result.stderr };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

test("positive control: a later real day passes and says both days", () => {
  const r = run({ base: "2026-10-30\n", head: "2026-11-09\n" });
  assert.equal(r.status, 0, r.err);
  assert.match(r.out, /release-date: ok, 2026-10-30 -> 2026-11-09/);
});

test("the first release that has the file passes (no file on the base)", () => {
  const r = run({ base: null, head: "2026-11-09\n" });
  assert.equal(r.status, 0, r.err);
  assert.match(r.out, /no base date -> 2026-11-09/);
});

test("a base that is not a real day is no predecessor, only the head is checked", () => {
  assert.equal(run({ base: "garbage\n", head: "2026-11-09\n" }).status, 0);
});

test("the same day as the base is refused as unchanged, naming the file", () => {
  const r = run({ base: "2026-10-30\n", head: "2026-10-30\n" });
  assert.equal(r.status, 1);
  assert.match(r.err, /::error file=src-tauri\/release-date\.txt::release-date: unchanged:/);
});

test("an earlier day than the base's is refused", () => {
  const r = run({ base: "2026-10-30\n", head: "2026-10-29\n" });
  assert.equal(r.status, 1);
  assert.match(r.err, /release-date: earlier:/);
});

test("a version pull request that deletes the file is refused as missing", () => {
  const r = run({ base: "2026-10-30\n", head: null });
  assert.equal(r.status, 1);
  assert.match(r.err, /release-date: missing:/);
});

test("a head with no file and no base file is also missing, not a pass", () => {
  const r = run({ base: null, head: null });
  assert.equal(r.status, 1);
  assert.match(r.err, /release-date: missing:/);
});

test("anything but exactly one real YYYY-MM-DD line with one newline is malformed", () => {
  const bad = ["2026-11-09", "2026-11-09\r\n", "2026-11-09\n\n", " 2026-11-09\n", "2026-11-09\n2026-11-10\n", "2026-13-40\n", "2026-02-30\n", "26-11-09\n", "", "\n"];
  for (const head of bad) {
    assert.throws(() => checkReleaseDateChange({ base: "2026-10-30\n", head }), (error) => error instanceof ReleaseDateRefusal && error.code === "malformed", JSON.stringify(head));
  }
  // and once through git, so the bytes read from a commit are the bytes judged
  const r = run({ base: "2026-10-30\n", head: "2026-11-09\r\n" });
  assert.equal(r.status, 1);
  assert.match(r.err, /release-date: malformed:/);
});

test("a commit git cannot find is exit 2 and says it is no verdict on the file, never a pass or a refusal", () => {
  const r = run({ base: "2026-10-30\n", head: "2026-11-09\n" }, (base) => [base, "0123456789abcdef0123456789abcdef01234567"]);
  assert.equal(r.status, 2);
  assert.match(r.err, /could not read src-tauri\/release-date\.txt from git/);
  assert.doesNotMatch(r.out + r.err, /release-date: ok|release-date: (unchanged|earlier|missing|malformed)/);
});

test("arguments that are not two full commit ids are exit 2", () => {
  for (const args of [() => [], (b) => [b], (b, h) => [b, h, h], () => ["main", "HEAD"], (b) => [b.slice(0, 8), b.slice(0, 8)]]) {
    const r = run({ base: "2026-10-30\n", head: "2026-11-09\n" }, args);
    assert.equal(r.status, 2);
    assert.match(r.err, /usage:/);
  }
});

test("the decision is typed: each refusal carries its code", () => {
  const codeOf = (sides) => { try { checkReleaseDateChange(sides); return "ok"; } catch (error) { assert.ok(error instanceof ReleaseDateRefusal); return error.code; } };
  assert.equal(codeOf({ base: "2026-10-30\n", head: null }), "missing");
  assert.equal(codeOf({ base: "2026-10-30\n", head: "x\n" }), "malformed");
  assert.equal(codeOf({ base: "2026-10-30\n", head: "2026-10-30\n" }), "unchanged");
  assert.equal(codeOf({ base: "2026-10-30\n", head: "2026-01-01\n" }), "earlier");
  assert.equal(codeOf({ base: "2026-10-30\n", head: "2026-10-31\n" }), "ok");
  assert.equal(codeOf({ base: "2025-12-31\n", head: "2026-01-01\n" }), "ok", "a year boundary");
});

test("parseReleaseDate accepts a leap day and refuses a day that does not exist", () => {
  assert.equal(parseReleaseDate("2028-02-29\n"), "2028-02-29");
  assert.equal(parseReleaseDate("2026-02-29\n"), null);
  assert.equal(parseReleaseDate(null), null);
});
