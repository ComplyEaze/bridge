# Results: #1039 R2, the content review dialog on R1's refused batch, 29 Oct 2026, through #2082's build at 2cf7c3e5 (exe a5d034ca…). Every EXPECT line held. The window's text equals the sheet's block, declined and then approved. The review is one five-key journal line, last. `verify_import` and `local_data_report` show the review while the batch stays unverified and open. A third call refused `ack_already_recorded` with no window. The book did not move (210 / 55; the 50 vouchers unchanged). Not shown by the tool: `egress_log` could not page back over this row, so the row's receipts were read from the data folder's log file

The sheet is `lab/1039-review-dialog-row` at 25c80910a3d7976fed6262df35314f9c79ccb790. `SHA256.txt` matched 5 of 5.

The input is R1's kept batch, data folder and book:
- B1 = bridge-a4eb06f8-cf9a-43a6-9255-3920518cf325;
- BRIDGE CONCUR LAB, 100091, 88239cf8-ef69-4cea-8dff-4d1fd91453ee.

The run order named this row and head. `ORDER.md` carries no row note for it.

- **Build:** #2082 at 2cf7c3e5b68b0327308c0853f0ad0c35d9c2c62d, run 40667848887, artifact 12807447884.
  - mcpb e966559a…, `bridge_mcp.exe` a5d034ca888addaf1eddf5da4b75f95b6dbbb8af447c5c5a84b65a59951c23b6, both equal to the smoke file and the same at the end.
  - Server 0.6.1.
  - One launch, with R1's data folder and `BRIDGE_AGENT_ENABLE_WRITES` present.
- **Tally:** 7.2, Silver. THE COMPANY was loaded alone; the current date was 28-Apr-2025 at the start and at the end.
- **Data folder copy (step 0.2):** `data-before`, 14 files. Both listings are equal (in `checks/`). J0 3c7a7918…, N0 7.

| step | EXPECT | observed |
|---|---|---|
| 0.3 | one company; M0 210, V0 55 | as expected |
| 0.4 | 50 rows R-1-01 to R-1-50 | 50 Receipts, R-1-01 to R-1-50 |
| 1.1 | refused `span_step_not_created`; `verification_incomplete`; posted_verified 0; matching 50; no `content_review` | as expected; batch_sha256 a2c1788e… (the answer is flagged an error, `import_reconciliation_required`, as before the review) |
| 1.2 | not_settled 1, binding_refused 1, reviewed 0 | as expected |
| 2.2 | the title and the block | title exact; the message equals the block line for line, not cut, not scrolled |
| 2.4 | `ack_review_declined`; no `content_review` line; later lines B1 status records | as expected (lines 8 and 9: `verification_status`, B1) |
| 3.2 to 3.3 | the same text; Yes; `content_reviewed`, `refused`, `verification_incomplete`, the next_step | as expected, the next_step word for word |
| 3.4 | one `content_review` line, last; five keys; batch_sha256 = step 1.1's | J1 f97b045d…, 11 lines; the last line `{"record_type":"content_review","batch_id":"bridge-a4eb06f8-cf9a-43a6-9255-3920518cf325","batch_sha256":"a2c1788eedae2fad79bd3ff9ec583ad21835e9fc385c80783a9ab610735e590f","status":"content_reviewed","reviewed_binding":"refused"}` |
| 4.1 | `content_review` recorded / refused / the next_step; the rest as at 1.1 | as expected (binding, status and counts equal to 1.1's) |
| 4.2 | not_settled 1, binding_refused 0, reviewed 1, no_response 0, not_verified 0 | as expected |
| 4.3 | `ack_already_recorded`, no window | as expected (0.1 s, no window) |
| 4.4 | 210 / 55; the same 50 rows, AlterIDs unchanged | as expected (narration, AlterID, GUID and master ID equal for all 50) |
| 4.5 | no import sent | none: every request was a status read or a POST of at most 2,524 B; no dispatch line in the journal |
| 4.6 | exe hash | a5d034ca… |

**The windows.** Both opened about 4 s into the call. Each is a standard dialog of a `bridge_mcp.exe` process with Yes, No and Cancel. Each was captured whole before the click, and its text was read from the window itself; both are in `checks/`. Each was answered by a mouse click at the button's centre; no key was typed.

**`egress_log` (step 4.5).** The tool could not show the whole row:
- with limit 200 and limit 40 it answered `agent_response_too_large`, because R1's receipts share the folder and the tool takes no offset;
- with limit 12 it gave the last 12 records.

The row's receipts (32 lines) were therefore read from `agent-egress.jsonl` in the data folder, a local file read. R1's 28 lines come first and are unchanged.

**Kept on the lab PC:**
- the data folder as this row left it (holding the review; the journal is now 12 lines, step 4.1 having added a status line after J1);
- `data-before`;
- THE COMPANY;
- R1's BK-R.

**Confidence:** one sitting, PARTIAL until repeated.

## Files
- `answers/`: every tool answer whole, 000 to 016, with UTC times.
- `client-log.txt`.
- `notes.txt` (UTC).
- `checks/`:
  - the two window texts as read (title and controls);
  - the two folder listings of step 0.2;
  - the row's egress receipts (`egress-row-lines.jsonl`);
  - step 0 and close checks (tasklist, the port 9001 lines of netstat, the listener, `/status`).
- `substitutions.txt`: the lab folders in pushed copies.
- `SCREENSHOTS.txt`: the captures by sha256. They stay on the lab PC.
