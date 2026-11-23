# Row 2162-SALES-Q: results

Sheet `lab/2162-sales-q-sheet`, run from 843fd68a7985d9f903171d5b9be854e1a9842571 and, from Part G's second sitting on, from revision 4 at 3bd08d0267a4387eb6d31ed73de10e6e5c1a5e41 (boxcheck.py's Narration line and the self-test; one commit on top of 843fd68a). Run 23 Nov 2026, 12:20Z to 14:17Z, on TallyPrime 7.2 Silver (the title band reads TallyPrime SILVER; Tally's current date Tuesday 15-Sep-2026, period 1-Apr-26 to 31-Mar-27), book BRIDGE PILOT LAB, GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85 (synthetic), loaded alone. Build: exe sha256=1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59, reports bridge-tally 0.7.0, each launch by `client.py` with the sheet's committed launch file. One line per step in UTC: `evidence/notes.txt`.

## Each part's result
| part | result | confidence |
|---|---|---|
| **P (item 1)** | The build's `gst_status` reads the keyed post-dated invoice `BP/26-27/0073` as `gst_status` `post_dated` with `status_flags` `included` (L1's 04; status-diff against L0's 04: exactly one row in L1 only, master_id 45). The raw read (s4) gave `ISPOSTDATED` Yes and included Yes. As expected. | partial (one voucher, one sitting, one release) |
| **R-B (inactive)** | With the one registration inactive from 1-Sep-2026 (Tally's own dated row, s6), the build admitted the 1 Aug invoice (05: one voucher, batch file 2,421 B equal to the answer's sha256) and refused the 15 Sep one as `invoice_company_registration_inactive` (06). As expected. | partial |
| **R-C2 (two registrations)** | With a second registration in force (Maharashtra, keyed at step 9; s9), the build refused both dates as `invoice_company_registration_ambiguous` (05 and 06), no file written. As expected. | partial |
| **G (the split, through the build)** | The build's post of `BP/26-27/0072` ended `posted_verified` on the first call (`created` 1, every other counter 0; `verify_import` `posted_verified`). GSTR-1's HSN summary moved by exactly row 2162-HS's raw deltas: 998313 by 630.42 / 600.40 / 15.01 / 15.01 / 30.02, 998314 by 420.42 / 400.40 / 10.01 / 10.01 / 20.02, the Total by 1,050.84 / 1,000.80 / 25.02 / 25.02 / 50.04, every other line unchanged, counts 40 / 28 / 7 / 5. Tally stored the six legs as sent, and count19's diff against HS's raw `0072` shows legs a=6, b=6 and no field line. As expected. | partial (one invoice) |
| **G, the block** | The six block fields of the build's post (V-0072) equal the keyed invoice's (V-0073): the three names `BRIDGE Walk-in`, `CONSIGNEESTATENAME` Rajasthan, `CONSIGNEECOUNTRYNAME` India, `VATDEALERTYPE` Regular (the keyed invoice's `VATDEALERTYPE` recorded: Regular). | partial (two invoices that differ in more than the sender) |
| **The smoke box** | First sitting: the helper saw the box and clicked No, but the replay was not ALL PASS: its Narration check compared the box's narration with the saved file's, and the saved file's narration carries the build's `[BRIDGE:<id>]` tag, which the box never shows (code: the box shows `posted_narration`). Part G stopped there, nothing posted. Revision 4 strips one trailing tag in boxcheck.py; its 34 self-test cases all right; the replay of the first sitting's kept box then ALL PASS. Second sitting: box seen, CLICKED No, box visible False, replay ALL PASS; then the real box: check ALL PASS, CLICKED Yes 0.6 s after BOX SEEN. | verified for this invoice box |

## Two findings
1. **A build refused after a GSTR-1 walk; cause a candidate, not shown.** After the second restore (R2) and step 12's GSTR-1 walk, the smoke build of `0074` was refused `invoice_sales_ledger_rate_shape_unmeasured` for both service ledgers (`BRIDGE Svc 998313 5%`, `BRIDGE Svc 998314 5%`), byte-identical in two launches (the first LG launch and the repeat: request sha256=6d6404a2e22acbf8d4acfc2e8c1f7b4cee998157358aed8da1a787cfa367836e, evidence 62,071 B, the same response sha256 in both). After a third restore (R3, a reload with no walk) the same call was admitted in three launches (request sha256=73e152f0d2a4..., evidence 1,974,198 B, the request equal to the admitted L0, L1 and LB-rerun builds). After the post and step 15's walk, a fourth launch (the walk test) was refused again with the same two refusals and the same request sha256, evidence 62,081 B. The candidate cause is session state left in Tally by the walk; it is not shown, because the build keeps no response bytes (its records hold request and response sizes only, and no file in any data folder names the ledgers' GST detail lists). For the walk test, the posted `0072` also sits between the admitted build and the refused one, so the walk is not its only difference. The request sha256 marks the reads sent before a refusal, not its cause: R-B's refused build (inactive) and R-C2's two (ambiguous) carry the same request sha256 and evidence size, 62,071 B.
2. **The box check's Narration line (fixed in revision 4).** See the smoke box above. The defect was in the lab's check, not the build: the box and the saved file each say what the code says they say.

## Each step's EXPECT
| step | EXPECT | met |
|---|---|---|
| 0 | files equal to SHA256.txt; selftest 29 cases right; C2 with no company loaded | met |
| 1 | the book's GUID, ALTVCHID 68, ALTMSTID 245, date 15-Sep-2026; BK1 taken (90,051 B, sha256=1313f43967b2d93b6352162b4b930933c418fdf99e8de182db14508acd6cc8b0) | met |
| 2 | two tax units, N-0072 / N-0073 / N-0074 0 rows, R-0010 six legs, marks equal | met |
| 3 | L0: 37 tools, the book, M 245 V 68, 04 total 32 with no post-dated row, 05 and 06 one voucher each, M 245 V 68 after (a build sends nothing) | met |
| 4 | the post-dated invoice keyed and saved; s4: N-0073 one row, `ISPOSTDATED` Yes, included Yes, M 245 V 69 | met, with the deviations below |
| 5 | L1: 04 total 33, `0073` `post_dated`, `included`; 05 and 06 built | met |
| 6 | the registration Inactive from 1-Sep-2026 by Tally's own dated row (9 leaves differ, RU's shape) | met |
| 7 | LB: 05 (1 Aug) admitted, 06 (15 Sep) `invoice_company_registration_inactive` | met on the rerun (deviation 4) |
| 8 | R1: marks 68/245 as s1, 0 leaves differ, N-0073 0 rows, R-0010 legs a=6, b=6 | met |
| 9 | the second registration keyed, three named prompts answered as named | met |
| 10 | LC: 05 and 06 `invoice_company_registration_ambiguous` | met |
| 11 | R2, graded as step 8 (N-0074 0 rows) | met |
| 12 | GSTR-1 for 1-Aug-26 to 31-Aug-26, counts and lines equal to HS's before | met |
| 13.1 to 13.3 | LG, 05 not refused | not met in the first sitting (finding 1); met after R3 |
| 13.4 | smoke box declined by the helper, replay ALL PASS | not met in the first sitting (finding 2); met in the second |
| 13.5 to 13.12 | `0072` built, batch file equal, M 245 V 68 before; `posted_verified`; verify `posted_verified`; status-diff one row in B only (master_id 45, alter_id 69); M 245 V 69 | met |
| 14 | N-0072 one row; R-0072 the six legs as sent; diff against HS's raw legs a=6, b=6; the block as above; R-0010 unchanged; M 245 V 69 | met |
| 15 | HS's exact deltas (Part G above); the look changed no mark | met |
| 16 | the final restore graded: marks 68/245 as s1, 0 leaves differ, N-0072 / N-0073 / N-0074 0 rows, R-0010 legs a=6, b=6, date 15-Sep-2026 | met (R4; see deviation 7) |
| 17 | C10: the company shut, one tally.exe on 9001, `/status` 200, no `bridge_mcp` | met |

## Deviations, each recorded in the notes
1. **The interpreter.** `python3` on the lab PC is the Store alias; every `python3 -I` ran as `python -I` (Python 3.10.11).
2. **Step 3, an operator slip.** `put.ps1` was first given a name that is not a call file; it stopped at its hash check before any copy. Nothing was sent.
3. **Step 4.** The key-by-key typing dropped the space in `BRIDGE Sales`; repaired by Backspace and the space key. The list then showed four lines with none highlighted, a stop; on the recorded word, Down once and Enter on `BRIDGE Sales` alone. The type list also holds `BRIDGE Sales Z0`, which the sheet's list does not name. F2 offered 16-9-2026; the No. field offered `1`. Two later notes lines correct their own stamps and one zoom word.
4. **Step 7, the first LB launch stopped at 03.** The client opened the call file while the copy helper still held it (a lab-helper race). Nothing reached the build. On the recorded word the data folder was renamed (not deleted) and the launch rerun as `run-b2` with a helper that copies to a `.part` name and renames it.
5. **Part G's first sitting stopped twice.** At 13.2 (finding 1) and, after R3, at 13.4 (finding 2). On the recorded word: two repeat launches, a restore (R3) graded as step 16 before Part G, Part G run from R3's state with no GSTR-1 walk before it, and step 12's reading standing as Part G's before. Each earlier `run-g` and `data-g` was listed and renamed before the next launch (`run-g-x0`, `run-g-x1`, `data-g-x1` to `data-g-x5`); nothing was deleted.
6. **A read log.** One `run.py` started from PowerShell with its output redirected onto the log file that `run.py` writes itself; it stopped before any request. The failed log was kept aside and the read rerun from Git Bash.
7. **The final restore.** R3 ran before Part G's second sitting, so the sheet's step 16 ran as R4 after the walk test, with the same checks: BK1 re-hashed equal, the restore plan and folder listing first, C6's two prompts only.
8. **The walk test** (one launch after step 15: calls 01, 02, 03, 05, 07, no box, no post) is not in the sheet; it ran on the recorded word.
9. **Step 12's transcription.** One Particulars line (`0019`) was written as `BRIDGE Walk-in`; its own capture reads `Bridge Walk-in`. Corrected by an appended line, amounts unchanged.
10. **Step 15's screens** were read from the session's own views; only the two Gateway captures were saved (step 12 saved six).
11. **A request hash check before the final reads** first printed a difference for each file: its pattern missed SHA256.txt's size column. Rechecked, each equal, and the reads graded.

## The record
Each launch's 01 (`list`, eleven launches: run-0, run-1, run-b, run-b2, run-c, run-g-x0, run-g-x1, run-g2, run-g3, run-g, run-g4) is graded by `calls.py show`'s line, the same in all eleven: `tools: 37; missing of this row's: []`. Those eleven answers stay on the lab PC (they hold the product's own tool descriptions, whose words the public-text gate flags), listed in `kept-local.txt` by sha256 and bytes.

Answers of every launch (`run-*`) but those, Tally's raw answers and run.py logs (`reads/`), the notes and every whole helper output (`evidence/`), the restore plans with `<company number>`. Captures, the box logs, BK1, the folder listings, Bridge's data folders and the lab helpers stay on the lab PC, listed by size and sha256 in `kept-local.txt`; the grading lines from them are quoted in the notes and above.
