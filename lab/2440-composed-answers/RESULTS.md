# Row 2440-CP: answers

Sheet `ROW.md` of row 2440-CP, run 2026-11-19 on TallyPrime Silver Release 7.2 (tally.exe ProductVersion 1.1.7.2, Licensed) against the synthetic book BRIDGE EINVOICE LAB (GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`), behind the backup BK1 taken at step 2 and restored at step 12.

**First line:** Part A met every EXPECT (with the sheet's Central Tax / State Tax wording recorded as CGST and SGST/UTGST, as the run lead directed). In Part B, steps 6 and 7 met every EXPECT; invoice A's post was never approved: its approval window opened minimised, and both attempts (the sheet's one and one retry on the run lead's word, a deviation from 8.3) ended `import_approval_timed_out` with nothing posted. Steps 8.4 to 9 and 0102's EXPECTs were not run (B was never built). Steps 10 to 12 met their EXPECTs for a book with nothing posted; the restore returned the book to BK1.

## Build

| item | read at the lab |
|---|---|
| build line | RC commit `e0467fbc344969ba8d8c8f119e7726e3bdca6cca`, the CI run named on the sheet's build line, artifact `unsigned-bundle-smoke-windows-latest`, 60,609,752 bytes, zip digest `092e41de16d96986ee9b59971beba7f0a893d2cf8aaabf19482ab5f4cf26139c` |
| `.mcpb` | `archive_sha256` `212f5880b8c1daf3e8b48b54002e7c1aded1e771524f613a7b0de354d8fa98a9` |
| `bridge_mcp.exe` run | sha256 `c437138a08dc10a8609e42d28104920401b4615daa69f3cdf404f6e37d6323dd`, equal to the build line before each launch; reports bridge-tally 0.7.0 |

## Launches

The sheet allows one launch from step 6 to step 10. Two were started; the run lead gave the word for the second.

| launch | `BRIDGE_AGENT_DATA_DIR` as the process got it | folder before the launch | stderr | exit |
|---|---|---|---|---|
| 1, 13:36:08Z, pid 19344 | 24 bytes, hex `43:3a:08:72:69:64:67:65:2d:6c:61:62:c2:a4:30:2d:63:70:5c:64:61:74:61:5c`, last 4 hex `6174615c` | existed (made by the lab, empty) | `bridge-mcp: agent_data_dir_unavailable` | not captured (the client stopped on the closed stdout without waiting) |
| 2, 13:41:52Z, pid 18616 | 27 bytes, `<lab folder>\data\`, last 4 hex `6174615c` | existed (the same empty folder) | empty | 0, at 13:49:45Z after the stop |

Launch 1 failed on a mangled value written by the lab's own shell, not on the product: the launch file was written through a shell that rewrites escapes, so a backspace (`08`) and a currency sign (`c2a4`) stand where `\b` and `\24` belong, and the path was invalid. It answered nothing and sent nothing to Tally. Launch 2 had the sheet's exact value: **the trailing backslash with a folder made beforehand is shown to work.** Steps 6 to 10 ran in launch 2. Both launches had `BRIDGE_AGENT_ENABLE_WRITES` present; the line was removed after step 10 (`run/launch-env-hashes.txt` holds each edit's sha256).

## Steps

| step | result | grade |
|---|---|---|
| 1 settings | BRIDGE EINVOICE LAB loaded alone; one COMPANY row, the book's GUID, ALTVCHID 3, ALTMSTID 215; `ISGSTCLASSIFON` No; one `CMPGSTDETAILS.LIST` from 20250401, `HSNSOURCEPRIORITY` Ledger, `HSNAPPLICABLESECTION` All Sections; `BRIDGE IRN Sales` IGST 18, HSN 998313; no customer, CGST or SGST ledger yet | PASS |
| 2 backup | BK1: one file, 32,101 bytes, sha256 `50b036450ee36181269313ad735f3ab5704ce4f363ad8233ee4e5de85d42b692`; `--same-row` against s1 equal | PASS |
| 3 customer, seeded | `I-seed.xml` CREATED 1, no error, no dialog; `BRIDGE CP Customer MH` under Sundry Debtors, not bill-wise, Regular, Maharashtra, from 20250401; ALTMSTID 216 | PASS |
| 4 duty ledgers, keyed | `BRIDGE CP CGST` and `BRIDGE CP SGST` under Duties & Taxes, GST, rate 9, rounding Not Applicable, limit 0; GSTDUTYHEAD reads CGST and SGST/UTGST where the sheet's wording said Central Tax and State Tax (this release's Tax type list offers IGST, CGST, SGST/UTGST, Cess; recorded as the run lead directed); ALTMSTID 218 | PASS, with the wording point |
| 5 Sales type, keyed | `BRIDGE CP Sales`, Sales, Manual, prevent duplicates Yes; ALTMSTID 220 (it rose by two for one save) | PASS |
| 6.1 step 0 | the book loaded alone; one tally.exe; the exe hash equal to the build line; WRITES present; the installed desktop app (pid 16736, left from an earlier row) pointed at port 9002, where nothing listened, and held no connection | PASS |
| 6.2 `tools/list` | 37 tools; `build_import_xml`'s `invoice` holds `e_invoice` requiring `irn`, `ack_no`, `ack_date`; `post_import`, `verify_import`; `vouchers` with `include_e_invoice` | PASS |
| 6.3 `list_companies` | BRIDGE EINVOICE LAB alone, `verified_tuple`, GUID equal to `count8.py`'s BOOK | PASS |
| 6.4 MARKS | M6 220, V6 3, `reused` false, equal to s5r | PASS |
| 6.5 `voucher_types` | `BRIDGE CP Sales`: `series_numbering` method manual, duplicates prevented, observed; the row also carries `numbering_method` `{"unrecognised": "None"}` (recorded) | PASS |
| 7 build of A | `voucher_count` 1, `batch_id` `bridge-50607914-c941-4a47-92b4-565578cc428b`, file sha256 `2a5f35ee4d6c9282de5037fb292403231ec3414e66860368d48a56c46419bd78`, totals 118 / 118, no HSN warning | PASS |
| 8 post of A, attempt 1 | V8a 3; `pending` at 13:43:20Z and 13:44:21Z; `import_approval_timed_out` at 13:45:25Z, `attempt_recorded` false; the window's text has no line beginning `E-invoice`; MARKS after: 220 / 3 | FAIL: not approved in time (window opened minimised, below) |
| 8 post of A, retry | a deviation from 8.3, on the run lead's word: one retry, same batch and arguments, MARKS before 220 / 3; `pending` at 13:46:56Z and 13:47:52Z; `import_approval_timed_out` at 13:48:48Z, `attempt_recorded` false; the window again minimised, its text equal to attempt 1's; MARKS after 220 / 3 | FAIL: not approved in time |
| 8.4, 8.5 | not run (A never posted) | NOT RUN |
| 9 post of B | not run (B never built) | NOT RUN |
| 10 read-back | MARKS 220 / 3; `vouchers` 2025-04-03 with `include_e_invoice`: the book's three vouchers only (Sales 1 with its own IRN, ack 112233 of 20250403; Sales 2 with each absent; Payment 1, `e_invoice` null); no CP voucher; exe closed, exit 0 | PASS for a book with nothing posted; the CP lines NOT RUN |
| 11 what Tally stored | F-0101 and F-0102: 0 VOUCHER rows each; M: the book's GUID, ALTVCHID 3, ALTMSTID 220; `--diff` not run | 0101 absent: PASS; 0102's EXPECT NOT RUN (B never built) |
| 12 restore | BK1 restored (C6); `--same-row` against s1 equal (ALTVCHID 3, ALTMSTID 215); K-customer 0 LEDGER rows; F-0101: 0 VOUCHER rows | PASS |
| C10 | the book shut; Select Company, none loaded; one tally.exe on 9001; `/status` 200 | PASS |

## The approval window opened minimised

- The window is owned by a second process, `bridge_mcp.exe --confirm-journal 18616` (pid 1232 for attempt 1, pid 19256 for the retry), the child of the running server.
- Its title: `ComplyEaze Bridge - post this voucher?` (window class `#32770`), with Yes, No and Cancel.
- It opened minimised: a title bar alone at the screen's bottom-left, just above the taskbar, both times. It was not brought forward.
- Neither process had a main window title: a list of titled windows taken during attempt 1 did not show it, and `MainWindowHandle` read 0 for both `bridge_mcp.exe` processes.
- During attempt 1 the lab found it on a screenshot at about 13:45:10Z, about 15 seconds before the timeout, and told the owner; it timed out at 13:45:37Z unanswered. During the retry the owner had been told beforehand where it opens; it timed out unanswered.
- The lab clicked nothing in it. Its text was read by UI Automation: `evidence/s8-window-a.txt` and `evidence/s8-window-a-retry.txt`.

A user can miss this window inside its time.

## Notes on the record

- The approval window's text for A as built: one Sales invoice in BRIDGE EINVOICE LAB, type `BRIDGE CP Sales`, 3-Apr-2025, number `CP/25-26/0101`, customer `BRIDGE CP Customer MH` (Regular, place of supply Maharashtra, not bill-wise), Dr 118.00 and Cr 100.00, 9.00, 9.00; no line beginning `E-invoice`. The company number line in that text is masked in the pushed copy.
- Each tool answer in `run/answers/` is whole but for `company_number`, replaced by a JSON parser; Tally's raw answers in `tally/` have the company number as X at equal length. Each original is listed by size and sha256 in `kept-local.txt`, with the files that stay on the lab PC (the tool list of step 6.2, screens, listings, the backup, the build, the server's data folder).
- The data folder after step 10: `evidence/s10-data-listing.txt`.
- After the restore the book's data folder lacks side files it held before, as after this lab's earlier restores; they return when the book is loaded.
