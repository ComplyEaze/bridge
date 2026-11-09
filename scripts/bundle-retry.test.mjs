import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

// Runs the real script of ci.yml's Windows bundle step under bash with a stand-in `corepack` that plays a list of
// attempt outputs, and a stand-in `sleep` that records its argument, and reads what the script decided: how many
// attempts it made, which pauses it took, and its exit status. The consistency check pins the step's text; this proves
// what the text does: only a bundler "http status: 50x" line is retried, three attempts at most, and the compile's
// output of the failed attempt is not hidden.
const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const workflowPath = join(repoRoot, ".github", "workflows", "ci.yml");
// The step runs on the Windows runner under Git Bash, but this test drives it with POSIX stand-ins (an extensionless
// `corepack` and `sleep` script found through PATH, with a `:`-joined PATH); the other cases of that kind skip there.
const noWindowsStandIns = process.platform === "win32" ? "the stand-in corepack and sleep are extensionless scripts on a ':'-joined PATH; the step's logic is the same on every system" : false;
const HEAD = "      - name: Build the Windows bundles, retrying only a bundler download 5xx";

function stepScript() {
  const lines = readFileSync(workflowPath, "utf8").split("\n");
  const start = lines.indexOf(HEAD);
  assert.notEqual(start, -1, "the Windows bundle step is missing");
  const endOffset = lines.slice(start + 1).findIndex((line) => /^      - /.test(line) || /^  [a-zA-Z0-9_-]+:/.test(line));
  const end = endOffset === -1 ? lines.length : start + 1 + endOffset;
  const run = lines.findIndex((line, index) => index > start && index < end && line === "        run: |");
  assert.notEqual(run, -1, "the Windows bundle step has no script");
  const body = [];
  for (let index = run + 1; index < end; index += 1) {
    const line = lines[index];
    if (line === "") { body.push(""); continue; }
    assert.match(line, /^ {10}/, "unexpected indentation in the Windows bundle step");
    body.push(line.slice(10));
  }
  return body.join("\n").trimEnd();
}

