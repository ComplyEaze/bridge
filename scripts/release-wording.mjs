// SPDX-License-Identifier: Apache-2.0
//
// The release process (docs/release-process.md, "Writing rules for notes") forbids user-facing text from
// calling a build signed, notarized or ready for production while it has no publisher signing, and from
// calling a release a "preview" or "for evaluation only". This module finds the sentences that do, in the
// files a person reads; scripts/release-wording.test.mjs holds the tree to it. It reads text and writes nothing.
//
// A sentence is a claim when it holds a banned term. It is denied when every banned term in it has a negator
// ("not", "no", "never", "without", "unsigned", ...) within a few words before it and no clause break between
// them, so "is not code-signed" passes and "is code-signed" does not. A claim that is neither denied nor named
// in ALLOWED is a violation. A bare match on "signed" is not a term: the tree also says "signed amount".
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

const HYPHENS = "\\-\\u2010-\\u2015";
const MAKERS = "apple|microsoft|authenticode|developer id|ev";
// The words that make a "preview" something other than the product: the import preview, the read-cost preview, a batch's approval preview.
const NOT_THE_PRODUCT = "in|on|of|from|to|for|by|at|import|approval|read-cost|batch|post|voucher|ledger|data|statement|call|window|dry";
const TERMS = [
  // signed: the named kinds of signing, and "is/are/was ... signed" said of a thing
  `code[${HYPHENS}\\s]?signed|digitally[${HYPHENS}\\s]signed|codesigned`,
  `(?<![A-Za-z])(?:${MAKERS})[${HYPHENS}\\s]signed`,
  "(?<![A-Za-z-])(?:is|are|was|were|be|been|fully|now|also|already|properly) signed(?![-A-Za-z])",
  "(?<![A-Za-z-])signed (?:by|with|using|\\.?(?:build|release|installer|package|binary|app|download|msi|mcpb|exe|dmg|extension|bundle|archive|file|artifact|version|asset|software|executable)s?)\\b",
  // notarized (the noun "notarization" names a gate in the rules and is not a claim)
  "notari[sz]ed",
  // ready for production
  "ready[- ](?:to use |for use )?(?:for|in) production|production[- ](?:ready|use|release|grade|quality|build)|(?:suitable|safe|fit) for production|prod-ready",
  // for evaluation
  "for evaluat(?:ion|ing)|evaluation[- ](?:only|build|version|copy|release)",
  // a preview or a pre-release: the product or a release called one (the import preview and the read-cost
  // preview are other things and are not matched)
  `(?:(?<![A-Za-z-])(?:is|was|remains|stays|as|currently|still|only|just)|(?<=[A-Za-z]['’])s)\\s+(?:(?:only|just|very much|still|an?|the)\\s+)*(?:(?!${NOT_THE_PRODUCT})[\\w-]+\\s+){0,2}previews?(?![-A-Za-z'’])`,
  "\\bcall(?:s|ed)? (?:this|it|bridge|the product|the app) an? previews?\\b|\\bbridge\\s*(?:\\(|:)\\s*previews?\\b|\\(previews?\\)",
  "(?<![A-Za-z-])(?:early|technical|public|free|private|developer|open|this|our|current) previews?(?![-A-Za-z])",
  "(?<![A-Za-z-])in preview(?![-A-Za-z])|previews? (?:release|build|package|version|channel|archive)s?(?![-A-Za-z])|(?<![A-Za-z])pre-?releases?(?![-A-Za-z])",
];
const CLAIM_SOURCE = TERMS.join("|");
const NEGATOR = /(?:\b(?:not|no|never|without|nor|cannot)\b(?!\s+(?:only|one|doubt)\b)|n['’]t\b|\bunsigned\b|\bun-)/i;
const CLAUSE_BREAK = /[,;:()–—]|\s-\s|\b(?:because|since|while|although|though|whereas|when|which|that|but|however|so|until|if|and|or|nor|yet\s+(?:it|this|that|the|a|an|we|there|they))\b/gi;
const NEGATOR_REACH = 5; // words between the negator and the term, at most

// Not read: third-party code, and pages the deploy generates from files that are covered.
export const NOT_COVERED = [/^site\/vendor\//, /^site\/(?:changelog|privacy|terms|terms-2026-10)\.html$/];

export const COVERED = {
  // single files
  files: ["CHANGELOG.md", "README.md", "SECURITY.md", "docs/security-and-privacy.md", "registry/server.json", "site/llms.txt", "site/feed.xml", "packaging/mcpb/manifest.json"],
  // `endings` select the files; every directory is searched whole. `mayBeEmpty`: the directory holds no file
  // right after a release cut, which removes the unreleased entries.
  directories: [
    { path: "docs/changelog/unreleased", endings: [".md"], mayBeEmpty: true },
    { path: "docs/agent", endings: [".md"] },
    { path: "docs/guides", endings: [".md"] },
    { path: "docs/legal", endings: [".md"] },
    { path: "packaging/mcpb", endings: [".md"] },
    { path: "site", endings: [".html", ".js", ".mjs"] },
  ],
};

// Files that must be among the covered ones, so a rename cannot take a page quietly out of the check.
export const MUST_BE_COVERED = [
  "CHANGELOG.md",
  "README.md",
  "SECURITY.md",
  "registry/server.json",
  "docs/agent/INSTALL.md",
  "docs/agent/README.md",
  "packaging/mcpb/UNSIGNED_PREVIEW_RELEASE.md",
  "packaging/mcpb/INSTALL_AND_CHECK_RELEASE.md",
  "site/index.html",
  "site/download.html",
  "site/faq.html",
  "site/security.html",
  "site/download.js",
];

function jsonStrings(value, out) {
  if (typeof value === "string") out.push(value);
  else if (Array.isArray(value)) for (const item of value) jsonStrings(item, out);
  else if (value && typeof value === "object") for (const item of Object.values(value)) jsonStrings(item, out);
  return out;
}

// The text a person reads. A page: visible words, the descriptions and labels carried in attributes, and the
// strings of its structured data. A JSON file: its string values, each apart. A script: its quoted strings, each apart. Anything else: as written, minus
// comments. A JSON file or structured-data block that does not parse throws: it is not read as empty.
function readableText(file, text) {
  let body = text.replace(/<!--[\s\S]*?-->/g, " ");
  if (file.endsWith(".json")) return jsonStrings(JSON.parse(text), []).join("\n\n");
  if (/\.m?js$/.test(file)) return [...text.matchAll(/(["'`])((?:\\.|(?!\1)[^\\\n])*)\1/g)].map((match) => match[2]).join("\n\n");
  if (!file.endsWith(".html")) return body;
  const structured = [];
  body = body.replace(/<script\b([^>]*)>([\s\S]*?)<\/script>/gi, (_all, attributes, inner) => {
    if (/type\s*=\s*["']application\/ld\+json["']/i.test(attributes)) jsonStrings(JSON.parse(inner), structured);
    return " ";
  });
  body = body.replace(/<style\b[\s\S]*?<\/style>/gi, " ");
  const spoken = [...body.matchAll(/\b(?:content|aria-label|alt|title|placeholder)=(?:"([^"]*)"|'([^']*)')/g)].map((match) => match[1] ?? match[2]);
  const visible = body
    .replace(/<\/(p|li|h[1-6]|tr|td|th|dt|dd|div|section|summary|blockquote)>|<br\s*\/?>/gi, "\n\n")
    .replace(/<[^>]+>/g, " ")
    .replace(/&nbsp;/g, " ")
    .replace(/&(?:rsquo|#39|#x27|apos);/g, "'")
    .replace(/&amp;/g, "&");
  return `${visible}\n\n${[...spoken, ...structured].join("\n\n")}`;
}

// The sentences of a text, whitespace collapsed to single spaces. A blank line, a list item, a table row, a
// heading and a quotation each start a new paragraph, so one item's words are not joined to the next one's.
export function sentencesOf(file, text) {
  const out = [];
  for (const paragraph of readableText(file, text).split(/\n\s*\n|\n(?=\s*(?:[-*+#>|]|\d+\.)\s?)|(?<=^#{1,6} [^\n]*)\n/m)) {
    const flat = paragraph.replace(/\s+/g, " ").trim();
    if (flat) out.push(...flat.split(/(?<=[.?!])\s+(?=[A-Z0-9`*#|\-("“'])/));
  }
  return out;
}

export function isClaim(sentence) {
  return new RegExp(CLAIM_SOURCE, "i").test(sentence);
}

// The words before `index`, back to the last clause break, at most NEGATOR_REACH + 1 of them.
function lead(sentence, index) {
  const before = sentence.slice(0, index);
  const cuts = [...before.matchAll(CLAUSE_BREAK)];
  const last = cuts.length ? cuts[cuts.length - 1] : null;
  const clause = last ? before.slice(last.index + last[0].length) : before;
  return clause.trim().split(/\s+/).slice(-(NEGATOR_REACH + 1)).join(" ");
}

// "not signed with a publisher identity or notarised": a term that follows a denied term, joined to it by
// "or", "nor" or "and" within a few words and with no verb or clause break between, is denied with it.
const JOINED = /^(?:(?![,;:()\u2013\u2014]|\b(?:is|are|was|were|has|have|will|being|because|since|while|although|though|whereas|when|which|that|but|however|so|until|if)\b)[^\s]+\s+){0,7}?(?:or|nor|and)\s+(?:also\s+)?$/i;

// A term that starts with its own verb ("and remains a preview") is a new statement, not a joined term.
const VERB_LED = /^(?:is|was|remains|stays|as|currently|still|only|just)\s/i;

export function isDenied(sentence) {
  const terms = [...sentence.matchAll(new RegExp(CLAIM_SOURCE, "gi"))];
  let previous = null; // whether the term before this one is denied, and where it ended
  for (const term of terms) {
    let denied = NEGATOR.test(lead(sentence, term.index));
    if (!denied && previous?.denied && !VERB_LED.test(term[0])) denied = JOINED.test(sentence.slice(previous.end, term.index).trimStart());
    if (!denied) return false;
    previous = { denied, end: term.index + term[0].length };
  }
  return terms.length > 0;
}

// The claims in a file's text that are not denied, in order.
export function bareClaims(file, text) {
  return sentencesOf(file, text).filter((sentence) => isClaim(sentence) && !isDenied(sentence));
}

// Sentences that are allowed to stay although `bareClaims` finds them, one entry each: the file and the
// whole sentence as `sentencesOf` returns it. Never a pattern. An entry that no longer matches a bare claim
// in its file fails the test, so this list cannot outlive the sentence.
export const ALLOWED = [
  {
    file: "README.md",
    sentence: "- Intel Macs, and a code-signed installer",
    reason: "An item in the list of what is not covered (\"A base currency other than INR\", \"Tally Cloud Access\", ...): the list's heading denies it.",
  },
  {
    file: "docs/agent/README.md",
    sentence: "The amount is signed as sent; the rows of one entry are not checked against the entry's own amount.",
    reason: "\"signed\" is the sign of a number (debit or credit), not a claim about a build.",
  },
  {
    file: "site/releases.js",
    sentence: "Pre-release",
    reason: "The label for a listed release that GitHub flags as a pre-release and the page does not offer to install; an installable one is labelled \"Not yet code-signed\" instead.",
  },
];

export function violationsIn(file, text) {
  return bareClaims(file, text).filter((sentence) => !ALLOWED.some((entry) => entry.file === file && entry.sentence === sentence));
}

// The covered files, as paths from the repository root with `/` separators, sorted.
export function coveredFiles(root) {
  const found = [];
  for (const file of COVERED.files) if (existsSync(join(root, file))) found.push(file);
  for (const { path, endings } of COVERED.directories) {
    const walk = (relative) => {
      if (!existsSync(join(root, relative))) return;
      for (const name of readdirSync(join(root, relative)).sort()) {
        const child = `${relative}/${name}`;
        if (statSync(join(root, child)).isDirectory()) walk(child);
        else if (endings.some((ending) => name.endsWith(ending)) && !NOT_COVERED.some((skip) => skip.test(child))) found.push(child);
      }
    };
    walk(path);
  }
  return found.sort();
}

export function readCovered(root) {
  return coveredFiles(root).map((file) => ({ file, text: readFileSync(join(root, file), "utf8") }));
}
