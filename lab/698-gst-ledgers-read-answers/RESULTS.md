# Results: 698 GST ledgers read, BRIDGE SHAPE LAB, 1 Apr to 30 Jun 2025 (25 Oct 2026). READ-ONLY

TallyPrime 7.2 (Latest), Silver, `tally.exe` 1.1.7.2, Education mode No. The item's files are at `lab/698-gst-ledgers-read-sheet` @ aa011637 (`SHA256.txt` 11/11 OK; `LAB-COMMON.md` is the v3 byte copy, 5370c6d2…).

THE COMPANY is BRIDGE SHAPE LAB, GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346, loaded alone; Tally's current date 20-Apr-2026, unchanged. Nothing was keyed, imported, backed up or restored. Every send was the C3 curl line, hash-checked first, and returned HTTP 200 with RESPSTATUS 1 and size equal to CONTENT-LENGTH (`send-log.txt`). Answers are listed in `MANIFEST` and `SHA256.txt`.

## Start
- The lab had BRIDGE PILOT LAB loaded alone, left there by the previous read-only item. It was shut by Company > Shut before this item, with the run's review informed. Then BRIDGE SHAPE LAB was loaded alone by Select Company.
- Step 0 (C2):
  - Gateway screenshot: no dialog.
  - One `tally.exe` owns 9001 (LISTENING lines present, including other ports).
  - GET /status: 200, 51 bytes.
  - MARKS: one COMPANY row, THE COMPANY's GUID, ALTVCHID 112, ALTMSTID 292.

## Steps (UTC 25 Oct 2026)
| UTC | Step | What was read |
|---|---|---|
| 09:54:43 | 0d marks | 112 / 292 |
| 09:54:44 | 1 company list | one company, BRIDGE SHAPE LAB, GUID equal to the request files' |
| 09:54:44 | 2 marks | 112 / 292 |
| 09:54:45 | 3 census | 62 VOUCHER rows dated 20250401 to 20250621, every GUID under THE COMPANY's (≤ 400, so step 4 went ahead) |
| 09:55:33–34 | 4 | company object 4,208 B; groups 62,904 B; ledgers 54,162 B (45 ledgers); trial balance 56,318 B; voucher types 35,278 B |
| 09:55:34 | 5 marks | 112 / 292 |
| 09:55:35 | 5 vouchers | 2,504,288 B; see the deviation below |
| 09:55:48 | state read after the stop (C8) | marks 112 / 292 |
| 09:57:29 | 5 marks again | 112 / 292 |
| 09:57:30 | 5 company list again | identical bytes to step 1 |

## Deviation: the vouchers bound
- The sheet names no bound, so LAB-COMMON C3's 2,000,000 bytes applied.
- The vouchers answer arrived whole: 2,504,288 bytes, equal to CONTENT-LENGTH, RESPSTATUS 1, sha256 097b70fc…. That is above the bound, so the item stopped before using it.
- One C8 state read followed (marks unchanged).
- The run's review then raised the bound to 4 MB for this request only, **after receipt; no re-send**. The answer received was used as it is.

## EXPECT, recorded
- Every answer HTTP 200, STATUS 1, no LINEERROR: yes.
- The vouchers part holds exactly the census's vouchers: yes. 62 VOUCHER rows; the same 62 GUIDs as the census, each with the same ALTERID; every GUID under THE COMPANY's.
  - By type: Purchase 20, Journal 14, Payment 12, Receipt 7, Contra 4, Sales 2, Debit Note 1, Credit Note 1, Stock Journal 1.
  - 146 ledger entry lines in all.
- The marks answers around the vouchers read are identical: yes. All five marks answers have the same bytes (c71056a0…, 112 / 292).
- Vouchers with a Duties & Taxes line: 10 of 62. These are lines on ledgers whose parent is Duties & Taxes in the ledgers answer: CGST, Cess, IGST, State Tax, TDS Payable. So the read holds what the parity test needs.

## Close (C10)
- BRIDGE SHAPE LAB was shut by Company > Shut (Alt+K, H, Enter on its one row).
- One `tally.exe` owns 9001; GET /status 200.
- Select Company shows no company loaded. No XML request was sent after the shut.

## Kept on the lab PC (listed by hash)
- The two company-list answers (`a01`, `a14`, 4,000 B each, sha256 52700a2c…). They hold the company's folder number.
- Screenshots `s00-step0-gateway.jpg` (113,578 B, ff37a4fe…) and `s99-close-no-company.jpg` (59,369 B, 511d779c…).
- The C2 command outputs.
