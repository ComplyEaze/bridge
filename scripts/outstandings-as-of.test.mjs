// SPDX-License-Identifier: Apache-2.0

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import {
  allCompaniesOutstandingsInvokeArgument,
  automaticOutstandingsAsOf,
  asOfBoundValueForAsOf,
  asOfYyyymmdd,
  bulkPartyStatementsInvokeArgument,
  bulkPartyStatementsPreviewInvokeArgument,
  operatorSelectedOutstandingsAsOf,
  partyLedgerMasterInvokeArgument,
  partyStatementInvokeArgument,
  refreshAutomaticOutstandingsAsOf,
  settleAsOfBoundValue,
  singleCompanyOutstandingsInvokeArgument,
  todayAsDateInput,
  workingPaperInvokeArgument,
} from "../src/outstandings-as-of.ts";

test("the visible as-of control defaults to the operator's local calendar date", () => {
  assert.equal(todayAsDateInput(new Date(2026, 7, 22, 0, 1)), "2026-08-22");
  assert.equal(asOfYyyymmdd("2026-08-22"), "20260822");
  assert.equal(asOfYyyymmdd("2026-8-22"), null);
});

test("an automatic as-of date rolls over locally without replacing an operator selection", () => {
  const beforeMidnight = new Date(2026, 7, 22, 23, 59, 59);
  const afterMidnight = new Date(2026, 7, 23, 0, 0, 1);

  assert.deepEqual(
    refreshAutomaticOutstandingsAsOf(automaticOutstandingsAsOf(beforeMidnight), afterMidnight),
    { value: "2026-08-23", operatorSelected: false },
  );
  assert.deepEqual(
    refreshAutomaticOutstandingsAsOf(operatorSelectedOutstandingsAsOf("2026-08-01"), afterMidnight),
    { value: "2026-08-01", operatorSelected: true },
  );
});

test("all-client rows are invalidated at rollover and a late old-date sweep is discarded", () => {
  const beforeMidnight = automaticOutstandingsAsOf(new Date(2026, 7, 22, 23, 59, 59));
  const afterMidnight = refreshAutomaticOutstandingsAsOf(
    beforeMidnight,
    new Date(2026, 7, 23, 0, 0, 1),
  );
  const entries = [{ company: "Synthetic Company", company_guid: "synthetic-guid" }];
  const completedBeforeRollover = settleAsOfBoundValue(
    asOfYyyymmdd(beforeMidnight.value),
    asOfYyyymmdd(beforeMidnight.value),
    entries,
  );

  assert.deepEqual(
    asOfBoundValueForAsOf(completedBeforeRollover, asOfYyyymmdd(beforeMidnight.value)),
    entries,
  );
  assert.equal(
    asOfBoundValueForAsOf(completedBeforeRollover, asOfYyyymmdd(afterMidnight.value)),
    null,
  );
  assert.equal(
    settleAsOfBoundValue(
      asOfYyyymmdd(afterMidnight.value),
      asOfYyyymmdd(beforeMidnight.value),
      entries,
    ),
    null,
  );
});

test("single-client totals are invalidated at rollover and a late old-date read is discarded", () => {
  const beforeMidnight = automaticOutstandingsAsOf(new Date(2026, 7, 22, 23, 59, 59));
  const afterMidnight = refreshAutomaticOutstandingsAsOf(
    beforeMidnight,
    new Date(2026, 7, 23, 0, 0, 1),
  );
  const completeResult = { state: "complete", report: { as_of_yyyymmdd: "20260822" } };
  const completedBeforeRollover = settleAsOfBoundValue(
    asOfYyyymmdd(beforeMidnight.value),
    asOfYyyymmdd(beforeMidnight.value),
    completeResult,
  );

  assert.deepEqual(
    asOfBoundValueForAsOf(completedBeforeRollover, asOfYyyymmdd(beforeMidnight.value)),
    completeResult,
  );
  assert.equal(
    asOfBoundValueForAsOf(completedBeforeRollover, asOfYyyymmdd(afterMidnight.value)),
    null,
  );
  assert.equal(
    settleAsOfBoundValue(
      asOfYyyymmdd(afterMidnight.value),
      asOfYyyymmdd(beforeMidnight.value),
      completeResult,
    ),
    null,
  );
});

