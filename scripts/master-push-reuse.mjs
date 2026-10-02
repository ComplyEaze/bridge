// SPDX-License-Identifier: Apache-2.0
//
// Decides, per family of heavy CI jobs, whether a push to master may skip that family because the
// merge queue already RAN it, and it passed, on this exact commit. The scope job in ci.yml runs it
// and reads exact lines from stdout: `reuse_native=true|false`, `reuse_bundle=true|false`, then
// `reason=...`.
//
// The evidence is positive and per job. The queue scopes its run like a pull request does, so a
// docs-only commit gets a green `Required checks` with every heavy job SKIPPED; that green says
// nothing about the heavy jobs. A family is reused only when every one of its jobs, named below, is
// present exactly once in the queue's run and concluded `success`. A skipped job is not that: the
// push then runs the family itself, as it always did.
//
// It FAILS OPEN. Any error, timeout, missing field, ambiguity or surprise prints `false` for both
// families, which is today's behaviour (every job runs).
//
// Conditions for any `true`, all required:
//   1. the push is an ordinary fast-forward: `before` is an ancestor of the commit, so
//      `before..commit` really is what the push changed;
//   2. that push changes none of Cargo.lock, a Cargo.toml, the toolchain file or anything under
//      .github/, so caches stay warm and a workflow change is always exercised in full;
//   3. exactly one `merge_group` run of ci.yml has this head SHA, it is completed with conclusion
//      `success`, and the API's total_count matches what it returned (no unseen second page);
//   4. that run's job listing is complete, and each job of the family is present once, `success`.
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";

const SHA = /^[0-9a-f]{40,64}$/;
const FORCE_FULL = /^(?:\.github\/|rust-toolchain(?:\.toml)?$|(?:.*\/)?Cargo\.(?:lock|toml)$)/;
const WORKFLOW_FILE = "ci.yml";
// The job names a queue run reports when the job ran: ci.yml's `name:` with the matrix expanded.
// scripts/master-push-reuse.test.mjs derives them from ci.yml and fails if these lists drift.
export const FAMILIES = {
  native: ["Native checks (windows-latest)", "Native checks (macos-latest)"],
  bundle: [
    "Bundle smoke (windows-latest)", "Bundle smoke (macos-latest)",
    "Seam positive control (windows-latest)", "Seam positive control (macos-latest)",
  ],
};
// A completed queue run can lag the landing push by a few seconds; wait a bounded time for it.
const ATTEMPTS = 4;
const RETRY_MS = 15_000;
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

const outcome = (native, bundle, reason) => ({ native, bundle, reason });
const full = (reason) => outcome(false, false, reason);

export function forcesFullRun(changedFiles) {
  return changedFiles.find((file) => FORCE_FULL.test(file));
}

function git(args, cwd) {
  return spawnSync("git", args, { cwd, encoding: "utf8", maxBuffer: 64 * 1024 * 1024, windowsHide: true });
}

function changedFilesOf(before, sha, cwd) {
  const result = git(["diff", "--name-only", "--no-renames", "-z", before, sha], cwd);
  if (result.error || result.status !== 0) throw new Error("git diff failed");
  return result.stdout.split("\0").filter(Boolean);
}

// `merge-base --is-ancestor` exits 0 for an ancestor and 1 for a non-ancestor; any other result is an error.
function isAncestorOf(before, sha, cwd) {
  const result = git(["merge-base", "--is-ancestor", before, sha], cwd);
  if (result.error || (result.status !== 0 && result.status !== 1)) throw new Error("git merge-base failed");
  return result.status === 0;
}

export async function decide({
  env = process.env, fetcher = fetch, changedFiles = changedFilesOf, isAncestor = isAncestorOf,
  sleep = wait, cwd = process.cwd(),
} = {}) {
  try {
    return await decideUnchecked({ env, fetcher, changedFiles, isAncestor, sleep, cwd });
  } catch (error) {
    return full(`lookup failed (${error?.status ?? error?.name ?? "error"}): running everything`);
  }
}

async function decideUnchecked({ env, fetcher, changedFiles, isAncestor, sleep, cwd }) {
  if (env.GITHUB_EVENT_NAME !== "push" || env.GITHUB_REF !== "refs/heads/master") {
    return full("not a push to master");
  }
  const sha = env.GITHUB_SHA ?? "";
  const before = env.BEFORE_SHA ?? "";
  if (!SHA.test(sha) || !SHA.test(before) || /^0+$/.test(before)) {
    return full("the push has no usable commit or before SHA");
  }
  if (!/^[\w.-]+\/[\w.-]+$/.test(env.GITHUB_REPOSITORY ?? "") || !env.GH_TOKEN) {
    return full("missing repository or token");
  }
  if (!isAncestor(before, sha, cwd)) return full("the push is not a fast-forward of its before commit");
  const forcing = forcesFullRun(changedFiles(before, sha, cwd));
  if (forcing) return full(`the push changes ${forcing}, which always runs everything`);

  const base = `https://api.github.com/repos/${env.GITHUB_REPOSITORY}/actions`;
  async function get(path) {
    const response = await fetcher(`${base}${path}`, {
      method: "GET", redirect: "error", signal: AbortSignal.timeout(30_000),
      headers: { Authorization: `Bearer ${env.GH_TOKEN}`, Accept: "application/vnd.github+json" },
    });
    if (!response.ok) throw Object.assign(new Error("GitHub API request failed"), { status: response.status });
    return response.json();
  }

  let run;
  for (let attempt = 1; attempt <= ATTEMPTS; attempt += 1) {
    const listing = await get(`/workflows/${WORKFLOW_FILE}/runs?event=merge_group&head_sha=${sha}&per_page=100`);
    const runs = listing?.workflow_runs;
    if (!Array.isArray(runs) || listing.total_count !== runs.length) {
      return full("the run listing is incomplete or malformed");
    }
    const matching = runs.filter((candidate) => candidate?.head_sha === sha && candidate?.event === "merge_group");
    if (matching.length === 0) return full("no merge-queue run for this commit");
    if (matching.length > 1 || matching.length !== runs.length) return full("more than one merge-queue run for this commit");
    run = matching[0];
    if (run.status === "completed") break;
    if (attempt === ATTEMPTS) return full("the merge-queue run had not completed");
    await sleep(RETRY_MS);
  }
  if (run.conclusion !== "success") return full(`the merge-queue run concluded ${run.conclusion}`);
  if (!Number.isSafeInteger(run.id) || run.id <= 0) return full("the merge-queue run has no usable id");

  const jobsListing = await get(`/runs/${run.id}/jobs?per_page=100`);
  const jobs = jobsListing?.jobs;
  if (!Array.isArray(jobs) || jobsListing.total_count !== jobs.length) {
    return full("the job listing is incomplete or malformed");
  }
  const passed = (name) => {
    const named = jobs.filter((job) => job?.name === name);
    return named.length === 1 && named[0].status === "completed" && named[0].conclusion === "success";
  };
  const native = FAMILIES.native.every(passed);
  const bundle = FAMILIES.bundle.every(passed);
  return outcome(native, bundle, `merge-queue run ${run.id}: native jobs ${native ? "passed" : "did not all run and pass"}, `
    + `bundle and seam jobs ${bundle ? "passed" : "did not all run and pass"}`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const result = await decide();
  console.log(`reuse_native=${result.native}`);
  console.log(`reuse_bundle=${result.bundle}`);
  console.log(`reason=${result.reason}`);
}
