measured under TallyPrime 7.2 Educational Mode (Silver Rental expired 24-Nov-2026); the sheet's Book section assumed Silver

# Row 2162-PA: results

Sheet `lab/2162-pa-sheet` at f682a6aa154247693b5de6f1b182ab0c72a5e2ca. Run 2026-11-25, 17:26Z to 17:51Z, in one sitting, book BRIDGE PILOT LAB, GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85 (synthetic), loaded alone; Tally's current date Tuesday 15-Sep-2026, period 1-Apr-26 to 31-Mar-27, no date or period key pressed; tally.exe pid 23160 on port 9001. One launch, LPA, of **BUILD 6f541ae4** (exe sha256=9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc, the exe row 2162-RC ran, on the lab disk), by `client.py` with the sheet's committed launch file, started whole as a background run of the operator's shell tool (Git Bash; no window, standard input from /dev/null). It reported bridge-tally 0.7.0.

**The edition.** Step 0's C2 (17:30Z) read the title TallyPrime SILVER. At step 1 (17:31Z), before the load, the title read TallyPrime EDU: the Silver Rental licence had lapsed between the two. The row held twice and went on in Educational Mode on the run lead's word, which added three stops: a title other than EDU before the post line, any licence or renewal dialog (answering nothing in it), and a refusal or question at C5 or C6. Every result below was measured in Educational Mode; the sheet's Book section, and row 2162-RC whose bytes it cites, assumed Silver.

## Q1 and Q2: not measured

**Q1: not measured.** **Q2: not measured.** BUILD 6f541ae4 did not admit the test invoice: call `05` (`build_import_xml`, sent 17:47:59Z, 4.75 s) answered `isError True` with error code `window_part_boundary_unsupported_in_education` and the message "ComplyEaze Bridge refused this operation"; evidence state `partial`, 124,154 B, request sha256=f1c5d5442e36e25e3b9ff0dcb152c196187e1351f3558409de9c3667cdf8eb81, response sha256=5084fbab19b6e93e3f52d9dad98509446a382bf11687a1fca67382de28550938; company state `not_company_scoped`; no `voucher_count`, no `batch_id`, no refusal list. `grade05.py`: `VERDICT other`, exit 0. So no box was raised, nothing was posted, and neither question was reached. Confidence: verified for this sitting (the answer is kept whole in `run-pa/answers/`, company_number redacted).

The refusal is the build's own read-window guard, not the rate-row question this sheet follows: at 6f541ae4, `src-tauri/src/agent_voucher_window.rs:424-432` (`EDUCATION_BOUNDARY_UNSUPPORTED`) refuses a read plan with a part that starts or ends on a day other than the 1st, 2nd or 31st when the endpoint is in Education mode, because Education answers such a read with a well-formed empty collection (bridge#581). The build's egress journal (`data-pa/agent-egress.jsonl`, kept local) shows its request trail for the call as `sent 71, failed 0`, the last 32 listed all `answered` (19 posts, 13 status checks), 39 omitted from the list; its own rate read in that trail answered 30,626 B, the grown size. Every request was answered; the guard refused the plan.

By the sheet's Decides ("`05` refused"): nothing was posted; Q1 and Q2 are not answered. The reason is the edition, not the state or the build: the same build admitted the same invoice in the same grown bytes on Silver (row 2162-RC). Confidence of that reading: partial (one book, one release, one sitting; Silver and Educational Mode not run side by side).

## P0, G, PA-pre, PA, PA-post and R

