# Results: #899 part 2, book O again, from the sheet at eeb25c83, 27 Oct 2026. PASS: the spanned (mark 21) and unspanned census of BRIDGE OUTSTANDINGS LAB are byte-identical, 17 vouchers

The sheet is `lab/899-spanned-census-captures` at eeb25c8382a31ad0d996636248948aac34541f7b. Its `o-c2.utf16le.xml` was rendered again for mark 21 (`$AlterID > 0 AND $AlterID <= 21`), and its stop rule for book O is now "not 21". All 8 files were checked against its `SHA256.txt`: 8/8 OK. The files are in `answers/book-o-from-eeb25c83/`. The first attempt's `o-m1` (`answers/o-m1.*`, mark 21, stopped there) is unchanged; see `RESULTS.md`.

`tally_status` (`o-tally_status.json`, build w1-2306d86d 92a899a2, new empty data folder) reports TallyPrime 7.2, silver, `education_mode` false, and exactly BRIDGE OUTSTANDINGS LAB loaded (100028, GUID 49f1fbda-ee59-4a4b-aacf-b45fe32402d7). The book was loaded alone, with no question or prompt. Requests were sent one at a time by the sheet's command (`log.txt`).

| response | bytes | sha256 | result |
|---|---|---|---|
| o-m1 | 3,408 | ad1a9e1be89e38c5ec9d6da5c207895d4a408d635b9bf9b7b3316d804f12ee7f | one COMPANY row: **ALTVCHID " 21"**, **ALTMSTID " 222"** (= the first attempt's reading) |
| o-c1 | 44,036 | 14fd798e88700df5a32ea6f9f0fc2c9bd3d354ecd0268d3e00307c4f94a53a31 | 17 VOUCHER |
| o-c2 | 44,036 | = o-c1 | 17 VOUCHER, the same |
| o-m2 | 3,408 | = o-m1 | 21 / 222 |

- **Step 6:** `o-c1` and `o-c2` are **byte-identical**. Marks are equal before and after.
- **Step 7**, counted with an XML parser (nodes named exactly `VOUCHER` under `ENVELOPE/BODY/DATA/COLLECTION`): the AlterIDs are 2, 3, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19 and 20.

| GUID suffix | ALTERID | DATE | type |
|---|---|---|---|
| -00000002 | 2 | 20250420 | Sales |
| -0000000c | 12 | 20250422 | Purchase |
| -00000007 | 7 | 20250425 | Sales |
| -00000003 | 3 | 20250510 | Receipt |
| -00000005 | 5 | 20250512 | Receipt |
| -00000008 | 8 | 20250515 | Receipt |
| -00000006 | 6 | 20250520 | Payment |
| -0000000a | 10 | 20250520 | Credit Note |
| -0000000b | 11 | 20250525 | Credit Note |
| -0000000d | 13 | 20250528 | Debit Note |
| -0000000e | 14 | 20250530 | Debit Note |
| -0000000f | 15 | 20250610 | Sales |
| -00000010 | 16 | 20250612 | Sales |
| -00000012 | 18 | 20250701 | Journal |
| -00000014 | 20 | 20250715 | Journal |
| -00000011 | 17 | 20250720 | Receipt |
| -00000013 | 19 | 20260610 | Sales |

**Recorded:** the voucher mark is 21, but no voucher in this window carries AlterID 21; the highest is 20. AlterIDs 1, 4 and 9 are not in the window either. The voucher date of 10 June 2026 is inside the window's end of 30 June 2026. What took the mark from 20 to 21 is not established from the lab's records.

**Close.** The book was shut by the Company menu > Shut, leaving no company loaded. Nothing was written. No dialog appeared. The evidence was zipped on the lab PC (sha256 8cd8873af53d40f1bfec45de9bcb28bb069979f9a87aa7062794b26ab2b6dbf4).
