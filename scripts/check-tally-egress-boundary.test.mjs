// SPDX-License-Identifier: Apache-2.0
// Drives the egress gate in three ways. The cargo-tree check runs with a stand-in `cargo` on PATH
// and the first row is its control: the trees cargo prints today must pass, so each failing row
// fails on its tree and not on the stand-in. The static half of check 2 runs against the real
// tree there, and against a small git repository of its own where a row needs a bad file. The
// census runs on a clippy capture: the output of a real clippy run over a small crate, so what
// is counted is a real message and not one this repository wrote.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { delimiter, dirname, join } from "node:path";
import { after, before, test } from "node:test";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../", import.meta.url)).slice(0, -1);
const gate = fileURLToPath(new URL("./check-tally-egress-boundary.mjs", import.meta.url));
const skip = process.platform === "win32" && "the stand-in cargo is a POSIX shell script";

// Each call records its workspace, package and edge set, then prints
// `<workspace>.<package>.<kind>.out` from the row's directory, if the row wrote
// one; kind is "all" for normal,build,dev and "shipped" for normal,build. A call
// without `--target all` or with any other edge set fails, so the control row
// also guards those flags.
const STAND_IN = `#!/bin/sh
manifest=""; package=""; target=""; edges=""
while [ $# -gt 0 ]; do
  case "$1" in
    --manifest-path) manifest="$2"; shift ;;
    -p) package="$2"; shift ;;
    --target) target="$2"; shift ;;
    --edges) edges="$2"; shift ;;
  esac
  shift
done
case "$edges" in
  normal,build,dev) kind=all ;;
  normal,build) kind=shipped ;;
  *) echo "stand-in cargo: unexpected --edges '$edges'" >&2; exit 3 ;;
esac
echo "\${manifest%%/*}.$package.$kind" >> "$0.calls"
if [ "$target" != "all" ]; then
  echo "stand-in cargo: expected --target all, got '$target'" >&2
  exit 2
fi
[ -f "$TREES/\${manifest%%/*}.$package.$kind.out" ] && cat "$TREES/\${manifest%%/*}.$package.$kind.out"
[ -n "$STAND_IN_STDERR" ] && echo "$STAND_IN_STDERR" >&2
exit "\${STAND_IN_EXIT:-0}"
`;

// As printed by `cargo tree --invert --depth 1 --prefix none --format {p}
// --target all` on 2026-09-26 over every edge kind (`.all`), with the repository
// root substituted. The rows over normal and build edges only (`.shipped`) are
// those trees with the app crate's line removed from the reqwest tree, because
// its reqwest edge is a dev-dependency; real cargo output on 2026-09-30
// confirmed that shape.
const todayTrees = (root) => {
  const EVERY_EDGE = {
    "src-tauri.reqwest": [
      "reqwest v0.13.5",
      `bridge v0.2.0 (${root}/src-tauri)`,
      `bridge-tally-transport v0.1.0 (${root}/src-tauri/crates/bridge-tally-transport)`,
      "tauri v2.11.5",
    ],
    "src-tauri.hyper": ["hyper v1.11.0", "hyper-rustls v0.27.9", "hyper-util v0.1.20", "reqwest v0.13.5"],
    "tools.reqwest": [
      "reqwest v0.13.4",
      `bridge-tally-transport v0.1.0 (${root}/src-tauri/crates/bridge-tally-transport)`,
    ],
    "tools.hyper": ["hyper v1.11.0", "hyper-rustls v0.27.9", "hyper-util v0.1.20", "reqwest v0.13.4"],
  };
  // The lower-level network crates, as printed on 2026-09-28: no first-party dependent in either workspace.
  for (const [workspace, reqwest] of [["src-tauri", "reqwest v0.13.5"], ["tools", "reqwest v0.13.4"]]) {
    EVERY_EDGE[`${workspace}.h2`] = ["h2 v0.4.16", "hyper v1.11.0", reqwest];
    EVERY_EDGE[`${workspace}.hyper-util`] = ["hyper-util v0.1.20", "hyper-rustls v0.27.9", reqwest];
    EVERY_EDGE[`${workspace}.socket2`] = ["socket2 v0.6.5", "hyper-util v0.1.20", "tokio v1.53.1"];
    EVERY_EDGE[`${workspace}.mio`] = ["mio v1.2.2", "tokio v1.53.1"];
    EVERY_EDGE[`${workspace}.tower-service`] = ["tower-service v0.3.3", "hyper-rustls v0.27.9", "hyper-util v0.1.20", reqwest, "tower v0.5.3", "tower-http v0.6.11"];
    EVERY_EDGE[`${workspace}.tower`] = ["tower v0.5.3", reqwest, "tower-http v0.6.11"];
  }
  const TODAY = {};
  for (const [key, lines] of Object.entries(EVERY_EDGE)) {
    TODAY[`${key}.all`] = lines;
    TODAY[`${key}.shipped`] = key === "src-tauri.reqwest" ? lines.filter((line) => !line.startsWith("bridge v")) : lines;
  }
  return TODAY;
};
const TODAY = todayTrees(root);
const APP_LINE = `bridge v0.2.0 (${root}/src-tauri)`;
const TRANSPORT_LINE = `bridge-tally-transport v0.1.0 (${root}/src-tauri/crates/bridge-tally-transport)`;
const EVERY_CALL = ["src-tauri", "tools"].flatMap((workspace) =>
  ["reqwest", "hyper", "h2", "hyper-util", "socket2", "mio", "tower-service", "tower"].flatMap((name) => [
    `${workspace}.${name}.all`,
    `${workspace}.${name}.shipped`,
  ]),
);

