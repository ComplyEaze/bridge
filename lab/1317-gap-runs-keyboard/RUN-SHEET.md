# 1317 gap runs, keyboard only (#1317): all four runs of the 500 ms and 250 ms request gaps again, with no mouse click after the first, on BRIDGE CORPUS DENSE

Not for merge. About 50 minutes. READS ONLY on a company the lab already holds. `LAB-COMMON.md` in this folder (v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6) applies in full; where this sheet and it differ, the stricter holds.

## Why this sheet exists
`lab/1317-request-gap-run-sheet` (the first sheet, at 5167f849) was run once on 27 Oct 2026 and ended after run A1: a mouse click on the Gateway's "Display More Reports" drew no change while census request c14 was being answered, the next keys then opened the Dashboard (a screen the sheet does not list), the Dashboard took 12.06 s to draw, and census request c19 was held 11.5 s. The row was stopped because the Dashboard is not a listed screen (the stop file came 30 s after the Enter), and B1, A2 and B2 were not run, so whether a 250 ms gap changes anything is still open (#1317).

The one change here: **every screen action after the first focus click is a key press.** The first run lost one click, and the keys that followed it went to the wrong item; why the click was dropped is not established (it fell 53 ms before census request c14 was sent, and c14 was held 0.188 s). A key burst that moves the highlight to the item and presses Enter does not depend on where the pointer is, and every group is preceded by a screenshot that shows the screen the group needs. Everything else is the first sheet's, unchanged: the same company, the same 40 census reads, the same `driver.ps1`, the same eight actions, the same STOP rules. All four runs are made again here, A1 included, because the first A1 did not complete.

## Rules and company
**Who acts.** As the lab order's WHO ACTS section sets out, on the owner's standing word of 27 Oct 2026: the lab does every step here, and records each as that section directs (LAB-COMMON C11's rule on names applies to what is pushed).

**THE COMPANY** is `BRIDGE CORPUS DENSE` (synthetic), as the first sheet left it: shut, with voucher mark `ALTVCHID` 29900 and master mark `ALTMSTID` 328. If it is absent, or step 0's marks differ, STOP and report. Nothing here writes to the book: M0 and every later marks read must be equal.

**Reads by curl.** Every request is an existing builder's request used as an instrument, as in the first sheet (the voucher census, `render_agent_voucher_census`, one day each, and the company marks, `render_agent_company_high_water`; the 41 files in `requests/` are byte copies of the first sheet's, checked by `SHA256.txt`). No request is new to this company, and none is edited.

**Read bounds (C3) and the first sheet's STOP rules, restated.** A marks answer is bounded at 8,192 bytes and a census answer at 2,000,000 bytes. After each run, an answer in its `log.tsv` above its bound = STOP before the next run. The driver stops at the first request that fails C3 or takes more than 20 s and never sends one twice.

**Screen.** The lab's screen tool presses keys and takes screenshots. It does not click, except ONE focus click on Tally's window at the start of each run (g0 below). It opens display screens only: the Gateway, the Display More Reports list, Day Book, Balance Sheet, Profit & Loss A/c and Trial Balance. Never an entry or alteration screen, never the Dashboard, never a dialog. Any dialog or screen this sheet does not name, or an end state not reached in 10 s after t0: the lab at once creates an empty file named `STOP` in this folder, presses no further key, takes a screenshot and reports (C7). The driver checks for that file before every send.

## Files
- `driver.ps1`, `requests/` (41 files), `check-request-ids.py`, `RESPONSE-FORM.md`, `LAB-COMMON.md`, `.gitattributes`: byte copies of `lab/1317-request-gap-run-sheet` at 5167f849 (their sha256 are those of its `SHA256.txt`).
- `tools/timer.ps1` and `tools/timing-analyse.py`: byte copies from `lab/1317-request-gap-answers` at c23428a6. They are read-only lab helpers (not product code): the timer captures Tally's window back to back with UTC millisecond times and logs each moment Windows records an input; the analyser takes a pass's folder and prints, per form action, t0, t1 and the latency, and R, the median interval between captures.
- `SHA256.txt` lists every file with its bytes; check it before step 0.

