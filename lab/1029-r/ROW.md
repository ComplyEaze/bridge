# Row 1029-R: five approved bank batches posted through Bridge on a book of about 30,000 vouchers, counting how often the read-back after the post is refused

Not for merge. Part of #1029. Written 4 Nov 2026. Lines below are at 187e882b, the build's head, and this branch starts there. Confidence of everything here until the row runs: unverified.

## Question
Issue #1029, item 3: when a batch goes through `post_import` on a busy book, how often is the read-back inside the call refused after the post was sent? The plan of 15 Oct calls this row B: five approved batch posts, expected none.

## Why this sheet, and not the old one
`lab/1029-large-book-posts` (872a56a0) was written for this item on 26 Oct and never ran. It is replaced, not repeated:
- **Its batches are dated 22 to 31 Mar 2027.** `validate_dates` refuses a date past the PC's own today (see Code), so all five would be refused at the build.
- **Its build** (`w1-2306d86d`) is older than the post path of today's master. Row 852-C1 (3 Nov, `lab/852-capture-answers`) posted two 50-voucher batches on this book on 838242cb, both clean. `agent_import_post.rs` has changed since then, so those two are context for this row, not part of its count.
- **Its approver** was the owner at the PC. Since 852-C1 the lab answers Bridge's box itself (Who acts, below).
- **Its Journals** needed voucher-type changes. These batches use the Payment, Receipt and Contra types, which 852-C1 read as automatic with retain on this book and did not alter.

