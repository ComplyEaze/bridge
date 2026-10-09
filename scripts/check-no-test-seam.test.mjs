import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { deflateRawSync, gzipSync } from "node:zlib";
import {
  PDFIUM_OVERRIDE_MARKER,
  POSITIVE_CONTROLS,
  SEAM_MARKER,
  STANDIN_MARKER,
  TEST_ONLY_MARKERS,
  assertNoTestSeam,
  holdsMarker,
  markedFiles,
  tauriBuildExecutables,
} from "./check-no-test-seam.mjs";

function scratch() {
  return mkdtempSync(join(tmpdir(), "bridge-seam-scan-"));
}

/** A fake executable holding every marker (`true`), none (`false`) or the ones listed. */
function binary(path, withMarker) {
  const markers = withMarker === true ? TEST_ONLY_MARKERS : withMarker || [];
  const bytes = Buffer.concat([
    Buffer.from([0x7f, 0x45, 0x4c, 0x46, 0x00, 0xff]),
    Buffer.from(markers.length > 0 ? markers.map((marker) => `..${marker}..`).join("") : "..no seam here..", "utf8"),
    Buffer.from([0x00, 0x01]),
  ]);
  writeFileSync(path, bytes);
  return path;
}

test("the marker is the one the Rust seam carries", () => {
  const source = readFileSync(
    new URL("../src-tauri/src/tally/approved_import.rs", import.meta.url),
    "utf8",
  );
  assert.ok(source.includes(`const SEAM_MARKER: &str = "${SEAM_MARKER}";`));
});

test("the PDFium marker is the variable the Rust override reads, under bare cfg(test)", () => {
  const source = readFileSync(new URL("../src-tauri/src/agent_bank_statement.rs", import.meta.url), "utf8");
  const lookup = `env::var_os("${PDFIUM_OVERRIDE_MARKER}")`;
  assert.equal(source.split(lookup).length - 1, 1, "one lookup of the override");
  assert.ok(source.includes(`    #[cfg(test)]\n    if let Some(path) = ${lookup} {`));
});

test("the stand-in marker is the one the approval stand-in example uses at run time", () => {
  const source = readFileSync(new URL("../src-tauri/examples/approval_standin.rs", import.meta.url), "utf8");
  assert.ok(source.includes(`const STANDIN_MARKER: &str = "${STANDIN_MARKER}";`));
  assert.ok(source.includes("    std::hint::black_box(STANDIN_MARKER);\n"));
});

// Named, not read from TEST_ONLY_MARKERS, so a marker dropped from that list fails here.
const EXPECTED_MARKERS = [SEAM_MARKER, PDFIUM_OVERRIDE_MARKER, STANDIN_MARKER];

test("each marker has one positive control, on the test build that holds it", () => {
  assert.deepEqual(POSITIVE_CONTROLS, {
    harness: [SEAM_MARKER, PDFIUM_OVERRIDE_MARKER],
    standin: [STANDIN_MARKER],
  });
  assert.deepEqual(Object.values(POSITIVE_CONTROLS).flat().sort(), [...EXPECTED_MARKERS].sort());
});

test("each marker alone marks a binary, and the refusal names it", () => {
  assert.deepEqual(TEST_ONLY_MARKERS, EXPECTED_MARKERS);
  const directory = scratch();
  for (const marker of EXPECTED_MARKERS) {
    const file = binary(join(directory, marker), [marker]);
    assert.equal(holdsMarker(file), true, marker);
    assert.throws(() => assertNoTestSeam([file]), (error) => error.message.includes(`(${marker})`), marker);
  }
});

test("a binary holding the marker is found, and one without it is not", () => {
  const directory = scratch();
  assert.equal(holdsMarker(binary(join(directory, "marked"), true)), true);
  assert.equal(holdsMarker(binary(join(directory, "clean"), false)), false);
  assert.throws(() => assertNoTestSeam([join(directory, "marked")]), /test-only code compiled into/);
  assert.doesNotThrow(() => assertNoTestSeam([join(directory, "clean")]));
});

test("a directory such as an .app bundle is scanned file by file", () => {
  const directory = scratch();
  const executables = join(directory, "Bridge.app", "Contents", "MacOS");
  mkdirSync(executables, { recursive: true });
  binary(join(executables, "bridge"), false);
  binary(join(executables, "bridge_mcp"), true);
  assert.deepEqual(markedFiles([join(directory, "Bridge.app")]), [join(executables, "bridge_mcp")]);
});

