# Row 2162-RX: results

Sheet `lab/2162-rx-sheet` at 6750ea49827ca10a9e8813ff1b49444b82d42ba5 (one forward commit on c7a385f48f531d5eedaf1e30e9445979b2e7d547 that added the keys for LVS and LVT). Run 24 Nov 2026, 06:44Z to 07:16Z, on TallyPrime 7.2 Silver (Tally's current date Tuesday 15-Sep-2026, period 1-Apr-26 to 31-Mar-27, no date or period key pressed), book BRIDGE PILOT LAB, GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85 (synthetic), loaded alone. Build: exe sha256=1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59, reports bridge-tally 0.7.0, each launch by `client.py` with the sheet's committed launch file. One line per step in UTC, and the arm table: `evidence/notes.txt`.

## The answer

**Opening either service code's voucher list alone, from GSTR-1's HSN summary, grows the rate answer of both service sales ledgers, with no build launch before it in the session. Ledger Vouchers of a service ledger, and of the CGST ledger, grew nothing. Shut Company and a reload cleared the state before every arm, and the build refused and admitted the invoice in step with the raw read.** Confidence: partial (one book, one release, one build, one sitting).

The request is the build's own rate request (`rates.xml`, 2,874 B, sha256=182f598aafaf333bc8284b6976e664cee00af1eadb5a29bd4f73c295f8601e08, the same bytes as row 2162-RW's). Every grown answer in this sitting, C313's, C314's and W1's, is 30,626 B with sha256=d7f5db99170ea9a643fab195a9465bd20d1f1af1102b76fbefaefe6673a12257: the same bytes as row 2162-RW's A1c. So the voucher list of 998313 alone, the voucher list of 998314 alone, and both lists in one walk give the same growth: in each of `BRIDGE Svc 998313 5%` and `BRIDGE Svc 998314 5%`, under `GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]`, `GSTRATEPERUNIT` (TYPE Number, value 0) on all five `RATEDETAILS.LIST` rows and `GSTRATE` (TYPE Number, value 0) on the Cess and State Cess rows. `ratecmp.py` reports added 28, removed 0, text changed 0 (14 paths under each service ledger) and grades both service ledgers `SHAPE unmeasured`; the CGST, SGST/UTGST and IGST rates (2.50, 2.50, 5) and the other four ledgers did not change. No arm gave a `READ refused`.

Every other answer in the sitting is 29,114 B with sha256=a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62, all six ledgers measured.

## Each arm

