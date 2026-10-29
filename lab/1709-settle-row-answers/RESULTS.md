# Results: #1709 settle row S1, 29 Oct 2026. **STOPPED at step 1.2: Tally asked a question the sheet does not name. It was answered No by mouse, as the sheet's rule says; nothing was posted.**

When step 1.2 set THE TYPE (BRIDGE SHAPE LAB's `Journal`) from Automatic to Manual numbering, Tally asked:

> Changing the method of voucher numbering will affect voucher numbers of existing transactions. Continue? Yes or No

- **What was done:** the sheet names no such window, and its rule is that any window it does not name gets No by mouse and is a stop. The prompt was captured, reported to the run lead, and answered **No** on the run lead's word.
- **The type afterwards:**
  - the alteration screen went back to Automatic and was left by Quit, without saving;
  - reopened, THE TYPE read exactly as step 1.1 recorded it (captured).
- **Not run:** steps 1.3 to 7. No post was made, no batch was built, no window from the build opened, and no settlement was recorded.
- **Why it matters for a rerun:**
  - THE COMPANY holds Journals on the days around the row's date: one on each of 15 to 21 Jun 2025, seven in all. The book's total number of Journals was not counted.
  - "Yes" may renumber existing Journals. A restore to Automatic at step 7 may ask again and renumber them again, so step 7's restore alone may not return the book to its first state.
  - The run lead's note: the sheet will be amended to restore the whole book from BK0 at the end, or to use a fresh synthetic book, and the row rerun.

**Who acted.** The lab, on the owner's go-ahead for this row (in the lab's own chat and the run lead's, recorded in `lab/0.7.0-auto-order` ORDER.md row 22 at 8efd9903) and the owner's standing word on synthetic books. The sheet is `lab/1709-settle-row` at bc6b5ea8e4484fa8ae5a27dbbcec30fc4b35577c (`SHA256.txt` 2 of 2 equal).

## The build
`evidence/build.txt` holds the build's record:
- **Source:** #2150 at ed0f5fc599acaa8a21d166c4963c79f438843cdd; Bridge CI run 40834023243 (success); artifact 12880706328 `unsigned-bundle-smoke-windows-latest`.
- **The zip:** 57,101,815 B, sha256 f14786e0…, equal to the digest GitHub gives.
- **`bridge-tally.mcpb`:** 15,554,512 B, sha256 1423584995fafa25c69467c5fe6e2912e5832194d58632a9d9da73e3999f38c3.
- **`bridge_mcp.exe`:** 32,607,744 B, sha256 e95269fb9ab2d72676902891615d733e2cb13e12cc003b483afbc0c0110d9a58, equal to `mcpb-smoke.json`'s `binary_sha256`. Equal again at the end.
- **`pdfium.dll`:** equal to the smoke file's.
- **The version the tool reports:** bridge-tally 0.6.1.
- **One launch over stdio:**
  - the data folder new and empty, posting ON, and the three unset keys absent;
  - stopped after the stop, then `BRIDGE_AGENT_ENABLE_WRITES` removed: `launch-env.json` sha256 5c048763…;
  - the server's error stream 0 B.

## Steps run
| step | what | reading |
|---|---|---|
| 0.1 | step 0 with THE COMPANY alone. One tally.exe; 9001 LISTENING by it; `/status` 200. Tally's current date Monday 20-Apr-2026. `BRIDGE_AGENT_ENABLE_WRITES` present. Exe hash equal | PASS |
| 0.2 | BK0 by Tally's Backup: `TDBK1800_100021.001`, 102,253 B, sha256 a95a19edb1678aca19444cef80470cb67d0374b5fe96f25520f84e76af933841 (kept) | PASS |
| 0.3 | `tally_status`: 7.2, silver, not education. `list_companies`: exactly BRIDGE SHAPE LAB (100021, books from 20250401, 3a6bd6e1-b835-4bff-89dd-8a6af138c346) | PASS |
| 0.3 marks | `ledger_masters` (basic) refused: `ledger_export_invalid`, cause `company_several_currency_masters` (THE COMPANY keeps more than one Currency master) | a FINDING for the sheet. The marks were read instead from the `vouchers` answers' snapshot: M0 292, V0 112 (`reused` false, twice) |
| date | `vouchers` 15 Jun 2025: one Journal (number 6). 16 to 30 Jun 2025: one Journal on each of 16 to 21 Jun, none from 22 Jun | by the row note, the row's date is **22 Jun 2025** |
| 1.1 | THE TYPE's numbering as found, captured: method Automatic; numbering behaviour on insertion/deletion "Retain Original Voucher No."; additional numbering details No; show unused numbers Yes | recorded |
| 1.2 | Manual chosen in Method of Voucher Numbering; Tally asked the question above | **stop**. No by mouse on the run lead's word |
| after | the form left by Quit without saving; THE TYPE reopened: unchanged from 1.1 (captured). A last `vouchers` read for 22 Jun 2025: `partial`, `empty_window_wider_read_nonempty`, no snapshot, so no closing marks were read | the type unchanged |
| close | launch stopped; posting key removed; THE COMPANY shut. C10: one tally.exe on 9001; `/status` 200. No bridge_mcp.exe of this build left (the four still running date from 27 Oct and belong to the desktop client) | done |

## Not measured
Steps 1.3 to 7: the not-created batch, the settle dialog, the settlement and what it changes. The full count of THE COMPANY's Journals. Closing marks.

**Confidence:** the stop and the unchanged type are directly observed; nothing else was tested.

## Files
- `answers/`: the six answers, whole.
- `client-log.txt`.
- `evidence/`: `build.txt`, `bk0.txt`, the step-0 and C10 checks (netstat cut to the port-9001 lines), `s0-extra.txt`, `c10-extra.txt`.
- `scripts/`: `call.py` and `step0.ps1` (the stdio client itself stays on the lab PC, as in earlier rows).
- Also: `substitutions.txt`, `notes.txt` (UTC) and `SCREENSHOTS.txt` (the captures, kept on the lab PC).