test("compressed artefacts, missing paths and empty scans are refused, never passed", () => {
  const directory = scratch();
  for (const name of ["Bridge.dmg", "Bridge.msi", "bridge-tally.mcpb", "Bridge_0.2.0_x64-setup.exe"]) {
    binary(join(directory, name), false);
    assert.throws(() => markedFiles([join(directory, name)]), /compressed/);
  }
  assert.throws(() => markedFiles([join(directory, "absent")]), /does not exist/);
  mkdirSync(join(directory, "empty"));
  assert.throws(() => markedFiles([join(directory, "empty")]), /no regular files/);
  assert.throws(() => markedFiles([]), /no files given/);
});

test("an archive inside a scanned directory is refused, not scanned as compressed bytes", () => {
  const directory = scratch();
  const bundle = join(directory, "bundle");
  mkdirSync(join(bundle, "nested"), { recursive: true });
  binary(join(bundle, "bridge"), false);
  // Its bytes stand for an archive's: the marker inside an archive is compressed, so
  // reading them raw would pass it.
  binary(join(bundle, "nested", "inner.zip"), false);
  assert.throws(() => markedFiles([bundle]), { code: "compressed_artefact" });
  assert.throws(() => assertNoTestSeam([bundle]), { code: "compressed_artefact" });
  assert.throws(() => markedFiles([join(bundle, "nested", "inner.zip")]), { code: "compressed_artefact" });
});

test("an archive that is the first file a scanned directory yields is refused too", () => {
  // Every walked file is checked, not every one after the first (#1177's review): a
  // directory holding only an archive makes the archive the first file the walk yields.
  const directory = join(scratch(), "only-an-archive");
  mkdirSync(directory);
  binary(join(directory, "a.zip"), false);
  assert.throws(() => markedFiles([directory]), { code: "compressed_artefact" });
});

// A compressed file is refused by what it starts with, not by what it is called (#839): the marker inside one is
// compressed, so scanning its raw bytes would pass it. The first bytes of each family a bundler or an installer uses.
const MARKED = Buffer.from(`..${SEAM_MARKER}..`, "utf8");
const DEFLATED = deflateRawSync(MARKED);
const SIGNATURES = {
  gzip: gzipSync(MARKED),
  zip: Buffer.concat([Buffer.from([0x50, 0x4b, 0x03, 0x04]), Buffer.alloc(26), DEFLATED]),
  zstd: Buffer.concat([Buffer.from([0x28, 0xb5, 0x2f, 0xfd]), DEFLATED]),
  xz: Buffer.concat([Buffer.from([0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00]), DEFLATED]),
  bzip2: Buffer.concat([Buffer.from("BZh9", "latin1"), DEFLATED]),
  sevenZip: Buffer.concat([Buffer.from([0x37, 0x7a, 0xbc, 0xaf, 0x27, 0x1c]), DEFLATED]),
};

test("a compressed file is refused by its first bytes whatever it is called or where it sits", () => {
  for (const [family, bytes] of Object.entries(SIGNATURES)) {
    for (const name of ["payload", "payload.bin", "payload.zst"]) {
      const directory = scratch();
      const file = join(directory, name);
      // A clean executable sorts before the payload and another after it, so a check of only the first or the last
      // walked file would miss it (the same class as #1177).
      binary(join(directory, "a-bridge"), false);
      binary(join(directory, "zz-bridge"), false);
      writeFileSync(file, bytes);
      // The scan must be able to miss it for this test to measure anything: the marker is inside, and invisible.
      assert.equal(holdsMarker(file), false, `${family} ${name}: the marker must not show in the raw bytes`);
      assert.throws(() => markedFiles([file]), { code: "compressed_artefact" }, `${family} ${name} as an argument`);
      assert.throws(() => markedFiles([directory]), { code: "compressed_artefact" }, `${family} ${name} in a directory`);
      assert.throws(() => assertNoTestSeam([directory]), { code: "compressed_artefact" }, `${family} ${name} through the hook`);
    }
  }
});

test("a file that merely resembles a signature is reported or passed, never refused as compressed", () => {
  const directory = scratch();
  // An uncompressed file holding the marker is found, not refused.
  const marked = binary(join(directory, "payload"), true);
  assert.deepEqual(markedFiles([marked]), [marked]);
  // Short files, a truncated signature and a signature that is not at the start are not containers.
  const shapes = {
    empty: Buffer.alloc(0),
    oneByte: Buffer.from([0x1f]),
    truncatedZip: Buffer.from([0x50, 0x4b, 0x03]),
    twoLetters: Buffer.from("BZ", "latin1"),
    // The read buffer is zero-filled: an xz signature ends in a zero byte, so five bytes must not be read as six.
    truncatedXz: Buffer.from([0xfd, 0x37, 0x7a, 0x58, 0x5a]),
    signatureAtOffsetOne: Buffer.concat([Buffer.from([0x00]), SIGNATURES.gzip]),
  };
  for (const [shape, bytes] of Object.entries(shapes)) {
    const file = join(directory, `shape-${shape}`);
    writeFileSync(file, bytes);
    assert.doesNotThrow(() => markedFiles([file]), `${shape} is not a container`);
  }
});

