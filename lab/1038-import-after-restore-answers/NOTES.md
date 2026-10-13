# What an imported voucher takes after a backup is restored, and how a batch is numbered (#1038, Windows lab, 13 Oct 2026)

Synthetic data only: BRIDGE IDENTITY LAB (its GUID is in every marks answer), Receipt vouchers of
July 2026. TallyPrime 7.1, Silver Rental, on Windows. One run. No ComplyEaze Bridge build: raw HTTP
requests by curl, one at a time, each import file sent once; the backup and the two restores were made on
Tally's own screens. Times in send-log.txt are UTC.

## Method

- Marks: `requests/marks.utf16le.xml`. Vouchers of 1 to 31 Jul 2026: `requests/b1-window.xml`.
- Imports in the shape ComplyEaze Bridge's builder sends (no number, a new REMOTEID on each voucher):
  `imp1` = one Receipt A (10 Jul, 301.00, "Lab identity probe 1"); `imp2` = two Receipts with the later
  date first in the file: C (12 Jul, 303.00), then B (11 Jul, 302.00).
- After every step: marks, vouchers, and the table in `tables/`. One more marks read was sent just before
  each import. `tools/read-answers.ps1` parses the answers into `derived/analysis-output.txt`.

## Result (marks = ALTVCHID / ALTMSTID)

| step | act | vouchers read | marks |
|---|---|---|---|
| r0 | as found; backup Z0 taken | the seed Receipt "1" (1 Jul) and Receipt "2" (10 Jul, 301.00, keyed by hand the day before): MASTERID 2, ALTERID 2, GUID ending -00000002 | 2 / 211 |
| r1 | an older backup restored (the seed only) | the seed alone | 1 / 211 |
| r2 | `imp1` | CREATED 1. A reads number "2", MASTERID 2, ALTERID 2, GUID ending -00000002 | 2 / 211 |
| r3 | `imp2` | CREATED 2. C (first in the file, later date): "3", 3, 3, -00000003. B: "4", 4, 4, -00000004 | 4 / 211 |
| End | Z0 restored | the two vouchers of r0; everything under COLLECTION equal to r0's; the answers byte-identical to r0's | 2 / 211 |

- Question 1, does the imported voucher take what the removed one held: number yes, MASTERID yes, ALTERID
  yes, GUID yes. Its two ledger entries read in the order of the file (bank ledger first); the hand-keyed
  voucher read the party first.
- Question 2, how is a batch numbered: in file order, not date order.
- Marks after a restore that removes one voucher and no master: ALTVCHID 2 to 1, ALTMSTID unchanged.
- Tally's current date followed the book: 10-Jul-2026 as found, 1-Jul-2026 after the older backup,
  10-Jul-2026 after Z0. It was never set by hand.

## Prompts

The backup asked "Accept ?". Each restore asked "Accept ?" and then "Company data folder (100056) already
exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite
the existing folder."; Replace was chosen. The imports raised nothing on Tally's screen. The run had no
pause.

## What it decides

On this book and this release:

- A voucher that arrives by import after a restore took the removed voucher's number, MasterID, AlterID
  and GUID, as a hand-keyed one did the day before. No identifier read here told the two apart.
- Two vouchers of one import file were numbered in the order of the file.

## Not covered

ComplyEaze Bridge's own verification over these states; a REMOTEID sent a second time after a restore;
other voucher types; a restore that also removes masters; security on; Gold; other releases.

## Files

`requests/` (the four files sent), `answers/` (every answer with its headers), `tables/`,
`derived/analysis-output.txt`, `send-log.txt`, `tools/read-answers.ps1`, `SHA256.txt`. Kept on the lab PC,
listed by hash in `SHA256-kept-on-lab-pc.txt`: the screenshots, the backup Z0, the process and port
listings, the restore plan, the data-folder listings and the working log.
