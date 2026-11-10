// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { checkVersionReleaseDate } from "./check-version-release-date.mjs";

const check = (baseText, headText, today = "2026-11-12") => checkVersionReleaseDate({ baseText, headText, today });

test("a version pull request that moved the date passes, with or without a trailing newline", () => {
  assert.equal(check("2026-10-30\n", "2026-11-10\n"), "2026-11-10");
  assert.equal(check("2026-10-30\n", "2026-11-10"), "2026-11-10");
});

test("a pull request that dropped the file is refused with a line naming it", () => {
  assert.throws(() => check("2026-10-30\n", null), /src-tauri\/release-date\.txt is missing/);
});

test("the date carried over from the previous release is refused on a later day, and allowed on that day", () => {
  assert.throws(() => check("2026-10-30\n", "2026-10-30\n", "2026-11-12"), /still names 2026-10-30, the day of the previous release, and today is 2026-11-12/);
  // a second release on the same UTC day writes the same date
  assert.equal(check("2026-11-12\n", "2026-11-12\n", "2026-11-12"), "2026-11-12");
});

test("a date before the base's, after today, or not a real day is refused", () => {
  assert.throws(() => check("2026-10-30\n", "2026-10-29\n"), /earlier than the last release tag's day/);
  assert.throws(() => check("2026-10-30\n", "2026-11-13\n"), /later than today/);
  for (const bad of ["", "\n", "2026-02-30\n", "2026-1-5\n", "soon\n", "2026-11-10\n2026-11-11\n", " 2026-11-10\n"]) {
    assert.throws(() => check("2026-10-30\n", bad), /not a YYYY-MM-DD day/, JSON.stringify(bad));
  }
});

test("a base without a usable date gives nothing to compare with, and the head is still checked", () => {
  assert.equal(check(null, "2026-11-10\n"), "2026-11-10");
  assert.equal(check("garbage\n", "2026-11-10\n"), "2026-11-10");
  assert.throws(() => check(null, "soon\n"), /not a YYYY-MM-DD day/);
});

// The command line, on a throwaway repository: an unreadable revision must fail, not read as "no file".
test("the command line reads the two revisions and fails on a revision it cannot read", () => {
  const script = fileURLToPath(new URL("./check-version-release-date.mjs", import.meta.url));
  const dir = mkdtempSync(join(tmpdir(), "release-date-"));
  try {
    const git = (...args) => execFileSync("git", args, { cwd: dir, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] }).trim();
    git("init", "-q");
    git("config", "user.email", "test@example.invalid");
    git("config", "user.name", "test");
    mkdirSync(join(dir, "src-tauri"));
    writeFileSync(join(dir, "src-tauri", "release-date.txt"), "2026-10-30\n");
    git("add", "-A");
    git("commit", "-q", "-m", "base");
    const base = git("rev-parse", "HEAD");
    writeFileSync(join(dir, "src-tauri", "release-date.txt"), "2026-10-31\n");
    git("commit", "-q", "-a", "-m", "moved");
    const moved = git("rev-parse", "HEAD");
    git("rm", "-q", "src-tauri/release-date.txt");
    git("commit", "-q", "-m", "dropped");
    const dropped = git("rev-parse", "HEAD");
    const run = (...revs) => spawnSync(process.execPath, [script, ...revs], { cwd: dir, encoding: "utf8", env: { ...process.env } });
    const ok = run(base, moved);
    assert.equal(ok.status, 0, ok.stderr);
    assert.match(ok.stdout, /src-tauri\/release-date\.txt: 2026-10-31/);
    const missing = run(base, dropped);
    assert.equal(missing.status, 1);
    assert.match(missing.stderr, /::error file=src-tauri\/release-date\.txt::.*is missing/);
    const unreadable = run(base, "no-such-revision");
    assert.equal(unreadable.status, 1);
    assert.doesNotMatch(unreadable.stderr, /is missing/, "an unreadable revision was reported as an absent file");
    assert.equal(run(base).status, 2);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});
