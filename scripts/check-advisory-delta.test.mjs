#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0
//
// Contract tests for scripts/check-advisory-delta.mjs.
//
// These deliberately run against REAL repository history rather than a
// synthetic fixture: commit 8b17f95c is the actual pre-fix state that had
// RUSTSEC-2026-0285 (rustls) as a live `cargo audit` error on 2026-09-15, and
// 9558a316 is the very next commit, which patched it (see that commit's own
// message for the incident). Using real history means these tests prove the
// script would have correctly classified the actual PR that fixed this, in
// both directions -- not just some hypothetical id string.
//
// A false negative here is the dangerous failure mode: a gate that silently
// never fires provides no protection while looking exactly like one that
// does, which is the whole risk category PROBLEM 1 exists to close. So this
// file tests the firing direction, not only the passing one.
//
// `corepack pnpm test` runs every scripts/*.test.mjs in the "Frontend build"
// CI job (.github/workflows/ci.yml), which has no Rust toolchain and does not
// install cargo-audit -- see .github/workflows/dependency-security.yml for
// the job that does. These tests need `cargo audit` (any lockfile-scanning
// invocation, not one particular pinned toolchain) and skip
// themselves with a clear reason when it is unavailable, rather than failing
// a job that was never given the tool. See docs/proposed-dependency-policy.md
// for the proposed CI wiring that would let them actually run.
//
// The advisory database is a FIXED SNAPSHOT built here (--advisory-db), not the live RustSec database. The first version of
// these tests read the live database, and a test that reads it measures the world and not the code: on 23 October 2026 a
// newly published advisory (RUSTSEC-2026-0347, which applies to the rustls 0.23.45 that the "fixed" commit locks) made the
// "only removed" case fail although nothing in the script or the repository had changed. The advisory texts below are
// copies of the RustSec entries (CC0-1.0, https://github.com/rustsec/advisory-db), so the fixture is captured, not authored.
//
// Run: node scripts/check-advisory-delta.test.mjs

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, dirname } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(here, "..");
const SCRIPT = join(here, "check-advisory-delta.mjs");

// RustSec advisory text, one string each (copied from the RustSec advisory database; a copy, so it does not change).
const ADVISORY_0285 = [
  "```toml",
  "[advisory]",
  "id = \"RUSTSEC-2026-0285\"",
  "package = \"rustls\"",
  "date = \"2026-09-14\"",
  "url = \"https://github.com/rustls/rustls/pull/3265\"",
  "categories = [\"crypto-failure\"]",
  "aliases = [\"GHSA-2mjx-qc3c-rqvc\"]",
  "cvss = \"CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:L/I:N/A:N\"",
  "",
  "[versions]",
  "patched = [\">= 0.23.45\"]",
  "unaffected = [\"< 0.23.13\"]",
  "```",
  "",
  "# TLS 1.3 handshake messages incorrectly accepted across encryption level boundaries",
  "",
  "Rustls accepted TLS 1.3 handshake messages sent at the wrong encryption level",
  "when they followed a key-changing message in the same record. For example,",
  "a plaintext `EncryptedExtensions` message packed into the same record as the",
  "`ServerHello` was accepted.",
  "",
  "RFC 8446 section 5.1 requires that handshake messages do not span key changes,",
  "and that implementations terminate the connection with an \"unexpected_message\"",
  "alert if they do.",
  "",
  "The handshake transcript is still authenticated, so a network-position attacker",
  "cannot use this to alter or complete a handshake; the practical effect is that",
  "a peer could send handshake messages that should be encrypted in plaintext",
  "without rustls rejecting the connection.",
  "",
  "This is functionally the same bug as Go's",
  "[GO-2026-4340](https://pkg.go.dev/vuln/GO-2026-4340) (CVE-2025-61730).",
].join("\n") + "\n";

