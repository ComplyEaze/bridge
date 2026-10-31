// SPDX-License-Identifier: Apache-2.0
// Run by the workflow-consistency job (`node --test scripts/check-ci-workflow-consistency.mutation.mjs`),
// not by `pnpm test`: the check runs `cargo metadata`, which needs that job's pinned toolchain.
// Each case runs the real check against a copy of the tracked tree with one change that a weaker rule
// would let through, and asserts the check's own message for it, so a failure for another reason
// cannot pass as a kill. The unmodified copy must pass first.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { cpSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

const root = fileURLToPath(new URL("../", import.meta.url));
const git = (cwd, ...args) => {
  const result = spawnSync("git", args, { cwd, encoding: "utf8", windowsHide: true });
  assert.equal(result.status, 0, result.stderr);
  return result.stdout;
};

const copy = mkdtempSync(join(tmpdir(), "ci-consistency-"));
test.after(() => rmSync(copy, { recursive: true, force: true }));
for (const file of git(root, "ls-files", "-z").split("\0").filter(Boolean)) {
  mkdirSync(dirname(join(copy, file)), { recursive: true });
  cpSync(join(root, file), join(copy, file));
}
git(copy, "init", "-q");
git(copy, "add", "-A");

const check = () => spawnSync(process.execPath, [join(root, "scripts/check-ci-workflow-consistency.mjs"), "--root", copy], { encoding: "utf8", windowsHide: true });
const withChange = (file, change) => {
  const original = readFileSync(join(copy, file), "utf8");
  const changed = change(original);
  assert.notEqual(changed, original, "the change must alter the file");
  writeFileSync(join(copy, file), changed);
  try { return check(); } finally { writeFileSync(join(copy, file), original); }
};
const refused = (result, message) => {
  assert.equal(result.status, 1, result.stdout + result.stderr);
  assert.ok((result.stderr + result.stdout).includes(message), `expected "${message}" in:\n${result.stderr}`);
};

test("the unmodified tree passes", () => {
  const result = check();
  assert.equal(result.status, 0, result.stderr);
});

// Each pinned step is read as text. YAML ignores a comment's indentation, so a comment at step
// indentation followed by a deeper line is still part of the step, and the pinned text must include it.
const PINNED = [
  [".github/workflows/ci.yml", "native", "      - name: Prove the approval-seam scan sees a test build"],
  [".github/workflows/ci.yml", "bundle-smoke", "      - name: Prove shipped executables lack the test-only approval seam"],
  [".github/workflows/ci.yml", "workflow-consistency", "      - run: node scripts/check-ci-workflow-consistency.mjs"],
  [".github/workflows/release-mcpb-preview.yml", "package", "      - name: Prove the release binary lacks the test-only approval seam"],
];
// A comment may be indented anywhere (0 to 6 spaces) before the deeper line, and a whitespace-only line
// wider than the step indentation is part of the step too.
const SHAPES = [
  ["a comment at step indentation, then a deeper line", ["      # note", "        working-directory: ."]],
  ["a comment at column 0, then a deeper line", ["# note", "        working-directory: ."]],
  ["a comment at 3 spaces, then a deeper line", ["   # note", "        working-directory: ."]],
  ["a blank line, then a deeper line", ["", "        working-directory: ."]],
  ["a whitespace-only line wider than the step indentation", ["            "]],
];
for (const [file, job, head] of PINNED) {
  for (const [shape, inserted] of SHAPES) {
    test(`${job}: ${shape} after the pinned step changes its pinned text`, () => {
      const result = withChange(file, (text) => {
        const lines = text.split("\n");
        const start = lines.findIndex((line) => line === `  ${job}:`);
        const step = lines.findIndex((line, index) => index > start && line === head);
        assert.ok(start !== -1 && step !== -1, `${job} and its pinned step exist`);
        const next = lines.findIndex((line, index) => index > step && /^ {0,6}- /.test(line));
        assert.ok(next !== -1, `${job} has a step after the pinned one`);
        lines.splice(next, 0, ...inserted);
        return lines.join("\n");
      });
      refused(result, `${job} changed before "${head.trim()}"`);
    });
  }
}

