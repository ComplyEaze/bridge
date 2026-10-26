# Results: #1331, the scoped compliance listing in its production shape, and its cost on a large book, 26 Oct 2026. Both EXPECTs met; on BRIDGE CORPUS DENSE the scoped listing saves little, because 121 of its 123 ledgers sit under the four parents

The sheet is `lab/1331-scoped-listing-run-sheet` at 6758758c482bc57edea209b9260d959cbe9dee70. `SHA256.txt` checks 9/9 OK on sha256 and bytes. `python -I check-request-ids.py requests` gave `census: ok 5, skipped 0, FAIL 0`.

The owner gave the go-ahead in the lab's chat: reads only, no keying. Every request was sent by curl, one at a time, as the sheet allows. The only screen acts were loading and shutting each company, one key per action.

TallyPrime 7.2, Silver, Licensed, not Educational mode. This was read through the built tool earlier the same day on the same tally.exe (PID 29692). The About screen was not opened.

No dialog appeared at any point. A whole-window screenshot was taken after p02 and after the first d03, both new shapes; both show the Gateway. Every answer stayed within its bound. Neither book changed: the marks after equal the marks before, byte for byte.

The reference is the 10 October `q1` answer: `a-q1-rates-four-parents.xml`, 44,418 bytes, sha256 23a0f525257dbfe50b68fb4aafca9ab4f72b7e09e72c67cdc2808f0cf65109bb, which equals its own `SHA256.txt`. It was read from commit b5c0ff86ceecf4fdd49a35f425ee6dd22d34e876 and is included here under `q1-10oct/`.

## Part P: BRIDGE PILOT LAB (100033)
The company was loaded alone. The Gateway showed period 1-Apr-26 to 31-Mar-27, last entry 15-Sep-26, and the footnote "(e) Data exceptions exist", which is a Gateway note, not a dialog.

| Step | Request | Bound | Answer | Time (send to answer) |
|---|---|---|---|---|
| 1 | company list (C2) | 65,536 | 4,000 B; one COMPANY 100033, GUID **6b43e498-430c-4d5c-bfef-d32e2ab93c85** (= EXPECT) | |
| 1 | p01 marks: **M_P0** | 8,192 | 3,394 B; **ALTVCHID 68, ALTMSTID 245** (35 and 234 on 10 Oct: the book has grown, recorded) | |
| 2 | **p02**, the production parent-part listing | 200,000 | **28,368 B**, sha256 094dc0cd…8b08 | 11:02:18.269Z to 11:02:18.346Z; curl time_total 0.038835 s |
| 3 | p01 marks: **M_P1** | 8,192 | 3,394 B; 68 / 245, **byte-identical to M_P0** | |

### EXPECT (p02 against q1), as observed
- **All 11 GUIDs of q1 are present.** For each one the ALTERID is unchanged (215 to 224, and 234), and **every element both rows hold is identical**.
- The only elements q1 has and p02 lacks are **GSTDETAILS.LIST, RATEOFTAXCALCULATION, ROUNDINGLIMIT and ROUNDINGMETHOD**. These come from q1's GST-details fetch, which the production listing does not ask for; ROUNDINGMETHOD appears on the four duty ledgers only.
- p02 has **two rows not in q1**, both under Sales Accounts, which is one of the four parents. They are newer ledgers, recorded and not a finding:

  | Ledger | GUID suffix | ALTERID |
  |---|---|---|
  | BRIDGE Svc 4D 5% | -000000e5 | 243 |
  | BRIDGE Svc INH 5% | -000000e3 | 242 |

- **No row's parent is outside the four.** The parents as returned are Duties & Taxes, Indirect Expenses, Sales Accounts and Sundry Debtors.
- The company GUID taken from every row's GUID is **PILOT LAB's**.

The per-GUID table is in `tables/P-guid-comparison.json`.

C10 (Part P): Company > Shut. Then Select Company with none loaded; one tally.exe; 9001 LISTENING; /status 200 51 B. No XML request was sent after the shut.

## Part D: BRIDGE CORPUS DENSE (100012)
The company was loaded alone. The Gateway showed period 1-Apr-25 to 31-Mar-26, so the books' first date is 1-Apr-25, and last entry 31-Mar-26. Company list: one COMPANY 100012, GUID d45bc1b0-e5e3-4261-b3b2-cce3915f42d3.

| Step | Request | Answer | Time (curl time_total) |
|---|---|---|---|
| 1 | d01 marks | **ALTVCHID 29,900, ALTMSTID 328** | |
| 2 | /status 200; **d02** whole listing, run 1 | **231,300 B**, 123 rows | 0.033536 s (11:03:40.274Z to .356Z) |
| 3 | /status 200; **d03** four parents, run 1 | **227,558 B**, 121 rows | 0.030379 s (11:03:40.391Z to .475Z) |
| 4 | /status 200; d02 run 2 | 231,300 B, byte-identical to run 1 | 0.030977 s |
| 4 | d03 run 2 | 227,558 B, byte-identical to run 1 | 0.027102 s |
| 5 | d01 marks again | 29,900 / 328, **byte-identical to step 1** | |

### EXPECT (d03 against d02), as observed
- d02's 123 rows by parent: Sundry Debtors 120, Cash-in-Hand 1, Sales Accounts 1, Primary 1.
- **d03's 121 GUIDs equal exactly the 121 d02 rows whose parent is one of the four** (case folded). None is missing and none is extra, and every one of the 121 rows is byte-identical to its d02 row.

The table is in `tables/D-guid-comparison.json`.

### Recorded for the build (no EXPECT)
- Ledgers: **123** (d02's rows), against the invoice build's limit of 2,000.
- Master mark: **ALTMSTID 328**, against the limit of 5,000.
- Voucher mark: **ALTVCHID 29,900**.

**Observation on cost.** On this book the scoped listing is only 3,742 bytes (1.6%) smaller than the whole listing, and about 3 ms faster per run. That is because 121 of its 123 ledgers are under the four parents (120 under Sundry Debtors). CORPUS DENSE is large in vouchers, not in ledgers outside those parents. So this row **does not show the saving the scoped form would give on a book with many ledgers outside the four parents**; measuring that needs such a book.

C10 (Part D): the same as Part P.

## In this commit
- `notes.txt`: UTC times, one line per step.
- `send-log.txt`: every raw read, with request sha256, send and answer times, curl output, bytes, answer sha256 and bound.
- `answers/`: every answer whole, with headers; the C2 and C10 observations and /status answers. d02 is 231,300 bytes, under the sheet's 2,000,000-byte threshold, so it is committed whole.
- `requests/`: the sent request files, copies of the sheet's plus LAB-COMMON's c0 company list.
- `q1-10oct/`: the reference answer.
- `tables/`: the two GUID comparisons.
- `compare.py`: the lab's comparison script, which parses text only.
- `SCREENSHOTS.txt` (name and sha256) and `SHA256.txt`.

Not committed: the screenshots, which stay on the lab PC.
