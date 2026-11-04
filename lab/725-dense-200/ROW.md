# Row 725-D1: one batch of 200 Journals posted through Bridge, on a book of about 30,000 vouchers

Not for merge. Part of #725. Lines below are at 9beedca5 (`fix/725-cap-200`, #2333), the build's head, and this branch starts there. Confidence of everything here until the row runs: unverified.

## Question
With the batch cap at 200, does one batch of 200 Journals reach `posted_verified` on BRIDGE CORPUS DENSE, and how long does its one import POST take, set against the transport's 20 s limit on every request?

## Why
#2333 raises the cap from 50 to 200, and stays a draft until this row answers. The import POST runs under the one request limit of 20 s (`DEFAULT_REQUEST_TIMEOUT`, bridge-tally-transport lib.rs:95, taken at runtime.rs:263). Past it the send ends and the answer is lost while Tally's write stands. A run on 26 Sep 2026 posted 200 Journals in one POST of 5.24 s on a small 7.1 book. Nothing has posted more than 50 through Bridge on a book this size since. 200 ships only if this POST takes at most 15 s and every verdict holds.

## Book
`BRIDGE CORPUS DENSE`, synthetic, TallyPrime 7.2 Silver, licensed, on the lab PC, loaded alone. Row 2265-R4b (`lab/register-large-book-answers`, `lab/register-large-book/RESULTS.md`) restored it on 3 Nov 2026 from its backup **BK0** (14,661,545 B, sha256 `223a2ceaef95bfae88d2e6115dcea8745401d8540c96d523ad61cb85c7c678da`, RESULTS.md:8) and left it at V 29,900, M 328 and 123 ledgers (RESULTS.md:71). Since then its data folder has not changed: 20 files, about 145 MiB. Its last voucher is dated 31-Mar-2026, the last day of FY 2025-26. Its ledgers include `Cash` (Cash-in-Hand) and `DN Sales` (Sales Accounts), as in the captured month's Trial Balance of this book (`statement_trial_balance_dense_month_live.utf16le.xml`, protocol fixtures).

Not a client book, and nothing seeded from client data. Not on the PC, another GUID, or Education mode: stop, the row has no book. Any prompt when it opens: press no key, screenshot, stop and report its words.

The row ends by restoring BK1, so the book is left as it was found.

## The one variable
The cap: `MAX_BATCH_POST_VOUCHERS` 200 (agent_import_ledger.rs:14), against 50 on master. The batch has the 26 Sep shape: 200 Journals, one Dr and one Cr each, no voucher numbers (Tally numbers them automatically), on an empty day. The 26 Sep ledgers are not in this book, so the batch uses two of its own: `Cash` Dr, `DN Sales` Cr, k + 0.25 for k = 1 to 200 (1.25 to 200.25, total 20,150.00), all dated 1-Apr-2026, the first day after the last voucher.

## Build
The Windows bundle of dispatched CI run 41779256660 (`workflow_dispatch` on `fix/725-cap-200`, head `9beedca5de5463282ceef1229d6a719a9123bc01`). Artifact `unsigned-bundle-smoke-windows-latest`, id 13299251439, digest `sha256:1d9cde82b9de8ea98c4068e7b7e67d73cc9919542653179cd869dbc23f5edb58`. Its native tests fail by intent, since some still spell 50. Its Windows bundle smoke passed. Check the zip's digest first: another digest is a stop. Record `bridge_mcp.exe` sha256 before and after.

Over stdio, through the relay: `BRIDGE_AGENT_ENABLE_WRITES=true`, `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9002`, one new empty `BRIDGE_AGENT_DATA_DIR` for the whole row, `BRIDGE_AGENT_ENABLE_BATCH_POST` absent, `BRIDGE_AGENT_REDACTION` unset. One `bridge_mcp` process for the whole row: the approval and the journal carry from each call to the next. Record the client and its call limit. A call over 60 s is recorded, not a stop.

## Kit
| file | what |
|---|---|
| `batches/b200.json` | the 200 Journals above, ids `r725-001` to `r725-200`, narrations `r725 cap 200 1` to `r725 cap 200 200` |
| `make-batch.py` | writes `b200.json`: `python -I make-batch.py OUT` gives the same bytes |
| `marks.utf16le.xml` | the company's GUID, ALTVCHID and ALTMSTID, UTF-16LE with a BOM; the request of `lab/1038-release-sheet` with only the company name changed. Send it with `Content-Type: text/xml; charset=utf-16`. |
| `relay.py` | the capture relay of `lab/852-capture-sheet`, unchanged: it times every exchange, keeps every import's request and answer, logs `reply_lost`, and writes no request of its own |
| `check-relay.py` | its 20 checks on a stand-in, unchanged; `python -I check-relay.py relay.py`, all pass here on macOS |

