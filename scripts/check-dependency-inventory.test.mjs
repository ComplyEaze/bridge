// SPDX-License-Identifier: Apache-2.0
// A failing inventory command must say why: the exit status and the tail of its stderr, bounded. The
// Rust inventory runs `cargo tree`; a stand-in `cargo` on PATH fails with known stderr lines.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

const script = fileURLToPath(new URL("./check-dependency-inventory.mjs", import.meta.url));
const skip = process.platform === "win32" && "the stand-in cargo is a POSIX shell script";

test("a failing cargo is reported with its exit status and the tail of its stderr, bounded", { skip }, () => {
  const bin = mkdtempSync(join(tmpdir(), "inventory-cargo-"));
  try {
    writeFileSync(join(bin, "cargo"), '#!/bin/sh\nfor i in $(seq 1 60); do echo "stand-in cargo noise $i" >&2; done\necho "error: failed to download from the registry" >&2\nexit 7\n');
    chmodSync(join(bin, "cargo"), 0o755);
    const result = spawnSync(process.execPath, [script, "--rust"], {
      encoding: "utf8",
      env: { ...process.env, PATH: `${bin}:${process.env.PATH}` },
    });
    assert.notEqual(result.status, 0);
    const message = result.stderr;
    assert.match(message, /Rust x86_64-pc-windows-msvc inventory command failed \(exit status 7\)/);
    assert.ok(message.includes("error: failed to download from the registry"), "the last stderr line is kept");
    assert.ok(message.includes("stand-in cargo noise 60"), "the tail is kept");
    assert.ok(!message.includes("stand-in cargo noise 1\n"), "the head is dropped: the tail is bounded");
  } finally {
    rmSync(bin, { recursive: true, force: true });
  }
});
