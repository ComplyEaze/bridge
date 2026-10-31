#!/usr/bin/env node
// Contract tests for the amount-field-without-type kind in
// scripts/check-tally-request-builder-hazards.mjs. Its pinned set is empty, so
// on the real tree a scanner that stopped matching would still pass; these
// synthetic --root trees are its positive control.
//
// The pinned set is empty (#1872), so a fixture tree with no hazard passes and
// one with a hazard fails listing it as `unexpected:`. Either way the gate's
// own pass or fail line proves the scan completed; the assertions are about
// what is, or is not, listed as `unexpected:`.
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";
import test from "node:test";
import { tmpdir } from "node:os";
import { fileURLToPath } from "node:url";

const here = fileURLToPath(new URL(".", import.meta.url));
const GATE = join(here, "check-tally-request-builder-hazards.mjs");
const FILE = "src-tauri/src/planted.rs";

// One request-builder function per FIELD, each returning one raw string.
function builders(fields) {
  return Object.entries(fields)
    .map(([name, field]) => `pub fn ${name}() -> &'static str {\n    r#"${field}"#\n}\n`)
    .join("\n");
}

// Runs the gate on `root` and returns its `unexpected:` lines, sorted: none
// when it passes, which its summary line must then say.
function gate(root) {
  let output;
  try {
    output = execFileSync("node", [GATE, "--root", root], { encoding: "utf8", stdio: "pipe" });
  } catch (error) {
    // The gate fails by an uncaught throw; on Windows Node prints that with CRLF line ends (#1471).
    output = `${error.stdout ?? ""}${error.stderr ?? ""}`.replaceAll("\r\n", "\n");
    assert.match(output, /allowlist changed:\nunexpected:\n/, "a failing scan must list what it found");
    assert.doesNotMatch(output, /\nmissing:\n/, "the pinned set is empty, so nothing can be missing");
    const unexpected = /\nunexpected:\n([\s\S]*?)$/.exec(output)?.[1] ?? "";
    return [...new Set(unexpected.split("\n").filter((line) => line.startsWith("- ")).map((line) => line.slice(2)))].sort();
  }
  assert.match(output, /match the pinned set \(0 violations; [1-9][0-9]* file\(s\) found,/, "a passing scan must say it completed");
  return [];
}

async function scan(fields) {
  const root = await mkdtemp(join(tmpdir(), ".request-builder-hazards-"));
  try {
    await mkdir(join(root, "src-tauri/src"), { recursive: true });
    await writeFile(join(root, FILE), builders(fields));
    return new Set(gate(root));
  } finally {
    await rm(root, { recursive: true, force: true });
  }
}

const violation = (builder, field) => `amount-field-without-type|${FILE}::${builder}|${field}`;

// Each shape V's review of #753 found passing untyped, plus the plain case.
const untyped = {
  plain_method: `<FIELD NAME="Plain"><SET>$ClosingBalance</SET></FIELD>`,
  sub_object_path: `<FIELD NAME="Sub Object"><SET>$LedgerEntries[1].Amount</SET></FIELD>`,
  formula_reference: `<FIELD NAME="Formula"><SET>@@BridgeClosing</SET></FIELD>`,
  unlisted_suffix: `<FIELD NAME="Credit Limit"><SET>$CreditLimit</SET></FIELD>`,
  second_set: `<FIELD NAME="Second Set"><SET>$Name</SET><SET>$ClosingBalance</SET></FIELD>`,
  nested_index: `<FIELD NAME="Nested Index"><SET>$LedgerEntries[$$Number:$Index[1]].Amount</SET></FIELD>`,
  type_text_in_set: `<FIELD NAME="Type In Set"><SET>"<TYPE>Amount</TYPE>" + $ClosingBalance</SET></FIELD>`,
  type_in_comment: `<FIELD NAME="Type In Comment"><!-- <TYPE>Amount</TYPE> --><SET>$ClosingBalance</SET></FIELD>`,
};

test("every untyped amount shape is reported, each by its FIELD", async () => {
  const actual = await scan(untyped);
  assert.deepEqual(
    [...actual].sort(),
    [
      violation("formula_reference", "Formula"),
      violation("nested_index", "Nested Index"),
      violation("plain_method", "Plain"),
      violation("second_set", "Second Set"),
      violation("sub_object_path", "Sub Object"),
      violation("type_in_comment", "Type In Comment"),
      violation("type_text_in_set", "Type In Set"),
      violation("unlisted_suffix", "Credit Limit"),
    ],
  );
});

test("the same shapes pass once each FIELD declares TYPE Amount", async () => {
  const typed = Object.fromEntries(
    Object.entries(untyped).map(([name, field]) => [
      name,
      field.replace("</FIELD>", "<TYPE>Amount</TYPE></FIELD>"),
    ]),
  );
  assert.deepEqual([...(await scan(typed))], []);
});

test("a formula reference passes with any declared TYPE; an amount needs TYPE Amount", async () => {
  const actual = await scan({
    formula_typed_as_text: `<FIELD NAME="Formula Text"><SET>@@BridgeName</SET><TYPE>String</TYPE></FIELD>`,
    amount_typed_as_text: `<FIELD NAME="Amount Text"><SET>$ClosingBalance</SET><TYPE>String</TYPE></FIELD>`,
    name_untyped: `<FIELD NAME="Name"><SET>$Name</SET></FIELD>`,
  });
  assert.deepEqual([...actual], [violation("amount_typed_as_text", "Amount Text")]);
});

// #837 slice 2: a tree of several files, so that what reaches a file can be
// tested. Returns the gate's `unexpected:` set, like `scan`.
async function scanTree(files) {
  const root = await mkdtemp(join(tmpdir(), ".request-builder-hazards-"));
  try {
    for (const [path, text] of Object.entries(files)) {
      await mkdir(join(root, path.split("/").slice(0, -1).join("/")), { recursive: true });
      await writeFile(join(root, path), text);
    }
    return gate(root);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
}

// With nothing pinned, a scan that found no files would otherwise pass (P5).
test("a tree with no Rust files is refused, not passed", async () => {
  const root = await mkdtemp(join(tmpdir(), ".request-builder-hazards-"));
  try {
    await mkdir(join(root, "src-tauri/src"), { recursive: true });
    assert.throws(
      () => execFileSync("node", [GATE, "--root", root], { encoding: "utf8", stdio: "pipe" }),
      (error) => /found no Rust files under src-tauri/.test(`${error.stdout ?? ""}${error.stderr ?? ""}`),
    );
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

// #1872 deleted the two pinned custom company reports, so the real tree no
// longer exercises this kind; this tree is its positive control. A bare
// #[cfg(test)] fn is still scanned, as the deleted test-only twin was.
test("a fixed custom report is reported by its name, in a bare test-only fn too", async () => {
  const actual = await scanTree({
    "src-tauri/src/lib.rs": `pub fn report() -> &'static str {
    r#"<REPORT NAME="Planted Report"><FORMS>Planted Form</FORMS></REPORT>"#
}
#[cfg(test)]
fn test_only_report() -> &'static str {
    r#"<REPORT  NAME="Planted Twin">"#
}
`,
  });
  assert.deepEqual(actual, [
    "custom-report|src-tauri/src/lib.rs::report|Planted Report",
    "custom-report|src-tauri/src/lib.rs::test_only_report|Planted Twin",
  ]);
});

const HAZARD = `pub fn planted() -> &'static str {\n    r#"<SET>$$NumItems:BRIDGE Planted Collection</SET>"#\n}\n`;
const planted = (file) => `function-argument-with-space|${file}::planted|$$NumItems:BRIDGE Planted Collection`;

test("a placeholder in an unquoted argument or a report name is reported as unresolved", async () => {
  const actual = await scanTree({
    "src-tauri/src/lib.rs": `pub fn unquoted(collection: &str) -> String {
    format!(r#"<SET>$$NumItems:{collection}</SET>"#)
}
pub fn positional(collection: &str) -> String {
    format!(r#"<SET>$$NumItems:{}</SET>"#, collection)
}
pub fn report(name: &str) -> String {
    format!(r#"<REPORT NAME="{name}">"#)
}
pub fn quoted(from: &str) -> String {
    format!(r#"<SET>$$Date:"{from}"</SET>"#)
}
pub fn escaped() -> String {
    format!(r#"<SET>$$NumItems:{{Fixed}}</SET>"#)
}
`,
  });
  assert.deepEqual(actual, [
    'unresolved-argument|src-tauri/src/lib.rs::positional|$$NumItems:{}',
    'unresolved-argument|src-tauri/src/lib.rs::report|<REPORT NAME="{name}">',
    "unresolved-argument|src-tauri/src/lib.rs::unquoted|$$NumItems:{collection}",
  ]);
});

test("a file reached only through a test-only mod declaration is quarantined, and what it declares too", async () => {
  const actual = await scanTree({
    "src-tauri/src/lib.rs": `#[cfg(test)]
#[path = "planted_tests.rs"]
mod planted_tests;
#[cfg(all(test, target_os = "macos"))]
mod mac_only;
#[cfg(any(test, feature = "seam"))]
mod seam;
mod production;
#[cfg(test)]
mod inline_tests {
    #[path = "../nested.rs"]
    mod nested;
}
`,
    "src-tauri/src/planted_tests.rs": `${HAZARD}mod deeper;\n`,
    "src-tauri/src/planted_tests/deeper.rs": HAZARD,
    "src-tauri/src/mac_only.rs": HAZARD,
    "src-tauri/src/seam.rs": HAZARD,
    "src-tauri/src/production.rs": HAZARD,
    "src-tauri/src/nested.rs": HAZARD,
  });
  // Only what production can reach is scanned: `any(test, ...)` is not test-only.
  assert.deepEqual(actual, [planted("src-tauri/src/production.rs"), planted("src-tauri/src/seam.rs")]);
});

test("a file named like a test but reached by an ordinary mod is still scanned", async () => {
  const actual = await scanTree({
    "src-tauri/src/lib.rs": `#[path = "looks_like_tests.rs"]\nmod looks_like_tests;\n`,
    "src-tauri/src/looks_like_tests.rs": HAZARD,
  });
  assert.deepEqual(actual, [planted("src-tauri/src/looks_like_tests.rs")]);
});

test("a file production also declares is scanned, and what it declares, whatever test code declares it", async () => {
  const actual = await scanTree({
    "src-tauri/src/lib.rs": `#[cfg(test)]
#[path = "shared.rs"]
mod shared_for_tests;
#[path = "shared.rs"]
mod shared;
#[cfg(test)]
#[path = "test_only.rs"]
mod test_only;
`,
    "src-tauri/src/shared.rs": `${HAZARD}mod inner;\n`,
    "src-tauri/src/shared/inner.rs": HAZARD,
    "src-tauri/src/test_only.rs": HAZARD,
  });
  // The shared file and what it declares are scanned; a file only test code reaches is not.
  assert.deepEqual(actual, [planted("src-tauri/src/shared.rs"), planted("src-tauri/src/shared/inner.rs")]);
});