- **P0: plain.** A0-first and A0-second both 29,114 B, sha256=a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62, `LEDGER ROWS 6`, all six measured, no structural difference; marks equal to `start`. A0-third not needed. Confidence: verified.
- **G: grown.** G's Shut Company met the licence pair (below: No, then Continue In Educational Mode), then the load. G-pre and G-pre2 plain and byte-equal to A0-second (the reload cleared any earlier state). After the W1 walk, G read 30,626 B, sha256=d7f5db99170ea9a643fab195a9465bd20d1f1af1102b76fbefaefe6673a12257; `ratecmp.py` A0-second against G: added 28, removed 0, text changed 0, every added path under `BRIDGE Svc 998313 5%` or `BRIDGE Svc 998314 5%` (`GSTRATEPERUNIT` and its `TYPE` on each of the five rate rows), those two unmeasured, the other four measured; marks equal. This repeats row 2162-RC's grown bytes exactly, under Educational Mode. Confidence: verified.
- **PA-pre: not reached.** Part 6c follows an admitted `05`.
- **PA: refused at `05`**, as above. Parts 6, 6b, 6c and 7 to 11 not run: no batch file, no `rb.xml`, no `pa.py`, no box, no raw read-back, no `verify_import`, no `07`.
- **PA-post** (LPA closed at part 12): marks V 68, M 245, equal to `start` (count19's before-write mode prints them; its same-row mode, added, prints `equal`); rates 30,626 B, sha256=d7f5db99…, still grown after the launch. Confidence: verified.
- **R: not run.** Nothing was posted, so the RESTORE condition in the GO (a post to undo) was not met; BK1 stays on the lab PC unused.

## The licence acts, each recorded in the notes

| UTC | where | screen (serial not copied) | act |
|---|---|---|---|
| 17:31Z | step 1, Select Company, before the load | the title read TallyPrime EDU (step 0: SILVER) | none; stop, reported |
| 17:33Z | Select Company, on the run lead's word | F1 Help, key b: About. Application status Educational Mode; License Error: Rental period expired; Edition Silver Rental; Rental Validity expired 24-Nov-2026 | read only; one Escape back to Select Company (allowed by the run lead); stop, reported |
| 17:40Z | G frame a, after Shut Company's Enter | a dialog over an empty Gateway: the Rental License has expired; "Do you want to visit Tally website to renew your license?" Yes or No | none; stop, reported |
| 17:41Z | the same dialog, on the run lead's word | as above | zoom, then a click on No; no key |
| 17:41Z | after No | a screen titled License: the Rental License has expired; "If you have renewed the license, then click Update Existing License." "Alternatively, select one of the following options." Buttons T: Continue In Educational Mode, U: Use License From Network, P: Update Existing License (P outlined as the default) | none; stop, reported |
| 17:42Z | the License screen, on the run lead's word | as above | zoom, then a click on T: Continue In Educational Mode; no key. Select Company returned, title EDU, no dialog |
| 17:51Z | C10's Shut Company | none appeared | none needed |

No renewal, licence or activation screen was opened, no Yes, U, P or "Manage Rental License" was pressed, and no licence account was signed in to. The captures of these screens show the serial and stay on the lab PC. The title read EDU in every screenshot from step 1 to C10.

## The arm table

| arm | state before | act | result | state after | marks before and after |
|---|---|---|---|---|---|
| P0 | step 1's load | two raw rate reads | plain, byte-equal (A0-third not sent) | plain | 68/245, 68/245 |
| G | Shut Company (licence pair: No, T) and load; G-pre and G-pre2 plain | the W1 walk | grown (sha256=d7f5db99…) | grown | 68/245, 68/245 |
| PA | grown (G), no act since G's read | LPA: `01`, `02`, `03`, `05` | `05` refused, `window_part_boundary_unsupported_in_education`; nothing posted | PA-post grown | 68/245, 68/245 |
| R | grown, LPA closed | not run: nothing posted | n/a | n/a | n/a |

Marks are ALTVCHID/ALTMSTID by `count19.py` against `start`. Plain means 29,114 B with sha256=a6197d90…; grown means 30,626 B with sha256=d7f5db99…, each checked `: OK` against the sheet's full hash.

## The launch

LPA, BUILD 6f541ae4: the exe `: OK`; the launch file's sha256 b5f91562f8067c2a15877a6fa24875af62cbb54ee5e3931d9fb14fdfa05302e8, equal to the sheet's; `BRIDGE_AGENT_ENABLE_WRITES present: True`; initialized at 17:47:13Z. `01` `tools: 37; missing of this row's: []`; `02` one company, BRIDGE PILOT LAB, the pinned GUID; `03` M 245, V 68, reused False; `05` as above. Part 12: the log's last line `session ended; server pid <pid> exit code 0` (17:48:30Z), client exit 0, the server error file 0 bytes, no `bridge_mcp.exe` before or after. Bridge's data folder held `agent-egress.jsonl`, the admission lock and `terms-acceptance.jsonl`, and no batch file or import journal.

## Each step's EXPECT

| step | EXPECT | met |
|---|---|---|
| 0 | the sheet's commit fetched and copied (the copy line's two exits both 0); nine self-checks; BUILD 6f541ae4's exe `: OK`; the grant step; C2 with no company loaded: one tally.exe, pid 23160, on 9001, `/status` status=200, bytes=51, no `bridge_mcp.exe` | met (the grant not offered, deviation 1) |
| 1 | the Gateway, 15-Sep-2026; `start` one row, the pinned GUID, V 68, M 245; BK1 one file (90,051 B, sha256=ddc2610a7711d92a2fdf39de5a6a173555faa70969764067af68c060d2a5bcfd); `setup` marks equal; `LEDGER ROWS 6` with the six names and GUID suffixes; `rates.xml` 2,874 B, sha256=182f598aafaf333bc8284b6976e664cee00af1eadb5a29bd4f73c295f8601e08 | met in Educational Mode (deviations 2 to 5); C5 neither refused nor asked anything |
| 2 | A0-first and A0-second plain and byte-equal; marks equal | met |
| 3 | Shut Company and load; G-pre and G-pre2 plain; the walk; G grown, marks equal | met (deviation 6: the licence pair at the Shut) |
| 4 | LPA: `05` admitted, then the post | not met: `05` refused; the sheet's path for a refusal taken (part 12, PA-post, stop); deviation 7 |
| 5 | R only on RESTORE, after a post | not run: nothing posted |
| 6 | `close` without R: V 68, M 245 (nothing posted); C10: Shut Company by the recorded keys, no company loaded, one tally.exe (pid 23160) owning 9001, `/status` 200, no `bridge_mcp.exe`, no XML after the shut | met |