// native's dependency-cache inputs (the shared key, the workspaces, the hashed env vars and the CARGO_PROFILE
// lines) sit before its pinned step, so a change to one is a changed job: a difference in them is a cold
// build on every run, which no test failure would show.
const NATIVE_CACHE_CASES = [
  ["a changed CARGO_PROFILE variable", (text) => text.replace("      CARGO_PROFILE_TEST_DEBUG: '0'\n    # A cold Windows native build", "      CARGO_PROFILE_TEST_DEBUG: '1'\n    # A cold Windows native build")],
  ["narrower cache workspaces", (text) => text.replace("            src-tauri -> target\n          # Separate the Windows", "          # Separate the Windows")],
  ["a different env-var list", (text) => text.replace("'ImageOS ImageVersion OPENSSL LIBCLANG' || ''", "'ImageOS ImageVersion OPENSSL' || ''")],
];
for (const [name, change] of NATIVE_CACHE_CASES) {
  test(`native: ${name} is refused`, () => {
    refused(withChange(".github/workflows/ci.yml", change), 'native changed before "- name: Prove the approval-seam scan sees a test build"');
  });
}

// The digest of a pinned job covers it only up to the pinned step, so a job-level key written after the
// steps would be outside the pinned text: `steps:` must be the job's last key.
for (const [file, job] of PINNED) {
  for (const [shape, inserted] of [
    ["a job-level key", ["    container: node:22"]],
    ["a job-level key after a comment at job-key indentation", ["    # note", "    container: node:22"]],
    ["a quoted job-level key", ['    "container": node:22']],
    ["a job-level key with a non-breaking space before it", ["    \u00a0container: node:22"]],
  ]) {
    test(`${job}: ${shape} after the job's steps is refused`, () => {
      const result = withChange(file, (text) => {
        const lines = text.split("\n");
        const start = lines.findIndex((line) => line === `  ${job}:`);
        assert.notEqual(start, -1, `${job} exists`);
        let end = lines.findIndex((line, index) => index > start && /^  [A-Za-z0-9_-]+:\s*$/.test(line));
        if (end === -1) end = lines.length;
        while (end > start + 1 && lines[end - 1].trim() === "") end -= 1;
        lines.splice(end, 0, ...inserted);
        return lines.join("\n");
      });
      refused(result, `${job}: \`steps:\` must be the job's last key`);
    });
  }
}

test("a lone carriage return, or a Unicode line separator, in either workflow is refused", () => {
  for (const [file, text] of [[".github/workflows/ci.yml", "\r"], [".github/workflows/ci.yml", "\u2028"], [".github/workflows/release-mcpb-preview.yml", "\u0085"]]) {
    refused(withChange(file, (original) => `${original.trimEnd()}\n#${text}x\n`), "must use only \\n or \\r\\n line breaks");
  }
});

test("an unexpected job in the release workflow is refused", () => {
  refused(withChange(".github/workflows/release-mcpb-preview.yml", (text) => `${text.trimEnd()}\n  extra-job:\n    runs-on: ubuntu-latest\n`), "must have exactly the jobs release-admission, package, attest and publish-preview");
});

// `continue-on-error` is refused except on five pinned, diagnostics-only artifact uploads. Each case below
// makes one change a weaker rule would let through; the unmodified tree has exactly five allowed flags.
const CI_FILE = ".github/workflows/ci.yml";
const FLAG = "        continue-on-error: true";
const UPLOAD = "actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a";
const TIMINGS = `${FLAG}\n        uses: ${UPLOAD} # v7.0.1\n        with:\n          name: native-compiler-timings`;
const TEST_TIMINGS_STEP = [
  "      - name: Retain macOS per-test timings",
  "        if: ${{ always() && runner.os == 'macOS' && steps.native-tests.outcome == 'success' }}",
  FLAG,
  `        uses: ${UPLOAD} # v7.0.1`,
  "        with:",
  "          name: native-test-timings-macos-shard-${{ matrix.shard }}",
  "          path: src-tauri/target/nextest/default/junit.xml",
  "          if-no-files-found: warn",
  "          retention-days: 7",
].join("\n");
const AFTER_NATIVE_PIN = "      - name: Test native workspace doctests\n";
const ONLY_EXACT = "ci.yml must not use continue-on-error except the one exact line in each pinned diagnostics-only upload";
const UPLOAD_SHAPE = "changed shape; review it and update diagnosticUploads";

