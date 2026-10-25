// SPDX-License-Identifier: Apache-2.0
//
// Runs the real `Determine bundle scope` step of ci.yml under bash, once per event, with a stand-in for
// the reuse script, and reads what it wrote to $GITHUB_OUTPUT. The consistency check pins the step's
// text; this proves what the text does: that only an exact `reuse_<family>=true` line from a script
// that exited zero ever turns a heavy job off on a push, and that no other event is changed.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { FAMILIES } from "./master-push-reuse.mjs";

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
  injectedAfterTheDecision: ["reuse_native=false\nreuse_bundle=false\nreason=a file name\nreuse_native=true\nreuse_bundle=true\n", 0],
  silent: ["", 0],
};

function runScope(event, { script = "silent", changed = [], draft = "", env = {} } = {}) {
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
    writeFileSync(join(dir, "bin", "node"), `#!/bin/bash\nprintf '%s' "$1" > "$STUB_ARGS"\ncat <<'EOF'\n${printed}EOF\nexit ${exitCode}\n`);
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
        GITHUB_OUTPUT: outputFile, GITHUB_STEP_SUMMARY: summaryFile, STUB_ARGS: join(dir, "args"), GITHUB_SHA: head, GH_TOKEN: "synthetic",
        EVENT_NAME: event, BEFORE_SHA: base, PR_BASE_SHA: base, PR_DRAFT: draft, MERGE_GROUP_BASE_SHA: base, ...env,
      },
    });
    assert.equal(result.status, 0, result.stderr);
    const lines = readFileSync(outputFile, "utf8").trim().split("\n").map((line) => [line.slice(0, line.indexOf("=")), line.slice(line.indexOf("=") + 1)]);
    assert.equal(new Set(lines.map(([key]) => key)).size, lines.length, "every output is written once");
    const outputs = Object.fromEntries(lines);
    const args = existsSync(join(dir, "args")) ? readFileSync(join(dir, "args"), "utf8") : null;
    return { outputs, summary: readFileSync(summaryFile, "utf8"), args };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}
const heavy = (outputs) => ({ native: outputs.native, bundle: outputs.bundle, tax_audit: outputs.tax_audit });
const ALL = { native: "true", bundle: "true", tax_audit: "true" };
// The systems each matrix runs on, for the jobs the scope selected (an unselected job's list is never read).
const systems = (outputs) => ({
  native: outputs.native === "true" ? JSON.parse(outputs.native_os) : null,
  bundle: outputs.bundle === "true" ? JSON.parse(outputs.bundle_os) : null,
});
const WIN = ["windows-latest"];
const MAC = ["macos-latest"];
const BOTH = ["windows-latest", "macos-latest"];

test("a push skips a family's Windows jobs only on an exact reuse line from a script that exited zero, and always runs the macOS native tests", () => {
  assert.deepEqual(systems(runScope("push", { script: "both" }).outputs), { native: MAC, bundle: null });
  assert.deepEqual(systems(runScope("push", { script: "nativeOnly" }).outputs), { native: MAC, bundle: WIN });
  assert.deepEqual(systems(runScope("push", { script: "bundleOnly" }).outputs), { native: BOTH, bundle: null });
  for (const script of ["both", "nativeOnly", "bundleOnly"]) assert.equal(runScope("push", { script }).outputs.tax_audit, "true", script);
});

test("a push runs everything on Windows when the script says no, says nothing, crashes, or prints look-alikes", () => {
  for (const script of ["neither", "silent", "crashedAfterTrue", "lookalikes", "injectedAfterTheDecision"]) {
    const { outputs } = runScope("push", { script });
    assert.deepEqual(heavy(outputs), ALL, script);
    assert.deepEqual(systems(outputs), { native: BOTH, bundle: WIN }, script);
  }
});

test("a push runs the macOS bundle and seam jobs when it changes a packaging path, or when its before commit cannot be compared", () => {
  for (const changed of [["src-tauri/tauri.conf.json"], ["packaging/mcpb/manifest.json"], ["LICENSE"], ["src-tauri/icons/icon.png"], [".github/workflows/ci.yml"]]) {
    assert.deepEqual(systems(runScope("push", { script: "both", changed }).outputs), { native: MAC, bundle: MAC }, changed[0]);
    assert.deepEqual(systems(runScope("push", { script: "neither", changed }).outputs), { native: BOTH, bundle: BOTH }, changed[0]);
  }
  for (const changed of [["src-tauri/src/x.rs"], ["src/App.tsx"], ["docs/a.md"], ["src-tauri/Cargo.lock"], ["scripts/check-windows-bundle-resources.ps1"]]) {
    assert.deepEqual(systems(runScope("push", { script: "neither", changed }).outputs), { native: BOTH, bundle: WIN }, changed[0]);
  }
  for (const BEFORE_SHA of ["", "0".repeat(40), "f".repeat(40)]) {
    assert.deepEqual(systems(runScope("push", { script: "both", env: { BEFORE_SHA } }).outputs), { native: MAC, bundle: MAC }, `before ${BEFORE_SHA}`);
  }
});

