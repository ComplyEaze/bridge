# Results: #485 part D, second sitting, 28 Oct 2026. The ledger-filtered collection answers the dense day in-band: d03's 82 rows equal the ground truth's 82 DN Sales vouchers, each with GUID, ALTERID and MASTERID, at 226,626 B against the window's 2,900,742 B. The week answers 574 rows

The sheet is `lab/485-ledger-filtered-vouchers-run-sheet` at 4e769a9c9dc1eaf875e1bf397c0c4859510a90ab, which sizes d02's bound from the 26 Oct measurement (6,000,000 bytes). Its `SHA256.txt` checks 11/11 on sha256 and bytes. `python -I check-request-ids.py requests` gave `census: ok 7, skipped 0, FAIL 0`. Part T stands as recorded on 26 Oct in this branch's first commit.

Reads only, by curl, one request at a time, on BRIDGE CORPUS DENSE (100012, synthetic). The sheet's [OWNER] steps were taken by the lab on the owner's word of 28 Oct. No dialog appeared, and Tally needed no restart. TallyPrime 7.2 Silver, one tally.exe (PID 13992) throughout; the About screen was not opened.

| Step | Request | Bound | Answer | logged send to answer |
|---|---|---|---|---|
| 1 | `d01` marks | 8,192 | 200, 3,406 B: one COMPANY row, GUID d45bc1b0-e5e3-4261-b3b2-cce3915f42d3, **ALTVCHID 29900, ALTMSTID 328** | 0.112 s |
| 2 | `d02` committed window, 15 May 2025 | 6,000,000 | 200, 2,900,742 B, sha256 9d72ea3c...6f1d (the same bytes as on 26 Oct); 82 VOUCHER rows | 1.193 s |
| 3 | /status 200; **`d03` candidate, DN Sales, 15 May 2025 (NEW SHAPE)**; /status 200 | 1,000,000 | **200, 226,626 B**, sha256 e400170a...0f3e | **0.286 s** |
| 4 | /status 200; `d04` candidate, DN Sales, 12-18 May 2025; /status 200 | 4,000,000 | 200, 1,568,802 B, sha256 8d53bf27...b702 | 0.683 s |
| 6 | `d01` marks again | 8,192 | 200, 3,406 B, **byte-equal to step 1** | 0.243 s |

The logged times run from the line before curl starts to the line after it returns, so they include the process start; they are for comparing these requests with each other.

A capture of the whole Tally window was taken straight after d03 and after d04. Each showed the Gateway with the company and no dialog, pixel-identical to the capture before d01.

## EXPECT, as observed
- **d03 parses.** Its row element is `VOUCHER`, under ENVELOPE/BODY/DATA/COLLECTION: **82 rows**, every one with GUID, ALTERID, MASTERID and DATE non-empty. ALTERID 3579 to 3660, all dated 20250515.
- **Against d02:** d02 holds 82 vouchers, and all 82 carry a `DN Sales` ledger line. **d03's 82 GUIDs equal those 82 exactly**: none missing, none extra.
- **d04:** **574 rows**, the figure the sheet gives for the week (recorded only; there is no ground-truth read for the week). 574 distinct GUIDs, every identity field non-empty, dates 20250512 to 20250518.
- **Cost, same day:** d03 226,626 B in 0.286 s, against d02's 2,900,742 B in 1.193 s (7.8% of the bytes).
- **Marks:** equal at the start and the end of the part.

The tables are in `tables/d-comparison.txt`.

C10: Company > Shut, then Select Company with none loaded; one tally.exe; 9001 LISTENING; /status 200, 51 B. Tally's current date was Tuesday 31-Mar-2026 throughout. No XML request was sent after the shut. Part E's condition is met, and part E ran next (`lab/485-ledger-census-spans-answers`).

## In this folder
- `answers/`: every answer whole with its headers, d02's body included.
- `send-log.txt`: each request with its sha256, the send time, curl's output, CONTENT-LENGTH, bytes and the answer's sha256, then the time after the answer; and each /status.
- `notes.txt`: UTC, one line per step.
- `tables/d-comparison.txt`, from `compare.py` (the lab's reading of the saved answers by LAB-COMMON C4).
- `SCREENSHOTS.txt` (name, bytes, sha256) and `SHA256.txt`.

Not committed: the captures, which stay on the lab PC.
