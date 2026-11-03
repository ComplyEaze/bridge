# Results: #2176 N2, a ledger create abandoned on a warmed connection behind a long import, BRIDGE ABANDON LAB, TallyPrime 7.2 Silver, 3 Nov 2026

**Both counted rows are PASS-present: with another client's long import in front, a ledger create abandoned at 20 s on a warmed, reused connection ran, and the read sent after it, answered only once the import ended, saw it. No FAIL, no dialog. One row was VOID on D and its repeat counted. Two repeats on one book: PARTIAL by the sheet's own limits.**

Sheet: `lab/2176-import-sheet` at 13c8c9a7. No build. `order.py` sha256 858c7acf..., as the sheet names it. N1's runner at sha256 0820f3eb... as pushed; the runnable copy equals it apart from the words and paths N1's stage replaced, and adds the one line `VARIANT['N2'] = ('K1', 'new', 20)`. [verified]

| row | Journals in A | D | K1 | B written | B end | C written | A first byte | C first byte | C | +1 | +5 | +15 | +30 | marks M before, after | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| N2a | 2700 | 58.67 s | warmed, reused | 5.02 | abandoned 25.02 | 26.03 | 59.69 | 59.97 | present | present | present | present | present | 233, 234 | **PASS-present** |
| N2b | 2700 | 54.36 s | warmed, reused | 5.05 | abandoned 25.05 | 26.05 | 55.38 | 55.75 | present | present | present | present | present | 234, 235 | **VOID** |
| N2b-2 | 2700 | 55.34 s | warmed, reused | 5.00 | abandoned 25.00 | 26.05 | 56.39 | 56.78 | present | present | present | present | present | 235, 236 | **PASS-present** |

Times are seconds after T0, from `events.jsonl`. D is A's send to A's first byte. Verdicts are `verdict.py`'s, in `evidence/times-final.txt`.

- **What counts, for N2a and N2b-2:** `warmed` for K1 and `reused` for B on K1; B written and abandoned; C written before A's first byte; C's first byte after A's first byte; C answered; D at least 55 s. Each A answered `CREATED 2700`, by the counters. [verified]
- **The mark moved by exactly one** in each row, for the one ledger. V rose by 2,700 each. Each probe ledger is listed once at the last read of N2b-2, and the ledgers the row added are its three names and no others. [verified]
- **N2b is VOID for one cause:** D 54.36 s, under the 55 s floor. Every other item held, and its create was present in C and in all four reads, but it does not count. It was repeated once as `N2b-2` with the count unchanged, as the sheet says.
- **The dialog watcher** read 0 % throughout all three rows, no change. Tally showed no dialog. [verified]
- **The runner's live VERDICT lines are not the verdict.** They read `C absent` and NOTE, because `read.py` lists only `Lab Probe` names, as N1 RESULTS says. `verdict.py` reads presence by `orderread.py`.

## Seeds
The sheet's ranges, N2a from 107800 and N2b from 110500; N2b-2 from 113200, past N2b's. 2,700 Journals each by the same builder. sha256 in `evidence/seeds.txt`.

**The range check differs from the sheet's wording.** Tally does not keep these seeds' REMOTEIDs: all 54,600 vouchers read back GUID-based ones, and a filter on the seed prefix answered empty. So the check read every voucher's narration, which carries the builder's index. Before N2a: 54,600 indexes, highest 107799, N1's own 2,700 found as the control, 0 inside 107800 to 113199. Before N2b-2: 60,000 distinct, highest 113199, 5,400 found as the control, 0 inside 113200 to 115899. `evidence/ranges.txt`, `evidence/ranges-2.txt`. [verified]

## Decides
The sheet's first line: N2a and a repeat of N2b both count and pass, so K1 is measured with a read and with an import in front. The sheet's text change for `NEXT_ABSENT` and the verdict's note is not made here.

## Limits
- Two counted repeats, one book, Silver 7.2, ledger creates only.
- N2b-2's D stood 0.34 s over the floor, and D fell under it once, so the floor is close at this count.
- The warmed connection served one read and sat about 5 s idle. Bridge's had served about 70 and its idle time was not reported. This row does not test whether that differs.
- Not covered: the restart row, Gold, other releases, other master types.

## The book now
BRIDGE ABANDON LAB holds 8,100 Journals more than before this row (V 62,700, from 54,600) and three more ledgers, `Lab Order N2a`, `N2b` and `N2b-2`, 29 ledgers, M 236. N2b's ledger and Journals stay in the book. A backup was taken by Tally's screen before N2a, 15,099,081 B, kept on the lab PC, hash in `backups-kept-local.txt`. No restore. Company shut, no company loaded, one tally.exe on 9001.

**Confidence:** PARTIAL.
