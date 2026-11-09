// SPDX-License-Identifier: Apache-2.0
//
// Runs the real step of .github/workflows/ci-failure-alert.yml under bash with a stand-in `gh`, once per kind of
// finished run, and reads the calls it made: a failed master push, nightly or manual run opens or adds to one
// tracking issue, a passing run only comments on an open one, any other conclusion changes nothing.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";

const workflow = readFileSync(new URL("../.github/workflows/ci-failure-alert.yml", import.meta.url), "utf8").split("\n");
function step() {
  const start = workflow.findIndex((line) => line.startsWith("      - name: Open, update or add to the tracking issue"));
  const run = workflow.findIndex((line, index) => index > start && line === "        run: |");
  assert.ok(start !== -1 && run !== -1, "the alert step is where this test expects it");
  const lines = [];
  for (let index = run + 1; index < workflow.length; index += 1) {
    if (workflow[index] !== "" && !workflow[index].startsWith("          ")) break;
    lines.push(workflow[index].slice(10));
  }
  return lines.join("\n");
}

// The stand-in `gh` hands the step canned JSON and applies the step's own `--jq` filter with the real jq, so a filter that
// selects the wrong jobs, or the wrong issue, shows. `open` is the open issues' numbers, `jobs` the finished run's jobs as
// [name, conclusion] pairs, `last` the body of the open issue's latest comment, and `runViewFails` makes the job listing fail.
function run(conclusion, { open = [], jobs = [["Frontend build", "success"], ["Native checks (macos-latest, shard 1 of 2)", "failure"], ["Native checks (windows-latest, shard 1 of 2)", "success"]], last = null, runViewFails = false } = {}) {
  const dir = mkdtempSync(join(tmpdir(), "ci-alert-"));
  try {
    writeFileSync(join(dir, "open.json"), JSON.stringify(open.map((number) => ({ number }))));
    writeFileSync(join(dir, "jobs.json"), JSON.stringify({ jobs: jobs.map(([name, conclusion]) => ({ name, conclusion })) }));
    writeFileSync(join(dir, "issue.json"), JSON.stringify({ comments: last === null ? [] : [{ body: "an earlier comment" }, { body: last }] }));
    writeFileSync(join(dir, "gh"), `#!/bin/bash
printf '%s\\n' "$*" >> "$STUB_CALLS"
filter() { while [ $# -gt 0 ]; do [ "$1" = "--jq" ] && { printf '%s' "$2"; return; }; shift; done; }
if [ "$1" = "issue" ] && [ "$2" = "list" ]; then jq -r "$(filter "$@")" "$STUB_DIR/open.json"; fi
if [ "$1" = "issue" ] && [ "$2" = "view" ]; then jq -r "$(filter "$@")" "$STUB_DIR/issue.json"; fi
if [ "$1" = "run" ] && [ "$2" = "view" ]; then [ "$STUB_RUN_VIEW_FAILS" = 1 ] && exit 1; jq -r "$(filter "$@")" "$STUB_DIR/jobs.json"; fi
if [ "$1" = "issue" ] && { [ "$2" = "create" ] || [ "$2" = "comment" ]; }; then
  while [ $# -gt 0 ]; do [ "$1" = "--body-file" ] && cp "$2" "$STUB_BODY"; shift; done
fi
`);
    chmodSync(join(dir, "gh"), 0o755);
    writeFileSync(join(dir, "step.sh"), step());
    const result = spawnSync("bash", ["step.sh"], {
      cwd: dir,
      encoding: "utf8",
      env: {
        PATH: `${dir}:${process.env.PATH}`, HOME: dir, STUB_DIR: dir, STUB_CALLS: join(dir, "calls"), STUB_BODY: join(dir, "body-sent"), STUB_RUN_VIEW_FAILS: runViewFails ? "1" : "0",
        CONCLUSION: conclusion, RUN_ID: "123", RUN_URL: "https://example.invalid/run/123", RUN_EVENT: "push", HEAD_SHA: "a".repeat(40),
      },
    });
    const read = (name) => (existsSync(join(dir, name)) ? readFileSync(join(dir, name), "utf8") : "");
    return { status: result.status, stderr: result.stderr, calls: read("calls").split("\n").filter(Boolean), body: read("body-sent") };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}
const issueCalls = (calls) => calls.filter((call) => call.startsWith("issue create") || call.startsWith("issue comment") || call.startsWith("issue close") || call.startsWith("issue edit"));

test("a failed run with no open tracking issue creates the label and opens one, with only the jobs that did not pass", () => {
  for (const conclusion of ["failure", "timed_out", "startup_failure"]) {
    const { status, calls, body, stderr } = run(conclusion);
    assert.equal(status, 0, stderr);
    assert.deepEqual(issueCalls(calls), ["issue create --label ci-failure --label area:infra --label severity:p2 --label type:bug --title Bridge CI failed on master --body-file body.md"], conclusion);
    assert.ok(calls.some((call) => call.startsWith("label create ci-failure ") && call.endsWith("--force")), "the label is created or refreshed");
    assert.match(body, /Run: https:\/\/example\.invalid\/run\/123/);
    assert.match(body, /push run, commit a{40}/);
    assert.match(body, /- Native checks \(macos-latest, shard 1 of 2\)/);
    assert.ok(!/Frontend build|windows-latest/.test(body), "a job that passed is not listed");
  }
});

test("a failed run adds a comment to the oldest open tracking issue instead of opening another", () => {
  const { status, calls } = run("failure", { open: [9, 7, 12] });
  assert.equal(status, 0);
  assert.deepEqual(issueCalls(calls), ["issue comment 7 --body-file body.md"]);
  assert.ok(!calls.some((call) => call.startsWith("label create")), "no label work when an issue is already open");
});

test("a failed run still reports when its job listing cannot be read, and when none failed", () => {
  for (const options of [{ runViewFails: true }, { jobs: [["Frontend build", "success"]] }]) {
    const { status, calls, body, stderr } = run("failure", options);
    assert.equal(status, 0, stderr);
    assert.equal(issueCalls(calls).length, 1);
    assert.match(body, /- none listed; see the run/);
  }
});

test("a passing run comments once per recovery on an open tracking issue, never closes it, and does nothing without one", () => {
  const first = run("success", { open: [7], last: "Run: https://example.invalid/run/9" });
  assert.equal(first.status, 0);
  assert.equal(issueCalls(first.calls).length, 1);
  assert.match(issueCalls(first.calls)[0], /^issue comment 7 --body Bridge CI passed again on master at a{40}\./);
  const noComments = run("success", { open: [7] });
  assert.equal(issueCalls(noComments.calls).length, 1);
  const again = run("success", { open: [7], last: "Bridge CI passed again on master at bbbb. https://example.invalid/run/9" });
  assert.deepEqual(issueCalls(again.calls), [], "a second passing push adds nothing");
  assert.deepEqual(issueCalls(run("success").calls), []);
  assert.ok(![...first.calls, ...again.calls].some((call) => call.startsWith("issue close")));
});

test("a cancelled, skipped or unknown conclusion changes nothing and calls nothing", () => {
  for (const conclusion of ["cancelled", "skipped", "neutral", "action_required", "stale", ""]) {
    const { status, calls } = run(conclusion, { open: [7] });
    assert.equal(status, 0, conclusion);
    assert.deepEqual(calls, [], conclusion);
  }
});

test("the trigger names the workflow it watches, and only this repository's master push and nightly runs reach the job", () => {
  const text = workflow.join("\n");
  const ci = readFileSync(new URL("../.github/workflows/ci.yml", import.meta.url), "utf8");
  const watched = /^ {4}workflows: \["([^"]+)"\]$/m.exec(text)?.[1];
  assert.equal(/^name: (.+)$/m.exec(ci)?.[1], watched, "ci.yml is renamed: this workflow would go silent");
  assert.match(text, /^ {4}if: >-\n {6}github\.event\.workflow_run\.head_repository\.full_name == github\.repository &&\n {6}github\.event\.workflow_run\.head_branch == github\.event\.repository\.default_branch &&\n {6}\(github\.event\.workflow_run\.event == 'push' \|\|\n {6}github\.event\.workflow_run\.event == 'schedule'\)$/m);
});

test("the job holds exactly the two permissions it needs, takes its fields through the environment, and checks out nothing", () => {
  const text = workflow.join("\n");
  assert.match(text, /^permissions: \{\}$/m);
  assert.match(text, /^ {4}permissions:\n {6}actions: read\n {6}issues: write\n {4}steps:$/m);
  for (const name of ["GH_TOKEN: ${{ github.token }}", "GH_REPO: ${{ github.repository }}", "CONCLUSION: ${{ github.event.workflow_run.conclusion }}", "RUN_ID: ${{ github.event.workflow_run.id }}", "RUN_URL: ${{ github.event.workflow_run.html_url }}", "RUN_EVENT: ${{ github.event.workflow_run.event }}", "HEAD_SHA: ${{ github.event.workflow_run.head_sha }}"]) {
    assert.ok(text.includes(`          ${name}\n`), name);
  }
  assert.ok(step().startsWith("set -euo pipefail\n"));
  assert.ok(!step().includes("${{"), "no expression is spliced into the script");
  assert.ok(!/actions\/checkout|uses:/.test(text), "the workflow checks out and runs nothing of the repository");
  assert.ok(!/^concurrency:/m.test(text), "a pending alert is never dropped for a newer one");
});
