// SPDX-License-Identifier: Apache-2.0

// Locks in the README's central privacy promise (README.md, 'What it does not do'):
//
//   "Your Tally data is never uploaded. Bridge reads it over a local
//   connection and hands it to the assistant you are talking to; nothing in
//   the Tally path sends it to a server of ours."
//
// and the separation promise for the upload-capable parts of the app
// (README.md, 'One part of the app does upload'):
//
//   "Bridge also contains a document feature that uploads files you choose
//   to ComplyEaze cloud storage, and an AXAL sign-in. Those are separate and
//   user-initiated, and share no code with the Tally path described here"
//
// Before this gate, both sentences were prose: nothing stopped a new
// `reqwest` call site from landing anywhere in the tree, including inside
// the Tally read/write path itself.
//
// Two checks, deliberately different in what they can see:
//
// 1. A `cargo tree` boundary (modelled on
//    scripts/check-tally-live-read-boundary.mjs): asserts which first-party
//    crates are allowed to declare a *direct* dependency on an outbound HTTP
//    client (reqwest) or its transport (hyper) at all. This is a
//    compile-time property -- cargo will not link a crate against reqwest
//    unless its Cargo.toml says so -- but it only sees crate boundaries. It
//    cannot see what a crate that *is* allowed to depend on reqwest
//    (`bridge`, the app crate, which legitimately needs it for
//    axal.rs/documents.rs -- see the note above APP_CRATE: both ship in the
//    extension binary too, not only in the desktop app) does with that
//    dependency inside its own files. It follows normal, build and dev
//    edges alike: a dev- or build-dependency on reqwest in a crate outside
//    the allow-list is refused too, since a test double or build script
//    that can open a connection is still egress from a developer's machine.
//    It resolves every target platform, so a Windows- or macOS-only
//    dependency is seen from the Linux CI job. The sets are pinned exactly and the tree must be seen: a crate that
//    drops out, a missing root line, a failed `cargo` or an unparseable line
//    all fail, so "nothing found" cannot stand in for "nothing was read".
//
// 2. A clippy census. src-tauri/clippy.toml refuses the egress actions
//    everywhere in the src-tauri workspace: sending an HTTP request, opening
//    or binding a socket, a DNS lookup, spawning a process, building a webview
//    window. A lint fires however the client or socket was obtained, which a
//    text scan cannot see. A reviewed call site carries
//    `#[expect(clippy::disallowed_methods, reason = "...")]`. CI's native job
//    then runs clippy over the shipped targets with the two egress lints forced
//    to warn, which no group allow, `-A` or lint table can turn off (measured),
//    and this script (`--census FILE OS`) requires that what fired match
//    scripts/tally-egress-census.json exactly, per OS. rustc measures what
//    fires, so no source syntax can hide a call in the code the run compiles.
//    Beside it, static checks keep the configuration in place: the digest of
//    clippy.toml, no second clippy.toml, no CLIPPY_CONF_DIR or CLIPPY_ARGS in
//    any tracked file, no cargo config, every build.rs under src-tauri pinned
//    by digest and no `build =` key in a src-tauri Cargo.toml, and no line
//    of src-tauri Rust naming `clippy` (outside a lint path), `debug_assertions`,
//    `panic =` or `dev` as a cfg list item, which the run compiles differently
//    from the shipped build.
//
// 3. A deny-list of network-capable crates and Tauri plugins, read from both
//    Cargo.lock files and the JS manifests.
//
// What this gate does NOT prove (read before relying on it further):
//  - It does not prove the Tally transport's loopback restriction
//    (`non_loopback_forbidden` in bridge-tally-transport, which rejects any
//    non-loopback host at request-construction time) is itself correct or
//    still wired up -- that is a runtime property with its own tests in
//    bridge-tally-transport, not this gate.
//  - The lints name specific methods. An egress path through a method they do
//    not name (a crate on neither list, an FFI function not listed, a native
//    library) is outside them; the deny-list and the cargo-tree half narrow
//    that. The Rust calls that navigate the webview or run script in it are
//    linted; what the page's own script can reach is the webview CSP's
//    concern, and the CSP does not govern a top-level navigation.
//  - The census sees only code its clippy run compiles: the shipped targets
//    (`--lib --bins`, dev profile) of the workspace members, with default
//    features, on Windows and macOS (arm64 only; scripts/package-mcpb.mjs lists
//    an x86_64 macOS target the census never lints, although README.md says
//    Intel Macs are unsupported). Not seen: code under a cfg the run compiles
//    differently from the shipped build (a line naming one of those words is
//    refused, as the static half says above; a cfg spelled some other way, or
//    a profile difference not named, is not); a file the token rule does not
//    read, pulled in with `#[path]` or `include!` from outside src-tauri or as
//    a non-`.rs` file; features the shipped build turns on that the run does
//    not (tauri.conf.json's build features, `tauri build --features`); other
//    features the run does not enable (voucher-scan, the calibration harness,
//    lab-writes and bridge-tally-protocol's evidence features); Linux-only code; path dependencies that are not workspace
//    members (clippy does not lint them; measured); and tests, examples and
//    benches, which do not ship. Measured on a two-file crate: a listed call
//    made by a dependency's own `macro_rules!` macro fires and is attributed to
//    its call site, and a typo in a listed path is reported as a warning located
//    in clippy.toml, which the census counts. A proc-macro's expansion was not
//    measured.
//  - Nine listed FFI paths are inert on Windows: the first Windows run reported
//    libc::posix_spawn, posix_spawnp and getaddrinfo and windows-sys's
//    ShellExecuteW, ShellExecuteA, ShellExecuteExW, CreateProcessW,
//    CreateProcessA and WinExec as resolving to nothing. The libc ones are
//    expected (libc is a unix-only dependency). The six windows-sys ones are
//    unexplained: windows-sys is a dependency of the app crate with the Shell
//    and Threading features. They rest on one run. The census pins them as
//    inert, so they are not seen firing; the static half refuses those function
//    names (and execv, execvp, execve) in src-tauri Rust.
//  - Egress routes that name no listed method: a direct dependency on `tower`
//    is refused, but reqwest's `blocking` client and any other crate's send
//    method are outside the lists. tauri.conf.json (the app's windows and its
//    CSP) is not read by this gate.
//  - A pin is (file, method, count): removing one reviewed call and adding
//    another of the same method in the same file passes; only the diff shows it.
//  - Removing or narrowing the census step in ci.yml is not detected here.
//    ci.yml is a compatibility-pinned file, so an edit to it is a resealed,
//    reviewed change.
//  - On Windows, opening a UNC or WebDAV path through std::fs reaches the
//    network; no method list can tell such a path from a local one.
//  - Dropping `-D warnings` from a CI clippy step, or narrowing what it
//    covers, is not detected here.
//  - tools/ is outside the lints: its binaries do not ship. The cargo-tree half
//    and the deny-list still cover it.
//  - The lints run in CI's clippy steps; this gate cannot see a build that
//    skips clippy.

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { posix } from "node:path";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../", import.meta.url));

