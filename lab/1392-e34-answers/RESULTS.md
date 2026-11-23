# Row 1392-E34: results

Sheet `lab/1392-e34-sheet` at 0e2fc769. Run 23 Nov 2026, 11:03Z to 11:21Z, on TallyPrime 7.2 Silver (Tally's date Tuesday 28-Apr-2026), book BRIDGE TRANSFER LAB, GUID d6e4c24f-8c7a-4797-93ec-e44bdff60bc9. Build NEW: exe sha256=1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59, reports 0.7.0. One `bridge_mcp` (pid 10680) under `client.py` for steps 2 to 7. Run order 0, 1, 2, 6, 3, 4, 5, 7, 8, 9, as the sheet says. One line per step in UTC: `evidence/notes.txt`.

## Each case's state
- **K1 (6.1): set up as designed.** The call returned `pending` after 40.114 s (call end 11:08:46.083Z). `box.ps1` saw the box at 11:08:13.191Z (title `post this voucher?`, naming K's batch) and clicked No at 11:09:23.493Z, 37.4 s after the call's end. Then box visible False.
- **K2 (6.2): EXPECT met.** Answered in 0.071 s: `import_approval_declined`, `error.cause` `approval_answered_earlier`, `error.next_step` beginning `This call showed no approval box.`, `attempt_recorded` false. NO BOX within 20 s (exit 3). One `bridge_mcp.exe` and no `--confirm-journal` process (`evidence/s6-k2-procs.txt`).
- **K3 (6.3): a new box shows.** BOX SEEN at 11:10:10.340Z, pid 18520, title `post this voucher?`, its `Batch:` line naming K. `evidence/s6-k3-procs.txt` holds pid 18520 as a `--confirm-journal` child of 10680. CLICKED No at 11:10:30.459Z, box visible False, `box.ps1` exit 0. The call answered after 27.272 s with `import_approval_declined` and no `error.cause`, `attempt_recorded` false.
- **Smoke test (6.4, 6.5): passed.** s6-marks V 12, M 211, byte-equal to m2.
- **B3 (step 3): posted.** Box seen at 11:13:22.078Z. The check printed ALL PASS on type, date and both lines (Contra, 29-Oct-2026, Dr 7003.00 TLab Bank B, Cr 7003.00 TLab Bank A), GUID and Batch. CLICKED Yes 0.8 s after the box was seen. `posted_verified`, `created` 1. `verify_import` `posted_verified` 1 of 1. V 13, M 211.
- **E3 (step 4): holds.** A3's build answered `skips_recorded`, `skipped_rows` 1. In `transfers`: `state` computed, `skips_paired` 1, `skips_open` 0, `open` empty. V 13, M 211.
- **B4 (5a): posted.** `voucher_count` 3: Receipt 1 Nov 1,204.00, Contra 2 Nov 9,004.00 (TLab Bank B Dr, TLab Bank A Cr), Receipt 3 Nov 1,205.00. Box title `post 3 vouchers?`, check ALL PASS, CLICKED Yes. `posted_verified` 3 of 3, `created` 3. V 16, M 211.
- **A4's build (5b): `matched_unconfirmed`, as designed.** `voucher_count` 2, the two Receipts (1 Nov 1,206.00, 3 Nov 1,207.00). In `transfers`: one open row of 2 Nov 2026, `matched_unconfirmed`, naming B4's batch. `skips_paired` 1, `skips_open` 1.
- **E4 (5c): holds.** Box title `post 2 vouchers?`, check ALL PASS, CLICKED Yes. `posted_verified` 2 of 2, `created` 2. V 18, M 211. A4's `verify_import` `transfers`: `skips_paired` 2, `skips_open` 0, `open` empty.
- **End of Bridge (step 7).** m3 V 18, M 211 (m0b's V 12 plus 6). The session ended within 2 s of the stop file, with server exit code 0, and no `bridge_mcp` was left. The server's error output is 0 B.
- **Restore of BK-E2 (step 8): the book is back.** The C6 checks and plan were done before the shut. Restore list: BRIDGE TRANSFER LAB (`<company number>`), 23-Nov-26 (4:36 pm). Both expected prompts were answered (Accept y, Replace r) and no other prompt showed. m4 GUID d6e4c24f-8c7a-4797-93ec-e44bdff60bc9, V 12, M 211, byte-equal to m0 and m0b (answer sha256=174d6672e4ce1d1ec2efec5dd1d0793b152cdc76d4626f27f3e3e521797c6b2e).
- **Close (step 9).** The company was shut. One `tally.exe` on 9001, `/status` 200. No request was sent after the shut. The password folder went to the Recycle Bin, and Test-Path then read False.

## What the result decides
| step | result | reading on #1392 | confidence |
|---|---|---|---|
| 3 | B3 `posted_verified`, V 13 | the fixed box check passes a one-voucher box: 5 Nov's helper defect is closed on the build | verified |
| 4 | `skips_paired` 1, A3 not open | **E3 holds, by reference.** | verified |
| 5b | A4 `matched_unconfirmed` at build | round 2 found the date pair but could not confirm A's cover yet, as designed | verified |
| 5c | `skips_paired` 2 in A4's `verify_import` | **E4 holds:** with both statements verified around 2 Nov, a row with no reference pairs by date | verified |
| 6.1 | the No clicked 37.4 s after K1 returned `pending` | the set-up of 5 Nov's call 2, reproduced on purpose | verified |
| 6.2 | `import_approval_declined`, cause `approval_answered_earlier`, no box, no `--confirm-journal` child | a kept No is returned once, at once, and says so (#2521) | verified |
| 6.3 | BOX SEEN naming K, a `--confirm-journal` child, CLICKED No, then a decline with no cause | **a repeat call after a kept No shows a new box**, as #2521's `next_step` says | verified |

The rows of the table not listed (A3 open, A4 still `matched_unconfirmed` after verify, a K2 with no cause, a K3 with no box, a mix, a refusal) did not occur.

## Deviations, each recorded in the notes
1. **Step 8, a Tally prompt raised by the operator.** After the shut, Tally showed Select Company. One Escape there raised Tally's own `Quit ?` prompt, a known behaviour of that screen. The capture is kept local; the foreground window belonged to the 9001 `tally.exe`. It was answered N and Tally returned to Select Company. No company was loaded, no Bridge process ran, and nothing was sent to Tally while the prompt was up. The restore then ran as C6 says. Under the sheet's stop rule (any dialog on Tally's screen) this is a dialog, and it is reported here for the run lead's word. The book's state after it is m4, equal to m0.
2. **Step 8, a listing rewritten.** `restore-prep.py after` was run twice: at 11:20:11Z, just after the restore (17 files, 276,992 B), and again at 11:20:31Z, after the load (23 files, 293,376 B). The second run rewrote the lab-PC listing file, so the 17-file listing survives only as its count line in `evidence/company-folder.txt`. The before listing (24 files) is whole on the lab PC.
3. **Step 9, the password folder** went to the Recycle Bin, not through `Remove-Item`: the lab does not delete files.
4. **6.1, `box.ps1`'s exit code for K1 was not captured.** The launching shell held no process handle, so `$b.ExitCode` was empty. Later launches held one. K1's box log is whole.
5. **The box logs stay on the lab PC.** Every box log of a box that was shown (k-p1, k-p3, b3-p1, b4-p1, a4-p1) holds the box's `Company number:` line. So do the self-test's saved logs and `p.py`'s three printouts (`s3-p.txt`, `s5a-p.txt`, `s5c-p.txt`), which repeat the box log. They are listed in `kept-local.txt` by size and sha256. Their lines that grade a step are quoted in this file and in the notes. The check outputs (`*-check.out`), `k-p2-box.txt` and every `-ps.out` are on the branch.
6. **Precheck.** `PendingFileRenameOperations` is set on the PC. The sheet does not check it; `RebootRequired` and `RebootPending` were both False.
7. **The sheet's archive.** The `run.tar` used to unpack the sheet was moved aside after unpacking, not removed.

## Kept on the lab PC
The statements, the password file, the screenshots and box captures, Bridge's data folder, BK-E2, the process and port listings in full, the password file's ACL read (it names the PC), and the captured files that hold the company number. Each is listed in `kept-local.txt` by size and sha256. BK-E2: 32,428 B, sha256=4432a3583dbca444bba6541facaca9d94c43e4fdd12ab5ad0d28cce4274c1e1d. The ACL read shows SYSTEM, Administrators and the lab user, each with inherited full control, and no one else.
