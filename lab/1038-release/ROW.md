# Row 1038-R1: post, restore, release and post again three Journals through Bridge

Not for merge. Part of #1038. Lines below are at master 96b30ca8, the build's head. Confidence of everything here until the row runs: unverified.

## Question
After a backup taken before a native post is restored over the company, does Bridge record the rollback cover, admit a person's release of the batch's rows in its native dialog, and then build and post the same rows again to `posted_verified`? And do the reposted vouchers take the removed vouchers' GUIDs, MASTERIDs and ALTERIDs?

## Why
#1038 stays open until this sequence passes on a real Tally. The repost's span bind admits a GUID that only a released batch had bound. That rests on one 7.1 run without Bridge (`lab/1038-import-after-restore-answers`), where a voucher imported after a restore took the removed voucher's GUID, MASTERID, ALTERID and number. This row runs the whole flow through Bridge, on 7.2.

## Book
`BRIDGE LARGE LAB`, GUID begins `2ba6df28` (whole on the lab PC, in `list_companies` and in `lab/1029-large-book-posts`), synthetic, books from 1-Apr-2026, on the lab PC. Row 852-C1 left it at V 30,030 and M 211 on 3 Nov 2026, with ledgers `Lab Vol Expense` (Indirect Expenses) and `Lab Vol Income` (Indirect Incomes). Its posts through this build line reached `posted_verified` on 7.2 there.

Not REGISTER LAB, and nothing seeded from client data. Not on the PC, or another GUID: stop, the row has no book. Last opened on 7.2. Any prompt when it opens: press no key, screenshot, stop and report its words.

The row ends by restoring BK1, so the book is left as it was found.

## Build
The Windows bundle of the merge-queue run of #2317: CI run 41629635266, `merge_group`, head 96b30ca8 (`96b30ca890ddede64d70b0c1138cff4c84664ee8`), success. That is the commit now on master; the master push run skipped the bundle. Artifact `unsigned-bundle-smoke-windows-latest`, id 13228397154, digest `sha256:f0cf3b888dcaeba414ea163ef76a77726ef89f6fee984452045374e2006d7ca5`. Check the zip's digest first: another digest is a stop. Record `bridge_mcp.exe` sha256 before and after.

