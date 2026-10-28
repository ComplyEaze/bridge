# #1317 screen response form, filled by the lab by the timing rule (one column per pass)

How the form was filled:
- The lab did the actions itself through its screen tool: key presses only, after one focus click per pass.
- t0 is the first input Windows recorded in the action's group (for a burst, its first arrow press).
- t1 is the first capture showing the action's end state; latency = t1 - t0.
- R (the median interval between two captures) was 0.115 s in B0 and 0.116 to 0.118 s in the runs.
- E = latency - B0's latency for that action.
- Class: **I** if E <= max(0.5 s, R) = 0.5 s; **L** if E is above that and no more than 3 s; **F** if E is above 3 s or the end state was not reached in 10 s.
- "flight" names the census requests in flight between t0 and t1 (from the run's `log.tsv`).
- "no flight" means no request was in flight; only cells with a request in flight count towards the question.

| # | action (from the Gateway of Tally) | B0 (idle) | A1 (500) | B1 (250), partial | A2 (500) | B2 (250) |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Open Display More Reports > Day Book | 10:45:12.540 → 13.347, 0.807 s | 10:49:14.935 → 15.800, 0.865 s, E 0.058, **I**, flight c29 | 10:53:32.801 → 33.869, 1.068 s, E 0.261, **I**, flight c39, c40 | 10:57:43.477 → 44.981, 1.504 s, E 0.697, **L**, flight c31 | 11:01:51.585 → 52.351, 0.766 s, E -0.041, I, no flight |
| 2 | Press Page Down five times | 10:45:19.761 → 19.961, 0.200 s | 10:49:22.748 → 22.972, 0.224 s, E 0.024, **I**, flight c36 | 10:53:39.578 → 39.654, 0.076 s, I, no flight | 10:57:53.450 → 53.527, 0.077 s, E -0.123, **I**, flight c38 | 11:01:59.868 → 02:00.242, 0.374 s, E 0.174, I, no flight |
| 3 | Escape back to the Gateway (two Escapes) | 10:45:29.662 → 32.764, 3.102 s (2.649 s between the Escapes) | 10:49:31.678 → 34.517, 2.839 s, I, no flight | 10:53:48.015 → 51.144, 3.129 s, I, no flight | 10:58:01.406 → 04.635, 3.229 s, I, no flight | 11:02:08.681 → 11.515, 2.834 s, I, no flight |
| 4 | Open Balance Sheet | 10:45:39.386 → 39.812, 0.426 s | 10:49:41.746 → 42.224, 0.478 s, I, no flight | 10:53:58.551 → 59.100, 0.549 s, I, no flight | 10:58:12.966 → 13.521, 0.555 s, I, no flight | 11:02:18.902 → 19.428, 0.526 s, I, no flight |
| 5 | Escape back to the Gateway | 10:45:46.537 → 47.195, 0.658 s | 10:49:48.456 → 48.698, 0.242 s, I, no flight | 10:54:06.466 → 06.679, 0.213 s, I, no flight | 10:58:19.701 → 19.950, 0.249 s, I, no flight | 11:02:26.201 → 26.437, 0.236 s, I, no flight |
| 6 | Open Profit & Loss A/c | 10:45:53.632 → 54.008, 0.376 s | 10:49:56.968 → 57.307, 0.339 s, I, no flight | 10:54:14.367 → 14.762, 0.395 s, I, no flight | 10:58:27.202 → 27.522, 0.320 s, I, no flight | 11:02:33.494 → 33.817, 0.323 s, I, no flight |
| 7 | Escape back to the Gateway | 10:46:00.548 → 08.062, 7.514 s as printed; from the captures 00.759, **0.211 s** (see notes) | 10:50:04.391 → 04.634, 0.243 s, I, no flight | 10:54:21.274 → 21.576, 0.302 s, I, no flight | 10:58:34.678 → 34.936, 0.258 s, I, no flight | 11:02:40.670 → 40.957, 0.287 s, I, no flight |
| 8 | Open Trial Balance | 10:46:14.834 → 15.148, 0.314 s | 10:50:19.609 → 19.964, 0.355 s, I, no flight | 10:54:35.373 → 35.732, 0.359 s, I, no flight | 10:58:49.077 → 49.448, 0.371 s, I, no flight | 11:02:55.448 → 55.852, 0.404 s, I, no flight |
| 8b | then Escape back to the Gateway (two Escapes) | 10:46:22.487 → 25.646, 3.159 s (2.697 s between the Escapes) | 10:50:27.045 → 30.114, 3.069 s, I, no flight | 10:54:42.481 → 45.481, 3.000 s, I, no flight | 10:58:57.055 → 59.921, 2.866 s, I, no flight | 11:03:03.322 → 06.383, 3.061 s, I, no flight |
| | cells with a request in flight | | 2 | 1 | 2 | 0 |

Notes (with the time):
- **B1 is partial.** The sheet's analysis printed "UNEXPECTED: 14 or 15 input groups expected" (16 groups): g1's burst was logged as two groups, its Enter 0.711 s after its last Down (10:53:16.703, 10:53:17.414). The B1 column is the lab's reading: the analyser was run unchanged on a copy of the pass whose `inputs.tsv` drops the focus click's two entries, so that the last 14 groups map to g1 to g14. Every B1 end state was reached on screen.
- **B0 action 7.** The analyser's end capture (00632.jpg, 10:46:08.062) already shows g11's first Down: the highlight moved to Stock Summary 76 ms before the timer logged that input. The Escape's own end state is first seen in 00574.jpg at 10:46:00.759 (0.211 s). The runs' action 7 cells are set against 0.211 s.
- **Actions 3 and 8b.** Both include the time between the two Escapes. The lab sent each pair in one call with a 1 s wait and a zoom between, about 2.6 to 2.7 s in every pass, so the columns compare with each other but not with the first sheet's 1 s pause.
- **Coverage.** Every pass took about 100 s, a group about every 7 s, each after its screenshot; the runs lasted 40.2 s (A1), 30.3 s (B1), 41.4 s (A2) and 27.6 s (B2). Each run had fewer than three cells with a request in flight, so none covered the screen under load beyond its first one or two actions.
