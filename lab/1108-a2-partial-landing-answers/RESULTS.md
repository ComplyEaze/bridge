# 1108-A2 partial landing: results (two sittings)

**First line: a stop at P1 in sitting 1; a pause at P1 in sitting 2; then P2 and P3 ran with no pause. Sitting 1 stop: "a stop at P1: the lab's box helper passed a malformed path and answered nothing; the box timed out unanswered; nothing reached Tally". Sitting 2's pause was over two differences in E-P1. The run lead decided both to be results, not faults of the run.**

Sheet: `lab/1108-a2-partial-landing` at 02b20332d48fe324ef2dcfe8ed80d635abc95d6a, queue item 84.
Book: BRIDGE STOCK CREATE LAB (integrated, inventory on), TallyPrime single-user edition, release 7.2, tally.exe 1.1.7.2.
Bridge: BUILD-M, bridge_mcp.exe sha256 9bda7144f54e5ef68477b5ca30be33a17759de2e1ad756f840785c67bcab2dc0, serverInfo bridge-tally 0.7.0.
Both sittings ended with BK-A2 restored on the run lead's word, s9 reads byte-equal to the BASELINE, and C10.

## Sitting 1 (folder `sitting-1-stopped/`)

Steps 0 to 5 met every EXPECT.

P1's box went unanswered. The lab's post helper `p1108.py` had passed box.ps1 a row folder whose backslashes the shell heredoc had cut down to single ones. The path arrived as `\b` and `\110` escapes, so box.ps1 had no valid folder: it logged nothing, saw nothing and clicked nothing. `box/p1-p1-ps.out` holds its errors, and `boxcheck.py` failed closed with No.

post_import was answered pending twice and then `import_approval_timed_out`, with `attempt_recorded` false and no dispatch. A box left unanswered to its timeout is a decline (Difference 5), so this was a stop. Nothing reached Tally: the proxy folder holds `proxy.jsonl` only and no import file.

C8 reads after the stop: V 1, M 213, byte-equal to m5. Then, on the run lead's RESTORE, BK-A2 was restored (24,219 B, sha256 03f05ab9df3df726ae609806b5c268c2488686bdd439d07b6adc800e2b925b26). s9 marks, ledgers and vouchers were byte-equal to the BASELINE (M 210, V blank). C10 followed.

## Sitting 2 (folder `sitting-2/`)

The run lead's rules for the rerun:
1. The fixed helper is a new file with its own sha: `p1108s2.py`, sha256 59b4554f0ef51be0c0856a88805293d60bf91c8466d2d4f72b428ef982f1f7a5. It was written whole by an editor, with no heredoc, and it computes the row folder with `os.path.normpath`.
2. box.ps1 was self-tested before step 0 with the helper's own argument list. It logged its watch start line, saw no box, and exited 3 with no console output.
3. A new bridge_mcp ran on a new empty data folder, so no decline from sitting 1 carried over.

Steps 0 to 5 met every EXPECT. Every read and ledger answer of steps 0 to 4 is byte-equal to sitting 1's.

| Post | Batch | Box seen to Yes | post_import | verify_import |
|---|---|---|---|---|
| P1 | bridge-9cd47867-ffa3-40be-80cd-a74fab8fdf49 | 1.1 s, check ALL PASS | one call, 13.686 s | once |
| P2 | bridge-2573f0c2-65df-4729-9244-ae59c6b01b86 | 0.6 s, check ALL PASS | one call, 11.604 s | once |
| P3 | bridge-14bc5272-847e-45f3-b2db-c5b418d939b0 | 0.6 s, check ALL PASS | one call, 12.007 s | once, 3.161 s |

Each box showed exactly one Create line, matched whole; the Company GUID and Batch lines whole; and totals as the build writes them (10, 66, 94). After each post, the capture of Tally showed the Gateway with no dialog.

### The EXPECT table, filled in

