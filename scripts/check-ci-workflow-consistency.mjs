// SPDX-License-Identifier: Apache-2.0

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, posix, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const scriptRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const rootArgument = process.argv.indexOf("--root");
if (rootArgument !== -1 && !process.argv[rootArgument + 1]) {
  throw new Error("--root requires a repository path");
}
const repositoryRoot = rootArgument === -1 ? scriptRoot : resolve(process.argv[rootArgument + 1]);
const workflowPath = resolve(repositoryRoot, ".github/workflows/ci.yml");
const workflow = readFileSync(workflowPath, "utf8");
const failures = [];
const metadataByWorkspace = new Map();

// The surface acknowledgement check runs in this job: it runs on every event, has the full
// history, and is required through `Required checks`. The step enforces: a pinned-file change
// without its acknowledgement file fails the job. It must stay, keep its exact shape (no
// `continue-on-error`, no step-level `if`, no `--report-only` flag), and any change to it is a
// pinned ci.yml change with its own acknowledgement.
const workflowConsistency = jobBlock(workflow, "workflow-consistency");
const toolchain = readFileSync(resolve(repositoryRoot, "rust-toolchain.toml"), "utf8").match(/^channel *= *"([^"]+)"/m)?.[1];
if (!toolchain) throw new Error("could not read [toolchain].channel from rust-toolchain.toml");
const pinnedRustSetup = new RegExp(
  `^      - uses: dtolnay/rust-toolchain@[^\\n]+\\n        with:\\n          toolchain: ${escapeRegex(toolchain)}$`,
  "m",
);
if (!pinnedRustSetup.test(workflowConsistency)) {
  failures.push("workflow-consistency must install the repository's pinned Rust toolchain");
}
const surfaceAckStep = [
  "      - name: Check the surface acknowledgement",
  "        env:",
  "          PR_NUMBER: ${{ github.event.pull_request.number }}",
  "          PR_HEAD_SHA: ${{ github.event.pull_request.head.sha }}",
  "          MERGE_GROUP_BASE_SHA: ${{ github.event.merge_group.base_sha }}",
  "          CHECK_MODE: ${{ github.event_name == 'pull_request' && 'pull_request' || github.event_name == 'merge_group' && 'merge_group' || github.event_name == 'push' && 'push' || 'workflow_dispatch' }}",
  '        run: node scripts/check-surface-ack.mjs --mode "$CHECK_MODE"',
].join("\n");
// The step block runs from its `- name:` line up to the next step (or the job end), so a line
// appended after `run:` (`continue-on-error`, `if`, ...) is a difference, not a pass. Blank and
// full-line comment lines at step indentation just before the next step belong to that step.
const surfaceAckLines = workflowConsistency.split("\n");
const surfaceAckStart = surfaceAckLines.indexOf("      - name: Check the surface acknowledgement");
let surfaceAckBlock = null;
if (surfaceAckStart !== -1) {
  let end = surfaceAckStart + 1;
  while (end < surfaceAckLines.length && !surfaceAckLines[end].startsWith("      - ")) end += 1;
  while (end > surfaceAckStart + 1 && /^(?: {6}#.*)?$/.test(surfaceAckLines[end - 1])) end -= 1;
  surfaceAckBlock = surfaceAckLines.slice(surfaceAckStart, end).join("\n");
}
if (surfaceAckBlock !== surfaceAckStep) {
  failures.push("workflow-consistency must run the surface acknowledgement check with its exact shape");
}

// bridge#583: the release positive control is what makes a clean seam scan of the shipped
// executables mean anything. It runs in its own job, so that job must stay required, keep
// bundle-smoke's scope and platforms, and keep its whole shape: a single pinned line would still
// pass with an extra `needs`, a `continue-on-error`, a step-level `if`, one OS dropped or an env
// override. Change the job and this copy together, deliberately.
const expectedSeamControl = [
  "  seam-control:",
  "    name: Seam positive control (${{ matrix.os }})",
  "    needs: changes",
  "    # The release-profile half of bundle-smoke's seam proof, run beside it",
  "    # rather than after it: it builds the bridge lib unit-test executable in",
  "    # release and requires the marker there, so a clean scan of the shipped",
  "    # executables means the scan could have seen the seam. Same scope and",
  "    # platforms as bundle-smoke, whose runs it guards (the scope job's bundle_os).",
  "    if: needs.changes.outputs.bundle == 'true'",
  "    runs-on: ${{ matrix.os }}",
  "    timeout-minutes: 45",
  "    permissions:",
  "      contents: read",
  "    strategy:",
  "      fail-fast: false",
  "      matrix:",
  "        os: ${{ fromJSON(needs.changes.outputs.bundle_os) }}",
  "    steps:",
  "      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7",
  "        with:",
  "          persist-credentials: false",
  "      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v6",
  "        with:",
  "          node-version-file: .node-version",
  "      - uses: dtolnay/rust-toolchain@4be7066ada62dd38de10e7b70166bc74ed198c30 # stable",
  "        with:",
  "          toolchain: 1.96.0",
  "      - name: Match macOS cache inputs to the Tauri deployment target",
  "        if: runner.os == 'macOS'",
  "        shell: bash",
  "        run: |",
  "          node --input-type=module <<'JS'",
  "          import { appendFileSync, readFileSync } from 'node:fs';",
  "          const config = JSON.parse(readFileSync('src-tauri/tauri.conf.json', 'utf8'));",
  "          const target = config.bundle?.macOS?.minimumSystemVersion;",
  "          if (typeof target !== 'string' || !/^\\d+\\.\\d+(?:\\.\\d+)?$/.test(target)) {",
  "            throw new Error('Expected an explicit macOS minimumSystemVersion in tauri.conf.json');",
  "          }",
  "          appendFileSync(process.env.GITHUB_ENV, `MACOSX_DEPLOYMENT_TARGET=${target}\\n`);",
  "          JS",
  "      # Restores bundle-smoke's release dependencies (same key) and never saves:",
  "      # a second cache family would only compete for the repository's quota.",
  "      - uses: Swatinem/rust-cache@6323deb102c322ba6fcbdcafc7e3dddab59af2b6 # v2",
  "        with:",
  "          workspaces: src-tauri -> target",
  "          shared-key: bundle-smoke",
  "          env-vars: ${{ runner.os == 'macOS' && 'MACOSX_DEPLOYMENT_TARGET' || '' }}",
  "          save-if: false",
  "      - name: Set up Windows native prerequisites",
  "        if: runner.os == 'Windows'",
  "        uses: ./.github/actions/setup-windows-native",
  "      - name: Prove the seam scan sees a release test build",
  "        shell: bash",
  "        run: node scripts/check-no-test-seam.mjs --test-harness --release",
].join("\n");
const seamControl = jobBlock(workflow, "seam-control");
const bundleSmoke = jobBlock(workflow, "bundle-smoke");
const bundleScope = bundleSmoke.match(/^    if: .*$/m)?.[0];
const bundleOs = bundleSmoke.match(/^        os: .*$/m)?.[0];
if (seamControl.trimEnd() !== expectedSeamControl) {
  failures.push("seam-control changed shape; review it against bridge#583 and update expectedSeamControl");
}
if (!bundleScope || seamControl.match(/^    if: .*$/m)?.[0] !== bundleScope) {
  failures.push("seam-control must run whenever bundle-smoke runs");
}
if (!bundleOs || seamControl.match(/^        os: .*$/m)?.[0] !== bundleOs) {
  failures.push("seam-control must cover every platform bundle-smoke builds");
}

// A step that must run is pinned whole: a pinned command line alone still passes with a step-level
// `if`, a `continue-on-error`, an `|| true`, or the command kept only in a comment. Everything that
// runs in its job before it can also change what it sees, so each job is pinned too, by SHA-256
// over its exact committed UTF-8 text from its key through that step (line endings included, so
// even a whitespace or CRLF change trips it). Change a step and its copy, or a job and its digest, together
// and deliberately; the failure prints the new digest for a reviewed change.
// The per-test timings artifact is public for 7 days and what it carries is set in nextest.toml, outside the pinned
// upload step: one JUnit table with the output stores off, and no setup scripts (their command, arguments and
// environment names would join the report). TOML has many spellings of the same table (a bracketed header, a dotted
// key, an inline table, a key written with an escape), and no TOML parser is a dependency of this repository, so the
// file is pinned by its lines instead: every line that is not blank and not a full-line comment must equal the list
// below, in order. A change to the file's settings is then a change to this list, made on purpose. A failing test's
// panic text still reaches the report, which is why the upload runs only after a passing test step.
const nextestConfig = readFileSync(resolve(repositoryRoot, "src-tauri/.config/nextest.toml"), "utf8").replace(/\r\n/g, "\n");
const nextestLines = [
  "[profile.default]",
  'slow-timeout = { period = "60s", terminate-after = 6 }',
  "[[profile.default.overrides]]",
  "filter = 'test(/access::tests::on_windows::/)'",
  'success-output = "immediate"',
  "[profile.default.junit]",
  'path = "junit.xml"',
  "store-success-output = false",
  "store-failure-output = false",
];
const nextestEffective = nextestConfig.split("\n").filter((line) => !/^\s*(?:#.*)?$/.test(line));
if (nextestEffective.length !== nextestLines.length || nextestEffective.some((line, index) => line !== nextestLines[index])) {
  failures.push("src-tauri/.config/nextest.toml must hold exactly one JUnit report table, the pinned one with both output stores off, and no setup scripts: every line that is not a blank or a full-line comment must equal nextestLines in scripts/check-ci-workflow-consistency.mjs");
}
const releaseWorkflow = readFileSync(resolve(repositoryRoot, ".github/workflows/release-mcpb-preview.yml"), "utf8");
// The release workflow's jobs are pinned by name: a job block is read up to the next `  name:` line, so an
// unexpected job key would cut the previous job's block short.
if (jobIds(releaseWorkflow).join(",") !== "release-admission,package,attest,publish-preview") {
  failures.push("release-mcpb-preview.yml must have exactly the jobs release-admission, package, attest and publish-preview");
}
// The records and nightly run starts its eight shards four at a time: eight at once would hold 8 of the account's 20
// runner slots for about two hours, and every pull request's checks wait behind them.
const nightlyWorkflow = readFileSync(resolve(repositoryRoot, ".github/workflows/tax-audit-mutations-nightly.yml"), "utf8");
if (jobBlock(nightlyWorkflow, "shard").match(/^ {6}max-parallel: .*$/gm)?.join("\n") !== "      max-parallel: 4") {
  failures.push("the mutation shards must run four at a time (strategy.max-parallel: 4 in tax-audit-mutations-nightly.yml)");
}
// Line-based readers split on \n (and \r\n); a lone \r, U+0085, U+2028 or U+2029 is a line break to other
// YAML readers but not to them, so neither workflow may contain one.
for (const [name, source] of [["ci.yml", workflow], ["release-mcpb-preview.yml", releaseWorkflow]]) {
  if (/\r(?!\n)|[\u0085\u2028\u2029]/.test(source)) failures.push(`${name} must use only \\n or \\r\\n line breaks`);
}
for (const [source, job, expected, digest] of [
  [workflow, "native", [
    "      - name: Prove the approval-seam scan sees a test build",
    "        shell: bash",
    "        run: node scripts/check-no-test-seam.mjs --test-harness",
  ], "b0320b1c8c35bc7c1210b1f070c73af39445ef9fe34af1c6c506b4a65686a131"],
  [workflow, "bundle-smoke", [
    "      - name: Prove shipped executables lack the test-only approval seam",
    "        shell: bash",
    "        run: |",
    "          set -euo pipefail",
    "          ext=\"${{ runner.os == 'Windows' && '.exe' || '' }}\"",
    "          node scripts/check-no-test-seam.mjs \"src-tauri/target/release/bridge$ext\" \"src-tauri/target/release/bridge_mcp$ext\"",
    "          if [[ \"$RUNNER_OS\" == \"macOS\" ]]; then",
    "            node scripts/check-no-test-seam.mjs src-tauri/target/release/bundle/macos",
    "          fi",
  ], "3d84b5d1b3a818c0cf867ceee2bb4ba64a952443a4dc9d67ecc2d5609c4cc0c1"],
  [workflow, "workflow-consistency", ["      - run: node scripts/check-ci-workflow-consistency.mjs"], "3694871963037bbb13bd4e71faa05a4b245dee9c0296a610142234d1604aebd4"],
  [releaseWorkflow, "package", [
    "      - name: Prove the release binary lacks the test-only approval seam",
    "        shell: bash",
    "        run: node scripts/check-no-test-seam.mjs src-tauri/target/release/${{ matrix.binary }}",
  ], "b8f6b70bdced2019e32e8fee1c1d20a7dcce0d270384849636182953702a384a"],
]) {
  if (stepBlock(jobBlock(source, job), expected[0]) !== expected.join("\n")) {
    failures.push(`${job}: step "${expected[0].trim()}" changed shape; review it and update its pinned copy`);
  }
  const actual = sha256(jobThrough(source, job, expected[0]));
  if (actual !== digest) failures.push(`${job} changed before "${expected[0].trim()}"; its digest is now ${actual}`);
  // The digest covers the job only up to the pinned step. A job-level key (container, env, services,
  // defaults, ...) written after the steps is outside it, so `steps:` must be the job's last key.
  const jobKeys = jobBlock(source, job).split("\n").filter((line) => /^ {4}[^ \t#-]/.test(line));
  if (jobKeys.at(-1)?.trimEnd() !== "    steps:") failures.push(`${job}: \`steps:\` must be the job's last key; a job-level key after the steps is outside its pinned text`);
}
// Each workflow's header (triggers, permissions, concurrency, env, defaults) applies to every job;
// jobIds refuses a workflow-level key after the jobs map, so the header is all of them.
// And native, bundle-smoke and package run a local composite action before their scans; a local
// action can call another, so every tracked file under .github/actions/ is pinned by its bytes.
for (const [name, source, digest] of [
  ["ci.yml", workflow, "37e4f47df2582a32e175b296dc710afb2b2f4be87e47fb7cd59d386199eeb2f1"],
  ["release-mcpb-preview.yml", releaseWorkflow, "c4a747416c492779cfd43305cfd619728d2c9821a17f3efb73da08f67dc56144"],
]) {
  const lines = source.split("\n");
  const jobs = lines.findIndex((line) => line.replace(/\r$/, "") === "jobs:");
  const actual = sha256(jobs === -1 ? source : lines.slice(0, jobs + 1).join("\n"));
  if (actual !== digest) failures.push(`${name}'s workflow header changed; its digest is now ${actual}`);
}
const localActions = createHash("sha256");
for (const path of trackedFiles().filter((file) => file.startsWith(".github/actions/"))) {
  localActions.update(`${path}\0`).update(readFileSync(resolve(repositoryRoot, path))).update("\0");
}
const localActionsDigest = localActions.digest("hex");
if (localActionsDigest !== "64490129722cf1c153ab7e9643a9c69bbc16b22aeef165f17a851ab2db5479da") {
  failures.push(`.github/actions/ changed; its digest is now ${localActionsDigest}`);
}
// The lookup that decides whether a master push may skip heavy jobs is pinned by its bytes: a change
// to it is a change to what can be skipped, so it needs this file edited (and acknowledged) with it.
const reuseScriptDigest = createHash("sha256").update(readFileSync(resolve(repositoryRoot, "scripts/master-push-reuse.mjs"))).digest("hex");
if (reuseScriptDigest !== "fed907ab68fe52ebd7eb227a377f546a51127303eef48d6cee560fa6e2560c6a") {
  failures.push(`scripts/master-push-reuse.mjs changed; its digest is now ${reuseScriptDigest}`);
}
if (jobBlock(workflow, "native").match(/^    if: .*$/gm)?.join("\n") !== "    if: needs.changes.outputs.native == 'true'") {
  failures.push("native must run on every pull request that changes native code");
}
if (jobBlock(workflow, "native").match(/^ {8}os: .*$/gm)?.join("\n") !== "        os: ${{ fromJSON(needs.changes.outputs.native_os) }}") {
  failures.push("native must run on the systems the scope job's native_os names");
}
// The native tests run as two halves, one per shard, by nextest's partitioning. What would silently lose
// coverage is pinned: a shard list other than [1, 2], a partition count that does not match it, a step
// that was meant to run once per operating system but never runs (or runs twice), and any other use of the
// shard number. A half that never runs is a green check with half the tests missing.
const nativeJob = jobBlock(workflow, "native");
const shardRows = nativeJob.match(/^ {8}shard:.*$/gm) ?? [];
if (shardRows.length !== 1 || shardRows[0] !== "        shard: [1, 2]") {
  failures.push("native must split its tests into exactly two shards: one matrix row `shard: [1, 2]`");
}
// An `exclude:` or `include:` entry under the matrix would drop or add a leg without touching the `shard:` row above
// (for example `exclude: [{os: macos-latest, shard: 2}]` silently runs half the macOS tests), so neither is allowed.
if (/^ {8}(?:include|exclude)\s*:/m.test(nativeJob)) {
  failures.push("native: the matrix must not use include: or exclude: (a leg dropped or added there changes which tests run)");
}
const nativeTestStep = [
  "      - name: Test native workspace",
  "        id: native-tests",
  "        run: >-",
  "          cargo nextest run --locked --manifest-path src-tauri/Cargo.toml",
  "          --workspace --no-fail-fast --retries 0 --test-threads num-cpus --timings",
  "          --partition count:${{ matrix.shard }}/2",
].join("\n");
if (stepBlock(nativeJob, "      - name: Test native workspace") !== nativeTestStep) {
  failures.push("native: the test step must run nextest on its own half of the tests (`--partition count:${{ matrix.shard }}/2`) and be exactly the pinned text");
}
const onceSteps = [
  ["      - name: Fetch pinned PDFium", 1],
  ["      - name: Test PDF extraction through PDFium", 1],
  ["      - name: Test native workspace doctests", 1],
  ["      - name: Lint the native workspace", 2],
  ["      - name: Census the Tally-path egress lints", 2],
];
for (const [head, shard] of onceSteps) {
  if (stepBlock(nativeJob, head)?.split("\n")[1] !== `        if: matrix.shard == ${shard}`) {
    failures.push(`native: step "${head.trim()}" must have \`if: matrix.shard == ${shard}\` as its first key, so it runs once per operating system`);
  }
}
const allowedShardLines = new Set([
  "    name: Native checks (${{ matrix.os }}, shard ${{ matrix.shard }} of 2)",
  "          save-if: ${{ (github.event_name == 'push' || github.event_name == 'workflow_dispatch' || github.event_name == 'schedule') && matrix.shard == 1 }}",
  "          --partition count:${{ matrix.shard }}/2",
  "          save-cache: ${{ matrix.shard == 1 }}",
  "          name: native-test-timings-macos-shard-${{ matrix.shard }}",
  "          name: native-crash-stacks-macos-shard-${{ matrix.shard }}",
  "          name: native-crash-binaries-macos-shard-${{ matrix.shard }}",
  "          name: native-compiler-timings-${{ matrix.os }}-shard-${{ matrix.shard }}",
  "        if: matrix.shard == 1",
  "        if: matrix.shard == 2",
]);
const shardUses = nativeJob.split("\n").filter((line) => line.includes("matrix.shard"));
if (shardUses.some((line) => !allowedShardLines.has(line))
  || shardUses.filter((line) => line === "        if: matrix.shard == 1").length !== 3
  || shardUses.filter((line) => line === "        if: matrix.shard == 2").length !== 2) {
  failures.push("native: matrix.shard may appear only in the pinned lines (job name, cache save, partition flag, artifact names, and five once-per-system step conditions)");
}
// Every non-condition line must be present exactly once: an artifact name without the shard would make both
// shards upload to one name (the second upload fails), and a dropped line would go unseen above.
for (const line of allowedShardLines) {
  if (line.startsWith("        if:")) continue;
  if (nativeJob.split("\n").filter((candidate) => candidate === line).length !== 1) {
    failures.push(`native: the pinned shard line must appear exactly once: ${line.trim()}`);
  }
}
// `continue-on-error` is refused everywhere (a failure would report success) except on these five
// diagnostics-only artifact uploads. Each runs under `always()`, has no `id`, and nothing in this
// repository downloads or reads its outcome; a GitHub service timeout in one of them (a merge-queue
// entry was dropped on 3 Oct 2026 by `CreateArtifact ETIMEDOUT`) is not a failure of the code under
// test. Left out on purpose: the failed-run executable evidence and the unsigned bundle
// upload, whose absence or failure must stay visible. Each allowed step is pinned
// whole (its job, its text and the upload action's SHA), so the flag cannot move to another step or
// take an expression, and the count and exact line are checked below.
const diagnosticUploads = [
  ["native", [
    "      - name: Retain minimized macOS test crash stacks",
    "        if: ${{ always() && runner.os == 'macOS' && steps.native-crash-boundary.outcome == 'success' }}",
    "        continue-on-error: true",
    "        uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1",
    "        with:",
    "          name: native-crash-stacks-macos-shard-${{ matrix.shard }}",
    "          path: ${{ runner.temp }}/bridge-native-crashes.json",
    "          if-no-files-found: error",
    "          retention-days: 7",
  ]],
  ["native", [
    "      - name: Retain native compiler timings",
    "        if: ${{ always() }}",
    "        continue-on-error: true",
    "        uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1",
    "        with:",
    "          name: native-compiler-timings-${{ matrix.os }}-shard-${{ matrix.shard }}",
    "          path: src-tauri/target/cargo-timings/*.html",
    "          if-no-files-found: warn",
    "          retention-days: 7",
  ]],
  ["native", [
    "      - name: Retain macOS per-test timings",
    "        if: ${{ always() && runner.os == 'macOS' && steps.native-tests.outcome == 'success' }}",
    "        continue-on-error: true",
    "        uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1",
    "        with:",
    "          name: native-test-timings-macos-shard-${{ matrix.shard }}",
    "          path: src-tauri/target/nextest/default/junit.xml",
    "          if-no-files-found: warn",
    "          retention-days: 7",
  ]],
  ["bundle-smoke", [
    "      - name: Retain macOS package diagnostics",
    "        if: ${{ always() && runner.os == 'macOS' && (steps.macos-bundle.outcome == 'success' || steps.macos-bundle.outcome == 'failure') }}",
    "        continue-on-error: true",
    "        uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1",
    "        with:",
    "          name: macos-package-diagnostics",
    "          path: |",
    "            package-diagnostics/**",
    "            src-tauri/target/release/bundle/dmg/bundle_dmg.sh",
    "            src-tauri/target/cargo-timings/*.html",
    "          if-no-files-found: warn",
    "          retention-days: 7",
  ]],
  ["bundle-smoke", [
    "      - name: Upload package compiler cache statistics",
    "        if: ${{ always() && steps.package-sccache-start.outcome == 'success' }}",
    "        continue-on-error: true",
    "        uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1",
    "        with:",
    "          name: package-compiler-cache-${{ matrix.os }}",
    "          path: package-compiler-cache.json",
    "          if-no-files-found: error",
    "          retention-days: 7",
  ]],
];
for (const [job, lines] of diagnosticUploads) {
  if (stepBlock(jobBlock(workflow, job), lines[0]) !== lines.join("\n")) {
    failures.push(`${job}: diagnostics-only upload "${lines[0].trim()}" changed shape; review it and update diagnosticUploads`);
  }
}
const flagLines = workflow.split("\n").filter((line) => line.includes("continue-on-error"));
if (flagLines.length !== diagnosticUploads.length || flagLines.some((line) => line.replace(/\r$/, "") !== "        continue-on-error: true")) {
  failures.push("ci.yml must not use continue-on-error except the one exact line in each pinned diagnostics-only upload: a failure would report success");
}
// The Playwright browsers cache is saved only by push, manual and scheduled runs (other caches, such as the Node
// package cache, still save from every ref). The combined restore-and-save cache action saves from
// every ref, and a pull-request or merge-queue ref saves a copy only that ref can read: on 15 Oct 2026 five copies of
// the 365 MiB Playwright browsers, saved by pull-request and merge-queue refs, sat in a repository cache that was at
// 9.96 of 10 GiB, and the eviction left the Windows runs cold. So the browsers are restored with the restore action and saved by one pinned step that runs after the install.
if (/actions\/cache@/.test(workflow)) {
  failures.push("ci.yml must not use the combined restore-and-save cache action (actions/cache@): a pull-request ref would save a copy of its own");
}
const playwrightSave = [
  "      - name: Save Playwright browsers",
  "        if: ${{ steps.playwright-cache.outputs.cache-hit != 'true' && (github.event_name == 'push' || github.event_name == 'workflow_dispatch' || github.event_name == 'schedule') }}",
  "        uses: actions/cache/save@55cc8345863c7cc4c66a329aec7e433d2d1c52a9 # v6.1.0",
  "        with:",
  "          path: ~/.cache/ms-playwright",
  "          key: playwright-${{ runner.os }}-${{ steps.playwright-version.outputs.version }}",
].join("\n");
const playwrightRestore = [
  "      - name: Restore Playwright browsers",
  "        id: playwright-cache",
  "        uses: actions/cache/restore@55cc8345863c7cc4c66a329aec7e433d2d1c52a9 # v6.1.0",
  "        with:",
  "          path: ~/.cache/ms-playwright",
  "          key: playwright-${{ runner.os }}-${{ steps.playwright-version.outputs.version }}",
].join("\n");
const frontendJob = jobBlock(workflow, "frontend");
if (stepBlock(frontendJob, "      - name: Restore Playwright browsers") !== playwrightRestore) {
  failures.push("the Playwright browsers restore step changed shape: its key must equal the save step's and it must have no restore-keys; review it and update playwrightRestore");
}
const playwrightInstallAt = frontendJob.indexOf("      - name: Install Playwright browsers and system dependencies\n");
const playwrightSaveAt = frontendJob.indexOf("      - name: Save Playwright browsers\n");
if (stepBlock(frontendJob, "      - name: Save Playwright browsers") !== playwrightSave || frontendJob.split("actions/cache/save@").length !== 3 || playwrightInstallAt === -1 || playwrightSaveAt < playwrightInstallAt) {
  failures.push("the Playwright browsers save step changed shape, or does not follow the install step; review it and update playwrightSave");
}
// The Playwright system packages (the .deb files apt downloads) are cached the same way and for the same reason: restored with
// the restore action, saved by one pinned step after the install, only by push, manual and scheduled runs, under a key that names the
// Playwright version and the runner image and with no restore-keys, so a pull-request or merge-queue ref can never write what a
// later run copies into apt's archive directory (bridge#2056). The job holds exactly two cache save steps: this one and the browsers'.
const playwrightDebsKey = "          key: playwright-debs-${{ runner.os }}-${{ steps.playwright-version.outputs.version }}-${{ steps.playwright-version.outputs.image }}";
const playwrightDebsSave = [
  "      - name: Save Playwright system packages",
  "        if: ${{ steps.apt-cache.outputs.cache-hit != 'true' && (github.event_name == 'push' || github.event_name == 'workflow_dispatch' || github.event_name == 'schedule') }}",
  "        uses: actions/cache/save@55cc8345863c7cc4c66a329aec7e433d2d1c52a9 # v6.1.0",
  "        with:",
  "          path: ~/apt-debs",
  playwrightDebsKey,
].join("\n");
const playwrightDebsRestore = [
  "      - name: Restore Playwright system packages",
  "        id: apt-cache",
  "        uses: actions/cache/restore@55cc8345863c7cc4c66a329aec7e433d2d1c52a9 # v6.1.0",
  "        with:",
  "          path: ~/apt-debs",
  playwrightDebsKey,
].join("\n");
if (stepBlock(frontendJob, "      - name: Restore Playwright system packages") !== playwrightDebsRestore || frontendJob.indexOf("      - name: Restore Playwright system packages\n") > playwrightInstallAt) {
  failures.push("the Playwright system packages restore step changed shape, or does not come before the install step: its key must equal the save step's and it must have no restore-keys; review it and update playwrightDebsRestore");
}
const playwrightDebsSaveAt = frontendJob.indexOf("      - name: Save Playwright system packages\n");
if (stepBlock(frontendJob, "      - name: Save Playwright system packages") !== playwrightDebsSave || playwrightDebsSaveAt < playwrightInstallAt) {
  failures.push("the Playwright system packages save step changed shape, or does not follow the install step; review it and update playwrightDebsSave");
}
for (const [name, source] of [["ci.yml", workflow], ["release-mcpb-preview.yml", releaseWorkflow]]) {
  if (name !== "ci.yml" && source.includes("continue-on-error")) failures.push(`${name} must not use continue-on-error: a failure would report success`);
  // A quoted key can spell a refused key with an escape, which the text rule above cannot read; refuse
  // one at the start of a line. (Other YAML spellings of a key are not covered by this line.)
  if (/^\s*(?:-\s+)?["'][^"'\n]*["']\s*:/m.test(source)) failures.push(`${name} must not start a line with a quoted mapping key: the continue-on-error rule reads keys as plain text`);
  // A step without its own `shell:` runs under the workflow's or job's defaults.
  if (/^\s*defaults\s*:/m.test(source)) failures.push(`${name} must not set defaults: they change how unpinned-shell steps run`);
}
if (/^    if\s*:/m.test(jobBlock(releaseWorkflow, "package"))) {
  failures.push("release-mcpb-preview.yml's package job must run whenever a release is admitted");
}

// Every MCPB is staged by package-mcpb.mjs, so its one seam scan must run unconditionally, with
// the scanner imported from its own module: main() is pinned from its first line through the
// scan, so nothing can return, branch or run first; no second call, no alias.
const packageMcpb = readFileSync(resolve(repositoryRoot, "scripts/package-mcpb.mjs"), "utf8");
const packageSeamScan = [
  "async function main() {",
  "  const { binaryPath, pdfiumDirectory } = packageMcpbArguments(process.argv.slice(2));",
  "  const host = mcpbHostTarget();",
  "  const sourceBinary = binaryPath ?? releaseMcpbBinaryPath(root, host.binary);",
  "  if (!binaryPath) {",
  "    const manifest = resolve(root, \"src-tauri\", \"Cargo.toml\");",
  "    const build = spawnSync(\"cargo\", [\"build\", \"--locked\", \"--release\", \"--manifest-path\", manifest, \"--bin\", \"bridge_mcp\"], {",
  "      cwd: root,",
  "      stdio: \"inherit\",",
  "    });",
  "    if (build.status !== 0) process.exit(build.status ?? 1);",
  "  }",
  "  try {",
  "    await access(sourceBinary);",
  "  } catch {",
  "    throw new Error(`MCPB binary is missing: ${sourceBinary}`);",
  "  }",
  "  // Every MCPB, from CI, a release or a local build, is staged here: refuse a",
  "  // binary compiled with the test-only approval seam (bridge#583).",
  "  assertNoTestSeam([sourceBinary]);",
].join("\n");
const packageSeamImport = 'import { assertNoTestSeam } from "./check-no-test-seam.mjs";\n';
if (
  packageMcpb.split("assertNoTestSeam").length !== 3 ||
  packageMcpb.split(packageSeamImport).length !== 2 ||
  packageMcpb.split(packageSeamScan).length !== 2
) {
  failures.push("package-mcpb must call assertNoTestSeam exactly once, unconditionally, before staging");
}

// The scope job decides whether native, bundle-smoke and seam-control run on a pull request, and
// required-checks turns every job into the one status branch protection reads. Its needs are every
// other job, so a new job cannot be left out, and a skipped aggregator cannot pass as green.
const expectedChanges = [
  "  changes:",
  "    name: Determine bundle scope",
  "    runs-on: ubuntu-latest",
  "    timeout-minutes: 5",
  "    permissions:",
  "      contents: read",
  "      # Only to read this workflow's own merge-queue runs and their jobs (scripts/master-push-reuse.mjs).",
  "      actions: read",
  "      # Only to read whether a pull request is still a draft when its draft run starts.",
  "      pull-requests: read",
  "    outputs:",
  "      bundle: ${{ steps.scope.outputs.bundle }}",
  "      bundle_os: ${{ steps.scope.outputs.bundle_os }}",
  "      draft: ${{ steps.scope.outputs.draft }}",
  "      native: ${{ steps.scope.outputs.native }}",
  "      native_os: ${{ steps.scope.outputs.native_os }}",
  "      tax_audit: ${{ steps.scope.outputs.tax_audit }}",
  "    steps:",
  "      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7",
  "        with:",
  "          persist-credentials: false",
  "          fetch-depth: 0",
  "      - id: scope",
  "        shell: bash",
  "        env:",
  "          EVENT_NAME: ${{ github.event_name }}",
  "          BEFORE_SHA: ${{ github.event.before }}",
  "          PR_BASE_SHA: ${{ github.event.pull_request.base.sha }}",
  "          PR_DRAFT: ${{ github.event.pull_request.draft }}",
  "          PR_NUMBER: ${{ github.event.pull_request.number }}",
  "          MERGE_GROUP_BASE_SHA: ${{ github.event.merge_group.base_sha }}",
  "          GH_TOKEN: ${{ (github.event_name == 'push' || github.event_name == 'pull_request') && github.token || '' }}",
  "        run: |",
  "          set -euo pipefail",
  "",
  "          # native_os and bundle_os are the systems the native, bundle-smoke and seam-control matrices run on. The macOS",
  "          # native tests run on a push to master, on a scheduled or manual run and on a release pull request (the owner's",
  "          # rule of 26 Oct 2026), never on another pull request or in the merge queue. The macOS bundle and seam jobs run",
  "          # on a scheduled or manual run and on a release pull request, and in the queue or on a push only when a packaging",
  "          # path below changed. Another pull request runs the bundle and seam jobs on Windows only.",
  "          windows='[\"windows-latest\"]'",
  "          both='[\"windows-latest\", \"macos-latest\"]'",
  "          # Packaging paths: src-tauri/tauri.conf.json, src-tauri/icons/, src-tauri/build.rs (it copies the bundled legal",
  "          # files), LICENSE, NOTICE, THIRD_PARTY_LICENSES.txt, THIRD_PARTY_LICENSES_RUST.txt, packaging/ (the MCPB manifest",
  "          # and the PDFium lock), package.json (the Tauri CLI and its build script), this workflow, and the scripts the",
  "          # bundle job runs: fetch-pdfium, capture-package-log, check-mcpb-bundle, package-mcpb, check-license-metadata,",
  "          # check-dependency-inventory, check-macos-bundle-resources and check-no-test-seam, with their tests. Also what",
  "          # pins the bundler and the build: pnpm-lock.yaml, .node-version, rust-toolchain.toml, .cargo/ and",
  "          # src-tauri/Cargo.lock, Cargo.toml and .cargo/.",
  "          packaging_paths='^(\\.github/workflows/ci\\.yml$|packaging/|package\\.json$|pnpm-lock\\.yaml$|\\.node-version$|rust-toolchain\\.toml$|\\.cargo/|src-tauri/(tauri\\.conf\\.json$|icons/|build\\.rs$|Cargo\\.(lock|toml)$|\\.cargo/)|LICENSE$|NOTICE$|THIRD_PARTY_LICENSES(_RUST)?\\.txt$|scripts/(fetch-pdfium(\\.test)?\\.mjs|capture-package-log(\\.test)?\\.mjs|check-mcpb-bundle(\\.test)?\\.py|package-mcpb\\.mjs|check-license-metadata\\.mjs|check-dependency-inventory\\.mjs|check-macos-bundle-resources(\\.mutation)?\\.mjs|check-no-test-seam(\\.test)?\\.mjs)$)'",
  "",
  "          # A draft pull request runs no native, bundle or seam job; every other job is scoped as below. Marking it ready",
  "          # (`ready_for_review`, a trigger above) starts a new run of the same head that is scoped in full (and cancels this",
  "          # one if it is still running), and Required checks there needs those jobs again. The merge queue's run is never a draft run. Only the",
  "          # exact payload value `true` is a draft, and then the pull request is read again: a re-run of an old draft run",
  "          # keeps its payload, and a push made just before the ready click can start its run after the ready run. A pull",
  "          # request that is no longer a draft, or whose state cannot be read, is scoped in full.",
  "          draft=false",
  "          if [[ \"$EVENT_NAME\" == \"pull_request\" && \"$PR_DRAFT\" == \"true\" ]]; then",
  "            live_draft=\"$(gh api \"repos/$GITHUB_REPOSITORY/pulls/$PR_NUMBER\" --jq .draft)\" || live_draft=unreadable",
  "            if [[ \"$live_draft\" == \"true\" ]]; then draft=true; fi",
  "          fi",
  "          echo \"draft=$draft\" >> \"$GITHUB_OUTPUT\"",
  "",
  "          # The unscoped run: every heavy job runs, whatever changed. Its caller writes the systems first.",
  "          full_run() {",
  "            echo 'bundle=true' >> \"$GITHUB_OUTPUT\"",
  "            echo 'native=true' >> \"$GITHUB_OUTPUT\"",
  "            echo 'tax_audit=true' >> \"$GITHUB_OUTPUT\"",
  "            exit 0",
  "          }",
  "",
  "          if [[ \"$EVENT_NAME\" == \"workflow_dispatch\" || \"$EVENT_NAME\" == \"schedule\" ]]; then",
  "            { echo \"native_os=$both\"; echo \"bundle_os=$both\"; } >> \"$GITHUB_OUTPUT\"",
  "            full_run",
  "          fi",
  "",
  "          # A push to master runs a family of heavy jobs on Windows unless the merge queue already RAN that",
  "          # family on this exact commit and it passed (scripts/master-push-reuse.mjs). A queue run that skipped",
  "          # the family by scope is no evidence, so it is run here as it always was. Any error or doubt",
  "          # in the lookup runs both: a non-zero exit discards whatever was printed (its status goes to the job",
  "          # summary), and only an exact",
  "          # `reuse_<family>=true` line among the first two skips anything. tax_audit is cheap and always runs on a push.",
  "          # The queue never runs the macOS native tests, so a push always does. It runs the macOS bundle and seam jobs",
  "          # when it changes a packaging path, or when its before commit cannot be compared.",
  "          # To turn the reuse off, delete the reuse lookup: every push then runs the Windows jobs again.",
  "          if [[ \"$EVENT_NAME\" == \"push\" ]]; then",
  "            lookup_status=0",
  "            decision=\"$(node scripts/master-push-reuse.mjs)\" || lookup_status=$?",
  "            if [[ \"$lookup_status\" -ne 0 ]]; then decision=''; fi",
  "            printf '%s\\n' \"$decision\"",
  "            { echo '### Master push reuse'; echo '```'; printf '%s\\n' \"$decision\"; echo '```'; echo \"Lookup exit status: $lookup_status\"; } >> \"$GITHUB_STEP_SUMMARY\"",
  "            reused() { printf '%s\\n' \"$decision\" | head -n 2 | grep -qx \"reuse_$1=true\"; }",
  "            packaging=true",
  "            if [[ -n \"$BEFORE_SHA\" && ! \"$BEFORE_SHA\" =~ ^0+$ ]] && pushed_files=\"$(git diff --name-only --no-renames -z \"$BEFORE_SHA\" \"$GITHUB_SHA\" | tr '\\0' '\\n')\" && ! grep -Eq \"$packaging_paths\" <<< \"$pushed_files\"; then",
  "              packaging=false",
  "            fi",
  "            echo 'native=true' >> \"$GITHUB_OUTPUT\"",
  "            if reused native; then",
  "              echo 'native_os=[\"macos-latest\"]' >> \"$GITHUB_OUTPUT\"",
  "            else",
  "              echo \"native_os=$both\" >> \"$GITHUB_OUTPUT\"",
  "            fi",
  "            if reused bundle && [[ \"$packaging\" == true ]]; then",
  "              { echo 'bundle=true'; echo 'bundle_os=[\"macos-latest\"]'; } >> \"$GITHUB_OUTPUT\"",
  "            elif reused bundle; then",
  "              { echo 'bundle=false'; echo \"bundle_os=$windows\"; } >> \"$GITHUB_OUTPUT\"",
  "            elif [[ \"$packaging\" == true ]]; then",
  "              { echo 'bundle=true'; echo \"bundle_os=$both\"; } >> \"$GITHUB_OUTPUT\"",
  "            else",
  "              { echo 'bundle=true'; echo \"bundle_os=$windows\"; } >> \"$GITHUB_OUTPUT\"",
  "            fi",
  "            echo 'tax_audit=true' >> \"$GITHUB_OUTPUT\"",
  "            exit 0",
  "          fi",
  "",
  "          if [[ \"$EVENT_NAME\" == \"pull_request\" ]]; then",
  "            base=\"$PR_BASE_SHA\"",
  "          else",
  "            base=\"$MERGE_GROUP_BASE_SHA\"",
  "          fi",
  "",
  "          # A release pull request is the version pull request of docs/release-process.md: `next-version.mjs --apply` changes",
  "          # the version in src-tauri/tauri.conf.json, so a pull request or queue entry whose version differs from its base's",
  "          # is one. A version that cannot be read on one side only counts as a difference. It runs both systems for the",
  "          # native, bundle and seam jobs, in the queue as well.",
  "          version_at() {",
  "            { git show \"$1:src-tauri/tauri.conf.json\" | grep -m1 -o '\"version\": *\"[^\"]*\"'; } 2>/dev/null || true",
  "          }",
  "          release=false",
  "          if [[ -n \"$base\" && ! \"$base\" =~ ^0+$ && \"$(version_at \"$base\")\" != \"$(version_at \"$GITHUB_SHA\")\" ]]; then release=true; fi",
  "          if [[ \"$release\" == true ]]; then",
  "            echo \"native_os=$both\" >> \"$GITHUB_OUTPUT\"",
  "          else",
  "            echo \"native_os=$windows\" >> \"$GITHUB_OUTPUT\"",
  "          fi",
  "          # A missing base has no useful predecessor. Keep the conservative path.",
  "          if [[ -z \"$base\" || \"$base\" =~ ^0+$ ]]; then",
  "            if [[ \"$EVENT_NAME\" == \"merge_group\" ]]; then echo \"bundle_os=$both\" >> \"$GITHUB_OUTPUT\"; else echo \"bundle_os=$windows\" >> \"$GITHUB_OUTPUT\"; fi",
  "            full_run",
  "          fi",
  "",
  "          # --no-renames lists a moved file under both paths, so a file moved out of a gated directory still selects it.",
  "          # -z: git would otherwise quote a path with non-ASCII bytes, and the quoted form matches no prefix below.",
  "          changed_files=\"$(git diff --name-only --no-renames -z \"$base\" \"$GITHUB_SHA\" | tr '\\0' '\\n')\"",
  "          if [[ \"$draft\" == false ]] && grep -Eq '^(\\.github/workflows/|\\.github/actions/|rust-toolchain\\.toml|glama/Dockerfile$|src-tauri/|scripts/package-mcpb\\.mjs|scripts/check-no-test-seam(\\.test)?\\.mjs|scripts/check-tally-egress-boundary(\\.test)?\\.mjs|scripts/tally-egress-census\\.json|scripts/testdata/egress-census-|\\.cargo/|\\.gitattributes$|package\\.json$|packaging/mcpb/manifest\\.json$|packaging/pdfium/|docs/adr/0016-master-binding-authority\\.md$|docs/agent/README\\.md$|docs/legal/terms\\.md$|docs/tally/compatibility/compatibility-surface\\.json$|LICENSE$|NOTICE$|THIRD_PARTY_LICENSES(_RUST)?\\.txt$|scripts/(fixtures/|fetch-pdfium\\.mjs$|check-tally-request-builder-hazards\\.mjs$|collect-macos-test-crashes(\\.test)?\\.mjs$|retain-macos-test-binaries(\\.test)?\\.mjs$))' <<< \"$changed_files\"; then",
  "            echo 'native=true' >> \"$GITHUB_OUTPUT\"",
  "          else",
  "            echo 'native=false' >> \"$GITHUB_OUTPUT\"",
  "          fi",
  "",
  "          # The mutation records check runs when the crate it guards changes, or this workflow does.",
  "          if grep -Eq '^(\\.github/workflows/ci\\.yml|src-tauri/crates/bridge-tax-audit/)' <<< \"$changed_files\"; then",
  "            echo 'tax_audit=true' >> \"$GITHUB_OUTPUT\"",
  "          else",
  "            echo 'tax_audit=false' >> \"$GITHUB_OUTPUT\"",
  "          fi",
  "",
  "          if [[ \"$draft\" == false ]] && grep -Eq '^(\\.github/workflows/ci\\.yml|\\.github/actions/setup-windows-native/|packaging/mcpb/|package\\.json|pnpm-lock\\.yaml|\\.node-version|vite\\.config\\.ts|tsconfig\\.json|postcss\\.config\\.js|index\\.html|src/|src-tauri/(src/|crates/|Cargo\\.lock|Cargo\\.toml|.*/Cargo\\.toml|tauri\\.conf\\.json|build\\.rs|icons/)|LICENSE$|NOTICE$|THIRD_PARTY_LICENSES\\.txt$|THIRD_PARTY_LICENSES_RUST\\.txt$|packaging/pdfium/|scripts/(fetch-pdfium(\\.test)?\\.mjs|capture-package-log(\\.test)?\\.mjs|check-mcpb-bundle(\\.test)?\\.py|package-mcpb\\.mjs|check-license-metadata\\.mjs|check-dependency-inventory\\.mjs|check-windows-bundle-resources\\.ps1|check-macos-bundle-resources(\\.mutation)?\\.mjs|check-no-test-seam(\\.test)?\\.mjs)$|\\.cargo/|src-tauri/\\.cargo/)' <<< \"$changed_files\"; then",
  "            echo 'bundle=true' >> \"$GITHUB_OUTPUT\"",
  "          else",
  "            echo 'bundle=false' >> \"$GITHUB_OUTPUT\"",
  "          fi",
  "",
  "          if [[ \"$release\" == true ]] || { [[ \"$EVENT_NAME\" == \"merge_group\" ]] && grep -Eq \"$packaging_paths\" <<< \"$changed_files\"; }; then",
  "            echo \"bundle_os=$both\" >> \"$GITHUB_OUTPUT\"",
  "          else",
  "            echo \"bundle_os=$windows\" >> \"$GITHUB_OUTPUT\"",
  "          fi",
].join("\n");
if (jobBlock(workflow, "changes").trimEnd() !== expectedChanges) {
  failures.push("changes changed shape; review its scope rules and update expectedChanges");
}
// The native job runs on a pull request only when the scope above selects it. A test in it that reads
// a file the scope does not list is skipped, with the file's change, by the queue, and only a master
// push would find the break. So: every file a Rust test pulls in from outside src-tauri/
// through include_str!/include_bytes!, and every file listed below that a test or a native-job step
// reads at run time, must be matched by the native scope's own pattern. A portable-core test that reads the
// repository at run time (`bridge-tally-core/tests/compatibility_surface_stays_whole.rs`) is not listed:
// `Tally portable core` runs those crates on every pull request, whatever the scope selects.
const nativeScope = new RegExp(/grep -Eq '(\^\([^']+\))' <<< "\$changed_files"; then\n\s+echo 'native=true'/.exec(jobBlock(workflow, "changes"))?.[1] ?? "(unreadable)");
const tracked = trackedFiles();
const readAtRunTime = [
  [".cargo/config.toml", "src-tauri/tests/approval_seam_gate.rs (read if present; none is tracked today)", true],
  [".gitattributes", "byte-exact fixture checkout on Windows"],
  ["package.json", "src-tauri/tests/approval_seam_gate.rs"],
  [".github/actions/", "src-tauri/tests/approval_seam_gate.rs (every composite action)"],
  ["glama/Dockerfile", "src-tauri/tests/approval_seam_gate.rs (it starts cargo)"],
  ["scripts/check-no-test-seam.mjs", "src-tauri/tests/approval_seam_gate.rs (it starts cargo)"],
  ["scripts/package-mcpb.mjs", "src-tauri/tests/approval_seam_gate.rs (it starts cargo)"],
  ["packaging/pdfium/pdfium.lock.json", "the native job's PDFium step"],
  ["scripts/fetch-pdfium.mjs", "the native job's PDFium step"],
  ["scripts/collect-macos-test-crashes.mjs", "the native job's macOS crash steps"],
  ["scripts/retain-macos-test-binaries.mjs", "the native job's macOS crash steps"],
  ["scripts/check-tally-request-builder-hazards.mjs", "src-tauri/src/tally/tdl_engine.rs"],
  ["scripts/fixtures/", "src-tauri/crates/bridge-bank-statement/tests/common/mod.rs"],
  ["docs/tally/compatibility/compatibility-surface.json", "src-tauri/tests/admission_and_egress_files_stay_pinned.rs"],
  ["LICENSE", "tauri.conf.json bundle resources, copied by the build script"],
  ["NOTICE", "tauri.conf.json bundle resources, copied by the build script"],
  ["THIRD_PARTY_LICENSES.txt", "tauri.conf.json bundle resources, copied by the build script"],
  ["THIRD_PARTY_LICENSES_RUST.txt", "tauri.conf.json bundle resources, copied by the build script"],
];
for (const [entry, reader, optional] of readAtRunTime) {
  // A listed directory stands for every tracked file in it, each of which must be selected.
  const files = entry.endsWith("/") ? tracked.filter((path) => path.startsWith(entry)) : tracked.filter((path) => path === entry);
  if (files.length === 0 && optional) { if (!nativeScope.test(entry)) failures.push(`the native scope omits ${entry}, which ${reader} reads`); }
  else if (files.length === 0) failures.push(`the run-time read list names ${entry}, which has no tracked file; update the list (${reader})`);
  for (const file of files) if (!nativeScope.test(file)) failures.push(`the native scope omits ${file}, which ${reader} reads`);
}
for (const file of tracked.filter((path) => /^src-tauri\/.+\.rs$/.test(path))) {
  const source = readFileSync(resolve(repositoryRoot, file), "utf8").replace(/^\s*\/\/.*$/gm, "");
  for (const opening of source.matchAll(/include_(?:str|bytes)!\(/g)) {
    // The argument runs to the matching parenthesis, so a nested concat!(env!(..), "..") is read whole.
    let depth = 1;
    let end = opening.index + opening[0].length;
    while (end < source.length && depth > 0) depth += source[end] === "(" ? 1 : source[end] === ")" ? -1 : 0, end += 1;
    const argument = source.slice(opening.index + opening[0].length, end - 1);
    const literal = /^\s*"([^"]+)"\s*,?\s*$/.exec(argument)?.[1];
    if (literal === undefined) {
      // A form this scan cannot resolve (concat!, env!) that climbs out of the crate must be listed by hand above.
      if (argument.includes("..")) failures.push(`${file} includes a path this check cannot resolve (${argument.trim().slice(0, 80)}); list it in readAtRunTime`);
      continue;
    }
    const resolved = posix.normalize(posix.join(posix.dirname(file), literal));
    if (!/^src-tauri\//.test(resolved) && !nativeScope.test(resolved)) {
      failures.push(`the native scope omits ${resolved}, which ${file} includes`);
    }
  }
}
const expectedRequiredChecks = [
  "  required-checks:",
  "    # A draft-scoped run posts `Required checks (draft)`, which no ruleset requires, never `Required checks`: its",
  "    # green means the heavy jobs were skipped on purpose, and must not stand in for the ready run's gate on the same",
  "    # head. The ready run, the merge queue's run and every other event post `Required checks`.",
  "    name: ${{ needs.changes.outputs.draft == 'true' && 'Required checks (draft)' || 'Required checks' }}",
  "    # Always runs so it can report a single, stable required status for branch",
  "    # protection. Point branch protection at this context instead of the",
  "    # individual matrix jobs, which report `skipped` when they are not selected.",
  "    # Each job it needs has one rule below. A job with no condition must succeed.",
  "    # native, bundle-smoke, seam-control and tax-audit-mutations run exactly when",
  "    # the scope job's output for them is 'true' (a draft pull request, a change",
  "    # outside their paths and a reused master push set it 'false'): each must",
  "    # succeed then and be skipped otherwise, so a job that should have run and",
  "    # did not fails this job. compiler-cache-retention runs on master only and",
  "    # may be skipped. A failure or cancellation fails every rule.",
  "    if: ${{ always() }}",
  `    needs: [${jobIds(workflow).filter((id) => id !== "required-checks").join(", ")}]`,
  "    runs-on: ubuntu-latest",
  "    timeout-minutes: 5",
  "    permissions:",
  "      contents: read",
  "      # Only to read whether a draft-scoped run's pull request is still a draft.",
  "      pull-requests: read",
  "    steps:",
  "      - name: Verify every job that should have run passed",
  "        shell: bash",
  "        env:",
  "          NEEDS_JSON: ${{ toJSON(needs) }}",
  "          GH_TOKEN: ${{ github.token }}",
  "          PR_NUMBER: ${{ github.event.pull_request.number }}",
  "        run: |",
  "          set -euo pipefail",
  "          printf '%s\\n' \"$NEEDS_JSON\"",
  "          node --input-type=module <<'JS'",
  "          import { execFileSync } from 'node:child_process';",
  "          // A rule is one of three kinds: { kind: 'always' }, { kind: 'scope', output } or { kind: 'optional' }.",
  "          const rules = new Map([",
  "            ['changes', { kind: 'always' }],",
  "            ['frontend', { kind: 'always' }],",
  "            ['rust-format', { kind: 'always' }],",
  "            ['workflow-consistency', { kind: 'always' }],",
  "            ['tally-portable', { kind: 'always' }],",
  "            ['native', { kind: 'scope', output: 'native' }],",
  "            ['bundle-smoke', { kind: 'scope', output: 'bundle' }],",
  "            ['seam-control', { kind: 'scope', output: 'bundle' }],",
  "            ['tax-audit-mutations', { kind: 'scope', output: 'tax_audit' }],",
  "            ['compiler-cache-retention', { kind: 'optional' }],",
  "          ]);",
  "          const RESULTS = new Set(['success', 'failure', 'cancelled', 'skipped']);",
  "          const needs = JSON.parse(process.env.NEEDS_JSON);",
  "          const scope = needs.changes?.outputs ?? {};",
  "          const problems = [];",
  "          // A draft-scoped run skipped the heavy jobs because its pull request was a draft when the scope job ran. It",
  "          // passes only while the pull request still is one, read again here: re-running the failed jobs of an old draft",
  "          // run after the ready click re-runs this job but not the scope job, and must not turn this job green.",
  "          if (scope.draft === 'true') {",
  "            let live = 'unreadable';",
  "            try {",
  "              live = execFileSync('gh', ['api', `repos/${process.env.GITHUB_REPOSITORY}/pulls/${process.env.PR_NUMBER}`, '--jq', '.draft'], { encoding: 'utf8' }).trim();",
  "            } catch { /* an unreadable pull request is not a draft */ }",
  "            console.log(`draft-scoped run; the pull request is a draft now: ${live}`);",
  "            if (live !== 'true') problems.push(`draft-scoped run, but the pull request is not a draft now (${JSON.stringify(live)}): its heavy jobs were skipped, so use the run of the ready state`);",
  "          } else if (scope.draft !== 'false') {",
  "            problems.push(`scope output draft is ${JSON.stringify(scope.draft)}, neither 'true' nor 'false'`);",
  "          }",
  "          // What this job requires of each result: the set of results that pass, or a reason it cannot judge.",
  "          function expected(rule) {",
  "            if (rule.kind === 'always') return { pass: ['success'] };",
  "            if (rule.kind === 'optional') return { pass: ['success', 'skipped'] };",
  "            if (rule.kind !== 'scope') return { error: `rule kind ${JSON.stringify(rule.kind)} is unknown` };",
  "            const selected = scope[rule.output];",
  "            if (selected === 'true') return { pass: ['success'] };",
  "            if (selected === 'false') return { pass: ['skipped'] };",
  "            return { error: `scope output ${rule.output} is ${JSON.stringify(selected)}, neither 'true' nor 'false'` };",
  "          }",
  "          for (const job of [...new Set([...Object.keys(needs), ...rules.keys()])].sort()) {",
  "            const result = Object.hasOwn(needs, job) ? needs[job]?.result : undefined;",
  "            const rule = rules.get(job);",
  "            if (!rule) { problems.push(`${job}: no rule says whether it must run`); continue; }",
  "            if (!RESULTS.has(result)) { problems.push(`${job}: result ${JSON.stringify(result)} is not one of GitHub's four`); continue; }",
  "            const want = expected(rule);",
  "            if (want.error) { problems.push(`${job}: ${want.error}`); continue; }",
  "            console.log(`${job}: ${result} (passes as ${want.pass.join(' or ')})`);",
  "            if (!want.pass.includes(result)) problems.push(`${job}: ${result}, required ${want.pass.join(' or ')}`);",
  "          }",
  "          if (problems.length) {",
  "            console.log(`Required jobs did not pass:\\n${problems.join('\\n')}`);",
  "            process.exit(1);",
  "          }",
  "          console.log('Every job that should have run passed, and every skipped job was not selected.');",
  "          JS",
].join("\n");
if (jobBlock(workflow, "required-checks").trimEnd() !== expectedRequiredChecks) {
  failures.push("required-checks changed shape, or does not need every other job; update expectedRequiredChecks");
}

if (/^env:/m.test(workflow)) {
  failures.push("a workflow-level env reaches seam-control's release build; set env per job instead");
}

for (const step of parseWorkflowSteps(workflow)) {
  for (const command of cargoCommands(step.run)) {
    const packages = [...command.matchAll(/(?:^|\s)-p\s+([A-Za-z0-9_.-]+)/g)].map((match) => match[1]);
    if (!packages.length) continue;

    const manifestPath = manifestForStep(step, command);
    const metadata = workspaceMetadata(manifestPath);
    for (const packageName of packages) {
      const packageManifest = metadata.packages.find((candidate) => candidate.name === packageName);
      if (!packageManifest) {
        failures.push(`${step.name}: package ${packageName} is not in ${relativePath(manifestPath)}`);
        continue;
      }

      for (const features of featureLists(command)) {
        for (const feature of features) {
          if (!Object.hasOwn(packageManifest.features, feature)) {
            failures.push(`${step.name}: feature ${feature} is absent from ${packageName}`);
          }
        }
      }
    }
  }
}

// ADR 0004: `lab-writes` compiles the lab-only Tally write tools. The only way in is an explicit
// `--features lab-writes`, which no workflow, action or Cargo config (an alias could carry it) may
// name, so no feature (default included) and no dependency declaration in the workspace may turn
// it on for a build that did not ask.
for (const manifest of ["src-tauri"]) {
  for (const candidate of workspaceMetadata(resolve(repositoryRoot, manifest, "Cargo.toml")).packages) {
    for (const [feature, members] of Object.entries(candidate.features)) {
      if (members.some((member) => /(?:^|\/)lab-writes$/.test(member))) {
        failures.push(`${candidate.name}: feature ${feature} enables lab-writes`);
      }
    }
    for (const dependency of candidate.dependencies ?? []) {
      if (dependency.features?.includes("lab-writes")) {
        failures.push(`${candidate.name}: its ${dependency.name} dependency enables lab-writes`);
      }
    }
  }
}
const buildInputs = /^\.github\/(?:workflows|actions)\/|(?:^|\/)\.cargo\/config(?:\.toml)?$/;
for (const file of trackedFiles().filter((path) => buildInputs.test(path))) {
  if (/lab[-_]writes|--all-features/i.test(readFileSync(resolve(repositoryRoot, file), "utf8"))) {
    failures.push(`${file} names lab-writes or --all-features; no CI build may compile the lab write tools`);
  }
}

if (failures.length) {
  throw new Error(`CI workflow references do not resolve:\n${failures.join("\n")}`);
}

console.log("CI workflow references and required execution contracts resolve.");

function parseWorkflowSteps(source) {
  const steps = [];
  const lines = source.split(/\r?\n/);
  for (let index = 0; index < lines.length; index += 1) {
    const start = lines[index].match(/^ {6}-\s+(.*)$/);
    if (!start) continue;

    const name = start[1].match(/^name:\s*(.*)$/)?.[1] ?? start[1];
    const step = { name, workingDirectory: ".", run: "" };
    for (index += 1; index < lines.length && !/^ {6}-\s/.test(lines[index]); index += 1) {
      const workingDirectory = lines[index].match(/^ {8}working-directory:\s*(.+?)\s*$/);
      if (workingDirectory) step.workingDirectory = workingDirectory[1];

      const run = lines[index].match(/^ {8}run:\s*(.*)$/);
      if (!run) continue;
      if (run[1] && run[1] !== ">-" && run[1] !== "|") {
        step.run = run[1];
        continue;
      }

      const commandLines = [];
      for (index += 1; index < lines.length && /^ {10}/.test(lines[index]); index += 1) {
        commandLines.push(lines[index].slice(10));
      }
      step.run = commandLines.join("\n");
      index -= 1;
    }
    index -= 1;
    if (step.run) steps.push(step);
  }
  return steps;
}

function jobBlock(source, jobName) {
  const lines = source.split(/\r?\n/);
  const start = lines.findIndex((line) => line === `  ${jobName}:`);
  if (start === -1) {
    failures.push(`CI workflow is missing job ${jobName}`);
    return "";
  }
  let end = start + 1;
  while (end < lines.length && !/^  [A-Za-z0-9_-]+:\s*$/.test(lines[end])) end += 1;
  return lines.slice(start, end).join("\n");
}

// Parses every job id, and refuses a line at job-key indentation it cannot read, so a job
// cannot hide from required-checks' needs behind a comment or a quoted key, and refuses any
// top-level key after the jobs map.
function jobIds(source) {
  const lines = source.split(/\r?\n/);
  const start = lines.indexOf("jobs:");
  if (start === -1) failures.push("CI workflow has no block-style jobs: map");
  const ids = [];
  for (const line of lines.slice(start + 1)) {
    // A workflow-level key after the jobs map would sit outside the pinned header.
    if (/^[^\s#]/.test(line)) {
      failures.push(`CI workflow has a top-level key after jobs: ${line}`);
      break;
    }
    if (!/^  [^\s#]/.test(line)) continue;
    const id = line.match(/^  ([A-Za-z0-9_-]+):$/)?.[1];
    if (id) ids.push(id);
    else failures.push(`CI workflow has a job key this gate cannot read: ${line}`);
  }
  return ids;
}

function sha256(text) {
  return createHash("sha256").update(text, "utf8").digest("hex");
}

// A job's exact text, from its key through the step that starts with `head`, split and rejoined on
// "\n" alone so any "\r" stays in the hashed text. Empty when either line is missing.
function jobThrough(source, job, head) {
  const lines = source.split("\n");
  const start = lines.findIndex((line) => line.replace(/\r$/, "") === `  ${job}:`);
  const step = lines.findIndex((line, index) => index > start && line.replace(/\r$/, "") === head);
  if (start === -1 || step === -1) return "";
  return lines.slice(start, stepEnd(lines, step + 1)).join("\n");
}

// A step ends at the next line that is not blank, not a comment and is indented six spaces or less.
// A comment line never ends a step, whatever its indentation: YAML ignores a comment's indentation, so
// the keys after it still belong to the step and are part of its pinned text. Blank lines and comments
// indented six spaces or less just before the next step are that step's, not this one's.
function stepEnd(lines, from) {
  let end = from;
  while (end < lines.length && !/^ {0,6}[^\s#]/.test(lines[end])) end += 1;
  while (end > from && /^ {0,6}(?:#.*)?$/.test(lines[end - 1].replace(/\r$/, ""))) end -= 1;
  return end;
}

function stepBlock(job, head) {
  const lines = job.split("\n");
  const starts = lines.flatMap((line, index) => (line === head ? [index] : []));
  if (starts.length !== 1) return undefined;
  return lines.slice(starts[0], stepEnd(lines, starts[0] + 1)).join("\n").trimEnd();
}

function escapeRegex(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function exactRunStepOffset(job, name, command) {
  const expected = `      - name: ${name}\n        run: ${command}`;
  const offset = job.indexOf(expected);
  if (offset === -1) return -1;
  const suffix = job.slice(offset + expected.length);
  return suffix === "" || suffix.startsWith("\n      - ") ? offset : -1;
}

function cargoCommands(run) {
  return [...run.matchAll(/(?:^|\n)\s*cargo\s+([\s\S]*?)(?=(?:\n\s*cargo\s)|$)/g)].map((match) => match[1]);
}

function featureLists(command) {
  return [...command.matchAll(/--features\s+([A-Za-z0-9_.,-]+)/g)].map((match) => match[1].split(","));
}

function manifestForStep(step, command) {
  const explicitManifest = command.match(/--manifest-path\s+([^\s]+)/)?.[1];
  return resolve(repositoryRoot, explicitManifest ?? step.workingDirectory, explicitManifest ? "" : "Cargo.toml");
}

function workspaceMetadata(manifestPath) {
  if (metadataByWorkspace.has(manifestPath)) return metadataByWorkspace.get(manifestPath);
  const result = spawnSync(
    "cargo",
    ["metadata", "--locked", "--no-deps", "--format-version", "1", "--manifest-path", manifestPath],
    { cwd: repositoryRoot, encoding: "utf8", windowsHide: true },
  );
  if (result.error || result.status !== 0) {
    const detail = result.error?.message ?? result.stderr.trim() ?? "unknown error";
    throw new Error(`cargo metadata failed for ${relativePath(manifestPath)}: ${detail}`);
  }
  const metadata = JSON.parse(result.stdout);
  metadataByWorkspace.set(manifestPath, metadata);
  return metadata;
}

function trackedFiles() {
  const tracked = spawnSync("git", ["-C", repositoryRoot, "ls-files", "-z"], {
    encoding: "utf8",
    windowsHide: true,
  });
  if (tracked.error || tracked.status !== 0) {
    const detail = tracked.error?.message ?? tracked.stderr.trim() ?? "unknown error";
    throw new Error(`git ls-files failed: ${detail}`);
  }
  return tracked.stdout.split("\0").filter(Boolean);
}

function relativePath(path) {
  return path.slice(repositoryRoot.length + 1).replaceAll("\\", "/");
}
