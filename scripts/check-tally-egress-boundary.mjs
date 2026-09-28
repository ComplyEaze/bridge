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
// 2. Clippy lints, kept honest. src-tauri/clippy.toml refuses the egress
//    actions everywhere in the src-tauri workspace: sending an HTTP request,
//    opening or binding a socket, a DNS lookup, spawning a process, building a
//    webview window. A lint fires however the client or socket was obtained,
//    which a text scan cannot see. A reviewed call site carries
//    `#[expect(clippy::disallowed_methods, reason = "...")]`; this gate allows
//    that exemption only in a pinned set of files (each with its exact count,
//    in that one form) and in test-only code. It refuses the ways it knows to
//    turn the lints off: a lint-group or renamed-lint allow, a lint table, an
//    `-A`/`--cap-lints` flag or CLIPPY_CONF_DIR in a workflow or Cargo config,
//    another clippy.toml, or an edit to clippy.toml without updating its digest.
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
//  - The declaration scan reads ordinary `mod` lines and `#[path]`/`include!`
//    forms. A declaration written some other way (an attribute on the same
//    line, a `cfg_attr` path, a macro-built attribute) is not seen.
//  - On Windows, opening a UNC or WebDAV path through std::fs reaches the
//    network; no method list can tell such a path from a local one.
//  - A lint fires only in code a CI clippy step compiles: a cfg branch or
//    feature CI never builds (another OS, lab-writes) is not linted.
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
const LOWER_LEVEL_NETWORK = { h2: [], "hyper-util": [], socket2: [], mio: [], "tower-service": [] };

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
// Check 2: clippy refuses egress anywhere in the src-tauri workspace;
// this keeps the exemptions where they were reviewed.
// ---------------------------------------------------------------------------

// src-tauri/clippy.toml lists the egress actions as `disallowed-methods` (sending an HTTP request,
// opening or binding a socket, a DNS lookup, spawning a process) and `disallowed-types`, and CI's
// clippy runs deny warnings. A reviewed site carries
// `#[expect(clippy::disallowed_methods, reason = "...")]`. A lint fires however the client or
// socket was obtained (an alias, a helper, a returned value), which a text scan could not see.
// What is left to check is where those exemptions may appear, and that nothing turns the lints off.

// Every production file allowed to hold an exemption, with exactly how many `clippy::disallowed_*`
// mentions it has. A new exemption, even in a listed file, changes a count and must be reviewed here.
const EGRESS_EXEMPTIONS = new Map([
  // AXAL sign-in and document upload: the two parts of the app the README names as uploading
  // ('One part of the app does upload'), on purpose and user-initiated.
  ["src-tauri/src/axal.rs", 2],
  ["src-tauri/src/documents.rs", 4],
  // The loopback-only Tally transport: canonical_loopback_origin rejects any other host before
  // a request is built.
  ["src-tauri/crates/bridge-tally-transport/src/lib.rs", 4],
  // Revealing an exported file in the OS file manager, and the native approval dialog helper.
  ["src-tauri/src/commands.rs", 3],
  ["src-tauri/src/tally/approved_import.rs", 2],
  // The synthetic Tally server (a dev-dependency only; it never ships).
  ["src-tauri/crates/tally-protocol-simulator/src/server.rs", 3],
]);

