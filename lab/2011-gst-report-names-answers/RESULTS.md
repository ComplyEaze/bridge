# Results: #2011 Part 1, Tally's GSTR-1, GSTR-3B and HSN/SAC summary requested by report name on BRIDGE PILOT LAB (August 2026), 26 Oct 2026. PASS: four names are refused in-band ("Could not find Report"), `GSTR3B` answers HTTP 200 with an empty body, and no voucher's GST status or mark moved

The sheet is `lab/2011-gst-report-names` at 23696859f8aff6ca9ac1be83dff00bbfa5b6ab1d. Preflight:
- `SHA256.txt` checked 22/22 OK on sha256 and bytes;
- `check-request-ids.py` gave `census: ok 5, skipped 12, FAIL 0` (exit 0);
- `answers.py --self-test` gave `self-test: ok (17 controls)` (exit 0).

**Scope:** Part 1 only. P10, the optional Yes, was not run, so P11 was not either. Part 2 (BRIDGE GST LEDGER LAB) was not run.

**Release and edition (rule 1)**, from Tally's own About screen: TallyPrime **Release 7.2 (Latest)**, Edition **Silver Rental**, not Educational mode; port 9001.

Every request was sent with the sheet's rule 5 command, one at a time; `answers/log.txt` holds each send. The screen steps were keyed by the lab one key at a time, with the owner present and watching.

## Part 1: BRIDGE PILOT LAB (100033), 1-8-2026 to 31-8-2026
**P1.** The company was loaded alone, with no question or prompt. The Gateway showed period 1-Apr-26 to 31-Mar-27, current date 15-Sep-2026 and the note "(e) Data exceptions exist".

**P2.**
- `p-m0`: one COMPANY row, GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85 (= the committed fixture), **ALTVCHID 68, ALTMSTID 245**, the committed reading of 13 Oct.
- `p-s0` and `p-s0r`: 40 rows each, byte-identical; `compare` gives `same rows`.
- `p-x0`: 32 rows.

**P3. Backup** (C5), into the new empty local folder `backups\p`:
- one file, `TDBK1800_100033.001`;
- 90,051 bytes, sha256 e9f86f0a07a452e05e52cb1c46eb15b935b1393f4c915e19f26a1f49660488ef;
- taken 21:27 IST.

It was kept and not restored.

**P4, the screen, before any report request.** The path was Display More Reports > Statutory Reports > GST Reports. The menu: Track GST Return Activities, GSTR-1, GSTR-3B, Annual Computation, e-Way Bill, GSTR-1 / 2A / 2B Reconciliation, Challan Reconciliation, Invoice Management System (IMS), GST Utilities, Other Reports. **There is no separate HSN/SAC summary entry.**

| Report | Question to update GST status | Figures as printed |
|---|---|---|
| GSTR-1 | **none: N1 = none** (opened at once) | GSTIN 08ZZZZZ0000Z1ZQ, Not Filed. Total Vouchers 39; Included in Return 27 (Ready for Upload 27; Modified in Books After Upload/Export 27); Not Relevant 7; Uncertain (Corrections needed) 5; Conflicts with Masters: Yes. Return View: B2C (Small) - 7: 27 (2), taxable 13,508.70, CGST 383.26, SGST 383.24, tax 766.50; every other row blank, including HSN Summary - 12 and Document Summary - 13. Total 13,508.70 / 383.26 / 383.24 / 766.50 |
| GSTR-3B | **none: N1 = none** | Total Vouchers 39; Included 27; Not Relevant 5; Uncertain 7; Conflicts: Yes. 3.1: 13,508.70 / CGST 383.26 / SGST 383.24 / tax 766.50. 3.2, 4A to 4D, 5 and 6.1 are blank |

Recorded: GSTR-3B's Not Relevant 5 and Uncertain 7 are the reverse of GSTR-1's 7 and 5.

**P5.** `p-s1` against `p-s0` gives `same rows` (40/40), and `p-m1` is byte-identical to `p-m0`. The screen step changed nothing. These two reads are rule 6's baseline.

