#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0
// Contract tests for scripts/review-tier.mjs and its committed path list, scripts/review-tier-paths.json.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { classify, loadRules, tierOf } from "./review-tier.mjs";

const here = fileURLToPath(new URL(".", import.meta.url));
const root = join(here, "..");
const SCRIPT = join(here, "review-tier.mjs");
const realRules = loadRules(JSON.parse(readFileSync(join(here, "review-tier-paths.json"), "utf8")));

const small = loadRules({
  schema_version: 1,
  rules: [
    { class: "write-path", match: { prefix: "src/post" } },
    { class: "tests", match: { suffix: "_tests.rs" } },
    { class: "production", match: { prefix: "src/" } },
    { class: "docs", match: { prefix: "docs/" } },
  ],
});

test("a path no rule classifies makes the whole change full, and is named", () => {
  const result = tierOf(["docs/a.md", "mystery/file.bin"], small);
  assert.equal(result.tier, "full");
  assert.deepEqual(result.unclassified, ["mystery/file.bin"]);
});

test("the first matching rule decides, so a write-path test file is full and another test file is one", () => {
  assert.equal(classify("src/post_tests.rs", small).class, "write-path");
  assert.equal(classify("src/other_tests.rs", small).class, "tests");
  assert.equal(tierOf(["src/other_tests.rs"], small).tier, "one");
  assert.equal(tierOf(["src/post_tests.rs"], small).tier, "full");
});

test("one write-path file makes a change full; docs and tests alone stay one", () => {
  assert.equal(tierOf(["docs/a.md", "src/x_tests.rs", "src/y.rs"], small).tier, "one");
  assert.equal(tierOf(["docs/a.md", "src/post/b.rs"], small).tier, "full");
});

test("a change with no files is full, not one", () => {
  assert.equal(tierOf([], small).tier, "full");
});

test("a rename counts both names, so moving a write-path file out of its directory is still full", () => {
  assert.equal(tierOf(["docs/moved.md", "src/post/old.rs"], small).tier, "full");
});

test("an invalid list is refused rather than read as fewer rules", () => {
  const bad = [
    null,
    { schema_version: 2, rules: [] },
    { schema_version: 1, rules: [] },
    { schema_version: 1, rules: [{ class: "other", match: { prefix: "a" } }] },
    { schema_version: 1, rules: [{ class: "docs", match: {} }] },
    { schema_version: 1, rules: [{ class: "docs", match: { prefix: "" } }] },
    { schema_version: 1, rules: [{ class: "docs", match: { regex: "a" } }] },
    { schema_version: 1, rules: [{ class: "docs", match: { prefix: "a" }, extra: 1 }] },
    { schema_version: 1, rules: [{ class: "docs", match: { prefix: "a" } }, { class: "docs", match: { prefix: "a" } }] },
    { schema_version: 1, rules: [{ class: "docs", match: { prefix: "a" } }], extra: 1 },
  ];
  for (const document of bad) assert.throws(() => loadRules(document), /review-tier/, JSON.stringify(document));
});

test("the real list puts the write path, credentials and the gate in the full tier", () => {
  const full = [
    "src-tauri/src/agent_import_post.rs",
    "src-tauri/src/agent_import_post_tests.rs",
    "src-tauri/src/tally/approved_import.rs",
    "src-tauri/crates/bridge-tally-protocol/src/ledger_create.rs",
    "src-tauri/crates/bridge-tally-protocol/src/import_outcome.rs",
    "src-tauri/crates/bridge-tally-protocol/src/xml_text.rs",
    "src-tauri/crates/bridge-bank-statement/src/mapping.rs",
    "src-tauri/crates/bridge-tally-transport/src/lib.rs",
    "src-tauri/src/agent_egress.rs",
    "src-tauri/src/agent_bank_statement.rs",
    "src-tauri/src/agent_transfer_pairing.rs",
    "src-tauri/src/agent_terms.rs",
    "src-tauri/src/db/encrypted.rs",
    "src-tauri/src/db/migrations/0028_new.sql",
    "src/JournalPostingScreen.tsx",
    "scripts/merge-gate.sh",
    "scripts/merge_gate_privacy.py",
    "scripts/merge-gate-synthetic-uuids.json",
    "scripts/review-tier.mjs",
    "scripts/review-tier-paths.json",
    "scripts/check-surface-ack.mjs",
    ".github/workflows/ci.yml",
    "docs/tally/compatibility/compatibility-surface.json",
    "docs/adr/0004-tally-write-safety.md",
    "docs/tally/TALLY_PROTOCOL_REFERENCE_VOUCHER_WRITES.md",
    "packaging/mcpb/manifest.json",
    "src-tauri/Cargo.lock",
    "src-tauri/crates/bridge-tally-core/Cargo.toml",
    "package.json",
    "pnpm-lock.yaml",
    "SECURITY.md",
    "AGENTS.md",
    "site/fonts/some.woff2",
    "src-tauri/crates/bridge-tally-protocol/tests/parse_record/x.txt",
    "scripts/testdata/capture.xml",
    "src-tauri/src/never_seen_before.rs",
  ];
  for (const path of full) assert.equal(tierOf([path], realRules).tier, "full", path);
});

