// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { parseDocument } from "yaml";

import { milestoneProblem } from "./check-pr-milestone.mjs";

const script = fileURLToPath(new URL("./check-pr-milestone.mjs", import.meta.url));
const run = (milestone) => spawnSync(process.execPath, [script], { env: { ...process.env, PR_MILESTONE: milestone }, encoding: "utf8" });

test("a named milestone passes", () => {
  assert.equal(milestoneProblem("0.6.0"), null);
  assert.equal(milestoneProblem("Next (after 0.8.0)"), null);
});

test("none, an empty title or a blank one is a problem", () => {
  assert.match(milestoneProblem(null), /needs a milestone/);
  assert.notEqual(milestoneProblem(""), null);
  assert.notEqual(milestoneProblem("   "), null);
  assert.notEqual(milestoneProblem(undefined), null);
});

test("the script exits 0 on a title and 1 on null, and fails closed on unreadable input", () => {
  assert.equal(run('"0.6.0"').status, 0);
  const bad = run("null");
  assert.equal(bad.status, 1);
  assert.match(bad.stderr, /needs a milestone/);
  assert.equal(run('""').status, 1);
  for (const broken of ["", "not json", "7", "[]", '{"title":"0.6.0"}']) {
    assert.notEqual(run(broken).status, 0, `input ${JSON.stringify(broken)} must not pass`);
  }
});

test("the milestone job runs on the milestone events, exempts Dependabot, and passes the title by environment", () => {
  const flow = parseDocument(readFileSync(new URL("../.github/workflows/pr-labels.yml", import.meta.url), "utf8")).toJS();
  for (const event of ["milestoned", "demilestoned", "synchronize", "opened"]) {
    assert.ok(flow.on.pull_request.types.includes(event), event);
  }
  const job = flow.jobs.milestone;
  assert.equal(job.name, "Milestone");
  assert.equal(job.concurrency, undefined, "a job-level group could cancel a run again");
  assert.equal(job.if, "github.event.pull_request.user.login != 'dependabot[bot]'");
  const check = job.steps.find((s) => s.name === "Require a milestone");
  assert.equal(check.if, "github.event_name == 'pull_request'");
  assert.equal(check.run, "node scripts/check-pr-milestone.mjs");
  assert.equal(check.env.PR_MILESTONE, "${{ toJSON(github.event.pull_request.milestone.title) }}");
  assert.equal(check["continue-on-error"], undefined);
  assert.equal(job["continue-on-error"], undefined);
});
