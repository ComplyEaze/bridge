// SPDX-License-Identifier: Apache-2.0
//
// Contract tests for scripts/audit-tracking-issue.mjs.
//
// The failure this file exists to prevent is not "the issue is wrong". It is
// the scheduled audit filing a fresh issue every single day an advisory stays
// unresolved, which is what a reconciler does the moment it can no longer read
// back what its previous run wrote. So the load-bearing test here is the body
// round trip: `recordedIds(issueBody(findings))` must return exactly the ids
// that went in. Reformat the issue body without keeping that true and this
// suite fails rather than the repository quietly accumulating duplicates.
//
// Everything is pure -- no gh, no network, no Rust toolchain. `pnpm test`
// globs scripts/*.test.mjs in the "Frontend build" CI job, which has none.

import assert from "node:assert/strict";
import test from "node:test";

import {
  MARKER,
  findingsFromReport,
  findingsFromReports,
  issueBody,
  parseReportArgument,
  planAction,
  recordedIds,
} from "./audit-tracking-issue.mjs";

const vuln = (id, name, version, title) => ({
  advisory: { id, title },
  package: { name, version },
});

const REPORT = {
  vulnerabilities: {
    found: true,
    list: [vuln("RUSTSEC-2026-0285", "rustls", "0.23.43", "TLS 1.3 handshake messages")],
  },
  warnings: {
    unmaintained: [
      { advisory: { id: "RUSTSEC-2024-0001", title: "unmaintained crate" }, package: { name: "old", version: "1.0.0" } },
    ],
    // A yanked crate carries no advisory at all. Without a synthetic key it
    // would be invisible to a set comparison, so a yank appearing or being
    // resolved would never move the tracking issue.
    yanked: [{ package: { name: "chacha20", version: "0.9.0" } }],
  },
};

test("a report becomes a sorted, deduplicated finding set", () => {
  const findings = findingsFromReport(REPORT);
  assert.deepEqual(
    findings.map((finding) => finding.id),
    ["RUSTSEC-2024-0001", "RUSTSEC-2026-0285", "yanked:chacha20@0.9.0"],
  );
  assert.equal(findings.find((f) => f.id === "RUSTSEC-2026-0285").kind, "vulnerability");
});

test("a warning with no advisory still gets a stable synthetic id", () => {
  const findings = findingsFromReport({ vulnerabilities: { list: [] }, warnings: { yanked: [{ package: { name: "c", version: "1.2.3" } }] } });
  assert.deepEqual(findings.map((f) => f.id), ["yanked:c@1.2.3"]);
});

test("an empty report yields no findings", () => {
  assert.deepEqual(findingsFromReport({ vulnerabilities: { found: false, list: [] }, warnings: {} }), []);
});

// "No findings" and "the findings could not be read" must never look alike: the
// second, read as the first, announces a standing advisory as gone (#2446).
test("a report without a vulnerabilities list or a warnings object is refused, not read as clean", () => {
  const settings = { ignore: [] };
  for (const report of [
    { settings },
    { settings, vulnerabilities: null, warnings: {} },
    { settings, vulnerabilities: { found: false }, warnings: {} },
    { settings, vulnerabilities: { found: false, list: null }, warnings: {} },
    { settings, vulnerabilities: { found: false, list: [] } },
    { settings, vulnerabilities: { found: false, list: [] }, warnings: null },
    { settings, vulnerabilities: { found: false, list: [] }, warnings: [] },
    null,
  ]) {
    assert.throws(() => findingsFromReport(report), /no vulnerabilities\.list array or no warnings object/);
    assert.throws(
      () => findingsFromReports([{ lockfile: "src-tauri/Cargo.lock", report }]),
      /no vulnerabilities\.list array or no warnings object/,
    );
  }
  // An absent warning kind is still none.
  assert.deepEqual(findingsFromReport({ vulnerabilities: { list: [] }, warnings: { yanked: null } }), []);
});

