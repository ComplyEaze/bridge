# Results: #2011 Part 2, BRIDGE GST LEDGER LAB (November 2025), 26 Oct 2026. Part 2: no registration. GSTR-1 opens with a blank GST Registration, so L5 to L11 were skipped and no report request was sent to this book. The book is unchanged (marks 64 / 225, status rows the same)

The sheet is `lab/2011-gst-report-names` at 23696859f8aff6ca9ac1be83dff00bbfa5b6ab1d. Preflight, again for this part:
- `SHA256.txt` checked 22/22 OK on sha256 and bytes;
- `check-request-ids.py` gave `census: ok 5, skipped 12, FAIL 0` (exit 0);
- `answers.py --self-test` gave `self-test: ok (17 controls)` (exit 0).

**Release and edition (rule 1)**, from Tally's own About screen: TallyPrime **Release 7.2 (Latest)**, Edition **Silver Rental**, not Educational mode; port 9001. Step 0: one tally.exe owning port 9001; GET /status 200 (51 bytes).

Every request was sent with the sheet's rule 5 command, one at a time; `answers/part2-log.txt` holds each send and `answers/part2-notes.txt` each step, with the `answers.py` output in full. Part 2's files are the `l-` files; Part 1's `p-` files, `log.txt` and `notes.txt` are unchanged.

## Part 2: BRIDGE GST LEDGER LAB (100077)
**L1.** The company was loaded alone, with no question or prompt. The Gateway showed period 1-Apr-25 to 31-Mar-26 and current date Thursday, 27-Nov-2025.

**L2.**
- `l-m0`: one COMPANY row, GUID aae372c5-a894-491c-8843-5532e43d2d2f (= `lab/gs1-l1-answers` `answers/s10-state-marks.xml`), **ALTVCHID 64, ALTMSTID 225**, the last committed reading.
- `l-s0` and `l-s0r`: 64 rows each, byte-identical; `compare` gives `same rows`.
- This part has no `status-sales` read.

**L3. Backup** (C5), into the new empty local folder `backups\l`:
- one file, `TDBK1800_100077.001`;
- 75,532 bytes, sha256 cb25df9701fadb5116f19a0dda1fe4087c37fb68e50c891622ea1e46b7079061;
- taken 23:48 IST.

It was kept and not restored (nothing was changed). `l-m0b`, read after the backup: 64 / 225.

**L4, the registration check.** The path was Display More Reports > Statutory Reports > GST Reports, by arrow keys and Enter. The menu is the same as Part 1's: Track GST Return Activities, GSTR-1, GSTR-3B, Annual Computation, e-Way Bill, GSTR-1 / 2A / 2B Reconciliation, Challan Reconciliation, Invoice Management System (IMS), GST Utilities, Other Reports. There is no separate HSN/SAC summary entry.

GSTR-1 opened at once, with no question to update GST status (**N1 = none**), at its default period 1-Oct-25 to 31-Oct-25. Its header:
- GST Registration: **blank**;
- Status: Not Filed; ARN and ARN Date blank;
- "Last online GST activity: No Activity Found";
- the voucher counts blank.

The period was not moved to November: the registration in the header is blank whatever the period. Escape.

**Recorded: "Part 2: no registration".** By the sheet, L5 to L11 were skipped: no report request (unknown name, GSTR-1, GSTR1, GSTR-3B, GSTR3B, HSN/SAC Summary) was sent to this book, and GSTR-3B was not opened on the screen.

**Closing state reads** (status and marks only, not report requests): `l-s1` against `l-s0` gives `same rows` (64/64, CMPINFO equal), and `l-m1` is byte-identical to `l-m0` (64 / 225).

**L12.** Shut by the Company menu (Alt+K) > Shut > the one row; Select Company with no company loaded. One tally.exe owning port 9001; GET /status 200. Tally's date was not changed. No request was sent after the shut.

## Deviations (keying slips, no request involved)
1. Before L1, at Select Company: a click on Tally's top bar and then typed filter letters opened Manage License > License Information, a read-only screen. One Escape returned to Select Company. Nothing was loaded or changed.
2. At the start of L4, from the Gateway: letters typed as a menu filter were taken as menu keys. They opened Display More Reports > Inventory Books > Stock Group Summary's "Select Stock Group" picker with a box "Error / Oops! / Spelling Error" (the leftover letters matched no group). No key was pressed and no request was sent while the box was up. It was reported, and cleared on the requester's word: one Escape (box and field cleared), a second Escape (Gateway, menu at Inventory Books). Then `l-m0c` read 64 / 225, byte-identical to `l-m0`.

From then on only arrow keys, Enter and Escape were used in Tally's menus.

## Responses

| name | bytes | sha256 | class | rows |
|---|---|---|---|---|
| l-m0, l-m0b, l-m0c, l-m1 | 3,404 each | 132baf47f8c849909e4b0f921c201f4b658dbf000e2da51d9b1fe9aaecc78529 (all four) | answered | 1 COMPANY |
| l-s0, l-s0r, l-s1 | 213,144 each | 3801d22853b2c62846489e31156662076bfa10745f0471a986559a06b7fd489a (all three) | answered | 64 VOUCHER |

Every send: curl exit 0, HTTP 200. No LINEERROR.

## Findings
- **Verified:** on BRIDGE GST LEDGER LAB, which was created with GST on but no registration, Tally 7.2's GSTR-1 opens without a question and shows a blank GST Registration. By L4 the gateway report names were not tried on this book.
- **Verified:** loading, the backup and the GSTR-1 screen changed no voucher row and no mark (64 / 225 throughout).
- **Not established:** how a report-name request behaves on a book with no GST registration. The sheet withholds that request, and it was not sent.

Screenshots stay on the lab PC, listed by name and sha256 in `SCREENSHOTS.txt`. The backup is kept on the lab PC and listed above by size and sha256.
