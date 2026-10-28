# Results: #1317 gap runs, keyboard only, 28 Oct 2026. All four runs and the baseline completed on BRIDGE CORPUS DENSE with key presses only. A 250 ms gap made the 40-request run 11 to 14 s shorter with no rise in the time Tally held each request. The screen was measured under load for only one or two actions per run: each run ended before the lab's screen pass, at about 7 s a group, reached its third action

The sheet is `lab/1317-gap-runs-keyboard` at 0274a839ad67155fe69b999f25f50725f8a228c2.
- **Sheet checks:** `SHA256.txt` 49 of 49; the driver and timer parsed with 0 errors; census ok 41.
- **Company:** BRIDGE CORPUS DENSE (100012), GUID d45bc1b0-e5e3-4261-b3b2-cce3915f42d3.
- **Tally:** TallyPrime 7.2, Silver, Licensed.
- **Marks:** M0 = M1 = M2 = M3 = M4: ALTVCHID 29900, ALTMSTID 328. They are byte-equal, so the book did not move.
- **Stop rules:** none was met. No dialog, no Dashboard, no unlisted screen, no click after a pass's focus click. Every end state was reached in every pass.

## The requests (from each run's `log.tsv`)
| run | gap | answered | time_total median | largest | first send to last answer |
|---|---|---|---|---|---|
| A1 | 500 ms | 40 of 40, DONE | 0.210 s | 0.279 s (c01) | 40.178 s |
| B1 | 250 ms | 40 of 40, DONE | 0.192 s | 0.346 s (c40) | 30.337 s |
| A2 | 500 ms | 40 of 40, DONE | 0.184 s | 0.284 s (c33) | 41.388 s |
| B2 | 250 ms | 40 of 40, DONE | 0.187 s | 0.261 s (c02) | 27.625 s |

- **Held time:** it did not rise at 250 ms. The B medians (0.192, 0.187) lie within the A medians' range (0.210, 0.184). No request was held more than 0.346 s.
- **Run time:** the 250 ms runs were 9.8 s (B1 against A1) and 13.8 s (B2 against A2) shorter. The A runs average 40.8 s and the B runs 29.0 s, a fall of 11.8 s against the 39 x 0.25 = 9.75 s the shorter gaps alone would save.

## The screen (`RESPONSE-FORM-filled.md`)
- **Cells with a request in flight:** A1 2 (Day Book I, Page Down I), B1 1 (Day Book I; B1 is partial), A2 2 (**Day Book L**, Page Down I), B2 0.
- **A2's Day Book:** it drew in 1.504 s with census c31 in flight, 0.697 s above B0's 0.807 s. It is the one L.
- **Every other cell is I**, with or without a request in flight.
- **Coverage:** by the sheet's rule each run had fewer than three loaded cells, so none covered the screen under load beyond its first one or two actions.
  - A pass took about 100 s: each of its 14 groups is one screen-tool call after its screenshot.
  - The runs lasted 28 to 41 s.
  - Whether the later actions (Balance Sheet, Profit & Loss A/c, Trial Balance and the Escapes) respond under load at either gap is not measured here.

## Deviations and readings, as recorded
- **B1's analysis printed UNEXPECTED (16 input groups).** g1's burst was logged as two groups: its Enter came 0.711 s after its last Down. By the sheet, B1 is recorded as it is, not repeated, and its form is partial. The B1 column is a lab reading: the analyser run unchanged on a copy without the focus click's inputs (`B1-analysis-lab-reading.txt`, `B1-reading-inputs.tsv`).
- **B0's action 7 printed 7.514 s.** The analyser's end capture already shows the next burst's first Down, drawn 76 ms before the timer logged it. Read from the captures, the Escape's end state came at 0.211 s, and the runs are set against that.
- **Actions 3 and 8b** include about 2.6 to 2.7 s between their two Escapes, not 1 s. The same holds in every pass.

## Files
- `runs/A1`, `runs/B1`, `runs/A2`, `runs/B2`: `log.tsv`, and every answer with its headers and curl stderr.
- `timing/<pass>/frames.tsv` and `inputs.tsv`.
- `timing/<pass>-analysis.txt` (B0, A1, B1, A2, B2).
- `timing/<pass>-captures-sha256.txt`: the timer's captures stay on the lab PC.
- `marks/m0.xml` to `m4.xml` with headers.
- `RESPONSE-FORM-filled.md`.
- `notes.txt` (UTC, with the rehearsal's press counts).
- `send-log.txt`.
- `SCREENSHOTS.txt`.
- The sheet's own files (driver, requests, tools, rules) are on the sheet branch at 0274a839 and are not repeated here.
