# Lab run sheet: Place of Supply, State and registration type in the vouchers read (#2027). SYNTHETIC COMPANY ONLY. Not for merge.

About 15 minutes. Reads only: four requests on one book. ComplyEaze Bridge writes nothing and nothing in Tally is changed.

**WHAT IT DECIDES:** #2027 asks `vouchers` to return a voucher's Place of Supply and the party's State and GST registration type, as Tally's "Party Details" screen shows them. Committed answers to other Bridge reads carry them as voucher-level `PLACEOFSUPPLY`, `STATENAME` and `GSTREGISTRATIONTYPE`, but the request the vouchers read sends does not ask for them. Adding them changes that request. AGENTS.md P7 says a changed request is sent to a lab book first and joins the shapes a release may send only after it is observed. This sheet sends the candidate beside today's request on the book of lab row W3 (#691), whose four vouchers' Party Details were recorded on screen. The parse is then built on the captured answer (P1).

**THE CANDIDATE (for review with this sheet):** today's vouchers request (`Bridge Agent Vouchers`, built by `render_agent_vouchers`) with three elements appended to its `FETCH` list: `PLACEOFSUPPLY,STATENAME,GSTREGISTRATIONTYPE`. Nothing else differs: the same collection, the same filter, the same company and dates. The two request files differ only by those 44 characters of the `FETCH` text. Both were rendered by the product's own builder, unedited; no request is written by hand. The candidate is not in any shipped build.

**EXPECT (an expectation, not a fact):** the candidate's answer is today's answer with, on each of the four vouchers, `PLACEOFSUPPLY` `Maharashtra`, `STATENAME` `Maharashtra` and `GSTREGISTRATIONTYPE` `Regular`, as W3's screens showed. Report what happened, not what is expected.

**REQUEST FILES** (each the product's own rendering, unedited: UTF-16LE with a byte-order mark, no final line break; check every one against `SHA256.txt` with `Get-FileHash -Algorithm SHA256` before the first send, and STOP on any mismatch):

| file | what it is |
|---|---|
| `m.utf16le.xml` | company marks (`Bridge Agent Company High Water`), BRIDGE NOTES LAB |
| `v-control.utf16le.xml` | the vouchers request as master sends it today, 1 to 2 June 2025 |
| `v-candidate.utf16le.xml` | the same with `PLACEOFSUPPLY,STATENAME,GSTREGISTRATIONTYPE` added to `FETCH` |

**REQUESTS BY CURL** are allowed by the owner's decision of 26 Oct 2026 that requests sent by curl may be used to discover shapes; every request here only reads. `m` and `v-control` are existing builders' requests used as instruments (`render_agent_company_high_water`; `render_agent_vouchers`, whose `FETCH` it equals on master); `v-candidate` is the candidate change to `render_agent_vouchers` named above. Before the first send, in this folder: `python -I check-request-ids.py .`; EXPECT `census: ok 3, skipped 0, FAIL 0`; anything else = STOP, nothing sent. Read bounds (C3): the marks read 8,192 bytes; each vouchers read 2,000,000 bytes; an answer above its bound = STOP.

**THE BUILD (for `tally_status` only):** the build staged at the lab as `w1-2306d86d`, `bridge_mcp.exe` sha256 `92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524` (hashed first; another hash = STOP), over stdio with `BRIDGE_TALLY_PORT=9001`, a new empty `BRIDGE_AGENT_DATA_DIR` and `BRIDGE_TERMS_ACCEPTED=true`; posting, import and redaction unset.

**RULES:**
- Company BRIDGE NOTES LAB only (company number 100084; the four vouchers of W3 on 1 and 2 June 2025). Load only it. Port 9001 only. Never touch a client company. If it is not on the lab PC, STOP and report; do not re-key it.
- Before the first send: ComplyEaze Bridge's `tally_status` shows a licensed release (not Education) and exactly that company loaded. Record the release and edition.
- One request at a time; never abandon one; no `--max-time`. STOP on any dialog or message box on Tally's screen (press no key, send nothing more, screenshot), a second company loaded, Education mode, or 10 minutes without an answer.
- Send with exactly: `curl.exe -sS --data-binary "@<file>" -H "Content-Type: text/xml; charset=utf-16" -o "evidence\<name>.response" -D "evidence\<name>.headers" -w "%{http_code} %{size_download} %{time_total}\n" http://localhost:9001/`. Record the exit code, the `-w` line and the ISO time of each send in `evidence\log.txt`. A non-zero exit or a non-200 answer is a transport failure: STOP, and report it as a failure, never as "no vouchers".
- Evidence goes in `C:\bridge-lab\2027\evidence\`; zip it and report its SHA256. Do not trim or re-save any response.

**STEPS**
1. Load the book alone; `tally_status` as above.
2. **Marks before.** Send `m.utf16le.xml` (name `m1`). Report `ALTVCHID` and `ALTMSTID` exactly as sent (W3 ended at 4 and 215). A different mark is not a stop, but report it.
3. **Today's request.** Send `v-control.utf16le.xml` (name `v1`).
4. **The candidate.** Send `v-candidate.utf16le.xml` (name `v2`).
5. **Marks after.** Send `m.utf16le.xml` again (name `m2`). If either mark differs from step 2, the book changed during the reads: report it; the answers are void.
6. **Count by structure, not by text.** Decode each vouchers answer as UTF-16LE and parse it as XML (for example PowerShell `[xml]`). Count the nodes named exactly `VOUCHER`. For each voucher list `VOUCHERTYPENAME`, `VOUCHERNUMBER`, `DATE`, and in `v2` the three new elements with their attributes and text: present or absent, and if present, empty or the value. A substring count of `<VOUCHER` also matches `<VOUCHERNUMBER>` and is not accepted.
7. **Compare.** Say whether `v2` equals `v1` except for the three elements on each voucher. Remove those elements from `v2` with the parser (not by text), serialise both the same way, and compare. If anything else differs, list each differing element by name and voucher.

**PASS / FAIL**
- PASS: all four answers received in full (HTTP 200, complete XML), the marks equal before and after, and steps 6 and 7 reported. The sheet passes whatever the answers hold, including the elements being absent: that is also an answer.
- FAIL: a transport failure, a dialog, a changed mark, or a count made by substring.

**REPORT, one message:** the release and edition; the marks of steps 2 and 5; per response its size, SHA256 and `VOUCHER` count; step 6's table; step 7's result; any STOP; the evidence zip SHA256. The responses, headers and log, unedited, go on a NEW branch `lab/2027-place-of-supply-answers` under `answers/`, with their SHA256 list, for the fixtures and their provenance: send its SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes. Nothing is committed to this branch. The answers carry the synthetic GSTINs typed in W3; they are synthetic and may be committed as received.
A command that errors is reported as an error, never as "absent" or "unchanged".
