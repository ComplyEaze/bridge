#!/usr/bin/env node
// Contract tests for scripts/check-unbounded-reads.mjs.
//
// Each case is a synthetic --root tree with one real git repository (the
// scanner shells out to `git ls-files`, so it needs one) holding a single
// Rust file exercising one branch of the scanner's logic.
import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";
import test from "node:test";
import { tmpdir } from "node:os";
import { fileURLToPath } from "node:url";

const here = fileURLToPath(new URL(".", import.meta.url));
const GATE = join(here, "check-unbounded-reads.mjs");

function git(cwd, ...args) {
  const out = spawnSync("git", args, { cwd, encoding: "utf8" });
  if (out.status !== 0) throw new Error(`git ${args.join(" ")}: ${out.stderr}`);
  return out.stdout;
}

async function makeRepo(rustFileRelativePath, rustSource) {
  const root = await mkdtemp(join(tmpdir(), ".unbounded-reads-"));
  const directory = rustFileRelativePath.split("/").slice(0, -1).join("/");
  await mkdir(join(root, directory), { recursive: true });
  await writeFile(join(root, rustFileRelativePath), rustSource);
  git(root, "init", "-q", ".");
  git(root, "config", "user.email", "test@example.invalid");
  git(root, "config", "user.name", "test");
  git(root, "add", "-A");
  git(root, "commit", "-qm", "seed");
  return root;
}

function runGate(root) {
  return execFileSync("node", [GATE, "--root", root], { encoding: "utf8", stdio: "pipe" });
}

function runGateExpectingFailure(root) {
  try {
    runGate(root);
  } catch (error) {
    // The gate fails by an uncaught throw; on Windows Node prints that with CRLF line ends (#1471).
    return `${error.stdout ?? ""}${error.stderr ?? ""}`.replaceAll("\r\n", "\n");
  }
  throw new Error("expected the gate to fail, but it passed");
}

