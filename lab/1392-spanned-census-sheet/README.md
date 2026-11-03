# Lab run sheet: the spanned census of ten transfer days, for #2164 (#1392). SYNTHETIC COMPANY ONLY. Not for merge.

About 15 minutes. Reads only: nothing is keyed, and ComplyEaze Bridge writes nothing.

**WHAT IT DECIDES:** since #2199, every census request Bridge sends carries a span on `$AlterID`. Pull request #2164 replays ten census exchanges captured on 2 Nov from `lab/1392-transfer-days-sheet`, which were sent without that span, so its replay no longer matches what the code sends. This sheet sends the ten requests the code sends now, so the tests replay exactly what Tally was sent. A replay rule that served the old answers for the new requests was refused, because Tally was never sent those requests.

**THE CODE THAT RENDERED THEM:** `render_voucher_census`, `src-tauri/src/agent_read_profiles.rs:249` (its `format!` at :264), reached through `voucher_census_read` at `src-tauri/src/agent_voucher_window.rs:2657`, at `b2cd7e32` on #2164's branch. The span `(0, 12]` was taken by the code from `m2` of `lab/1392-transfer-days-answers`, where `ALTVCHID` was 12. The files are `transfer-days-lab/dDD-c2.request.utf16le.xml` from that commit. Each was checked from the bytes: it is the earlier `dDD-c` request with exactly one insertion before `</SYSTEM>`, ` AND $AlterID &gt; 0 AND $AlterID &lt;= 12`, and nothing else. The marks file is `m.utf16le.xml` of `lab/1392-transfer-days-sheet`, unchanged.

**REQUEST FILES** (UTF-16LE with a byte-order mark, no final line break; check every file in this folder against `SHA256.txt` with `Get-FileHash -Algorithm SHA256` before the first send, and STOP on any mismatch):

| file | what it is |
|---|---|
| `m.utf16le.xml` | company marks, BRIDGE TRANSFER LAB |
| `dDD-c2.utf16le.xml` | census of that one day, April 2026, spanned `(0, 12]` |

Days `DD`: `12`, `13`, `14`, `19`, `20`, `21`, `26`, `27`, `28`, `29`.

`earlier-answers.sha256` holds the sha256 of each day's census answer of 2 Nov (unspanned). It is for the report only: the lab sends nothing from it.

**RULES:**
- Company BRIDGE TRANSFER LAB only, the book of `lab/1392-transfer-days-answers`. Load only it. Port 9001 only. Never touch a client company. If it is not on the lab PC, STOP and report; do not recreate it.
- Before the first send: ComplyEaze Bridge's `tally_status` shows a licensed release (not Education) and exactly that company loaded. Record the release, the edition and the build's `bridge_mcp.exe` sha256. Redaction stays off.
- One request at a time; never abandon one; no `--max-time`. STOP on any dialog or message box you did not expect (press no key, send nothing more), a second company loaded, Education mode, or 10 minutes without an answer.
- Send with exactly: `curl.exe -sS --data-binary "@<file>" -H "Content-Type: text/xml; charset=utf-16" -o "evidence\<name>.response" -D "evidence\<name>.headers" -w "%{http_code} %{size_download} %{time_total}\n" http://localhost:9001/` and record the exit code, the `-w` line and the ISO time of each send in `evidence\log.txt`. A non-zero exit or a non-200 answer is a transport failure: STOP.
- Evidence goes in `C:\bridge-lab\1392-spanned\evidence\`; zip it and report its SHA256. Do not trim or re-save any response.
- Do not key, alter, delete or re-save any voucher or master.

**STEPS**
1. Load the book alone; `tally_status` as above.
2. **Marks before.** Send `m.utf16le.xml` (name `m1`). Report `ALTVCHID` and `ALTMSTID` exactly as received. **If `ALTVCHID` is not 12 or `ALTMSTID` is not 211, STOP and report:** the book has changed since 2 Nov, and these requests' span was set from that day's mark.
3. **Each day, in the order above:** send `dDD-c2` (name `dDD-c2`).
4. **Marks after.** Send `m.utf16le.xml` (name `m2`). If `m2` differs from `m1`, the book changed during the reads: report it; the answers are void.
5. **Count by structure, not by text.** Decode each census response as UTF-16LE and parse it as XML. Count the voucher objects (nodes named exactly `VOUCHER` that carry attributes or children; Tally's header count `<VOUCHER>0</VOUCHER>` is not one), and list each one's `GUID`, `ALTERID`, `DATE`, `VOUCHERTYPENAME` and `VOUCHERNUMBER`. A day that answers with no voucher object is reported as an answer with a count of 0; a failed send is reported as an error, never as 0.
6. **Against 2 Nov.** For each day, say whether the response's sha256 equals that day's line in `earlier-answers.sha256`. Where it does not, give the first differing byte offset and the voucher objects of both.

**PASS / FAIL**
- PASS: all 12 answers received in full (HTTP 200, complete XML), `m1` equals `m2`, and steps 5 and 6 reported. The sheet passes whatever the answers hold: a day that differs from 2 Nov is also an answer.
- FAIL: a transport failure, an unexpected dialog, a mark other than step 2's, a changed mark between `m1` and `m2`, or a count made by substring.

**REPORT, one message:** the release, edition and build; the marks of steps 2 and 4; per response its size, SHA256 and voucher objects; the result of step 6; any STOP; the evidence zip SHA256. The responses, headers and log, unedited, go on a NEW branch `lab/1392-spanned-census-answers`, never on this sheet's branch.
A command that errors is reported as an error, never as "absent" or "unchanged".
