# Results: #1029 item 2, a seven-day voucher range read in one call on BRIDGE CORPUS DENSE, 26 Oct 2026. PASS: one `vouchers` call over 1 to 7 April 2025 answered `complete` with total 574, equal to Tally's own count, in 12.1 s, through 4 census reads and 8 single-day parts

The sheet is `lab/1029-dense-range-read` at 99b7f841ddefa4d06baab13992fcff6ee7b7f5c5. Its `SHA256.txt` checks 3/3 OK on sha256 and bytes.

Reads only; no owner act was needed. No request file was sent: every read was a call of the built tool, one at a time. The screen acts were loading and shutting the company and reading the Statistics and Day Book reports, one key per action. No dialog appeared at any point.

The build is the staged `w1-2306d86d`. The client checked `bridge_mcp.exe` sha256 **92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524** before every call, and it equals the sheet's. It ran over stdio with:
- `BRIDGE_TALLY_HOST=127.0.0.1`, port 9001;
- one new empty data folder;
- `BRIDGE_TERMS_ACCEPTED=true`;
- posting, import and redaction unset.

## Step 0
- **Load:** BRIDGE CORPUS DENSE was loaded alone, with no dialog. The Gateway showed period 1-Apr-25 to 31-Mar-26, current date Tuesday 31-Mar-2026, last entry 31-Mar-26.
- **C2 step 0, without its MARKS request:** one tally.exe; 9001 LISTENING; /status 200, 51 B.
- **`list_companies`:** exactly THE COMPANY: 100012, GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, `books_from` 20250401, `verified_tuple` (= EXPECT).
- **`tally_status`:** TallyPrime 7.2, Silver, `education_mode` false.
- **`ledger_masters`** (first page, no `snapshot_id`): **M0 328, V0 29,900**, `reused` false.

## Step 1: the oracle on Tally's screen
| Report and period | What it shows |
|---|---|
| Statistics, as opened (1-Apr-25 to 31-Mar-26) | Sales 29,900; every other type 0; Total 29,900 |
| Statistics, Alt+F2 1-4-2025 to 7-4-2025 | **Sales 574; every other type 0; Total 574** |
| Day Book, Alt+F2 1-4-2025 to 7-4-2025, first row | 1-Apr-25, DN Party 002, Sales, Vch No. **1** |
| Day Book, same period, last row (Ctrl+End) | 7-Apr-25, DN Party 095, Sales, Vch No. **574** |

Statistics was reached by Go To ("Statistics", Common Reports > Statistics) and the Day Book by Go To ("Day Book", Common Reports > Day Book). No row was opened. Every voucher is of one type and the numbers run 1 to 574, so 574 - 1 + 1 = 574: **N = 574**, and the two screens agree.

Both periods were set back as the screens offered: the Day Book to 31-Mar-26 alone, as it opened, and Statistics to 1-Apr-25 to 31-Mar-26. Tally's current date was Tuesday 31-Mar-2026 before and after.

## Step 2: `voucher_read_cost`
`voucher_mark` 29900. For `vouchers` under `per_call`:
- `census_reads` **4**, as expected for this mark;
- `floor_seconds` **1** (`floor_basis` derived);
- `state` counted;
- `request_spacing_ms` 500.

The whole answer, `03-voucher_read_cost.json` (9,599 bytes, sha256 b0b3aaf8794e2b454f219db655ac21b28a601219dc7739df761727737768c343), is in `answers/`. It was added in a second commit: its `host_limits` list names the host apps by the tool's own product field values. Its other fields are given above. Its `measured` block quotes the two calls from protocol reference 11c.5 (34.3 s and 104.7 s, `not_a_bound`).