Over stdio, straight to Tally (no relay): `BRIDGE_AGENT_ENABLE_WRITES=true`, `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, one new empty `BRIDGE_AGENT_DATA_DIR` for the whole row, `BRIDGE_AGENT_ENABLE_BATCH_POST` absent, `BRIDGE_AGENT_REDACTION` unset. One `bridge_mcp` process for the whole row: the journal carries from each call to the next, and the release is bound to this endpoint. Record the client and its call limit. A call over 60 s is recorded, not a stop.

## Kit
| file | what |
|---|---|
| `CODE.md` | what master admits at each step, file:line at 96b30ca8 |
| `batches/r1.json` | three numberless Journals dated 2 Nov 2026: `Lab Vol Expense` Dr, `Lab Vol Income` Cr, 1038.17, 1038.29 and 1038.41, ids `r1038-001` to `-003`. The same file goes to both builds, unchanged. |
| `marks.utf16le.xml` | the company's GUID, ALTVCHID and ALTMSTID, UTF-16LE with a BOM. Send it with `Content-Type: text/xml; charset=utf-16`. |

`SHA256.txt` lists all of it. Check it first.

## Steps
One at a time. Every answer goes to a file, whole. A file with no answer in it is a failed step, not an empty one. "Marks" means `marks.utf16le.xml` sent raw to 9001; write down V (ALTVCHID) and M (ALTMSTID).

0. Load the book alone. Screenshot. `GET /status` on 9001. Marks: EXPECT one company, this GUID; call its V **V0** (30,030 expected; another V is recorded, not a stop). Record Tally's release from `tally_status`, Tally's date and the PC's date. The PC's date must be 2 Nov 2026 or later: if it is earlier, stop.
1. `list_companies`: EXPECT this GUID. `masters` with `kind=voucher_types`: record the Journal row's `series_numbering.method` and `sub_method`. If they read automatic and auto_retain, leave them. Otherwise, in Tally, alter the Journal type and set only two fields: the method of voucher numbering `Automatic`, and "Retain Original Voucher No." `Yes`. Screenshot before and after, accept with Ctrl+A, and leave with Ctrl+Q, never Escape. A field that is not on the screen is a stop. Read `masters` again: EXPECT automatic and auto_retain. Marks: EXPECT V0; record M.
2. **BK1:** Tally's Backup screen, a full backup of the company into a new empty local folder. Record its name, size and sha256. Marks: EXPECT V0.
3. **Build a.** `build_import_xml` with `{"company_guid": <GUID>, "vouchers": <r1.json>}`. Keep the whole answer and record batch id **A**. EXPECT `voucher_count` 3 and no refusal.
4. **Post a.** `post_import` with `company_guid` and A. When the approval box shows, check that it names this company, A and 3 vouchers, then click Yes with the mouse. `approval.state` `pending`: call again after `retry_after_s`; `approved`: call again at once. Never call again after an attempt is recorded. EXPECT `dispatch.state` `posted_verified`, `counters.created` 3. Marks: EXPECT V0+3.
5. `verify_import` with A: EXPECT `posted_verified` 3 of 3. Record each row's `bridge_txn_id`, `guid`, `master_id`, `alter_id` and `voucher_number`: **table G-a**. EXPECT the ALTERIDs V0+1 to V0+3 and each GUID `<GUID>-<master_id as 8 hex digits>`. Screenshot Tally's Day Book for 2 Nov 2026, showing the three narrations `r1038 release 1` to `3`.
6. **Restore BK1.** Shut the company with Tally's own Shut Company. On Tally's Restore screen, restore BK1 into Tally's data folder. Answer only these two prompts: "Accept ?" with Yes, and "Company data folder (…) already exists … Replace" with Replace. Any other prompt: press no key, screenshot, stop. Load the company. Screenshot. Marks: EXPECT this GUID and **V0**. Another V: stop.
7. **Cover.** `verify_import` with A. EXPECT:
   - `post_span_binding.state` `book_rolled_back`, with `current_mark` V0 and `expected_at_least` V0+3;
   - the three rows `book_rolled_back`;
   - `verification_status` `verification_incomplete`.

   Call `verify_import` with A once more: EXPECT `book_rolled_back` with `mark_read` V0, and a `seen_by` naming a batch. Copy the journal file `agent-import-ledger.jsonl` from the data folder: EXPECT exactly one `"record_type":"rollback_seen"` line. Count lines by their parsed `record_type`, never with a substring search.
8. **The person's check.** In Tally's Day Book for 2 Nov 2026, look for any of the three: the narrations `r1038 release 1` to `3`, and any Journal of `Lab Vol Expense` against `Lab Vol Income` for 1038.17, 1038.29 or 1038.41. Screenshot. If any is there: stop, and release nothing.
9. **Release.** `acknowledge_post_review` with `{"company_guid": <GUID>, "batch_id": A, "doubt": "rolled_back"}`. A system box opens, titled "ComplyEaze Bridge — release this batch's rows to be built again?", with Yes, No and Cancel, and **No pre-selected**. Screenshot it. Check its text before you answer. It must:
   - say "THIS RELEASES THIS BATCH'S ROWS, SO THAT THEY CAN BE BUILT AND POSTED AGAIN.";
   - name this company and batch A, and say "3 vouchers";
   - read the mark at V0, and say "It may read higher now.";
   - list the three vouchers (2 Nov 2026, Journal, the three amounts).

   If it does, click **Yes with the mouse**: never Enter, which would answer No. If anything differs, click No, screenshot, stop. EXPECT the result `state` `rows_released`.
10. `verify_import` with A: EXPECT `book_rolled_back` as at step 7, plus `row_release` with `state` `rows_released`. `local_data_report`: record the journal counts; EXPECT `released_by_person` 1. Marks: EXPECT V0.
11. **Build b.** `build_import_xml` with the same `r1.json`, unchanged. EXPECT a new batch id **B**, `voucher_count` 3, and no `import_txn_already_posted`.
12. **Post b.** `post_import` with B, as at step 4. EXPECT `posted_verified`, `counters.created` 3. Marks: EXPECT V0+3.
13. `verify_import` with B: EXPECT `posted_verified` 3 of 3. Record **table G-b** as at step 5. Day Book screenshot as at step 5.
14. `verify_import` with A once more: record it whole. This is an observation, with no EXPECT. `egress_log`, then `local_data_report`, whole. Copy `agent-import-ledger.jsonl` again: record each `post_span_verdict` line's `batch_id` and `bindings[]` (`bridge_txn_id`, `guid`, `master_id`).
15. **Way back.** Shut the company and restore BK1 as at step 6. Marks: EXPECT V0. Shut the company. Retire the data folder: keep it for the answers, and use it for nothing else.

No one keys in Tally except at steps 1, 6, 8, 9 and 15, and then only as those steps say.

## What to report
| step | result |
|---|---|
| V0, M; Tally's release; the dates | |
| Journal numbering, and any change made | |
| BK1: name, size, sha256 | |
| A: build, post (`dispatch`, created, call seconds), marks | |
| G-a | |
| restore: prompts, marks | |
| step 7: `post_span_binding` both reads, rows, status; `rollback_seen` lines | |
| Day Book check | |
| release: dialog text as read, the answer, the result | |
| step 10: `row_release`, `released_by_person`, marks | |
| B: build (no `import_txn_already_posted`), post, marks | |
| G-b, set against G-a row by row: GUID, MASTERID, ALTERID, number equal or not | |
| step 14: A's verification; the two `post_span_verdict` bindings | |
| way back: marks | |

Read every number from its file, not from the screen. Put any refusal, and any EXPECT not met, in the first line.

## Decides
| what the row shows | reading on #1038 |
|---|---|
| every EXPECT met, and G-b equals G-a in GUID, MASTERID and ALTERID | the release passes live on 7.2, and the repost reused the removed GUIDs: #1038's live proof is met on 7.2 |
| every EXPECT met, but G-b's GUIDs differ from G-a's | the release passes live on 7.2, and Tally did not reuse the GUIDs on this release: record it, since the code admits either |
| post b refused `span_identity_reused` | the bind still refuses a released batch's GUID: a defect in #2317. Keep everything, stop, report |
| post b reaches no `posted_verified`, its binding refused for position, MASTERID sequence or GUID shape | Tally numbered the reposted vouchers in a way the bind does not admit: #1038 stays open with this row |
| the release refused (any `ack_` code) | record the code and the step: the code note's claim for that step fails, so #1038 stays open |
| step 6 or 7 does not read V0 and `book_rolled_back` | the restore did not take the book back as expected: stop. This is not a reading on the release |

Nothing is sent again after a refusal. `verify_import` reads, and never posts.

## Stop rules
- Any refusal before an attempt is recorded.
- Any dialog on Tally's screen, other than the two restore prompts at steps 6 and 15.
- A release box whose text differs from step 9: answer No.
- V not as EXPECTed after a post or a restore.
- Any call with no answer in 10 minutes.
- Education mode.
- Any voucher of the batch found in the book at step 8.

## Limits
- TallyPrime 7.2 Silver, one user, one book, one run, three Journals. The 7.1 claim of the pull request is not covered here.
- Payment and Receipt are admitted by the same code but are not run here.
- A voucher keyed again with other figures, which the person is the only guard against, is not tested.
- The book's own vouchers have whole-rupee amounts by its build formula (inference), so the paise amounts are chosen to stay apart from them. A match would show as `import_preexisting_identity` at step 4 or 12.
- No one works in Tally during the posts. Activity alongside a post stays unmeasured.

## Keep
On `lab/1038-release-answers`, with `SHA256.txt`: every tool answer whole; every marks answer with its headers; both copies of `agent-import-ledger.jsonl`; G-a and G-b; BK1's name, size and sha256; the call times; the screens by name and hash. Screens stay local. A `RESULTS.md` holding the table above. Send the sha.
