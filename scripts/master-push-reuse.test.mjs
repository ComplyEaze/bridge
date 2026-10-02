// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import { FAMILIES, decide, forcesFullRun } from "./master-push-reuse.mjs";

const SHA = "a".repeat(40);
const BEFORE = "b".repeat(40);
const env = {
  GITHUB_EVENT_NAME: "push", GITHUB_REF: "refs/heads/master", GITHUB_SHA: SHA, BEFORE_SHA: BEFORE,
  GITHUB_REPOSITORY: "example/bridge", GH_TOKEN: "synthetic-test-token",
};
const response = (status, body) => ({ status, ok: status >= 200 && status < 300, json: async () => body });
const queueRun = (extra = {}) => ({
  id: 101, head_sha: SHA, event: "merge_group", status: "completed", conclusion: "success", ...extra,
});
const job = (name, conclusion = "success", extra = {}) => ({ name, status: "completed", conclusion, ...extra });
const listing = (runs, extra = {}) => ({ total_count: runs.length, workflow_runs: runs, ...extra });
const jobsOf = (jobs, extra = {}) => ({ total_count: jobs.length, jobs, ...extra });

const ALWAYS = ["Frontend build", "Workflow consistency", "Rust format", "Determine bundle scope", "Tally portable core"];
const everyJob = [...FAMILIES.native, ...FAMILIES.bundle];
// The listing of a queue run whose heavy jobs all ran, and of one that skipped them. The skipped
// shape is what GitHub reported for a real queue run of a documentation-only commit (2 Oct 2026):
// a skipped matrix job is listed once under its unexpanded name, never once per matrix entry.
const ranEverything = jobsOf([...ALWAYS, ...everyJob, "Tax-audit mutation records", "Required checks"].map((name) => job(name)));
const skippedHeavyJobs = jobsOf([
  ...ALWAYS.map((name) => job(name)),
  ...["Native checks (${{ matrix.os }})", "Bundle smoke (${{ matrix.os }})", "Tax-audit mutation records",
    "Seam positive control (${{ matrix.os }})"].map((name) => job(name, "skipped")),
  job("Required checks"),
]);
const without = (names, base = ranEverything) => jobsOf(base.jobs.filter((entry) => !names.includes(entry.name)));
const withConclusion = (name, conclusion, extra = {}) =>
  jobsOf(ranEverything.jobs.map((entry) => (entry.name === name ? { ...entry, conclusion, ...extra } : entry)));
const reused = (outcome) => ({ native: outcome.native, bundle: outcome.bundle });
const NOTHING = { native: false, bundle: false };
const EVERYTHING = { native: true, bundle: true };

// A GitHub stand-in that answers the two lookups and records every URL it was asked for.
function github({ runs = listing([queueRun()]), jobs = ranEverything, failWith } = {}) {
  const calls = [];
  const fetcher = async (url, options) => {
    calls.push({ url, options });
    if (failWith !== undefined) {
      if (typeof failWith === "number") return response(failWith, {});
      throw failWith;
    }
    if (url.includes("/workflows/ci.yml/runs")) return response(200, runs);
    if (url.includes("/runs/101/jobs")) return response(200, jobs);
    return response(404, {});
  };
  return { fetcher, calls };
}
const ordinaryChange = () => ["docs/release-process.md", "src-tauri/src/agent.rs"];
const run = (overrides = {}, files = ordinaryChange) =>
  decide({ env, changedFiles: files, isAncestor: () => true, sleep: async () => {}, ...overrides });

test("a queue run whose native, bundle and seam jobs all passed lets the push reuse all of them", async () => {
  const { fetcher, calls } = github();
  const outcome = await run({ fetcher });
  assert.deepEqual(reused(outcome), EVERYTHING, outcome.reason);
  assert.match(calls[0].url, /event=merge_group&head_sha=a{40}&per_page=100/);
  assert.equal(calls[0].options.redirect, "error");
});

test("a queue run that skipped the heavy jobs is not evidence that they passed", async () => {
  // The real shape of a documentation-only commit's queue run: Required checks is green and every heavy job is skipped.
  const outcome = await run({ fetcher: github({ jobs: skippedHeavyJobs }).fetcher });
  assert.deepEqual(reused(outcome), NOTHING, outcome.reason);
});

test("each family is judged on its own jobs", async () => {
  const nativeOnly = await run({ fetcher: github({ jobs: without(FAMILIES.bundle) }).fetcher });
  assert.deepEqual(reused(nativeOnly), { native: true, bundle: false });
  const bundleOnly = await run({ fetcher: github({ jobs: without(FAMILIES.native) }).fetcher });
  assert.deepEqual(reused(bundleOnly), { native: false, bundle: true });
});

