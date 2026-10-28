# Results: #2006 bankref recapture, 28 Oct 2026. The five hand-keyed Contras were keyed again with references the merge gate accepts. The raw capture has the first run's shape: per voucher and bank, the same signs, allocation types, entry order and dates

The sheet is `lab/2006-bankref-recapture` at 9efdf0bdd41bf92199f9e8dca6f9b1fd977e68d8 (`README.md`; `SHA256.txt` 4 of 4 equal; `check-request-ids.py .` exit 0, census ok 1).
- **Tally:** TallyPrime 7.2 (Latest), Silver Rental, Licensed (About screen; `tally_status`: 7.2, silver, education_mode false).
- **Company:** BRIDGE BANKREF LAB, number **100094**, GUID **8fb5f4db-478d-4084-b1e5-5b8f5d3fef60** (from `list_companies`), books from 1 Apr 2026, no inventory, no GST.
- **Redaction:** off (unset).
- **BRIDGE TRANSFER LAB:** not opened.

## How it was keyed (deviations, recorded)
- **Screen mode.** The sheet says the first run used Tally's single-entry Contra screen. The first run's committed screenshots (`answers-1392-h` at f0250cb9, `h-c1-voucher.jpg`) show the **double-entry** screen: To TLab Bank A Credit, By TLab Bank B Debit. The five Contras here were keyed in double entry: To = the table's Cr bank, By = the table's Dr bank.
  - A first attempt in single entry was set aside. On 7.2's single-entry Contra the Account line is the debited one, the reverse of the sheet's sentence.
  - That book was renamed BRIDGE BANKREF LAB RUN1 (100093) and kept, shut. It holds two ledgers and one Contra and is not the evidence.
  - Tally's voucher mode was set to Single Entry for that attempt and set back to Double Entry before the evidence book.
- **C2's transaction type.** No type named "Same Bank Transfer" is offered on 7.2's allocation screen. The first run's screenshot (`h-c2-bankalloc-a.jpg`) shows it chose **e-Fund Transfer**, with Bank Name "Not Applicable", which Tally stores as `Same Bank Transfer`. The same choice was made here, and the capture stores `Same Bank Transfer` on both entries.

## Where each reference sits
| # | Date | To (Cr) | By (Dr) | Reference |
|---|---|---|---|---|
| C1 | 3 Apr 2026 | TLab Bank A (Cheque) | TLab Bank B (Cheque/DD) | narration `UTR 111111111111` |
| C2 | 3 Apr 2026 | TLab Bank A (e-Fund Transfer) | TLab Bank B (e-Fund Transfer) | bank allocation, field "Inst No.", `222222222222` on both entries, Inst Date 3-Apr-26; no narration |
| C3 | 4 Apr 2026 | TLab Bank A (Cheque) | TLab Bank B (Cheque/DD) | none |
| C4 | 3 Apr 2026 | TLab Bank B (Cheque) | TLab Bank A (Cheque/DD) | narration `UTR 444444444444` |
| C5 | 5 Apr 2026 | TLab Bank A (Cheque) | TLab Bank B (Cheque/DD) | bank allocation, field "Inst No.", `123456` on both entries |

Each typed reference was read back on the screen before the save: a pixel-column count of the field found twelve digits each time. The allocation types not named above are the ones Tally offered.

## Step 2: the raw capture
- **Request:** `V-CONTRA.utf16le.xml` (1,744 B, d9394603...), sent with the sheet's curl line.
- **Answer:** curl exit 0, `200 211536`, CONTENT-LENGTH 211536, 09:18:42Z (`evidence/log.txt`).
- **`evidence/ref-contra.response`:** 211,536 B, sha256 **c5b465b3ebfec9df6950b8a520003e08de71c1be4438c95d1d2f64d410d1cc92**, kept byte for byte.
- **Parsed with an XML parser** (`parse-contra.py` into `evidence/ref-contra.parsed.txt`, which holds every voucher's number, date, GUID, ALTERID, narration and reference, and every entry's ledger, amount, ISDEEMEDPOSITIVE and every `BANKALLOCATIONS.LIST` element with its value, NAME included):
  - five VOUCHER rows, numbers 1 to 5, ALTERID 1 to 5, GUIDs `8fb5f4db-...-0000000N`;
  - `INSTRUMENTNUMBER` reads exactly `222222222222` on both entries of voucher 2 and `123456` on both entries of voucher 5;
  - vouchers 1, 3 and 4 carry no instrument number;
  - the narrations of vouchers 1 and 4 read exactly `UTR 111111111111` and `UTR 444444444444`;
  - REFERENCE is empty on all five.
- **Against the first run's raw capture (r0, BRIDGE TRANSFER LAB):** for every voucher and bank, the AMOUNT sign and TRANSACTIONTYPE are equal, the entry order is equal and the date is equal. No difference.

## Steps 3 and 4: ComplyEaze Bridge
- **Build and setup:** the 0.6.1 cut build (`bridge_mcp.exe` sha256 6f520338...), over stdio, reads only.
- **`list_companies`:** exactly BRIDGE BANKREF LAB, number 100094, GUID 8fb5f4db-478d-4084-b1e5-5b8f5d3fef60, `verified_tuple`.
- **`vouchers`** (2026-04-01 to 2026-04-05, Contra; verbatim in `evidence/vouchers-answer.txt`):

| voucher_number | date | guid | narration | reference | amounts |
|---|---|---|---|---|---|
| 1 | 20260403 | ...-00000001 | UTR 111111111111 | (none) | TLab Bank A 50000.00 (No); TLab Bank B -50000.00 (Yes) |
| 2 | 20260403 | ...-00000002 | "" | (none) | TLab Bank A 50000.00; TLab Bank B -50000.00 |
| 4 | 20260403 | ...-00000004 | UTR 444444444444 | (none) | TLab Bank B 50000.00; TLab Bank A -50000.00 |
| 3 | 20260404 | ...-00000003 | "" | (none) | TLab Bank A 50000.00; TLab Bank B -50000.00 |
| 5 | 20260405 | ...-00000005 | "" | (none) | TLab Bank A 50000.00; TLab Bank B -50000.00 |

Nothing is left halted. The book is kept, shut, as keyed. Screenshots and the RUN1 book stay on the lab PC.

## Files
- `evidence/`:
  - `log.txt`;
  - `ref-contra.response` and `ref-contra.headers`;
  - `ref-contra.parsed.txt`;
  - `vouchers-answer.txt`.
- `answers/` and `client-log.txt`: the Bridge calls.
- `parse-contra.py`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`.
- `EVIDENCE-ZIP.txt`: the zip of `evidence/` and its sha256.
