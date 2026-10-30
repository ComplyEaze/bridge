# Results: #2142 drain repeat, `GET /status` after a read cut at 20 s, on BRIDGE CORPUS DENSE (synthetic), 30 Oct 2026 (UTC). **B4R (the first row's B4 again, its timed read on time): the timed `ledger_masters` went out 2.28 s after the block's end and answered in 0.764 s, within the at-rest threshold (2.815 s). With B2 of the first row (0.802 s), two decisive readings agree: a quick `/status` after a cut meant Tally was ready for a data read, for this read on this book. B5 and B6 (no probe until the cut read ended; the first about 1 s after it): the first probe waited, 10.79 s and 9.29 s, ending 11.86 s and 10.34 s after the cut. `/status` sent after a cut waits while Tally's work on the cut read runs on. The three quick probes that followed were each followed by a timed read that answered in 0.797 s and 0.784 s, within the threshold. No harmful case was seen (a quick first probe followed by a slow or refused timed read). Marks unchanged (328 / 29900).**

## The cut blocks
| block | the read | cut (read_end_t) | first probe: sent t (after the cut), time_total, end after the cut | third quick probe ends after the cut | timed `ledger_masters`: sent after the block's `end` line | its seconds | marks |
|---|---|---|---|---|---|---|---|
| B4R | curl exit 28, `000 0 20.010126` | 20.010 | 2.078 (sent 17.93 s before the cut, while the read was alive), 30.028 s, **ends 12.10 s after the cut** | 18.32 s | **2.28 s** | **0.764 s**, answered | 328 / 29900 |
| B5 | curl exit 28, `000 0 20.009366` | 20.009 | 21.079 (1.07 s after), **10.791 s, ends 11.86 s after the cut** | 18.04 s | **1.51 s** | **0.797 s**, answered | 328 / 29900 |
| B6 | curl exit 28, `000 0 20.011639` | 20.012 | 21.062 (1.05 s after), **9.288 s, ends 10.34 s after the cut** | 16.54 s | **1.68 s** | **0.784 s**, answered | 328 / 29900 |

- **At rest:** `ledger_masters` took 0.759 s (step 1) and 0.815 s (step 8); the threshold, from the slower, is the larger of 2 x 0.815 = 1.630 s and 0.815 + 2 = 2.815 s: **2.815 s**.
- **B4R's reading:** answered within the threshold: agrees with B2 (0.802 s). The drain as `/status` saw it: 12.10 s after the cut (B2 25.40 s, B4 8.86 s in the first row).
- **B5 and B6, the first probe:** each **waited** (1 s or more) and ended some seconds after the cut (11.86 s, 10.34 s): how long Tally's work ran on after this cut, as `/status` sees it. They agree. Each block's timed read, sent after three quick probes (7 s or more after the cut), answered within the threshold.

## At rest (A1, A2, A3: ten probes each)
| block | least | median | most |
|---|---|---|---|
| A1 | 0.003 s | 0.005 s | 0.046 s |
| A2 | 0.003 s | 0.004 s | 0.007 s |
| A3 | 0.003 s | 0.004 s | 0.005 s |

Every probe of every block: exit 0, status 200, 51 bytes. Each probe's time is in `tables.md`; each block's log is pushed whole.

## Checks
- **Sheet:** `lab/2142-drain-repeat` at 33d18f5e81a3c28c933c9182a81d6f1ef17d0447: `SHA256.txt` 5 of 5 equal; `probe.py` compiled (pycache outside the folder) and read whole: it differs from the first row's helper only by `--first-probe-after-cut` (its argument, its check, the read's end time noted by the reaper, the wait before the first probe); it sends only the read (C3's line with `--max-time 20`) and `GET /status` by the sheet's line.
- **The capture watcher** (rule 4) checks each second whether `probe.py` has exited and ends within 2 s of it (each block's watch log ends 1 s after the exit), capturing every 30 s on the way; nothing waits for a capture not yet due. No change seen in any capture.
- **Rule 1's check** (the run lead's): as recorded for the first row.
- **Step 0** (no company loaded): one tally.exe (PID 26300) owns 9001; `/status` 200; the exe hashed. THE COMPANY loaded alone by the arrow keys (Down 13, the highlight read before Enter); the Gateway: 1-Apr-25 to 31-Mar-26, current date Tuesday 31-Mar-2026.
- **Build:** `bridge_mcp.exe` (master 609776ae) sha256 4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44 before and after; version 0.6.0; `tools/list` holds `list_companies` and `ledger_masters`; `list_companies`: THE COMPANY only, `verified_tuple`, 20250401. Server exit 0, error stream empty.
- **Marks:** M0 to M4 all 328 / 29900.
- **Blocks:** each by the sheet's own `Start-Process` line from the sheet's folder; exit 0 every time; a capture before each block, the watcher's captures during it, and one straight after; rule 6's 120 s wait with a capture every 30 s and one `GET /status` 200 after every B block.
- **C10:** a capture at the Gateway (dates unchanged); Company > Shut; Select Company, none loaded; one tally.exe owns 9001; `/status` 200. No XML after the shut.

## Deviations
1. Tally's PID is 26300, not the first row's 13252: Tally was restarted at 690-R0's step G1 earlier this night. The block history is as the sheet describes (B4R the first heavy read after the book was loaded).

**Confidence:** one sitting: PARTIAL. Not covered: as the sheet lists.
