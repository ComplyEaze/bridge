# Lab run sheet: the census sent as an AlterID span on two small books (#899, part 2). SYNTHETIC COMPANIES ONLY. Not for merge.

About 20 minutes. Reads only: eight requests, four per book. ComplyEaze Bridge writes nothing and nothing in Tally is changed.

**WHAT IT DECIDES:** issue #899 part 2 is to send the voucher census of a small book as the span `$AlterID > 0 AND $AlterID <= M` (M the book's voucher mark) instead of with no AlterID filter, so every census is bounded by construction. The maintainers chose to build it only on committed captures of the spanned request (4 Oct): a response captured for the unspanned request may not stand in for the spanned one. This sheet captures, on two books, the company marks, the census as master sends it today and the census as part 2 would send it, so the change can be built and replayed on real answers (AGENTS.md P1).

**WHICH BUILDER IT BECOMES:** the voucher census builder (`render_agent_voucher_census`), sending a small book's census as the span `$AlterID > 0 AND $AlterID <= M`; the spanned files here differ from today's census only by that filter (checked byte for byte). Requests by curl are allowed for shape discovery (the owner's decision of 26 Oct 2026); this item only reads.

**EXPECT (an expectation, not a fact):** on each book the two census answers are byte-identical, since every voucher's AlterID is at most the mark (seen once on 5 Oct on two other books). Report what happened, not what is expected.

**REQUEST FILES** (each the product's own rendering, unedited: UTF-16LE with a byte-order mark, no final line break; check every one against `SHA256.txt` with `Get-FileHash -Algorithm SHA256` before the first send, and STOP on any mismatch):

| file | what it is | book |
|---|---|---|
| `t-m.utf16le.xml` | company marks (`Bridge Agent Company High Water`) | BRIDGE TRANSFER LAB |
| `t-c1.utf16le.xml` | census, 1 to 5 April 2026, no AlterID filter (what master sends) | BRIDGE TRANSFER LAB |
| `t-c2.utf16le.xml` | the same census with `$AlterID > 0 AND $AlterID <= 5` | BRIDGE TRANSFER LAB |
| `o-m.utf16le.xml` | company marks | BRIDGE OUTSTANDINGS LAB |
| `o-c1.utf16le.xml` | census, 20 April 2025 to 30 June 2026, no AlterID filter | BRIDGE OUTSTANDINGS LAB |
| `o-c2.utf16le.xml` | the same census with `$AlterID > 0 AND $AlterID <= 21` (re-rendered 27 Oct for the mark book O then read; only that literal changed) | BRIDGE OUTSTANDINGS LAB |

The spanned files are rendered for the marks these books last showed: 5 for book T, and 21 for book O (it read ALTVCHID 21, not 20, on 27 Oct, and `o-c2` was rendered again for 21). A census spanned to any other mark is not what the product would send, so a different mark is a STOP for that book (step 2), never a reason to send `c2` anyway.

**RULES:**
- **Order:** BRIDGE TRANSFER LAB is read here before item #1231's reconciliation (`lab/1231-bank-reconciliation-run-sheet`): that item sets bank dates on this book and keeps them, which was seen to move a voucher's AlterID, so after it this book's voucher mark is no longer 5.
- One book loaded at a time, and only that book: BRIDGE TRANSFER LAB (the five hand-keyed Contras of 3 to 5 April 2026, ledgers `TLab Bank A` and `TLab Bank B`), then BRIDGE OUTSTANDINGS LAB. Port 9001 only. Never touch a client company. If a book is not on the lab PC, skip it and report that; do not create or re-key it.
- Before each book: ComplyEaze Bridge's `tally_status` shows a licensed release (not Education) and exactly that one company loaded. Record the release and edition.
- One request at a time; never abandon one; no `--max-time`. STOP on any dialog or message box on Tally's screen (press no key, send nothing more), a second company loaded, Education mode, or 10 minutes without an answer.
- Send with exactly: `curl.exe -sS --data-binary "@<file>" -H "Content-Type: text/xml; charset=utf-16" -o "evidence\<name>.response" -D "evidence\<name>.headers" -w "%{http_code} %{size_download}\n" http://localhost:9001/` and record the exit code, the `-w` line and the ISO time of each send in `evidence\log.txt`. A non-zero exit or a non-200 answer is a transport failure: STOP, and report it as a failure, never as "no rows".
- Evidence goes in `C:\bridge-lab\899\evidence\`; zip it and report its SHA256. Do not trim or re-save any response.
- **The build for `tally_status`:** the build staged at the lab as `w1-2306d86d`, `bridge_mcp.exe` sha256 `92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524` (hash it first; another hash = STOP), over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, a new empty `BRIDGE_AGENT_DATA_DIR` and `BRIDGE_TERMS_ACCEPTED=true`; posting, import and redaction unset. It is used for `tally_status` only.

**STEPS (book T = BRIDGE TRANSFER LAB, then the same for book O = BRIDGE OUTSTANDINGS LAB with the `o-` files)**
1. Load the book alone; `tally_status` as above.
2. **Marks before.** Send `t-m.utf16le.xml` (name `t-m1`). Report the company row's `ALTVCHID` and `ALTMSTID` exactly as sent (an absent element is reported as absent). **If `ALTVCHID` is not 5 (book O: not 21), STOP for this book**: report both marks and go to the next book; the spanned request will be rendered again for the mark reported.
3. **Census as master sends it.** Send `t-c1.utf16le.xml` (name `t-c1`).
4. **Census as a span.** Send `t-c2.utf16le.xml` (name `t-c2`).
5. **Marks after.** Send `t-m.utf16le.xml` again (name `t-m2`). Report both marks. If either differs from step 2, the book changed during the read: report it; the pair is void.
6. **Compare.** Report whether `t-c1.response` and `t-c2.response` are byte-identical (`fc /b`, or equal SHA256). If they differ, report the first differing offset and both sizes; do not summarise a difference as "the same".
7. **Count by structure, not by text.** For each census response, decode it as UTF-16LE and count the `VOUCHER` elements with an XML parser (for example PowerShell `[xml]`, counting the nodes named exactly `VOUCHER`), and list each one's `ALTERID`. A substring count of `<VOUCHER` also matches `<VOUCHERNUMBER>` and is not accepted.

**PASS / FAIL**
- PASS for a book: its four answers received in full (HTTP 200, complete XML), the marks equal before and after, and steps 6 and 7 reported. The sheet passes whatever the comparison shows, including answers that differ: that is also an answer.
- FAIL: a transport failure, a dialog, a changed mark, or a count made by substring.

**REPORT, one message:** per book, the release and edition; the marks of steps 2 and 5; for each response its size, SHA256 and `VOUCHER` count with the AlterIDs; the result of step 6; any STOP; the evidence zip SHA256. The responses and headers, unedited, go on a NEW branch `lab/899-spanned-census-answers` under `answers/`, with their SHA256 list, for the fixtures and their provenance: send its SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes. Nothing is committed to this branch.
A command that errors is reported as an error, never as "absent" or "unchanged".