// ---------------------------------------------------------------------------
// Check 2: a clippy census of the egress lints, and the settings that keep it honest.
// ---------------------------------------------------------------------------

// CI's native job writes clippy's JSON to a file (see ci.yml) and calls
//     node scripts/check-tally-egress-boundary.mjs --census FILE OS [PINS]
// The census counts the egress-lint firings per file and method and requires an exact two-way
// match with the reviewed list for that OS: a new firing, a firing that stopped, a build that did
// not finish, an empty file and a list with a zero count all fail. A list is never empty, so a run
// whose lints never fired cannot pass as clean. Measured on a two-file crate with clippy-driver
// 1.96.0 (scripts/testdata/egress-census-clippy-capture.PROVENANCE.md), `--force-warn` still fires
// under `#[allow(clippy::all)]`, `#![allow(warnings)]`, `-A clippy::all` and a
// `[lints.clippy] all = "allow"` table, and RUSTFLAGS `--cap-lints allow` silences every lint
// message; that run reports nothing, which the nonempty list refuses.
//
// scripts/tally-egress-census.json lists, per OS, [file, method, count] for each reviewed call
// site. The entries come from a real run's output, never typed in by hand: a run that disagrees
// prints the list it observed, and a reviewer confirms each entry before copying it there.
const CENSUS_PINS = `${root}scripts/tally-egress-census.json`;

