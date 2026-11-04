# Row 2119-L: Ledger Outstandings for one party against the Bills Receivable report, BRIDGE CORPUS DENSE, 31 March 2026, by curl: results

**First line: no stop and no dialog. FINDING: the Receivable report took 45.19 s against the expected "about 33 to 42 s" (an EXPECT not met; no cause is given; not a stop rule). p1's Ledger Outstandings answer ties to the report bill for bill (248 matched, 248 equal, differ 0, only_ref 0, only_lo_debit 0, no credit bill, no On Account). Its total and ten printed rows equal Tally's screen. The two sends are byte-equal, at 1.667 s and 1.586 s. This meets the sheet's first "What it decides" row: a read by party is admitted for the design note.** (verified)

Sheet `lab/2119-l/ROW.md` at 9145f6cd (SHA256.txt 4 of 4, by sha256 and size). Every step and stamp is in `notes.txt` (UTC). The C3 lines with their times are in `evidence/send-log.txt`. The answers stay on the lab PC by size and sha256 (`kept-local.txt`).

| step | result |
|---|---|
| Tally's release, date; V and M at steps 1 and 6 | TallyPrime 7.2 Silver. Current Date Tuesday 31-Mar-2026, unchanged at the close. V 29,900 and M 328 at step 1 and at step 6, the two answers byte-equal (fcc9086d…). (verified) |
| the report: rows, sum, seconds, bytes | 29,900 rows, sum −1,425,845,144.50 (= R4 of 29 Oct). First byte 45.088 s, total 45.190 s. 13,679,490 B (= CONTENT-LENGTH, = R4's bytes), sha256 5a406cab…. (verified) |
| the pick: parties in the report, in the alphabet; p1's bills in the report; `lo-p1`'s size and sha256 | 120 parties in the report, all 120 in the alphabet. p1 holds 248 bills there. `lo-p1` is 1,196 B, sha256 87b020a84e143bcb1d769f2d478d2d3925f76333364de3e794da9fd16779a37c. (verified) |
| `lo-p1`: each send's first-byte and total seconds, bytes, sha256 equal or not | Send 1: 1.664 s / 1.667 s. Send 2: 1.582 s / 1.586 s. 205,336 B each. sha256 76e1b3e5… for both, **equal**. No dialog after either (captures equal to the Gateway's). (verified) |
| `compare` and `screen` outputs, whole | Below. (verified) |
| the screen tie: equal or not, row by row and the total | **Equal**: the total 1,18,15,459.60 Dr (Opening and Pending), and each of the ten printed rows (date, reference, opening, pending Dr, due date, overdue days). The display read 1-Apr-25 to 31-Mar-26. `evidence/screen-tie.txt`. (verified) |
| bills per second, report against `lo-p1` | The report: 29,900 bills in 45.19 s, about 662 bills/s. `lo-p1`: 248 bills in 1.667 s (send 1), about 149 bills/s; 1.586 s (send 2), about 156 bills/s. The one-party read is slower per bill, but its whole time is under 2 s. (verified) |
| screenshots: any change on Tally's screen | None beyond the navigation of step 5. Every capture after a send equals the step-0 Gateway. (verified) |

## What it decides (the sheet's row met)

p1's debit bills tie to the report, the screen equals the answer, the two sends are byte-equal, and each send takes under 2 s. So **a read by party is admitted for the design note**: ten parties fit inside the 20 s limit at this rate. The next row measures parties in turn. (verified; the reading is the sheet's)

Note, a hypothesis: every amount is negative in the report's sum and in the answer's sum (−11,815,459.60), and the screen shows the same figure as Dr. `compare` matched every amount equal, so the two reads agree in sign with each other. (hypothesis about the meaning of the sign; the equality is verified)

## bills.py, whole outputs (the party's name shown as p1)

`fixed`:
```
marks.utf16le.xml 982 040ac47754c40a4e2e736f8be2b3f37cac924213ea66c8c4f0074222f9a00280
ref-receivable.utf16le.xml 770 6520d94ec6d013e3e44d9af62ee9c9830a65085b4cb5b54710340df0d03c750f
```
`ref` (the captured file `evidence/bills-ref.txt` stays on the lab PC by size and sha256, unaltered, since its sum is a 10+ digit run; quoted here with the digits grouped):
```
ref ok bytes 13679490 rows 29900 sum -1,425,845,144.50
```
`select`:
```
parties 120 in the report; 120 in the alphabet; the pick holds 248 bills there
lo-p1.utf16le.xml 1196 87b020a84e143bcb1d769f2d478d2d3925f76333364de3e794da9fd16779a37c
```
`lo` (send 1):
```
lo ok bytes 205336 rows 248 sum -11815459.60
```
`compare`:
```
ref (this party) ok bytes 13679490 rows 248 sum -11815459.60
lo ok bytes 205336 rows 248 sum -11815459.60
lo bills 248 debit 248 credit 0 (sum 0) on_account 0 (sum 0) voucher_lines 248
key ref+date matched 248 equal 248 differ 0 only_ref 0 only_lo_debit 0 repeated_keys 0
```
`screen`: in `evidence/bills-screen.txt` and `evidence/screen-tie.txt`.

## Deviations, recorded
- **Before step 2, a lab slip with no request sent.** The first attempt's shell line put its folder change into a background group. So the send did not start, and a 30-s capture loop ran with no request running. Its four captures equal the step-0 Gateway. They are kept as `stray-idle-1..4` and are not counted as step 2's captures. (verified)
- The four desktop-client connector servers stayed running: no call was made to them from this sitting, and their calls were not measured. (verified)
