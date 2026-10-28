import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

// Runs the real aggregation step of ci.yml's Required checks job under bash, with a synthetic
// `toJSON(needs)`, and reads its exit status and output. The consistency check pins the step's text;
// this proves what the text decides: a job the scope selected must succeed, a job it did not select
// must be skipped, a job without a condition must succeed, and anything the rules cannot judge fails.
const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const workflowPath = join(repoRoot, ".github", "workflows", "ci.yml");

function requiredChecksCommand() {
  const lines = readFileSync(workflowPath, "utf8").split("\n");
  const stepStart = lines.findIndex((line) => line === "      - name: Verify every job that should have run passed");
  assert.notEqual(stepStart, -1, "required-checks aggregation step is missing");

  const stepEndOffset = lines.slice(stepStart + 1).findIndex(
    (line) => /^      - name:/.test(line) || /^  [a-zA-Z0-9_-]+:/.test(line),
  );
  const stepEnd = stepEndOffset === -1 ? lines.length : stepStart + 1 + stepEndOffset;
  const runStart = lines.findIndex(
    (line, index) => index > stepStart && index < stepEnd && line === "        run: |",
  );
  assert.notEqual(runStart, -1, "required-checks aggregation command is missing");

  const commandLines = [];
  for (let index = runStart + 1; index < stepEnd; index += 1) {
    const line = lines[index];
    if (line === "") {
      commandLines.push("");
      continue;
    }
    assert.match(line, /^ {10}/, "required-checks command has unexpected indentation");
    commandLines.push(line.slice(10));
  }
  const command = commandLines.join("\n").trimEnd();
  assert.match(command, /NEEDS_JSON/, "required-checks command must consume NEEDS_JSON");
  return command;
}

// Built from required-checks' own needs, which check-ci-workflow-consistency.mjs pins to every
// other job, so a new job is in every case below without editing this file, and the step fails it
// until the step gives it a rule.
function requiredChecksNeeds() {
  const lines = readFileSync(workflowPath, "utf8").split("\n");
  const job = lines.indexOf("  required-checks:");
  const needs = job === -1 ? undefined : lines.slice(job + 1).find((line) => line.startsWith("    needs: ["));
  assert.ok(needs?.endsWith("]"), "required-checks needs are missing");
  return needs.slice("    needs: [".length, -1).split(", ");
}