test("the unmodified tree has exactly five allowed flags", () => {
  assert.equal(readFileSync(join(copy, CI_FILE), "utf8").split("\n").filter((line) => line === FLAG).length, 5);
});

for (const [name, change, messages] of [
  ["a flag on a step that is not one of the five", (text) => text.replace(AFTER_NATIVE_PIN, `${AFTER_NATIVE_PIN}${FLAG}\n`), [ONLY_EXACT]],
  ["a job-level flag", (text) => text.replace("  seam-control:\n", "  seam-control:\n    continue-on-error: true\n"), [ONLY_EXACT]],
  ["a flag moved from an allowed step to another step", (text) => text.replace(TIMINGS, TIMINGS.slice(FLAG.length + 1)).replace(AFTER_NATIVE_PIN, `${AFTER_NATIVE_PIN}${FLAG}\n`), [UPLOAD_SHAPE]],
  ["an id on an allowed step", (text) => text.replace(TIMINGS, TIMINGS.replace(FLAG, `${FLAG}\n        id: timings`)), [UPLOAD_SHAPE]],
  ["an expression instead of true on an allowed step", (text) => text.replace(TIMINGS, TIMINGS.replace(FLAG, "        continue-on-error: ${{ always() }}")), [ONLY_EXACT, UPLOAD_SHAPE]],
  ["a different upload action on an allowed step", (text) => text.replace(TIMINGS, TIMINGS.replace("043fb46d", "043fb46e")), [UPLOAD_SHAPE]],
  // The per-test timings report carries a failing test's panic text, so it is uploaded only after a passing test step.
  ["the per-test timings upload without its passing-test condition", (text) => text.replace(TEST_TIMINGS_STEP, TEST_TIMINGS_STEP.replace(" && steps.native-tests.outcome == 'success'", "")), [UPLOAD_SHAPE]],
  ["the per-test timings upload on every operating system", (text) => text.replace(TEST_TIMINGS_STEP, TEST_TIMINGS_STEP.replace(" && runner.os == 'macOS'", "")), [UPLOAD_SHAPE]],
  ["a wider path on the per-test timings upload", (text) => text.replace(TEST_TIMINGS_STEP, TEST_TIMINGS_STEP.replace("default/junit.xml", "default/*")), [UPLOAD_SHAPE]],
  ["a longer retention on the per-test timings upload", (text) => text.replace(TEST_TIMINGS_STEP, TEST_TIMINGS_STEP.replace("retention-days: 7", "retention-days: 90")), [UPLOAD_SHAPE]],
]) {
  test(`diagnostics uploads: ${name} is refused`, () => {
    const result = withChange(CI_FILE, change);
    for (const message of messages) refused(result, message);
  });
}

test("diagnostics uploads: an allowed step moved to another job is refused although the count stays five", () => {
  const step = [
    "      - name: Retain native compiler timings",
    "        if: ${{ always() }}",
    FLAG,
    `        uses: ${UPLOAD} # v7.0.1`,
    "        with:",
    "          name: native-compiler-timings-${{ matrix.os }}-shard-${{ matrix.shard }}",
    "          path: src-tauri/target/cargo-timings/*.html",
    "          if-no-files-found: warn",
    "          retention-days: 7",
  ].join("\n");
  const result = withChange(CI_FILE, (text) => {
    assert.equal(text.split(step).length, 2, "the step exists once");
    const without = text.replace(`${step}\n`, "");
    const jobStart = without.indexOf("  seam-control:\n");
    const jobEnd = without.indexOf("  compiler-cache-retention:\n");
    assert.ok(jobStart !== -1 && jobEnd > jobStart, "seam-control block exists");
    const block = without.slice(jobStart, jobEnd).trimEnd();
    return `${without.slice(0, jobStart)}${block}\n${step}\n\n${without.slice(jobEnd)}`;
  });
  refused(result, UPLOAD_SHAPE);
});

