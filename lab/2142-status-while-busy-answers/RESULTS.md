# Results: #2142, `GET /status` while Tally builds a long read, on BRIDGE CORPUS DENSE (synthetic), 30 Oct 2026 (UTC). **`/status` waits behind the read on this Tally.** In B1 and B3, the one probe sent while the Bills Receivable read was being built (sent 2.0 s after the read) did not answer until the read had ended: it took 31.57 s and 30.11 s and ended 0.06 s and 0.03 s after the read's own end. Every probe sent after the read answered in 0.003 to 0.032 s. After the same read was cut at 20 s, `/status` stayed slow for 25.4 s (B2) and 8.9 s (B4) beyond the cut. In B2 the timed `ledger_masters`, sent 8.1 s after the block's end, answered in 0.802 s, within the at-rest threshold: a quick `/status` after the cut meant Tally was ready for a data read. B4's timed call went out 24.4 s after the block's end, past the sheet's 15 s, so its reading is not decisive (it answered in 0.801 s). Marks unchanged throughout (328 / 29900).

## The decisive reading (B1, B3)
| block | the read (curl `-w`) | read_end_t | probes sent more than 1.5 s before read_end_t | each: sent t, time_total, end | reading |
|---|---|---|---|---|---|
| B1 | exit 0, `200 13679490 33.540201` | 33.540 | 1 | 2.031 s, 31.573 s, 33.604 s | **waited** (ended no earlier than 0.5 s before the read's end, in 1 s or more) |
| B3 | exit 0, `200 13679490 32.121480` | 32.121 | 1 | 2.047 s, 30.105 s, 32.152 s | **waited** |

Both answers were 13,679,490 B, equal to CONTENT-LENGTH, sha256 5a406cab1a2fe7f6581591a0e910215c48cf380ce04f7558b0896f109a560f5a: R4's and R5's answer. B1 and B3 agree: every decisive probe waited, so `/status` waits behind the read on this Tally.

## The cut read (B2, B4)
| block | the read | cut (read_end_t) | slow probe after the cut ends at | drain (cut to that end) | third quick probe in a row ends at | timed `ledger_masters`: sent after the block's end line | its seconds | marks |
|---|---|---|---|---|---|---|---|---|
| B2 | curl exit 28, `000 0 20.014386` | 20.014 | 45.413 (probe 1, 43.382 s) | **25.40 s** | 51.721 (31.71 s after the cut) | 8.06 s | **0.802 s**, answered | 328 / 29900 |
| B4 | curl exit 28, `000 0 20.012673` | 20.013 | 28.873 (probe 1, 26.842 s) | **8.86 s** | 35.081 (15.07 s after the cut) | **24.39 s** (past 15 s: not decisive) | 0.801 s, answered | 328 / 29900 |

- **At rest:** `ledger_masters` took 0.859 s (step 1) and 0.914 s (step 9). The threshold, from the slower: the larger of 2 x 0.914 = 1.828 s and 0.914 + 2 = 2.914 s, so 2.914 s.
- **B2's reading:** answered in 0.802 s, within 2.914 s. A quick `/status` after the cut meant Tally was ready for a data read.
- **B4:** not decisive by the sheet (sent 24.39 s after the end line). It answered in 0.801 s, recorded.

## At rest (A1, A2, A3: ten probes each)
| block | least | median | most |
|---|---|---|---|
| A1 | 0.003 s | 0.004 s | 0.023 s |
| A2 | 0.003 s | 0.004 s | 0.008 s |
| A3 | 0.003 s | 0.003 s | 0.038 s |

Every probe of every block: exit 0, status 200, 51 bytes. Each probe's time is in `tables.md`, and each block's log is pushed whole.

## Checks
- **Sheet:** `lab/2142-status-while-busy` at ac1e8e5a60c7e6b20429c25a664eb5429735ea31: `SHA256.txt` 5 of 5 equal (sha256 and size); `probe.py` compiled (pycache outside the folder) and read whole: it sends only the read (C3's line, `--max-time 20` in the cut blocks) and `GET /status` by the sheet's line.
- **Rule 1's check** (the run lead's): "private record checked on 30 Oct 2026; no rule found; concurrent use not previously measured" (the sheet's words).
- **Step 0** (no company loaded): one tally.exe (PID 13252) owns 9001; `/status` 200; the exe hashed. THE COMPANY loaded alone by the arrow keys (Down 13 from the demo row, the highlight read before Enter); the Gateway: 1-Apr-25 to 31-Mar-26, current date Tuesday 31-Mar-2026.
- **Build:** `bridge_mcp.exe` sha256 4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44 before the first call and after the last. Over stdio with the host, port, a new empty data folder and terms accepted; writes off. `tools/list` holds `list_companies` and `ledger_masters`. `list_companies`: exactly THE COMPANY, `identity_state` `verified_tuple`, `books_from` 20250401. Stopped by its stop file: server exit 0, error stream empty.
- **Marks:** M0 / V0 328 / 29900 (step 1), M1 / V1 (B2's call), M2 / V2 (B4's call), M3 / V3 (step 9): all 328 / 29900.
- **Blocks:** each run by the sheet's own `Start-Process` line from the sheet's folder; exit 0 every time. A capture of Tally's window before each block, every 30 s during it (the watcher: no change seen in any block), and straight after it; rule 6's 120 s wait with a capture every 30 s, then one `GET /status` 200, after every B block.
- **C10:** a capture at the Gateway (dates unchanged), Company > Shut, Select Company with none loaded, one tally.exe owns 9001, `/status` 200. No XML after the shut.

## Deviations
1. **B4's timed `ledger_masters` went out 24.39 s after the block's `end` line,** past rule 5's 15 s: the lab's block runner waited for its watcher to end before the closing capture, and the watcher sleeps in 30 s steps. B2's went out at 8.06 s. Not repeated (rule 5).
2. The watcher's cadence is 30 s from its start, so the capture "every 30 s while it runs" includes the start capture and one or two in-block captures, then the closing one.
3. THE COMPANY's step 1 read is the tool's `list_companies` plus `ledger_masters`, as the sheet says (no raw MARKS).

**Confidence:** one sitting: PARTIAL. Not covered: other reads and books, a cut read other than this one, Gold, a second process sending at the same time, ComplyEaze Bridge's own transport, and the time between a cut and Tally's real end of work.
