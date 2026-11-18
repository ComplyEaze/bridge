// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import test, { after } from "node:test";
import { fileURLToPath } from "node:url";
import { LISTED, OUTSIDE_INPUTS, outsideInputs, render } from "./nightly-outside-inputs.mjs";

// Real repositories in a temporary directory: the listing is `git diff` between two commits, so the
// cases run git rather than a stand-in for it.
const scripts = dirname(fileURLToPath(import.meta.url));
const sandbox = mkdtempSync(join(tmpdir(), "outside-inputs-"));
after(() => rmSync(sandbox, { recursive: true, force: true }));
let counter = 0;

function git(cwd, ...args) {
  return execFileSync("git", ["-c", "user.name=t", "-c", "user.email=dev@example.invalid", "-c", "commit.gpgsign=false", ...args], {
    cwd,
    encoding: "utf8",
    env: { ...process.env, GIT_CEILING_DIRECTORIES: sandbox },
  }).trim();
}

/** A repository whose first commit holds `before` and whose second commit adds or changes `after`; returns both commit ids. */
function history(before, after_) {
  const dir = join(sandbox, `repo-${(counter += 1)}`);
  mkdirSync(dir);
  git(dir, "init", "-q");
  const write = (files, tag) => {
    for (const file of files) {
      mkdirSync(dirname(join(dir, file)), { recursive: true });
      writeFileSync(join(dir, file), `${file} ${tag}\n`);
    }
    git(dir, "add", "-A");
    git(dir, "commit", "-q", "-m", tag);
    return git(dir, "rev-parse", "HEAD");
  };
  const base = write(before, "first");
  const head = write(after_, "second");
  return { dir, base, head };
}

const UNRELATED = ["src-tauri/crates/bridge-tax-audit/src/lib.rs", "src/App.tsx", "src-tauri/crates/bridge-tally-protocol-extra/src/lib.rs", "src-tauri/crates/bridge-tally-primitives-extra/x.rs"];

test("no listed path changed: none changed, even when other files did", () => {
  const { dir, base, head } = history(["src-tauri/Cargo.lock", ...UNRELATED], UNRELATED);
  assert.deepEqual(outsideInputs({ base, head, cwd: dir }), { status: "none", files: [], reason: "" });
  assert.match(render(outsideInputs({ base, head, cwd: dir }), base, head), /\nNone changed\.\n$/);
});

test("one changed file of each listed kind is named, and the near-miss directories are not", () => {
  for (const file of [
    "src-tauri/Cargo.lock",
    "src-tauri/Cargo.toml",
    "rust-toolchain.toml",
    "src-tauri/crates/bridge-tally-protocol/src/lib.rs",
    "src-tauri/crates/bridge-tally-protocol/tests/fixtures/native/a.xml",
    "src-tauri/crates/bridge-tally-primitives/src/lib.rs",
  ]) {
    const { dir, base, head } = history(["src-tauri/Cargo.lock", ...UNRELATED], [file, ...UNRELATED.map((u) => u)]);
    const result = outsideInputs({ base, head, cwd: dir });
    assert.equal(result.status, "changed", file);
    assert.deepEqual(result.files, [file].sort(), file);
    const text = render(result, base, head);
    assert.match(text, /\n1 changed:\n\n- `/, file);
    assert.ok(text.includes(`- \`${file}\``), file);
    assert.ok(!text.includes("None changed"), file);
  }
  // A change only in the directories whose names merely start like a listed one selects nothing.
  const { dir, base, head } = history(UNRELATED, ["src-tauri/crates/bridge-tally-protocol-extra/src/new.rs", "src-tauri/crates/bridge-tally-primitives-extra/y.rs"]);
  assert.equal(outsideInputs({ base, head, cwd: dir }).status, "none");
});