let bin;
before(() => {
  bin = mkdtempSync(join(tmpdir(), "egress-gate-"));
  writeFileSync(join(bin, "cargo"), STAND_IN);
  chmodSync(join(bin, "cargo"), 0o755);
});
after(() => rmSync(bin, { recursive: true, force: true }));

function runGate(trees, env = {}, gateFile = gate, cwd = root) {
  const dir = mkdtempSync(join(bin, "trees-"));
  for (const [key, lines] of Object.entries(trees)) writeFileSync(join(dir, `${key}.out`), lines.map((l) => `${l}\n`).join(""));
  rmSync(join(bin, "cargo.calls"), { force: true });
  const result = spawnSync(process.execPath, [gateFile], {
    cwd,
    encoding: "utf8",
    env: { ...process.env, PATH: `${bin}${delimiter}${process.env.PATH}`, TREES: dir, ...env },
  });
  const calls = existsSync(join(bin, "cargo.calls")) ? readFileSync(join(bin, "cargo.calls"), "utf8").trim().split("\n") : [];
  return { ...result, calls };
}

function assertRefused(result, message) {
  assert.notEqual(result.status, 0, `gate passed:\n${result.stdout}`);
  assert.ok(result.stderr.includes(message), `expected ${JSON.stringify(message)} in:\n${result.stderr}`);
  assert.ok(!result.stdout.includes("is sealed"), result.stdout);
}

test("control: the trees cargo prints today pass, and every one is read over both edge sets", { skip }, () => {
  const result = runGate(TODAY);
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /^Tally-path egress boundary is sealed:/);
  assert.deepEqual(result.calls, EVERY_CALL);
});

test("an empty tree from a cargo that exits 0 fails closed", { skip }, () => {
  const result = runGate({});
  assertRefused(result, 'dependency tree for reqwest (src-tauri/Cargo.toml) did not start with reqwest; got "" (0 line(s))');
  assert.deepEqual(result.calls, ["src-tauri.reqwest.all"]);
});

// hyper's pinned set is empty, so the pinned-set comparison cannot tell an
// empty read from a clean one; only the root-line check refuses this.
test("an empty hyper tree fails closed although its pinned set is empty", { skip }, () => {
  const { "tools.hyper.all": _omitted, ...rest } = TODAY;
  const result = runGate(rest);
  assertRefused(result, 'dependency tree for hyper (tools/Cargo.toml) did not start with hyper; got "" (0 line(s))');
  // It stops at the tree it could not read.
  assert.deepEqual(result.calls, EVERY_CALL.slice(0, EVERY_CALL.indexOf("tools.hyper.all") + 1));
});

