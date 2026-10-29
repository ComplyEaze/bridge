# Results: #1738, an abandoned build's file imported by hand, then rebuilt, 29 Oct 2026, master 218eaba1, BRIDGE TRANSFER LAB (synthetic). After the hand import, `verify_import` calls X1 **`posted_verified`** (by its narration tag), `local_data_report` counts it **`sent_or_found_posted`**, and statement B's two rows **pair** with it (`skips_paired` 2) with no step sentence. The rebuild X2 is **refused** (`import_txn_already_posted`, blocking X1), so no duplicate could be booked. The book was restored from S0 and holds what it held before

The sheet is `lab/1738-hand-import-rebuild` at 9be65c906d00ac70f7b095b291a1edad20a736f5 (amended after attempt 1). `SHA256.txt` matched 3 of 3, and the inputs matched by sha256.

**Who acted.** The lab did each write on the owner's standing word of 27 Oct (synthetic company):
- the S0 backup;
- the TLab Suspense ledger;
- the hand import;
- the restore.

Posting was on for launch 2 only, on the owner's word of 28 Oct.

**Deviations:**
1. **Attempt 1 (sheet 56b1884e) stopped at step 0.** The book had no `TLab Suspense`, which that head required. Nothing was written. The sheet was amended, and this run is from its step 0. Attempt 1's calls and notes are in `attempt-1/`.
2. **Step 1's first parse was refused (`statement_pdf_engine_unavailable`).** The lab's build folder held only `bridge_mcp.exe`, without the bundle's `pdfium.dll`: a lab setup fault, with nothing parsed and nothing sent.
   - On the coordinating word, the whole bundle was extracted. `pdfium.dll` is 7,211,520 B, sha256 79d4676b…, equal to the bundle and to `mcpb-smoke.json`.
   - Launch 1 was started again on the same data folder, and p1 was redone.
   - At that point the book had been changed only by step 0b.
3. **The hand import's result box shows no counters.** It reads only "Import completed successfully. Press any key to continue". The count of two created is taken from the reads instead: V went from 7 to 9, and exactly two Contras appeared.
4. **Launch 2 lists 34 tools, not the sheet's 33.** Posting adds `post_import` and `acknowledge_post_review`.
5. **Path placeholders.** Pushed copies have lab-PC paths replaced (`substitutions.txt`). The folder listings keep only each file's size, time and name.

## Build and setup
- **Build:**
  - master 218eaba1b5df10dfc8953f5ae3785e4a608f041e;
  - Bridge CI run 40818364258, artifact 12874111950;
  - mcpb 92025bda…, `bridge_mcp.exe` 41b86dac5c964b5e0106f5b392f854f4f13bf7ca181940902c796dcd24eb67bd, `pdfium.dll` 79d4676b….

  The exe hash was the same at every launch and at the end.
- **Launches.** Two, sharing one new data folder:
  - launch 1 posting off, launch file e0e28183…;
  - launch 2 posting on, launch file b5333447…; back to e0e28183… after.
- **Company:** BRIDGE TRANSFER LAB, 100051, d6e4c24f-8c7a-4797-93ec-e44bdff60bc9, loaded alone.
- **S0:** `TDBK1800_100051.001`, 28,318 B, 11:53 IST, sha256 79bc512a7349fe580b76f3dc3eb5768813ca3f372cf89297aff600dc4151d5c5.

