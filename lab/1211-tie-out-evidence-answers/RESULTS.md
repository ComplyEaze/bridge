# #1211 `statement_tie_out` on a live bank ledger: results

Sheet `lab/1211-tie-out-evidence` at `cdeaea0c` (LAB-COMMON v3). TallyPrime **7.2** Silver (licensed, `education_mode` false), synthetic book BRIDGE TIE OUT LAB 3 (GUID `9c97970c-…`), the published `mcp-v0.7.0` Windows package (`.mcpb` sha256 `291a4ed9…`, `bridge_mcp.exe` sha256 `48d2f743…`, unchanged at the start and end of both launches). LAUNCH A (reads, `BRIDGE_AGENT_ENABLE_IMPORT`) for step 0, R1 and R2; LAUNCH B (plus `BRIDGE_AGENT_ENABLE_WRITES`) for R3. Every tie-out figure below is book less statement.

The keying, the `BRIDGE Bank B` ledger and the answer to the post window were done by the lab on the owner's word of 28 Oct (synthetic book); both restores of BK1 on the run lead's recorded word.

| Row | Result | Figures |
|---|---|---|
| Step 0 | PASS | `statement_tie_out` listed, `post_import` not; one company, `books_from` 20260401; marks 210 / 8; Union Bank 07-01..09-30 opening -10000.00, closing -11800; Tally 7.2 Silver. |
| R1.1 (08-01..08-15) | PASS | `opening_gap`, `closing_gap`, `change_in_window`: `not_established`, reason `window_not_whole_calendar_months`, no amount; `evidence.state` `partial` with that `reason_code`; request trail **4** (two 2,524 B company posts, two status). |
| R1.2 (07-01..08-15) | PASS | the same three `not_established`, no amount, `partial`; trail 4. |
| R1.3 (July, control) | PASS | `tied` 0.00 ×3; trail 70. |
| R2.3 (July, August with `BRIDGE Bank B` and W1 in the book) | PASS | `tied` 0.00 ×3 each. |
| R2.4 (Tally's own figures) | PASS | `trial_balance` Union Bank: -10000.00 to 30 June; -10249.50 to 31 July = minus the July proposals' `window.tie_out.closing_balance` 10249.50; -11800.00 to 31 August. `BRIDGE Bank B` -700.00 to 31 July. Ledger display 1-7 to 31-8: Opening 10,000.00 Dr, Closing 11,800.00 Dr. |
| R2.6 (after W2, off the statement) | PASS | August: `opening_gap` `tied` 0.00; `closing_gap` and `change_in_window` `differs` **-1000.00**; `trial_balance` Union Bank -10800.00 to 31 August; ledger display 1-8 to 31-8 Closing 10,800.00 Dr. |
| R2.7 (to 08-15, no EXPECT) | note | Answered with no refusal: Union Bank -12249.50. |
| R2 restore | PASS | marks 210 / 8; `list_companies` and `ledger_movement` equal to step 0's; no `BRIDGE Bank B`. |
| R3.1-R3.2 | PASS | LAUNCH B lists `post_import`; Payment and Receipt series numbering `observed` / `automatic` / `auto_retain`. |
| R3.4 (STATEMENT B parse) | PASS | 4 rows, 4 in window, 4 vouchers, 0 skipped, 0 cash questions, running balance every row true, closing 13950.00; proposals `whole_statement` true, 2026-09-03..2026-09-28, opening 12300.00, closing 13950.00, `established`. |
| R3.5 | PASS | Ledger display 3-9 to 28-9 Opening 12,300.00 Dr, Closing 12,300.00 Dr; `trial_balance` to 09-02 -12300.00. |
| R3.6 `before_build` | PASS | window 09-03..09-28, `rows_without_voucher` 0, `tied` 0.00 ×3 (H1 counted in the opening, H2 not counted at the close). |
| R3.7-R3.11 post by ComplyEaze Bridge | PASS | `build_import_xml` 4 vouchers; the window showed the four rows and the batch, answered Yes; one `post_import` call: `posted_verified`, created 4; `verify_import` `posted_verified`; V +4. |
| R3.12 `after_post` | PASS | `tied` 0.00 ×3. |
| R3.13 `before_build` again | PASS | `opening_gap` `tied` 0.00; `closing_gap` and `change_in_window` `not_established`, reason `file_already_partly_posted`. |
| R3.14 | PASS | Ledger display Closing 13,950.00 Dr; `trial_balance` to 09-28 -13950.00; `vouchers` six: H1, the four of the batch dated and valued as STATEMENT B's rows, H2. |
| R3.16 (after twin T1) | PASS | `opening_gap` `tied` 0.00; `closing_gap` and `change_in_window` `differs` **650.00**; ledger display Closing 14,600.00 Dr. |
| R3 restore, close | PASS | marks 210 / 8; `list_companies` and `ledger_movement` equal to step 0's; the launch file back to its pre-row hash; no company loaded. |

**Release this item ran on:** TallyPrime 7.2 (every earlier tie-out run was on 7.1).

**What the rows show.** A window ending mid-month gives a typed `not_established` answer with no amount, from the file after the company check alone (4 requests, no ledger read). A second bank ledger leaves the tie-out tied, and Tally's own trial balance and ledger display agree with the statement's figures. An entry off the statement shows as its gap (-1000.00, 650.00). A statement starting and ending mid-month is read at the 3rd and the 29th: the edge vouchers H1 (the 2nd) and H2 (the 29th), and after the post a voucher on each edge day, all landed on the right side; the post by ComplyEaze Bridge's own `post_import` tied, and a re-run of `before_build` after it reports `file_already_partly_posted` rather than counting the batch twice.

## Recorded alongside

- R2.1: Enter on the empty GSTIN/UIN of the new ledger raised Tally's advisory "GSTIN/UIN not specified" warning. The lab stopped (C7); on the run lead's recorded word one Enter closed it (as the lab's 26 Oct #1237 record found), the field stayed empty, and M rose by exactly one.
- Ledger displays were set with the report's own F2 Period (the side bar), so the Gateway's period never changed; the sheet's "Alt+F2" was read as the report's period.
- Keying a voucher with F2 moved Tally's current date; each restore's reload set it back to 10-Oct-2026 (step 0's), as the close shows.
- Tally opened Bank Allocations for every bank line keyed by hand; each was passed with its defaults.
- H2: Tally's autofill pre-filled the second line (To Union Bank 200.00), which was the line the sheet asks for.
- R3.6, R3.12, R3.13 and R3.16 tie out the step 4 parse itself (`tie2.py`); the R1/R2 tie-outs each follow their own parse (`tie.py`).
- The first box watcher, started from Bash, wrote no log (the folder argument was mangled) and exited with no box and no click before any post; a dry run from PowerShell then worked and was used.

## Not covered

As the sheet lists: a book of 1,000 or more vouchers; Educational mode; a voucher posted inside the window during the reads; a statement ending on 31 March; banks other than Union Bank's layout; Gold; TallyPrime 7.1.

## Evidence

`notes.txt` (one line per step, UTC); `bridge/` (every ComplyEaze Bridge answer whole, `company_number` redacted); `evidence/` (marks, tie-out and trial-balance logs, the egress log the data folder gained, C2 at step 0 and the close, BK1 record, tools log, the post arguments); `restore-plan.txt`; `scripts/`; `SCREENSHOTS.txt` and `kept-local.txt` (screenshots, BK1, STATEMENT A, the data folder's other files, the box log and the C6 listings, by size and sha256; never pushed); `substitutions.txt`; `SHA256.txt`.
