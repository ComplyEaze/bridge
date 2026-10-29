# Changelog

All notable changes to ComplyEaze Bridge are documented here. The project follows
[Semantic Versioning](https://semver.org/).

## [Unreleased]

Published builds are MCPB packages that are not yet code-signed (tags
`mcp-preview-*` and, from 0.4.0, `mcp-v*`): so far
`mcp-preview-0.2.0`, `mcp-preview-0.3.0`, `mcp-v0.4.0`, `mcp-v0.4.1`, `mcp-v0.4.2`, `mcp-v0.5.0`, `mcp-v0.5.1`, `mcp-v0.6.0` and `mcp-v0.6.1`. The
number of the next build is chosen when it is released.
The version boundary between the published MIT-licensed `v0.1.0` release and
Apache-2.0 builds from current source stays unambiguous.

## [0.7.0] - <cut date>

### In plain words: ComplyEaze Bridge 0.7.0, since 0.6.1 (27 Oct 2026)

These changes are in ComplyEaze Bridge 0.7.0. Most lines name the pull requests or issues they come from. Like every build so far, it is not yet code-signed. The desktop app has no published installer, so lines about it apply only to a build from source. <check at the cut: whether Sales invoice posting stays switched off in 0.7.0, as in 0.6.1>

**Should I upgrade?**

- **If you read stock, statements or a ledger's movement: yes.** `stock_summary` now reads as of the last day of any month, not only 31 March, and gives the quantity of an item inside a stock group (#1233). `ledger_movement` reads only the named ledger's vouchers, so a large book no longer fails as too large for a ledger with a small share of it (#485). A Trial Balance row that does not add up is marked, and `profit_and_loss` and `balance_sheet` name its ledger and gap when they refuse (#2068, #692).
- **If you use posting:** what Bridge writes into Tally is still only what you approve in the approval window, and the kinds of voucher it posts are the same as in 0.6.1 (Journal, Payment, Receipt and Contra). On Windows the approval boxes now stay above Tally when you click into Tally while one is open, and show dates as Tally does, for example `14-Sep-2026` (#1780, #2101). A post's answer that reported `STATUS` 0 no longer counts toward the voucher mark it proved (#2035), and a date that names no calendar day is refused in four checks (#2009).
- **If you want Bridge to create a ledger:** it can now create one ledger at a time, through its own approval box, only when `BRIDGE_AGENT_ENABLE_MASTER_WRITES` and posting are both on. The desktop extension has no setting for the first, so a default install does not offer it (#690).
- **Before you upgrade:** once 0.7.0 has recorded a person's check of a post matched by content only (`acknowledge_post_review` with `doubt` `content_only`), or a person's settlement of a post Tally answered as creating none of its vouchers (`doubt` `not_created`), do not go back to 0.6.1: an older build refuses the whole import journal once either record is written (#1039, #1709).
- **Everyone else:** nothing else needs doing. Install steps are the same as for 0.6.1: [installation guide](https://github.com/ComplyEaze/bridge/blob/master/docs/agent/INSTALL.md); after installing, check the extension shows 0.7.0, the Terms setting is ticked and the posting setting is as you want it; quit Claude Desktop completely and reopen it.

**What you can do now**

- `prepare_ledger_creates` and `post_ledger_creates` create ONE new ledger in a company, the first write a published
  build makes to Tally's masters (part of #690). They are listed only when `BRIDGE_AGENT_ENABLE_MASTER_WRITES` and posting
  (`BRIDGE_AGENT_ENABLE_WRITES`) are both set to true; the desktop extension has no setting for the first, so a default
  install does not offer them. The ledger is a party under Sundry Debtors or Sundry Creditors, a Suspense ledger, or a
  GST duty ledger whose head has been measured, from typed fields; ComplyEaze Bridge builds the request itself, accepts
  no XML, and can set no opening balance or parent. `prepare_ledger_creates` reads the company's ledger and group names,
  refuses a name that already exists as a ledger, a group or an alias or that differs from one only in case, spaces or
  symbols (Tally can take such a Create for the existing ledger and change it), writes nothing to Tally, and returns a
  `batch_id` and the preview the person will be shown (its ledger name masked under `mask_parties`). `post_ledger_creates` asks the local user in a native dialog and sends
  the create once; a batch already sent is never sent again. After the send the book is read back and one state is
  returned, `posted_verified` only when Tally's counters show one create and no alteration, the company's own master
  mark moved by exactly one (on a later call, the ledger is the first master change after the last read before the
  send), exactly one new ledger of the name exists with the parent, tax fields and GSTIN as sent, no ledger or
  group that existed before changed, and no other loaded company's master mark moved (bill-wise is not read back). Measured: one ledger created and read back as `posted_verified` through the tools on a synthetic
  company (TallyPrime 7.1 Silver, 14 Oct 2026), and the approval box shown whole, declined with nothing sent, accepted
  and then answered from the journal with no second box (TallyPrime 7.2, a continuous-integration package of an earlier
  head of the change, 239896f8, 28 Oct 2026). Not run: a create whose answer is lost, other TallyPrime releases and editions, and the build
  of this release. A name Tally resolves to an existing ledger is reported loudly and is not retried.
- A ledger create with no answer from Tally on file (it could not be recorded, or Tally never answered) can now settle. A later `post_ledger_creates` call for the batch answers `posted_unattributed` when it finds the ledger as sent and the ledger's `ALTERID` shows it as the first master change in the company after ComplyEaze Bridge's last read of it before sending the create, and not changed since. That releases the ledger name the batch held. It is never `posted_verified`: with no answer from Tally on file, nothing shows the change was ComplyEaze Bridge's own create rather than the same ledger made by someone else, so the answer asks the person to check the ledger in Tally. A ledger changed since stays open with the reason `ledger_altered_since_create` (#2003).
- `acknowledge_post_review` with `doubt` set to `content_only` records, after the native review dialog, that a person
  checked in Tally a native post matched by content only: its binding to its own vouchers was refused, or no answer from
  Tally was recorded. The record changes nothing in Tally and no status or verdict, binds no voucher, lifts no invoice
  stop and releases no row; `verify_import` reports it as `content_review`, and `local_data_report` counts the batch as
  `not_settled_reviewed_not_attributed`. A later `posted_verified`, a later answer from Tally or a rollback of the book
  voids it. It is a new kind of import journal record: a build older than this one refuses the whole journal once one is
  written, so do not downgrade after recording one (part of #1039).
- `acknowledge_post_review` with `doubt` set to `not_created` lets a person settle a native post that Tally answered
  as creating none of its vouchers, in its own native dialog, once they say each voucher is in the book or will not be
  posted. ComplyEaze Bridge never sends those vouchers again. The record changes nothing in Tally and no status or
  verdict; the rows stay held, so they are never built or posted again, and a Sales invoice keeps its stop until the
  separate release. `verify_import` reports it as `person_settlement`, and `local_data_report` counts the batch as
  `settled_by_person`, outside `not_settled`. A later `posted_verified`, a later answer from Tally, a binding or a
  rollback of the book voids it. It is a new kind of import journal record: a build older than this one refuses the
  whole journal once one is written, so do not downgrade after recording one (part of #1709).
  On Windows its box opens system-modal like the other approval boxes. After a settlement, the dialog counts only matches in the accounts and each voucher's next step no longer asks for it again.
- `gst_by_counter_ledger` totals, for a period, the GST the books record on ledgers under Duties & Taxes per counter
  ledger (the one Purchase Accounts, Direct Expenses, Indirect Expenses or Fixed Assets ledger on a purchase, debit
  note, journal or payment with a supplier on it, named with its GUID), debit and credit side apart, per head, and sets
  every other voucher with GST aside under a named reason with its own totals (a voucher with two such ledgers, a
  reverse-charge or set-off journal, a sale or credit note, a tax payment with no supplier, a duty ledger not typed GST,
  TDS or TCS, a ledger under another group). Books only: not a return figure, and no eligibility or availment decision.
  Measured on fourteen vouchers typed on TallyPrime 7.2's screen in a synthetic book, item invoices among them. The
  same synthetic book, after its company and one supplier were registered and December's vouchers keyed, was read with an
  earlier build of this tool that did not yet admit Direct Expenses or Fixed Assets ledgers or name the ledger GUID: none
  of December's six vouchers was set aside, and a supplier invoice entered twice (the same reference, date and bill) was
  totalled twice with nothing flagged, so the answer does not detect a duplicate invoice. Gold, other releases and a
  registered book on the build of this release are not measured (part of #1866).
- `vouchers` takes `bank_allocations: true` to add, on each ledger entry, the bank date, instrument number and date,
  transaction type and status of its bank allocations, each field saying whether Tally sent it absent, empty or with a
  value; a transfer reference typed in a Contra's bank allocation is its instrument number. `status` is Tally's own
  `STATUS` element, not a reconciliation state, and nothing returned says whether an entry is reconciled. On two
  entries reconciled on Tally's screen with a bank date of 6 April 2026, Tally's raw answer filled the bank date and set
  `BANKMANUALSTATUS` and `BANKRECONSTATUS` to Reconciled, while `STATUS` stayed No; this read returns the bank date but
  neither of those two elements. The built tool has not been run against a live Tally on a reconciled entry; its parser,
  run on the saved raw answer, read the bank date (#2006). Without the argument, answers are as
  before. Measured on one synthetic book of five hand-keyed Contras (TallyPrime 7.2, keyed in double entry), and, for the
  reconciled entries, by a raw read of five Contras of a second synthetic book (TallyPrime 7.2); no request changed (#1231,
  #1392).
- `vouchers` takes `goods_lines: true` to add, on each ledger entry of an invoice voucher, its goods lines: the stock
  item, actual and billed quantity, rate and amount, and each godown and batch row, each field saying whether Tally sent
  it absent, empty or with a value. An entry with a line whose batch amounts do not add up to its amount shows no lines, and says
  why. A Stock Journal showed no lines in the one read of it (TallyPrime 7.2, not committed); a Delivery Note or Receipt Note has not been captured. Without the argument, answers
  are as before. Measured on Sales and Purchase invoices of two synthetic books (TallyPrime 7.1, one also on 7.2); no
  request changed (#2015).
- `vouchers` takes `summarise_by: cost_centre` to sum a window's cost-centre allocations into one bucket per cost
  centre under each cost category, with each category's own totals (`category_totals`: a category allocates the whole
  entry, so categories overlap) and the entries that carry no allocation (`entries_unallocated`). An entry whose
  allocations cannot be read, or whose allocations under one category do not add up to the entry, refuses the summary
  (`summary_cost_centre_unreadable`, `summary_cost_centre_allocation_mismatch`). The summary does not read the company's
  Cost Centres setting: allocations were read on a book with it off, so an empty summary means nothing was allocated in
  the window. Measured on the saved raw answers of one synthetic book with Cost Centres on (TallyPrime 7.2): a Payment
  allocated to two centres of one category and a Journal entry allocated in full under two categories; the summary has
  not been run through the tool against a live TallyPrime; no request changed (#1231).
- A party's mailing address, PIN code, state and country can now be read from Tally's dated mailing details
  (`LEDMAILINGDETAILS.LIST`) (#1490). On the measured book (one synthetic company, TallyPrime 7.1 Silver, 12 Oct 2026) the
  flat address, PIN code and state fields came back empty or absent for a party that held an address, and the dated list
  held it: two address lines, the PIN code, the state, the country, the date it applies from and the mailing name.
  `ledger_masters fields=compliance` takes `include_mailing_details` (default false): without it no request names the
  list and no row has the key; with it each row carries the dated entries as `compliance.mailing_details`, oldest first.
  It is refused together with `include_gst_details`, as the request naming both has not been captured. Under
  `mask_parties` the mailing name, every address line and the flat address field are shortened like a party's name, to
  their first two and last two characters; the PIN code, state and country are returned under every redaction mode. A
  second mailing entry with a later date was read on TallyPrime 7.2 (one party, oldest first); three or more entries, an
  entry with no date and other releases are not measured. The
  registers do not read the mailing details and keep their request and their estimate.
- The party/ledger master workbook, a local file written by the desktop app, gains five columns at its end: the mailing
  address, PIN code and state in force on the Balances-as-of date, the date they apply from, and a status that says why
  they are blank (no mailing details, none in force on that date, not observed, or unreadable). The workbook asks Tally
  for the mailing details only for a book whose master-alteration mark (an upper bound on its ledgers) is at most 3,200;
  for a larger book it sends the same request as before, every ledger's status reads not observed, and a note in the
  workbook's header says so, so a book that exported before is never refused for the larger per-ledger estimate. For a book
  past that mark the five columns are still there but empty, and every status reads not observed: the choice is made on
  the mark, which counts every master alteration, so a book with few ledgers and many alterations is read without the
  mailing details too (#1490).
- `stock_summary` now reads stock as of the last day of any month, not only 31 March. The period runs from 1 April of that financial year, or the book's start if later; any other date is refused as `stock_summary_as_of_not_measured`, as before. On two lab books on TallyPrime 7.2, month ends inside the year matched Tally's own Stock Summary, items inside stock groups included; a voucher dated on the end date was counted on a 15 October read, and a month end with a voucher on it was not measured. A company whose financial year does not start on 1 April is still read from 1 April (part of #1233).
- `stock_summary` now returns the closing quantity of a stock item inside a stock group. Tally's plain Stock Summary has no line for such an item, so the quantity is the item's own in the stock item list and is not compared: the item says `inside_stock_group`, with `item_list_amount` and `item_list_unit`, and is never `agreed`. On one lab book with groups three deep, each such quantity equalled Tally's Stock Summary screen with every level expanded. An item whose parent reads `Primary` without Tally's root marker now says `parent_not_resolved` and carries no quantity, and `checks.closing_quantity` is `checked_where_agreed` (part of #1233).
- `ledger_movement` with a `ledger` now reads only that ledger's vouchers for the period, instead of every voucher in the book, so a large book no longer fails as too large for a ledger that holds a small share of it. The window is planned from a count of the ledger's vouchers, and the ledger is taken from the book's own spelling of its name. The result says how it was read (`window_scope`: `ledger_scoped`, with the vouchers read, or `whole_book` with the reason). When the ledger's name has a character the scoped read does not send or holds a word Tally's formulas read as an operator (such as "and" or "not"), when a day of its vouchers is too heavy for one read, or when the scoped read finds nothing, the whole book's vouchers for the period are read as before and the result says so. A returned voucher with no line on the ledger refuses the read. The lab has compared the scoped read with the Trial Balance for one ledger and one month only, where its debit matched; `ledger_movement` as a whole has not yet been read on a Tally with this change (part of #485).
- The `outstandings` party detail can now answer for a party on a book whose window is too large to read. Where the read of the whole
  company's vouchers is refused as needing too many requests, a party matched to one ledger alone, whose vouchers carry the ledger's name as
  stored and whose name holds only ASCII letters, digits and single spaces and no TDL operator word, is counted by its own ledger and its
  vouchers are read by that ledger in planned parts, each checked against the count voucher by voucher. Such a detail says so in
  `window_scope` (`basis` `ledger_scoped`, with a note on what that list was compared with) and counts `party_vouchers_read`; every other
  detail is read as before and keeps `company_vouchers_read`. Where the ledger is not read, or its read finds nothing or would need a day
  divided by AlterID, the original refusal stands and its `window_scope` gives the reason; a refusal of the ledger read itself (a part not
  admitted, a row with no line on the ledger) keeps its own code (part of #1356).
- `egress_log` no longer refuses a read whose receipts are too large for one response. When the receipts asked for (they can be about 11 KiB each) would not fit the response size cap, it returns the newest whole receipts that do fit, measured on the response as it is sent, marks the answer `truncated`, and counts the receipts left out in `result.omitted_for_size`. A single receipt that alone is over the cap is still refused (`agent_response_too_large`). The result now says `order` is `newest_first`. Older receipts are still not reachable by paging (part of #2125).
- The desktop Trial Balance and its Excel workbook now mark a ledger whose closing is not its opening plus its debit and credit totals, as the `trial_balance` answer does: the row is shown as Tally sent it with the difference beside its closing (a `movements_differ_by` column in the workbook), and the same explanation the answer gives appears above the table and in the workbook. Such a row was seen on a Closing Stock ledger whose closing balance was keyed on Tally's screen; the screen and the workbook were tested on its captured answers and not opened in the running app. A read where every ledger adds up looks as before (#2132, part of #2068).
- `parse_bank_statement_csv` reads a local CSV bank statement in the one export layout measured, requires `controls_source` (`prior_statement`, `bank_ledger_in_tally` or `this_file`) and refuses every file with a typed code: one of fifteen shape categories, or `statement_csv_account_line_unmeasured` for a file that reads, because the export's account line is not yet established; it proposes nothing yet (part of #1320).

**Safer or fixed**

- On Windows, the boxes that ask a person to approve a post, to record a review, to release an invoice stop or to create a ledger are now opened system-modal, to keep each above Tally when the person clicks into Tally while it is open. When a box opened a few seconds after the person's last input to Tally, Windows had not kept it on top, and one click into a maximised Tally covered it until it was brought back from the taskbar; in one lab run on Windows 10, the post and ledger-create boxes, opened system-modal, stayed above Tally after that click, and Tally still took a click and a key while a box was open; the other two boxes were not shown on a screen. In that run the taskbar showed no task buttons while a box was open; the cause was not tested (#2161). Windows 11 was not run (part of #1780).
- On Windows, the box that asks a person to create a ledger is now shown as the post's box is: brought to the front once when it opens, with its taskbar button flashing if Windows refuses, so that the Y and N its first line names can reach it. In one lab run it opened some seconds after the person's last input to Tally and never had the keyboard, so a Y pressed as its first line says would have gone to Tally. It is also measured against the screen: a box that does not fit is refused as `master_review_too_large`, with nothing sent. Like the other approval boxes, it now needs Windows 10 version 1703 or later; on an older Windows the answer is `master_approval_unavailable` (part of #1780).
- The approval and review boxes now show dates in Tally's day-month-year form with the year in full, for example `14-Sep-2026` and `Books from: 1-Apr-2026`, where they showed Tally's request form `20260914`. This applies to the one-voucher, invoice and batch approval texts, the one-voucher and batch review texts, and the box that asks to create a ledger, on Windows and macOS. Only the text a person reads changes: what is sent to Tally stays `YYYYMMDD`. Each date is up to three characters longer, so a text that sat exactly at a box's size limit before may now be shown as a batch's totals, or refused, as any text over the limit is (fixes #2101).
- A native post whose recorded answer reported `STATUS` 0 no longer counts that answer's `CREATED` toward the voucher
  mark it proved, so whichever of the company's posts is checked next, a mark read unchanged does not record a
  rollback for it (#2035). After the update, a rollback record that read a mark at or above such a post's pre-POST
  mark no longer covers it: the post no longer reads `book_rolled_back` because of that record; its batch no longer
  carries `restored` where the `transfers` section or a `withdraw_skips` refusal names it, and a held row may name
  another batch that holds it; and the post stays unverified, as an answer reporting `STATUS` 0 always leaves it. The
  readback in that post's own `post_import` call still reads `book_rolled_back` when the mark falls between the one
  read before the post and the one read just after it; that post's own floor then records no rollback, though another
  native post of the company at that Tally address can still record one in the same call. The answers the tests use are
  written by hand: no import answer with `STATUS` 0 and a `CREATED` count has been captured from a live Tally, and the
  native posts measured answer with a `RESPONSE` root that reports no status, so this has not been seen on a live post.
- Four checks that accepted any eight digits as a Tally date now refuse a value that names no calendar day (for example
  `20251340` or `20250229`): a mirror batch's requested date range, which HSN item is in force on an invoice date,
  whether a sales ledger's GST rate is in force on an invoice date, and a company context's from and to dates. Before,
  such a value passed and, in the HSN and rate checks, could be chosen as the item or rate in force. A batch, an invoice
  build or a parsed export that carried an impossible date is now refused where it was accepted. No captured answer
  holds one, and whether Tally ever sends one in these fields is not measured; tested on invented facts (fixes #2009).
- When `post_import` or `verify_import` cannot confirm a post because the company's voucher mark moved by another amount than the
  post created, or the post's span later held another number of vouchers, the post's summary now also says what was
  observed and what can cause it (for example a GST return exported from Tally, an edit of one of the vouchers, or another
  user's entry), and that ComplyEaze Bridge sent the post once and Tally reported creating each voucher once. That sentence is shown only
  for those two refusals, never for an answer that reported `STATUS` 0 or whose counters were not clean. Measured on
  TallyPrime 7.2, on one synthetic book each: two GSTR-1 JSON exports each raised the voucher mark by 3 without changing
  a voucher, while one local GSTR-3B JSON export of one month left it unchanged (17 before and after) and moved no
  voucher's alter id; a voucher saved by hand, once, inside a post's span made the mark move by 51 for 50 created, and the span
  binding was refused. Nothing about how a post is proven changes (part of #1633).
- The `outstandings` party detail (#1960) now reads the company's two change marks before the statement of open bills and again after
  the voucher window, and refuses (`voucher_window_changed_during_read`, the same refusal and next step as every read
  that brackets the book) when either moved; the voucher window no longer reads the marks a third time. Before, the statement, the ledger catalogue and the vouchers were tied by the
  party's spelling with nothing checking that the book was the same between them, so a ledger renamed in Tally between two of
  those reads could answer as a finding (for example no bill for the party). It adds one marks read net (two, and the window reads none of its own), and any voucher or
  master edit in Tally during the call refuses it; run the same request again once. A change that moves neither mark is not
  seen. Measured: a rename moves the master mark and a voucher edit moves the voucher mark (TallyPrime 7.1 Silver, 21
  Sep 2026), but one rename that changed only a ledger's letter case moved neither mark (one book), so that kind of
  rename is still not caught; this read has not been run against Tally.
- Under `mask_parties`, `ledger_masters` with `fields=compliance` now masks the flat `address` of a ledger
  (and, from the same change, every mailing address line) like a party name: before, the flat address was
  returned as Tally sent it under every redaction mode. An address Tally returned empty stays empty. The flat
  PIN code and state, which every compliance read returns, and the mailing entry's PIN code, state and
  country, which are returned only when the assistant asks for the mailing details, are still not masked
  under any mode (#1490).
- The `vouchers` snapshot shows `voucher_alter_id`, and the audit read's manifest `alter_voucher_id`, as null,
  not 0, when Tally's answer carries no voucher mark, which it omits for a company that has never held a
  voucher; `ledger_masters` already showed null for the same book. A mark Tally sends, 0 included, is shown as
  sent. Every comparison of the marks still reads an absent voucher mark as 0, so no read or post decides
  differently (fixes #1984).
- A `trial_balance` row whose closing is not its opening plus its debit and credit totals (an empty amount counted as zero) now carries `movements_differ_by` (the exact difference), the first line counts such ledgers and a limitation explains, and the row is returned as Tally sent it. Seen on a Closing Stock ledger whose closing balance was keyed on Tally's screen: Tally kept the opening as the closing and showed the change only in the debit total, so the closing is not necessarily the figure its statements use. Every row of the committed captures adds up, so no other ledger is flagged; `profit_and_loss` and `balance_sheet` already refuse a read where a row does not add up, by the same check (part of #2068, #692).
- When `profit_and_loss` or `balance_sheet` refuses because a Trial Balance row does not add up, the refusal now names
  that row's ledger and its gap, says what to tell the user, and carries the evidence of the Trial Balance and statement
  reads already made, not only the company read (#692).
- `balance_sheet` and `profit_and_loss` now say when Tally's own Balance Sheet does not balance: on the one book
  measured (a stock item's opening value edited, TallyPrime 7.2) the export left out the `Difference in opening
  balances` line Tally's screen showed, so when its lines do not sum to zero the result carries
  `tally_balance_sheet_sides` (the credit and debit totals), the first line gives both and nothing is established;
  the new reason `tally_balance_sheet_does_not_balance` is given when no other reason applies (#2063).
- A refused `trial_balance` read now carries a `cause` when the Trial Balance answer could not be used (the rule it failed, such as `trial_balance_source_identity_missing` or `trial_balance_amount_invalid`), where it gave the code `trial_balance_read_failed` alone. `profit_and_loss`, `balance_sheet` and `cash_flow` show the same `cause`, and no other reason or step changes. Every `trial_balance_read_failed` refusal that has no step of its own now carries one (a timeout, an HTTP error, an oversized answer or an unusable one): it tells the assistant to report what the cause says, or that the read failed when none is named, and not to repeat the same read in a loop. The refusals that have their own codes (before the books began, a period not honoured, Education mode, the base-currency codes) keep their own text and get no step from this. On the one book with foreign-currency vouchers measured (TallyPrime 7.2), the Trial Balance was refused with no cause before this change; a build with it, read in two sittings on 29 Oct 2026, refused with the cause `trial_balance_amount_invalid` and its next step. The later change that sets TallyPrime 7.2's composite layout aside clears that cause on the book's captured answer; it has not been read on that book through the tool. A failure in the totals after the parse still names no cause (part of #2071, part of #683).
- When a read of the company's ledgers (the compliance read of `ledger_masters`, the sales and purchase registers, or the invoice import's admission) is refused because Tally wrote a ledger's foreign-currency balance in a layout ComplyEaze Bridge does not recognise yet (`party_ledger_master_read_failed`, cause `native_foreign_currency_ledger_balance`), the refusal now says what to do: that the read is refused rather than guessed at (#683), that a retry refuses again, and to tell the user that this read cannot give the company's ledgers while Tally writes a balance that way. The refusal was seen in the lab on one TallyPrime 7.2 book, on a build without this next step; that book's layout (no space after the foreign currency symbol) is now recognised and set aside by name like the 7.1 layout, from its captured answers, not from a read through the tool. What still refuses is a layout no capture shows, such as a rate with decimals; the next step has not been seen on a live refusal (part of #2071). No request changed.
- A ledger balance that TallyPrime 7.2 writes as a foreign-currency composite with no space after the foreign symbol is now set aside by name, as the 7.1 layout is, instead of refusing the whole read. This applies to the compliance read of `ledger_masters`, the sales and purchase registers and the invoice import's admission (which read the same ledger source), to `trial_balance` on a book with several Currency masters, where such a ledger was named as left out only for the 7.1 layout, and to `outstandings`, where on a book with several Currency masters a party whose balance is in that layout leaves every figure and is named in `mixed_currency_ledgers_excluded`, so the result is partial, as it is for the 7.1 layout (on a book with one Currency master the read refuses, as before). A voucher entry in that layout is still refused, as no 7.2 voucher answer has been captured. On the lab book the `trial_balance` parse that refused with `trial_balance_amount_invalid` now returns the plain rupee ledgers and names the four it sets aside; none of `trial_balance`, `outstandings` and `ledger_masters` has been read on that book through the tool with this change (part of #683). No request changed.
- When a request of Tally's Bills Receivable or Payable report takes longer than the 20-second limit, `outstandings` is now refused with its own cause, `native_bills_report_deadline_exceeded`, in place of `request_deadline_exceeded`. Its next step now also says that Tally can go on building the report after the limit, so the next read of that company should wait until `tally_status` answers. Another request of the same read cut at the limit (the groups, the ledgers, the company checks) keeps `request_deadline_exceeded` and no longer gets the Bills report's next step. A call the host withdraws while the read runs now finishes the request in flight and sends nothing more to Tally. The cause was tested with a simulated Tally, and a withdrawn call was not run against a live TallyPrime (part of #2119).
- When the `outstandings` read of Tally's Bills Receivable or Payable report is refused because one of its requests took longer than the
  20-second limit, the refusal now names a next step from what was measured on one synthetic book of about 30,000 vouchers: about 15,000
  open bills answered and about 22,500 and 29,900 did not, and an earlier `as_of` with fewer bills open answered there, which on
  another book depends on the bills open then (#2119). A party detail refused by a step after its
  party was read by its ledger (the closing marks read, a book that moved, or the detail's checks) now says so in `window_scope`, and the
  next step for `party_ledger_row_without_line` says that only a bill trail can be narrowed by a `reference` (part of #1356).
- When `ledger_movement` reads a ledger by its own vouchers and Tally returns a voucher with no line on that ledger, the refusal now names a next step: the figure was not released, the call is not repeated in a loop, and where Tally is not in Education mode `trial_balance` gives that ledger's opening, debit, credit and closing for the dates. When the read falls back to the whole book after a ledger-scoped attempt was sent and given up, a result that carries a `read_cost` says in `window_scope` that the block leaves that attempt out. No request changed (part of #485).
- When `ledger_movement` reads the whole book's vouchers for a ledger instead of the ledger's own (a name the scoped read does not send, a name holding a word Tally's formulas read as an operator, a day too heavy for one scoped read, or a scoped read that found nothing) and that read is then refused, the refusal now carries `window_scope` with the reason and its own sentence (no figure was taken). Only a refusal of that read is marked; an argument or catalogue refusal is not. No request changed (part of #485).
- A `parent` that is Tally's root is now `&#4; Primary`, the text Tally sends, in the `masters` rows (cost centres, godowns, stock groups, groups), the `stock_summary` items, the `ledger_masters`, `ledger_movement`, `gst_by_ledger` and `trial_balance` rows (the Profit & Loss A/c ledger). It was a replacement character followed by `#4; Primary`, which read as a corrupted string. A group a user named Primary is still written plain, so the two stay apart, and `gst_by_ledger` no longer masks the root under `mask_parties`. The working form that the readers compare is untouched (fixes #2070).
- A folded twin's `parent` in an import report's `live_ledgers` is now `&#4; Primary` when it is Tally's root, the text Tally sends, where it was a replacement character followed by `#4; Primary`; a group a user named Primary is still written plain. The `vouchers` summary's `Primary` bucket label is unchanged: it is a label for the root's bucket, kept apart from a user group by the bucket's own key (part of #2070).

**What works differently**

- A skipped bank-statement row that no statement carries is now `tally_unchecked`, not `not_found`, in the `transfers`
  section of a build and of a verification. `not_found` said the other account's Contra does not exist, but ComplyEaze
  Bridge reads only the statements it built and verified: a Contra entered in Tally another way is not in its records.
  `not_found` is now said only after a read of every voucher of the row's date in Tally finds no Contra between the two
  accounts, and no such read exists yet, so every row that read `not_found` reads `tally_unchecked`. Its next step
  tells the assistant to read that date's vouchers with `vouchers`, to look for a voucher of the Contra voucher class,
  or of any other type such as a Journal or Payment, that carries the row's amount on the bank ledger the other
  account's statement was built for, to tell the person what it found and to keep the skip; a voucher dated another
  day or booked against another ledger is not looked for, a transfer may print on different days, and a skip is
  withdrawn (`withdraw_skips`), on one side only, solely when the person confirms that no entry in Tally carries the
  transfer. Reading a day sends its vouchers to the AI provider: `mask_parties` shortens party and ledger names,
  `drop_narration` drops narrations, and neither hides amounts. A row the other account's statements do not cover stays `not_established`. Tested on invented
  facts. No tool makes the whole-day read yet, and it has not run against a live TallyPrime. The requests it would send were run
  by hand on TallyPrime 7.2 against one synthetic book (four days, 3 to 6 April 2026): each day's two censuses were
  byte-identical and agreed with the vouchers answer, which held the book's five Contras and none on 6 April (part of
  #1392).
- The `post_import` tool text no longer tells the assistant that a row of a batch Tally rejected, or a genuinely
  different transaction that shares a voucher's fingerprint, is entered in Tally by hand. Like the refusals' own
  next steps, it now says that ComplyEaze Bridge cannot post that row yet and to tell the user so (#1709).
- The desktop Journal review no longer answers a Journal too large for it by pointing to the manual workflow. It now
  says that ComplyEaze Bridge cannot post that Journal from the review and names the next step. When the review itself
  is too large, that is to ask the assistant to build it again with a shorter narration, or to decide whether to split
  it into smaller Journals, each built as its own batch; when the Journal's file is larger than the review reads, a
  shorter narration cannot help, so it says only to split it (#1709).
- The tax-audit financial statements limitation about closing stock on books not integrated with inventory no longer calls the Stock-in-Hand ledger value "manually-entered"; it describes the balance the client recorded in the books, and its meaning is unchanged (#1709).
- The voucher-type masters read's note on how `sub_method` was measured now says a voucher was entered "on Tally's voucher screen" where it said "by hand"; the note only describes how the reading was taken, and its meaning is unchanged (#1709).
- The `held_elsewhere` step of the transfer section now names both outcomes of sending the batch that holds a
  skipped row: once the skip is live, `post_import` refuses that send as `skip_row_paired_elsewhere`, and a skip
  that was only built still lets it through, which leaves the row `posted_against_skip` once found posted (#1988).
- The `vouchers` tool description and the agent guide say that a row's `party` is Tally's own
  `PARTYLEDGERNAME`, which on a Receipt, Payment or Contra can name the bank or cash ledger rather than the
  counterparty, and that the counterparty of those vouchers is read from `amounts`. The parse is unchanged
  (fixes #1986).
- The text about cost-centre allocations in voucher reads that 0.6.1 shipped is corrected in three places. The vouchers tool no longer says
  that each category restates the entry's full amount and that summing rows across categories double-counts: that was not measured (only one
  category had been captured then), so it now says the rows are not checked against the entry and not to add rows across categories,
  each category's rows having added up to the whole entry on the one book since measured with two categories on one entry. The privacy page no longer says that an allocation in a shape the read does not recognise is not sent and not flagged: it
  is flagged (`cost_centre_allocation_shape_unrecognised`), and the tool text, the guide and the protocol reference now list all eleven
  reasons an entry's list can be unreadable. On an item invoice the sales or purchase ledger's entry carries the goods line: an allocation list
  beside the goods line is read like any other and one inside it is flagged; no captured item invoice carries allocations. No request and no
  answer changed (part of #1231).
- The `validate_masters` description and the voucher schema's ledger field now say what the alias work already
  returns: a `missing` or `near_miss` name that is exactly one ledger's alias carries `alias_of` and is still not
  bound, and the twin check counts aliases, naming `through_alias` (#1688).
- `negative_stock` now says which sign a value carries when it is present: the export's, as in the Stock Summary and the trial balance, where a debit is negative and a credit positive (the screen shows a credit as `(-)`). On the one book measured, the item at -3 Nos and a rate of 120.00 was exported with the value `360.00` on TallyPrime 7.1 and on 7.2, and the 7.2 screen showed `(-)360.00`; the other two short items captured had no value. The sentence is in the tool text and is the second of the result's limitations; no value changes (fixes #2051, part of #1358).
- Reading the mailing details makes the answer larger, and the read is sized for it (#1490). In the one capture (12 Oct
  2026, TallyPrime 7.1 Silver, eight ledgers) the answer grew from 19,094 to 20,896 bytes: 948 bytes for the one party that
  held a mailing entry and 122 bytes for each of the seven rows that carried an empty placeholder. With
  `include_mailing_details` the read is sized at 5,000 bytes a ledger instead of 3,750, above the largest row measured (4,902
  bytes: that party with a second dated entry, TallyPrime 7.2); it is marked unverified in the source and not measured on a
  book of many populated parties, so a large book may be read in more parts, or refused, sooner, and a party with three
  entries or long address lines can still be larger than the figure. A dense book reads about 950 bytes more for each party that holds an address; how
  that compares with the 20-second read deadline is not measured.
- The README now says what `egress_log` returns when its answer is trimmed for size: it names its order (`result.order`), counts the receipts cut for size in `result.omitted_for_size` and says they cannot be reached by this tool, and its evidence response digest hashes every receipt line the call read, which on a trimmed answer is more than the lines returned. A test pins the digest on a trimmed answer. No request or answer changed (part of #2125).

**What has been run against a live TallyPrime, and what has not**

- Ledger create (#690): created and read back as `posted_verified` once through the tools on a synthetic company (TallyPrime 7.1 Silver, 14 Oct 2026); the approval box shown whole, declined, accepted and answered from the journal once on a CI package of the change (TallyPrime 7.2, 28 Oct 2026). On the build of this release: <the masters row of the release-candidate sitting, to fill at the cut>. A create whose answer is lost (#2003) is not run.
- The approval boxes opened system-modal (#1780): one lab run on Windows 10 on a CI build of the change; the post and ledger-create boxes stayed above Tally after a click into Tally. Windows 11 and the review and release boxes on a screen were not run.
- `stock_summary` at month ends (#1233): two synthetic books on TallyPrime 7.2, one sitting each; items inside stock groups on one book with groups three deep.
- A Trial Balance row that does not add up (#2068, #692): seen on a master build's Windows package on two synthetic books (TallyPrime 7.2, 29 Oct 2026): only the Closing Stock ledger was flagged, and the statements refused naming it and its gap. One sitting per book.
- Refusal causes on a book with foreign-currency vouchers (#2071): the Trial Balance cause and next step were seen in two sittings on TallyPrime 7.2; the 7.2 composite layout set aside by name (#683) is read from the captured answers only, not through the tools.
- The Bills report refusal past the 20-second limit (#2119): measured on one synthetic book of about 30,000 vouchers; the new cause was tested with a simulated Tally.
- `vouchers` with `bank_allocations`, `goods_lines` and `summarise_by: cost_centre` (#1231, #2015): measured on saved raw answers of synthetic books; the cost-centre summary through the tool: <the release-candidate row, to fill at the cut>.
- A ledger asked for by its alias (#1688): <the release-candidate row, to fill at the cut>.
- Settling a native post Tally answered as creating none of its vouchers (#1709): <the release-candidate row R3, to fill at the cut; if Tally gives no whole refusal there, the dialog is covered by tests only>.
- Not run on a live TallyPrime for this build, with only captures or tests behind them: a post answer reporting `STATUS` 0 (#2035), an impossible calendar date (#2009), the `outstandings` party detail of a book that moved during the read (#1960), the desktop Trial Balance screen and workbook (#2132), a withdrawn `outstandings` call (#2119).

**Live pass of writes:** <the live write pass of the release-candidate sitting, to fill at the cut: build, company, date and what each check showed>.

**Known limits**

- **Not run on a live TallyPrime for this build (unless the lines above say otherwise):** anything that writes to Tally other than the rows of the live pass of writes and the masters row, Gold, macOS, other TallyPrime releases than 7.1 and 7.2 Silver, a book of about 30,000 vouchers for posting (#1029), and the Microsoft Store install of Claude Desktop (#1584, #1459).
- **Ledger create** makes one ledger at a time and alters none (#690). A create that Tally runs after Bridge stopped waiting is possible: in the lab Tally ran every abandoned create it was sent (#1713), so a create with no answer is never sent again.
- **Foreign-currency books:** no read reports a foreign-currency amount; each sets such a ledger aside by name or refuses (#683).
- **`stock_summary`** still refuses a book whose master mark is above 874, however few stock items it holds (#1233), and dates other than a month's last day.
- **`outstandings` at a year end on a dense book:** the Bills report can take longer than the 20-second limit; the read is refused with its own cause and next step, and no figure is released (#2119).
- **On Windows, while an approval box is open,** the taskbar may show no task buttons (#2161). The macOS alert is unchanged (#1780).
- **The whole-day read for a skipped transfer row** is not called by any tool yet, so no row reads `not_found` (#1392).
- **The file tested is not the file that will be tagged.** Each live row names the build it ran on; the published file is built again by the release workflow and checked after publication against the release's checksum, with no write.
- What has been run against a real TallyPrime is listed in the README.

## [0.6.1] - 2026-10-27

### In plain words: ComplyEaze Bridge 0.6.1, since 0.6.0 (16 Oct 2026)

These changes are in ComplyEaze Bridge 0.6.1. Most lines name the pull requests or issues they come from. Like every build so far, it is not yet code-signed. The desktop app has no published installer, so lines about it apply only to a build from source. Sales invoice posting stays switched off in 0.6.1; the lines about invoices apply only once it is offered.

**Should I upgrade?**

- **If you read ledger GST details (`ledger_masters` with `include_gst_details`): yes.** 0.6.0 named Tally's input tax credit flag `itc_eligible` and read Yes as "eligible". A ledger whose "Eligible for Input Tax Credit" setting was keyed No came back as `itc_eligible: true`. The key is now `ineligible_itc`, an ineligibility flag (measured on one ledger under Purchase Accounts; read a Yes on any other ledger as unconfirmed). Keys named `itc_eligible` in anything you built on 0.6.0 must change to `ineligible_itc`, and their meaning is the opposite of what the old name suggested (#1741).
- **If you use posting:** what Bridge writes into Tally is still only what you approve in the approval window, and the kinds of voucher it posts are the same as in 0.6.0 (Journal, Payment, Receipt and Contra; Sales invoices and ledger creation are not offered). What a post is checked against did change, and this build refuses more than 0.6.0 did. A post is refused before anything is sent when its last read before the send does not show the company's voucher mark (#1945). A verification whose voucher mark Tally leaves out twice, after its own window read saw one, is refused, not read as a rollback (#1819). A bank-statement batch is checked against the company's skipped rows and Contras at the moment it is sent, not only when it was built (#1714): if that check refuses, nothing is sent, any approval of the batch is withdrawn, and the batch has to be built again (after `transfer_view_not_established` the batch need not be built again, but it stays refused until the record is repaired; report it). The approval window and the review window now open with a line saying what each answer does (#1832). On Windows a window too tall for the screen it would open on is cancelled and the post is refused as too large, with nothing sent (#1879), and a batch listed line by line that does not fit is shown as totals when asked again (#1961). That fit check was measured on one Windows screen only (1920x1080 at 125 percent). On a Mac the alert's first line is no longer wrapped twice (#1854), but a Mac batch window can still be taller than the screen. While the approval window is open on Windows, do not type in Tally: a typed Y or N answers the window. Click the window with the mouse (#1784). The live pass of writes below ran one approved post and one statement batch that the send-time check refused, on a live book; the other new mark refusals (#1945, #1819) have run in tests only.
- **Before you upgrade:** once 0.6.1 has seen a restored book, do not go back to 0.6.0 (see the restore paragraph below). Saved batches that name a ledger whose name equals another ledger's alias, and some `voucher_presence` calls that declare a type manual, are refused now where 0.6.0 answered them.
- **Everyone else:** nothing else needs doing. Install steps are the same as for 0.6.0: [installation guide](https://github.com/ComplyEaze/bridge/blob/master/docs/agent/INSTALL.md); after installing, check the extension shows 0.6.1, the Terms setting is ticked and the posting setting is as you want it; quit Claude Desktop completely and reopen it.

**What you can do now**

- `audit_test_run`, off by default behind the extension's new **Allow tax-audit test runs (audit_test_run)**
  setting (`BRIDGE_AGENT_ENABLE_TAX_AUDIT`; off, it is not listed and a call is refused as `audit_tool_disabled`),
  runs one tax-audit test on a company's own books for a date range, today only `cash_44ab`:
  figure and finding ids with their values, confidence and clauses, and the count of any failed book check,
  with no ledger, party or voucher name; you name the cash and bank groups. Books only, not a tax opinion;
  refused while a redaction setting is on (`audit_redaction_unsupported`), and for a period outside
  1 April 2025 to 31 March 2026, the year its rules are for (`audit_period_outside_ay_2026_27`). The engine's test passed a real-book parity check on 21 Oct 2026, on one book with no cash payments (that side was checked at zero), at reference pin 6168aeaa (crate tree 53938eb8); the tool
  is replayed from the engine's synthetic book and not yet run against a live Tally (part of #697).

- `gst_uncertain_vouchers` lists the vouchers of one voucher type (a Sales type is what was measured) in a window that Tally marks Uncertain in its
  GSTR-1, and the vouchers a person accepted as they stand (Accept As Is), each with its party ledger, its invoice total
  (without its sign, with its side) and Tally's identifiers, and the line a CA reads before filing. It counts the window's
  vouchers with the status read first and refuses a window above 1,000 vouchers (provisional) before it reads amounts,
  and refuses an amounts read that returns fewer vouchers than the count found; the
  line names the voucher type it covers, and is withheld when no voucher was read or none is uncertain, and, with the
  result partial, when a voucher's flags or a listed voucher's party or amount were not read. Its answer carries party names and amounts
  to the assistant; `mask_parties` masks the names and not the amounts. The reason for uncertainty is not read, and bills
  missing an HSN/SAC are not split out. Measured on one synthetic book of TallyPrime 7.1 Silver, before and after one
  Accept As Is; item invoices, a registered buyer, credit notes, foreign currency and several accepts at once are not
  measured. It reads at most 93 days and refuses a book whose voucher mark is above 25,000 (part of #1342, #1692).

- `profit_and_loss` and `balance_sheet` can now be established on a book with stock items (part of #692), using
  Tally's own `Less: Closing Stock` figure from its Profit and Loss, shown as `closing_stock` (`applied`, with its
  value and source, or `declined`, with the reason) and not derived or taken from `stock_summary`. It is used only when
  that one figure closes every gap at once (Current Assets, the Balance Sheet's Profit & Loss A/c line and the
  trading result, and the `Cost of Sales :` heading), the company's own flags show inventory on and integrated, the
  `Opening Stock` line is shown empty and no Stock-in-Hand ledger carries an amount; otherwise the read refuses as it
  did. Both tools now read Tally's own Profit and Loss (one more pair of requests for `balance_sheet`, which stays the
  sheet it was if that read fails), and a book that offers a closing stock also has its inventory flags read (a failed
  read declines the figure as `inventory_flags_not_read`). The derived lines must also sum to zero. Measured on one synthetic book; a book with opening stock is declined (shown on captured answers), and a costing method, a book that is not integrated and a window spanning two years are not measured.

- `masters` can list a company's stock items (`kind` `stock_items`): each row carries the item's own dated HSN
  entries (`hsn`) and GST rate entries (`gst`) as Tally sent them, nothing choosing the code or the rate for a date
  (part of #1233). An empty or missing list is not "no HSN or GST": after a code was keyed on a stock group, the group's four other items still carried empty lists
  (one book); a GST rate or an HSN code on the company, and whether an item that defers to a coded group inherits its code, were not measured; the group's and the company's setting is not read, and every row says so
  (`inherited_from_group_or_company: not_read`). A defect in one item's list marks that list `unreadable` with its
  defect and the read stands; a Tally `ERROR`, markup inside a value or a cut-off list refuses the read. Item names
  and parents are masked under `mask_parties`, and so are HSN descriptions (a person can edit them); HSN codes and rates are returned as sent. Tally's
  `GSTINELIGIBLEITC` is returned as sent under `ineligible_itc`: it was Yes where the screen read
  `Eligible for Input Tax Credit: No` and absent where it read Yes (one item, two rows, PARTIAL), which fits an
  ineligibility flag; null is neither eligible nor ineligible, and what an absent element means for the tax is
  not known. On an entry whose source is `As per Company/Stock Group` (the entry Tally adds itself on a first save; a person can also choose that source) a flag Tally sent is withheld:
  `ineligible_itc` is null and `ineligible_itc_withheld` is `tally_default_entry`, because the Yes seen on such an
  entry was never compared with the screen; a flag Tally did not send stays null with no marker, and an entry with any other source is sent as it is. `ledger_masters` names the same element the same way (#1741). A book is read only up to a master mark of 489 (a limit of 489 refuses, by arithmetic, both client
  books measured; not tried on them). Measured on one synthetic book of TallyPrime 7.1 Silver (three captures, one item
  with two dated entries in each list); a state row other than `Any`, slab rates, another release, a larger book
  and a zero-row stock-item answer are not measured. `stock_summary` is unchanged.

- `negative_stock` (new tool; part of #1358): Tally's own Negative Stock list for a window, as `item`, `quantity`,
  `rate` and `value` per listed stock item, returned as Tally prints them and compared with no other report
  (`checks` says so). Being listed is not a finding that the quantity is below zero, and an empty answer is
  `not_established`, never "no negative stock": Tally's empty answer cannot be told from a report that was not
  rendered. Tally's report was captured from synthetic books: a short item listed for a whole financial year, the same item at
  another quantity for a window ending mid-year (with its rate and value empty), and a book with none short answered
  an empty envelope. Any other window is named as unmeasured, a book whose master mark is over 874 is
  refused before the report is asked for (a stated guess, as for `stock_summary`), and the cost on a large book is not known. The tool itself has not yet been run against Tally.

- `negative_ledgers` (new tool; part of #1358): Tally's own Negative Ledgers list for a window, as
  `ledger`, `side` and `amount` per listed ledger, returned as Tally lists it and compared with no other report
  (`checks` says so). An empty list is `not_established`, never "no negative ledger": Tally's empty answer cannot be told
  from a report that was not rendered. Tally's report was captured from three synthetic books, each for one whole financial year (on the first, a creditor with a
  debit balance and a debtor with a credit balance were listed and a Suspense ledger with a credit balance was not; on the
  other two, the one ledger listed matched the trial balance); any other window is named as unmeasured, and the cost on a large book is not known. The tool itself has not yet been run against Tally.

- Voucher reads now carry cost-centre allocations: an entry that has them gets `cost_centre_allocations` (`category`,
  `cost_centre`, `amount`; names trimmed, the amount as sent; both names are shortened under `mask_parties`, in the
  vouchers read and the two registers). A list that cannot be read whole (a centre with no name or
  no amount, an amount that is not a decimal, a centre or a category named twice, an empty allocation container, an allocation list in a shape not recognised) does not refuse the read: the entry carries
  `cost_centre_allocations_unreadable` with a reason and no partial list. No request changed; the allocations were
  already in the answers and were dropped (part of #1231).

**Safer or fixed**

- A batch built from a bank statement is checked against the company's skipped rows and Contras when `post_import`
  sends it, as it was when it was built, with the same refusals (`withdraw_skip_paired`,
  `skip_row_paired_elsewhere`, `skip_row_already_posted`, `withdraw_skip_invalid`, `transfer_double_booked`,
  `transfer_view_not_established`): before the post (before the dialog, or in the call that redeems an approval) and
  again under the admission lock. The refusals at send speak of the saved batch: nothing was sent, any approval of it
  is withdrawn and, except for `transfer_view_not_established`, the batch must be built again. A row the batch withdrew is judged on the
  withdrawal it recorded (`withdrawn_skips`), whether it was built from a statement or from vouchers given
  directly, and it counts only if the batch was built after the newest record of that skip: a skip recorded since
  refuses the batch (part of #1703).

- When a batch that names skipped rows in `withdraw_skips` is refused at send as `withdraw_skip_paired`, its next
  step now ends by saying which build to make: this statement again, with the rows in `paired_rows` still skipped
  and not named in `withdraw_skips`, instead of a bare "build it again" after steps that may speak of the other
  account's statement (#1714).

- A company restored from a backup after a native post is remembered once it is seen (part of #1050). When
  `verify_import` reads the company's voucher mark below the floor a natively posted batch had proved (a mark Tally
  omits twice counts as a company holding no voucher), the import journal records it, a second restore too, and from then on no natively posted batch of that company at that Tally
  address that it covers reads `posted_verified`, even after vouchers keyed again by hand raise the mark past it:
  a voucher it finds reads as matching content, one it does not as `book_rolled_back`, and the answer says to check
  them in Tally. The answer that records it names the company's other native batches it covers, to verify again. Before, a voucher keyed again
  with the same content could read verified once the mark had risen. Limits: a restore that is keyed or edited past
  the mark before any check is still not seen; a wrong backup restored and then the right one leaves the batches it
  covered reading rolled back (as would a repair or rewrite that lowered the mark, not measured); and a covered Sales
  invoice batch stops that company's invoices until released.
  **Do not go back to an earlier release after this one has seen a restore:** an earlier release refuses an import
  journal holding the new record, and stops every build, post and verification until ComplyEaze Bridge is upgraded
  again.

- The check of a bank-statement batch against skipped rows and Contras now treats a batch that a recorded restore covers
  as neither found posted nor verified, whatever its last status (part of #1050). Before, a skipped row such a batch held
  could be retired, or listed as `posted_against_skip`, for good, and its Contra could pair the other account's skip,
  although the restore may have removed the voucher. Now the row stays open (`held_elsewhere`, naming the covered batch),
  its Contra is an unverified candidate, and its days give no coverage. Every row that names such a batch is marked
  `"restored": true`, and the next step says that no step inside ComplyEaze Bridge settles it: do not withdraw the row or
  build or amend its rows, check in Tally and tell the person, and do not enter the transfer by hand or import that batch's
  file. Nothing else in the journal changes and no new record is written. Limits: the note is advice, which the code backs
  with refusals. A withdrawal of the row is refused (`withdraw_skip_paired`, `import_txn_already_posted` or
  `withdraw_skip_invalid`), so is an amendment of the covered batch (`import_amend_natively_posted`), and so is a new build
  that posts a row the covered batch holds (`import_txn_already_posted`). The covered statement's days stay blocked for the
  other account's coverage, so skips there stay `not_established` or `matched_unconfirmed`. A voucher that comes back in
  Tally after the restore is no longer flagged as `posted_against_skip`. Not run against a live TallyPrime.

- `voucher_presence` now checks a voucher type declared `manual` against the book's own numbering before it
  reads the window, because a voucher number is an identity only where the book's series for the type says
  Manual and prevents duplicate numbers. A call that declares a type manual now makes 20 more requests (34 become 54 on a small book: the
  company's master mark, then the voucher-type numbering) and is refused as a whole, with
  `presence_numbering_method_differs_from_book` and the types in `candidates`, when the book does not number a
  declared-manual type Manual with exactly one series that prevents duplicate numbers (the check now also reads the
  duplicate setting, and shows it in `book`) or holds no type of that exact name; a numbering that
  cannot be read refuses with the read's own code (`masters_education_unqualified` in Education mode, for
  example) or `presence_numbering_unreadable`. Calls that were answered before are refused in those cases.
  A sub-method printed on a Manual series is ignored (a sub-method printed twice still refuses the read as
  `field_invalid`). Names match exactly. Every type declared manual is
  checked, whether or not a proposal names it. The check reads the book's numbering as it is now and holds for
  the moment of the read. A call that declares no type manual reads no numbering and its result says
  `numbering_read` `not_read`; a call that does read it returns what the book said for each declared type, or
  only their counts when the rows would not fit the response budget. Request counts are from test doubles; not yet run against a live Tally.

- `ledger_masters` with `include_gst_details` now names Tally's `GSTINELIGIBLEITC` as `ineligible_itc` (true for Yes,
  false for No, null when the element is absent), where 0.6.0 named it `itc_eligible` and so told an assistant that
  an entry with a keyed "Eligible for Input Tax Credit: No" was eligible. Measured on one ledger created
  under Purchase Accounts in a synthetic book (TallyPrime 7.1): the screen's Yes read as the element absent and
  its No as Yes, so the element is an ineligibility flag (PARTIAL: one book, one release, one ledger, one dated
  entry; part of #1741). A Yes on any other ledger says nothing either way: the Sales ledgers of one captured book
  carry a Yes on their keyed entries, and Tally's own default entry (source `As per Company/Group`) carries one with
  no keyed No; neither was read against the screen. The value
  mapping is unchanged: do not read `true` as eligible.

- The tool list is answered whole under its own fixed ceiling of 1,000,000 bytes, apart from the cap on what a
  tool returns from the books (#1798). Before, a `BRIDGE_AGENT_MAX_BYTES` set below about 197 KB made the list
  itself too large (`agent_response_too_large`) and left the assistant with no tools. Every tool answer still
  keeps the configured cap, and a test holds the tool list under 225,000 bytes.

- A ledger name that is an alias of another ledger is now named as one (part of #1688). `validate_masters` and the refusal of a
  build (`masters_not_exact`) add `alias_of` to a requested name that is, character for character, one alias of exactly one
  live ledger, naming that ledger as Tally lists it, and say that an operator should confirm it is the ledger meant before
  the name is replaced; ComplyEaze Bridge still never sends an alias. The check for two ledgers whose names read alike now
  counts a ledger's aliases too (`through_alias` names the alias). Before, a name that was one ledger's own name and
  folded equal to another ledger's alias matched the first exactly and could be built and posted; it is now refused as a
  twin, and so is a batch built or approved before this change that names it, at post or in the queue
  (`ledger_has_folded_twin`), until the alias or the name is changed in Tally and the batch is built again. A name that is
  only an alias used to read as missing or near. On Tally's own screen an alias equal to another ledger's name was
  refused in the cases tried, so such a twin is probably rare (PARTIAL: one company; lowercase not tried). Not yet run
  against Tally through the built tool.

- Checking a native post with `verify_import` no longer records a rollback when Tally leaves the company's voucher mark
  out of its answer twice in a row while the check's own window read did see a mark. That case is now refused as
  `voucher_mark_contradiction`, with nothing recorded, and the refusal says to verify again once. A window read that saw
  no mark, or a mark of 0, records mark 0 as before. No omission of this kind has been seen from a live Tally (part of #1050, fixes #1819, #1853).

- `audit_test_run` now refuses a company whose books begin after the audit period ends (`C5-books-from`) before it
  reads the books (it first asks Tally which companies are loaded); before, it read the whole book first and refused afterwards. The refusal's code is
  unchanged and it still may not be retried. Books that begin inside the period (a first year) still read (fixes #1934).

- When a row is refused as already posted and `verify_import` reports the blocking batch `book_rolled_back` (the
  company's voucher mark was seen below that batch's), the refusal's next step now says that ComplyEaze Bridge cannot write the row
  again and cannot tell whether its voucher is gone, and to tell the user (#1038).

- On Windows, the approval box for a post, a review and a stop release is now measured when it opens. A box that is not
  wholly inside the screen it opened on is cancelled and never answered, whatever key was pressed, and the call is
  refused as `import_review_too_large` (the post) or `ack_review_too_large` (the review and the release), with
  nothing sent to Tally. Before, Windows left a box taller than the screen with its buttons below the edge, where
  a person could press `Y` on lines they could not see. A Windows that cannot be made per-monitor DPI aware shows no
  box at all, and the call is refused as `import_approval_unavailable` (`ack_review_unavailable` for the review and
  the release). A long batch or a small or highly scaled screen can now be refused at the box where it was shown
  before; a larger or less scaled screen, or a shorter narration, helps, and the `post_import` and
  `acknowledge_post_review` texts say so. A saved batch or Journal whose window is too large to show whole is refused with a message that says so: nothing was sent, a larger or less magnified screen may help, a batch whose window listed its vouchers one line each is shown as totals only when asked again soon (see the totals paragraph below), and a batch can be split into parts (a single voucher this long cannot be posted from here). The refusal message was run in tests only. The box is also brought to the front when it
  opens, and its taskbar button flashes if Windows refuses the foreground to a background process (the flash was not observed). Measured on one
  Windows PC with a work area 1,030 pixels high (1920x1080 at 125 percent, one monitor), in a CI build of that change:
  boxes of 40 and 44 lines (935 and 1,011 pixels high) were shown and answered normally, and boxes of 48, 60, 80 and
  120 lines (1,087 to 2,455 pixels) were cancelled in about a quarter of a second, not seen on that PC, and refused. That screen
  holds about 44 lines. Scales of 100, 150 and 175 percent, 1366x768, a second monitor and the Windows text-size
  setting are not measured. The macOS alert is not covered by this change (part of #1780, #1832, #1879, #1889).

- An amendment now refuses a reference that differs from any build of its batch, whether added, changed or removed
  (`import_amend_voucher_reference_changed`, #1941). Before, an amendment that changed only a Journal's reference
  could read `posted_verified` once anything else altered the voucher, without its new reference ever being read
  back, and one that removed a reference could not clear it. Change a reference in Tally after the amendment is
  imported and verified.

- A native post refuses before it sends anything (`post_mark_unrecorded`) when its last read before the send does
  not show the company's voucher mark, instead of reading the missing mark as zero and binding its vouchers from it;
  the voucher was never misattributed, but it could be posted and then left for reconciliation. A read after the
  post that omits the mark is likewise no mark, so the post is bound from its own answer instead of being refused as a
  mark that went back, and its `post_location` reads `after_snapshot_unavailable` with
  `marks_readback_voucher_mark_omitted` instead of a step to zero. A company that has never held a voucher is still
  refused when the batch is built (fixes #1945).

- On Windows, a batch whose approval box with one line per voucher does not fit the screen is refused as before
  (`import_review_too_large`, nothing posted), and asked again it shows the totals only: the per-ledger totals, the
  company and the cautions, with the sentence that the voucher lines are not shown. Without this the same batch could
  only be split into parts. The approval is for that text, and a later call posts exactly it. It applies to a batch
  of two or more vouchers once its box was seen not to fit, for as long as an approval would be kept; one voucher and the
  review window have no shorter text and are refused as before. If the totals box does not fit either it is refused
  again, and a further ask shows the totals again for as long as an approval would be kept, so the batch has to
  be split. Run on a real screen in the lab for the check that finds the box too tall (part of
  #1780); the second ask and the posting after it have been run in tests only.

- Dependencies were updated to clear the advisories RUSTSEC-2026-0382 and RUSTSEC-2026-0383 (#1793) and two more (#1849), and `zip` moved to 9.0.5 (#1782).

**What works differently**

- The three cash-line answers ComplyEaze Bridge cannot build yet (cash paid straight to someone, a deposit from
  the cash box, unbooked cash sales) no longer say to record the line by hand or to record the sales in Tally
  first; each keeps its reason and offers `dont_know`, which builds the line to the suspense ledger, marked for
  the CA to move (part of #1709).

- On macOS the first line of the approval alert (what each button does) is no longer broken by ComplyEaze Bridge at 100 characters: the alert wraps its own text, so a break of ours was wrapped a second time and left a ragged second row. The Windows box keeps its 100-character lines. That change does not alter the box's size caps (part of #1780, #1854).

- The next steps for five situations no longer send a person to enter a voucher in Tally themselves; each now says what
  ComplyEaze Bridge does and that it cannot put the voucher in the book yet: a voucher Tally reported as not
  created (and one later not found after that report), a row a rejected batch carried, a statement day that is
  partly posted, the extra row of same-day twins, and the first voucher of a company that has never held one
  (part of #1709).

- A saved batch built while voucher posting is off now says whether turning on **Allow voucher posting** would give
  it a route: the build checks the batch as `post_import` would at the batch limit of 50, and the assistant is told
  to say that, with the setting on (it means accepting the limits in Terms of Use section 9.2), ComplyEaze Bridge
  would check the batch again and ask for approval, and may still refuse it; the file import stays the route until
  then. Every text that offers both routes says to use one only: once the file is imported, do not post the batch,
  and once the batch is posted, do not import the file (an import after a post is not checked against it). With
  posting on, a batch `post_import` can take no longer suggests importing its file as well; a batch over 50 is told to
  be built again in batches of at most 50, and never to import the original file once one of them is posted or
  imported. The desktop review names the assistant's batch posting for a saved batch it cannot post, says a saved
  invoice is posted only that way, and no longer suggests choosing a Journal without its number. Wording only otherwise; the file routes for a voucher that carries
  its own number and for text the approval window cannot show are unchanged (part of #1709).

- The approval window, and the review window, now open with a line that says what each answer does, before any
  detail (part of #1780). On Windows, whose box takes the keyboard, it reads "Press Y (or click Yes) to post this
  voucher into Tally. Press N (or click No) to cancel. Nothing is posted until you answer." (a batch says "these N
  vouchers"; the review window says "record that you reviewed" and "recorded"); on a Mac it names the buttons to click.
  The invoice-stop release, which shares the review window, says that Yes releases the stop, under its own
  statement. The text is wrapped under 100 characters a line, so the window's caps grew by its size (3 lines and 150 characters at most), and every text that fitted the caps before still fits them, though a box 3 lines taller can now be too tall for a screen it fitted before. The `post_import` tool text carries the same
  sentence, with a note to say it before calling. The keys are unchanged: Y and N still answer the Windows window.
  The new first line has not been seen above Tally, and on a non-English Windows the box's own button letters may
  differ, which was not measured. Boxes of the new size were shown on one Windows screen (see the Windows box
  paragraph below).

- A read refused because the company it names is not loaded in Tally, was renamed since `list_companies`, or is
  listed more than once now says so and says to open the company in Tally and ask again (#1699). Before,
  an assistant saw only an operation code such as `ledger_movement_read_failed`, and the all-companies
  outstandings sweep and the All clients screen reported the same cases as a failed currency probe. The refusal
  carries `company_identity_absent_or_ambiguous`, `company_identity_presentation_collision` or
  `company_identity_not_matched` as its cause. Nothing about what is refused changes; only the reason and the
  next step are named. Not yet run by us against a renamed company in a live Tally. A presentation collision (`company_identity_presentation_collision`) is named as the cause but carries no next step.

- A read that stops because the company's books changed in Tally while it was being read
  (`voucher_window_changed_during_read` and the `*_extent_changed` codes) now says that nothing is wrong with the
  request and to run the same request again once the saving or exporting in Tally has finished; a later page of a
  held listing that cannot be served (`listing_snapshot_changed`) says to start again from the first page without
  `snapshot_id` (part of #1633). One GST return export in TallyPrime 7.1 Silver raised the voucher mark by 3
  with no voucher change, which is why a routine export can trip these. Not changed: a correct post that reads as
  unverified after such an export is still open on #1633.

- A snapshot request that reaches its 20-second deadline is recorded as `request_deadline_exceeded` instead of
  `tally_unreachable` (#1671), so a person is no longer told to check that Tally is running when it was
  only slow. The deadline and the rule that such a request is not sent again are unchanged. Covered by tests only.

- The Proof of Sync screen no longer shows a rejected-row count (#1634): it could only ever read zero, because only
  accepted observations enter the local mirror. The summary line drops the "rejected rows" clause and the table
  column reads "Accepted returned rows"; a run that rejected source records is still shown by its gap. The count
  stays in the proof's data and in the export.

- The summary of a post whose binding was refused now says that ComplyEaze Bridge cannot close the batch or amend it
  (part of #1039). Before, it said only that the batch "stays open". The notice for a multi-voucher batch whose
  voucher mark step was not confirmed no longer suggests that recording a review with `acknowledge_post_review`
  clears it: the record is kept on this computer and the notice stays. No behaviour changes. A way to close a batch
  with a refused binding from ComplyEaze Bridge is not built: a person's confirmation cannot stand for a voucher's
  identity before it is measured whether Tally names the post's own vouchers when another user is writing.

- When a withdrawn skipped transfer row's other statement may sit in a batch that is not verified
  (`other_side_unverified`), the next step now offers to build that statement again only if every batch named for
  the row was never sent to Tally, `verify_import` listed every one of their vouchers as `not_found`, and the person
  confirms none of their files was ever imported; if any voucher was found or a file was imported, it says to check
  in Tally which of their vouchers are there (#1820). Wording only.

- The desktop app now shows a window asking the person to accept the Terms of Use, before it shows anything else and
  before it reads from or sends anything to Tally. Until the box for the current Terms version is ticked and accepted,
  every read and every send from the app is refused. The acceptance is a line in a small file in the app's own data
  folder (the Terms version, the time and that it came from the desktop app); if that file cannot be read or written the
  window stays closed and names the file. A new Terms version asks again. Reads of data already kept on the machine and
  saved drafts are not gated. The extension and the MCP server keep their own acceptance, unchanged. Tested with unit,
  component and browser tests; the packaged app and the acceptance window were not run (part of #1106).

- When the local mirror refuses to commit a snapshot run because of something in the run's own saved batch, the Proof of
  Sync screen now says so and offers no resume. Before, it showed the same "mirror failed" title as a locked or full
  database and offered a resume, which could never succeed, because the same commit is refused every time. The run
  stays listed and inspectable: its status line reads "Commit refused: a resume cannot succeed. Start a new read.", the
  recent-runs table reads "Commit refused", and nothing is lost from the last verified baseline. Other mirror
  failures keep their title and their resume. After the app is restarted, a refused run shows again as one that can be
  resumed until the resume is tried, and is then refused again with the new text. Only unit and component tests ran; no
  run was refused this way by a real mirror, since no release writes such a batch (fixes #1677, #1900).

- A call to `audit_test_run` that names a test the tool does not admit is refused as `argument_invalid:test`, before any
  request to Tally; the guide now names that code (it had named `audit_test_not_admitted`, which a call could not reach),
  and the list of admitted tests in the tool's schema is taken from the tool's own list instead of a second copy
  (fixes #1915).

- The receipt of a failed call now keeps the error code of a tax-audit refusal (a rule code such as
  `BIND-GROUP-UNKNOWN`) and of a refused argument (`argument_invalid:test`). Before, only lower-case tool codes were kept,
  so those two kinds of refusal left a receipt with no error. A code is kept only if it is one of three kinds (a
  lower-case tool code, or one of two closed lists) and is at most 64 bytes; anything else is still dropped. The rule codes are the 61 tax-audit codes, and an argument name is kept only when one of the registered tools publishes it (fixes #1916, #1923).

- The entry for a change that is not yet in a published build is now one file under `docs/changelog/unreleased/`
  (a folder each for added, changed and removed), not lines added to `CHANGELOG.md`, so two changes no longer
  edit the same lines. The install site's "What changed" page and the release notes list these files under
  the unreleased changes, and cutting a release moves them into its section (#1929).

- The window that asks a person to release an invoice stop is titled for that, "ComplyEaze Bridge — release the
  invoice stop" (on Windows, "… release the invoice stop for this company?"), and on a Mac its button reads "Release
  the stop". It was titled as recording a review of one voucher, with the review's "I reviewed it" button. It runs in
  its own dialog mode and answers with its own token, so a review window cannot answer it (#1828).

- When `vouchers`, `voucher_presence`, `ledger_movement` or a register comes back empty for a window on a large
  book, the second look at the window widened by a day each side now counts it instead of reading its vouchers,
  so it usually sends fewer requests (#1240; the same number where the count takes several requests and the
  widened window is small). The labels do not change: an empty window with vouchers on a neighbouring day is
  still `partial` (`empty_window_wider_read_nonempty`), and a voucher inside the window still refuses the read.
  What differs: a count that took several requests is refused (`voucher_window_changed_during_read`) when the
  company's marks differ from the ones the first read opened on, as a divided read is; a count that took one
  request is not bracketed, where a widened window that used to be read in several parts was; and because no
  vouchers are read, a refusal that came from reading them (a part that failed or was not admitted, a part
  Education cannot serve, a read plan that could not be made or went over its allowance, a disagreement between
  the count and the read) no longer happens. A count of several requests can still be refused when it passes
  its own allowance. A book that fits one request is read as before. One live run on TallyPrime 7.2 gave the same answers and
  the request counts the test doubles predict, except one window of a book counted in several requests whose
  neighbouring day held vouchers, which sent fewer requests than predicted (#1999).

- A read for a company that is not among those open in Tally, refused as `company_identity_not_found`, now carries
  a next step: call `list_companies`, use an ID it returns, and ask the user to open the company in Tally if it is
  not listed. The refusal itself is unchanged. Checked on a test double, not a live Tally (#1903).

- The Download page, the install guide and the top of each release now help a person check a download. The Download
  page gives the two commands that print a file's SHA-256 code (`certutil -hashfile "<file>" SHA256` on Windows,
  `shasum -a 256 "<file>"` on a Mac) with the file name in double quotes, says to run them in the Downloads folder,
  shows beside each download the code as GitHub reports it, when GitHub returns one, and says where a newer version
  appears. A GitHub release now opens with a short header (which file to take, the install guide, the checksum step)
  before the change list. What an older release needs moved to the last section of the Download page, below the
  steps for a new install, and the README gained an "At a glance" block with what can be asked and what cannot yet.
  This release opens with that header, and the Download page changes when its deploy is next run (part of
  #1148, #1046, #1785, #1786, #1788, #1790, #1837, #1850, #1882).

- The install guide and the Download page now say that a Windows computer needs the Microsoft Visual C++
  Redistributable (x64) to start ComplyEaze Bridge, and link Microsoft's own installer for it. The package does not
  carry it: the Windows program of 0.6.0 uses it, and a computer without it should not start the program (no clean Windows computer was tried). Only the
  texts changed, not the build (fixes #1860, #1861).

- The `NOTICE` file in every package, and the README's License section, now say that Tally and TallyPrime are trade
  marks of Tally Solutions Pvt. Ltd., that ComplyEaze Bridge uses the names only to describe compatibility, and that
  it is not affiliated with or endorsed by Tally Solutions (part of #1865, #1868). `NOTICE` also names the copyright
  holders of the code in the AWS-LC cryptography library that the packages link, as that library's own licence file
  states them (BoringSSL, OpenSSL-derived code, fiat-crypto, s2n-bignum and the Jitter Entropy library), for
  version 0.44.0 of the crate; a newer version should be read against it (fixes #1858, #1859).

- The amendment warning now says that saving a voucher unchanged in Tally, or cancelling it, moves its ALTERID (one run
  each), and so did changing a bill allocation's reference, adding a cost-centre allocation and setting a bank-reconciliation
  date (one run each), where it said an edit made on Tally's own screens had not been shown to move it; the amendment check
  itself is unchanged (part of #685, #1927).

- The party and ledger master export now reads the CA's stored Schedule III grouping decisions from the encrypted local
  mirror, so the first export after the app starts can raise the macOS keychain prompt where it did not before. When
  the decisions cannot be read, the workbook says "Could not be read" and is NOT FINAL instead of saying none were
  applied; and when the mirror holds decisions for this company GUID under another company number or books-from date
  (for example books restored into another data folder), the export reports that count, is NOT FINAL, and applies none
  of them (part of #737, #1878).

- `parse_bank_statement`'s tool text now tells the assistant it may ask the person once about all the open cash lines, listing
  each line's date and amount, then answer each line on its own and ask again about any line the person did not confirm;
  one answer is never applied to another line for them. What the tool returns is unchanged: an open cash line still carries
  only its id, date, amount and party name. Not run against a live TallyPrime (part of #1761).

- Checking a native post with `verify_import` no longer binds the post to the vouchers in its range of Tally AlterIDs when
  the post's recorded answer reported `STATUS` 0, whatever the answer's counters say. The check now reports
  `post_span_binding` as refused with the code `span_response_status_failure`; the post itself already treated that answer
  as not clean (#2008).

**What has been run against a live TallyPrime, and what has not**

- `voucher_read_cost` (#1465): run once on the published 0.6.0 build (the code is unchanged since) on one synthetic company with a voucher mark of 68. The preview said one census read for `vouchers` and none for `ledger_movement` and `verify_import`, with a floor of 0 seconds; its own requests were identity, marks and status reads only. One `vouchers` call over a one-day window (11 vouchers) came back `complete` with one census request, equal to the preview's. The floor formula was not exercised (one census read gives a floor of 0). The refusal above a voucher mark of 2,097,152 was not measured (no book that size).
- Approval window size (#1780, #1832, #1879): run on one Windows screen only (1920x1080 at 125 percent, one monitor); see "What works differently" for the box sizes shown and refused. Not run: Windows at 1366x768 or at 100 or 150 percent, a second monitor, a moved box. Not run on a Mac against this build. A capture of the Mac alert on 23 October, before #1854 (a 1512x982-point screen) showed the one-voucher worst-case box fitting and the 43-line batch box not fitting and not scrolling; this build was not measured on a Mac.
- The first line of the approval window (#1832): seen above Tally once, on the approval window of the approved post in the live pass of writes; not looked at on any other window.
- Empty-window count (#1240, #1999): one live run on TallyPrime 7.2 (on a CI build of #1897) gave the same answers and the request counts the test doubles predict, except one window of a book counted in several requests whose neighbouring day held vouchers, which sent fewer requests than predicted. The live pass of writes and the restore row (#1814) also ran on 7.2; no other read did.
- Reads on TallyPrime 7.2 (`ineligible_itc` #1741, voucher numbering check #1704, the not-loaded cause #1787): not run. The 0.6.1 reads were measured on TallyPrime 7.1, apart from the empty-window count above; the live pass of writes, the restore row and that count ran on 7.2.
- `audit_test_run` (#1733): not run against a live TallyPrime in this build, replayed from the engine's synthetic book only.
- `statement_tie_out` (#1211): not run.
- Voucher-type numbering (#724): run on one approved post and one refusal of a Receipt under Manual numbering (see the live pass of writes below). Not covered: Gold, Contra under Manual, a type that already held numbered vouchers.
- A restore remembered after a native post (#1814): run on 27 October 2026 on master build fd6f7c0d (of 26 October), which holds #1814 (not the cut build), on TallyPrime 7.2 Silver, on one synthetic company (BRIDGE TRANSFER PAIR LAB), ten steps. After the book was restored from a backup, the verification of the posted batch named the restore (`book_rolled_back`), and kept naming it after a Bridge restart and after a voucher with the same number was keyed by hand; its records were unchanged across the restart, and the book ended at its starting mark.
- Guidance after a post Tally refused (#1108): the wording is corrected on every path that can be reached (one voucher, a whole batch, the summary line, a later check, a post whose answer was not clean). It was not run live: no way is known to make Tally refuse a post after Bridge's own checks pass, so the issue stays open for 0.7.0. A ledger rename and back is not run. Labelling a batch Tally created only part of is not built.
- Port locks on a Microsoft Store install of Claude Desktop (#1584) and where the import file really is on a Store install (#1459): not run.
- Voucher ranges and direct posts on a book of about 30,000 vouchers (#1029) and a batch post that completes in Tally but whose reply is not kept (#852): not run.
- Not run on a live TallyPrime for this build, with only captures or tests behind them: `gst_uncertain_vouchers`, `negative_stock`, `negative_ledgers`, `masters` `stock_items`, the closing stock used by `profit_and_loss` and `balance_sheet` (#692), cost-centre allocations in voucher reads (#1231), ledger aliases (#1688), the check of a statement batch when it is sent (#1714), except the one refusal named in the live pass of writes below, the refusal of a post whose last read shows no voucher mark (#1945), the amendment reference check (#1941) and the invoice-stop window title (#1828).

**Live pass of writes:** run on 27 October 2026 on the CI package built from commit 67f647e1, the commit this release is cut from; that package reports 0.6.0 as its own version; Tally was TallyPrime 7.2 Silver. Two rows ran, each on its own synthetic company. (1) On BRIDGE NUMBERADM LAB: one approved post under automatic voucher-type numbering, and the preparing of a post refused under manual numbering (#724): both passed. For the approved post, two Receipts were keyed first (numbers 1 and 2). The approved batch post, answered by the person in the approval window, posted and verified (posted_verified, the voucher mark up by one step, the post bound to its voucher, no rollback and no mark contradiction), took number 3 and left the two keyed vouchers unchanged; the approval window's first line (#1832) was seen above Tally. For the refusal, with the Receipt type set to Manual numbering, building the post was refused before anything was sent (import_voucher_numbering_not_automatic, refused type Receipt), the voucher mark and the vouchers were unchanged, and the company was restored from its backup afterwards. (2) On BRIDGE TIE OUT LAB 3: the send-time check of a statement batch that would post a row which a statement of this company skipped as carried by the other account's Contra (#1714): The batch was refused at send, in 0.17 seconds, as skip_row_paired_elsewhere. Its next step says nothing was sent to Tally, any approval of the batch is withdrawn and the batch must be built again. No attempt was recorded; the voucher mark, the batch journal and the skip records were unchanged, and no voucher appeared on that day. A control post of a fresh one-voucher batch was not completed: its approval window timed out unanswered and nothing was posted. The approved post in (1) shows a post going through on the same build. The check against Contras was not run live. The file that ran is the CI package; the published file is built again by the release workflow and is checked after publication against the release's checksum, with no write.

**Known limits**

- **Not run on a live TallyPrime for this build (unless the lines above say otherwise):** anything that writes to Tally other than the approved post and the refused statement batch of the live pass of writes above, including a batch of Journals, a batch near 50, a post Tally refused or that landed partly, amendments, the review acknowledgement window (no voucher raised a doubt), a post that needs an On Account approval (a bill-wise party; the build refuses one as designed) and an import file taken to Tally's Import menu; Sales invoice posting and ledger creation (not offered); the desktop app and its Terms of Use window (unit, component and browser tests only; the packaged app was not run); Gold; macOS; any Windows or Tally release other than Windows 10 with TallyPrime 7.1 Silver for the reads and TallyPrime 7.2 Silver for the three rows that ran on it (the live pass of writes, the restore row and the empty-window count; the other reads listed above have not run on 7.2); a second monitor. The earlier live passes and the 0.6.0 package check are recorded in the 0.6.0 section below, and say nothing about the file in this release.
- **The file tested is not the file that will be tagged.** Each live row names the build it ran on: `voucher_read_cost` on the published 0.6.0 build, the window fit on a CI build of #1879, the empty-window count (#1240) on a CI build of #1897, the restore row (#1814) on master build fd6f7c0d; the write pass ran on the CI package of the commit the release is cut from, which reports 0.6.0 as its own version. The release workflow builds `bridge_mcp` again on its own, with a plain release build of that binary, where the CI package takes it from the Tauri build; the two files are the same source but not byte-identical, and no build is claimed to be reproducible. The published file is checked after publication against the release's checksum, with no write.
- **Approval window fit.** Measured on one Windows screen (1920x1080 at 125 percent). On other screens a window can be refused as too large, or fit, where it was not measured; on a Mac the alert's size is not checked and a batch window can be taller than the screen (#1780).
- The `headline` of a batch post has not been seen on a live Tally post (#1090, #1532).
- What has been run against a real TallyPrime is listed in the README.

## [0.6.0] - 2026-10-16

### In plain words: ComplyEaze Bridge 0.6.0, since 0.5.1 (16 Oct 2026)

These changes are in ComplyEaze Bridge 0.6.0. Most lines name the pull
requests or issues they come from. Like every build so far, it is not yet
code-signed.

**Should I upgrade?**

- **If you use posting, or want to post batches: yes, read this first.** One
  setting, **Allow voucher posting, one voucher or a batch of up to 50**, now
  covers a single voucher and a saved batch of 2 to 50. It is a new setting and
  is off by default: if you turned posting on in 0.5.1 or earlier, turn it on
  again after installing 0.6.0. Every post still waits for your approval in a
  separate window, and for a batch you approve its summary (#1090, #1630).
- **If you use the bank-statement tool:** a skipped row must now name the other
  account's ledger, and a transfer between two of your own accounts is checked so
  that it is not booked twice (#1585, #1625). Several refusals are new; see
  "What works differently" below.
- **If you only read your books:** the new reads below are the reason to update,
  if any. Nothing in 0.6.0 writes to Tally unless posting is on.
- **How:** ComplyEaze Bridge does not update itself. Follow the
  [installation guide](https://github.com/ComplyEaze/bridge/blob/master/docs/agent/INSTALL.md):
  (1) close any other program that runs ComplyEaze Bridge; (2) in Claude Desktop,
  Settings, Extensions, install the new file; (3) keep ComplyEaze Bridge's data
  folder; (4) check that the extension shows 0.6.0, that the Terms setting is
  ticked, and that **Allow voucher posting, one voucher or a batch of up to 50**
  is as you want it; (5) quit Claude Desktop completely and reopen it.

**What you can do now**

- `month_register` is a new read-only tool: Tally's own month-wise Sales Register or Purchase Register for whole
  months, one row per month with Tally's debit, credit and closing columns exactly as printed. A window that is not
  whole months, or that starts before the books, is refused. The rows are not tied to the vouchers, to the trial
  balance or to `sales_register` and `purchase_register`, and not compared with Tally's register screen. Its request
  renders as the lab sent it, and its parser reads the answers BRIDGE PILOT LAB gave on 13 October 2026 (one
  synthetic book, TallyPrime 7.1 Silver). The Receipt and Payment Registers are not read (part of #1232, #1759).

- A transfer between two of a firm's own bank accounts, printed on both statements and booked once, is now followed
  from one statement to the other. A `skip` row must name the other account's ledger (a `skip` with no ledger, or one
  naming the statement's own bank ledger or the suspense ledger, is refused at its row), and the parse records each
  row's network transfer reference and printed instrument, for the transfer references both banks print. A build
  records each Contra and each skipped row, and a successful build and the first page of a verification carry a
  `transfers` section: each skipped row is paired with the other account's Contra by reference, then by date, or
  listed in one of its open states (not found, ambiguous, other side unverified or not established, among others), with counts only, no amount, reference,
  party or ledger. A build that would book a transfer a second time is refused (part of #1392, #1585, #1592, #1625).
  Run once, on 15 October 2026, on a Windows lab PC (TallyPrime 7.1 Silver, one synthetic company) with the Contras
  imported by hand. Not measured: which banks print the same reference on both statements of a transfer, how many
  days apart two banks print one transfer, and posting a batch that holds a transfer with `post_import`.

- `masters` with `kind` `voucher_types` now returns, on each row, `series_numbering`: the numbering method
  (`automatic`, `manual`, `automatic_manual_override`, or the raw text under `unrecognised`, never read as
  `manual`) and whether duplicate numbers are prevented, read at the number-series level from a second paired
  read of the voucher-type export and joined to the row by GUID with the name as a second equality; a type with no
  series or several returns `series_count` and nothing is read from it, and a series missing or repeating a field
  returns `field_invalid`. Under `automatic`, and under no other method, a row also carries `sub_method`, the
  series' own setting for the numbers of vouchers already entered: `auto_retain` ("Retain Original Voucher No."),
  `auto_renumber` ("Renumber Vouchers"), `absent` where the series printed none, or the raw text under
  `unrecognised`. A type found in one read only (`masters_numbering_names_differ`), or a GUID and name that disagree between the
  two reads (`masters_numbering_identity_differs`), refuses the whole read; so does the numbering read changing
  between its two reads (`masters_numbering_collection_changed`) and a numbering answer whose row count or size
  breaks the mark's premise (`masters_bound_premise_violated`), and one that does not parse (a missing or repeated
  GUID among the causes) refuses at once; a moved book is reported as `masters_extent_changed` in place of a
  join or premise refusal. The type-level `numbering_method` stays as it was, with its limitation.
  Measured on one synthetic book (TallyPrime 7.1 Silver, Windows): 24 voucher types, each with one series named
  `Default`, of which three (Journal Automatic, Payment Manual with duplicates prevented, Receipt Manual with
  duplicates not prevented) were checked against Tally's screen. Automatic (Manual Override) through this tool (it is
  mapped from the series text measured on a synthetic book on 13 October 2026), more than one series, Multi-user
  Auto, a single series not named `Default`, a type created new and other releases are not measured. The two
  requests were run live as one tool call on 14 October 2026 on two synthetic books (TallyPrime 7.1 Silver,
  Windows; 24 voucher types each, exactly one series per type, its name not read): the read was not refused, and on
  the six types whose numbering was set or checked on Tally's screen it returned that numbering (Manual with
  duplicates prevented, Manual with duplicates not prevented, Automatic); the other types, left at a new company's
  defaults and not checked on screen, read Automatic (part of #724). `sub_method` was measured on a second
  synthetic book of the same release and edition, on one voucher type (Receipt, set to "Renumber Vouchers" and
  back); any other wording, a series that prints none, Gold and other releases are not measured.

- `ledger_masters` with `fields=compliance` takes an opt-in `include_gst_details` (a boolean, default false; with
  any other `fields` it is refused as `ledger_masters_gst_details_requires_compliance`): each ledger's row then
  carries `compliance.gst_details`, Tally's dated ledger-level GST rate details as sent, per entry and state row,
  with a rate that is `absent`, `empty`, `zero` or a `value` (the text as sent and the decimal), never merged
  (part of #1342). HSN/SAC and type of supply are not returned because no capture shows them, the place of
  supply stays in the registration history, and slab rates are only flagged. Without the flag the requests, the
  sizing and the result are unchanged. With it the read is admitted at 8,000 bytes a ledger instead of 3,750, a
  margin over the one capture (a ledger with an empty list 2,138 to 2,704 bytes; one dated entry 5,248 to 5,394; UTF-16 bytes of the ledger's own element),
  not a measured ratio; a large book may be read in more parts or refused as too large. Measured on one 14-ledger lab book of TallyPrime 7.1 Silver and not yet run through this tool on a
  live Tally.

- `stock_summary` now returns an item's closing quantity, `closing.quantity` with its `amount` and `unit`, where
  Tally's own plain Stock Summary has exactly one line with the item's name and shows the same quantity, unit
  and amount; every other item says why none is returned (`none_sent`, `unread`, `inside_stock_group`,
  `report_has_no_line`, `report_name_not_unique`, `report_differs`) (part of #1233). An item inside a stock group
  returns none: in the captures, the report listed only what sits directly under the root (one capture holds
  only groups, the other only items under the root; none holds both). Checked against two real captures of one
  synthetic book taken after a sale of an item that had no stock: Tally kept it as `-5 Nos` with no value, in the
  item rows and in the report alike, and the parser and the tool's tests read it. Then run through the tool on a
  live Tally by an outside contributor, on a new synthetic book built to the same list (TallyPrime 7.1 Silver):
  the answer equalled Tally's own Stock Summary for 31 March 2026 on every point, the closing quantity and unit
  of both items, the empty value of the negative one, and the book's stock-item count (2 of 2). Not measured: a
  book whose report lists a group and an item under the root side by side, an item inside a stock group, a
  compound unit, other dates, other releases, macOS, and a group that carries an item's name.

- `statement_tie_out` is a new read-only tool: after `parse_bank_statement`, it says whether the bank ledger in
  Tally stands where the statement says it stood at the start and at the end of the statement's dates, as three
  gaps (book less statement): the opening gap, the closing gap, and the change in between, before an import or
  after one made by hand. It covers a whole statement, or whole calendar months of it, names no cause and writes
  nothing. It returns the two dates and the gaps, not the balances, though a gap plus the statement's balance at
  that date (for a whole statement, the balance you supplied) gives the book's. The parse's proposals file now also
  records the statement's first and last dates (part of #1211). It was run once in the lab on 14 October
  2026 (TallyPrime 7.1 Silver, one synthetic company): whole months before and after their post, a whole
  statement whose last month was not posted, and reads after the ledger's last voucher all gave the gaps
  expected; a read at a date in the middle of a month is not measured (see the pull request).
- `statement_tie_out` can check whole calendar months of a longer statement: a file parsed with `from` and `to` (or
  one of them) from the 1st of a month to the last day of a month, within the statement's rows, is compared with the
  book at those dates, against the statement's balance there as worked out from its own rows. Any other narrowed
  window is `not_established` (`window_not_whole_calendar_months`, `window_beyond_statement_rows`). With the book's
  figures, the gaps then give the statement's balance at those month ends and its net over those months: never a
  part of a month except the part-months at the statement's own start and end (whose net a month-end balance gives
  together with the opening and closing supplied to the parse), though a month in which every other row's amount is already known (one row alone, or one row
  beside open cash lines, whose amounts the parse returns) gives that row's amount (part of #1211). No month window
  has been run against a live book.

- `gst_status` reads Tally's own GSTR-1 status of the vouchers of one voucher type in a window. For
  each voucher it says whether Tally lists it as included, uncertain or excluded, or does not list it in the
  return (measured for Receipts only), or is cancelled, optional or post-dated (decided first from Tally's own flags, never
  counted as included, with what its status flags read returned beside it; #1655, #1664), and whether a person accepted it as it stands (Accept As Is). The status is Tally's own flag and not a
  judgement of ComplyEaze Bridge; the reason for a status is on Tally's screen only, so none is returned. A voucher
  whose flags are absent or odd, or whose deleted flag reads Yes (never seen), is returned as unread, never as included. The voucher type is named exactly (a
  type of the same class under another name is not read), a window is at most 93 days, and a book whose voucher
  mark is above 25,000 is refused. Measured on one synthetic book of TallyPrime 7.1 Silver, for the unregistered
  buyer of its Sales invoices; a registered buyer, other releases and Gold are not measured, and an excluded
  voucher has not been observed.

- `company_features` reads three settings that a company's record in Tally holds today (cost centres, GST
  and batch-wise stock) and its currency symbol (part of #1231). Each setting is yes, no or
  not reported, and says what it rests on: each of the three was compared with Tally's own
  F11 screen on three synthetic books (nine values, all equal; not this company). A setting Tally did not send is never read as no, and an empty or odd one refuses the read. The
  settings are the ones Tally holds today, not what the books contain or what they were during a year. The
  currency is a symbol, not a currency code. Measured on three synthetic books of one TallyPrime 7.1 Silver;
  a setting changed inside a sitting, Education mode and other releases are not measured.

- `voucher_read_cost` says, before a call, the most census reads one call of `vouchers`, `ledger_movement` or
  `verify_import` will send on a company (a read can be refused before its census ends), and the least time a call
  that is not refused earlier spends on the gaps between them (part of #1465, after #1239). It reads the company's
  identity and then its marks (the opening every windowed call makes) and sends no census and no voucher request. The census follows the book's voucher mark, not the window, so the figure is for
  one call: a narrower window does not lower it, and N calls pay it N times. The count and the floor are derived
  from the mark and the read's own rules, not measured on the book, and they leave out the data parts and the
  wider re-read an empty window gets in `vouchers` and `ledger_movement`; no estimate is made. Beside them are the
  whole calls the protocol reference records, with their census reads and data parts (one book, about 250,000
  vouchers, 31 census reads), as data marked as not a bound. Nothing is refused on it. Not measured: Windows hosts, and any book other than that one.

- `parse_bank_statement` reads Bank of Baroda statements in one measured layout (`bank: "bob"`; Part of #1457). The layout was measured on
  two real statements, held privately, and the profile follows what was seen there:
  the statements print neither an opening nor a closing balance nor totals, so both balances come from you, and
  every page must print `Page N of M` and every page after the first must start with a row. They print the account
  number masked (first and last three digits clear), so `account_label` must carry the whole number; only those six
  digits are compared, the masked middle cannot be. With no printed totals those balances and the footers are the
  only checks on which rows were read: missing rows whose amounts net to zero would not be seen. They print the newest row first; the rows are read oldest first and
  numbered that way. A statement is read as Bank of Baroda only if page 1 prints an IFSC Code with the bank's prefix on its `IFSC Code:` line; otherwise it is refused as `statement_bank_not_recognised`. The measured statements cut a narration at about 50 characters, so a name can end mid-word. A party is named
  only where the shape settles it: a UPI row names its VPA (when the VPA is whole), NEFT and RTGS rows name what
  follows the UTR (not when a line was broken at the cell edge inside the narration), and bank charges and a cash deposit are recognised by their wording; a loan recovery is named as printed, loan number included, so two loans are two parties. Two payers whose names agree up to the cut read as one party. IMPS,
  mobile-banking and internet-banking rows, interest and every other wording go to suspense. A statement that
  opens without a password needs `password_file` to be an empty file (on a Mac it must be owned by you with no group or other permission bits; on Windows it must be owned by you or Administrators, and no one but you, SYSTEM and Administrators may read or change it). Not measured: other
  account types and branches, statements printed oldest first, other wordings, and a statement downloaded another way.
- `vouchers` can keep only the vouchers of a ledger whose narration ends in a suspense
  tag ComplyEaze Bridge writes (`suspense_tagged: true` with `ledger`, #810 slice 2):
  "purpose not confirmed" for a cash line answered "don't know", or "UNIDENTIFIED -
  reallocate from" the ledger for a party no mapping names; each item says which in
  `matched.suspense_tag`. It reads the tag from the narration, so a voucher whose
  narration was edited after the tag is not listed and the same text typed by hand
  matches: read the ledger without the filter too. Not measured: a tagged narration
  read back from a real Tally. The same tag test now also counts, in a build's `suspense_lines`, a voucher whose tag names its ledger in another case, spacing or hyphenation.

- `post_import` (after a send was read back) and `verify_import` now begin with a `headline` for a batch
  ComplyEaze Bridge sent: whether the batch is confirmed in Tally (a batch of several vouchers also says how
  many of its vouchers read back and how many are not confirmed), with the counts, and a count for each other
  status, in `batch`, counted from the result's own rows. A batch whose vouchers all read back but whose
  readback as a whole is not confirmed is not called confirmed, and neither is a batch of which only some
  vouchers read back: its lead says that they read back and that the batch is not confirmed. A batch not yet posted, a file imported by
  hand and a refusal before anything was sent have none. The result is unchanged (part of #1090).

**What works differently**

- One setting now covers all posting: with posting on, ComplyEaze Bridge posts one voucher per
  approval, or a saved batch of 2 to 50 Journals, or of 2 to 50 Payments, Receipts and Contras, after
  one approval of the batch's summary (#1090). In the extension it is a new setting, **Allow voucher
  posting, one voucher or a batch of up to 50**, off by default: it is a new setting, so if you turned posting on in 0.5.1 or
  earlier, turn it on again under the new text. On the command
  line, `BRIDGE_AGENT_ENABLE_WRITES` now admits batches too, and the separate
  `BRIDGE_AGENT_ENABLE_BATCH_POST` is retired: while it is set and posting is on, `post_import` refuses
  as `setting_retired` until you remove it. A batch saved earlier that was refused as more than one
  voucher now posts after its own approval.
- `post_import` and `build_import_xml` now refuse a voucher sent without a voucher number into a voucher type whose
  voucher numbering does not read Automatic, as `import_voucher_numbering_not_automatic`, and into one that reads
  Automatic without the sub-method `Auto Retain` (the screen's "Retain Original Voucher No."), as
  `import_voucher_numbering_not_retain`. Either names each such voucher type and what was read in `refused_types`,
  and a batch that holds both is refused as `import_voucher_numbering_not_automatic` (part of #724). Under Manual
  voucher numbering, a second voucher sent without a number was seen to take a number already used, or to be refused
  whole. With a voucher type set to "Renumber Vouchers" (the series reads `Auto Renumber`), Tally was seen to change
  the numbers of vouchers already in the book: at the switch, when a voucher with an earlier date was entered by
  hand, and when a voucher with an earlier date was sent by import without a number, without moving any voucher's
  AlterID and with the import's answer reading CREATED 1, ALTERED 0; a voucher dated after every other changed
  nothing (one synthetic book, one voucher type, TallyPrime 7.1 Silver). A series that prints no sub-method, or one
  ComplyEaze Bridge does not recognise, is refused the same way. An amendment's vouchers are checked the same
  way. It is refused before its own check of the book, so its next step says that
  whether the earlier batch's vouchers are in the book was not checked, and has the assistant call `verify_import`
  for that batch first: a voucher found in the book is never entered again, one that is not found is not built as a
  new voucher either, and the vouchers of the refused voucher types are left out until ComplyEaze Bridge numbers vouchers
  itself; no refusal sends anyone to enter a voucher in Tally. A Journal that carries its own voucher number (one built for Tally's Import menu)
  is checked by `build_import_xml` too: it is written into a voucher type that reads Manual with duplicate numbers
  prevented, where Tally keeps the number and refuses one a Journal already holds, or Automatic with `Auto Retain`,
  where Tally discards the number
  and assigns the next, and it is refused otherwise under the same two codes, Manual with duplicates allowed included,
  where a Payment carrying a held number was created as a second voucher. It is refused on a voucher type set
  to "Renumber Vouchers": there Tally was seen to discard the number a Receipt carried and give it the number of its
  place in date order, and the later-dated Receipts' numbers went up by one. So a build that holds only Journals carrying their own numbers,
  which made no numbering read before, now makes that read: it can fail with that read's own codes, or as `pre_import_mark_unobserved` when the
  company's master mark cannot be read, and it is
  refused on a book in Education mode, as a build of vouchers without numbers is. On the desktop's Journal
  screen the refusal names the voucher type and what was read of it. The next step, and a post's message naming
  the voucher types, are sent only where `BRIDGE_AGENT_MAX_BYTES` is at least 8,192; from 4,096 up to that a refused build
  still returns its `reason` and `refused_types`, and a refused post its code and `refused_types`. On a type set to "Renumber Vouchers",
  a file of more than two vouchers, a mixed file, a Journal that carries a number or is dated after every other,
  Payment and Contra are not measured; nor are Gold, other releases, a series not named Default and the
  same through ComplyEaze Bridge's own post: on a
  release, an edition or a book whose series prints no sub-method, or words it differently, every Automatic voucher
  type is refused.
- Before approval a post now reads the company's master mark and then the voucher numbering, and compares the mark
  again just before the post is sent: any master saved in that span, while the person decides included (a ledger
  created, a voucher type changed), refuses the post as `post_masters_moved` and withdraws the approval; post again.
  Each of thirteen saves of a voucher type's screen in Tally moved the mark by 2 (nine of a type holding no vouchers,
  and two in each of two runs on a type holding vouchers, all on one synthetic book, TallyPrime 7.1 Silver). A voucher type
  saved during the read itself refuses as `masters_bound_premise_violated` (cause
  `masters_alter_id_above_master_mark`). A book in Education mode is now refused, as
  `masters_education_unqualified`, for a post and for a build. Each pass of a post
  sends 20 more requests to Tally before the approval (12 of them POSTs), so a held approval redeemed in a later call
  sends them again; a build sends 14 more (8 POSTs), under its admission lock.

- **`verify_import` now compares a voucher's narration.** A posted voucher whose narration in Tally
  differs from the one approved reads `posted_divergent`, with `narration` in its `diffs`; before,
  every other compared field matched and it read `posted_verified`, and a divergent voucher never
  listed a narration difference. A narration edited in Tally after the post therefore leaves the
  batch unsettled until the narration in Tally matches the approved text again. The read-back, the post's span binding and the amendment
  check now share one comparison: the text as written (trimmed, with its `[BRIDGE:…]` tag for a
  file imported by hand) against the text read, byte for byte; a narration Tally holds none of
  equals only an empty one written. Whether Tally changes trailing spaces or line breaks on import
  is unmeasured; the first live run will show it (#1608).

- On Windows, `parse_bank_statement` now refuses a password file owned by neither you nor
  Administrators, or that anyone but you, SYSTEM and Administrators can read or change
  (`statement_password_file_permissions`), as a Mac already refuses one that other users can read.
  The refusal says how to make the file private: make a new copy in a folder of your own and
  delete the original, because moving a file keeps who can read it. A password file on a drive
  that keeps no access list, such as a FAT or exFAT stick, is expected to be refused as well;
  that is not yet measured.
- The hand-import script `scripts/bank_statement_import.py` (not part of the downloaded package, #1430) now writes the same
  `| Statement party: <name>` segment under the same rules as the app, and
  refuses a run whose narration holds a control character or the reserved marker, or runs past 2,000 characters, as
  `narration_not_admissible`, as the app does (`party_not_admissible` when the party's own segment is what breaks
  the rule). Importing its file again for a statement already imported is expected to rewrite those vouchers'
  narrations in the book: their keys do not depend on the narration the script writes. A same-key re-import has been measured over
  the gateway; through Tally's own Import menu, which is how this script's file is imported, only an unchanged re-import of a file the app built has been seen, on one Windows computer (#1413).
- A character that draws nothing (Unicode Default_Ignorable_Code_Point or Format, except ZWJ,
  ZWNJ and the prepended concatenation marks) is not read from a bank-statement PDF, and a soft
  hyphen is read as the hyphen it prints. A narration holding
  such a character is refused as `voucher_text_invalid` when a batch is built, and before a native
  post of a batch saved earlier; the reference is not checked. A statement that held one gets new
  `bridge_txn_id` labels, so a batch built from it by an earlier build does not line up with a new
  run's `amends_batch_id`.
- `verify_import` no longer reads its window a second time when the first read was divided, every part was
  checked against a count of the window that names each voucher, and the company's marks were unchanged when
  read again after the last part (#1241). That is a window of more than about 170 vouchers in a book whose
  voucher mark is above about 170; it holds every voucher of the book in those dates, not only the batch's. The proof's `voucher_read_corroboration` then
  reads `{"state": "not_sent", "reason": "counted_and_bracketed_read"}` instead of a second read. A window read
  in one request, or divided without a count, is read twice as before. `ledger_movement` still reads its
  window twice. The request counts come from test doubles, not from a live Tally.

- **Verifying a batch no longer replaces its saved proof, so the host need not ask before each
  read-back.** Each `verify_import`, and each of the two verifications `acknowledge_post_review`
  runs, now saves its proof as a new pair of files, `<batch>.proof.<time saved>.<SHA-256>.json`
  and `.md`, beside every earlier pair, and the status it adds to the import journal names which
  pair is current. The finished masters check and the first verified ALTERIDs go to files of their
  own, written once. Nothing these two tools find in the folder is replaced or deleted, so they are
  now marked as writes that are not destructive. The folder therefore grows with every
  verification; nothing deletes old proofs. A batch last verified by an earlier build keeps its
  single `<batch>.proof.json`, which no longer changes. After the first verification with this
  build, a connector older than it refuses the import journal (a downgrade was already refused after
  the first native post), so keep this version for recovery (#911).

- **On Windows, the approval window now opens above a full-screen Tally (seen once).** It could open behind TallyPrime when Tally filled the screen, so the person saw
  nothing and the approval lapsed unanswered. Both windows, the post approval and the review
  acknowledgement, are now drawn above every window that is not itself kept on top. Nothing
  else changes: the text, the buttons, No as the default and the two-minute limit. Seen once, on
  one screen, in front of a full-screen Tally and in front of Claude Desktop; the same run found that the window
  takes keyboard focus (see "Known limits") (#1589).

- **A one-voucher approval now shows a long narration in parts, where it refused it before.**
  A bank-statement voucher's narration is often too long for one line of the dialog, so posting
  it alone was refused and the file had to be imported by hand. Such a narration is now shown
  under the heading in numbered parts, each quoted on its own line and split only at a space,
  so that the parts joined in order are the text that is posted. A line above the heading
  counts the parts, and the date line says the voucher's own text is below the dashes. It is
  still refused whole, never cut, when no space splits it into parts that fit, when it holds a
  run of dashes or a number in brackets, or when the whole text passes the dialog's caps. A
  reference is never split: one too long for its line is refused as before. Each part past the
  first, and the counting line, leave one line fewer for entries. The review dialog of
  `acknowledge_post_review` now shows the narration last, under the same heading, on one line
  or in parts, which leaves it room for nine entries instead of ten. An approval text that fits
  as before is unchanged. Not yet measured: how the Windows and macOS dialogs draw the parts
  (#1535).

- **A `post_import` refusal made before anything was sent now says what to do next.** Its
  `error.next_step` carries the same step every other tool gives for that refusal: its code's,
  or else its cause's. Before, a code raised inside the post, such as a date range too large to
  check in one request, came back with no step. Only when the journal shows nothing was sent: a
  refusal after a send, or when a send could not be ruled out, still carries no such step, and is
  reconciled with `verify_import`. The steps the post already gave (a busy port, a row another
  batch posted) are unchanged (#1573).

- **A batch approval now shows its totals first.** The window for a batch shows, in this order:
  the count and company, the company number, the count by type, the dates, the total debit and
  credit and the money lines; then the company GUID and the Tally address, the voucher lines or
  the sentence in their place, the per-ledger lines, the batch id and the cautions. Every line
  keeps its wording. The window for one voucher is unchanged. This order has not yet been seen
  in a running window on Windows or macOS (#746).

- **A batch approval's Journal lines now name a ledger on each side, and the window no longer
  says narrations are shown exactly.** In a batch of up to ten, each Journal's line named only
  its first debit ledger, so the person could not see what each Journal credited; it now shows
  `Dr "<first debit>" +N more  Cr "<first credit>" +M more`, with up to 20 characters of its
  narration (other types keep 40), cut shorter, never under 10, when that makes the line fit. A
  Journal line that fits at no such cut leaves the totals and the reason line, as any line that
  does not fit does. The line above the voucher lines said each
  narration was quoted "exactly as it will be posted", above narrations cut at 40 characters;
  it now says a cut narration is marked. Payment, Receipt and Contra lines still name one
  ledger. On every line, the further entries on a side now read "+N more", never a bare "+N"
  that could be read as an amount.

- The approval dialog of `post_import` now takes a voucher on a ledger whose
  name in Tally ends in one line break; until now such a voucher could be
  built and was refused for posting. The approval dialog, and the review
  dialog if a doubt is recorded after the post, show such a name quoted
  with the line break written out and the words "Line break" before the
  quote, on every line that shows the name (`Line break  "Name\r\n"`), and
  add one line saying what the mark means and that no other name in that
  text has one. A line break anywhere else in a name is refused as before,
  and no dialog shows a line break in any other value. The desktop Journal
  screen takes no saved Journal that names such a ledger, to review or
  post, or to reconcile once it was sent: it says so and says to use the
  assistant for it. An approval text that carries both this line and the
  On Account sentence holds one entry fewer. The mark takes 12 of a line's
  100 characters and the written-out break 4: a text with a line over the
  cap is refused, never cut, except that a batch's per-voucher lines give
  way to a reason line.
  Measured once on a licensed TallyPrime 7.1 Silver on Windows: one Journal
  on such a ledger posted through the dialog, Tally read the name back with
  the break as `&#13;&#10;`, and the post ended verified; the read-back is
  now the test's fixture. Not measured: how macOS draws and wraps the mark
  and the quoted name, and the review dialog against a live Tally (#626).

- The review dialog of one voucher (`acknowledge_post_review`) now shows the
  voucher's type quoted (`Record that you reviewed ONE "Journal" in "Books"`),
  as it shows every other value read from Tally, so every Tally value in
  that text is escaped (#626).

- **An empty window whose neighbouring days hold vouchers now says so in its reason.** When the read of a
  window returns nothing and the read widened by a day on each side finds rows, none inside the window,
  `vouchers`, `voucher_presence`, `purchase_register`, `sales_register` and `ledger_movement` now give
  `empty_window_wider_read_nonempty` where they gave `nonempty_uncorroborated`, which read as if the window
  itself held rows. Only the name changes: the state stays `partial` (a refusal, for `ledger_movement`),
  and an empty window next to a busy day still cannot reach `complete`. The other reasons
  (`empty_uncorroborated`, `company_has_no_vouchers`, `window_contradicted`) are unchanged, and the `vouchers` and
  `voucher_presence` tool descriptions and the agent guide now list all three outcomes of an empty window.

- **The install steps are rewritten for a first-time accountant.** The installation guide gives one short action
  per step, the exact words on each button, what the screen shows next and what to do when yours differs; the
  download page and the home page give the same steps in short form, and the download page points to the guide.
  They name the three things a new user meets in Claude Desktop (the notice that the extension is not verified by
  Anthropic, the confirm box, and the permission prompt before a tool runs), give TallyPrime's path (F1,
  Settings, Connectivity, Client/Server configuration), and say to leave posting off. The guide now lists what
  the data folder holds, says the list is not complete, and includes the last four digits of a statement's
  account number kept with its proposals, the ledger names chosen for a statement, the sign-in name a recorded
  review can include, and the record of accepting the Terms. TallyPrime's Step 1 screens, the browser check and
  the Alt+K and Alt+Y menus were checked by looking on Windows (TallyPrime 7.1, 10 October 2026); the guide says
  which steps were not tried (#1017).

- The texts that say what Response redaction leaves unhidden now also say that the last four digits of a bank
  statement's account number (`account_last4` in the answer of `parse_bank_statement`) are always sent: the
  extension's description and its redaction setting, the two tool descriptions that describe redaction, the
  README, both guides, the security page, the FAQ and one article. Only the wording changed; what is sent is the
  same (#1616).

- `parse_bank_statement`, `build_import_xml` and the agent guide now tell the assistant to import a bank
  statement in whole-day batches: cut a batch only between days, at most 50 vouchers. `post_import` refuses a
  whole batch with `import_preexisting_identity` when a voucher has a twin with the same date, type, ledgers and
  amounts that an earlier batch posted, while twins inside one batch pass together; a statement cut across two
  batches on one day hit that refusal. A day with more than 50 rows does not fit one batch, and the text says to
  ask you. Batch posting is off by default, so same-day twins post together only with it on or through a hand
  import of the built file. Wording only: no new refusal and no change to the check (#1614).

- `verify_import`'s description now says in plain words that after a restore from a backup, a native voucher
  keyed again with the same content before any check can read verified, that no identifier a read returns tells
  the two apart, and that after restoring a backup over a company you should check its posted vouchers in Tally.
  Tally gave the vouchers keyed again the post's MasterIDs, GUIDs and voucher numbers (measured once, on one
  synthetic company on TallyPrime 7.1 Silver; the run that compared every identifier had Tally's security off);
  the description had called that reuse unmeasured. Behaviour is unchanged (#1612, #1636).

- The installation guide now says that a batch posted from the command line is approved on a summary of
  per-ledger totals and, when there are ten or fewer vouchers and the lines fit, a line for each voucher; it said
  the summary showed no voucher's date or narration. The window has listed each voucher of such a small batch
  since 0.5.0 (#1632).

- **A native post whose binding was refused now says what the book holds.** When `verify_import` cannot bind a
  batch to the vouchers its own post created, `post_span_binding.summary` says the batch stays open (except when
  every voucher reads `tally_reported_not_created`, where the summary differs) and, when every voucher reads
  `matching_content_observed` against a voucher that is neither cancelled nor optional (or only some do), that
  the book holds, for each voucher sent (or for some), one with the same date, voucher type and ledger entries,
  and that ComplyEaze Bridge cannot tell whether that voucher is this post's; it never says the post's vouchers
  are in the book. `local_data_report` counts such batches as `not_settled_binding_refused`, and
  `not_settled_not_verified` no longer includes them. The `post_import` description now says no review can be
  recorded for such a batch: `acknowledge_post_review` refuses it, because its vouchers are matched by content
  only, so the person checks them in Tally and the batch is not rebuilt (#1469).

- **A refusal because Tally did not answer now carries a next step.** `endpoint_unreachable` tells the assistant
  to ask you to confirm that Tally is running with the XML server enabled on the port named in the error, and not
  to repeat the request until you have; `endpoint_circuit_cooldown` (a request held back after repeated failed
  requests to Tally, nothing sent) says it can be repeated after about ten seconds. A code's own step still comes
  first, and the other unanswered reasons still name none. The `parse_bank_statement` description and the
  security and privacy notes in the repository also now say that ` | Statement party: <name>` sits at the end of
  the narration text, and that a file imported by hand adds the `[BRIDGE:...]` tag after it (#1483).

- The Terms of Use setting is now told as "tick" wherever a person is asked to set it: the refusal a tool gives
  before the Terms are accepted, the installation guide, the connection guide, the download and home pages and
  the FAQ said "turn on" or "switch on". "Turn on" stays for the HTTP gateway and the posting setting (#1484).

- The `masters` tool no longer calls a voucher type's numbering value its numbering method. Its description and
  its result's limitation now say the value is the one Tally reports at the type level, that it read `None` for
  types set to Automatic and to Manual on Tally's screen (one synthetic book, Windows), that it can differ from
  the number series that decides what a supplied number does, and that it is not to be passed to
  `voucher_presence`'s `numbering`. That input now says the method is the one you confirm for the series used.
  What is read is unchanged (#1487).

- The Outstandings screen's ledger-master export now ends its balances at the screen's "As of" date, so a
  workbook can be exported as of 31 March; it ignored the field and used the date of the computer. With no date
  chosen nothing changes, and the assistant's `ledger_masters` still uses the computer's date. A date before the
  company's books begin is refused as `as_of_precedes_books_from` before any master, balance or group is
  requested. Only the 1st, 2nd and 31st of a month are accepted as a balance-period end, so a typed 30 June ends
  on 1 July and that day's vouchers are inside the balances; the notice says so (#1531).

- The security and privacy notes in the repository now say the 0.5.1 Terms refusal (`terms_not_accepted`, naming
  the setting) was the answer the tools the assistant called gave on one Mac with a test package and on one
  Windows computer with the build our checks made for 0.5.1; they had said it was not yet checked on Windows
  (#1543).

- The README's table of tools now has a row for each tool offered with posting off, adding `company_features`,
  `gst_status` and `voucher_read_cost` (#1672).

- The FAQ answer and the security page about a second computer reaching Tally's gateway now state the rule: do
  not assume the gateway is closed to your office network; the firewall of the computer that runs Tally is what
  keeps other computers out, so confirm it blocks incoming connections to Tally's port. They keep the scope of
  the one test on TallyPrime 7.1 (a second computer got the status answer; nothing else was checked) and no
  longer say it was "not recorded in the repository" (#1685).

- The README and two blog pages no longer say that the record of a test run is our dated notes, "kept privately";
  what was and was not run is stated as before (#1528).

- **The core for posting one GST Sales invoice is in this build and switched off.** Sales is not in the qualified
  list, so every Sales build is refused before anything is sent to Tally, no file is written, the tools' schema does
  not offer the type, and a saved Sales batch is refused at post. A Sales invoice that was sent and is not verified
  posted would stop every further Sales invoice of its company until it verifies or a person releases it; that cannot
  happen while Sales is off. For Journal, Payment, Receipt and Contra one change is accepted: `"invoice": null` is
  read as absent. A first rehearsal on 7 October 2026 posted two invoices and found what must change before Sales can
  be switched on. In 0.6.0 Sales stays switched off, and a Sales invoice would be built only when voucher posting is on, so
  there would be no hand import of one (part of #691, #1342, #1463, #1488, #1491).

**Safer or fixed**

- On Windows, every request to one Tally port is now also serialised through a lock file under your user profile
  (`.complyeaze-bridge\coordination`), in addition to the one under AppData. Windows writes a Store-installed Claude
  Desktop extension's AppData files into the package's private folder, so the extension and an unpackaged
  ComplyEaze Bridge process could each hold "the" lock in a different folder and a post could overlap a read on the
  same Tally. A process that cannot create that folder refuses every send as `import_admission_lock_unavailable`.
  Not measured: the cost of the extra lock per send (#1584, #1641).
- `build_import_xml` shows where the import file really is when a Microsoft Store install of Claude Desktop keeps
  Bridge's files in the package's private folder, so File Explorer and Tally can open the path it prints; a new
  `path_state` field says whether the path was resolved. Where files are stored and read is unchanged. Measured
  once, partly, on one Windows PC with Claude Desktop from the Microsoft Store, on 15 October 2026 (#1459, #1569).
- When the local Tally mirror's key is missing from the computer's credential store, or is stored but is
  not a valid key, the app now says so with its own codes (`tally_mirror_key_missing`,
  `tally_mirror_key_invalid`) and tells you that a retry will not help, instead of asking you to approve a
  credential prompt and retry. Restoring the credential (the macOS login keychain, or that user's Windows
  Credential Manager) or opening the app as the same computer user is what recovers the mirror. The
  app deletes nothing, and no reset exists yet (#1563). A key is also no longer created when the mirror file's
  existence cannot be checked. Not measured on a real
  credential store: what the macOS keychain or the Windows Credential Manager returns after a keychain reset
  or a profile move.
- A later check (`verify_import`, a repeated `post_import`, or the check
  before a review is recorded) of a post that Tally answered as creating none
  of its vouchers no longer suggests the voucher was edited in Tally. Its
  status is unchanged (`sent_not_attributed`, the batch still
  `reconciliation_required`), but the voucher's next step now says that Tally
  reported it as not created when it was sent, that it is not found for these
  dates now, and to look for it in Tally before entering it by hand, since it
  may have been entered in another form meanwhile. This reads what the post
  left on record: its saved answer (every counter reported, none created, one
  exception for each voucher sent, nothing else counted) and, for a batch,
  its own verdict that the voucher step matched. A partly created batch, and
  a batch whose voucher mark moved across the post, keep the earlier line.
  Measured live once, on a single voucher refused whole; a batch is not
  measured live (#1108).
- A voucher of an unbound native post that ComplyEaze Bridge cannot find in the book no longer has its next
  step suggest that it was edited in Tally when Tally's own answer to that post was not clean (for example a
  batch Tally created only partly, or an answer that did not parse): the line now says only that it cannot be
  matched, and to check in Tally before posting it again (#1108).
- `build_import_xml` refuses a voucher whose own `voucher_number` begins or ends with white space, or is only
  white space, as `voucher_number_invalid`, before any file is written: on a raw import Tally read a number
  sent with a space at its end back without it, so such a voucher could never verify as posted (#1730). A
  space inside a number is still admitted.

- `parse_bank_statement` no longer reads a statement PDF's text in render mode 3 (invisible), which is how a
  scanner app lays a hidden OCR layer under a page image, so that text is not parsed as if the bank had printed
  it. A scan whose only text is a hidden layer, or whose drawn text yields no statement row while most of the
  document's characters were hidden, is refused as `statement_no_readable_text` (before: its text was parsed, or
  the refusal was `statement_no_account_number_line` or `statement_page_sequence_unproven`). A character PDFium
  cannot classify refuses the file as `statement_unreadable_pdf`. No OCR is attempted. Neither PDFium failure was
  seen on the real files checked (#1637).

- A Tally read that timed out, or whose response body failed after Tally had begun answering, is now sent once
  instead of up to three times: a client timeout does not stop the work Tally is doing, so a further send could
  queue behind the first. This covers the company-list reads, the ledger listing's extent read and the desktop
  snapshot's small reads. A read whose connection or send failed, or that Tally answered with a server error or a
  rate limit, is still retried (#1538, #1556).

- The desktop core snapshot sends each of its group, ledger, voucher-type and voucher-window exports once: when
  the connection closes after the request was sent, it returns the error instead of sending the export up to
  three times. Its small reads (the pair of book-extent reads around each export, and the company list) keep the
  three-attempt retry, so one lost extent read does not make you start the run again (#1674).

- The desktop core snapshot no longer sends the ledger period-balance report it used after each window to
  corroborate balances: one TallyPrime release could not evaluate it (others were not tried), and it was never
  seen returning a usable answer. A core run therefore cannot reach Verified; every window it reads records
  `report_tie_out_unavailable`, as one already did when the report was refused (Education mode). A resumed run no
  longer reopens a completed window, though a run resumed from an earlier build's saved state keeps a tie-out
  already stored. No published package sent this read, only the unreleased desktop app's Proof of Sync (#1643).

- A desktop core snapshot that stops because the book changed while its groups, ledgers or vouchers were read is
  now recorded as `source_changed_during_run` ("Run again during a controlled quiet period"); one that stops
  because the two reads of a book extent differ, or because Tally gave no master-change marker, is recorded as
  `company_book_extent_drifted` or `company_altmstid_missing`, each with Gap Map guidance, where all showed
  "Response parse failed". A well-formed extent that names no row for the selected company is now
  `company_identity_mismatch` where it showed "Application response rejected". The two native-export drift
  conditions still use the catch-all (#1554).

- The import journal's first record is now made durable in its folder: when the journal is new or empty, the data
  folder is synced after the write, and if that sync fails the record is taken back and the append refuses with
  `import_ledger_unavailable` (`import_ledger_rollback_failed` if the take-back also fails). A failed folder sync
  on the acknowledgement, the proofs, the verified-baseline additions and the doubt record is now reported instead
  of ignored: the acknowledgement refuses with `import_file_write_failed`, and publishing proofs refuses before
  the status record is appended. The masters-check verdict still ignores a failed folder sync, and the built
  import file is not folder-synced, as before. Before, a power loss soon after the first record could leave no
  journal and so no local record of a post. On Windows the folder sync does nothing, as before (#1548).

- Desktop setup now refuses a selected company GUID that is not in the strict form of a Tally GUID, as
  `company_selection_invalid` (retry not recommended), and the snapshot identity check refuses a stored one as
  `company_identity_invalid`; before, the setup accepted any short trimmed ASCII text. Every company GUID in the
  committed captures (some of them synthetic) is in the strict form, so none is expected to be refused; a
  TallyPrime release that writes another spelling would be refused until it is observed (#1627).

- The `changed_since` tool, which no build offered, and the code only it reached are deleted (#1769).

- Locked dependencies moved, with no source change: `rustls` 0.23.45 to 0.23.46, which clears four advisories
  published on 14 October 2026 (RUSTSEC-2026-0346 to -0349: secrets in `Debug` output of client session values,
  a Brotli certificate decompression that can allocate up to 1 GiB before authentication, and a ServerHello
  downgrade of later connections to classical key exchange), and `tokio`, `uuid`, `zeroize`, `libc` and `objc2`
  by a patch or minor release (tokio 1.53.2 fixes a Windows process-handle wait and a blocking-pool shutdown bug,
  per its release notes). The Rust licence inventory is regenerated (#1718, #1580).

**Known limits**

- **Live pass of what 0.6.0 writes into Tally (16 October 2026).** The release candidate was installed as the
  extension in Claude Desktop (Microsoft Store build 2.37337.1.0) on Windows 10 Home 22H2 (build 19045), and run against
  TallyPrime 7.1 Silver on port 9001, on one new synthetic company, BRIDGE RC GATE LAB. Build: commit
  `a3b7316947cdf3c5ce46615c8b89e45d538345f3` (pull request #1775, CI run 38877254488), `bridge-tally.mcpb` SHA-256
  `6b83ca74377d7ed86a2b524f4d53bd2e9753b3318bfa6634078d5f5645184022`, containing `bridge_mcp.exe` SHA-256
  `e9780806594e60afabdb83d35bb751f0aeefba708b2b63c33b11452bbf0e5a84`; the installed copy was checked against that hash
  before the run, and the server answered every call (Bridge's own log). With the one posting setting on, each of the
  four voucher types was posted once with no voucher number after its approval window, and one saved batch of three (a
  Payment, a Receipt and a Contra) was posted after one approval. In every post Tally's answer reported every counter,
  created 1 (3 for the batch) and nothing else counted; each voucher then read `posted_verified` in the post's own
  readback and again in `verify_import`, with no duplicates, and the company's voucher mark moved by exactly the
  number created. Tally gave the numbers itself: Journal 1, Receipt 2 (the book's seed receipt held 1), Contra 1,
  Payment 1, then in the batch Payment 2, Receipt 3, Contra 2. On Windows the approval window showed in front of a
  full-screen Tally and in front of Claude Desktop, with No as the default (one screen, one run). In that run a key meant for Tally (the Down arrow) moved the selection
  inside the approval window (No, Cancel, Yes) and Tally did not react: the window takes keyboard focus. No is still the
  default, so Enter alone refuses; arrow keys followed by Enter can reach Yes. A typed letter was not tried in that run (later that day, on the published file, a typed n
  declined the window and a typed y approved it: #1780) (#1148,
  #1589, #1630).
- **Not run in that pass:** a batch of Journals; a batch larger than 3 or near 50; a post Tally refused or that landed
  partly; amendments; the review acknowledgement window (no voucher raised a doubt); a post that needs an On Account
  approval (a bill-wise party; the build refused one as designed); the retired batch variable (`setting_retired`); an
  import file taken to Tally's Import menu; Sales invoice posting (not offered in 0.6.0); the desktop app; Gold; macOS;
  any other Windows or Tally release. Also not run: the voucher-numbering rows (#724, #1668: a back-dated numberless
  Journal, a second numberless Payment, the refusal of a numberless voucher into a Manual-numbered type);
  `voucher_read_cost` against a real `vouchers` call (#1465); the Store-install proof of the lock (#1584, #1459: lock
  contention, `local_data_report` from the chat, which folder the desktop app uses); a second monitor. The candidate was
  installed while 0.5.1 was present, and its settings were not compared with 0.5.1's; what Claude Desktop passes for a
  new setting on an upgrade is not measured (#1090, #1630).
- **The file tested above is not the file that will be tagged.** The pass ran on the CI package of the release pull
  request. The release workflow builds `bridge_mcp` again on its own, with a plain release build of that binary, where
  the CI package takes it from the Tauri build; the two files are the same source but not byte-identical, and no build
  is claimed to be reproducible. Between that commit and the commit the release is built from, no file that goes into
  the package or the desktop bundles changed (the check is recorded on the pull request that adds this paragraph).
- **Release package check (16 October 2026, 19:13 local time).** A package built by the release workflow's own package
  steps (rehearsal run 38885882661, pull request #1777, source commit `d6561887130d4fd4265121d6ef40db650b538280`, which
  is `a3b73169` plus one workflow file) was installed over the release candidate in Claude Desktop (Microsoft Store
  2.37337.1.0) on the same Windows 10 computer, TallyPrime 7.1 Silver and company. `bridge-tally-mcp-v0.6.0-windows-x64.mcpb`
  SHA-256 `3e76e8f6edf4d00fa8a1420b696661be9ddbe0b6ab3c84465f9cff58cc3fda05` (13,821,469 bytes), holding `bridge_mcp.exe`
  SHA-256 `10e4614f74eab975897834ec1c9750edbe9fc7ad91584b802e5cde57a8180899`, which is not the release candidate's file;
  its manifest is byte-identical to the candidate's. The settings were kept by that replace (port 9001, posting on); one
  run, same version, and it says nothing about an upgrade from 0.5.1. One Journal (12 August 2026, debit BRIDGE RC Expense,
  credit BRIDGE RC Suspense 5.00, no voucher number) was posted after its approval window: Tally answered created 1,
  every other counter 0 (538 bytes, SHA-256 `42e4de026f72a1e7d882ad6f19067a8f9a6cf549dc1c40f90c752aefdfebfec5`), the
  company's voucher mark moved from 8 to 9, the post read `posted_verified` in its own readback and again in
  `verify_import`, and Tally gave it number 2. The four voucher types and the batch ran on the release candidate, not on
  this file. The published file is built by the release workflow's own run, after these notes are written, so its
  `bridge_mcp.exe` is not compared with the one above here.
- The `headline` of a batch post has not been seen on a live Tally post (#1090, #1532).
- Nothing was run on a Mac against a live TallyPrime for this release; the Mac package is built and started by our
  checks without Tally.
- What has been run against a real TallyPrime is listed in the README.

## [0.5.1] - 2026-10-09

### In plain words: ComplyEaze Bridge 0.5.1, since 0.5.0 (9 Oct 2026)

These changes are in ComplyEaze Bridge 0.5.1. Each line names the pull
requests it comes from, except where it names an issue.

**Should I upgrade?**

- **If you updated from 0.4.2 to 0.5.0 and Claude Desktop did not start
  ComplyEaze Bridge: yes.** 0.5.0 marked the Terms of Use setting as required.
  On one Mac, after an update from 0.4.2, 0.5.0 was not started and Claude
  Desktop showed "Unable to connect to extension server". 0.5.1 no longer marks
  the setting required. After installing 0.5.1, turn on the Terms setting in
  the extension's settings, then quit and reopen Claude Desktop. Until the
  setting is on, every tool answers `terms_not_accepted` and names the
  setting; nothing is read from Tally. What we saw: a package built from the
  Terms-setting change (#1436) on one Mac (only its version set to 0.5.1),
  installed over 0.4.2 and over 0.5.0, was started by Claude Desktop (in one run
  only after Claude Desktop was quit and reopened), and with the setting off the
  tools the assistant called gave that answer. On one Windows 10 computer with Claude Desktop 2.26454.2
  from the Microsoft Store, the published 0.5.0 file over 0.4.2 was not started
  (no ComplyEaze Bridge tool in the chat and no error), and the build our
  checks made for this release (commit d744d25a, not the published file) was
  started after a fresh install, after an
  update from 0.4.2 and after an update from 0.5.0 that was switched on (as the
  update leaves it, and with the Terms ticked); with the Terms off the tools the
  assistant called gave that answer, and with them on the Tally check ran. Not tried: the published 0.5.1 file, and a release
  older than 0.4.2 (#1413).
- **If 0.5.0 already starts for you:** the other changes below are the reason
  to update, if any.
- **If you use the bank-statement tool and build vouchers for mapped ledgers:**
  such a voucher now carries the statement's party at the end of its narration
  text (see "What you can do now" below). Vouchers already imported keep their
  narration, but amending a batch built before this change rewrites the
  narrations of that batch's vouchers in the book (#1430).
- **After updating, if the assistant does not find ComplyEaze Bridge, check that it is switched on in Settings, Extensions.** On one Windows computer, an update kept the extension's switch as it was: an extension switched off before the update was still off after it, and the assistant showed no ComplyEaze Bridge tools and no error. Switching it on brought the tools back there, and they then answered `terms_not_accepted` until the Terms setting was on (#1413).
- **If you use ComplyEaze Bridge on Windows:** the build of this release made by our checks was tried on one Windows 10 computer, with TallyPrime 7.1 (Silver) and a company made for the test; see "Known limits" below for what was and was not tried.
- **How:** ComplyEaze Bridge does not update itself. Follow the
  [installation guide](https://github.com/ComplyEaze/bridge/blob/master/docs/agent/INSTALL.md):
  (1) close any other program that runs ComplyEaze Bridge; (2) in Claude
  Desktop, Settings, Extensions, install the new file (on one Mac a new
  install opened no settings window: open Settings, Extensions, ComplyEaze
  Bridge yourself); (3) keep ComplyEaze Bridge's data folder; (4) check that
  the extension shows 0.5.1, turn on the Terms setting if it is not on, and
  check that "Allow voucher posting" is as you want it; (5) quit Claude
  Desktop completely and reopen it.

**What you can do now**

- No new tool.
- A voucher built from a bank statement for a mapped ledger now carries `| Statement party: <name>`, the
  counterparty the statement printed, at the end of its narration text (#1430). A file you import by hand adds
  ComplyEaze Bridge's `[BRIDGE:...]` tag after it. A line sent to suspense and a
  cash line are written as before. A party that contains `|` or a control character, that carries the reserved marker, or that makes the narration longer than 2,000
  characters now refuses the statement run (`party_not_admissible`); `skip` that party or map it to the suspense
  ledger. A label the parser gives a line by its wording (bank charges, a card fee, an instalment) is not written as a party. Vouchers already imported keep their narration,
  but amending a batch built before this change rewrites the narrations of that batch's vouchers in the book.
  The name is part of the narration, so `mask_parties` does not shorten it and `drop_narration` removes it.

**Safer or fixed**

- **An update now starts ComplyEaze Bridge before the Terms are turned on, and
  its tools say why they refuse.** The Terms of Use setting is no longer
  marked required and still starts off. Claude Desktop now starts ComplyEaze
  Bridge, and until you turn the setting on every tool answers
  `terms_not_accepted`, naming the setting. That the app starts and the tools the
  assistant called answered was seen on one Mac with a package built from the Terms-setting change
  (#1436), and on one Windows computer with the build of this release (#1413).
  The setting no longer shows "(required)" beside its title, and its description
  now begins "Tick to use ComplyEaze Bridge: it refuses every tool call until you
  accept the Terms of Use." A test checks that wording, and it was seen in
  Claude Desktop on that Windows computer.
- The schema of every paged read no longer names a default `limit` of 500. When `limit` is left out
  the server uses its configured row limit (500 unless it is configured otherwise), as the tools' own
  text already said; the schema said 500 even when the row limit was lower or higher (#1148).
- A State Bank of India statement line whose UPI, NEFT, RTGS or transfer text prints an empty name
  field now names its party as `UNNAMED`, as the IMPS reader already did. Before, the party was empty and
  the suspense line's narration named no one. `UNNAMED` cannot be mapped to a ledger, so the line still goes to the suspense ledger (#1429).
- The import journal (`agent-import-ledger.jsonl`) is now read with each of its dates as a checked
  date. A record holding a date that is not `YYYYMMDD` refuses the whole journal
  (`import_ledger_invalid`): building, posting, verifying and reconciling every batch, and the
  desktop journal screen, stop until the record is put right, and the local data report shows the
  journal as not read. The current code writes only that form, so only a record edited by hand is
  expected to hold such a date, and the stored bytes do not change (#1307).
- The PDF library is loaded from the folder that holds the program (#1422).

**Known limits**

- Windows was tried on one computer (Windows 10, Claude Desktop 2.26454.2 from
  the Microsoft Store, TallyPrime 7.1 Silver, a company made for the test), with
  the build our checks made for this release at commit d744d25a. The published
  0.5.1 file was not tried, and nothing was tried on other Windows versions or
  other Tally editions (#1413).
- On that computer, a bank-statement import file built for six vouchers was
  imported by hand into Tally and read back: the verification found all six, and
  a voucher posted to a mapped ledger showed its narration with
  `| Statement party: <name>` before the `[BRIDGE:...]` tag. An unchanged
  re-import of the same file was matched to the six vouchers already there and
  added none. A changed re-import was not tried (#1430).
- On one Windows computer where Claude Desktop was installed from the Microsoft Store, the folder shown for a built import file did not exist: the file was under the user's AppData\Local\Packages\Claude_<id>\LocalCache\Local\Bridge\agent\imports folder. If Tally's Import screen does not find the file at the path shown, look there. Seen with 0.5.0 as well (#1459).
- The library that reads Tally's replies was updated (#1198). Its output for
  the saved replies in our test record is unchanged. The Windows runs above used
  it for a company list, the read-back of six vouchers and an August trial
  balance on a small company; it was not run against a large book.

## [0.5.0] - 2026-10-08

### In plain words: ComplyEaze Bridge 0.5.0, since 0.4.2 (3 Oct 2026)

These changes are in ComplyEaze Bridge 0.5.0. Each line names the pull
requests it comes from, except where it names an issue. A "mark" below is a
counter Tally keeps that moves when vouchers or ledgers change.

**Should I upgrade?**

- **After updating, accept the Terms again, and check the posting setting.**
  0.5.0 names the setting that accepts the Terms for version 2026-10.1, a
  different setting from the one you accepted for 0.4.2. After updating, tick
  the new Terms setting in the extension's settings, check that "Allow voucher
  posting" is as you want it, then quit and reopen Claude Desktop. What we saw:
  we installed the package our checks built for this release (not the published
  file) over 0.4.2 on one Mac. It replaced 0.4.2 and kept the Tally port,
  Response redaction and Allow voucher posting as they were (a posting
  setting that was on stayed on); the old Terms setting was gone and the new one
  had no value. Before the tick, and before any quit, Claude Desktop did not
  start ComplyEaze Bridge, and a request to one of its tools failed with a 400
  error that did not mention the Terms. After the tick was saved Claude Desktop
  started it at once and a new chat got an answer from it, but the assistant
  first called a leftover second entry for the same extension, which failed with
  the same error. After Claude Desktop was quit and reopened there was one connector and
  no error. Not tried: quitting and reopening without the tick, installing over a
  release older than 0.4.2 (remove such a release first), and Windows. If
  Settings, Extensions lists two ComplyEaze Bridge entries (as happened when
  0.4.2 was installed beside an older release), remove the older entry before
  you use either. Then enter your settings in the new one, tick the Terms
  setting, and quit and reopen Claude Desktop (#1010).
- **If you turn posting on: do not post a batch built by 0.5.0 with an older
  version.** Versions 0.3.0 to 0.4.2 refuse to post a batch built with 0.5.0,
  and nothing is posted. Their message says to build the batch again; do not do
  that there: reinstall 0.5.0 (or a newer version) and post the batch from it.
  An older version makes none of 0.5.0's checks on bill-wise and cash-in-hand
  ledgers for what it builds and posts itself. As before, from the first post
  attempted with 0.4.2 or later, versions before 0.4.2 stop reading the import
  history. 0.2.0 and earlier have no such refusal: do not run them over a data
  folder 0.5.0 has built in (#1234, #1343).
- **A batch built by an earlier version must be built again once** before it
  can be posted (the bill-wise and cash-in-hand records); check first whether
  its file was already imported by hand (#815, #1234).
- **If you only read your books:** nothing here is urgent. The reasons to
  update are the search and summaries in `vouchers`, the small-company count,
  the new `cash_flow` tool and the safer refusals below.
- **If you use ComplyEaze Bridge on Windows, treat it as untried:** no one on
  our side has installed the Windows package of this build in Claude
  Desktop.
- **How:** ComplyEaze Bridge does not update itself. Follow the
  [installation guide](https://github.com/ComplyEaze/bridge/blob/master/docs/agent/INSTALL.md):
  (1) close any other program that runs ComplyEaze Bridge; (2) in Claude
  Desktop, Settings, Extensions, install the new file (remove a release older than
  0.4.2 first, and see the first bullet above if two entries are listed); (3) keep ComplyEaze
  Bridge's data folder; (4) tick the new Terms setting, check that the extension shows 0.5.0 and
  that "Allow voucher posting" is as you want it, and enter your other settings
  again only if they were cleared; (5) quit Claude Desktop
  completely and reopen it.

**What you can do now**

- `masters` can list a company's cost centres (`cost_centres`, with each centre's category) and cost categories (`cost_categories`, with their three allocation flags). They are returned whether or not the company's Cost Centres setting is on: a book whose setting read No still returned its two centres, so a No setting is not "no centres", and an empty list does not say whether the feature is off or none is defined. A centre without a category, and an answer whose collection does not carry the type of the kind asked for, are refused rather than read as "none". An empty cost-category list is refused (`masters_cost_categories_empty`) on the expectation that the predefined Primary Cost Category always exists, which was seen in both books whose categories were captured and was not captured for a book with no centre defined. ComplyEaze Bridge does not return how a voucher was allocated to a cost centre. Measured on three synthetic books for cost centres and two for cost categories (the setting reading No on two, Yes on one: that book, captured in a separate sitting, returned its three centres, one under another, and the default category, in the same shape); a larger book is refused when its master mark is too large for the response budget (1,037 for cost centres, 1,264 for cost categories). Not measured: centres nested more than one level deep, an alias beyond a centre's own name, a larger book (#1398).
- `vouchers` and the sales and purchase registers now return `reference_date`, a voucher's
  `REFERENCEDATE` (`YYYYMMDD`), where the voucher has one (#1257). Checked once against a
  live TallyPrime 7.1 on one synthetic book: the element came back on every voucher, empty on all but one
  and filled on that one (a Purchase) and on a voucher keyed for the check; it was not checked on a sale or on
  another voucher type, and a value that is not a date refuses the read.
- `vouchers` and the `outstandings` party detail now reach a ledger by either
  spelling of its name when Tally's own name for it differs in case or symbols from the
  spelling its vouchers carry (26 of 4,017 ledgers in a separate census of 13 books,
  not reproducible from this repository), and `ledger_match` shows Tally's own name,
  with `ledger_row_spelling` when the vouchers spell it differently. A spelling
  that is two ledgers' is refused as `ledger_ambiguous`. The voucher filter and the
  trail still use the spelling the vouchers carry; `ledger_movement` and imports are
  unchanged. Not measured: that Tally accepts the voucher-row spelling when it
  differs from the ledger's own name, what a voucher row carries for such a ledger
  on a book other than the one measured, and what the outstandings report that the
  trail reads carries for it, so a party detail that finds no bill or no unallocated
  row for such a ledger says `report_spelling: not_established`; that the first name
  list is the primary language (#1085).
- `ledger_movement` now says what a slow read cost, as `vouchers` does: when its two reads of the voucher window together took 20 seconds or more, or the census was 16 reads or more, the result carries `read_cost` with both reads' observed seconds, a floor derived from the gaps between census reads (derived, not measured), and whether those two voucher reads together stayed inside the one measured host limit, which leaves out the two ledger catalogue reads, the company check and the wider read an empty window gets (#1239). It makes no estimate for another window, adds no request and refuses nothing. Checked once live on a synthetic book whose voucher mark is 29,900: a one-day window (82 vouchers) took 24.6 seconds and carried no block; August (2,542 vouchers) took 80.9 seconds and carried a block of 65 seconds over the two voucher reads, 4 census reads and a floor of 1 second, its voucher and census counts equal to the relay's own log. Not measured: the sixteen-census-read case (a much larger book) and a call that is slow for reasons the block leaves out.
- `cash_flow` reads Tally's own Cash Flow for whole months (the month-wise
  movement of the cash and bank ledgers, not a cash flow statement under AS 3). The
  months are returned only after their net total has been compared with the
  trial balance's cash and bank ledgers and found equal; the split into months is
  Tally's own and is not checked, and Tally's debit and credit columns are not
  returned. A ledger under Bank OD A/c or Bank OCC A/c with movement refuses the
  result: Tally was seen counting one such ledger, which the check does not yet
  count, and the refusal is covered by tests and has not been seen against Tally.
  The tool was run against Tally on two synthetic books: the net total tied on
  five windows (three with only debits, two with a credit amount, one of them
  with a positive closing as well) and a quiet window was answered as nothing to
  compare. Tally's own answers were captured on four synthetic books in all, one
  of them a year of empty months on a large book. A Bank OD credit or a Bank OCC ledger, a contra, a month with entries that
  cancel, a window ending in February, a window crossing a financial year,
  optional or post-dated vouchers, a several-currency book and a large book with
  cash activity are not measured (#1232).
- `vouchers` can now find a voucher by its number, reference, a phrase of its
  narration or an amount (`voucher_number`, `reference`, `narration_contains`,
  `amount`), and can add a window up by ledger, month, voucher type, group or
  primary group (`summarise_by`), with debit, credit and voucher counts per bucket and the
  cancelled, optional and entry-less vouchers counted apart. Search and the
  ledger, month and voucher-type summaries run on the rows the window read already
  holds and send no new request to Tally; the group and primary-group summaries
  also read the ledger list and the group list, before and after the window. They
  refuse when any ledger's group chain, or the set of ledgers, changed meanwhile, and
  when a ledger the window touches cannot be placed, so no entry is left out of a
  group total, naming the ledger it could not place. A book whose master-alteration
  mark is above 11,983 (a provisional limit, computed and not measured, about half of what the transport admits for the same list) is refused before the
  ledger list is read. A `group` summary also
  gives each group's whole figure, descendants included, as `subtree_totals`, and every
  row says what it `covers`. A zero from a counted window is a checked zero. A narration phrase is refused
  where narrations are withheld from the assistant. A summary sums post-dated
  vouchers (counted, with the vouchers Tally sent no flag for counted apart) and any
  non-posting voucher type a book uses, and says so in the result. Checked once
  against a live TallyPrime 7.1 on a synthetic book of 67 vouchers: every bucket
  equalled the sums over the listed vouchers, the ledger buckets equalled
  `trial_balance` for the same year, and each search returned what the same
  criterion selects from the listing; it did not cover a large book, a memorandum,
  a reversing journal or a voucher withheld for a foreign-currency amount. That check
  covered search and the ledger, month and voucher-type summaries. A second check on
  7 October 2026 on the same book read the whole year once as `group` and once as
  `primary_group`: 12 and 8 buckets over the 64 vouchers that count (a cancelled, an optional and
  an entry-less voucher left out); every group bucket equalled the ledger buckets added up by the
  trial balance's own parent column, the 18 `subtree_totals` equalled the trial balance rolled up
  the group tree, and each call took 64 requests (the window's 34 and the group modes' 30, as counted from the code) and
  about 11 seconds. It did not cover a large book, a group renamed or moved while a window is
  read, or a held later page (#1230).

**Safer or fixed**

- `purchase_register` and `sales_register` now take their `state` from the rule
  `vouchers` uses. A non-empty window is `complete` only when every voucher read
  was checked against a separate count of the window. Before, the registers called
  such a window `complete` on the company marks and the ledger masters alone, so
  the same window had two answers. A row's `status` is unchanged. Not measured live:
  a window admitted against a census through the registers on a large book (#1031).
- `vouchers`, `voucher_presence`, `purchase_register` and `sales_register` now
  count a small book's window too (a company with a few dozen vouchers or fewer),
  where they used to read it without a count. A nonempty window of such a book can
  now be `complete`, and `voucher_presence` can say `absent` for it, which it
  could not before. The cost is one more read of the book's vouchers: six more
  requests to Tally on every call that counts (a `vouchers` call on a small book
  goes from 16 to 22 requests, `voucher_presence` from 28 to 34); the added time
  was measured on windows of one voucher, on debug builds, at 0.5 to 2 seconds a
  call (one run of the day and week calls, three of the presence call), and not
  on a larger book. `absent` still needs a window proven complete, nothing
  resembling the proposal, the voucher number of a `manual` type, and a party that was
  supplied and compared completely: a proposal with no party (a Journal), no manual
  number, or a party whose comparison was cut short reads `possibly_present` with its
  reason, however complete the window. A voucher created, altered or deleted between the
  count and the read now refuses a small book as `voucher_window_part_not_admitted`
  (cause `part_census_mismatch`), as a large one always did. The Ledger Entries
  screen reads through `vouchers`' window read, so it counts too and meets the
  same refusal. An empty window of a small book is answered as before, by the
  check of an empty window: the count adds its requests there and changes
  nothing else. `ledger_movement`, `verify_import`, `outstandings` and that check
  are unchanged. Not measured against a live book: what Tally does when it cannot serve the
  count, which refuses the call as `voucher_window_volume_unestimated` where the small book used
  to answer `partial` (it served the count every time in the run above), a
  larger book, and the registers and the Ledger Entries screen, which use the same
  count but were not called live (#1029).
- `profit_and_loss` now compares Tally's `Cost of Sales :` heading with the
  derived cost of sales (Purchase Accounts plus Direct Expenses) even when the
  heading reads zero or empty. Over a non-zero cost of sales such a heading
  refuses gross and net as `tally_profit_and_loss_differs`, naming the heading;
  over a zero one it ties. Before, a zero or empty heading passed whatever the
  cost of sales was. Both committed Profit and Loss captures still tie; whether a
  real Tally ever prints the heading zero or empty over a non-zero cost of sales
  is not measured (#1070).
- `post_import` and `verify_import` proofs now measure `alter_id_delta` from the
  company's voucher mark read just before the POST, for a native post, instead
  of from the mark recorded when the batch was built. Anything posted between
  the build and the POST no longer counts as this post's, so the delta agrees
  with what Tally created. `alter_id_delta` gains `from` (`pre_post_mark`, or
  `build_mark` for a file imported by hand, which has no POST of its own);
  `pre_import_mark` still reports the build-time mark (#1087).
- `build_import_xml` now refuses a batch that names a ledger which keeps bills in
  Tally, as `bill_wise_party_unapproved`, until the person approves each such
  party (the build is repeated with `on_account_approvals`): an entry on it carries no bill allocation, so Tally lands it On Account
  and the person must match it to a bill by hand. Before, the build only printed a
  warning. `post_import` and the queue admission read the flag again, from the
  ledger list they already read, and refuse `import_bill_wise_changed` for a
  ledger that became bill-wise since the build. A batch built before this change
  is refused for posting as `import_batch_predates_bill_wise_record` and is
  rebuilt (check first whether its file was already imported by hand). The
  approval is the assistant's word, not proof that a person said yes, and a hand
  import of the file is not checked. The approval dialog of a native post
  shows the person which ledgers these are: "On Account" stands before the name
  of each approved ledger, on each of its entries for one voucher and on its
  totals line for a batch, with one sentence saying the ledger was bill-wise
  when the batch was built and its entries carry no bill allocation; the
  desktop Journal review shows the mark and the sentence too. A batch's
  per-voucher lines carry no mark, and their heading then says it is marked
  below; in such a batch a narration that holds the sentence's opening is
  withheld on its per-voucher line. A dialog that
  cannot show the mark refuses. The mark adds no line, only characters, so a
  batch already at the dialog's 100-character line limit or its character
  limit can now be refused as too large (`import_review_too_large`). The
  dialog for one voucher ends with the voucher's own text: its reference and
  narration are the last two lines, quoted as they will be posted, under a
  line saying that no line below it is an entry, a total or an instruction;
  every line the dialog writes itself stands above that line, and the date
  line says where the two are. That line and the note on the date line count
  toward the dialog's 100-character line limit and its 1,600-character limit.
  Looked at once on each system, with a one-voucher text at all three limits
  (24 lines, 1,600 characters, a 100-character narration line): on one Mac
  and on one Windows PC the whole text, its last two lines included, and the
  buttons were visible without scrolling. Not measured: a smaller screen or a
  larger display scaling, a batch dialog on either system, a large book, and
  the bills of a ledger whose flag reads No (#1234).
- Versions 0.3.0 to 0.4.2 refuse to post a batch this version builds. They do
  not read the cash-in-hand and bill-wise records a saved batch now carries,
  so after a rollback they could have posted it with neither check; the batch's
  ledger binding is now saved under a name they do not read, and they refuse a
  batch with no binding before any approval window or request to Tally
  (`import_batch_predates_ledger_binding`, nothing posted). Their message for
  that refusal says "Build the batch again"; for a batch this version built,
  reinstall this version (or a newer one) and post the batch from it instead,
  because a batch built again on an older version has neither check. A batch an
  earlier version saved is read as before. Which versions can read the import
  history is unchanged: from the first post attempted with 0.4.2 or later,
  only 0.4.2 and later read it. 0.2.0 and earlier have no such refusal: do not
  run them over a data folder this version has built in (#1234).
- The posting setting is shorter and names its four known limits in plainer
  words, and Terms of Use section 9.2 (version 2026-10.1, effective 7 October
  2026) now lists the same limits and more: a company or ledger renamed or opened
  at the moment of a post, another Tally connector that can change entries
  without the approval window, and that dealing with a voucher that reaches the
  wrong company remains yours. The setting that accepts the Terms is now named
  for version 2026-10.1, so the new setting has to be accepted after an update
  (see "Should I upgrade?"); version 2026-10 stays published for the builds that asked for it
  (#1010).
- When a ledger name you gave is not in the book, the candidates now list ahead
  of the others the ledgers that hold every word you typed
  (`rule`: `shared_every_distinctive_token`), then those that share only some;
  a ledger matched by a stronger rule still comes first. The same ledgers are
  found and counted as before and none is chosen for you; when more than 25 are
  found, the ones left off the list are the ledgers holding only some of the
  words before those holding all of them (#1076).
- Five lists of a batch's ledgers in refusals now come in the batch's own
  order, not in name order. `masters` of `masters_not_exact`, `ledger_twins`
  and `ledgers_changed` list a ledger where the batch first names it;
  `refused_ledgers` of `cash_bank_ledger_not_established` lists a ledger where
  one of its entries is first refused, and `refused_ledgers` of the cash-answer
  refusals (in `build_import_xml` and `post_import`) where an answer first
  names it. The order is the same whether or not party names are masked. The
  same ledgers are found and refused as before.
  Where a list is cut (`ledgers_changed` names eight; a small response cap
  leaves rows of `refused_ledgers` out), which ledgers are named, and how many
  `refused_ledgers_omitted` counts, can differ from before. The ledgers a
  `post_import` or `verify_import` answer names in `masters_after_post.ledgers`
  come in the same order, where the batch first names each, in the answer and
  in the proof saved from it (the desktop screen names the first eight of
  them); the recorded verdict, and the review dialog that reads it, keep the
  order they were recorded in. Other lists are unchanged (#1234).
- The approved-party list in the build result is new in this release and
  follows the masking setting, as the refusal's list does. Under
  `mask_parties` the party lists `build_import_xml` returns come in the order of
  each party's digest, not of the names, so the order does not follow the
  alphabet of the real names; the order can change from one call to the next
  (#1234, #1338, #1345).
- What redaction does and does not hide is now stated in the extension. Its
  description, its Response redaction setting and the descriptions of the two
  log tools say that redaction only shortens party and ledger names and bank
  account numbers (or drops narrations), and does not hide amounts, company
  names, PAN, GSTIN, IFSC, MSME or Udyam numbers, contact details or
  references, nor a name written inside a narration or reference. This replaces
  the earlier wording "can only mask party names or drop narration". Not
  measured: the shortening of bank account numbers rests on code, not on a live
  capture (#1046, #1364).
- `post_import` refuses a business-cash entry whose cash ledger was moved out of
  Cash-in-Hand after the batch was built, before any approval and again when the
  post is queued (`cash_ledger_not_cash_in_hand`); a batch built before this
  release is built again once (`import_batch_predates_cash_ledger_record`).
  Tested on captured data only; not measured: a live regroup between build and
  post (#815, #1228, #1229).
- `build_import_xml` refuses a bank statement built onto a Cash-in-Hand ledger
  (`statement_bank_ledger_not_a_bank`), and warns, without refusing, about a
  suspense ledger that is outside Suspense A/c, whose group is not established,
  or that is not in the book. No live run; the read-only check of the
  bank ledger in Tally against the statement is not in this build (#1211, #1214).
- An HDFC statement that 0.4.2 refused with `unparseable_date` at the last row of
  a page is read with the statement-period line skipped, so its dates are no
  longer mixed into the previous row. No real statement has been run through a
  package with this fix; one whose period line does not match still ends in the
  old refusal (#1310, #1325).
- When Tally rejects a post entirely, the post's summary now says so and no
  longer suggests posting again. Not measured: a live post that Tally rejects
  whole (#1108, #1213).
- A busy Tally port gets four answers. Busy before `post_import` sends anything:
  the refusal tells the assistant to call `post_import` again with the same
  batch after 5 seconds, that you will be asked to approve again, and not to
  rebuild it. Busy after the send: it tells the assistant to use
  `verify_import` with the original batch and never to post again. Whether the
  post was sent could not be observed: the refusal says to use `verify_import`
  with the original batch. The second pass of an approved post has no time left:
  it refuses at once as `tally_endpoint_busy` with nothing sent (#869, #893,
  #1219, #1244).
- The approval dialog for a batch of up to ten vouchers lists each voucher on
  one line (type, date, amount, ledger, narration cut at 40 characters); a larger
  batch, one with an unsafe narration or reference, or one whose lines do not
  fit the dialog shows the totals with a line saying why. A narration that reads like a line of the dialog itself is not
  shown. A single-voucher dialog shows the narration without its outer spaces,
  as posted. The per-voucher lines were seen on macOS only (#1055, #1063,
  #1223, #1226, #1278, #1283).
- `ledger_movement` no longer refuses a read because of a zero entry written as
  -0.00 and flagged with Tally's debit-side sign flag. We have not seen Tally
  send one (#1251, #1265).
- `ledger_masters`: a refused native ledger export now names a fixed cause
  (`native_collection_not_success`, `native_collection_status_absent`,
  `native_collection_malformed_response`, `native_collection_identity_mismatch`,
  `native_collection_bounds_exceeded`, `native_collection_row_unusable`); a
  company ledger-count answer past the response cap refuses as
  `ledger_count_company_response_too_large`; the catalogue read is sent once and
  a timeout refuses as `ledger_export_invalid` with `request_deadline_exceeded`
  (#718, #937, #1033, #1144, #1167, #1184, #1216).
- A refused date carries one next step: give each date as YYYYMMDD or YYYY-MM-DD,
  a real day, with from no later than to (#1268, #1273).
- `verify_import` lists, on a voucher cancelled in Tally, any effective unmarked
  copy entered again by hand (`effective_copies_observed`), as a report only
  that does not attribute it. Proven on Journal vouchers only (#806, #1221).
- The tool descriptions the assistant reads are clearer: every input is
  described, six easily confused pairs end with "use X instead", and the two log
  tools say when to use each and that reading changes nothing but its own
  receipt line. The `ledger_masters` description is about 4 percent shorter than
  in 0.4.2 (6,714 to 6,448 characters in the source) without dropping a limit or safety
  phrase, and the `vouchers` description says a bill allocation carries its bill
  date and credit period when Tally sends them. No claim is made that assistants
  choose better (#945, #1155, #1157, #1161, #1164, #1166, #1176, #1180, #1191,
  #1260).
- The extension's listing text (a long description, keywords and links) is
  added to the manifest; it says the extension is untested outside Claude Desktop
  and that ComplyEaze Bridge is not affiliated with Tally Solutions (#1375).
- ComplyEaze Bridge is named in full in the text of the desktop app's screens
  and messages that these changes reached. The app's product name and window
  title in its configuration still read "Bridge", and some error strings in
  its source still begin with it (#962, #1163, #1165, #1168, #1170, #1179, #1190).

**Known limits**

- Search and summaries were checked on one synthetic book of 67 vouchers; large
  books, masking and foreign-currency vouchers are not checked, and a group
  summary is refused above a provisional limit that was not measured (#1230).
- On a large book a voucher read is practical a day at a time: one day took
  about 170 seconds on the largest book measured, and a month is read day by day.
  What a read cost is stated after it, not before (#485, #595, #703, #1239).
- The either-spelling ledger lookup was checked live on one book and one ledger;
  a second check on a book where a ledger's spelling differs from its own name is
  pending (#1085).
- Bill-wise approval is the assistant's word, not proof that a person said yes;
  batch posting is not offered (#1090, #1234).
- A bill's trail and an unallocated amount's detail read the whole date window;
  a party-scoped request is not built (#945, #1356). The slice L captures of
  #1257 (a duty-head change or delete moving the master change counter; an item
  purchase on the class-entry shape) have not been run.
- Whether a company has cost centres or GST switched on, how a voucher is
  allocated to a cost centre, and bank-reconciliation fields are not read
  (`trial_balance` and `cash_flow` do return the company's currency, and
  `stock_summary` the inventory settings) (#1231).
  Tally's Funds Flow, Ratio Analysis, Negative Stock and Negative Ledgers
  reports are not read (#1358).
- GST Sales and Purchase posting are not in this build (#628).
- No Windows install of this build has been tried in Claude Desktop; the
  per-voucher approval lines were seen on macOS only.
- Posting limits: Terms of Use section 9.2 lists the four known limits.
- What Claude's AI provider receives is stated in the README and the security
  page; the list there is open, and redaction shortens names rather than hiding
  them (#1350, #1354, #1361, #1368).
- A bank statement is not checked against the bank ledger in Tally (#1211); the
  HDFC fix has not been run on a real statement.

## [0.4.2] - 2026-10-03

### In plain words: ComplyEaze Bridge 0.4.2, since 0.4.1 (2 Oct 2026)

These changes are in ComplyEaze Bridge 0.4.2. Each line names the pull
requests it comes from, except where it names an issue. A "mark" below is a
counter Tally keeps that moves when vouchers or ledgers change.

**Should I upgrade?**

- **Remove the old extension first.** 0.4.2 installs beside 0.4.1 or earlier
  instead of replacing it, because its author line changed from "Bridge
  contributors" to "ComplyEaze contributors" and Claude Desktop builds an
  extension's identity partly from its author name (seen with 0.4.1 on a Mac;
  earlier releases carry the same author line, so the same is expected; we did
  not try Windows). Before installing 0.4.2, in Claude Desktop open Settings,
  Extensions and remove the older ComplyEaze Bridge (release 0.2.0 shows as
  "Bridge Tally"). Keep ComplyEaze Bridge's data folder: both versions use it,
  so do not delete it. After installing, enter your settings again: the Tally
  port, the posting setting (posting starts off), the Terms setting, which every
  tool needs, and Response redaction, which starts at none (set it again if you
  had shortened or masked names). If you already have two, remove the older one:
  once 0.4.2 has tried a post, the older one can no longer prepare, post or
  check vouchers (see the posting points below).
- **The number is a patch number on a larger change.** 0.4.2 adds a tool, asks
  you about ledger names it used to read, and once you post with it you cannot
  go back to 0.4.1.
- **If you use 0.3.0 or 0.4.0 on Windows: yes.** 0.4.2 contains everything in
  0.4.1, including the fix for the network-path forms of the bank-statement path
  issue published as
  [advisory GHSA-vm5g-r3p7-wxx7](https://github.com/ComplyEaze/bridge/security/advisories/GHSA-vm5g-r3p7-wxx7).
  Some paths still pass; the advisory lists which. If you cannot upgrade yet,
  follow the advisory's steps.
- **On a Mac with 0.3.0 or 0.4.0: recommended, though not urgent.** The advisory
  rates the issue low there; some paths still pass, and the advisory lists
  which.
- **If you use 0.4.1: nothing below is urgent.** The reasons to upgrade are that
  a ledger name that differs by a symbol, an accent or run-together words now
  makes the assistant ask you instead of being read, later pages of a long
  voucher list can be served faster, and a post that Tally refuses is now named
  plainly.
- **If you used 0.4.1 or earlier: check answers for a ledger named loosely.** If
  you asked about a ledger by a name that was not its exact spelling, the
  figures may be for a different ledger with the same letters and digits, and an
  earlier `vouchers` answer did not show which ledger was matched. Ask again
  with 0.4.2, which asks you instead (#1092).
- **If you turn posting on, the way a posted voucher is matched has changed.** A
  post no longer adds a ComplyEaze Bridge tag to the narration, and each voucher
  is matched by its place in the run of changes Tally records for that post
  (#1054). From the first post 0.4.2 sends to Tally (its record is written just
  before sending, so this includes a post Tally refuses or never answers), do
  not run 0.4.1 or earlier, of the extension or of the desktop app, on this
  computer: the record of what was sent (the import journal) then holds fields
  they refuse to read, and they stop preparing, posting and checking vouchers.
  Before that first post, going back is harmless. The one-voucher post that the
  extension makes was not run live with this matching (see Known limits).
- **If you turn posting on: `voucher_presence` cannot recognise these
  vouchers.** It cannot identify a voucher this version posted (it matches a
  ComplyEaze Bridge voucher by the tag only for files you imported by hand), so
  a voucher posted with 0.4.2 that someone then edited or re-dated in Tally can
  read `absent` there. Check what ComplyEaze Bridge posted with `verify_import`,
  never with `voucher_presence`, before entering any of it again by hand. In
  Tally's own screens these vouchers can no longer be told from hand-entered
  ones by their narration. Posting is off in a new install, and 0.4.2 installs
  as one: check the setting after you install it.
- **How:** ComplyEaze Bridge does not update itself. Follow the
  [installation guide](https://github.com/ComplyEaze/bridge/blob/master/docs/agent/INSTALL.md):
  (1) close any other program that runs ComplyEaze Bridge; (2) in Claude
  Desktop, remove the older ComplyEaze Bridge in Settings, Extensions, then
  install the new file from the same screen; (3) keep ComplyEaze Bridge's data
  folder, which holds its record of what it has sent to Tally (from 0.4.2 it is
  the only record of which vouchers it posted); (4) enter your settings again,
  including Response redaction (it starts at none), and check that the extension
  shows 0.4.2 and that "Allow voucher posting" is as you want it; (5) quit
  Claude Desktop completely and reopen it.
- **What was tried** is under "Known limits" below. No one on our side installed
  the Windows package of this build in Claude Desktop on a Windows PC.

**What you can do now**

- **A new tool, `sales_register`.** It reads a register of the tax in the books
  for sales: the Sales and Credit Note vouchers of a date window that touch a
  Duties & Taxes ledger, with each entry's tax taken only from the GST duty head
  on that ledger's master, never from a name or an amount. It reads as
  `purchase_register` does, and it is a register of the books, not a GST return.
  A Credit Note keeps Tally's signs (nothing is netted or flipped, so add signed
  amounts), and may be a sales return or a credit note to a supplier: each row
  says the party's group and the tool does not choose. A Debit Note is listed
  apart, without an amount. The state-side tax head has two recognised forms,
  `state_tax` and `sgst_utgst`, for the same side. A row is marked
  `not_measured_live` only for kinds the row itself shows (for example
  inter-state, cancelled, optional or post-dated); an unmarked row is not
  thereby measured, because a sale typed on Tally's screen, a tax Tally
  computed, a duty head such as cess, an invoice with several goods lines and a
  currency other than the book's base cannot be marked. It needs a book whose
  base currency is INR and refuses a book too large to list. It reads the whole
  date window once and the ledger masters twice (before and after), and all of
  it again for every page. One call sent 96 requests to Tally on one small book
  and 118 on another. Use a narrow date range: its cost on a large book was not
  measured. It was run against a live Tally, once each, on synthetic companies:
  one taxed Sales item invoice, one untaxed one (read once by an earlier build;
  its masters and the tool's answer were not kept) and one Credit Note. What was
  not shown is under "Known limits" (#1009).
- **Later pages of a long voucher list can be served from the first page's
  read.** `vouchers` no longer reads the whole window again for every page when
  the window was read `complete`, which means counted first. On one synthetic
  book, one month of 2,542 vouchers read in a release build, the first page took
  about 66 to 68 seconds and a later page about 1 second (one run each). A later
  page that names the first page's `snapshot_id` is refused if the company's
  marks moved or the held read is gone, instead of continuing from a different
  read. A read is held for ten minutes and is also dropped after a write through
  ComplyEaze Bridge, when the size cap that all held windows share (64 MiB)
  evicts it, or when a newer read of the same question replaces it. A small or
  new company is not counted, so its window reads `partial` and every page reads
  it again. A later page that does not name the `snapshot_id` reads afresh when
  the books moved, and says so (`earlier_snapshot`: its offsets do not continue
  the earlier pages, so start again from the first page); when the hold ran out
  it reads afresh without that flag, and only a page whose `snapshot.reused` is
  true continues the earlier ones. A date is the same question as `2026-08-01`
  or `20260801`, while the `ledger` argument must be repeated exactly as typed
  (#1053, #1118).
- **A refused ledger name now tells the assistant to ask you which ledger you
  mean.** When `ledger_movement`, `vouchers` (with a ledger) or the party detail
  of `outstandings` cannot find the name you gave, the refusal can now list the
  ledgers it may mean (not when party names are masked), and the assistant is
  told to ask you, even when there is one candidate, and never to choose. A name
  is now read without asking only when it matches a ledger's spelling exactly,
  or differs only in upper or lower case English letters or in spaces. Before, a
  name that differed by a dropped symbol, an accent or run-together words was
  read if one ledger had the same letters and digits; in a lab list of synthetic
  ledger names that read the truncated name `Input Cess (` as a different
  ledger, `Input Cess`. Each answer for a named ledger now says how the name
  matched. A name containing a no-break, figure, narrow no-break or ideographic
  space asks, as a test now pins (#1057, #1092, #1127; issue #1095).
- **The assistant is told how to pick a company, and to say which one it used.**
  `list_companies` now begins "Start here" and states the rule (use the open
  company only if it is the only one open and you named no client, or if exactly
  one open company matches the client you named; otherwise ask, and never
  guess), and Claude Desktop is sent the same rule at start-up, with the
  instruction to name the company, dates and ledger in the first line of an
  answer, and what to do after a refusal. The start-up text follows your posting
  setting: with posting off or import only, it says this connection cannot post
  to Tally. It also tells the assistant to ask before reading vouchers over more
  than a month unless you gave the period, and before an `outstandings` party
  detail (#1058, #1113).
- **A plain opening line on three reads.** `trial_balance`, `profit_and_loss`
  and `balance_sheet` now begin with a `headline` when they answer: the company,
  the exact period and, in words, whether the figures were read for every
  ledger, partial, or established or not. A refusal has no headline yet. The
  codes and figures stay in `result` unchanged (#1062, #1066).
- **`outstandings` says what it counted and which date it used.** It returns
  `open_bills_total` (every open bill in that direction, counted before paging)
  and `open_bills_shown`, so a cut page no longer reads like the whole list (on
  a partial read these cover base-currency ledgers only). In every answer that
  reads (a refusal carries none) it returns the `as_of` date it used, which is
  today's date on this computer when you gave none; `tally_status` returns that
  date as `today` (#1049, #1079).
- **Pages that are not in the package:** a Questions page on the install site
  with a table of what it can and cannot do, a before-you-start list and plain
  answers, including how far our liability goes and how to reach us (#1061,
  #1084); a "What it costs" section in the README and a line on the Download
  page: ComplyEaze has set no price and sells no licence today (#1059); the
  Privacy Policy (version 2026-10.1, effective 3 October 2026) and every footer
  now name the publisher with its registration number and registered office, the
  policy adds a purpose for publishing written quotes you consent to, and says a
  grievance is answered within one month (it said 30 days) (#1104). The site
  also gained canonical addresses, a sitemap and an `llms.txt` (#1047).

**Safer or fixed**

- **A withdrawn approval is named.** If an approval is withdrawn after the post
  has taken it but before anything is recorded or sent, `post_import` now
  answers `import_approval_revoked`; before, it answered
  `import_dispatch_outcome_unknown` although nothing had been recorded or sent.
  The message ("No posting attempt was recorded. Review the error before
  requesting approval again.") and the `attempt_recorded: false` field are
  unchanged, and no next step is attached. It was tested on a scripted Tally,
  not a live one (#1131; issue #791).
- **A post that Tally refuses is named plainly.** If Tally's answer to a post
  counted nothing created (`CREATED 0`, an exception for each voucher sent and
  every other counter reported as 0) and the voucher is not found, `post_import`
  now says Tally reported it as not created, to check it is not in Tally, and to
  enter that one voucher in Tally's voucher entry screen, never through Tally's
  Import menu; in the answer to that post itself the voucher's status reads
  `tally_reported_not_created`. This covers a single voucher, which is what the
  extension posts, and a saved batch of two or more vouchers rejected whole when
  the company's mark was read before and after and did not move (batch posting
  is an environment setting, `BRIDGE_AGENT_ENABLE_BATCH_POST`, that the
  extension's settings do not offer). Tally's answer was captured live only for
  missing ledgers, on TallyPrime 7.1 Silver (one Education answer to a bad date
  was also used, for its counter shape only), and the reading of it was tested
  on those saved answers, not in a live post; the code applies it to any answer
  of that shape, including on Gold, where it was not measured. After you enter
  the voucher by hand as told, the saved batch stays "reconciliation required"
  and ComplyEaze Bridge cannot close it (#1039 is open; #1116, #1126).
- **Where a posted voucher cannot be matched, the status changed.** In 0.4.1 an
  unfound voucher of a post read `not_found`. In 0.4.2 a voucher of a post made
  with 0.4.2 that cannot be matched reads `sent_not_attributed` ("check in Tally
  before posting it again", for example after an edit), never `not_found`; posts
  made with 0.4.1 or earlier are read as before. That is also the reading for a
  batch that landed in part, and for a later `verify_import`, including of a
  voucher Tally refused. Other new readings name why a voucher is missing:
  `bound_not_in_window` (it may have been deleted, re-dated or restored from a
  backup; whether an edit does this was not tried: check in Tally) and
  `book_rolled_back` (the books look older than the post). A post is refused as
  `post_mark_unrecorded`, with nothing sent and the approval withdrawn, when the
  company's voucher mark cannot be read before it (#1054; #1116, #1126 and #1107
  for the refused and part-landed cases).
- **Clearer next steps on refused batches.** When no attempt was made, a saved
  batch refused because a narration, reference or voucher number cannot be
  posted now says that nothing was sent, and to correct the text, build the
  batch again and post the new one (before, it said to review the error before
  asking for approval again, which fails the same way each time) (#1105). A
  batch that mixes a Journal with Payment, Receipt or Contra vouchers is refused
  as before, and now says to split it into one batch of Journals and one of the
  others (#1094). A date or company ID refused before anything is read now says
  what it should have been: a date as text in the form YYYYMMDD or YYYY-MM-DD,
  and the 36-character company ID from `list_companies`, not its name (#1115).
  The refusal for a changed ledger count names an altered ledger as a possible
  cause, marked as reasoned and not reproduced (#1032).
- **The posting path says what is sent.** Narrations are sent as given, and the
  approval dialogs say "narrations sent as prepared, nothing added" instead of
  saying a batch reference is added. A narration that contains the character
  U+FFFD (a diamond with a question mark) followed by `#`, digits and `;` is now
  refused when the batch is built, because it is compared byte for byte. The
  file you import by hand keeps its tag. A bound voucher whose amount was
  changed in Tally reads `posted_divergent`, and `acknowledge_post_review` finds
  a bound voucher by its Tally ID (#1054).
- **`purchase_register` explains a cancelled Purchase.** A cancelled Purchase or
  Debit Note sits in the list of vouchers without a tax entry, with `cancelled:
  true`, because the cancelled vouchers measured came back from Tally with no
  ledger entries, so being listed there does not mean the voucher was untaxed. A
  cancelled voucher that keeps its entries has not been measured (issue #1013;
  #1027).
- **A statement line that reads 0 and 0 is compared, not skipped.**
  `profit_and_loss` and `balance_sheet` now compare a Tally statement line that
  has `0.00` present in both columns with the line derived from the trial
  balance; a derived amount over such a line is reported as a difference and the
  statement is marked not established, with its derived lines withheld, instead
  of being called established. On four synthetic lab books every zero line came
  back empty in both columns, so this has not been seen in a real answer (issue
  #1067; #1071).
- **`outstandings` now reads one far-future due date.** A due date that Tally
  prints with a four-digit year of 2100 or later (such as `1-Dec-2108`) is read
  as written, and gives no overdue days in the ageing; before, that one bill
  failed the whole read with no reason. A bill row whose dates cannot be read
  still refuses, and the refusal now carries a named cause, the report, the row
  number when a row is what failed (also for an error found while the report is
  read, such as an unreadable amount or a repeated field, but not for a value
  that appears before any bill row), and a next step, never the bill's party,
  reference or date. A whole call on the synthetic book that printed such a date
  has since completed once (see Known limits) (#1098, #1128, #1136; issues
  #1091, #1096).
- **The two log tools now say what they do not show.** `read_evidence` and
  `egress_log` say what each holds and that neither shows what your AI provider
  received, and tell the assistant never to say that no data has left the
  computer. What the assistant reads, amounts included, goes to your AI provider
  (#1045, #1113).
- **Source only, not in the extension: the desktop app's "Show in folder" opens
  only a file it exported since it started.** ComplyEaze Bridge publishes no
  desktop installer, and the extension does not contain the desktop app. Any
  other path is refused before the file manager opens. A file replaced by a link
  after export would still open, the record keeps the 1,000 newest exports, and
  the Windows and Linux paths and the button itself were not tried by hand
  (#1051; issue #915).
- **Wording.** The extension's settings are shorter overall (the redaction
  setting now says more) and the three longest read descriptions lead with the
  outcome; assistant texts say "ComplyEaze Bridge" in full, and so do the
  desktop app's messages (source only); the extension's author line reads
  "ComplyEaze contributors", which is why 0.4.2 installs beside an older version
  (#1042); `acknowledge_post_review` names the refusal for a review recorded for
  a different doubt (#1026, #1028, #1124, #1129, #1130; issues #962, #1010,
  #830).

**Known limits**

- **`sales_register` was not shown** on an invoice-view Credit Note, an
  inter-state (IGST) line, a cancelled or optional sale, an unrecognised or
  missing duty head on a sale, more than one voucher in a window, paging, a
  company with a registration, a tax Tally computes itself, a duty head no sales
  run showed (such as cess), an invoice of another shape than the one run (for
  example several goods lines), a sale typed on Tally's screen,
  accounting-invoice mode, a post-dated sale, a `REFERENCE` or a filled
  `PARTYGSTIN` on a sale, a `REFERENCEDATE` on a sale, or a currency
  other than the book's base. The invoice it was replayed on was imported by
  this project. Its `complete` state rests on the company marks and the ledger
  masters reading the same before and after, not on a separate voucher count
  (#1009).
- **Long voucher lists:** a change that moves neither company mark is not seen
  between pages, a window larger than that cap is not held and is read again for
  each page, and what happens if someone changes the book at Tally's screen
  between two live pages, or a later page of a window with a voucher type or
  ledger filter, was not tried against a live Tally. Not established: whether a
  save by another Tally Gold user, a restored copy of the company, or a company
  setting change moves the marks at once (#1053, #1118).
- **Ledger names:** a spelling that contains the mask mark (`…` or `...`) and is
  not typed exactly as the ledger is spelled is refused as `ledger_name_masked`,
  whatever the setting. When `mask_parties` is on, a spelling that shares a key
  with another ledger's masked form is refused the same way, which can also
  refuse an ordinary name typed in a different case until it is typed exactly,
  and the refusal lists no candidates. The candidates were tested on saved Tally
  answers, not a live book, and the ledger-name changes on a lab list of 144
  synthetic names; a real book was not tried (#1057, #1092).
- **Posting, what was tried:** two live posts through ComplyEaze Bridge's own
  path, of 3 vouchers each (3 Journals; a Payment, a Receipt and a Contra), made
  with the environment batch setting, and raw runs against Tally of 10 untagged
  Payment, Receipt and Contra vouchers and of 5 Journals, all on a synthetic
  company in TallyPrime 7.1 Silver. The extension posts one voucher per
  approval; that one-voucher path was not run live with 0.4.2's untagged
  matching (#1054).
- **Posting, what is not seen or not closed:** if a post's vouchers cannot be
  matched (for example one was edited in Tally before the matching was made, or
  Tally stored a narration differently; only one Hindi-and-rupee-sign narration
  was checked), that is final: the batch stays "reconciliation required",
  `acknowledge_post_review` does not apply, and you check its vouchers in Tally
  yourself (#1039 is open). Another Tally Gold client writing during a post is
  not seen. After a backup is restored, if the books are keyed past the post's
  mark before the next check, the restore is not detected as one (the voucher
  reads `bound_not_in_window`); if Tally reuses the lost vouchers' IDs (not
  measured), a different voucher that took a lost voucher's ID and has the same
  content can read `posted_verified` (#1050). Such vouchers cannot be posted
  again through ComplyEaze Bridge and must be entered by hand (#1038). Not
  tried: whether a voucher keeps its Tally ID when someone edits it in Tally's
  screens (if it does not, `verify_import` reads that voucher as
  `bound_not_in_window`, which does not mean it was deleted), whether deleting a
  voucher can lower the company's mark, a Windows approval dialog, Education
  mode, and more than 3 vouchers through ComplyEaze Bridge's own path (#1054).
- **Statement gate (#1071):** whether a real Tally ever prints `0` in both
  columns is not shown; the `Cost of Sales :` heading is not compared when it
  reads zero or empty (#1070).
- **`outstandings` on a book with such a date:** one whole call on the synthetic
  book that printed `1-Dec-2108` completed in a lab run (state `complete`, 52
  requests, 4.0 seconds, a page of 500 of 1,498 open bills). The bill with that
  date was not on the page returned, later pages were not read, and other books
  were not tried. A bill dated after `as_of` still refuses, and a two-digit due
  year can be read in the wrong century, as before. A bill cannot be left out
  and the rest returned as partial, because that would misstate the party's
  balance (#1136, #1098; #1091 stays open).
- **Assistant text:** whether Claude Desktop passes the start-up instructions to
  the model at all, and whether any of the new wording changes what an assistant
  does, was not measured; the studies were plan-only, without tool calls or real
  users (#1058, #1113, #1115). The new opening lines were tested on saved Tally
  answers, and no live Tally was read for them, for the ledger candidates, for
  the date echo or for the open-bill counts (#1049, #1062, #1066, #1079).
- **Tried against TallyPrime** (7.1 Silver in a lab, synthetic companies, one
  run each unless a line above says otherwise): the `sales_register` reads, the
  paging run, the untagged-post runs, and the rejected-voucher answers. The
  other changes are covered by automated tests, with a saved real answer behind
  the due-date item; the statement fix is tested on hand-built lines, because no
  saved answer has a 0 / 0 line. The release check starts each package, lists
  its tools and reads a sample bank statement, and does not run against
  TallyPrime. No one on our side installed the Windows package of this build in
  Claude Desktop on a Windows PC.
- **Pages:** no lawyer or CA has read the Questions page answers on liability
  and claims or the changed Privacy Policy text; the registration number and
  address were checked against the certificate of incorporation, not the current
  record (#1084, #1104).

## [0.4.1] - 2026-10-02

### In plain words: ComplyEaze Bridge 0.4.1, since 0.4.0 (2 Oct 2026)

These changes are in ComplyEaze Bridge 0.4.1. Each line names the pull
requests it comes from, except where it names an issue.

**Should I upgrade?**

- **If you use 0.3.0 or 0.4.0 on Windows: yes.** 0.4.1 closes the network-path
  forms (a share, its WebDAV form, and the long-path prefixes) of a
  medium-severity security issue in how the bank-statement tool opens file
  paths, published as
  [advisory GHSA-vm5g-r3p7-wxx7](https://github.com/ComplyEaze/bridge/security/advisories/GHSA-vm5g-r3p7-wxx7).
  Some paths still pass; the advisory lists which. If you cannot upgrade yet,
  follow the advisory's steps.
- **On a Mac: recommended, though not urgent.** The advisory rates the issue
  low there, and the other changes below apply there too.
- **How:** ComplyEaze Bridge does not update itself. Follow the
  [installation guide](https://github.com/ComplyEaze/bridge/blob/master/docs/agent/INSTALL.md):
  (1) close any other program that runs ComplyEaze Bridge; (2) in Claude
  Desktop, install the newer file from Settings, Extensions; (3) keep
  ComplyEaze Bridge's data folder, which holds its record of what it has sent
  to Tally; (4) check that the extension shows 0.4.1 and that "Allow voucher
  posting" is as you want it, since an earlier default may still be saved as
  on; (5) quit Claude Desktop completely and reopen it.
- **What was tried** is under "Known limits" below. No one on our side
  installed the Windows package of this build in Claude Desktop on a Windows PC.

**What you can do now**

- No new tool. On a company large enough to be counted first,
  `voucher_presence` can now answer `absent` for a voucher it finds nowhere in
  a date range that holds vouchers and was checked against that count. An empty
  range is called absent only on a company that has never held a voucher, as
  before (#985, #1020).

**Safer or fixed**

- `parse_bank_statement` accepts a statement file or password file path only
  when its text names a local disk. A path that starts with two separators
  (two slashes, two backslashes or one of each), such as the Windows long-path
  form (starting with `\\?\`) or a network share (`\\computer\folder`), is
  now refused: copy the file to this computer and give its path there (#1024).
- `vouchers` and `voucher_presence` now call a date range with vouchers in it
  complete by one rule: only when every voucher read was checked against a
  separate count of that range. Before, `vouchers` called any such range
  complete (unless it withheld a voucher), and `voucher_presence` called none
  complete. On a small or new company, which ComplyEaze Bridge does not count
  first, both now say `partial` (#985, #1020).
- A voucher changed in Tally between the count and the read now refuses the
  read (`voucher_window_part_not_admitted`, cause `part_census_mismatch`),
  also when the range is read in one request, where it used to return without
  that check. Call again once while the book is quiet; if it repeats, read the
  range in Tally. `ledger_movement`, `purchase_register`, `build_import_xml`,
  verifying an import and the party detail of `outstandings` can give the same
  refusal. After a post was sent, this refusal on the read-back means the
  voucher may already be in Tally: use `verify_import` on the same batch and
  never post it again. In some cases, verifying an import for one day that
  Tally cannot serve in one request is now read in parts instead of refused,
  which sends more requests to Tally (#985, #1020).
- When `post_import` refuses a batch before it is sent because some of its
  rows may already be in the book (`import_preexisting_identity`), the answer
  now names those rows by their transaction ids and says what to check in Tally
  next, instead of returning a bare code, unless the response size limit
  leaves no room for the list. The same batches are refused as before (#901,
  #908).
- `vouchers` returns a voucher's `master_id` (Tally's internal voucher id) as
  the plain number (`"1"`), not as Tally sends it with a leading space
  (`" 1"`), so it matches the same id returned by `verify_import` (#989, #1021).
- If voucher posting is turned on, you approve a batch in its approval dialog,
  and the post is then refused before it is sent (for example because the
  import journal is busy, the batch is not found, or the call names a
  different company from the batch's), your approval is withdrawn, and the next
  post that gets past those checks asks you again. Before, in these cases the
  approval stayed held for up to 15 minutes, and posting any other batch was
  refused until it was used or lapsed (#857, #904).

**Known limits**

- A date range is called complete when the vouchers read match a separate
  count of the range. If Tally's own date selection leaves a voucher out, the
  count and the read both miss it, and the range is still called complete. A
  small or new company is not counted, so it cannot get `absent`; adding a
  count for it is tracked in #1029, with the multi-day and after-post cases
  (#985).
- `parse_bank_statement` checks the text of the path. A mapped drive letter
  still passes, and so, on a Mac, does a path under a mounted network volume
  (#1024).
- A batch refused just before it is sent can still return no rows and no next
  step. The rows named before that can be more than the vouchers the book
  holds, when several rows look the same; count the vouchers in Tally before
  leaving any row out. A row is still refused when a voucher that an earlier
  batch or a hand entry put in the book has the same date, type, ledgers,
  amounts and sides (#865, #901).
- Tried against TallyPrime 7.1 Silver in a lab on 2 October 2026, before
  #1020 merged: one-day ranges on two synthetic books, and the read-back of one
  posted Journal. Not tried: a range of several days read in one request, the
  new refusal itself (seen only in tests), how often it appears on a busy book
  with several users, and other Tally editions. The other changes have not been
  run against TallyPrime; each is covered by automated tests. No one on our
  side installed the Windows package of this build in Claude Desktop on a
  Windows PC. The release check starts each package, lists its tools and reads
  a sample bank statement, and does not run against TallyPrime.

## [0.4.0] - 2026-10-02

### In plain words: ComplyEaze Bridge 0.4.0, since `mcp-preview-0.3.0` (26 Sep 2026)

These changes are in ComplyEaze Bridge 0.4.0. Each line names the pull
requests it comes from, except where it names an issue.

**What you can do now**

- Read a company's masters as a list: voucher types (with their numbering
  method), godowns, units, stock groups, or ledger groups, in pages. The book is
  checked before and after the read. For godowns, units and stock groups, a book
  with very many masters is refused, with the size named, rather than answered
  in part (#952).
- See what a party's unallocated amount is made of, from data the tool already
  reads, and what that data cannot tell apart. Each bill now carries its own
  date and credit period, and payable and receivable follow the sign of the
  bill's balance (#945, #946, #957, #959, #961).
- See what ComplyEaze Bridge keeps on this computer. `local_data_report` (and
  `bridge_mcp --local-data-report` on the command line) counts files, bytes and
  the age of the oldest file by kind, and gives the state of the import journal
  and how many saved batches are not settled. It changes no book and deletes
  nothing; like every call, it records its own receipt line in the local log. It
  names no file path unless you ask on the command line (#925).
- Read the purchase register of tax in the books: the Purchase and Debit Note
  vouchers of a date window that touch a ledger under Duties & Taxes, with the
  tax amount each entry records under the ledger's GST duty head. Other voucher
  types that touch those ledgers (Sales, Journal, Payment) are listed apart with
  exact counts (at most 100 are listed), and an entry whose ledger has no recognised GST head is listed and
  never given one. Nothing is posted and nothing is inferred: whether an entry
  belongs in a return is for you to decide (#971).
- Ask `outstandings` for one party, with `detail`, to see why a bill is open and
  what an unallocated amount holds, party by party. A bill trail lists the
  allocations of each of the party's bills, oldest first, leaving out those in
  cancelled and optional vouchers, and says whether they
  add up to Tally's own balance for the bill. The unadjusted view lists the
  party's on-account, advance and pending credit or debit note allocations and
  compares the on-account total with its unallocated amount; a note is
  recognised only when its voucher type is named exactly Credit Note or Debit
  Note. A bill that does
  not tie, or whose identity is ambiguous, is shown as such and nothing is
  merged. The detail reads the company's vouchers from the start of the books,
  so a large book can be refused, with a next step (#981).
- Read Profit and Loss and Balance Sheet. A figure is shown only when it ties
  line for line to Tally's own statement. Otherwise ComplyEaze Bridge's own
  statement is withheld; Tally's own amount for each line is still returned,
  with the derived amount on each line that differs, and the lines that differ
  are named. A book with stock items is expected to be
  refused, because no such book has been measured, and the tie-out itself has
  been measured on two synthetic books only (#774).
- Read closing stock values per item. An item is returned only when the values
  add up to Tally's own Stock Summary and ComplyEaze Bridge read exactly as many
  items as Tally's own count; otherwise no item is returned, with the reason and
  the next step. A company with no stock items is told so. A value keeps
  Tally's sign, as in the Trial Balance: stock held is a negative number.
  Quantities are not returned, because nothing checks them, and names, parents
  and units are returned but not checked. Only a 31 March and small books are
  read (#980, #979, #1001).
- Read books that define more than one currency. Outstandings set aside
  foreign-currency ledgers, and a rupee ledger with a foreign-currency
  balance, and name them. Compliance ledgers and the Trial Balance are read
  through the book's base currency. The `vouchers` tool withholds a voucher
  whose amount Tally stored in a foreign currency instead of refusing the
  whole date window. The desktop app follows the same read (#642, #647, #649, #715,
  #781, #824, #825).
- Read a book too large for one read, in parts, including ledgers under parents
  that cannot be named, and have the ledger count cross-checked against the
  company's own count. A movement read names an oversized ledger catalogue and
  refuses an unknown ledger before it reads any voucher (#679, #885, #891, #936,
  #938, #939, #960).
- Recognise SGST/UTGST as a GST duty head (#968).
- The app and the extension carry the ComplyEaze Bridge tick logo as their icon
  (#950, #978).
- A ledger read on a several-currency book, or one whose base currency is not
  INR, is refused before any request to Tally, instead of returning bare
  numbers (#751).
- A recognised bank-statement cash line is now asked its purpose. An
  unanswered line blocks the import file; "don't know" posts to a suspense
  ledger, tagged and listed. Other unmapped cash lines still go to suspense,
  tagged and counted (#817).

**Safer or fixed**

- `profit_and_loss` and `balance_sheet` no longer open with `"state": "observed"`
  when nothing was established. The top-level `state` is `observed` only when
  the tool's result is established; otherwise it is `not_established`, with
  the same `reason` as the nested result. This changes the tool's output; no
  figure, check or withheld line changes (#984).
- Every tool is refused until the Terms of Use are accepted, and the extension
  carries a privacy policy (#943).
- The party-master balance snapshot ends at today's date, not at a stray voucher
  date (#896).
- The Markdown proof shows the verification status and any duplicate vouchers in
  the batch (#811).
- ComplyEaze Bridge sends one request to Tally at a time, across all its
  running processes (#883).
- Each send to Tally is recorded in the local log with its place, kind, size,
  outcome and time, and no book content (#918, #941).
- ComplyEaze Bridge refuses to build or post a row that another batch already
  sent to Tally (#876, #898).
- A bank statement's result returns no amount except the figures you supplied
  and an open cash line's amount, and names the saved proposals file by its id
  alone (#848, #850).
- The configured redaction also applies to ledger names in posting and
  verification results (#851).
- The native posting windows say ComplyEaze Bridge (#970).
- Each package's build is attested, and the attestation is checked before it is
  published (#923).
- If you are slow at the approval window, the agent's call no longer waits
  on it. The agent is told the approval is pending and asks again. A click
  made while no call is waiting is posted by the next call, not one call later
  (#792, #854).
- The approval window for a batch names how many vouchers it covers, in its
  title and its button. The review window shows a debit the way the post
  window does (#757, #762).
- A voucher you cancel in Tally after ComplyEaze Bridge posted it reads back
  as posted but not effective, not as changed. Cancelled vouchers are no
  longer reported as duplicates of each other (#771, #789).
- A recorded review can no longer answer for a different doubt than the one it
  covers (#755, #769, #809, #813, #831).
- Tally's own error text on a rejected line is read safely, including text
  with an `&` in it (#763).
- Each tool now says whether it changes anything. Reads are marked read-only
  and say they only record local receipt lines for the call, never book
  content. Some assistants may now run the read tools without asking each
  time (#909, #921).
- The tools that save a file on this computer are marked as writes: preparing
  an import file and reading a bank statement add new files, while
  verification and acknowledgement replace the batch's saved proof and are
  marked destructive. Posting stays marked destructive, and no tool is marked
  as reaching outside this computer. `parse_bank_statement` had been marked
  read-only by mistake (#909, #921).

**Known limits**

- Stock: the closing-value total is checked against Tally's own Stock Summary on
  one synthetic book, and the item count against the rows on one synthetic
  company, and the sign of a value was measured on one synthetic company. A
  book with many stock items is refused: the read is for small books, and
  typical stock-heavy books refuse today. A book in which no item carries a value
  returns nothing. A book with
  stock items is expected to be refused by Profit and Loss and Balance Sheet,
  because no such book has been measured (#774, #980, #1001).
- A book with very many masters is refused by `masters` for godowns, units and
  stock groups, and how common that is across real books has not been measured
  (#952).
- `local_data_report` has not been run on a Windows host, and how it reports a
  Windows junction is unverified. It does not cover the desktop app's other
  settings, its database or its logs (#925).
- The purchase register has been read from a disposable synthetic book only. A
  purchase typed on screen, item invoices whose purchase ledger sits in an
  inventory allocation, and books with several currencies are not covered, and
  it does not decide whether a Debit Note is a purchase return or a debit note to
  a customer (#971).
- The party detail of `outstandings` has been measured on one synthetic book.
  Its cost on a large book is not measured, and a window that needs too many
  requests is refused (#981).

**Also in source**

- The tax-audit engine gained its stock and party-monthly tests. The engine is
  still not in the extension, and its accuracy is not yet proven publicly (#738,
  #744, #788).
- The desktop app moved to a Tauri release that fixes GHSA-w28w-mhc8-qvjv
  (#805).

**Removed**

- The unfinished document-sync feature is gone from the source, and so from
  both the desktop app and the binary the Claude Desktop extension runs: the
  AXAL sign-in, the code that scanned a folder and uploaded the files you chose
  to ComplyEaze cloud storage, and the two hidden screens for them. No ComplyEaze Bridge
  tool could reach them, and no navigation led to them. After this change the
  only network client in ComplyEaze Bridge's own code connects to Tally on your own
  computer. The CI egress gate fails if a first-party crate other than the Tally
  transport depends on reqwest in its shipped dependencies (the app crate keeps
  it only as a dev-dependency) or if any first-party crate depends on hyper. It
  also counts the calls that send, connect or start a process in library and
  binary code, and refuses any outside the reviewed Tally files; test code is
  not counted. The unused delivery types in the portable core crate (no network
  code) went with it. The bulk party-statement export no longer records a hash
  for each file it writes and no longer refuses to write a file it cannot
  record; a record file left by an earlier development build is never read. The
  feature can be rebuilt from the git history if it is needed
  again (#914).

## [0.3.0] - 2026-09-26

### In plain words: `mcp-preview-0.3.0` (26 Sep 2026), since `mcp-preview-0.2.0` (16 Sep 2026)

These changes are in the published `mcp-preview-0.3.0` build. Each line names
the pull requests it comes from, except where it names an issue.

**What you can do now**

- Post a Payment, Receipt or Contra voucher, as well as a Journal. Each one
  still waits for your approval in a separate ComplyEaze Bridge window
  (#585, #600).
- Read password-protected bank statement PDFs into voucher proposals, including
  Union Bank of India statements (#444, #481).
- See a party's GSTIN as it stood on a date you choose, from its dated
  registration history (#657, #700).
- See each ledger's group ancestry, and the date its opening balance is as of
  (#498, #499, #569).
- See the post-dated flag, invoice and reference details, and the party GSTIN
  on voucher reads (#498).
- Pick vouchers by voucher class the way Tally classifies them (#663).
- Work with books of more than 1,000 ledgers in the ledger catalogue (#643).
- Get a whole `verify_import` verdict at once, with the verified vouchers in
  pages (#673).
- Source builds only: with `BRIDGE_AGENT_ENABLE_BATCH_POST` on as well as
  posting, post 2 to 50 vouchers of one saved batch after one approval, which
  shows a summary by ledger. It is off by default and not in the extension
  until a live batch post is proven. A batch counts as clean only when Tally's
  counts match exactly; otherwise you review it (#712, #721).

**Safer posting**

- Voucher posting is off by default in the Claude Desktop extension. Turn it
  on in the extension settings (#577).
- An approval counts only when the approval window returns a fresh one-time
  token. How the window closes no longer decides it (#665, #704).
- ComplyEaze Bridge confirms the company as its last step before posting, and
  reports where the voucher landed. A post can still land in a company that was
  renamed after that check (#607, #574).
- It refuses to post if a ledger changed since the voucher was prepared, if
  the company's masters changed between its final checks and the post, or if
  the saved file no longer matches its record (#578, #615, #616).
- If the company's masters change while a post is landing, the post is
  reported as not verified and needing reconciliation, so you check it in
  Tally (#623).
- It records each voucher's import identity before sending it, and never sends
  one twice: it refuses an import identity its own journal already records
  (#582, #678).
- It refuses to post into a book with more than one currency defined (#613).
- When Tally rejects a line, you see Tally's own error text, kept short and
  safe to display (#695).
- When you correct a voucher it built, ComplyEaze Bridge checks the
  correction against the book as it prepares the file. It does not post
  corrections itself; you import the file in Tally (#439, #620, #639, #660).

**Clearer answers when it cannot answer**

- A refusal now names its cause instead of a generic failure. Causes include:
  - a read that ran out of time (#493);
  - an empty book (#489);
  - a missing company identity (#514);
  - no reply from Tally (#659);
  - a foreign-currency opening balance, or a book with more than one currency
    (#606, #706, #720);
  - a voucher type that doesn't exist (#677);
  - a list Tally would not serve (#719);
  - two ledgers whose names differ only by a hidden line break (#708).
- Education-mode Tally: ComplyEaze Bridge refuses a read that Education would
  return empty without saying so (#588, #598).

**Big books**

- ComplyEaze Bridge splits a long voucher window when Tally cannot serve it in
  one request, and sizes each request by expected response size, not date span
  (#494, #506).
- The `vouchers` tool reports how long each request to Tally took (#608).

**Tax-audit engine (in development, not in the extension)**

- A Rust tax-audit engine, `bridge-tax-audit`, now covers:
  - cash payments and receipts (s.40A(3), s.269ST, s.269SS/T);
  - depreciation;
  - s.44AB applicability;
  - financial statements;
  - the trial balance;
  - stale balances;
  - ledger scrutiny;
  - the cash book;
  - TDS payees;
  - s.43B and s.43B(h);
  - book-keeping quality;
  - loans and interest;
  - partners (s.40(b), s.194T);
  - bank reconciliation;
  - a high-value register.
  (#501, #504, #508, #560, #561, #571, #592, #593, #618, #636, #710, #713)
- Its tests are mutation-tested: a check limited to the paths a change
  touches runs in the pull request, and the recorded list is re-run nightly
  (#646, #682). Coverage of every module is not claimed.
- Its accuracy is **not yet proven publicly**. Issue #738 proposes how to
  prove it.

**Security and upkeep**

- The tools workspace moved off a yanked `chacha20` and a vulnerable `rustls`,
  and CI audits its lockfile (#455, #458).
- CI now fails if an HTTP call appears outside the files allowed to make
  one. This checks where network code may live; it does not by itself prove
  that no data leaves the machine (#701).
- We deleted legacy code that nothing reached (#473, #495).

**Known limits of 0.3.0**

- The package is unsigned. It is for evaluation, not production.
- Native Windows validation with Tally and Claude Desktop is outstanding
  (the Tauri desktop app's own Windows catalog flow is #293). The posting
  approval's decision also has no behavioural test on Windows (#702). Intel
  Mac is not qualified.
- How long Claude Desktop waits on one tool call is not measured (#703).
- There is no tool to delete or undo a posted voucher, and a ledger that was
  replaced is not always noticed (#623).

## [0.2.0] - 2026-09-16

These are the detailed entries that were in this file when `mcp-preview-0.2.0`
was published. They were written before the plain-words summaries began, and
there is no summary for that build.

### Removed

- DSC (digital-signature certificate) hardware-token detection, certificate
  extraction, and their AXAL sync path have been withdrawn from Bridge's
  scope, along with the `pkcs11` and `cryptoki` dependencies that reached the
  PKCS#11 driver. `pkcs11` 0.5.0 was unsound (RUSTSEC-2022-0034) and
  unmaintained; the capability may be rebuilt properly later if needed. AXAL's
  Tally and Documents integrations are unaffected.

### Changed

- `build_import_xml` now reports `live_evidence` as an array of
  `{observation, report, voucher_types}` records rather than a single string,
  and no longer emits `live_evidence_report`. The previous shape could name
  only one source for a whole batch, so a Payment build cited a report that
  records Payment being refused. A client branching on the old string value
  needs updating; the accompanying voucher types make the provenance readable
  without one.
- Relicensed future Bridge distributions from the MIT License to the Apache
  License, Version 2.0. The previously published `v0.1.0` release remains
  available under the MIT License that accompanied that release.
- Core snapshot canaries now authorize attempts through stable observed
  sealed-profile execution evidence without claiming field support from an
  incidental first-day dataset; snapshot rows are always re-fetched after the
  durable run starts.
- A probe that no longer returns the selected company now clears and
  invalidates every company-scoped evidence, proof, mirror, diagnostic, and
  snapshot view before installing the replacement probe, so its fresh review
  remains usable without displaying stale company data.
- Snapshot lifecycle probes no longer replace interactive setup-review state;
  restart admission uses the exact sealed Core receipt, and ambiguous duplicate
  live company identities fail before any snapshot read with their concrete
  terminal proof reason preserved.
- Snapshot recovery now durably replays backward-clock abandonment evidence,
  enforces a 100,000-record aggregate hydration ceiling, and recovers an exact
  already-committed receipt from compact hash-bound proof authority without
  rehydrating canonical membership.

### Added

- Local import files may now carry Payment, Receipt and Contra vouchers as well
  as Journals, so a bank statement can be expressed in the voucher types Tally
  files it under. Each of the three is admitted only as two entries over two
  distinct ledgers carrying neither a voucher number nor a reference. The side
  that must hold money is refused unless that ledger's live group ancestry
  reaches a reserved Bank Accounts or Cash-in-Hand identity — the two where a
  captured ledger is observed sitting under a captured group. The counterparty
  side must be established as holding no money: any money group there means the
  voucher is really a Contra, and a ledger whose group ancestry cannot be
  resolved is refused as well, because neither leg is admitted on an absence of
  evidence. A build that names a
  counterparty warns that its amount lands On Account. Native posting is
  unchanged and still accepts only one unnumbered Journal.
- A local-first Tally Truth Layer with capability passports, explicit truth
  states, encrypted mirror evidence, resumable/adaptive snapshots, Proof of
  Sync and Gap Map output, and a safer operator console. The migrations are
  additive; rollback requires restoring the prior application and retaining
  the encrypted database for forward recovery rather than deleting evidence.
- Portable, bounded Tally protocol, canonicalization, transport, runtime,
  compatibility, incremental-policy, qualification, observability, and
  write-safety crates backed by a synthetic loopback protocol simulator.
- Reviewed single-use setup authority, exact selected-read qualification, and
  fail-closed compatibility manifests/runbooks. Live Education behavior and
  every write capability remain unknown or disabled until exact reviewed
  evidence exists.
- Native Windows and macOS CI coverage for formatting, tests, builds, and
  Clippy.
- Repository-local Windows and macOS application icons.
- Open-source contribution, security, review, and rectification guidance.
- Reproducible Node and Rust toolchain baselines, installer smoke builds, and
  complete lockfile-to-license-inventory checks.
- Automated legal-resource inspection for Windows MSI/NSIS installers and the
  staged and DMG-packaged macOS app bundles.

### Security

- SQLCipher/keyring-backed local Tally state, immutable proof/checkpoint
  receipts, loopback-only HTTP without a proxy, bounded incremental decoding,
  cancellation and lease enforcement, idempotent crash replay, and sealed
  no-write qualification boundaries.
- SQLCipher pool replacement connections now receive raw key bytes from
  zeroizing storage without retaining a key-derived pragma string. Proof
  contract v3 binds detailed record counts, and historical crash recovery no
  longer depends on current checkpoint ownership.
- File-backed snapshot ownership now uses per-run kernel advisory locks, so a
  crash can be reclaimed after wall-clock rollback without allowing a live
  owner to be stolen. Persisted/live company profiles correlate through an
  opaque endpoint-scoped identity key, and macOS qualification reports
  `ru_maxrss` in its native byte units.
- Losing checkpoint compare-and-swap decisions terminalize as durable failed
  proofs and close staging attempts instead of remaining falsely resumable;
  unrelated checkpoint advances do not rewrite Failed or Cancelled outcomes.
- Compatibility claims now require verified synthetic-fixture identity before
  an explicit parsed Tally application rejection can establish `Unsupported`;
  fixture, context, sentinel, parser, malformed-response, and transport
  failures remain fail-closed observations rather than incompatibility claims.
- Updated the XML parsing graph and removed unused Linux-only dialog
  dependencies from the supported Windows and macOS build graph.
- Updated the Tauri runtime to 2.11.5 and tauri-runtime-wry to 2.11.4.
- HTTPS-only AXAL endpoints with redirect blocking, bounded responses, and
  credential validation.
- Safer DSC PIN transport and PKCS#11 library discovery without exposing
  arbitrary native-library loading to the webview.
- Bounded Tally and document responses with endpoint and upload validation.

## [0.1.0] - 2026-07-12

### Added

- Initial open-source Bridge application with React, Rust, and Tauri support
  for Tally, GST, DSC, document, sync, and local database workflows.

[Unreleased]: https://github.com/ComplyEaze/bridge/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ComplyEaze/bridge/releases/tag/v0.1.0
