# Results: LR6, sheet 699 (SFT-013 candidates), Phase A on TallyPrime 7.2, 26 Oct 2026. Phase A done as written; Phase B not run (skipped by the run's review)

The sheet is `lab/699-sft-candidates-sheet` @ fbb77e4803d0ab018d4fd985b15308512bed0b2b: `SHA256.txt` 3/3 OK, `LAB-COMMON.md` sha256 5370c6d2… as the sheet states. TallyPrime 7.2, Silver, not Educational mode. The owner authorised the lab, in the lab's own chat, to key this sitting itself ("one act at a time as the sheets say"); every act below was done by the lab on Tally's screens, with a screenshot (kept on the lab PC, listed in `SCREENSHOTS.txt`) before each master and voucher was accepted.

## THE COMPANY
`BRIDGE SFT LAB`, company number 100079, GUID **00f0c618-b0df-43bd-bd69-b4c7b92c616f**, financial year and books from 1-Apr-2025.

## Phase A
| Step | What | Result |
|---|---|---|
| Start | No company loaded (the previous item's company had been shut) | as C1 |
| A0 | Whole Tally data folder backed up as a tar into a new empty folder (listing before: empty), no company loaded | `tally-data-BK-LR6.tar`, 819,680,768 bytes, 1,909 entries, sha256 **604a5f0499f48a7585fcfb245de157008ded550a93adf75cf14b476f6afa2475**; one file after. It stays on the lab PC; restoring it is the way back for everything below, on the run's review's word only |
| A1 | No change to any other company | none made |
| A2 | Created BRIDGE SFT LAB (Select Company showed no company of that name first). Features: Maintain Inventory set **No**, Enable GST set **No**; bill-wise entry left Yes (Party Bill needs it); no GSTIN, no PAN; nothing else changed. Tally asked for no GSTIN or PAN | loaded alone |
| Step 0 (C2) | Screenshot (no dialog); tasklist one tally.exe, exit 0; netstat 9001 LISTENING with that PID (IPv4 and IPv6), exit 0; GET /status 200, 51 bytes; company list one COMPANY row (100079, the GUID above); MARKS one row | ALTVCHID none, ALTMSTID 208 |
| A3 | `Cash` is Tally's own ledger under Cash-in-Hand and was not created again. The other 11 ledgers created with the groups in the sheet; bill-by-bill No on all but Party Bill (Yes) | ALTMSTID 219 (+11). Chart of Accounts: 13 ledgers (the 11, Cash, Profit & Loss A/c) |
| A4 | The 34 vouchers keyed exactly as written; see `A5-VOUCHERS.md`. No dialog of any kind, and no negative-cash warning (cash never went below nil on any date) | ALTVCHID 34, ALTMSTID 219 |
| A4 screenshots | Day Book 1-Apr-2025 to 31-Mar-2026, all 34 vouchers on one screen with amounts; Ledger Outstandings of Party Bill (pending bills) | Party Bill has **no pending bill**: every bill is settled |
| A5 | Table of every voucher as keyed, and the Cash Book totals for the year | `A5-VOUCHERS.md`: Cash Dr 46,10,000.01, Cr 3,20,000.00, closing 42,90,000.01 Dr |

Keying notes (recorded, not findings):
- Vouchers were keyed type by type, each type in the sheet's order (Sales 1-13, Receipts 1-19, Payment 1, Contra 1), so each type's numbering follows the sheet's order.
- Tally pre-fills a new line with the last ledger used and the remaining balance. Where that pre-fill differed from the sheet, it was overtyped before accepting: Receipt 5 and Receipt 8 line 3 (Cash to Bank SFT); Receipt 15 line 2 (By Bank SFT to To Party Two) and line 3 (Bank SFT to Cash).
- Sales were in accounting invoice mode; each opened a Dispatch Details and a Party Details sub-form, accepted blank / as offered.
- The bill-wise screen for Party Bill offered New Ref on sales and Agst Ref on receipts by default; the pending-bills list on a receipt listed only bills dated on or before it.
- While leaving a report, one Escape landed on that report's F12 Configuration and blanked its "Show Opening Amount" field; it was set back to Yes and accepted. That is a display setting of the report, not book data.

## Phase B
Not run: the run's review ruled that builder-rendered requests sent by curl are not admitted under the request-shape rule. No request other than C2's own reads (company list, MARKS) was sent to Tally in this item.

## C10
Company > Shut (Alt+K, h, Enter); Select Company with no company loaded; tasklist one tally.exe (exit 0); netstat 9001 LISTENING with that PID (exit 0); GET /status 200; no XML request after the shut.

## In this commit
- `A5-VOUCHERS.md`, `notes.txt` (UTC times, one line per step), `send-log.txt` (every raw read: request sha256, bytes, answer sha256).
- `answers/`: step 0 and C10 observations (tasklist, the 9001 lines of netstat with the count of LISTENING lines, /status), the company list and every MARKS answer with headers.
- `requests/`: the company list request and the filled MARKS request (the template's `@@COMPANY@@` and `@@GUID@@` only).
- `MANIFEST.md`, `SCREENSHOTS.txt`, `SHA256.txt`.
- Not committed: the screenshots, the A0 tar, and the full netstat outputs (they list every connection on the PC; kept on the PC, sha256 66959150… and a6f27418…).
