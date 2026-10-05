#!/usr/bin/env node
// Contract tests for scripts/check-fixture-provenance.mjs.
//
// Every case builds a small synthetic tree with the four real fixture-root
// paths (via --root) rather than touching the repository's own fixtures:
// this gate's directory list is a fixed mirror of
// check-fixture-byte-integrity.mjs's, not something it discovers, so there is
// no way to exercise "a fresh directory with no coverage at all" the way that
// gate's own test does — every case here instead varies what is inside one of
// the four already-covered directories.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";
import test from "node:test";
import { tmpdir } from "node:os";
import { fileURLToPath } from "node:url";

const here = fileURLToPath(new URL(".", import.meta.url));
const GATE = join(here, "check-fixture-provenance.mjs");

const FIXTURE_DIRS = [
  "src-tauri/crates/bridge-bank-statement/tests/fixtures",
  "src-tauri/crates/bridge-tax-audit/tests/fixtures",
  "src-tauri/crates/bridge-tally-protocol/tests/fixtures",
  "src-tauri/crates/tally-protocol-simulator/fixtures",
  "docs/tally/compatibility/fixtures",
  "scripts/fixtures",
  "tools/bridge-tally-compatibility/tests/fixtures",
];

async function makeTree() {
  const root = await mkdtemp(join(tmpdir(), ".fixture-provenance-"));
  for (const dir of FIXTURE_DIRS) await mkdir(join(root, dir), { recursive: true });
  return root;
}

function runGate(root) {
  return execFileSync("node", [GATE, "--root", root], { encoding: "utf8", stdio: "pipe" });
}

function runGateExpectingFailure(root) {
  try {
    runGate(root);
  } catch (error) {
    return `${error.stdout ?? ""}${error.stderr ?? ""}`;
  }
  throw new Error("expected the gate to fail, but it passed");
}

