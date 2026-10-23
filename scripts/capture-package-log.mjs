#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0
// Drain package output with bounded retention; the caller must use pipefail.
//
//   corepack pnpm run tauri:build -- --timings 2>&1 | node scripts/capture-package-log.mjs
//
// Keeps the first 4 MiB in package-diagnostics/build.log and the last 64 KiB in build-tail.log,
// writes the byte counts to capture.json, and prints the tail, so a long build's final error is
// both shown and retained however much it printed.

import { closeSync, existsSync, mkdirSync, openSync, realpathSync, writeFileSync, writeSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

export const PREFIX_BYTES = 4 * 1024 * 1024;
export const TAIL_BYTES = 64 * 1024;

/** Drains `chunks` (an async iterable of Buffers) into `directory`; returns the retained tail. */
export async function capture(chunks, directory) {
  mkdirSync(directory, { recursive: true });
  let total = 0;
  let kept = 0;
  let tail = Buffer.alloc(0);
  const log = openSync(join(directory, "build.log"), "w");
  try {
    for await (const chunk of chunks) {
      total += chunk.length;
      const prefix = chunk.subarray(0, Math.max(0, PREFIX_BYTES - kept));
      writeSync(log, prefix);
      kept += prefix.length;
      tail = Buffer.concat([tail, chunk]);
      if (tail.length > TAIL_BYTES) tail = tail.subarray(tail.length - TAIL_BYTES);
    }
  } finally {
    closeSync(log);
  }
  writeFileSync(join(directory, "build-tail.log"), tail);
  const counts = { total_bytes: total, prefix_bytes: kept, tail_bytes: tail.length, prefix_truncated: total > kept };
  writeFileSync(join(directory, "capture.json"), `${JSON.stringify(counts, null, 2)}\n`);
  return tail;
}

if (process.argv[1] && existsSync(process.argv[1]) && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url))) {
  if (process.argv.length > 2) {
    process.stderr.write("usage: corepack pnpm run tauri:build 2>&1 | node scripts/capture-package-log.mjs (no arguments)\n");
    process.exit(2);
  }
  const tail = await capture(process.stdin, "package-diagnostics");
  process.stdout.write("Package output tail (at most 64 KiB); retained prefix and byte counts are in the diagnostic artifact.\n");
  process.stdout.write(tail);
}