test("the real list puts reads, interface, documents and plain tests in the one tier", () => {
  const one = [
    "src-tauri/src/reports/trial_balance.rs",
    "src-tauri/src/reports/trial_balance_tests.rs",
    "src-tauri/src/agent_trial_balance.rs",
    "src-tauri/src/agent_month_register.rs",
    "docs/tally/compatibility/acks/pr-1.txt",
    "packaging/mcpb/README.md",
    "src-tauri/src/agent_vouchers.rs",
    "src-tauri/src/tally/runtime_masters.rs",
    "src-tauri/crates/bridge-tally-protocol/src/native_trial_balance/mod.rs",
    "src-tauri/crates/bridge-tally-protocol/src/native_masters_tests.rs",
    "src/OutstandingsScreen.tsx",
    "src/outstandings-csv.ts",
    "README.md",
    "CHANGELOG.md",
    "docs/guides/connect-claude-to-tallyprime.md",
    "docs/tax-audit/README.md",
    "site/faq.html",
    "site/home.css",
    "site/llms.txt",
    "scripts/outstandings-csv.test.mjs",
    "scripts/bank_statement_import.test.py",
  ];
  for (const path of one) assert.equal(tierOf([path], realRules).tier, "one", path);
});

test("no tracked file whose name says import, post, ledger creation, narration, approval, egress, credential or secret is in the one tier", () => {
  const listed = spawnSync("git", ["ls-files", "-z"], { cwd: root, encoding: "utf8" });
  assert.equal(listed.status, 0, listed.stderr);
  const hazardous = /import|ledger_create|narration|approv|(^|[^a-z])egress|credential|secret|encrypt|_post|post_/i;
  // Documents and tests about those subjects are one-reviewer by design; this guards the code.
  const code = /\.(rs|ts|tsx|mjs|py|sh|sql|toml|json)$/;
  const offenders = listed.stdout.split("\0").filter(Boolean).filter((path) => {
    if (!hazardous.test(path) || !code.test(path)) return false;
    if (/(_tests?\.rs|\.test\.(mjs|py|tsx?)|\.spec\.ts)$/.test(path)) return false;
    // The tax-audit engine reads exported narrations to audit them; it writes nothing to Tally.
    if (path.startsWith("src-tauri/crates/bridge-tax-audit/")) return false;
    return tierOf([path], realRules).tier === "one";
  });
  assert.deepEqual(offenders, []);
});

test("the command line reads a file of paths and prints the tier as JSON", () => {
  const directory = mkdtempSync(join(tmpdir(), "review-tier-"));
  try {
    writeFileSync(join(directory, "paths.txt"), "README.md\nsite/faq.html\n");
    const one = spawnSync(process.execPath, [SCRIPT, "--paths-file", join(directory, "paths.txt")], { encoding: "utf8" });
    assert.equal(one.status, 0, one.stderr);
    assert.equal(JSON.parse(one.stdout).tier, "one");
    writeFileSync(join(directory, "paths.txt"), "README.md\nsrc-tauri/src/agent_import_post.rs\n");
    const full = spawnSync(process.execPath, [SCRIPT, "--paths-file", join(directory, "paths.txt")], { encoding: "utf8" });
    assert.equal(full.status, 0, full.stderr);
    assert.equal(JSON.parse(full.stdout).tier, "full");
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("an input that cannot be read is exit 2 with the tier full, never one", () => {
  const missing = spawnSync(process.execPath, [SCRIPT, "--paths-file", join(tmpdir(), "review-tier-missing-file")], { encoding: "utf8" });
  assert.equal(missing.status, 2);
  assert.equal(JSON.parse(missing.stdout).tier, "full");
  const noArgument = spawnSync(process.execPath, [SCRIPT], { encoding: "utf8" });
  assert.equal(noArgument.status, 2);
  assert.equal(JSON.parse(noArgument.stdout).tier, "full");
  const unknown = spawnSync(process.execPath, [SCRIPT, "--bogus"], { encoding: "utf8" });
  assert.equal(unknown.status, 2);
});