| arm | the one act added to a fresh load | outcome word | first differing path (against A0's first answer) | the six SHAPE verdicts | pre against pre2 | `05` | marks |
|---|---|---|---|---|---|---|---|
| A0 | none: loaded, no report | n/a | none (first and second byte-equal) | all measured | n/a (two reads, equal) | not launched | 68/245 |
| C313 | GSTR-1, its HSN summary, the voucher list of 998313 only (19 lines, Total 11,137.40) | both | `LEDGER[NAME='BRIDGE Svc 998313 5%']/GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[1]/GSTRATEPERUNIT[1]` | the two service ledgers unmeasured, the other four measured | A0's two reads on the same load, equal | not launched | 68/245 |
| C314 | GSTR-1, its HSN summary, the voucher list of 998314 only (7 lines, Total 1,366.70) | both | the same path; answer byte-equal to C313's | the two service ledgers unmeasured, the other four measured | equal, both equal to A0 (cleared) | not launched | 68/245 |
| LVS | Ledger Vouchers of `BRIDGE Svc 998313 5%` (1-Sep-26 to 15-Sep-26, 1 voucher line, Closing Balance 13,410.20) | none | none | all measured | equal, both equal to A0 (cleared) | not launched | 68/245 |
| LVT | Ledger Vouchers of `BRIDGE CGST 2.5%` (1-Sep-26 to 15-Sep-26, 1 voucher line, Closing Balance 355.34) | none | none | all measured | equal, both equal to A0 (cleared) | not launched | 68/245 |
| W1 | row 2162-RW's whole walk: both codes' voucher lists | both | the same path; answer byte-equal to C313's | the two service ledgers unmeasured, the other four measured | equal, both equal to A0 (cleared) | L1C refused `invoice_sales_ledger_rate_shape_unmeasured` (both service ledgers); W1-post equal to W1 | 68/245 |
| W2 | Shut Company and a reload after W1 | n/a | none | all measured | equal, both equal to A0 (cleared) | L2 admitted, voucher_count 1; W2 equal to A0 and to pre2 | 68/245 |

No arm was skipped. The full first differing path begins `/ENVELOPE/BODY[1]/DATA[1]/COLLECTION[1]/`.

What the sheet's Decides section draws from this:

- **A0 first and second byte-equal:** the raw read alone changes nothing in this sitting.
- **C313 `both`, C314 `both`:** one code's voucher list is enough to grow both service ledgers, and either list alone does it. Neither needs a build launch before it: A0 launched nothing, and C313 followed A0 on the same load. C313's Escapes ran from the first list, so the second list's Escapes are not needed either.
- **LVS `none`:** Ledger Vouchers of `BRIDGE Svc 998313 5%`, which listed one voucher, did not grow its answer in this sitting. The growth seen here comes from GSTR-1's voucher register, not from every screen that lists that ledger's vouchers.
- **LVT `none`:** this display, which listed one voucher, did not grow the tax ledger's answer in this sitting; it does not show that no screen does. Whether the build refuses on a grown tax ledger stays unmeasured.
- **W1 unmeasured with the target refusal:** row 2162-RW's finding is seen in a second sitting, after a reload and with no launch before the walk in that load.
- **W2 cleared and admitted:** Shut Company and a reload clear the walk's state, again.
- **pre and pre2 equal after every load:** this sitting saw no first-read-after-load effect on the rate answer.
- **Marks never moved:** V 68, M 245 at every read from `start` to `close`.

## The builds

| launch | `05` | evidence bytes | request sha256 | response sha256 | batch |
|---|---|---|---|---|---|
| L1C (W1) | refused, invoice_not_admitted: two `invoice_sales_ledger_rate_shape_unmeasured` (BRIDGE Svc 998313 5%, BRIDGE Svc 998314 5%) | 62,071 | 6d6404a2e22a… | 5db1811db4c0… | none; no file written |
| L2 (W2) | admitted, voucher_count 1, 5 warnings | 1,974,198 | 73e152f0d2a4… | 598e857614d9… | bridge-c27cfaa4-2b0e-454c-a4b4-7a0b57ff5bf5, sha256=c5b0dfcb9e89b4605f554f6ff0f9c61c30499a22c58b39c5900e72ee181d4272 |

The evidence sizes and request and response hashes are the same as row 2162-RW's refused L1C and admitted L0 and L2. In each launch `01` reads `tools: 37; missing of this row's: []`, `02` one company with the book's GUID, `03` and `07` M 245, V 68, reused False. Each launch ended `session ended; server pid <pid> exit code 0`, with no `bridge_mcp` left and an empty server error file.

## Each step's EXPECT

| step | EXPECT | met |
|---|---|---|
| 0 | sheet files equal to SHA256.txt; the four self-checks; exe hash; C2 with no company loaded; LVS and LVT keys proposed and the reply received (the sheet at 6750ea49, its two changed files checked again) | met (check_sheet.py in a clean copy: deviation 2) |
| 1 | the Gateway, 15-Sep-2026; `start` one row, the book's GUID, V 68, M 245; BK1 one non-empty file (90,051 B, sha256=b6fd34f13339a6612a2bba0244e207929de9c714b2ac20a521872cc02a9df0c5); `setup` marks equal; `LEDGER ROWS 6`, the six names and GUIDs as row 2162-RW; `rates.xml` made | met |
| 2 | A0 first and second equal in bytes and sha256, all measured | met |
| 3 | C313: `Vouchers of 998313` (19 lines), back to the Gateway by three Escapes; graded | met |
| 4 | C314: shut and reload, cleared; `Vouchers of 998314` (7 lines), back by three Escapes; graded | met |
| 5 | LVS: shut and reload, cleared; Ledger Vouchers titled with the ledger; back to the Gateway; graded | met, after two stops and the run lead's word on each (deviations 3 and 4) |
| 6 | LVT: the same for `BRIDGE CGST 2.5%`, by the run lead's way back | met |
| 7 | W1: cleared; the walk; L1C graded as the finding seen again; W1-post equal to W1 | met |
| 8 | W2: cleared; L2 `05` admitted, voucher_count 1; W2 equal to A0 and to W2-pre2 | met |
| 9 | arms not run written (none); `close` marks equal, so no restore needed; C10: Shut Company by the recorded keys, no company loaded, one tally.exe owning 9001, `/status` 200, no `bridge_mcp`, date as at step 1, no XML after the shut | met |

## Deviations and notes, each recorded in the notes

1. **The interpreter.** `python3` on the lab PC is the Store alias; every `python3 -I` ran as `python -I` (Python 3.10.11).
2. **check_sheet.py** reads every file under its folder, so in the run folder it listed step 0's own outputs (evidence, captures, lab helpers) as absent from the manifest. It was run in a clean copy of the commit's folder, byte-equal to the 22 sheet files, and passed on both lines.
3. **The Create row at the ledger filter (LVS).** With one ledger line highlighted, Select Ledger still showed its standing Create row above the list, unhighlighted. The keys then read "a Create line ... no Enter", so the lab stopped with no key (capture `s5-lvs-filter-stop.png`, kept local). The run lead's word: that standing row is not the stop meant; the stop there is a highlighted line other than the named ledger, the Create row highlighted, more than one or no ledger line, or any prompt. With the one ledger highlighted: a zoom once more, then Enter. No key was added or changed.
4. **The way back from Ledger Vouchers (LVS, then LVT).** The first Escape returned to Select Ledger; the second, expected to reach Account Books, emptied the box's field and left the box up. The lab stopped with no key (capture `s5-lvs-stop-2.png`, kept local). The run lead's word replaced the sheet's way back in steps 5 and 6: from Ledger Vouchers one Escape; in the Select Ledger box a capture, then one Escape, at most two in that box; at Account Books one Escape; at Display More Reports one Escape; no Escape at the Gateway; a capture before each showing which of those screens is up. Both arms came back by it: one Escape to the box, two in the box, one at Account Books, one at Display More Reports.
5. **A notes repair.** The notes' step 0 fetch line had lost two characters of the row folder's path when it was written; the path was put back, and the W2 shut and load times were set from the captures' write times. The copy before the repair stays local.
6. **count19 files written after the fact.** The count19 checks of `start`, `setup`, A0-first and C313 had gone to the screen only; `count19.py` reads only the saved answers, so it was run again on the same files into `evidence/` (each exit 0, the result the notes record).
7. **The encoding of the ratecmp outputs.** Every `ratecmp.py` line here ran as `python -I` with its output sent to a file, so the twenty outputs were written in the PC's ANSI code page; their one non-ASCII sign is the multiplication sign. Each was turned into UTF-8 with the same text (checked by turning it back to the same bytes); the originals stay local.

Also: the lab helpers (not sheet files, kept local) are row 2162-RW's with the folder changed, plus two that run frame b's and frame d's reads and comparisons in the sheet's order. BK1's Restore-list time is the backup file's write time (12:25:38 local, 06:55:38Z); no restore list was opened. Captures after raw sends with no screen change are byte-equal to each other (the Gateway).

## The record

Tally's raw answers and `run.py` logs (`reads/`), the generated `rates.xml`, every `ratecmp.py`, `count19.py` and `calls.py` output and the notes (`evidence/`), each launch's client log, requests and answers (`run-*`) with `company_number` redacted. Each launch's `01` (`list`) answer holds the product's own tool descriptions and stays on the lab PC; its grading line is quoted above. Captures, BK1, Bridge's data folders, the whole port and process listings and the lab helpers stay on the lab PC, listed by size and sha256 in `kept-local.txt`.
