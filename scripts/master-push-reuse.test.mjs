// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import test from "node:test";
import { FAMILIES, decide, forcesFullRun, printable, render } from "./master-push-reuse.mjs";

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
  assert.match(calls[1].url, /\/runs\/101\/jobs\?per_page=100$/);
  assert.equal(calls.length, 2);
  for (const { url, options } of calls) {
    assert.match(url, /^https:\/\/api\.github\.com\/repos\/example\/bridge\/actions\//);
    assert.equal(options.method, "GET");
    assert.equal(options.redirect, "error");
    assert.equal(options.headers.Authorization, "Bearer synthetic-test-token");
    assert.ok(options.signal instanceof AbortSignal, "every request is bounded by a timeout");
  }
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
  assert.equal(outcome.code, "job_listing_incomplete");
});

test("no queue run for the commit runs everything", async () => {
  const outcome = await run({ fetcher: github({ runs: listing([]) }).fetcher });
  assert.deepEqual(reused(outcome), NOTHING);
  assert.equal(outcome.code, "no_queue_run");
});

test("a failed or cancelled queue run runs everything, whatever its jobs say", async () => {
  for (const conclusion of ["failure", "cancelled", "timed_out", "skipped", null]) {
    const outcome = await run({ fetcher: github({ runs: listing([queueRun({ conclusion })]) }).fetcher });
    assert.deepEqual(reused(outcome), NOTHING, String(conclusion));
    assert.equal(outcome.code, "queue_run_not_success");
  }
});

test("a queue run for a different commit runs everything", async () => {
  const other = queueRun({ head_sha: "c".repeat(40) });
  const outcome = await run({ fetcher: github({ runs: listing([other]) }).fetcher });
  assert.deepEqual(reused(outcome), NOTHING);
  assert.equal(outcome.code, "no_queue_run");
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
    assert.equal(outcome.code, "lookup_failed");
  });
}

test("a lockfile, manifest, toolchain or .github change runs everything without asking the API", async () => {
  for (const file of [
    "src-tauri/Cargo.lock", "Cargo.toml", "src-tauri/crates/bridge-tally-core/Cargo.toml", "tools/Cargo.lock",
    "rust-toolchain.toml", "rust-toolchain", "scripts/master-push-reuse.mjs", ".cargo/config.toml", "src-tauri/.cargo/config.toml",
    "src-tauri/tauri.conf.json", ".github/workflows/ci.yml", ".github/actions/setup-windows-native/action.yml",
  ]) {
    const { fetcher, calls } = github();
    const outcome = await run({ fetcher }, () => ["docs/a.md", file]);
    assert.deepEqual(reused(outcome), NOTHING, file);
    assert.equal(outcome.code, "forced_full_path", file);
    assert.equal(calls.length, 0, file);
  }
  assert.equal(forcesFullRun(["src/Cargo.toml.md", "docs/Cargo.lockfile", "x.github/a", "src-tauri/tauri.conf.json.md", "docs/cargo/x"]), undefined);
});

