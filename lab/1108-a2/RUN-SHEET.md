# Lab rows 1108-P1, P2 and P3: A1's voucher alone, then in a batch of three, then between twins, each through `post_import`. SYNTHETIC COMPANY ONLY. Not for merge.

Part of #1108. Written 5 Nov 2026 from master c33adaf8. About 75 minutes. Every EXPECT below is
from reading the code (confidence: unverified) until this row runs. A different status is a result,
and is recorded; only the STOP RULES stop the row.

`SHA256.txt` lists every file here with its bytes; check it before step 0.

## Why

Row 1108-A1 (`lab/1108-a1-stock-in-hand-journal-answers` at 9d8ae44e) found a voucher that
ComplyEaze Bridge admits and Tally refuses for that voucher alone. The voucher was a Journal with a
Stock-in-Hand ledger on an integrated book. Tally answered EXCEPTIONS 1 with nothing created, no
dialog, and V unchanged. This sheet sends that voucher through Bridge's own `post_import` three ways:

| Post | Batch | What it decides |
|---|---|---|
| P1 | the voucher alone | the first live `tally_reported_not_created` (#1116) |
| P2 | three Journals, with the voucher second | #1108's acceptance: a partial landing recorded through `post_import`; the answer, the marks and the readback become fixtures |
| P3 | three Journals, the first and third identical, with the voucher second | whether content matching can say which voucher Tally refused when twins share a fingerprint |

## Rules and company

`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy (sha256 5370c6d2) of A1's.

THE COMPANY is `BRIDGE STOCK CREATE LAB`, as A1's restore of BK-1108 left it: M 210 and no voucher
mark (V blank), the state that 1381-M's restore also left. If it is not on the PC, or its GUID
differs from the one in those rows' lab notes: STOP. The row has no book; never rebuild it.

Differences from LAB-COMMON (the stricter rule holds everywhere else):
1. **Raw writes**, by `curl` to this lab book only, as the owner allowed on 26 Oct 2026. They are
   W1, W2, W3 and the seed S0, each sent once. Each is a shape this book has already answered:
   W1 and W2 are A1's requests byte for byte, W3 is the same envelope with another name and parent,
   and S0 is A1-B's Journal byte for byte. C4's EXPECT applies to each: `CREATED 1`, every other
   counter 0.
2. **Bridge writes**: the three posts P1, P2 and P3, through `post_import` only, each from its own
   fresh `build_import_xml` made just before it. No voucher of P1 to P3 is ever sent raw. Tally's
   counters for a post are recorded and checked against the EXPECT table, and they never stop the
   row by themselves.
3. **Who answers the box.** The lab answers Bridge's approval box itself, by mouse on the box's own
   button, through the guarded box helper. That is the helper of 852-C1, with the one-voucher
   wording fixed as row 1392-E asked. The helper allows Yes only if all of these hold:
   - the box's first line is exactly `Create ONE Journal in "BRIDGE STOCK CREATE LAB"` for P1, or
     `Create 3 vouchers in "BRIDGE STOCK CREATE LAB"` for P2 and P3 (`agent_import_post.rs:3447`
     and `:4034`);
   - it has `Company GUID: <THE COMPANY's GUID>` and `Batch: <the batch id this build returned>`;
   - it has `Total debit: X  Total credit: X`, where X is 10.00 for P1, 66.00 for P2 and 94.00
     for P3.

   Per-voucher lines are not required. For three vouchers the box writes them when they fit, and
   otherwise one sentence in their place (`:3703`, `:3733`), and the helper accepts either. A box
   that fails any of these: Decline, record its text whole, and STOP.
4. **Keys and processes.**
   - No key is pressed in Tally in this sheet.
   - If the coordinator rules that one must be (C7), run `front.ps1` first so Tally is in front.
     Never press Escape on a master, voucher or configuration screen.
   - No process is ended. At the end, the client closes `bridge_mcp`'s input and the server exits
     by itself. Tally and the relay are left running until C10.
5. **A decline is kept once.** A No, a timeout or an unanswered box is kept and "returned once to
   its batch's next call, which does not ask the person again" (`agent_import_approval.rs:14`;
   `begin` `:648`, `take_refusal` `:556`). After any decline, then:
   - the next `post_import` of that batch returns `import_approval_declined` with no box, and that
     is not Tally's answer;
   - the call after it asks again.

   Here, any decline is followed by a STOP (3), so that second call is not made.
