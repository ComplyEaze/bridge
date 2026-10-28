# Results: #485 part E, 28 Oct 2026. The ledger-scoped census answers in AlterID spans: DN Sales's day falls whole in span 1 and equals part D's 82; DN Party 001's year is 248 vouchers over four spans, equal to the unspanned read; its May movement sums to the committed trial balance's debit exactly

The sheet is `lab/485-ledger-census-spans` at 97fdc0635a436c28fa5811cc5f47619eaf3f4a13. Its `SHA256.txt` checks 15/15 on sha256 and bytes. `python -I check-request-ids.py requests` gave `census: ok 11, skipped 0, FAIL 0`.

Run straight after part D (`lab/485-ledger-filtered-vouchers-run-sheet` at 4e769a9c; its d03 answered in-band and equal to d02's 82 DN Sales vouchers), in the same sitting, on BRIDGE CORPUS DENSE (100012, synthetic). Reads only, by curl, one at a time. The sheet's [OWNER] steps were taken by the lab on the owner's word of 28 Oct. No dialog appeared. TallyPrime 7.2 Silver, one tally.exe (PID 13992) throughout; the About screen was not opened.

| Step | Request | Window | AlterID span | Bound | Answer | VOUCHER rows | logged send to answer |
|---|---|---|---|---|---|---|---|
| E1 | `e01` marks | | | 8,192 | 200, 3,406 B: one COMPANY row, the company's GUID, **ALTVCHID 29900**, ALTMSTID 328 | | 0.139 s |
| E2 | **`e02` (NEW SHAPE)** DN Sales | 15 May 2025 | (0, 8192] | 1,000,000 | 200, 201,862 B | **82** | 0.299 s |
| E3 | `e03` | 15 May 2025 | (8192, 16384] | 1,000,000 | 200, 2,926 B | 0 | 0.248 s |
| E3 | `e04` | 15 May 2025 | (16384, 24576] | 1,000,000 | 200, 2,926 B | 0 | 0.149 s |
| E3 | `e05` | 15 May 2025 | (24576, 29900] | 1,000,000 | 200, 2,926 B | 0 | 0.174 s |
| E5 | **`e06` (NEW SHAPE, a year)** DN Party 001 | 1 Apr 2025 to 31 Mar 2026 | (0, 8192] | 4,000,000 | 200, 165,380 B | **67** | 2.416 s |
| E6 | `e07` | the year | (8192, 16384] | 4,000,000 | 200, 168,216 B | 68 | 0.439 s |
| E6 | `e08` | the year | (16384, 24576] | 4,000,000 | 200, 170,738 B | 69 | 0.381 s |
| E6 | `e09` | the year | (24576, 29900] | 4,000,000 | 200, 110,020 B | 44 | 0.366 s |
| E7 | `e10` (d03's shape) DN Party 001 | the year | none | 8,000,000 | 200, 680,460 B | 248 | 0.502 s |
| E7 | `e11` (movement shape) DN Party 001 | 1-31 May 2025 | none | 8,000,000 | 200, 108,918 B | 21 | 0.277 s |
| E8 | `e01` marks again | | | 8,192 | 200, 3,406 B, **byte-equal to E1** | | 0.176 s |

GET /status answered 200 before E1 and after every request from e02 on. e03, e04 and e05 are byte-identical empty collections (sha256 52f69354...f40b). The sum of e06 to e09 (248) is under 1,000, so e10 and e11 were sent. A capture of the whole Tally window straight after e02, after e06 and after e11 showed the Gateway with the company and no dialog, pixel-identical to the capture before e01. The logged times run from the line before curl starts to the line after it returns, so they include the process start.

## The comparisons the sheet lists (`tables/e-comparisons.txt`)
- **e02 to e05 against part D's d03:** the union is 82 GUIDs and **equals d03's 82 exactly**. All 82 are in span 1 (ALTERID 3579 to 3660); spans 2 to 4 are empty.
- **e06 to e09 against e10:** the union is 248 GUIDs and **equals e10's 248 exactly**.
- **Spans and windows:** every row's ALTERID is inside its request's span, and its DATE inside its window. Every GUID begins with the company's GUID.
- **e11 against the census:** e11's 21 GUIDs **equal the census rows dated May 2025** (21) exactly.
- **e11 against the trial balance:** the 21 DN Party 001 lines sum to **-933428.20** on the debit side (ISDEEMEDPOSITIVE Yes, negative amounts) and 0 on the credit side. The committed one-month trial balance of this book (`statement_trial_balance_dense_month_live.utf16le.xml`, its parse record on master) gives DN Party 001 debit -933428.20 and an empty credit: **equal**.
- **Cost:** the four day spans took 0.299, 0.248, 0.149 and 0.174 s (one full, three empty). The year's first span took 2.416 s and the other three 0.366 to 0.439 s. Part D did not record the shipped census on its day, so there is no figure to set them against.
- **Size per row:** e02's 82 census rows take 201,862 B, against d03's 226,626 B for the same 82 vouchers with nine fields, so each census row carries much more than the fetch list's three fields.

The e11 voucher table (each ledger line's LEDGERNAME, AMOUNT and ISDEEMEDPOSITIVE, sorted by MASTERID) is `tables/e11-voucher-table.txt`.

C10: Company > Shut, then Select Company with none loaded; one tally.exe; 9001 LISTENING; /status 200, 51 B. The current date was unchanged. No XML request was sent after the shut.

## In this folder
- `answers/`: every answer whole with its headers.
- `send-log.txt`, `notes.txt` (UTC, one line per step), `tables/`.
- `compare.py` and `analyse-e.py`: the lab's reading of the saved answers by LAB-COMMON C4, and the comparisons.
- `SCREENSHOTS.txt` (name, bytes, sha256) and `SHA256.txt`.

Not committed: the captures, which stay on the lab PC.
