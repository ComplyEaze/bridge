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

function runScope(event, { script = "silent", changed = [], draft = "", env = {}, versions = null } = {}) {
  const dir = mkdtempSync(join(tmpdir(), "scope-step-"));
  try {
    const sh = (command) => {
      const result = spawnSync("bash", ["-c", command], { cwd: dir, encoding: "utf8" });
      assert.equal(result.status, 0, result.stderr);
      return result.stdout.trim();
    };
    const commit = (message) => sh(`git -c user.name=t -c user.email=t@example.invalid -c commit.gpgsign=false commit -q --allow-empty -m ${message} && git rev-parse HEAD`);
    sh("git init -q -b main");
    // `versions` is [the base's version, the head's]: a src-tauri/tauri.conf.json that carries it at each commit.
    const tauriConf = (version) => {
      mkdirSync(join(dir, "src-tauri"), { recursive: true });
      writeFileSync(join(dir, "src-tauri/tauri.conf.json"), `{\n  "productName": "x",\n  "version": "${version}",\n  "identifier": "x"\n}\n`);
    };
    if (versions) { tauriConf(versions[0]); sh("git add -A"); }
    const base = commit("base");
    for (const file of changed) {
      mkdirSync(join(dir, file, ".."), { recursive: true });
      writeFileSync(join(dir, file), "x");
    }
    if (versions) { if (versions[1] === null) rmSync(join(dir, "src-tauri/tauri.conf.json")); else tauriConf(versions[1]); }
    sh("git add -A");
    const head = commit("change");
    mkdirSync(join(dir, "bin"));
    const [printed, exitCode] = stand[script];
    writeFileSync(join(dir, "bin", "node"), `#!/bin/bash\nprintf '%s' "$1" > "$STUB_ARGS"\ncat <<'EOF'\n${printed}EOF\nexit ${exitCode}\n`);
    chmodSync(join(dir, "bin", "node"), 0o755);
    // A stand-in `gh api repos/<repo>/pulls/<n> --jq .draft`: prints the live draft state, or fails.
    writeFileSync(join(dir, "bin", "gh"), `#!/bin/bash\nprintf '%s' "$*" > "$STUB_GH_ARGS"\n[ "$STUB_GH_EXIT" = 0 ] || exit "$STUB_GH_EXIT"\nprintf '%s\\n' "$STUB_LIVE_DRAFT"\n`);
    chmodSync(join(dir, "bin", "gh"), 0o755);
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
        GITHUB_REPOSITORY: "o/r", PR_NUMBER: "7", STUB_GH_ARGS: join(dir, "gh-args"), STUB_GH_EXIT: "0", STUB_LIVE_DRAFT: "true",
        EVENT_NAME: event, BEFORE_SHA: base, PR_BASE_SHA: base, PR_DRAFT: draft, MERGE_GROUP_BASE_SHA: base, ...env,
      },
    });
    assert.equal(result.status, 0, result.stderr);
    const lines = readFileSync(outputFile, "utf8").trim().split("\n").map((line) => [line.slice(0, line.indexOf("=")), line.slice(line.indexOf("=") + 1)]);
    assert.equal(new Set(lines.map(([key]) => key)).size, lines.length, "every output is written once");
    const outputs = Object.fromEntries(lines);
    const args = existsSync(join(dir, "args")) ? readFileSync(join(dir, "args"), "utf8") : null;
    const ghArgs = existsSync(join(dir, "gh-args")) ? readFileSync(join(dir, "gh-args"), "utf8") : null;
    return { outputs, summary: readFileSync(summaryFile, "utf8"), args, ghArgs };
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
  for (const changed of [["src-tauri/tauri.conf.json"], ["packaging/mcpb/manifest.json"], ["LICENSE"], ["src-tauri/icons/icon.png"], [".github/workflows/ci.yml"], ["pnpm-lock.yaml"], [".node-version"], ["rust-toolchain.toml"], ["src-tauri/Cargo.lock"], ["src-tauri/Cargo.toml"], ["src-tauri/.cargo/config.toml"], [".cargo/config.toml"]]) {
    assert.deepEqual(systems(runScope("push", { script: "both", changed }).outputs), { native: MAC, bundle: MAC }, changed[0]);
    assert.deepEqual(systems(runScope("push", { script: "neither", changed }).outputs), { native: BOTH, bundle: BOTH }, changed[0]);
  }
  for (const changed of [["src-tauri/src/x.rs"], ["src/App.tsx"], ["docs/a.md"], ["src-tauri/crates/bridge-core/Cargo.toml"], ["scripts/check-windows-bundle-resources.ps1"]]) {
    assert.deepEqual(systems(runScope("push", { script: "neither", changed }).outputs), { native: BOTH, bundle: WIN }, changed[0]);
  }
  for (const BEFORE_SHA of ["", "0".repeat(40), "f".repeat(40)]) {
    assert.deepEqual(systems(runScope("push", { script: "both", env: { BEFORE_SHA } }).outputs), { native: MAC, bundle: MAC }, `before ${BEFORE_SHA}`);
  }
});

