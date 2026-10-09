// Compares the per-test outcomes of the Windows run of the Node suite with the known failures
// (.github/node-windows-known-failures.txt, bridge#1471) and writes the job summary. It exits 0
// when every failure is a known one, 1 when a failure is not, and 2 when the run cannot be trusted:
// no outcomes, a different set of test files from scripts/*.test.mjs, or a non-zero exit of node
// with no failed test to explain it. A run that cannot be trusted is never reported as a pass.
// Use: node scripts/check-node-windows-outcomes.mjs --outcomes FILE --baseline FILE --node-exit N [--summary FILE]
import { appendFileSync, existsSync, readdirSync, readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

export const EXIT = { ok: 0, newFailures: 1, untrusted: 2 };
const LIST_CAP = 50;

export const failureKey = (outcome) => `${outcome.file} > ${outcome.names.join(" > ")}`;

export function parseOutcomes(text) {
  return text.split("\n").filter((line) => line.trim() !== "").map((line) => JSON.parse(line));
}

export function parseBaseline(text) {
  return text.split("\n").map((line) => line.trim()).filter((line) => line !== "" && !line.startsWith("#"));
}

const counts = (keys) => {
  const map = new Map();
  for (const key of keys) map.set(key, (map.get(key) ?? 0) + 1);
  return map;
};

export function compare({ outcomes, baseline, testFiles, nodeExit }) {
  const base = { newFailures: [], fixed: [], table: [] };
  if (outcomes.length === 0) return { ...base, code: EXIT.untrusted, reason: "outcomes_missing" };
  const reporting = new Set(outcomes.map((outcome) => outcome.file));
  const expected = new Set(testFiles);
  const missing = [...expected].filter((file) => !reporting.has(file)).sort();
  const extra = [...reporting].filter((file) => !expected.has(file)).sort();
  if (missing.length > 0 || extra.length > 0) {
    return { ...base, code: EXIT.untrusted, reason: "file_set_differs", missing, extra };
  }
  const failures = outcomes.filter((outcome) => outcome.outcome === "fail");
  if (nodeExit !== 0 && failures.length === 0) return { ...base, code: EXIT.untrusted, reason: "exit_without_failure" };

  const known = counts(baseline);
  const seen = new Map();
  const newFailures = [];
  for (const outcome of failures) {
    const key = failureKey(outcome);
    const used = (seen.get(key) ?? 0) + 1;
    seen.set(key, used);
    if (used > (known.get(key) ?? 0)) newFailures.push({ key, error: outcome.error ?? "" });
  }
  const fixed = [];
  for (const [key, count] of known) {
    for (let i = (seen.get(key) ?? 0); i < count; i += 1) fixed.push(key);
  }
  const perFile = new Map();
  for (const outcome of outcomes) {
    const row = perFile.get(outcome.file) ?? { file: outcome.file, pass: 0, fail: 0, skip: 0, known: 0 };
    row[outcome.outcome] += 1;
    perFile.set(outcome.file, row);
  }
  for (const [key, count] of known) {
    const row = perFile.get(key.split(" > ")[0]);
    if (row) row.known += count;
  }
  const table = [...perFile.values()].sort((a, b) => a.file.localeCompare(b.file));
  return { code: newFailures.length > 0 ? EXIT.newFailures : EXIT.ok, reason: newFailures.length > 0 ? "new_failures" : "ok", newFailures, fixed, table, failures: failures.map(failureKey) };
}

export function summary(result, host) {
  const lines = ["## Node suite on Windows (informational; not a required check, bridge#1471)", ""];
  if (host) lines.push(host, "");
  if (result.reason === "outcomes_missing") lines.push("**No outcomes were written. This run cannot be trusted.**");
  else if (result.reason === "file_set_differs") {
    lines.push("**The test files that reported differ from `scripts/*.test.mjs`. This run cannot be trusted.**");
    if (result.missing.length) lines.push(`- did not report: ${result.missing.join(", ")}`);
    if (result.extra.length) lines.push(`- not in the suite: ${result.extra.join(", ")}`);
  } else if (result.reason === "exit_without_failure") lines.push("**node exited non-zero but no test failed. This run cannot be trusted.**");
  else {
    const total = result.table.reduce((sum, row) => sum + row.pass + row.fail + row.skip, 0);
    const failed = result.table.reduce((sum, row) => sum + row.fail, 0);
    const skipped = result.table.reduce((sum, row) => sum + row.skip, 0);
    lines.push(`${total} tests: ${failed} failed (${failed - result.newFailures.length} known), ${skipped} skipped. **New failures: ${result.newFailures.length}.**`, "");
    if (result.newFailures.length > 0) {
      lines.push("### New failures", "");
      for (const { key, error } of result.newFailures.slice(0, LIST_CAP)) lines.push(`- \`${key}\`${error ? `: ${error}` : ""}`);
      if (result.newFailures.length > LIST_CAP) lines.push(`- and ${result.newFailures.length - LIST_CAP} more`);
      lines.push("");
    }
    if (result.fixed.length > 0) {
      lines.push("### Known failures that now pass (remove them from the known-failures file)", "");
      for (const key of result.fixed.slice(0, LIST_CAP)) lines.push(`- \`${key}\``);
      if (result.fixed.length > LIST_CAP) lines.push(`- and ${result.fixed.length - LIST_CAP} more`);
      lines.push("");
    }
    lines.push("### By test file", "", "| file | pass | fail | known | skip |", "| --- | ---: | ---: | ---: | ---: |");
    for (const row of result.table) lines.push(`| ${row.file} | ${row.pass} | ${row.fail} | ${row.known} | ${row.skip} |`);
    lines.push("", "<details><summary>Every failing test of this run, in the known-failures file's format</summary>", "", "```");
    lines.push(...[...result.failures].sort());
    lines.push("```", "", "</details>");
  }
  return `${lines.join("\n")}\n`;
}

function main(argv) {
  const args = new Map();
  for (let i = 0; i < argv.length; i += 2) args.set(argv[i], argv[i + 1]);
  const outcomesPath = args.get("--outcomes");
  const baselinePath = args.get("--baseline");
  const nodeExit = Number(args.get("--node-exit"));
  if (!outcomesPath || !baselinePath || !Number.isInteger(nodeExit)) {
    console.error("usage: check-node-windows-outcomes.mjs --outcomes FILE --baseline FILE --node-exit N [--summary FILE]");
    return EXIT.untrusted;
  }
  const testFiles = readdirSync(new URL(".", import.meta.url)).filter((name) => name.endsWith(".test.mjs")).map((name) => `scripts/${name}`);
  let outcomes = [];
  try {
    outcomes = existsSync(outcomesPath) ? parseOutcomes(readFileSync(outcomesPath, "utf8")) : [];
  } catch {
    outcomes = [];
  }
  const baseline = existsSync(baselinePath) ? parseBaseline(readFileSync(baselinePath, "utf8")) : [];
  const result = compare({ outcomes, baseline, testFiles, nodeExit });
  const text = summary(result, args.get("--host"));
  process.stdout.write(text);
  if (args.get("--summary")) appendFileSync(args.get("--summary"), text);
  if (result.code === EXIT.newFailures) console.log(`::error::${result.newFailures.length} Node test(s) failed on Windows that are not in the known-failures file`);
  if (result.code === EXIT.untrusted) console.log(`::error::the Windows run of the Node suite cannot be trusted (${result.reason})`);
  if (result.fixed.length > 0) console.log(`::warning::${result.fixed.length} known Windows failure(s) now pass; remove them from .github/node-windows-known-failures.txt`);
  return result.code;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) process.exitCode = main(process.argv.slice(2));
