// SPDX-License-Identifier: Apache-2.0
// Retain bounded stack evidence from fresh Bridge test crash reports, not raw IPS.
// Format: https://developer.apple.com/documentation/xcode/interpreting-the-json-format-of-a-crash-report
//
// Usage: node scripts/collect-macos-test-crashes.mjs --since-file FILE --output FILE [--wait-seconds 0..30]
// Node built-ins only: the native CI job runs this without actions/setup-node (Node 20 or newer).
// Untrusted report JSON is read only through dict()/list()/shape(), which throw on an unexpected shape, so a
// malformed report is one "report_read_or_parse_failed" entry and never stops the collector.
import assert from "node:assert/strict";
import { closeSync, constants, existsSync, lstatSync, openSync, readdirSync, readSync, realpathSync, statSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { join, posix } from "node:path";
import { fileURLToPath } from "node:url";
import { parseArgs } from "node:util";

export const MAX_REPORTS = 8;
export const MAX_BYTES = 8 * 1024 * 1024;
export const MAX_OUTPUT_BYTES = 8 * 1024 * 1024; // Below the executable retainer's 16 MiB input bound.
const PROCESS = /^bridge_lib(?:-[0-9a-f]+)?$/;
const UUID = /^[0-9a-fA-F]{8}(?:-[0-9a-fA-F]{4}){3}-[0-9a-fA-F]{12}$/;
const ARCHES = new Set(["arm64", "arm64e", "x86_64", "x86_64h", "i386"]);
const USAGE = "usage: node scripts/collect-macos-test-crashes.mjs --since-file FILE --output FILE [--wait-seconds 0..30]\n";

const shape = (ok) => assert(ok, "unexpected report shape");
const dict = (value) => (shape(value !== null && typeof value === "object" && !Array.isArray(value)), value);
const list = (value = []) => (shape(Array.isArray(value)), value);
export const label = (value) => (typeof value !== "string" || /[/\\\u0000-\u001f]/.test(value) ? null : [...value.slice(0, 1024)].slice(0, 512).join(""));
const offset = (value) => (Number.isInteger(value) && value >= 0 ? value : null);

export function minimize(text) {
  const split = text.indexOf("\n");
  shape(split >= 0 && String(dict(JSON.parse(text.slice(0, split))).bug_type) === "309");
  const report = dict(JSON.parse(text.slice(split + 1)));
  const proc = report.procName;
  shape(typeof proc === "string" && PROCESS.test(proc));
  const images = list(report.usedImages).map(dict);
  const threads = list(report.threads).map(dict);
  const baseName = (image) => {
    const name = image.name || image.path || "unknown";
    shape(typeof name === "string");
    return posix.basename(name);
  };
  const identity = (image) => ({
    uuid: typeof image.uuid === "string" && UUID.test(image.uuid) ? image.uuid.toLowerCase() : null,
    arch: ARCHES.has(image.arch) ? image.arch : null,
  });
  const own = images.flatMap((image, image_index) => (baseName(image) === proc ? [{ image_index, ...identity(image) }] : []));
  const exception = report.exception === undefined ? {} : dict(report.exception);
  const faulting = offset(report.faultingThread);
  const all = threads.map((_, index) => index);
  // The faulting and triggered threads come first so the 64-thread cap cannot drop them.
  const chosen = [...new Set([faulting, ...all.filter((index) => threads[index].triggered === true), ...all])]
    .filter((index) => Number.isInteger(index) && index >= 0 && index < threads.length)
    .slice(0, 64);
  return {
    process: proc,
    process_images: own.slice(0, 2),
    process_images_truncated: own.length > 2,
    exception: { type: label(exception.type), signal: label(exception.signal) },
    faulting_thread: faulting,
    threads_truncated: threads.length > 64,
    threads: chosen.map((index) => {
      const frames = list(threads[index].frames);
      return {
        index,
        triggered: threads[index].triggered === true,
        frames_truncated: frames.length > 64,
        frames: frames.slice(0, 64).map((frame) => {
          const at = dict(frame).imageIndex;
          const image = Number.isInteger(at) && at >= 0 && at < images.length ? images[at] : null;
          return {
            image: label(baseName(image ?? {})),
            image_index: image ? at : null,
            ...identity(image ?? {}),
            symbol: label(frame.symbol),
            image_offset: offset(frame.imageOffset),
            symbol_offset: offset(frame.symbolLocation),
          };
        }),
      };
    }),
  };
}

// At most limit + 1 bytes, so an over-limit file is detectable without reading all of it.
function readCapped(path, limit) {
  // Non-blocking and no-follow: a FIFO or a link swapped in after the scan cannot stall or redirect the read.
  const descriptor = openSync(path, constants.O_RDONLY | constants.O_NONBLOCK | constants.O_NOFOLLOW);
  try {
    const buffer = Buffer.alloc(limit + 1);
    let size = 0;
    for (let count = 1; count > 0 && size < buffer.length; size += count) count = readSync(descriptor, buffer, size, buffer.length - size, null);
    return buffer.subarray(0, size);
  } finally {
    closeSync(descriptor);
  }
}

/** `since` is a cutoff mtime in milliseconds. */
export function collect(directories, since) {
  const result = { status: "no_fresh_reports", reports: [], errors: [], truncated: false };
  const candidates = [];
  for (const directory of directories) {
    let names;
    try {
      names = readdirSync(directory);
    } catch (error) {
      // As the Python glob did, a directory that is missing or cannot be listed is skipped: an
      // unreadable diagnostics folder must not fail a native run whose tests passed.
      if (!["ENOENT", "ENOTDIR", "EACCES", "EPERM"].includes(error.code)) result.errors.push("directory_read_failed");
      continue;
    }
    for (const name of names.filter((entry) => /^bridge_lib.*\.ips$/.test(entry))) {
      const path = join(directory, name);
      try {
        const info = lstatSync(path);
        if (info.isSymbolicLink()) result.errors.push("symlink_report");
        else if (!info.isFile()) result.errors.push("report_read_or_parse_failed");
        else if (info.mtimeMs >= since) candidates.push(path);
      } catch (error) {
        // A report removed between the listing and its stat is gone, not unread; the next one is still read.
        if (error.code !== "ENOENT") result.errors.push("directory_read_failed");
      }
    }
  }
  result.truncated = candidates.length > MAX_REPORTS;
  for (const path of candidates.sort().slice(0, MAX_REPORTS)) {
    try {
      const data = readCapped(path, MAX_BYTES);
      if (data.length > MAX_BYTES) result.errors.push("report_too_large");
      else result.reports.push(minimize(new TextDecoder("utf-8", { fatal: true }).decode(data)));
    } catch {
      result.errors.push("report_read_or_parse_failed");
    }
  }
  if (result.errors.length || result.truncated) result.status = "capture_error";
  else if (result.reports.length) result.status = "captured";
  return result;
}

/** `clock.now` (seconds, monotonic) and `clock.sleep` (seconds) are injectable for tests. */
export function waitForReports(directories, since, waitSeconds, clock = {}) {
  const now = clock.now ?? (() => performance.now() / 1000);
  const sleep = clock.sleep ?? ((seconds) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, seconds * 1000));
  const deadline = now() + waitSeconds;
  let lastError = null;
  for (;;) {
    // Refresh the bounded snapshot through the grace period: later IPS files
    // can arrive after the first complete report. Replacing avoids duplicates.
    const result = collect(directories, since);
    if (result.status === "capture_error") lastError = result;
    if (now() >= deadline) return result.status === "captured" ? result : lastError ?? result;
    sleep(1);
  }
}

