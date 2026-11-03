# Results: row 1038-R1, post, restore, release and post again three Journals through Bridge, 3 Nov 2026

**Every EXPECT met, and G-b equals G-a in GUID, MASTERID, ALTERID and voucher number.** The release passes live on 7.2, and the repost reused the removed vouchers' GUIDs: by the sheet's Decides table, #1038's live proof is met on 7.2. No refusal. One thing to note first: each `verify_import` of batch A after the restore answered `isError` true with `error.code` `import_reconciliation_required`, beside the `book_rolled_back` and `verification_incomplete` the sheet expects. It is the read's own status for a batch that needs a person's action, not a refusal of a write. [verified]

Sheet: `lab/1038-release-sheet` at be689efe, folder `lab/1038-release/`. `SHA256.txt` hashes to 56f7d1c0ebfa…, and all 5 files it lists are equal. [verified]

Build: the merge-queue run of #2317, CI run 41629635266, `merge_group`, head 96b30ca890ddede64d70b0c1138cff4c84664ee8, success. Artifact `unsigned-bundle-smoke-windows-latest`, id 13228397154, zip sha256 f0cf3b888dcaeba414ea163ef76a77726ef89f6fee984452045374e2006d7ca5, equal to the sheet and the API digest. `bridge_mcp.exe` sha256 e35da79849b034bffb96fd4887e580cbef77531a4adcc77b14dcdafa806976ea, equal to the bundle's smoke file, before and after; `pdfium.dll` equal too. [verified]

Env, as the sheet: `BRIDGE_AGENT_ENABLE_WRITES=true`, `BRIDGE_TERMS_ACCEPTED=true`, host 127.0.0.1, port 9001 direct, one new empty data folder for the whole row, `BRIDGE_AGENT_ENABLE_BATCH_POST` absent, redaction unset (`run/launch-env.json`). One `bridge_mcp` process (pid 2808) for every call from `tools/list` to the last `local_data_report`. **The client** is the lab's stdio client: one call at a time, no cancel, no call limit (it waits with no timeout). Every call answered within 26 s. Four `bridge_mcp` processes ran idle under the desktop client, as in earlier rows. They took no call. Whether they sent anything to Tally was not measured. [verified; the processes unverified]

## What to report
| step | result |
|---|---|
| V0, M; Tally's release; the dates | **V0 30,030**, M 211, one COMPANY row, GUID 2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab. `tally_status`: TallyPrime 7.2, silver, `education_mode` false, `today` 20261103. Tally's current date at load: 31-Mar-2027. PC's date: 3-Nov-2026 (20:56 IST), on or after 2 Nov 2026. `GET /status` 200. |
| Journal numbering, and any change made | `masters` `voucher_types`: Journal `series_numbering.method` automatic, `sub_method` auto_retain. Left as it was; nothing altered in Tally. Marks: V0, M 211. |
| BK1: name, size, sha256 | `TDBK1800_<company number>.001`, 9,111,833 B, sha256 aecad8f11661e00858ad1de3ffa72df8038ae73ea31685aa54e220cf90fb6ca3, taken 20:58 local by Tally's Backup screen into a new empty local folder; kept on the lab PC. Marks after: V0. |
| A: build, post, marks | build: batch **A** `bridge-06a59be7-4767-43e3-9b30-5388dfeee4d4`, `voucher_count` 3, no refusal, 7.624 s. post: box "ComplyEaze Bridge — post 3 vouchers?" checked for company, GUID, A and "3 vouchers", Yes by mouse 1.4 s after it showed; `dispatch.state` `posted_verified`, `counters.created` 3, `response_state` `response_clean`, LASTVCHID 30033; call 25.599 s. Marks **V0+3** (30,033), M 211. |
| G-a | below |
| restore (step 6): prompts, marks | Shut Company; Tally's Restore screen from BK1's folder, list showing BRIDGE LARGE LAB at 3-Nov-26 8:58 pm. Prompts: "Accept ?" (Yes), "Company data folder (<company number>) already exists … Replace" (Replace); no other. Loaded; Gateway, no dialog. Marks: this GUID, **V0**, answer byte-identical to step 0's. |
| step 7: `post_span_binding` both reads, rows, status; `rollback_seen` lines | read 1: `book_rolled_back`, `current_mark` 30030, `expected_at_least` 30033, `rollback_recorded` true. read 2: `book_rolled_back`, `mark_read` 30030, `seen_by` `bridge-06a59be7-…` (batch A). Both: the three rows `book_rolled_back` (in `unverified_vouchers`), counts `book_rolled_back` 3, `verification_status` `verification_incomplete`, `error.code` `import_reconciliation_required`. Journal copy: exactly **1** `rollback_seen` line (counted by parsed `record_type`). |
| Day Book check | Day Book for 2-Nov-26, on three screens covering the day's vouchers 17631 to 17712: 82 of the book's own Journals, none for 1,038.17, 1,038.29 or 1,038.41, and no voucher 29931 to 29933. The narrations column is not in this view without an F12 change, which was not made. Nothing of the batch found. |
| release: dialog text as read, the answer, the result | below; Yes by mouse; `state` **`rows_released`**, `verification_status` `verification_incomplete`, no error; call 24.008 s. |
| step 10: `row_release`, `released_by_person`, marks | `verify_import` A: `book_rolled_back` (`mark_read` 30030, `seen_by` A) plus `row_release` `state` `rows_released`. `local_data_report`: `released_by_person` **1**. Marks: V0. |
| B: build, post, marks | build with the same `r1.json` (sha256 4962a343…, unchanged): batch **B** `bridge-fc1c283e-fbc6-4536-b310-8a9a01866309`, `voucher_count` 3, **no `import_txn_already_posted`**, 7.553 s. post: box checked as at step 4, Yes by mouse 1 s after it showed; `posted_verified`, `counters.created` 3, `response_clean`, LASTVCHID 30033; Tally's answer byte-identical to post a's (sha256 31aa884d…); call 22.841 s. Marks **V0+3** (30,033). |
| G-b, set against G-a row by row | below: every GUID, MASTERID, ALTERID and number equal |
| step 14: A's verification; the two `post_span_verdict` bindings | A: `book_rolled_back` (`mark_read` 30030, `seen_by` A), `row_release` `rows_released`, `verification_incomplete`, `error.code` `import_reconciliation_required`. Its three rows now read **`matching_content_observed`**, as the code note inferred. Journal: 2 `post_span_verdict` lines, A's and B's, with identical bindings (below). Still 1 `rollback_seen`; 1 `row_release`. |
| way back: marks | Shut Company; BK1 restored as at step 6, the same two prompts and no other. Marks: this GUID, **V0**, byte-identical to step 0's. Shut. The data folder is kept for the answers and used for nothing else. |

