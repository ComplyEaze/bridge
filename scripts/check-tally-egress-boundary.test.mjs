// SPDX-License-Identifier: Apache-2.0
// Drives the egress gate's cargo-tree check with a stand-in `cargo` on PATH.
// The first row is the control: the same stand-in, given the trees cargo
// prints today, must pass, so each failing row fails on its tree and not on
// the stand-in. The gate reads each tree twice, over every edge kind and over
// normal and build edges only, so a dependency that is dev-only cannot move
// into [dependencies] unnoticed. Check 2 (the source scan) runs against the
// real tree except in the rows that plant a tree of their own.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { delimiter, join } from "node:path";
import { after, before, test } from "node:test";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../", import.meta.url)).slice(0, -1);
const gate = fileURLToPath(new URL("./check-tally-egress-boundary.mjs", import.meta.url));
const skip = process.platform === "win32" && "the stand-in cargo is a POSIX shell script";

// Each call records its workspace, package and edge set, then prints
// `<workspace>.<package>.<kind>.out` from the row's directory, if the row wrote
// one; kind is "all" for normal,build,dev and "shipped" for normal,build. A
// call without `--target all` or with any other edge set fails, so the control
// row also guards those flags.
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
// --target all` on 2026-09-26 over every edge kind (`.all`). The rows over
// normal and build edges only (`.shipped`) are those trees with the app crate's
// line removed, because its reqwest edge is a dev-dependency; real cargo output
// on 2026-09-30 confirmed that shape (the app crate absent from the reqwest
// tree over normal and build edges, present over every edge).
const APP_LINE = `bridge v0.2.0 (${root}/src-tauri)`;
const TRANSPORT_LINE = `bridge-tally-transport v0.1.0 (${root}/src-tauri/crates/bridge-tally-transport)`;
const HYPER = ["hyper v1.11.0", "hyper-rustls v0.27.9", "hyper-util v0.1.20", "reqwest v0.13.5"];
const TODAY = {
  "src-tauri.reqwest.all": ["reqwest v0.13.5", APP_LINE, TRANSPORT_LINE, "tauri v2.11.5"],
  "src-tauri.reqwest.shipped": ["reqwest v0.13.5", TRANSPORT_LINE, "tauri v2.11.5"],
  "src-tauri.hyper.all": HYPER,
  "src-tauri.hyper.shipped": HYPER,
  "tools.reqwest.all": ["reqwest v0.13.4", TRANSPORT_LINE],
  "tools.reqwest.shipped": ["reqwest v0.13.4", TRANSPORT_LINE],
  "tools.hyper.all": ["hyper v1.11.0", "hyper-rustls v0.27.9", "hyper-util v0.1.20", "reqwest v0.13.4"],
  "tools.hyper.shipped": ["hyper v1.11.0", "hyper-rustls v0.27.9", "hyper-util v0.1.20", "reqwest v0.13.4"],
};
const ALL_CALLS = Object.keys(TODAY);

let bin;
before(() => {
  bin = mkdtempSync(join(tmpdir(), "egress-gate-"));
  writeFileSync(join(bin, "cargo"), STAND_IN);
  chmodSync(join(bin, "cargo"), 0o755);
});
after(() => rmSync(bin, { recursive: true, force: true }));

function runGate(trees, env = {}) {
  const dir = mkdtempSync(join(bin, "trees-"));
  for (const [key, lines] of Object.entries(trees)) writeFileSync(join(dir, `${key}.out`), lines.map((l) => `${l}\n`).join(""));
  rmSync(join(bin, "cargo.calls"), { force: true });
  const result = spawnSync(process.execPath, [gate], {
    cwd: root,
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

test("control: the trees cargo prints today pass, and all eight reads are made", { skip }, () => {
  const result = runGate(TODAY);
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /^Tally-path egress boundary is sealed:/);
  assert.deepEqual(result.calls, ALL_CALLS);
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
  assert.deepEqual(result.calls, ALL_CALLS.slice(0, ALL_CALLS.indexOf("tools.hyper.all") + 1));
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

// Check 2 over a tree the row plants, through BRIDGE_EGRESS_APP_SOURCE_ROOT.
function plant(files) {
  const dir = mkdtempSync(join(bin, "src-"));
  for (const [name, text] of Object.entries(files)) {
    mkdirSync(join(dir, name, ".."), { recursive: true });
    writeFileSync(join(dir, name), text);
  }
  return dir;
}

const ALLOWED = {
  "tally/connection.rs": "// a test helper\nfn helper() { let _ = reqwest::Client::builder(); }\n",
  "tally/connection_tests.rs": "// a test double\nfn double() { let _ = reqwest::Url::parse(\"http://127.0.0.1\"); }\n",
};

test("control: a planted tree whose only HTTP mentions are the allow-listed files passes check 2", { skip }, () => {
  const result = runGate(TODAY, { BRIDGE_EGRESS_APP_SOURCE_ROOT: plant(ALLOWED) });
  assert.equal(result.status, 0, result.stderr);
});

test("a planted call site in an unlisted app file is refused", { skip }, () => {
  const result = runGate(TODAY, {
    BRIDGE_EGRESS_APP_SOURCE_ROOT: plant({ ...ALLOWED, "planted.rs": "fn go() { let _ = reqwest::get(\"https://example.invalid\"); }\n" }),
  });
  assertRefused(result, "outside the pinned allow-list");
  assert.ok(result.stderr.includes("src-tauri/src/planted.rs"), result.stderr);
});

test("a raw socket in an unlisted app file is refused", { skip }, () => {
  const result = runGate(TODAY, {
    BRIDGE_EGRESS_APP_SOURCE_ROOT: plant({ ...ALLOWED, "socket.rs": "fn go() { let _ = std::net::TcpStream::connect(\"127.0.0.1:1\"); }\n" }),
  });
  assertRefused(result, "outside the pinned allow-list");
  assert.ok(result.stderr.includes("src-tauri/src/socket.rs"), result.stderr);
});

test("an allow-listed file that no longer names an HTTP client is reported as stale", { skip }, () => {
  const result = runGate(TODAY, {
    BRIDGE_EGRESS_APP_SOURCE_ROOT: plant({ ...ALLOWED, "tally/connection_tests.rs": "// nothing to see\n" }),
  });
  assertRefused(result, "allow-listed file(s) no longer contain an outbound HTTP client");
  assert.ok(result.stderr.includes("src-tauri/src/tally/connection_tests.rs"), result.stderr);
});