// Two lockfiles, one tracker. RUSTSEC-2026-0285 sat unreported in
// tools/Cargo.lock because only src-tauri/Cargo.lock was audited.
const TOOLS_ONLY_REPORT = {
  vulnerabilities: { found: true, list: [vuln("RUSTSEC-2026-0285", "rustls", "0.23.43", "TLS 1.3 handshake messages")] },
  warnings: { yanked: [{ package: { name: "chacha20", version: "0.10.1" } }] },
};
const APP_ONLY_REPORT = {
  vulnerabilities: { found: false, list: [] },
  warnings: { yanked: [{ package: { name: "chacha20", version: "0.10.1" } }] },
};

test("a finding present only in the second lockfile is still reported", () => {
  const findings = findingsFromReports([
    { lockfile: "src-tauri/Cargo.lock", report: APP_ONLY_REPORT },
    { lockfile: "tools/Cargo.lock", report: TOOLS_ONLY_REPORT },
  ]);
  assert.deepEqual(findings.map((f) => f.id), ["RUSTSEC-2026-0285", "yanked:chacha20@0.10.1"]);
  assert.equal(findings[0].package, "rustls@0.23.43 in `tools/Cargo.lock`");
});

test("the same advisory in both lockfiles is one finding naming both", () => {
  const findings = findingsFromReports([
    { lockfile: "src-tauri/Cargo.lock", report: APP_ONLY_REPORT },
    { lockfile: "tools/Cargo.lock", report: TOOLS_ONLY_REPORT },
  ]);
  assert.equal(
    findings.find((f) => f.id === "yanked:chacha20@0.10.1").package,
    "chacha20@0.10.1 in `src-tauri/Cargo.lock`, chacha20@0.10.1 in `tools/Cargo.lock`",
  );
});

test("auditing a second lockfile does not change the ids of an already-tracked finding", () => {
  // The tracker was written from src-tauri alone. The chacha20 yank is in both
  // lockfiles; if adding tools/Cargo.lock renamed its id, the next run would
  // announce a change that moved nothing.
  const tracked = recordedIds(issueBody(findingsFromReport(APP_ONLY_REPORT), []));
  const both = findingsFromReports([
    { lockfile: "src-tauri/Cargo.lock", report: APP_ONLY_REPORT },
    { lockfile: "tools/Cargo.lock", report: APP_ONLY_REPORT },
  ]);
  assert.equal(planAction(both, { number: 445, ids: tracked }, []).action, "unchanged");
});

test("an unlabelled report keeps the single-lockfile rendering", () => {
  assert.deepEqual(findingsFromReports([{ lockfile: null, report: REPORT }]), findingsFromReport(REPORT));
});

test("--report pairs a lockfile with its report, or takes a bare path", () => {
  assert.deepEqual(parseReportArgument("tools/Cargo.lock=tools-audit.json"), {
    lockfile: "tools/Cargo.lock",
    path: "tools-audit.json",
  });
  assert.deepEqual(parseReportArgument("audit-report.json"), { lockfile: null, path: "audit-report.json" });
  assert.throws(() => parseReportArgument("=audit.json"), /<lockfile>=<report.json>/);
  assert.throws(() => parseReportArgument("tools/Cargo.lock="), /<lockfile>=<report.json>/);
});

// The one that matters: a body this script wrote must be readable by the next
// run. If this breaks, every scheduled run files a duplicate issue.
test("the issue body round-trips its recorded ids", () => {
  const findings = findingsFromReport(REPORT);
  const body = issueBody(findings, []);
  assert.ok(body.includes(MARKER), "the body must carry the marker used to find it again");
  assert.deepEqual(
    recordedIds(body),
    findings.map((f) => f.id).sort(),
  );
});

test("a body with no recorded-ids line reads as no ids, not a crash", () => {
  assert.deepEqual(recordedIds("nothing here"), []);
  assert.deepEqual(recordedIds(undefined), []);
});

test("clean run with nothing tracked does nothing", () => {
  assert.deepEqual(planAction([], null, []), { action: "none" });
});

test("clean run with something tracked closes it", () => {
  assert.deepEqual(planAction([], { number: 7, ids: ["RUSTSEC-2026-0285"] }, []), {
    action: "close",
    issue: 7,
  });
});

