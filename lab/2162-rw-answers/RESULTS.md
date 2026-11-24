# Row 2162-RW: results

Sheet `lab/2162-rw-sheet` at 6b6c6915b4ba1d4efa3275db164d66b441e7dd54. Run 24 Nov 2026, 05:40Z to 06:02Z, on TallyPrime 7.2 Silver (Tally's current date Tuesday 15-Sep-2026, period 1-Apr-26 to 31-Mar-27, no date or period key pressed), book BRIDGE PILOT LAB, GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85 (synthetic), loaded alone. Build: exe sha256=1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59, reports bridge-tally 0.7.0, each launch by `client.py` with the sheet's committed launch file. One line per step in UTC, and the arm table: `evidence/notes.txt`.

## The answer

**Opening the voucher lists of the two service codes from GSTR-1's HSN summary changes Tally's ledger answer for the two service ledgers, and that change is what the build refuses. Shut Company and a reload clear it.** Confidence: partial (one book, one release, one build, one sitting).

After the walk into `998313` and `998314`, Tally's answer to the build's own rate request (`rates.xml`, the exact request text of the build, 2,874 B, sha256=182f598aafaf333bc8284b6976e664cee00af1eadb5a29bd4f73c295f8601e08) grew from 29,114 B to 30,626 B. In each of `BRIDGE Svc 998313 5%` and `BRIDGE Svc 998314 5%`, under `GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]`, Tally added:

- `GSTRATEPERUNIT` (TYPE Number, value 0) to all five `RATEDETAILS.LIST` rows (CGST, SGST/UTGST, IGST, Cess, State Cess);
- `GSTRATE` (TYPE Number, value 0) to the Cess row and the State Cess row, which had none before.

That is 14 elements and their 14 TYPE attributes: `ratecmp.py` reports added 28, removed 0, text changed 0. The CGST, SGST/UTGST and IGST rates (2.50, 2.50, 5) did not change, and no other ledger changed. `ratecmp.py` grades both service ledgers `SHAPE unmeasured`: each rate row "has outside child GSTRATEPERUNIT". The build, launched against that state, refused `05` with `invoice_sales_ledger_rate_shape_unmeasured` for exactly those two ledgers. The build and the raw read agree on the shape, and the named difference is the cause this sitting shows. The added fields are all zero; they carry no rate the book did not already hold.

## Each arm

| arm | what was done | rate answer (bytes, sha256) | first differing path | the six SHAPE verdicts | `05` | marks |
|---|---|---|---|---|---|---|
| A0 | loaded, no report; two reads, then L0 | 29,114 B, a6197d90… (first, second and final equal) | none | all measured | admitted, voucher_count 1 | 68/245 before and after |
| A1a | GSTR-1 opened, Esc | 29,114 B, a6197d90…, equal to A0 | none | all measured | no build check | 68/245 |
| A1b | GSTR-1 and its HSN summary, Esc, Esc | 29,114 B, a6197d90…, equal to A0 | none | all measured | no build check | 68/245 |
| A1c | the summary, then the voucher lists of 998313 and 998314, Esc to the Gateway | 30,626 B, d7f5db99… (pre and final equal) | `LEDGER[NAME='BRIDGE Svc 998313 5%']/GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[1]/GSTRATEPERUNIT[1]` | the two service ledgers unmeasured, the other four measured | refused `invoice_sales_ledger_rate_shape_unmeasured` (both service ledgers) | 68/245 |
| A2 | Shut Company, load the book alone; two reads, then L2 | 29,114 B, a6197d90…, pre, pre2 and final equal to A0 | none | all measured | admitted, voucher_count 1 | 68/245 |
| A3 | not run: A2 cleared the refusal | | | | | |
| A4 | not run: A2 cleared the refusal; no restore was made | | | | | |

The answer hashes in full: a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62 and d7f5db99170ea9a643fab195a9465bd20d1f1af1102b76fbefaefe6673a12257.

What the sheet's Decides section draws from this:

- **A0's two reads byte-equal**, and A0's final read equal after L0: a raw rate read alone changes nothing, and neither does a build launch.
- **A1a and A1b quiet, first difference in A1c-pre:** opening a code's voucher list is the added trigger. Opening GSTR-1, and opening its HSN summary, changed nothing. A1c opens both codes, so it does not tell 998313's list from 998314's; both ledgers changed together.
- **A1c target refusal with an unmeasured raw row:** the walk and the build's rule agree.
- **First clear at A2:** shutting and loading the company clears the state; no Tally restart and no restore were needed. A2's pre-read and its second read were byte-equal, so this sitting saw no first-read-after-load effect on the rate answer.
- **The build changes nothing:** each arm's final read equals its pre-read (A1c: d7f5db99… both; A2: a6197d90… all three).

## The builds

| launch | `05` | evidence bytes | request sha256 | response sha256 | batch |
|---|---|---|---|---|---|
| L0 (A0) | admitted, voucher_count 1, 5 warnings | 1,974,198 | 73e152f0d2a4… | 598e857614d9… | bridge-1610adb5-8ac8-4926-8704-05bcb085530f, sha256=4298a8b48d38da4bcd2a7751fa8d218f0d0b4302c355cb3320c81e09f0a2778c |
| L1C (A1c) | refused, invoice_not_admitted: two `invoice_sales_ledger_rate_shape_unmeasured` | 62,071 | 6d6404a2e22a… | 5db1811db4c0… | none; no file written |
| L2 (A2) | admitted, voucher_count 1, 5 warnings | 1,974,198 | 73e152f0d2a4… | 598e857614d9… | bridge-921b8b60-4775-46fc-968e-32d25bb87657, sha256=05d06c4d27073f385398b00c33fc949f5e2a2583d11256b5f9579eecc90bb799 |

The admitted and refused evidence sizes and request hashes are the same as row 2162-SALES-Q's admitted and refused builds. In each launch `01` reads `tools: 37; missing of this row's: []`, `02` one company with the book's GUID, `03` and `07` M 245, V 68, reused False. Each launch ended `session ended; server pid <pid> exit code 0`.

## Each step's EXPECT

| step | EXPECT | met |
|---|---|---|
| 0 | sheet files equal to SHA256.txt; the four self-checks; exe hash; C2 with no company loaded; keys asked and the reply received | met |
| 1 | the Gateway, 15-Sep-2026; `start` one row, the book's GUID, V 68, M 245; BK1 one non-empty file (90,051 B, sha256=fc092e6ce2b77cafb1986e7edbac2744643a19e1985b4c7028da341ef3a2259c); `setup` marks equal; `LEDGER ROWS 6`, the six names, the service ledgers under Sales Accounts as `-000000d1` and `-000000d2`; `rates.xml` made | met |
| 2 | A0 first and second equal in bytes and sha256, all measured; L0 `05` admitted; A0 final equal | met |
| 3 | GSTR-1 on Total Vouchers, period 1-Aug-26 to 31-Aug-26 (no period key); marks equal | met |
| 4 | the 24 lines in the recorded order, the HSN summary; marks equal | met |
| 5 | 998313's and 998314's voucher lists; L1C graded as the first case (candidate holds); A1c final equal to A1c-pre | met |
| 6 | Shut Company by the named keys, the list BRIDGE PILOT LAB alone, Select Company with no prompt; the load; A2 graded cleared; A2 final equal to pre2 | met |
| 7, 8 | not run (A2 cleared) | n/a |
| 9 | `close` marks equal (no restore needed); C10: Select Company, no company loaded, one tally.exe owning 9001, `/status` 200, no `bridge_mcp`, date as at step 1 | met |

## Deviations and notes, each recorded in the notes

1. **The interpreter.** `python3` on the lab PC is the Store alias; every `python3 -I` ran as `python -I` (Python 3.10.11).
2. **Lab helpers** (not sheet files, kept local): `send.sh` sends one request per `run.py` call after checking its sha256 and takes the capture the sheet asks for after every raw send; `launch.ps1` and `put-part.ps1` do the Launches section's steps 1 to 4 (the call file goes in as `.part`, is hash-checked and renamed). The first `launch.ps1` (L0) returned to its caller only when the client exited; the launch itself was not affected.
3. **BK1's Restore-list time** is the backup file's write time (11:19:25 local, 05:49:25Z). No restore list was opened, since A4 was not run.
4. **guids.xml and rates.xml** gave byte-identical answers at A0 and `setup` (a6197d90…).
5. **The captures** after raw sends with no screen change are byte-equal to each other (the Gateway); each is listed in `kept-local.txt`.

## The record

Tally's raw answers and `run.py` logs (`reads/`), the generated `rates.xml`, every `ratecmp.py`, `count19.py` and `calls.py` output and the notes (`evidence/`), each launch's client log, requests and answers (`run-*`) with `company_number` redacted. Each launch's `01` (`list`) answer holds the product's own tool descriptions and stays on the lab PC; its grading line is quoted above. Captures, BK1, Bridge's data folders, the whole port and process listings and the lab helpers stay on the lab PC, listed by size and sha256 in `kept-local.txt`.
