// SPDX-License-Identifier: Apache-2.0
// A version pull request must move src-tauri/release-date.txt (#2416). `next-version.mjs --apply` writes the file
// together with the version; a release cut without it carries the previous release's date, and the build notice that
// reads the date then comes too soon. CI runs this for a pull request or queue entry whose version differs from its
// base's (the `changes` job of ci.yml); the file is one line, YYYY-MM-DD in UTC.
//
//   node scripts/check-release-date-changed.mjs <base-sha> <head-sha>
//
// Exit 0: the head's date is a real day, later than the base's. Exit 1: a typed refusal about the file. Exit 2: the
// check could not run (a bad argument, or git failed), which is never reported as a pass or as a refusal about the file.
import { execFileSync } from "node:child_process";
import { existsSync, realpathSync } from "node:fs";
import { fileURLToPath } from "node:url";

export const RELEASE_DATE_FILE = "src-tauri/release-date.txt";

// The typed reasons a version pull request is refused. Each message names the file.
export const REFUSALS = {
  missing: `${RELEASE_DATE_FILE} is missing from the version pull request; run \`node scripts/next-version.mjs --apply\``,
  malformed: `${RELEASE_DATE_FILE} must hold one line, a real YYYY-MM-DD day, ending in a single newline`,
  unchanged: `${RELEASE_DATE_FILE} is the same as the base's; a version pull request must set it to a later release day (\`node scripts/next-version.mjs --apply\` does, but not on the base's own UTC day: wait for the next day)`,
  earlier: `${RELEASE_DATE_FILE} names a day earlier than the base's; check the computer's clock and run \`node scripts/next-version.mjs --apply\` again (a revert of a version pull request needs a fresh, later date too)`,
};

export class ReleaseDateRefusal extends Error {
  constructor(code) {
    super(REFUSALS[code]);
    this.name = "ReleaseDateRefusal";
    this.code = code;
  }
}

// Parse once at the edge: the day, or null when the text is not exactly `YYYY-MM-DD\n` naming a real day.
export function parseReleaseDate(text) {
  if (typeof text !== "string" || !/^\d{4}-\d{2}-\d{2}\n$/.test(text)) return null;
  const day = text.slice(0, 10);
  const parsed = new Date(`${day}T00:00:00Z`);
  return Number.isNaN(parsed.getTime()) || parsed.toISOString().slice(0, 10) !== day ? null : day;
}

// base and head are the file's text on each side, or null when the file is absent there. A base that is absent or
// not a real day is no predecessor (it is already merged), so only the head is checked against it.
export function checkReleaseDateChange({ base, head }) {
  if (head === null) throw new ReleaseDateRefusal("missing");
  const headDay = parseReleaseDate(head);
  if (headDay === null) throw new ReleaseDateRefusal("malformed");
  // The base is read leniently (a missing final newline still names its day): it is already merged, and a strict read
  // would let a same-day head pass as having no predecessor.
  const baseDay = base === null ? null : parseReleaseDate(`${base.trim()}\n`);
  if (baseDay === null) return { baseDay, headDay };
  if (headDay === baseDay) throw new ReleaseDateRefusal("unchanged");
  if (headDay < baseDay) throw new ReleaseDateRefusal("earlier");
  return { baseDay, headDay };
}

// The file's text at a commit, or null when the commit has no such file. `git ls-tree` exits 0 with no output for an
// absent path and non-zero for a bad commit, so "absent" is told apart from "git failed" (which throws).
function textAt(sha) {
  const listed = execFileSync("git", ["ls-tree", "--full-tree", sha, "--", RELEASE_DATE_FILE], { encoding: "utf8" });
  if (listed === "") return null;
  return execFileSync("git", ["show", `${sha}:${RELEASE_DATE_FILE}`], { encoding: "utf8" });
}

export function main(args) {
  if (args.length !== 2 || !args.every((sha) => /^[0-9a-f]{40}$/.test(sha))) {
    console.error("usage: node scripts/check-release-date-changed.mjs <40-hex base sha> <40-hex head sha>");
    return 2;
  }
  let base;
  let head;
  try {
    base = textAt(args[0]);
    head = textAt(args[1]);
  } catch (error) {
    console.error(`::error::release-date: could not read ${RELEASE_DATE_FILE} from git (${String(error.message).split("\n")[0]}); this is not a verdict on the file`);
    return 2;
  }
  try {
    const { baseDay, headDay } = checkReleaseDateChange({ base, head });
    console.log(`release-date: ok, ${baseDay ?? "no base date"} -> ${headDay}`);
    return 0;
  } catch (error) {
    if (!(error instanceof ReleaseDateRefusal)) throw error;
    console.error(`::error file=${RELEASE_DATE_FILE}::release-date: ${error.code}: ${error.message}`);
    return 1;
  }
}

if (process.argv[1] && existsSync(process.argv[1]) && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url))) process.exitCode = main(process.argv.slice(2));
