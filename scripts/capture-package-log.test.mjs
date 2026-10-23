// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, statSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { Readable } from "node:stream";
import test from "node:test";
import { fileURLToPath } from "node:url";

import { capture, PREFIX_BYTES, TAIL_BYTES } from "./capture-package-log.mjs";

const script = fileURLToPath(new URL("./capture-package-log.mjs", import.meta.url));

function directory(t) {
  const root = mkdtempSync(join(tmpdir(), "package-log-"));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  return root;
}

function chunked(data, size = 8192) {
  const parts = [];
  for (let start = 0; start < data.length; start += size) parts.push(data.subarray(start, start + size));
  return Readable.from(parts);
}

test("retains a small child error without modification", async (t) => {
  const root = directory(t);
  const data = Buffer.from("creating disk image\nchild stderr: diagnostic control\n");
  assert.deepEqual(await capture(chunked(data), root), data);
  assert.deepEqual(readFileSync(join(root, "build.log")), data);
  assert.equal(JSON.parse(readFileSync(join(root, "capture.json"), "utf8")).prefix_truncated, false);
});

test("large output is drained and the final error retained", async (t) => {
  const root = directory(t);
  const data = Buffer.concat([Buffer.alloc(PREFIX_BYTES * 2, "x"), Buffer.from("\nfinal error\n")]);
  let read = 0;
  const source = chunked(data).map((chunk) => ((read += chunk.length), chunk));
  const tail = await capture(source, root);
  assert.equal(read, data.length);
  assert.equal(statSync(join(root, "build.log")).size, PREFIX_BYTES);
  assert.equal(tail.length, TAIL_BYTES);
  assert.ok(tail.toString().endsWith("final error\n"));
  const counts = JSON.parse(readFileSync(join(root, "capture.json"), "utf8"));
  assert.deepEqual(counts, { total_bytes: data.length, prefix_bytes: PREFIX_BYTES, tail_bytes: TAIL_BYTES, prefix_truncated: true });
});

test("pipefail preserves success and a child's failure", (t) => {
  for (const code of [0, 37]) {
    const root = directory(t);
    const result = spawnSync(
      "bash",
      ["-o", "pipefail", "-c", `"$1" -e "console.error('child stderr control'); process.exit($3)" 2>&1 | "$1" "$2"`, "package-control", process.execPath, script, String(code)],
      { cwd: root },
    );
    assert.equal(result.status, code);
    assert.ok(result.stdout.includes("child stderr control"));
    assert.ok(readFileSync(join(root, "package-diagnostics/build.log")).includes("child stderr control"));
  }
});
