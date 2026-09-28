// SPDX-License-Identifier: Apache-2.0

import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
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

// These suites may skip in the frontend job, which intentionally has no Rust.
// Bind their real execution to an existing required job with the pinned toolchain.
const workflowConsistency = jobBlock(workflow, "workflow-consistency");
const toolchain = readFileSync(resolve(repositoryRoot, "rust-toolchain.toml"), "utf8").match(/^channel *= *"([^"]+)"/m)?.[1];
if (!toolchain) throw new Error("could not read [toolchain].channel from rust-toolchain.toml");
const pinnedRustPreflight = `rustup which --toolchain ${toolchain} rustc`;
const resealRegressions = "node --test scripts/reseal.test.mjs scripts/reseal-merge-driver.test.mjs";
const pinnedRustSetup = new RegExp(
  `^      - uses: dtolnay/rust-toolchain@[^\\n]+\\n        with:\\n          toolchain: ${escapeRegex(toolchain)}$`,
  "m",
);
if (!pinnedRustSetup.test(workflowConsistency)) {
  failures.push("workflow-consistency must install the repository's pinned Rust toolchain");
}
const preflightOffset = exactRunStepOffset(workflowConsistency, "Require pinned Rust for reseal regressions", pinnedRustPreflight);
const regressionOffset = exactRunStepOffset(workflowConsistency, "Test reseal and merge-driver regressions", resealRegressions);
if (preflightOffset === -1) failures.push("workflow-consistency must fail when pinned Rust is unavailable");
if (regressionOffset === -1) failures.push("workflow-consistency must run the reseal regression suites");
if (preflightOffset !== -1 && regressionOffset !== -1 && preflightOffset > regressionOffset) {
  failures.push("workflow-consistency must require pinned Rust before running reseal regressions");
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
  "    # platforms as bundle-smoke, whose runs it guards.",
  "    if: github.event_name != 'pull_request' || needs.changes.outputs.bundle == 'true'",
  "    runs-on: ${{ matrix.os }}",
  "    timeout-minutes: 45",
  "    permissions:",
  "      contents: read",
  "    strategy:",
  "      fail-fast: false",
  "      matrix:",
  "        os: [windows-latest, macos-latest]",
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

// A step that must run is pinned whole, as seam-control is: a pinned command line alone still
// passes with a step-level `if`, a `continue-on-error`, an `|| true`, or the command kept only in
// a comment. Change a step and its copy here together, deliberately.
const releaseWorkflow = readFileSync(resolve(repositoryRoot, ".github/workflows/release-mcpb-preview.yml"), "utf8");
for (const [source, job, expected] of [
  [workflow, "native", [
    "      - name: Prove the approval-seam scan sees a test build",
    "        shell: bash",
    "        run: node scripts/check-no-test-seam.mjs --test-harness",
  ]],
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
  ]],
  [workflow, "workflow-consistency", ["      - run: node scripts/check-ci-workflow-consistency.mjs"]],
  [releaseWorkflow, "package", [
    "      - name: Prove the release binary lacks the test-only approval seam",
    "        shell: bash",
    "        run: node scripts/check-no-test-seam.mjs src-tauri/target/release/${{ matrix.binary }}",
  ]],
]) {
  if (stepBlock(jobBlock(source, job), expected[0]) !== expected.join("\n")) {
    failures.push(`${job}: step "${expected[0].trim()}" changed shape; review it and update its pinned copy`);
  }
}
if (jobBlock(workflow, "native").match(/^    if: .*$/gm)?.join("\n") !== "    if: github.event_name != 'pull_request' || needs.changes.outputs.native == 'true'") {
  failures.push("native must run on every pull request that changes native code");
}
for (const [name, source] of [["ci.yml", workflow], ["release-mcpb-preview.yml", releaseWorkflow]]) {
  if (source.includes("continue-on-error")) failures.push(`${name} must not use continue-on-error: a failure would report success`);
}