test("one job of a family missing, repeated, unsuccessful or unfinished takes that whole family back", async () => {
  for (const [family, other] of [["native", "bundle"], ["bundle", "native"]]) {
    for (const name of FAMILIES[family]) {
      for (const jobs of [
        without([name]),
        jobsOf([...ranEverything.jobs, job(name)]),
        withConclusion(name, "failure"),
        withConclusion(name, "cancelled"),
        withConclusion(name, "skipped"),
        withConclusion(name, null, { status: "in_progress" }),
        withConclusion(name, "success", { status: "queued" }),
      ]) {
        const outcome = await run({ fetcher: github({ jobs }).fetcher });
        assert.equal(outcome[family], false, `${family}: ${name}`);
        assert.equal(outcome[other], true, `${other} is judged on its own jobs: ${name}`);
      }
    }
  }
});

test("a job listing with more jobs than it returned runs everything", async () => {
  const outcome = await run({ fetcher: github({ jobs: { ...ranEverything, total_count: 101 } }).fetcher });
  assert.deepEqual(reused(outcome), NOTHING);
  assert.match(outcome.reason, /incomplete/);
});

test("no queue run for the commit runs everything", async () => {
  const outcome = await run({ fetcher: github({ runs: listing([]) }).fetcher });
  assert.deepEqual(outcome, { native: false, bundle: false, reason: "no merge-queue run for this commit" });
});

test("a failed or cancelled queue run runs everything, whatever its jobs say", async () => {
  for (const conclusion of ["failure", "cancelled", "timed_out", "skipped", null]) {
    const outcome = await run({ fetcher: github({ runs: listing([queueRun({ conclusion })]) }).fetcher });
    assert.deepEqual(reused(outcome), NOTHING, String(conclusion));
    assert.match(outcome.reason, /concluded/);
  }
});

test("a queue run for a different commit runs everything", async () => {
  const other = queueRun({ head_sha: "c".repeat(40) });
  const outcome = await run({ fetcher: github({ runs: listing([other]) }).fetcher });
  assert.deepEqual(reused(outcome), NOTHING);
  assert.match(outcome.reason, /no merge-queue run/);
});

test("a run that is not a merge_group run is not a queue run", async () => {
  const outcome = await run({ fetcher: github({ runs: listing([queueRun({ event: "pull_request" })]) }).fetcher });
  assert.deepEqual(reused(outcome), NOTHING);
});

for (const [name, failWith] of [
  ["an API error status", 500],
  ["a missing token scope", 403],
  ["a network failure", new TypeError("fetch failed")],
  ["a timeout", Object.assign(new Error("timed out"), { name: "TimeoutError" })],
]) {
  test(`${name} runs everything`, async () => {
    const outcome = await run({ fetcher: github({ failWith }).fetcher });
    assert.deepEqual(reused(outcome), NOTHING);
    assert.match(outcome.reason, /lookup failed/);
  });
}

test("a lockfile, manifest, toolchain or .github change runs everything without asking the API", async () => {
  for (const file of [
    "src-tauri/Cargo.lock", "Cargo.toml", "src-tauri/crates/bridge-tally-core/Cargo.toml", "tools/Cargo.lock",
    "rust-toolchain.toml", "rust-toolchain", ".github/workflows/ci.yml", ".github/actions/setup-windows-native/action.yml",
  ]) {
    const { fetcher, calls } = github();
    const outcome = await run({ fetcher }, () => ["docs/a.md", file]);
    assert.deepEqual(reused(outcome), NOTHING, file);
    assert.match(outcome.reason, /always runs everything/, file);
    assert.equal(calls.length, 0, file);
  }
  assert.equal(forcesFullRun(["src/Cargo.toml.md", "docs/Cargo.lockfile", "x.github/a"]), undefined);
});

test("a push whose before commit is not an ancestor runs everything without asking the API", async () => {
  const { fetcher, calls } = github();
  const asked = [];
  const outcome = await run({ fetcher, isAncestor: (before, sha) => { asked.push([before, sha]); return false; } });
  assert.deepEqual(reused(outcome), NOTHING);
  assert.match(outcome.reason, /not a fast-forward/);
  assert.deepEqual(asked, [[BEFORE, SHA]]);
  assert.equal(calls.length, 0);
});

test("a git failure while testing ancestry or listing changed files runs everything", async () => {
  const boom = () => { throw new Error("git failed"); };
  for (const overrides of [{ isAncestor: boom }, { changedFiles: boom }]) {
    const { fetcher, calls } = github();
    const outcome = await run({ fetcher, ...overrides });
    assert.deepEqual(reused(outcome), NOTHING);
    assert.equal(calls.length, 0);
  }
});

