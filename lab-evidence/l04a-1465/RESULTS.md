# Results: L04-a (#1465), the voucher read-cost preview against one real call. Run on the published 0.6.0 build

Windows lab PC, TallyPrime 7.1 Silver (licensed; `tally_status` reports `education_mode` false), port 9001, the installed ComplyEaze Bridge extension 0.6.0. BRIDGE PILOT LAB (company number 100033, GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85) loaded alone. Read-only: nothing keyed or posted. The steps came to the lab as the requester's message text (sheet L04-reads-1465-1211-724.md, section L04-a; its sha256 3677d4e4… was not checkable on the lab PC). Sizes and sha256 of every file are in `ANSWERS.sha256`.

## Step 1: status, company and voucher mark
- `answers/s1-tally_status.json`, `answers/s1-list_companies.json`: one loaded company, BRIDGE PILOT LAB, `identity_state` verified_tuple.
- `requests/r02-marks.xml` is `lab/1641-rows-run-sheet/requests/r02-marks.xml` at 97226a3c (976 bytes, sha256 81892a74…). Sent once by plain HTTP (`send-log.txt`): HTTP 200, 3,394 bytes, sha256 8a0c4268…, byte-identical to the 697-S1 marks answer of the same afternoon. **ALTVCHID 68**, ALTMSTID 245. The mark is at most 170, so the no-census clause of step 5 applies.

## Step 2: `voucher_read_cost`
`answers/s2-voucher_read_cost.json` (whole answer), egress lines in `answers/egress-s1-s2.jsonl` (the response_prepared and stdio_write_completed lines of steps 1 and 2, copied byte-exact from the extension's local receipt log).
- `voucher_mark` 68, `request_spacing_ms` 500.
- vouchers: `census_reads` 1, `floor_seconds` 0 (derived). ledger_movement: `census_reads` 0, `floor_seconds` 0. verify_import: `census_reads` 0, `floor_seconds` 0.
- The call's own request trail (response_prepared line, receipt 8f863f12…): 10 sends, none failed: the identity read (2,524-byte request, 4,000-byte answer) 4 times, the marks read (976-byte request, 3,394-byte answer) twice, and 4 endpoint status checks. No census request and no voucher request.

## Step 3: one `vouchers` call over one day
- D = 1 Aug 2026, the earliest FY 2026-27 date in `lab-evidence/697-s1-census/answers/4.xml` on `lab/697-s1-1605-s1-answers` (the c02 census for FY 2026-27: 41 vouchers, 11 of them on 20260801).
- `answers/s3-vouchers-20260801.json` (whole answer): state complete, total 11 (BRIDGE Sales BP/26-27/0001 to 0011), snapshot master 245 / voucher 68, `window.census.requests` 1, `window.marks.requests` 1, one data part of 809,932 bytes with 11 rows.
- Egress lines in `answers/egress-s3.jsonl` (receipt f13eb126…). The trail: 22 sends, none failed. Identity reads (2,524 bytes), marks reads (976 bytes), the census request (1,436 bytes, answer 29,814 bytes), and the window request (1,740 bytes, answer 809,932 bytes), each sent as a pair with a status check between. The census request appears twice in the trail as one paired read; the call reports it as `census.requests` 1.

## Checks
| Check | Result |
|---|---|
| (i) the call's census reads equal the preview's for `vouchers` | PASS: preview `census_reads` 1; call `window.census.requests` 1 (one paired census read in the trail) |
| (ii) the preview sent only identity and marks reads | PASS: 4 identity reads, 2 marks reads, 4 status checks; no census, no voucher request |
| (iii) floor = (N-1) x 0.5 s rounded down | vacuous at this mark: N = 1, floor 0 s as reported |
| (iv) ledger_movement and verify_import report no census at a mark of at most 170 | PASS: both `census_reads` 0 at mark 68 |

The Gateway of this company shows "(e) Data exceptions exist", as it did when the lab found it this evening; nothing was changed.
