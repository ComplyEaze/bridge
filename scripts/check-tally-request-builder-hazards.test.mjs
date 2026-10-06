#!/usr/bin/env node
// Contract tests for the amount-field-without-type kind in
// scripts/check-tally-request-builder-hazards.mjs. Its pinned set is empty, so
// on the real tree a scanner that stopped matching would still pass; these
// synthetic --root trees are its positive control.
//
// A fixture tree holds none of the pinned violations, so every run here fails
// with them listed as `missing:`. That line proves the scan completed; the
// assertions are about what is, or is not, listed as `unexpected:`.
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

async function scan(fields) {
  const root = await mkdtemp(join(tmpdir(), ".request-builder-hazards-"));
  try {
    await mkdir(join(root, "src-tauri/src"), { recursive: true });
    await mkdir(join(root, "tools"), { recursive: true });
    await writeFile(join(root, FILE), builders(fields));
    try {
      execFileSync("node", [GATE, "--root", root], { encoding: "utf8", stdio: "pipe" });
    } catch (error) {
      const output = `${error.stdout ?? ""}${error.stderr ?? ""}`;
      assert.match(output, /\nmissing:\n/, "the scan must complete and report the pinned set missing");
      const unexpected = /\nunexpected:\n([\s\S]*?)(?:\nmissing:|$)/.exec(output)?.[1] ?? "";
      return new Set(unexpected.split("\n").filter(Boolean).map((line) => line.replace(/^- /, "")));
    }
    throw new Error("a fixture tree lacks the pinned set, so the gate must fail");
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
    await mkdir(join(root, "tools"), { recursive: true });
    for (const [path, text] of Object.entries(files)) {
      await mkdir(join(root, path.split("/").slice(0, -1).join("/")), { recursive: true });
      await writeFile(join(root, path), text);
    }
    try {
      execFileSync("node", [GATE, "--root", root], { encoding: "utf8", stdio: "pipe" });
    } catch (error) {
      const output = `${error.stdout ?? ""}${error.stderr ?? ""}`;
      assert.match(output, /\nmissing:\n/, "the scan must complete and report the pinned set missing");
      const unexpected = /\nunexpected:\n([\s\S]*?)(?:\nmissing:|$)/.exec(output)?.[1] ?? "";
      return [...new Set(unexpected.split("\n").filter(Boolean).map((line) => line.replace(/^- /, "")))].sort();
    }
    throw new Error("a fixture tree lacks the pinned set, so the gate must fail");
  } finally {
    await rm(root, { recursive: true, force: true });
  }
}

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
