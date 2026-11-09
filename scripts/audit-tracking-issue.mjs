// SPDX-License-Identifier: Apache-2.0
//
// Keeps exactly one open tracking issue in step with what `cargo audit` finds
// on the default branch, for the scheduled audit in
// `.github/workflows/dependency-security-scheduled.yml`.
//
// Why a scheduled audit needs this at all: on 2026-09-15 RUSTSEC-2026-0285 was
// published against `rustls` and nobody knew until the next person to open a
// pull request found the pipeline frozen. An advisory published against
// unchanged code is not a pull request's fault and cannot be discovered by
// anything that only runs on pull requests -- it needs a job that looks at the
// default branch on a clock. That job then has to put its finding somewhere a
// person will see, which is what this script does.
//
// The hard part is not opening an issue, it is not opening one every day. An
// advisory commonly stays unresolved for days while an upgrade is
// investigated, so "file a finding" naively becomes a daily duplicate. This
// script therefore reconciles rather than reports: one open issue per
// repository, found by a hidden marker, updated only when the finding SET
// changes, and closed when the findings are gone. A run that changes nothing
// is the normal case and must stay silent.
//
// The issue also lists every id `.cargo/audit.toml` ignores (#2446). An ignored
// id is absent from the findings, so without that list an accepted
// vulnerability would be invisible until its Review-by date failed
// scripts/audit-review-by.test.mjs. The ids come from the report's own
// `settings.ignore`, which is what cargo-audit applied; the file adds each
// entry's Review-by date and issue number, and the two must name the same ids.
// The issue therefore stays open while anything is accepted, and a newly
// accepted id, a removed one or a changed date is announced once.
//
// Split deliberately into a pure planner and a thin applier: the planner is
// what the tests exercise, so the reconciliation rules are provable without a
// network, a GitHub token, or a Rust toolchain -- `pnpm test` runs every
// scripts/*.test.mjs in the "Frontend build" CI job, which has none of those.

import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";

// A hidden marker rather than a title match: titles get edited by people, and
// a tracking issue whose title someone improved must still be found.
export const MARKER = "<!-- dependency-security-tracker -->";

// The label already exists in this repository. A new one would have to be
// created as a side effect of the first run, which is a repository
// configuration change this script has no business making on its own.
const LABEL = "area:security";

const TITLE = "Dependency advisories on master";

// Mirrors `idsAndDetails` in check-advisory-delta.mjs deliberately rather than
// importing it: that module runs `main()` at import time, so importing it here
// would run a full audit as a side effect. Keep the two in step -- especially
// the synthetic key, which exists because a yanked-crate warning carries no
// advisory at all and would otherwise be invisible to a set comparison.
//
// A report without a `vulnerabilities.list` array and a `warnings` object is
// unreadable, not clean: read as "no findings", it would announce a standing
// advisory as gone (#2446). Every real report carries both (the captured ones
// under scripts/fixtures do); a warning kind that is absent is still none.
export function findingsFromReport(report) {
  const warnings = report?.warnings;
  if (
    !Array.isArray(report?.vulnerabilities?.list) ||
    warnings === null ||
    typeof warnings !== "object" ||
    Array.isArray(warnings)
  ) {
    throw new Error(
      "audit-tracking-issue: the report has no vulnerabilities.list array or no warnings object, so its findings " +
        "cannot be read; refusing rather than reading it as no findings",
    );
  }
  const findings = new Map();
  const record = (id, detail) => {
    if (!findings.has(id)) findings.set(id, detail);
  };
  for (const vulnerability of report.vulnerabilities.list) {
    record(vulnerability.advisory.id, {
      id: vulnerability.advisory.id,
      package: `${vulnerability.package.name}@${vulnerability.package.version}`,
      summary: vulnerability.advisory.title,
      kind: "vulnerability",
    });
  }
  for (const kind of Object.keys(warnings)) {
    for (const warning of warnings[kind] ?? []) {
      const id =
        warning.advisory?.id ?? `${kind}:${warning.package.name}@${warning.package.version}`;
      record(id, {
        id,
        package: `${warning.package.name}@${warning.package.version}`,
        summary: warning.advisory?.title ?? kind,
        kind,
      });
    }
  }
  return [...findings.values()].sort((a, b) => a.id.localeCompare(b.id));
}