// The two lints the census counts, and the message rustc prints for them, which names the method or
// type in backticks. A listed path that resolves to nothing (a typo, or a function this OS's crate
// lacks) is reported as a warning located in clippy.toml, and would otherwise silently disable its
// entry, so those are counted as well.
const EGRESS_LINTS = ["clippy::disallowed_methods", "clippy::disallowed_types"];
const FIRING = /^use of a disallowed (?:method|type) `([^`]+)`/;
const UNREACHABLE = /^`([^`]+)` does not refer to a reachable /;

// Where a firing is reported: the outermost macro expansion site of the primary span, as a path
// from the repository root. cargo reports paths from the workspace root (src-tauri/), or absolute
// (clippy.toml, a path outside the workspace).
function firingFile(message) {
  let span = message.spans.find((candidate) => candidate.is_primary);
  while (span?.expansion?.span) span = span.expansion.span;
  if (!span) return null;
  // Windows prints an absolute path as `\\?\D:\...`, seen in CI as `//?/D:/...` here.
  const name = span.file_name.replaceAll("\\", "/").replace(/^\/\/\?\//, "");
  const here = root.replaceAll("\\", "/");
  // A Windows drive letter may differ in case between cargo's path and this script's own.
  if (name.toLowerCase().startsWith(here.toLowerCase())) return posix.normalize(name.slice(here.length));
  return /^(?:[A-Za-z]:)?\//.test(name) ? name : posix.normalize(`src-tauri/${name}`);
}

function censusViolations(text, os, pins) {
  const expected = Array.isArray(pins[os]) ? pins[os] : [];
  // A count of zero, a repeated key or a malformed row would let a run that fired nothing match.
  const shape = [];
  const keys = new Set();
  for (const row of expected) {
    const valid = Array.isArray(row) && row.length === 3 && typeof row[0] === "string" && typeof row[1] === "string" && Number.isInteger(row[2]) && row[2] >= 1;
    if (!valid) shape.push(`a reviewed entry is not [file, method, count of at least 1]: ${JSON.stringify(row)}`);
    else if (keys.has(`${row[0]}\t${row[1]}`)) shape.push(`a reviewed entry is repeated: ${row[0]} ${row[1]}`);
    else keys.add(`${row[0]}\t${row[1]}`);
  }
  if (shape.length) return shape;
  const lines = text.split(/\r?\n/).filter((line) => line !== "");
  if (lines.length === 0) return ["the clippy output is empty: the lints did not run"];
  const finished = [];
  const seen = new Map();
  for (const [index, line] of lines.entries()) {
    let entry;
    try {
      entry = JSON.parse(line);
    } catch {
      return [`clippy output line ${index + 1} is not JSON: ${JSON.stringify(line.slice(0, 80))}`];
    }
    if (entry.reason === "build-finished") finished.push(entry.success);
    if (entry.reason !== "compiler-message") continue;
    const listed = EGRESS_LINTS.includes(entry.message?.code?.code) || FIRING.test(entry.message?.message ?? "");
    const unreachable = UNREACHABLE.test(entry.message?.message ?? "");
    if (!listed && !unreachable) continue;
    const method = entry.message.message.match(listed ? FIRING : UNREACHABLE)?.[1];
    const file = firingFile(entry.message);
    if (!method || !file) return [`an egress lint message could not be read: ${JSON.stringify(entry.message.message.slice(0, 120))}`];
    if (!listed && !file.endsWith("/clippy.toml")) continue;
    const key = `${file}\t${method}`;
    // clippy reports an unresolved path once per crate, so how often depends on the crate count;
    // it counts once. A firing counts each time.
    seen.set(key, listed ? (seen.get(key) ?? 0) + 1 : 1);
  }
  const violations = [];
  // An empty list would let a run that fired nothing pass, so it is a failure that still prints
  // what this run observed, for the review that fills the list.
  if (expected.length === 0) violations.push(`no reviewed census for runner OS ${JSON.stringify(os)}; add one from a real run`);
  if (finished.length !== 1 || finished[0] !== true) {
    violations.push(`clippy did not report exactly one successful build (build-finished: ${JSON.stringify(finished)})`);
  }
  const pinned = new Map(expected.map(([file, method, count]) => [`${file}\t${method}`, count]));
  for (const key of new Set([...seen.keys(), ...pinned.keys()])) {
    const [file, method] = key.split("\t");
    const found = seen.get(key) ?? 0;
    const reviewed = pinned.get(key) ?? 0;
    const what = file.endsWith("/clippy.toml") ? "listed path(s) that resolve to nothing" : "egress-lint firing(s)";
    if (found !== reviewed) violations.push(`${file}: ${found} ${what} of ${method}, ${reviewed} reviewed`);
  }
  if (violations.length) {
    const observed = [...seen].map(([key, count]) => `    [${JSON.stringify(key.split("\t")[0])}, ${JSON.stringify(key.split("\t")[1])}, ${count}],`);
    violations.push(`observed for ${os}, to be reviewed and pinned in scripts/tally-egress-census.json:\n${observed.join("\n")}`);
  }
  return violations;
}

const censusFlag = process.argv.indexOf("--census");
if (censusFlag !== -1) {
  const [file, os, pinsFile = CENSUS_PINS] = process.argv.slice(censusFlag + 1);
  if (!file || !os) throw new Error("usage: check-tally-egress-boundary.mjs --census <clippy-json-file> <runner-os> [pins-file]");
  const pins = JSON.parse(readFileSync(pinsFile, "utf8"));
  const violations = censusViolations(readFileSync(file, "utf8"), os, pins);
  if (violations.length) {
    throw new Error(`Tally-path egress census failed (${os}):\n${violations.map((violation) => `- ${violation}`).join("\n")}`);
  }
  const total = pins[os].reduce((sum, [, , count]) => sum + count, 0);
  console.log(`Tally-path egress census matches for ${os}: ${total} reviewed firing(s) in ${pins[os].length} place(s), and no other.`);
  process.exit(0);
}

// ---------------------------------------------------------------------------
// Check 1: which first-party crates may directly depend on an outbound HTTP
// client at all.
// ---------------------------------------------------------------------------

// The one crate that is allowed to carry reqwest as a *direct* dependency
// for talking to Tally itself: the loopback-only HTTP transport. It is
// reached by `bridge` (the app crate) and, through it, by the read-only
// tools in tools/bridge-tally-live-read -- both expected, both already
// covered by scripts/check-tally-live-read-boundary.mjs's own first-party
// boundary list. This gate only asks one narrower question: which crates
// declare reqwest *directly*, not which crates reach it transitively.
const TALLY_HTTP_TRANSPORT_CRATE = "bridge-tally-transport";

// `bridge` is the app crate. It legitimately depends on reqwest directly for
// two things that are NOT the Tally path: axal.rs (AXAL sign-in / cloud
// storage) and documents.rs (the document upload feature). The README paragraph
// 'One part of the app does upload' names both explicitly as the parts of the
// app that DO upload.
//
// Do not read "app crate" as "Tauri only". Both modules are declared
// unconditionally in lib.rs, with no cfg(feature) gate; src/bin/bridge_mcp.rs
// links bridge_lib; and scripts/package-mcpb.mjs ships bridge_mcp as the
// extension binary. So this reqwest edge is compiled into the artifact a user
// installs, not just into the desktop app. The honest claim is "present and
// unreachable from the agent surface", not "absent" -- and "unreachable" is
// what the lint check below exists to keep true.
//
// The standard this gate is modelled on is the Tally transport's own loopback
// guard, which is stronger than a file allow-list: `endpoint_url` special-cases
// only the literal string "localhost" and hardcodes 127.0.0.1 without ever
// resolving DNS, so a hosts-file entry aiming a hostname at loopback cannot
// pass. It runs on every network method rather than once at construction,
// redirects are disabled, and no environment variable or cargo feature relaxes
// it. Where a future control can be written that way, prefer it to a list.
const APP_CRATE = "bridge";

function directDependents(manifestPath, packageName) {
  const result = spawnSync(
    "cargo",
    [
      "tree",
      "--locked",
      "--manifest-path",
      manifestPath,
      "-p",
      packageName,
      "--invert",
      "--depth",
      "1",
      "--edges",
      "normal,build,dev",
      // Every platform, not only the host: CI runs on Linux, and a
      // `[target.'cfg(windows)'.dependencies]` edge is otherwise invisible.
      "--target",
      "all",
      "--prefix",
      "none",
      "--format",
      "{p}",
    ],
    { cwd: root, encoding: "utf8", maxBuffer: 64 * 1024 * 1024, windowsHide: true },
  );
  // Both workspaces resolve both packages, so there is no "nothing to check"
  // exit here: any failure, including "did not match any packages", means
  // the tree was not read.
  if (result.error) {
    throw new Error(`dependency tree for ${packageName} (${manifestPath}) could not run cargo: ${result.error.message}`);
  }
  if (result.status !== 0) {
    throw new Error(`dependency tree for ${packageName} (${manifestPath}) exited ${result.status}: ${result.stderr}`);
  }
  const lines = result.stdout.split(/\r?\n/).filter((line) => line !== "");
  const parsed = lines.map((line) => {
    const match = line.match(/^([A-Za-z0-9_.+-]+) v\S+(?: \((.+)\))?$/);
    if (!match) {
      throw new Error(`dependency tree for ${packageName} (${manifestPath}) printed an unparseable line: ${JSON.stringify(line)}`);
    }
    return { name: match[1], source: match[2] };
  });
  // `--invert` prints the package itself first. Without that line no tree
  // was produced, and an empty dependent list would prove nothing.
  if (parsed[0]?.name !== packageName) {
    throw new Error(
      `dependency tree for ${packageName} (${manifestPath}) did not start with ${packageName}; ` +
        `got ${JSON.stringify(lines[0] ?? "")} (${lines.length} line(s))`,
    );
  }
  // Only lines carrying a parenthesized on-disk path are first-party
  // (workspace or path) dependencies -- `cargo tree`'s `{p}` format appends
  // that path for anything not resolved from a registry. A third-party
  // crate that happens to also depend on `packageName` directly (e.g.
  // hyper-util and hyper-rustls both depend on hyper directly, same as
  // reqwest does) has no such path and is not what this gate is asking
  // about: it cares which crates *we* wrote declare the dependency, not
  // reqwest's own internal transport plumbing.
  const names = new Set();
  for (const { name, source } of parsed.slice(1)) {
    if (source?.startsWith(root.slice(0, -1))) names.add(name);
  }
  return [...names].sort();
}

// The exact first-party crates with a direct dependency on each package, per
// workspace, as `cargo tree` reports them today. The tools workspace reaches
// reqwest only through the transport; hyper is reqwest's own transport, and
// a first-party crate using it directly would build an HTTP client that
// bypasses reqwest and bridge-tally-transport's loopback check entirely.
// Network crates such as these sit in the lockfiles under reqwest and tokio, so the deny-list
// below cannot refuse them; no first-party crate may depend on one directly, since each is a way
// to open a connection that names no linted method. The list is not exhaustive.
const LOWER_LEVEL_NETWORK = { h2: [], "hyper-util": [], socket2: [], mio: [], "tower-service": [], tower: [] };

const workspaces = [
  {
    label: "src-tauri",
    manifestPath: "src-tauri/Cargo.toml",
    expected: { reqwest: [APP_CRATE, TALLY_HTTP_TRANSPORT_CRATE], hyper: [], ...LOWER_LEVEL_NETWORK },
  },
  {
    label: "tools",
    manifestPath: "tools/Cargo.toml",
    expected: { reqwest: [TALLY_HTTP_TRANSPORT_CRATE], hyper: [], ...LOWER_LEVEL_NETWORK },
  },
];

const egressViolations = [];

for (const workspace of workspaces) {
  for (const [packageName, expectedNames] of Object.entries(workspace.expected)) {
    const expected = [...expectedNames].sort();
    const actual = directDependents(workspace.manifestPath, packageName);
    const gained = actual.filter((name) => !expected.includes(name));
    const lost = expected.filter((name) => !actual.includes(name));
    if (gained.length) {
      egressViolations.push(
        `${workspace.label}: crate(s) gained a direct ${packageName} dependency outside the pinned set ` +
          `(${expected.join(", ") || "none"}): ${gained.join(", ")}`,
      );
    }
    if (lost.length) {
      egressViolations.push(
        `${workspace.label}: pinned crate(s) no longer show a direct ${packageName} dependency: ` +
          `${lost.join(", ")}. Either the tree was not read in full or the dependency moved; narrow the ` +
          "pinned set in scripts/check-tally-egress-boundary.mjs only after confirming which.",
      );
    }
  }
}

// ---------------------------------------------------------------------------
// Check 2, static half: the settings that keep the census honest.
// ---------------------------------------------------------------------------

function trackedFiles() {
  const result = spawnSync("git", ["-C", root, "ls-files", "-z"], { encoding: "utf8", windowsHide: true });
  if (result.error || result.status !== 0) {
    throw new Error(`git ls-files failed: ${result.error?.message ?? result.stderr}`);
  }
  return result.stdout.split("\0").filter(Boolean);
}

// Clippy reads the nearest clippy.toml, so a second one under src-tauri would replace these lists
// for its crate, and CLIPPY_CONF_DIR would point it elsewhere. An edit to the lists needs review.
const CLIPPY_CONFIG_DIGEST = "b0545bfeb20c2ef9881c497470e927c746e7a57714ca1c9088b8ca707e7c144f";
const clippyConfig = createHash("sha256").update(readFileSync(`${root}src-tauri/clippy.toml`)).digest("hex");
if (clippyConfig !== CLIPPY_CONFIG_DIGEST) {
  egressViolations.push(`src-tauri/clippy.toml changed; review its egress lists, then set CLIPPY_CONFIG_DIGEST to ${clippyConfig}`);
}
const tracked = trackedFiles();
for (const path of tracked) {
  if (/(?:^|\/)\.?clippy\.toml$/i.test(path) && path.startsWith("src-tauri/") && path !== "src-tauri/clippy.toml") {
    egressViolations.push(`${path} would replace src-tauri/clippy.toml's egress lints for its crate`);
  }
  // A cargo config can alias `clippy`, wrap rustc or set the environment of the census run.
  if (/(?:^|\/)\.cargo\/config(?:\.toml)?$/i.test(path)) {
    egressViolations.push(`${path} is a cargo config; it could redirect the census run, so none is tracked`);
  }
}
// A build script can set a cfg or an environment variable for the crate it builds. Each tracked one
// under src-tauri is pinned by digest (one statement today); a new one is refused until it is added.
const BUILD_SCRIPT_DIGESTS = new Map([
  ["src-tauri/build.rs", "487059eaf8a947b80f20a9aacac038a5047b2ad69d2401b827376c67d6fe847f"],
]);
for (const path of tracked.filter((name) => name.startsWith("src-tauri/") && /(?:^|\/)build\.rs$/.test(name))) {
  const digest = createHash("sha256").update(readFileSync(`${root}${path}`)).digest("hex");
  if (BUILD_SCRIPT_DIGESTS.get(path) !== digest) {
    egressViolations.push(`${path} is new or changed; check it sets no cfg or environment for the census run, then pin ${digest} in BUILD_SCRIPT_DIGESTS`);
  }
}
for (const path of BUILD_SCRIPT_DIGESTS.keys()) {
  if (!tracked.includes(path)) egressViolations.push(`BUILD_SCRIPT_DIGESTS names ${path}, which is not a tracked file`);
}
// A plain substring in any tracked text file but this gate and its test. `git grep` exits 1 for no
// match; any other failure means nothing was read.
const settings = spawnSync(
  "git",
  ["-C", root, "grep", "-l", "-I", "-F", "-e", "CLIPPY_CONF_DIR", "-e", "CLIPPY_ARGS", "--", ".",
    ":!scripts/check-tally-egress-boundary.mjs", ":!scripts/check-tally-egress-boundary.test.mjs"],
  { encoding: "utf8", windowsHide: true },
);
if (settings.error || (settings.status !== 0 && settings.status !== 1)) {
  throw new Error(`git grep for the clippy settings failed: ${settings.error?.message ?? settings.stderr}`);
}
for (const path of settings.stdout.split("\n").filter(Boolean)) {
  egressViolations.push(`${path} names CLIPPY_CONF_DIR or CLIPPY_ARGS, which can point clippy away from src-tauri/clippy.toml`);
}
// Code the census run does not compile is not seen: code under a `clippy` cfg (it only ever
// compiles under clippy, so the run that would see it skips it), under `dev` (tauri-build sets it
// unless the shipped `custom-protocol` feature is on), under `debug_assertions`, or under a `panic`
// cfg. A source scan cannot parse an attribute soundly (a `]` in a string, a comment or a macro
// that builds it defeats one), so no attribute is parsed: the words are refused in any tracked
// src-tauri Rust line that is not a `//` comment, tests included, since a test file can be pulled
// into shipped code with `#[path]`; only the `.rs` files under src-tauri are read. `dev` is common in prose and paths (`/dev/null`, a
// dev-dependency), so it counts only as a list item (`(dev)`, `, dev,`) or alone on a line; the
// others count anywhere. The cost is loud false alarms (a block comment or a string that says a
// word). The one file that names them in strings is src-tauri/tests/approval_seam_gate.rs, whose
// inputs to another scanner they are: only its lines of exactly that form are skipped.
const DIFFERS = /\bclippy\b(?!\s*::)|\bdebug_assertions\b|\bpanic\s*=|[(,]\s*dev\s*[),]|^\s*dev\s*[,)]*\s*$/;
// The FFI process and network functions in clippy.toml resolve to nothing on the Windows runner
// (measured in CI: libc's posix_spawn, posix_spawnp and getaddrinfo, and windows-sys's ShellExecute*,
// CreateProcess* and WinExec), so a call to one is invisible to the census there. Their bare names
// are refused instead.
const FFI_EGRESS = /\b(?:ShellExecute[AWEx]*|CreateProcess[AW]?|WinExec|posix_spawnp?|getaddrinfo|execv|execvp|execve)\b/;
const NAMES_THEM_IN_STRINGS = "src-tauri/tests/approval_seam_gate.rs";
const STRING_INPUT = /^\s*"#\[cfg\(not\(debug_assertions\)\)\]\\n[^"]*",$/;
for (const path of tracked.filter((name) => name.startsWith("src-tauri/") && name.endsWith(".rs"))) {
  for (const [index, line] of readFileSync(`${root}${path}`, "utf8").split(/\r?\n/).entries()) {
    if (path === NAMES_THEM_IN_STRINGS && STRING_INPUT.test(line)) continue;
    if (/^\s*\/\//.test(line)) continue;
    if (DIFFERS.test(line)) {
      egressViolations.push(`${path}:${index + 1} names clippy, dev, debug_assertions or a panic cfg, which the census run compiles differently from the shipped build`);
    }
    if (FFI_EGRESS.test(line)) {
      egressViolations.push(`${path}:${index + 1} names an FFI process or network function; on Windows the lints cannot see it (its listed path does not resolve there)`);
    }
  }
}
// A build script under another name (`build = "setup.rs"`) escapes the digest pins above.
for (const path of tracked.filter((name) => /^src-tauri\/(?:.*\/)?Cargo\.toml$/.test(name))) {
  if (/(?:^|[\s{,.])["']?build["']?\s*=/m.test(readFileSync(`${root}${path}`, "utf8"))) {
    egressViolations.push(`${path} sets a build script by name; only the default build.rs, pinned by digest above, is allowed`);
  }
}

// ---------------------------------------------------------------------------
// Check 3: no network-capable crate or Tauri plugin beyond the pinned HTTP client.
// ---------------------------------------------------------------------------

// Each would add an egress path that the lints above do not name. Read from the lockfiles, so a
// transitive arrival counts as well; a legitimate need is a reviewed change to this list.
const DENIED_CRATES = [
  "ureq", "isahc", "curl", "surf", "attohttpc", "tungstenite", "tokio-tungstenite", "async-tungstenite",
  "open", "opener", "webbrowser",
  "tauri-plugin-http", "tauri-plugin-opener", "tauri-plugin-shell", "tauri-plugin-websocket", "tauri-plugin-upload",
];
const DENIED_JS_PACKAGES = /@tauri-apps\/plugin-(?:http|opener|shell|websocket|upload)\b/;
for (const lockfile of ["src-tauri/Cargo.lock", "tools/Cargo.lock"]) {
  const names = new Set([...readFileSync(`${root}${lockfile}`, "utf8").matchAll(/^name = "([^"]+)"$/gm)].map((match) => match[1]));
  if (names.size === 0) egressViolations.push(`${lockfile} lists no packages; it was not read`);
  for (const name of DENIED_CRATES) {
    if (names.has(name)) egressViolations.push(`${lockfile} contains ${name}, a network-capable crate outside the pinned HTTP client`);
  }
}
for (const manifest of ["package.json", "pnpm-lock.yaml"]) {
  const denied = readFileSync(`${root}${manifest}`, "utf8").match(DENIED_JS_PACKAGES);
  if (denied) egressViolations.push(`${manifest} contains ${denied[0]}, a Tauri plugin that opens URLs or the network`);
}

if (egressViolations.length) {
  throw new Error(
    "Tally-path egress boundary violated -- this protects the README promise " +
      '"Your Tally data is never uploaded ... nothing in the Tally path sends it to a server of ours" ' +
      "(README.md, 'What it does not do'):\n" +
      egressViolations.map((violation) => `- ${violation}`).join("\n"),
  );
}

console.log(
  "Tally-path egress boundary is sealed: reqwest/hyper are confined to the pinned crates, the clippy " +
    "configuration is unchanged, and no denied crate or plugin is present. The clippy census runs in the native job (--census).",
);
