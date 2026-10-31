// SPDX-License-Identifier: Apache-2.0

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import { isNonRetryableOutstandingsBoundary, outstandingsAgeingAnchorLabel, outstandingsAgeingDisclosure, outstandingsPartialReason, outstandingsPartialState, workingPaperUnavailableState } from "../src/outstandings-copy.ts";

test("the backend ageing anchor is disclosed in the bucket label", () => {
  assert.equal(outstandingsAgeingAnchorLabel("due_date"), "aged from due date");
  assert.equal(outstandingsAgeingAnchorLabel("bill_date"), "aged from bill date");
});

test("new native and sweep boundaries have operator-readable reasons", () => {
  assert.equal(
    outstandingsPartialReason(
      "native_outstandings_as_of_refused",
      "20260822",
      "20260731",
    ),
    "Tally refused the requested as-of date (20260822) and returned overdue days as of 20260731, so ComplyEaze Bridge withheld the totals",
  );
  assert.match(outstandingsPartialReason("native_overdue_crosscheck_mismatch"), /overdue-day cross-check/i);
  assert.match(
    outstandingsPartialReason("native_outstandings_as_of_unconfirmed_without_bill_references"),
    /no bill references/i,
  );
  assert.match(
    outstandingsPartialReason("native_outstandings_as_of_unconfirmed_without_effective_date_evidence"),
    /no overdue-day evidence.*effective date/i,
  );
  assert.match(outstandingsPartialReason("company_currency_probe_failed"), /base currency/i);
  assert.match(outstandingsPartialReason("company_base_currency_not_inr"), /not INR/i);
  assert.match(
    outstandingsPartialReason("company_base_currency_undetermined"),
    /more than one currency.*could not tell.*base currency/i,
  );
  assert.match(outstandingsPartialReason("company_outstandings_read_failed"), /company read failed/i);
  for (const code of ["company_identity_absent_or_ambiguous", "company_identity_not_matched"]) {
    assert.match(outstandingsPartialReason(code), /not .* one loaded company.*Open it in Tally/i);
  }
  assert.match(outstandingsPartialReason("company_identity_presentation_collision"), /another company whose name matches/i);
});

test("a foreign-currency ledger names the blocked book without inviting a repeat", () => {
  const state = outstandingsPartialState(
    "company_foreign_currency_ledger_balance",
    undefined,
    undefined,
    "Synthetic FX Debtor",
  );

  assert.match(state.title, /not available for this company/i);
  assert.match(state.message, /Synthetic FX Debtor/);
  assert.match(state.message, /rather than guessing a base-currency amount/i);
  assert.equal(state.retryable, false);
  assert.equal(state.tallyReadAttempted, true);
});

test("a ledger kept in another currency names the ledger without inviting a repeat", () => {
  for (const code of ["ledger_currency_base_unmatched"]) {
    const state = outstandingsPartialState(code, undefined, undefined, "Synthetic FX Debtor");
    assert.match(state.title, /not available for this company/i, code);
    assert.match(state.message, /ledger Synthetic FX Debtor/, code);
    assert.match(state.message, /rather than count amounts in another currency as rupees/i, code);
    assert.equal(state.retryable, false, code);
    assert.equal(state.tallyReadAttempted, true, code);
  }
  for (const code of ["ledger_currency_base_unmatched", "ledger_currency_unobserved"]) {
    const state = outstandingsPartialState(code);
    assert.doesNotMatch(state.message, /could not prove every requested segment/i, code);
    assert.equal(state.retryable, false, code);
  }
});

