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
    { class: "write-path", match: { exact: "guard/guarded.rs" } },
    { class: "tests", match: { suffix: "_tests.rs" } },
    { class: "tests", match: { exact: "guard/tests.rs" } },
    { class: "production", match: { prefix: "guard/" } },
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

test("a test file carries the tier of the module it tests, and an unclassified module makes it full", () => {
  assert.equal(tierOf(["src/post_extra_tests.rs"], small).tier, "full");
  assert.equal(tierOf(["src/post/tests.rs"], small).tier, "full");
  assert.equal(tierOf(["src/other_tests.rs"], small).tier, "one");
  assert.equal(tierOf(["lib/stray_tests.rs"], small).tier, "full");
  assert.deepEqual(tierOf(["lib/stray_tests.rs"], small).unclassified, ["lib/stray.rs"]);
  assert.equal(tierOf(["src/other/tests.rs"], small).tier, "one");
  // the test file itself matches a plain-tests rule; only its module's rule makes it full
  assert.equal(classify("guard/guarded_tests.rs", small).class, "tests");
  assert.equal(tierOf(["guard/guarded_tests.rs"], small).tier, "full");
  assert.equal(tierOf(["guard/other_tests.rs"], small).tier, "one");
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
    "src-tauri/src/tally/box_fit.rs",
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
    // tests of write-path features, by name, in any language and any case
    "scripts/journal-posting-state.test.mjs",
    "scripts/journal-posting-ui.test.mjs",
    "scripts/journal-posting-screen.test.tsx",
    "scripts/source-draft-shell.spec.ts",
    "scripts/bank_statement_import.test.py",
    "src-tauri/tests/post_something.rs",
    "src-tauri/tests/something_send.rs",
    "src/BankImport.tsx",
    "src-tauri/src/agent_company_checkpoint.rs",
    "src-tauri/src/reports/bulk_party_statement.rs",
    "src-tauri/src/reports/bulk_party_statement_tests.rs",
    // company identity the interface relies on, and the pages that carry the installer links
    "src/tally-company-selection.ts",
    "src/company-identity.ts",
    "site/download.html",
    "docs/agent/INSTALL.md",
    // Rust that talks to Tally is full whatever it is called: the read tools, their request builders and parsers, the company pin,
    // the runtime requests, the core models
    "src-tauri/src/agent_company.rs",
    "src-tauri/src/agent_company_identity_tests.rs",
    "src-tauri/src/agent_ledgers.rs",
    "src-tauri/src/agent_trial_balance.rs",
    "src-tauri/src/agent_trial_balance_tests.rs",
    "src-tauri/src/agent_vouchers.rs",
    "src-tauri/src/agent_month_register.rs",
    "src-tauri/src/tally/runtime_masters.rs",
    "src-tauri/crates/bridge-tally-protocol/src/native_masters.rs",
    "src-tauri/crates/bridge-tally-protocol/src/native_masters_tests.rs",
    "src-tauri/crates/bridge-tally-protocol/src/native_trial_balance/mod.rs",
    "src-tauri/crates/bridge-tally-protocol/src/xml_read_profiles.rs",
    "src-tauri/crates/bridge-tally-core/src/reconciliation.rs",
    "src-tauri/crates/bridge-tax-audit/src/read.rs",
    // the read path's own guards and the egress record, and privacy and security texts
    "src-tauri/src/tally/agent_read_request.rs",
    "src-tauri/src/tally/agent_read_request_tests.rs",
    "src-tauri/src/agent_read_profiles.rs",
    "src-tauri/src/agent_read_validation.rs",
    "src-tauri/src/agent_read_cost.rs",
    "src-tauri/src/agent_delivery.rs",
    "docs/security-and-privacy.md",
    "docs/legal/privacy.md",
    "docs/legal/terms.md",
    "docs/tally/privacy-model.md",
    "docs/adr/0099-privacy-notes.md",
    "site/security.html",
    "site/privacy.html",
    "src-tauri/tests/egress_gate.rs",
    "src-tauri/tests/redaction_check.rs",
    // tests that are #[path] children of write-path modules
    "src-tauri/src/agent_post_cancellation_tests.rs",
    "src-tauri/src/agent_post_recovery_tests.rs",
    "src-tauri/src/tally/runtime_import_admission_tests.rs",
    "src-tauri/crates/bridge-tally-protocol/src/ledger_create_tests.rs",
    "src-tauri/src/agent_lab_extra_tests.rs",
    "src-tauri/src/unknown_thing_tests.rs",
    // data files under tests/ and docs/ are not test code or prose
    "src-tauri/tests/golden_books.json",
    "src-tauri/crates/bridge-tax-audit/tests/data/client_ledger.xml",
    "docs/notes/client-tb.csv",
    "docs/notes/books.json",
    // interface files that call the write or setup commands
    "src/main.tsx",
    "src/SourceDraftScreen.tsx",
    "src/TallyReadinessFlow.tsx",
    // new files named for a write, an import, a post or an upload, and new unlisted files in a listed crate
    "src-tauri/crates/bridge-tally-core/src/post_builder.rs",
    "src-tauri/crates/bridge-tally-core/src/new_module.rs",
    "src-tauri/crates/bridge-tax-audit/src/upload.rs",
    "src-tauri/crates/bridge-tax-audit/src/import_ledger.rs",
    "src-tauri/src/reports/import_helper.rs",
    "src-tauri/src/reports/send_mail.rs",
    // names that only look like a documented file
    "packaging/mcpb/README.md/manifest.json",
    "NOTICE.sh",
    "README.md.bak",
    "LICENSE-extra",
  ];
  for (const path of full) assert.equal(tierOf([path], realRules).tier, "full", path);
});