export function encodeResult(result) {
  // Keep every admitted process-image identity. Thin deep frame tails evenly
  // across reports rather than losing all executable linkage to an input cap.
  if (result.errors.length > 64) {
    result.errors_omitted = result.errors.length - 64;
    result.errors = result.errors.slice(0, 64);
    Object.assign(result, { status: "capture_error", truncated: true });
  }
  let encoded = Buffer.from(`${JSON.stringify(result, null, 2)}\n`);
  while (encoded.length > MAX_OUTPUT_BYTES) {
    let removed = 0;
    for (const report of result.reports) {
      for (const thread of report.threads) {
        const cut = Math.ceil(thread.frames.length / 2);
        if (!cut) continue;
        removed += cut;
        report.frames_removed_for_output_limit = (report.frames_removed_for_output_limit ?? 0) + cut;
        thread.frames = thread.frames.slice(0, thread.frames.length - cut);
        thread.frames_truncated = true;
      }
    }
    if (!removed) throw new Error("crash_metadata_exceeds_output_limit");
    Object.assign(result, { status: "capture_error", truncated: true, details_truncated: true });
    encoded = Buffer.from(`${JSON.stringify(result, null, 2)}\n`);
  }
  return encoded;
}

// The parsed options, or a string describing why the command line is refused.
export function parseCli(argv) {
  let values;
  try {
    ({ values } = parseArgs({ args: argv, options: { "since-file": { type: "string" }, output: { type: "string" }, "wait-seconds": { type: "string" } } }));
  } catch (error) {
    return error.message;
  }
  const wait = values["wait-seconds"] ?? "0";
  if (!values["since-file"] || !values.output) return "--since-file and --output are required";
  if (!/^\d{1,2}$/.test(wait) || Number(wait) > 30) return "--wait-seconds must be an integer from 0 to 30";
  return { sinceFile: values["since-file"], output: values.output, waitSeconds: Number(wait) };
}

/** Returns the exit status: 2 for a refused command line, 1 on capture_error. `env` lets tests inject directories, streams and clock. */
export function main(argv, env = {}) {
  const args = parseCli(argv);
  if (typeof args === "string") {
    (env.stderr ?? process.stderr).write(`${args}\n${USAGE}`);
    return 2;
  }
  const directories = env.directories ?? [join(homedir(), "Library/Logs/DiagnosticReports"), "/Library/Logs/DiagnosticReports"];
  const result = waitForReports(directories, statSync(args.sinceFile).mtimeMs, args.waitSeconds, env.clock);
  writeFileSync(args.output, encodeResult(result));
  (env.stdout ?? process.stdout).write(`${JSON.stringify({ status: result.status, reports: result.reports.length, errors: result.errors })}\n`);
  return result.status === "capture_error" ? 1 : 0;
}

if (process.argv[1] && existsSync(process.argv[1]) && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url))) {
  process.exitCode = main(process.argv.slice(2));
}