6. **A box left open** gives `approval.state` `pending` with `retry_after_s` (1392-E: about 40 s,
   then 15). Call again after `retry_after_s`, with the box left as it is. `approved` without a
   post: call again at once. Never call again after an attempt is recorded.
7. **Size bound** for every raw read: 200,000 bytes.

Licensed mode only; Education mode = STOP.

## Build

The build is BUILD-M, the one staged for rows 852-C2 and 1392-E. That sheet names its artifact
and digest; check both, and `bridge_mcp.exe` against `mcpb-smoke.json`, before the first call. Either
differing is a STOP. Record the exe's sha256 before and after.

BUILD-M's head is 187e882b. Line numbers here are at master c33adaf8. Between the two, the post path
changed only in invoice text: `agent.rs` gained 8 lines at `:2259` (one refusal's remediation),
and `agent_import_post.rs` changed one constant at `:2514` (an invoice's next step). So every cited
`agent_import_post.rs` line is the build's own, and so is every `agent.rs` line below `:2259`. Those
past it are 8 lower in the build: `:4130` is `:4122` there.

Over stdio, through the relay:
- `BRIDGE_AGENT_ENABLE_WRITES=true`, `BRIDGE_TERMS_ACCEPTED=true`;
- `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9003`. Port 9003 is used because 852-C2's
  relay may still be listening on 9002, and no process is ended (Difference 4);
- one new empty `BRIDGE_AGENT_DATA_DIR` for the whole row;
- `BRIDGE_AGENT_ENABLE_BATCH_POST` absent (set, it refuses as `setting_retired`, `agent.rs:1425`);
- `BRIDGE_AGENT_REDACTION` unset, so Tally's `LINEERROR` text is kept in the answer (`agent.rs:4130`).

The row uses one `bridge_mcp` process. The client is the lab's stdio client of 852-C1: no cancel,
and no call limit under 10 minutes.

## The files

- `gen_requests.py` writes `requests/` and `batches/`. Run `python3 -I gen_requests.py` here; it
  prints `8 request files, 3 batches`.
  - `marks`, `flags`, `ledgers`, `vouchers`, `w1-stock-ledger` and `w2-capital-ledger` come out
    byte-equal to A1's `requests/`.
  - `w3-expense-ledger` creates ledger `A2 Expense` under `Indirect Expenses`.
  - `s0-seed-journal` is A1-B's request byte for byte: Cash Dr 200.00, A1 Capital Cr 200.00,
    10-Apr-2025.
- `batches/p1.json`, `p2.json` and `p3.json` are the `vouchers` arrays for `build_import_xml`:
  - every `bridge_txn_id` is new;
  - each post has its own date and amounts;
  - narrations are plain ASCII.
- `check_1108.py` is A1's, byte for byte. Its modes are `marks`, `import`, `rows` and
  `diff BEFORE AFTER`. A read that failed prints `NOT READ` and exits 1; that is never "nothing" or
  "no change".
- `check-request-ids.py`: run `python3 -I check-request-ids.py requests` before the first send.
  EXPECT `census: ok 4, skipped 4, FAIL 0`.
- `relay.py` and `check-relay.py` are 852-C2's, byte for byte (sha256 80ffdb20 and 9a806848).
  `python3 -I check-relay.py relay.py` printed `0 failed` here on macOS before this sheet was sent.

| Post | Date | Voucher 1 | Voucher 2 | Voucher 3 |
|---|---|---|---|---|
| P1 | 12-Apr-2025 | `A1 Closing Stock` Dr / `A1 Capital` Cr 10.00 | | |
| P2 | 14-Apr-2025 | `A2 Expense` Dr / `A1 Capital` Cr 21.00 | `A1 Closing Stock` Dr / `A1 Capital` Cr 22.00 | `A2 Expense` Dr / `A1 Capital` Cr 23.00 |
| P3 | 16-Apr-2025 | `A2 Expense` Dr / `A1 Capital` Cr 31.00, narration `Row 1108-P3 twin` | `A1 Closing Stock` Dr / `A1 Capital` Cr 32.00 | identical to voucher 1, with its own `bridge_txn_id` |

## The code admits each trigger (c33adaf8)

| Point | Code | For this sheet |
|---|---|---|
| A first voucher is needed | With no voucher mark, the build refuses as `empty_book_first_import` (`agent_import.rs:176-191`; test `only_the_empty_book_parse_failure_is_named_distinctly`). This book's marks carry no `ALTVCHID` (`COMPANY_MARKS_EMPTY_CAPTURE_PROVENANCE.md`). | S0 seeds the voucher mark before any build. |
| Writes on, the batch included | One setting governs every post (`agent.rs:525`, `posting_from_env` `:703`); on, the limit is 50. | the launch above |
| Journal numbering | `post_import` sends no number (`agent_import.rs:262`). `admit_numbering_for_post` (`agent_import_numbering.rs:493`, called at `agent_import_post.rs:1140`) admits only Automatic with Retain Original Voucher No (`agent_import_numbering.rs:298`). | step 5's EXPECT; any other = STOP, and no setting is changed |
| A Stock-in-Hand line | A Journal takes any balanced set (`agent_import_schema.rs:67`), and Stock-in-Hand is non-money (`agent_import_cash_bank.rs:133`). | The builds of P1 to P3 EXPECT no refusal. |
| Dates | `validate_dates` admits from the books-from date to the host's today (`agent_import.rs:4851`). | April 2025, books from 1-Apr-2025 |
| Twins | The build refuses only a repeated `bridge_txn_id` (`agent_import.rs:4380`). The pre-read refuses only a voucher already in the book (`agent_import_post.rs:2475`). | P3's twins have distinct ids, and their date and amounts are on no earlier voucher. |
| Rows of another batch | `vouchers_already_posted` (`agent_import_ledger.rs:1357`) matches only the same `bridge_txn_id` with the same date and figures (`:1041`). | every id is new |

### Is `acknowledge_post_review` needed between posts? No.

The question was whether a batch left `reconciliation_required` and unreviewed refuses the next
`post_import` of another batch. On master it does not, for Journals:
- **Under the lock**, a post is refused for its own batch already sent (`AlreadyAttempted`,
  `agent_import_post.rs:1315`), a REMOTEID already on a dispatch intent (`:1320`), its own batch
  changed (`:1328`), and a row another batch holds (`TxnAlreadyPosted`, `:1335`).
- **The company stop** (`:1348`) judges only a batch that holds an invoice
  (`stopped_company_while_admitted`, `agent_import.rs:3567-3575`: "a stopped company stops no
  Journal, Payment, Receipt or Contra"). Test: `agent_import_stop_tests.rs:797`
  `the_stop_judges_only_an_invoice_post`.
- **`post_approval_busy`** fires only while another batch holds the one approval slot
  (`agent_import_approval.rs:656-660`). A batch sent and doubted holds none. Tests:
  `agent_import_approval_tests.rs:433` `a_dialog_declined_while_nobody_waits_does_not_block_another_batch`
  and `:864` `a_dialog_that_ended_unanswered_does_not_block_another_batch`.
- **Row sharing** is the one refusal that reaches across batches (test
  `agent_import_ledger_stream_tests.rs:707` `a_typed_transaction_id_matches_only_the_same_shape`,
  and e2e `agent_import_post_e2e_tests.rs:506`). This sheet shares no row.

**No test posts a second batch after a `reconciliation_required` one.** P2 and P3 are the first
live check of this, so it is an EXPECT of theirs (E-NB).

`acknowledge_post_review` is not run in this sheet at all. Its every doubt shows a native dialog,
and it writes to the data folder only. Doubt `not_created` fits P1 (`agent_import_settle.rs:38`).
For P2 and P3, no doubt fits: a partly created batch is never read as not created
(`agent_import_verification.rs:763-766`). The data folder is discarded with the row.

## Steps

One at a time. Every answer goes to a file. A file with no answer is a failed step, not an empty one.

0. **LAB-COMMON step 0.**
   - Load THE COMPANY alone.
   - **m0**, marks: EXPECT one COMPANY row, with M 210 and no `ALTVCHID`. Any other value: record it
     and STOP.
   - Record TallyPrime's release and edition as the screen shows them.
   - Run the census, check every file against `SHA256.txt`, and run
     `python3 -I check-relay.py relay.py` (EXPECT `0 failed`).
   - **Nothing listens on port 9003.** This read changes nothing: `netstat -ano -p tcp | findstr ":9003"`, with
     its whole output kept. EXPECT no line whose local address ends in `:9003`. Any such line, or a
     netstat that does not run: record it and STOP. Never end the process that holds the port.

1. **The first reads.** Send `flags`, `ledgers` and `vouchers`, each followed by `check_1108.py rows`.
   - `flags`: EXPECT ISINVENTORYON `Yes` and ISINTEGRATED `Yes`, else STOP.
   - `ledgers`: EXPECT `Cash` under `Cash-in-Hand`, and no `A1 Closing Stock`, `A1 Capital` or
     `A2 Expense`, else STOP.
   - `vouchers`: EXPECT no row.
   - Every GUID must begin with THE COMPANY's, else STOP.

2. **BK-A2** (C5), into the new empty folder `C:\bridge-lab\1108a2\backups\bk\`. Record its name,
   bytes and sha256. Then send `marks`, `ledgers` and `vouchers`: these are the BASELINE.

3. **W1, W2, then W3.** For each one:
   1. `marks` (C3);
   2. send the write once;
   3. take a screenshot of the whole Tally window;
   4. run `check_1108.py import`;
   5. send `ledgers` and `diff` it against the previous one.

   EXPECT `CREATED 1` and one row ADDED with the sent name and parent. Anything else: STOP, because
   every post needs all three.

4. **S0, the seed.** Send it the same way, then `marks` and `vouchers`, and `diff` `vouchers`.
   - EXPECT `CREATED 1`, the COMPANY row now carrying an `ALTVCHID`, and one voucher ADDED:
     Cash Dr 200.00, A1 Capital Cr 200.00.
   - Record that ALTVCHID as **V0**. Anything else: STOP.

5. **Relay and Bridge.**
   - Start the relay: `python -I relay.py 9003 9001 C:\bridge-lab\1108a2\relay`. It stays up for
     the whole row, and each import lands as `import-N` in that folder.
   - Start `bridge_mcp` as Build says.
   - `list_companies`: EXPECT THE COMPANY's GUID.
   - `tally_status`: record the release, licence and mode.
   - `masters` with `kind=voucher_types`: record the Journal row's `series_numbering.method` and
     `sub_method`. EXPECT `automatic` and `auto_retain`. Any other: STOP; this row alters no type.
   - **m5**, marks on 9001: EXPECT as after step 4.

6. **P1, P2, then P3.** For each post:
   1. `build_import_xml` with `{"company_guid": <GUID>, "vouchers": <batches/pN.json>}`. Keep the
      whole answer and record the batch id, `voucher_count` (EXPECT 1, 3, 3) and every warning. A
      refusal: keep it and STOP.
   2. **mN-before**, marks on 9001.
   3. `post_import` with `company_guid` and that batch id. When the box shows, the helper checks it
      (Difference 3), then Yes. Record each call's start and end in UTC, when the box showed and
      when Yes was clicked.
   4. Keep the post's whole answer. Record:
      - `created`, `attempt_recorded`, and `dispatch` (`state`, `response_state`, `resent`);
      - `dispatch.response.outcome`'s counters and `tally_line_errors`;
      - `error.code` and `next_step`;
      - each voucher's status, attribution and next step;
      - `duplicates`.
   5. **mN-after**, marks on 9001, then `vouchers`, and `diff` it against the previous `vouchers`.
   6. `verify_import` with that batch id, once: keep the whole answer, and record the same fields.
   7. Check the row of the EXPECT table below. A counter or mark that differs from it: record it
      and report to the coordinator before the next post. That is a pause, not a STOP.

7. **End of posts.** `egress_log`: the whole answer. Close the client so `bridge_mcp` exits by
   itself.

8. **The last reads.** Send `marks`, `ledgers` and `vouchers`, and `diff` the last two against the
   BASELINE. EXPECT, beyond the BASELINE:
   - ledgers: the three ledgers ADDED;
   - vouchers: five ADDED (S0, P2's 21.00 and 23.00, and P3's two 31.00);
   - V: V0 + 4.

9. **Restore BK-A2** (C6), only on the coordinator's explicit RESTORE. Then send `marks`,
   `ledgers` and `vouchers`. EXPECT:
   - the COMPANY row's GUID, V and M equal to the BASELINE's (M 210, no `ALTVCHID`);
   - each `diff` against the BASELINE showing no row ADDED, REMOVED or CHANGED.

   Anything else = STOP.

10. LAB-COMMON C10. Then stop the relay and keep its folder whole.

## EXPECT for each post

Each EXPECT has a counter, a mark and a status. The counters are from the relay's `import-N.answer`,
read by `check_1108.py import`, and agree with the post's `dispatch.response.outcome`. The marks are
from mN-before and mN-after and the `vouchers` diff. The statuses are from the post's answer and
then `verify_import`.

| Id | Post | Counter | Mark | Status |
|---|---|---|---|---|
| E-P1 | P1 | CREATED 0, EXCEPTIONS 1, every other counter 0; one `LINEERROR` naming `A1 Closing Stock` (recorded only: no verdict reads it, `import_outcome.rs:179`) | V unchanged (V0), M unchanged; no voucher ADDED | voucher `b1108-p1-1` `tally_reported_not_created` (`unmatched_cause` gives `ReportedNotCreated` for one voucher sent, `agent_import_verification.rs:789`, `:812`), with the next step of `:880`; the batch `reconciliation_required`; the post's `error.code` `import_reconciliation_required` (`agent_import_post.rs:2293`); `attempt_recorded` true; `response_state` `response_not_clean` (`:2320`) |
| E-NB | P2, P3 | (none before the box) | (none) | the post is not refused before the box: no `post_approval_busy`, `import_txn_already_posted` or `invoice_company_stopped` (the previous section) |
| E-P2 | P2 | CREATED 2, EXCEPTIONS 1, every other counter 0 | V = V0 + 2, M unchanged; two vouchers ADDED, 21.00 and 23.00 | no span binding, because the counters are not clean (`clean_create` gives `span_counters_not_clean`, `agent_import_span_identity.rs:189`, `:210`); the batch `verification_incomplete`, so `reconciliation_required` |
| **E-PL** | P2 | (as E-P2) | (as E-P2) | **The partial-landing finding.** `b1108-p2-1` and `b1108-p2-3` read `matching_content_observed` with attribution `not_established`. **Neither reads `posted_verified`.** Refuted if either does, in the post or in `verify_import`. |
| E-P2m | P2 | (as E-P2) | (as E-P2) | `b1108-p2-2` reads `sent_not_attributed`, with the neutral next step for an unclean answer (`agent_import_verification.rs:973`) |
| E-P3 | P3 | CREATED 2, EXCEPTIONS 1, every other counter 0 | V up by 2 from P2's, M unchanged; two vouchers ADDED, both 31.00 on 16-Apr-2025 | the batch `reconciliation_required` |
| E-T | P3 | (as E-P3) | (as E-P3) | `b1108-p3-1` and `b1108-p3-3` each read `duplicate_fingerprint`, with matches 2 and `ambiguous_within_batch` true; the batch's `duplicates` holds an `accounting_fingerprint` set (`agent_import_verification.rs:~515`, `:577`, `:1207`); `b1108-p3-2` reads `sent_not_attributed` |

The twin statuses come from reading the code, not from a test of this exact case.

## STOP RULES

- C7 and C8. Any dialog on Tally's screen after a write or a post: screenshot it, press no key,
  report.
- A box the helper does not pass, or any decline.
- A refusal by `build_import_xml`, or by `post_import` before an attempt is recorded.
- A raw write (W1 to W3, S0) that does not answer `CREATED 1` with every other counter 0, or a raw
  answer that does not parse or lacks a counter.
- A Journal type that is not `automatic` with `auto_retain`.
- A marks answer with other than one COMPANY row; a GUID that does not begin with THE COMPANY's.
- A mark that moves in a step that sends no write and no post.
- Any call with no answer in 10 minutes.
- Education mode.

After a stop, nothing more is sent or keyed (C8). The book then holds what was written before it,
and the way back is BK-A2.

## What the result decides

Recorded by the coordinator, each with its confidence.

| What the row shows | Reading |
|---|---|
| E-P1 as written | The `tally_reported_not_created` path ran live for the first time. Its answer and readback become fixtures for #1116. |
| P1 `CREATED 1` | Through `post_import`, Tally stores what it refused raw. Report before P2, since A2 then has no refused voucher. |
| E-P2, E-PL and E-P2m as written | #1108's acceptance is met: a partial landing recorded through `post_import`. The relay's request and answer, the marks and the readback are its fixtures. E-PL is confirmed: in a partial landing, the created vouchers are not `posted_verified` either. |
| E-PL refuted (a created voucher `posted_verified`) | The code binds part of an unclean span. Keep everything; the code is next. |
| E-T as written | Content matching cannot settle twins. This is the evidence for "do not name the rejected voucher". |
| A twin bound or named | Naming may be possible. Keep everything; it needs a design before code. |
| E-NB refuted (P2 or P3 refused before the box) | An unreviewed doubted batch does block the next post. Record the code and STOP; the sheet then needs `acknowledge_post_review` between posts. |

## Not measured by this row

- a book not integrated with inventory;
- a Payment, Receipt or Contra with a Stock-in-Hand line;
- `acknowledge_post_review` on any of these batches;
- a batch of more than three, or one past the 10-voucher line limit of the box;
- TallyPrime 7.1.

## Evidence

On a new branch, `lab/1108-a2-partial-landing-answers`, with `SHA256.txt`:
- every raw answer whole, with its size and sha256, and every `check_1108.py` output;
- the relay folder whole: `relay.jsonl`, and every `import-N.request` and `.answer`;
- every tool answer whole, with its size and sha256;
- each post's times and the box helper's log;
- BK-A2's name, bytes and sha256, and `restore-plan.txt`;
- a RESULTS.md with the EXPECT table filled in. Each Id is marked confirmed, refuted or not
  reached, and its first line names any pause or stop;
- a notes file with UTC times, one line per step.

The evidence shows THE COMPANY's GUID as `<company guid>` and its number as `<company number>`.
Screens stay on the lab PC, listed by name, size and sha256. Send the sha to the coordinator before
pushing, and push only after the pre-push scan and the coordinator's yes.
