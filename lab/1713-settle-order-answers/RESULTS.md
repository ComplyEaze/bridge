# Results: #1713 / #2176 order rows on BRIDGE ABANDON LAB (synthetic, TallyPrime 7.2 Silver), 29-30 Oct 2026 (UTC). **Every timed row that ran counts, and every one is PASS-present: K0a, K1a, K2a, K3a, K0b, K1b, K2b and K3b (a ledger create abandoned at 20 s, on new and on reused connections), and the L1B order control again (PASS-present). In every row the create had run before C was answered: the probe ledger was present in C and in every observation read at +1, +5, +15 and +30 minutes, and the master mark stepped by exactly one per row, the new ledger at the next ALTERID. No FAIL, no PASS-dropped, no VOID. By the sheet, the order rule may be cited for K0, K1, K2 and K3 (both repeats passed). R0 not run (its build's download waits for the owner's OK); N1 not run (the row note: no earlier import held the gateway 60 s); the optional restart row not run (D under 120 s from K3a on).**

## Rows (verdicts by `recompute.py`, from the saved answers; `evidence/times-final.txt` has the full line of each)
| row | B on | C on | D (A sent to A's first byte) | B | C written / C first byte / A first byte (s after T0) | probe in C | +1 / +5 / +15 / +30 min | mark before B -> after | the probe's ALTERID | verdict |
|---|---|---|---|---|---|---|---|---|---|---|
| K0a | new | new | 173.719 s | abandoned at 20 s | 26.016 / 176.047 / 174.75 | present | present / present / present / present | 217 -> 218 | 218 | **PASS-present** |
| K1a | K1 | new | 173.531 s | abandoned at 20 s | 26.047 / 175.25 / 174.562 | present | present / present / present / present | 218 -> 219 | 219 | **PASS-present** |
| K2a | new | K2 | 169.094 s | abandoned at 20 s | 26.032 / 170.797 / 170.094 | present | present / present / present / present | 219 -> 220 | 220 | **PASS-present** |
| K3a | K1 | K2 | 63.562 s | abandoned at 20 s | 26.016 / 64.938 / 64.625 | present | present / present / present / present | 220 -> 221 | 221 | **PASS-present** |
| K0b | new | new | 64.046 s | abandoned at 20 s | 26.015 / 65.328 / 65.046 | present | present / present / present / present | 221 -> 222 | 222 | **PASS-present** |
| K1b | K1 | new | 66.078 s | abandoned at 20 s | 26.0 / 67.672 / 67.109 | present | present / present / present / present | 222 -> 223 | 223 | **PASS-present** |
| K2b | new | K2 | 62.735 s | abandoned at 20 s | 26.031 / 64.375 / 63.797 | present | present / present / present / present | 223 -> 224 | 224 | **PASS-present** |
| K3b | K1 | K2 | 60.672 s | abandoned at 20 s | 26.047 / 62.093 / 61.687 | present | present / present / present / present | 224 -> 225 | 225 | **PASS-present** |
| L1B-repeat | new | new | 61.234 s | answered (no limit) | 26.0 / 62.938 / 62.25 | present | present / present / present / present | 225 -> 226 | 226 | **PASS-present** |

- **What counts** (the sheet): B written, B abandoned (L1B-repeat: answered), C written before A's first byte, C's first byte after A's first byte, no bad end, D at least 55 s. Every row met every condition. The watcher (a capture of Tally's window every 20 s) saw no change in any row.
- **The pattern rows:** K1 sent B on a warmed connection (idle about 5 s), K2 sent C on a warmed connection (idle about 26 s), K3 both; order.py logged each `reused` with its idle seconds.
- **L1B-repeat:** B was sent with no limit and answered (`import ok CREATED 1`) at T0+62.922 s (C answered at T0+63.047 s), before C's answer: the order control of the earlier sheet, seen again.

