# R2: the masters row, run CUT, on the 0.7.0 release candidate: PASS

Build: candidate A, RC COMMIT 026ba163dfcbf75d515a24d0dbbf14c5245534ec, EXE SHA256 576cc0c5aab4e72c43c05799116d76a29429c595a86dd54834d841c551f50d7c (bridge-tally 0.6.1; `tools/list` names `prepare_ledger_creates` and `post_ledger_creates`). Company: BRIDGE LEDGERBOX CUT LAB, new, created on Tally's screen with books from 1 April 2025, GUID shown as `<company guid>` (README rule 8). TallyPrime 7.2 Silver; display 1920x1080 at 125 % (work area 1920x1030). 30 Oct 2026, 06:34 to 06:45 UTC. Folder `<LAB_DIR>\`.

| step | result | record |
|---|---|---|
| the decline (step 5) | PASS | `answers/009`: `master_approval_declined`, attempt_recorded false, not_dispatched; M 207 = M0′ |
| the box (steps 5 and 6) | whole on the screen | rect 688,342 to 1252,764 on 1920x1080; topmost (0x10108), above Tally, holding the keyboard when it opened (#2171); its text the answer line, a blank line, then step 3's nine preview lines unchanged, with `Books from: 1-Apr-2025` (rule 6) |
| the create and its verdict (step 6) | PASS | `answers/011`: `posted_verified`, attempt_recorded true, `dispatched`, created 1, every other counter 0; `target_mark_step_one` passed |
| the marks and rows (step 7) | PASS | M1 208 = M0′ + 1; one new row `LBox Suspense` under Suspense A/c; GUID not shown in the basic rows |
| the journal answer (step 7b) | PASS | `answers/013`: no box; `posted_verified`, `resent` false, `journal.response_recorded` true, `journal.counters.created` 1, `journal.verdict` `posted_verified`; M 208 |

Lab error, no product finding: the first step 5 call (`answers/007`) passed `company_guid` beside `batch_id`. The tool's schema takes `batch_id` only, so it was refused as `argument_unknown` in 0.07 s, before any approval step: no box opened, nothing was sent, M was unchanged. On the run lead's word, step 5 ran again from the same prepared batch with `batch_id` alone.

Both boxes were answered by mouse, No and then Yes, by the lab on the standing word of 28 Oct. THE COMPANY is kept, shut, with its one created ledger. Captures and BK1 stay on the lab PC, listed in `SCREENSHOTS.txt` and `kept-local.txt`.