test("the real list puts reads, interface, documents and plain tests in the one tier", () => {
  const one = [
    "src-tauri/src/reports/trial_balance.rs",
    "src-tauri/src/reports/trial_balance_tests.rs",
    "docs/tally/compatibility/acks/pr-1.txt",
    "packaging/mcpb/README.md",
    "src/LedgerEntriesScreen.tsx",
    "src-tauri/crates/bridge-tax-audit/src/lib.rs",
    "src-tauri/crates/bridge-tax-audit/tests/narration_payees.rs",
    "src/outstandings-csv.ts",
    "docs/img/diagram.svg",
    "README.md",
    "CHANGELOG.md",
    "docs/guides/connect-claude-to-tallyprime.md",
    "docs/tax-audit/README.md",
    "site/faq.html",
    "site/home.css",
    "site/llms.txt",
    "scripts/outstandings-csv.test.mjs",
    "scripts/tally-endpoint-reconnect-hint.test.mjs",
  ];
  for (const path of one) assert.equal(tierOf([path], realRules).tier, "one", path);
});

test("no tracked file whose name says import, post, ledger creation, narration, approval, egress, credential or secret is in the one tier", () => {
  const listed = spawnSync("git", ["ls-files", "-z"], { cwd: root, encoding: "utf8" });
  assert.equal(listed.status, 0, listed.stderr);
  const hazardous = /import|ledger_create|narration|approv|(^|[^a-z])egress|credential|secret|encrypt|_post|post_/i;
  // Documents about those subjects are one-reviewer by design (the code pattern leaves them out); tests of them are not.
  const code = /\.(rs|ts|tsx|mjs|py|sh|sql|toml|json)$/;
  const offenders = listed.stdout.split("\0").filter(Boolean).filter((path) => {
    if (!hazardous.test(path) || !code.test(path)) return false;
    // The tax-audit engine reads exported narrations to audit them; it writes nothing to Tally.
    if (path.startsWith("src-tauri/crates/bridge-tax-audit/")) return false;
    return tierOf([path], realRules).tier === "one";
  });
  assert.deepEqual(offenders, []);
});

test("no interface file that calls a write, setup, enrolment, destination or lifecycle command is in the one tier", () => {
  const listed = spawnSync("git", ["ls-files", "-z", "src"], { cwd: root, encoding: "utf8" });
  assert.equal(listed.status, 0, listed.stderr);
  const writes = /invoke[<(][^"]*"(save_tally_setup|enroll_|revoke_|desktop_|select_party_statement_destination|bootstrap_)/;
  const offenders = listed.stdout.split("\0").filter((path) => /\.tsx?$/.test(path)).filter((path) => {
    const text = readFileSync(join(root, path), "utf8");
    return writes.test(text) && tierOf([path], realRules).tier === "one";
  });
  assert.deepEqual(offenders, []);
});

test("no tracked file named for privacy, security, redaction or egress, in any directory, is in the one tier", () => {
  const listed = spawnSync("git", ["ls-files", "-z"], { cwd: root, encoding: "utf8" });
  assert.equal(listed.status, 0, listed.stderr);
  const offenders = listed.stdout.split("\0").filter(Boolean).filter((path) => {
    if (path.startsWith("docs/tally/compatibility/acks/")) return false;
    // The tax-audit engine's own spec packs and text carry the word in their subject matter.
    if (path.startsWith("src-tauri/crates/bridge-tax-audit/") || path.startsWith("docs/tax-audit/")) return false;
    return /privacy|security|redact|egress/i.test(path) && tierOf([path], realRules).tier === "one";
  });
  assert.deepEqual(offenders, []);
});

test("no tracked data file under tests/ or docs/ is in the one tier", () => {
  const listed = spawnSync("git", ["ls-files", "-z"], { cwd: root, encoding: "utf8" });
  assert.equal(listed.status, 0, listed.stderr);
  const offenders = listed.stdout.split("\0").filter(Boolean).filter((path) => {
    // Acknowledgement files are notes filed beside a pinned change; the pinned change is classified on its own.
    if (path.startsWith("docs/tally/compatibility/acks/")) return false;
    const underTests = path.includes("/tests/") || path.startsWith("docs/");
    return underTests && /\.(json|csv|xml|txt|xlsx|yaml|yml|toml|sql|db)$/.test(path) && tierOf([path], realRules).tier === "one";
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