test("both outstandings screens wire date-bound rendering and stale-settlement rejection", async () => {
  const [singleClient, allClients] = await Promise.all([
    readFile(new URL("../src/OutstandingsScreen.tsx", import.meta.url), "utf8"),
    readFile(new URL("../src/AllClientsScreen.tsx", import.meta.url), "utf8"),
  ]);

  for (const source of [singleClient, allClients]) {
    assert.match(source, /asOfBoundValueForAsOf/);
    assert.match(source, /settleAsOfBoundValue/);
    assert.match(source, /requestVersion\.current \+= 1/);
  }
});

test("the single-company request emits the selected canonical as-of date", () => {
  assert.deepEqual(
    singleCompanyOutstandingsInvokeArgument(
      { host: "127.0.0.1", port: 9000 },
      { name: "Bridge Validation Lab", guid: "guid-1", company_number: "100001", books_from_yyyymmdd: "20260401" },
      "2026-08-17",
      "bill_date",
    ),
    {
      request: {
        config: { host: "127.0.0.1", port: 9000 },
        selected_company: {
          display_name: "Bridge Validation Lab",
          company_guid: "guid-1",
          company_number: "100001",
          books_from_yyyymmdd: "20260401",
        },
        as_of_yyyymmdd: "20260817",
        ageing_anchor: "bill_date",
      },
    },
  );
  assert.equal(singleCompanyOutstandingsInvokeArgument({ host: "127.0.0.1", port: 9000 }, { name: "Lab", guid: "guid-1", company_number: "100001", books_from_yyyymmdd: "20260401" }, "2026-8-17", "due_date"), null);
});

test("the single-company request carries no currency assertion (bridge#551)", () => {
  const argument = singleCompanyOutstandingsInvokeArgument(
    { host: "127.0.0.1", port: 9000 },
    { name: "Bridge Validation Lab", guid: "guid-1", company_number: "100001", books_from_yyyymmdd: "20260401" },
    "2026-08-17",
    "due_date",
  );
  assert.equal(Object.hasOwn(argument.request, "currency_assertion"), false);
  assert.equal(argument.request.as_of_yyyymmdd, "20260817");
});

test("compare clients emits the same selected canonical as-of date", () => {
  assert.deepEqual(
    allCompaniesOutstandingsInvokeArgument(
      { host: "127.0.0.1", port: 9000 },
      [
        { name: "Bridge Validation Lab", guid: "guid-1", company_number: "100001", books_from_yyyymmdd: "20260401" },
        { name: "Bridge Ageing Lab", guid: "guid-2", company_number: "100014", books_from_yyyymmdd: "20270401" },
      ],
      "2026-08-17",
      "bill_date",
    ),
    {
      request: {
        config: { host: "127.0.0.1", port: 9000 },
        companies: [
          { selected_company: { display_name: "Bridge Validation Lab", company_guid: "guid-1", company_number: "100001", books_from_yyyymmdd: "20260401" } },
          { selected_company: { display_name: "Bridge Ageing Lab", company_guid: "guid-2", company_number: "100014", books_from_yyyymmdd: "20270401" } },
        ],
        as_of_yyyymmdd: "20260817",
        ageing_anchor: "bill_date",
      },
    },
  );
});