**P6 and P7: the report names.** After each request a screenshot showed the Gateway with no dialog, and rule 6 gave `same rows` with marks equal to the baseline (68 / 245).

| File (ID) | HTTP, bytes, curl time | Class (`answers.py shape`) |
|---|---|---|
| `p-unknown` (Bridge Unknown Report Probe) | 200, 448 B, 0.024 s | **refused**: LINEERROR "Could not find Report 'Bridge Unknown Report Probe'!" |
| `p-gstr1-a` (`GSTR-1`) | 200, 406 B, 0.020 s | **refused**: "Could not find Report 'GSTR-1'!" |
| `p-gstr1-b` (`GSTR1`) | 200, 404 B, 0.020 s | **refused**: "Could not find Report 'GSTR1'!" |
| `p-gstr3b-a` (`GSTR-3B`) | 200, 408 B, 0.020 s | **refused**: "Could not find Report 'GSTR-3B'!" |
| `p-gstr3b-b` (`GSTR3B`) | **200, 0 B, 0.921 s** | **does not parse** (shape exit 3: "no element found") |
| `p-hsn-a` (`HSN/SAC Summary`) | 200, 424 B, 0.024 s | **refused**: "Could not find Report 'HSN/SAC Summary'!" (RESPSTATUS 0) |

`GSTR3B`'s answer stands apart:
- its headers are only `HTTP/1.1 200 OK`, `RESPSTATUS: 1`, `CONNECTION: KEEP-ALIVE` and `CONTENT-LENGTH: 0`, with no CONTENT-TYPE and no `Unicode` line;
- it took 0.92 s, against about 0.02 s for the refusals.

It cannot be told from a report that was not rendered (§12a.11). Status: unverified (one request, one run).

**P8.** `p-x1` against `p-x0` gives `same rows` (32/32).

**P9, the screen again.** GSTR-1 and GSTR-3B both opened with no question, so **N2 = none** for each, and every figure is as at P4. N1 and N2 are both none, so no update question was pending. **This run cannot show whether a gateway request would update a pending GST status.**

**P12.** Company > Shut, then Select Company with none loaded; one tally.exe; 9001 LISTENING; /status 200.

## Findings
- **No report name in `requests/` is answered with a report on 7.2.**
  - `GSTR-1`, `GSTR1`, `GSTR-3B` and `HSN/SAC Summary` are refused in the same shape as the unknown name (verified, one run).
  - `GSTR3B` gives an empty 200 (unverified).
  - As the sheet says, this shows that these names are refused, not that the reports cannot be read.
- **No request changed any voucher's GST status flags or the company's marks.** Every rule-6 compare was `same rows` and every mark 68 / 245 (verified for this book; no update question was pending).
- **The screen figures for August 2026** are as in the P4 table (verified, two readings).

## Lab notes (no effect on the requests)
- The lab's helper scripts cut their own output short twice.
  - Once, after `p-s2` had answered 200, the rule-6 script misread this as a failed send. It stopped before `p-m2`, which was then sent about 30 s later with nothing between. `answers/log.txt` shows the order.
  - `p-s1`, `p-m1` and `p-s2` missed their `shape` read at send time; it was run on the saved files afterwards and kept in `notes.txt`.
- The C5 marks read after the backup was not sent, because P3 names no request. P5's marks read follows.

## In this commit
- `answers/`: every response and headers file, unedited (25 of each); `log.txt`; `notes.txt` (UTC, one line per step, with every `shape` and `compare` output whole; in this copy the lab PC's own folder is written `<lab folder>`).
- `SCREENSHOTS.txt` (name and sha256) and `SHA256.txt`.

The evidence folder as a zip: 59,720 bytes, sha256 45b9f70d204af7cba1379d5045a556b82c90115c2941c32414946d94c2778d28, kept on the lab PC. Also not committed: the screenshots (the About screen carries licence and machine details) and the backup.
