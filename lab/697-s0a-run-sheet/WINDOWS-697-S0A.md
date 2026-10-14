# Windows lab item 697-S0A: do the audit-read requests work against a real Tally? (#697, 15 Oct 2026). Synthetic company only. Read only.

About 30 minutes. No ComplyEaze Bridge build, no write, no Tally setting changed. The operator sends eight ready request files with curl.exe to port 9001, one at a time, and keeps every answer.

**WHAT IT DECIDES (#697, first slice):** the audit read of ComplyEaze Bridge (pull request #1727) sends seven request shapes to build one book: the company marks, the company Object export, the group, ledger, trial balance and voucher-type collections, and one voucher window with the entry wildcard. Until this runs, every one of them is unmeasured through the shape Bridge sends; the company Object export has only ever been captured with curl on another book. The answers are replayed through the reader afterwards, so they have to be kept byte for byte.

**ALREADY KNOWN (do not re-measure):** `r01` (the company list) and `r02` (the marks) are the files of item 1703-WR, unchanged; `r02` equals what the reader renders for BRIDGE PILOT LAB. The other six were rendered by the reader's own functions for BRIDGE PILOT LAB and the period 2026-04-01 to 2027-03-31. BRIDGE PILOT LAB had `ALTVCHID` 68 in item 1703-WR, so ComplyEaze Bridge would count the window first (a census) before reading it, and might divide it into several parts. This item does not send a census: it sends `s08` as one whole-window request, and the replay afterwards is configured to plan one request, so what is compared is the assembly of real answers, not the planning. The counted and divided path is a later item with a Bridge build. Bridge gives each request 20 seconds before abandoning it.

**FILES** (`C:\bridge-lab\697s0a\requests\` (the files in `requests/` here); each SHA-256 must equal `SHA256.txt`, else STOP):
| file | bytes | what it asks |
|---|---|---|
| r01-company-extent.xml | 2524 | the company list |
| r02-marks.xml | 976 | the company's two high-water marks |
| s03-company-object.xml | 896 | the company Object export (GUID, books-from) |
| s04-groups.xml | 1066 | the group collection |
| s05-ledgers.xml | 1278 | the ledger collection with opening balances and GST registration details |
| s06-trial-balance.xml | 1144 | the ledger trial balance for the period |
| s07-voucher-types.xml | 972 | the voucher-type collection |
| s08-vouchers.xml | 1740 | every voucher of the period, entry wildcard |

**SETUP.** Work in `C:\bridge-lab\697s0a\`. Create `answers\` there, and put a file `.gitattributes` whose only content is `* -text` in `C:\bridge-lab\697s0a\` itself (it covers `answers\` and `send-log.txt`) before saving any answer, so line endings are never rewritten when the folder is committed.

**SEND.** Never abandon a read. Use no `--max-time` and no `--max-filesize`, and never press Ctrl+C: an abandoned read can leave Tally busy for minutes.
`curl.exe -sS --data-binary "@requests\<file>" -H "Content-Type: text/xml; charset=utf-16" -o "answers\<answer>.xml" -D "answers\<answer>.headers.txt" -w "%{http_code} %{size_download} %{time_total}\n" http://127.0.0.1:9001/`
Log one line per request in `send-log.txt`: UTC time the request completed, step, file, curl exit code, HTTP status, bytes, seconds, sha256 of the answer.
**If a request has not returned after 5 minutes:** touch neither curl nor Tally. Take a screenshot, keep waiting, and report.

**PARSE** (PowerShell; never count substrings). Use the same snippet for every answer:
```
$t = [IO.File]::ReadAllText($path, [Text.Encoding]::Unicode)
$t = [regex]::Replace($t, '[\x00-\x08\x0B\x0C\x0E-\x1F]', '')
$t = [regex]::Replace($t, '&#(x[0-9A-Fa-f]+|[0-9]+);', { param($m) $v = $m.Groups[1].Value; if ($v.StartsWith('x')) { $c = [Convert]::ToInt32($v.Substring(1), 16) } else { $c = [int]$v }; if ($c -lt 9 -or $c -eq 11 -or $c -eq 12 -or ($c -ge 14 -and $c -lt 32)) { '' } else { $m.Value } })
$x = [xml]$t
$status = $x.SelectSingleNode('/ENVELOPE/HEADER/STATUS').InnerText
$lineErrors = $x.SelectNodes('//LINEERROR').Count
```
Row counts are taken with `SelectNodes` on the element named in the step, under `/ENVELOPE/BODY/DATA/COLLECTION`, except step 2. A parse error counts as a failed answer, never as zero rows.

**RULES.** ONE TALLY on port 9001, with BRIDGE PILOT LAB loaded alone (never a client company). One request at a time. Nothing typed in Tally during the item. Every EXPECT for an answer also includes HTTP 200, STATUS 1 and no LINEERROR.

**STEPS**
0a. Create the folders as under SETUP. Check the eight hashes and the Tally version. Take a screenshot with the licence serial and email cropped out. Screenshots stay on the PC and are never committed.
0b. Send `r01` (answer `answers\0b.xml`). EXPECT exactly one company, BRIDGE PILOT LAB; Education mode No; Silver Yes; books-from on or before 2026-04-01. Record its GUID and books-from.
1. Send `r02` (answer `answers\1.xml`). EXPECT one COMPANY row with the GUID from 0b. Record `ALTVCHID` (M) and `ALTMSTID`. EXPECT M between 1 and 170 (item 1703-WR recorded 68). If M is 0 or above 170: STOP and report M.
2. Send `s03` (answer `answers\2.xml`). EXPECT exactly one `/ENVELOPE/BODY/DATA/TALLYMESSAGE/COMPANY` element (the answer also holds a `COMPANY` element under `DESC/CMPINFO`, which is a count and is ignored), with a GUID child equal to 0b's GUID and a BOOKSFROM child equal to 0b's books-from. Record whether ISINTEGRATED is present. A different company, a second COMPANY at that path, or a missing GUID or BOOKSFROM: STOP.
3. Send `s04` (answer `answers\3.xml`). Record the GROUP count. EXPECT at least 1, and every GROUP to carry a PARENT child (a top-level group's parent text is a control character and `Primary`; the PARSE snippet removes the control character, so it reads ` Primary`).
4. Send `s05` (answer `answers\4.xml`). Record the LEDGER count and the seconds. EXPECT at least 1 and every LEDGER to carry a PARENT child.
5. Send `s06` (answer `answers\5.xml`). Record the LEDGER count and the seconds. EXPECT at least 1.
6. Send `s07` (answer `answers\6.xml`). Record the VOUCHERTYPE count. EXPECT at least 1; every row has a NAME attribute and a PARENT child.
7. Send `s08` (answer `answers\7.xml`). Record the VOUCHER count (every VOUCHER element under `/ENVELOPE/BODY/DATA/COLLECTION`, skipping a row with no GUID child and no REMOTEID attribute), the bytes and the seconds. EXPECT the count to be at most M, and record the bytes. (The entry wildcard is heavier per voucher than the verification read of 1703-WR; there is no size stop here other than the 16 MiB one, and steps 8 and 9 are sent whatever the size.) A count of 0 is recorded as "envelope only" (still valid, but say so).
8. Send `r02` again (answer `answers\8.xml`). EXPECT `ALTVCHID` = M and `ALTMSTID` unchanged from step 1.
9. Send `r01` again (answer `answers\9.xml`). EXPECT the same single company and GUID.

**STOP** (change nothing, take a screenshot, save, report): a non-200 status or a curl error; STATUS other than 1, or any LINEERROR; a POST over 20 s (let it finish, record it, send nothing more); an answer over 16 MiB (checked after it arrives); a dialog in Tally; a second company loaded; Education mode; any result unlike an EXPECT line.

**REPORT:**
- a table of steps 1 to 9: HTTP status, bytes, seconds, the count recorded;
- M, ALTMSTID and the GUID and books-from from 0b;
- whether step 2's BOOKSFROM was present and in what form (record the element text exactly);
- every answer file, the log and SHA256.txt.
Confidence: one run, PARTIAL.

**Not established:** the counted and divided read of a book above the single-request size (the census requests, and parts planned from it); Education's outward rounding; Bridge's own HTTP headers (curl's User-Agent and Accept differ; the body is the same); the identity check Bridge makes before and after each of these requests, and the marks read its voucher window makes just before the vouchers; timings under load; any client book.
