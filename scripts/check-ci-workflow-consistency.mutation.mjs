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
  ["narrower cache workspaces", (text) => text.replace("            tools -> target\n          # Separate the Windows", "          # Separate the Windows")],
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

// `continue-on-error` is refused except on four pinned, diagnostics-only artifact uploads. Each case below
// makes one change a weaker rule would let through; the unmodified tree has exactly four allowed flags.
const CI_FILE = ".github/workflows/ci.yml";
const FLAG = "        continue-on-error: true";
const UPLOAD = "actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a";
const TIMINGS = `${FLAG}\n        uses: ${UPLOAD} # v7.0.1\n        with:\n          name: native-compiler-timings`;
const AFTER_NATIVE_PIN = "      - name: Test native workspace doctests\n";
const ONLY_EXACT = "ci.yml must not use continue-on-error except the one exact line in each pinned diagnostics-only upload";
const UPLOAD_SHAPE = "changed shape; review it and update diagnosticUploads";

test("the unmodified tree has exactly four allowed flags", () => {
  assert.equal(readFileSync(join(copy, CI_FILE), "utf8").split("\n").filter((line) => line === FLAG).length, 4);
});

for (const [name, change, messages] of [
  ["a flag on a step that is not one of the four", (text) => text.replace(AFTER_NATIVE_PIN, `${AFTER_NATIVE_PIN}${FLAG}\n`), [ONLY_EXACT]],
  ["a job-level flag", (text) => text.replace("  seam-control:\n", "  seam-control:\n    continue-on-error: true\n"), [ONLY_EXACT]],
  ["a flag moved from an allowed step to another step", (text) => text.replace(TIMINGS, TIMINGS.slice(FLAG.length + 1)).replace(AFTER_NATIVE_PIN, `${AFTER_NATIVE_PIN}${FLAG}\n`), [UPLOAD_SHAPE]],
  ["an id on an allowed step", (text) => text.replace(TIMINGS, TIMINGS.replace(FLAG, `${FLAG}\n        id: timings`)), [UPLOAD_SHAPE]],
  ["an expression instead of true on an allowed step", (text) => text.replace(TIMINGS, TIMINGS.replace(FLAG, "        continue-on-error: ${{ always() }}")), [ONLY_EXACT, UPLOAD_SHAPE]],
  ["a different upload action on an allowed step", (text) => text.replace(TIMINGS, TIMINGS.replace("043fb46d", "043fb46e")), [UPLOAD_SHAPE]],
]) {
  test(`diagnostics uploads: ${name} is refused`, () => {
    const result = withChange(CI_FILE, change);
    for (const message of messages) refused(result, message);
  });
}

test("diagnostics uploads: an allowed step moved to another job is refused although the count stays four", () => {
  const step = [
    "      - name: Retain native compiler timings",
    "        if: ${{ always() }}",
    FLAG,
    `        uses: ${UPLOAD} # v7.0.1`,
    "        with:",
    "          name: native-compiler-timings-${{ matrix.os }}",
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
const PLAYWRIGHT_RESTORE_KEY = "          key: playwright-${{ runner.os }}-${{ steps.playwright-version.outputs.version }}\n      - name: Install Playwright browsers and system dependencies";
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
  ["restore-keys added to the restore step", (text) => text.replace(PLAYWRIGHT_RESTORE_KEY, PLAYWRIGHT_RESTORE_KEY.replace("\n      - name", "\n          restore-keys: playwright-\n      - name")), RESTORE_SHAPE],
]) {
  test(`cache saving: ${name} is refused`, () => {
    refused(withChange(CI_FILE, change), message);
  });
}

test("diagnostics uploads: the flag in any other workflow is refused", () => {
  refused(withChange(".github/workflows/release-mcpb-preview.yml", (text) => text.replace(/(\n {6}- uses: actions\/checkout)/, `\n${FLAG}$1`)), "release-mcpb-preview.yml must not use continue-on-error");
});

test("a quoted mapping key at the start of a line is refused (an escaped spelling would not contain the refused text)", () => {
  refused(withChange(CI_FILE, (text) => text.replace(AFTER_NATIVE_PIN, `${AFTER_NATIVE_PIN}        "continue\\x2Don-error": true\n`)), "must not start a line with a quoted mapping key");
});