// Every MCPB is staged by package-mcpb.mjs, so its one seam scan must run unconditionally, right
// after the binary is found: no second call, no alias, nothing between them.
const packageMcpb = readFileSync(resolve(repositoryRoot, "scripts/package-mcpb.mjs"), "utf8");
const packageSeamScan = [
  "    throw new Error(`MCPB binary is missing: ${sourceBinary}`);",
  "  }",
  "  // Every MCPB, from CI, a release or a local build, is staged here: refuse a",
  "  // binary compiled with the test-only approval seam (bridge#583).",
  "  assertNoTestSeam([sourceBinary]);",
].join("\n");
if (packageMcpb.split("assertNoTestSeam").length !== 3 || packageMcpb.split(packageSeamScan).length !== 2) {
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
  "        run: |",
  "          set -euo pipefail",
  "",
  "          if [[ \"$EVENT_NAME\" == \"workflow_dispatch\" ]]; then",
  "            echo 'bundle=true' >> \"$GITHUB_OUTPUT\"",
  "            echo 'native=true' >> \"$GITHUB_OUTPUT\"",
  "            echo 'tax_audit=true' >> \"$GITHUB_OUTPUT\"",
  "            exit 0",
  "          fi",
  "",
  "          if [[ \"$EVENT_NAME\" == \"pull_request\" ]]; then",
  "            base=\"$PR_BASE_SHA\"",
  "          else",
  "            base=\"$BEFORE_SHA\"",
  "          fi",
  "",
  "          # A new branch has no useful predecessor. Keep the conservative path.",
  "          if [[ -z \"$base\" || \"$base\" =~ ^0+$ ]]; then",
  "            echo 'bundle=true' >> \"$GITHUB_OUTPUT\"",
  "            echo 'native=true' >> \"$GITHUB_OUTPUT\"",
  "            echo 'tax_audit=true' >> \"$GITHUB_OUTPUT\"",
  "            exit 0",
  "          fi",
  "",
  "          # --no-renames lists a moved file under both paths, so a file moved out of a gated directory still selects it.",
  "          # -z: git would otherwise quote a path with non-ASCII bytes, and the quoted form matches no prefix below.",
  "          changed_files=\"$(git diff --name-only --no-renames -z \"$base\" \"$GITHUB_SHA\" | tr '\\0' '\\n')\"",
  "          if printf '%s\\n' \"$changed_files\" | grep -Eq '^(\\.github/workflows/|\\.github/actions/setup-windows-native/|rust-toolchain\\.toml|src-tauri/|tools/|scripts/package-mcpb\\.mjs|scripts/check-no-test-seam(\\.test)?\\.mjs)'; then",
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
// cannot hide from required-checks' needs behind a comment or a quoted key.
function jobIds(source) {
  const lines = source.split(/\r?\n/);
  const start = lines.indexOf("jobs:");
  if (start === -1) failures.push("CI workflow has no block-style jobs: map");
  const ids = [];
  for (const line of lines.slice(start + 1)) {
    if (/^\S/.test(line)) break;
    if (!/^  [^\s#]/.test(line)) continue;
    const id = line.match(/^  ([A-Za-z0-9_-]+):$/)?.[1];
    if (id) ids.push(id);
    else failures.push(`CI workflow has a job key this gate cannot read: ${line}`);
  }
  return ids;
}

function stepBlock(job, head) {
  const lines = job.split("\n");
  const starts = lines.flatMap((line, index) => (line === head ? [index] : []));
  if (starts.length !== 1) return undefined;
  let end = starts[0] + 1;
  while (end < lines.length && !/^ {0,6}\S/.test(lines[end])) end += 1;
  return lines.slice(starts[0], end).join("\n").trimEnd();
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
  const tracked = spawnSync("git", ["-C", repositoryRoot, "ls-files", "-z"], {
    encoding: "utf8",
    windowsHide: true,
  });
  if (tracked.error || tracked.status !== 0) {
    const detail = tracked.error?.message ?? tracked.stderr.trim() ?? "unknown error";
    throw new Error(`git ls-files failed: ${detail}`);
  }

  const stale = [];
  for (const file of tracked.stdout.split("\0").filter(Boolean)) {
    const contents = readFileSync(resolve(repositoryRoot, file), "utf8");
    for (const path of legacyPaths) {
      if (contents.includes(path)) stale.push({ file, path });
    }
  }
  return stale;
}

function relativePath(path) {
  return path.slice(repositoryRoot.length + 1).replaceAll("\\", "/");
}