test("first finding opens an issue", () => {
  const findings = findingsFromReport(REPORT);
  const plan = planAction(findings, null, []);
  assert.equal(plan.action, "create");
  assert.deepEqual(plan.ids, findings.map((f) => f.id).sort());
});

test("an unchanged finding set is silent rather than a daily duplicate", () => {
  const findings = findingsFromReport(REPORT);
  const plan = planAction(findings, { number: 7, ids: findings.map((f) => f.id) }, []);
  assert.equal(plan.action, "unchanged");
  assert.equal(plan.issue, 7);
});

test("ordering alone is not a change", () => {
  const findings = findingsFromReport(REPORT);
  const reversed = findings.map((f) => f.id).reverse();
  assert.equal(planAction(findings, { number: 7, ids: reversed }, []).action, "unchanged");
});

test("a changed finding set updates and names what moved", () => {
  const findings = findingsFromReport(REPORT);
  const plan = planAction(findings, { number: 7, ids: ["RUSTSEC-2024-0001", "RUSTSEC-1999-0000"] }, []);
  assert.equal(plan.action, "update");
  assert.deepEqual(plan.added, ["RUSTSEC-2026-0285", "yanked:chacha20@0.9.0"]);
  assert.deepEqual(plan.removed, ["RUSTSEC-1999-0000"]);
});

test("a tracked issue whose ids cannot be read is treated as changed, not as clean", () => {
  const findings = findingsFromReport(REPORT);
  const plan = planAction(findings, { number: 7, ids: [] }, []);
  assert.equal(plan.action, "update", "an unreadable body must not silently look up to date");
});

// --- the GitHub-touching half -------------------------------------------
//
// This is where the invariant "exactly one open issue" actually lives, and
// where every duplicate-issue path found in review lived. `run` is injected so
// these need no token and no network; a fake records the argv it was handed.

import { apply, findExisting } from "./audit-tracking-issue.mjs";

const fakeRun = (issues) => {
  const calls = [];
  const run = (args) => {
    calls.push(args);
    if (args[0] === "issue" && args[1] === "list") return JSON.stringify(issues ?? []);
    return "";
  };
  run.calls = calls;
  return run;
};

const tracked = (number, ids) => ({
  number,
  body: `${MARKER}\nTracked advisory ids: ${JSON.stringify(ids)}`,
});

test("an advisory id containing a comma or a backtick still round-trips", () => {
  const findings = [
    { id: "RUSTSEC-2026-0001, and more", package: "p@1", summary: "s", kind: "vulnerability" },
    { id: "has`backtick", package: "q@2", summary: "s", kind: "vulnerability" },
  ];
  assert.deepEqual(recordedIds(issueBody(findings, [])), ["RUSTSEC-2026-0001, and more", "has`backtick"]);
});

test("a malformed recorded-ids line reads as no ids rather than throwing", () => {
  assert.deepEqual(recordedIds(`${MARKER}\nTracked advisory ids: {not json`), []);
  assert.deepEqual(recordedIds(`${MARKER}\nTracked advisory ids: "a string"`), []);
});

test("findExisting returns null when no open issue carries the marker", () => {
  assert.equal(findExisting(fakeRun([{ number: 1, body: "unrelated" }])), null);
});

test("findExisting reads the tracked ids off the matching issue", () => {
  const existing = findExisting(fakeRun([{ number: 2, body: "no marker" }, tracked(9, ["B", "A"])]));
  // A body written before #2446 has no accepted line, and reads as none accepted.
  assert.deepEqual(existing, { number: 9, ids: ["A", "B"], accepted: [] });
});

// The contract is "exactly one". Quietly picking the first of several would
// honour it wrongly: the rest are never updated and never closed.
test("findExisting refuses rather than choosing between duplicate trackers", () => {
  assert.throws(
    () => findExisting(fakeRun([tracked(9, ["A"]), tracked(10, ["A"])])),
    /2 open issues carry the tracker marker .*#9, #10/,
  );
});