## Step 3: `vouchers`, 2025-04-01 to 2025-04-07, offset 0, one call
- **Wall clock (the client's tools/call round trip): 12.113 s.** The answer's own `evidence.duration_ms` is 11,837.
- `state` **complete**, `total` **574**. The first page holds 145 items (`next_offset` 145, `truncated` true): the page is trimmed to the tool's byte limit, and `total` counts the window.
- `result.window.census`: **4 requests**, 3,065 ms. `result.window.marks`: 2 requests, 994 ms.
- `snapshot` 328 / 29900, `reused` false. `evidence.bytes` 41,911,884, state complete.
- No refusal or stop code anywhere in the answer: no `voucher_window_part_not_admitted`, `request_deadline_exceeded`, `response_size_limit_exceeded`, `voucher_window_volume_unestimated`, `voucher_window_part_over_budget` or `voucher_window_too_many_reads`.

`result.window.parts`, as answered:

| Part | Day | After / through | Rows | Bytes | ms |
|---|---|---|---|---|---|
| 1 | 20250401 | 0 / 42 | 42 | 1,430,724 | 824 |
| 2 | 20250401 | 42 / 29900 | 40 | 1,362,946 | 785 |
| 3 | 20250402 | | 82 | 2,791,512 | 988 |
| 4 | 20250403 | | 82 | 2,791,772 | 931 |
| 5 | 20250404 | | 82 | 2,792,330 | 949 |
| 6 | 20250405 | | 82 | 2,792,338 | 935 |
| 7 | 20250406 | | 82 | 2,792,338 | 931 |
| 8 | 20250407 | | 82 | 2,792,338 | 936 |
| | | | **574** | 19,546,298 | 7,279 |

Every part covers a single day. Recorded: the first day was read in two parts (42 and 40 rows), and each of the other six days in one part of 82 rows, about 2.79 MB.

**`egress_log`** (limit 40, same server session): the call's `response_prepared` record has:
- `request_trail`: `sent` 88, `failed` 0, `omitted` 56, with the last 32 kept (21 posts, 11 status checks; the largest response kept is 2,792,338 B);
- `bytes_prepared` 199,077 and `rows_prepared` 145.

Tally's screen showed the Gateway, with no dialog, after the call.

## Step 4: `read_evidence` (limit 40)
`read_evidence` returns the server's in-memory records, so it was called in the same server session as step 3, after its `egress_log`. Its 2 records:
- **the step 3 call, state complete**: `request_sha256` 889df0f6…084b and `response_sha256` 828659a7…cad5, bytes 41,911,884, `duration_ms` 11,837. These are equal to step 3's own `evidence`.
- the `egress_log` call, state complete.

## Step 5: marks
`ledger_masters`: **M1 328, V1 29,900 = M0, V0**, `reused` false.

## EXPECT, as observed
| EXPECT | Observed |
|---|---|
| `state` complete; `total` = N | complete; 574 = 574 |
| no `voucher_window_part_not_admitted` | none |
| census request count = step 2's `census_reads` | 4 = 4 |
| every part covers a single day | all 8 parts, each one day |
| wall-clock seconds not less than `floor_seconds` | 12.113 ≥ 1 |
| step 4 holds the step 3 call in state complete | yes, matching fingerprints |

**PASS**: steps 0 to 5 are recorded whole and every EXPECT line holds.

C10: Company > Shut. Then Select Company with none loaded; one tally.exe; 9001 LISTENING; /status 200, 51 B. Tally's current date at the shut was Tuesday 31-Mar-2026, as at step 0. No request was sent after the shut.

## Not measured (as the sheet says)
#1029 item 1 and item 3; a window crossing a month or a financial year; Gold; Education mode.

## In this commit
- `answers/`: every tool answer whole, with the client's timing; step 3's `egress_log` answer; the `read_evidence` answer; the C2 and C10 /status answers and the C10 observation.
- `notes.txt`: UTC times, one line per step.
- `cost-log.txt`: the client's one-line log of each call.
- `call1029.py`: the lab's client, not product code.
- `SCREENSHOTS.txt` (name and sha256) and `SHA256.txt`.

Not committed: the screenshots and the client's data folder.
