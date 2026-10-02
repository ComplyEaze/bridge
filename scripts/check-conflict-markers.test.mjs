// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
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