test("a long list names the first files and counts the rest", () => {
  const many = Array.from({ length: LISTED + 5 }, (_, i) => `src-tauri/crates/bridge-tally-protocol/src/f${String(i).padStart(3, "0")}.rs`);
  const { dir, base, head } = history(["README.md"], many);
  const result = outsideInputs({ base, head, cwd: dir });
  assert.equal(result.files.length, LISTED + 5);
  const text = render(result, base, head);
  assert.match(text, new RegExp(`\\n${LISTED + 5} changed:\\n`));
  assert.equal(text.split("\n").filter((line) => line.startsWith("- `")).length, LISTED);
  assert.match(text, /\n- and 5 more\n/);
});

test("every way of not being able to compute reads as a failure, never as none changed", () => {
  const { dir, base, head } = history(["src-tauri/Cargo.lock"], ["src-tauri/Cargo.lock", "x"]);
  const unknown = "0123456789abcdef0123456789abcdef01234567";
  const cases = [
    ["no earlier green run", { base: "", head, cwd: dir }, /there is no earlier green run to compare with/],
    ["no earlier run, with the reason given", { base: "", head, cwd: dir, reason: "could not read the run list (exit 1)" }, /could not read the run list \(exit 1\)/],
    ["a base that is not a commit id", { base: "main", head, cwd: dir }, /not a full commit id/],
    ["a head that is not a commit id", { base, head: "HEAD", cwd: dir }, /this run's commit is not a full commit id/],
    ["an option-like base", { base: "--output=x", head, cwd: dir }, /not a full commit id/],
    ["a base missing from the checkout", { base: unknown, head, cwd: dir }, /not in this checkout/],
    ["a head missing from the checkout", { base, head: unknown, cwd: dir }, /git diff failed/],
    ["a directory that is not a repository", { base, head, cwd: mkdtempSync(join(sandbox, "bare-")) }, /not in this checkout/],
  ];
  for (const [name, input, reason] of cases) {
    const result = outsideInputs(input);
    assert.equal(result.status, "failed", name);
    assert.deepEqual(result.files, [], name);
    assert.match(result.reason, reason, name);
    const text = render(result, input.base, input.head);
    assert.match(text, /\nCould not be computed: /, name);
    assert.ok(!text.includes("None changed"), name);
    assert.match(text, /This is not the same as "none changed"/, name);
  }
});

test("the section shows the two commits at 8 characters and the listed paths", () => {
  const { dir, base, head } = history(["a"], ["src-tauri/Cargo.lock"]);
  const text = render(outsideInputs({ base, head, cwd: dir }), base, head);
  assert.ok(text.includes(`Compared ${base.slice(0, 8)} (the last green nightly run) with ${head.slice(0, 8)} (this run)`), text);
  assert.ok(!text.includes(base), "no full commit id in the section");
  for (const path of ["src-tauri/Cargo.lock", "src-tauri/Cargo.toml", "rust-toolchain.toml"]) assert.ok(text.includes(path), path);
  assert.deepEqual(OUTSIDE_INPUTS.length, 5);
});

test("the command prints the section and exits 0 for each outcome; a bad argument exits 2", () => {
  const { dir, base, head } = history(["a"], ["src-tauri/Cargo.lock"]);
  const run = (...args) => spawnSync(process.execPath, [join(scripts, "nightly-outside-inputs.mjs"), ...args], { cwd: dir, encoding: "utf8" });
  const changed = run("--base", base, "--head", head);
  assert.equal(changed.status, 0, changed.stderr);
  assert.match(changed.stdout, /\n1 changed:/);
  const none = run("--base", head, "--head", head);
  assert.equal(none.status, 0, none.stderr);
  assert.match(none.stdout, /\nNone changed\.\n$/);
  const noBase = run("--base", "", "--head", head, "--reason", "could not read the run list (exit 1)");
  assert.equal(noBase.status, 0, noBase.stderr);
  assert.match(noBase.stdout, /Could not be computed: could not read the run list \(exit 1\)\./);
  for (const bad of [["--bogus"], ["--base"], ["base", "x"], []]) {
    const r = run(...bad);
    assert.equal(r.status, bad.length ? 2 : 0, JSON.stringify(bad));
    if (bad.length) assert.match(r.stderr, /usage: nightly-outside-inputs\.mjs/);
  }
});