const ADVISORY_0347 = [
  "```toml",
  "[advisory]",
  "id = \"RUSTSEC-2026-0347\"",
  "package = \"rustls\"",
  "date = \"2026-10-14\"",
  "url = \"https://github.com/rustls/rustls/pull/3300\"",
  "categories = [\"crypto-failure\"]",
  "aliases = [\"GHSA-vp88-798v-h67f\"]",
  "cvss = \"CVSS:3.1/AV:L/AC:L/PR:H/UI:N/S:U/C:L/I:N/A:N\"",
  "",
  "[versions]",
  "patched = [\">= 0.23.46\"]",
  "unaffected = [\"< 0.23.44\"]",
  "```",
  "",
  "# Buffered application data sent to outer server after ECH rejection",
  "",
  "When a server rejects Encrypted Client Hello (ECH), the client completes the",
  "handshake using the outer ClientHello and authenticates the server against the",
  "ECH config's `public_name`. At the end of that handshake, rustls encrypted and",
  "queued any application data buffered via `ClientConnection::writer()` *before*",
  "queueing the `ech_required` alert.",
  "",
  "The application correctly receives",
  "`Error::PeerIncompatible(ServerRejectedEncryptedClientHello)`, but the next",
  "`write_tls()` call (needed to send the alert) also sends the buffered",
  "application data. Plaintext meant for the inner server therefore goes to the",
  "client-facing server named by `public_name`.",
  "",
  "That server is authenticated, so this is not exploitable by an unauthenticated",
  "party, but it violates RFC 9849 section 6.1.6, which requires the client to",
  "abort with `ech_required` \"before sending any application data to the server\".",
  "",
  "This affects only clients that:",
  "",
  "- use `EchMode::Enable` (GREASE ECH is not affected),",
  "- write application data before the handshake completes, and",
  "- connect to a server that rejects their ECH config (for example, because it is stale).",
  "",
  "The underlying ordering bug is older, but before 0.23.44 the client checked",
  "the server certificate against the inner server name rather than `public_name`,",
  "so certificate verification failed first and no data was sent.",
].join("\n") + "\n";

/** A throwaway advisory database holding exactly the given entries ({ crate, id, text }), as a git repository the
 * way cargo-audit expects one, so a scan with it cannot see any other advisory. Returns its path. */
function snapshot(entries) {
  const directory = mkdtempSync(join(tmpdir(), "advisory-snapshot-"));
  for (const { crate, id, text } of entries) {
    mkdirSync(join(directory, "crates", crate), { recursive: true });
    writeFileSync(join(directory, "crates", crate, `${id}.md`), text);
  }
  mkdirSync(join(directory, "crates"), { recursive: true });
  const git = (...args) => spawnSync("git", ["-c", "user.name=test", "-c", "user.email=test@example.invalid", ...args], { cwd: directory, encoding: "utf8" });
  git("init", "-q");
  git("add", "-A");
  git("commit", "-q", "--allow-empty", "-m", "snapshot");
  return directory;
}

const databases = [];
function database(entries) {
  const path = snapshot(entries);
  databases.push(path);
  return path;
}
test.after(() => {
  for (const path of databases) rmSync(path, { recursive: true, force: true });
});

const FIX_COMMIT = "9558a316a21bad1dce0b90ea83734e3dcc8557eb"; // fix(deps): patch rustls for RUSTSEC-2026-0285 (#381)
const PRE_FIX_COMMIT = "8b17f95cd0de6a0258f6765eda410bc10ca8fd1f"; // its parent

function cargoAuditAvailable() {
  const version = spawnSync("cargo", ["audit", "--version"], { encoding: "utf8" });
  if (version.error || version.status !== 0) {
    return { ok: false, reason: "`cargo audit` is not available (cargo-audit not installed)" };
  }
  return { ok: true, reason: null };
}

function commitsPresent() {
  for (const commit of [FIX_COMMIT, PRE_FIX_COMMIT]) {
    const result = spawnSync("git", ["cat-file", "-e", commit], { cwd: repoRoot, encoding: "utf8" });
    if (result.status !== 0) {
      return { ok: false, reason: `fixture commit ${commit} is not reachable (shallow clone?)` };
    }
  }
  return { ok: true, reason: null };
}

const audit = cargoAuditAvailable();
const commits = audit.ok ? commitsPresent() : { ok: false, reason: null };
const unavailable = !audit.ok ? audit.reason : !commits.ok ? commits.reason : null;
const skip = unavailable ? `prerequisite unavailable: ${unavailable}` : false;

// Every case below scans against a snapshot that holds only the advisory the real history is about, unless it says otherwise.
const ONLY_0285 = () => ["--advisory-db", database([{ crate: "rustls", id: "RUSTSEC-2026-0285", text: ADVISORY_0285 }])];

function run(args) {
  return spawnSync(process.execPath, [SCRIPT, ...args], {
    cwd: repoRoot,
    encoding: "utf8",
    maxBuffer: 64 * 1024 * 1024,
  });
}