// The shell is started the way Actions starts `shell: bash` (bash --noprofile --norc -eo pipefail), so errexit is on.
// attempts: one entry per call of `corepack`: { out, code, stderr }; stderr: true prints the output on stderr only, where tauri prints its errors. A fourth call (more than the script may make) has code 99.
function run(attempts, { failingRm = false } = {}) {
  const dir = mkdtempSync(join(tmpdir(), "bundle-retry-"));
  try {
    mkdirSync(join(dir, "bin"));
    mkdirSync(join(dir, "work", "src-tauri", "target", "release", "bundle"), { recursive: true });
    writeFileSync(join(dir, "work", "src-tauri", "target", "release", "bundle", "half-written.msi"), "x");
    attempts.forEach((a, i) => {
      writeFileSync(join(dir, `out-${i}`), a.out);
      if (a.stderr) writeFileSync(join(dir, `err-${i}`), "");
    });
    writeFileSync(join(dir, "plan"), attempts.map((a) => a.code).join("\n") + "\n99\n99\n");
    writeFileSync(join(dir, "bin", "corepack"), [
      "#!/bin/bash",
      `n=$(cat "${dir}/count" 2>/dev/null || echo 0)`,
      `echo $((n + 1)) > "${dir}/count"`,
      `echo "$*" >> "${dir}/calls"`,
      `if [ -f "${dir}/out-$n" ]; then if [ -f "${dir}/err-$n" ]; then cat "${dir}/out-$n" >&2; else cat "${dir}/out-$n"; fi; fi`,
      `[ -d "${dir}/work/src-tauri/target/release/bundle" ] && echo present >> "${dir}/bundle-at-start" || echo gone >> "${dir}/bundle-at-start"`,
      `exit $(sed -n "$((n + 1))p" "${dir}/plan")`,
    ].join("\n") + "\n");
    writeFileSync(join(dir, "bin", "sleep"), `#!/bin/bash\necho "$1" >> "${dir}/pauses"\n`);
    if (failingRm) writeFileSync(join(dir, "bin", "rm"), "#!/bin/bash\nexit 1\n");
    if (failingRm) chmodSync(join(dir, "bin", "rm"), 0o755);
    chmodSync(join(dir, "bin", "corepack"), 0o755);
    chmodSync(join(dir, "bin", "sleep"), 0o755);
    const result = spawnSync("bash", ["--noprofile", "--norc", "-eo", "pipefail", "-c", stepScript()], {
      cwd: join(dir, "work"),
      encoding: "utf8",
      env: { ...process.env, PATH: `${join(dir, "bin")}:${process.env.PATH}`, RUNNER_TEMP: dir },
    });
    const read = (name) => (existsSync(join(dir, name)) ? readFileSync(join(dir, name), "utf8").trim().split("\n").filter(Boolean) : []);
    return {
      status: result.status,
      output: result.stdout + result.stderr,
      calls: read("calls"),
      pauses: read("pauses"),
      bundleAtStart: read("bundle-at-start"),
    };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

const FAIL_504 = { out: "Downloading https://example.invalid/wix.zip\nfailed to bundle project: `http status: 504`\n", code: 1, stderr: true };
const FAIL_504_STDOUT = { ...FAIL_504, stderr: false };
const OK = { out: "Finished 2 bundles\n", code: 0 };

test("a build that succeeds the first time runs once, never pauses and says so", { skip: noWindowsStandIns }, () => {
  const r = run([OK]);
  assert.equal(r.status, 0, r.output);
  assert.equal(r.calls.length, 1);
  assert.deepEqual(r.pauses, []);
  assert.match(r.output, /bundle-retry: attempt 1\/3 outcome=ok/);
});

test("a download 504 is retried after 30 s, and the half-written bundle folder is gone before the next attempt", { skip: noWindowsStandIns }, () => {
  const r = run([FAIL_504, OK]);
  assert.equal(r.status, 0, r.output);
  assert.equal(r.calls.length, 2);
  assert.deepEqual(r.pauses, ["30"]);
  assert.deepEqual(r.bundleAtStart, ["present", "gone"]);
  assert.match(r.output, /bundle-retry: attempt 1\/3 outcome=download-5xx \(http status: 504\), retrying in 30 s/);
  assert.match(r.output, /bundle-retry: attempt 2\/3 outcome=ok/);
  assert.match(r.output, /failed to bundle project/, "the failed attempt's own output stays in the log");
});

test("two download failures pause 30 s then 90 s, and a third success passes", { skip: noWindowsStandIns }, () => {
  const r = run([FAIL_504, FAIL_504, OK]);
  assert.equal(r.status, 0, r.output);
  assert.equal(r.calls.length, 3);
  assert.deepEqual(r.pauses, ["30", "90"]);
});

test("three download failures fail the step after exactly three attempts and two pauses", { skip: noWindowsStandIns }, () => {
  const r = run([FAIL_504, FAIL_504, FAIL_504, OK]);
  assert.notEqual(r.status, 0, r.output);
  assert.equal(r.calls.length, 3, "never a fourth attempt");
  assert.deepEqual(r.pauses, ["30", "90"]);
  assert.match(r.output, /bundle-retry: attempt 3\/3 outcome=download-5xx \(http status: 504\), no attempts left/);
});

test("502 and 503 count as download errors too", { skip: noWindowsStandIns }, () => {
  for (const code of ["502", "503"]) {
    const r = run([{ out: `failed to bundle project: \`http status: ${code}\`\n`, code: 1 }, OK]);
    assert.equal(r.status, 0, `${code}\n${r.output}`);
    assert.equal(r.calls.length, 2, code);
  }
});

test("any other failure fails at once, with no pause and no second attempt", { skip: noWindowsStandIns }, () => {
  for (const out of [
    "error[E0432]: unresolved import\n",
    "failed to bundle project: `http status: 404`\n",
    "failed to bundle project: `http status: 403`\n",
    "an unrelated line http status: 5\n",
    "failed to bundle project: `http status: 5040`\n",
    "Error failed to bundle project: hash mismatch\n",
    "",
  ]) {
    const r = run([{ out, code: 1 }, OK]);
    assert.notEqual(r.status, 0, JSON.stringify(out));
    assert.equal(r.calls.length, 1, JSON.stringify(out));
    assert.deepEqual(r.pauses, [], JSON.stringify(out));
    assert.match(r.output, /bundle-retry: attempt 1\/3 outcome=other-failure/, JSON.stringify(out));
  }
});

test("a build that fails for another reason after a download retry also fails at once", { skip: noWindowsStandIns }, () => {
  const r = run([FAIL_504, { out: "error: linker failed\n", code: 1 }, OK]);
  assert.notEqual(r.status, 0, r.output);
  assert.equal(r.calls.length, 2);
  assert.deepEqual(r.pauses, ["30"]);
  assert.match(r.output, /bundle-retry: attempt 2\/3 outcome=other-failure/);
});

test("a download line from an earlier attempt is not read as the latest attempt's failure", { skip: noWindowsStandIns }, () => {
  const r = run([FAIL_504, { out: "error: linker failed\n", code: 1 }]);
  assert.notEqual(r.status, 0, r.output);
  assert.equal(r.calls.length, 2, "the second attempt's log is read on its own");
});

test("the command run is the same tauri:build as before", { skip: noWindowsStandIns }, () => {
  const r = run([OK]);
  assert.deepEqual(r.calls, ["pnpm run tauri:build"]);
});

test("a download error printed on stdout is retried the same as one on stderr", { skip: noWindowsStandIns }, () => {
  const r = run([FAIL_504_STDOUT, OK]);
  assert.equal(r.status, 0, r.output);
  assert.equal(r.calls.length, 2);
});

test("a leftover file the folder removal cannot delete does not stop the retry", { skip: noWindowsStandIns }, () => {
  // The stand-in rm fails; errexit must not turn that into a silent exit before the next attempt.
  const r = run([FAIL_504, OK], { failingRm: true });
  assert.equal(r.status, 0, r.output);
  assert.equal(r.calls.length, 2);
});
