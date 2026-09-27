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