test("currency ledgers left out withhold desktop totals and name the ledgers", () => {
  const excluded = [
    { ledger: "Synthetic FX Debtor A", currency: "$" },
    { ledger: "Synthetic FX Debtor B", currency: "$" },
  ];
  const state = outstandingsPartialState("currency_ledgers_excluded", undefined, undefined, undefined, excluded);
  assert.match(state.title, /totals withheld/i);
  assert.match(state.message, /2 ledgers in a currency other than this company's base currency/);
  assert.match(state.message, /Synthetic FX Debtor A \(\$\), Synthetic FX Debtor B \(\$\)/);
  assert.match(state.message, /shows no totals here/);
  assert.match(state.message, /MCP outstandings\) reports the plain base-currency ledgers only/);
  assert.doesNotMatch(state.message, /shows values in another currency/);
  assert.equal(state.retryable, false);
  assert.equal(state.tallyReadAttempted, true);
  const many = Array.from({ length: 7 }, (_, index) => ({ ledger: `Synthetic FX ${index}`, currency: "$" }));
  const long = outstandingsPartialState("currency_ledgers_excluded", undefined, undefined, undefined, many);
  assert.match(long.message, /7 ledgers/);
  assert.match(long.message, /Synthetic FX 4 \(\$\) and 2 more/);
  assert.doesNotMatch(long.message, /Synthetic FX 5/);
  const unnamed = outstandingsPartialState("currency_ledgers_excluded");
  assert.match(unnamed.message, /keeps some of this company's ledgers in another currency/);
  assert.equal(isNonRetryableOutstandingsBoundary("currency_ledgers_excluded"), true);
});

// bridge#642: base-currency ledgers with a value in another currency are
// named on their own, beside or without the foreign ones.
test("mixed base-currency ledgers left out are named with the foreign ones", () => {
  const both = outstandingsPartialState(
    "currency_ledgers_excluded",
    undefined,
    undefined,
    undefined,
    [{ ledger: "Synthetic FX Debtor A", currency: "$" }],
    ["Synthetic Rupee Party", "Synthetic Sales"],
  );
  assert.match(both.message, /1 ledger in a currency other than this company's base currency: Synthetic FX Debtor A \(\$\)/);
  assert.match(both.message, /shows values in another currency on 2 base-currency ledgers: Synthetic Rupee Party, Synthetic Sales/);
  assert.match(both.message, /with their bills/);
  const mixedOnly = outstandingsPartialState("currency_ledgers_excluded", undefined, undefined, undefined, [], ["Synthetic Rupee Party"]);
  assert.match(mixedOnly.message, /on 1 base-currency ledger: Synthetic Rupee Party/);
  assert.doesNotMatch(mixedOnly.message, /in a currency other than this company's base currency:/);
  assert.equal(mixedOnly.retryable, false);
});

test("missing, empty, or zero-only counters name the unconfirmed effective-date boundary", () => {
  const state = outstandingsPartialState(
    "native_outstandings_as_of_unconfirmed_without_effective_date_evidence",
  );

  assert.equal(state.title, "Tally did not confirm this as-of date");
  assert.match(state.message, /no overdue-day evidence/i);
  assert.match(state.message, /effective date/i);
  assert.equal(state.tallyReadAttempted, true);
});

test("an empty bill report with ledger money names the unconfirmed as-of boundary", () => {
  const state = outstandingsPartialState("native_outstandings_as_of_unconfirmed_without_bill_references");

  assert.equal(state.title, "Tally did not confirm this as-of date");
  assert.match(state.message, /ledger still carried a balance/i);
  assert.match(state.message, /could not confirm the requested as-of date/i);
  assert.equal(state.tallyReadAttempted, true);
});

// bridge#2071: when every bill belongs to a set-aside ledger, the totals are
// withheld for the unconfirmed date, and the set-aside ledgers are still named.
test("a withheld result names the ledgers it also set aside", () => {
  const state = outstandingsPartialState(
    "native_outstandings_as_of_unconfirmed_without_bill_references",
    undefined,
    undefined,
    undefined,
    [{ ledger: "Synthetic FX Debtor A", currency: "$" }],
    ["Synthetic Rupee Party"],
  );
  assert.equal(state.title, "Tally did not confirm this as-of date");
  assert.match(state.message, /^Tally returned no bill references .* withheld the totals\. /);
  assert.match(state.message, /1 ledger in a currency other than this company's base currency: Synthetic FX Debtor A \(\$\)\. /);
  assert.match(state.message, /on 1 base-currency ledger: Synthetic Rupee Party\. ComplyEaze Bridge left those ledgers out, with their bills\.$/);
  assert.equal(state.retryable, true);
  const unnamed = outstandingsPartialState("native_outstandings_as_of_unconfirmed_without_bill_references", undefined, undefined, undefined, [], []);
  assert.doesNotMatch(unnamed.message, /left those ledgers out/);
});

test("a refused period is distinct from a row-level disagreement", () => {
  const refused = outstandingsPartialState(
    "native_outstandings_as_of_refused",
    "20260822",
    "20260731",
  );
  const inconsistent = outstandingsPartialState("native_overdue_crosscheck_mismatch");

  assert.equal(refused.title, "Tally did not accept this as-of date");
  assert.match(refused.message, /requested as-of date \(20260822\)/i);
  assert.match(refused.message, /as of 20260731/i);
  assert.notEqual(refused.message, inconsistent.message);
});

test("a partial result reports a completed check instead of an unread request", async () => {
  const screen = await readFile(new URL("../src/OutstandingsScreen.tsx", import.meta.url), "utf8");

  assert.match(screen, /const tallyReadAttempted = result\?\.state === "partial" && partialState\?\.tallyReadAttempted;/);
  assert.match(screen, /: tallyReadAttempted\s*\? `Checked \$\{relativeTime\(result\.synced_at_unix_ms\)\}`\s*: "No Tally data was read"/s);
});

test("unknown reason codes remain readable", () => {
  assert.equal(outstandingsPartialReason("date_partition_scope_mismatch"), "date partition scope mismatch");
});

test("working-paper failures stay distinct from completed report availability", () => {
  const resource = workingPaperUnavailableState("working_paper_resource_limit");
  assert.match(resource.title, /working paper unavailable/i);
  assert.match(resource.message, /report is complete/i);
  assert.match(resource.message, /safe export limits/i);
  // bridge#551: statements come from the same held source, so they are
  // unavailable too, and the copy must not promise them.
  assert.match(resource.message, /party statements/i);
  assert.doesNotMatch(resource.message, /other report exports remain available/i);

  const source = workingPaperUnavailableState("working_paper_complete_source_unavailable");
  assert.match(source.message, /native bill and unallocated controls/i);
  assert.match(source.message, /party statements/i);
  assert.doesNotMatch(source.message, /other report exports remain available/i);

  const store = workingPaperUnavailableState("working_paper_export_store_unavailable");
  assert.match(store.message, /one-use working-paper snapshot/i);
  assert.match(store.message, /refresh outstandings/i);
  assert.match(store.message, /other report exports remain available/i);
});

test("the screen renders a complete-result working-paper reason in band", async () => {
  const screen = await readFile(new URL("../src/OutstandingsScreen.tsx", import.meta.url), "utf8");

  assert.match(screen, /workingPaperUnavailableState\(completeResult\.working_paper_unavailable_reason_code\)/);
  assert.match(screen, /workingPaperUnavailable && \(/);
  assert.match(screen, /<strong>\{workingPaperUnavailable\.title\}<\/strong>/);
  assert.match(screen, /<span>\{workingPaperUnavailable\.message\}<\/span>/);
});

test("unaged receivables disclose the ageing scope without inventing an On Account total", () => {
  const disclosure = outstandingsAgeingDisclosure(true);
  assert.match(disclosure, /excluded from these buckets/i);
  assert.match(disclosure, /no bill reference or age/i);
  assert.match(disclosure, /does not show an On Account amount/i);
  assert.match(disclosure, /cannot prove the full unallocated balance/i);
  assert.equal(outstandingsAgeingDisclosure(false), null);
});

test("a path that can prove the unallocated balance says so instead of disclaiming it", () => {
  // A read that cannot establish the unallocated remainder keeps an honest
  // disclaimer. The native bills path recovers that figure exactly from the
  // party ledgers, so
  // repeating "ComplyEaze Bridge does not show an On Account amount" there would be false
  // while a screen right above it displays exactly that amount.
  const known = outstandingsAgeingDisclosure(true, true);
  assert.match(known, /shown as Unallocated above/i);
  assert.doesNotMatch(known, /cannot prove/i);
  assert.doesNotMatch(known, /does not show/i);

  // Absent knowledge must keep the original disclaimer, never silently claim a
  // figure it does not have.
  assert.match(outstandingsAgeingDisclosure(true, false), /cannot prove the full unallocated balance/i);
  assert.equal(outstandingsAgeingDisclosure(false, true), null);
});
