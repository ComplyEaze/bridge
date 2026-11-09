// SPDX-License-Identifier: Apache-2.0
//
// A script that runs its `main` only when it is the program node was started on compares its own
// location with `process.argv[1]`. `import.meta.url` is the real path, `process.argv[1]` is as typed,
// so a comparison that does not resolve links is false when the script is started through a symlink
// to the file or to a directory above it: `main` is skipped, with exit 0 and no output (#1148 item 19).
//
// Each of these scripts has one guard, the same expression in every file. For each, this test:
//   1. shows the probe tells a run from a skipped `main`: loading the script by `import` (which never
//      runs `main`) exits 0 with no output, while the probe through the real path does not;
//   2. starts the script through a symlink to its file, and through a symlink to its directory, and
//      expects the probe's result of step 1.
// It also pins the `existsSync` clause (a program whose `process.argv[1]` is not a file can import a
// script: the import neither throws nor runs `main`), and finds the next script: every direct-run
// guard line in a script of scripts/*.mjs is the one guard, at column 0, with a probe above, or its
// file is named in EXEMPT. A guard line is any line that starts `if (process.argv[1]` (at any
// indentation) or holds both `process.argv[1]` and `import.meta.url` (the comparison written the
// other way round, or kept in a variable).
// No probe starts cargo or reaches the network: each is a usage error or a refused argument.
//
// Where a symlink cannot be made (a Windows account without the privilege) the case is reported as
// skipped, with the reason, and a final test fails if any case was skipped on a platform where
// symlinks must work. A directory link on Windows is a junction, which needs no privilege.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, symlinkSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test, { after } from "node:test";
import { fileURLToPath, pathToFileURL } from "node:url";

const scriptsDirectory = fileURLToPath(new URL(".", import.meta.url));
const root = mkdtempSync(join(tmpdir(), "direct-run-"));
after(() => rmSync(root, { recursive: true, force: true }));

// Not a git work tree, so a probe that asks git finds none even if the temporary directory sits in one.
const workingDirectory = join(root, "cwd");
mkdirSync(workingDirectory);

// The condition every one of these scripts uses, character for character.
const GUARD =
  "if (process.argv[1] && existsSync(process.argv[1]) && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url)))";

// script, arguments and environment of a harmless probe, the exit status it ends with and a pattern
// its output (stdout and stderr) matches when `main` ran.
const PROBES = [
  { script: "capture-package-log.mjs", args: ["--bogus"], status: 2, pattern: /usage: corepack pnpm run tauri:build/ },
  { script: "check-conflict-markers.mjs", status: 2, pattern: /could not check for conflict markers/ },
  { script: "check-pr-milestone.mjs", env: { PR_MILESTONE: "null" }, status: 1, pattern: /A pull request needs a milestone/ },
  { script: "check-pr-type-label.mjs", env: { PR_LABELS: "[]" }, status: 1, pattern: /A pull request needs exactly one of type:/ },
  { script: "check-surface-ack.mjs", args: ["--mode", "bogus"], status: 2, pattern: /--mode must be pull_request/ },
  { script: "check-unbounded-reads.mjs", args: ["--root"], status: 1, pattern: /--root requires a repository path/ },
  { script: "fetch-pdfium.mjs", status: 1, pattern: /usage: fetch-pdfium\.mjs/ },
  { script: "prune-package-compiler-cache.mjs", args: ["--bogus"], status: 1, pattern: /Unexpected argument/ },
  { script: "release-notes.mjs", status: 2, pattern: /usage: release-notes\.mjs/ },
  { script: "render-legal.mjs", status: 2, pattern: /usage: render-legal\.mjs/ },
  { script: "check-mcpb-preview-admission.mjs", status: 1, pattern: /usage: node scripts\/check-mcpb-preview-admission\.mjs/ },
  { script: "check-no-test-seam.mjs", args: ["--expect-present"], status: 1, pattern: /a positive control must name the markers it expects/ },
  { script: "check-registry-server-json.mjs", args: ["--bogus"], status: 1, pattern: /registry\/server\.json refused: usage: unknown argument --bogus/ },
  { script: "next-version.mjs", args: ["--bogus"], status: 1, pattern: /next-version: unknown flag --bogus/ },
  { script: "package-mcpb.mjs", status: 1, pattern: /usage: node scripts\/package-mcpb\.mjs/ },
  { script: "review-tier.mjs", status: 2, pattern: /give --paths-file FILE or --pr N/ },
  { script: "collect-macos-test-crashes.mjs", status: 2, pattern: /--since-file and --output are required/ },
  { script: "retain-macos-test-binaries.mjs", status: 2, pattern: /--crash-json, --target and --output are required/ },
  { script: "mcp-sleep-server.mjs", args: ["--bogus"], status: 2, pattern: /usage: node scripts\/mcp-sleep-server\.mjs/ },
  { script: "check-ai-attribution.mjs", status: 2, pattern: /check-ai-attribution: the input is not the expected JSON object/ },
];