test("findExisting scopes its query to open issues carrying the label", () => {
  const run = fakeRun([]);
  findExisting(run);
  const [args] = run.calls;
  assert.deepEqual(args.slice(0, 2), ["issue", "list"]);
  assert.ok(args.includes("--state") && args[args.indexOf("--state") + 1] === "open");
  assert.ok(args.includes("--label"), "discovery is label-scoped, which the issue body warns about");
  assert.ok(args.includes("--limit"), "an explicit limit, not gh's default");
});

// If the close fails, the wanted end state has not happened and the next run
// recomputes the same plan. Commenting first would leave a 'Closing' comment on
// a still-open issue, once per day, for as long as the failure lasts.
test("close happens before the comment that announces it", () => {
  const run = fakeRun([]);
  apply({ action: "close", issue: 9 }, run);
  assert.deepEqual(
    run.calls.map((args) => args[1]),
    ["close", "comment"],
  );
});

test("a repair rewrites the body and says nothing", () => {
  const run = fakeRun([]);
  apply({ action: "repair", issue: 9, ids: ["A"], findings: [{ id: "A", package: "p@1", summary: "s", kind: "vulnerability" }], accepted: [] }, run);
  assert.deepEqual(run.calls.map((args) => args[1]), ["edit"]);
});

test("an update edits the body and names what moved", () => {
  const run = fakeRun([]);
  apply(
    {
      action: "update",
      issue: 9,
      ids: ["A"],
      findings: [{ id: "A", package: "p@1", summary: "s", kind: "vulnerability" }],
      added: ["A"],
      removed: ["B"],
      accepted: [],
    },
    run,
  );
  assert.deepEqual(run.calls.map((args) => args[1]), ["edit", "comment"]);
  const comment = run.calls[1].at(-1);
  assert.match(comment, /newly reported: A/);
  assert.match(comment, /no longer reported: B/);
});

test("nothing to do means no gh calls at all", () => {
  for (const action of ["none", "unchanged"]) {
    const run = fakeRun([]);
    apply({ action, issue: 9 }, run);
    assert.equal(run.calls.length, 0, `${action} must be silent`);
  }
});

test("a recorded line naming the same set twice repairs without announcing a change", () => {
  const findings = [{ id: "A", package: "p@1", summary: "s", kind: "vulnerability" }];
  assert.equal(planAction(findings, { number: 9, ids: ["A", "A"] }, []).action, "repair");
});

// --- accepted entries (#2446) --------------------------------------------
//
// An ignored id is absent from the findings, so the issue lists what
// `.cargo/audit.toml` accepts. The ids come from the report's own
// `settings.ignore` (what cargo-audit applied), the dates and issue numbers
// from the file, and the two must agree. The reports below are captured
// cargo-audit 0.22.2 output (scripts/fixtures/README.md), not written here.

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import {
  acceptedEntries,
  acceptedFromReport,
  parseArguments,
  entriesFromAuditToml,
  parseReport,
  recordedAccepted,
  updateComment,
} from "./audit-tracking-issue.mjs";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const readRoot = (path) => readFileSync(join(root, path), "utf8");
const AUDIT_TOML = readRoot(".cargo/audit.toml");
const ACCEPTED_REPORT = JSON.parse(readRoot("scripts/fixtures/cargo-audit-report-0459-accepted.json"));
const REPORTED_REPORT = JSON.parse(readRoot("scripts/fixtures/cargo-audit-report-0459-reported.json"));
const ACCEPTED_ID = "RUSTSEC-2026-0459";

const entry = (id, review_by, issue, shared = false) => ({ id, review_by, issue, shared });

test("the accepted ids are the report's settings.ignore, as cargo-audit applied them", () => {
  const accepted = acceptedFromReport(ACCEPTED_REPORT);
  assert.equal(accepted.length, 11);
  assert.ok(accepted.includes(ACCEPTED_ID));
  // Ignored, so absent from the findings: the gap this list closes.
  assert.ok(!findingsFromReport(ACCEPTED_REPORT).some((finding) => finding.id === ACCEPTED_ID));
  const reported = acceptedFromReport(REPORTED_REPORT);
  assert.equal(reported.length, 10);
  assert.ok(!reported.includes(ACCEPTED_ID));
  assert.ok(findingsFromReport(REPORTED_REPORT).some((finding) => finding.id === ACCEPTED_ID));
});

