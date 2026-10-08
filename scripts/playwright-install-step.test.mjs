// SPDX-License-Identifier: Apache-2.0
//
// Runs the real "Install Playwright browsers and system dependencies" step of ci.yml under bash with stand-ins for
// sudo, timeout, node and corepack, and records what ran. It proves the properties the step exists for: the apt-based
// system install runs under a `timeout` only when that `timeout` itself runs as root, around a root `node` (a
// `timeout` run as the runner user kills the wrapper, not the root apt-get it started, and that orphan then holds
// the dpkg lock against the next attempt), apt is told to wait for the lock and to keep dpkg in the process group,
// a failed install is retried at most three times, and the browser download keeps its bounded, retried form and
// runs only when the browser cache missed.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";

const workflow = readFileSync(new URL("../.github/workflows/ci.yml", import.meta.url), "utf8").split("\n");
function stepScript() {
  const start = workflow.findIndex((line) => line.includes("- name: Install Playwright browsers and system dependencies"));
  const run = workflow.indexOf("        run: |", start);
  const end = workflow.findIndex((line, index) => index > run && /^      - /.test(line));
  assert.ok(start !== -1 && run !== -1 && end !== -1, "the step is where this test expects it");
  return workflow.slice(run + 1, end).map((line) => line.slice(10)).join("\n");
}

// The stand-ins are written once and read their counts from the environment: a newly written executable can take
// seconds to start on some hosts, and each test would otherwise write them again.
const stubDir = mkdtempSync(join(tmpdir(), "playwright-stubs-"));
process.on("exit", () => rmSync(stubDir, { recursive: true, force: true }));
const stub = (name, body) => { writeFileSync(join(stubDir, name), `#!/bin/bash\n${body}\n`); chmodSync(join(stubDir, name), 0o755); };
// STUB_ROOT marks what sudo started, so a record says whether timeout and node ran as root.
stub("sudo", `echo "sudo $*" >> "$STUB_DIR/calls"
if [ "$1" = tee ]; then cat > "$STUB_DIR/apt-conf-written"; fi
if [ "$1" = timeout ]; then STUB_ROOT=1 exec "$@"; fi`);
stub("timeout", 'echo "timeout root=${STUB_ROOT:-0} $*" >> "$STUB_DIR/calls"; while [[ "$1" == --* ]]; do shift; done; shift; exec "$@"');
stub("node", `echo "node root=\${STUB_ROOT:-0} $*" >> "$STUB_DIR/calls"
case "$*" in
  *install-deps*) n=$(grep -c "^node.*install-deps" "$STUB_DIR/calls"); [ "$n" -le "$FAIL_DEPS" ] && exit 1 ;;
esac
exit 0`);
stub("corepack", `echo "corepack $*" >> "$STUB_DIR/calls"
case "$*" in
  *install\\ chromium*) n=$(grep -c "^corepack.*install chromium" "$STUB_DIR/calls"); [ "$n" -le "$FAIL_DOWNLOAD" ] && exit 1 ;;
esac
exit 0`);

