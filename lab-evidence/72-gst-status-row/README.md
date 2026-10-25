# Optional lab item after the 7.2 reads: does opening the GST Annual Computation report change what a Bridge read returns? (a WRITE item)

Why: on TallyPrime 7.2 the lab reported that opening Gateway of Tally, then GST Annual Computation, asked to update the GST status of 20 vouchers; answering Yes changed stored data while `ALTVCHID` and `ALTMSTID` stayed the same and only a `CMPINFO` counter moved. This item measures what a Bridge read returns before and after, on one book. It is NOT part of the read-only sheet in `lab-evidence/72-reads/`: opening the report writes. It runs only after that sheet, only on the word of the requester, and every screen action is done by the person at the lab computer, one at a time.

## Company and rules
- Company: `BRIDGE PILOT LAB` only (the synthetic book of the earlier uncertain-voucher captures), loaded ALONE. Before anything, send `requests/c0-company-list`: STOP if any other company is loaded or the company is not listed; record its identifier. TallyPrime 7.2 only; STOP on another release or on Education mode.
- Window and arguments of the tool: the voucher type `Sales`, 1 to 31 August 2026 (the window of the 13 October run on `lab/1342-w21d-answers`), the built tool of the release-candidate build, posting OFF.
- One request at a time, from one sender with no client timeout; never cancel, retry or re-send a request in flight; the Gateway of Tally shows no dialog before each call; STOP on a dialog these steps do not describe.
- Save every answer whole with its HTTP status and headers, size and sha256. A failed call is reported as failed.
- `requests/c1-marks-TEMPLATE` (placeholders `@@COMPANY@@` and `@@GUID@@` only) is the marks read; keep every answer, including its `CMPINFO` block.

## Steps (the person at the lab computer does 1, 6, 9 and 12; the requester says when)
1. Backup `BK-G` of the data of `BRIDGE PILOT LAB`, into a NEW folder, with its name, size and sha256.
2. `c0-company-list` (the proof above), then the marks read M0.
3. Tool call A: `gst_uncertain_vouchers` for the company, voucher type `Sales`, 1 to 31 August 2026. Save the whole result (the counts and every listed voucher with its status).
4. Marks read M1. It must equal M0, including the `CMPINFO` counters.
5. Wait for the requester's word.
6. On the word only: open the report GST Annual Computation from Gateway of Tally. If a prompt to update the GST status of vouchers appears, say how many vouchers it names and answer Yes; if none appears, say so. Screenshot. Return to the Gateway of Tally with no dialog open.
7. Marks read M2 (and its `CMPINFO` block).
8. Tool call B: the same call as A, same book, same arguments. Save the whole result.
9. Marks read M3, equal to M2.
10. Compare, in writing and without judging: M0, M2 (do `ALTVCHID` and `ALTMSTID` differ; which `CMPINFO` counters differ); A against B (count of vouchers per status; any voucher whose status differs, by voucher number only).
11. `c0-company-list` again.
12. Restore `BK-G`, then marks read M4: the marks and the result of a third call C of the same tool call must equal A's. If not, STOP and report.
Not a pass or fail: the answer is what changed. The step 6 screen action writes; a restore (step 12) is part of the item, and the item is not run twice on the same book without a fresh backup.

## Not measured by this item
Release 7.1; other report screens; whether the update changes anything other than the GST status; a client book (never used).