// Any mention of the egress lints, however spaced or line-broken, the lints' old singular names
// included (they still work through `renamed_and_removed_lints`).
const MENTION = /clippy\s*::\s*(?:r#)?disallowed_(?:method|type)s?\b/g;
// The one form a reviewed production site may use: an outer `#[expect(..., reason = ...)]`, which
// covers the statement or item it sits on and fails when that site stops needing it.
const REVIEWED = /#\[\s*expect\s*\(\s*clippy\s*::\s*disallowed_(?:methods|types)\s*,\s*reason\s*=/g;
// Lint groups that would silence the egress lints wholesale: an attribute only (`cfg_attr`
// included), not a method call such as `.expect("warnings")`.
const LINT_ESCAPE =
  /#!?\[[^\]]*\b(?:allow|expect)\s*\([^)\]]*\b(?:warnings|clippy\s*::\s*(?:all|style)|renamed_and_removed_lints)\b/;

function trackedFiles() {
  const result = spawnSync("git", ["-C", root, "ls-files", "-z"], { encoding: "utf8", windowsHide: true });
  if (result.error || result.status !== 0) {
    throw new Error(`git ls-files failed: ${result.error?.message ?? result.stderr}`);
  }
  return result.stdout.split("\0").filter(Boolean);
}

// A file is test-only when Cargo builds it only for tests (under a crate's tests/ directory, and
// declared or included by nothing), or when its one declaration is a module line carrying exactly `#[cfg(test)]`, or
// `#[cfg(all(test, ...))]` with `test` as its first condition. A second declaration, or an
// `include!` of the file anywhere, makes it production.
function isTestOnly(path, rustSources) {
  // Where a plain `mod name;` for this file would be written: name/mod.rs belongs to the directory above.
  let moduleDirectory = posix.dirname(path);
  let moduleName = posix.basename(path, ".rs");
  if (moduleName === "mod") {
    moduleName = posix.basename(moduleDirectory);
    moduleDirectory = posix.dirname(moduleDirectory);
  }
  const declarations = [];
  for (const [candidate, lines] of rustSources) {
    const text = lines.join("\n");
    const candidateDirectory = posix.dirname(candidate);
    const candidateStem = posix.basename(candidate, ".rs");
    const modules = /((?:^[ \t]*#\[[^\n]*\][ \t]*\n)*)^[ \t]*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;/gm;
    for (const [, attributeLines, name] of text.matchAll(modules)) {
      const attributes = attributeLines.split("\n").map((line) => line.trim()).filter(Boolean);
      const pathAttribute = attributes.map((attribute) => attribute.match(/^#\[path\s*=\s*"([^"]+)"\]$/)?.[1]).find(Boolean);
      const declares = pathAttribute
        ? posix.normalize(`${candidateDirectory}/${pathAttribute}`) === path
        : name === moduleName &&
          ((["mod", "lib", "main"].includes(candidateStem) && candidateDirectory === moduleDirectory) ||
            `${candidateDirectory}/${candidateStem}` === moduleDirectory);
      if (declares) declarations.push(attributes);
    }
    for (const [, included] of text.matchAll(/\binclude(?:_str|_bytes)?!\s*\(\s*"([^"]+)"/g)) {
      if (posix.normalize(`${candidateDirectory}/${included}`) === path) declarations.push([]);
    }
  }
  // An integration test is its own crate root: test-only only while nothing else pulls it in.
  if (/^src-tauri\/(?:crates\/[^/]+\/)?tests\//.test(path)) return declarations.length === 0;
  return (
    declarations.length === 1 &&
    declarations[0].some((attribute) => attribute === "#[cfg(test)]" || /^#\[cfg\(all\(test,/.test(attribute))
  );
}

const tracked = trackedFiles();
const rustSources = new Map(
  tracked
    .filter((path) => path.startsWith("src-tauri/") && path.endsWith(".rs"))
    .map((path) => [path, readFileSync(`${root}${path}`, "utf8").split(/\r?\n/)]),
);
for (const [path, lines] of rustSources) {
  const text = lines.join("\n");
  if (LINT_ESCAPE.test(text)) {
    egressViolations.push(`${path} silences a lint group that includes the egress lints (warnings, clippy::all, clippy::style) or re-enables their old names (renamed_and_removed_lints)`);
  }
  const count = text.match(MENTION)?.length ?? 0;
  if (EGRESS_EXEMPTIONS.has(path)) {
    const reviewed = text.match(REVIEWED)?.length ?? 0;
    if (count !== reviewed) {
      egressViolations.push(
        `${path} mentions the egress lints ${count - reviewed} time(s) outside an outer #[expect(..., reason = ...)]; ` +
          "a reviewed file exempts one statement or item at a time, and only that way",
      );
    } else if (reviewed !== EGRESS_EXEMPTIONS.get(path)) {
      egressViolations.push(
        `${path} has ${reviewed} egress-lint exemption(s), not the ${EGRESS_EXEMPTIONS.get(path)} reviewed; ` +
          "review each call site, then update EGRESS_EXEMPTIONS in scripts/check-tally-egress-boundary.mjs",
      );
    }
  } else if (count && !isTestOnly(path, rustSources)) {
    egressViolations.push(
      `${path} exempts itself from the egress lints but is neither a reviewed egress file nor test-only. ` +
        'This falsifies the README promise "nothing in the Tally path sends it to a server of ours" unless ' +
        "the call site is one of the app's documented upload features; if it is, add it to EGRESS_EXEMPTIONS.",
    );
  }
}
for (const path of EGRESS_EXEMPTIONS.keys()) {
  if (!rustSources.has(path)) egressViolations.push(`EGRESS_EXEMPTIONS names ${path}, which is not a tracked file`);
}

// The lint configuration itself. Clippy reads the nearest clippy.toml, so a second one under
// src-tauri would replace these lists for its crate; a lint table, a CI flag or CLIPPY_CONF_DIR could
// switch them off.
const CLIPPY_CONFIG_DIGEST = "9e77707784b1a70a2a69a95137c300afc41012f14ff25aab7f4a680cfa45083e";
const clippyConfig = createHash("sha256").update(readFileSync(`${root}src-tauri/clippy.toml`)).digest("hex");
if (clippyConfig !== CLIPPY_CONFIG_DIGEST) {
  egressViolations.push(`src-tauri/clippy.toml changed; review its egress lists, then set CLIPPY_CONFIG_DIGEST to ${clippyConfig}`);
}
for (const path of tracked) {
  if (/(?:^|\/)\.?clippy\.toml$/.test(path) && path.startsWith("src-tauri/") && path !== "src-tauri/clippy.toml") {
    egressViolations.push(`${path} would replace src-tauri/clippy.toml's egress lints for its crate`);
  }
  const buildInput =
    /^\.github\//.test(path) || /(?:^|\/)\.cargo\/config(?:\.toml)?$/.test(path) || /^src-tauri\/(?:.*\/)?Cargo\.toml$/.test(path);
  if (!buildInput) continue;
  const text = readFileSync(`${root}${path}`, "utf8");
  const buildEscape =
    /disallowed[_-](?:methods?|types?)|CLIPPY_CONF_DIR|renamed_and_removed_lints|--cap-lints|-A\s*(?:warnings|clippy\s*::\s*(?:all|style))\b/;
  if (buildEscape.test(text) || (path.endsWith("Cargo.toml") && /^\s*(?:all|style|warnings)\s*=/m.test(text))) {
    egressViolations.push(`${path} configures the egress lints or a group containing them; only src-tauri/clippy.toml may`);
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
  "Tally-path egress boundary is sealed: reqwest/hyper are confined to the pinned crates, egress-lint " +
    `exemptions to ${EGRESS_EXEMPTIONS.size} reviewed files and test-only code, and no denied crate or plugin is present.`,
);
