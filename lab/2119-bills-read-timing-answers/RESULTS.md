# Results: #2119 Bills Receivable read timing, 29 Oct 2026, on BRIDGE CORPUS DENSE through the 609 build (exe 4c9bace3…). Sent by curl with no client limit, the Receivable read to 31 March 2026 took 41.5 s (again: 36.7 s), and to 31 December 2025 30.4 s. The tool answered at 30 September 2025, where its Receivable pair was held 18.1 s and 19.4 s, just under its 20-second limit. It refused at 31 December 2025 and 31 March 2026, each time on a 770-byte post at seq 26 held 20,008 / 20,009 ms. Reads only; the marks did not move

The sheet is `lab/2119-bills-read-timing` at 6af5124cad185f971c5631a7fc334a86a8e6f63a.
- `SHA256.txt` matched 10 of 10.
- `gen_requests --check` printed exactly `gen_requests: ok (5 requests; Receivable 770 bytes, Payable 764 bytes)`.
- A fresh `--out` gave the same five sha256 values.

**Build.** Master 609776ae9f5cec48e6563f165480d3fe1adae445: run 40366984202, artifact 12670984633.
- `bridge_mcp.exe` 4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44, the same at the end.
- Server 0.6.0; `outstandings` and `egress_log` are listed.
- The data folder was new, and writes were off.

**Company.** BRIDGE CORPUS DENSE, 100012, d45bc1b0-e5e3-4261-b3b2-cce3915f42d3, `verified_tuple`, books from 20250401. Tally's current date was 31-Mar-2026 at the start and at the end.

**Marks.** 328 / 29900 at step 1, after the raw reads, and after the tool reads.

**Dialogs.** None. Tally's window was captured every 30 s during each request and wait.

## Raw reads (C3's line with the timing fields; no `--max-time`)
| read | as of | started (UTC) | first byte s | total s | bytes (= CONTENT-LENGTH) | BILLFIXED | sum of BILLCL |
|---|---|---|---|---|---|---|---|
| R1 | 20250630 | 02:38:34.971 | 11.464 | 11.489 | 3,381,626 | 7,432 | -336,858,579.88 |
| R2 | 20250930 | 02:39:16.366 | 24.439 | 24.517 | 6,835,092 | 14,976 | -690,485,110.56 |
| R3 | 20251231 | 02:39:52.983 | 30.317 | 30.384 | 10,295,012 | 22,520 | -1,058,455,999.60 |
| R4 | 20260331 | 02:40:37.061 | 41.472 | 41.543 | 13,679,490 | 29,900 | -1,425,845,144.50 |
| R5 = R4 | 20260331 | 02:41:31.175 | 36.633 | 36.734 | 13,679,490 (the same sha256 as R4) | 29,900 | the same |
| R6 = R1 | 20250630 | 02:42:11.726 | 9.037 | 9.054 | 3,381,626 (the same sha256 as R1) | 7,432 | the same |
| R7 Payable | 20260331 | 02:42:22.231 | 2.698 | 2.700 | 46 | 0 | 0 |

Every request uploaded 770 B (Payable 764 B), and every answer was 200. No read reached 300 s, so the climb ran to the end.

R1 equals the tool's earlier figure for that day: 7,432 bills, 336,858,579.88. The time grows with the open bills, at about 1.4 ms a bill; R4 is twice the tool's 20 s limit.

**Exit code.** For R1 the helper did not capture curl's exit code (PowerShell's process object returned none). Its error stream is 0 B, and the status, size and CONTENT-LENGTH agree. R2 to R7 exit 0.

## Tool reads (`outstandings`, no party, no detail; then `egress_log` limit 50)
| call | as of | sent (UTC) | seconds | answer | trail |
|---|---|---|---|---|---|
| T1 | 20250930 | 02:42:57 | 44.4 | **answered**: `open_bills_total` 14,976 (= R2), receivable 690,485,110.56 (= R2's sum), payable 0; evidence 14,112,326 B, 44,313 ms; the tool answer marked truncated (the 200,000-byte bound) | sent 48, failed 0, omitted 16; the Receivable pair at **seq 26 and 28**, 770 B, 6,835,092 B each, **held 18,096 and 19,424 ms**; the Payable pair at seq 34/36, 764 B, 1,461 / 2,474 ms |
| T2 | 20251231 | 02:44:05 | 21.3 | **refused**: `native_outstandings_read_failed`, cause `request_deadline_exceeded`, endpoint 127.0.0.1:9001 | sent 26, failed 1, omitted 0; **seq 26, post 770 B, held 20,008 ms, `request_deadline_exceeded`**; seq 1 to 25 answered in at most 7 ms each |
| T3 | 20260331 | 02:47:56 | 21.3 | **refused**, the same code and cause | sent 26, failed 1; **seq 26, post 770 B, held 20,009 ms** |

**As inferred.** With no party asked for, the held request is the first post of the Receivable pair at seq 26, not 32.

**After each refusal (rule 5).** The lab waited 180 s with six captures, then sent one `GET /status`. It answered 200 in 0.003 s after T2 and 0.002 s after T3. A quick 200 does not show that the abandoned read had finished.

**What the timings say.**
- **Under the limit, at about 15,000 bills.** R2 took 24.5 s by curl, but the tool's pair at the same day each took under 20 s.
- **Over the limit, at about 22,500 bills.** R3 took 30.4 s, and the tool was refused at that day.

**Confidence:** one sitting, PARTIAL until repeated.

**Not covered:**
- other days;
- the Payable report on a book with payables;
- the groups and ledger snapshot reads;
- Gold;
- any tool change.

## Files
- `answers/`: every tool answer whole, with UTC times.
- `client-log.txt`.
- `notes.txt` (UTC).
- `raw/`:
  - `times.txt`: per request, the start and end, curl's fields, CONTENT-LENGTH, size and sha256, and the `read.py` line;
  - each answer's headers.
- `trails/`: the three `outstandings` trails as tables.
- `checks/`: step 0 and close checks.
- `RAW-ANSWERS.txt`: the raw answers by size and sha256. They stay on the lab PC.
- `SCREENSHOTS.txt`: the captures by sha256. They stay on the lab PC.
