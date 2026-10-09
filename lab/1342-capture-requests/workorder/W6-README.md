# W6: one invoice, one sales ledger on two lines

One more measurement, after W1 to W5. It is one voucher: a Sales invoice dated 15-Sep-2026 that carries the same sales ledger on two lines, imported once. It tells us whether Tally keeps the two lines apart when a voucher is imported, and which tax it expects on it. Take the steps in order and stop wherever time runs out.

**Before you start**
- Take a fresh backup of the company, and open only `BRIDGE PILOT LAB`.
- Check each request file against `SHA256.txt` first (`Get-FileHash`). If one differs, do not send it.
- Change no setting and no other voucher. Key nothing.

**How to send** (as before)
- One request at a time, with the same `curl.exe` line and `-w "%{http_code} %{size_download}"`, each answer written to a file with `-o` and never piped. Note curl's exit code and the printed line for every request.
- A read that finds nothing is an answer only if the file is a complete envelope with a `COLLECTION` element and no `LINEERROR`; otherwise keep it and say it failed.
- Send the import file **once**. If it gets no answer within 10 minutes, send nothing more except the reads of steps 4 and 5, and tell us.

| Step | Send | Save as | Expect, and what to do if not |
| --- | --- | --- | --- |
| 1 | `r02-marks.xml` (as before) | `aw6-marks-before.xml` | the two counters |
| 2 | `w6-window-0915.xml` | `aw6-window-before.xml` | a complete envelope with no vouchers; if it lists one, stop and tell us |
| 3 | `w6-same-ledger-two-lines.xml` | `aw6-import.xml` | one voucher created (`BP/26-27/0020`); whatever the answer is, keep it whole, do not send the file again, and go on to step 4 |
| 4 | `r02-marks.xml` | `aw6-marks-after.xml` | |
| 5 | `w6-window-0915.xml` | `aw6-window-after.xml` | |
| 6 | `d-vouchers.xml` (as before) | `vouchers-w6.xml` | the export of every voucher; it holds `0020` if Tally created it |

**Screens** (only if step 3 created the voucher; look, change nothing, do not use Accept As Is)
- The voucher `BP/26-27/0020` in display mode, showing its lines: say whether it shows one line of `BRIDGE Svc 998313 5%` or two.
- GSTR-1 for 1-Sep-2026 to 30-Sep-2026: the summary, and whether `0020` is listed as included or as uncertain, with Tally's reason if uncertain.
- The GST tax analysis of `0020` (Ctrl+O in display mode), with the tax amounts it shows.

**What to post back.** The answers and the cropped screenshots in a folder `workorder/` on the same branch, their hashes added to `SHA256.txt`, and for each step what happened, with curl's exit code and printed line. Say whether you did anything that differs from this list.