// Reports can come from more than one Rust lockfile (the app's is `src-tauri/Cargo.lock`; there was
// a second, the tools workspace's, until #1745 removed it), and arrive for one tracking issue.
// `cargo audit --json` does not name the lockfile it
// read, so each report arrives paired with the lockfile it came from.
//
// Ids stay exactly what `findingsFromReport` produces. The same advisory in both
// lockfiles is one finding naming both places, not two ids -- adding a lockfile
// to the id would change the recorded set of an already-tracked advisory and
// announce a change that moved nothing.
export function findingsFromReports(entries) {
  const merged = new Map();
  for (const { lockfile, report } of entries) {
    for (const finding of findingsFromReport(report)) {
      const where = lockfile ? `${finding.package} in \`${lockfile}\`` : finding.package;
      const current = merged.get(finding.id);
      if (!current) merged.set(finding.id, { ...finding, package: where });
      else if (!current.package.split(", ").includes(where)) current.package = `${current.package}, ${where}`;
    }
  }
  return [...merged.values()].sort((a, b) => a.id.localeCompare(b.id));
}

// `--report <lockfile>=<report.json>` pairs a report with its lockfile; a bare
// `--report <report.json>` is accepted unlabelled. Split on the first `=` only,
// so the separator cannot be confused with anything later in the report path.
export function parseReportArgument(value) {
  const separator = value.indexOf("=");
  if (separator === -1) return { lockfile: null, path: value };
  const lockfile = value.slice(0, separator);
  const path = value.slice(separator + 1);
  if (!lockfile || !path) {
    throw new Error(`audit-tracking-issue: --report expects <lockfile>=<report.json>, got: ${value}`);
  }
  return { lockfile, path };
}

// The recorded set is written into the issue body on its own line so a later
// run can read back exactly what the last run saw. Comparing against the
// rendered prose instead would make an editorial change to the issue look
// like a change in the findings.
const RECORDED_PREFIX = "Tracked advisory ids:";

// JSON rather than a comma-separated list. `advisory.id` is free text from the
// community-run RustSec database, and an id containing a comma or a backtick
// round-trips wrong through a hand-rolled encoding -- it reads back as two ids,
// the set compares unequal forever, and the issue collects a "changed" comment
// every single day. That is exactly the noise this script exists to prevent, so
// the encoding may not be the thing that reintroduces it.
export function recordedIds(body) {
  const line = (body ?? "")
    .split("\n")
    .map((entry) => entry.trim())
    .find((entry) => entry.startsWith(RECORDED_PREFIX));
  if (!line) return [];
  try {
    const parsed = JSON.parse(line.slice(RECORDED_PREFIX.length).trim());
    if (!Array.isArray(parsed)) return [];
    return parsed.filter((id) => typeof id === "string").sort();
  } catch {
    // An unreadable line is reported as no ids, which `planAction` treats as a
    // changed set. That rewrites the body and repairs the line, rather than
    // silently believing the issue is up to date.
    return [];
  }
}

// The ids cargo-audit applied, from the report it wrote. `cargo audit --json`
// records its configuration under `settings` (measured with cargo-audit 0.22.2
// on 2026-11-09: `settings.ignore` held exactly the ten ids of master's file,
// eleven with RUSTSEC-2026-0459 added). A report without that list is refused:
// read as "nothing accepted", it would drop every accepted id from the issue,
// which is the silence this list exists to end.
export function acceptedFromReport(report) {
  const ignore = report?.settings?.ignore;
  if (!Array.isArray(ignore) || !ignore.every((id) => typeof id === "string" && id.trim() !== "")) {
    throw new Error(
      "audit-tracking-issue: the report has no settings.ignore list of advisory ids, so what " +
        "cargo-audit accepted cannot be read; refusing rather than posting an empty accepted list",
    );
  }
  return [...new Set(ignore)].sort();
}

// A report file, parsed or refused. An empty or unparseable file is a failed
// audit, not a clean one; the workflow refuses an empty file too, and this
// says the same for any caller.
export function parseReport(path, text) {
  if (!text || !text.trim()) {
    throw new Error(`audit-tracking-issue: ${path} is empty -- treating it as a failed audit, not a clean one`);
  }
  try {
    return JSON.parse(text);
  } catch (error) {
    throw new Error(`audit-tracking-issue: ${path} is not JSON (${error.message}) -- treating it as a failed audit`);
  }
}