test("a tree with only its root line reports every pinned crate as lost", { skip }, () => {
  const result = runGate({ ...TODAY, "src-tauri.reqwest.all": ["reqwest v0.13.5"] });
  assertRefused(result, "src-tauri: pinned crate(s) no longer show a direct reqwest dependency: bridge, bridge-tally-transport.");
});

test("cargo failing, even with 'did not match any packages', is not an empty workspace", { skip }, () => {
  const result = runGate(TODAY, {
    STAND_IN_EXIT: "101",
    STAND_IN_STDERR: "error: package ID specification `reqwest` did not match any packages",
  });
  assertRefused(result, "dependency tree for reqwest (src-tauri/Cargo.toml) exited 101: error: package ID specification");
});

test("a new first-party dependent is refused", { skip }, () => {
  const extra = `bridge-tally-protocol v0.1.0 (${root}/src-tauri/crates/bridge-tally-protocol)`;
  const result = runGate({ ...TODAY, "tools.hyper.all": [...TODAY["tools.hyper.all"], extra] });
  assertRefused(result, "tools: crate(s) gained a direct hyper dependency outside the pinned set (none): bridge-tally-protocol");
});

test("a first-party crate depending on a lower-level network crate is refused", { skip }, () => {
  const extra = `bridge v0.2.0 (${root}/src-tauri)`;
  const result = runGate({ ...TODAY, "src-tauri.socket2.all": [...TODAY["src-tauri.socket2.all"], extra] });
  assertRefused(result, "src-tauri: crate(s) gained a direct socket2 dependency outside the pinned set (none): bridge");
});

test("an unparseable line fails instead of being skipped", { skip }, () => {
  const result = runGate({ ...TODAY, "tools.reqwest.all": [...TODAY["tools.reqwest.all"], "warning: something else"] });
  assertRefused(result, 'dependency tree for reqwest (tools/Cargo.toml) printed an unparseable line: "warning: something else"');
});

// The app crate's reqwest edge is dev-only. Moving it into [dependencies] adds
// it to the tree over normal and build edges, which the gate pins separately.
test("reqwest returning to the app crate's normal dependencies is refused", { skip }, () => {
  const result = runGate({ ...TODAY, "src-tauri.reqwest.shipped": [...TODAY["src-tauri.reqwest.shipped"], APP_LINE] });
  assertRefused(
    result,
    "src-tauri: crate(s) gained a direct reqwest dependency on a normal or build edge outside the pinned shipped set (bridge-tally-transport): bridge",
  );
});

test("a hyper dependency on a shipped edge is refused even for the pinned transport", { skip }, () => {
  const result = runGate({ ...TODAY, "src-tauri.hyper.shipped": [...TODAY["src-tauri.hyper.shipped"], TRANSPORT_LINE] });
  assertRefused(
    result,
    "src-tauri: crate(s) gained a direct hyper dependency on a normal or build edge outside the pinned shipped set (none): bridge-tally-transport",
  );
});

test("the transport losing its shipped reqwest edge is reported", { skip }, () => {
  const shipped = TODAY["src-tauri.reqwest.shipped"].filter((line) => !line.startsWith("bridge-tally-transport"));
  const result = runGate({ ...TODAY, "src-tauri.reqwest.shipped": shipped });
  assertRefused(
    result,
    "src-tauri: pinned crate(s) no longer show a direct reqwest dependency on a normal or build edge: bridge-tally-transport.",
  );
});

// ---------------------------------------------------------------------------
// The static half of check 2, run in a small git repository of its own so a row can plant a bad
// file. The gate finds its root from its own location, so it is copied into the repository.
// ---------------------------------------------------------------------------