test("the ancestry test is git's own: an ancestor passes, a stranger and a missing commit do not", async () => {
  const { spawnSync } = await import("node:child_process");
  const { mkdtempSync, rmSync, writeFileSync } = await import("node:fs");
  const { tmpdir } = await import("node:os");
  const { join } = await import("node:path");
  const dir = mkdtempSync(join(tmpdir(), "reuse-ancestry-"));
  try {
    const git = (...args) => {
      const result = spawnSync("git", ["-c", "user.name=t", "-c", "user.email=t@example.invalid", "-c", "commit.gpgsign=false", ...args],
        { cwd: dir, encoding: "utf8" });
      assert.equal(result.status, 0, result.stderr);
      return result.stdout.trim();
    };
    git("init", "-q", "-b", "main");
    const commit = (name) => { writeFileSync(join(dir, name), name); git("add", name); git("commit", "-q", "-m", name); return git("rev-parse", "HEAD"); };
    const first = commit("a");
    const second = commit("b");
    git("checkout", "-q", "-b", "other", first);
    const stranger = commit("c");
    const defaultAncestry = async (before, sha) => (await decide({
      env: { ...env, BEFORE_SHA: before, GITHUB_SHA: sha }, cwd: dir, changedFiles: ordinaryChange,
      fetcher: github({ failWith: 500 }).fetcher, sleep: async () => {},
    })).reason;
    // An ancestor gets past the ancestry test (and is then stopped by the stubbed API failure).
    assert.match(await defaultAncestry(first, second), /lookup failed \(500\)/);
    // A commit on another line of history, and one the clone does not hold, are refused or error out.
    assert.match(await defaultAncestry(stranger, second), /not a fast-forward/);
    assert.match(await defaultAncestry("d".repeat(40), second), /lookup failed \(Error\)/);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("two queue runs for one commit is ambiguous and runs everything", async () => {
  const outcome = await run({ fetcher: github({ runs: listing([queueRun(), queueRun({ id: 102 })]) }).fetcher });
  assert.deepEqual(reused(outcome), NOTHING);
  assert.match(outcome.reason, /more than one/);
});

test("a listing with more runs than it returned runs everything", async () => {
  const outcome = await run({ fetcher: github({ runs: listing([queueRun()], { total_count: 2 }) }).fetcher });
  assert.deepEqual(reused(outcome), NOTHING);
  assert.match(outcome.reason, /incomplete/);
});

test("a queue run that has not completed is waited for a bounded time, then ignored", async () => {
  const pending = queueRun({ status: "in_progress", conclusion: null });
  let sleeps = 0;
  const outcome = await run({ fetcher: github({ runs: listing([pending]) }).fetcher, sleep: async () => { sleeps += 1; } });
  assert.deepEqual(reused(outcome), NOTHING);
  assert.equal(sleeps, 3);
});

test("a queue run that completes during the wait is used", async () => {
  const states = [queueRun({ status: "in_progress", conclusion: null }), queueRun()];
  const fetcher = async (url) => {
    if (url.includes("/workflows/ci.yml/runs")) return response(200, listing([states.shift() ?? queueRun()]));
    return response(200, ranEverything);
  };
  const outcome = await run({ fetcher });
  assert.deepEqual(reused(outcome), EVERYTHING, outcome.reason);
});

test("anything but a push to master, or an unusable SHA, runs everything", async () => {
  for (const change of [
    { GITHUB_EVENT_NAME: "schedule" }, { GITHUB_EVENT_NAME: "workflow_dispatch" }, { GITHUB_REF: "refs/heads/other" },
    { BEFORE_SHA: "0".repeat(40) }, { BEFORE_SHA: "" }, { GITHUB_SHA: "not-a-sha" }, { GH_TOKEN: "" },
    { GITHUB_REPOSITORY: "not a repository" },
  ]) {
    const { fetcher, calls } = github();
    const outcome = await decide({ env: { ...env, ...change }, fetcher, changedFiles: ordinaryChange, isAncestor: () => true, sleep: async () => {} });
    assert.deepEqual(reused(outcome), NOTHING, JSON.stringify(change));
    assert.equal(calls.length, 0);
  }
});

// The job names above are what a queue run reports for ci.yml's matrix jobs. Derive them from the
// workflow itself, so a renamed job or a changed matrix fails here instead of silently never matching
// (the push would then run everything) or, for a new OS, matching without the new job being judged.
test("the family job names are exactly ci.yml's native, bundle-smoke and seam-control matrix jobs", () => {
  const workflow = readFileSync(new URL("../.github/workflows/ci.yml", import.meta.url), "utf8");
  const block = (job) => {
    const start = workflow.indexOf(`\n  ${job}:\n`);
    assert.notEqual(start, -1, job);
    const next = workflow.slice(start + 1).search(/\n  [a-z][a-z-]*:\n/);
    return workflow.slice(start, next === -1 ? undefined : start + 1 + next);
  };
  const expand = (job) => {
    const text = block(job);
    const label = /^    name: (.+) \(\$\{\{ matrix\.os \}\}\)$/m.exec(text)?.[1];
    const systems = /^        os: \[(.+)\]$/m.exec(text)?.[1].split(",").map((entry) => entry.trim());
    assert.ok(label && systems?.length, `${job} has a matrix name and an os list`);
    return systems.map((system) => `${label} (${system})`);
  };
  assert.deepEqual(FAMILIES.native, expand("native"));
  assert.deepEqual(FAMILIES.bundle, [...expand("bundle-smoke"), ...expand("seam-control")]);
});