// "Nothing accepted" and "the report could not be read" must never look alike:
// the second, read as the first, would announce every accepted id as gone.
test("a report without a readable settings.ignore is refused, never read as nothing accepted", () => {
  const { settings, ...noSettings } = ACCEPTED_REPORT;
  const { ignore, ...noIgnore } = settings;
  for (const report of [
    noSettings,
    { ...ACCEPTED_REPORT, settings: noIgnore },
    { ...ACCEPTED_REPORT, settings: { ...settings, ignore: "RUSTSEC-2026-0459" } },
    { ...ACCEPTED_REPORT, settings: { ...settings, ignore: [ACCEPTED_ID, 7] } },
    { ...ACCEPTED_REPORT, settings: { ...settings, ignore: [" "] } },
    null,
  ]) {
    assert.throws(() => acceptedFromReport(report), /no settings\.ignore list of advisory ids/);
    assert.throws(() => acceptedEntries([report], AUDIT_TOML), /no settings\.ignore list of advisory ids/);
  }
  assert.deepEqual(acceptedFromReport({ settings: { ignore: [] } }), [], "an empty list is a real answer");
});

test("an empty or unparseable report file fails as a failed audit", () => {
  assert.throws(() => parseReport("audit-report.json", ""), /audit-report\.json is empty/);
  assert.throws(() => parseReport("audit-report.json", " \n"), /audit-report\.json is empty/);
  assert.throws(() => parseReport("audit-report.json", '{"settings": {'), /audit-report\.json is not JSON/);
  const text = readRoot("scripts/fixtures/cargo-audit-report-0459-accepted.json");
  assert.deepEqual(parseReport("captured", text), ACCEPTED_REPORT);
});

test("every entry in the real file carries its date, and the accepted vulnerability names its issue", () => {
  const entries = entriesFromAuditToml(AUDIT_TOML);
  assert.deepEqual(
    entries.map((e) => e.id),
    acceptedFromReport(ACCEPTED_REPORT),
  );
  for (const e of entries) assert.match(e.review_by ?? "", /^\d{4}-\d{2}-\d{2}$/, e.id);
  assert.deepEqual(entries.find((e) => e.id === ACCEPTED_ID), entry(ACCEPTED_ID, "2026-12-09", 2432));
  // Five ids under one comment block share its date.
  const unic = entries.filter((e) => e.id.startsWith("RUSTSEC-2025-"));
  assert.equal(unic.length, 5);
  assert.ok(unic.every((e) => e.review_by === "2026-12-15"));
  assert.deepEqual(unic.map((e) => e.shared), [false, true, true, true, true]);
  assert.deepEqual(acceptedEntries([ACCEPTED_REPORT], AUDIT_TOML), entries);
});

test("the file and the report disagreeing fails the run, whichever side has the extra id", () => {
  // In the file, not the report: the captured report from master's file.
  assert.throws(
    () => acceptedEntries([REPORTED_REPORT], AUDIT_TOML),
    /disagree \(only in the file: RUSTSEC-2026-0459; only in the report: none\)/,
  );
  // In the report, not the file: the same file without that entry.
  const start = AUDIT_TOML.indexOf("  # --- RUSTSEC-2026-0459 --");
  const end = AUDIT_TOML.indexOf(`"${ACCEPTED_ID}",`) + `"${ACCEPTED_ID}",`.length;
  assert.ok(start > 0 && end > start);
  const without = AUDIT_TOML.slice(0, start) + AUDIT_TOML.slice(end);
  assert.ok(!entriesFromAuditToml(without).some((e) => e.id === ACCEPTED_ID));
  assert.throws(
    () => acceptedEntries([ACCEPTED_REPORT], without),
    /disagree \(only in the file: none; only in the report: RUSTSEC-2026-0459\)/,
  );
});

