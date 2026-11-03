# #2016 live row: `vouchers` with `stock_lines` on the class shape. Not for merge.

First run of this code against a live Tally. The #2230 captures read one voucher each on the bare inventory FETCH, with no class COMPUTEs, so the COMPUTEs with that FETCH have never been sent and no answer holds a mix of kinds. This row sends them (P1, P7).

Code admits the trigger:
- The argument: src-tauri/src/agent_voucher_parse.rs:351 (`StockLines::from_args`), read at src-tauri/src/agent_vouchers.rs:601.
- The request: src-tauri/src/agent_vouchers.rs:721 selects `VoucherReadShape::ClassInventoryWildcard`, rendered at src-tauri/src/agent_voucher_window.rs:187; FETCH at src-tauri/src/agent_read_profiles.rs:63, renderer at :197 with the class COMPUTEs.
- Which kinds show lines: src-tauri/src/agent_voucher_parse.rs:366 (`covers`, reserved name only); which must hold one: :372 (`needs_a_line`, all but Credit Note). `kinds_not_read`: src-tauri/src/agent_vouchers.rs:350.
- The tool schema: src-tauri/src/agent_catalog.rs:571 and :830.

Head and build: sent with this row (branch `fix/2016-stock-lines`). Record bridge_mcp.exe's sha256 before and after.
Book: one synthetic book with two godowns and batches, TallyPrime 7.2 Silver, licensed, loaded alone. The #2230 book serves if it has the two godowns. Reads only through the tool, apart from the seeding below.

Set up, seeded by direct XML, all on ONE day (set-up only, never evidence of its own shape):
- a Stock Journal moving one item between the godowns, batch named
- a Delivery Note, a Receipt Note, a Credit Note, each with one item line
- a voucher type of your own made under Delivery Note, and one voucher of it with one item line
- a voucher type of your own made under Credit Note, and one voucher of it with one item line
- a Credit Note with no item, accounts only
- a second Delivery Note with one item line, then cancelled in Tally (set-up by keying is fine)
- a third Delivery Note with one item line, marked optional (`ISOPTIONAL` Yes)
- a Sales, a Purchase and a Debit Note, each with one item line

1. Marks. `list_companies`. Record each voucher's ALTERID from the plain read in step 2.
2. `vouchers` for that day, no `stock_lines`.
   EXPECT all twelve rows, no `stock_lines`, `stock_lines_not_read` or `kinds_not_read` key anywhere.
3. `vouchers` for that day, `stock_lines: true`. Keep the request as sent and the raw answer.
   EXPECT the FETCH list is exactly `DATE,VOUCHERNUMBER,VOUCHERTYPENAME,PARTYLEDGERNAME,NARRATION,GUID,ALTERID,MASTERID,ISCANCELLED,ISOPTIONAL,ISPOSTDATED,ALLINVENTORYENTRIES.*,REFERENCE,ISINVOICE,PARTYGSTIN,REFERENCEDATE`, with the voucher type COMPUTEs beside it.
   EXPECT one window, twelve rows, each row's date, number, type and GUID equal to step 2's. If the optional voucher is missing from either read, say so; both reads must agree.
   EXPECT Stock Journal: one `in` line and one `out` line, item, both quantities, godown and batch under `batches`, as on Tally's screen. An `empty` rate or amount on the in side is fine; say which.
   EXPECT Delivery Note `out`, Receipt Note `in`, Credit Note `in`, each with a line.
   EXPECT the Credit Note with no item: `stock_lines` is `[]`, no `stock_lines_unreadable`.
   EXPECT the Sales, the Purchase, the Debit Note and both vouchers of your own types: `stock_lines_not_read`, no lines.
   EXPECT `kinds_not_read` lists those five by voucher type, one voucher each.
   RECORD `voucher_type_reserved_name` and `voucher_class` as returned on each row, all twelve, in the sheet. Section 8.2e committed the row-level answer for Purchase only, and the code keys on these words.
   RECORD what the cancelled Delivery Note reads: its lines, `stock_lines_unreadable`, or anything else. No EXPECT.
   RECORD the same for the optional Delivery Note. No EXPECT.
   EXPECT `stock_summary_tie` is `not_compared`, with `stock_summary_tie_reason` beside it.
4. Repeat step 3 once. EXPECT byte-equal apart from any read time.
5. `vouchers` for that day, `stock_lines: true` with `ledger` set to any ledger name.
   EXPECT refused as `voucher_stock_lines_excludes_ledger`, and no request in `egress_log` for it.
6. `vouchers` for that day, `stock_lines: true` with `voucher_type: "Delivery Note"`. The stock shape is chosen ahead of the type filter (src-tauri/src/agent_vouchers.rs:721).
   EXPECT only Delivery Note rows, each with lines as in step 3 (the cancelled and optional ones as step 3 recorded them).
   RECORD whether the voucher of your own type under Delivery Note is included, and how it reads.
7. Paging. `vouchers` for that day, `stock_lines: true`, `limit: 4`. Then the same with `offset: 4`, `limit: 4` and the first page's `snapshot_id`.
   EXPECT the second page's `snapshot.reused` is true and `egress_log` shows no window read for it, only the identity and marks reads.
   EXPECT `kinds_not_read`, `stock_summary_tie` and `stock_summary_tie_reason` equal on both pages and equal to step 3's.
8. Record the request and answer sizes of step 3 as encoded on the wire, and bytes per voucher and per stock line, one figure for each of the twelve if the raw answer splits cleanly, else the total and the count of lines. These go in the limits table and size the 384 KiB default.
9. Stock Summary and a note. Take an item with no movement in the book yet. `stock_summary` for that day, keep it. Seed one Delivery Note on that item, on that day. `stock_summary` for that day again, keep it. No EXPECT: this settles whether a note moves Stock Summary.
10. Marks. EXPECT unchanged from step 1, apart from the voucher step 9 seeds.

A Stock Journal or note reading `stock_lines_not_read` is a finding, not a failure of Tally: keep the raw answer and say which type and what its reserved name answered, since the code keys on that word. If `$ReservedName` does not answer those words every row reads `stock_lines_not_read`: keep the raw answer and stop there. The same finding if a voucher of your own type shows lines or answers a non-empty reserved name, or if a Sales, Purchase or Debit Note answer holds a different shape from the notes. Fails if a call errors, a figure differs from the screen, a line's direction contradicts the screen, step 2 carries a stock key, or any mark moves other than by step 9's seed.

Keep: tool answers, the request as sent, the raw answer of step 3 unchanged (it becomes a committed fixture with a real record, so synthetic names only), both `stock_summary` answers, marks, sizes. Answers branch: lab/2016-stock-lines-answers.
