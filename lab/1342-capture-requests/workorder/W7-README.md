# W7: two reads of what the code needs, and an optional three-line invoice

After W6. Part A is two reads and no write; it is the part we need first. Part B is optional, only if time allows, and keys one voucher. Stop wherever time runs out.

**Before you start**
- Open only `BRIDGE PILOT LAB`. Check each request file against `SHA256.txt` first (`Get-FileHash`); if one differs, do not send it.
- Part A changes nothing. Part B needs a fresh backup first.

**How to send** (as before)
- One request at a time, with the same `curl.exe` line and `-w "%{http_code} %{size_download}"`, each answer written to a file with `-o` and never piped. Note curl's exit code and the printed line for every request.
- A read that finds nothing is an answer only if the file is a complete envelope with a `COLLECTION` element and no `LINEERROR`; otherwise keep it and say it failed.

## Part A: reads

| Step | Send | Save as | Expect, and what to do if not |
| --- | --- | --- | --- |
| 1 | `w7-ledgers-rates.xml` | `aw7-ledgers-rates.xml` | the ledgers of the company, each with its GST details and its rate and rounding fields |
| 2 | `w7-readback-status.xml` | `aw7-readback-status.xml` | three vouchers (`BP/26-27/0002`, `0010`, `0016`) with their GSTR-1 status fields; if the status fields are missing from the answer, keep it whole and say so |

## Part B (optional): one invoice with three sales lines

This tells us whether the tax Tally works out on three lines of one rate adds up the same way as on two. It needs a third 5% sales ledger.

1. Take a fresh backup.
2. Create the ledger `BRIDGE Svc 998315 5%` under Sales Accounts, with the same GST details as `BRIDGE Svc 998314 5%` (taxable, 5%, the rate specified in the ledger). Screenshot its GST details.
3. Key `BP/26-27/0021`: type `BRIDGE Sales`, dated 5-Aug-2026, the walk-in customer, invoice mode, no round off. Sales lines `BRIDGE Svc 998313 5%` 100.10, `BRIDGE Svc 998314 5%` 100.10 and `BRIDGE Svc 998315 5%` 100.10. Select `BRIDGE CGST 2.5%` and `BRIDGE SGST 2.5%` and leave the amounts exactly as Tally fills them. If Tally fills no amount, stop and tell us. Save it. Screenshot the voucher.
4. Send `r02-marks.xml` (as before) → `aw7-marks-b.xml`.
5. Send `d-vouchers.xml` (as before) → `vouchers-w7.xml`.
6. Screens (look, change nothing, do not use Accept As Is): GSTR-1 for August 2026, showing whether `0021` is included or uncertain, with Tally's reason if uncertain; and the GST tax analysis of `0021` (Ctrl+O in display mode), with the tax amounts it shows.

**What to post back.** The answers and the cropped screenshots in a folder `workorder/` on the same branch, their hashes added to `SHA256.txt`, and for each step what happened, with curl's exit code and printed line. Say whether you did anything that differs from this list.
