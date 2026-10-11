// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { findMarkers, markerLines } from "./check-conflict-markers.mjs";

const script = fileURLToPath(new URL("./check-conflict-markers.mjs", import.meta.url));
function repo(files) {
  const dir = mkdtempSync(join(tmpdir(), "markers-"));
  const git = (...args) => spawnSync("git", ["-c", "user.name=t", "-c", "user.email=t@example.invalid", ...args], { cwd: dir, encoding: "utf8" });
  git("init", "-q");
  for (const [name, content] of Object.entries(files)) {
    mkdirSync(join(dir, name, ".."), { recursive: true });
    writeFileSync(join(dir, name), content);
  }
  git("add", "-A");
  return dir;
}

test("every marker form on its own line is found, with its line number", () => {
  assert.deepEqual(markerLines("a\n<<<<<<< ours\nb\n||||||| a99c49393\nc\n=======\nd\n>>>>>>> theirs\n"), [2, 4, 6, 8]);
  assert.deepEqual(markerLines("<<<<<<<\n>>>>>>>\n"), [1, 2], "a marker with no label");
  assert.deepEqual(markerLines("<<<<<<< x\r\n=======\r\n"), [1, 2], "CRLF files");
});

test("text that merely contains marker characters is not a marker", () => {
  assert.deepEqual(markerLines("a ======= b\n========\n<<<<<<<x\n x\n  <<<<<<< indented\nlabel: >>>>>>> mid\n|||||||x\n"), []);
});

test("it reads committed files and names each file and line; a clean tree and binary files pass", () => {
  const dir = repo({ "clean.txt": "ok\n", "docs/notes.md": "a\n<<<<<<< ours\nb\n=======\nc\n>>>>>>> theirs\n", "bin.dat": "\0\n<<<<<<< x\n" });
  try {
    assert.deepEqual(findMarkers(dir), ["docs/notes.md:2", "docs/notes.md:4", "docs/notes.md:6"]);
    const run = spawnSync(process.execPath, [script, "--root", dir], { encoding: "utf8" });
    assert.equal(run.status, 1);
    assert.match(run.stderr, /docs\/notes\.md:2/);
    const clean = repo({ "a.txt": "fine\n" });
    try {
      const ok = spawnSync(process.execPath, [script, "--root", clean], { encoding: "utf8" });
      assert.equal(ok.status, 0, ok.stderr);
    } finally { rmSync(clean, { recursive: true, force: true }); }
  } finally { rmSync(dir, { recursive: true, force: true }); }
});

test("a directory that is not a repository is exit 2, never 'none found'", () => {
  const dir = mkdtempSync(join(tmpdir(), "markers-none-"));
  try {
    const run = spawnSync(process.execPath, [script, "--root", dir], { encoding: "utf8" });
    assert.equal(run.status, 2);
  } finally { rmSync(dir, { recursive: true, force: true }); }
});

// Skipped where `chmod 000` does not stop a read: as root, and on Windows (#1471), as the byte-integrity test does.
const unreadableSkip = process.getuid?.() === 0 ? "root can read any file" : process.platform === "win32" ? "chmod does not stop a read on Windows" : false;
test("a tracked file that cannot be read is exit 2, not a clean result", { skip: unreadableSkip }, () => {
  const dir = repo({ "locked.txt": "<<<<<<< x\n", "fine.txt": "ok\n" });
  try {
    chmodSync(join(dir, "locked.txt"), 0o000);
    const run = spawnSync(process.execPath, [script, "--root", dir], { encoding: "utf8" });
    assert.equal(run.status, 2, run.stdout + run.stderr);
  } finally { chmodSync(join(dir, "locked.txt"), 0o644); rmSync(dir, { recursive: true, force: true }); }
});

test("a repository in the middle of a conflicted merge reports each marker once", () => {
  const dir = repo({ "a.txt": "line\n" });
  const git = (...args) => spawnSync("git", ["-c", "user.name=t", "-c", "user.email=t@example.invalid", "-c", "commit.gpgsign=false", ...args], { cwd: dir, encoding: "utf8" });
  try {
    git("commit", "-q", "-m", "base");
    git("switch", "-q", "-c", "other");
    writeFileSync(join(dir, "a.txt"), "theirs\n"); git("commit", "-qam", "theirs");
    git("switch", "-q", "-");
    writeFileSync(join(dir, "a.txt"), "ours\n"); git("commit", "-qam", "ours");
    git("merge", "other"); // conflicts: git lists a.txt three times (stages 1, 2, 3)
    assert.match(git("ls-files", "-u").stdout, /a\.txt/);
    const found = findMarkers(dir); // three lines, or four when the user config writes diff3 base markers
    assert.ok(found.length >= 3 && found[0] === "a.txt:1", found.join());
    assert.deepEqual(found, [...new Set(found)], "each marker is reported once");
  } finally { rmSync(dir, { recursive: true, force: true }); }
});