function sandbox(extra = {}) {
  const dir = realpathSync(mkdtempSync(join(bin, "repo-")));
  const files = {
    "src-tauri/clippy.toml": readFileSync(join(root, "src-tauri/clippy.toml"), "utf8"),
    "src-tauri/build.rs": readFileSync(join(root, "src-tauri/build.rs"), "utf8"),
    "src-tauri/Cargo.lock": '[[package]]\nname = "reqwest"\n',
    "tools/Cargo.lock": '[[package]]\nname = "reqwest"\n',
    "package.json": "{}\n",
    "pnpm-lock.yaml": "lockfileVersion: 9\n",
    "src-tauri/src/lib.rs": '// clippy is only a word in this comment\n#[expect(clippy::disallowed_methods, reason = "reviewed")]\nfn send() {}\n',
    ...extra,
  };
  for (const [path, text] of Object.entries(files)) {
    mkdirSync(dirname(join(dir, path)), { recursive: true });
    writeFileSync(join(dir, path), text);
  }
  mkdirSync(join(dir, "scripts"), { recursive: true });
  copyFileSync(gate, join(dir, "scripts/check-tally-egress-boundary.mjs"));
  const git = spawnSync("git", ["init", "-q"], { cwd: dir });
  assert.equal(git.status, 0, String(git.stderr));
  const add = spawnSync("git", ["add", "-A"], { cwd: dir });
  assert.equal(add.status, 0, String(add.stderr));
  return { dir, run: (env) => runGate(todayTrees(dir), env, join(dir, "scripts/check-tally-egress-boundary.mjs"), dir) };
}

test("static control: a clean repository passes, and a comment naming clippy is not refused", { skip }, () => {
  const result = sandbox().run();
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /^Tally-path egress boundary is sealed:/);
});

test("CLIPPY_CONF_DIR or CLIPPY_ARGS in any tracked file is refused", { skip }, () => {
  for (const name of ["CLIPPY_CONF_DIR", "CLIPPY_ARGS"]) {
    const result = sandbox({ ".github/workflows/x.yml": `env:\n  ${name}: elsewhere\n` }).run();
    assertRefused(result, `.github/workflows/x.yml names CLIPPY_CONF_DIR or CLIPPY_ARGS`);
  }
});

test("a clippy cfg in src-tauri Rust is refused with its line", { skip }, () => {
  const result = sandbox({ "src-tauri/src/lib.rs": "fn ok() {}\n#[cfg(not(clippy))]\nfn hidden() {}\n" }).run();
  assertRefused(result, "src-tauri/src/lib.rs:2 " + DIFFERENT);
});

const DIFFERENT = "names clippy, dev, debug_assertions or a panic cfg, which the census run compiles differently";

test("a line naming a cfg the census run compiles differently is refused, however it is spelled", { skip }, () => {
  const spellings = [
    "#[cfg(not(dev))]",
    "#[cfg_attr(dev, allow(dead_code))]",
    "#[cfg(not(debug_assertions))]",
    "if cfg!(debug_assertions) {",
    '#[cfg(panic = "abort")]',
    // Each of these defeated a scan that parsed the attribute.
    '#[cfg(any(feature = "]", not(debug_assertions)))]',
    "#[cfg(all(/* ] */ not(dev)))]",
    `#[cfg(${" ".repeat(500)}not(dev))]`,
    "macro_rules! m { ($c:meta, $i:item) => { #[cfg($c)] $i }; }\nm!(not(debug_assertions), fn hidden() {});",
  ];
  for (const text of spellings) {
    const result = sandbox({ "src-tauri/src/lib.rs": `fn ok() {}\n${text}\nfn hidden() {}\n` }).run();
    assertRefused(result, `src-tauri/src/lib.rs:`);
    assert.ok(result.stderr.includes(DIFFERENT), `${JSON.stringify(text.slice(0, 40))}: ${result.stderr}`);
  }
  // A cfg split over lines is refused on the line that names the word.
  const split = sandbox({ "src-tauri/src/lib.rs": "fn ok() {}\n#[cfg(not(\n    debug_assertions\n))]\nfn hidden() {}\n" }).run();
  assertRefused(split, "src-tauri/src/lib.rs:3 " + DIFFERENT);
  const splitDev = sandbox({ "src-tauri/src/lib.rs": "fn ok() {}\n#[cfg(not(\n    dev\n))]\nfn hidden() {}\n" }).run();
  assertRefused(splitDev, "src-tauri/src/lib.rs:3 " + DIFFERENT);
  // A tests/ file is scanned too: it can be pulled into shipped code with #[path].
  const helper = sandbox({ "src-tauri/tests/helper.rs": "#[cfg(not(debug_assertions))]\nfn hidden() {}\n" }).run();
  assertRefused(helper, "src-tauri/tests/helper.rs:1 " + DIFFERENT);
  // The one file that holds the words in strings is exempt only on lines of exactly that form, so a
  // real cfg or call put in it (and pulled in with #[path]) is still refused; a comment line is skipped.
  const string = '            "#[cfg(not(debug_assertions))]\\nuse confirm as approve;",\n';
  const exempt = sandbox({
    "src-tauri/tests/approval_seam_gate.rs": string,
    "src-tauri/src/lib.rs": "// #[cfg(not(dev))] is only a comment here\nfn ok() {}\n",
  }).run();
  assert.equal(exempt.status, 0, exempt.stderr);
  const pulledIn = sandbox({ "src-tauri/tests/approval_seam_gate.rs": `${string}#[cfg(not(clippy))]\nfn hidden() {}\n` }).run();
  assertRefused(pulledIn, "src-tauri/tests/approval_seam_gate.rs:2 " + DIFFERENT);
  // Plain cfgs stay legal, and so does prose that says `dev` (a path, a dev-dependency, a script name).
  const legal = sandbox({
    "src-tauri/src/lib.rs":
      '#[cfg(target_os = "macos")]\nfn ok() {}\n#[cfg(feature = "dev-tools")]\nfn also_ok() {}\nconst A: &str = "cat > /dev/null; pnpm run dev";\n' +
      '#[expect(clippy::disallowed_methods, reason = "a dev-dependency only")]\nfn fine() {}\n',
  }).run();
  assert.equal(legal.status, 0, legal.stderr);
});