// `live` is what a stand-in `gh api repos/<repo>/pulls/<n> --jq .draft` prints (the pull request as it is when this job
// runs); `ghExit` makes it fail instead.
function runRequiredChecks(needs, { live = "true", ghExit = "0" } = {}) {
  const dir = mkdtempSync(join(tmpdir(), "required-checks-"));
  try {
    mkdirSync(join(dir, "bin"));
    writeFileSync(join(dir, "bin", "gh"), `#!/bin/bash\nprintf '%s' "$*" > "$STUB_GH_ARGS"\n[ "$STUB_GH_EXIT" = 0 ] || exit "$STUB_GH_EXIT"\nprintf '%s\\n' "$STUB_LIVE_DRAFT"\n`);
    chmodSync(join(dir, "bin", "gh"), 0o755);
    const result = spawnSync("bash", ["-c", requiredChecksCommand()], {
      cwd: repoRoot,
      encoding: "utf8",
      env: {
        ...process.env, PATH: `${join(dir, "bin")}:${process.env.PATH}`, NEEDS_JSON: JSON.stringify(needs), GITHUB_REPOSITORY: "o/r", PR_NUMBER: "7",
        STUB_GH_ARGS: join(dir, "gh-args"), STUB_GH_EXIT: ghExit, STUB_LIVE_DRAFT: live,
      },
    });
    let ghArgs = null;
    try { ghArgs = readFileSync(join(dir, "gh-args"), "utf8"); } catch { /* gh was never called */ }
    return { status: result.status, output: result.stdout + result.stderr, ghArgs };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

// The jobs gated on a scope output, and the output that gates each.
const SCOPED = { native: "native", "bundle-smoke": "bundle", "seam-control": "bundle", "tax-audit-mutations": "tax_audit" };
const UNCONDITIONAL = ["changes", "frontend", "rust-format", "workflow-consistency", "tally-portable"];
// A run in which every job did what its scope said: selected jobs succeeded, the rest were skipped.
function run(scope, { results = {}, master = false } = {}) {
  const needs = {};
  for (const job of requiredChecksNeeds()) {
    const result = Object.hasOwn(SCOPED, job) ? (scope[SCOPED[job]] === "true" ? "success" : "skipped")
      : job === "compiler-cache-retention" ? (master ? "success" : "skipped") : "success";
    needs[job] = { result, outputs: {} };
  }
  needs.changes.outputs = { draft: "false", ...scope };
  for (const [job, result] of Object.entries(results)) needs[job] = { ...needs[job], result };
  return needs;
}
const NOTHING = { native: "false", bundle: "false", tax_audit: "false" };
const FULL = { native: "true", bundle: "true", tax_audit: "true" };
const passes = (needs, label, options) => {
  const { status, output } = runRequiredChecks(needs, options);
  assert.equal(status, 0, `${label}\n${output}`);
  assert.match(output, /Every job that should have run passed, and every skipped job was not selected/);
};
const fails = (needs, message, label, options) => {
  const { status, output } = runRequiredChecks(needs, options);
  assert.equal(status, 1, `${label}\n${output}`);
  assert.ok(output.includes(message), `${label}: expected "${message}" in\n${output}`);
};

test("every job required-checks needs is classified here, so each case below covers it", () => {
  const classified = new Set([...Object.keys(SCOPED), ...UNCONDITIONAL, "compiler-cache-retention"]);
  assert.deepEqual(requiredChecksNeeds().filter((job) => !classified.has(job)), []);
});

test("a run whose jobs all did what the scope said passes, selected or not", () => {
  passes(run(NOTHING), "a docs-only pull request: native, bundle, seam and tax-audit skipped");
  passes(run(FULL), "a full run");
  passes(run(FULL, { master: true }), "a master run with the cache pruner");
  passes(run({ native: "true", bundle: "false", tax_audit: "false" }), "native only");
});

// The step runs on Linux. Its Node script starts `gh` without a shell, which Windows resolves only to an .exe, so the stand-in
// below (an extensionless script found through PATH) cannot be started there; the other cases never call `gh`.
const noWindowsGh = process.platform === "win32" ? "the stand-in gh is an extensionless script, which Windows does not start without a shell; the step runs on Linux" : false;
test("a draft-scoped run passes only while its pull request is still a draft, read again when this job runs", { skip: noWindowsGh }, () => {
  const draftRun = () => run({ ...NOTHING, draft: "true" });
  passes(draftRun(), "a draft pull request, native, bundle and seam skipped", { live: "true" });
  assert.equal(runRequiredChecks(draftRun()).ghArgs, "api repos/o/r/pulls/7 --jq .draft", "it reads this pull request's draft flag");
  assert.equal(runRequiredChecks(run(FULL)).ghArgs, null, "a run not scoped as a draft never asks");
  for (const live of ["false", "", "TRUE", "null"]) {
    fails(draftRun(), "draft-scoped run, but the pull request is not a draft now", `ready now: ${JSON.stringify(live)}`, { live });
  }
  fails(draftRun(), `draft-scoped run, but the pull request is not a draft now ("unreadable")`, "the lookup failed", { ghExit: "1" });
  // A run that was not scoped as a draft never asks, and an odd value fails.
  passes(run(FULL), "a full run", { ghExit: "1" });
  const odd = run(FULL);
  odd.changes.outputs.draft = "yes";
  fails(odd, `scope output draft is "yes", neither 'true' nor 'false'`, "odd draft output");
});

test("a selected job that was skipped, failed or cancelled fails the aggregator", () => {
  for (const job of Object.keys(SCOPED)) {
    for (const result of ["skipped", "failure", "cancelled"]) {
      fails(run(FULL, { results: { [job]: result } }), `${job}: ${result}, required success`, `${job} ${result}`);
    }
  }
});

test("a job the scope did not select must be skipped: a success there means its condition and the scope disagree", () => {
  fails(run(NOTHING, { results: { native: "success" } }), "native: success, required skipped", "native ran unselected");
  fails(run(NOTHING, { results: { "seam-control": "failure" } }), "seam-control: failure, required skipped", "seam failed unselected");
});

test("a job without a condition must succeed", () => {
  for (const job of UNCONDITIONAL) {
    for (const result of ["skipped", "failure", "cancelled"]) {
      fails(run(FULL, { results: { [job]: result } }), `${job}: ${result}, required success`, `${job} ${result}`);
    }
  }
});

test("the cache pruner may succeed or be skipped, never fail or be cancelled", () => {
  for (const result of ["failure", "cancelled"]) {
    fails(run(FULL, { results: { "compiler-cache-retention": result } }), `compiler-cache-retention: ${result}, required success or skipped`, result);
  }
});

test("what the rules cannot judge fails: a missing or odd scope output, an unknown result, a job without a rule or a missing job", () => {
  const noScope = run(FULL);
  noScope.changes.outputs = {};
  fails(noScope, "native: scope output native is undefined, neither 'true' nor 'false'", "no outputs");
  fails(run({ ...FULL, bundle: "TRUE" }), `bundle-smoke: scope output bundle is "TRUE"`, "odd output");
  fails(run(FULL, { results: { frontend: "neutral" } }), `frontend: result "neutral" is not one of GitHub's four`, "unknown result");
  const extra = run(FULL);
  extra["new-job"] = { result: "success", outputs: {} };
  fails(extra, "new-job: no rule says whether it must run", "a job the rules do not name");
  for (const job of requiredChecksNeeds()) {
    const missing = run(FULL);
    delete missing[job];
    fails(missing, `${job}: result undefined is not one of GitHub's four`, `${job} missing from needs`);
  }
});
