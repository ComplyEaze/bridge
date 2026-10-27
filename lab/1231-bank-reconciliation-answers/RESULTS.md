# Results: #1231 third read, a reconciled bank entry, TallyPrime 7.2 Silver, 27 Oct 2026 (UTC)

The sheet is `lab/1231-bank-reconciliation/README.md` at 4a36e7e592d5710765efc3f9d4415ffd9f6ca863 (SHA256.txt 4 of 4 equal; census ok 1). The company is BRIDGE TRANSFER LAB, number 100051, GUID d6e4c24f-8c7a-4797-93ec-e44bdff60bc9, with five Contras of 50,000.00 between TLab Bank A and TLab Bank B. Reads used the sheet's request (`V-CONTRA.utf16le.xml`) by curl; marks came from the w1-2306d86d build's `ledger_masters`.

## FINDINGS
**1. What a reconciled entry reads (r1 against r0).** Contra 1 and Contra 2 were reconciled on TLab Bank B with bank date 6-Apr-2026. On exactly that entry of each voucher (`ALLLEDGERENTRIES.LIST[2]/BANKALLOCATIONS.LIST`), and nowhere else:

| element | r0 | r1 |
|---|---|---|
| BANKERSDATE | (empty) | 20260406 |
| BANKMANUALSTATUS | (empty) | Reconciled |
| BANKRECONSTATUS | Available Only in Books | Reconciled |
| STATUS | No | No (unchanged) |

- **The marks moved.** Each voucher's ALTERID moved: Contra 1 from 1 to 6, Contra 2 from 2 to 7. The company's ALTVCHID moved from 5 to 7; ALTMSTID stayed 210.
- **Nothing else changed.** The TLab Bank A entries of the same two vouchers did not change, and Contras 3, 4 and 5 are byte-identical in r0 and r1.

**2. Clearing a reconciliation is refused on this path.** On 7.2 a manual reconciliation is not cleared by blanking its bank date in Set Status. The path was Banking > Banking Activities > TLab Bank B > F5 View Reconciled > April > F9 Set Status on Contra 1. Clearing Bank Date and pressing Enter gave the error "The Bank Date cannot be earlier than the Reconciliation Beginning Date. Enter a valid date to proceed." The form was left without saving, and the marks stayed at 210 / 7. So the sheet's step 4 (clear one) and its read r2 were not done, and step 5 compares r0 with r1 only.

**3. An unnamed dialog appears first.** On the way in, the manual reconciliation screen opened under a dialog: "Want to save your time? Switch to Auto-reconciliation with Bank Statements", with the buttons Know More and Don't Show Again. One Escape closed it without pressing either button, and the manual screen stayed.

## Control and book state
- **r0** (before any change): 200, 211,552 B, sha256 262e3003…. Every bank allocation of all five Contras reads BANKERSDATE empty, STATUS No, PAYMENTMODE Transacted and BANKRECONSTATUS "Available Only in Books" (`evidence/r0-bankalloc.txt`).
- **r1**: 200, 211,698 B, sha256 b9ac75ff….
- **BK0**, a backup taken before the first change for later rows on this book: `TDBK1800_100051.001`, 28,076 B, sha256 25fdc1e373403ad0ec0f35e728d2d8ee4a67aa970f5459398013e02b3ef60de6. It stays on the lab PC.
- **The book is left as step 2 made it**, as the sheet intends: Contra 1 and 2 are reconciled on TLab Bank B at 6-Apr-2026.
- **Through ComplyEaze Bridge** (step 6): `vouchers` for 1 to 5 April, Contra, shows alter_id 6 and 7 for those two vouchers and no bank-date or reconciliation field.

## PASS / FAIL
r0 and r1 were received whole (200, RESPSTATUS 1, parsed XML), and every difference is named above. r2 was not taken, because Tally refused the clearing; that refusal is itself recorded as finding 2. No voucher outside step 2 changed. The two dialogs were stopped on and handled one key at a time, as decided at each stop.

## Files
- `evidence/`: r0 and r1 with their headers; the per-entry tables and the r0/r1 diff (`r0-r1-diff.txt`); `log.txt`; the two read-only helpers.
- `tool-answers/`: every ComplyEaze Bridge answer, plus `client-log.txt`.
- `notes.txt`: UTC, one line per step.
- `SCREENSHOTS.txt`: the captures and BK0 by name, bytes and sha256 (they stay on the lab PC).
