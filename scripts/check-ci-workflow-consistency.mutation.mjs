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

// native-legacy is pinned whole, and the cache inputs it shares with native must stay equal: a difference
// in them is a cold build on every run, which no test failure would show.
const inLegacyJob = (change) => (text) => {
  const start = text.indexOf("  native-legacy:\n");
  const end = text.indexOf("  bundle-smoke:\n    name: Bundle smoke");
  assert.ok(start !== -1 && end > start, "native-legacy block exists");
  return text.slice(0, start) + change(text.slice(start, end)) + text.slice(end);
};
const GUARD = "      - name: Guard legacy feature test filter scope\n";
const CHANGED = "native-legacy changed; its digest is now";
const LEGACY_CASES = [
  ["a step-level `if` written as the first key of a step", inLegacyJob((job) => job.replace(GUARD, "      - if: false\n        name: Guard legacy feature test filter scope\n")), CHANGED],
  ["a step-level `if` with a space before the colon", inLegacyJob((job) => job.replace(`${GUARD}        shell: bash\n`, `${GUARD}        if : false\n        shell: bash\n`)), CHANGED],
  ["a test command that can no longer fail", inLegacyJob((job) => job.replace("binary(unit_a_live)'\n          cargo nextest run", "binary(unit_a_live)' || true\n          cargo nextest run")), CHANGED],
  ["a step removed", inLegacyJob((job) => job.replace(/      - name: Lint legacy voucher-scan and calibration harness features\n[\s\S]*$/, "")), CHANGED],
  ["a cache save", inLegacyJob((job) => job.replace("save-if: false", "save-if: true")), CHANGED],
  ["a changed timeout", inLegacyJob((job) => job.replace("timeout-minutes: 50", "timeout-minutes: 5")), CHANGED],
  ["a different shared key", inLegacyJob((job) => job.replace("setup-windows-native/action.yml", "setup-windows-native/other.yml")), "must restore native's Windows dependency cache"],
  ["narrower cache workspaces", inLegacyJob((job) => job.replace("            tools -> target\n", "")), "must share this cache input: workspaces: |"],
  ["a dropped hashed env var list", inLegacyJob((job) => job.replace("env-vars: ImageOS ImageVersion OPENSSL LIBCLANG", "env-vars: ImageOS")), "must share this cache input: ImageOS ImageVersion OPENSSL LIBCLANG"],
  ["a dropped CARGO_PROFILE variable", inLegacyJob((job) => job.replace("      CARGO_PROFILE_TEST_DEBUG: '0'\n", "")), "must share this cache input: CARGO_PROFILE_DEV_DEBUG: '0'"],
  ["a changed CARGO_PROFILE variable in native", (text) => text.replace("      CARGO_PROFILE_TEST_DEBUG: '0'\n    # A cold Windows native build", "      CARGO_PROFILE_TEST_DEBUG: '1'\n    # A cold Windows native build"), "must share this cache input: CARGO_PROFILE_DEV_DEBUG: '0'"],
  ["narrower cache workspaces in native", (text) => text.replace("            tools -> target\n          # Separate the Windows", "          # Separate the Windows"), "must share this cache input: workspaces: |"],
  ["a different env-var list in native", (text) => text.replace("'ImageOS ImageVersion OPENSSL LIBCLANG' || ''", "'ImageOS ImageVersion OPENSSL' || ''"), "must share this cache input: ImageOS ImageVersion OPENSSL LIBCLANG"],
];
for (const [name, change, message] of LEGACY_CASES) {
  test(`native-legacy: ${name} is refused`, () => {
    refused(withChange(".github/workflows/ci.yml", change), message);
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

test("diagnostics uploads: the flag in any other workflow is refused", () => {
  refused(withChange(".github/workflows/release-mcpb-preview.yml", (text) => text.replace(/(\n {6}- uses: actions\/checkout)/, `\n${FLAG}$1`)), "release-mcpb-preview.yml must not use continue-on-error");
});

test("a quoted mapping key at the start of a line is refused (an escaped spelling would not contain the refused text)", () => {
  refused(withChange(CI_FILE, (text) => text.replace(AFTER_NATIVE_PIN, `${AFTER_NATIVE_PIN}        "continue\\x2Don-error": true\n`)), "must not start a line with a quoted mapping key");
});