test("an FFI process or network function name is refused in src-tauri Rust, because its listed path is inert on Windows", { skip }, () => {
  for (const line of ["use windows_sys::Win32::UI::Shell::ShellExecuteW;", "let _ = CreateProcessW(0);", "libc::posix_spawn(a, b);", "unsafe { getaddrinfo(a, b, c, d) };"]) {
    const result = sandbox({ "src-tauri/src/lib.rs": `fn ok() {}\n${line}\n` }).run();
    assertRefused(result, "src-tauri/src/lib.rs:2 names an FFI process or network function");
  }
  const legal = sandbox({ "src-tauri/src/lib.rs": "// ShellExecuteW in a comment\nfn ok() { let _ = SHGetKnownFolderPath; }\n" }).run();
  assert.equal(legal.status, 0, legal.stderr);
});

test("a build script under another name is refused, in any spelling of the key", { skip }, () => {
  for (const toml of ['[package]\nname = "x"\nbuild = "setup.rs"\n', '[package]\n"build" = "setup.rs"\n', 'package.build = "setup.rs"\n', 'package = { name = "x", build = "setup.rs" }\n']) {
    const result = sandbox({ "src-tauri/crates/x/Cargo.toml": toml }).run();
    assertRefused(result, "src-tauri/crates/x/Cargo.toml sets a build script by name");
  }
  // A build-dependency is not a build script.
  const legal = sandbox({ "src-tauri/crates/x/Cargo.toml": '[build-dependencies]\ntauri-build = { version = "2" }\n' }).run();
  assert.equal(legal.status, 0, legal.stderr);
});

test("a differently-cased clippy.toml or cargo config is refused", { skip }, () => {
  assertRefused(sandbox({ "src-tauri/crates/x/Clippy.toml": "disallowed-methods = []\n" }).run(), "src-tauri/crates/x/Clippy.toml would replace");
  assertRefused(sandbox({ ".cargo/Config.toml": "[env]\n" }).run(), ".cargo/Config.toml is a cargo config");
});

test("a tracked cargo config is refused", { skip }, () => {
  assertRefused(sandbox({ ".cargo/config.toml": "[alias]\nclippy = \"true\"\n" }).run(), ".cargo/config.toml is a cargo config");
});

test("a new build script under src-tauri is refused until it is pinned", { skip }, () => {
  const result = sandbox({ "src-tauri/crates/x/build.rs": 'fn main() {\n    println!("cargo:rustc-cfg=shipping");\n}\n' }).run();
  assertRefused(result, "src-tauri/crates/x/build.rs is new or changed; check it sets no cfg or environment for the census run");
});