## The key sequence (fixed in the rehearsal, then identical in every pass)
The analyser expects 14 input groups (15 with the focus click, g0, if it falls after the timer's first capture), a group being inputs closer than 0.6 s to one another, in this order. Its t0 for an action is the **first** input logged in the action's group (for a burst, the first arrow press as the timer logged it; a burst's latency therefore includes the time of its own presses, the same in B0 and in every run, so the comparisons hold). Each of the original sequence's clicks is replaced by ONE key burst, sent as ONE screen-tool call: the Up or Down presses that move the Gateway's highlight to the item, then Enter. The presses of a burst fall within 0.6 s of each other, so the analyser counts the burst as one group.

| group | what | action |
|---|---|---|
| g0 | the focus click (optional: it may fall before the first capture) | |
| g1 | burst: highlight Display More Reports, Enter | opens the list |
| g2 | Down | |
| g3 | Enter | **1 Day Book** |
| g4 | Page Down five times | **2** |
| g5, g6 | Escape, one second pause, Escape | **3** back to the Gateway |
| g7 | burst: highlight Balance Sheet, Enter | **4** |
| g8 | Escape | **5** back to the Gateway |
| g9 | burst: highlight Profit & Loss A/c, Enter | **6** |
| g10 | Escape | **7** back to the Gateway |
| g11 | burst: highlight Display More Reports, Enter | |
| g12 | Enter | **8 Trial Balance** |
| g13, g14 | Escape, one second pause, Escape | **8b** back to the Gateway |

The number of Up and Down presses in each burst depends on where the highlight stands (after an Escape from a report the Gateway keeps the highlight on the item that was opened). The rehearsal (step 2) fixes those numbers; they are written in `notes.txt` and used unchanged in every pass. **Before every pass (B0 and each run), with the timer NOT running,** the lab sets the Gateway highlight to its resting position (the item step 1 records) with single Up or Down presses and a screenshot, so that a pass needs no correction press. **Before every group**, a screenshot must show the screen that group needs (the Gateway with the highlight on the item the rehearsal says for a burst; the report for a Page Down or an Escape; the Display More Reports list for g2). If a group has not been acted on (the screen is not the one the next group needs), the lab sends nothing more in that pass: it creates the `STOP` file, takes a screenshot and reports. A single Escape at the Gateway is never sent.

## Before the sitting
1. Check every file here against `SHA256.txt` (`Get-FileHash -Algorithm SHA256 FILE`).
2. Parse the driver and the timer without running them, from **cmd.exe**:

   ```
   powershell -NoProfile -Command "$e=$null; [void][System.Management.Automation.Language.Parser]::ParseFile((Resolve-Path .\driver.ps1),[ref]$null,[ref]$e); $e.Count"
   powershell -NoProfile -Command "$e=$null; [void][System.Management.Automation.Language.Parser]::ParseFile((Resolve-Path .\tools\timer.ps1),[ref]$null,[ref]$e); $e.Count"
   ```

   EXPECT `0` from each, else STOP. The operator reads `driver.ps1` in full and confirms that it sends only to `http://127.0.0.1:9001`, and only the files under `requests/`.
3. `python -I check-request-ids.py requests`: EXPECT `census: ok 41, skipped 0, FAIL 0`; anything else = STOP, nothing sent.
4. No folder named `runs` exists in this folder, and no file named `STOP` (the driver refuses a run name that exists).

## Steps
0. C2 step 0 (a, b, c, d), with a screenshot of the whole Tally window first (a dialog, a message box or a half-entered screen = STOP before anything is sent). Tally's About screen: record the release (7.2 expected) and the licence (licensed; Education mode = STOP). Load THE COMPANY alone (arrow keys). Send `requests/m00_marks.request.utf16le.xml` by C3: **M0**, one COMPANY row, THE COMPANY's GUID, ALTVCHID 29900, ALTMSTID 328.
1. Screenshot of the Gateway with the highlight at its resting position; record which item is highlighted.
2. **Rehearsal, no request sent.** Perform the whole key sequence once, with a screenshot before each burst and after each action, to fix the press counts of g1, g7, g9 and g11 and to see that every end state is reached (Day Book with rows, Balance Sheet, Profit & Loss A/c, Trial Balance, each time back at the Gateway). Write the counts and the highlight position before each burst in `notes.txt`. If any burst reaches a screen the sequence does not name (the Dashboard above all), press nothing more, create `STOP`, report.
3. **B0 (idle baseline).** The same sequence, again with no request, under the timer: `powershell -NoProfile -File .\tools\timer.ps1 -Handle <Tally window handle> -Out .\timing\B0 -Seconds 150`, started with `Start-Process` from the lab's shell tool (so that no key is typed on the PC to start it; the window handle is `(Get-Process tally).MainWindowHandle`), then the focus click on Tally's title bar (g0), then the 14 groups. Do not read `frames.tsv` or `inputs.tsv` while the timer runs (a read can stop it, as one stopped the first sheet's first attempt on `log.tsv`). Then `python -I -X utf8 tools\timing-analyse.py .\timing\B0`: record R and each action's latency. A baseline whose analysis prints `UNEXPECTED` is repeated once, in a new folder `.\timing\B0b` (the first is kept, and B0b is then the baseline); a second one = STOP. A pass must not cross 00:00 UTC (the analyser reads times without a date): if it would, wait until after it.
4. **Run A1 (gap 500).** At least two minutes with no request and the Gateway on screen. `GET /status` (200). Set the Gateway highlight to its resting position (timer not running). Start the timer (`-Out .\timing\A1`, `-Seconds 120`) and then the driver, each with `Start-Process` from the lab's shell tool in its own console (no key typed on the PC): `powershell -NoProfile -File .\driver.ps1 -GapMs 500 -RunName A1`. **Do not read `runs\A1\log.tsv` while the driver runs**: the first sheet's first attempt ended after one request because a read held the file when the driver appended to it. The driver prints nothing; watch the answer files appearing in `runs\A1\`. The first send is seen when `c01`'s answer file appears. Then the focus click (g0) and the 14 groups, in order, each after the screenshot the sequence requires, as fast as that allows (the same rule as B0). When the driver's last line is `DONE` or a refusal or `STOP`: wait for the timer to end, then `GET /status` (200), then marks (**M1**, `m00` sent again by C3; equal to M0). `python -I -X utf8 tools\timing-analyse.py .\timing\A1`.
5. **Run B1 (gap 250).** As step 4, with `-GapMs 250 -RunName B1`, timer `.\timing\B1`, marks **M2** (saved as `marks\m2.xml` with its headers).
6. **Run A2 (gap 500).** As step 4, with `-GapMs 500 -RunName A2`, timer `.\timing\A2`, marks **M3** (`marks\m3.xml`).
7. **Run B2 (gap 250).** As step 4, with `-GapMs 250 -RunName B2`, timer `.\timing\B2`, marks **M4** (`marks\m4.xml`). (M0 and M1 are `marks\m0.xml` and `marks\m1.xml`.)
8. C10: Company > Shut, then Select Company with none loaded; one tally.exe; 9001 LISTENING; `GET /status` 200.

## Per action, in each run (the first sheet's RESPONSE-FORM, filled by the lab)
For each of the eight actions, t0 is the input time the analyser takes (the first input logged in the action's group); t1 is the first capture showing its end state; the latency is t1 - t0; the lab marks the cell **I** (immediate: the latency is no more than max(0.5 s, R) above the B0 value), **L** (a noticeable lag: above that and no more than 3 s above it) or **F** (frozen: more than 3 s above it, or not reached in 10 s). `RESPONSE-FORM.md` here is the first sheet's table (it says the owner fills it in; the lab fills it, as the first run did, adding the row 8b the first run added and using the absolute marks above); the filled copy goes in the evidence with each cell's t0, t1, latency and mark, and R for each pass. **In flight:** a run's census requests last about 40 s at 500 ms and about 30 s at 250 ms, while a pass of the 14 groups took about 60 s in B0, so the later actions of a run may fall after the driver has finished. For each action of each run, the lab records whether a request was in flight between t0 and t1 (from `log.tsv`: a request's `utc_send` to `utc_send` + `time_total`), and marks the cell **no request in flight** when none was. Only cells with a request in flight count towards the question; the number of such cells for each run is stated in the notes, and if a run has fewer than three the result says the run did not cover the screen under load.

A key press that is recorded by Windows but does not change the screen is a group that was not acted on: the lab sends nothing more in that pass, creates the `STOP` file, and records in the notes its time and what was on screen (the screenshot rule above makes this visible before the next group).

## STOP rules (beyond LAB-COMMON C7 and C8 and the first sheet's)
- The driver ends with STOP in its `log.tsv`, any request over 20 s, or a `STOP` file present before a run starts.
- Any dialog; any screen the sequence does not name (the Dashboard above all); an action not at its end state 10 s after t0 (the lab creates the `STOP` file at once). After such a STOP, no further run starts, and what has run is the result.
- A marks read with a COMPANY row count other than one, or another company's GUID, or marks differing from M0 (the book must not move).
- A baseline analysis that prints `UNEXPECTED` twice. A run whose analysis prints `UNEXPECTED` is recorded as it is, is not repeated, and its form is marked partial.
- A mouse click after g0 (a click in a pass is a deviation: it is recorded, and the pass is marked void for the form).

## EXPECT (recorded; the lab judges only the STOP rules)
- Per run, from its `log.tsv`: the 40 held times (`time_total`, `time_starttransfer`) with their median and largest; the run's total time from the first send to the last answer.
- Between A and B runs: whether the held time per request rises at 250 ms, and by how much against A's median; whether the run time falls, and by about the 39 x 0.25 s the shorter gaps alone would save.
- From the form: each action's response in each run.
- The first sheet's A1 held times (median 0.219 s, c19 11.508 s at the Dashboard stall) are the only earlier figure; A1 here is a new measurement, not a repeat of that one.
- The issue's rule is unchanged: only if the screen stays responsive and the saving holds does the constant change, with its own review. If not, it stays 500 ms, and these results say why.

## Not covered
Any screen other than the five listed; a person using the screen by mouse; a book of another size; Gold; another release.

## Evidence
- Pushed to a new branch, `lab/1317-gap-runs-keyboard-answers`, with `SHA256.txt`, only after the pre-push scan and the run coordinator's yes (C11).
- Per run: its folder under `runs\` (`log.tsv`, every answer with its headers and curl stderr); the filled `RESPONSE-FORM.md`; the analyser's output for B0 and each run; `timing\<pass>\frames.tsv`, `inputs.tsv` and the capture hash list (the captures themselves stay on the lab PC, listed by name, size and sha256); M0 to M4 with headers; `notes.txt` (UTC, one line per step, with the press counts of the rehearsal); `SCREENSHOTS.txt`.
- THE COMPANY's GUID is already in the merge gate's registry; no real figure is recorded beyond the book's own synthetic marks.
