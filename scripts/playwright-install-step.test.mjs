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
import { createHash } from "node:crypto";
import { chmodSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
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
if [ "$1" = timeout ]; then STUB_ROOT=1 exec "$@"; fi
if [ "$1" = cp ] || [ "$1" = /usr/bin/find ]; then exec "$@"; fi`);
stub("timeout", 'echo "timeout root=${STUB_ROOT:-0} $*" >> "$STUB_DIR/calls"; while [[ "$1" == --* ]]; do shift; done; shift; exec "$@"');
// A modification time in January 2020 (touch -t YYYYMMDDhhmm), written in parts so that no run of digits appears in this file.
const OLD_STAMP = ["2020", "0101", "0000"].join("");
stub("node", `echo "node root=\${STUB_ROOT:-0} $*" >> "$STUB_DIR/calls"
case "$*" in
  *install-deps*) n=$(grep -c "^node.*install-deps" "$STUB_DIR/calls"); [ "$n" -le "$FAIL_DEPS" ] && exit 1
    # what apt does: the package arrives after the step started, carrying the mirror's old modification time
    if [ -n "$FAKE_DOWNLOAD" ]; then sleep 1.1; for name in $FAKE_DOWNLOAD; do echo "$name" > "$APT_ARCHIVES/$name"; touch -t ${OLD_STAMP} "$APT_ARCHIVES/$name"; done; fi ;;
esac
exit 0`);
stub("corepack", `echo "corepack $*" >> "$STUB_DIR/calls"
case "$*" in
  *install\\ chromium*) n=$(grep -c "^corepack.*install chromium" "$STUB_DIR/calls"); [ "$n" -le "$FAIL_DOWNLOAD" ] && exit 1 ;;
