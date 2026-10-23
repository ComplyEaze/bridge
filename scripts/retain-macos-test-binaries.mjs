// SPDX-License-Identifier: Apache-2.0
// Retain only test executables whose image identity matches fresh minimized IPS.
// No build is invoked. UUID/architecture linkage identifies the executable image,
// not the individual nextest test or missing source line tables.
//
// Usage: node scripts/retain-macos-test-binaries.mjs --crash-json FILE --target DIR --output DIR
// Node built-ins only: the native CI job runs this without actions/setup-node (Node 20 or newer).
import { spawnSync } from "node:child_process";
import { createHash, randomUUID } from "node:crypto";
import { chmodSync, closeSync, constants, existsSync, fstatSync, mkdirSync, openSync, readSync, realpathSync, renameSync, rmSync, writeFileSync, writeSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parseArgs } from "node:util";

export const MAX_BYTES = 256 * 1024 * 1024;
export const MAX_REPORTS = 8;
const PROCESS = /^bridge_lib(?:-[0-9a-f]+)?$/;
const UUID = /^[0-9a-fA-F]{8}(?:-[0-9a-fA-F]{4}){3}-[0-9a-fA-F]{12}$/;
const IDENTITY = /^UUID: ([0-9a-fA-F]{8}(?:-[0-9a-fA-F]{4}){3}-[0-9a-fA-F]{12}) \((arm64|arm64e|x86_64|x86_64h|i386)\) /;
const ARCHES = ["arm64", "arm64e", "x86_64", "x86_64h", "i386"];
const DEBUG_INFO = "line_tables_unavailable_ci_profile_debug_0";
const USAGE = "usage: node scripts/retain-macos-test-binaries.mjs --crash-json FILE --target DIR --output DIR\n";

export class EvidenceError extends Error {}

// Run `command` with stdout and stderr captured: at most 16 KiB between them and `timeout` seconds.
// A missing tool, a non-zero exit, undecodable output or either bound exceeded is an EvidenceError.
export function boundedTool(command, timeout = 10) {
  const [file, ...args] = command;
  const run = spawnSync(file, args, { stdio: ["inherit", "pipe", "pipe"], maxBuffer: 16 * 1024, timeout: Math.ceil(timeout * 1000), killSignal: "SIGKILL" });
  if (run.error?.code === "ENOBUFS") throw new EvidenceError("image_tool_output_limit");
  if (run.error?.code === "ETIMEDOUT") throw new EvidenceError("image_tool_timeout");
  if (run.error || run.status !== 0) throw new EvidenceError("image_tool_failed");
  try {
    return new TextDecoder("utf-8", { fatal: true }).decode(Buffer.concat([run.stdout, run.stderr]));
  } catch {
    throw new EvidenceError("image_tool_failed");
  }
}

export function binaryIdentities(path) {
  const identities = [];
  for (const line of boundedTool(["dwarfdump", "--uuid", path]).split(/\r?\n/)) {
    const match = IDENTITY.exec(line);
    if (match) identities.push([match[1].toLowerCase(), match[2]]);
  }
  if (!identities.length || identities.length > 8) throw new EvidenceError("image_identity_unavailable");
  return identities;
}

// sha256 and size of a file read from the start; `sink` also receives the bytes. Over `limit` is `overLimit`.
function digest(descriptor, limit, overLimit, sink) {
  const hash = createHash("sha256");
  const buffer = Buffer.allocUnsafe(1024 * 1024);
  let size = 0;
  for (let count; (count = readSync(descriptor, buffer, 0, buffer.length, size)) > 0; size += count) {
    if (size + count > limit) throw new EvidenceError(overLimit);
    hash.update(buffer.subarray(0, count));
    if (sink !== undefined) for (let done = 0; done < count; ) done += writeSync(sink, buffer, done, count - done);
  }
  return { sha256: hash.digest("hex"), size };
}

