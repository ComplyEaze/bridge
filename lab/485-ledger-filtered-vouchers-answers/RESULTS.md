# Results: #485, a ledger-filtered voucher collection, 26 Oct 2026. Part T answered in-band: the candidate's 7 rows equal the ground truth's Lab Party P1 vouchers, each with GUID, ALTERID and MASTERID. Part D stopped at d02, the ground-truth read, on its size bound. Part E was not run

The sheet is `lab/485-ledger-filtered-vouchers-run-sheet` at 04a50315cc9e08bf4a1d40aad0ce8e8f468a0119. Its `SHA256.txt` checks 11/11 OK on sha256 and bytes. `python -I check-request-ids.py requests` gave `census: ok 7, skipped 0, FAIL 0`.

Reads only, by curl, one request at a time. The owner was at the lab machine for the new-shape requests. The screen acts were loading and shutting each company, one key per action. No dialog appeared at any point.

TallyPrime 7.2, Silver, Licensed: this is the same tally.exe (PID 29692) that the built tool read on the same day. The About screen was not opened.

## Part T: BRIDGE BILLTRAIL LAB (100041)
The company was loaded alone, with no dialog. The Gateway showed period 1-Apr-25 to 31-Mar-26, current date Monday 1-Sep-2025, last entry 1-Sep-25. C2: one tally.exe; 9001 LISTENING; /status 200, 51 B.

| Step | Request | Bound | Answer | curl time_total |
|---|---|---|---|---|
| 1 | `t01` marks | 8,192 | 200, 3,400 B, sha256 0fe9f835…342c | 0.005 s |
| 2 | `t02` committed window, 1 Apr 2025 to 31 Mar 2026 | 500,000 | 200, 249,288 B, sha256 80c78964…87e8 | 0.064 s |
| 3 | /status 200; **`t03` candidate, Lab Party P1, same dates (NEW SHAPE)**; /status 200 | 200,000 | **200, 21,904 B**, sha256 9a90cab5…598f | **0.036 s** |
| 5 | `t01` marks again | 8,192 | 200, 3,400 B, **byte-identical to step 1** | 0.004 s |

A screenshot of the whole Tally window was taken straight after t03's answer. It showed the Gateway and no dialog.

### EXPECT (t03), as observed
- **It parses.** Its row element is `VOUCHER`, under ENVELOPE/BODY/DATA/COLLECTION, and it has **7 rows**.
- **Every row carries GUID, ALTERID and MASTERID, each non-empty.**
- **Against t02:** t02 holds 7 vouchers, and all 7 carry a `Lab Party P1` ledger entry. **t03's 7 GUIDs equal those 7 exactly**; none is missing and none is extra.
- **Cost on this tiny book:** t03 answered 21,904 B in 0.036 s, against t02's 249,288 B in 0.064 s.

The row table is in `tables/T-guid-comparison.json`.

C10 (Part T): Company > Shut. Then Select Company with none loaded; one tally.exe; 9001 LISTENING; /status 200, 51 B. No XML request was sent after the shut. Part T answered in-band, so Part D followed.

## Part D: BRIDGE CORPUS DENSE (100012), stopped at step 2
The company was loaded alone, with no dialog. The Gateway showed period 1-Apr-25 to 31-Mar-26, current date Tuesday 31-Mar-2026, last entry 31-Mar-26. /status 200, 51 B.

| Step | Request | Bound | Answer | curl time_total |
|---|---|---|---|---|
| 1 | `d01` marks | 8,192 | 200, 3,406 B: one COMPANY row, GUID d45bc1b0-e5e3-4261-b3b2-cce3915f42d3, **ALTVCHID 29900, ALTMSTID 328** | 0.004 s |
| 2 | `d02` committed window, 15 May 2025 | **1,000,000** | 200, **2,900,742 B**, sha256 9d72ea3c342187937a0126a400de210840fc877b3961e50282626dca40dd6f1d | 0.310 s |

**d02's answer is above its size bound: the stop rule (C3).**
- The answer is kept as received on the lab PC, listed here by size and sha256 with its headers in `answers/`, and **marked unused**: it was not parsed or compared.
- **Nothing more was sent**: no /status, no `d03`, no `d04` and no second marks read.
- Tally's screen after the stop showed the Gateway with the company and no dialog.

For the sheet's next version, as observed: 2,900,742 B for one day of 82 vouchers in the committed window shape. On the same book the same day, the build's own `vouchers` read carried an 82-voucher day in about 2.79 MB per part.

C10 (Part D, after the stop): Company > Shut. Then Select Company with none loaded; one tally.exe; 9001 LISTENING; /status 200, 51 B. Tally's current date was Tuesday 31-Mar-2026 throughout. No XML request was sent after the shut.

## Part E (`lab/485-ledger-census-spans` at 97fdc063)
Not run. Its condition is part D's `d03` matching the 82 DN Sales vouchers, and `d03` was never sent. Nothing was sent from its folder. Its `SHA256.txt` had checked 15/15 OK.

## In this commit
- `answers/`: every answer whole, with its headers, except `d02`'s body (headers only; the body stays on the lab PC, listed above by size and sha256). Also the C2 and C10 /status answers and observations.
- `send-log.txt`: every request, with request sha256, send and answer times (UTC), curl output, bytes, answer sha256 and bound; and each /status.
- `notes.txt`: UTC times, one line per step.
- `tables/T-guid-comparison.json`: the Part T comparison.
- `compare485.py`: the lab's comparison script, which parses text only.
- `SCREENSHOTS.txt` (name and sha256) and `SHA256.txt`.

Not committed: the screenshots and d02's body.
