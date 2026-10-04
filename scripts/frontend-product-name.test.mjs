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

test("the front end's displayed text names the product in full", async () => {
  for (const file of SOURCES) {
    const lines = (await readFile(new URL(`../${file}`, import.meta.url), "utf8")).split("\n");
    let inBlockComment = false;
    lines.forEach((line, index) => {
      const trimmed = line.trimStart();
      if (inBlockComment) {
        if (line.includes("*/")) inBlockComment = false;
        return;
      }
      if (["//", "*", "/*", "{/*"].some((comment) => trimmed.startsWith(comment))) {
        inBlockComment = trimmed.includes("/*") && !trimmed.includes("*/");
        return;
      }
      for (const match of line.matchAll(/(?<![\w])Bridge(?![\w])/g)) {
        assert.ok(
          line.slice(0, match.index).endsWith("ComplyEaze "),
          `${file}:${index + 1}: a bare Bridge: ${trimmed}`,
        );
      }
    });
  }
});