## Steps
| step | read | marks (M, V) |
|---|---|---|
| 0 | four ledgers, no `TLab Suspense`; Contra automatic, `auto_retain`; August empty; 32 tools, no `post_import` | 210, 7; after S0 210, 7 |
| 0b | `TLab Suspense` created under Suspense A/c, no opening balance (every other field as offered) | 211, 7 |
| 1 | p1: E1 `st-20260803-0c23821e3874220e` (5,000.00, ref) and E2 `st-20260804-b1d36f58182d117c` (2,000.00). x1: batch `bridge-0a5b76a3-8a5f-4585-a235-7a1fb1a26aed`, 2 Contras (Dr TLab Bank B, Cr TLab Bank A); file 1,809 B, sha256 87e90def10b6b4ac66e09cd0c512c08a329b4b65e82c336d533323d4150ec746 | |
| 2 | hand import of X1's file (Import > Transactions; XML; Backup before import No): "Import completed successfully". August: exactly two Contras (no. 6 on 3 Aug, MasterID 6, AlterID 8; no. 7 on 4 Aug, MasterID 7, AlterID 9) | before 211, 7; after 211, 9 |
| 3 | `verify_import` X1: **`posted_verified`**, 2 of 2 (marker `narration_tag`), AlterID delta 2. `local_data_report`: journal `sent_or_found_posted` 1, no transfers section. p2 and s1: `skips_recorded`, `skipped_rows` 2 | |
| 4 | 32 tools, `post_import` absent, no call; August the same two | |
| 5 | 34 tools, `post_import` listed (required `company_guid`, `batch_id`) | 211, 9 |
| 6 | p3: the same two ids. x2: **refused `import_txn_already_posted`**, `blocking_batch_id` X1, no file | |
| 7 | not run: the sheet sends a refused x2 to step 8 | |
| 8 | restore from S0 (Accept Yes, then Replace for 100051). Ledgers: the four, **`TLab Suspense` absent**. August: none. Contra as step 0 | 210, 7 = after S0 |
| C10 | no company loaded; one tally.exe on 9001; `/status` 200 | |

## The transfers section, side by side
| call | state | skips_paired | skips_open | open | next_steps |
|---|---|---|---|---|---|
| x1 (step 1) | computed | 0 | 0 | [] | {} |
| `verify_import` X1 (step 3) | computed | 0 | 0 | [] | {} |
| s1 (step 3, statement B) | computed | **2** | 0 | [] | {} |
| `local_data_report` (steps 3, 5) | no transfers section | | | | |

## The finding
1. **What `verify_import` says of a batch whose file was imported by hand.** `posted_verified` for both vouchers, attributed by the narration tag the file carries. It does not tell a hand import from a post (`dispatch_response` null).
2. **What the transfers section and its step sentences say.** Statement B's two rows are paired against the hand-imported Contras (`skips_paired` 2). None is open, and there is no sentence at all, so nothing sends the person to rebuild. `local_data_report` has no transfers section in this build.
3. **Whether the tool admits the rebuild.** No. X2 is refused before any file is written (`import_txn_already_posted`, naming X1). The approval box was never reached, so whether it would warn of vouchers already in the book is not measured.
4. **Whether posting the rebuild books a duplicate.** Not reachable on this build: the refusal comes first. The book held two Contras, not four.

**PASS (the sheet ran):**
- every step the sheet sends a refused x2 through was done, and every call answered and was kept;
- X1 was never sent by the tool;
- the hand import made exactly two vouchers (by the reads);
- the restore returned the marks, an empty August and the four ledgers.

**Not measured:**
- the approval box of X2, and step 7's post;
- a partial or edited hand import;
- a rejected send; `transfer_double_booked` clearing; a cancel or delete;
- more than two rows; TallyPrime 7.1; Gold;
- Tally's import counters (the result box shows none).

**Confidence:** one sitting, PARTIAL until repeated.

## Files
- `answers/`: every call of both launches, whole, with the `init` answers.
- `attempt-1/`: the stopped first attempt's calls, client log and notes.
- `client-log.txt`.
- `evidence/`:
  - step 0 and C10 checks; `s0-schemas.json`; `s5-post_import-schema.json`;
  - X1's file;
  - `import-journal.jsonl`;
  - the two folder listings of step 8.
- `restore-plan.txt`.
- `scripts/`: `q.py` and `step.py`.
- `substitutions.txt`, `notes.txt` (UTC), and `SCREENSHOTS.txt` (the captures by sha256; they stay on the lab PC).
