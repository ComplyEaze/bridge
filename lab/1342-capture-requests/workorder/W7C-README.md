# W7C: one ledger detail, then one read

After W7 part B. It changes one detail of the ledger `BRIDGE Svc 998315 5%` and reads one voucher. It does not touch `BP/26-27/0021`. It answers whether `0021` is still uncertain once the ledger has its HSN/SAC.

**Before you start.** Open only `BRIDGE PILOT LAB`. Take a fresh backup. Check the request file against `SHA256.txt` first (`Get-FileHash`); if it differs, do not send it.

1. **The ledger.** Alter `BRIDGE Svc 998315 5%` (Alter, Ledgers). Under HSN/SAC & Related Details set HSN/SAC Details to `Specify Details Here` and enter the HSN/SAC `998315`. Change nothing else on the ledger (its GST rate and the rest stay as they are). Save it. Screenshot the ledger before you save it.
2. **GSTR-1.** Open GSTR-1 for August 2026 and look at the summary and at the list of uncertain transactions. Screenshot both. Do not use Accept As Is and do not open or alter the voucher.
3. **The read.** Send `w7c-readback-status-0021.xml` with the same `curl.exe` line as before, adding `-w "%{http_code} %{size_download}"`, writing the answer to `aw7c-readback-status-0021.xml` with `-o` and never piping it. Note curl's exit code and the printed line.
4. **The counters.** Send `r02-marks.xml` (as before) and save the answer as `aw7c-marks.xml`.

A read that finds nothing is an answer only if the file is a complete envelope with a `COLLECTION` element and no `LINEERROR`; otherwise keep it and say it failed.

**What to post back.** The answers and the cropped screenshots in a folder `workorder/` on the same branch, their hashes added to `SHA256.txt`, and for each step what happened, with curl's exit code and printed line. Say whether you did anything that differs from this list.