test("a pull request and the merge queue run the native tests on Windows only; the queue adds the macOS bundle and seam jobs for a packaging change", () => {
  const nativeAndBundle = ["src-tauri/src/x.rs"];
  const packaging = ["src-tauri/src/x.rs", "src-tauri/tauri.conf.json"];
  assert.deepEqual(systems(runScope("pull_request", { changed: nativeAndBundle }).outputs), { native: WIN, bundle: WIN });
  assert.deepEqual(systems(runScope("pull_request", { changed: packaging }).outputs), { native: WIN, bundle: WIN });
  assert.deepEqual(systems(runScope("merge_group", { changed: nativeAndBundle }).outputs), { native: WIN, bundle: WIN });
  assert.deepEqual(systems(runScope("merge_group", { changed: packaging }).outputs), { native: WIN, bundle: BOTH });
  // A missing base runs every job, still without macOS native tests; the queue then assumes a packaging change.
  assert.deepEqual(systems(runScope("pull_request", { env: { PR_BASE_SHA: "" } }).outputs), { native: WIN, bundle: WIN });
  assert.deepEqual(systems(runScope("merge_group", { env: { MERGE_GROUP_BASE_SHA: "0".repeat(40) } }).outputs), { native: WIN, bundle: BOTH });
  for (const event of ["schedule", "workflow_dispatch"]) {
    assert.deepEqual(systems(runScope(event).outputs), { native: BOTH, bundle: BOTH }, event);
  }
});

test("a queue run that selects a family runs exactly the systems the reuse lookup judges", () => {
  const judged = (names) => [...new Set(names.map((name) => /\((windows-latest|macos-latest)[,)]/.exec(name)[1]))];
  const queue = systems(runScope("merge_group", { changed: ["src-tauri/src/x.rs"] }).outputs);
  assert.deepEqual(queue.native, judged(FAMILIES.native));
  assert.deepEqual(queue.bundle, judged(FAMILIES.bundle));
});

test("a push records the decision in the step summary, in a fenced block, and runs the lookup script", () => {
  const { summary, args } = runScope("push", { script: "both" });
  assert.equal(summary, "### Master push reuse\n```\nreuse_native=true\nreuse_bundle=true\nreason=stub\n```\nLookup exit status: 0\n");
  assert.equal(args, "scripts/master-push-reuse.mjs");
});

test("a crashed lookup is recorded in the summary with its exit status, and prints no decision", () => {
  const { summary, outputs } = runScope("push", { script: "crashedAfterTrue" });
  assert.equal(summary, "### Master push reuse\n```\n\n```\nLookup exit status: 3\n");
  assert.deepEqual(heavy(outputs), ALL);
});

test("the token reaches the scope step only on a push", () => {
  const step = workflow.join("\n");
  assert.match(step, /^ {10}GH_TOKEN: \$\{\{ github\.event_name == 'push' && github\.token \|\| '' \}\}$/m);
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

test("a change list larger than a pipe buffer is still scoped by its first heavy path", () => {
  // `printf | grep -q` under pipefail failed when grep stopped at the first match while printf still had more to write
  // (SIGPIPE, status 141), and the step then wrote native=false. The paths sort with the workflow file first, so the heavy
  // match is the first line and the list that follows is well over 64 KiB.
  const docs = Array.from({ length: 1700 }, (_, index) => `docs/tally/scope-fixture/a-long-directory-name/${String(index).padStart(5, "0")}-reference-file-name.md`);
  const changed = [".github/workflows/a.yml", ...docs];
  assert.ok(changed.join("\n").length > 100_000, "the list is over a pipe buffer");
  for (const event of ["pull_request", "merge_group"]) {
    assert.deepEqual(heavy(runScope(event, { script: "both", changed }).outputs),
      { native: "true", bundle: "false", tax_audit: "false" }, `${event} with a heavy path first in a long list`);
  }
});

test("a draft pull request runs no native or bundle job, and keeps the tax-audit scope", () => {
  const changed = ["src-tauri/crates/bridge-tax-audit/src/lib.rs"];
  assert.deepEqual(heavy(runScope("pull_request", { changed, draft: "true" }).outputs),
    { native: "false", bundle: "false", tax_audit: "true" }, "draft");
  // Only the exact payload value `true` is a draft; ready_for_review and every later push carry `false`.
  for (const draft of ["false", "", "TRUE", "true "]) {
    assert.deepEqual(heavy(runScope("pull_request", { changed, draft }).outputs),
      { native: "true", bundle: "true", tax_audit: "true" }, `draft=${JSON.stringify(draft)}`);
  }
  // The flag is read on a pull request alone: no other event is a draft run, whatever the variable holds.
  for (const event of ["merge_group", "push", "schedule", "workflow_dispatch"]) {
    assert.deepEqual(heavy(runScope(event, { changed, draft: "true" }).outputs), ALL, event);
  }
});

test("ready_for_review is a pull_request trigger, so marking a draft ready starts a run scoped in full", () => {
  assert.ok(workflow.includes("    types: [opened, reopened, synchronize, ready_for_review]"));
});