test("an edit to src-tauri/build.rs without its digest is refused", { skip }, () => {
  const result = sandbox({ "src-tauri/build.rs": 'fn main() {\n    println!("cargo:rustc-cfg=dev");\n    tauri_build::build()\n}\n' }).run();
  assertRefused(result, "src-tauri/build.rs is new or changed; check it sets no cfg or environment for the census run");
});

test("a second clippy.toml under src-tauri is refused", { skip }, () => {
  const result = sandbox({ "src-tauri/crates/x/clippy.toml": "disallowed-methods = []\n" }).run();
  assertRefused(result, "src-tauri/crates/x/clippy.toml would replace src-tauri/clippy.toml's egress lints");
});

test("an edit to src-tauri/clippy.toml without its digest is refused", { skip }, () => {
  const original = readFileSync(join(root, "src-tauri/clippy.toml"), "utf8");
  const result = sandbox({ "src-tauri/clippy.toml": `${original}\n# an edit\n` }).run();
  assertRefused(result, "src-tauri/clippy.toml changed; review its egress lists");
});

// ---------------------------------------------------------------------------
// The census, on a clippy capture. The capture is the JSON output of a real
// `cargo clippy --message-format=json -- --force-warn clippy::disallowed_methods` over a small
// crate (scripts/testdata/egress-census-clippy-capture.PROVENANCE.md): one plain call, one under
// `#[expect]`, one under `#[allow(clippy::all)]` and one made by a macro that another file calls.
// ---------------------------------------------------------------------------

const captureFile = join(root, "scripts/testdata/egress-census-clippy-capture.jsonl");
let captureText;
const captured = () => (captureText ??= readFileSync(captureFile, "utf8"));
// What the capture holds, as the census reports it (paths are relative to the crate).
const CAPTURE_PINS = {
  macOS: [
    ["src-tauri/src/lib.rs", "std::process::Command::new", 2],
    ["src-tauri/src/lib.rs", "std::net::TcpStream::connect", 1],
    ["src-tauri/src/other.rs", "std::process::Command::new", 1],
  ],
};

function census(capture, os = "macOS", pins = CAPTURE_PINS) {
  const dir = mkdtempSync(join(bin, "census-"));
  writeFileSync(join(dir, "capture.jsonl"), capture);
  writeFileSync(join(dir, "pins.json"), JSON.stringify(pins));
  return spawnSync(process.execPath, [gate, "--census", join(dir, "capture.jsonl"), os, join(dir, "pins.json")], {
    cwd: root,
    encoding: "utf8",
  });
}

function assertCensusRefused(result, message) {
  assert.notEqual(result.status, 0, `census passed:\n${result.stdout}`);
  assert.ok(result.stderr.includes(message), `expected ${JSON.stringify(message)} in:\n${result.stderr}`);
  assert.ok(!result.stdout.includes("census matches"), result.stdout);
}

test("census control: the capture matches the reviewed list exactly", () => {
  const result = census(captured());
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /census matches for macOS: 4 reviewed firing\(s\) in 3 place\(s\)/);
});

test("a firing the list does not hold is refused, and the observed list is printed for review", () => {
  const pins = { macOS: CAPTURE_PINS.macOS.slice(0, 2) };
  const result = census(captured(), "macOS", pins);
  assertCensusRefused(result, "src-tauri/src/other.rs: 1 egress-lint firing(s) of std::process::Command::new, 0 reviewed");
  assert.ok(result.stderr.includes('["src-tauri/src/other.rs", "std::process::Command::new", 1]'), result.stderr);
});

test("a reviewed firing that no longer fires is refused", () => {
  const pins = { macOS: [...CAPTURE_PINS.macOS, ["src-tauri/src/gone.rs", "std::net::UdpSocket::bind", 1]] };
  assertCensusRefused(census(captured(), "macOS", pins), "src-tauri/src/gone.rs: 0 egress-lint firing(s) of std::net::UdpSocket::bind, 1 reviewed");
});