test("the bundle hook scans the executables of the profile tauri built", () => {
  const root = scratch();
  const release = join(root, "src-tauri", "target", "release");
  const debug = join(root, "src-tauri", "target", "debug");
  const targeted = join(root, "src-tauri", "target", "aarch64-apple-darwin", "release");
  for (const directory of [release, debug, targeted]) mkdirSync(directory, { recursive: true });
  binary(join(release, "bridge"), false);
  binary(join(release, "bridge_mcp"), false);
  binary(join(debug, "bridge.exe"), false);
  binary(join(targeted, "bridge"), false);
  assert.deepEqual(tauriBuildExecutables({}, root).sort(), [
    join(release, "bridge"),
    join(release, "bridge_mcp"),
    join(targeted, "bridge"),
  ].sort());
  assert.deepEqual(tauriBuildExecutables({ TAURI_ENV_DEBUG: "true" }, root), [join(debug, "bridge.exe")]);
  // Cargo runs from src-tauri, so a relative target directory is resolved there.
  const custom = join(root, "src-tauri", "elsewhere", "release");
  mkdirSync(custom, { recursive: true });
  binary(join(custom, "bridge_mcp"), false);
  assert.deepEqual(tauriBuildExecutables({ CARGO_TARGET_DIR: "elsewhere" }, root), [join(custom, "bridge_mcp")]);
  assert.deepEqual(tauriBuildExecutables({ CARGO_BUILD_TARGET_DIR: "elsewhere" }, root), [join(custom, "bridge_mcp")]);
});

test("a bundle hook that finds no executable fails rather than passing", () => {
  const root = scratch();
  assert.throws(() => tauriBuildExecutables({}, root), /no bridge or bridge_mcp executable/);
});

test("the command line fails on a marked binary and on a control that sees nothing", () => {
  const directory = scratch();
  const script = fileURLToPath(new URL("./check-no-test-seam.mjs", import.meta.url));
  const run = (...argumentsList) =>
    spawnSync(process.execPath, [script, ...argumentsList], { encoding: "utf8" }).status;
  const marked = binary(join(directory, "marked"), true);
  const clean = binary(join(directory, "clean"), false);
  assert.equal(run(clean), 0);
  assert.equal(run(marked), 1);
  assert.equal(run(clean, marked), 1);
  assert.equal(run("--expect-present", marked, ...EXPECTED_MARKERS), 0);
  assert.equal(run("--expect-present", clean, ...EXPECTED_MARKERS), 1);
  // A control must name what it expects, from the test-only markers.
  assert.equal(run("--expect-present", marked), 1);
  assert.equal(run("--expect-present", marked, "not-a-test-only-marker"), 1);
  // A control that sees only some of the test-only code proves nothing about the rest.
  for (const marker of EXPECTED_MARKERS) {
    assert.equal(run("--expect-present", binary(join(directory, `only-${marker}`), [marker]), ...EXPECTED_MARKERS), 1, marker);
  }
});

test("each positive control passes on its own test build and fails on the other's (#702)", () => {
  const directory = scratch();
  const script = fileURLToPath(new URL("./check-no-test-seam.mjs", import.meta.url));
  const run = (...argumentsList) => spawnSync(process.execPath, [script, ...argumentsList], { encoding: "utf8" });
  // What the run said is missing, so a control that fails for another reason does not pass a row.
  const missing = (result) => [result.status, result.stderr.match(/does not hold (.*): the scan/)?.[1]];
  for (const [control, expected] of Object.entries(POSITIVE_CONTROLS)) {
    // The build the control is run on holds its own markers and none of the others'.
    assert.equal(run("--expect-present", binary(join(directory, `${control}-own`), expected), ...expected).status, 0, control);
    for (const [other, markers] of Object.entries(POSITIVE_CONTROLS)) {
      if (other === control) continue;
      const result = run("--expect-present", binary(join(directory, `${control}-on-${other}`), markers), ...expected);
      assert.deepEqual(missing(result), [1, expected.join(", ")], `${control} on ${other}`);
    }
    // Missing any one of its markers fails it, while the file still holds another marker.
    for (const marker of expected) {
      const held = TEST_ONLY_MARKERS.filter((kept) => kept !== marker);
      const result = run("--expect-present", binary(join(directory, `${control}-without-${marker}`), held), ...expected);
      assert.deepEqual(missing(result), [1, marker], `${control} without ${marker}`);
    }
  }
});