const IGNORE_OPEN = /^ignore\s*=\s*\[\s*$/;
// An advisory id in the characters RustSec ids use, so an id cannot carry
// Markdown or a mention into the issue.
const ID_LINE = /^"([A-Za-z0-9._:-]+)",?$/;
const REVIEW_BY_LINE = /^#\s*Review-by:\s*(\d{4}-\d{2}-\d{2})\b/;
// `#2432`, but not `bridge#445` or a URL fragment.
const ISSUE_REFERENCE = /(?<![A-Za-z0-9_/])#(\d{1,7})\b/;

// Each id in `.cargo/audit.toml`'s `ignore` array, with the Review-by date and
// first issue number from the comment block above it. An id with no comment
// line of its own between it and the id before it shares that block, and is
// marked `shared`, so the issue says so rather than showing it as separately
// dated and tracked. Only those two values leave the comment, so no comment
// text reaches the issue. A line the walk does not recognise
// inside the array is refused rather than skipped, so an id cannot go
// unlisted because it was written in another form.
export function entriesFromAuditToml(text) {
  const lines = text.split("\n").map((line) => line.trim());
  const open = lines.findIndex((line) => IGNORE_OPEN.test(line));
  if (open === -1) throw new Error("audit-tracking-issue: .cargo/audit.toml has no `ignore = [` line");
  const entries = [];
  let pending = [];
  let block = [];
  for (let i = open + 1; i < lines.length; i += 1) {
    const line = lines[i];
    if (line === "]") {
      const ids = entries.map((entry) => entry.id);
      const repeated = ids.filter((id, index) => ids.indexOf(id) !== index);
      if (repeated.length) {
        throw new Error(`audit-tracking-issue: .cargo/audit.toml ignores ${[...new Set(repeated)].join(", ")} more than once`);
      }
      return entries.sort((a, b) => a.id.localeCompare(b.id));
    }
    if (line === "") continue;
    if (line.startsWith("#")) {
      pending.push(line);
      continue;
    }
    const id = ID_LINE.exec(line);
    if (!id) {
      throw new Error(`audit-tracking-issue: .cargo/audit.toml line ${i + 1} is not an id or a comment inside \`ignore\`: ${line}`);
    }
    const shared = pending.length === 0 && entries.length > 0;
    if (pending.length) {
      block = pending;
      pending = [];
    }
    const reviewBy = block.map((entry) => REVIEW_BY_LINE.exec(entry)).find(Boolean);
    const issue = block.map((entry) => ISSUE_REFERENCE.exec(entry)).find(Boolean);
    entries.push({
      id: id[1],
      review_by: reviewBy ? reviewBy[1] : null,
      issue: issue ? Number(issue[1]) : null,
      shared,
    });
  }
  throw new Error("audit-tracking-issue: .cargo/audit.toml's `ignore` array is not closed");
}

// The accepted entries for the issue: the file's entries, once every report's
// `settings.ignore` names exactly the same ids. A difference either way means
// the file the job read is not the file cargo-audit read (the wrong working
// directory reads no file at all), so it fails the job and names both sides.
export function acceptedEntries(reports, auditTomlText) {
  const entries = entriesFromAuditToml(auditTomlText);
  const inFile = entries.map((entry) => entry.id);
  for (const report of reports) {
    const inReport = acceptedFromReport(report);
    const onlyInFile = inFile.filter((id) => !inReport.includes(id));
    const onlyInReport = inReport.filter((id) => !inFile.includes(id));
    if (onlyInFile.length || onlyInReport.length) {
      throw new Error(
        "audit-tracking-issue: .cargo/audit.toml and the report's settings.ignore disagree " +
          `(only in the file: ${onlyInFile.join(", ") || "none"}; only in the report: ${onlyInReport.join(", ") || "none"})`,
      );
    }
  }
  return entries;
}

// The accepted entries as last written, from their own recorded line. JSON for
// the same reason as the ids line. An absent or unreadable line reads as none,
// so the run after a body without it (an issue written before #2446) announces
// every accepted id once and writes the line.
const ACCEPTED_PREFIX = "Accepted advisory entries:";

export function recordedAccepted(body) {
  const line = (body ?? "")
    .split("\n")
    .map((entry) => entry.trim())
    .find((entry) => entry.startsWith(ACCEPTED_PREFIX));
  if (!line) return [];
  try {
    const parsed = JSON.parse(line.slice(ACCEPTED_PREFIX.length).trim());
    if (!Array.isArray(parsed)) return [];
    return parsed
      .filter(
        (row) =>
          Array.isArray(row) &&
          row.length === 4 &&
          typeof row[3] === "boolean" &&
          typeof row[0] === "string" &&
          (row[1] === null || (typeof row[1] === "string" && /^\d{4}-\d{2}-\d{2}$/.test(row[1]))) &&
          (row[2] === null || (Number.isInteger(row[2]) && row[2] > 0)),
      )
      .map(([id, review_by, issue, shared]) => ({ id, review_by, issue, shared }))
      .sort((a, b) => a.id.localeCompare(b.id));
  } catch {
    return [];
  }
}

const entryRow = (entry) => [entry.id, entry.review_by, entry.issue, entry.shared];
const entryKey = (entry) => JSON.stringify(entryRow(entry));

// What moved between the accepted entries last written and today's.
function acceptedMoves(recorded, accepted) {
  const before = new Map(recorded.map((entry) => [entry.id, entry]));
  const after = new Map(accepted.map((entry) => [entry.id, entry]));
  return {
    newlyAccepted: accepted.filter((entry) => !before.has(entry.id)),
    noLongerAccepted: recorded.filter((entry) => !after.has(entry.id)).map((entry) => entry.id),
    changed: accepted
      .filter((entry) => before.has(entry.id) && entryKey(before.get(entry.id)) !== entryKey(entry))
      .map((entry) => ({ id: entry.id, from: before.get(entry.id), to: entry })),
  };
}

// No default: a caller that leaves the accepted entries out would read as
// "nothing accepted" and close the issue (#2446). It throws instead.
function requireAccepted(accepted, caller) {
  if (!Array.isArray(accepted)) {
    throw new Error(`audit-tracking-issue: ${caller} needs the accepted entries (an array, empty only when nothing is accepted)`);
  }
}

/// The whole contract, in one pure function.
///
/// `none` and `unchanged` are distinct on purpose: both do nothing, but the
/// first means "clean, nothing tracked" and the second means "still broken,
/// already tracked". Collapsing them would hide a standing advisory behind the
/// same silence as a clean run.
///
/// `accepted` is the list of `acceptedEntries`, and required. The issue closes only when
/// there are no findings AND nothing is accepted: an accepted id is the one
/// thing the findings cannot show.
export function planAction(findings, existing, accepted) {
  requireAccepted(accepted, "planAction");
  const ids = findings.map((finding) => finding.id).sort();
  const entries = [...accepted].sort((a, b) => a.id.localeCompare(b.id));
  if (ids.length === 0 && entries.length === 0) {
    return existing ? { action: "close", issue: existing.number } : { action: "none" };
  }
  if (!existing) return { action: "create", ids, findings, accepted: entries };
  const recorded = [...(existing.ids ?? [])].sort();
  const recordedEntries = [...(existing.accepted ?? [])].sort((a, b) => a.id.localeCompare(b.id));
  const sameIds = recorded.length === ids.length && recorded.every((id, i) => id === ids[i]);
  const sameAccepted =
    recordedEntries.length === entries.length &&
    recordedEntries.every((entry, i) => entryKey(entry) === entryKey(entries[i]));
  if (sameIds && sameAccepted) return { action: "unchanged", issue: existing.number, ids };
  const added = ids.filter((id) => !recorded.includes(id));
  const removed = recorded.filter((id) => !ids.includes(id));
  const moves = acceptedMoves(recordedEntries, entries);
  // A recorded line that merely repeats an id differs as a list while naming
  // the same set. Rewriting the body is right; announcing a change that moved
  // nothing is not.
  if (
    added.length === 0 &&
    removed.length === 0 &&
    moves.newlyAccepted.length === 0 &&
    moves.noLongerAccepted.length === 0 &&
    moves.changed.length === 0
  ) {
    return { action: "repair", issue: existing.number, ids, findings, accepted: entries };
  }
  return { action: "update", issue: existing.number, ids, findings, added, removed, accepted: entries, ...moves };
}

const describeEntry = (entry) =>
  `review-by ${entry.review_by ?? "none in the file"}, ${entry.issue ? `#${entry.issue}` : "no issue named in the file"}` +
  (entry.shared ? " (shares the entry above)" : "");

// The update comment: every move, each named once.
export function updateComment(plan) {
  const changed = [
    plan.added?.length ? `newly reported: ${plan.added.join(", ")}` : null,
    plan.removed?.length ? `no longer reported: ${plan.removed.join(", ")}` : null,
    plan.newlyAccepted?.length
      ? `newly accepted in .cargo/audit.toml: ${plan.newlyAccepted
          .map((entry) => (entry.shared ? `${entry.id} (shares the entry above)` : entry.id))
          .join(", ")}`
      : null,
    plan.noLongerAccepted?.length ? `no longer accepted: ${plan.noLongerAccepted.join(", ")}` : null,
    ...(plan.changed ?? []).map(
      ({ id, from, to }) => `accepted entry changed: ${id} (${describeEntry(from)} -> ${describeEntry(to)})`,
    ),
  ]
    .filter(Boolean)
    .join("; ");
  return `Scheduled audit update -- ${changed}.`;
}

export function issueBody(findings, accepted) {
  requireAccepted(accepted, "issueBody");
  const lines = findings.map(
    (finding) => `- \`${finding.id}\` (${finding.kind}) ${finding.package} -- ${finding.summary}`,
  );
  const entries = [...accepted].sort((a, b) => a.id.localeCompare(b.id));
  const acceptedLines = entries.map((entry) => `- \`${entry.id}\` -- ${describeEntry(entry)}`);
  return [
    MARKER,
    "",
    "`cargo audit` reports the following on the default branch's Rust",
    "lockfiles, after `.cargo/audit.toml`'s ignore list is applied.",
    "",
    ...(lines.length ? lines : ["- none"]),
    "",
    "Accepted in `.cargo/audit.toml` (ignored, so `cargo audit` does not report",
    "them above), as `cargo audit` applied them on this run:",
    "",
    ...(acceptedLines.length ? acceptedLines : ["- none"]),
    "",
    "This issue is maintained by the scheduled dependency audit. It is updated",
    "when the findings or the accepted entries change, and closed automatically",
    "when there are no findings and nothing is accepted. Edit the title or add",
    "comments freely.",
    "",
    "Three things are load-bearing for finding this issue again: the marker",
    `above, the recorded-ids line below, and the \`${LABEL}\` label -- the next`,
    "run looks for the marker only among open issues carrying that label.",
    "Remove any of the three and the next run opens a duplicate and leaves this",
    "one orphaned.",
    "",
    `${RECORDED_PREFIX} ${JSON.stringify(findings.map((finding) => finding.id).sort())}`,
    `${ACCEPTED_PREFIX} ${JSON.stringify(entries.map(entryRow))}`,
  ].join("\n");
}

function gh(args) {
  const result = spawnSync("gh", args, { encoding: "utf8", maxBuffer: 32 * 1024 * 1024 });
  if (result.status !== 0) {
    throw new Error(`audit-tracking-issue: gh ${args.join(" ")} failed: ${result.stderr?.trim()}`);
  }
  return result.stdout;
}

// `run` is injected so the GitHub-touching half is testable without a token or
// a network. It is the half the reconciliation invariant actually lives in, so
// leaving it untestable would mean the tests cover the arithmetic and not the
// thing that can file a duplicate issue.
//
// `gh issue list` rather than `gh api .../issues`: the repository's
// `check-gh-api-pagination` gate exempts gh's own list subcommands because
// their paging is gh-owned, and an explicit --limit says what this one expects.
export function findExisting(run = gh) {
  const issues = JSON.parse(
    run([
      "issue",
      "list",
      "--state",
      "open",
      "--label",
      LABEL,
      "--limit",
      "100",
      "--json",
      "number,body",
    ]),
  );
  const matches = issues.filter((issue) => (issue.body ?? "").includes(MARKER));
  // "Exactly one open issue" is this script's whole contract. Picking the first
  // of several would honour it silently and wrongly: the others would never be
  // updated and never closed, and nothing would ever say so. Refuse instead --
  // a human duplicated something, and a human should decide which survives.
  if (matches.length > 1) {
    throw new Error(
      `audit-tracking-issue: ${matches.length} open issues carry the tracker marker ` +
        `(${matches.map((issue) => `#${issue.number}`).join(", ")}). ` +
        "Close all but one, then re-run -- this script will not choose for you.",
    );
  }
  const match = matches[0];
  return match
    ? { number: match.number, ids: recordedIds(match.body), accepted: recordedAccepted(match.body) }
    : null;
}

export function apply(plan, run = gh) {
  if (plan.action === "none" || plan.action === "unchanged") return;
  if (plan.action === "create") {
    run(["issue", "create", "--title", TITLE, "--label", LABEL, "--body", issueBody(plan.findings, plan.accepted)]);
    return;
  }
  if (plan.action === "repair") {
    // The set is the same; only the recorded line was malformed. Rewrite it
    // and say nothing, so a cosmetic repair does not read as a new finding.
    run(["issue", "edit", String(plan.issue), "--body", issueBody(plan.findings, plan.accepted)]);
    return;
  }
  if (plan.action === "update") {
    run(["issue", "edit", String(plan.issue), "--body", issueBody(plan.findings, plan.accepted)]);
    run(["issue", "comment", String(plan.issue), "--body", updateComment(plan)]);
    return;
  }
  // Close first, then comment. The other order leaves a "Closing" comment on an
  // issue that is still open if the close fails, and the next run recomputes
  // the same plan and leaves another -- one redundant comment per day for as
  // long as the failure lasts. Closing first means a failed comment leaves the
  // issue in the state that was wanted, and the next run sees nothing open and
  // does nothing.
  run(["issue", "close", String(plan.issue)]);
  run([
    "issue",
    "comment",
    String(plan.issue),
    "--body",
    "Scheduled audit reports no findings and nothing accepted on the default branch. Closed automatically.",
  ]);
}

// `--audit-config` is required in both modes, for the same reason the preview
// queries the issue: without the file there are no accepted entries to compare,
// and a run that left them out would announce every accepted id as no longer
// accepted.
export function parseArguments(argv) {
  const reports = [];
  let shouldApply = false;
  let auditConfig = null;
  for (let i = 0; i < argv.length; i += 1) {
    if (argv[i] === "--report") {
      if (i + 1 >= argv.length) throw new Error("audit-tracking-issue: --report needs a value");
      reports.push(parseReportArgument(argv[++i]));
    } else if (argv[i] === "--audit-config") {
      if (i + 1 >= argv.length) throw new Error("audit-tracking-issue: --audit-config needs a value");
      auditConfig = argv[++i];
    } else if (argv[i] === "--apply") shouldApply = true;
    else throw new Error(`audit-tracking-issue: unknown argument: ${argv[i]}`);
  }
  if (reports.length === 0) throw new Error("audit-tracking-issue: at least one --report is required");
  if (!auditConfig) {
    throw new Error("audit-tracking-issue: --audit-config <path to .cargo/audit.toml> is required");
  }
  return { reports, shouldApply, auditConfig };
}

function main() {
  const { reports, shouldApply, auditConfig } = parseArguments(process.argv.slice(2));

  const loaded = reports.map(({ lockfile, path }) => ({
    lockfile,
    report: parseReport(path, readFileSync(path, "utf8")),
  }));
  const findings = findingsFromReports(loaded);
  const accepted = acceptedEntries(
    loaded.map(({ report }) => report),
    readFileSync(auditConfig, "utf8"),
  );
  // Queried in both modes. A preview that assumed no issue existed could only
  // ever print "create" or "none", which is not a preview of what --apply would
  // do -- it is a different answer that happens to share a format.
  const plan = planAction(findings, findExisting(), accepted);
  process.stdout.write(`${JSON.stringify(plan, null, 2)}\n`);
  if (shouldApply) apply(plan);
}

if (process.argv[1] && process.argv[1].endsWith("audit-tracking-issue.mjs")) main();
