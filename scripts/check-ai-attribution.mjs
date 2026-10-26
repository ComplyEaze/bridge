// SPDX-License-Identifier: Apache-2.0
//
// Owner rule of 25 Oct 2026: no AI attribution in public text. The merge gate (scripts/merge-gate.sh) feeds this the
// pull request's title, body and commits as one JSON object on stdin and refuses when
//   - the title or the body, comments included, holds a "Generated with Claude Code" line, in the plain form or the
//     Markdown-link form, or
//   - a commit made on or after RULE_FROM carries a Co-authored-by trailer that names Claude or Anthropic (or a
//     "Generated with Claude" line in its message).
// An older commit with the same text is reported, not refused: it predates the rule (the owner decides what to do
// with them). A commit whose date cannot be read is treated as made after the rule, because a refusal can be argued
// with and an unprovable pass cannot.
//
// Input:  {"title": string, "body": string, "commits": [{"sha": string, "message": string, "committed_at": string}]}
// Output: one line per finding, "REFUSE <where>: <what>" or "NOTE <where>: <what>"; exit 0 clean, 1 refused, 2 bad input.
// Run:    node scripts/check-ai-attribution.mjs < input.json

import { realpathSync } from "node:fs";
import { fileURLToPath } from "node:url";

export const RULE_FROM = "2026-10-25T13:00:00Z";

// "Generated with Claude Code", "Generated with [Claude Code](https://...)", and the "by" and bare-"Claude" variants, any case.
const GENERATED_LINE = /generated\s+(?:with|by)\s+\[?\s*claude\b/i;
// A trailer is a line that starts with the key (git reads them at the end of a message; a line anywhere in it is refused too).
const CO_AUTHOR_TRAILER = /^[ \t]*co-authored-by[ \t]*:[^\r\n]*(?:claude|anthropic)[^\r\n]*$/im;

export function evaluate(input, ruleFrom = RULE_FROM) {
  const cutoff = Date.parse(ruleFrom);
  const refusals = [];
  const notes = [];
  for (const [where, text] of [["title", input.title], ["body", input.body]]) {
    if (GENERATED_LINE.test(text)) refusals.push(`${where}: a "Generated with Claude" line`);
  }
  for (const commit of input.commits) {
    const found = [];
    if (CO_AUTHOR_TRAILER.test(commit.message)) found.push("a Co-authored-by trailer naming Claude or Anthropic");
    if (GENERATED_LINE.test(commit.message)) found.push('a "Generated with Claude" line');
    if (found.length === 0) continue;
    const when = Date.parse(commit.committed_at);
    const label = `commit ${commit.sha.slice(0, 12)}`;
    if (Number.isNaN(when)) refusals.push(`${label}: ${found.join(" and ")}, and its date cannot be read`);
    else if (when >= cutoff) refusals.push(`${label}: ${found.join(" and ")}, made after the rule (${commit.committed_at})`);
    else notes.push(`${label}: ${found.join(" and ")}, made before the rule (${commit.committed_at})`);
  }
  return { refusals, notes };
}

function parseInput(text) {
  let input;
  try {
    input = JSON.parse(text);
  } catch {
    return null;
  }
  const isString = (value) => typeof value === "string";
  if (input === null || typeof input !== "object" || !isString(input.title) || !isString(input.body) || !Array.isArray(input.commits)) return null;
  if (!input.commits.every((c) => c !== null && typeof c === "object" && isString(c.sha) && isString(c.message) && isString(c.committed_at))) return null;
  return input;
}

if (process.argv[1] && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url))) {
  const chunks = [];
  for await (const chunk of process.stdin) chunks.push(chunk);
  const input = parseInput(Buffer.concat(chunks).toString("utf8"));
  if (input === null) {
    console.error("check-ai-attribution: the input is not the expected JSON object");
    process.exit(2);
  }
  const { refusals, notes } = evaluate(input);
  for (const line of refusals) console.log(`REFUSE ${line}`);
  for (const line of notes) console.log(`NOTE ${line}`);
  process.exit(refusals.length > 0 ? 1 : 0);
}
