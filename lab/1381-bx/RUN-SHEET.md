# Row 1381-BX: the stock-item approval box on Windows

For #1381. Pull request 2596 (head 48a64965) adds the dialog child mode `--confirm-stock-item`: a Windows message box titled `ComplyEaze Bridge — create one stock item?` that shows the stock item's whole review text and answers Yes, No or Cancel. No row has opened it. This row opens it on the Windows lab PC from the real exe, with the real text, four times: once as a control on the unchanged ledger box, then the stock box answered No, Cancel and Yes. It records the box, the display it opened on, and what the child printed and returned. The child needs no Tally and no company. Nothing is sent to Tally.

**How an EXPECT is marked.** **(code `file:line`)** is a rule read at commit 48a6496507132576a6be150c155192f647528e1e (all code lines cited are that commit's); **(lab `row`)** is an earlier observation; **(not shown)** is what this sitting measures. The build that runs is the master commit that merges pull request 2596. It is named in `BUILD.txt` only, filled by the run coordinator in one commit once the merge exists, and checked against this commit for the files cited (The build, below).

**Rules.** `LAB-COMMON-BX.md` (B1 to B9) applies in full: no Tally request, nobody clicks the box, screenshots only between arms, nothing ends a process, everything the lab adds under `local/`, every STOP named and left as it stands. This row does not carry LAB-COMMON v3 (B1 says why).

## The questions and the arms

- **Q1.** Does the real stock box show the exact title, its answer lines and the whole 29-line preview the code makes, line for line, with three buttons Yes, No and Cancel, No the default, all inside the box, and the box wholly inside the work area?
- **Q2.** Does each answer give what the code says: No and Cancel print nothing and exit 1; Yes prints the stock token for the arm's nonce and a line feed, nothing else, and exits 0?
- **Q3.** Does the ledger box, whose code pull request 2596 does not change, still open and answer No as before (lab 1780-K)?

| arm | mode | input | answer | EXPECT |
|---|---|---|---|---|
| LN | `masters` (`--confirm-masters`) | `in/ledger-input.txt` | No | title `ComplyEaze Bridge — create one ledger?`, its text line for line, `fits=true`; standard output 0 bytes, exit 1 (lab 1780-K: box 688,342,1252,764 in work 0,0,1920,1030, 0 bytes, exit 1) |
| SN | `stock` (`--confirm-stock-item`) | `in/stock-input.txt` | No | Q1 (not shown); standard output 0 bytes, exit 1 (code) |
| SC | `stock` | `in/stock-input.txt` | Cancel | Q1 (not shown); standard output 0 bytes, exit 1 (code) |
| SY | `stock` | `in/stock-input.txt` | Yes | Q1 (not shown); standard output exactly `bridge-create-stock-item:<the arm's nonce>` and one LF, exit 0 (code) |

**The one variable.** Each arm is one launch of the same exe with a fresh nonce; the stock arms differ only in the button `box.ps1` clicks. LN differs from the stock arms only in its mode and its input.

**Order.** LN first: if the unchanged ledger box does not behave as lab 1780-K saw, the stock arms are not run (STOP-CONTROL). Then SN, SC, SY. SY last, so that the one arm whose box prints a token comes after both refusals are on file.

## What the code says

All at 48a64965.

- **The argument.** `bridge_mcp.exe` with exactly one argument `--confirm-stock-item` runs `run_stock_item_confirmation`, and with `--confirm-masters` runs `run_masters_confirmation`; exit 0 when the dialog confirmed, else 1 (code `src-tauri/src/lib.rs:112-126`, `src-tauri/src/bin/bridge_mcp.rs`). With any second argument the exe starts as the server, so `startc.ps1` passes the one argument alone.
- **The input.** The child reads at most 8,064 bytes of standard input (code `src-tauri/src/tally/approved_import.rs:3243-3269`), splits a nonce line, a count line and the text shown (`dialog_input`, :3273-3283): the nonce must be a UUID, the count canonical, the text non-empty, without NUL and at most 8,000 bytes (`MAX_PREVIEW_BYTES`, :14). An input refused there shows no box and exits 1 with nothing printed, the same as a No; so every arm needs `box.ps1`'s `BOX SEEN` and `CLICKED` lines and a `fits=true` line to count.
- **The text shown.** The parent passes the child the whole text the box shows, answer lines first (`confirm_stock_item_dialog`, :3050-3054, through `displayed_text`, :2904); the child shows it as given (`show_create_stock_item`, :3884-3921). On Windows the answer lines are `answer_text` (:2764-2770) wrapped at 100 characters (:2792, :2798-2816) from the stock item's answer words (:2755-2758).
- **The box.** `MessageBoxW` with `MB_YESNOCANCEL | MB_DEFBUTTON2 | MB_ICONWARNING | MB_SETFOREGROUND | MB_TOPMOST | MB_SYSTEMMODAL`, owned by a hidden topmost window, inside `box_fit::guarded`; the title is `stock_item_words` (:99-104, `STOCK_ITEM_TITLE` :91); only `IDYES` is a confirmation (:3884-3921). The ledger box is the same call with its own title (`show_create_ledger`, :3820-3858; `MASTERS_TITLE` :60).
- **The token.** Only a confirmation writes `dialog_token`: the prefix, the nonce and `\n` (:3154, :3243-3269). The stock prefix is `bridge-create-stock-item:` (:88), the ledger prefix `bridge-create-ledger:` (:57).
- **The fit.** The child makes itself per-monitor DPI aware; at each activation of the box it measures the box's outer rectangle against the work area of the monitor nearest it and writes one line `box-fit: box=(l,t,r,b) work=(l,t,r,b) fits=true|false` to its error output; a box not wholly inside is cancelled and the child exits 77 (code `src-tauri/src/tally/box_fit.rs:38`, :530-575). A box that could not be measured or shown exits 0 with nothing printed (unavailable).
- **No timeout here.** The 120 s limit and the nonce check belong to the parent (`nonce_bound_dialog`, :3104-3150). This row starts the child alone, so an unanswered box stays up until something answers it (STOP-UNANSWERED, below).

## The build

**Named in `BUILD.txt` only.** Seven fields are `<TO FILL>` until the merge of pull request 2596 exists: BUILD COMMIT, CI RUN, ARTIFACT ID, ARTIFACT BYTES, ZIP DIGEST, MCPB SHA256 and EXE SHA256. The run coordinator fills all seven in one commit on this branch, from the Bridge CI run on BUILD COMMIT that made the Windows bundle `unsigned-bundle-smoke-windows-latest` (its `mcpb-smoke.json` holds `archive_sha256` and `binary_sha256`). `build.py check` exits 1 (STOP-BUILD-UNFILLED) while any is unfilled, and `arm.py` will not start. `build.py` and `arm.py` read every value from `BUILD.txt`; no other file carries them (`check_sheet.py` checks).

**The fill check.** Before that commit, and again at step 0 from the lab's clone, the cited code must be the same at BUILD COMMIT as at 48a64965: the line in step 0 must print nothing and exit 0. Any output: STOP-BUILD-DIFFERS; the input and the citations would have to be made again from BUILD COMMIT.

**The fill, one commit, by the run coordinator** (not lab lines; each typed from the root of a checkout of this branch that also holds BUILD COMMIT and 48a64965):

1. Replace each `<TO FILL: …>` in `lab/1381-bx/BUILD.txt` with its value, in the shape `build.py` names for its field; change nothing else.
2. Rewrite the one `BUILD.txt` row of `SHA256.txt` (every other row stays as it is), then run the two checks. EXPECT `check_sheet.py` exit 0 with `0 field(s) <TO FILL>`, and `build.py check` exit 0 ending `0 field(s) to fill: filled`.
   ```bash
   ( cd lab/1381-bx && python3 -I -c "import hashlib; b = open('BUILD.txt', 'rb').read(); rows = open('SHA256.txt', encoding='utf-8').read().splitlines(); rows = [hashlib.sha256(b).hexdigest() + '  ' + str(len(b)) + '  BUILD.txt' if r.endswith('  BUILD.txt') else r for r in rows]; open('SHA256.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(rows) + '\n')"; echo "exit $?" )
   ( cd lab/1381-bx && python3 -I -X utf8 check_sheet.py; echo "exit $?"; python3 -I -X utf8 build.py check; echo "exit $?" )
   ```
3. The fill check, with BUILD COMMIT in place of `"$B"` in step 0's line: nothing printed.
4. One commit changing those two files only. Its id is the `<SHEET>` the GO names.

**Where it runs.** On `GRANTED PATH`, `C:/bridge-lab/2569-r/exe-new/mcpb/bin/x86_64-pc-windows-msvc/bridge_mcp.exe`, the path the screen tool's grant names (lab 2569-R). The exe there now is row 2569-R's (its hash is `BUILD.txt`'s ASIDE SHA256). `build.py stage` moves it to `local/aside/`, copies the row's exe in, and records each act with its hash in `local/evidence/copies.txt`; `build.py restore` does the reverse at the close. Nothing is deleted.

**The download.** The one network act of the row, on the run coordinator's word: the artifact's zip, saved unchanged as `local/download/artifact.zip`. GitHub keeps the artifact 7 days from its run; after that the row needs a new run and a new fill.

## Shell

Every command is typed in **Git Bash**, from `/c/bridge-lab/1381-bx`, exactly as written: each line begins `cd /c/bridge-lab/1381-bx && ` and depends on no shell variable set by an earlier line. The four arm start lines are not typed at the prompt: each is started whole as a background run of the operator's own shell tool. The interpreter is `python` (3.10) with `-I -X utf8`; `python3` is the Store alias and is never used. Every helper's output goes to a file under `local/evidence/` with its exit code, then is printed whole. A line that begins `#` is an instruction, not typed. A wait for a file is a loop of the line given, typed about every 5 s, at most the number of times the step names. `arm.py` starts Windows PowerShell 5.1 itself, hidden, for `screen.ps1`, `box.ps1`, `startc.ps1` and `boxstate.ps1`; step 0 also starts it for `screen.ps1` and the parse check.

## The input

**`in/stock-input.txt`: 1,494 bytes, 33 LF, none at the end, sha256 `c94ecf5c1e3046412e1ebf26cbad5e9993d2724622eab55049f6fc8ac872d87f`.** Its first 39 bytes are the nonce `synthetic_guid(0)` (code `src-tauri/src/lib.rs:9-12`), LF, the count `1`, LF; the rest is the text the box shows on Windows: 1,455 bytes, 32 lines, sha256 `1728bdeb60a05e6a270eb9aafac5282f5ae74d485d8399d02818064c626f50eb`. No line was typed.

- **Made by** `geninput.py write`, from the test file at 48a64965 (`git show 48a6496507132576a6be150c155192f647528e1e:src-tauri/src/agent_master_review_tests.rs`): it reads the `STOCK_ITEM_PREVIEW` literal (code `src-tauri/src/agent_master_review_tests.rs:361`, 29 lines) and the Windows `STOCK_ITEM_ANSWER` literal (:890), and joins header, answer, a blank line and preview. It writes only if (1) the text is 32 lines and 1,455 bytes, what the test `the_stock_item_dialog_shows_its_answer_then_the_whole_preview` asserts on Windows, which pull request 2596's CI ran on Windows and passed; (2) the answer equals its own copy of `wrapped()` at 100 characters applied to the code's answer words; (3) the Mac output of the product's own writer, the ignored test `write_the_stock_item_box_input` (:912-925), run at 48a64965 on a Mac, holds the same header, the Mac answer literal (:888), a blank line and the same preview byte for byte.
- **To make it again** on a checkout at 48a64965, from `src-tauri`: on Windows, `BRIDGE_STOCK_BOX_INPUT=<new file> cargo test --lib -- --ignored --exact agent::agent_import::master_review::tests::write_the_stock_item_box_input` writes these 1,494 bytes directly (`cmp` against `in/stock-input.txt` must be silent); on any other system the same test writes the Mac text, and `python -I -X utf8 geninput.py write <test file> <that output> <new file>` writes these bytes.
- **At the sitting** step 0 runs `geninput.py check` against the test file from the lab's clone at 48a64965; `arm.py` then writes each stock arm's `local/run/<arm>-in.txt`: a fresh nonce (uuid4) in place of the first line and every byte after it unchanged (it checks), with its sha256 in `local/run/<arm>-in.sha256`.

**`in/ledger-input.txt`: 476 bytes, sha256 `833ac1daa719973158f538a3599212b8da573ebea4f403dc82d235f6a4328377`**, byte-equal to the input lab 1780-K opened this ledger box with (`lab/1780-ledger-box-keyboard`, `requests/confirm-masters-input.txt`, blob 13be3bbb). Its text is synthetic (no real company) and ends with one LF; LN feeds it with a fresh first line, as above. It is a control on the unchanged ledger box, not a text the code makes at this commit; the ledger box shows whatever text it is given.

## The box: read, guarded, checked, answered

All of it runs inside `arm.py`, started by the arm's one start line, so nothing is typed while a box is up.

- **Before the launch.** `arm.py` reads `BUILD.txt` (STOP-BUILD-UNFILLED), hashes the exe on GRANTED PATH against EXE SHA256 (STOP-EXE-HASH), the arm's input against `SHA256.txt`, refuses any output name already used, and refuses while a `bridge_mcp.exe` runs (STOP-STRAY). It runs `screen.ps1` into `local/run/<arm>-screen.txt`; anything but exactly one monitor is STOP-MONITORS, before any launch.
- **The watch and the launch.** It starts `box.ps1 -RowDir C:\bridge-lab\1381-bx\local -Tag <arm> -SeeWithin 30 -AnswerBy 90`, then 2 s later `startc.ps1` with the exe, the arm's mode, `-Show 1`, the arm's input, and new files `local/run/<arm>-out.bin` and `local/run/<arm>-err.txt` for the child's standard output and error. `box.ps1` logs `BOX SEEN` with the box's process, title and class, its rectangle and z-order place, every child window's class, rectangle and text, copies the screen by GDI (`open`), then logs `WAITING` (`local/box/<arm>-box.txt`).
- **The guard.** At `WAITING`, `arm.py` passes only when: the box's process is the child `startc.ps1` started; `boxstate.ps1` on that process (into `local/box/<arm>-R1.txt`) exits 0 and holds exactly one `WINDOW` line named `[ComplyEaze Bridge…` with class `#32770`, `iconic=False`, `visible=True` and `topmost=True`; `box.ps1`'s rectangle of the box lies inside the work area `screen.ps1` read; the arm's button lies inside the box; and the box is first in the z-order. Its lines go to `local/box/<arm>-guard.txt`. Any miss: no check, no answer, nothing pressed (STOP-GUARD).
- **The text check.** Only a passed guard starts `boxcheck.py`. It checks the title exactly, the class, the input's nonce and count, exactly three buttons Yes, No and Cancel, exactly one text child beginning `Press Y `, that text equal line for line to the arm's input after its two header lines, and the text and buttons inside the box with the buttons below the text. Every check passed: it writes the arm's answer to `local/box/<arm>.answer.tmp` and renames it to `local/box/<arm>.answer`; otherwise it writes none (STOP-CHECK). Its record is `local/box/<arm>-check.out`.
- **The answer.** On the answer file `box.ps1` clicks that button at its centre by the mouse, logs `CLICKED <answer>` and whether the box is still visible. Without a valid answer file by 90 s after the box was seen it presses nothing and exits 6 (this row's change to `box.ps1`): the box stays up and the child keeps running (STOP-GUARD, STOP-CHECK or STOP-UNANSWERED, as `arm.py` names it). Nothing ends it (B4).
- **The grade.** After a click `arm.py` waits up to 60 s for `startc.ps1`'s `EXITED` line and grades from the files, into `local/evidence/<arm>-arm.txt`. **Deciding**: `CLICKED` once with the arm's answer; the box gone after it; the exit code (0 for Yes, 1 for No and Cancel); the standard output (exactly the token for the arm's nonce and one LF for Yes, 0 bytes otherwise); the error output only `box-fit:` lines, at least one, each `fits=true`. **Recorded, not deciding**: W1 the box not minimised and W2 in front at the first read; W3 the keyboard focus on No in the box's process (No is the default button); W4 each `box-fit` work area equal to `screen.ps1`'s; W5 the first `box-fit` box equal to `box.ps1`'s rectangle. `ARM <arm> AS THE CODE SAYS` and exit 0 only when every deciding line passed.
- **By eye.** After the arm, with no `bridge_mcp.exe` running, the operator opens `local/box/<arm>-open.png` (`box.ps1`'s copy, kept on the lab PC) and records whether every text line is wholly visible, none cut at the box's edge, with no scroll bar (not shown).

## Display facts

`screen.ps1` reads, per-monitor DPI aware as the child is: the Windows product, display version and build; each monitor's rectangle, size (the resolution), work area and effective DPI with its scale. EXPECT one monitor, 1920x1080, DPI 120 (125%), work area 0,0,1920,1030 (lab 1780-K's `box-fit` line; `box_fit.rs` notes the lab display so). A different single monitor is recorded, not a STOP; two or more are STOP-MONITORS.

## Files

Byte-equal to earlier rows' files (git blob ids):

- `.gitattributes`: blob 79cd6402, as rows 2569-S (`lab/2569-s` at cd80f722) and 2162-PA (`lab/2162-pa` at f682a6aa).
- `boxstate.ps1`: blob c3d6b747, row 2569-S's reader (cd80f722), also row 2162-PA's.
- `in/ledger-input.txt`: blob 13be3bbb, row 1780-K's input (`lab/1780-ledger-box-keyboard` at 1f273633).

Changed copies, each change marked `1381-BX` in the file:

- `box.ps1`: row 2162-SALES-Q's (blob b105f7cc, also row 2162-PA's) with three changes: Cancel is an answer; no valid answer by `-AnswerBy` presses nothing (exit 6) where it pressed No; no running Tally is no error.
- `startc.ps1`: row 2569-R's `starth.ps1` (blob aef601eb, `lab/2569-restore` at 208932f7) with one change: `-Mode stock|masters` picks the one argument in place of its fixed `--confirm-journal`.

Written for this row, in the shape of row 2162-PA's files where named:

- `arm.py` (shape of `pa.py`): one arm from its start line to its grade; `--selftest` grades 21 synthetic cases.
- `boxcheck.py` and `selftest-box.py` (shape of 2162-PA's): the whole-text check, for both titles and all three answers; 26 synthetic cases, and `replay` on a real arm's log.
- `check_sheet.py` (shape of 2162-PA's): the manifest, references, arms, start lines and held lines.
- `build.py` and `BUILD.txt`: the build, checked, staged and restored; `--selftest` 17 cases.
- `geninput.py`: the stock input, made and checked from the code (The input).
- `screen.ps1`: the display facts, read-only.
- `LAB-COMMON-BX.md`: the row's rules. `SHA256.txt`: every other file's sha256, bytes and path.

Row 2569-S's sheet carries `boxstate.ps1` only of these helpers; `box.ps1`, `boxcheck.py` and `selftest-box.py` come from rows 2162-SALES-Q and 2162-PA, which answered the same kind of box by the same means.

## Self-checks before the sitting

Run from `/c/bridge-lab/1381-bx` at step 0. Nothing is launched and nothing is sent.

```bash
cd /c/bridge-lab/1381-bx && python -I -X utf8 arm.py --selftest > local/evidence/selfcheck-arm.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-arm.txt; tail -n 2 local/evidence/selfcheck-arm.txt
cd /c/bridge-lab/1381-bx && python -I -X utf8 build.py --selftest > local/evidence/selfcheck-build.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-build.txt; tail -n 2 local/evidence/selfcheck-build.txt
cd /c/bridge-lab/1381-bx && python -I -X utf8 selftest-box.py cases C:/bridge-lab/1381-bx > local/evidence/selfcheck-box.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-box.txt; tail -n 3 local/evidence/selfcheck-box.txt
cd /c/bridge-lab/1381-bx && python -I -X utf8 geninput.py check local/gen/agent_master_review_tests.rs in/stock-input.txt > local/evidence/selfcheck-input.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-input.txt; cat local/evidence/selfcheck-input.txt
cd /c/bridge-lab/1381-bx && python -I -c "import ast,sys;[ast.parse(open(f, encoding='utf-8').read()) for f in sys.argv[1:]]" *.py > local/evidence/selfcheck-parse.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-parse.txt; cat local/evidence/selfcheck-parse.txt
cd /c/bridge-lab/1381-bx && powershell -NoProfile -Command 'foreach ($f in (Get-Item box.ps1, boxstate.ps1, startc.ps1, screen.ps1)) { $e = $null; [void][System.Management.Automation.Language.Parser]::ParseFile($f.FullName, [ref]$null, [ref]$e); Write-Output $f.Name, $e.Count }' > local/evidence/selfcheck-ps-parse.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-ps-parse.txt; cat local/evidence/selfcheck-ps-parse.txt
cd /c/bridge-lab/1381-bx && python -I -X utf8 check_sheet.py > local/evidence/selfcheck-sheet.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-sheet.txt; cat local/evidence/selfcheck-sheet.txt
```

EXPECT, in order: `21 cases, 21 right`, exit 0; `17 cases, 17 right`, exit 0; `26 cases ran, 26 right`, `ALL CASES RIGHT`, exit 0; three `PASS` lines and `EQUAL 1494 bytes, 33 LF, none at the end, sha256 c94ecf5c…`, exit 0; exit 0 alone; each of the four names followed by `0`, exit 0; three `OK` lines, exit 0. Any other: STOP-SELFCHECK.

## Steps

**Prompt rule.** No dialog is expected but the four boxes, and each is handled by `arm.py` alone. Any other dialog or message on the screen (a licence dialog above all, B1): copy its text, press no key, STOP-DIALOG. Never end a process (B4).

0. **Before the sitting.**
   - Make the row folder and copy this sheet's folder into it byte for byte from the lab's Bridge clone. In every line of steps 0 and 1, replace `<CLONE>` with the Git Bash path of the lab's Bridge clone; in the second line below, replace `<SHEET>` with the 40-character commit of this branch the run coordinator's GO names (the commit that filled `BUILD.txt`).
     ```bash
     if [ -e /c/bridge-lab/1381-bx ]; then echo "STOP: /c/bridge-lab/1381-bx exists"; else mkdir /c/bridge-lab/1381-bx && echo "made /c/bridge-lab/1381-bx"; fi
     git -C <CLONE> fetch origin && git -C <CLONE> archive <SHEET> lab/1381-bx | tar -x -C /c/bridge-lab/1381-bx --strip-components=2; echo "exit ${PIPESTATUS[0]} ${PIPESTATUS[1]}"
     cd /c/bridge-lab/1381-bx && if [ -e local ]; then echo "STOP: local exists"; else mkdir local local/evidence local/gen && echo "made local"; fi
     ```
   - The two commits in the lab's clone. Master merges by squash, so 48a64965 is fetched by the pull request's own ref. EXPECT the fetch `exit 0`, then two lines `present`, one per commit; any `ABSENT`: STOP-CODE-COMMIT.
     ```bash
     cd /c/bridge-lab/1381-bx && git -C <CLONE> fetch origin refs/pull/2596/head > local/evidence/s0-fetch.txt 2>&1; echo "exit $?" >> local/evidence/s0-fetch.txt; cat local/evidence/s0-fetch.txt
     cd /c/bridge-lab/1381-bx && B=$(sed -n 's/^BUILD COMMIT: \([0-9a-f]\{40\}\)$/\1/p' BUILD.txt); for c in 48a6496507132576a6be150c155192f647528e1e "$B"; do if git -C <CLONE> cat-file -e "$c^{commit}" 2>/dev/null; then echo "present $c"; else echo "ABSENT [$c]"; fi; done > local/evidence/s0-commits.txt 2>&1; cat local/evidence/s0-commits.txt
     ```
   - The test file at 48a64965, for the input check, and the fill check (The build). EXPECT the first line `exit 0`; the second line prints nothing above its `exit 0`. Any output: STOP-BUILD-DIFFERS.
     ```bash
     cd /c/bridge-lab/1381-bx && git -C <CLONE> show 48a6496507132576a6be150c155192f647528e1e:src-tauri/src/agent_master_review_tests.rs > local/gen/agent_master_review_tests.rs; echo "exit $?"
     cd /c/bridge-lab/1381-bx && B=$(sed -n 's/^BUILD COMMIT: \([0-9a-f]\{40\}\)$/\1/p' BUILD.txt) && git -C <CLONE> diff --stat 48a6496507132576a6be150c155192f647528e1e "$B" -- src-tauri/src/tally/approved_import.rs src-tauri/src/tally/box_fit.rs src-tauri/src/lib.rs src-tauri/src/bin src-tauri/src/agent_master_review_tests.rs > local/evidence/s0-fill-check.txt 2>&1; echo "exit $?" >> local/evidence/s0-fill-check.txt; cat local/evidence/s0-fill-check.txt
     ```
   - Run all seven Self-checks. Any miss: STOP-SELFCHECK.
   - `BUILD.txt` filled. EXPECT `BUILD.txt: 0 field(s) to fill: filled`, exit 0. Exit 1: STOP-BUILD-UNFILLED.
     ```bash
     cd /c/bridge-lab/1381-bx && python -I -X utf8 build.py check > local/evidence/s0-build-check.txt 2>&1; echo "exit $?" >> local/evidence/s0-build-check.txt; cat local/evidence/s0-build-check.txt
     ```
   - The exe now on GRANTED PATH, read only: row 2569-R's, as `BUILD.txt`'s ASIDE SHA256 says. EXPECT `: OK`, exit 0. `FAILED` or no file: STOP-ASIDE; nothing is staged.
     ```bash
     cd /c/bridge-lab/1381-bx && A=$(sed -n 's/^ASIDE SHA256: \([0-9a-f]\{64\}\)$/\1/p' BUILD.txt) && echo "$A  C:/bridge-lab/2569-r/exe-new/mcpb/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" | sha256sum -c - > local/evidence/s0-aside.txt 2>&1; echo "exit $?" >> local/evidence/s0-aside.txt; cat local/evidence/s0-aside.txt
     ```
   - Processes. EXPECT for `bridge_mcp.exe` the one line `INFO: No tasks are running which match the specified criteria.`, exit 0 (else STOP-STRAY); for `tally.exe`, record what is there (none, or one line with its PID).
     ```bash
     cd /c/bridge-lab/1381-bx && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > local/evidence/s0-bridge.txt 2>&1; echo "exit $?" >> local/evidence/s0-bridge.txt; cat local/evidence/s0-bridge.txt
     cd /c/bridge-lab/1381-bx && tasklist //FI "IMAGENAME eq tally.exe" //FO CSV //NH > local/evidence/s0-tally.txt 2>&1; echo "exit $?" >> local/evidence/s0-tally.txt; cat local/evidence/s0-tally.txt
     ```
   - **The screen and the grant, recorded.** With no `bridge_mcp.exe` running: one screen-tool screenshot of the whole screen. EXPECT no dialog (a licence dialog: B1, STOP-DIALOG); Tally, if running, at Select Company with no company loaded (else STOP-TALLY, touch nothing). Then `list_granted_applications`; record every entry with its path in `local/s0-grants.txt` (typed from the tool's answer), and an evidence copy `local/evidence/s0-grants.txt` writing every path outside `C:\bridge-lab` as `<path>`. EXPECT GRANTED PATH among them; absent: STOP-GRANT.
   - The display. EXPECT `RESULT exit 0` and the facts in Display facts; record them.
     ```bash
     cd /c/bridge-lab/1381-bx && powershell -NoProfile -File screen.ps1 -Out local/evidence/s0-screen.txt > local/evidence/s0-screen-out.txt 2>&1; echo "exit $?" >> local/evidence/s0-screen-out.txt; cat local/evidence/s0-screen-out.txt
     ```
1. **The build: download, verify, stage.** Only on the run coordinator's word for the download.
   - Download the artifact named in `BUILD.txt`. EXPECT `exit 0` and a file; any other: STOP-DOWNLOAD (nothing staged).
     ```bash
     cd /c/bridge-lab/1381-bx && mkdir local/download && A=$(sed -n 's/^ARTIFACT ID: \([0-9][0-9]*\)$/\1/p' BUILD.txt) && cd <CLONE> && gh api "repos/{owner}/{repo}/actions/artifacts/$A/zip" > /c/bridge-lab/1381-bx/local/download/artifact.zip; echo "exit $?"
     ```
   - Verify. EXPECT the zip, mcpb and exe lines and `VERIFIED: …`, exit 0. Exit 1: STOP-BUILD-HASH or STOP-BUILD-LAYOUT as printed.
     ```bash
     cd /c/bridge-lab/1381-bx && python -I -X utf8 build.py verify local/download/artifact.zip > local/evidence/s1-verify.txt 2>&1; echo "exit $?" >> local/evidence/s1-verify.txt; cat local/evidence/s1-verify.txt
     ```
   - Stage. EXPECT `granted path before: sha256` equal to ASIDE SHA256 and `STAGED: …`, exit 0, and `local/evidence/copies.txt` with three lines (extract, move-aside, copy-in). Exit 1: STOP-STAGE, nothing further moved.
     ```bash
     cd /c/bridge-lab/1381-bx && python -I -X utf8 build.py stage > local/evidence/s1-stage.txt 2>&1; echo "exit $?" >> local/evidence/s1-stage.txt; cat local/evidence/s1-stage.txt; cat local/evidence/copies.txt
     ```
2. **The arms.** Each arm: its start line as a background run; the wait; then its after-lines. Nobody at the PC and no screen-tool act from the start line to `arm exit` (B2).
   - **After every arm**, typed with `<arm>` replaced by the arm's name and `<mode> <answer>` by its table row (`masters No`, `stock No`, `stock Cancel`, `stock Yes`):
     ```bash
     cd /c/bridge-lab/1381-bx && cat local/evidence/<arm>-out.txt
     cd /c/bridge-lab/1381-bx && python -I -X utf8 selftest-box.py replay C:/bridge-lab/1381-bx <arm> <mode> <answer> > local/evidence/<arm>-replay.txt 2>&1; echo "exit $?" >> local/evidence/<arm>-replay.txt; cat local/evidence/<arm>-replay.txt
     cd /c/bridge-lab/1381-bx && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > local/evidence/<arm>-after-tasklist.txt 2>&1; echo "exit $?" >> local/evidence/<arm>-after-tasklist.txt; cat local/evidence/<arm>-after-tasklist.txt
     ```
     EXPECT `ARM <arm> AS THE CODE SAYS` and `arm exit 0`; `REPLAY ALL PASS`, exit 0; no `bridge_mcp.exe`. Then one screen-tool screenshot of the whole screen (no box, no dialog, Tally as at step 0), and the by-eye read of `local/box/<arm>-open.png`. Any `arm exit` but 0, or a `bridge_mcp.exe` still running: STOP, as `arm.py` named it (else STOP-ARM); nothing further.
   - The wait, typed about every 5 s, at most 60 times, until its last line begins `arm exit`; 60 times without: STOP-ARM-WAIT, nothing further:
     ```bash
     cd /c/bridge-lab/1381-bx && tail -n 1 local/evidence/<arm>-out.txt
     ```
   - **LN: the ledger control, answered No.** Start:
     ```bash
     ( cd /c/bridge-lab/1381-bx && if [ -e local/evidence/LN-out.txt ]; then echo "STOP: local/evidence/LN-out.txt exists"; else python -I -X utf8 arm.py LN < /dev/null > local/evidence/LN-out.txt 2>&1; echo "arm exit $?" >> local/evidence/LN-out.txt; fi )
     ```
     EXPECT as the table. Any deciding line not so: STOP-CONTROL, and the stock arms are not run.
   - **SN: the stock box, answered No.** Start:
     ```bash
     ( cd /c/bridge-lab/1381-bx && if [ -e local/evidence/SN-out.txt ]; then echo "STOP: local/evidence/SN-out.txt exists"; else python -I -X utf8 arm.py SN < /dev/null > local/evidence/SN-out.txt 2>&1; echo "arm exit $?" >> local/evidence/SN-out.txt; fi )
     ```
   - **SC: the stock box, answered Cancel.** Start:
     ```bash
     ( cd /c/bridge-lab/1381-bx && if [ -e local/evidence/SC-out.txt ]; then echo "STOP: local/evidence/SC-out.txt exists"; else python -I -X utf8 arm.py SC < /dev/null > local/evidence/SC-out.txt 2>&1; echo "arm exit $?" >> local/evidence/SC-out.txt; fi )
     ```
   - **SY: the stock box, answered Yes.** Start:
     ```bash
     ( cd /c/bridge-lab/1381-bx && if [ -e local/evidence/SY-out.txt ]; then echo "STOP: local/evidence/SY-out.txt exists"; else python -I -X utf8 arm.py SY < /dev/null > local/evidence/SY-out.txt 2>&1; echo "arm exit $?" >> local/evidence/SY-out.txt; fi )
     ```
     The token printed is the arm's own: it approves nothing, since no parent is waiting for it.
3. **Close.**
   - Only on the run coordinator's RESTORE word: put row 2569-R's exe back. EXPECT `RESTORED: …`, exit 0, and `copies.txt` with five lines. Without the word the row's exe stays on GRANTED PATH and the aside exe in `local/aside/`, both recorded.
     ```bash
     cd /c/bridge-lab/1381-bx && python -I -X utf8 build.py restore > local/evidence/s3-restore.txt 2>&1; echo "exit $?" >> local/evidence/s3-restore.txt; cat local/evidence/s3-restore.txt; cat local/evidence/copies.txt
     ```
   - The state left, and the files. EXPECT no `bridge_mcp.exe`; `check_sheet.py` exit 0.
     ```bash
     cd /c/bridge-lab/1381-bx && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > local/evidence/s3-bridge.txt 2>&1; echo "exit $?" >> local/evidence/s3-bridge.txt; cat local/evidence/s3-bridge.txt
     cd /c/bridge-lab/1381-bx && python -I -X utf8 check_sheet.py > local/evidence/s3-sheet.txt 2>&1; echo "exit $?" >> local/evidence/s3-sheet.txt; cat local/evidence/s3-sheet.txt
     cd /c/bridge-lab/1381-bx && find local -type f ! -name s3-listing.txt | sort | xargs -d '\n' sha256sum > local/evidence/s3-listing.txt 2>&1; echo "exit $?" >> local/evidence/s3-listing.txt; wc -l local/evidence/s3-listing.txt
     ```
   - One screen-tool screenshot of the whole screen: no box, no dialog, Tally as at step 0 (B9).

## Record

For each arm: `local/evidence/<arm>-out.txt` (the whole of `arm.py`'s report), `<arm>-arm.txt`, `<arm>-replay.txt`; the box log, guard, R1 read and check (`local/box/`); the input fed and its hash, the screen read, `startc.ps1`'s lines, the child's standard output (`<arm>-out.bin`, its bytes) and error output (`local/run/`); the by-eye line. For the sitting: step 0's files, `copies.txt`, `s3-listing.txt`, the notes. Screen copies and screenshots stay on the lab PC, listed by name, size and sha256 (B8). The evidence branch carries `local/evidence/`, `local/box/` without its `.png` files, and `local/run/`.

## Decides

- **Q1** by SN, SC and SY: `boxcheck.py`'s `ALL PASS` and the replay's `REPLAY ALL PASS` on each (title, three buttons, the text line for line), the guard (inside the work area, topmost, first in the z-order), each `fits=true`, and the by-eye read (no line cut); its "No the default" part by W3, read on each stock arm. A miss on any stock arm answers Q1 no, with the line that missed.
- **Q2** by SN and SC (0 bytes, exit 1) and SY (the token for its nonce and one LF, exit 0): the deciding lines of `<arm>-arm.txt`.
- **Q3** by LN: its deciding lines, against lab 1780-K's box and outcome; W5 compared with lab 1780-K's rectangle is recorded.
- Every W line is recorded with its arm and stops no arm; W3 is the one that answers part of a question (Q1).

## Not covered

- **Too large.** No arm makes the box not fit: that needs a smaller work area or a larger scale, and on this lab both are display settings a person changes. So exit 77 (the child's no-fit end) is not observed here; the code is cited only. A review's plan asked that a stock box that cannot be measured whole end as `master_review_too_large`: at 48a64965 that string is the ledger review's (code `src-tauri/src/agent_master_review.rs:48`); the stock parent maps a no-fit exit to `StockItemDialogRefusal::TooLarge` (code `src-tauri/src/tally/approved_import.rs:3074`), and the one caller of the stock dialog, `confirm_stock_item_draft`, is reached from tests only at this commit (code `src-tauri/src/agent_master_review.rs:349-358`, `#[cfg_attr(not(test), allow(dead_code))]`), so no answer string exists yet to measure.
- **The parent.** The server path (the nonce check, the 120 s limit, the refusal on a wrong token) is not run: no tool reaches the stock dialog at this commit (above).
- **Other displays.** One monitor at the lab's one scale. No second monitor, no other scale, no taskbar position, no smaller work area.
- **Other ways to answer.** Only the Yes, No and Cancel buttons by the mouse. Not the Y and N keys the answer lines name, not Escape, not the close box.
- **Between the guard and the click.** `box.ps1` clicks the button's centre without reading the box again; that nothing covers it in those seconds rests on B2 (nobody at the PC), not on a check.
- **macOS.** The Mac alert is another arm on another machine, not this row.
- **The trailing line break of the ledger input.** No earlier box log holds a text ending in a line break, so whether `box.ps1` reads LN's final LF back is not shown; a miss there stops LN at STOP-CHECK with the box up (B4), and is a finding about the reader, not the box.

## Close and evidence

Step 3 leaves no box and no `bridge_mcp.exe`, Tally as found, and GRANTED PATH as the run coordinator's word says. Report to the run coordinator: each arm's `ARM` line and exit, every FAIL line, the W lines, the display facts, and any STOP with its name and files. The evidence branch is pushed only after the pre-push scan and the run coordinator's yes (B8).
