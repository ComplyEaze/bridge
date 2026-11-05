# 1108-A1 and 1108-B: does Tally refuse a Journal line that ComplyEaze Bridge admits? (results)

**A1: refused per voucher. B: stored.** One run each, PARTIAL.

- **A1**, a Journal with a Stock-in-Hand ledger on an integrated book (`A1 Closing Stock` Dr 100.00, `A1 Capital` Cr 100.00, 10-Apr-2025): `CREATED 0`, `EXCEPTIONS 1`, every other counter 0, `LINEERROR` "Cannot use 'A1 Closing Stock': Ledger under Stock-in-Hand!". V did not move, no voucher was added, and no dialog showed. By the sheet's table: **a fault Tally refuses per voucher after Bridge's checks.** A2 and A2′ (the `post_import` batch, then the twins case) are the next sheet; they are not part of this one.
- **B**, a Journal with a cash ledger (`Cash` Dr 200.00, `A1 Capital` Cr 200.00, 10-Apr-2025): `CREATED 1`, every other counter 0, `LASTVCHID 1`, no `LINEERROR`; V from blank to 1; one Journal added (number 1, Auto Retain, `Cash` Dr 200.00 and `A1 Capital` Cr 200.00). By the sheet's table: **dead: Tally stores a cash line on a Journal**, consistent with Bridge's own summary line.

Sheet: `lab/1108-a1-stock-in-hand-journal` at 72b54de8, folder `lab/1108-a1/`, `RUN-SHEET.md`; SHA256.txt 17 of 17 equal. `gen_requests.py`: "8 request files", every file still equal after it. `check-request-ids.py requests`: census ok 4, skipped 4, FAIL 0. LAB-COMMON v3 (5370c6d2). THE COMPANY: BRIDGE STOCK CREATE LAB (row 1381-M's book), GUID shown as `<company guid>` (equal to 1381-M's record), number as `<company number>`. TallyPrime 7.2 Silver (the screen shows SILVER; tally.exe 1.1.7.2), licensed; Tally's date Tuesday 1-Apr-2025. No Bridge build ran: every request was sent by `curl` on 9001 (C3), with a 200,000-byte bound.

## The steps

| step | what | result |
|---|---|---|
| 0 | load, m0 | one COMPANY row, this GUID, **M 210, V blank** (= EXPECT). No prompt. |
| 1 | flags, ledgers, vouchers (NEW SHAPES) | flags: ISINVENTORYON `Yes`, ISINTEGRATED `Yes`. ledgers: `Cash` under `Cash-in-Hand`, `Profit & Loss A/c` under `Primary`; no `A1` ledger. vouchers: 0 rows. Every GUID begins with THE COMPANY's; a capture after each read, no dialog (= EXPECT). |
| 2 | BK-1108, BASELINE | `TDBK1800_<company number>.001`, 24,219 B, sha256 3aba50fe98516e61059fbd471cfabb0d6f0f9cf7ed7b73465de15c007efbd465, in `<lab folder>\1108a\backups\bk\`. BASELINE marks, ledgers and vouchers byte-equal to steps 0 and 1. |
| 3 | W1, W2 | W1: `CREATED 1`, every other counter 0; one row ADDED, `A1 Closing Stock` under `Stock-in-Hand`. W2: `CREATED 1`, every other counter 0; one row ADDED, `A1 Capital` under `Capital Account` (= EXPECT; nothing skipped). M 210 to 212. |
| 4 | screen read, no write | below. Marks after: M 212, V blank (unchanged since W2) (= EXPECT). |
| 5 | A1, B | above. |
| 6 | last reads | marks V 1, M 212; ledgers against the BASELINE: the two A1 ledgers ADDED; vouchers: B's Journal ADDED. |
| 7 | restore BK-1108 on the run lead's RESTORE | the two expected prompts only. marks, ledgers and vouchers each **byte-equal to the BASELINE** (M 210, V blank, no A1 ledger, 0 vouchers) (= EXPECT). |
| 8 | C10 | the company shut; one tally.exe owning 9001, `/status` 200, none loaded. |

## Step 4, the screen read

Vouchers opened Accounting Voucher Creation already as a Journal (F7 shown greyed, the current type), with the first line's ledger list open.

**F12, Voucher Configuration, word for word:** "Show more configurations : No". General Details: "Use Cr/Dr instead of To/By during voucher entry : No"; "Preallocate bills for Payment/Receipt/Journal Vouchers : No"; "Use default Bill-wise details for Bill Allocation : No"; "Show Inventory details : Yes"; "Show Bill-wise Details : Yes"; "Show Bill-wise details in multiple lines : Yes"; "Show list of Bills for selection : Yes"; "Show Final Balances for each Bill : Yes"; "Enable Stripe View : No". Bank Details: "Use default Bank Allocations : No"; "Use Auto Cheque Numbering : Yes"; "Select Cheque Range : Yes"; "Print Cheque after saving Voucher : Yes"; "Show Cheque details before printing : Yes". GST Details: "Provide Party details for GST : No"; "Modify GST & HSN/SAC related details : No". **None is about cash or bank accounts in journals**; the Bank Details options are about bank allocations and cheques. Nothing was changed, and "Show more configurations" was not opened.

**The first line's ledger list:** "Create", "Show More", "A1 Capital", "Profit & Loss A/c". **`A1 Closing Stock` is not offered and `Cash` is not offered** in the list as shown ("Show More" was not pressed).

## Deviations

1. **Ctrl+Q, not Esc (step 4).** The owner's rule (no Escape on voucher or configuration screens) holds over the sheet, on the run lead's standing word. Ctrl+Q left the F12 panel back to the Journal with no prompt, and a second Ctrl+Q left the blank Journal straight to the Gateway, with no "Quit ?" prompt shown; nothing was answered and nothing changed (marks unchanged).
2. After A1, the Gateway showed the line "Data exceptions exist" (seen behind the Shut list of step 7); it is a Gateway line, not a dialog, and was gone after the restore.
3. The marks and vouchers answers after A1 differ in bytes from the ones before only in header count nodes (`LEDGER`, `TAXUNIT`, `VOUCHERNUMBERSERIES`: the objects Tally holds loaded), not in marks or rows.
4. In step 7 a first try of the three reads sent nothing: the lab's PowerShell loop passed an empty file name, `send.py` stopped on its arguments before any send, and the checks found no file (NOT READ). The reads were then sent and checked; their check outputs replace that try's.

Not measured, as the sheet says: the same lines through `post_import` (A2, conditional on A1, now due); a book not integrated with inventory; a Bank ledger on a Journal; Payment, Receipt and Contra; TallyPrime 7.1. No Bridge process and no desktop-client connector server was used: no call was made to them from this sitting, and their calls were not measured. Nothing ended a process.

## Files

- `answers/`: every answer whole that holds no GUID or company number (the rest stay on the lab PC, `kept-local.txt`), with headers.
- `evidence/`: every `check_1108.py` output (the GUID as `<company guid>`), the send log, BK-1108's record, the restore plan, the company folder counts, the census and `gen_requests.py` output, the C2 records.
- `notes.txt`: UTC, one line per step. `SCREENSHOTS.txt`, `kept-local.txt`, `substitutions.txt`, `scripts/` and `SHA256.txt`.