// The Playwright browsers cache is saved only by push, manual and scheduled runs. The combined restore-and-save cache action saves from
// every ref, and a pull-request or merge-queue ref saves a copy only that ref can read: on 15 Oct 2026 five copies of
// the 365 MiB Playwright browsers, saved by pull-request and merge-queue refs, sat in a repository cache at 9.96 of 10 GiB.
const PLAYWRIGHT_SAVE = [
  "      - name: Save Playwright browsers",
  "        if: ${{ steps.playwright-cache.outputs.cache-hit != 'true' && (github.event_name == 'push' || github.event_name == 'workflow_dispatch' || github.event_name == 'schedule') }}",
  "        uses: actions/cache/save@55cc8345863c7cc4c66a329aec7e433d2d1c52a9 # v6.1.0",
  "        with:",
  "          path: ~/.cache/ms-playwright",
  "          key: playwright-${{ runner.os }}-${{ steps.playwright-version.outputs.version }}",
].join("\n");
const COMBINED = "ci.yml must not use the combined restore-and-save cache action (actions/cache@)";
const PLAYWRIGHT_RESTORE_KEY = "          key: playwright-${{ runner.os }}-${{ steps.playwright-version.outputs.version }}\n      # The system packages Playwright needs";
const PLAYWRIGHT_INSTALL_HEAD = "      - name: Install Playwright browsers and system dependencies\n";
const SAVE_SHAPE = "the Playwright browsers save step changed shape";
const RESTORE_SHAPE = "the Playwright browsers restore step changed shape";
for (const [name, change, message] of [
  ["the combined action back in the restore step", (text) => text.replace("        id: playwright-cache\n        uses: actions/cache/restore@", "        id: playwright-cache\n        uses: actions/cache@"), COMBINED],
  ["the save step without its event condition", (text) => text.replace(PLAYWRIGHT_SAVE, PLAYWRIGHT_SAVE.replace(" && (github.event_name == 'push' || github.event_name == 'workflow_dispatch' || github.event_name == 'schedule')", "")), "the Playwright browsers save step changed shape"],
  ["the save step on pull requests too", (text) => text.replace(PLAYWRIGHT_SAVE, PLAYWRIGHT_SAVE.replace(" || github.event_name == 'schedule')", " || github.event_name == 'schedule' || github.event_name == 'pull_request')")), "the Playwright browsers save step changed shape"],
  ["the save step removed", (text) => text.replace(`${PLAYWRIGHT_SAVE}\n`, ""), "the Playwright browsers save step changed shape"],
  ["the save step moved before the install step", (text) => text.replace(`${PLAYWRIGHT_SAVE}\n`, "").replace(PLAYWRIGHT_INSTALL_HEAD, `${PLAYWRIGHT_SAVE}\n${PLAYWRIGHT_INSTALL_HEAD}`), SAVE_SHAPE],
  ["a second cache save step in the job", (text) => text.replace(`${PLAYWRIGHT_SAVE}\n`, `${PLAYWRIGHT_SAVE}\n${PLAYWRIGHT_SAVE.replace("Save Playwright browsers", "Save more browsers")}\n`), SAVE_SHAPE],
  ["a restore key that differs from the save key", (text) => text.replace(PLAYWRIGHT_RESTORE_KEY, PLAYWRIGHT_RESTORE_KEY.replace("key: playwright-", "key: playwright-v2-")), RESTORE_SHAPE],
  ["restore-keys added to the restore step", (text) => text.replace(PLAYWRIGHT_RESTORE_KEY, PLAYWRIGHT_RESTORE_KEY.replace("\n      #", "\n          restore-keys: playwright-\n      #")), RESTORE_SHAPE],
]) {
  test(`cache saving: ${name} is refused`, () => {
    refused(withChange(CI_FILE, change), message);
  });
}