## D: the busy read's time
Step 3 measured D 174.969 s with A alone; K0a, K1a and K2a 173.719, 173.531 and 169.094 s. From K3a on, the same read answered in about 61 to 66 s (K3a 63.562, K0b 64.046, K1b 66.078, K2b 62.735, K3b 60.672, L1B-repeat 61.234), with the same rows (`busy ok rows 48000` by structure). Nothing else was sent between rows. Recorded, not acted on; no cause is claimed. Every D stayed above the sheet's 55 s floor, so no row was repeated.

## Checks
- **Sheet:** `lab/1713-settle-order` at 869bca9e7de78aa7cef7b3496dab4bfc7f713c0e: `ORDER-ROWS.md`, `LAB-COMMON.md`, `order.py` equal to `SHA256.txt` and the row note. The earlier sheet at 811aed8b511b3af3cc855688e315ad6e9daf0361: `busy-year.xml`, `ledgers.xml`, `marks.xml`, `l2-create.xml`, `read.py` equal to the row note's sha256s. `order.py` checked every request file's sha256 before each run.
- **Step 0:** one tally.exe (PID 13252) owns 9001; `/status` 200. About: TallyPrime 7.2, Silver, licensed. One Currency master. MARKS: one row, THE COMPANY's GUID, ALTVCHID 48000, **ALTMSTID 217** (the row note says 216 after L5; recorded). LEDGERS: 12 rows, none named `Lab Order`.
- **BK0** (C5): `TDBK1800_100096.001`, sha256 abaa57234d7796f413daf7787a53069192272a57075e8b95607f8c600b552637, 00:48 local; MARKS after it unchanged. No restore was made (none planned; the probes are harmless ledgers of a synthetic company).
- **Before every row:** a capture of Tally's window, `GET /status` 200, MARKS one row, LEDGERS with the row's name absent.
- **C10:** BRIDGE ABANDON LAB shut from the Company menu; Select Company with none loaded; one tally.exe owns 9001; `/status` 200 (`evidence/c10b-*`); no XML after the shut. The files `evidence/c10-*` were taken a moment earlier, while the Shut Company list was still open, and are not the C10 check.
- **Requests:** every create B is `l2-create.xml`'s bytes with only the ledger name changed (`Lab Probe L2` -> `Lab Order <row>`, both places), recorded with its sha256 in `rows-log.txt`; each plan file is in `plans/`.

## Deviations
1. **Order of rows:** R0 is first in the sheet; it was not run, because its build's download waits for the owner's OK in the lab's chat. On the run lead's word the K rows and L1B-repeat ran first, in their order.
2. **The probe reading:** the earlier sheet's `read.py` lists only ledgers named `Lab Probe ...`, so its `ledgers <answer> <guid> <probe>` reading of a `Lab Order` name is always `probe=absent`. The driver's live verdicts (`rows-log.txt`, `times.txt`) used it and read "absent" / NOTE; they are wrong on that point. `orderread.py` reads the same answers with read.py's own `rows()` and `number()` (C4 decoding) and lists every `Lab Order` ledger; `recompute.py` gives the verdicts in `times-final.txt`. The pre-row "name absent" check is re-read the same way (absent before every row).
3. **The pre-row MARKS and LEDGERS** were sent by C3's curl line (`c3.py`); the observation reads by `order.py`, each MARKS then LEDGERS on fresh connections, one at a time.
4. **The dialog cadence** (difference 4) was kept by a script (`watch.ps1`): a PrintWindow capture of Tally's whole window every 20 s, compared with the row's first capture, the stop file made on a change over 2 % of grid points. None changed.
5. **The busy read's answers** stay on the lab PC, listed by sha256 in `answers-kept-local.txt`; so does `order.py`'s own `events.jsonl` of each run, pushed as `events-public.jsonl` with one key removed from A's end event (see that file's notes).

**Confidence:** one observation per pattern per repeat, on one book, Silver 7.2, ledger creates only: PARTIAL. Gold, other releases and other master types are not covered; nor whether ComplyEaze Bridge's own pool ever holds two idle connections at once (K2 and K3 test the possibility only).

**Who acted.** The lab, on the owner's standing word for synthetic books (the writes) and the run lead's word for the row order.
