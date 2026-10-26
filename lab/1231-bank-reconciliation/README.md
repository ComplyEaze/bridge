# Lab run sheet: what Tally sends for a RECONCILED bank entry (#1231, third read). SYNTHETIC COMPANY ONLY. Not for merge.

About 30 minutes. Reconciling on screen plus three reads; ComplyEaze Bridge writes nothing.

**WHAT IT DECIDES:** issue #1231 asks for the bank-reconciliation status of bank entries (bank date, instrument, status), "which already arrives in voucher responses but is not parsed". The committed capture of five hand-keyed Contras (BRIDGE TRANSFER LAB, `h-contra.response` on branch `lab/transfer-1392-requests`) shows every `BANKALLOCATIONS.LIST` with `BANKERSDATE` empty and `STATUS` `No`, because nothing was reconciled. Nothing in the repository shows a reconciled entry, so it is not known which element carries the reconciliation, what it reads when reconciled and when cleared, or whether the voucher's `ALTERID` and the company's marks move. This sheet captures that, so the read can be built on a real answer (AGENTS.md P1) and not on a guess.

**EXPECT (an expectation, not a fact):** `BANKERSDATE` of the reconciled entry reads the bank date typed on the screen (`20260406`) and the unreconciled entries stay empty; `STATUS` may or may not change. Report what happened, not what is expected.

**RULES:**
- Company "BRIDGE TRANSFER LAB" only (the synthetic book of the #1392-H run: ledgers `TLab Bank A` and `TLab Bank B`, five Contras of 50,000.00, 3 to 5 April 2026). Load only it. Port 9001 only. Never touch a client company.
- Check first that the five Contras are there and none is reconciled (step 1 shows it). If the company is not loaded, or the Contras are not there, STOP and report; do not re-key them from memory.
- One request at a time; never abandon one; no `--max-time`. STOP on a Tally dialog you did not expect, a second company loaded, Education mode (check with ComplyEaze Bridge's `tally_status`), or 10 minutes without an answer.
- Reconciliation is the only change made to the book, and only the three below. Do not alter, delete or re-save any voucher. Do not run Auto Reconciliation or import a bank statement.
- Evidence goes in `C:\bridge-lab\1231\evidence\`; zip it and report its SHA256. Screenshots of every screen named below.

**STEPS**
1. **Control read (before any change).** Check `V-CONTRA.utf16le.xml` against `SHA256.txt` with `Get-FileHash -Algorithm SHA256`. Send it (the same request the #1392-H run sent):
   `curl.exe -sS --data-binary "@V-CONTRA.utf16le.xml" -H "Content-Type: text/xml; charset=utf-16" -o "evidence\r0.response" -D "evidence\r0.headers" -w "%{http_code} %{size_download}\n" http://localhost:9001/`
   Record the exit code, the `-w` line and the ISO time in `evidence\log.txt`. A non-zero exit or a non-200 answer is a transport failure: STOP. Report, per voucher (VOUCHERNUMBER, DATE, GUID, ALTERID) and per `ALLLEDGERENTRIES.LIST`, the value of every child of its `BANKALLOCATIONS.LIST` that is not empty, and the values of `BANKERSDATE`, `STATUS` and `PAYMENTMODE`. Read the company's marks (`ALTVCHID`, `ALTMSTID`) with `tally_status` or the company-marks read ComplyEaze Bridge uses, and record them.
2. **Reconcile two entries on screen.** Banking > Bank Reconciliation, ledger `TLab Bank B`, 1 April to 30 April 2026. Put the bank date `6-Apr-2026` on the entries of Contra 1 (3 Apr, the narration reference) and Contra 2 (3 Apr, the instrument number `900000000002`) **on TLab Bank B only**. Leave Contra 3, 4 and 5 and every entry of `TLab Bank A` untouched. Screenshot the reconciliation screen before and after, and the "Reconciled Balance" and "Unreconciled" lines.
3. **Read after reconciling.** Send the same request again (`evidence\r1.response`, same options). Record the marks again. Report, per voucher and entry, which children of `BANKALLOCATIONS.LIST` differ from `r0`, with both values, and which voucher-level elements differ (ALTERID, anything else). Report the entries of Contra 1 and 2 on `TLab Bank A` separately: they were not reconciled.
4. **Clear one.** On the same screen clear the bank date of Contra 1's entry on `TLab Bank B` (back to unreconciled). Screenshot. Send the request again (`evidence\r2.response`). Report whether `BANKERSDATE` is empty or absent again, every other element that changed from `r1`, and the marks.
5. **Compare byte for byte.** For every voucher not touched in steps 2 and 4 (Contra 3, 4, 5), report whether `r0`, `r1` and `r2` carry identical bytes for it (`fc` or a hash of the voucher's element), and for the touched ones the exact set of differing elements. Do not summarise a difference as "the same": list the elements.
6. **Read through ComplyEaze Bridge (optional, only if time allows).** `vouchers` for 2026-04-01 to 2026-04-05, `voucher_type` Contra, after step 3 (before step 4's clearing): save the answer verbatim. The tool does not parse the bank date today; the point is to see what it shows.

**PASS / FAIL**
- PASS: `r0`, `r1` and `r2` all received in full (HTTP 200, `RESPSTATUS` 1, complete XML) and every difference above reported by element name and value. The sheet passes whatever the differences are, including "nothing changed in the allocation": that is also an answer.
- FAIL: a transport failure, a dialog, a changed voucher outside steps 2 and 4, or a difference summarised without naming its elements.

**REPORT, one message:** the control values of step 1; the marks before and after each step; for each of steps 3, 4 and 5 the element-by-element differences; the screenshots' names; any STOP; the evidence zip SHA256.
A command that errors is reported as an error, never as "absent" or "unchanged".
