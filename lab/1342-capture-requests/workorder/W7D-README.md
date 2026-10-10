# W7D: save voucher 0021 again, unchanged, and read it

After W7C and W9. One voucher is opened and saved with no change, and one read is sent. It tells us whether Tally works out the voucher's GSTR-1 status again when the voucher itself is saved, after the ledger's HSN/SAC was set (W7C: the voucher stayed uncertain).

**Before you start.** Open only `BRIDGE PILOT LAB`. Take a fresh backup. Check the request file against `SHA256.txt` first; it is the file you used in W7C, `w7c-readback-status-0021.xml`.

1. Send `r02-marks.xml` (as before) and save the answer as `aw7d-marks-before.xml`.
2. In Tally open the voucher `BP/26-27/0021` (Alter, Vouchers), change nothing, and save it. Screenshot the voucher before you save it.
3. Open GSTR-1 for August 2026 and screenshot the summary and the list of uncertain transactions. Do not use Accept As Is.
4. Send `w7c-readback-status-0021.xml` again, with the same `curl.exe` line and `-w "%{http_code} %{size_download}"`, and save the answer as `aw7d-readback-status-0021.xml`. Note curl's exit code and the printed line.
5. Send `r02-marks.xml` and save the answer as `aw7d-marks-after.xml`.

A read that finds nothing is an answer only if the file is a complete envelope with a `COLLECTION` element and no `LINEERROR`; otherwise keep it and say it failed.

**What to post back.** The answers and the cropped screenshots in a folder `workorder/` on the same branch, their hashes added to `SHA256.txt`, and for each step what happened, with curl's exit code and printed line. Say whether you did anything that differs from this list.
