// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import test from "node:test";
import { parseDocument } from "yaml";

import { TYPE_LABELS, typeLabelProblem } from "./check-pr-type-label.mjs";

const script = new URL("./check-pr-type-label.mjs", import.meta.url).pathname;
const run = (labels) => spawnSync(process.execPath, [script], { env: { ...process.env, PR_LABELS: labels }, encoding: "utf8" });

test("exactly one known type label passes", () => {
  for (const label of TYPE_LABELS) assert.equal(typeLabelProblem(["area:infra", label, "documentation"]), null);
});

test("none, two, or an unknown type label is a problem that names what was found", () => {
  assert.match(typeLabelProblem(["area:infra"]), /it has none/);
  assert.match(typeLabelProblem(["type:bug", "type:chore"]), /"type:bug","type:chore"/);
  assert.match(typeLabelProblem(["type:docs"]), /"type:docs"/);
  assert.notEqual(typeLabelProblem([]), null);
});

test("the script exits 0 on a good set and 1 on a bad one, and fails closed on unreadable input", () => {
  assert.equal(run('["type:feature","area:tally"]').status, 0);
  const bad = run('["area:tally"]');
  assert.equal(bad.status, 1);
  assert.match(bad.stderr, /exactly one of/);
  for (const broken of ["", "not json", '{"a":1}', "[1]"]) {
    assert.notEqual(run(broken).status, 0, `input ${JSON.stringify(broken)} must not pass`);
  }
});

test("the workflow runs on pull_request with the label names passed by environment, not spliced into the shell", () => {
  const flow = parseDocument(readFileSync(new URL("../.github/workflows/pr-labels.yml", import.meta.url), "utf8")).toJS();
  assert.deepEqual(Object.keys(flow.on), ["pull_request", "merge_group"], "not pull_request_target");
  assert.deepEqual(flow.on.pull_request.types, ["opened", "reopened", "synchronize", "labeled", "unlabeled"]);
  assert.deepEqual(flow.permissions, { contents: "read" });
  const job = flow.jobs["type-label"];
  assert.equal(job.if, "github.event.pull_request.user.login != 'dependabot[bot]'");
  const check = job.steps.find((s) => s.name === "Require exactly one type label");
  assert.equal(check.run, "node scripts/check-pr-type-label.mjs");
  assert.equal(check.env.PR_LABELS, "${{ toJSON(github.event.pull_request.labels.*.name) }}");
  assert.equal(check["continue-on-error"], undefined);
  assert.equal(job["continue-on-error"], undefined);
});
