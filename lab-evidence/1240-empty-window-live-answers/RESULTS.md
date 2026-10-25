# Results: the empty-window check by census on a live Tally (#1897 / #1240), 26 Oct 2026. Every answer the same on P and M; the request counts as predicted except D1, where P sent fewer than predicted. PARTIAL: one run on one release

The sheet is `lab-evidence/1240-empty-window-live` @ f25a179885672bdec60e9ae0f8499c75edbd1376 (`SHA256.txt` 7/7 OK). TallyPrime 7.2, Silver, not Education mode (from `tally_status` on both books), one `tally.exe` on port 9001. Reads only: nothing was keyed, imported, backed up or restored, and no setting changed. Tally was touched only for Select Company, Shut Company, the Day Book periods and screenshots, never while a call ran.

## Builds
| | What | Hashes (all checked before the first call) |
|---|---|---|
| **P** | PR #1897 head 3367a1b9, run 40182690146, artifact 12595393056; the owner approved the download | zip 61,202,241 B 9056a6d42adcfb73a0a331a31c971c4d2c1a3cee47c901766f050f6f6a37048b; `bridge-tally.mcpb` 16,387,518 B 35debe4fc9da85ff3b265612f98df16a77d1b4882e79a678f463eccc05132783 (= `archive_sha256`); `bridge_mcp.exe` 33,703,424 B **1f5dd92d43b3ffacb83d3bf72026be22794521f3bb35f481ae2ab9a7438993fd** (= `binary_sha256`); server 0.6.0 |
| **M** | master 2306d86d bundle (artifact 12553782422), already staged | `bridge_mcp.exe` **92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524** |

**PR head.** When the item started, #1897's head had moved from 3367a1b9 to 594af5da by a merge of master into the branch (today). The sheet makes a moved head a stop; the run's review ruled that P is defined by its hashes and is run as written. P is the artifact above.

