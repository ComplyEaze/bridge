// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import test from "node:test";
import { decide, forcesFullRun } from "./master-push-reuse.mjs";

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

// A GitHub stand-in that answers the two lookups and records every URL it was asked for.
function github({ runs = listing([queueRun()]), jobs = jobsOf([job("Frontend build"), job("Required checks")]), failWith } = {}) {
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
const docsOnly = () => ["docs/release-process.md", "src-tauri/src/agent.rs"];
const run = (overrides = {}, files = docsOnly) =>
  decide({ env, changedFiles: files, sleep: async () => {}, ...overrides });

test("a green queue run for the same commit lets the push reuse it", async () => {
  const { fetcher, calls } = github();
  const outcome = await run({ fetcher });
  assert.equal(outcome.reuse, true, outcome.reason);
  assert.match(calls[0].url, /event=merge_group&head_sha=a{40}&per_page=100/);
  assert.equal(calls[0].options.redirect, "error");
});

test("no queue run for the commit runs everything", async () => {
  const outcome = await run({ fetcher: github({ runs: listing([]) }).fetcher });
  assert.deepEqual(outcome, { reuse: false, reason: "no merge-queue run for this commit" });
});

test("a failed queue run runs everything", async () => {
  const outcome = await run({ fetcher: github({ runs: listing([queueRun({ conclusion: "failure" })]) }).fetcher });
  assert.equal(outcome.reuse, false);
  assert.match(outcome.reason, /concluded failure/);
});

test("a cancelled queue run runs everything", async () => {
  const outcome = await run({ fetcher: github({ runs: listing([queueRun({ conclusion: "cancelled" })]) }).fetcher });
  assert.equal(outcome.reuse, false);
  assert.match(outcome.reason, /concluded cancelled/);
});

test("a queue run for a different commit runs everything", async () => {
  const other = queueRun({ head_sha: "c".repeat(40) });
  const outcome = await run({ fetcher: github({ runs: listing([other]) }).fetcher });
  assert.equal(outcome.reuse, false);
  assert.match(outcome.reason, /no merge-queue run/);
});

test("a run that is not a merge_group run is not a queue run", async () => {
  const outcome = await run({ fetcher: github({ runs: listing([queueRun({ event: "pull_request" })]) }).fetcher });
  assert.equal(outcome.reuse, false);
});

for (const [name, failWith] of [
  ["an API error status", 500],
  ["a missing token scope", 403],
  ["a network failure", new TypeError("fetch failed")],
  ["a timeout", Object.assign(new Error("timed out"), { name: "TimeoutError" })],
]) {
  test(`${name} runs everything`, async () => {
    const outcome = await run({ fetcher: github({ failWith }).fetcher });
    assert.equal(outcome.reuse, false);
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
    assert.equal(outcome.reuse, false, file);
    assert.match(outcome.reason, /always runs everything/, file);
    assert.equal(calls.length, 0, file);
  }
  assert.equal(forcesFullRun(["src/Cargo.toml.md", "docs/Cargo.lockfile", "x.github/a"]), undefined);
});

test("a git failure while listing changed files runs everything", async () => {
  const outcome = await run({ fetcher: github().fetcher }, () => { throw new Error("git diff failed"); });
  assert.equal(outcome.reuse, false);
});

test("two queue runs for one commit is ambiguous and runs everything", async () => {
  const outcome = await run({ fetcher: github({ runs: listing([queueRun(), queueRun({ id: 102 })]) }).fetcher });
  assert.equal(outcome.reuse, false);
  assert.match(outcome.reason, /more than one/);
});

test("a listing with more runs than it returned runs everything", async () => {
  const outcome = await run({ fetcher: github({ runs: listing([queueRun()], { total_count: 2 }) }).fetcher });
  assert.equal(outcome.reuse, false);
  assert.match(outcome.reason, /incomplete/);
});

test("a run whose Required checks job failed, is missing or is repeated runs everything", async () => {
  for (const jobs of [
    jobsOf([job("Required checks", "failure")]),
    jobsOf([job("Required checks", "cancelled")]),
    jobsOf([job("Required checks", null, { status: "in_progress" })]),
    jobsOf([job("Frontend build")]),
    jobsOf([job("Required checks"), job("Required checks")]),
    jobsOf([job("Required checks")], { total_count: 101 }),
  ]) {
    const outcome = await run({ fetcher: github({ jobs }).fetcher });
    assert.equal(outcome.reuse, false, JSON.stringify(jobs));
  }
});

test("a queue run that has not completed is waited for a bounded time, then ignored", async () => {
  const pending = queueRun({ status: "in_progress", conclusion: null });
  let sleeps = 0;
  const outcome = await run({ fetcher: github({ runs: listing([pending]) }).fetcher, sleep: async () => { sleeps += 1; } });
  assert.equal(outcome.reuse, false);
  assert.equal(sleeps, 3);
});

test("a queue run that completes during the wait is used", async () => {
  const states = [queueRun({ status: "in_progress", conclusion: null }), queueRun()];
  const fetcher = async (url) => {
    if (url.includes("/workflows/ci.yml/runs")) return response(200, listing([states.shift() ?? queueRun()]));
    return response(200, jobsOf([job("Required checks")]));
  };
  const outcome = await run({ fetcher });
  assert.equal(outcome.reuse, true, outcome.reason);
});

test("anything but a push to master, or an unusable SHA, runs everything", async () => {
  for (const change of [
    { GITHUB_EVENT_NAME: "schedule" }, { GITHUB_EVENT_NAME: "workflow_dispatch" }, { GITHUB_REF: "refs/heads/other" },
    { BEFORE_SHA: "0".repeat(40) }, { BEFORE_SHA: "" }, { GITHUB_SHA: "not-a-sha" }, { GH_TOKEN: "" },
    { GITHUB_REPOSITORY: "not a repository" },
  ]) {
    const { fetcher, calls } = github();
    const outcome = await decide({ env: { ...env, ...change }, fetcher, changedFiles: docsOnly, sleep: async () => {} });
    assert.equal(outcome.reuse, false, JSON.stringify(change));
    assert.equal(calls.length, 0);
  }
});