## Deviations and notes, each recorded in the notes

1. **The grant.** `request_access` for the 6f541ae4 exe path was not offered (the path matches no installed app); the row went on with no screen-tool screenshot while the exe ran, as the sheet allows. Accepted by the run lead at step 0.
2. **The edition.** Tally changed from Silver to Educational Mode between step 0 and step 1 (the Silver Rental licence expired 24-Nov-2026). The row held before the load, read the About screen on the run lead's word, and went on under the run lead's added stops (above).
3. **Focus before Enter.** At step 1 the screen tool refused the Enter at Select Company because the operator's own window held the keyboard; no key reached any window. The Select Company filter box was clicked to give Tally the keyboard, then Enter. The filter text and the highlighted line were unchanged.
4. **BK1's time and path.** BK1's time is the backup file's write time (23:08 local, 17:38Z); Tally showed the typed path as `C:\<lab root name>\2162-Pa\backup-1` with its own capitals, as in row 2162-RC. Backup Companies opened with the data path on `C:\` and the cursor on the company line; Backspace opened the List of Folders as the sheet expects.
5. **The shot helper.** Screenshots were saved to `local/shots/` by copying the screen tool's own image of each step; no extra screenshot was taken for this.
6. **The licence pair at G's Shut Company**, answered No and then T by click on the run lead's word (table above). The load that followed met no dialog; G-pre and G-pre2 were plain, so the reload cleared the state as in row 2162-RC.
7. **PA-post read two of its three files.** `PA-post rb.xml` was not sent: `rb.xml` is rendered at part 6b from the admitted build's journal record, and with `05` refused there is none; `rb.xml` and `local/rb.sha256` do not exist. Accepted by the run lead as a correct deviation. `count19.py`'s same-row mode was also run on PA-post's marks (`evidence/PA-post-count19-same-row.txt`).
8. **The waits for answers** were typed as the sheet's `ls` and `tail` line, one shell call each; no answer took more than a few seconds.

Also: Select Company shows no current date, so C10's date check is the Gateway's date at the last load (15-Sep-2026). The notes' header line says "Tally 7.2 Silver": it was written at step 0, before the lapse was seen.

## Not covered

All of the sheet's Not covered, and, because `05` was refused: the post in the grown state, its box, guard and check, the build's read-back and both raw read-backs, `verify_import` and the post's marks; whether this build admits the invoice in Educational Mode with a window whose parts fall on the 1st, 2nd or 31st; whether the licence pair appears at every Shut Company (it appeared once of two).

## The record

Tally's raw answers and `run.py` logs (`reads/`, the company number as X at equal length), the generated `rates.xml`, every `ratecmp.py`, `count19.py`, `calls.py` and `grade05.py` output, the self-checks, the C2 and C10 checks and the notes (`evidence/`), and LPA's client log, launch file, calls and answers (`run-pa/`, `company_number` redacted). The `01` answer holds the product's own tool descriptions and stays on the lab PC; its grading line is quoted above. Screenshots (including the About, renewal and License screens), BK1 and its listing, Bridge's data folder, the server error file, the box self-test's outputs, the whole port and process listings, the `/status` captures and the lab's own `local/` files stay on the lab PC, listed by size and sha256 in `kept-local.txt`. The sheet's own files are on the sheet branch and are not copied again.
