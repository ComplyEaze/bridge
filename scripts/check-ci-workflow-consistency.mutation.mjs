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