test("an empty covered tree passes (nothing to document)", async () => {
  const root = await makeTree();
  try {
    assert.doesNotThrow(() => runGate(root));
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("a fixture named in no provenance record fails the gate", async () => {
  const root = await makeTree();
  try {
    await writeFile(join(root, "scripts/fixtures/mystery-capture.xml"), "<x/>\n");
    const output = runGateExpectingFailure(root);
    assert.match(output, /mystery-capture\.xml/);
    assert.match(output, /no hash row under/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

// #838: prose pins nothing, so a fixture named only in prose is undocumented.
// Where its capture is not established it still gets a row: the digest of its
// committed bytes, labelled an integrity digest.
test("a fixture named only in prose (no table row) fails until it has an integrity-digest row", async () => {
  const root = await makeTree();
  try {
    const bytes = "<x/>\n";
    const prose =
      "`prose-only.xml` was normalised by Git on first commit; byte fidelity is not established.\n";
    await writeFile(join(root, "scripts/fixtures/prose-only.xml"), bytes);
    await writeFile(join(root, "scripts/fixtures/README.md"), prose);
    assert.match(runGateExpectingFailure(root), /prose-only\.xml: no hash row under scripts\/fixtures/);
    const sha256 = createHash("sha256").update(bytes).digest("hex");
    await writeFile(
      join(root, "scripts/fixtures/README.md"),
      `${prose}\n| \`prose-only.xml\` | ${bytes.length} | \`${sha256}\` | capture not established |\n`,
    );
    assert.match(runGate(root), /\(1 captured-fixture hash\(es\) verified, 0 undocumented\)/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("a captured fixture whose table entry matches its real bytes passes", async () => {
  const root = await makeTree();
  try {
    const bytes = "captured, byte-exact\n";
    const dir = join(root, "src-tauri/crates/bridge-tally-protocol/tests/fixtures/encoding");
    await mkdir(dir, { recursive: true });
    await writeFile(join(dir, "sample.bin"), bytes);
    const sha256 = createHash("sha256").update(bytes).digest("hex");
    await writeFile(
      join(dir, "PROVENANCE.md"),
      `| \`sample.bin\` | ${bytes.length} | \`${sha256}\` |\n`,
    );
    assert.doesNotThrow(() => runGate(root));
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

// The headline case: this is the "hand-authored fixture standing in for a
// captured one" finding class made mechanical. A table declares specific
// bytes as captured evidence; the file on disk is something else entirely.
test("a captured fixture whose bytes no longer match its declared hash fails the gate", async () => {
  const root = await makeTree();
  try {
    const declaredBytes = "the real captured response\n";
    const actualBytes = "a hand-typed stand-in someone swapped in\n";
    const dir = join(root, "src-tauri/crates/bridge-tally-protocol/tests/fixtures/encoding");
    await mkdir(dir, { recursive: true });
    await writeFile(join(dir, "swapped.bin"), actualBytes);
    const declaredSha256 = createHash("sha256").update(declaredBytes).digest("hex");
    await writeFile(
      join(dir, "PROVENANCE.md"),
      `| \`swapped.bin\` | ${declaredBytes.length} | \`${declaredSha256}\` |\n`,
    );
    const output = runGateExpectingFailure(root);
    assert.match(output, /swapped\.bin/);
    assert.match(output, /hand-authored-substitute pattern/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

// #759: a row naming its fixture with a directory, relative to the Markdown
// file, was matched against no file, so a swapped fixture passed while it
// still counted as documented.
test("a row naming its fixture with a directory is hash-checked against that file", async () => {
  const root = await makeTree();
  try {
    const fixtures = join(root, "src-tauri/crates/bridge-tally-protocol/tests/fixtures");
    await mkdir(join(fixtures, "agent"), { recursive: true });
    const declaredBytes = "the real captured response\n";
    const declaredSha256 = createHash("sha256").update(declaredBytes).digest("hex");
    const row = `| \`agent/captured.bin\` | ${declaredBytes.length} | \`${declaredSha256}\` |\n`;
    await writeFile(join(fixtures, "CAPTURE_PROVENANCE.md"), row);
    // The control: the declared bytes pass, and are counted as checked.
    await writeFile(join(fixtures, "agent", "captured.bin"), declaredBytes);
    assert.match(runGate(root), /\(1 captured-fixture hash\(es\) verified/);
    // A swapped file under the same path fails.
    await writeFile(join(fixtures, "agent", "captured.bin"), "a hand-typed stand-in\n");
    const output = runGateExpectingFailure(root);
    assert.match(output, /agent\/captured\.bin/);
    assert.match(output, /hand-authored-substitute pattern/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("a row naming a directory path where no fixture is fails rather than checking nothing", async () => {
  const root = await makeTree();
  try {
    const fixtures = join(root, "src-tauri/crates/bridge-tally-protocol/tests/fixtures");
    await mkdir(join(fixtures, "agent"), { recursive: true });
    const bytes = "captured\n";
    const sha256 = createHash("sha256").update(bytes).digest("hex");
    await writeFile(join(fixtures, "agent", "captured.bin"), bytes);
    await writeFile(
      join(fixtures, "CAPTURE_PROVENANCE.md"),
      `captured.bin\n| \`agnt/captured.bin\` | ${bytes.length} | \`${sha256}\` |\n`,
    );
    const output = runGateExpectingFailure(root);
    assert.match(
      output,
      /agnt\/captured\.bin: a hash row in .* names no file in .*; the row checks nothing/,
    );
    // The control: the same row with its path spelt right passes.
    await writeFile(
      join(fixtures, "CAPTURE_PROVENANCE.md"),
      `captured.bin\n| \`agent/captured.bin\` | ${bytes.length} | \`${sha256}\` |\n`,
    );
    assert.doesNotThrow(() => runGate(root));
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

// A path documents the file at that path, never a sibling that shares its
// basename: `other/captured.bin` is not documented by a row for
// `agent/captured.bin`, and so cannot pass as prose-only.
test("a path row does not document another fixture with the same basename", async () => {
  const root = await makeTree();
  try {
    const fixtures = join(root, "src-tauri/crates/bridge-tally-protocol/tests/fixtures");
    await mkdir(join(fixtures, "agent"), { recursive: true });
    await mkdir(join(fixtures, "other"), { recursive: true });
    const bytes = "captured\n";
    const sha256 = createHash("sha256").update(bytes).digest("hex");
    await writeFile(join(fixtures, "agent", "captured.bin"), bytes);
    await writeFile(
      join(fixtures, "CAPTURE_PROVENANCE.md"),
      `| \`agent/captured.bin\` | ${bytes.length} | \`${sha256}\` |\n`,
    );
    // The control: alone, the documented fixture passes.
    assert.doesNotThrow(() => runGate(root));
    await writeFile(join(fixtures, "other", "captured.bin"), "hand-made\n");
    const output = runGateExpectingFailure(root);
    assert.match(output, /other\/captured\.bin: no hash row under/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

// A path in prose, as REOPEN_CAPTURE_PROVENANCE.md once relied on, documents
// nothing (#838); the path row beside it does.
test("a path named in prose does not document the fixture; a path row does", async () => {
  const root = await makeTree();
  try {
    const fixtures = join(root, "src-tauri/crates/bridge-tally-protocol/tests/fixtures");
    const bytes = "print()\n";
    const prose = "Generated by [`generators/build.py`](./generators/build.py).\n";
    await mkdir(join(fixtures, "generators"), { recursive: true });
    await writeFile(join(fixtures, "generators", "build.py"), bytes);
    await writeFile(join(fixtures, "NOTE.md"), prose);
    const output = runGateExpectingFailure(root);
    assert.match(output, /generators\/build\.py: no hash row under/);
    const sha256 = createHash("sha256").update(bytes).digest("hex");
    await writeFile(
      join(fixtures, "NOTE.md"),
      `${prose}\n| \`generators/build.py\` | ${bytes.length} | \`${sha256}\` |\n`,
    );
    assert.match(runGate(root), /\(1 captured-fixture hash\(es\) verified, 0 undocumented\)/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("a path row resolving outside its fixture root fails with its own message", async () => {
  const root = await makeTree();
  try {
    const fixtures = join(root, "src-tauri/crates/bridge-tally-protocol/tests/fixtures");
    const bytes = "captured\n";
    const sha256 = createHash("sha256").update(bytes).digest("hex");
    await writeFile(join(root, "scripts/fixtures", "shared.bin"), bytes);
    await writeFile(
      join(root, "scripts/fixtures", "README.md"),
      `| \`shared.bin\` | ${bytes.length} | \`${sha256}\` |\n`,
    );
    await writeFile(
      join(fixtures, "CAPTURE_PROVENANCE.md"),
      `| \`../../../../../scripts/fixtures/shared.bin\` | ${bytes.length} | \`${sha256}\` |\n`,
    );
    const output = runGateExpectingFailure(root);
    assert.match(output, /scripts\/fixtures\/shared\.bin: a hash row in .* resolves outside /);
    // The control: the same row inside its own fixture root passes.
    await writeFile(join(fixtures, "shared.bin"), bytes);
    await writeFile(
      join(fixtures, "CAPTURE_PROVENANCE.md"),
      `| \`./shared.bin\` | ${bytes.length} | \`${sha256}\` |\n`,
    );
    assert.match(runGate(root), /\(2 captured-fixture hash\(es\) verified/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("a path row naming a provenance record fails with its own message", async () => {
  const root = await makeTree();
  try {
    const fixtures = join(root, "src-tauri/crates/bridge-tally-protocol/tests/fixtures");
    await mkdir(join(fixtures, "agent"), { recursive: true });
    const text = "notes\n";
    const sha256 = createHash("sha256").update(text).digest("hex");
    await writeFile(join(fixtures, "agent", "NOTES.md"), text);
    await writeFile(
      join(fixtures, "CAPTURE_PROVENANCE.md"),
      `| \`agent/NOTES.md\` | ${text.length} | \`${sha256}\` |\n`,
    );
    const output = runGateExpectingFailure(root);
    assert.match(output, /agent\/NOTES\.md: a hash row in .* names a provenance record/);
    // The control: the same row naming a fixture with those bytes passes.
    await writeFile(join(fixtures, "agent", "notes.bin"), text);
    await writeFile(
      join(fixtures, "CAPTURE_PROVENANCE.md"),
      `| \`agent/notes.bin\` | ${text.length} | \`${sha256}\` |\n`,
    );
    assert.match(runGate(root), /\(1 captured-fixture hash\(es\) verified/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("a byte-count-only mismatch (same filename, wrong declared size) still fails", async () => {
  const root = await makeTree();
  try {
    const actualBytes = "0123456789\n";
    const dir = join(root, "docs/tally/compatibility/fixtures");
    await mkdir(dir, { recursive: true });
    await writeFile(join(dir, "size-mismatch.json"), actualBytes);
    const realSha256 = createHash("sha256").update(actualBytes).digest("hex");
    await writeFile(
      join(dir, "PROVENANCE.md"),
      // Right hash, wrong declared byte count.
      `| \`size-mismatch.json\` | 999999 | \`${realSha256}\` |\n`,
    );
    const output = runGateExpectingFailure(root);
    assert.match(output, /size-mismatch\.json/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

// The real repository is not asserted to pass here — it does not yet (see
// docs/proposed-ci-gates.md: 82 of 125 fixtures across the four covered
// directories currently have no provenance mention). That is exactly why
// this gate is proposed as REPORTING rather than BLOCKING; this suite
// verifies behaviour on synthetic trees, not repository readiness.
console.log("\nrunning check-fixture-provenance contract tests via node:test above");

// --- provenance that is not Markdown ------------------------------------
//
// The agent fixtures record provenance as a JSON sidecar per capture. Reading
// only Markdown text reported 82 undocumented fixtures where 51 are, and left
// 13 declared hashes unchecked -- a gate understating its own strength and
// overstating its own backlog. A sidecar documents its fixtures by the hash it
// declares (#838).

const SIDECAR_DIR = join(
  "src-tauri/crates/bridge-tally-protocol/tests/fixtures",
  "agent",
);

test("a JSON sidecar carrying `source` but no hash documents nothing", async () => {
  const root = await makeTree();
  await mkdir(join(root, SIDECAR_DIR), { recursive: true });
  const bytes = "captured bytes";
  await writeFile(join(root, SIDECAR_DIR, "cap.utf16le.xml"), bytes);
  const source = "Live licensed TallyPrime synthetic-company read";
  await writeFile(join(root, SIDECAR_DIR, "cap.json"), JSON.stringify({ source }));
  assert.match(runGateExpectingFailure(root), /cap\.utf16le\.xml: no hash row under/);
  await writeFile(
    join(root, SIDECAR_DIR, "cap.json"),
    JSON.stringify({ source, fixture_sha256: createHash("sha256").update(bytes).digest("hex") }),
  );
  assert.match(runGate(root), /\(1 captured-fixture hash\(es\) verified, 0 undocumented\)/);
});

// A record that shares its stem with no other file describes only itself, as
// the outstandings request sequences do. A file cannot hold its own hash, so
// it is a fixture that needs a row in a note (#838).
test("a JSON record documenting no other file is a fixture that needs its own row", async () => {
  const root = await makeTree();
  try {
    const bytes = JSON.stringify({ source: "the ordered record of one call's requests" });
    await mkdir(join(root, SIDECAR_DIR), { recursive: true });
    await writeFile(join(root, SIDECAR_DIR, "sequence.json"), bytes);
    assert.match(runGateExpectingFailure(root), /agent\/sequence\.json: no hash row under/);
    const sha256 = createHash("sha256").update(bytes).digest("hex");
    await writeFile(
      join(root, SIDECAR_DIR, "..", "SEQUENCE_PROVENANCE.md"),
      `| \`agent/sequence.json\` | ${bytes.length} | \`${sha256}\` |\n`,
    );
    assert.match(runGate(root), /\(1 captured-fixture hash\(es\) verified, 0 undocumented\)/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("a sidecar declaring a fixture hash escalates to the same check a table row does", async () => {
  const root = await makeTree();
  await mkdir(join(root, SIDECAR_DIR), { recursive: true });
  const bytes = "captured bytes";
  await writeFile(join(root, SIDECAR_DIR, "cap.utf16le.xml"), bytes);
  await writeFile(
    join(root, SIDECAR_DIR, "cap.json"),
    JSON.stringify({
      source: "Live licensed TallyPrime synthetic-company read",
      fixture_bytes: Buffer.byteLength(bytes),
      fixture_sha256: createHash("sha256").update(bytes).digest("hex"),
    }),
  );
  assert.match(runGate(root), /1 captured-fixture hash\(es\) verified|hash\(es\) verified/);

  // And the whole point: a swapped fixture under a stale sidecar must fail.
  await writeFile(join(root, SIDECAR_DIR, "cap.utf16le.xml"), "hand-authored substitute");
  const failure = runGateExpectingFailure(root);
  assert.match(failure, /cap\.utf16le\.xml/);
  assert.match(failure, /hand-authored-substitute pattern/);
});

test("a JSON file without a `source` string is a fixture, not a record", async () => {
  const root = await makeTree();
  await mkdir(join(root, SIDECAR_DIR), { recursive: true });
  // An ordinary expected-output fixture. It must still need its own paper
  // trail -- otherwise any .json in the tree could excuse itself.
  await writeFile(join(root, SIDECAR_DIR, "expected.json"), JSON.stringify({ rows: [] }));
  const failure = runGateExpectingFailure(root);
  assert.match(failure, /expected\.json/);
});

test("a sidecar cannot document a fixture that merely shares a prefix", async () => {
  const root = await makeTree();
  await mkdir(join(root, SIDECAR_DIR), { recursive: true });
  await writeFile(
    join(root, SIDECAR_DIR, "cap.json"),
    JSON.stringify({
      source: "a live read",
      fixture_sha256: createHash("sha256").update("documented").digest("hex"),
    }),
  );
  await writeFile(join(root, SIDECAR_DIR, "cap.utf16le.xml"), "documented");
  // `cap-extra` shares the prefix `cap` but not the stem `cap.`; documenting
  // it would be the gate excusing a neighbouring fixture by accident.
  await writeFile(join(root, SIDECAR_DIR, "cap-extra.utf16le.xml"), "undocumented");
  const failure = runGateExpectingFailure(root);
  assert.match(failure, /cap-extra\.utf16le\.xml/);
  assert.doesNotMatch(failure, /(?<!-extra)(?<!-)\bcap\.utf16le\.xml/);
});

// The filename pairing no longer documents `<stem>.*` (#838): the note needs
// its fixture's row, as native-company-book-extents-with-number.PROVENANCE.md
// now carries.
test("`<stem>.PROVENANCE.md` documents `<stem>.*` only through its row", async () => {
  const root = await makeTree();
  await mkdir(join(root, SIDECAR_DIR), { recursive: true });
  const bytes = "captured bytes";
  const note = "# A field observation\n\nExact decoded XML captured on 2026-09-09.\n";
  await writeFile(join(root, SIDECAR_DIR, "cap.utf8.xml"), bytes);
  await writeFile(join(root, SIDECAR_DIR, "cap.PROVENANCE.md"), note);
  assert.match(runGateExpectingFailure(root), /cap\.utf8\.xml: no hash row under/);
  const sha256 = createHash("sha256").update(bytes).digest("hex");
  await writeFile(
    join(root, SIDECAR_DIR, "cap.PROVENANCE.md"),
    `${note}\n| \`cap.utf8.xml\` | ${bytes.length} | \`${sha256}\` |\n`,
  );
  assert.match(runGate(root), /\(1 captured-fixture hash\(es\) verified, 0 undocumented\)/);
});

test("a bare PROVENANCE.md still documents only what it names", async () => {
  const root = await makeTree();
  await mkdir(join(root, SIDECAR_DIR), { recursive: true });
  await writeFile(join(root, SIDECAR_DIR, "cap.utf8.xml"), "captured bytes");
  // No stem, so no filename pairing: the directory-level file has to say which
  // fixture it means, exactly as before this change.
  await writeFile(join(root, SIDECAR_DIR, "PROVENANCE.md"), "# Notes\n\nNothing named here.\n");
  const failure = runGateExpectingFailure(root);
  assert.match(failure, /cap\.utf8\.xml/);
});

test("the failure names the real backlog, not just the sample it prints", async () => {
  const root = await makeTree();
  await mkdir(join(root, SIDECAR_DIR), { recursive: true });
  for (let index = 0; index < 30; index += 1) {
    await writeFile(join(root, SIDECAR_DIR, `undocumented-${index}.xml`), "bytes");
  }
  const failure = runGateExpectingFailure(root);
  // Capped output, uncapped count: quoting the sample as the total is how a
  // headline number ends up wrong.
  assert.match(failure, /shown of 30 undocumented fixture\(s\)/);
});

// The real sidecars are not schema-consistent, and reading only the canonical
// key name exempted two of them from swap detection while their own records
// held the correct hash. These cases use the key names actually present in
// the repository rather than only the one the convention prefers.

test("a sidecar declaring its hash as `sha256` is checked, not treated as prose", async () => {
  const root = await makeTree();
  await mkdir(join(root, SIDECAR_DIR), { recursive: true });
  const bytes = "captured bytes";
  await writeFile(join(root, SIDECAR_DIR, "cap.utf16le.xml"), bytes);
  await writeFile(
    join(root, SIDECAR_DIR, "cap.json"),
    // `sha256`, not `fixture_sha256` -- the spelling in
    // native-three-vouchers.json and native-empty-collection.json.
    JSON.stringify({
      source: "Live licensed TallyPrime synthetic-company read",
      sha256: createHash("sha256").update(bytes).digest("hex"),
    }),
  );
  assert.match(runGate(root), /hash\(es\) verified/);

  await writeFile(join(root, SIDECAR_DIR, "cap.utf16le.xml"), "hand-authored substitute");
  const failure = runGateExpectingFailure(root);
  assert.match(failure, /cap\.utf16le\.xml/);
  assert.match(failure, /hand-authored-substitute pattern/);
});

test("a sidecar with no byte count is checked on its hash and says so", async () => {
  const root = await makeTree();
  await mkdir(join(root, SIDECAR_DIR), { recursive: true });
  const bytes = "captured bytes";
  await writeFile(join(root, SIDECAR_DIR, "cap.utf16le.xml"), bytes);
  await writeFile(
    join(root, SIDECAR_DIR, "cap.json"),
    // `fixture_sha256` without `fixture_bytes` -- native-namespaced-journal.json.
    JSON.stringify({
      source: "a live read",
      fixture_sha256: createHash("sha256").update(bytes).digest("hex"),
    }),
  );
  assert.match(runGate(root), /hash\(es\) verified/);

  // The hash must still be able to fail. Inferring the byte count from the
  // file would compare it against itself and never fail on size.
  await writeFile(join(root, SIDECAR_DIR, "cap.utf16le.xml"), "swapped");
  const failure = runGateExpectingFailure(root);
  assert.match(failure, /\(no byte count\)/);
  assert.match(failure, /cap\.utf16le\.xml/);
});

// #838: a row or sidecar documents exactly the file at the path it resolves
// to. A bare name means the file beside the record, never a file of that name
// elsewhere under the root; `bridge-tax-audit` keeps today's basename matching
// behind a named exemption until its rows are converted.
const PROTOCOL = "src-tauri/crates/bridge-tally-protocol/tests/fixtures";
const TAX_AUDIT = "src-tauri/crates/bridge-tax-audit/tests/fixtures";

async function rowAboveItsFixture(root, fixtureRoot) {
  const bytes = "captured bytes\n";
  await mkdir(join(root, fixtureRoot, "agent"), { recursive: true });
  await writeFile(join(root, fixtureRoot, "agent", "nested.bin"), bytes);
  const sha256 = createHash("sha256").update(bytes).digest("hex");
  await writeFile(
    join(root, fixtureRoot, "PROVENANCE.md"),
    `| \`nested.bin\` | ${bytes.length} | \`${sha256}\` |\n`,
  );
}

test("a bare-name row documents only the file beside its Markdown file", async () => {
  const root = await makeTree();
  try {
    await rowAboveItsFixture(root, PROTOCOL);
    const failure = runGateExpectingFailure(root);
    assert.match(failure, new RegExp(`${PROTOCOL}/nested\\.bin: a hash row`));
    assert.match(failure, /names no file in/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("the exempt root keeps basename matching for its rows", async () => {
  const root = await makeTree();
  try {
    await rowAboveItsFixture(root, TAX_AUDIT);
    assert.match(runGate(root), /1 captured-fixture hash\(es\) verified/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("a wider table whose cells hold hashes is not read as a fixture row", async () => {
  const root = await makeTree();
  try {
    const hash = "a".repeat(64);
    await writeFile(
      join(root, PROTOCOL, "READS.md"),
      `| Fixture | Request SHA-256 | Bytes | Response SHA-256 |\n| --- | --- | ---: | --- |\n| \`read.xml\` | \`${hash}\` | 1,170 | \`${"b".repeat(64)}\` |\n`,
    );
    assert.match(runGate(root), /Fixture provenance is intact/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("a sidecar's hash checks the fixture beside it, not one of the same name elsewhere", async () => {
  const root = await makeTree();
  try {
    const bytes = "captured bytes";
    await mkdir(join(root, SIDECAR_DIR), { recursive: true });
    await writeFile(join(root, SIDECAR_DIR, "cap.utf16le.xml"), bytes);
    await writeFile(
      join(root, SIDECAR_DIR, "cap.json"),
      JSON.stringify({
        source: "Live licensed TallyPrime synthetic-company read",
        fixture_sha256: createHash("sha256").update(bytes).digest("hex"),
      }),
    );
    // Another fixture of the same name, with other bytes and its own row.
    const other = "other bytes";
    await mkdir(join(root, PROTOCOL, "other"), { recursive: true });
    await writeFile(join(root, PROTOCOL, "other", "cap.utf16le.xml"), other);
    await writeFile(
      join(root, PROTOCOL, "other", "PROVENANCE.md"),
      `| \`cap.utf16le.xml\` | ${other.length} | \`${createHash("sha256").update(other).digest("hex")}\` |\n`,
    );
    assert.match(runGate(root), /2 captured-fixture hash\(es\) verified/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});
