# Results: #1317 request gap on BRIDGE CORPUS DENSE, TallyPrime 7.2 Silver, 27 Oct 2026 (UTC)

The sheet is `lab/1317-request-gap-run-sheet/RUN-SHEET.md` at 5167f849604b55d606874d68150007b7150e7c1a (SHA256.txt 47 of 47 equal; census ok 41; `driver.ps1` parsed with 0 errors and read in full). The company is BRIDGE CORPUS DENSE, number 100012, GUID d45bc1b0-e5e3-4261-b3b2-cce3915f42d3, voucher mark 29,900. **The row ended after run A1, by the decision taken at the stop: end the row with the evidence as it stands (the other choice was to redo A1 with keyboard-only navigation). B1, A2 and B2 were not run**, so the 250 ms question is not answered here.

## FINDING (verified once)
**Under the 500 ms request cadence, one click was not acted on, and the next screen, the Dashboard (not on the form), took 12.06 s to draw while Windows showed Tally as not responding.**

| time (UTC) | what happened |
|---|---|
| 21:50:32.165 | a click on the Gateway's "Display More Reports" drew no change; census request c14 was being answered then |
| 21:50:34.398 | Down moved the highlight to "Dashboard" (drawn 0.12 s later) |
| 21:50:36.494 | Enter opened the **Dashboard**, first drawn at 21:50:48.554: **12.06 s** |

While the Dashboard was being drawn:
- Windows put up its stand-in for a not-responding window.
- **Census request c19, sent at 21:50:37.077, was held 11.508 s** and answered at 21:50:48.585, 31 ms after the Dashboard's first capture.
- Every other A1 request was held 0.178 to 0.378 s (the largest c26 0.378, c12 0.325 and c02 0.306).

The Dashboard is not a listed screen, so the run was stopped there. The stop file was created at 21:51:06.6, and the driver sent nothing after c34.

**Not measured:**
- the listed screens under load (none was reached in A1), and the 250 ms gap;
- whether the lost click is load-related (it happened once, during c14);
- how long the Dashboard takes to draw with no request running.

## B0 (idle, no request)
R, the median interval between captures, was 0.104 s.

| action | latency |
|---|---|
| 1 Day Book | 0.681 s |
| 2 Page Down ×5 | 0.322 s |
| 3 Escape to the Gateway (two Escapes, 1 s apart) | 1.214 s |
| 4 Balance Sheet | 0.530 s |
| 5 Escape | 0.699 s |
| 6 Profit & Loss A/c | 0.509 s |
| 7 Escape | 0.160 s |
| 8 Trial Balance | 0.353 s |
| 8b Escape to the Gateway (two Escapes, 1 s apart) | 1.227 s |

The filled form is `RESPONSE-FORM-filled.md`.

## Run A1 (gap 500 ms)
- **Requests.** 34 were sent (c01 to c34), all 200, bytes equal to CONTENT-LENGTH: 201,818 B for c01 and 201,958 B for each of the others.
- **Held time** (`time_total`): median 0.219 s, smallest 0.178 s, largest 11.508 s (c19).
- **Run time.** 46.6 s from the first send to c34's answer.
- **Capture interval.** R during A1 was 0.133 s (largest interval 0.966 s).
- **The book did not move.** M0, the marks after the first attempt and the marks after A1 are byte-equal: ALTVCHID 29900, ALTMSTID 328, one COMPANY row.

## The first attempt at A1 (`runs/A1-ended-c01`)
To see the first send, the lab read the driver's `log.tsv` while the driver was running. That read held the file at the moment the driver appended c01's line. `Add-Content` failed and the driver ended after one request (exit 1). c01 was answered 200 (201,818 B, equal to CONTENT-LENGTH); its answer and headers are kept, and its log line is missing for this reason. No screen action had started. The run was repeated after two minutes with no request, and this time the first send was seen only by c01's answer file appearing.

## How the timing was taken
The lab did the actions itself through its screen tool. A read-only timer (`tools/timer.ps1`) captured Tally's window back to back with UTC millisecond times, and logged each time Windows recorded an input. `tools/timing-analyse.py` maps those input times to the form's actions. For each action:
- t0 is the input time of the action's key or click;
- t1 is the first capture showing the action's end state.

A rehearsal pass with no request fixed the key sequence (`timing/rehearsal`). The captures stay on the lab PC, listed by sha256 in `timing/*-captures-sha256.txt`. Each capture's time is in `timing/<pass>/frames.tsv` and each input's time in `timing/<pass>/inputs.tsv`.

## Files
- `runs/A1/`: the driver's log and its 34 answers, headers and curl stderr files.
- `runs/A1-ended-c01/`: the first attempt.
- `marks/`: M0, the marks after the first attempt, and the marks after A1.
- `RESPONSE-FORM-filled.md`, `notes.txt` (UTC), `send-log.txt` (the marks sends).
- `timing/`: B0's analysis, and each pass's frames.tsv, inputs.tsv and capture hash list.
- `tools/`: the timer and the analysis script.
- `SCREENSHOTS.txt`: the other captures by name, bytes and sha256.
