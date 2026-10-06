// SPDX-License-Identifier: Apache-2.0
// A failing inventory command must say why: the exit status and the tail of its stderr, bounded. The
// Rust inventory runs `cargo tree`; a stand-in `cargo` on PATH fails with known stderr lines.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
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

// A network failure of `cargo tree` is retried (twice); any other failure is not. The stand-in counts its
// calls in a file beside it, so the number of attempts is observed, not inferred from the message.
const countingCargo = (body) => {
  const bin = mkdtempSync(join(tmpdir(), "inventory-retry-"));
  const counter = join(bin, "calls");
  writeFileSync(join(bin, "cargo"), `#!/bin/sh\necho x >> "${counter}"\nn=$(wc -l < "${counter}")\n${body}\n`);
  chmodSync(join(bin, "cargo"), 0o755);
  const calls = () => readFileSync(counter, "utf8").split("\n").filter(Boolean).length;
  return { bin, calls };
};
const runInventory = (bin) => spawnSync(process.execPath, [script, "--rust"], {
  encoding: "utf8",
  env: { ...process.env, PATH: `${bin}:${process.env.PATH}`, INVENTORY_RETRY_DELAY_MS: "0" },
});
const NETWORK = 'echo "error: failed to get lock_api as a dependency" >&2; echo "  download of lo/ck/lock_api failed" >&2; echo "  [16] Error in the HTTP2 framing layer" >&2; exit 101';

test("a network failure of cargo is retried and a later success is used", { skip }, () => {
  const { bin, calls } = countingCargo(`if [ "$n" -lt 3 ]; then ${NETWORK}; fi\nexit 0`);
  try {
    const result = runInventory(bin);
    assert.equal(calls(), 5, "the first command succeeded on its third attempt, then the two other targets ran once each");
    assert.ok(result.stderr.includes("(attempt 1 of 3: download of lo/ck/lock_api failed); retrying"));
    assert.ok(result.stderr.includes("(attempt 2 of 3: download of lo/ck/lock_api failed); retrying"));
    assert.ok(!result.stderr.includes("inventory command failed"), "the retried command did not fail the run");
  } finally {
    rmSync(bin, { recursive: true, force: true });
  }
});

test("a network failure that persists fails after three attempts", { skip }, () => {
  const { bin, calls } = countingCargo(NETWORK);
  try {
    const result = runInventory(bin);
    assert.equal(calls(), 3);
    assert.match(result.stderr, /Rust x86_64-pc-windows-msvc inventory command failed \(exit status 101\)/);
  } finally {
    rmSync(bin, { recursive: true, force: true });
  }
});

test("a failure that is not a network failure is not retried", { skip }, () => {
  const { bin, calls } = countingCargo('echo "error: could not find Cargo.toml in the manifest path" >&2; exit 101');
  try {
    const result = runInventory(bin);
    assert.equal(calls(), 1);
    assert.match(result.stderr, /inventory command failed \(exit status 101\)/);
    assert.ok(!result.stderr.includes("retrying"));
  } finally {
    rmSync(bin, { recursive: true, force: true });
  }
});
