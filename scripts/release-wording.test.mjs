// SPDX-License-Identifier: Apache-2.0
//
// Holds the files a person reads to the release process's writing rule: no text calls a build signed,
// notarized or ready for production, or a release a preview or for evaluation only, unless the sentence
// denies it (see scripts/release-wording.mjs).
import assert from "node:assert/strict";
import { existsSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { ALLOWED, bareClaims, COVERED, coveredFiles, isClaim, isDenied, MUST_BE_COVERED, readCovered, sentencesOf, violationsIn } from "./release-wording.mjs";

const root = fileURLToPath(new URL("../", import.meta.url));
const covered = readCovered(root);

// One banned sentence for each kind of term, so a broken arm of the check cannot hide behind another.
const BANNED = [
  "This build is code-signed.",
  "The installer is notarized.",
  "This build is ready for production.",
  "This build is for evaluation only.",
  "This is a preview.",
  "The installer is signed by us.",
];

// Puts `sentence` into the text through a path the file type really reads, and fails if nothing changed.
function inject(file, text, sentence) {
  let out;
  if (file.endsWith(".html")) out = text.replace("</body>", `<p>${sentence}</p></body>`);
  else if (file.endsWith(".json")) out = text.replace(/^\s*\{/, `{"injected": ${JSON.stringify(sentence)},`);
  else if (/\.m?js$/.test(file)) out = `${text}\nconst injected = ${JSON.stringify(sentence)};\n`;
  else out = `${text}\n\n${sentence}\n`;
  assert.notEqual(out, text, `${file}: the injection changed nothing`);
  return out;
}

test("the named files are covered, each covered directory holds a file, and no covered file is empty", () => {
  for (const file of COVERED.files) assert.ok(existsSync(join(root, file)), `${file} is named as covered and is missing`);
  const found = new Set(coveredFiles(root));
  for (const file of MUST_BE_COVERED) assert.ok(found.has(file), `${file} must be covered and is not`);
  for (const { path, endings, mayBeEmpty } of COVERED.directories) {
    if (mayBeEmpty) continue;
    assert.ok(
      [...found].some((file) => file.startsWith(`${path}/`) && endings.some((ending) => file.endsWith(ending))),
      `${path} is named as covered and holds no file`,
    );
  }
  for (const { file, text } of covered) assert.ok(text.trim().length > 0, `${file} is covered and empty`);
});

test("third-party code and pages the deploy generates are not read, and their sources are", () => {
  const found = coveredFiles(root);
  assert.ok(!found.some((file) => file.startsWith("site/vendor/") || file === "site/changelog.html"), "a vendored or generated file is covered");
  assert.ok(found.includes("site/changelog.template.html") && found.includes("docs/legal/terms.md"));
});

test("no covered file makes a claim that is neither denied nor named in the allowlist", () => {
  const violations = covered.flatMap(({ file, text }) => violationsIn(file, text).map((sentence) => `${file}: ${sentence}`));
  assert.deepEqual(violations, []);
});

test("a banned sentence put into any covered file, through the path its type is read by, is found", () => {
  for (const { file, text } of covered) {
    for (const sentence of BANNED) {
      const found = violationsIn(file, inject(file, text, sentence));
      assert.ok(found.some((candidate) => candidate.includes(sentence)), `${file}: "${sentence}" put into it was not found`);
    }
  }
});

test("a page's attributes and structured data are read, not only its visible words", () => {
  const page = (inner) => `<html><head>${inner}</head><body><p>Hello.</p></body></html>`;
  const sentence = "This build is code-signed.";
  assert.deepEqual(violationsIn("x.html", page(`<meta name="description" content="${sentence}">`)), [sentence]);
  assert.deepEqual(violationsIn("x.html", page(`<meta name="description" content='${sentence}'>`)), [sentence]);
  assert.deepEqual(violationsIn("x.html", page(`<script type="application/ld+json">{"a":{"description":"${sentence}"}}</script>`)), [sentence]);
  assert.deepEqual(violationsIn("x.html", page(`<script>var a = "${sentence}";</script>`)), [], "a page script is not text");
  assert.throws(() => violationsIn("x.html", page(`<script type="application/ld+json">{broken</script>`)), "unreadable structured data must fail, not read as empty");
  assert.throws(() => violationsIn("x.json", "{broken"), "unreadable JSON must fail, not read as empty");
  assert.deepEqual(violationsIn("x.js", `const a = "${sentence}"; // This build is notarized.`), [sentence], "a script is read by its strings");
});

test("each wording of a banned claim is found when bare", () => {
  const bare = [
    "The installer is code-signed.",
    "The installer is code signed.",
    "The installer is code‑signed.",
    "The release is digitally signed.",
    "The release is digitally-signed.",
    "The app is notarized by Apple.",
    "The app is notarised.",
    "Apple-notarized.",
    "Download the signed installer.",
    "Signed MSI.",
    "The installer is signed.",
    "The MSI is fully signed.",
    "Signed by ComplyEaze.",
    "Signed with an EV certificate.",
    "Apple-signed and Microsoft-signed.",
    "Developer ID signed.",
    "This build is ready for production.",
    "Ready to use in production.",
    "Ready for use in production.",
    "Fit for production.",
    "A production-ready build.",
    "It is for evaluation.",
    "For evaluation purposes.",
    "For evaluation only.",
    "Evaluation version.",
    "Evaluation-only.",
    "This remains a preview.",
    "This is an early preview.",
    "Bridge is in preview.",
    "Currently in preview.",
    "Download the preview release.",
    "Install the preview build.",
    "This preview supports reads.",
    "A technical preview.",
    "This is a pre-release.",
    "A prerelease build.",
    "Signed builds.",
    "We publish signed installers.",
    "Signed MSIs for Windows.",
    "A signed .msi for Windows.",
    "It's a preview.",
    "Bridge is only a preview.",
    "Bridge is very much a preview.",
    "Bridge is a limited preview.",
    "Bridge was a preview.",
    "We call this a preview.",
    "Bridge (preview)",
    "This is a preview of Bridge.",
  ];
  for (const sentence of bare) assert.ok(isClaim(sentence) && !isDenied(sentence), `${sentence} should be a bare claim`);
});

test("a denial passes only when its negator governs the claim", () => {
  const denied = [
    "The installer is not code-signed.",
    "There is no digitally signed file.",
    "It is not notarized.",
    "It was never ready for production.",
    "It is an unsigned installer, not a signed installer.",
    "There is no signed build and no notarized build.",
    "The builds are not signed with a verified publisher identity or notarised.",
    "This build is not yet code-signed.",
    "Bridge isn’t code-signed.",
    "It has not been notarized.",
  ];
  for (const sentence of denied) assert.ok(isClaim(sentence) && isDenied(sentence), `${sentence} should be a denied claim`);
  // A negator that governs something else, or a claim after a clause break, leaves the claim bare.
  const stillBare = [
    "It is not signed, but it is ready for production.",
    "No one doubts the build is code-signed.",
    "There is no warning because the build is code-signed.",
    "The previous build was not code-signed and this one is notarized.",
    "This build is not code-signed while the older one is notarized.",
    "This build is not yet code-signed — it is ready for production.",
    "This build is not only code-signed but notarized.",
    "It is not code-signed (it is notarized).",
    "It is not code-signed. 2026 builds are notarized.",
    "It is not code-signed. (It is notarized.)",
    "Bridge is not yet code-signed and remains a preview.",
    "The installer is not code-signed and is a preview release.",
    "There is no code-signed installer and a notarized one is planned.",
    "It is not code-signed yet it is notarized.",
  ];
  for (const sentence of stillBare) assert.ok(sentencesOf("x.md", sentence).some((one) => isClaim(one) && !isDenied(one)), `${sentence} should leave a bare claim`);
});

test("a heading, a table row and a list item do not lend their negator to the next line", () => {
  assert.deepEqual(bareClaims("x.md", "## Not signed\nThe next build is notarized."), ["The next build is notarized."]);
  assert.deepEqual(bareClaims("x.md", "| Windows | not code-signed |\n| macOS | notarized |"), ["| macOS | notarized |"]);
  assert.deepEqual(bareClaims("x.md", "- This is not code-signed.\n- The installer is code-signed."), ["- The installer is code-signed."]);
  assert.deepEqual(bareClaims("x.html", "<table><tr><td>Not code-signed</td><td>The installer is notarized</td></tr></table>"), ["The installer is notarized"]);
});

test("words that look like a claim and are not one are not found", () => {
  const plain = [
    "A signed amount is negative for a credit.",
    "Each signed-in user sees a list.",
    "The unsigned installer asks Windows for a decision.",
    "The import preview lists every voucher.",
    "The read-cost preview says how many requests.",
    "The preview window shows the buttons.",
    "Notarization is a gate in the release rules.",
    "A GSTIN is in the preview and in the batch's local record.",
    "Show a data preview first.",
    "The preview is only a screen: nothing is written.",
    "This is a read-cost preview.",
  ];
  for (const sentence of plain) assert.ok(!isClaim(sentence), `${sentence} should not be a claim`);
});

test("an allowlist entry names one file and one whole sentence, and still matches a bare claim there", () => {
  const seen = new Set();
  for (const entry of ALLOWED) {
    assert.deepEqual(Object.keys(entry).sort(), ["file", "reason", "sentence"], `entry for ${entry.file} has other keys`);
    assert.ok(entry.reason.length > 20, `entry for ${entry.file} gives no reason`);
    const key = `${entry.file}\n${entry.sentence}`;
    assert.ok(!seen.has(key), `${entry.file}: the same entry is listed twice`);
    seen.add(key);
    const file = covered.find((candidate) => candidate.file === entry.file);
    assert.ok(file, `${entry.file} is not a covered file`);
    const matching = bareClaims(entry.file, file.text).filter((sentence) => sentence === entry.sentence);
    assert.equal(matching.length, 1, `${entry.file}: no bare claim reads "${entry.sentence}" any more`);
  }
});

test("an allowlist entry covers its own sentence only: not a longer one, not one word changed", () => {
  for (const entry of ALLOWED) {
    // Entries in pages, JSON and scripts are read through their markup; the variants below are added as plain paragraphs.
    if (/\.(html|json|m?js)$/.test(entry.file)) continue;
    const file = covered.find((candidate) => candidate.file === entry.file);
    const first = entry.sentence.match(/[A-Za-z]{4,}/)[0];
    const changed = entry.sentence.replace(first, `${first}x`);
    assert.notEqual(changed, entry.sentence);
    assert.ok(violationsIn(entry.file, `${file.text}\n\n${changed}\n`).includes(changed), `${entry.file}: a sentence with one word changed was let through`);
    const longer = `${entry.sentence.replace(/[.?!]$/, "")}, and this build is code-signed.`;
    assert.ok(violationsIn(entry.file, `${file.text}\n\n${longer}\n`).includes(longer), `${entry.file}: a longer sentence that starts with an allowed one was let through`);
  }
});