// The Playwright system packages (the .deb files) are cached and saved under the same rules (bridge#2056): a write from a
// pull-request or merge-queue ref would plant what a later run copies into apt's archive directory.
const DEBS_KEY = "          key: playwright-debs-${{ runner.os }}-${{ steps.playwright-version.outputs.version }}-${{ steps.playwright-version.outputs.image }}";
const DEBS_SAVE = [
  "      - name: Save Playwright system packages",
  "        if: ${{ steps.apt-cache.outputs.cache-hit != 'true' && (github.event_name == 'push' || github.event_name == 'workflow_dispatch' || github.event_name == 'schedule') }}",
  "        uses: actions/cache/save@55cc8345863c7cc4c66a329aec7e433d2d1c52a9 # v6.1.0",
  "        with:",
  "          path: ~/apt-debs",
  DEBS_KEY,
].join("\n");
const DEBS_RESTORE_KEY = `${DEBS_KEY}\n      - name: Install Playwright browsers and system dependencies`;
const DEBS_SAVE_SHAPE = "the Playwright system packages save step changed shape";
const DEBS_RESTORE_SHAPE = "the Playwright system packages restore step changed shape";
for (const [name, change, message] of [
  ["the packages save step on pull requests too", (text) => text.replace(DEBS_SAVE, DEBS_SAVE.replace(" || github.event_name == 'schedule')", " || github.event_name == 'schedule' || github.event_name == 'pull_request')")), DEBS_SAVE_SHAPE],
  ["the packages save step on the merge queue too", (text) => text.replace(DEBS_SAVE, DEBS_SAVE.replace(" || github.event_name == 'schedule')", " || github.event_name == 'schedule' || github.event_name == 'merge_group')")), DEBS_SAVE_SHAPE],
  ["the packages save step without its event condition", (text) => text.replace(DEBS_SAVE, DEBS_SAVE.replace(" && (github.event_name == 'push' || github.event_name == 'workflow_dispatch' || github.event_name == 'schedule')", "")), DEBS_SAVE_SHAPE],
  ["the packages save step removed", (text) => text.replace(`${DEBS_SAVE}\n`, ""), "the Playwright browsers save step changed shape"],
  ["the packages save step moved before the install step", (text) => text.replace(`${DEBS_SAVE}\n`, "").replace(PLAYWRIGHT_INSTALL_HEAD, `${DEBS_SAVE}\n${PLAYWRIGHT_INSTALL_HEAD}`), DEBS_SAVE_SHAPE],
  ["a packages key without the runner image", (text) => text.split(DEBS_KEY).join(DEBS_KEY.replace("-${{ steps.playwright-version.outputs.image }}", "")), DEBS_RESTORE_SHAPE],
  ["restore-keys added to the packages restore step", (text) => text.replace(DEBS_RESTORE_KEY, DEBS_RESTORE_KEY.replace("\n      - name", "\n          restore-keys: playwright-debs-\n      - name")), DEBS_RESTORE_SHAPE],
  ["the combined action for the packages restore", (text) => text.replace("        id: apt-cache\n        uses: actions/cache/restore@", "        id: apt-cache\n        uses: actions/cache@"), COMBINED],
]) {
  test(`package cache saving: ${name} is refused`, () => {
    refused(withChange(CI_FILE, change), message);
  });
}