test("only a date and an issue number leave a comment, and an unknown line is refused", () => {
  const file = (body) => `[advisories]\nignore = [\n${body}\n]\n`;
  assert.deepEqual(
    entriesFromAuditToml(file('  # see bridge#445 and https://x/#1, then #77; **bold** text\n  # Review-by: 2027-01-02 (later)\n  "A",\n  "B"')),
    [entry("A", "2027-01-02", 77), entry("B", "2027-01-02", 77, true)],
  );
  assert.deepEqual(entriesFromAuditToml(file('  "A",')), [entry("A", null, null)]);
  assert.deepEqual(entriesFromAuditToml(file("")), []);
  assert.throws(() => entriesFromAuditToml(file("  'A',")), /not an id or a comment/);
  assert.throws(() => entriesFromAuditToml(file('  "A`<b>",')), /not an id or a comment/);
  assert.throws(() => entriesFromAuditToml(file('  "@team/x",')), /not an id or a comment/);
  assert.throws(() => entriesFromAuditToml(file('  "A", "B",')), /not an id or a comment/);
  assert.throws(() => entriesFromAuditToml(file('  "A",\n  "A",')), /ignores A more than once/);
  assert.throws(() => entriesFromAuditToml("[advisories]\n"), /no `ignore = \[` line/);
  assert.throws(() => entriesFromAuditToml('ignore = [\n  "A",\n'), /not closed/);
});

const ACCEPTED = [entry("RUSTSEC-2026-0459", "2026-12-09", 2432), entry("RUSTSEC-2024-0429", "2027-03-15", null)];

test("the issue stays open while anything is accepted, and closes only when nothing is", () => {
  assert.equal(planAction([], null, ACCEPTED).action, "create");
  const open = { number: 7, ids: [], accepted: ACCEPTED };
  assert.equal(planAction([], open, ACCEPTED).action, "unchanged");
  assert.deepEqual(planAction([], open, []), { action: "close", issue: 7 });
  assert.deepEqual(planAction([], null, []), { action: "none" });
});

test("a body written before the accepted list announces every accepted id once", () => {
  const plan = planAction([], { number: 7, ids: [] }, ACCEPTED);
  assert.equal(plan.action, "update");
  assert.deepEqual(plan.newlyAccepted.map((e) => e.id), ["RUSTSEC-2024-0429", "RUSTSEC-2026-0459"]);
  assert.deepEqual([plan.added, plan.removed, plan.noLongerAccepted, plan.changed], [[], [], [], []]);
});

test("a new, a removed and a renewed entry are each named once in one comment", () => {
  const before = { number: 7, ids: [], accepted: [entry("A", "2026-12-09", 2432), entry("B", "2027-01-01", null)] };
  const after = [entry("A", "2027-01-08", 2432), entry("C", "2027-02-01", 9)];
  const plan = planAction([], before, after);
  assert.equal(plan.action, "update");
  assert.deepEqual(plan.newlyAccepted.map((e) => e.id), ["C"]);
  assert.deepEqual(plan.noLongerAccepted, ["B"]);
  assert.deepEqual(plan.changed, [{ id: "A", from: before.accepted[0], to: after[0] }]);
  const comment = updateComment(plan);
  assert.equal(
    comment,
    "Scheduled audit update -- newly accepted in .cargo/audit.toml: C; no longer accepted: B; " +
      "accepted entry changed: A (review-by 2026-12-09, #2432 -> review-by 2027-01-08, #2432).",
  );
  const run = fakeRun([]);
  apply(plan, run);
  assert.deepEqual(run.calls.map((args) => args[1]), ["edit", "comment"]);
  assert.equal(run.calls[1].at(-1), comment);
});

