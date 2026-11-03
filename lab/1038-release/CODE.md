# Row 1038-R1: what master admits at each step (master 96b30ca8, the build's head)

Paths are under `src-tauri/src/`. Each line was read at 96b30ca8. Nothing here was built or run for this note.

## Setting up
- `BRIDGE_AGENT_ENABLE_WRITES=true` is enough for the whole row. It turns on import (agent.rs:596), and without it `post_import` and `acknowledge_post_review` refuse `import_posting_disabled` (agent.rs:3308). `BRIDGE_AGENT_ENABLE_BATCH_POST` must be absent, with any value (`setting_retired`, agent.rs:3312). `verify_import` needs no setting.
- Journal is a posted type: `LIVE_QUALIFIED_VOUCHER_TYPES` (agent_import.rs:392-397) and `PostScope::Vouchers` (agent_import_post.rs:36-43). A Journal has no bank leg (agent_import.rs:372). It needs two entries or more (agent_import.rs:4352) and must balance (:4414).
- No number: a numbered voucher is refused (`import_post_numbered_journal_unsupported`, agent_import_post.rs:3101). A numberless one needs the Journal type at Automatic with Retain Original Voucher No. (`admit_numbering_for_post`, called at agent_import_post.rs:1140; agent_import_numbering.rs:276-345). The master mark must not move between that read and the POST (agent_import_numbering.rs:439). So the type is set, if needed, before the backup, and nobody alters a master during a post.
- Dates: each from books-from to the host PC's own today, at the build and the post (`validate_dates`, agent_import.rs:4812-4820). So the batch is dated 2 Nov 2026, not March 2027 like the batches of `lab/1029-large-book-posts`.

## The first post
- The build checks rows already posted (agent_import.rs:1423). The post mints fresh REMOTEIDs (agent_import_post.rs:888). It reads the book first and refuses a row already there by content (`require_absent_verification_result`, agent_import_post.rs:926-932). So the amounts carry paise that no voucher of the book's formula has.
- The native post records the voucher mark before the POST: the `pre_post_voucher_mark` that everything below rests on.

## The restore, and the cover
- `verify_import` on a native post reads the rollback scan and the current mark (agent_import.rs:2337-2345). A batch not yet covered is `book_rolled_back` when `current < before + created` (the floor, agent_import.rs:2971-2980). That is the mark alone: the vouchers need not be read as missing, and the batch need not have been verified.
- When the mark is below the floor it writes a `rollback_seen` record (`rollback_would_extend`, agent_import.rs:2393; `record_rollback_seen`, :3131, under the lock with a generation check at :3142). A later read finds the batch covered and reports `book_rolled_back` with `mark_read` and `seen_by` (agent_import.rs:2367).
- Rows read `book_rolled_back` (agent_import.rs:2452). `verification_status` is `verification_incomplete`, since it needs all rows `posted_verified` (`readback_verdict`, agent_import_verification.rs:1206).

## The release
`release_rolled_back_rows`, agent_import_release.rs:141-242, refuses in this order:
1. `import_batch_not_found`, then `ack_batch_not_posted` (never sent), then `import_batch_company_mismatch`.
2. `ack_rolled_back_not_established` when there is no pre-POST mark.
3. `ack_release_type_not_admitted` unless every voucher is a Journal, Payment or Receipt (`type_admitted`, :46-50).
4. `ack_release_vouchers_not_listed` for more than ten vouchers or a line the box cannot show (:171; agent_import_post.rs:3817).
5. `ack_already_recorded` when a release stands.
6. A full verification (`verify_for_review`, :181), whose own refusals pass through. Then `ack_rolled_back_not_established` unless it shows `book_rolled_back` and a cover stands.
7. The dialog: `ack_review_too_large`, `ack_review_declined`, `ack_review_timed_out`, `ack_review_unavailable`.
8. Under the lock (:214): `ack_changed_while_reviewing` if the batch's records changed or the cover, company or endpoint is not the one shown (`row_release_after`, agent_import_ledger.rs:2041).

On Windows the dialog is a system message box titled "ComplyEaze Bridge — release this batch's rows to be built again?" (tally/approved_import.rs:3329) with Yes, No and Cancel. **No is the default**, and only Yes releases (:3353-3364). It runs in the child mode `--confirm-row-release` (lib.rs:118). The result is `{batch_id, state: "rows_released", verification_status, next_step}` (agent_import_release.rs:232). After it, `verify_import` adds `row_release` beside `book_rolled_back` (agent_import.rs:2644), and `local_data_report` counts `released_by_person` (agent_import_local_data.rs:691; agent_import_ledger.rs:989).

## The second build and post
- The rows-already-posted lookup leaves a released batch out (agent_import_ledger.rs:1410-1418), so the same rows build into a new batch. The skips' scan runs only for statement skips (agent_import_transfers.rs:793), so not here.
- The post's own checks (agent_import_post.rs:899-904, again under the lock) leave the released batch out the same way. Its read of the book finds nothing by content, because the restore removed the vouchers.
- The span bind wants the rows at ALTERID `before+1…` and MASTERIDs in sequence, with each GUID `<company GUID>-<MASTERID as 8 hex digits>` (agent_import_span_identity.rs:326-349). It also wants no GUID bound by another batch (:350). The GUIDs only a released batch bound are left out of that set (`guids_bound_elsewhere`, agent_import_ledger.rs:2378, used at agent_import.rs:3025 and :3083). **This is the claim the row tests:** after the restore, Tally gives the new vouchers the removed ones' MASTERIDs and GUIDs (seen once on 7.1, `lab/1038-import-after-restore-answers`), and the bind admits them.
- Nothing compares the new post's mark with the old batch's. The post writes a rollback-seen record only for batches not yet covered (agent_import_post.rs:1386; agent_import_ledger.rs:2359). The new batch's `posted_verified` is refused only if a cover covers it (agent_import.rs:2713), and the old record cannot, because the new intent comes after it (agent_import_ledger.rs:2098). The new floor is its own mark plus 3 (agent_import.rs:2971).

## After the second post
The old batch stays covered: `book_rolled_back` with the cover's `mark_read`, and `verification_incomplete` (agent_import.rs:2367). No new record is written, because the old floor is not above the current mark (agent_import_ledger.rs:2359). Its rows may now read as content matches of the new vouchers, but that is an inference, not something read. Nothing refuses or blocks on it.

## Where the GUIDs are read
- `verify_import` gives each `posted_verified` row its `guid`, `master_id`, `alter_id` and `voucher_number` (agent_import_verification.rs:724).
- The journal `agent-import-ledger.jsonl` in `BRIDGE_AGENT_DATA_DIR` holds each `post_span_verdict` line's `bindings[]` (`bridge_txn_id`, `guid`, `master_id`).