test("a changed count is refused in both directions", () => {
  for (const [count, message] of [
    [1, "src-tauri/src/lib.rs: 2 egress-lint firing(s) of std::process::Command::new, 1 reviewed"],
    [3, "src-tauri/src/lib.rs: 2 egress-lint firing(s) of std::process::Command::new, 3 reviewed"],
  ]) {
    const pins = { macOS: [["src-tauri/src/lib.rs", "std::process::Command::new", count], ...CAPTURE_PINS.macOS.slice(1)] };
    assertCensusRefused(census(captured(), "macOS", pins), message);
  }
});

test("a build that did not finish successfully is refused, however the lines look", () => {
  const failed = captured().replace('"success":true', '"success":false');
  assert.notEqual(failed, captured(), "the capture has a build-finished line to change");
  assertCensusRefused(census(failed), "clippy did not report exactly one successful build (build-finished: [false])");
  const withoutFinish = captured().split("\n").filter((line) => !line.includes('"build-finished"')).join("\n");
  assertCensusRefused(census(withoutFinish), "clippy did not report exactly one successful build (build-finished: [])");
});

test("an empty file, a non-JSON line and an unreadable lint message are refused", () => {
  assertCensusRefused(census(""), "the clippy output is empty: the lints did not run");
  assertCensusRefused(census(`${captured()}Compiling something\n`), "is not JSON");
  const unreadable = captured().replaceAll("use of a disallowed method", "use of an unusual method");
  assert.notEqual(unreadable, captured(), "the capture has a firing message to change");
  assertCensusRefused(census(unreadable), "an egress lint message could not be read");
});

// These shapes are derived from the capture by string substitution, not captured on Windows.
test("paths in the shapes cargo prints on Windows, below a crates/ member, or absolute, name the same repository files", () => {
  const withOther = (path) => captured().replaceAll('"file_name":"src/other.rs"', `"file_name":${JSON.stringify(path)}`);
  const other = (file) => ({ macOS: [CAPTURE_PINS.macOS[0], CAPTURE_PINS.macOS[1], [file, "std::process::Command::new", 1]] });
  const cases = [
    ["crates\\x\\src\\other.rs", "src-tauri/crates/x/src/other.rs"],
    ["crates/x/src/other.rs", "src-tauri/crates/x/src/other.rs"],
    [`${root}/src-tauri/src/other.rs`, "src-tauri/src/other.rs"],
  ];
  for (const [printed, file] of cases) {
    assert.notEqual(withOther(printed), captured(), "the capture names src/other.rs");
    const result = census(withOther(printed), "macOS", other(file));
    assert.equal(result.status, 0, `${printed}: ${result.stderr}`);
  }
});

test("a second build-finished line is refused, and a firing without its lint code still counts", () => {
  const finishedLine = captured().split("\n").find((line) => line.includes('"build-finished"'));
  assertCensusRefused(census(`${captured()}${finishedLine}\n`), "(build-finished: [true,true])");
  // The message text counts even when its code is missing or different, so a changed code cannot hide a firing.
  const renamed = captured().replaceAll('"code":{"code":"clippy::disallowed_methods","explanation":null}', '"code":null');
  assert.notEqual(renamed, captured(), "the capture has lint codes to change");
  assert.equal(census(renamed).status, 0);
});

test("a list that could match a silent run is refused: a zero count, a repeated key, a malformed row", () => {
  const silent = captured()
    .split("\n")
    .filter((line) => !line.includes("disallowed"))
    .join("\n");
  assert.ok(silent.includes("build-finished"), "the silent run still finished");
  const zero = { macOS: [["src-tauri/src/x.rs", "std::net::TcpStream::connect", 0]] };
  assertCensusRefused(census(silent, "macOS", zero), "a reviewed entry is not [file, method, count of at least 1]");
  const repeated = { macOS: [["a.rs", "m::f", 1], ["a.rs", "m::f", 2]] };
  assertCensusRefused(census(captured(), "macOS", repeated), "a reviewed entry is repeated: a.rs m::f");
  assertCensusRefused(census(captured(), "macOS", { macOS: [["a.rs", 5, 1]] }), "a reviewed entry is not [file, method, count of at least 1]");
  // The same silent run against the real list is refused too, because nothing fired.
  assertCensusRefused(census(silent), "0 egress-lint firing(s) of std::process::Command::new, 2 reviewed");
});

