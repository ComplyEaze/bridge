// SPDX-License-Identifier: Apache-2.0
// node --test scripts/retain-macos-test-binaries.test.mjs
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, symlinkSync, unlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { encodeResult, MAX_BYTES as CAPTURE_MAX_BYTES, MAX_OUTPUT_BYTES, minimize } from "./collect-macos-test-crashes.mjs";
import { boundedTool, collect as retainCollect, EvidenceError, main, parseCli } from "./retain-macos-test-binaries.mjs";

const FIXTURE = join(dirname(fileURLToPath(import.meta.url)), "testdata/macos-sigtrap-control.ips");
// Adversarial identity metadata and opaque executable bytes, not live evidence.
const UUID = "00000000-0000-0000-0000-000000000001";
const EXPECTED = [UUID, "arm64"];

// A fresh target/output pair with one synthetic bridge_lib-cafe executable, removed with the test.
function setup(t) {
  const root = mkdtempSync(join(tmpdir(), "retain-binaries-"));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  const [target, output] = [join(root, "target"), join(root, "output")];
  mkdirSync(target);
  mkdirSync(output);
  const binary = join(target, "bridge_lib-cafe");
  writeFileSync(binary, "synthetic executable bytes");
  const report = { process: "bridge_lib-cafe", process_images: [{ uuid: UUID, arch: "arm64", image_index: 0 }] };
  const collect = (reports = [report], options = {}) => retainCollect({ status: "captured", reports }, target, output, options);
  return { root, target, output, binary, report, collect };
}

// A stand-in for binaryIdentities: the next answer, or the next error, of a list.
const answers = (...values) => () => {
  const value = values.length > 1 ? values.shift() : values[0];
  if (value instanceof Error) throw value;
  return value;
};

test("exact identity copy, digest and deduplication", (t) => {
  const { root, output, binary, report, collect } = setup(t);
  const result = collect([report, report], { identities: answers([EXPECTED]) });
  assert.equal(result.status, "linked");
  assert.equal(result.binaries.length, 1);
  assert.equal(result.binaries[0].sha256, createHash("sha256").update(readFileSync(binary)).digest("hex"));
  assert.deepEqual(readFileSync(join(output, result.binaries[0].file)), readFileSync(binary));
  assert.ok(!JSON.stringify(result).includes(root));
  assert.ok(result.debug_info.includes("debug_0"));
});

test("mismatch or tool error never leaves a qualified copy", (t) => {
  const cases = [[[], "image_identity_mismatch"], [[EXPECTED, EXPECTED], "image_identity_mismatch"], [new EvidenceError("image_tool_failed"), "image_tool_failed"]];
  for (const [value, error] of cases) {
    const { output, collect } = setup(t);
    assert.deepEqual(collect(undefined, { identities: answers(value) }).errors, [error]);
    assert.deepEqual(readdirSync(output), []);
  }
});

test("staged identity is verified", (t) => {
  const { output, collect } = setup(t);
  assert.deepEqual(collect(undefined, { identities: answers([EXPECTED], []) }).errors, ["staged_image_identity_mismatch"]);
  assert.deepEqual(readdirSync(output), []);
});

test("symlink, missing and byte limit are errors", (t) => {
  const { root, binary, collect } = setup(t);
  unlinkSync(binary);
  symlinkSync(join(root, "missing"), binary);
  assert.deepEqual(collect().errors, ["binary_read_or_copy_failed"]);
  unlinkSync(binary);
  assert.deepEqual(collect().errors, ["binary_read_or_copy_failed"]);
  writeFileSync(binary, "too large");
  assert.deepEqual(collect(undefined, { maxBytes: 1 }).errors, ["binary_byte_limit"]);
});

test("fifo is rejected without waiting for a writer", (t) => {
  const { binary, collect } = setup(t);
  unlinkSync(binary);
  assert.equal(spawnSync("mkfifo", [binary]).status, 0);
  assert.deepEqual(collect().errors, ["not_regular_file"]);
});