const matches = (identities, [uuid, arch]) => identities.filter((identity) => identity[0] === uuid && identity[1] === arch).length === 1;

/** `identities` replaces binaryIdentities (tests). Returns the manifest entry for the retained copy. */
export function retain(source, output, expected, budget, identities = binaryIdentities) {
  let temporary = null;
  try {
    // Open the exact report basename under the caller's known target root.
    // No glob, symlink traversal, post-test cargo command or rebuild.
    const original = openSync(source, constants.O_RDONLY | constants.O_NOFOLLOW | constants.O_NONBLOCK);
    let first;
    let second;
    let initial;
    try {
      initial = fstatSync(original);
      if (!initial.isFile()) throw new EvidenceError("not_regular_file");
      if (initial.size <= 0 || initial.size > budget) throw new EvidenceError("binary_byte_limit");
      if (!matches(identities(source), expected)) throw new EvidenceError("image_identity_mismatch");
      temporary = join(output, `.tmp-${randomUUID()}`);
      const staged = openSync(temporary, "wx", 0o600);
      try {
        first = digest(original, budget, "binary_byte_limit", staged);
      } finally {
        closeSync(staged);
      }
      second = digest(original, initial.size, "binary_changed_during_capture");
    } finally {
      closeSync(original);
    }
    if (first.size !== initial.size || first.sha256 !== second.sha256) throw new EvidenceError("binary_changed_during_capture");
    const reread = openSync(temporary, "r");
    try {
      if (digest(reread, initial.size, "binary_changed_during_capture").sha256 !== second.sha256) throw new EvidenceError("staged_digest_mismatch");
    } finally {
      closeSync(reread);
    }
    if (!matches(identities(temporary), expected)) throw new EvidenceError("staged_image_identity_mismatch");
    const file = `${expected[0]}-${expected[1]}-${basename(source)}`;
    chmodSync(temporary, 0o700);
    renameSync(temporary, join(output, file));
    temporary = null;
    return { file, sha256: second.sha256, bytes: first.size, uuid: expected[0], arch: expected[1] };
  } catch (error) {
    if (error.syscall) throw new EvidenceError("binary_read_or_copy_failed", { cause: error });
    throw error;
  } finally {
    if (temporary !== null) rmSync(temporary, { force: true });
  }
}

/** `maxBytes` and `identities` replace MAX_BYTES and binaryIdentities (tests). */
export function collect(snapshot, target, output, { maxBytes = MAX_BYTES, identities } = {}) {
  const reports = snapshot?.reports ?? [];
  if (snapshot === null || typeof snapshot !== "object" || Array.isArray(snapshot) || !Array.isArray(reports)) throw new EvidenceError("invalid_reports");
  const result = {
    status: "unavailable",
    binaries: [],
    errors: [],
    report_count: reports.length,
    reports_truncated: reports.length > MAX_REPORTS,
    debug_info: DEBUG_INFO,
  };
  if (snapshot.status === "capture_error" || snapshot.truncated) result.errors.push("crash_capture_incomplete");
  if (reports.length > MAX_REPORTS) result.errors.push("report_limit");
  const seen = new Set();
  let remaining = maxBytes;
  for (const report of reports.slice(0, MAX_REPORTS)) {
    try {
      const { process: proc, process_images: images } = report ?? {};
      if (typeof proc !== "string" || !PROCESS.test(proc)) throw new EvidenceError("invalid_process_name");
      if (!Array.isArray(images) || images.length !== 1 || report.process_images_truncated) throw new EvidenceError("process_image_ambiguous_or_missing");
      const { uuid, arch } = images[0] ?? {};
      if (typeof uuid !== "string" || !UUID.test(uuid) || !ARCHES.includes(arch)) throw new EvidenceError("process_image_identity_unavailable");
      const expected = [uuid.toLowerCase(), arch];
      if (seen.has(expected.join())) continue;
      const binary = { ...retain(join(target, proc), output, expected, remaining, identities), process: proc };
      result.binaries.push(binary);
      remaining -= binary.bytes;
      seen.add(expected.join());
    } catch (error) {
      if (!(error instanceof EvidenceError)) throw error;
      result.errors.push(error.message);
    }
  }
  if (result.errors.length) result.status = result.binaries.length ? "partial" : "error";
  else if (result.binaries.length) result.status = "linked";
  return result;
}