## Book
`BRIDGE LARGE LAB`, GUID begins `2ba6df28` (whole on the lab PC, in `lab/1029-large-book-posts` and in 852-C1's RESULTS), books from 1-Apr-2026, TallyPrime 7.2 Silver, licensed. Synthetic, built by the lab on 21 Oct 2026: 29,930 Journals, 82 a day, and the ledgers `Lab Vol Expense`, `Lab Vol Income`, `Lab Vol Cash` and (since 852-C1) `Lab Vol Bank` under Bank Accounts.

Its state, from 852-C1's "The book now": V 30,030, M 211. It holds 852-C1's 100 vouchers (11 to 30 Oct 2026) and `Lab Vol Bank`, was not restored, and was left shut with one tally.exe on 9001. No row since has used it. Tally's current date read 31-Mar-2027 there.

Not on the PC, or another GUID: stop, the row has no book; never rebuild it. Any prompt when it opens: press no key, screenshot, stop and report its words. Education mode: stop.

## Build
BUILD-M, the one already staged for row 2161-C:
- the `unsigned-bundle-smoke-windows-latest` artifact of CI run 41772223948, the merge-queue run of #2329 at 187e882b5176a3b577d57f257a6ec4319fc2aecd;
- artifact id 13295846440, 59,336,955 bytes, GitHub digest `sha256:1940bd9736edc2aa76d811b3bcd0d6229ee69b5e01f5f59321a46b6ca28f691a`, expiring 11 Nov 2026.

Check the zip's sha256 against the digest, and `bridge_mcp.exe` against `mcpb-smoke.json`, before the first call. Either differing is a stop. Record the exe's sha256 before and after.

Over stdio, directly on 9001, no relay: `BRIDGE_AGENT_ENABLE_WRITES=true`, `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, one new empty `BRIDGE_AGENT_DATA_DIR` for the whole row, `BRIDGE_AGENT_ENABLE_BATCH_POST` absent (set, `post_import` refuses `setting_retired`), `BRIDGE_AGENT_REDACTION` unset. One `bridge_mcp` process for the whole row, so the batch journal carries from each build to its post and its check.

**The client** is the lab's stdio client of 852-C1: no cancel and no call limit. Record it. A call over 60 s is recorded beside the count, not a stop: a real host might have cut it.

## Who acts
- **The lab answers Bridge's approval box itself, Yes, by mouse on the box's own button,** as in 852-C1. Before the click, the box-check helper reads the box's text and allows Yes only if it names this company, its GUID, this batch's id and this batch's voucher count. A box that names anything else, or that cannot be read: Decline, record its text, and stop.
- **Nobody keys in Tally** from the first build to the last check. The one screen act is the backup (step 1).

## Kit
| file | what |
|---|---|
| `batches/r03.json`, `r10.json`, `r25.json`, `r50a.json`, `r50b.json` | 3, 10, 25, 50 and 50 numberless vouchers: 70 Receipts (`Lab Vol Bank` Dr, `Lab Vol Income` Cr), 55 Payments (`Lab Vol Expense` Dr, `Lab Vol Bank` Cr) and 13 Contras between `Lab Vol Bank` and `Lab Vol Cash`. All dated 1 to 7 Oct 2026, each batch over all seven days, except r03 over 1 to 3 Oct. Ids `r1029-<batch>-<nnn>`, narrations `r1029 <batch> <nnn>`. |
| `make-batches.py` | writes those five: `python -I make-batches.py OUT` gives the same bytes. No two of the 138 share a date, a type and an amount. |
| `marks.utf16le.xml` | the company's GUID, ALTVCHID and ALTMSTID, UTF-16LE with a BOM: `lab/725-dense-200`'s request with only the company name changed. Send it with `Content-Type: text/xml; charset=utf-16`. |

`SHA256.txt` lists every other file here with its sha256, in `sha256sum -c` form. Check it first, and check each batch's sha256 again when it is used.

The days 1 to 7 Oct 2026 hold the book's own Journals (82 a day) and none of 852-C1's vouchers. No batch voucher has those Journals' ledger pair (`Lab Vol Expense` and `Lab Vol Income`), so `unrelated_duplicates_in_window` is expected empty. Anything in it is recorded.

## Steps
One at a time. Every answer goes to a file. A file with no answer is a failed step, not an empty one.

0. Load the book alone. Screenshot. `GET /status` on 9001. Marks raw on 9001: EXPECT one COMPANY row, this GUID, **V 30,030 and M 211**. Another V or M is recorded and reported before step 1, not a stop. Record Tally's release, Tally's date and the PC's date. The PC's date must be on or after 7-Oct-2026.
1. **BK.** Tally's Backup screen: a full backup of the company into a new empty local folder. The one prompt expected is "Accept ?", answered y. Any other prompt: stop, press nothing, and ask. Record the file's name, size and sha256. Marks: EXPECT V and M as step 0.
2. Start `bridge_mcp` as above.
   - `list_companies`: EXPECT this GUID, `books_from` 20260401.
   - `tally_status`: record release, licence and mode.
   - `masters` with `kind=voucher_types`: record the Payment, Receipt and Contra rows' `series_numbering.method` and `sub_method`. EXPECT automatic and auto_retain for all three, as 852-C1 read them. Any other: stop. This row alters no type.
   - Marks: EXPECT as step 0.
3. For each batch in order, r03, r10, r25, r50a, r50b:
   a. **Build.** `build_import_xml` with `{"company_guid": <GUID>, "vouchers": <the file's array>}`, the file unchanged. Keep the whole answer and record the batch id, `voucher_count` and every warning. A refusal: keep it and stop.
   b. **Post.** `post_import` with `company_guid` and the batch id. Record each call's start and end as UTC with milliseconds, when the box showed, and when Yes was clicked (Who acts).
      - A result with `approval.state` `pending`: call again after its `retry_after_s`.
      - `approved` without a post: call again at once.
      - Each call is its own record. Never call again after an attempt is recorded.
      - Keep each result whole: `attempt_recorded`, `dispatch` (its `state`, `counters`, `response_state`, `resent`), and any `error` with its `code`, `retry_after_s` and `next_step`.
   c. **Screen.** Screenshot the whole Tally window: the Gateway, no dialog.
   d. **Check.** If the post's result names `retry_after_s` before `verify_import`, wait it out. Then `verify_import` with the batch id: the whole answer. EXPECT `posted_verified` for every voucher of the batch. A deadline inside `verify_import` itself is recorded with its code, and `verify_import` is called once more only if its next step says so.
   e. `egress_log`: the whole answer (the post's `response_prepared` line among it).
   f. Marks raw on 9001: EXPECT V up by the batch's count (r03 +3, r10 +10, r25 +25, r50a +50, r50b +50) and M unchanged. Otherwise: report before the next batch.
4. Marks: EXPECT **V 30,168**, M 211. Stop `bridge_mcp`. Shut the company by Tally's own Shut Company and write down the path and keys. Screenshot: no company loaded. One tally.exe owns 9001.

The 138 vouchers stay: this book is a fixture for this kind of row. BK is the way back if the run coordinator says restore.

## What counts
**A refused read-back** is a post whose attempt was recorded (`attempt_recorded` true), and whose result either:
- reads `dispatch.state` `reconciliation_required` (with `error.code` `import_reconciliation_required`, or another code); or
- carries an `error` whose `next_step` says to call `verify_import` (the port was busy or draining after the send, or a read after the send was refused).

Each one is counted with its code, `retry_after_s` and next step, and settled by step 3d.

Counted apart, and not as refusals:
- a refusal **before** an attempt is recorded (`attempt_recorded` false): that is a stop;
- `import_dispatch_outcome_unknown` (the reply was lost);
- a call over 60 s;
- any `request_deadline_exceeded` or refused part after the send that the call itself got past, read from `egress_log`.

## Record
| | r03 | r10 | r25 | r50a | r50b |
|---|---|---|---|---|---|
| build: batch id, `voucher_count`, seconds, warnings | | | | | |
| calls: each start, end and seconds; box shown at; Yes at | | | | | |
| `approval.state` of each call | | | | | |
| `attempt_recorded`; `dispatch.state`; `response_state`; `resent` | | | | | |
| Tally's counters (`created` and every other) | | | | | |
| `error`: code, `retry_after_s`, `next_step` | | | | | |
| refused read-back (What counts): yes or no | | | | | |
| `egress_log`: `response_prepared` present; any deadline or refused part, before or after the send | | | | | |
| `verify_import`: status counts, seconds | | | | | |
| marks after: V, M | | | | | |
| Tally's screen after | | | | | |

Plus: step 0's marks, release and dates; BK's name, size and sha256; step 2's three type rows; the client and its limit; the exe's sha256 before and after.

## Decides
| what the row shows | reading on #1029 item 3 |
|---|---|
| five posts, each `posted_verified` in the call with `created` its count, no refused read-back, and `verify_import` `posted_verified` n of n for each | **0 of 5 on this book:** item 3 is measured. With row A's two halves it closes #1029. |
| one or more refused read-backs, each settled by `verify_import` as `posted_verified` | **k of 5, each with its code:** the refusal fires on a busy book and its next step settles it. Item 3 is measured; whether the rate is acceptable is a separate decision, recorded on the issue. |
| a refused read-back that `verify_import` does not settle as `posted_verified` | **A finding:** stop, report it whole before the next batch. |
| a post whose counters show anything other than the batch created | **A finding:** stop, report it whole before the next batch. |

## Stop rules
- Any dialog on Tally's screen, before or after a call.
- Any refusal before an attempt is recorded.
- Any call with no answer in 10 minutes.
- A box the helper does not pass (Who acts).
- A type in step 2 that is not automatic with retain.
- V not as expected after a batch: report before the next batch.
- A mark that moves outside the steps that change it.
- Education mode.

After a stop, nothing more is sent or keyed. Marks may be read once more to record the state, if no dialog is up.

## Limits
- Silver, one user, four bank and income ledgers, a book of about 30,000 vouchers built for this kind of row. Not Gold, not a client book, and nobody working in Tally during a post.
- Five posts. Zero refusals in five does not show the rate is zero; it says the refusal is not common at this size.
- The client does not cancel, so a reply lost to a host cancel cannot appear here.
- Each batch's read-back covers its own dates (seven days for four batches, three for r03). How long each call takes depends on that span: inference from 852-C1, where a ten-day span took about 49.5 s a call.
- Not measured here: item 3 under a host limit; row A's `voucher_presence` half, which needs a Manual-numbered type and a party (the tool cannot identify a natively posted voucher, as its description says).

## Code admits the trigger (187e882b, the build's head)
- **The cap:** `MAX_BATCH_POST_VOUCHERS` 50 (agent_import_ledger.rs:14). `post_voucher_limit` (agent_import_post.rs:432) gives it once posting is on. Each batch is at most 50.
- **A click in the call's wait posts in that call** when a redeem still fits: a redeem is measured for 50 vouchers (`MEASURED_REDEEM_VOUCHERS`, agent_import_approval.rs:68; `fits_within`, :73), under `CALL_BUDGET` 40 s (:46), from `ask_and_hold` (agent_import_post.rs:578).
- **The types:** `LIVE_QUALIFIED_VOUCHER_TYPES` (agent_import.rs:419) holds Journal, Payment, Receipt and Contra.
- **No numbers:** a numbered voucher is refused by `require_native_numbering` (agent_import_post.rs:3100). Numberless needs Automatic with retain, checked by `admit_numbering_for_post` (called at agent_import_post.rs:1140, defined at agent_import_numbering.rs:493). Hence step 2's check.
- **Dates:** `validate_dates` (agent_import.rs:4851) refuses a date before books-from or past the PC's today (`tally_host_today`, agent.rs:3950). 1 to 7 Oct 2026 is inside both from 7 Oct 2026 on. March 2027 is not, which is why the old batches cannot be used.
- **The send:** through `post_approved_import` (called at agent_import_post.rs:1288, defined at tally/runtime.rs:3499).
- **The read-back inside the call:** `verify_import_after_current_dispatch` (called at agent_import_post.rs:1614, defined at agent_import.rs:2235). Its result is finished by `finalize_current_dispatch` (agent_import_post.rs:2441), which sets `posted_verified` only when the response is clean, every voucher is verified and no masters doubt was recorded. Otherwise it sets `reconciliation_required` with `import_reconciliation_required` (`mark_reconciliation_required`, :2293).
- **A busy or draining port after the send:** next step `BUSY_AFTER_POST_NEXT_STEP` (agent_import_post.rs:2141, set at :2237), which sends the caller to `verify_import`.
- **The lost reply:** `import_dispatch_outcome_unknown` (agent_import_post.rs:2100).

## Who sends each EXPECT's request, and what each count counts (187e882b)
| step | request | sent by | decided by | the count |
|---|---|---|---|---|
| 0, 1, 2, 3f, 4 | marks, raw | the operator's curl on 9001, not Bridge | the step itself | one COMPANY row. V is that company's voucher mark (ALTVCHID), so +n is n vouchers created and no other voucher change. M is its master mark (ALTMSTID). |
| 2 | the company list | `list_companies`, agent.rs:3380 | always sent by that call | one company with this GUID |
| 2 | voucher types | `masters`, agent.rs:3394 | `kind=voucher_types` in the call | none; the three rows' method and sub-method |
| 3a | the build's reads | `build_import_xml`, agent.rs:3368 | the call itself | `voucher_count` counts the batch's vouchers |
| 3b | reads, the one import, then the read-back | `post_import`, agent.rs:3406, then agent_import_post.rs:636 | the import only after the approval, through `post_approved_import` (:1288); the read-back after it (:1614) | Tally's `CREATED` counts vouchers created; `posted_verified` counts vouchers found |
| 3d | the verification reads | `verify_import`, agent.rs:3432, then agent_import.rs:2098 | the call itself | `posted_verified` counts the batch's vouchers found by the GUIDs its post created |
| 3e | none sent to Tally | `egress_log`, agent.rs:3374 (defined at :3524) | the call itself | the log's records, read back from the data folder |

## Keep
On `lab/1029-r-answers`, with `SHA256.txt`:
- every tool answer whole, with its size and sha256;
- each call's times and the box log;
- the marks answers;
- BK's name, size and sha256;
- a RESULTS.md with the Record table filled in and the count first;
- a notes file with UTC times, one line per step.

Screens stay on the lab PC, listed by name, size and sha256. Send the sha to the run coordinator before any push.
