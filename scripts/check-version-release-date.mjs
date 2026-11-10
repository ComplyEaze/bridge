// SPDX-License-Identifier: Apache-2.0
//
// A version pull request (the one `scripts/next-version.mjs --apply` writes, recognised in
// .github/workflows/ci.yml by a changed version in src-tauri/tauri.conf.json) must also move
// src-tauri/release-date.txt (#2416). The file is compiled into the program (#2391) and a build whose
// date is stale tells a new user their fresh build is old. A release cut by hand, or from a pull request
// that dropped the file, carries the previous release's date.
//
//   node scripts/check-version-release-date.mjs BASE_REV HEAD_REV     (run from the repository root)
//
// The rule is next-version.mjs's own (checkReleaseDate: a real YYYY-MM-DD day, not before the base's day,
// not after today in UTC), plus one thing it cannot know: the date is unchanged from the base. That is
// allowed only on the day the base's date names, because a second release on the same UTC day writes the
// same date; on any later day an unchanged date is the previous release's date carried over.
import { execFileSync } from "node:child_process";
import { existsSync, realpathSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { checkReleaseDate, RELEASE_DATE_FILE, utcDay } from "./next-version.mjs";

const FIX = "run node scripts/next-version.mjs --apply in the version pull request";

// baseText / headText: the file's text at each revision, or null when the file is absent there.
// Returns the head's day; throws an Error whose message is the one line to show.
export function checkVersionReleaseDate({ baseText, headText, today = utcDay() }) {
  if (headText === null) throw new Error(`${RELEASE_DATE_FILE} is missing in this version pull request; ${FIX}`);
  const head = headText.replace(/\n$/, "");
  let base;
  if (baseText !== null) {
    base = baseText.replace(/\n$/, "");
    try {
      checkReleaseDate(base, { today: "9999-12-31" });
    } catch {
      base = undefined; // a base that is not a day gives nothing to compare with; the head is still checked
    }
  }
  try {
    checkReleaseDate(head, { tagDay: base, today });
  } catch (error) {
    throw new Error(`${RELEASE_DATE_FILE}: ${error.message}; ${FIX}`);
  }
  if (base !== undefined && head === base && head !== today) {
    throw new Error(`${RELEASE_DATE_FILE} still names ${head}, the day of the previous release, and today is ${today} (UTC); ${FIX}`);
  }
  return head;
}

// The file's text at `rev`, or null when `rev` has no such file. An unreadable revision is an error,
// never "absent": `git ls-tree` names the path when it exists and prints nothing when it does not.
function textAt(rev) {
  const git = (args) => execFileSync("git", args, { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"], timeout: 60_000 });
  const listed = git(["ls-tree", "--name-only", rev, "--", RELEASE_DATE_FILE]);
  if (listed.trim() === "") return null;
  return git(["show", `${rev}:${RELEASE_DATE_FILE}`]);
}

if (process.argv[1] && existsSync(process.argv[1]) && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url))) {
  const [base, head, ...rest] = process.argv.slice(2);
  if (!base || !head || rest.length) {
    console.error("usage: node scripts/check-version-release-date.mjs BASE_REV HEAD_REV");
    process.exit(2);
  }
  try {
    const day = checkVersionReleaseDate({ baseText: textAt(base), headText: textAt(head) });
    console.log(`${RELEASE_DATE_FILE}: ${day}`);
  } catch (error) {
    console.error(`::error file=${RELEASE_DATE_FILE}::${error.message}`);
    process.exit(1);
  }
}