function environment(extra = {}) {
  const env = { ...process.env, GIT_CEILING_DIRECTORIES: root, ...extra };
  for (const name of ["GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "PR_MILESTONE", "PR_LABELS"]) {
    if (!(name in extra)) delete env[name];
  }
  return env;
}

function outcome(result) {
  assert.equal(result.error, undefined, "the probe could not be started");
  return { status: result.status, output: `${result.stdout}${result.stderr}`.replaceAll("\r\n", "\n") };
}

// The program is started on the script, so `process.argv[1]` is `path`.
function run(path, probe) {
  return outcome(
    spawnSync(process.execPath, [path, ...(probe.args ?? [])], {
      cwd: workingDirectory,
      env: environment(probe.env),
      encoding: "utf8",
      timeout: 60_000,
    }),
  );
}

// The script is loaded by `import` from a program that was not started on it: its `main` never runs.
function importOnly(path) {
  return outcome(
    spawnSync(process.execPath, ["--input-type=module", "-e", "await import(process.env.DIRECT_RUN_TARGET)"], {
      cwd: workingDirectory,
      env: environment({ DIRECT_RUN_TARGET: pathToFileURL(path).href }),
      encoding: "utf8",
      timeout: 60_000,
    }),
  );
}

const real = (script) => join(scriptsDirectory, script);
const skipped = [];

test("every probe names a distinct script of this directory", () => {
  assert.equal(new Set(PROBES.map((probe) => probe.script)).size, PROBES.length);
  for (const { script } of PROBES) assert.doesNotThrow(() => readFileSync(real(script)), script);
});

for (const probe of PROBES) {
  test(`${probe.script} has the one guard`, () => {
    const lines = readFileSync(real(probe.script), "utf8").split("\n").filter((line) => line.startsWith("if (process.argv[1]"));
    assert.equal(lines.length, 1, "one direct-run guard");
    assert.ok(lines[0].startsWith(GUARD), lines[0]);
  });
}

for (const probe of PROBES) {
  test(`${probe.script}: the probe tells a run from a skipped main`, () => {
    assert.deepEqual(importOnly(real(probe.script)), { status: 0, output: "" });
    const ran = run(real(probe.script), probe);
    assert.equal(ran.status, probe.status);
    assert.match(ran.output, probe.pattern);
  });
}

function refusal(error) {
  if (!["EPERM", "EACCES", "ENOSYS", "EOPNOTSUPP"].includes(error.code)) throw error;
  return `symlink refused: ${error.code}`;
}

// Makes the link, or reports the case as skipped (visibly, and counted) and returns false.
function link(t, target, path, type) {
  try {
    symlinkSync(target, path, type);
    return true;
  } catch (error) {
    const reason = refusal(error);
    skipped.push(`${t.name}: ${reason}`);
    t.skip(reason);
    return false;
  }
}

for (const probe of PROBES) {
  test(`${probe.script}: runs when started through a symlink to the file`, (t) => {
    const path = join(root, `file-${probe.script}`);
    if (!link(t, real(probe.script), path, "file")) return;
    const ran = run(path, probe);
    assert.equal(ran.status, probe.status);
    assert.match(ran.output, probe.pattern);
  });
}

// One link to the whole directory, made once: the scripts are reached by a path that goes through it.
const directoryLink = join(root, "scripts-link");
let directoryRefusal;
try {
  symlinkSync(scriptsDirectory, directoryLink, process.platform === "win32" ? "junction" : "dir");
} catch (error) {
  directoryRefusal = refusal(error);
}

for (const probe of PROBES) {
  test(`${probe.script}: runs when started through a symlink to its directory`, (t) => {
    if (directoryRefusal) {
      skipped.push(`${t.name}: ${directoryRefusal}`);
      t.skip(directoryRefusal);
      return;
    }
    const ran = run(join(directoryLink, probe.script), probe);
    assert.equal(ran.status, probe.status);
    assert.match(ran.output, probe.pattern);
  });
}

// What `existsSync` is for: with it gone this import throws ENOENT out of `realpathSync`.
test("a script imported by a program whose argv[1] is not a file neither throws nor runs main", () => {
  const script = "check-pr-milestone.mjs";
  assert.ok(PROBES.some((probe) => probe.script === script));
  const missing = join(root, "no-such-file.mjs");
  const result = outcome(
    spawnSync(
      process.execPath,
      ["--input-type=module", "-e", "await import(process.env.DIRECT_RUN_TARGET)", missing],
      {
        cwd: workingDirectory,
        env: environment({ DIRECT_RUN_TARGET: pathToFileURL(real(script)).href }),
        encoding: "utf8",
        timeout: 60_000,
      },
    ),
  );
  assert.deepEqual(result, { status: 0, output: "" });
});

// Scripts that start with `if (process.argv[1]` and are not the one guard, each with why. A new
// script with the old comparison is therefore found here, not copied from one of these.
const EXEMPT = new Map([
  ["master-push-reuse.mjs", "its bytes are pinned by a digest in check-ci-workflow-consistency.mjs, so the maintainers change it"],
  ["audit-tracking-issue.mjs", "it compares by file name (endsWith), a different form that is decided separately"],
]);

const GUARD_START = "if (process.argv[1]";

// Whether a line of a script is, or builds, a direct-run guard.
function isGuardLine(line) {
  return line.trimStart().startsWith(GUARD_START) || (line.includes("process.argv[1]") && line.includes("import.meta.url"));
}

const guardLinesOf = (text) => text.split("\n").filter(isGuardLine);

// File name -> its guard lines, for the scripts (not the tests) of this directory.
function guardLinesByFile() {
  const found = new Map();
  for (const name of readdirSync(scriptsDirectory)) {
    if (!name.endsWith(".mjs") || name.endsWith(".test.mjs")) continue;
    const lines = guardLinesOf(readFileSync(join(scriptsDirectory, name), "utf8"));
    if (lines.length) found.set(name, lines);
  }
  return found;
}

// The problems with `found`, given the scripts that have a probe and the exemptions.
function guardProblems(found, { probed, exempt }) {
  for (const [file, reason] of exempt) {
    if (typeof reason !== "string" || reason.trim() === "") throw new Error(`EXEMPT: ${file} has no reason; every exemption says why`);
  }
  const problems = [];
  for (const [file, lines] of found) {
    if (exempt.has(file)) continue;
    for (const line of lines) {
      if (!line.startsWith(GUARD)) problems.push(`${file}: a guard that is not the one guard: ${line}`);
      else if (!probed.has(file)) problems.push(`${file}: has the guard but no probe in this test`);
    }
  }
  for (const file of exempt.keys()) {
    if (!found.has(file)) problems.push(`${file}: exempt, but it has no direct-run guard line`);
  }
  return problems;
}

test("every direct-run guard in scripts/*.mjs is the one guard, with a probe, or exempt with a reason", () => {
  const probed = new Set(PROBES.map((probe) => probe.script));
  assert.deepEqual(guardProblems(guardLinesByFile(), { probed, exempt: EXEMPT }), []);
});

test("the scan finds a guard that is not the one guard, a guard with no probe, an empty reason and a stale exemption", () => {
  const probed = new Set(["a.mjs"]);
  const old = "if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {";
  assert.deepEqual(guardProblems(new Map([["b.mjs", [old]]]), { probed, exempt: new Map() }), [
    `b.mjs: a guard that is not the one guard: ${old}`,
  ]);
  assert.deepEqual(guardProblems(new Map([["c.mjs", [`${GUARD} {`]]]), { probed, exempt: new Map() }), [
    "c.mjs: has the guard but no probe in this test",
  ]);
  assert.deepEqual(guardProblems(new Map([["a.mjs", [`${GUARD} {`]]]), { probed, exempt: new Map() }), []);
  assert.deepEqual(guardProblems(new Map([["b.mjs", [old]]]), { probed, exempt: new Map([["b.mjs", "kept on purpose"]]) }), []);
  assert.throws(() => guardProblems(new Map(), { probed, exempt: new Map([["b.mjs", " "]]) }), {
    message: "EXEMPT: b.mjs has no reason; every exemption says why",
  });
  assert.deepEqual(guardProblems(new Map(), { probed, exempt: new Map([["b.mjs", "kept on purpose"]]) }), [
    "b.mjs: exempt, but it has no direct-run guard line",
  ]);
  for (const [file, reason] of EXEMPT) assert.ok(reason.trim() !== "", file);
});

test("a guard line is found wherever it sits and however it is written", () => {
  const written = [
    "if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {",
    "    if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {",
    "if (import.meta.url === pathToFileURL(process.argv[1]).href) {",
    "const isMain = import.meta.url === pathToFileURL(process.argv[1]).href;",
    "if (process.argv[1] && process.argv[1].endsWith(\"x.mjs\")) main();",
    "  if (process.argv[1] && process.argv[1].endsWith(\"x.mjs\")) main();",
  ];
  assert.deepEqual(guardLinesOf(`${written.join("\n")}\nconst unrelated = 1;\nconsole.log(process.argv[2]);\n`), written);
  // None but the one guard, at column 0, is accepted.
  const problems = guardProblems(new Map([["a.mjs", written.slice(1, 4)]]), { probed: new Set(["a.mjs"]), exempt: new Map() });
  assert.equal(problems.length, 3);
  assert.deepEqual(
    guardProblems(new Map([["a.mjs", [`  ${GUARD} {`]]]), { probed: new Set(["a.mjs"]), exempt: new Map() }),
    [`a.mjs: a guard that is not the one guard:   ${GUARD} {`],
  );
});

test("no symlink case was skipped where symlinks must work", () => {
  console.log(`direct-run symlink cases skipped: ${skipped.length}${skipped.length ? `\n  ${skipped.join("\n  ")}` : ""}`);
  if (process.platform !== "win32") assert.deepEqual(skipped, []);
});