`SHA256.txt` lists all of it. Check it first.

## Prompts this row expects
1. **"Accept ?"** on the Backup screen (step 2): y.
2. **"Accept ?"** on saving the Journal voucher type (step 3), only if it has to be altered: y.
3. **"Accept ?"** at the restore (step 10): y.
4. **"Company data folder (<company number>) already exists"** at the restore: Replace.

Bridge's approval box at step 5 is Bridge's, not Tally's, and step 5 says how to answer it. Any other prompt or dialog: press no key, screenshot, record its words, stop. A prompt that is not named here is a stop.

## Steps
One at a time. Every answer goes to a file, whole. A file with no answer in it is a failed step, not an empty one. "Marks" means `marks.utf16le.xml` sent raw to 9001, never through the relay. Write down V (ALTVCHID) and M (ALTMSTID) and keep the answer with its sha256. Time every tool call: its start and end from the client's log, in seconds.

0. Check `SHA256.txt`. Load BRIDGE CORPUS DENSE alone. Screenshot. `GET /status` on 9001. Record Tally's release, Tally's date and the PC's date. EXPECT the data folder to hold 20 files of about 145 MiB in all, and BK0 to be in place at 14,661,545 B with sha256 `223a2cea…` (whole above). Another count, size or hash is recorded, and it is a stop before any write.
1. **s1.** Marks. EXPECT one company with this book's GUID, V 29,900 and M 328. Another V or M is a stop before any write. Keep the answer as `s1-marks.xml`.
2. **BK1.** Tally's Backup screen: a full backup of the company into a new empty local folder (prompt 1). Record its name, size and sha256. Marks: EXPECT V and M as s1.
3. Start the relay: `python -I relay.py 9002 9001 <new folder>\a`. Start `bridge_mcp` pointed at 9002.
   - `tally_status`: record it.
   - `list_companies`: EXPECT this GUID. Record its books-from date.
   - `masters` with `kind=voucher_types`: record the Journal row's `series_numbering.method` and `sub_method`. If they read automatic and auto_retain, leave them. Otherwise alter the Journal type in Tally and set only two fields: the method of voucher numbering `Automatic`, and "Retain Original Voucher No." `Yes`. Screenshot before and after, save (prompt 2) and leave with Ctrl+Q, never Escape. A field that is not on the screen is a stop. Read `masters` again: EXPECT automatic and auto_retain. Marks: EXPECT V as s1; record M.
   - `vouchers` from 2026-04-01 to 2026-04-01: EXPECT no voucher on the day.
4. **Build.** `build_import_xml` with `{"company_guid": <GUID>, "vouchers": <b200.json>}`. Keep the whole answer and record batch id **A**. EXPECT `voucher_count` 200 and no refusal. Marks: EXPECT V as s1.
5. **Call 1.** `post_import` with `company_guid` and A. Touch nothing while it runs. EXPECT `approval.state` `pending`, with Bridge's box open on the screen. If instead it refuses `import_review_too_large` with `attempt_recorded` false, record it and call once more: its own message says the next ask shows the totals only. Any other answer is a stop.
   Only **after call 1 has returned**, read the box. It must say `Create 200 vouchers in "BRIDGE CORPUS DENSE"`, `Types: 200 Journal`, both dates on 1-Apr-2026, a total debit and a total credit of 20,150.00, one line per ledger with 200 entries each for `Cash` and `DN Sales`, `Company GUID:` this GUID, and `Batch:` A. Screenshot it. If it does, click **Yes with the mouse**, never Enter, and record the time of the click. If anything differs, click No, screenshot and stop.
6. **Call 2.** `post_import` with A again. The code admits two answers; record which one came:
   - `approval.state` `approved`: the click was taken by this call, and a redeem of 200 does not fit in it. Go to step 7 at once.
   - the post itself, redeemed from the start of this call. Read it as step 7 does, and skip the call there.
7. **The post call.** `post_import` with A, once. EXPECT:
   - `dispatch.state` `posted_verified`, `counters.created` 200 and `attempt_recorded` true.
   - From `a\relay.jsonl`: exactly one `sent` line and one import exchange. An import exchange is a line with no `event` field and a `number` that is not null; count lines by their parsed fields, never with a substring search. Record that line's `sent`, `first_byte_s`, `answer_s` and `bridge_left_s`, and the bytes of `import-1.request` and `import-1.answer`. EXPECT `answer_s` under 20, no `reply_lost`, `tally_closed` or `relay_error`, and Tally's counters in `import-1.answer` CREATED 200, everything else 0.
   - Marks: EXPECT V s1 + 200.

   If this call ends `import_dispatch_outcome_unknown`, or with `dispatch.state` `reconciliation_required`, that is a result, not a failure of the row. Record the answer whole, the relay's import line (`bridge_left_s` near 20, `reply_lost`) and Tally's counters from `import-1.answer`, and keep them all. Do not resend and do not post again: no other `post_import` call with A or any batch. Go to step 8 and then step 10.