test("tool stdout and stderr are bounded while reading", () => {
  const refuses = (code) => (error) => error instanceof EvidenceError && error.message === code;
  for (const descriptor of [1, 2]) {
    assert.throws(() => boundedTool([process.execPath, "-e", `require("fs").writeSync(${descriptor}, Buffer.alloc(1000000, 120))`]), refuses("image_tool_output_limit"), `descriptor ${descriptor}`);
  }
  assert.throws(() => boundedTool([process.execPath, "-e", "process.exit(7)"]), refuses("image_tool_failed"));
  assert.throws(() => boundedTool([process.execPath, "-e", "setTimeout(() => {}, 10000)"], 0.05), refuses("image_tool_timeout"));
});

test("ambiguous, missing or unsafe report identity is refused", (t) => {
  const { report, collect } = setup(t);
  const cases = [
    [{ process_images: [] }, "process_image_ambiguous_or_missing"],
    [{ process_images: [report.process_images[0], report.process_images[0]] }, "process_image_ambiguous_or_missing"],
    [{ process_images: [{ ...report.process_images[0] }], process_images_truncated: true }, "process_image_ambiguous_or_missing"],
    [{ process: "../bridge_lib-cafe" }, "invalid_process_name"],
    [{ process: 7 }, "invalid_process_name"],
    [{ process_images: [{ uuid: UUID, arch: "private/path" }] }, "process_image_identity_unavailable"],
    [{ process_images: [{ uuid: [UUID], arch: "arm64" }] }, "process_image_identity_unavailable"],
  ];
  for (const [mutation, error] of cases) assert.deepEqual(collect([{ ...report, ...mutation }]).errors, [error], error);
  assert.deepEqual(collect([7]).errors, ["invalid_process_name"]);
});

test("a snapshot that is not a crash capture is refused", (t) => {
  const { target, output } = setup(t);
  for (const bad of [[1], null, { reports: "x" }, { reports: {} }]) {
    assert.throws(() => retainCollect(bad, target, output), (error) => error instanceof EvidenceError && error.message === "invalid_reports");
  }
});

test("no report is unavailable and truncation is incomplete", (t) => {
  const { target, output, report, collect } = setup(t);
  const empty = retainCollect({ status: "no_fresh_reports", reports: [] }, target, output);
  assert.deepEqual([empty.status, empty.errors], ["unavailable", []]);
  const bounded = collect(Array(9).fill(report), { identities: answers([EXPECTED]) });
  assert.equal(bounded.status, "partial");
  assert.equal(bounded.reports_truncated, true);
});

test("one retained binary and one refused identity is a partial result", (t) => {
  const { target, output, report, collect } = setup(t);
  writeFileSync(join(target, "bridge_lib-beef"), "other executable bytes");
  const other = { process: "bridge_lib-beef", process_images: [{ uuid: "00000000-0000-0000-0000-000000000002", arch: "arm64" }] };
  const result = collect([report, other], { identities: (path) => (path.endsWith("beef") ? [] : [EXPECTED]) });
  assert.deepEqual([result.status, result.errors, result.binaries.length, readdirSync(output).length], ["partial", ["image_identity_mismatch"], 1, 1]);
});

function fixtureReport(mutate) {
  const text = readFileSync(FIXTURE, "utf8");
  const split = text.indexOf("\n");
  const body = JSON.parse(text.slice(split + 1));
  mutate(body);
  return `${text.slice(0, split)}\n${JSON.stringify(body)}`;
}