test(
  "fires on the exact known-vulnerable commit pair, in the direction that introduces the advisory",
  { skip },
  () => {
    // base = the FIXED state, head = the PRE-FIX state: head's Cargo.lock
    // genuinely carries an advisory (RUSTSEC-2026-0285) that base's does not,
    // so this must fail. (Backwards from a real PR's chronology on purpose --
    // see the file header for why real history is used at all -- the
    // direction of "which side lacks the finding" is what the script checks,
    // and this is the one real pair of commits on this repository's history
    // that differs by exactly one known, named advisory.)
    const result = run(["--base", FIX_COMMIT, "--head", PRE_FIX_COMMIT, ...ONLY_0285()]);
    assert.notEqual(result.status, 0, `expected failure; stdout:\n${result.stdout}\nstderr:\n${result.stderr}`);
    assert.match(result.stderr, /RUSTSEC-2026-0285/);
    assert.match(result.stderr, /introduces 1 new advisory finding/);
  },
);

test("passes in the real chronological direction, where the advisory is only removed", { skip }, () => {
  // base = PRE-FIX, head = FIXED: this is the actual PR, and it must not be
  // reported as introducing anything -- it removes a finding, which is
  // informational (logged) but not a failure.
  const result = run(["--base", PRE_FIX_COMMIT, "--head", FIX_COMMIT, ...ONLY_0285()]);
  assert.equal(result.status, 0, `expected success; stdout:\n${result.stdout}\nstderr:\n${result.stderr}`);
  assert.match(result.stdout, /no longer present at HEAD/);
  assert.match(result.stdout, /RUSTSEC-2026-0285/);
  assert.match(result.stdout, /no new advisory findings/);
});

test("passes with no findings to report when base and head share the same lockfile", { skip }, () => {
  const result = run(["--base", FIX_COMMIT, "--head", FIX_COMMIT]);
  assert.equal(result.status, 0, `expected success; stdout:\n${result.stdout}\nstderr:\n${result.stderr}`);
  assert.match(result.stdout, /byte-identical/);
});

test("a bad --base ref fails with a clear error rather than a stack trace on `undefined`", { skip }, () => {
  const result = run(["--base", "not-a-real-ref-xyz", "--head", FIX_COMMIT]);
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /does not resolve to a commit/);
});

test("a scan reads only the snapshot it is given: an empty one reports nothing for the pre-fix commit", { skip }, () => {
  // With the live database the pre-fix lockfile carries RUSTSEC-2026-0285. An empty snapshot must not.
  const result = run(["--base", PRE_FIX_COMMIT, "--head", FIX_COMMIT, "--advisory-db", database([])]);
  assert.equal(result.status, 0, `expected success; stdout:\n${result.stdout}\nstderr:\n${result.stderr}`);
  assert.doesNotMatch(result.stdout + result.stderr, /RUSTSEC-2026-0285/);
  assert.match(result.stdout, /no new advisory findings \(base has 0, head has 0/);
});

test("an advisory that applies to the head's version and not to the base's is reported as introduced", { skip }, () => {
  // RUSTSEC-2026-0347 applies from rustls 0.23.44; the fixed commit locks 0.23.45 and the pre-fix one 0.23.43.
  // This is the case that made the live-database version of the "only removed" test fail on 23 October 2026,
  // now fixed in time: the script must report it, in the real chronological direction.
  const both = database([
    { crate: "rustls", id: "RUSTSEC-2026-0285", text: ADVISORY_0285 },
    { crate: "rustls", id: "RUSTSEC-2026-0347", text: ADVISORY_0347 },
  ]);
  const result = run(["--base", PRE_FIX_COMMIT, "--head", FIX_COMMIT, "--advisory-db", both]);
  assert.notEqual(result.status, 0, `expected failure; stdout:\n${result.stdout}\nstderr:\n${result.stderr}`);
  assert.match(result.stderr, /RUSTSEC-2026-0347/);
  assert.match(result.stderr, /introduces 1 new advisory finding/);
  assert.match(result.stdout, /RUSTSEC-2026-0285/); // and the one it removes is still logged
});

test("a missing or malformed --advisory-db is refused before any scan", { skip }, () => {
  const missing = run(["--base", PRE_FIX_COMMIT, "--head", FIX_COMMIT, "--advisory-db", join(tmpdir(), "no-such-advisory-snapshot")]);
  assert.notEqual(missing.status, 0);
  assert.match(missing.stderr, /--advisory-db .* is not a directory holding crates/);
  const empty = mkdtempSync(join(tmpdir(), "advisory-not-a-db-"));
  databases.push(empty);
  const notDb = run(["--base", PRE_FIX_COMMIT, "--head", FIX_COMMIT, "--advisory-db", empty]);
  assert.notEqual(notDb.status, 0);
  assert.match(notDb.stderr, /--advisory-db .* is not a directory holding crates/);
  const noValue = run(["--base", PRE_FIX_COMMIT, "--head", FIX_COMMIT, "--advisory-db"]);
  assert.notEqual(noValue.status, 0);
  assert.match(noValue.stderr, /--advisory-db requires a value/);
});
