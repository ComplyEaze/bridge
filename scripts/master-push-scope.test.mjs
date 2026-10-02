// SPDX-License-Identifier: Apache-2.0
//
// Runs the real `Determine bundle scope` step of ci.yml under bash, once per event, with a stand-in for
// the reuse script, and reads what it wrote to $GITHUB_OUTPUT. The consistency check pins the step's
// text; this proves what the text does: that only an exact `reuse_<family>=true` line from a script
// that exited zero ever turns a heavy job off on a push, and that no other event is changed.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";

const workflow = readFileSync(new URL("../.github/workflows/ci.yml", import.meta.url), "utf8").split("\n");
function scopeScript() {
  const start = workflow.indexOf("      - id: scope");
  const run = workflow.indexOf("        run: |", start);
  const end = workflow.findIndex((line, index) => index > run && /^  [a-z][a-z-]*:$/.test(line));
  assert.ok(start !== -1 && run !== -1 && end !== -1, "the scope step is where this test expects it");
  return workflow.slice(run + 1, end).map((line) => line.slice(10)).join("\n");
}

// What a stand-in `node scripts/master-push-reuse.mjs` prints, and how it exits.
const stand = {
  both: ["reuse_native=true\nreuse_bundle=true\nreason=stub\n", 0],
  nativeOnly: ["reuse_native=true\nreuse_bundle=false\nreason=stub\n", 0],
  bundleOnly: ["reuse_native=false\nreuse_bundle=true\nreason=stub\n", 0],
  neither: ["reuse_native=false\nreuse_bundle=false\nreason=stub\n", 0],
  crashedAfterTrue: ["reuse_native=true\nreuse_bundle=true\n", 3],
  lookalikes: [" reuse_native=true\nreuse_native=true \nxreuse_bundle=true\nreason=reuse_native=true\nreuse_native=truefalse\nreuse_bundle=TRUE\n", 0],
  silent: ["", 0],
};

function runScope(event, { script = "silent", changed = [] } = {}) {
  const dir = mkdtempSync(join(tmpdir(), "scope-step-"));
  try {
    const sh = (command) => {
      const result = spawnSync("bash", ["-c", command], { cwd: dir, encoding: "utf8" });
      assert.equal(result.status, 0, result.stderr);
      return result.stdout.trim();
    };
    const commit = (message) => sh(`git -c user.name=t -c user.email=t@example.invalid -c commit.gpgsign=false commit -q --allow-empty -m ${message} && git rev-parse HEAD`);
    sh("git init -q -b main");
    const base = commit("base");
    for (const file of changed) {
      mkdirSync(join(dir, file, ".."), { recursive: true });
      writeFileSync(join(dir, file), "x");
    }
    sh("git add -A");
    const head = commit("change");
    mkdirSync(join(dir, "bin"));
    const [printed, exitCode] = stand[script];
    writeFileSync(join(dir, "bin", "node"), `#!/bin/bash\ncat <<'EOF'\n${printed}EOF\nexit ${exitCode}\n`);
    chmodSync(join(dir, "bin", "node"), 0o755);
    writeFileSync(join(dir, "scope.sh"), scopeScript());
    const outputFile = join(dir, "out");
    const summaryFile = join(dir, "summary");
    writeFileSync(outputFile, "");
    writeFileSync(summaryFile, "");
    const result = spawnSync("bash", ["scope.sh"], {
      cwd: dir, encoding: "utf8",
      env: {
        PATH: `${join(dir, "bin")}:${process.env.PATH}`, HOME: dir,
        GITHUB_OUTPUT: outputFile, GITHUB_STEP_SUMMARY: summaryFile, GITHUB_SHA: head, GH_TOKEN: "synthetic",
        EVENT_NAME: event, BEFORE_SHA: base, PR_BASE_SHA: base, MERGE_GROUP_BASE_SHA: base,
      },
    });
    assert.equal(result.status, 0, result.stderr);
    const outputs = Object.fromEntries(readFileSync(outputFile, "utf8").trim().split("\n").map((line) => line.split("=")));
    return { outputs, summary: readFileSync(summaryFile, "utf8") };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}
const heavy = (outputs) => ({ native: outputs.native, bundle: outputs.bundle, tax_audit: outputs.tax_audit });
const ALL = { native: "true", bundle: "true", tax_audit: "true" };

test("a push skips a family only on an exact reuse line from a script that exited zero", () => {
  assert.deepEqual(heavy(runScope("push", { script: "both" }).outputs), { native: "false", bundle: "false", tax_audit: "true" });
  assert.deepEqual(heavy(runScope("push", { script: "nativeOnly" }).outputs), { native: "false", bundle: "true", tax_audit: "true" });
  assert.deepEqual(heavy(runScope("push", { script: "bundleOnly" }).outputs), { native: "true", bundle: "false", tax_audit: "true" });
});

test("a push runs everything when the script says no, says nothing, crashes, or prints look-alikes", () => {
  for (const script of ["neither", "silent", "crashedAfterTrue", "lookalikes"]) {
    assert.deepEqual(heavy(runScope("push", { script }).outputs), ALL, script);
  }
});

test("a push records the decision in the step summary", () => {
  assert.match(runScope("push", { script: "both" }).summary, /reuse_native=true\nreuse_bundle=true/);
});

test("a scheduled or manual run is a full run and never asks the script", () => {
  for (const event of ["schedule", "workflow_dispatch"]) {
    const { outputs, summary } = runScope(event, { script: "both" });
    assert.deepEqual(heavy(outputs), ALL, event);
    assert.equal(summary, "", event);
  }
});

test("a pull request or merge group is scoped by its diff, as before, whatever the script says", () => {
  for (const event of ["pull_request", "merge_group"]) {
    assert.deepEqual(heavy(runScope(event, { script: "both", changed: ["docs/a.md"] }).outputs),
      { native: "false", bundle: "false", tax_audit: "false" }, `${event} docs only`);
    assert.deepEqual(heavy(runScope(event, { script: "both", changed: ["src-tauri/src/x.rs"] }).outputs),
      { native: "true", bundle: "true", tax_audit: "false" }, `${event} native code`);
  }
});