test("a push whose before commit is not an ancestor runs everything without asking the API", async () => {
  const { fetcher, calls } = github();
  const asked = [];
  const outcome = await run({ fetcher, isAncestor: (before, sha) => { asked.push([before, sha]); return false; } });
  assert.deepEqual(reused(outcome), NOTHING);
  assert.equal(outcome.code, "not_a_fast_forward");
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
  const { mkdirSync, mkdtempSync, rmSync, writeFileSync } = await import("node:fs");
  const { tmpdir } = await import("node:os");
  const { join } = await import("node:path");
  const dir = mkdtempSync(join(tmpdir(), "reuse-ancestry-"));
  try {
    const git = (...args) => {
      const result = spawnSync("git", ["-c", "user.name=t", "-c", "user.email=t@example.invalid", "-c", "commit.gpgsign=false", "-c", "core.excludesFile=", ...args],
        { cwd: dir, encoding: "utf8", env: { ...process.env, GIT_CONFIG_GLOBAL: "/dev/null", GIT_CONFIG_NOSYSTEM: "1" } });
      assert.equal(result.status, 0, result.stderr);
      return result.stdout.trim();
    };
    git("init", "-q", "-b", "main");
    const commit = (name) => { writeFileSync(join(dir, name), name); git("add", name); git("commit", "-q", "-m", name); return git("rev-parse", "HEAD"); };
    const first = commit("a");
    const second = commit("b");
    git("checkout", "-q", "-b", "other", first);
    const stranger = commit("c");
    const codeFor = async (before, sha) => (await decide({
      env: { ...env, BEFORE_SHA: before, GITHUB_SHA: sha }, cwd: dir, changedFiles: ordinaryChange,
      fetcher: github({ runs: listing([]) }).fetcher, sleep: async () => {},
    })).code;
    // An ancestor gets past the ancestry test (and is then stopped by the empty queue listing).
    assert.equal(await codeFor(first, second), "no_queue_run");
    // A commit on another line of history is refused; one the clone does not hold is an error, not a "no".
    assert.equal(await codeFor(stranger, second), "not_a_fast_forward");
    assert.equal(await codeFor("d".repeat(40), second), "lookup_failed");
    // The real changed-file listing, not an injected one: a lockfile under a non-ASCII directory is
    // quoted by git, with a trailing quote that defeats `Cargo.lock$`, unless asked for NUL-separated names.
    git("checkout", "-q", "main");
    mkdirSync(join(dir, "src-tauri", "\u00e9"), { recursive: true });
    writeFileSync(join(dir, "src-tauri", "\u00e9", "Cargo.lock"), "lock");
    git("add", "-A");
    git("commit", "-q", "-m", "lock");
    const realListing = (await decide({
      env: { ...env, BEFORE_SHA: second, GITHUB_SHA: git("rev-parse", "HEAD") }, cwd: dir,
      fetcher: github({ runs: listing([]) }).fetcher, sleep: async () => {},
    })).code;
    assert.equal(realListing, "forced_full_path");
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("two queue runs for one commit is ambiguous and runs everything", async () => {
  const outcome = await run({ fetcher: github({ runs: listing([queueRun(), queueRun({ id: 102 })]) }).fetcher });
  assert.deepEqual(reused(outcome), NOTHING);
  assert.equal(outcome.code, "ambiguous_queue_run");
});

test("a listing with more runs than it returned runs everything", async () => {
  const outcome = await run({ fetcher: github({ runs: listing([queueRun()], { total_count: 2 }) }).fetcher });
  assert.deepEqual(reused(outcome), NOTHING);
  assert.equal(outcome.code, "run_listing_incomplete");
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

test("a file name cannot add lines to what the script prints", async () => {
  const hostile = ".github/x\nreuse_native=true\nreuse_bundle=true\n::warning::y`";
  const outcome = await run({ fetcher: github().fetcher }, () => [hostile]);
  assert.deepEqual(reused(outcome), NOTHING);
  assert.equal(/[\n`]/.test(outcome.reason), false, "the reason is one line even before it is printed");
  assert.equal(/[\x00-\x1f\x7f`]/.test(printable(hostile)), false);
  assert.equal(printable("a\nb`c\r"), "a b c ");
});

test("a run with no usable id, or a listing that mixes in another commit's run, runs everything", async () => {
  for (const id of [0, -1, 1.5, "101", null]) {
    const outcome = await run({ fetcher: github({ runs: listing([queueRun({ id })]) }).fetcher });
    assert.deepEqual(reused(outcome), NOTHING, String(id));
    assert.equal(outcome.code, "queue_run_id", String(id));
  }
  const mixed = await run({ fetcher: github({ runs: listing([queueRun(), queueRun({ id: 102, head_sha: "c".repeat(40) })]) }).fetcher });
  assert.deepEqual(reused(mixed), NOTHING);
  assert.equal(mixed.code, "ambiguous_queue_run");
});

// A job gated on the native or bundle scope output is skipped on a reused push without the lookup ever
// judging it, and so is a job that `needs:` one of the three. Reading ci.yml's jobs (any id, a multi-line
// `if:`, either form of the output reference, an inline or a block `needs:`), the jobs gated on the
// outputs must be exactly the jobs the families judge, and the only jobs that may need them are the two
// aggregators, which must run under `always()` so a skip cannot cascade into them or pass as a success.
const FAMILY_JOBS = ["native", "bundle-smoke", "seam-control"];
const MAY_NEED_A_FAMILY_JOB = ["compiler-cache-retention", "required-checks"];
function skipHazards(workflow) {
  const jobs = new Map();
  const problems = [];
  // A job written in a form the reader below cannot see (a quoted id, a trailing comment, a flow mapping)
  // would be skipped unseen, so every key at job level must be a plain `  id:` line.
  const jobLevel = workflow.slice(workflow.indexOf("\njobs:\n")).split("\n").filter((line) => /^ {2}\S/.test(line) && !line.startsWith("  #"));
  for (const line of jobLevel) if (!/^ {2}[A-Za-z0-9_-]+:$/.test(line)) problems.push(`a job this reader cannot parse: ${line.trim()}`);
  for (const [, id, body] of workflow.matchAll(/\n  ([A-Za-z0-9_-]+):\n((?:(?!\n  [A-Za-z0-9_-]+:\n)[\s\S])*)/g)) {
    const fields = new Map();
    let key = null;
    for (const line of body.split("\n")) {
      const start = /^    ([A-Za-z0-9_-]+):(.*)$/.exec(line);
      const uncommented = line.replace(/(^|\s)#.*$/, "");
      if (start) { key = start[1]; fields.set(key, start[2].replace(/(^|\s)#.*$/, "")); } else if (key && /^(?:\s{5,}|\s*$)/.test(line)) fields.set(key, `${fields.get(key)}\n${uncommented}`);
      else key = null;
    }
    jobs.set(id, fields);
  }
  const gated = [...jobs].filter(([, fields]) => /outputs(?:\.|\s*\[\s*['"])(?:native|bundle)\b/.test(fields.get("if") ?? "")).map(([id]) => id);
  if (gated.sort().join() !== [...FAMILY_JOBS].sort().join()) problems.push(`jobs gated on the family outputs: ${gated.join(", ")}`);
  for (const [id, fields] of jobs) {
    const needs = (fields.get("needs") ?? "").match(/[A-Za-z0-9_-]+/g) ?? [];
    if (!needs.some((need) => FAMILY_JOBS.includes(need))) continue;
    if (!MAY_NEED_A_FAMILY_JOB.includes(id)) problems.push(`${id} needs a family job and is not an aggregator`);
    else if (!/\balways\(\)/.test(fields.get("if") ?? "")) problems.push(`${id} needs a family job without always()`);
  }
  return problems;
}

test("only the jobs the families judge are gated on the scope outputs, and only aggregators need them", () => {
  const workflow = readFileSync(new URL("../.github/workflows/ci.yml", import.meta.url), "utf8");
  assert.deepEqual(skipHazards(workflow), []);
  const withJob = (job) => `${workflow.trimEnd()}\n${job}\n`;
  const hazards = {
    "a push-only job that needs a family job": "\n  push-only-extra:\n    needs: [bundle-smoke]\n    if: github.event_name == 'push'\n    runs-on: ubuntu-latest\n",
    "the same with a block needs and a digit in the id": "\n  extra2_job:\n    needs:\n      - changes\n      - seam-control\n    runs-on: ubuntu-latest\n",
    "a job gated on an output by a multi-line if": "\n  extra-gated:\n    needs: changes\n    if: >-\n      github.event_name == 'push' ||\n      needs.changes.outputs.native == 'true'\n    runs-on: ubuntu-latest\n",
    "a job gated by the bracket form": "\n  extra-bracket:\n    needs: changes\n    if: needs.changes.outputs['bundle'] == 'true'\n    runs-on: ubuntu-latest\n",
    "a quoted job id": "\n  \"quoted\":\n    needs: [bundle-smoke]\n    runs-on: ubuntu-latest\n",
    "a job id with a trailing comment": "\n  commented: # note\n    needs: [seam-control]\n    runs-on: ubuntu-latest\n",
    "a flow-mapping job": "\n  flow: {needs: native, runs-on: ubuntu-latest}\n",
    "an extra job with always() that is not an allowed aggregator": "\n  extra-always:\n    needs: [native]\n    if: ${{ always() }}\n    runs-on: ubuntu-latest\n",
    "the output through needs['changes']": "\n  extra-index:\n    needs: changes\n    if: needs['changes'].outputs.native == 'true'\n    runs-on: ubuntu-latest\n",
    "an aggregator without always()": null,
  };
  const aggregator = "    if: ${{ always() }}\n    needs: [changes";
  const edited = {
    "required-checks whose always() is only in a trailing comment": workflow.replace(aggregator, "    if: github.event_name == 'push' # always()\n    needs: [changes"),
    "required-checks whose always() is only in a comment line of a block if": workflow.replace(aggregator, "    if: >-\n      github.event_name == 'push'\n      # always()\n    needs: [changes"),
  };
  for (const [name, changed] of Object.entries(edited)) {
    assert.notEqual(changed, workflow, name);
    assert.notDeepEqual(skipHazards(changed), [], name);
  }
  for (const [name, job] of Object.entries(hazards)) {
    const changed = job === null ? workflow.replace("    if: ${{ always() }}\n    needs: [changes", "    needs: [changes") : withJob(job);
    assert.notEqual(changed, workflow, name);
    assert.notDeepEqual(skipHazards(changed), [], name);
  }
});

test("the script prints the decision lines first, in a fixed order, and never swaps the families", () => {
  assert.deepEqual(render({ native: true, bundle: false, code: "evaluated", reason: "r" }),
    ["reuse_native=true", "reuse_bundle=false", "code=evaluated", "reason=r"]);
  assert.deepEqual(render({ native: false, bundle: true, code: "evaluated", reason: "r" }).slice(0, 2),
    ["reuse_native=false", "reuse_bundle=true"]);
});

test("run as a command outside a master push, the script prints exactly four lines and no reuse", async () => {
  const { spawnSync } = await import("node:child_process");
  const result = spawnSync(process.execPath, [fileURLToPath(new URL("./master-push-reuse.mjs", import.meta.url))], {
    encoding: "utf8", env: { PATH: process.env.PATH, GITHUB_EVENT_NAME: "pull_request" },
  });
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stdout, "reuse_native=false\nreuse_bundle=false\ncode=not_a_master_push\nreason=not a push to master\n");
});

test("a commit SHA must be a full-length hex SHA", async () => {
  for (const GITHUB_SHA of ["a".repeat(39), "abcd", "A".repeat(40)]) {
    const outcome = await decide({ env: { ...env, GITHUB_SHA }, fetcher: github().fetcher, changedFiles: ordinaryChange, isAncestor: () => true, sleep: async () => {} });
    assert.equal(outcome.code, "bad_sha", GITHUB_SHA);
  }
});
