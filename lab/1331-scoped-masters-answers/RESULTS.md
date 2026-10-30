# 1331: an invoice's five ledgers read by GUID or by name answer exactly those rows at 18, 10,018 and 20,018 ledgers in under 0.35 s; the whole catalogue grows to 24.7 MB (Part of #1331)

Sheet `lab/1331-scoped-masters` at 09ce612e (LAB-COMMON v3). BRIDGE PILOT LAB, GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`. TallyPrime 7.2 Silver. 30 Oct 2026, 19:15 to 19:38 UTC. Every request was sent raw by curl; no ComplyEaze Bridge build was used.

**The sheet ran: PASS.**
- Part A answered whole.
- BK0 and BK1 are recorded.
- All 40 imports created 500 ledgers each, with every other counter 0.
- The S10 and S20 reads answered whole.
- The restore check is byte-identical.

## Part A: the book as it is (18 ledgers)
- **Marks:** A-m0 and A-m1 are both 245 / 68. NUMLEDGERS 18, NUMGROUPS 29.
- **The catalogue** holds 18 rows. The five invoice ledgers have the sheet's GUIDs and parents, and no name begins `Lab Scale Debtor`.

| step | request | rows | bytes | s | result |
|---|---|---:|---:|---:|---|
| A4 | c-master-whole | 18 | 38,278 | 0.139 | control |
| A4 | c-balance-whole | 18 | 25,480 | 0.051 | control |
| A4 | p1 (four parents) | 13 | 63,016 | 0.068 | = the catalogue's ledgers under the four parents |
| A5 | g1 (`$GUID`, five) | 5 | 19,730 | 0.057 | the five, each row equal to its p1 row |
| A6 | g2 (four GUIDs + an absent one) | 4 | 17,282 | 0.035 | the four, each equal; the absent GUID adds nothing |
| A7 | g3 (absent GUID only) | 0 | 2,994 | 0.028 | a complete envelope with an empty `COLLECTION` |
| A8 | g4 (master, `$GUID`, five) | 5 | 12,986 | 0.026 | each equal to its c-master-whole row |
| A8b | g5 (balance, `$GUID`, five) | 5 | 9,204 | 0.021 | each equal by NAME to its c-balance-whole row |
| A9 | n1 (`$Name`, five) | 5 | 19,730 | 0.170 | byte-identical to A5's answer |
| A10 | n2 (`BRIDGE WALK-IN`) | 1 | 5,712 | 0.028 | one row, `BRIDGE Walk-in`: **`$Name` folds case** on this request |
| A11 | n3 (absent name) | 0 | 2,994 | 0.032 | byte-identical to A7's answer |
| A12 | p2 (three parents) | 12 | 60,302 | 0.038 | = the catalogue's ledgers under the three parents |

- "Equal" means: the same element paths with the same values, compared by parsing both answers (`compare-A.txt`).
- No answer held a `LINEERROR`. Every capture after a new shape showed the Gateway with no dialog.

## Part B: grown to 10,018 and 20,018 ledgers
- **BK0:** 90,051 B, sha256 4a47fe92…, taken at 00:50 local on 31 Oct.
- **The imports:** 40 × 500 plain debtor ledgers, each import sent once. Each answered `CREATED` 500, every other counter 0, no `LINEERROR`, in 9.4 to 37.2 s (the first two were the slowest).
- **The marks** rose by exactly one per ledger: 245, then 10,245, then 20,245. ALTVCHID stayed at 68.

| read | 18 ledgers | 10,018 ledgers | 20,018 ledgers |
|---|---|---|---|
| NUMLEDGERS | 18 | 10,018 | 20,018 |
| catalogue (whole) | 25,010 B, 0.049 s | 12,365,018 B, 1.21 s | 24,705,018 B, 2.02 s |
| groups (whole) | 55,954 B, 0.073 s | 55,962 B, 0.017 s | 55,962 B, 0.008 s |
| g1 `$GUID` five | 19,730 B, 0.057 s | 19,738 B, 0.109 / 0.123 s | 19,738 B, 0.306 / 0.259 s |
| n1 `$Name` five | 19,730 B, 0.170 s | 19,738 B, 0.049 s | 19,738 B, 0.117 s |
| g3 absent GUID | 2,994 B, 0.028 s | 3,002 B, 0.042 s | 3,002 B, 0.072 s |
| g4 master `$GUID` five | 12,986 B, 0.026 s | 12,994 B, 0.115 s | 12,994 B, 0.243 s |
| g5 balance `$GUID` five | 9,204 B, 0.021 s | 9,212 B, 0.133 s | 9,212 B, 0.319 s |
| p2 three parents | 60,302 B, 0.038 s | 60,310 B, 0.056 s | 60,310 B, 0.082 s |

- **The filtered answers do not depend on the book's size.** At both sizes every filtered answer equals its Part A answer outside `CMPINFO`. They are not byte-identical: `CMPINFO`'s `LEDGER` count line reads `10000` instead of `0`, which adds 8 bytes. Those are Tally's in-memory counts, not rows.
- **The GUID filter's time grows with the book,** from 0.06 s at 18 ledgers to about 0.3 s at 20,018. The name filter stayed under 0.2 s.
- **The whole catalogue grows** at about 1,234 B per added ledger.
- At every size, the 18 original catalogue rows are unchanged.
- **The gate at B2 passed:** the catalogue was 12.4 MB in 1.2 s, and no read took over 120 s.
- c-master-whole, c-balance-whole and p1 were never sent after the first import.
- **BK1, the grown book:** 946,410 B, sha256 862404d6…, taken at 01:04 local. It is **kept** on the lab PC for a later sitting.

## Restore
- BK0 was restored on Tally's screen under C6, with the plan in `restore-plan.txt`.
- The two expected prompts appeared: Accept, then Replace for THE COMPANY's own number.
- **The restore check is byte-identical:** R-m (245 / 68) equals A-m1, R-k equals A2's catalogue, and R-g1 equals A5's answer.
- At the close, BRIDGE PILOT LAB was shut, and one Tally owns 9001.

## Notes
- The Gateway of this book shows "(e) Data exceptions exist". It is a Gateway line, not a dialog; recorded.
- The capture after i003 was not byte-equal to the reference capture. It was looked at before anything more was sent: it showed the Gateway with no dialog, with the menu highlight grey because the window had lost focus. It was then used as a second reference.
- The catalogue answers at 10,018 and 20,018 ledgers (12.4 MB and 24.7 MB), and their row tables, stay on the lab PC, listed by name, bytes and sha256 in `kept-local.txt`. Screenshots and both backups also stay on the lab PC.