// The per-test timings artifact is public for 7 days. What it carries is set in nextest.toml, outside the
// pinned upload step, so the file is pinned too: exactly one JUnit table, with the output stores off, and no
// setup scripts (their command, arguments and environment names would join the report).
const NEXTEST = "src-tauri/.config/nextest.toml";
const NEXTEST_MESSAGE = "src-tauri/.config/nextest.toml must hold exactly one JUnit report table";
for (const [name, change] of [
  ["a passing test's output stored", (text) => text.replace("store-success-output = false", "store-success-output = true")],
  ["a failing test's output stored", (text) => text.replace("store-failure-output = false", "store-failure-output = true")],
  ["the report written elsewhere", (text) => text.replace('path = "junit.xml"', 'path = "/tmp/junit.xml"')],
  ["a second profile with a report", (text) => `${text.trimEnd()}\n\n[profile.ci.junit]\npath = "ci.xml"\n`],
  ["a setup script section", (text) => `${text.trimEnd()}\n\n[scripts.setup.probe]\ncommand = "echo"\n`],
  ["a property-adding profile script", (text) => `${text.trimEnd()}\n\n[[profile.default.scripts]]\nfilter = "all()"\nsetup = "probe"\n`],
  // The same tables without a bracketed header: valid TOML that nextest reads as setup scripts.
  ["a setup script as a top-level dotted key", (text) => `scripts.setup.probe.command = "echo"\n${text}`],
  ["a setup script as a top-level inline table", (text) => `scripts = { setup = { probe = { command = "echo" } } }\n${text}`],
  ["a profile script as an inline array", (text) => text.replace("[profile.default]\n", '[profile.default]\nscripts = [{ filter = "all()", setup = "probe" }]\n')],
  ["a setup script under a key spelled with an escape", (text) => `"scr\\u0069pts".setup.probe.command = "echo"\n${text}`],
]) {
  test(`nextest report: ${name} is refused`, () => {
    refused(withChange(NEXTEST, change), NEXTEST_MESSAGE);
  });
}

// Comments are not pinned: the pin reads every line that is not blank or a full-line comment.
test("nextest report: a changed comment line is accepted", () => {
  const result = withChange(NEXTEST, (text) => text.replace("# Hang guard for the native test run", "# Hang guard, reworded, for the native test run"));
  assert.equal(result.status, 0, result.stderr);
});

test("diagnostics uploads: the flag in any other workflow is refused", () => {
  refused(withChange(".github/workflows/release-mcpb-preview.yml", (text) => text.replace(/(\n {6}- uses: actions\/checkout)/, `\n${FLAG}$1`)), "release-mcpb-preview.yml must not use continue-on-error");
});

test("a quoted mapping key at the start of a line is refused (an escaped spelling would not contain the refused text)", () => {
  refused(withChange(CI_FILE, (text) => text.replace(AFTER_NATIVE_PIN, `${AFTER_NATIVE_PIN}        "continue\\x2Don-error": true\n`)), "must not start a line with a quoted mapping key");
});

// The native tests run as two halves (shard 1 and 2). A half that never runs is a green check with half the
// tests missing, so each way to lose one is a case: the shard list, the partition count, the partition flag,
// and a once-per-system step whose shard condition is changed, dropped or moved, or a stray use of the shard number.
const SHARD_ROW = "        shard: [1, 2]";
const PARTITION = "          --partition count:${{ matrix.shard }}/2";
const TWO_SHARDS = "native must split its tests into exactly two shards";
const OWN_HALF = "native: the test step must run nextest on its own half of the tests";
const ONCE = "so it runs once per operating system";
const STRAY = "native: matrix.shard may appear only in the pinned lines";

test("native shards: the unmodified workflow carries both shards, the partition flag and five once-per-system conditions", () => {
  const text = readFileSync(join(copy, CI_FILE), "utf8");
  assert.equal(text.split("\n").filter((line) => line === SHARD_ROW).length, 1);
  assert.equal(text.split("\n").filter((line) => line === PARTITION).length, 1);
  assert.equal(text.split("\n").filter((line) => line === "        if: matrix.shard == 1").length, 3);
  assert.equal(text.split("\n").filter((line) => line === "        if: matrix.shard == 2").length, 2);
});

