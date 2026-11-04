# Row 699-C16: nineteen vouchers added to BRIDGE SFT LAB by hand, and the book's audit read recorded byte for byte: results

**First line: every EXPECT met. No stop after the input slip, no refusal, no `refused` line in any manifest, no answer over 2,000,000 bytes. All nineteen vouchers keyed, C10, C15, C16, C17, C18 and C19 among them. a6 V 53. b9 byte-identical to a6. `audit_test_run` answered `completed` with no failed book check. One input slip in A5, recorded below: the first try at C1 was quit unsaved after Tally's "Spelling Error" box, the marks were rechecked (V 34, unchanged), and C1 was keyed again.** (verified)

Sheet `lab/699-c16-sheet` at ed42cd65 (RUN-SHEET sha256 d13b7a4c…; SHA256.txt 6 of 6 equal, by sha256 and size). LAB-COMMON v3. Every step and stamp is in `notes.txt` (UTC). Phase B is row 699-R1's steps 2 to 10 with the sheet's changes.

| step | result |
|---|---|
| a0: V, M, sha256; Tally's release; the dates | One COMPANY row, GUID 00f0c618-b0df-43bd-bd69-b4c7b92c616f, **V 34, M 219**, 4,878 B, sha256 6d3aebaa05b2d03b8384df58fb940adae098fb82e33275c8834599fce3ff933a (byte-equal to 699-R1's s1 and s9). TallyPrime 7.2 Silver. Tally's Current Date Tuesday 10-Mar-2026; the PC's date 2026-11-04. (verified) |
| a1: the backup's name, bytes, sha256, time; a1 marks | `BK-C16/TDBK1800_<company number>.001`, 42,052 B, sha256 92cbf1217873eaea120c55e8d85d75bc71b9aaed74353a4311ec3b10d3ae53db, taken 15:12 IST on 4 Nov 2026 (the time the Restore list will show). It stays on the lab PC. a1 marks byte-equal to a0 (V 34, M 219). (verified) |
| a3, a4: each master as created; a4 marks | Group `Grants C16` under Primary, Nature Income, "affects gross profits" No, every other field as Tally showed. Ten ledgers as the sheet's table, each name read before Enter. `Refund C16` and `Tax Held C16`: Type of Duty/Tax Others, Tally's only offer. `Creditor C16`: bill-by-bill shown Yes, typed No. `OD C16`: no bank details. No opening balance on any. The values of every field are in `notes.txt`. **a4 marks: V 34, M 230** (219 + 11), 4,880 B, sha256 19b30ea0…. (verified) |
| A6: the table; C10, C15, C16, C17, C18 and C19 keyed or not; a6 marks | The table is `evidence/a6-table.tsv`: one row per voucher with its type, number, date, every line's ledger, amount and side, its narration, and what Tally prefilled. **All nineteen keyed**: C10 keyed (Tally offered `Profit & Loss A/c` in the Journal list), C15 keyed (Capital SFT 1,000 and 2,000 stayed two lines), C16, C17 and C19 keyed (two lines each as written), C18 keyed (accounting invoice mode, party Party One, Sales SFT and Tax Held C16 as its two lines). **a6 marks: V 53** (34 + 19), **M 230**, 4,886 B, sha256 dda4ec59f9d5139eb1403d37b989e60c973da1b16556f443ba2e6a0e00fc5455. (verified) |
| Phase B: build; `bridge_mcp.exe` sha256 at B3 and B7 | The build 699-R1 ran: zip sha256 bd7e796174a185b1523258c3052acad67c29a7d8b2e68d99d5b12d6ce60cc2bf, `bridge_mcp.exe` c41fe5df31b3dc6a81c1eb90848837012579c041b6034531721c80c2cabb531c at B3 and at B7. bridge-tally 0.7.0. One `bridge_mcp` process (pid 4668) for the whole of Phase B; it exited 0 on the client's stop. (verified) |
| relay self-test | `self-test: ok (7 forwarded unchanged, 1 answer withheld, 15 refused, none of the refused reached Tally)`. Relay sha256 060c3b6c… (= R1's). (verified) |
| c1: `list_companies`, seconds; manifest lines | One open company, BRIDGE SFT LAB, this GUID, verified_tuple; 0.535 s. Tools list: 33 tools, `audit_test_run` among them. Manifest: 4 lines (2 POST, 2 GET /status), each forwarded, 200 and answer returned. No `events.txt`. (verified) |
| c2: `audit_test_run` answer, seconds; refusal code if any; manifest lines, each with method and bytes; largest answer | Answered with a result, 8.058 s, 7,322 B. No refusal. Manifest: **37 lines** (35 POST, 2 GET /status), each forwarded, 200 and answer returned. Request and response bytes per line are in the manifest and `notes.txt`. Largest answer 1,391,690 B. No `events.txt`. (verified) |
| c2: `result.state`, figures, findings, failed book checks | `completed`. Book checks evaluated: ID-1, MAP-0, MAP-1, POP-0, POP-1, POP-2, POP-3, POP-5; **failed: none**. Figures (paise, unless bp): applicable_threshold 1,000,000,000; bank_payments 450,000; bank_receipts 47,690,000; cash_payments 5,000,000; cash_receipts 461,900,001; cash_share_payments 9,174 bp; cash_share_receipts 9,064 bp. Finding: `cash_44ab/applicability`, confidence `needs_document`, clauses s.44AB(a) and 3CD-8. Rules version 2026-09-17.1, test version 1. (verified) |
| `read_evidence`: records, their states | 2 records, both `complete`: 8,000 B in 311 ms (the company list) and 1,989,202 B in 7,968 ms (the audit read). (verified) |
| c3: vouchers by type and in all; pages; manifest lines | **53 in all**: 14 Sales, 28 Receipt, 7 Journal, 3 Payment, 1 Contra, each with this GUID's prefix. One page (no next offset), 4.360 s. Type, number and date equal, as multisets, A6's 19 rows plus A5-VOUCHERS.md's 34 (`evidence/c6-check.txt`, by `check-c6.py`). C1 to C19's ledgers and amounts as read are listed there too, each as keyed. Manifest: 34 lines (22 POST, 12 GET /status), each forwarded, 200 and answer returned. No `events.txt`. (verified) |
| b9 against a6 | Read one minute after `bridge_mcp` stopped: V 53, M 230; the answer is **byte-identical to a6's** (dda4ec59…). (verified) |

## The input slip in A5 (C1)

- On the first try at C1, Tally filled the By line with Cash. The lab typed "Bank" while the cursor was on the line's To/By selector, and Tally raised an "Error" box: "Oops!" and "Spelling Error" (`shots/a5-c1-spelling-error.png`). The voucher was not accepted.
- On the recorded word, one click closed the box (`shots/a5-c1-box-cleared.png`). Ctrl+Q went straight back to the Gateway; no "Quit?" prompt showed, so no Y was pressed (`shots/a5-c1-quit-gateway.png`). Nothing was saved.
- The marks were read again before any key on C1: byte-equal to a4's, V 34, M 230 (`reads/a5-c1-recheck-marks.xml` is kept on the lab PC, listed in `kept-local.txt`).
- C1 was then keyed again from the start, pressing Enter on the selector before typing the ledger name. No box other than "Accept ?" showed after that, in C1 to C19.

## How Tally's prefills were met in A5 (recorded, not a stop)

Where Tally prefilled a later line (C11, C15, C16, C17, C19), the lab pressed Enter on the To/By selector, then typed the ledger name and the amount over the prefilled ones. Each voucher's lines therefore stand in a different order on screen from the sheet's listing; the ledgers, amounts and sides are as written, and the c3 read confirms each. Every bank or OD line opened Bank Allocations, accepted as offered. C9's OD C16 balance showed 3,000.00 Cr in red, with no warning box. C14 showed no bill-wise screen. (verified)

## Kept on the lab PC

No captured byte is altered. A captured file is pushed whole unless it holds the company number or a run of 10 or more digits. Two places are exempt: inside a whole 64-hex or 40-hex hash value, and the whole value of a VOUCHERKEY element. Each kept file is listed in `kept-local.txt` by size and sha256. Screens and the backup stay here too, listed in `SCREENSHOTS.txt` and `kept-local.txt`. (verified)
- **The company and marks reads in the relay folders** (c1 2, c2 24, c3 12; 38 in all): byte-equal, 4,000 B each. They hold the company number. The requests that drew them are pushed. (verified)
- **One voucher-window answer, recorded three times** (`relay/c2/0033`, `relay/c3/0024` and `relay/c3/0026`, each `-response.bin`): 428,826 B each, byte-equal (sha256 f678fb46…). It holds one NAME element whose value is GUID-shaped, with a run of 10 digits in its final group, so by the rule it stays here. Its VOUCHERKEY values are exempt. It also holds the Tally element name described in the next item. (verified)
- **The largest voucher-window answer, recorded three times** (`relay/c2/0030`, `relay/c3/0018` and `relay/c3/0020`, each `-response.bin`): 1,391,690 B each, byte-equal (sha256 e8f9564a…). It holds no company number and no counted digit run, but one of Tally's own element names, `VATWCVALUEOFTAX` + four letters + `GOODS` (88 times, empty each time), carries a word the push gate refuses, so it stays here unedited. (verified)
- The 130,712 B voucher-window answers are pushed whole, as are the requests that drew every answer kept here. (verified)
- **`list_companies`' answer** (`run/answers/001-c3-list-companies.json`): 1,560 B. JSON; the tool result's key `companies`, a list of 1. (verified)
- **`audit_test_run`'s answer** (`run/answers/002-c4-audit-test-run.json`): JSON. The structured content's keys are company, evidence, read_at, result and truncated; the result's keys are book_checks, figures (7), findings (1), limitations (2), population_note, state and test. (verified)
  - whole file: 7,322 B, sha256 c27b10791444c14d4f6a082c74d798a800ad98b8fa03e35ee4d11b041cf31b19
  - audit result (canonical JSON): 2,671 B, sha256 517b8c9493649eb17e8667ddde5cc6c6339170d6718926ef5cd67cb5d4eeb942

  The second line is the `result` member alone, as `json.dumps(obj, sort_keys=True, separators=(",",":"), ensure_ascii=False).encode("utf-8")`, as in 699-R1. Its bytes are written to no file. (verified)
- **`vouchers`' answer** (`run/answers/004-c6-vouchers-p0.json`): 61,935 B. JSON; the tool result's keys are items (53), offset, profile, reason, snapshot, state, total and window. It carries the company number. (verified)
- **The marks reads** (`reads/`): a0 and a1 4,878 B (6d3aebaa…), a4 and the C1 recheck 4,880 B (19b30ea0…), a6 and b9 4,886 B (dda4ec59…). Each holds the company number. Their header files are pushed. (verified)

## Close

Tally's Gateway showed no dialog after Phase B (`shots/b10-gateway-before-shut.png`). Shut Company by Tally's own menu (Alt+K, Down ×4 to "SHut", Enter, Enter on BRIDGE SFT LAB, the only row). Select Company, none loaded (`shots/b10-after-shut.png`). One tally.exe (pid 1200) on 9001; GET /status 200. No restore, as the sheet says: the book stays as it stands after Phase B, and `BK-C16` holds it as 699-R1 read it, with 34 vouchers. (verified)

## Decides (the sheet's)

Phase A ran as written. In Phase B, c2 holds every answer, `audit_test_run` answers with a result, `read_evidence` is all complete, c3 counts 53 with the types and dates on record, and b9 equals a6. So, by the sheet's first row, `c2`'s exchanges become the fixtures of clause 16(a)/(d)'s tests, measured against the synthetic lab book only. No shape is unkeyed. (verified; the reading is the sheet's)

The four desktop-client connector servers stayed running: no call was made to them from this sitting, and their calls were not measured.
