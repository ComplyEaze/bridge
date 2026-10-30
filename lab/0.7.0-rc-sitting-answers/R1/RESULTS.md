# R1: the live write pass (W1) on the 0.7.0 release candidate: PASS

Build: candidate A, RC COMMIT 026ba163dfcbf75d515a24d0dbbf14c5245534ec, EXE SHA256 576cc0c5aab4e72c43c05799116d76a29429c595a86dd54834d841c551f50d7c (initialize: bridge-tally 0.6.1, as expected). One launch over stdio, 06:18:24Z to 06:30:24Z on 30 Oct 2026, posting on for the row and removed at its end. Company: BRIDGE TIE OUT LAB 3 (GUID 9c97970c-e25d-4348-9740-6ddb1caaa56d), synthetic. TallyPrime 7.2 Silver; Tally's current date 10-Oct-2026. Folder: `<LAB_DIR>\`.

| check | result | record |
|---|---|---|
| 1967 (post_import text) | PASS | `answers/000`: both new phrases once each, both old phrases absent |
| 1995 (held_elsewhere step) | PASS | `answers/012`: begins and ends as the sheet says; both phrases present once; "sending it gives" absent |
| step 5 (A refused before the box) | PASS | `answers/014`: `skip_row_paired_elsewhere`, skipped_rows = R, attempt_recorded false; no box in 20 s; V, the journal and the skips file unchanged; the receipt has no `request_trail` |
| the post (B) | PASS | `answers/020`: `posted_verified`, `not_reported`, created 1, `response_clean`, `target_only` 8 to 9 (step 1, matches_created), `bound`; `answers/021` verify `posted_verified` from `pre_post_mark` 8; V 9; one Receipt of 400.00 on 30 Sep; `request_trail.sent` 140 |
| the box's date line (rule 6) | PASS | `Date: 30-Sep-2026`, `Books from: 1-Apr-2026` (`box/s6-postB-box.txt`) |
| 1896 no false line | PASS | `answers/021`: `post_span_binding.summary` exactly "Each voucher was matched to the Tally voucher this post created." |
| rollback record | PASS | `answers/028`: `book_rolled_back`, current_mark 8, expected_at_least 9, rollback_recorded true, other_batches_covered {"count": 0, "first": []}, counts book_rolled_back 1 / not_found 0, `verification_incomplete` |
| rollback cover | PASS | `answers/029`: `book_rolled_back`, mark_read 8, seen_by B, no `rollback_recorded`, `verification_incomplete`, posted_verified 0 |

The box (step 6): "ComplyEaze Bridge — post this voucher?", seen 7.3 s after the call was sent. It was topmost (extended style 0x10108), above Tally in z-order, and held the foreground. Every line named step 6's company, ledgers, date, amount and batch. The lab answered Yes by mouse 10.7 s after it was seen, on the standing word of 28 Oct. The first call answered `posted_verified` in 22.9 s, so no repeat call was needed.

The book afterwards: restored from BK1 on the run lead's RESTORE word. Marks 210 / 8, Union Bank as at step 0, no BRIDGE Bank B. The PASS lines for 1896, the rollback record and the rollback cover are regression checks: they would pass on 0.6.1 too, and none is a PASS of #2047 or #1896 (the sheet's NOT COVERED).

Notes, one line per step with UTC times: `notes.txt`. Every answer whole: `answers/`. The box's record: `box/`. Captures, BK1 and the data folder stay on the lab PC, listed in `SCREENSHOTS.txt` and `kept-local.txt`.
