// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, existsSync, mkdirSync, symlinkSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

// Runs the real script of the nightly workflow's crate-fetch step under bash with a stand-in `cargo` that plays a
// list of attempt outputs and a stand-in `sleep` that records its argument, and reads what the script decided: how
// many attempts it made, which pauses it took, its exit status and the refusal it left in place of results. The
// consistency check pins the step's text; this proves what the text does. The final case feeds the refusal to the
// real merge of the mutation runner, to show the report names a fetch failure.
const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const workflow = readFileSync(join(repoRoot, ".github", "workflows", "tax-audit-mutations-nightly.yml"), "utf8");
const noStandIns = process.platform === "win32" ? "the stand-in cargo and sleep are extensionless scripts on a ':'-joined PATH; the step runs on ubuntu" : false;
const HEAD = "      - name: Fetch the locked crates, retrying a network failure";
// The host's own coreutils `timeout` (GNU `timeout`, or `gtimeout` from Homebrew's coreutils), for the one test that
// proves the bound cuts a real process off; the other cases use a stand-in that plays exit 124.
const hostTimeout = process.platform === "win32" ? undefined : ["timeout", "gtimeout"].map((name) => spawnSync("sh", ["-c", `command -v ${name}`], { encoding: "utf8" }).stdout.trim()).find((path) => path && spawnSync(path, ["--version"]).status === 0);
const noRealTimeout = noStandIns || (hostTimeout ? false : "no coreutils timeout on this host");

