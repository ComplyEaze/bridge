# Row 2162-S9a answers: stopped at step 2, before any write. The book holds no ledger named BRIDGE Customer RJ

Run on 9 Nov 2026, 11:26Z to 11:31Z, on TallyPrime Silver (the Gateway on 9001), by the sheet at `lab/2162-s9a-sheet` (f884b8b2), in a sitting of its own on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic). Every value below is M, measured by this run.

**Result: the sheet's step 2 stop.** "The ledger not found ... record it and stop before any write." Neither read of step 2 holds a ledger named `BRIDGE Customer RJ`, and the two reads are byte-equal. Nothing was keyed or saved: no SEZ, no Deemed Export, no restore. The company was shut by C10 on the run lead's word.

## Why the ledger is absent
Row 2162-BC created `BRIDGE Customer RJ` itself: its step 3 imported `L-seed.xml` (its C read went from 18 LEDGER rows to 19, one of them `BRIDGE Customer RJ`). It then restored `BRIDGE PILOT LAB` from the backup it had taken before that import, and its s9 C read held 18 rows again, none of them `BRIDGE Customer RJ` (`lab/2162-bc-answers`, RESULTS). This row's s1 marks are byte-equal to row 2162-BC's s1 (`8a0c4268…`). So the book this row found is the book row 2162-BC found before its seed, and it never held that ledger. ROW.md's "Book" lines read the ledger from row 2162-BC's step 3 answer (`a-s3r-K-customer.xml`), taken after the seed.

## Checks
| check | result |
|---|---|
| Sheet files against `SHA256.txt` (10 files) | PASS: all equal. `gen.py` rebuilt `K-rj.xml` byte for byte in a separate folder. |
| Step 0 of C2, no company loaded | PASS: one tally.exe, which alone owns 9001; `GET /status` 200. |
| C1, load alone | PASS: `PILOT` typed at Select Company, one row listed, Enter. No prompt. The Gateway showed BRIDGE PILOT LAB alone, current period 1-Apr-26 to 31-Mar-27, current date Tuesday 15-Sep-2026, date of last entry 15-Sep-26, and the footnote "(e) Data exceptions exist". |
| s1 `M-marks.xml` | PASS: one COMPANY row with the book's GUID, ALTVCHID 68, ALTMSTID 245 (as expected). 3,394 B, sha256 `8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6`. |
| BK (C5) | PASS: to a new empty folder, BRIDGE PILOT LAB alone, "Accept ?" answered y. One file, `TDBK1800_<company number>.001`, 90,051 B, sha256 `6463ce768caf83fdd13ecd3dd4807535f9502f41383bac43e20c04d4a94f771b`. Kept on the lab PC for the amended run. |
| s2 and s2b equal | PASS: `a-s2-C-compliance.xml` = `a-s2b-C-compliance.xml` (38,278 B, `4976bbec…`) and `a-s2-K-rj.xml` = `a-s2b-K-rj.xml` (2,994 B, `6e9e95e8…`), byte for byte (`cmp`). Nothing was pressed in Tally between them. |
| The ledger as found (step 2's EXPECT) | FAIL: C holds 18 LEDGER rows, 0 named `BRIDGE Customer RJ`. K holds no LEDGER row: STATUS 1, every CMPINFO count 0, an empty COLLECTION. `count7.py --leafdiff a-s2-K-rj.xml a-s2b-K-rj.xml` stopped: 0 rows named `BRIDGE Customer RJ`, not one. |
| Steps 3 and 4 (SEZ, Deemed Export) | not run (the step 2 stop) |
| Step 5, the restore | not needed: nothing was saved |
| C10 | PASS: Company > Shut, BRIDGE PILOT LAB, no prompt; Select Company with no company loaded; one tally.exe, which alone owns 9001. |

The 18 LEDGER rows of C include `BRIDGE Supplier RJ` and `BRIDGE Walk-in`, and no other party ledger.

## Not established here
The element that holds the party type, where it sits, and its values: steps 3 and 4 did not run. The sheet goes back to its author. The amended run adds row 2162-BC's seed import, with a before-write check, ahead of s2, and can restore from this row's BK, which predates the seed.

## Files
- `answers/`: the marks answer and the K answers, byte for byte. The C answers hold a GSTIN shape and stay local, by size and sha256 (`kept-local.txt`).
- `evidence/`: the notes (UTC), each count output, the answer list and the send times. The process and port listings stay local.
- `logs/`: `run.py`'s own logs. `scripts/`: the step 0 helper and the window capture helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.