// Each change is announced by the run that sees it. The next run reads back
// what it wrote, finds nothing moved, and makes no call at all.
test("a re-run after a create or an update posts nothing", () => {
  const findings = findingsFromReport(ACCEPTED_REPORT);
  const accepted = acceptedEntries([ACCEPTED_REPORT], AUDIT_TOML);
  const created = fakeRun([]);
  apply(planAction(findings, null, accepted), created);
  const body = created.calls[0][created.calls[0].indexOf("--body") + 1];
  assert.deepEqual(recordedAccepted(body), accepted);
  const rerun = fakeRun([]);
  apply(planAction(findings, findExisting(fakeRun([{ number: 5, body }])), accepted), rerun);
  assert.equal(rerun.calls.length, 0, "an unchanged accepted list must be silent");

  const updated = fakeRun([]);
  apply(planAction(findings, { number: 5, ids: findings.map((f) => f.id) }, accepted), updated);
  assert.deepEqual(updated.calls.map((args) => args[1]), ["edit", "comment"]);
  const updatedBody = updated.calls[0][updated.calls[0].indexOf("--body") + 1];
  const again = fakeRun([]);
  apply(planAction(findings, findExisting(fakeRun([{ number: 5, body: updatedBody }])), accepted), again);
  assert.equal(again.calls.length, 0);
});