test("a release pull request, whose version differs from its base's, runs the native, bundle and seam jobs on both systems, in the queue as well; an unchanged version does not", () => {
  const code = ["src-tauri/src/x.rs"];
  for (const event of ["pull_request", "merge_group"]) {
    assert.deepEqual(systems(runScope(event, { changed: code, versions: ["0.6.0", "0.6.1"] }).outputs), { native: BOTH, bundle: BOTH }, `${event} with a new version`);
    assert.deepEqual(systems(runScope(event, { changed: code, versions: ["0.6.0", "0.6.0"] }).outputs).native, WIN, `${event} with the same version`);
    // One side unreadable counts as a change; both unreadable (no such file) does not.
    assert.deepEqual(systems(runScope(event, { changed: code, versions: ["0.6.0", null] }).outputs).native, BOTH, `${event} that removes the file`);
    assert.deepEqual(systems(runScope(event, { changed: code }).outputs).native, WIN, `${event} without the file`);
  }
  // A release pull request that is still a draft runs none of them, and the ready run does.
  const draft = runScope("pull_request", { changed: code, draft: "true", versions: ["0.6.0", "0.6.1"] });
  assert.deepEqual(heavy(draft.outputs), { native: "false", bundle: "false", tax_audit: "false" });
  assert.deepEqual(systems(runScope("pull_request", { changed: code, draft: "false", versions: ["0.6.0", "0.6.1"] }).outputs), { native: BOTH, bundle: BOTH });
  // A push is judged by its own rules (macOS native always), and a version change does not widen what the queue ran.
  assert.deepEqual(systems(runScope("push", { script: "both", changed: code, versions: ["0.6.0", "0.6.1"] }).outputs).native, MAC);
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

test("the token reaches the scope step only on a push and a pull request", () => {
  const step = workflow.join("\n");
  assert.match(step, /^ {10}GH_TOKEN: \$\{\{ \(github\.event_name == 'push' \|\| github\.event_name == 'pull_request'\) && github\.token \|\| '' \}\}$/m);
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

test("a draft payload is checked against the live pull request, so a re-run of an old draft run is scoped in full once the pull request is ready", () => {
  const changed = ["src-tauri/crates/bridge-tax-audit/src/lib.rs"];
  const full = { native: "true", bundle: "true", tax_audit: "true" };
  const asked = runScope("pull_request", { changed, draft: "true" });
  assert.equal(asked.ghArgs, "api repos/o/r/pulls/7 --jq .draft");
  assert.deepEqual(heavy(asked.outputs), { native: "false", bundle: "false", tax_audit: "true" });
  assert.equal(asked.outputs.draft, "true", "Required checks reads the draft scope from this output");
  for (const live of ["false", "", "TRUE", "null"]) {
    assert.deepEqual(heavy(runScope("pull_request", { changed, draft: "true", env: { STUB_LIVE_DRAFT: live } }).outputs), full, `live draft ${JSON.stringify(live)}`);
  }
  // A lookup that fails leaves the pull request unjudged as a draft: it runs in full.
  assert.deepEqual(heavy(runScope("pull_request", { changed, draft: "true", env: { STUB_GH_EXIT: "1" } }).outputs), full, "gh failed");
  // No other payload value asks, and no other event does.
  for (const draft of ["false", ""]) assert.equal(runScope("pull_request", { changed, draft }).ghArgs, null, `draft ${JSON.stringify(draft)}`);
  for (const event of ["merge_group", "push", "schedule", "workflow_dispatch"]) {
    const { ghArgs, outputs } = runScope(event, { changed, draft: "true" });
    assert.equal(ghArgs, null, event);
    assert.equal(outputs.draft, "false", event);
  }
  assert.equal(runScope("pull_request", { changed, draft: "true", env: { STUB_LIVE_DRAFT: "false" } }).outputs.draft, "false");
});

test("ready_for_review is a pull_request trigger, so marking a draft ready starts a run scoped in full", () => {
  assert.ok(workflow.includes("    types: [opened, reopened, synchronize, ready_for_review]"));
});

// The workflow's concurrency header, evaluated: GitHub's `${{ }}` expressions here are plain `==`, `&&` and `||` over strings,
// so they run as JavaScript against a stand-in `github` context.
function concurrencyOf(event, { ref = "refs/pull/1/merge", runId = "100" } = {}) {
  const text = workflow.join("\n");
  const block = /^concurrency:\n((?: {2}.*\n| *\n)+)/m.exec(text)?.[1] ?? "";
  const field = (name) => new RegExp(`^ {2}${name}: (.+)$`, "m").exec(block)?.[1];
  const github = { workflow: "Bridge CI", event_name: event, ref, run_id: runId };
  const evaluate = (template) => template.replace(/\$\{\{ (.+?) \}\}/g, (_, expression) => String(new Function("github", `return (${expression});`)(github)));
  return { group: evaluate(field("group")), cancel: evaluate(field("cancel-in-progress")) };
}

test("pull requests supersede by branch; master pushes share one group and never cancel a run; every other run has a group of its own", () => {
  const pr = concurrencyOf("pull_request", { ref: "refs/pull/7/merge", runId: "1" });
  assert.equal(pr.cancel, "true");
  assert.equal(pr.group, concurrencyOf("pull_request", { ref: "refs/pull/7/merge", runId: "2" }).group, "a newer push to the same branch is in the same group");
  assert.notEqual(pr.group, concurrencyOf("pull_request", { ref: "refs/pull/8/merge", runId: "1" }).group);
  const push = concurrencyOf("push", { ref: "refs/heads/master", runId: "1" });
  assert.equal(push.cancel, "false", "a master push never cancels a run in progress; only a pending one is replaced");
  // Every `push` run is a master push only because the trigger says so: widen it and other branches would share this group.
  assert.match(workflow.join("\n"), /^on:\n {2}push:\n(?: {4}#.*\n)* {4}branches: \[master\]$/m);
  assert.equal(push.group, concurrencyOf("push", { ref: "refs/heads/master", runId: "2" }).group, "two master pushes share a group");
  assert.notEqual(push.group, pr.group);
  for (const event of ["merge_group", "schedule", "workflow_dispatch"]) {
    const first = concurrencyOf(event, { ref: "refs/heads/gh-readonly-queue/master/x", runId: "1" });
    assert.equal(first.cancel, "false", event);
    assert.notEqual(first.group, concurrencyOf(event, { ref: "refs/heads/gh-readonly-queue/master/x", runId: "2" }).group, `${event} runs are never in one group`);
    assert.notEqual(first.group, push.group);
  }
});
