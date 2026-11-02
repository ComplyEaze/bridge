# Results: #2176 N1, a ledger create abandoned during a busy import, BRIDGE ABANDON LAB, TallyPrime 7.2 Silver, 2 Nov 2026

**Both counted rows are PASS-present: with Tally busy on a long import, a ledger create abandoned at 20 s ran, and the read sent after it, answered only once the import ended, saw it. N1 did not contradict the order rule for this pattern. Two repeats on one book: PARTIAL by the sheet's own convention for N1.**

Sheet: `lab/1713-settle-order` at 869bca9e, `order.py` sha256 858c7acf... equal to its `SHA256.txt`. N1 had never run: the row note found no bulk file of the earlier sheet that held the gateway 60 s.

| row | Journals in A | D | B written | B end | C written | A first byte | C first byte | C | +1 | +5 | +15 | +30 | marks M before, after | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| N1a | 1200 | 29.73 s | 5.02 | abandoned 25.03 | not sent | 30.84 | - | no answer | present | present | present | present | 226, 227 | **VOID** |
| N1a-2 | 2700 | 56.13 s | 5.03 | abandoned 25.05 | 26.00 | 57.14 | 57.41 | present | present | present | present | present | 227, 228 | **PASS-present** |
| N1b | 2700 | 56.39 s | 5.03 | abandoned 25.05 | 26.02 | 57.41 | 57.66 | present | present | present | present | present | 228, 229 | **PASS-present** |

Times are seconds after T0, from `events.jsonl`. D is A's send to A's first byte.

- **What counts, for N1a-2 and N1b:** B written and abandoned; C written before A's first byte; C's first byte after A's first byte; D at least 55 s; C answered. Each A answered with its Journals created (`import ok CREATED 2700`). Observation reads are each at C answered + 1, 5, 15 and 30 minutes. [verified]
- **The mark moved by exactly one** in each counted row, for the one ledger, and each probe ledger is listed once. [verified]
- **N1a is VOID, for two causes:**
  1. D 29.73 s, under the 55 s floor, with 1,200 Journals.
  2. The row's dialog watcher created the stop file at about T0+19, 14.68 % changed, so C was never sent. Its capture shows no dialog: the Gateway with its title bands grey, because Tally lost focus after a blue baseline. A false alarm.

  B still ran in N1a (present in every observation, the mark up by one), but with no C it says nothing of the order rule.

## Local changes, named
All in this row's own copies; the 1713 kit's files are unchanged.
- **The watcher copy (`scripts/watch.ps1`):** the baseline is taken 4 s after the row starts, once focus has settled, and the comparison leaves out the window's top 132 rows. In N1a-2 and N1b it read 0 % throughout.
- **The runner copy (`scripts/rows_n1.py`):** the K rows' runner with step A as a seeded import and the working folder changed.
- **Presence is read by `orderread.py`, not `read.py`.** `read.py` lists only `Lab Probe` names, so its line in `evidence/rows-log.txt` reads `probe=absent` for every `Lab Order` name, and the live VERDICT lines there are NOTE or wrong for that reason. `evidence/times-final.txt`, from `scripts/verdict.py`, holds the verdicts that stand.

## Deviations from the sheet
1. **A is a seeded file**, not a bulk file of the earlier sheet: on the run lead's word, since none held 60 s. Built by the earlier sheet's own `gen.py` builder, with REMOTEIDs past those in the book. Seeded rows are set-up here, not evidence of their own shape. The seeds are not pushed: their sha256 are in `seeds-kept-local.txt`.
2. **The count was changed once** from 1,200 to 2,700, on the run lead's word, after N1a; nothing else changed.
3. **The sheet's "repeat under `<row>-2`":** N1a's repeat is `Lab Order N1a-2`; N1b is the second counted row.
4. **D of the busy read was not measured again after N1:** no timed row follows it.
5. **Close:** no restore, on the run lead's word.

## The book now
BRIDGE ABANDON LAB holds 6,600 Journals more than before this row (54,600 by the mark, from 48,000) and three more ledgers, `Lab Order N1a`, `N1a-2` and `N1b`. M 229. Two backups were taken by Tally's screen, one before N1a and one before N1a-2; they stay on the lab PC, listed by sha256 in `backups-kept-local.txt`. Close: company shut, Select Company with none loaded, one tally.exe owns 9001, `GET /status` 200.

**Confidence:** two repeats, one book, Silver 7.2, ledger creates only, one busy pattern (an import, not a read): PARTIAL. Not covered: Gold, other releases, other master types, a create on a reused connection during an import. D sat about 1 s over the floor both times.