test("the body lists each accepted entry with only its id, date and issue", () => {
  const body = issueBody([], ACCEPTED);
  assert.match(body, /^- none$/m, "no findings is said, not left blank");
  assert.match(body, /^- `RUSTSEC-2026-0459` -- review-by 2026-12-09, #2432$/m);
  assert.match(body, /^- `RUSTSEC-2024-0429` -- review-by 2027-03-15, no issue named in the file$/m);
  assert.deepEqual(recordedAccepted(body), [...ACCEPTED].sort((a, b) => a.id.localeCompare(b.id)));
  assert.deepEqual(recordedAccepted(`${MARKER}\nAccepted advisory entries: {broken`), []);
  assert.deepEqual(recordedAccepted(`${MARKER}\nAccepted advisory entries: [["A", 7, null, false]]`), []);
});

test("the tracker refuses to run without the audit config, in either mode", () => {
  const report = ["--report", "src-tauri/Cargo.lock=audit-report.json"];
  for (const argv of [report, [...report, "--apply"]]) {
    assert.throws(() => parseArguments(argv), /--audit-config .* is required/);
  }
  assert.throws(() => parseArguments([...report, "--audit-config"]), /--audit-config needs a value/);
  assert.deepEqual(parseArguments([...report, "--audit-config", ".cargo/audit.toml", "--apply"]), {
    reports: [{ lockfile: "src-tauri/Cargo.lock", path: "audit-report.json" }],
    shouldApply: true,
    auditConfig: ".cargo/audit.toml",
  });
});

test("ids sharing one comment block share its date, and two reports must each agree", () => {
  const entries = entriesFromAuditToml(AUDIT_TOML);
  const zbus = entries.filter((e) => ["RUSTSEC-2026-0370", "RUSTSEC-2026-0371", "RUSTSEC-2026-0372"].includes(e.id));
  assert.equal(zbus.length, 3);
  assert.ok(zbus.every((e) => e.review_by === "2027-04-18" && e.issue === null));
  assert.deepEqual(zbus.map((e) => e.shared), [false, true, true]);
  assert.deepEqual(acceptedEntries([ACCEPTED_REPORT, ACCEPTED_REPORT], AUDIT_TOML), entries);
  assert.throws(() => acceptedEntries([ACCEPTED_REPORT, REPORTED_REPORT], AUDIT_TOML), /only in the file: RUSTSEC-2026-0459/);
  // An id the report repeats is still one id.
  const repeated = { settings: { ignore: [...ACCEPTED_REPORT.settings.ignore, ACCEPTED_ID] } };
  assert.deepEqual(acceptedEntries([repeated], AUDIT_TOML), entries);
});

test("a recorded line naming the same entries twice repairs silently, and bad rows are not echoed", () => {
  const twice = [...ACCEPTED, ...ACCEPTED];
  assert.equal(planAction([], { number: 7, ids: [], accepted: twice }, ACCEPTED).action, "repair");
  const rows = JSON.stringify([
    ["A", "2026-12-09", 1, false],
    ["B", "not a date", null, false],
    ["C", null, -4, false],
    ["D", null, 1.5, false],
    ["E", null, null, "yes"],
    ["F", "2026-12-09", 1],
  ]);
  assert.deepEqual(recordedAccepted(`${MARKER}\nAccepted advisory entries: ${rows}`), [entry("A", "2026-12-09", 1)]);
});

// The scheduled job passes the file, and its permissions stay the two it had.
test("the scheduled workflow passes the audit config and keeps its permissions", () => {
  const workflow = readRoot(".github/workflows/dependency-security-scheduled.yml");
  assert.match(workflow, /node scripts\/audit-tracking-issue\.mjs\n\s+--report src-tauri\/Cargo\.lock=audit-report\.json\n\s+--audit-config \.cargo\/audit\.toml\n\s+--apply\n/);
  const permissions = workflow.slice(workflow.indexOf("\npermissions:\n") + 1, workflow.indexOf("\nconcurrency:"));
  assert.equal(permissions.trim(), "permissions:\n  contents: read\n  issues: write # the one permission beyond the pull-request audit's");
});

// A caller that leaves the accepted entries out must not read as "nothing
// accepted": that is the reading that closes the issue.
test("planAction and issueBody refuse a call without the accepted entries", () => {
  const open = { number: 2320, ids: [], accepted: ACCEPTED };
  assert.throws(() => planAction([], open), /planAction needs the accepted entries/);
  assert.throws(() => planAction([], open, null), /planAction needs the accepted entries/);
  assert.throws(() => planAction([], null), /planAction needs the accepted entries/);
  assert.throws(() => issueBody([]), /issueBody needs the accepted entries/);
  assert.throws(() => apply({ action: "update", issue: 9, ids: [], findings: [], added: [], removed: [] }, fakeRun([])), /issueBody needs the accepted entries/);
});

// An id with no comment of its own takes the block above it. The issue says so,
// so a new id never looks separately dated and tracked.
test("an id under another entry's comment is shown as sharing it, and an id with no comment as having none", () => {
  const file = '[advisories]\nignore = [\n  "LONE",\n  # Review-by: 2027-01-02, see #77\n  "A",\n  "B",\n\n  "C",\n]\n';
  assert.deepEqual(entriesFromAuditToml(file), [
    entry("A", "2027-01-02", 77),
    entry("B", "2027-01-02", 77, true),
    entry("C", "2027-01-02", 77, true),
    entry("LONE", null, null),
  ]);
  const body = issueBody([], entriesFromAuditToml(file));
  assert.match(body, /^- `A` -- review-by 2027-01-02, #77$/m);
  assert.match(body, /^- `B` -- review-by 2027-01-02, #77 \(shares the entry above\)$/m);
  assert.match(body, /^- `C` -- review-by 2027-01-02, #77 \(shares the entry above\)$/m);
  assert.match(body, /^- `LONE` -- review-by none in the file, no issue named in the file$/m);

  // An id added straight under RUSTSEC-2026-0459's entry borrows its date and
  // issue: it is named as sharing them, in the body and in the comment.
  const added = AUDIT_TOML.replace(`"${ACCEPTED_ID}",\n`, `"${ACCEPTED_ID}",\n  "RUSTSEC-2099-0001",\n`);
  assert.notEqual(added, AUDIT_TOML);
  const after = entriesFromAuditToml(added);
  assert.deepEqual(after.find((e) => e.id === "RUSTSEC-2099-0001"), entry("RUSTSEC-2099-0001", "2026-12-09", 2432, true));
  const before = entriesFromAuditToml(AUDIT_TOML);
  const plan = planAction([], { number: 2320, ids: [], accepted: before }, after);
  assert.equal(updateComment(plan), "Scheduled audit update -- newly accepted in .cargo/audit.toml: RUSTSEC-2099-0001 (shares the entry above).");
  assert.match(issueBody([], after), /^- `RUSTSEC-2099-0001` -- review-by 2026-12-09, #2432 \(shares the entry above\)$/m);
  // The shared mark is recorded, so the re-run stays silent.
  const body2 = issueBody([], after);
  assert.equal(planAction([], { number: 2320, ids: [], accepted: recordedAccepted(body2) }, after).action, "unchanged");
});