for (const [name, change, message] of [
  ["a shard list with one shard runs half the tests", (text) => text.replace(SHARD_ROW, "        shard: [1]"), TWO_SHARDS],
  ["a shard list with a third shard that the partition does not know", (text) => text.replace(SHARD_ROW, "        shard: [1, 2, 3]"), TWO_SHARDS],
  ["a second shard row", (text) => text.replace(SHARD_ROW, `${SHARD_ROW}\n        shard: [1, 2]`), TWO_SHARDS],
  ["a partition count of three", (text) => text.replace(PARTITION, "          --partition count:${{ matrix.shard }}/3"), OWN_HALF],
  ["the partition flag removed (both shards run every test)", (text) => text.replace(`\n${PARTITION}`, ""), OWN_HALF],
  ["the partition fixed to shard one", (text) => text.replace(PARTITION, "          --partition count:1/2"), OWN_HALF],
  ["a retry added to the test step", (text) => text.replace("--retries 0 --test-threads num-cpus --timings", "--retries 2 --test-threads num-cpus --timings"), OWN_HALF],
  ["the doctests step condition moved to a shard that does not exist", (text) => text.replace("      - name: Test native workspace doctests\n        if: matrix.shard == 1", "      - name: Test native workspace doctests\n        if: matrix.shard == 3"), ONCE],
  ["the doctests step condition dropped (it would run twice)", (text) => text.replace("      - name: Test native workspace doctests\n        if: matrix.shard == 1\n", "      - name: Test native workspace doctests\n"), ONCE],
  ["the clippy step moved to shard one", (text) => text.replace("      - name: Lint the native workspace\n        if: matrix.shard == 2", "      - name: Lint the native workspace\n        if: matrix.shard == 1"), ONCE],
  ["the egress census condition made always true", (text) => text.replace("      - name: Census the Tally-path egress lints\n        if: matrix.shard == 2", "      - name: Census the Tally-path egress lints\n        if: matrix.shard >= 1"), ONCE],
  ["the PDFium tests step condition dropped", (text) => text.replace("      - name: Test PDF extraction through PDFium\n        if: matrix.shard == 1\n", "      - name: Test PDF extraction through PDFium\n"), ONCE],
  ["a shard condition on a step that is not pinned (the seam proof would skip a shard)", (text) => text.replace("      - name: Prove the approval-seam scan sees a test build\n", "      - name: Prove the approval-seam scan sees a test build\n        if: matrix.shard == 1\n"), STRAY],
  ["a shard condition on the native job's own checkout step", (text) => { const at = text.indexOf("\n  native:\n"); assert.ok(at !== -1); const head = text.slice(0, at); const rest = text.slice(at); return head + rest.replace("      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7\n", "      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7\n        if: matrix.shard == 1\n"); }, STRAY],
]) {
  test(`native shards: ${name} is refused`, () => {
    refused(withChange(CI_FILE, change), message);
  });
}

for (const [name, change] of [
  ["an exclude entry that drops half the macOS tests", (text) => text.replace(SHARD_ROW, `${SHARD_ROW}\n        exclude:\n          - os: macos-latest\n            shard: 2`)],
  ["an include entry that adds a leg", (text) => text.replace(SHARD_ROW, `${SHARD_ROW}\n        include:\n          - os: macos-latest\n            shard: 3`)],
]) {
  test(`native shards: ${name} is refused`, () => {
    refused(withChange(CI_FILE, change), "native: the matrix must not use include: or exclude:");
  });
}

test("native shards: the Windows prerequisites cache saved by both shards (the shard condition dropped) is refused", () => {
  refused(withChange(CI_FILE, (text) => text.replace("          save-cache: ${{ matrix.shard == 1 }}", "          save-cache: true")), "native: the pinned shard line must appear exactly once: save-cache: ${{ matrix.shard == 1 }}");
});

test("native shards: both shards uploading one artifact name (a name without the shard) is refused", () => {
  refused(withChange(CI_FILE, (text) => text.replace("          name: native-crash-binaries-macos-shard-${{ matrix.shard }}", "          name: native-crash-binaries-macos")), "native: the pinned shard line must appear exactly once: name: native-crash-binaries-macos-shard-${{ matrix.shard }}");
});

