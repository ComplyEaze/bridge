// Proves the test-only native-approval seam (bridge#583) and the test-only
// PDFium library override are absent from every shipped executable, and
// present where they must be, so that their absence means something.
//
// The seam lives in src-tauri/src/tally/approved_import.rs under bare
// `#[cfg(test)]` and carries SEAM_MARKER, which it uses at runtime so the
// optimiser keeps it. The override is pdfium_library() in
// src-tauri/src/agent_bank_statement.rs, whose bare `#[cfg(test)]` lookup
// reads the variable PDFIUM_OVERRIDE_MARKER names. A binary holding either
// marker was compiled with that test-only code.
//
//   node scripts/check-no-test-seam.mjs <file-or-directory>...
//       Fails if any regular file at or under the paths holds a marker.
//   node scripts/check-no-test-seam.mjs --expect-present <file>
//       Fails unless the file holds every marker (a positive control).
//   node scripts/check-no-test-seam.mjs --tauri-bundle-hook
//       Tauri's beforeBundleCommand: scans the bridge and bridge_mcp
//       executables `tauri build` just produced.
//   node scripts/check-no-test-seam.mjs --test-harness [--release]
//       Builds (or reuses) the bridge lib unit-test executable and requires
//       every marker in it: proof the scan can see the test-only code when it
//       is compiled in, in that profile.
//
// Only uncompressed executables prove anything. A .dmg, .msi, .zip, .mcpb or
// installer compresses its contents, so a clean scan of one would pass
// whatever it holds; those are refused rather than scanned. A file is refused by
// its name, and also by its first bytes (a gzip, zip, zstd, xz, bzip2 or 7z
// signature) wherever it sits, whatever it is called (#839).
import { spawnSync } from "node:child_process";
import { closeSync, existsSync, openSync, readdirSync, readFileSync, readSync, statSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

export const SEAM_MARKER = "bridge-test-approval-seam-5f1c9e7a";
export const PDFIUM_OVERRIDE_MARKER = "BRIDGE_PDFIUM_LIBRARY";
export const TEST_ONLY_MARKERS = [SEAM_MARKER, PDFIUM_OVERRIDE_MARKER];

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const COMPRESSED = /\.(dmg|msi|zip|mcpb|gz|tgz|xz|bz2|7z|pkg|appimage|deb|rpm)$|-setup\.exe$/i;
// What a compressed file starts with, whatever it is called. The list is not exhaustive: the exact executables a bundle
// is built from are scanned separately (ci.yml, the bundle smoke step), which does not depend on this list.
const CONTAINER_SIGNATURES = [
  ["gzip", [0x1f, 0x8b]],
  ["zip", [0x50, 0x4b, 0x03, 0x04]],
  ["zstd", [0x28, 0xb5, 0x2f, 0xfd]],
  ["xz", [0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00]],
  ["bzip2", [0x42, 0x5a, 0x68]],
  ["7z", [0x37, 0x7a, 0xbc, 0xaf, 0x27, 0x1c]],
];
const SHIPPED_BINARIES = ["bridge", "bridge_mcp"];

/** The markers `path` holds, in TEST_ONLY_MARKERS order. */
export function markersIn(path) {
  const bytes = readFileSync(path);
  return TEST_ONLY_MARKERS.filter((marker) => bytes.includes(Buffer.from(marker, "utf8")));
}

export function holdsMarker(path) {
  return markersIn(path).length > 0;
}

/** Every regular file at or under `path`. */
export function filesUnder(path) {
  const stat = statSync(path);
  if (stat.isFile()) return [path];
  if (!stat.isDirectory()) return [];
  return readdirSync(path).flatMap((entry) => filesUnder(join(path, entry)));
}

/** Throws, with code `compressed_artefact`, if `path` names a compressed archive. */
function refuseCompressed(path) {
  if (COMPRESSED.test(path)) {
    throw Object.assign(
      new Error(`${path} is compressed: scan the executables it was built from instead`),
      { code: "compressed_artefact" },
    );
  }
}

/** The family of container `path` starts with, or undefined: read from the first bytes, never from the name. */
function containerSignature(path) {
  const head = Buffer.alloc(6);
  const descriptor = openSync(path, "r");
  let length;
  try {
    length = readSync(descriptor, head, 0, head.length, 0);
  } finally {
    closeSync(descriptor);
  }
  return CONTAINER_SIGNATURES.find(
    ([, signature]) => length >= signature.length && signature.every((byte, index) => head[index] === byte),
  )?.[0];
}

/** Throws, with code `compressed_artefact`, if `path` names a compressed archive or starts like one. */
function refuseCompressedFile(path) {
  refuseCompressed(path);
  const family = containerSignature(path);
  if (family) {
    throw Object.assign(
      new Error(`${path} is compressed (it starts like a ${family} file): scan the executables it was built from instead`),
      { code: "compressed_artefact" },
    );
  }
}

/**
 * The files among `paths` that hold the marker. Refuses compressed archives, both
 * as arguments and wherever a walked directory holds one: an archive's bytes are
 * compressed, so a clean scan of one inside a directory proves no more than a clean
 * scan of one given directly (#839).
 */
export function markedFiles(paths) {
  if (paths.length === 0) throw new Error("no files given to scan");
  const files = [];
  for (const path of paths) {
    refuseCompressed(path);
    if (!existsSync(path)) throw new Error(`${path} does not exist`);
    files.push(...filesUnder(path));
  }
  if (files.length === 0) throw new Error(`no regular files under ${paths.join(", ")}`);
  files.forEach(refuseCompressedFile);
  return files.filter(holdsMarker);
}

/** Throws if any of `paths` holds the marker; for callers such as package-mcpb. */
export function assertNoTestSeam(paths) {
  const marked = markedFiles(paths);
  if (marked.length > 0) {
    throw new Error(
      `test-only code compiled into: ${marked.map((path) => `${path} (${markersIn(path).join(", ")})`).join(", ")}; ` +
        "a shipped binary was built with cfg(test)",
    );
  }
}

/**
 * The executables a `tauri build` produced: bridge and bridge_mcp in the
 * profile directory the build used. Tauri gives the hook TAURI_ENV_DEBUG and
 * TAURI_ENV_TARGET_TRIPLE, but a build without `--target` still writes to
 * `target/<profile>`, so the host directory and every `target/<triple>/<profile>`
 * are scanned; scanning a stale binary too is harmless. Cargo runs from
 * src-tauri, so a relative CARGO_TARGET_DIR (or CARGO_BUILD_TARGET_DIR) is
 * resolved there. Finding none is a failure: a hook that saw nothing proved
 * nothing. A target directory set only in a Cargo config file, or a custom
 * `--profile`, is not followed; CI scans its exact output paths separately.
 */
export function tauriBuildExecutables(environment = process.env, sourceRoot = root) {
  const profile = environment.TAURI_ENV_DEBUG === "true" ? "debug" : "release";
  const configured = environment.CARGO_TARGET_DIR || environment.CARGO_BUILD_TARGET_DIR;
  const target = configured
    ? resolve(sourceRoot, "src-tauri", configured)
    : resolve(sourceRoot, "src-tauri", "target");
  const directories = [join(target, profile)];
  if (existsSync(target)) {
    for (const entry of readdirSync(target)) {
      const candidate = join(target, entry, profile);
      if (entry !== profile && existsSync(candidate)) directories.push(candidate);
    }
  }
  const executables = directories.flatMap((directory) =>
    SHIPPED_BINARIES.flatMap((name) => [name, `${name}.exe`])
      .map((name) => join(directory, name))
      .filter((path) => existsSync(path) && statSync(path).isFile()),
  );
  if (executables.length === 0) {
    throw new Error(`no bridge or bridge_mcp executable under ${target} (${profile})`);
  }
  return executables;
}

/** The bridge lib unit-test executable Cargo builds for `profile`. */
export function testHarnessExecutable(release, sourceRoot = root) {
  const argumentsList = [
    "test", "--locked", "--no-run", "--lib", "--message-format=json",
    "--manifest-path", resolve(sourceRoot, "src-tauri", "Cargo.toml"),
  ];
  // A `-p bridge` build resolves features differently from the `--workspace` build the
  // native job has just made, and recompiles the tauri stack. The debug path selects the
  // workspace so it reuses that build; the release path keeps `-p bridge`, whose
  // release dependencies are what bundle-smoke's cache holds.
  if (release) argumentsList.push("-p", "bridge", "--release");
  else argumentsList.push("--workspace");
  const build = spawnSync("cargo", argumentsList, {
    cwd: sourceRoot,
    encoding: "utf8",
    maxBuffer: 256 * 1024 * 1024,
    stdio: ["ignore", "pipe", "inherit"],
  });
  if (build.status !== 0) throw new Error(`cargo test --no-run failed (${build.status})`);
  const executables = build.stdout
    .split("\n")
    .filter((line) => line.startsWith("{"))
    .map((line) => JSON.parse(line))
    .filter(
      (message) =>
        message.reason === "compiler-artifact" &&
        message.target?.name === "bridge_lib" &&
        message.profile?.test === true &&
        message.executable,
    )
    .map((message) => message.executable);
  if (executables.length !== 1) {
    throw new Error(`expected one bridge lib test executable, found ${executables.length}`);
  }
  return executables[0];
}

function main(argumentsList) {
  if (argumentsList[0] === "--expect-present") {
    const [file] = argumentsList.slice(1);
    // The same scan the negative check runs, so the control proves that code.
    if (!file || markedFiles([file]).length !== 1) {
      throw new Error(`${file ?? "(no file)"} holds no marker: the scan cannot see the test-only code`);
    }
    const missing = TEST_ONLY_MARKERS.filter((marker) => !markersIn(file).includes(marker));
    if (missing.length > 0) {
      throw new Error(`${file} does not hold ${missing.join(", ")}: the scan cannot see that test-only code`);
    }
    console.log(`every marker present in ${basename(file)}, as a positive control requires`);
    return;
  }
  if (argumentsList[0] === "--test-harness") {
    const release = argumentsList.includes("--release");
    const executable = testHarnessExecutable(release);
    main(["--expect-present", executable]);
    return;
  }
  const paths = argumentsList[0] === "--tauri-bundle-hook" ? tauriBuildExecutables() : argumentsList;
  assertNoTestSeam(paths);
  console.log(`no test-only code in ${paths.length} path(s): ${paths.map((path) => basename(path)).join(", ")}`);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  try {
    main(process.argv.slice(2));
  } catch (error) {
    console.error(`check-no-test-seam: ${error.message}`);
    process.exit(1);
  }
}
