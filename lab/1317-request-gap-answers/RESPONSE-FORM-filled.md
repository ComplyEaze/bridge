# #1317 screen response form, filled by the lab by the timing rule (one column per pass)

The lab did the actions itself through its screen tool. t0 is the time Windows recorded for the action's key or click; t1 is the first capture showing the action's end state. Latency = t1 - t0. Captures were taken back to back by a read-only timer; R (the median interval between two captures) was 0.104 s in B0. Excess E = latency - B0's latency for that action. Class: I if E <= max(0.5 s, R); L if E is between that and 3 s; F if E > 3 s.

| # | action (from the Gateway of Tally) | B0 (idle) | A1 (500) | B1 (250) | A2 (500) | B2 (250) |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Open Display More Reports > Day Book | t0 21:43:44.307, t1 21:43:44.988, 0.681 s | not performed as listed: the click on Display More Reports (21:50:32.165) was not acted on; the following Down (21:50:34.398, drawn at 34.519) moved the Gateway highlight to Dashboard and Enter (21:50:36.494) opened the **Dashboard**, first drawn at 21:50:48.554: **12.06 s**, an unlisted screen; the run stopped there | not run | not run | not run |
| 2 | Press Page Down five times | 21:43:52.585 → 52.907, 0.322 s | not reached | not run | not run | not run |
| 3 | Escape back to the Gateway (two Escapes, one second apart) | 21:43:58.223 → 59.437, 1.214 s (includes the 1 s pause) | not reached | not run | not run | not run |
| 4 | Open Balance Sheet | 21:44:13.867 → 14.397, 0.530 s | not reached | not run | not run | not run |
| 5 | Escape back to the Gateway | 21:44:21.845 → 22.544, 0.699 s | not reached | not run | not run | not run |
| 6 | Open Profit & Loss A/c | 21:44:28.042 → 28.551, 0.509 s | not reached | not run | not run | not run |
| 7 | Escape back to the Gateway | 21:44:34.736 → 34.896, 0.160 s | not reached | not run | not run | not run |
| 8 | Open Trial Balance | 21:44:52.834 → 53.187, 0.353 s | not reached | not run | not run | not run |
| 8b | then Escape back to the Gateway (two Escapes, one second apart) | 21:44:59.536 → 45:00.763, 1.227 s (includes the 1 s pause) | not reached | not run | not run | not run |

Notes (with the time):
- 21:50:36.494 to 21:50:48.554 (A1): while Tally drew the Dashboard, Windows showed its stand-in for a window that is not responding; the screen tool reported an unidentified application in front and refused the next key. Census request c19, sent at 21:50:37.077, was held 11.508 s (every other A1 request 0.178 to 0.378 s) and answered at 21:50:48.585, 31 ms after the Dashboard's first capture.
- 21:50:32.165 to 32.345 (A1): the click that was not acted on fell while census request c14 (sent 32.218, held 0.188 s) was being answered.
- 21:51:06.6: the stop file was created; the driver sent nothing after c34.