| Id | Result | What the row shows |
|---|---|---|
| E-P1 | **refuted, on `attempt_recorded` alone** | Counter as written: CREATED 0, EXCEPTIONS 1, every other 0, one LINEERROR "Cannot use 'A1 Closing Stock': Ledger under Stock-in-Hand!". Mark as written: V stayed 1 (V0), M stayed 213, no voucher ADDED. Status in the post as written: `b1108-p1-1` `tally_reported_not_created` with its next step; the batch `reconciliation_required`; `error.code` `import_reconciliation_required`; `response_state` `response_not_clean`. But neither the post nor verify_import carries an `attempt_recorded` field (nor a `created` field). The sheet named that field from reading the code; the fields each answer does carry are in `sitting-2/evidence/p1-answer-fields.txt`. |
| E-NB | confirmed | Neither P2 nor P3 was refused before the box: no `post_approval_busy`, `import_txn_already_posted` or `invoice_company_stopped`. Each box was seen and answered Yes. |
| E-P2 | confirmed | CREATED 2, EXCEPTIONS 1, every other 0, LASTVCHID 3. V = V0 + 2 = 3, M 213. Two vouchers ADDED: 21.00 ('Row 1108-P2 first') and 23.00 ('Row 1108-P2 third'). `post_span_binding` refused with `span_counters_not_clean`; the batch `verification_incomplete`, dispatch `reconciliation_required`. |
| **E-PL** | **confirmed** | `b1108-p2-1` and `b1108-p2-3` read `matching_content_observed` with attribution `not_established`, in the post and in verify_import (there with marker `accounting_fingerprint`, `accounting_effective` true, diffs []). **Neither reads `posted_verified`.** verified_total 0. |
| E-P2m | confirmed | `b1108-p2-2` reads `sent_not_attributed`, with the neutral next step "This voucher was sent to Tally, but ComplyEaze Bridge cannot match it in the book now. Check in Tally before posting it again.", in the post and in verify_import. |
| E-P3 | confirmed | CREATED 2, EXCEPTIONS 1, every other 0, LASTVCHID 5. V up by 2 from P2's: from 3 to 5. M 213. Two vouchers ADDED, both 31.00 on 16-Apr-2025 ('Row 1108-P3 twin'). The batch `reconciliation_required`. |
| E-T | confirmed | `b1108-p3-1` and `b1108-p3-3` each read `duplicate_fingerprint`, matches 2, `ambiguous_within_batch` true. The batch's `duplicates` holds one `accounting_fingerprint` set, fingerprint 43344f64aa75b7d7c124e4d258eeebe4aaa8df82d561a9922166aa0edebda519, naming the two new vouchers. `b1108-p3-2` reads `sent_not_attributed`. The post and verify_import agree. |

For each post, the proxy's answer body (`proxy/import-N.answer`, body length equal to CONTENT-LENGTH) gave the same counters as the post's `dispatch.response.outcome`.

### P1 difference (1): by design

post_import reads `b1108-p1-1` as `tally_reported_not_created`. A later verify_import on the same batch reads it as `sent_not_attributed`, under `unverified_vouchers`.

The run lead decided this by design on master, not a finding. A later verify_import uses `later_unmatched_cause`, which gives `ReportedNotCreatedWhenSent`. That cause keeps the voucher `sent_not_attributed`, since it may have been keyed by hand since.

Asked: does the voucher's next step in the kept verify_import answer begin "When ComplyEaze Bridge sent this voucher, Tally reported it as not created"? **Yes.** The text whole, from the voucher's own row under `unverified_vouchers`:

> When ComplyEaze Bridge sent this voucher, Tally reported it as not created, and ComplyEaze Bridge does not find it in the book for these dates now. It may since have been entered in Tally with a different date, voucher type, ledger or amount: look for it there. ComplyEaze Bridge will not send this saved voucher again; do not import its file through Tally's Import menu. Once the user says each voucher of the batch is in the book or will not be posted, call acknowledge_post_review with doubt not_created: it asks them in a dialog you cannot answer, and closes the batch.

The verify answers' `items` list is [] for P1, P2 and P3; the per-voucher rows are under `unverified_vouchers`. The notes first called this a FINDING; a later notes line supersedes that.

### Steps 7 to 10

