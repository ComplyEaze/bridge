// SPDX-License-Identifier: Apache-2.0
//
// Decides whether a push to master may skip the heavy CI jobs because the merge queue already ran
// them on this exact commit. The scope job in ci.yml runs it and reads one line from stdout:
// `reuse=true` or `reuse=false`, followed by `reason=...`.
//
// It FAILS OPEN. `reuse=true` is printed only when every condition below was positively
// established; any error, timeout, missing field, ambiguity or surprise prints `reuse=false`, which
// is today's behaviour (every job runs). Nothing here can make a check run less except by finding a
// green, completed `merge_group` run of this same workflow for this same commit SHA.
//
// Conditions for `reuse=true`, all required:
//   1. the push is not a forced or unverifiable one: `before` is a usable ancestor of the commit;
//   2. the push changes none of Cargo.lock, a Cargo.toml, the toolchain file or anything under
//      .github/, so caches stay warm and a workflow change is always exercised;
//   3. exactly one `merge_group` run of ci.yml has this head SHA, it is completed with conclusion
//      `success`, and the API's total_count matches what it returned (no unseen second page);
//   4. that run's `Required checks` job is present exactly once and concluded `success`, and the
//      job listing is complete.
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";

const SHA = /^[0-9a-f]{40,64}$/;
const FORCE_FULL = /^(?:\.github\/|rust-toolchain(?:\.toml)?$|(?:.*\/)?Cargo\.(?:lock|toml)$)/;
const WORKFLOW_FILE = "ci.yml";
const REQUIRED_JOB = "Required checks";
// A completed queue run can lag the landing push by a few seconds; wait a bounded time for it.
const ATTEMPTS = 4;
const RETRY_MS = 15_000;
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

export const reuse = (reason) => ({ reuse: true, reason });
export const full = (reason) => ({ reuse: false, reason });

export function forcesFullRun(changedFiles) {
  return changedFiles.find((file) => FORCE_FULL.test(file));
}

function changedFilesOf(before, sha, cwd) {
  const result = spawnSync("git", ["diff", "--name-only", "--no-renames", "-z", before, sha], {
    cwd, encoding: "utf8", maxBuffer: 64 * 1024 * 1024, windowsHide: true,
  });
  if (result.error || result.status !== 0) throw new Error("git diff failed");
  return result.stdout.split("\0").filter(Boolean);
}

export async function decide({
  env = process.env, fetcher = fetch, changedFiles = changedFilesOf, sleep = wait, cwd = process.cwd(),
} = {}) {
  try {
    return await decideUnchecked({ env, fetcher, changedFiles, sleep, cwd });
  } catch (error) {
    return full(`lookup failed (${error?.status ?? error?.name ?? "error"}): running everything`);
  }
}

async function decideUnchecked({ env, fetcher, changedFiles, sleep, cwd }) {
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
  const required = jobs.filter((job) => job?.name === REQUIRED_JOB);
  if (required.length !== 1) return full("the merge-queue run has no single Required checks job");
  if (required[0].status !== "completed" || required[0].conclusion !== "success") {
    return full("the merge-queue run's Required checks job did not succeed");
  }
  return reuse(`merge-queue run ${run.id} passed Required checks on this commit`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const outcome = await decide();
  console.log(`reuse=${outcome.reuse}`);
  console.log(`reason=${outcome.reason}`);
}