test("statement exports name the Rust-held source, never rows, dates or a basis", () => {
  const result = {
    report: { company_name: "Bridge Validation Lab", as_of_yyyymmdd: "20260801" },
    ageing_anchor: "bill_date",
    statement_open_bills: [{ party: "Alpha", amount: "1" }],
    statement_unallocated_by_party: [{ party: "Alpha", amount: "2" }],
    party_statement_source_id: "synthetic-statements",
  };

  assert.deepEqual(partyStatementInvokeArgument(result, "Alpha", "xlsx"), {
    request: { source_id: "synthetic-statements", party: "Alpha", format: "xlsx" },
  });
  assert.deepEqual(
    bulkPartyStatementsInvokeArgument(result, "/tmp/statements", "synthetic-approval", "pdf"),
    {
      request: {
        source_id: "synthetic-statements",
        destination: "/tmp/statements",
        approval_id: "synthetic-approval",
        format: "pdf",
      },
    },
  );
  assert.deepEqual(bulkPartyStatementsPreviewInvokeArgument(result), {
    request: { source_id: "synthetic-statements" },
  });

  const withoutSource = { ...result, party_statement_source_id: undefined };
  assert.equal(partyStatementInvokeArgument(withoutSource, "Alpha", "xlsx"), null);
  assert.equal(
    bulkPartyStatementsInvokeArgument(withoutSource, "/tmp/statements", "synthetic-approval", "pdf"),
    null,
  );
  assert.equal(bulkPartyStatementsPreviewInvokeArgument(withoutSource), null);
});

test("working-paper export sends only the opaque Rust-owned read binding", () => {
  assert.deepEqual(
    workingPaperInvokeArgument({ working_paper_export_id: "synthetic-export-id" }),
    { request: { export_id: "synthetic-export-id" } },
  );
  assert.equal(workingPaperInvokeArgument({}), null);
  assert.equal(workingPaperInvokeArgument({ working_paper_export_id: "" }), null);
});

test("the ledger-master request carries the screen's date only when the operator chose it (bridge#903)", () => {
  const config = { host: "127.0.0.1", port: 9000 };
  const company = {
    name: "Synthetic Accounts",
    guid: "00000000-0000-0000-0000-000000000000",
    company_number: "100001",
    books_from_yyyymmdd: "20260401",
  };
  const chosen = partyLedgerMasterInvokeArgument(config, company, "2027-03-31", true);
  assert.equal(chosen.request.as_of_yyyymmdd, "20270331");
  assert.deepEqual(chosen.request.selected_company, {
    display_name: "Synthetic Accounts",
    company_guid: "00000000-0000-0000-0000-000000000000",
    company_number: "100001",
    books_from_yyyymmdd: "20260401",
  });
  // An automatic date is the host's own: nothing is sent, so the backend's
  // default is untouched.
  const automatic = partyLedgerMasterInvokeArgument(config, company, "2027-03-31", false);
  assert.equal("as_of_yyyymmdd" in automatic.request, false);
  // A chosen date the contract cannot carry blocks the export.
  assert.equal(partyLedgerMasterInvokeArgument(config, company, "", true), null);
  assert.equal(partyLedgerMasterInvokeArgument(config, company, "2027-3-31", true), null);
});

test("main.tsx tells the Outstandings screen whether the operator chose the As of date (bridge#903)", async () => {
  const frontend = await readFile(new URL("../src/main.tsx", import.meta.url), "utf8");
  const start = frontend.indexOf("<OutstandingsScreen");
  assert.notEqual(start, -1);
  assert.equal(frontend.indexOf("<OutstandingsScreen", start + 1), -1, "one Outstandings screen");
  const element = frontend.slice(start, frontend.indexOf("/>", start));
  assert.match(element, /asOf=\{outstandingsAsOfSelection\.value\}/);
  assert.match(element, /asOfOperatorSelected=\{outstandingsAsOfSelection\.operatorSelected\}/);
  // The screen reports the operator's date through the handler that marks it as chosen;
  // a no-op handler would leave the automatic (host) date in force for the export.
  assert.match(element, /onAsOfChange=\{changeOutstandingsAsOf\}/);
  // An operator's date is the one that marks it as chosen; the automatic one never does.
  assert.match(
    frontend,
    /const changeOutstandingsAsOf = React\.useCallback\(\(value: string\) => \{\s*setOutstandingsAsOfSelection\(operatorSelectedOutstandingsAsOf\(value\)\);/,
  );
  assert.match(frontend, /useState\(\s*\(\) => automaticOutstandingsAsOf\(\),\s*\)/);
});
