# W8: four small reads of the GSTR-1 status, narrower than before

Reads only: no write, no backup needed, nothing to key. After W7 part A. They tell us whether the status fields come back on a short field list, whether a voucher-type filter works beside a date window, and what the code's own single-voucher read-back returns. Stop wherever time runs out.

**Before you start.** Open only `BRIDGE PILOT LAB`. Check each request file against `SHA256.txt` first (`Get-FileHash`); if one differs, do not send it.

**How to send** (as before). One request at a time, with the same `curl.exe` line and `-w "%{http_code} %{size_download}"`, each answer written to a file with `-o` and never piped. Note curl's exit code and the printed line for every request. A read that finds nothing is an answer only if the file is a complete envelope with a `COLLECTION` element and no `LINEERROR`; otherwise keep it and say it failed.

| Step | Send | Save as | Expect, and what to do if not |
| --- | --- | --- | --- |
| 1 | `w8-status-narrow-window.xml` | `aw8-status-narrow-window.xml` | the vouchers dated 2-Aug and 3-Aug (the five Sales invoices and the four Receipts you made), with the status fields; if the status fields are missing, keep the answer whole and say so |
| 2 | `w8-status-narrow-window-sales.xml` | `aw8-status-narrow-window-sales.xml` | the same window with only the `BRIDGE Sales` vouchers |
| 3 | `w8-readback-status-single.xml` | `aw8-readback-status-single.xml` | one voucher, `BP/26-27/0010`, with the status fields |
| 4 | `w8-status-narrow-window-sales-byname.xml` | `aw8-status-narrow-window-sales-byname.xml` | the same window as step 2, with the voucher type named instead of its GUID: only the `BRIDGE Sales` vouchers; if the answer includes the Receipts, or is not a complete envelope, keep it and say so |

**What to post back.** The three answers in a folder `workorder/` on the same branch, their hashes added to `SHA256.txt`, and for each step what happened, with curl's exit code and printed line. Say whether you did anything that differs from this list.
