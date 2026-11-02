# Results: #1713 Row S, a lost create answer followed by a delete, 2 Nov 2026

**Every step's EXPECT holds. A create whose answer was withheld was recorded as `reconciliation_required`. After the ledger was deleted on Tally's screen, the same batch settled `confirmed_absent_after_change`, reasons `not_in_book` and `mark_moved_since_intent`, basis `read_after_send_order`, sending nothing. `pre_image_master_changed` did not appear. A new batch for the same name opened the box, and was declined. 0.7.0 refuses the journal with `master_ledger_invalid`.**

Sheet `lab/1713-row-s` at 13ebb499, 2 of 2 files equal. Build master 3e87d00e, run 41474115716, artifact 13156753754, zip digest equal, `bridge_mcp.exe` sha256 15c423e05b8d057d25370ea169809a5e79dc530fb677929d34bc42d5cf96987b, before and after. Old build: mcp-v0.7.0 windows-x64 .mcpb, sha256 291a4ed95c758fad9fa04cd21a66be889e90a2fd03960fbceeec2c2a7242d4ec, `bridge_mcp.exe` sha256 48d2f7430b26da0d8f670eb74a27df46b99b7d45e05e102504482ddf24c8ba2d, before and after. BRIDGE ABANDON LAB alone, TallyPrime 7.2 Silver. Posting settings on, as the note names them, port 9002. A backup was taken by Tally's screen first; sha256 in `kept-local.txt`.

| step | what was done | outcome | M after | `Lab Order S` |
|---|---|---|---|---|
| 1 | relay on 9002; reads | V 54600 | 229 | absent |
| 2 | prepare, post, Yes by mouse | isError true, `import_dispatch_outcome_unknown`, cause `request_deadline_exceeded`, `attempt_recorded` true, dispatch `reconciliation_required`, resent false; `relay/create.answer` CREATED 1, held 19.66 s; 32.15 s round trip, box wait included | 230 | present, ALTERID 230 |
| 3 | no post | - | 230 | present |
| 4 | delete on Tally's screen | 24 ledger rows, as at step 1 | 231 | absent |
| 5 | post, same batch_id | `confirmed_absent_after_change`, no box, `resent` false, `settled` true | 231 | absent |
| 6 | new prepare, post, No by mouse | box opened; `master_approval_declined`, `attempt_recorded` false, `not_dispatched` | 231 | absent |
| 7 | old build, copy of the journal, step 2's batch_id | `master_ledger_invalid`, 0.08 s, no box | 231 | absent |

[verified]

## Step 5's checks
`pre_image_unchanged` passed, `other_marks_unmoved` passed, `target_mark_moved` passed, `name` failed, the rest not checked. `answer_recorded` false, and `dispatch.state` reads `dispatched`. The answer says it cannot tell whether the create ran, though the row knows it did. [verified]

The note left open whether a delete moves another master's ALTERID. Here, for ledgers, it did not: the 24 ledgers in the raw reads at steps 1 and 4 carry the same ALTERIDs, and the mark moved only by the create (230) and the delete (231). Groups are covered only by the step 5 check `pre_image_unchanged`; their ALTERIDs were not read here. That is one delete of a ledger with no vouchers. [verified]

## Limits
- The relay made the lost answer; the row says what follows one and nothing about how one gets lost, as the note says.
- Step 2's kind is `suspense`: the note names no kind, and the book's other `Lab Order` ledgers sit under Suspense A/c.
- My first prepare carried the wrong argument name and was refused `argument_unknown`, nothing sent. The answer is kept.
- Step 6 ends at the box, so the under-lock check is not run, as the note says.
- The relay logged 1 held answer across all steps. The second batch reports `not_dispatched`, and M held at 231.

Left behind: the delete moved the book to M 231 with its ledger rows as before; no restore. Company shut, relay ended.

**Confidence:** one book, one Tally, one sitting, a ledger with no vouchers: PARTIAL.
