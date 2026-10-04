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
  "    # platforms as bundle-smoke, whose runs it guards (on a pull request, Windows only).",
  "    if: needs.changes.outputs.bundle == 'true'",
  "    runs-on: ${{ matrix.os }}",
  "    timeout-minutes: 45",
  "    permissions:",
  "      contents: read",
  "    strategy:",
  "      fail-fast: false",
  "      matrix:",
  "        os: ${{ fromJSON(github.event_name == 'pull_request' && '[\"windows-latest\"]' || '[\"windows-latest\", \"macos-latest\"]') }}",
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
const releaseWorkflow = readFileSync(resolve(repositoryRoot, ".github/workflows/release-mcpb-preview.yml"), "utf8");
// The release workflow's jobs are pinned by name: a job block is read up to the next `  name:` line, so an
// unexpected job key would cut the previous job's block short.
if (jobIds(releaseWorkflow).join(",") !== "release-admission,package,attest,publish-preview") {
  failures.push("release-mcpb-preview.yml must have exactly the jobs release-admission, package, attest and publish-preview");
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
  ], "956e3337285aaa4880e97ac8cc4f0ee88889f90512143f45885e2e1e0126a533"],
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
  ], "11471d580af5f7bbe5af1a6a080f97de8c946504de6e3772a3a2461fae4879c0"],
  [workflow, "workflow-consistency", ["      - run: node scripts/check-ci-workflow-consistency.mjs"], "3694871963037bbb13bd4e71faa05a4b245dee9c0296a610142234d1604aebd4"],
  [releaseWorkflow, "package", [
    "      - name: Prove the release binary lacks the test-only approval seam",
    "        shell: bash",
    "        run: node scripts/check-no-test-seam.mjs src-tauri/target/release/${{ matrix.binary }}",
  ], "b0f27a021c9d0fe13a3021a2177a3e649b5afa33e7e90e6738e4f7fd9b72fee4"],
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
  ["ci.yml", workflow, "25526dcdd7691fef35c27e07c0bb0291543cbf3ba55e7ac4ee40bd59f802b1e9"],
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
if (reuseScriptDigest !== "1cf4382fc71ab7339022544962a5ce55fd8128269fe7bf9f269b022a430d8854") {
  failures.push(`scripts/master-push-reuse.mjs changed; its digest is now ${reuseScriptDigest}`);
}
if (jobBlock(workflow, "native").match(/^    if: .*$/gm)?.join("\n") !== "    if: needs.changes.outputs.native == 'true'") {
  failures.push("native must run on every pull request that changes native code");
}
for (const [name, source] of [["ci.yml", workflow], ["release-mcpb-preview.yml", releaseWorkflow]]) {
  if (source.includes("continue-on-error")) failures.push(`${name} must not use continue-on-error: a failure would report success`);
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
  "    outputs:",
  "      bundle: ${{ steps.scope.outputs.bundle }}",
  "      native: ${{ steps.scope.outputs.native }}",
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
  "          MERGE_GROUP_BASE_SHA: ${{ github.event.merge_group.base_sha }}",
  "          GH_TOKEN: ${{ github.event_name == 'push' && github.token || '' }}",
  "        run: |",
  "          set -euo pipefail",
  "",
  "          # The unscoped run: every heavy job runs, whatever changed.",
  "          full_run() {",
  "            echo 'bundle=true' >> \"$GITHUB_OUTPUT\"",
  "            echo 'native=true' >> \"$GITHUB_OUTPUT\"",
  "            echo 'tax_audit=true' >> \"$GITHUB_OUTPUT\"",
  "            exit 0",
  "          }",
  "",
  "          if [[ \"$EVENT_NAME\" == \"workflow_dispatch\" || \"$EVENT_NAME\" == \"schedule\" ]]; then",
  "            full_run",
  "          fi",
  "",
  "          # A push to master runs a family of heavy jobs unless the merge queue already RAN that family",
  "          # on this exact commit and it passed (scripts/master-push-reuse.mjs). A queue run that skipped",
  "          # the family by scope is no evidence, so it is run here as it always was. Any error or doubt",
  "          # in the lookup runs both: a non-zero exit discards whatever was printed (its status goes to the job",
  "          # summary), and only an exact",
  "          # `reuse_<family>=true` line among the first two skips anything. tax_audit is cheap and always runs on a push.",
  "          # To turn the reuse off, delete this block: every push is then a full run again.",
  "          if [[ \"$EVENT_NAME\" == \"push\" ]]; then",
  "            lookup_status=0",
  "            decision=\"$(node scripts/master-push-reuse.mjs)\" || lookup_status=$?",
  "            if [[ \"$lookup_status\" -ne 0 ]]; then decision=''; fi",
  "            printf '%s\\n' \"$decision\"",
  "            { echo '### Master push reuse'; echo '```'; printf '%s\\n' \"$decision\"; echo '```'; echo \"Lookup exit status: $lookup_status\"; } >> \"$GITHUB_STEP_SUMMARY\"",
  "            for family in native bundle; do",
  "              if printf '%s\\n' \"$decision\" | head -n 2 | grep -qx \"reuse_${family}=true\"; then",
  "                echo \"${family}=false\" >> \"$GITHUB_OUTPUT\"",
  "              else",
  "                echo \"${family}=true\" >> \"$GITHUB_OUTPUT\"",
  "              fi",
  "            done",
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
  "          # A missing base has no useful predecessor. Keep the conservative path.",
  "          if [[ -z \"$base\" || \"$base\" =~ ^0+$ ]]; then",
  "            full_run",
  "          fi",
  "",
  "          # --no-renames lists a moved file under both paths, so a file moved out of a gated directory still selects it.",
  "          # -z: git would otherwise quote a path with non-ASCII bytes, and the quoted form matches no prefix below.",
  "          changed_files=\"$(git diff --name-only --no-renames -z \"$base\" \"$GITHUB_SHA\" | tr '\\0' '\\n')\"",
  "          if printf '%s\\n' \"$changed_files\" | grep -Eq '^(\\.github/workflows/|\\.github/actions/setup-windows-native/|rust-toolchain\\.toml|src-tauri/|tools/|scripts/package-mcpb\\.mjs|scripts/check-no-test-seam(\\.test)?\\.mjs|scripts/check-tally-egress-boundary(\\.test)?\\.mjs|scripts/tally-egress-census\\.json|scripts/testdata/egress-census-|\\.cargo/|\\.gitattributes$|package\\.json$|packaging/mcpb/manifest\\.json$|packaging/pdfium/|docs/adr/0016-master-binding-authority\\.md$|docs/agent/README\\.md$|docs/tally/compatibility/compatibility-surface\\.json$|LICENSE$|NOTICE$|THIRD_PARTY_LICENSES(_RUST)?\\.txt$|scripts/(fixtures/|fetch-pdfium\\.py$|check-tally-request-builder-hazards\\.mjs$|collect-macos-test-crashes\\.py$|retain-macos-test-binaries\\.py$))'; then",
  "            echo 'native=true' >> \"$GITHUB_OUTPUT\"",
  "          else",
  "            echo 'native=false' >> \"$GITHUB_OUTPUT\"",
  "          fi",
  "",
  "          # The mutation records check runs when the crate it guards changes, or this workflow does.",
  "          if printf '%s\\n' \"$changed_files\" | grep -Eq '^(\\.github/workflows/ci\\.yml|src-tauri/crates/bridge-tax-audit/)'; then",
  "            echo 'tax_audit=true' >> \"$GITHUB_OUTPUT\"",
  "          else",
  "            echo 'tax_audit=false' >> \"$GITHUB_OUTPUT\"",
  "          fi",
  "",
  "          # tools/ is deliberately excluded: it is not shipped in the bundle, which validates the artifact and its resources.",
  "          if printf '%s\\n' \"$changed_files\" | grep -Eq '^(\\.github/workflows/ci\\.yml|\\.github/actions/setup-windows-native/|packaging/mcpb/|package\\.json|pnpm-lock\\.yaml|\\.node-version|vite\\.config\\.ts|tsconfig\\.json|postcss\\.config\\.js|index\\.html|src/|src-tauri/(src/|crates/|Cargo\\.lock|Cargo\\.toml|.*/Cargo\\.toml|tauri\\.conf\\.json|build\\.rs|icons/)|LICENSE$|NOTICE$|THIRD_PARTY_LICENSES\\.txt$|THIRD_PARTY_LICENSES_RUST\\.txt$|packaging/pdfium/|scripts/(fetch-pdfium(\\.test)?\\.py|capture-package-log(\\.test)?\\.py|check-mcpb-bundle(\\.test)?\\.py|package-mcpb\\.mjs|check-license-metadata\\.mjs|check-dependency-inventory\\.mjs|check-windows-bundle-resources\\.ps1|check-macos-bundle-resources(\\.mutation)?\\.mjs|check-no-test-seam(\\.test)?\\.mjs)$|\\.cargo/|src-tauri/\\.cargo/)'; then",
  "            echo 'bundle=true' >> \"$GITHUB_OUTPUT\"",
  "          else",
  "            echo 'bundle=false' >> \"$GITHUB_OUTPUT\"",
  "          fi",
].join("\n");
if (jobBlock(workflow, "changes").trimEnd() !== expectedChanges) {
  failures.push("changes changed shape; review its scope rules and update expectedChanges");
}
// The native job runs on a pull request only when the scope above selects it. A test in it that reads
// a file the scope does not list is skipped, with the file's change, by the queue, and only a master
// push would find the break. So: every file a Rust test pulls in from outside src-tauri/ and tools/
// through include_str!/include_bytes!, and every file listed below that a test or a native-job step
// reads at run time, must be matched by the native scope's own pattern. Tests in tools/ are not listed: `Tally
// portable core` runs that workspace on every pull request, whatever the scope selects.
const nativeScope = new RegExp(/grep -Eq '(\^\([^']+\))'; then\n\s+echo 'native=true'/.exec(jobBlock(workflow, "changes"))?.[1] ?? "(unreadable)");
const tracked = trackedFiles();
const readAtRunTime = [
  [".cargo/config.toml", "src-tauri/tests/approval_seam_gate.rs (read if present; none is tracked today)", true],
  [".gitattributes", "byte-exact fixture checkout on Windows"],
  ["package.json", "src-tauri/tests/approval_seam_gate.rs"],
  ["packaging/pdfium/pdfium.lock.json", "the native job's PDFium step"],
  ["scripts/fetch-pdfium.py", "the native job's PDFium step"],
  ["scripts/collect-macos-test-crashes.py", "the native job's macOS crash steps"],
  ["scripts/retain-macos-test-binaries.py", "the native job's macOS crash steps"],
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
for (const file of tracked.filter((path) => /^(?:src-tauri|tools)\/.+\.rs$/.test(path))) {
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
    if (!/^(?:src-tauri|tools)\//.test(resolved) && !nativeScope.test(resolved)) {
      failures.push(`the native scope omits ${resolved}, which ${file} includes`);
    }
  }
}
const expectedRequiredChecks = [
  "  required-checks:",
  "    name: Required checks",
  "    # Always runs so it can report a single, stable required status for branch",
  "    # protection. Workflow consistency is unconditional and must succeed. Other",
  "    # upstream jobs may be legitimately skipped (e.g. native/bundle on docs-only",
  "    # PRs), but any failure or cancellation fails this job. Point branch",
  "    # protection at this context instead of the individual matrix jobs so",
  "    # docs-only PRs are not blocked by skipped native/bundle checks.",
  "    # tax-audit-mutations is required too, on pull requests that touch the",
  "    # crate: it runs the mutation runner's tests and `--verify` (no build), so a",
  "    # bridge-tax-audit change merges only with its selected mutations proven on",
  "    # the merged tree. Skipped on other pull requests, which is a pass here.",
  "    if: ${{ always() }}",
  `    needs: [${jobIds(workflow).filter((id) => id !== "required-checks").join(", ")}]`,
  "    runs-on: ubuntu-latest",
  "    timeout-minutes: 5",
  "    permissions:",
  "      contents: read",
  "    steps:",
  "      - name: Verify no required job failed",
  "        shell: bash",
  "        env:",
  "          NEEDS_JSON: ${{ toJSON(needs) }}",
  "        run: |",
  "          set -euo pipefail",
  "          printf '%s\\n' \"$NEEDS_JSON\"",
  "          failed=\"$(",
  "            printf '%s' \"$NEEDS_JSON\" \\",
  "              | python3 -c \"import json,sys; d=json.load(sys.stdin); failed=[k for k,v in d.items() if v.get('result') in ('failure','cancelled')]; failed += [] if d.get('workflow-consistency', {}).get('result') == 'success' else ['workflow-consistency']; print(' '.join(dict.fromkeys(failed)))\"",
  "          )\"",
  "          if [[ -n \"$failed\" ]]; then",
  "            echo \"Required jobs did not pass: $failed\"",
  "            exit 1",
  "          fi",
  "          echo \"All required jobs succeeded or were legitimately skipped.\"",
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
// name, so no feature (default included) and no dependency declaration in either workspace may turn
// it on for a build that did not ask.
for (const manifest of ["src-tauri", "tools"]) {
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

for (const stalePath of staleToolPaths()) {
  failures.push(`stale tools-workspace path: ${stalePath.file} references ${stalePath.path}`);
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

function staleToolPaths() {
  const toolsMetadata = workspaceMetadata(resolve(repositoryRoot, "tools", "Cargo.toml"));
  const legacyPaths = toolsMetadata.packages.map((candidate) => `src-tauri/crates/${candidate.name}`);
  const stale = [];
  for (const file of trackedFiles()) {
    const contents = readFileSync(resolve(repositoryRoot, file), "utf8");
    for (const path of legacyPaths) {
      if (contents.includes(path)) stale.push({ file, path });
    }
  }
  return stale;
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
