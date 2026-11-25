# LAB-COMMON-BX: the rules row 1381-BX carries (written for this row, 25 Nov 2026)

This row sends nothing to Tally and loads no company, so it does not carry LAB-COMMON v3, whose clauses are about a loaded company and requests to it (C1 also makes Educational Mode a STOP, which on today's lab would stop every row). These clauses take its place for this row only. Where the run sheet and this text differ, the stricter holds. The lab's standing rules stand above both. Do not start the row without this text in front of you.

B1. NO TALLY. Nothing in this row sends anything to Tally: no request on any port, no key, no click and no screen act in Tally's window. The dialog child modes open no connection (code `src-tauri/src/lib.rs:112-126` at 48a64965: the argument picks a dialog, which reads its standard input and answers on its standard output). Tally may be running or not. If it is running it stays as found, at Select Company with no company loaded; if it shows anything else at step 0 (a company loaded, a screen half entered), STOP and touch nothing. The lab's Tally runs in Educational Mode; that is the lab's state and not a STOP for this row. Any licence dialog, Tally's or another program's, on the screen at any read = STOP: press no key, copy its text, report.

B2. NOBODY CLICKS THE BOX. The approval box is answered only by `box.ps1`, started by `arm.py`, and only after `arm.py`'s guard and `boxcheck.py`'s whole-text check have passed. Nobody presses a key or a button in it, by hand or by the screen tool, at any time. From an arm's start line until its `arm exit` line, nobody is at the PC and the screen tool does nothing: no screenshot, zoom, click, key, `request_access` or `list_granted_applications`.

B3. SCREENSHOTS. A screen-tool screenshot is taken only between arms, after the arm's `arm exit` line and after `tasklist` shows no `bridge_mcp.exe` running (lab 2569-S: a screen-tool screenshot minimised the window of an exe the tool had no grant for). `box.ps1`'s own screen copies, made by GDI in its own process, are not screen-tool screenshots.

B4. NOTHING ENDS A PROCESS. No `taskkill`, no `Stop-Process`, no End Task, by name or by number. A box left up, and the child that shows it, stay up until the run coordinator's word.

B5. A COMMAND THAT FAILED. A command that fails, exits non-zero or prints an error is "the command failed", never "none found" or "no change". Every output is written to a file with its exit code and read from that file.

B6. FILES. The row folder is `C:/bridge-lab/1381-bx`, copied whole from the sheet's commit. Everything the lab adds goes into `C:/bridge-lab/1381-bx/local/`; no file is ever overwritten (every helper refuses a name already used), except inside `local/selftest/`, the self-tests' own scratch folder, which `selftest-box.py` writes afresh at each run. `check_sheet.py` skips `local/` and nothing else, so any other file outside `SHA256.txt` stops it.

B7. STOPS. Each STOP has a name, given where it can happen. A STOP leaves things as they stand: nothing further is run, no file is moved back, no process is ended. Report it, with the files that show it, to the run coordinator before anything else; the row resumes only on the coordinator's word, from the first step not done.

B8. EVIDENCE. Every helper output whole, with its exit code, under `local/evidence/`. Screenshots and screen copies are never pushed: they stay on the lab PC, listed by name, size and sha256. A notes file with UTC times and one line per step. No real client data anywhere; notes name no roles and no people, and carry no user names and no machine paths beyond `C:/bridge-lab`. The evidence branch is pushed only after the pre-push scan and the run coordinator's yes.

B9. CLOSE. The lab is left as found, except as the run sheet's close step says: no box open, no `bridge_mcp.exe` running, Tally as found at step 0.