**Route.** Each file was run as the sheet says: from cmd.exe in a new empty folder, `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, a new data folder per file, `BRIDGE_TERMS_ACCEPTED=true`, redaction and posting settings unset, `bridge_mcp.exe < filled > .out 2> .err`. Every `.err` is empty. Every filled file differs from its template on the placeholder line only (`calls/*/filled-diff.txt`). The raw company-list and marks reads were sent with curl (`send-log.txt`); the marks requests are `c1-marks-TEMPLATE` (4bcdcfd4…) filled with the company and GUID (`requests/`).

**Exit codes.** Captured for every vouchers call (all 0). Not captured for the first `a-list` call (B0): the lab's launcher did not hold the process handle; it was fixed before the next call. That call's answer is complete.

## Part B: BRIDGE BATCH LAB (counted in one request)
**B0.** Loaded alone. Company list: one COMPANY, 100050, `a30f23c3-4819-4334-96ab-ffa6fba70976`. Marks M-B0: ALTVCHID **87**, ALTMSTID 224, LastVoucherDate 20260821 (82/224 on 12 October; recorded, not a stop; well above 42, so the book is counted). `list_companies`: the same one company; `tally_status`: 7.2, silver, education false.

**B1, the oracle (Day Book):** 20 Jul 10 vouchers (Contra 11, 12; Payment 21 to 24; Receipt 22 to 25), 21, 22 and 23 Jul none; 9 to 13 Nov none; 18 Aug one (Journal 2), 19 and 20 Aug none, 21 Aug one (Lab Sales Bills LSB/0001). The same as the 12 October book, so the EXPECT lines stood.

**B2 and B4:**
| Window | Build | state, reason | items, total | census | `sent` (failed, omitted) | Expected `sent` | seconds | `evidence.bytes` |
|---|---|---|---|---|---|---|---|---|
| W1 21-22 Jul | P | partial, `empty_window_wider_read_nonempty` | 0, 0 | 1 request | **28** (0, 0) | 28 | 3.1 | 81,232 |
| W2 10-12 Nov | P | partial, `empty_uncorroborated` | 0, 0 | 1 | **34** (0, 2) | 34 | 4.4 | 39,708 |
| W3 19-20 Aug | P | partial, `empty_window_wider_read_nonempty` | 0, 0 | 1 | **28** (0, 0) | 28 | 2.6 | 42,628 |
| W1 | M | the same | 0, 0 | 1 | **34** (0, 2) | 34 | 3.5 | 692,716 |
| W2 | M | the same | 0, 0 | 1 | **40** (0, 8) | 40 | 3.8 | 45,752 |
| W3 | M | the same | 0, 0 | 1 | **34** (0, 2) | 34 | 3.4 | 169,552 |

Each answer's window: census 1 request, marks 1 request, one part of 3,022 bytes with 0 rows. Marks after every call and M-B1, M-B2: 87/224, equal to M-B0. A whole-window screenshot after each call showed the Gateway with no dialog.

## Part C: BRIDGE CORPUS DENSE (counted in four requests)
**C0.** Loaded alone. Company list: one COMPANY, BRIDGE CORPUS DENSE, 100012, `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`. The fixture the sheet names for the number (`company_identity_fields_after_split.utf8.xml`) is not in the repository at the lab's checkout; the number is the one the company list and Select Company show, and the GUID matches. Marks M-C0: ALTVCHID **29,900**, ALTMSTID 328, LastVoucherDate 20260331. `list_companies` one company; `tally_status` 7.2, silver, education false.

**C1.** `L` = 31 Mar 2026 from LastVoucherDate. The Day Book for March 2026 ends on 31-Mar-26, Sales 29900. They agree.

**C2.** The Day Book period 31-3-2026 to 4-5-2026 was accepted by Tally and shows 82 rows, all on 31-Mar-26 (29819 to 29900), and none after. So `L`+1 to `L`+34 are empty. D1 = 2026-04-01 to 2026-04-02; D2 = 2026-04-30 to 2026-05-02.

**C3 to C5:**
| Window | Build | state, reason | census | `sent` (failed, omitted) | Expected | seconds | `evidence.bytes` | largest retained `response_bytes` |
|---|---|---|---|---|---|---|---|---|
| D1 | P | partial, `empty_window_wider_read_nonempty`, 0 items | 4 requests | **70** (0, 38) | equal to M | 6.9 | 475,228 | 202,614 |
| D2 | P | partial, `empty_uncorroborated`, 0 items | 4 | **76** (0, 44) | equal to M | 7.7 | 82,856 | 4,012 |
| D1 | M | the same | 4 | **88** (0, 56) | | 8.9 | 6,075,936 | 1,432,706 |
| D2 | M | the same | 4 | **76** (0, 44) | | 6.8 | 82,088 | 4,012 |

Marks M-C1 to M-C4: 29,900/328, all equal to M-C0. No `voucher_window_changed_during_read`, no deadline or size refusal, no dialog.

**D1 is a finding against the prediction.** The sheet expected equal counts on a book counted in four requests (M reads the widened window whole; P sends a second marks read instead). P sent 18 fewer requests than M (70 against 88) and read about a thirteenth of the bytes (475 KB against 6.08 MB), with the same answer. The trail keeps only the last 32 sends (P omitted 38, M 56), so the retained tails cannot say which requests M's extra 18 were. M's largest retained answer (1,432,706 bytes) is consistent with it reading the widened window's vouchers in several parts; that is not established from the trail. D2, where the widened window is empty, gave equal counts as predicted.

**C6 (C10).** DENSE shut; Select Company with no company loaded; the company list read afterwards returned no COMPANY row; one `tally.exe`.

## What this run decides
1. **The answer stays the same on a real Tally** for all five windows (W1 to W3, D1, D2): state, reason, items and total equal between P and M, on TallyPrime 7.2.
2. **On a book counted in one request, P needs fewer requests:** 28 against 34 (W1, W3) and 34 against 40 (W2), exactly the counts predicted from the test doubles and the 12 October M counts.
3. **On a book counted in four requests, the answer is the same and the bracket held** (marks unchanged, no changed-during-read refusal). P's count was lower than M's for D1 (not equal, as predicted) and equal for D2.

## In this commit
- `RESULTS.md`, `notes.txt` (UTC times, one line per step), `send-log.txt` (every raw read: request sha256, bytes, answer sha256), `call-log.txt` (every file run: build, exit, seconds, sizes, filled sha256).
- `answers/`: every company-list and marks answer with its headers (CMPINFO blocks kept).
- `calls/<call>/`: the filled file and its diff record, the `.out` and `.err`, and the data folder the run wrote (`agent-egress.jsonl`, `terms-acceptance.jsonl`).
- `requests/`: the two filled marks requests.
- `SCREENSHOTS.txt`: every screenshot by name, size and sha256; the images stay on the lab PC.

Not measured here, as the sheet says: `voucher_presence`, `ledger_movement` and the registers by name, a window holding a voucher, Education mode, Gold, a client book, a count that moves during the read, and the time saved (timed once on one machine).
