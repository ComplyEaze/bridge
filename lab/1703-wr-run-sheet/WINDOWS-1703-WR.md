# Windows lab item 1703-WR: the cost of one widened verification read (#1703, 14 Oct 2026). Synthetic company only. Read only.

About 25 minutes. No Bridge build, no write, no Tally setting changed. The operator sends four ready request files with curl.exe to port 9001, one at a time.

**WHAT IT DECIDES (#1703):** how much one verification read costs (seconds, bytes, voucher rows) when a batch's window is widened by 7 days either side, against the same read over a single day. A retire POSTs the widened body 4 times: a paired send, then a replay of the pair. Each pair is wrapped in company-list reads and `/status` checks. The cost is reported per POST, and the retire's cost is stated as about 4 × that plus the wrapper. Until this runs, the pull request says "not measured".

**ALREADY KNOWN (do not re-measure):** `r15` is byte for byte what ComplyEaze Bridge renders for BRIDGE PILOT LAB, 2026-08-01. `w01` is the same template for 2026-07-25 to 2026-08-08, also checked against the renderer. `r01` (the company list) equals the bytes of a recorded live Bridge request. When a book's voucher mark (`ALTVCHID`) is 170 or less, Bridge skips the census and sends the window as one request. Bridge gives each request 20 seconds before abandoning it and halving the part.

**FILES** (`C:\bridge-lab\1703wr\requests\` (the files in `requests/` here); each SHA-256 must equal this table, else STOP):
| file | bytes | sha256 |
|---|---|---|
| r01-company-extent.xml | 2524 | 9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8 |
| r02-marks.xml | 976 | 81892a744009b6cf462a6f8d2fd505e650f8ad022e1462b23414cf2b8b3d2d7c |
| r15-verification-window.xml | 1822 | a40f220e631d1c1d0ad0028e1f52f651288af6098f8960bff0aa5e24f3ca02e7 |
| w01-verification-window-widened.xml | 1822 | c9a03ac29b5adb2807b21ec6dfa01581f9ef660159ee443bec245a0f3370bfd3 |

**SEND.** Never abandon a read. Use no `--max-time` and no `--max-filesize`, and never press Ctrl+C: an abandoned voucher read can leave Tally busy for minutes.
`curl.exe -sS --data-binary "@requests\<file>" -H "Content-Type: text/xml; charset=utf-16" -o "answers\<step>.xml" -D "answers\<step>.headers.txt" -w "%{http_code} %{size_download} %{time_total}\n" http://127.0.0.1:9001/`
`/status` between the two sends of a step: `curl.exe -sS -o "answers\<step>-status.txt" -w "%{http_code} %{time_total}\n" http://127.0.0.1:9001/status`
Log one line per request in `send-log.txt`: UTC time the request completed, step, file, curl exit code, HTTP status, bytes, seconds, sha256 of the answer.
**If a request has not returned after 5 minutes:** touch neither curl nor Tally. Take a screenshot, keep waiting, and report.

**PARSE** (PowerShell; never count substrings). Use the same snippet for every answer:
```
$t = [IO.File]::ReadAllText($path, [Text.Encoding]::Unicode)
$t = [regex]::Replace($t, '[\x00-\x08\x0B\x0C\x0E-\x1F]', '')
$t = [regex]::Replace($t, '&#(x[0-9A-Fa-f]+|[0-9]+);', { param($m) $v = $m.Groups[1].Value; if ($v.StartsWith('x')) { $c = [Convert]::ToInt32($v.Substring(1), 16) } else { $c = [int]$v }; if ($c -lt 9 -or $c -eq 11 -or $c -eq 12 -or ($c -ge 14 -and $c -lt 32)) { '' } else { $m.Value } })
$x = [xml]$t
$status = $x.SelectSingleNode('/ENVELOPE/HEADER/STATUS').InnerText
$rows = $x.SelectNodes('/ENVELOPE/BODY/DATA/COLLECTION/VOUCHER').Count
$lineErrors = $x.SelectNodes('//LINEERROR').Count
```
A parse error counts as a failed answer, never as zero rows.

**RULES.** ONE TALLY on port 9001, with BRIDGE PILOT LAB loaded alone (never a client company). One request at a time. Nothing typed in Tally during the item. Every EXPECT for an answer also includes STATUS 1 and no LINEERROR.

**STEPS**
0a. Check the four hashes and the Tally version. Take a screenshot with the licence serial and email cropped out. Screenshots stay on the PC and are never committed.
0b. Send `r01` (company list). EXPECT exactly one company, BRIDGE PILOT LAB; Education mode No; Silver Yes; its books-from date on or before 2026-07-25 (otherwise Bridge would narrow the window, so STOP). Record its GUID.
1. Send `r02` (marks). EXPECT one COMPANY row with the GUID from 0b. Record `ALTVCHID` (M) and `ALTMSTID`. EXPECT M between 1 and 170. If M is 0, the window holds nothing: STOP. If M is above 170, Bridge would count the window first (a census), so this item does not reproduce its request sequence: STOP and report M.
2. Send `r15`, then `/status`, then `r15` again (2a, 2s, 2b). EXPECT HTTP 200 for each; each POST's `time_total` under 20 s; the two answers byte-identical. Record the seconds, the bytes and the VOUCHER count.
3. Send `w01`, then `/status`, then `w01` again (3a, 3s, 3b). The same EXPECTs. EXPECT the VOUCHER count to be at least step 2's. A count of 0 is recorded as "envelope only" (still valid, but say so).
4. Send `r02` again. EXPECT `ALTVCHID` = M and `ALTMSTID` unchanged.
5. Send `r01` again. EXPECT the same single company and GUID.

**STOP** (change nothing, take a screenshot, save, report): a non-200 status or a curl error; STATUS other than 1, or any LINEERROR; a POST over 20 s (let it finish, record it, send nothing more); two answers of one step that differ (save both, do not resend); an answer over 16 MiB (checked after it arrives); a dialog in Tally; a second company loaded; Education mode; any result unlike an EXPECT line.

**REPORT:**
- a table of 2a, 2b, 3a and 3b: seconds, bytes, VOUCHER rows;
- the per-POST cost difference between steps 3 and 2, and the retire estimate (about 4 × the step-3 POST plus the wrapper);
- M and the marks before and after;
- every answer file, the log and SHA256.txt.
Confidence: one run, PARTIAL.

**Not established:** a book above the single-request size; Education's outward rounding; Bridge's own HTTP headers (curl's User-Agent and Accept differ; the body is the same); timings under load.
