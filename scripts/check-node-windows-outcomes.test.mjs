import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath, pathToFileURL } from "node:url";
import { compare, EXIT, failureKey, parseBaseline, parseOutcomes, summary } from "./check-node-windows-outcomes.mjs";
import { repoFile } from "./node-test-outcomes.mjs";

const scripts = path.dirname(fileURLToPath(import.meta.url));
const pass = (file, ...names) => ({ file, names, outcome: "pass" });
const fail = (file, ...names) => ({ file, names, outcome: "fail", error: "boom" });
const files = ["scripts/a.test.mjs", "scripts/b.test.mjs"];
const run = (overrides) => compare({ outcomes: [pass(files[0], "t"), pass(files[1], "t")], baseline: [], testFiles: files, nodeExit: 0, ...overrides });

test("a run whose only failures are known passes, and a failure outside the known ones fails", () => {
  const known = failureKey(fail(files[0], "outer", "inner"));
  const outcomes = [fail(files[0], "outer", "inner"), pass(files[1], "t")];
  const ok = run({ outcomes, baseline: [known], nodeExit: 1 });
  assert.equal(ok.code, EXIT.ok);
  assert.equal(ok.reason, "ok");
  // the negative control: the same run without the known-failures file must fail on that test
  const control = run({ outcomes, baseline: [], nodeExit: 1 });
  assert.equal(control.code, EXIT.newFailures);
  assert.deepEqual(control.newFailures.map((entry) => entry.key), [known]);
});

test("a failure counted twice against one known entry is one new failure", () => {
  const key = failureKey(fail(files[0], "dup"));
  const result = run({ outcomes: [fail(files[0], "dup"), fail(files[0], "dup"), pass(files[1], "t")], baseline: [key], nodeExit: 1 });
  assert.equal(result.code, EXIT.newFailures);
  assert.equal(result.newFailures.length, 1);
});

test("a known failure that now passes is listed, and does not fail the run", () => {
  const result = run({ baseline: [failureKey(fail(files[0], "fixed"))] });
  assert.equal(result.code, EXIT.ok);
  assert.deepEqual(result.fixed, [failureKey(fail(files[0], "fixed"))]);
});

test("a run that cannot be trusted exits 2 with its own reason, never as a pass", () => {
  assert.deepEqual([run({ outcomes: [] }).code, run({ outcomes: [] }).reason], [EXIT.untrusted, "outcomes_missing"]);
  const missing = run({ outcomes: [pass(files[0], "t")] });
  assert.deepEqual([missing.code, missing.reason, missing.missing], [EXIT.untrusted, "file_set_differs", [files[1]]]);
  const extra = run({ outcomes: [pass(files[0], "t"), pass(files[1], "t"), pass("scripts/c.test.mjs", "t")] });
  assert.deepEqual([extra.code, extra.reason, extra.extra], [EXIT.untrusted, "file_set_differs", ["scripts/c.test.mjs"]]);
  const silent = run({ nodeExit: 1 });
  assert.deepEqual([silent.code, silent.reason], [EXIT.untrusted, "exit_without_failure"]);
});

test("a line break in a test name does not split its key", () => {
  assert.equal(failureKey(fail(files[0], "a\nb")), "scripts/a.test.mjs > a\\nb");
});

test("the baseline file ignores comments and blank lines", () => {
  assert.deepEqual(parseBaseline("# a header\n\nscripts/a.test.mjs > t\n  scripts/b.test.mjs > u  \n"), ["scripts/a.test.mjs > t", "scripts/b.test.mjs > u"]);
});