esac
exit 0`);

// The package index: "apt-cache show name:arch=version" prints the SHA256 line the test's index file lists for it,
// "apt-get update" can be made to fail, and a host without sha256sum (macOS) gets one that calls shasum.
stub("apt-cache", `echo "apt-cache $*" >> "$STUB_DIR/calls"
[ "$1" = show ] || exit 1
awk -F '\t' -v key="$2" '$1 == key { print "SHA256: " $2; found = 1 } END { exit found ? 0 : 1 }' "$STUB_DIR/index"`);
stub("apt-get", `echo "apt-get $*" >> "$STUB_DIR/calls"
[ -z "$FAIL_UPDATE" ]`);
if (spawnSync("bash", ["-c", "command -v sha256sum"], { encoding: "utf8" }).status !== 0) stub("sha256sum", 'exec shasum -a 256 "$@"');

// failDeps / failDownload: how many leading calls of that command fail.
const sha256 = (text) => createHash("sha256").update(text).digest("hex");
// cachedDebs: file names put in the restored directory. Each holds "<name>\n"; the index lists the digest of that text for
// every name in `indexed` (defaults to all of them), and a name in `altered` holds other text of the same size.
function runStep({ cacheHit, failDeps = 0, failDownload = 0, failUpdate = false, debsHit = "false", cachedDebs = [], indexed = cachedDebs, altered = [], fakeDownload = "" }) {
  const dir = mkdtempSync(join(tmpdir(), "playwright-step-"));
  try {
    writeFileSync(join(dir, "step.sh"), `${stepScript()}\n`);
    // apt's archive directory holds one package from before the step; $HOME/apt-debs is what the cache restored.
    let linked = false;
    writeFileSync(join(dir, "index"), "");
    mkdirSync(join(dir, "archives"));
    writeFileSync(join(dir, "archives", "before.deb"), "before\n");
    if (cachedDebs.length) {
      mkdirSync(join(dir, "apt-debs"));
      for (const name of cachedDebs) writeFileSync(join(dir, "apt-debs", name), altered.includes(name) ? `${name.slice(0, -1)}!\n` : `${name}\n`);
      // the index line for name_version_arch.deb is "name:arch=version" (an epoch is written %3a in the file name)
      const keyOf = (file) => { const [pkg, version, arch] = file.slice(0, -4).split("_"); return version === undefined ? file : `${pkg}:${arch}=${version.replace("%3a", ":")}`; };
      // the link's target holds text the index lists for the link's own name, so only the step's own check keeps it out
      writeFileSync(join(dir, "index"), [...indexed, "linked_1.0_amd64.deb"].map((name) => `${keyOf(name)}\t${sha256(`${name}\n`)}`).join("\n") + "\n");
      writeFileSync(join(dir, "other-file"), "linked_1.0_amd64.deb\n");
      writeFileSync(join(dir, "apt-debs", "readme.txt"), "not named like a package\n");
      // a link named like a package, and a file not named like one: neither may reach the archive directory
      try { symlinkSync(join(dir, "other-file"), join(dir, "apt-debs", "linked_1.0_amd64.deb")); linked = true; } catch { /* no symlinks on this host */ }
      mkdirSync(join(dir, "apt-debs", "folder.deb"));
    }
    // GitHub runs a step with no `shell:` key as `bash -e {0}`: errexit, no pipefail. Run it the same way.
    const result = spawnSync("bash", ["-e", join(dir, "step.sh")], {
      cwd: dir, encoding: "utf8",
      env: { PATH: `${stubDir}:${process.env.PATH}`, HOME: dir, STUB_DIR: dir, CACHE_HIT: cacheHit, DEBS_HIT: debsHit, APT_ARCHIVES: join(dir, "archives"), FAKE_DOWNLOAD: fakeDownload, FAIL_UPDATE: failUpdate ? "1" : "", FAIL_DEPS: String(failDeps), FAIL_DOWNLOAD: String(failDownload) },
    });
    let calls = [];
    try { calls = readFileSync(join(dir, "calls"), "utf8").trim().split("\n"); } catch { /* none */ }
    let aptConf = null;
    try { aptConf = readFileSync(join(dir, "apt-conf-written"), "utf8"); } catch { /* none */ }
    const names = (sub) => { try { return readdirSync(join(dir, sub)).sort(); } catch { return null; } };
    return { status: result.status, calls, aptConf, archives: names("archives"), kept: names("apt-debs"), linked, stderr: result.stderr };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}
const deps = (calls) => calls.filter((call) => call.startsWith("node") && call.includes("install-deps"));
const rootDeps = "node root=1 node_modules/@playwright/test/cli.js install-deps chromium webkit";
// The step passes `$(command -v node)`, bash's own spelling of the stand-in's path: under Git Bash on Windows that
// is `/tmp/...`, not the `C:\...` that `join` gives (#1471). Elsewhere the two are the same path.
const nodeLookup = spawnSync("bash", ["-c", "command -v node"], { encoding: "utf8", env: { PATH: `${stubDir}:${process.env.PATH}` } });
assert.equal(nodeLookup.status, 0, `bash could not find the node stand-in: ${nodeLookup.stderr}`);
const stubNode = nodeLookup.stdout.trim();
assert.ok(stubNode, "bash printed no path for the node stand-in");
if (process.platform !== "win32") assert.equal(stubNode, join(stubDir, "node"));
const rootTimeout = `timeout root=1 --kill-after=10 600 ${stubNode} node_modules/@playwright/test/cli.js install-deps chromium webkit`;
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

// The cached system packages (bridge#2056): the mirror's throughput, not the install, was what failed 45 steps.
const stepBlock = (name) => {
  const start = workflow.findIndex((line) => line.includes(`- name: ${name}`));
  assert.ok(start !== -1, `the step "${name}" exists`);
  const end = workflow.findIndex((line, index) => index > start && /^      - /.test(line));
  return workflow.slice(start, end === -1 ? workflow.length : end).join("\n");
};

// the restored directory also holds a link, a folder and a text file the step must leave alone
const withoutExtras = (names) => names.filter((name) => !["folder.deb", "linked_1.0_amd64.deb", "readme.txt"].includes(name));
const updateIndex = (calls) => calls.findIndex((call) => call === "timeout root=1 --kill-after=10 120 apt-get update -qq");
const firstCopy = (calls) => calls.findIndex((call) => call.startsWith("sudo cp"));

test("on a package-cache hit each cached .deb whose SHA-256 the updated package index lists is put in apt's archive directory", () => {
  const cachedDebs = ["one_1.0_amd64.deb", "two_2.0_amd64.deb", "epoch_1%3a2.0_all.deb"];
  const { status, calls, archives, kept } = runStep({ cacheHit: "true", debsHit: "true", cachedDebs });
  assert.equal(status, 0);
  assert.deepEqual(archives, ["before.deb", ...cachedDebs].sort());
  assert.deepEqual(withoutExtras(kept), [...cachedDebs].sort(), "the restored directory is not changed");
  assert.ok(calls.includes("apt-cache show one:amd64=1.0") && calls.includes("apt-cache show epoch:all=1:2.0"), "looked up by package, architecture and version, an epoch written as apt writes it");
  assert.ok(updateIndex(calls) !== -1 && updateIndex(calls) < firstCopy(calls), "the index is updated, under a time limit and as root, before anything is copied");
  assert.ok(firstCopy(calls) < calls.findIndex((call) => call.includes("install-deps")), "copied before apt installs");
  assert.equal(deps(calls).length, 1);
});

test("a cached .deb of the right name and size but other content is refused and deleted, and the others are still used", () => {
  const { status, archives, kept } = runStep({ cacheHit: "true", debsHit: "true", cachedDebs: ["one_1.0_amd64.deb", "two_2.0_amd64.deb"], altered: ["one_1.0_amd64.deb"] });
  assert.equal(status, 0);
  assert.deepEqual(archives, ["before.deb", "two_2.0_amd64.deb"], "the changed package does not reach apt's archive directory");
  assert.deepEqual(withoutExtras(kept), ["two_2.0_amd64.deb"], "and is deleted from the restored directory");
});

test("a cached .deb whose version the index does not list (a newer one replaced it on the mirror) is deleted", () => {
  const { archives, kept } = runStep({ cacheHit: "true", debsHit: "true", cachedDebs: ["one_1.0_amd64.deb", "old_0.9_amd64.deb"], indexed: ["one_1.0_amd64.deb"] });
  assert.deepEqual(archives, ["before.deb", "one_1.0_amd64.deb"]);
  assert.deepEqual(withoutExtras(kept), ["one_1.0_amd64.deb"]);
});

test("only files named as apt names its downloads are looked up, and a link or a folder is neither copied nor followed, even when the index lists the name", () => {
  const odd = ["with space_3.0_amd64.deb", "-dash_4.0_amd64.deb", "Upper_5.0_amd64.deb", "noversion.deb", "two_2.0_amd64.deb"];
  const { status, calls, archives, linked } = runStep({ cacheHit: "true", debsHit: "true", cachedDebs: odd });
  assert.equal(status, 0);
  assert.deepEqual(archives, ["before.deb", "two_2.0_amd64.deb"], "a link named like a package whose target the index vouches for, a folder named folder.deb and the odd names stay out");
  assert.equal(calls.filter((call) => call.startsWith("apt-cache")).length, 1, "the odd names never reach apt-cache");
  if (!linked) console.warn("playwright-install-step: this host cannot make a symbolic link, so the link case was not tested");
});

test("if the package index cannot be updated nothing is copied or deleted, and the install goes on and downloads as on a miss", () => {
  const { status, calls, archives, kept } = runStep({ cacheHit: "true", debsHit: "true", cachedDebs: ["one_1.0_amd64.deb"], failUpdate: true });
  assert.equal(status, 0);
  assert.deepEqual(archives, ["before.deb"]);
  assert.deepEqual(withoutExtras(kept), ["one_1.0_amd64.deb"]);
  assert.equal(firstCopy(calls), -1);
  assert.equal(deps(calls).length, 1);
});

test("on a package-cache miss the packages the install downloaded, and only those, are kept for the save step", () => {
  const { status, archives, kept } = runStep({ cacheHit: "true", debsHit: "false", fakeDownload: "new_1.0_amd64.deb new_2.0_amd64.deb" });
  assert.equal(status, 0);
  assert.deepEqual(kept, ["new_1.0_amd64.deb", "new_2.0_amd64.deb"], "found by inode change time although apt sets an old modification time; the package that was there before is not kept");
  assert.deepEqual(archives, ["before.deb", "new_1.0_amd64.deb", "new_2.0_amd64.deb"]);
});

test("a miss that downloaded nothing keeps no directory, so that no empty cache is saved under the key", () => {
  const { status, kept } = runStep({ cacheHit: "true", debsHit: "false" });
  assert.equal(status, 0);
  assert.equal(kept, null);
});

test("a failed system install keeps nothing and fails the step before any package is kept", () => {
  const { status, kept } = runStep({ cacheHit: "true", debsHit: "false", failDeps: 99, fakeDownload: "x_1.0_amd64.deb" });
  assert.equal(status, 1);
  assert.equal(kept, null);
});

test("the package cache is keyed by Playwright version and runner image, and only a push, manual or scheduled run writes it", () => {
  const read = stepBlock("Read the Playwright version");
  assert.match(read, /echo "image=\$\{ImageVersion:-unknown\}" >> "\$GITHUB_OUTPUT"/);
  const restore = stepBlock("Restore Playwright system packages");
  const save = stepBlock("Save Playwright system packages");
  const key = "playwright-debs-${{ runner.os }}-${{ steps.playwright-version.outputs.version }}-${{ steps.playwright-version.outputs.image }}";
  assert.ok(restore.includes(`key: ${key}`) && save.includes(`key: ${key}`), "restore and save use one key, with the version and the image");
  assert.match(restore, /uses: actions\/cache\/restore@[0-9a-f]{40}/);
  assert.match(save, /uses: actions\/cache\/save@[0-9a-f]{40}/);
  assert.ok(restore.includes("path: ~/apt-debs") && save.includes("path: ~/apt-debs"));
  const condition = save.split("\n").find((line) => line.trim().startsWith("if:"));
  assert.ok(condition, "the save step is conditional");
  for (const event of ["push", "workflow_dispatch", "schedule"]) assert.ok(condition.includes(`github.event_name == '${event}'`), event);
  for (const event of ["pull_request", "merge_group"]) assert.ok(!condition.includes(event), `${event} never saves`);
  assert.ok(condition.includes("steps.apt-cache.outputs.cache-hit != 'true'"), "a hit is not saved again");
  const order = workflow.findIndex((line) => line.includes("- name: Restore Playwright system packages"));
  assert.ok(order !== -1 && order < workflow.findIndex((line) => line.includes("- name: Install Playwright browsers and system dependencies")), "restored before the install");
  assert.ok(workflow.findIndex((line) => line.includes("- name: Save Playwright system packages")) > workflow.findIndex((line) => line.includes("- name: Install Playwright browsers and system dependencies")), "saved after the install");
});