- **Step 7:** egress_log was called once and answered: 21 records shown in reverse time order, 3 left out for size. The client then closed; bridge_mcp exited with code 0.
- **Step 8:** the final reads against the BASELINE.
  - Ledgers: 3 ADDED (A1 Closing Stock, A1 Capital, A2 Expense), 2 unchanged.
  - Vouchers: 5 ADDED (S0 200.00, 21.00, 23.00, and two at 31.00).
  - Marks: V = V0 + 4 = 5, M 213.
  - EXPECT met.
- **Step 9:** done on the run lead's RESTORE. BK-A2 of this sitting was restored (24,219 B, sha256 8365e521e52c2f48d26cbc43a4f181a88686a91affaf3d236fca6ea8d45770f3). s9 marks, ledgers and vouchers were byte-equal to the BASELINE, and the Gateway capture was byte-equal to step 0's.
- **Step 10:** C10. The proxy was stopped and its folder kept whole.

Every read was answered.

## Answer files (sitting 2), whole, as kept on the lab PC

| File | Bytes | sha256 |
|---|---|---|
| 004-p1-p1.json (P1 post) | 13,285 | f03ecf72afd40e30737b4e723196dd4aff9cc2cf09af4a0a5e20eb3716514fef |
| 005-p1-verify.json | 13,298 | ecb81d6b861c9af6ddcec4c54d8f3596d3e88a90a08f44dd0b624b49a858b2e1 |
| 007-p2-p1.json (P2 post) | 14,781 | 425e2e3642e42a1806ed05bdaa7e6d4a4ee9165fa9a3d5d5edeea2cd5dcfa9d3 |
| 008-p2-verify.json | 14,263 | 547066aa6284e3833967e22d8c34b09445ae149b45673d9beb3c30417538ec92 |
| 010-p3-p1.json (P3 post) | 14,567 | a5cedaa926480eae6438a388bf9b0bdffc3ae6af54ba6f733e1df590208b76b9 |
| 011-p3-verify.json | 14,048 | 8c41e120977b51fc2201f41b66e52ae1421b47194521ec1431c45ff9bea09e0b |
| 012-egress.json | 199,775 | 7b7d6e395e3d4ea950f916b6cf69f458aeeb2b74e8789eaf415da9f919abb64d |

On this branch, each of these files has THE COMPANY's GUID as `<company guid>` and its number as `<company number>`. Every other byte is as captured. `substitutions.txt` names each change and the original's size and sha256. Files that still hold a value that stays local after those two changes are in `kept-local.txt`, by size and sha256.

## Deviations, each recorded in the notes

1. **Difference 3's box rule.** It was corrected by the run lead before the first write. In the run lead's words: "one Create line matched whole, the GUID and Batch lines whole, totals as the build writes them (10, 66, 94); any other spelling, .00 included, is a Decline and a stop". `boxcheck.py` carries exactly that.
2. **gen_requests.py line ends.** In sitting 1, gen_requests.py wrote the batch files with CRLF line ends on this PC, so the census found 3 BAD. The JSON of each was equal to the sheet's. The sheet's own files were put back and the census read 22 of 22. The CRLF copies are kept on the lab PC.
3. **Python command.** `python` (3.10) was run for the sheet's `python3`.
4. **Menu clicks.** Tally's menus were opened by mouse clicks. Keys were pressed only for load, backup and restore, and for the one-row Shut list.
5. **Start-up company.** At sitting 1's step 0, a start-up company loaded by itself and was shut first (Company, Shut, Enter).
6. **Sitting 2 folders.** Sitting 2 ran in its own new folder, with its own new backup and proxy folders. `setup2.py` made it from sitting 1's: the helpers' folder name was changed and nothing else.
7. **`body.py`.** The proxy keeps each import's whole HTTP answer, headers included. `body.py` writes the body alone, after checking it against CONTENT-LENGTH, for `check_1108.py import`. The proxy's own files are unchanged.
8. **Stopping the proxy.** The proxy has no stop of its own. It was stopped by stopping the background task the lab started it in.
9. **tally_status argument.** In sitting 1, tally_status was first called with `company_guid` and refused with `argument_unknown`; it was then called with no argument. Sitting 2 called it with no argument.
10. **P1 differences.** P1's two differences in sitting 2 are a pause, not a stop, on the run lead's word.