export function provenance(environ = process.env) {
  const git = spawnSync("git", ["rev-parse", "HEAD"], { encoding: "utf8", timeout: 10_000 });
  if (git.error || git.status !== 0) throw new EvidenceError("provenance_failed");
  const result = { checkout_sha: /^[0-9a-f]{40}$/.test(git.stdout.trim()) ? git.stdout.trim() : null };
  for (const name of ["GITHUB_SHA", "PR_HEAD_SHA", "GITHUB_RUN_ID", "GITHUB_RUN_ATTEMPT", "GITHUB_JOB"]) {
    result[name.toLowerCase()] = /^[A-Za-z0-9_-]{1,100}$/.test(environ[name] ?? "") ? environ[name] : null;
  }
  return result;
}

// At most limit + 1 bytes, so an over-limit file is detectable without reading all of it.
function readCapped(path, limit) {
  const descriptor = openSync(path, "r");
  try {
    const buffer = Buffer.alloc(limit + 1);
    let size = 0;
    for (let count = 1; count > 0 && size < buffer.length; size += count) count = readSync(descriptor, buffer, size, buffer.length - size, null);
    return buffer.subarray(0, size);
  } finally {
    closeSync(descriptor);
  }
}

// The parsed options, or a string describing why the command line is refused.
export function parseCli(argv) {
  let values;
  try {
    ({ values } = parseArgs({ args: argv, options: { "crash-json": { type: "string" }, target: { type: "string" }, output: { type: "string" } } }));
  } catch (error) {
    return error.message;
  }
  if (!values["crash-json"] || !values.target || !values.output) return "--crash-json, --target and --output are required";
  return { crashJson: values["crash-json"], target: values.target, output: values.output };
}

/** Returns the exit status: 2 for a refused command line, 1 when the manifest lists any error. `env` lets tests inject streams, provenance and identities. */
export function main(argv, env = {}) {
  const args = parseCli(argv);
  if (typeof args === "string") {
    (env.stderr ?? process.stderr).write(`${args}\n${USAGE}`);
    return 2;
  }
  mkdirSync(dirname(resolve(args.output)), { recursive: true });
  mkdirSync(args.output); // Refuses an existing output so one run never mixes with another's evidence.
  let result;
  try {
    const data = readCapped(args.crashJson, 16 * 1024 * 1024);
    if (data.length > 16 * 1024 * 1024) throw new EvidenceError("crash_json_byte_limit");
    result = collect(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(data)), resolve(args.target), args.output, { identities: env.identities });
  } catch (error) {
    // Unreadable or unparsable input is an error manifest, never a crash with no evidence at all.
    result = { status: "error", binaries: [], errors: [error instanceof EvidenceError ? error.message : "input_or_provenance_failed"] };
  }
  result.debug_info ??= DEBUG_INFO;
  try {
    result.provenance = (env.provenance ?? provenance)();
  } catch {
    result.errors.push("provenance_failed");
    result.status = "error";
  }
  writeFileSync(join(args.output, "manifest.json"), `${JSON.stringify(result, null, 2)}\n`);
  (env.stdout ?? process.stdout).write(`${JSON.stringify({ status: result.status, binaries: result.binaries.length, errors: result.errors })}\n`);
  return result.errors.length ? 1 : 0;
}

if (process.argv[1] && existsSync(process.argv[1]) && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url))) {
  process.exitCode = main(process.argv.slice(2));
}