test("a .take()-guarded read_to_end passes", async () => {
  const root = await makeRepo(
    "src-tauri/src/example.rs",
    `fn read_bounded(mut reader: impl std::io::Read) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();
    reader.take(1024 + 1).read_to_end(&mut output)?;
    Ok(output)
}
`,
  );
  try {
    const output = runGate(root);
    assert.match(output, /0 unbounded/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("an unguarded read_to_end fails the gate", async () => {
  const root = await makeRepo(
    "src-tauri/src/example.rs",
    `fn read_unbounded(mut reader: impl std::io::Read) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();
    reader.read_to_end(&mut output)?;
    Ok(output)
}
`,
  );
  try {
    const output = runGateExpectingFailure(root);
    assert.match(output, /example\.rs:3/);
    assert.match(output, /unbounded Read::read_to_end/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

// quick_xml::Reader::read_to_end(QName) is a same-named, different method —
// "skip to this closing tag", not a byte sink — and must not be flagged.
test("quick_xml's read_to_end(QName) is not a false positive", async () => {
  const root = await makeRepo(
    "src-tauri/src/example.rs",
    `fn skip_element(reader: &mut quick_xml::Reader<&[u8]>, name: &[u8]) -> quick_xml::Result<()> {
    reader.read_to_end(quick_xml::name::QName(name).to_owned())?;
    Ok(())
}
`,
  );
  try {
    const output = runGate(root);
    assert.match(output, /0 read_to_end\/read_to_string call site\(s\) scanned|0 unbounded/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("an unguarded read inside a #[test] function is excluded", async () => {
  const root = await makeRepo(
    "src-tauri/src/example.rs",
    `fn production_code() {}

#[cfg(test)]
mod tests {
    #[test]
    fn reads_a_small_fixture() {
        let mut reader = std::io::Cursor::new(b"fixture bytes".to_vec());
        let mut output = Vec::new();
        std::io::Read::read_to_end(&mut reader, &mut output).expect("read fixture");
    }
}
`,
  );
  try {
    assert.doesNotThrow(() => runGate(root));
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("an unguarded read in a #[cfg(test)] mod without a per-fn #[test] attribute is still excluded", async () => {
  const root = await makeRepo(
    "src-tauri/src/example.rs",
    `#[cfg(test)]
mod tests {
    fn helper_reads_fixture(mut reader: impl std::io::Read) -> Vec<u8> {
        let mut output = Vec::new();
        reader.read_to_end(&mut output).expect("read fixture");
        output
    }
}
`,
  );
  try {
    assert.doesNotThrow(() => runGate(root));
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("scope tracking exits the test module: code after it is still checked", async () => {
  const root = await makeRepo(
    "src-tauri/src/example.rs",
    `#[cfg(test)]
mod tests {
    #[test]
    fn reads_a_small_fixture() {
        let mut reader = std::io::Cursor::new(b"x".to_vec());
        let mut output = Vec::new();
        std::io::Read::read_to_end(&mut reader, &mut output).expect("read fixture");
    }
}

fn production_code_after_tests(mut reader: impl std::io::Read) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();
    reader.read_to_end(&mut output)?;
    Ok(output)
}
`,
  );
  try {
    const output = runGateExpectingFailure(root);
    assert.match(output, /example\.rs:13/);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("a path under a tests/ directory is excluded regardless of content", async () => {
  const root = await mkdtemp(join(tmpdir(), ".unbounded-reads-"));
  try {
    await mkdir(join(root, "src-tauri/tests"), { recursive: true });
    await writeFile(
      join(root, "src-tauri/tests/integration.rs"),
      `fn helper(mut reader: impl std::io::Read) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();
    reader.read_to_end(&mut output)?;
    Ok(output)
}
`,
    );
    git(root, "init", "-q", ".");
    git(root, "config", "user.email", "test@example.invalid");
    git(root, "config", "user.name", "test");
    git(root, "add", "-A");
    git(root, "commit", "-qm", "seed");
    assert.doesNotThrow(() => runGate(root));
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

// #837: every case below reads the gate's whole verdict, never a fragment: the
// exact list of reported sites, or the exact summary of a passing tree.
function reported(output) {
  return output
    .split("\n")
    .filter((line) => line.startsWith("  - "))
    .map((line) => line.slice(4));
}

function summary(scanned) {
  return (
    `Read bound coverage holds: ${scanned} read_to_end/read_to_string call site(s) scanned, ` +
    `${scanned} bounded, 0 unbounded (0 reviewed exception(s)).\n`
  );
}

async function gateOn(source, expectReported) {
  const root = await makeRepo("src-tauri/src/example.rs", source);
  try {
    return expectReported ? reported(runGateExpectingFailure(root)) : runGate(root);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
}

test("the qualified form without a take is reported", async () => {
  const output = await gateOn(
    `fn read(path: &std::path::Path) -> Option<Vec<u8>> {
    let mut file = std::fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    std::io::Read::read_to_end(&mut file, &mut bytes).ok()?;
    Some(bytes)
}

async fn read_async(mut pipe: tokio::process::ChildStdout) -> String {
    let mut text = String::new();
    tokio::io::AsyncReadExt::read_to_string(&mut pipe, &mut text).await.ok();
    text
}
`,
    true,
  );
  assert.deepEqual(output, [
    "src-tauri/src/example.rs:4: std::io::Read::read_to_end(&mut file, &mut bytes).ok()?;",
    "src-tauri/src/example.rs:10: tokio::io::AsyncReadExt::read_to_string(&mut pipe, &mut text).await.ok();",
  ]);
});

test("the qualified form whose reader is a take passes", async () => {
  const output = await gateOn(
    `fn read(file: std::fs::File) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    std::io::Read::read_to_end(
        &mut std::io::Read::take(file, 1024 + 1),
        &mut bytes,
    )
    .ok()?;
    Some(bytes)
}

async fn read_async(mut pipe: tokio::process::ChildStdout) -> Vec<u8> {
    let mut answer = Vec::new();
    tokio::io::AsyncReadExt::read_to_end(
        &mut tokio::io::AsyncReadExt::take(&mut pipe, 128),
        &mut answer,
    )
    .await
    .ok();
    answer
}

fn read_method(file: std::fs::File) -> Option<String> {
    let mut text = String::new();
    std::io::Read::read_to_string(&mut file.take(64), &mut text).ok()?;
    Some(text)
}
`,
    false,
  );
  assert.equal(output, summary(3));
});

test("a call split across lines is read as one call", async () => {
  const output = await gateOn(
    `fn read(mut reader: impl std::io::Read) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();
    reader
        .read_to_end(
            &mut output,
        )?;
    std::io::Read::read_to_end(
        &mut reader,
        &mut output,
    )?;
    Ok(output)
}
`,
    true,
  );
  assert.deepEqual(output, [
    "src-tauri/src/example.rs:4: .read_to_end(",
    "src-tauri/src/example.rs:7: std::io::Read::read_to_end(",
  ]);
});

test("a take in a neighbouring statement, or beside the reader, does not bound it", async () => {
  const output = await gateOn(
    `fn read(reader: impl std::io::Read, other: impl std::io::Read) -> std::io::Result<Vec<u8>> {
    let mut limited = reader.take(1024 + 1);
    let mut output = Vec::new();
    limited.read_to_end(&mut output)?;
    let _ = (other.take(1), std::io::Read::read_to_end(&mut limited, &mut output)?);
    Ok(output)
}
`,
    true,
  );
  assert.deepEqual(output, [
    "src-tauri/src/example.rs:4: limited.read_to_end(&mut output)?;",
    "src-tauri/src/example.rs:5: let _ = (other.take(1), std::io::Read::read_to_end(&mut limited, &mut output)?);",
  ]);
});

test("a take the reader is unwrapped from does not bound it", async () => {
  const output = await gateOn(
    `fn read(file: std::fs::File) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();
    std::io::Read::read_to_end(&mut file.take(64).into_inner(), &mut output)?;
    Ok(output)
}
`,
    true,
  );
  assert.deepEqual(output, [
    "src-tauri/src/example.rs:3: std::io::Read::read_to_end(&mut file.take(64).into_inner(), &mut output)?;",
  ]);
});

test("a call in a comment or a string is not a call", async () => {
  const output = await gateOn(
    `// reader.read_to_end(&mut output) is what this avoids.
/* std::io::Read::read_to_end(&mut file, &mut bytes) */
fn describe() -> &'static str {
    let _quote = '"';
    "std::io::Read::read_to_end(&mut file, &mut bytes)"
}

fn read(reader: impl std::io::Read) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();
    reader.take(8).read_to_end(&mut output)?;
    let _raw = r#"reader.read_to_end(&mut output)"#;
    Ok(output)
}
`,
    false,
  );
  assert.equal(output, summary(1));
});

// #837 slice 1b: the allow-list lives in the scanned tree, and an entry that no
// longer excuses an unbounded read fails the gate.
async function makeTree(files) {
  const root = await mkdtemp(join(tmpdir(), ".unbounded-reads-"));
  for (const [path, text] of Object.entries(files)) {
    await mkdir(join(root, path.split("/").slice(0, -1).join("/")), { recursive: true });
    await writeFile(join(root, path), text);
  }
  git(root, "init", "-q", ".");
  git(root, "config", "user.email", "test@example.invalid");
  git(root, "config", "user.name", "test");
  git(root, "add", "-A");
  git(root, "commit", "-qm", "seed");
  return root;
}

async function gateOnTree(files, expectFailure) {
  const root = await makeTree(files);
  try {
    return expectFailure ? runGateExpectingFailure(root) : runGate(root);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
}

// The gate's one error line, whole.
function errorLine(output) {
  return output.split("\n").find((line) => line.startsWith("Error: "));
}

const ALLOW = (entries) => ({ "scripts/unbounded-reads-allowed.json": JSON.stringify(entries) });
const UNCAPPED = `fn read(mut reader: impl std::io::Read) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();
    reader.read_to_end(&mut output)?;
    Ok(output)
}
`;

test("an allowed file's uncapped read passes, and counts as one reviewed exception", async () => {
  const output = await gateOnTree(
    { ...ALLOW({ "src-tauri/src/example.rs": "reads a bundled template, not outside input" }), "src-tauri/src/example.rs": UNCAPPED },
    false,
  );
  assert.equal(
    output,
    "Read bound coverage holds: 1 read_to_end/read_to_string call site(s) scanned, " +
      "0 bounded, 0 unbounded (1 reviewed exception(s)).\n",
  );
});

test("an allowed file with no unbounded read left is a stale exception", async () => {
  const output = await gateOnTree(
    {
      ...ALLOW({ "src-tauri/src/example.rs": "was a template read", "src-tauri/src/bounded.rs": "was a template read" }),
      "src-tauri/src/example.rs": "fn nothing_read_here() {}\n",
      "src-tauri/src/bounded.rs": `fn read(reader: impl std::io::Read) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();
    reader.take(8).read_to_end(&mut output)?;
    Ok(output)
}
`,
    },
    true,
  );
  // A file with no read, and one whose only read is bounded, both excuse nothing.
  assert.deepEqual(reported(output), ["src-tauri/src/example.rs", "src-tauri/src/bounded.rs"]);
  assert.equal(
    errorLine(output),
    "Error: 2 stale exception(s) in scripts/unbounded-reads-allowed.json: each names a file " +
      "with no unbounded read left to excuse, so a new one there would pass unreported. " +
      "Remove the entry:",
  );
});

test("an allowed path with no file is a stale exception", async () => {
  const output = await gateOnTree(
    { ...ALLOW({ "src-tauri/src/gone.rs": "was a template read" }), "src-tauri/src/example.rs": "fn f() {}\n" },
    true,
  );
  assert.deepEqual(reported(output), ["src-tauri/src/gone.rs"]);
});

test("an exception with no reason is refused", async () => {
  const output = await gateOnTree(
    { ...ALLOW({ "src-tauri/src/example.rs": " " }), "src-tauri/src/example.rs": UNCAPPED },
    true,
  );
  assert.equal(
    errorLine(output),
    "Error: scripts/unbounded-reads-allowed.json: src-tauri/src/example.rs has no reason; " +
      "every exception says why",
  );
});

console.log("\nrunning check-unbounded-reads contract tests via node:test above");
