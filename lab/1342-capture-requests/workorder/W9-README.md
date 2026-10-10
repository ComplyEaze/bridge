# W9: one ledger, read two ways

After W7C. Two reads that change nothing, of the one ledger `BRIDGE Svc 998315 5%` (HSN/SAC 998315 set in W7C). They ask Tally for the ledger's HSN/SAC details and its type of supply, which the earlier ledger answers did not carry. No backup is needed and nothing is keyed.

**Before you start.** Open only `BRIDGE PILOT LAB`. Check each request file against `SHA256.txt` first (`Get-FileHash`); if one differs, do not send it.

**How to send** (as before). One request at a time, with the same `curl.exe` line and `-w "%{http_code} %{size_download}"`, each answer written to a file with `-o` and never piped. Note curl's exit code and the printed line for every request. A read that finds nothing is an answer only if the file is a complete envelope with a `COLLECTION` element and no `LINEERROR`; otherwise keep it and say it failed.

| Step | Send | Save as | Expect, and what to do if not |
| --- | --- | --- | --- |
| 1 | `w9-ledger-hsn-all.xml` | `aw9-ledger-hsn-all.xml` | the one ledger with every field Tally returns for it; if no ledger comes back, keep the answer and say so |
| 2 | `w9-ledger-hsn-fields.xml` | `aw9-ledger-hsn-fields.xml` | the same ledger with a short field list; if a field is missing from the answer, keep it whole and say so |

**A screen.** Open the ledger `BRIDGE Svc 998315 5%` in display mode (do not alter it) and screenshot the whole ledger screen, showing the HSN/SAC and the GST rate details and the type of supply.

**What to post back.** The two answers and the screenshot in a folder `workorder/` on the same branch, their hashes added to `SHA256.txt`, and for each step what happened, with curl's exit code and printed line. Say whether you did anything that differs from this list.