// failDeps / failDownload: how many leading calls of that command fail.
function runStep({ cacheHit, failDeps = 0, failDownload = 0 }) {
  const dir = mkdtempSync(join(tmpdir(), "playwright-step-"));
  try {
    writeFileSync(join(dir, "step.sh"), `${stepScript()}\n`);
    // GitHub runs a step with no `shell:` key as `bash -e {0}`: errexit, no pipefail. Run it the same way.
    const result = spawnSync("bash", ["-e", join(dir, "step.sh")], {
      cwd: dir, encoding: "utf8",
      env: { PATH: `${stubDir}:${process.env.PATH}`, HOME: dir, STUB_DIR: dir, CACHE_HIT: cacheHit, FAIL_DEPS: String(failDeps), FAIL_DOWNLOAD: String(failDownload) },
    });
    let calls = [];
    try { calls = readFileSync(join(dir, "calls"), "utf8").trim().split("\n"); } catch { /* none */ }
    let aptConf = null;
    try { aptConf = readFileSync(join(dir, "apt-conf-written"), "utf8"); } catch { /* none */ }
    return { status: result.status, calls, aptConf };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}
const deps = (calls) => calls.filter((call) => call.startsWith("node") && call.includes("install-deps"));
const rootDeps = "node root=1 node_modules/@playwright/test/cli.js install-deps chromium webkit";
const rootTimeout = `timeout root=1 --kill-after=10 600 ${join(stubDir, "node")} node_modules/@playwright/test/cli.js install-deps chromium webkit`;
const downloads = (calls) => calls.filter((call) => call.startsWith("corepack") && call.includes("install chromium"));

test("on a cache hit only the system dependencies are installed, with apt configured first", () => {
  const { status, calls, aptConf } = runStep({ cacheHit: "true" });
  assert.equal(status, 0);
  assert.deepEqual(deps(calls), [rootDeps], "both browsers' dependencies, exactly");
  assert.equal(calls[0], "sudo tee /etc/apt/apt.conf.d/99-bridge-lock-timeout", "apt is configured before anything installs");
  assert.equal(downloads(calls).length, 0);
  assert.match(aptConf, /^DPkg::Lock::Timeout "\d+";$/m);
  assert.match(aptConf, /^Dpkg::Use-Pty "0";$/m, "without a pty, apt does not start dpkg in its own session, out of timeout's reach");
});

test("on a cache miss the browsers are downloaded as well, under a time limit", () => {
  const { status, calls } = runStep({ cacheHit: "false" });
  assert.equal(status, 0);
  assert.equal(downloads(calls).length, 1);
  assert.ok(calls.some((call) => call === "timeout root=0 --kill-after=10 240 corepack pnpm exec playwright install chromium webkit"), "the download keeps its limit, for both browsers");
  assert.deepEqual(deps(calls), [rootDeps], "the dependencies are installed on a miss too");
  assert.ok(calls.findIndex((call) => call.includes("install-deps")) < calls.findIndex((call) => call.includes("install chromium")), "dependencies before the download");
});

test("the apt-based install runs only under a timeout that itself runs as root, around node run as root", () => {
  for (const cacheHit of ["true", "false"]) {
    const { calls } = runStep({ cacheHit, failDeps: 1 });
    const at = calls.reduce((found, call, index) => (call === rootDeps ? [...found, index] : found), []);
    assert.equal(at.length, 2, cacheHit);
    assert.equal(deps(calls).length, at.length, `every install-deps runs as root (${cacheHit})`);
    for (const index of at) assert.equal(calls[index - 1], rootTimeout, `each one directly under a root timeout of 600 s (${cacheHit})`);
    assert.equal(calls.filter((call) => call.startsWith("timeout") && call.includes("install-deps")).length, at.length, cacheHit);
    assert.equal(calls.filter((call) => call.includes("--with-deps")).length, 0, cacheHit);
    assert.equal(calls.filter((call) => call.startsWith("corepack") && call.includes("install-deps")).length, 0, cacheHit);
  }
});

test("a failed system install is retried, at most three times, finishing an interrupted dpkg between attempts, and then fails the step", () => {
  const twice = runStep({ cacheHit: "true", failDeps: 2 });
  assert.equal(twice.status, 0);
  assert.equal(deps(twice.calls).length, 3);
  assert.equal(twice.calls.filter((call) => call === "sudo dpkg --configure -a").length, 2, "dpkg is finished before each retry, not after the last failure");
  const always = runStep({ cacheHit: "true", failDeps: 99 });
  assert.equal(always.status, 1);
  assert.equal(deps(always.calls).length, 3);
  assert.equal(downloads(always.calls).length, 0, "no download after a failed system install");
});

test("a failed browser download is retried, at most three times, and then fails the step", () => {
  const twice = runStep({ cacheHit: "false", failDownload: 2 });
  assert.equal(twice.status, 0);
  assert.equal(downloads(twice.calls).length, 3);
  const always = runStep({ cacheHit: "false", failDownload: 99 });
  assert.equal(always.status, 1);
  assert.equal(downloads(always.calls).length, 3);
});