// The scope job names the operating systems of the native tests (Windows on a pull request and in the queue, macOS added on a
// push, the nightly and a manual run). A matrix with a fixed list would drop macOS from those runs and every check would pass.
for (const [name, replacement] of [
  ["a fixed Windows-only list", "os: [windows-latest]"],
  ["the bundle job's system list", "os: ${{ fromJSON(needs.changes.outputs.bundle_os) }}"],
]) {
  test(`native systems: ${name} is refused`, () => {
    refused(withChange(".github/workflows/ci.yml", (text) => text.replace("        os: ${{ fromJSON(needs.changes.outputs.native_os) }}", `        ${replacement}`)),
      "native must run on the systems the scope job's native_os names");
  });
}

// A draft-scoped run must post a name no ruleset requires. If it posted `Required checks`, its green (heavy jobs skipped on
// purpose) would satisfy the ruleset on a head whose ready run has not started its own gate yet.
for (const [name, replacement] of [
  ["a constant name", "name: Required checks"],
  ["the draft name made the required one", "name: ${{ needs.changes.outputs.draft == 'true' && 'Required checks' || 'Required checks' }}"],
  ["the condition inverted", "name: ${{ needs.changes.outputs.draft != 'true' && 'Required checks (draft)' || 'Required checks' }}"],
]) {
  test(`required checks name: ${name} is refused`, () => {
    refused(withChange(".github/workflows/ci.yml", (text) => text.replace("    name: ${{ needs.changes.outputs.draft == 'true' && 'Required checks (draft)' || 'Required checks' }}", `    ${replacement}`)),
      "required-checks changed shape, or does not need every other job");
  });
}

// The merge gate's test is wired to the merge-gate-scope step's output. A step that runs it always, never, or twice, or that
// loses the scope step, changes what a merge-queue run proves.
for (const [name, change] of [
  ["its condition inverted", (text) => text.replace("      - if: steps.merge-gate-scope.outputs.run == 'true'\n        run: python3 scripts/merge-gate.test.py", "      - if: steps.merge-gate-scope.outputs.run != 'true'\n        run: python3 scripts/merge-gate.test.py")],
  ["its condition dropped", (text) => text.replace("      - if: steps.merge-gate-scope.outputs.run == 'true'\n        run: python3 scripts/merge-gate.test.py", "      - run: python3 scripts/merge-gate.test.py")],
  ["its condition made always()", (text) => text.replace("      - if: steps.merge-gate-scope.outputs.run == 'true'\n", "      - if: always()\n")],
  ["the test step deleted", (text) => text.replace("      - if: steps.merge-gate-scope.outputs.run == 'true'\n        run: python3 scripts/merge-gate.test.py\n", "")],
  ["the scope step renamed", (text) => text.replace("      - id: merge-gate-scope\n", "      - id: merge-gate-scoped\n")],
]) {
  test(`merge gate test wiring: ${name} is refused`, () => {
    refused(withChange(".github/workflows/ci.yml", change), "the merge gate's test must run once, exactly when the merge-gate-scope step says so");
  });
}
test("merge gate scope: a base SHA taken from the pull request's head is refused", () => {
  const scope = "      - id: merge-gate-scope\n        env:\n          EVENT_NAME: ${{ github.event_name }}\n          PR_BASE_SHA: ${{ github.event.pull_request.";
  refused(withChange(".github/workflows/ci.yml", (text) => text.replace(`${scope}base.sha }}\n`, `${scope}head.sha }}\n`)),
    "the merge-gate-scope step must read the event name and the pull request's base SHA");
});

// The nightly records run must not hold eight of the account's twenty runner slots at once.
for (const [name, replacement] of [
  ["the limit removed", ""],
  ["the limit raised to eight", "      max-parallel: 8\n"],
]) {
  test(`mutation shards: ${name} is refused`, () => {
    refused(withChange(".github/workflows/tax-audit-mutations-nightly.yml", (text) => text.replace("      max-parallel: 4\n", replacement)),
      "the mutation shards must run four at a time");
  });
}
