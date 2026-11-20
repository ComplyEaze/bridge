# Row 2440-CP, sitting 2: answers

Sheet `ROW.md` of row 2440-CP (lab/2440-composed-sheet at `3d8275c06a1b983c7c3879d6b9f7d5c83455a116`, folder lab/2440-composed/), steps 3 to 12, run 2026-11-20 on TallyPrime Silver Release 7.2 (Licensed) against the synthetic book BRIDGE EINVOICE LAB (GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`), from the backup BK1 of sitting 1 (2026-11-19) as restored at sitting 1's step 12, and restored again at this sitting's step 12. Steps 1 and 2 are sitting 1's (lab/2440-composed-answers); this sitting's start read t1 stands for s1 in its before-write checks and is equal to sitting 1's s1.

**First line:** every EXPECT of steps 3 to 12 met. Two deviations, each on the run lead's word with the owner's word in chat that he was at the lab screen: invoice A and invoice B were each called once more after a first call that ended `import_approval_timed_out` with nothing posted (step 8.3 says A is not called again without that word); each retry posted `posted_verified` with `created` 1. One sheet correction: B's window shows the two e-invoice lines in a different place from the one step 9.2's wording gives (below); both values are as sent. A's windows opened minimised, as in sitting 1 (product issue #2569, below).

## Build

The build of sitting 1, unchanged.

| item | read at the lab |
|---|---|
| build line | RC commit `e0467fbc344969ba8d8c8f119e7726e3bdca6cca`, artifact `unsigned-bundle-smoke-windows-latest`, as in sitting 1's RESULTS |
| `bridge_mcp.exe` run | 35,869,184 bytes, sha256 `c437138a08dc10a8609e42d28104920401b4615daa69f3cdf404f6e37d6323dd`, equal to the build line at step 6.1; reports bridge-tally 0.7.0 |

## Launch

One launch from step 6 to step 10, as the sheet says: 07:43:42Z, server pid 7068, `BRIDGE_AGENT_DATA_DIR` 27 bytes, `<lab folder>\data\`, last 4 hex `6174615c`, the folder made new and empty at step 6.1 (sitting 1's data folder moved aside first, not removed); `BRIDGE_AGENT_ENABLE_WRITES` present (launch file sha256 `379fea09fa957d55608c7e0a8a7edeaa5c4bc088e03b775fbf4320af997f606d`, equal to sitting 1's launch 2 file); stderr empty; closed by the client's stop at 08:43:50Z, exit code 0. The WRITES line was then removed (back to `98411a75ed4fea8739daf1b33ee98409d24853b80dfee44ed21923c73ee9b8cf`); `run/launch-env-hashes.txt` holds each edit.

## Steps

| step | result | grade |
|---|---|---|
| start, t1 | the book loaded alone (period 1-Apr-25 to 31-Mar-26, current date Thursday 3-Apr-2025, no dialog); one COMPANY row, the book's GUID, ALTVCHID 3, ALTMSTID 215, `--same-row` against sitting 1's s1 equal; no customer, CGST or SGST ledger; `BRIDGE IRN Sales` IGST 18, HSN 998313 | at BK1 |
| 3 customer, seeded | before-write check against t1 ok; `I-seed.xml` CREATED 1, every other counter 0, no LINEERROR, no dialog; `BRIDGE CP Customer MH` under Sundry Debtors, not bill-wise, Regular, Maharashtra, from 20250401; ALTVCHID 3, ALTMSTID 216 | PASS |
| 4 duty ledgers, keyed | keyed with Tax type CGST and SGST/UTGST, as the run lead's word for this sitting says; `BRIDGE CP CGST` and `BRIDGE CP SGST` under Duties & Taxes, `TAXTYPE` GST, `GSTDUTYHEAD` CGST and SGST/UTGST, rate 9, rounding Not Applicable, limit 0; ALTVCHID 3, ALTMSTID 218; every field of each screen before its save in `evidence/s4-cgst-screen.txt` and `evidence/s4-sgst-screen.txt` | PASS, with the sheet's Central Tax / State Tax wording read as CGST and SGST/UTGST, as in sitting 1 |
| 5 Sales type, keyed | `BRIDGE CP Sales`, Sales, Manual, prevent duplicates Yes; Ctrl+A saved with no prompt; two Ctrl+Q to the Gateway; ALTVCHID 3, ALTMSTID 220 (it rose by two for one save, as in sitting 1); every field in `evidence/s5-screen.txt` | PASS |
| 6.1 step 0 | step 0 ok; the book loaded alone; one tally.exe; the exe hash equal to the build line; WRITES present; the data folder new and empty; the installed desktop app (bridge.exe pid 16736, left from an earlier row, its endpoint port 9002) still running | PASS |
| 6.2 `tools/list` | 37 tools; `build_import_xml`'s `invoice` holds `e_invoice` requiring `irn`, `ack_no`, `ack_date`; `post_import`, `verify_import`; `vouchers` with `include_e_invoice` | PASS |
| 6.3 `list_companies` | BRIDGE EINVOICE LAB alone, `verified_tuple`, GUID equal to `count8.py`'s BOOK | PASS |
| 6.4 MARKS | M6 220, V6 3, `reused` false, equal to s5r | PASS |
| 6.5 `voucher_types` | `BRIDGE CP Sales`: `series_numbering` method manual, duplicates prevented, observed; `numbering_method` `{"unrecognised": "None"}` (recorded, as in sitting 1) | PASS |
| 7 build of A | 07:44:14Z: `voucher_count` 1, `batch_id` `bridge-47f1e200-4148-426f-801e-dd6fdeb47fa8`, file sha256 `e0b8224925b3a466662e61ca886cc0e171fdf6597f416d6c56645766fd2fab9d`, no HSN warning, no refusal | PASS |
| 8.1 V8a | 3, equal to V6 | PASS |
| 8.2 post of A, first call | `pending` at 07:44:49Z and 07:45:45Z; `import_approval_timed_out` at 07:46:41Z, `attempt_recorded` false (answers 006 to 008); no Yes given; the window's text has no line beginning `E-invoice` (`evidence/s8-window-a.txt`) | window EXPECT met; not approved in time (window opened minimised, below) |
| 8.3 | MARKS once at 07:47:29Z: M 220, V 3, nothing posted; held, and asked the run lead | as the sheet says |
| 8 post of A, retry | a deviation from 8.3, on the run lead's word (one more call, same launch, batch and arguments, only with the owner at the screen), the owner's word in chat; MARKS before at 08:31:28Z M 220, V 3; `pending` at 08:31:37Z; `approved` at 08:32:33Z, not dispatched (the owner answered Yes by mouse); the call at 08:32:58Z that sent: `dispatch.state` `posted_verified`, `counters.created` 1, every other 0, `CP/25-26/0101` alter_id 4 (answers 010 to 013); the window's text equal to the first call's apart from the read time (`evidence/s8-window-a-retry.txt`) | PASS |
| 8.4 capture | Tally captured before any other call: the Gateway drawn inactive, no dialog, no half-entered screen | PASS |
| 8.5 `verify_import` A | 08:34:01Z: `posted_verified`, confirmed 1 (answer 014) | PASS |
| 9.1 build of B | 08:34:22Z: `voucher_count` 1, `batch_id` `bridge-4ff4223c-66ad-4851-a9b7-ef2908c0b716`, file sha256 `c633c1e7c783eaf04a3434159c418091cb15ea7185b9237d99d0000eda5305f5`, no HSN warning, no refusal | PASS |
| 9.2 V9a | 4, V8a + 1 | PASS |
| 9.2 post of B, first call | `pending` at 08:34:53Z and 08:35:48Z; `import_approval_timed_out` at 08:36:44Z, `attempt_recorded` false (answers 017 to 019); no Yes given; MARKS once after at 08:37:26Z: M 220, V 4, B not posted; held, and asked the run lead | not approved in time (below) |
| 9.2 B's window | read at 08:35:20Z, word for word in `evidence/s9-window-b.txt`: `E-invoice IRN: ` and the IRN sent, whole, on its own line after the place-of-supply line and before the bill allocation; `E-invoice ack: 445577 of 2025-04-03` on the bill allocation's own line, after `Bill allocation: none (customer not bill-wise)` and two spaces | both values as sent; SHEET CORRECTION on their place (below) |
| 9 post of B, retry | a deviation, on the run lead's word (one more call; a second timeout ends B), the owner's word in chat that he was at the screen and watching; MARKS before at 08:41:47Z M 220, V 4; the call at 08:41:55Z (45.819 s) answered once, with no pending state: `dispatch.state` `posted_verified`, `counters.created` 1, every other 0, `CP/25-26/0102` alter_id 5, no `invoice_fields`, `posted_divergent` 0 (answer 022): the owner answered Yes within the call's first wait | PASS |
| 9.3 | `posted_verified`, `created` 1 | PASS |
| 9.4 capture, `verify_import` B | Tally captured before any other call: the Gateway, no dialog; `verify_import` B at 08:43:09Z: `posted_verified`, confirmed 1 (answer 023) | PASS |
| 10 read-back | MARKS at 08:43:29Z: M 220, V 5 (answer 024); `vouchers` 2025-04-03 with `include_e_invoice` (answer 025): the book's three vouchers as last read, and `CP/25-26/0101` and `CP/25-26/0102` on `BRIDGE CP Sales`, each with the four entries sent (customer Dr 118.00; Sales Cr 100.00; CGST and SGST Cr 9.00 each); 0101 `irn`, `ack_no`, `ack_date` each absent; 0102 `irn` the IRN sent, `ack_no` 445577, `ack_date` 20250403; the exe closed, exit 0 | PASS |
| 11 what Tally stored | 0101: one VOUCHER row, `IRN` empty, `IRNACKNO` and `IRNACKDATE` empty; 0102: one VOUCHER row, `IRN` 64 characters, equal to the IRN sent, `IRNACKNO` 445577, `IRNACKDATE` 20250403; GST header fields equal on both; M ALTVCHID 5, ALTMSTID 220; the `--diff` list whole in `evidence/s11-count8.txt` (ALTERID, GUID, IRN, IRNACKDATE, IRNACKNO, MASTERID, REFERENCE, REMOTEGUID, VOUCHERKEY, VOUCHERNUMBER, VOUCHERRETAINKEY; the last from 1 to 5, recorded, not judged) | PASS |
| 12 restore | BK1 rechecked before the shut (one file, 32,101 bytes, sha256 `50b036450ee36181269313ad735f3ab5704ce4f363ad8233ee4e5de85d42b692`, as recorded at sitting 1's step 2); restored (C6) on the run lead's word for this sitting; `--same-row` against sitting 1's s1 equal and against t1 equal (ALTVCHID 3, ALTMSTID 215); K-customer 0 LEDGER rows; F-0101: 0 VOUCHER rows | PASS |
| C10 | the book shut; Select Company, none loaded; one tally.exe on 9001; `/status` 200; no `bridge_mcp.exe` running | PASS |

## Sheet correction: where B's window shows the e-invoice lines

Step 9.2 says the window shows the two lines `E-invoice IRN: ` and `E-invoice ack: ` after the bill allocation. The build puts `E-invoice IRN: <IRN>` on its own line before the bill allocation, and `E-invoice ack: <number> of <date>` on the bill allocation's line. The run lead's word: this is the design of PR 2440, which a test on that PR's branch pins; it is a correction of the sheet's wording, not a product finding. Both values in the window are as sent. A sheet that runs this step again should give the lines in that place.

## For #2569: the approval window opened minimised

This sitting's sighting, for the issue's write-up.

- **Launch method** (`evidence/launch-method.txt`): the exe was started by the lab's MCP client (`scripts/client.py`) with `subprocess.Popen`, pipes for stdin and stdout, stderr to a file, no creation flags and no startup info; that client ran under a background shell command of the desktop client's command-line process. No process in the chain from the desktop client down to the exe has a main window. No console window for the shell, the client or the exe showed on the whole-screen screenshots. The show state the exe's startup info carried was not read.
- **Each window's process**: `bridge_mcp.exe --confirm-journal 7068`, the child of the server (pids 4824 and 14012 for A, 17004 and 4112 for B).
- **A, first call**: called 07:44:49Z; the helper seen at about 07:45:05Z; a screenshot at about 07:45:20Z showed the window as a minimised title bar (`ComplyEaze B...`) at the screen's bottom-left, above the taskbar; the owner had been told before the call where it opens and was told again; it timed out at 07:46:41Z unanswered.
- **A, retry**: called 08:31:37Z; the helper seen at 08:31:55Z, about 18 seconds after the call, again a minimised bar at the bottom-left; the owner, at the screen and told, answered Yes; `approved` at 08:32:33Z.
- **B, first call**: called 08:34:53Z; the helper seen at 08:35:12Z, about 19 seconds after the call; the owner told it was open (how the window showed on the screen is not in the notes); it timed out at 08:36:44Z unanswered.
- **B, retry**: called at 08:41:55.308Z (`run/s9-retry-start.txt`); the helper seen at 08:42:13Z, about 18 seconds after the call. A screenshot at about 08:42:20Z to 08:42:29Z showed a dark minimised bar at the screen's bottom-left above the taskbar and, over Tally at the top-left, the console window of the installed desktop app (bridge.exe pid 16736, class `ConsoleWindowClass`, title `Select `), not the approval window. A list of visible titled windows at 08:42:31Z showed that console then minimised and no `ComplyEaze Bridge - post this voucher?` window. The helper had ended by about 08:42:40Z; the call answered `posted_verified`. The bar's title was not read; the bottom-left above the taskbar is where A's windows opened minimised. The approval window was not seen in front, nor behind Tally. Its text was not read (it closed before the read).
- In every call the lab clicked nothing in the window and pressed no key while it was open. A user who is not told where the window opens can miss it inside its time: both first calls timed out.

## Answers

`run/answers/`, one file per call, sequence 000 to 025: 000 `tools/list` (kept on the lab PC, see below), 001 `list_companies`, 002 MARKS, 003 `voucher_types`; 004 build A; 005 V8a; 006 to 008 post A; 009 MARKS after; 010 MARKS before the retry; 011 to 013 post A, retry; 014 `verify_import` A; 015 build B; 016 V9a; 017 to 019 post B; 020 MARKS after; 021 MARKS before the retry; 022 post B, retry; 023 `verify_import` B; 024 MARKS; 025 `vouchers`. `run/inbox/` holds each request as sent; `run/args-s7-a.json` and `run/args-s9-b.json` the build arguments.

## Notes on the record

- Each tool answer in `run/answers/` is whole but for `company_number`, replaced by a JSON parser; Tally's raw answers in `tally/` have the company number as X at equal length; `tally/sitting1-a-s1-M-marks.xml` and `tally/sitting1-a-s12-M-marks.xml` are sitting 1's reads used for the same-row checks. Each original is listed by size and sha256 in `kept-local.txt`, with the files that stay on the lab PC (the tool list of step 6.2, screens, process, port and folder listings, the backup, the build, the server's data folder, its stderr file).
- At the load, the first typing in Select Company's filter went in through the clipboard and pasted other text; clearing it moved the list up to the drive level; the lab went back down to the data folder folder by folder and typed in parts. No company was touched and nothing was loaded until the filter showed BRIDGE EINVOICE LAB alone.
- The data folder after step 10: `evidence/s10-data-listing.txt` (32 files).
- After the restore the book's data folder lacks side files it held before, as after this lab's earlier restores.
- The lab's own notes, with every step's times: `evidence/notes.txt`.