test("the summary names the new failures and the untrusted reason", () => {
  const failed = run({ outcomes: [fail(files[0], "t"), pass(files[1], "t")], nodeExit: 1 });
  assert.match(summary(failed), /New failures: 1\./);
  assert.match(summary(failed), /scripts\/a\.test\.mjs > t`: boom/);
  assert.match(summary(run({ outcomes: [] })), /No outcomes were written/);
});

test("the reporter writes one line per test, nested names, skips, and no suite or derived failure", () => {
  const dir = mkdtempSync(path.join(os.tmpdir(), "node-outcomes-"));
  try {
    writeFileSync(path.join(dir, "probe.test.mjs"), [
      'import test, { describe, it } from "node:test";',
      'describe("outer", () => {',
      '  it("passes", () => {});',
      '  it("fails", () => { throw new Error("boom\\nsecond line"); });',
      '  it.skip("skipped", () => {});',
      "});",
      'describe("clean suite", () => { it("also passes", () => {}); });',
      'test("parent", async (t) => { await t.test("child fails", () => { throw new Error("child"); }); });',
      'test("top fails", () => { throw new Error("x"); });',
      "",
    ].join("\n"));
    const out = path.join(dir, "outcomes.jsonl");
    // the test runner marks its children; a nested `node --test` must not inherit that mark
    const { NODE_TEST_CONTEXT: _context, ...env } = process.env;
    const result = spawnSync(process.execPath, ["--experimental-strip-types", "--test", `--test-reporter=${pathToFileURL(path.join(scripts, "node-test-outcomes.mjs")).href}`, `--test-reporter-destination=${out}`, path.join(dir, "probe.test.mjs")], { encoding: "utf8", env });
    assert.equal(result.status, 1, result.stderr);
    const lines = parseOutcomes(readFileSync(out, "utf8"));
    assert.deepEqual(lines, [
      { file: "scripts/probe.test.mjs", names: ["outer", "passes"], outcome: "pass" },
      { file: "scripts/probe.test.mjs", names: ["outer", "fails"], outcome: "fail", error: "boom" },
      { file: "scripts/probe.test.mjs", names: ["outer", "skipped"], outcome: "skip" },
      { file: "scripts/probe.test.mjs", names: ["clean suite", "also passes"], outcome: "pass" },
      { file: "scripts/probe.test.mjs", names: ["parent", "child fails"], outcome: "fail", error: "child" },
      { file: "scripts/probe.test.mjs", names: ["top fails"], outcome: "fail", error: "x" },
    ]);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("a test file is named the same whatever the host's path form", () => {
  assert.equal(repoFile("C:\\a\\bridge\\scripts\\x.test.mjs"), "scripts/x.test.mjs");
  assert.equal(repoFile("/home/runner/work/bridge/scripts/x.test.mjs"), "scripts/x.test.mjs");
});

test("the command line exits 0, 1 and 2 for a known run, a new failure and a missing outcomes file", () => {
  const dir = mkdtempSync(path.join(os.tmpdir(), "node-outcomes-cli-"));
  try {
    const suite = readdirSync(scripts).filter((name) => name.endsWith(".test.mjs")).map((name) => `scripts/${name}`);
    const outcomes = path.join(dir, "outcomes.jsonl");
    const baseline = path.join(dir, "baseline.txt");
    const lines = suite.map((file, index) => JSON.stringify(index === 0 ? fail(file, "t") : pass(file, "t")));
    writeFileSync(outcomes, `${lines.join("\n")}\n`);
    const cli = (args) => spawnSync(process.execPath, [path.join(scripts, "check-node-windows-outcomes.mjs"), ...args], { encoding: "utf8" });
    writeFileSync(baseline, `${failureKey(fail(suite[0], "t"))}\n`);
    assert.equal(cli(["--outcomes", outcomes, "--baseline", baseline, "--node-exit", "1"]).status, EXIT.ok);
    writeFileSync(baseline, "# nothing known\n");
    assert.equal(cli(["--outcomes", outcomes, "--baseline", baseline, "--node-exit", "1"]).status, EXIT.newFailures);
    assert.equal(cli(["--outcomes", path.join(dir, "absent.jsonl"), "--baseline", baseline, "--node-exit", "0"]).status, EXIT.untrusted);
    assert.equal(cli(["--outcomes", outcomes]).status, EXIT.untrusted);
    // an unset or non-numeric node exit status is never read as 0
    for (const bad of ["", "x", "-1", "1.5"]) {
      assert.equal(cli(["--outcomes", outcomes, "--baseline", baseline, "--node-exit", bad]).status, EXIT.untrusted, bad);
    }
    // a line that is not a reporter line is its own untrusted reason, not "missing" and not a crash
    writeFileSync(outcomes, '{"file":"scripts/a.test.mjs","outcome":"fail"}\n');
    const malformed = cli(["--outcomes", outcomes, "--baseline", baseline, "--node-exit", "1"]);
    assert.equal(malformed.status, EXIT.untrusted);
    assert.match(malformed.stdout, /could not be read/);
    // every test file reports and node exited 0, but one line has an outcome the reporter never writes
    const weird = suite.map((file, index) => (index === 0 ? JSON.stringify({ file, names: ["t"], outcome: "weird" }) : JSON.stringify(pass(file, "t"))));
    writeFileSync(outcomes, `${weird.join("\n")}\n`);
    assert.equal(cli(["--outcomes", outcomes, "--baseline", baseline, "--node-exit", "0"]).status, EXIT.untrusted);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});
