// SPDX-License-Identifier: Apache-2.0

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

// The text these files show names the product in full, never by its bare short name (#1168).
// A line that is not a comment and holds the word "Bridge" must have it as "ComplyEaze Bridge",
// so a new message in one of these files cannot bring the short name back. A comment is a line
// starting with a comment marker, or any line inside a block comment that spans several lines,
// such as a multi-line JSX comment.
const SOURCES = [
  "src/AllClientsScreen.tsx",
  "src/ErrorBoundary.tsx",
  "src/JournalPostingScreen.tsx",
  "src/LedgerEntriesScreen.tsx",
  "src/MirrorProofScreen.tsx",
  "src/NativeLifecycleController.tsx",
  "src/OutstandingsEvidencePanel.tsx",
  "src/OutstandingsScreen.tsx",
  "src/SourceDraftScreen.tsx",
  "src/main.tsx",
  "src/outstandings-copy.ts",
  "src/tally-capability-evidence.tsx",
  "src/tally-company-selection.ts",
  "src/tally-error-copy.ts",
];

/**
 * The bare "Bridge" words outside comments in `text`, as `{ line, text }`. A line starting with
 * `//` is a comment; a block comment is skipped from `/*` to its `*\/`, on one line or across
 * several, and whatever follows its close on the same line is checked (#1190's review).
 */
export function bareBridges(text) {
  const found = [];
  let inBlockComment = false;
  text.split("\n").forEach((line, index) => {
    let visible = "";
    let rest = line;
    while (rest.length > 0) {
      if (inBlockComment) {
        const close = rest.indexOf("*/");
        if (close < 0) break;
        rest = rest.slice(close + 2);
        inBlockComment = false;
        continue;
      }
      const trimmed = rest.trimStart();
      if (visible.trim() === "" && (trimmed.startsWith("//") || trimmed.startsWith("* "))) break;
      const open = rest.indexOf("/*");
      if (open < 0) {
        visible += rest;
        break;
      }
      visible += rest.slice(0, open);
      rest = rest.slice(open + 2);
      inBlockComment = true;
    }
    for (const match of visible.matchAll(/(?<![\w])Bridge(?![\w])/g)) {
      if (!visible.slice(0, match.index).endsWith("ComplyEaze ")) {
        found.push({ line: index + 1, text: line.trim() });
      }
    }
  });
  return found;
}

test("the front end's displayed text names the product in full", async () => {
  for (const file of SOURCES) {
    const text = await readFile(new URL(`../${file}`, import.meta.url), "utf8");
    assert.deepEqual(
      bareBridges(text).map(({ line, text: shown }) => `${file}:${line}: ${shown}`),
      [],
      "a bare Bridge outside a comment",
    );
  }
});

test("text after a comment closes is checked, and text inside one is not", () => {
  assert.deepEqual(bareBridges("{/* Bridge's report\n   still Bridge */}<span>Bridge</span>"), [
    { line: 2, text: "still Bridge */}<span>Bridge</span>" },
  ]);
  assert.deepEqual(bareBridges("{/* x */}<span>Bridge</span>"), [
    { line: 1, text: "{/* x */}<span>Bridge</span>" },
  ]);
  assert.deepEqual(bareBridges("// Bridge\n * Bridge\n<span>ComplyEaze Bridge</span>"), []);
  assert.deepEqual(bareBridges("<p>Bridge</p>"), [{ line: 1, text: "<p>Bridge</p>" }]);
});
