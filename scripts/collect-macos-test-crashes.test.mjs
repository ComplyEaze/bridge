// SPDX-License-Identifier: Apache-2.0
// node --test scripts/collect-macos-test-crashes.test.mjs
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { chmodSync, mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { collect, encodeResult, label, main, minimize, parseCli, waitForReports } from "./collect-macos-test-crashes.mjs";

const FIXTURE = join(dirname(fileURLToPath(import.meta.url)), "testdata/macos-sigtrap-control.ips");

function report(mutate) {
  const text = readFileSync(FIXTURE, "utf8");
  const split = text.indexOf("\n");
  const body = JSON.parse(text.slice(split + 1));
  mutate?.(body);
  return `${text.slice(0, split)}\n${JSON.stringify(body)}`;
}

function temporaryDirectory(t) {
  const directory = mkdtempSync(join(tmpdir(), "collect-crashes-"));
  t.after(() => rmSync(directory, { recursive: true, force: true }));
  return directory;
}

// A monotonic clock answering from a script; `onSleep` runs on each sleep.
function scriptedClock(values, onSleep) {
  const sleeps = [];
  return { sleeps, now: () => values.shift(), sleep: (seconds) => (sleeps.push(seconds), onSleep?.()) };
}

test("captured control retains signal and stack", () => {
  const result = minimize(report());
  assert.equal(result.exception.signal, "SIGTRAP");
  assert.deepEqual(result.threads[0].frames.map((frame) => frame.symbol), ["__pthread_kill", "pthread_kill", "raise", "main", "start"]);
});

test("omits raw process data and paths with spaces", () => {
  for (const path of ["/Volumes/Synthetic Volume/private source.rs", String.raw`D:\Synthetic Share\private source.rs`]) {
    const result = minimize(
      report((body) => {
        body.procPath = path;
        body.environment = { TOKEN: "must-not-escape" };
        body.exception.message = "must-not-escape";
        body.threads[0].threadState = { register: "must-not-escape" };
        body.threads[0].frames[0].symbol = `worker_exit ${path}`;
      }),
    );
    const encoded = JSON.stringify(result);
    for (const leaked of ["must-not-escape", "Synthetic", "private source.rs"]) assert.ok(!encoded.includes(leaked), path);
    assert.equal(result.threads[0].frames[0].symbol, null);
    assert.equal(result.threads[0].frames[0].image, "libsystem_kernel.dylib");
  }
});

test("distinguishes missing from invalid fresh report", (t) => {
  const directory = temporaryDirectory(t);
  assert.equal(collect([directory], 0).status, "no_fresh_reports");
  writeFileSync(join(directory, "bridge_lib-test.ips"), "invalid");
  assert.equal(collect([directory], 0).status, "capture_error");
  assert.equal(collect([join(directory, "absent")], 0).status, "no_fresh_reports");
});

test("waits for partial report to finish", (t) => {
  const directory = temporaryDirectory(t);
  const path = join(directory, "bridge_lib-cafe.ips");
  writeFileSync(path, report().slice(0, 60));
  const result = waitForReports([directory], 0, 2, scriptedClock([0, 0, 2], () => writeFileSync(path, report())));
  assert.equal(result.status, "captured");
  assert.equal(result.reports.length, 1);
});

test("waits for later reports without duplicating earlier report", (t) => {
  const directory = temporaryDirectory(t);
  writeFileSync(join(directory, "bridge_lib-cafe-first.ips"), report());
  const clock = scriptedClock([0, 0, 1, 2], () => writeFileSync(join(directory, "bridge_lib-cafe-second.ips"), report()));
  const result = waitForReports([directory], 0, 2, clock);
  assert.equal(result.status, "captured");
  assert.equal(result.reports.length, 2);
  assert.equal(clock.sleeps.length, 2);
});

test("preserves parse error at deadline", (t) => {
  const directory = temporaryDirectory(t);
  writeFileSync(join(directory, "bridge_lib-cafe.ips"), "invalid");
  const clock = scriptedClock([0, 0, 2]);
  const result = waitForReports([directory], 0, 2, clock);
  assert.equal(result.status, "capture_error");
  assert.deepEqual(result.errors, ["report_read_or_parse_failed"]);
  assert.deepEqual(clock.sleeps, [1]);
});

test("excludes old reports and bounds frames", (t) => {
  const directory = temporaryDirectory(t);
  const path = join(directory, "bridge_lib-cafe.ips");
  writeFileSync(path, report());
  assert.equal(collect([directory], statSync(path).mtimeMs + 1000).status, "no_fresh_reports");
  const result = minimize(report((body) => (body.threads[0].frames = Array(70).fill(body.threads[0].frames[0]))));
  assert.equal(result.threads[0].frames_truncated, true);
  assert.equal(result.threads[0].frames.length, 64);
});

test("retains late faulting and triggered threads", () => {
  const result = minimize(
    report((body) => {
      body.threads = Array.from({ length: 70 }, () => ({ ...body.threads[0], triggered: false }));
      body.faultingThread = 69;
      body.threads[68].triggered = true;
    }),
  );
  assert.equal(result.threads_truncated, true);
  assert.equal(result.threads.length, 64);
  assert.deepEqual(result.threads.slice(0, 2).map((thread) => thread.index), [69, 68]);
  assert.equal(result.threads[1].triggered, true);
});

// Behaviour the port could get wrong.

test("labels cut at 512 code points and refuse separators and controls", () => {
  assert.equal([...label("\u{1f600}".repeat(600))].length, 512);
  for (const bad of ["a/b", "a\\b", "a\tb", 7]) assert.equal(label(bad), null);
});

test("a report of the wrong shape is one parse error, not a crash", (t) => {
  const directory = temporaryDirectory(t);
  const bad = [
    (body) => (body.usedImages = [7]),
    (body) => (body.threads = 5),
    (body) => (body.procName = ["bridge_lib"]),
    (body) => (body.threads[0].frames = [null]),
  ];
  bad.forEach((mutate, index) => writeFileSync(join(directory, `bridge_lib-${index}.ips`), report(mutate)));
  writeFileSync(join(directory, "bridge_lib-x.ips"), '{"bug_type": "1"}\n{}');
  const result = collect([directory], 0);
  assert.equal(result.status, "capture_error");
  assert.deepEqual(result.errors, Array(5).fill("report_read_or_parse_failed"));
});

test("symlinked reports are errors and are never read", (t) => {
  const directory = temporaryDirectory(t);
  writeFileSync(join(directory, "real.txt"), report());
  symlinkSync(join(directory, "real.txt"), join(directory, "bridge_lib-link.ips"));
  const result = collect([directory], 0);
  assert.deepEqual(result.errors, ["symlink_report"]);
  assert.deepEqual(result.reports, []);
});

test("more than eight fresh reports are truncated and reported as a capture error", (t) => {
  const directory = temporaryDirectory(t);
  for (let index = 0; index < 9; index += 1) writeFileSync(join(directory, `bridge_lib-${index}.ips`), report());
  const result = collect([directory], 0);
  assert.deepEqual([result.truncated, result.reports.length, result.status], [true, 8, "capture_error"]);
});

test("output over the limit thins frames evenly; an unthinnable one is refused", () => {
  const base = minimize(report());
  const frame = { ...base.threads[0].frames[0], symbol: "s".repeat(512) };
  const heavy = () => ({ ...structuredClone(base), threads: Array.from({ length: 64 }, (_, index) => ({ index, triggered: true, frames_truncated: false, frames: Array(64).fill(frame) })) });
  const encoded = encodeResult({ status: "captured", reports: Array.from({ length: 8 }, heavy), errors: [], truncated: false });
  assert.ok(encoded.length <= 8 * 1024 * 1024);
  const bounded = JSON.parse(encoded);
  assert.deepEqual([bounded.details_truncated, bounded.status], [true, "capture_error"]);
  const errors = Array(70).fill("report_read_or_parse_failed");
  const capped = JSON.parse(encodeResult({ status: "capture_error", reports: [], errors, truncated: false }));
  assert.deepEqual([capped.errors.length, capped.errors_omitted], [64, 6]);
  const unthinnable = Array.from({ length: 8 }, () => ({ ...structuredClone(base), threads: [], process: "x".repeat(2 * 1024 * 1024) }));
  assert.throws(() => encodeResult({ status: "captured", reports: unthinnable, errors: [], truncated: false }), /crash_metadata_exceeds_output_limit/);
});

test("the command line refuses what it does not recognise", () => {
  assert.deepEqual(parseCli(["--since-file", "a", "--output", "b", "--wait-seconds", "30"]), { sinceFile: "a", output: "b", waitSeconds: 30 });
  assert.equal(parseCli(["--since-file", "a", "--output", "b"]).waitSeconds, 0);
  const refused = [[], ["--since-file", "a"], ["--since-file", "a", "--output", "b", "extra"], ["--since", "a", "--output", "b"], ["--since-file", "a", "--output", "b", "--wait-seconds", "31"], ["--since-file", "a", "--output", "b", "--wait-seconds", "x"]];
  for (const argv of refused) assert.equal(typeof parseCli(argv), "string", argv.join(" "));
  let printed = "";
  assert.equal(main([], { stderr: { write: (text) => (printed += text) } }), 2);
  assert.match(printed, /^--since-file and --output are required\nusage: /);
});

test("main writes the file, prints the summary and exits 1 only for a capture error", (t) => {
  const directory = temporaryDirectory(t);
  writeFileSync(join(directory, "start"), "");
  const reports = join(directory, "reports");
  const run = (name) => {
    let printed = "";
    const code = main(["--since-file", join(directory, "start"), "--output", join(directory, name)], { directories: [reports], stdout: { write: (text) => (printed += text) } });
    return [code, JSON.parse(printed)];
  };
  assert.deepEqual(run("none.json"), [0, { status: "no_fresh_reports", reports: 0, errors: [] }]);
  mkdirSync(reports);
  writeFileSync(join(reports, "bridge_lib-cafe.ips"), report());
  assert.deepEqual(run("ok.json"), [0, { status: "captured", reports: 1, errors: [] }]);
  assert.equal(JSON.parse(readFileSync(join(directory, "ok.json"), "utf8")).reports[0].exception.signal, "SIGTRAP");
  writeFileSync(join(reports, "bridge_lib-bad.ips"), "invalid");
  assert.deepEqual(run("bad.json"), [1, { status: "capture_error", reports: 1, errors: ["report_read_or_parse_failed"] }]);
});

test("a diagnostics folder that cannot be listed is skipped, as the Python glob did", (t) => {
  if (process.getuid?.() === 0) return t.skip("root can list any folder");
  const root = mkdtempSync(join(tmpdir(), "crash-unreadable-"));
  t.after(() => {
    chmodSync(root, 0o700);
    rmSync(root, { recursive: true, force: true });
  });
  chmodSync(root, 0o000);
  assert.deepEqual(collect([root], 0), { status: "no_fresh_reports", reports: [], errors: [], truncated: false });
});

test("a FIFO named like a report is an error and never blocks the read", (t) => {
  const root = mkdtempSync(join(tmpdir(), "crash-fifo-"));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  execFileSync("mkfifo", [join(root, "bridge_lib-fifo.ips")]);
  const result = collect([root], 0);
  assert.equal(result.status, "capture_error");
  assert.deepEqual(result.errors, ["report_read_or_parse_failed"]);
});