// A second real capture (scripts/testdata/egress-census-probe-capture.PROVENANCE.md, same file as
// the first): a listed call made by a dependency's own macro, a listed type, a listed path with a
// typo, and a non-member path dependency's own call, which clippy does not lint at all.
const probeFile = join(root, "scripts/testdata/egress-census-probe-capture.jsonl");
const PROBE_PINS = {
  macOS: [
    ["src-tauri/src/lib.rs", "std::process::Command::new", 1],
    ["src-tauri/src/lib.rs", "std::net::TcpListener", 1],
    ["src-tauri/clippy.toml", "std::process::Commnad::new", 1],
  ],
};
// The capture's clippy.toml is an absolute path outside this repository, as cargo prints it. A path
// goes into the capture's JSON escaped, since on Windows `root` holds backslashes (#1471).
const inJson = (text) => JSON.stringify(text).slice(1, -1);
const probeToml = inJson(`${root}/src-tauri/clippy.toml`);
const probe = () => readFileSync(probeFile, "utf8").replaceAll("/work/probe2/main/clippy.toml", probeToml);

test("a dependency's macro, a listed type and a listed path that resolves to nothing are counted", () => {
  const result = census(probe(), "macOS", PROBE_PINS);
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /3 reviewed firing\(s\) in 3 place\(s\)/);
  // The path with a typo is what a review would otherwise never see: nothing fires for it.
  const without = { macOS: PROBE_PINS.macOS.slice(0, 2) };
  assertCensusRefused(census(probe(), "macOS", without), "src-tauri/clippy.toml: 1 listed path(s) that resolve to nothing of std::process::Commnad::new, 0 reviewed");
  // The dependency's own (non-member) call is not reported by clippy, so it is not in the capture.
  assert.ok(!probe().includes("nonmember_call"), "the capture holds no message for the non-member call");
});

test("a verbatim Windows path and a repeated unresolved-path message count as one row", () => {
  // CI printed clippy.toml's path on Windows as //?/D:/a/... (a \\?\ verbatim path); this is that shape on this root.
  const verbatimPath = `\\\\?\\${(root + "/src-tauri/clippy.toml").replaceAll("/", "\\")}`;
  const verbatim = probe().replaceAll(probeToml, inJson(verbatimPath));
  assert.ok(verbatim.includes("?"), "the path was rewritten");
  const result = census(verbatim, "macOS", PROBE_PINS);
  assert.equal(result.status, 0, result.stderr);
  // clippy reports an unresolved path once per crate; the census counts it once however many crates.
  const typo = probe().split("\n").find((line) => line.includes("does not refer to a reachable"));
  const repeated = `${probe()}${typo}\n${typo}\n`;
  assert.notEqual(repeated, probe());
  const again = census(repeated, "macOS", PROBE_PINS);
  assert.equal(again.status, 0, again.stderr);
});

test("a message about a listed path that is not located in clippy.toml is not counted", () => {
  const elsewhere = probe().replaceAll(probeToml, inJson(`${root}/src-tauri/src/lib.rs`));
  assert.notEqual(elsewhere, probe(), "the capture places the message in clippy.toml");
  const result = census(elsewhere, "macOS", { macOS: PROBE_PINS.macOS.slice(0, 2) });
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /2 reviewed firing\(s\) in 2 place\(s\)/);
});

test("an OS with no reviewed list is refused", () => {
  assertCensusRefused(census(captured(), "Linux"), 'no reviewed census for runner OS "Linux"');
  assertCensusRefused(census(captured(), "macOS", { macOS: [] }), 'no reviewed census for runner OS "macOS"');
});

test("the committed lists name real files and cover both native OSes", () => {
  const pins = JSON.parse(readFileSync(join(root, "scripts/tally-egress-census.json"), "utf8"));
  for (const os of ["macOS", "Windows"]) {
    assert.ok(Array.isArray(pins[os]) && pins[os].length > 0, `${os} has reviewed firings`);
    for (const [file, method, count] of pins[os]) {
      assert.ok(existsSync(join(root, file)), `${os}: ${file} is a file`);
      assert.ok(typeof method === "string" && method.includes("::"), `${os}: ${method} is a path`);
      assert.ok(Number.isInteger(count) && count > 0, `${os}: ${file} count`);
    }
  }
});
