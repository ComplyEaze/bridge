// SPDX-License-Identifier: Apache-2.0
//
// `.cargo/audit.toml` cannot hold a review date as a field (cargo-audit refuses unknown keys), so each ignore entry
// carries `Review-by: YYYY-MM-DD` in its comment. `cargo audit` prints nothing for an ignored id, and the scheduled
// audit's tracking issue only lists the date (scripts/audit-tracking-issue.mjs), so without this test an accepted
// advisory would stay accepted after the day it was to be looked at again. Once a date has passed the test fails for everyone, which is the point: re-run the entry's checks, then
// either remove the entry or renew its date with the new finding written in its tracking issue (#2432 for
// RUSTSEC-2026-0459). It reads dates only; whether the reason still holds is a human's call.
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const auditToml = join(dirname(fileURLToPath(import.meta.url)), "..", ".cargo", "audit.toml");
const REVIEW_BY = /^\s*#\s*Review-by:\s*(\d{4}-\d{2}-\d{2})\b/gm;

// Every Review-by day in the text, as { day, line }. A day that is not a real date is returned as-is so the caller refuses it.
export function reviewDates(text) {
  return [...text.matchAll(REVIEW_BY)].map((m) => ({ day: m[1], line: text.slice(0, m.index).split("\n").length }));
}

export function expired(text, today) {
  const real = (day) => new Date(`${day}T00:00:00Z`).toISOString().slice(0, 10) === day;
  return reviewDates(text).filter(({ day }) => !real(day) || day < today);
}

test("no Review-by date in .cargo/audit.toml has passed", () => {
  const today = new Date().toISOString().slice(0, 10);
  const late = expired(readFileSync(auditToml, "utf8"), today);
  assert.deepEqual(late, [], `a Review-by date has passed (today is ${today} UTC): re-run the entry's checks, then remove it or renew its date with the new finding written in its tracking issue`);
});

test("every ignore entry that is an error carries a Review-by date in the file", () => {
  const text = readFileSync(auditToml, "utf8");
  const block = text.slice(text.indexOf("RUSTSEC-2026-0459 --"));
  assert.match(block.split('"RUSTSEC-2026-0459"')[0], /Review-by: \d{4}-\d{2}-\d{2}/, "RUSTSEC-2026-0459 must keep its Review-by date");
});

test("the check finds a passed date, the day itself still passes, and an impossible date is refused", () => {
  const entry = (day) => `  # Review-by: ${day} (30 days)\n  "RUSTSEC-0000-0000",\n`;
  assert.deepEqual(expired(entry("2026-12-09"), "2026-12-09"), [], "the review-by day itself is not late");
  assert.deepEqual(expired(entry("2026-12-09"), "2026-12-10"), [{ day: "2026-12-09", line: 1 }]);
  assert.deepEqual(expired(entry("2026-02-30"), "2026-01-01"), [{ day: "2026-02-30", line: 1 }]);
  assert.deepEqual(expired("# no date here\n", "2030-01-01"), []);
  const two = entry("2026-11-01") + entry("2027-01-01");
  assert.deepEqual(expired(two, "2026-12-01").map((x) => x.day), ["2026-11-01"]);
});

test("the real file has dates to check, so the first test cannot pass by finding none", () => {
  const found = reviewDates(readFileSync(auditToml, "utf8"));
  assert.ok(found.length >= 5, `expected the file's Review-by lines, found ${found.length}`);
  assert.ok(found.some(({ day }) => day === "2026-12-09"));
});
