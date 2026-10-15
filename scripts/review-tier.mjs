#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0
// Assigns a pull request a review tier from the paths it changes, using the committed list scripts/review-tier-paths.json.
//   one  - every changed path is documentation, a plain test or production code that does not touch the write path
//   full - any path is on the write path, a credential or security path, fixture data, or is not classified by the list
// The first rule whose `match` holds decides a path, so a more specific rule goes above a general one (a write-path test file
// is listed above the rule for plain tests). A path no rule matches, a change with no paths, an unreadable input and an
// invalid list all give `full`: the tool never answers `one` from missing information.
//
//   node scripts/review-tier.mjs --paths-file FILE       one changed path per line (a rename lists both names)
//   node scripts/review-tier.mjs --pr N [--repo O/R]     the pull request's files and previous names from the GitHub API
// Prints one JSON object on stdout. Exit 0 when it could classify, 2 when it could not (the object then says full).
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, realpathSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const CLASS_TIER = {
  "write-path": "full",
  "credential-security": "full",
  "fixture-data": "full",
  production: "one",
  tests: "one",
  docs: "one",
};
const MATCH_KEYS = ["exact", "prefix", "suffix", "contains", "icontains"];
const here = dirname(fileURLToPath(import.meta.url));

function refuse(message) {
  throw new Error(`review-tier: ${message}`);
}

export function loadRules(document) {
  if (document === null || typeof document !== "object" || Array.isArray(document)) refuse("the list must be an object");
  if (Object.keys(document).sort().join() !== "rules,schema_version") refuse("the list must hold exactly rules and schema_version");
  if (document.schema_version !== 1) refuse("unsupported schema_version");
  if (!Array.isArray(document.rules) || document.rules.length === 0) refuse("rules must be a non-empty array");
  const seen = new Set();
  const rules = document.rules.map((rule, index) => {
    if (rule === null || typeof rule !== "object") refuse(`rule ${index} is not an object`);
    const keys = Object.keys(rule).filter((key) => key !== "note").sort().join();
    if (keys !== "class,match") refuse(`rule ${index} must hold class, match and optionally note`);
    if (!Object.hasOwn(CLASS_TIER, rule.class)) refuse(`rule ${index} has an unknown class`);
    if (rule.match === null || typeof rule.match !== "object") refuse(`rule ${index} has no match`);
    const matchKeys = Object.keys(rule.match);
    if (matchKeys.length === 0 || matchKeys.some((key) => !MATCH_KEYS.includes(key))) refuse(`rule ${index} has an empty or unknown match`);
    for (const key of matchKeys) {
      if (typeof rule.match[key] !== "string" || rule.match[key] === "") refuse(`rule ${index} has an empty ${key}`);
    }
    const fingerprint = JSON.stringify([rule.class, MATCH_KEYS.map((key) => rule.match[key] ?? null)]);
    if (seen.has(fingerprint)) refuse(`rule ${index} repeats an earlier rule`);
    seen.add(fingerprint);
    return { class: rule.class, match: rule.match, index };
  });
  return rules;
}

function matches(path, match) {
  if (match.exact !== undefined && path !== match.exact) return false;
  if (match.prefix !== undefined && !path.startsWith(match.prefix)) return false;
  if (match.suffix !== undefined && !path.endsWith(match.suffix)) return false;
  if (match.contains !== undefined && !path.includes(match.contains)) return false;
  if (match.icontains !== undefined && !path.toLowerCase().includes(match.icontains.toLowerCase())) return false;
  return true;
}

export function classify(path, rules) {
  for (const rule of rules) {
    if (matches(path, rule.match)) return { class: rule.class, rule: rule.index };
  }
  return null;
}

// A test file carries the tier of the module it tests: `x_tests.rs` is judged with `x.rs`, and `dir/tests.rs` with `dir/mod.rs`. A module
// that no rule classifies makes its test file full as well.
function moduleOf(path) {
  if (path.endsWith("_tests.rs")) return `${path.slice(0, -"_tests.rs".length)}.rs`;
  if (path.endsWith("/tests.rs")) return `${path.slice(0, -"tests.rs".length)}mod.rs`;
  return null;
}

export function tierOf(paths, rules) {
  const files = [];
  const unclassified = [];
  for (const path of paths) {
    const hit = classify(path, rules);
    const module = moduleOf(path);
    const moduleHit = module === null ? undefined : classify(module, rules);
    if (hit === null) {
      unclassified.push(path);
      files.push({ path, class: null, tier: "full" });
    } else if (moduleHit === null) {
      unclassified.push(module);
      files.push({ path, class: hit.class, tier: "full", module });
    } else if (moduleHit !== undefined && CLASS_TIER[moduleHit.class] === "full") {
      files.push({ path, class: moduleHit.class, tier: "full", module });
    } else {
      files.push({ path, class: hit.class, tier: CLASS_TIER[hit.class] });
    }
  }
  const tier = files.length > 0 && files.every((file) => file.tier === "one") ? "one" : "full";
  return { tier, files, unclassified };
}

function readRules() {
  return loadRules(JSON.parse(readFileSync(join(here, "review-tier-paths.json"), "utf8")));
}

function pathsFromPull(pr, repo) {
  const output = execFileSync("gh", ["api", "--paginate", "--slurp", `repos/${repo}/pulls/${pr}/files?per_page=100`], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  const pages = JSON.parse(output);
  if (!Array.isArray(pages) || !pages.every(Array.isArray)) refuse("the file list did not come back as pages");
  const paths = [];
  for (const file of pages.flat()) {
    if (typeof file.filename !== "string") refuse("a file entry has no name");
    paths.push(file.filename);
    if (typeof file.previous_filename === "string") paths.push(file.previous_filename);
  }
  return paths;
}

function main(argv) {
  let source = null;
  let repo = "ComplyEaze/bridge";
  for (let index = 0; index < argv.length; index += 2) {
    const flag = argv[index];
    const value = argv[index + 1];
    if (value === undefined) refuse(`${flag} needs a value`);
    if (flag === "--paths-file") source = { kind: "file", value };
    else if (flag === "--pr") source = { kind: "pr", value };
    else if (flag === "--repo") repo = value;
    else refuse(`unknown argument ${flag}`);
  }
  if (source === null) refuse("give --paths-file FILE or --pr N");
  const rules = readRules();
  const paths = source.kind === "file"
    ? readFileSync(source.value, "utf8").split("\n").map((line) => line.replace(/\r$/, "")).filter((line) => line !== "")
    : pathsFromPull(source.value, repo);
  return tierOf(paths, rules);
}

if (process.argv[1] && existsSync(process.argv[1]) && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url))) {
  try {
    console.log(JSON.stringify(main(process.argv.slice(2))));
  } catch (error) {
    console.log(JSON.stringify({ tier: "full", error: String(error.message ?? error).slice(0, 300), files: [], unclassified: [] }));
    process.exitCode = 2;
  }
}