function stepScript() {
  const lines = workflow.split("\n");
  const start = lines.indexOf(HEAD);
  assert.notEqual(start, -1, "the fetch step is missing");
  const next = lines.slice(start + 1).findIndex((line) => /^      (- |#)/.test(line));
  const end = next === -1 ? lines.length : start + 1 + next;
  const run = lines.findIndex((line, index) => index > start && index < end && line === "        run: |");
  assert.notEqual(run, -1, "the fetch step has no script");
  const body = [];
  for (let index = run + 1; index < end; index += 1) {
    const line = lines[index];
    if (line === "") { body.push(""); continue; }
    assert.match(line, /^ {10}/, "unexpected indentation in the fetch step");
    body.push(line.slice(10));
  }
  return body.join("\n").trimEnd();
}

// The shell is started with errexit and pipefail on, which is stricter than the default `bash -e` Actions gives a `run`
// step without a `shell:` key; the step sets its own `set +e` and reads PIPESTATUS, so it does not depend on either.
// attempts: one entry per call of `timeout` (the step's wrapper around cargo): { out, code } runs the stand-in cargo, which
// prints out and exits with code; { hang: true } plays an attempt that outlasts the bound (the stand-in timeout exits 124
// without running cargo); a call beyond the list exits 99. With realTimeout the host's own `timeout` is used, the bound
// is taken from FETCH_TIMEOUT, and an attempt { hang: true } is a stand-in cargo that sleeps far past it.
function run(attempts, { shard = "3", realTimeout = false, bound = "15m" } = {}) {
  const dir = mkdtempSync(join(tmpdir(), "nightly-fetch-"));
  try {
    mkdirSync(join(dir, "bin"));
    const cargoAttempts = realTimeout ? attempts : attempts.filter((a) => !a.hang && !a.kill);
    cargoAttempts.forEach((a, i) => writeFileSync(join(dir, `out-${i}`), a.out ?? ""));
    writeFileSync(join(dir, "plan"), cargoAttempts.map((a) => (a.hang ? "hang" : a.code)).join("\n") + "\n99\n99\n");
    writeFileSync(join(dir, "tplan"), attempts.map((a) => (a.hang ? "hang" : a.kill ? "kill" : "run")).join("\n") + "\nrun\nrun\n");
    writeFileSync(join(dir, "bin", "cargo"), [
      "#!/bin/bash",
      `n=$(cat "${dir}/count" 2>/dev/null || echo 0)`,
      `echo $((n + 1)) > "${dir}/count"`,
      `echo "$*" >> "${dir}/calls"`,
      `if [ -f "${dir}/out-$n" ]; then cat "${dir}/out-$n" >&2; fi`,
      `code=$(sed -n "$((n + 1))p" "${dir}/plan")`,
      `if [ "$code" = hang ]; then exec /bin/sleep 20; fi`,
      `exit $code`,
    ].join("\n") + "\n");
    if (realTimeout) symlinkSync(hostTimeout, join(dir, "bin", "timeout"));
    if (!realTimeout) {
      // A stand-in for coreutils `timeout`: records the bound it was given, then runs the command or plays an attempt
      // that hit the bound (exit 124, as the real one does).
      writeFileSync(join(dir, "bin", "timeout"), [
        "#!/bin/bash",
        `n=$(cat "${dir}/tcount" 2>/dev/null || echo 0)`,
        `echo $((n + 1)) > "${dir}/tcount"`,
        `echo "$*" >> "${dir}/timeouts"`,
        `if [ "$1" != "--kill-after=30s" ]; then exit 98; fi`,
        `shift 2`,
        `if [ "$(sed -n "$((n + 1))p" "${dir}/tplan")" = hang ]; then exit 124; fi`,
        `if [ "$(sed -n "$((n + 1))p" "${dir}/tplan")" = kill ]; then exit 137; fi`,
        `exec "$@"`,
      ].join("\n") + "\n");
      chmodSync(join(dir, "bin", "timeout"), 0o755);
    }
    writeFileSync(join(dir, "bin", "sleep"), `#!/bin/bash\necho "$1" >> "${dir}/pauses"\n`);
    chmodSync(join(dir, "bin", "cargo"), 0o755);
    chmodSync(join(dir, "bin", "sleep"), 0o755);
    const result = spawnSync("bash", ["--noprofile", "--norc", "-eo", "pipefail", "-c", stepScript()], {
      cwd: dir,
      encoding: "utf8",
      env: { ...process.env, PATH: `${join(dir, "bin")}:${process.env.PATH}`, RUNNER_TEMP: dir, SHARD: shard, ...(realTimeout ? { FETCH_TIMEOUT: bound } : {}) },
    });
    const read = (name) => (existsSync(join(dir, name)) ? readFileSync(join(dir, name), "utf8").trim().split("\n").filter(Boolean) : []);
    const refusalPath = join(dir, `shard-${shard}.json`);
    return {
      status: result.status,
      output: result.stdout + result.stderr,
      calls: read("calls"),
      pauses: read("pauses"),
      timeouts: read("timeouts"),
      refusal: existsSync(refusalPath) ? JSON.parse(readFileSync(refusalPath, "utf8")) : null,
      refusalText: existsSync(refusalPath) ? readFileSync(refusalPath, "utf8") : null,
    };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

// Captured from the nightly run of 2 Nov (a shard refused when the build of the unmutated copy could not download
// a crate); only the runner's checkout path is shortened. A fetch fails with the same cargo text for this class.
const NETWORK_FAILURE = [
  "error: failed to get `either` as a dependency of package `sqlx-core v0.9.0`",
  "    ... which satisfies dependency `sqlx-core = \"=0.9.0\"` (locked to 0.9.0) of package `sqlx v0.9.0`",
  "",
  "Caused by:",
  "  failed to load source for dependency `either`",
  "",
  "Caused by:",
  "  unable to update registry `crates-io`",
  "",
  "Caused by:",
  "  download of ei/th/either failed",
  "",
  "Caused by:",
  "  curl failed",
  "",
  "Caused by:",
  "  [55] Failed sending data to the peer (OpenSSL SSL_read: SSL_ERROR_SYSCALL, errno 0)",
  "",
].join("\n");
// Captured from cargo 1.96.0 (the toolchain the nightly pins) run as `cargo fetch --locked` against a local stand-in
// sparse registry that answers with the named status, with only the port (8080) and the project path normalised;
// a real registry differs in the host name only. They replace guessed texts: the HTTP error line cargo prints is
// "failed to get successful HTTP response from `<url>` (<ip>), got <code>".
const CAPTURED = {
  download404: "error: failed to download from `http://127.0.0.1:8080/dl/foo/1.0.0/download`\n\nCaused by:\n  failed to get successful HTTP response from `http://127.0.0.1:8080/dl/foo/1.0.0/download` (127.0.0.1), got 404\n  body:\n  upstream says no\n",
  download410: "error: failed to download from `http://127.0.0.1:8080/dl/foo/1.0.0/download`\n\nCaused by:\n  failed to get successful HTTP response from `http://127.0.0.1:8080/dl/foo/1.0.0/download` (127.0.0.1), got 410\n  body:\n  upstream says no\n",
  download503: "warning: spurious network error (3 tries remaining): failed to get successful HTTP response from `http://127.0.0.1:8080/dl/foo/1.0.0/download` (127.0.0.1), got 503\nbody:\nupstream says no\nwarning: spurious network error (2 tries remaining): failed to get successful HTTP response from `http://127.0.0.1:8080/dl/foo/1.0.0/download` (127.0.0.1), got 503\nbody:\nupstream says no\nwarning: spurious network error (1 try remaining): failed to get successful HTTP response from `http://127.0.0.1:8080/dl/foo/1.0.0/download` (127.0.0.1), got 503\nbody:\nupstream says no\nerror: failed to download from `http://127.0.0.1:8080/dl/foo/1.0.0/download`\n\nCaused by:\n  failed to get successful HTTP response from `http://127.0.0.1:8080/dl/foo/1.0.0/download` (127.0.0.1), got 503\n  body:\n  upstream says no\n",
  download403: "error: failed to download from `http://127.0.0.1:8080/dl/foo/1.0.0/download`\n\nCaused by:\n  failed to get successful HTTP response from `http://127.0.0.1:8080/dl/foo/1.0.0/download` (127.0.0.1), got 403\n  body:\n  upstream says no\n",
  download429: "warning: spurious network error (3 tries remaining): failed to get successful HTTP response from `http://127.0.0.1:8080/dl/foo/1.0.0/download` (127.0.0.1), got 429\nbody:\nupstream says no\nwarning: spurious network error (2 tries remaining): failed to get successful HTTP response from `http://127.0.0.1:8080/dl/foo/1.0.0/download` (127.0.0.1), got 429\nbody:\nupstream says no\nwarning: spurious network error (1 try remaining): failed to get successful HTTP response from `http://127.0.0.1:8080/dl/foo/1.0.0/download` (127.0.0.1), got 429\nbody:\nupstream says no\nerror: failed to download from `http://127.0.0.1:8080/dl/foo/1.0.0/download`\n\nCaused by:\n  failed to get successful HTTP response from `http://127.0.0.1:8080/dl/foo/1.0.0/download` (127.0.0.1), got 429\n  body:\n  upstream says no\n",
  indexMissing: "error: no matching package named `foo` found\nlocation searched: `fake` index\nrequired by package `cap v0.1.0 (/work/cap)`\n",
  index503: "warning: spurious network error (3 tries remaining): failed to get successful HTTP response from `http://127.0.0.1:8080/3/f/foo` (127.0.0.1), got 503\nbody:\nunavailable\nwarning: spurious network error (2 tries remaining): failed to get successful HTTP response from `http://127.0.0.1:8080/3/f/foo` (127.0.0.1), got 503\nbody:\nunavailable\nwarning: spurious network error (1 try remaining): failed to get successful HTTP response from `http://127.0.0.1:8080/3/f/foo` (127.0.0.1), got 503\nbody:\nunavailable\nerror: failed to get `foo` as a dependency of package `cap v0.1.0 (/work/cap)`\n\nCaused by:\n  failed to load source for dependency `foo`\n\nCaused by:\n  unable to update registry `fake`\n\nCaused by:\n  failed to get successful HTTP response from `http://127.0.0.1:8080/3/f/foo` (127.0.0.1), got 503\n  body:\n  unavailable\n",
};
const FAIL_NET = { out: `    Updating crates.io index\n${NETWORK_FAILURE}`, code: 101 };
const FAIL_LOCK = { out: "error: the lock file needs to be updated but --locked was passed to prevent this\n", code: 101 };
const OK = { out: "  Downloaded 3 crates\n", code: 0 };
const COMMAND = "fetch --locked --manifest-path src-tauri/Cargo.toml --target x86_64-unknown-linux-gnu";

test("a fetch that succeeds the first time runs once, never pauses, leaves no refusal and says so", { skip: noStandIns }, () => {
  const r = run([OK]);
  assert.equal(r.status, 0, r.output);
  assert.deepEqual(r.calls, [COMMAND]);
  assert.deepEqual(r.timeouts, [`--kill-after=30s 15m cargo ${COMMAND}`], "each attempt runs under the 15-minute bound");
  assert.deepEqual(r.pauses, []);
  assert.equal(r.refusal, null);
  assert.match(r.output, /dependency-fetch: attempt 1\/3 outcome=ok/);
});

test("a network failure is retried after 30 s, and a later success passes", { skip: noStandIns }, () => {
  const r = run([FAIL_NET, OK]);
  assert.equal(r.status, 0, r.output);
  assert.equal(r.calls.length, 2);
  assert.deepEqual(r.pauses, ["30"]);
  assert.match(r.output, /dependency-fetch: attempt 1\/3 outcome=network-failure, retrying in 30 s/);
  assert.match(r.output, /dependency-fetch: attempt 2\/3 outcome=ok/);
  assert.equal(r.refusal, null);
});

test("two network failures pause 30 s then 60 s, and a third success passes", { skip: noStandIns }, () => {
  const r = run([FAIL_NET, FAIL_NET, OK]);
  assert.equal(r.status, 0, r.output);
  assert.equal(r.calls.length, 3);
  assert.deepEqual(r.pauses, ["30", "60"]);
});

test("three network failures end the step after exactly three attempts and two pauses, with a named error and a typed refusal", { skip: noStandIns }, () => {
  const r = run([FAIL_NET, FAIL_NET, FAIL_NET, OK]);
  assert.notEqual(r.status, 0, r.output);
  assert.equal(r.calls.length, 3, "never a fourth attempt");
  assert.deepEqual(r.pauses, ["30", "60"]);
  assert.match(r.output, /dependency-fetch: attempt 3\/3 outcome=network-failure, no attempts left/);
  assert.match(r.output, /::error::dependency fetch failed after 3 of 3 attempts: no mutation was run on shard 3 of 8/);
  assert.equal(r.refusal.refused.shard, "3/8");
  assert.deepEqual(r.refusal.refused.problems, ["dependency fetch failed after 3 of 3 attempts: no mutation was run"]);
  assert.equal(r.refusal.refused.output[0], "error: failed to get `either` as a dependency of package `sqlx-core v0.9.0`");
  assert.ok(r.refusal.refused.output.some((line) => line.includes("curl failed")));
  assert.ok(r.refusal.refused.output.every((line) => !/^\s*(Updating|Downloading)/.test(line)), "progress lines are left out");
  assert.ok(r.refusal.refused.output.length <= 20);
});

test("a failure that is not a network failure ends the step at once, with no pause and no second attempt", { skip: noStandIns }, () => {
  for (const out of [FAIL_LOCK.out, "error: failed to parse manifest at `src-tauri/Cargo.toml`\n", ""]) {
    const r = run([{ out, code: 101 }, OK]);
    assert.notEqual(r.status, 0, JSON.stringify(out));
    assert.equal(r.calls.length, 1, JSON.stringify(out));
    assert.deepEqual(r.pauses, [], JSON.stringify(out));
    assert.match(r.output, /dependency-fetch: attempt 1\/3 outcome=other-failure/, JSON.stringify(out));
    assert.equal(r.refusal.refused.shard, "3/8", JSON.stringify(out));
    assert.match(r.refusal.refused.problems[0], /^dependency fetch failed after 1 of 3 attempts/, JSON.stringify(out));
  }
});

test("a network line from an earlier attempt is not read as the latest attempt's failure", { skip: noStandIns }, () => {
  const r = run([FAIL_NET, FAIL_LOCK, OK]);
  assert.notEqual(r.status, 0, r.output);
  assert.equal(r.calls.length, 2, "the second attempt's own output decides");
  assert.deepEqual(r.pauses, ["30"]);
  assert.match(r.output, /dependency-fetch: attempt 2\/3 outcome=other-failure/);
});

test("the refusal is read by the mutation runner's merge, and the report names a fetch failure and fails", { skip: noStandIns }, () => {
  const dir = mkdtempSync(join(tmpdir(), "nightly-merge-"));
  try {
    const r = run([FAIL_NET, FAIL_NET, FAIL_NET]);
    const shardFile = join(dir, "shard-3.json");
    writeFileSync(shardFile, r.refusalText);
    const merged = spawnSync(process.execPath, [
      join(repoRoot, "src-tauri", "crates", "bridge-tax-audit", "parity", "mutations.mjs"),
      "--merge", shardFile, "--results", join(dir, "all.json"), "--report", join(dir, "report.md"),
    ], { cwd: repoRoot, encoding: "utf8" });
    assert.equal(merged.status, 1, merged.stdout + merged.stderr);
    const report = readFileSync(join(dir, "report.md"), "utf8");
    assert.match(report, /\n## Shards that refused to run \(1\)\n\n- `3\/8`: dependency fetch failed after 3 of 3 attempts: no mutation was run \(error: failed to get `either` as a dependency of package `sqlx-core v0\.9\.0`\)\n/);
    assert.match(report, /<!-- mutation-nightly-failing: /);
    assert.match(report, /\n## Not run, their shard refused \(\d+\)\n/);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("an attempt that outlasts the bound is cut off and retried, and a later success passes", { skip: noStandIns }, () => {
  const r = run([{ hang: true }, OK]);
  assert.equal(r.status, 0, r.output);
  assert.equal(r.timeouts.length, 2);
  assert.equal(r.calls.length, 1, "the hung attempt never reached cargo's output");
  assert.deepEqual(r.pauses, ["30"]);
  assert.match(r.output, /dependency-fetch: attempt 1\/3 outcome=timeout after 15m, retrying in 30 s/);
  assert.match(r.output, /dependency-fetch: attempt 2\/3 outcome=ok/);
});

test("three attempts that each outlast the bound end the step after three attempts, with a named error and a typed refusal", { skip: noStandIns }, () => {
  const r = run([{ hang: true }, { hang: true }, { hang: true }, OK]);
  assert.notEqual(r.status, 0, r.output);
  assert.equal(r.timeouts.length, 3, "never a fourth attempt");
  assert.deepEqual(r.pauses, ["30", "60"]);
  assert.match(r.output, /dependency-fetch: attempt 3\/3 outcome=timeout after 15m, no attempts left/);
  assert.match(r.output, /::error::dependency fetch failed after 3 of 3 attempts: no mutation was run on shard 3 of 8/);
  assert.deepEqual(r.refusal.refused.problems, ["dependency fetch failed after 3 of 3 attempts: no mutation was run"]);
});

test("a timed-out attempt followed by a non-network failure ends at once", { skip: noStandIns }, () => {
  const r = run([{ hang: true }, FAIL_LOCK, OK]);
  assert.notEqual(r.status, 0, r.output);
  assert.equal(r.timeouts.length, 2);
  assert.match(r.output, /dependency-fetch: attempt 2\/3 outcome=other-failure/);
});

test("a crate the registry reports missing (a 404 or 410 on its download, or no such package in the index) ends the step after one attempt, with no pause, even though the text also says failed to download", { skip: noStandIns }, () => {
  for (const name of ["download404", "download410", "indexMissing"]) {
    const r = run([{ out: CAPTURED[name], code: 101 }, OK]);
    assert.notEqual(r.status, 0, name);
    assert.equal(r.calls.length, 1, name);
    assert.deepEqual(r.pauses, [], name);
    assert.match(r.output, /dependency-fetch: attempt 1\/3 outcome=missing-crate/, name);
    assert.equal(r.refusal.refused.shard, "3/8", name);
    assert.match(r.refusal.refused.problems[0], /^dependency fetch failed after 1 of 3 attempts/, name);
  }
});

test("a 5xx, 403 or 429 answer, on a download or on the index, is still retried: only 404 and 410 are permanent", { skip: noStandIns }, () => {
  for (const name of ["download503", "download403", "download429", "index503"]) {
    const r = run([{ out: CAPTURED[name], code: 101 }, OK]);
    assert.equal(r.status, 0, `${name}\n${r.output}`);
    assert.equal(r.calls.length, 2, name);
    assert.deepEqual(r.pauses, ["30"], name);
    assert.match(r.output, /outcome=network-failure, retrying in 30 s/, name);
  }
});

test("the real coreutils timeout cuts a stalled attempt off at the bound, and the retry passes", { skip: noRealTimeout }, () => {
  const r = run([{ hang: true }, OK], { realTimeout: true, bound: "1" });
  assert.equal(r.status, 0, r.output);
  assert.equal(r.calls.length, 2, "both attempts started cargo; the first never finished");
  assert.deepEqual(r.pauses, ["30"]);
  assert.match(r.output, /dependency-fetch: attempt 1\/3 outcome=timeout after 1, retrying in 30 s/);
  assert.match(r.output, /dependency-fetch: attempt 2\/3 outcome=ok/);
});

test("an attempt that had to be killed after the bound (exit 137) is retried like a timeout", { skip: noStandIns }, () => {
  const r = run([{ kill: true }, OK]);
  assert.equal(r.status, 0, r.output);
  assert.equal(r.timeouts.length, 2);
  assert.deepEqual(r.pauses, ["30"]);
  assert.match(r.output, /dependency-fetch: attempt 1\/3 outcome=timeout after 15m, retrying in 30 s/);
});
