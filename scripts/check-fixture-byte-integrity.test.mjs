// SPDX-License-Identifier: Apache-2.0

import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { chmod, mkdir, mkdtemp, rm, symlink, writeFile } from "node:fs/promises";
import { join, sep } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { FIXTURE_ROOTS, fixtureSignal } from "./fixture-roots.mjs";

const root = fileURLToPath(new URL("../", import.meta.url));

function runGate() {
  return execFileSync(process.execPath, ["scripts/check-fixture-byte-integrity.mjs"], {
    cwd: root,
    encoding: "utf8",
    stdio: "pipe",
  });
}

function gateFailure() {
  try {
    runGate();
  } catch (cause) {
    return `${cause.stderr}${cause.stdout}`;
  }
  throw new Error("expected the byte-integrity gate to fail, but it passed");
}

// Builds `files` in a fresh directory `name` under a scratch directory in the
// repository, runs `check`, and removes it.
async function withDirectory(name, files, check) {
  const scratch = await mkdtemp(join(root, ".fixture-integrity-"));
  try {
    await mkdir(join(scratch, name), { recursive: true });
    for (const [file, text] of Object.entries(files)) {
      await writeFile(join(scratch, name, file), text);
    }
    await check(`${scratch.slice(root.length).replaceAll("\\", "/")}/${name}`);
  } finally {
    await rm(scratch, { force: true, recursive: true });
  }
}

test("an unregistered fixture directory fails the byte-integrity gate", async () => {
  const directory = await mkdtemp(join(root, ".fixture-integrity-"));
  const fixtureDirectory = join(directory, "fixtures");

  try {
    await mkdir(fixtureDirectory, { recursive: true });
    await writeFile(join(fixtureDirectory, "synthetic.xml"), "<fixture />\n");

    let failure = null;
    try {
      execFileSync(process.execPath, ["scripts/check-fixture-byte-integrity.mjs"], {
        cwd: root,
        encoding: "utf8",
        stdio: "pipe",
      });
    } catch (cause) {
      failure = cause;
    }

    assert.ok(failure, "an unregistered fixture directory must fail the gate");
    assert.match(`${failure.stderr}${failure.stdout}`, /unexpected fixture directories/);
    // The name alone finds it, with no note: the floor kept from before #838.
    assert.match(`${failure.stderr}${failure.stdout}`, /fixtures \(named fixture\(s\)\)/);
  } finally {
    await rm(directory, { force: true, recursive: true });
  }
});

test("a git-ignored fixture directory is outside the byte-integrity inventory", async () => {
  const ignoredRoot = join(root, "dist");
  await mkdir(ignoredRoot, { recursive: true });
  const directory = await mkdtemp(join(ignoredRoot, ".fixture-integrity-"));
  const fixtureDirectory = join(directory, "fixtures");

  try {
    await mkdir(fixtureDirectory, { recursive: true });
    await writeFile(join(fixtureDirectory, "synthetic.xml"), "<ignored-fixture />\n");
    assert.doesNotThrow(() => execFileSync(
      process.execPath,
      ["scripts/check-fixture-byte-integrity.mjs"],
      { cwd: root, encoding: "utf8", stdio: "pipe" },
    ));
  } finally {
    await rm(directory, { force: true, recursive: true });
  }
});

// The floor keeps what master's walk found: a `fixtures` directory that is a
// symlinked alias of a directory already walked is recorded and fails, though
// the walk does not descend into it again (#1227 review).
test("a symlinked `fixtures` alias of a walked directory fails the gate", async () => {
  await withDirectory("a", {}, async (directory) => {
    await symlink(
      join(root, "packaging"),
      join(root, directory, "fixtures"),
      process.platform === "win32" ? "junction" : "dir",
    );
    const failure = gateFailure();
    assert.ok(failure.includes(`${directory}/fixtures (named fixture(s))`), failure);
  });
});

// #838: a fixture directory is found by what it holds, whatever it is named.
test("a directory holding a provenance note fails the gate until it is registered", async () => {
  await withDirectory(
    "captures",
    { "run.PROVENANCE.md": "# A capture\n", "run.jsonl": "{}\n" },
    async (directory) => {
      const failure = gateFailure();
      assert.match(failure, /unexpected fixture directories/);
      assert.ok(failure.includes(`${directory} (holds a provenance note)`), failure);
    },
  );
});

test("a directory holding a `source` sidecar beside its file fails the gate", async () => {
  await withDirectory(
    "reads",
    { "read.json": JSON.stringify({ source: "a live read" }), "read.utf16le.xml": "<x/>" },
    async (directory) => {
      const failure = gateFailure();
      assert.ok(failure.includes(`${directory} (holds a provenance record (read.json))`), failure);
    },
  );
});

// "Nothing found" must not stand in for "could not read" (P5): an unreadable
// JSON beside its file fails the walk instead of reading as no record. Skipped
// where `chmod 000` does not stop a read (Windows, or running as root).
test(
  "an unreadable JSON beside its file fails the gate rather than reading as no record",
  { skip: process.platform === "win32" || process.getuid?.() === 0 },
  async () => {
    // Unparseable, so only a read failure can make the gate fail here.
    await withDirectory("reads", { "read.json": "{", "read.utf16le.xml": "<x/>" }, async (directory) => {
      assert.doesNotThrow(runGate);
      await chmod(join(root, directory, "read.json"), 0o000);
      try {
        const failure = gateFailure();
        assert.match(failure, /EACCES/);
        assert.ok(failure.includes(`${directory}/read.json`), failure);
      } finally {
        await chmod(join(root, directory, "read.json"), 0o644);
      }
    });
  },
);

// The shape of packaging/pdfium/pdfium.lock.json: a `source` URL with no file
// sharing its stem is not a provenance record.
test("a `source` JSON with no companion file does not make a fixture directory", async () => {
  await withDirectory(
    "packaging",
    { "tool.lock.json": JSON.stringify({ source: "https://example.invalid/tool" }), "README.md": "x\n" },
    async () => assert.doesNotThrow(runGate),
  );
});

// Measured on master `6136a818`: every tracked file outside the roots with
// `text` unset was an image, so `text` unset is not a signal.
test("an image Git treats as binary does not make a fixture directory", async () => {
  await withDirectory("icons", { "icon.png": "\x89PNG" }, async () => assert.doesNotThrow(runGate));
});

// The roots live only in fixture-roots.mjs: neither gate may carry a copy.
test("neither fixture gate carries its own list of roots", () => {
  for (const gate of ["check-fixture-byte-integrity.mjs", "check-fixture-provenance.mjs"]) {
    const source = readFileSync(join(root, "scripts", gate), "utf8");
    assert.match(source, /import \{[^}]*\bFIXTURE_ROOTS\b[^}]*\} from "\.\/fixture-roots\.mjs";/, gate);
    for (const fixtureRoot of FIXTURE_ROOTS) {
      assert.ok(!source.includes(`"${fixtureRoot}"`), `${gate} names ${fixtureRoot} itself`);
    }
  }
});

// A directory's name is its last component split on the OS separator (#1471): `\` on Windows, where
// `a\fixtures` was once read whole; where the separator is `/`, a backslash stays part of the name.
test("a directory is named by its last path component, on the OS separator only", () => {
  assert.equal(fixtureSignal(join("a", "fixtures"), []), "named fixture(s)");
  if (sep === "/") assert.equal(fixtureSignal("a/b\\fixtures", []), null);
});