8. **Verify.** Wait one minute. Marks: record V. `verify_import` with A. EXPECT `posted_verified` 200 of 200. Record the call's seconds and, from `relay.jsonl`, the requests it sent, their bytes in all, and its largest answer. Marks: EXPECT V as before the call. After a step 7 that ended `reconciliation_required`, this one read shows whether Tally's write stood: record whatever it says, with no EXPECT.
9. Screenshot Tally's Day Book for 1-Apr-2026: EXPECT Journals with the narrations `r725 cap 200 1` and onward, and Tally's count of them on the screen.
10. **Way back.** Stop `bridge_mcp`, then stop the relay. Shut the company with Tally's own Shut Company. On Tally's Restore screen, restore BK1 into Tally's data folder, answering prompts 3 and 4 only. Load the company. Screenshot. **s9:** Marks. EXPECT the s9 answer byte-identical to s1's (compare their sha256): V 29,900, M 328. Another answer: stop and report, and run nothing more on this book. Shut the company. Retire the data folder: keep it for the answers, and use it for nothing else.

No one keys in Tally except at steps 0, 2, 3 (only if the Journal type is altered), 5 (Bridge's box), 9 and 10, and then only as those steps say.

## What to report
| step | result |
|---|---|
| s1: V, M, sha256; Tally's release; the dates; folder count and size; BK0 | |
| BK1: name, size, sha256 | |
| Journal numbering, and any change made | |
| build: A, seconds | |
| call 1: answer, seconds; the box as read; Yes at | |
| call 2: which answer, seconds | |
| post call: `dispatch`, created, seconds | |
| the POST from the relay: `sent`, `first_byte_s`, `answer_s`, `bridge_left_s`, request and answer bytes, Tally's counters | |
| requests before the import, their bytes and summed `answer_s`; requests after it, the same | |
| any `reply_lost`, `tally_closed`, `relay_error` | |
| marks after the post | |
| verify: verdict, seconds, requests, bytes, largest answer; marks before and after | |
| s9: V, M, sha256 equal to s1's or not | |

Read every number from its file, not from the screen. Put any refusal, and any EXPECT not met, in the first line.

## Decides
| what the row shows | reading on #725 |
|---|---|
| `posted_verified` 200 of 200 at the post and the verify, CREATED 200, one import, V s1 + 200, s9 equal to s1, and the POST's `answer_s` at most 15 | 200 ships: #2333 leaves draft on this row |
| every verdict holds, but `answer_s` over 15 and under 20 | the cap stays 50. The next step is a cap of 100, or a deadline for the import leg alone, which needs its own ruling |
| the POST ends at 20 s: `reconciliation_required`, `bridge_left_s` near 20, `reply_lost` | the cap stays 50, as above. Tally's counters and step 8 show what Tally wrote |
| a refusal before an attempt is recorded | record the code and the step. The row has no reading on the cap |
| s9 not equal to s1 | the restore did not take the book back: stop. This is not a reading on the cap |

Nothing is sent again after a refusal. `verify_import` reads, and never posts.

## Stop rules
- Any refusal before an attempt is recorded, apart from the one `import_review_too_large` that step 5 names.
- Any prompt or dialog not named under "Prompts this row expects".
- A box whose text differs from step 5: answer No.
- Any `post_import` call after an attempt is recorded, other than the post call itself.
- V or M not as EXPECTed at s1, after the post, or at s9.
- Any call with no answer in 10 minutes.
- Education mode.

## Code admits the trigger (9beedca5, the build's head)
Paths are under `src-tauri/src/`. Each line was read at 9beedca5. Nothing here was built or run for this note.
- **The cap:** `MAX_BATCH_POST_VOUCHERS` 200, agent_import_ledger.rs:14. `post_voucher_limit` (agent_import_post.rs:431) gives it for the four voucher types once posting is on (`post_voucher_limit_when_enabled`, :441). `BRIDGE_AGENT_ENABLE_WRITES` turns posting on (agent.rs:687). Without it `post_import` refuses `import_posting_disabled` (agent.rs:3235), and with `BRIDGE_AGENT_ENABLE_BATCH_POST` set it refuses `setting_retired` (agent.rs:3238). A saved batch over the cap is refused `import_post_batch_too_large` (agent_import_post.rs:3047), and the journal reads at most the cap (agent_import_ledger.rs:1490).
- **The type and the legs:** Journal is in `LIVE_QUALIFIED_VOUCHER_TYPES` (agent_import.rs:388), and a Journal has no bank leg (agent_import.rs:368), so `Cash` and `DN Sales` are admitted as its two entries.
- **No numbers:** a numberless voucher needs the Journal type at Automatic with Retain Original Voucher No. (`admit_numbering_for_post`, called at agent_import_post.rs:1103), hence step 3.
- **Dates:** `validate_dates` (agent_import.rs:4550) admits each date from books-from to the host PC's own today (`tally_host_today`, agent.rs:3845). 1-Apr-2026 is inside both.
- **The book read before the post:** the post reads the book and refuses a row already there (`require_absent_verification_result`, agent_import_post.rs:924 and :2412). The day is empty at step 3, so it finds none.
- **The approval:** a click taken in the call's wait posts in that same call only if a redeem still fits it. A redeem is measured live for 50 vouchers and no more (`MEASURED_REDEEM_VOUCHERS`, agent_import_approval.rs:68; `fits_within`, :73). So for 200, a click found while joining fails the redeem guard (agent_import_post.rs:551-555) and returns `approved` (:559), and an approval held when a call starts is redeemed by that call (`Begin::Redeem`, agent_import_post.rs:517). These are the two answers of step 6. `CALL_BUDGET` is 40 s (agent_import_approval.rs:46).
- **The send:** taken for dispatch at agent_import_post.rs:1206, inside the endpoint's dispatch lease (:1222), through `post_approved_import` (called at agent_import_post.rs:1249, defined at runtime.rs:3829), which takes the wire lock before the attempt is recorded (runtime.rs:4193). The one import send is `post_probe_xml` at runtime.rs:4235, under the 20 s limit.
- **The endings:** `finalize_current_dispatch` (agent_import_post.rs:2378) gives `posted_verified` only on a clean answer, a verified readback and no master doubt, and `reconciliation_required` otherwise (:2388). A send that fails after the attempt is recorded is `import_dispatch_outcome_unknown` (`refusal_code`, agent_import_post.rs:2037).

## Who sends each EXPECT's request, and what each count counts (9beedca5)
| step | request | sent by | decided by | the count |
|---|---|---|---|---|
| 1, 2, 3, 4, 7, 8, 10 | marks, raw | the operator, on 9001, not Bridge | the step itself | one COMPANY row. V is that company's voucher mark (ALTVCHID), so + 200 is 200 vouchers created and no other voucher change. M is its master mark (ALTMSTID) |
| 3 | the company list | `list_companies`, agent.rs:3276 | always sent by that call | one company with this GUID |
| 3 | voucher types | `masters`, agent.rs:3290, then agent_masters.rs:15; kind arm `"voucher_types"`, runtime_masters.rs:27 | `kind=voucher_types` in the call | none |
| 3 | the day's vouchers | `vouchers`, the one-day window | the call itself | vouchers on 1-Apr-2026: none |
| 4 | the build's reads | `build_import_xml`, agent.rs:3264, then agent_import.rs:1070 | the call itself | `voucher_count` counts the batch's vouchers: 200 |
| 5, 6 | the checks before the ask | `post_import`, agent.rs:3302, then agent_import_post.rs:635 | the call itself | none |
| 7 | reads, then the one import | `post_import`, as above; the import at runtime.rs:4235 | reached only after the approval is taken (agent_import_post.rs:1206), inside the dispatch lease (:1222), through `post_approved_import` (:1249) | Tally's `CREATED` counts vouchers Tally created: 200. The relay's import exchanges count Import Data requests: 1 |
| 7 | the readback | `verify_import_after_current_dispatch`, agent_import_post.rs:1562, defined at agent_import.rs:2027 | a recorded dispatch | as step 8 |
| 8 | the verification reads | `verify_import`, agent.rs:3327, then agent_import.rs:1890 | the call itself | `posted_verified` counts vouchers found by their markers: 200 of 200 |

## Limits
- TallyPrime 7.2 Silver, one user, one book, one run, one batch of Journals. 7.1 is not covered, and neither are Payment, Receipt and Contra, which the same cap admits.
- The 26 Sep run was on another book, release and build, so it is set beside this row for size only, not as a controlled pair.
- The relay reads each answer whole before it forwards it, so whole-call seconds run a little over a direct call. The POST's seconds are the relay's, from its own clock.
- No one works in Tally during the post. Activity alongside a post stays unmeasured.

## Keep
On `lab/725-dense-200-answers`, with `SHA256.txt`: every tool answer whole; `relay.jsonl` and every `import-N.request` and `.answer`; every marks answer, s1 and s9 among them; BK1's name, size and sha256; the call times; the screens by name and hash. Screens stay local. A `RESULTS.md` holding the table above. Send the sha.