test("captured format maps frame identity and rejects boolean index", () => {
  const result = minimize(
    fixtureReport((body) => {
      Object.assign(body.usedImages[0], { uuid: UUID.toUpperCase(), arch: "arm64" });
      body.threads[0].frames[0].imageIndex = true;
    }),
  );
  assert.equal(result.process_images[0].uuid, UUID);
  assert.equal(result.threads[0].frames[0].image_index, null);
  assert.equal(result.threads[0].frames[3].uuid, UUID);
  assert.equal(result.threads[0].frames[3].image_index, 0);
  const odd = minimize(fixtureReport((body) => Object.assign(body.usedImages[0], { uuid: "private/path", arch: [] })));
  assert.deepEqual([odd.process_images[0].uuid, odd.process_images[0].arch], [null, null]);
});

test("large valid stack output preserves executable linkage", (t) => {
  const { target, output } = setup(t);
  const raw = fixtureReport((body) => {
    Object.assign(body.usedImages[0], { uuid: UUID, arch: "arm64" });
    body.usedImages[1].name = "library".repeat(73);
    const frame = { imageIndex: 1, imageOffset: 1, symbol: "s".repeat(512), symbolLocation: 0 };
    body.threads = Array.from({ length: 64 }, () => ({ triggered: true, frames: Array(64).fill(frame) }));
  });
  assert.ok(Buffer.byteLength(raw) < CAPTURE_MAX_BYTES);
  const minimized = minimize(raw);
  const snapshot = { status: "captured", reports: Array.from({ length: 8 }, () => structuredClone(minimized)), errors: [], truncated: false };
  assert.ok(Buffer.byteLength(JSON.stringify(snapshot)) > 16 * 1024 * 1024);
  const encoded = encodeResult(snapshot);
  assert.ok(encoded.length <= MAX_OUTPUT_BYTES && MAX_OUTPUT_BYTES < 16 * 1024 * 1024);
  const bounded = JSON.parse(encoded);
  assert.equal(bounded.details_truncated, true);
  for (const item of bounded.reports) {
    assert.deepEqual(item.process_images, minimized.process_images);
    assert.ok(item.frames_removed_for_output_limit > 0);
  }
  const result = retainCollect(bounded, target, output, { identities: answers([EXPECTED]) });
  assert.deepEqual([result.status, result.binaries.length, result.errors], ["partial", 1, ["crash_capture_incomplete"]]);
});

test("the command line refuses what it does not recognise", () => {
  assert.deepEqual(parseCli(["--crash-json", "a", "--target", "b", "--output", "c"]), { crashJson: "a", target: "b", output: "c" });
  for (const argv of [[], ["--crash-json", "a", "--target", "b"], ["--crash-json", "a", "--target", "b", "--output", "c", "x"], ["--crash", "a", "--target", "b", "--output", "c"]]) {
    assert.equal(typeof parseCli(argv), "string", argv.join(" "));
  }
});

test("main: unreadable or oversized input is an error manifest with exit 1; an existing output is refused", (t) => {
  const { root, target } = setup(t);
  let printed = "";
  const run = (crash, destination) =>
    main(["--crash-json", crash, "--target", target, "--output", destination], { stdout: { write: (text) => (printed = text) }, provenance: () => ({ checkout_sha: null }) });
  const output = join(root, "evidence");
  assert.equal(run(join(root, "missing.json"), output), 1);
  assert.deepEqual(JSON.parse(printed), { status: "error", binaries: 0, errors: ["input_or_provenance_failed"] });
  assert.deepEqual(JSON.parse(readFileSync(join(output, "manifest.json"), "utf8")), {
    status: "error", binaries: [], errors: ["input_or_provenance_failed"], debug_info: "line_tables_unavailable_ci_profile_debug_0", provenance: { checkout_sha: null },
  });
  assert.throws(() => run(join(root, "missing.json"), output), { code: "EEXIST" });
  const oversized = join(root, "big.json");
  writeFileSync(oversized, Buffer.alloc(16 * 1024 * 1024 + 1, 32));
  assert.equal(run(oversized, join(root, "evidence2")), 1);
  assert.deepEqual(JSON.parse(printed).errors, ["crash_json_byte_limit"]);
});
