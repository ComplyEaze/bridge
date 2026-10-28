# C1 (#2062): one Journal sent in the native shape, then cancelled by the candidate renderer's request, then the book restored

Not for merge. `LAB-COMMON.md` (v3) and `README.md` in this folder apply in full; where this row and they differ, the stricter holds. Steps marked [OWNER]: see WHO ACTS in `README.md`. The row runs only after the run coordinator has given a recorded go-ahead for this branch head.

## THE COMPANY
BRIDGE SHAPE LAB (synthetic), as the earlier rows left it; its period is 1 April 2026 to 31 March 2027. Only THE COMPANY is loaded, alone, from step 0 to the shut. If it is absent: STOP; never create a substitute. Licensed mode only; Educational mode anywhere = STOP. The row adds two ledgers and one Journal dated 15 June 2026, and the restore at step 5 removes all three. No period, date or feature setting is changed.

## Step 0
1. Check `SHA256.txt`: every listed file here equal in hash and bytes, the two request files included. Any difference = STOP.
2. LAB-COMMON C2 step 0 with THE COMPANY loaded alone, without its MARKS request (its GET /status checks stay). Hash the exe.
3. `list_companies`: exactly THE COMPANY, `identity_state` `verified_tuple`; record its GUID. MARKS: M0, V0.
4. `vouchers` for 15 June 2026: EXPECT no row; any row = STOP.
5. Backup BK-CX (C5) into a NEW empty folder `C:\bridge-lab\2062\backups\bk-cx\`: name, size, time and sha256. Then MARKS: Mb, Vb (a difference from M0, V0 is recorded, not a STOP).

The book: unchanged.

## Step 1: two ledgers, on Tally's screen
1. [OWNER] Create > Ledger `SYN CX Expense`, Under `Indirect Expenses`, every other field as offered. Screenshot before Ctrl+A.
2. [OWNER] Create > Ledger `SYN CX Party`, Under `Sundry Creditors`, Maintain balances bill-by-bill **No**, no GSTIN, every other field as offered. Screenshot before Ctrl+A.
3. Leave the Create screen as C7 says (a screenshot of its title, then Ctrl+Q) and screenshot the Gateway. A prompt or dialog other than the screen's own accept = C7, STOP.
4. MARKS: M1, V1. Record M1 − Mb (two ledgers are expected to move M; the figure is recorded, not judged) and EXPECT V1 = Vb.

The book: plus the two ledgers.

## Step 2: the Journal, in the native post's shape
1. MARKS: M2, V2 (the read just before the write, C3). EXPECT M2 = M1 and V2 = V1.
2. Send `requests/w1-create.xml` once (C3; Get-FileHash first). Record the answer whole (C4). EXPECT `CREATED` 1 and `ALTERED`, `DELETED`, `COMBINED`, `IGNORED`, `ERRORS`, `CANCELLED` and `EXCEPTIONS` 0, with no `LINEERROR`; record `LASTVCHID` and `LASTMID`. Anything else = STOP: the cancel is not sent.
3. Screenshot the whole Tally window (C3).
4. `vouchers` for 15 June 2026: EXPECT one row, `voucher_type` Journal, `narration` `CX-1`, `cancelled` false, `optional` false, amounts 10.00 on `SYN CX Expense` (debit) and `SYN CX Party` (credit). Record its `guid` (G), `master_id` (MID) and `alter_id` (A1). None, several or other values = STOP.
5. MARKS: M3, V3. Record V3 − V2 (EXPECT 1) and M3 − M2.

The book: plus the Journal `CX-1`.

## Step 3: the cancel (NEW SHAPE)
1. MARKS: M4, V4. EXPECT M4 = M3 and V4 = V3.
2. Send `requests/w2-cancel.xml` once (C3; Get-FileHash first). It is never sent again, whatever its answer.
3. Screenshot the whole Tally window at once (C3, a NEW SHAPE). A dialog there = C7: nothing further is sent.
4. Record the answer whole, every counter present or absent, `LASTVCHID`, `LASTMID` and every `LINEERROR`. This step replaces C4's import EXPECT: the answer §9.14 observed for a cancel is `ALTERED` 1 and every other counter 0, with no `LINEERROR`. Record whether this answer is that one. Any other answer is a FINDING: record it whole and do step 4's reads, then report before step 5.
5. Nothing else is sent until step 4.

The book: the Journal `CX-1` cancelled, if the cancel took effect; otherwise as after step 2.

## Step 4: the readback
1. `vouchers` for 15 June 2026. Record every row whole. EXPECT, as §9.14 observed (once):
   - exactly one row, with `guid` G and `master_id` MID;
   - `cancelled` true;
   - no amount on either ledger (an empty entry list);
   - `alter_id` above A1.
   Each of these is recorded as held or not held. A second row, or no row, is a FINDING.
2. MARKS: M5, V5. Record V5 − V4 and M5 − M4 (the cancel's mark step is not established; it is recorded, not judged).

## Step 5: restore BK-CX
1. On the run coordinator's word RESTORE only: C6, restoring BK-CX as recorded at step 0.5.
2. After it: MARKS (EXPECT M equal to Mb and V equal to Vb), `vouchers` for 15 June 2026 (EXPECT no row). Otherwise STOP: BK-CX is the only backup, and a restore that does not complete is a STOP.

The book: as at step 0.

STOP RULES: C7 and C8; any dialog or prompt in Tally; a mark that moves outside a step that changes it; a voucher count other than expected. A file that writes is sent once in the row's life (C3).

## Close and evidence
Before C10, check that Tally's current date is the one recorded at step 0. Then C10: Shut Company; no company loaded.

Evidence goes on a new branch `lab/2062-cancel-capture-answers` with `SHA256.txt`, after the pre-push scan and the run coordinator's yes:
- the two request files as sent, with their sha256, and both import answers whole (the raw bytes as received, with size and sha256, and CONTENT-LENGTH);
- every tool answer whole;
- the marks at each step and the differences named above;
- the voucher rows of steps 0.4, 2.4, 4.1 and 5.2;
- BK-CX's name, size and sha256, and the build's hashes;
- a notes file with UTC times, one line per step.

Screenshots and BK-CX stay on the lab PC, listed by name, size and sha256.

## Not covered
- A delete: its request body is not established (#2062 waits on a maintainer comment for it).
- Any voucher type other than Journal; a voucher posted by `post_import`; a batch of more than one voucher; Gold.
- Whether a later Create with the same REMOTEID un-cancels the voucher (§9.14 observed it once); it is not sent here.