[verified]

## G-a (step 5, batch A, `posted_verified` 3 of 3) and G-b (step 13, batch B, `posted_verified` 3 of 3)
| row | GUID | MASTERID | ALTERID | number | G-b against G-a |
|---|---|---:|---:|---:|---|
| r1038-001 | 2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab-0000754f | 30031 | 30031 | 29931 | GUID, MASTERID, ALTERID, number all equal |
| r1038-002 | 2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab-00007550 | 30032 | 30032 | 29932 | all equal |
| r1038-003 | 2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab-00007551 | 30033 | 30033 | 29933 | all equal |

- In both tables the ALTERIDs are V0+1 to V0+3, and each GUID is `<company GUID>-<MASTERID as 8 hex digits>` (checked by `<GUID>-{master_id:08x}`). `evidence/G-a.txt`, `evidence/G-b.txt`. [verified]
- The journal's two `post_span_verdict` lines bind the same three: A and B each `r1038-001` → …-0000754f / 30031, `r1038-002` → …-00007550 / 30032, `r1038-003` → …-00007551 / 30033. [verified]
- The Day Book screen of the reposted vouchers (step 13) is byte-identical to the one of the first post (step 5): Journals 29931, 29932, 29933 for 1,038.17, 1,038.29, 1,038.41 at the day's end. [verified]

## The release dialog, as read (step 9)
Title: "ComplyEaze Bridge — release this batch's rows to be built again?", buttons Yes, No, Cancel. The text box.ps1 read from the dialog (`box/release-box.txt`) said, among the rest:
- "THIS RELEASES THIS BATCH'S ROWS, SO THAT THEY CAN BE BUILT AND POSTED AGAIN.";
- "Company: BRIDGE LARGE LAB", "Batch: bridge-06a59be7-4767-43e3-9b30-5388dfeee4d4", "… 3 vouchers.";
- "A verification read this company's voucher mark at 30030, below the mark this post left." and "It may read higher now.";
- three lines, each "Journal 2-Nov-2026", with 1038.17, 1038.29 and 1038.41, Dr "Lab Vol Expense", Cr "Lab Vol Income", and the narrations "r1038 release 1" to "3".

Every point of step 9 was in it. Yes was clicked with the mouse at the button's centre, 16 s after the box showed, once the text was read; no key was pressed. The dialog was captured (kept on the lab PC). [verified]

## Notes
- **Tally's current date:** at step 5, the Day Book's F2 date (2-11-2026) moved Tally's current date from 31-Mar-2027 to 2-Nov-2026. The step 6 reload set it back to 31-Mar-2027, and step 8's Day Book moved it to 2-Nov-2026 again; the step 15 reload set it back. Bridge's date check uses the PC's own date (`tally_status` `today` 20261103), so the posts were not affected. [verified]
- **Step 0:** the first `GET /status` answered 200 but curl could not write its file (the evidence folder was made a moment later). It was repeated at once: 200, `TallyPrime Server is Running`. [verified]
- **The book's own amounts** carry paise (the Day Book shows e.g. 442.31), not whole rupees as the sheet guessed. No voucher of the book matched the batch by content: post a and post b found none. [verified]

## Decides
Every EXPECT met, and G-b equals G-a in GUID, MASTERID and ALTERID: **the release passes live on 7.2, and the repost reused the removed GUIDs. #1038's live proof is met on 7.2.** [verified]

## Kept
- Every tool answer whole (`company_number` redacted, in the JSON and in its text).
- Every marks answer with its headers.
- Both copies of `agent-import-ledger.jsonl`, with the company number replaced in their text.
- G-a and G-b, BK1's name, size and sha256, the call times (`run/evidence/tools-log.txt`), the box logs, the notes.

On the lab PC only, listed in `kept-local.txt` by name, size and sha256: the screens, the box captures, the backup, the folder listings, and the originals of every file changed for the company number or a path.

## The book now
BRIDGE LARGE LAB restored from BK1: V0 30,030, M 211, marks answer byte-identical to step 0's, as found. Shut, no company loaded, one tally.exe on 9001.

**Confidence:** one book, three Journals, one run, 7.2 Silver: PARTIAL.
