# Row 852-C1: a 50-voucher bank batch posted twice behind a capture relay, on a book of about 30,000 vouchers

Not for merge. Part of #852. Lines below are at master 838242cb, the build's head. Confidence of everything here until the row runs: unverified.

## Question
When a 50-voucher bank batch goes through `post_import` on a book of about 30,000 vouchers, how long does the import exchange take, how long does the whole call take against a host limit of about 60 s, and does Bridge keep Tally's reply?

## Why
#852 says nothing is built until the lost reply is captured. On a client's Gold book a batch of 50 ended `import_dispatch_outcome_unknown` after about 43 s, with an intent, no response record, `response_missing` and every voucher read back. No capture holds that exchange, so nobody can say whether the reply was cut by Bridge's own 20 s deadline, by the host, or by something else.

**The best result here reads "not reproduced at this scale". It does not close #852:** this is a Silver book with four ledgers, and the failing book was a client's Gold book.

## Book
`BRIDGE LARGE LAB`, GUID begins `2ba6df28` (whole on the lab PC and in `lab/1029-large-book-posts`), books from 1-Apr-2026, built 21 Oct 2026 on 7.1 for this row: 29,930 Journals, 82 a day; ledgers `Lab Vol Expense`, `Lab Vol Income`, `Lab Vol Cash` (opening 10,00,000.00 Dr). Backup BKL-B 9,047,789 B, sha256 4165fc8a40746e11e21a846eabddf2a83bd24a2f398e0f555a5bfd47d1189a8a.

Not BRIDGE CORPUS DENSE: other rows rely on its marks, and it holds none of these ledgers.

Not on the PC: restore BKL-B. Neither: stop, the row has no book.

Last opened on 7.1. Any prompt when 7.2 opens it: press no key, screenshot, stop and report its words.

## Build
The Windows bundle of the newest green master run that built one: CI run 41597505434, a push of master at 838242cb (`838242cb54a720baa2cf397cf8d0980e1c47ba45`), artifact `unsigned-bundle-smoke-windows-latest`, id 13213581672, digest `sha256:a865654b1fc3c2b5a9dcd5bc4f8c555605fbcb180f1b8f7d66a196fea91c01e4`. Check the zip's digest first: another digest is a stop. Record `bridge_mcp.exe` sha256 before and after.

Over stdio: `BRIDGE_AGENT_ENABLE_WRITES=true`, `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9002`, one new `BRIDGE_AGENT_DATA_DIR` for the whole row, `BRIDGE_AGENT_ENABLE_BATCH_POST` absent, `BRIDGE_AGENT_REDACTION` unset.

**The client sends no cancel and has no call limit under 10 minutes.** Record what it is. Over 60 s here, a real host would have cut the call; this row keeps the reply and counts that apart.

## Kit
| file | what |
|---|---|
| `relay.py` | the relay of `lab/2176-r0`, without its drop mode, plus exchange times, every import kept, and `reply_lost` |
| `check-relay.py` | 20 checks on a stand-in; `python -I check-relay.py relay.py`, all pass here on macOS |
| `seed-bank-ledger.xml` | one ledger, `Lab Vol Bank` under Bank Accounts, the shape of `lab/demo-traders-book`, UTF-8 |
| `batches/c50a.json`, `c50b.json` | 24 Receipts, 20 Payments, 6 Contras each, numberless; a on 11 to 20 Oct 2026, b on 21 to 30 Oct |
| `make-batches.py` | writes those two, the same bytes each run |

`SHA256.txt` lists all of it. Check it first.

## Steps
One at a time. Every answer goes to a file. A file with no `open` line is a failed row, not an empty one.

0. Load the book alone. Screenshot. `GET /status` on 9001. Marks read raw on 9001: EXPECT one company, this GUID, V 29,930; another V is recorded and reported, not a stop. Tally's date and the PC's date.
1. BK: full backup into a new empty folder: name, size, sha256.
2. `seed-bank-ledger.xml` to 9001, `charset=utf-8`: EXPECT `CREATED` 1, every other counter 0. A seeded row sets the test up and is no evidence of its own shape. Marks.
3. Voucher types. Through the build, with Tally on 9001 for this step only: `masters` with `kind=voucher_types`: record Payment, Receipt and Contra, `series_numbering.method` and `sub_method`. A type that reads automatic with auto_retain stays. For each other one, in Tally: method of voucher numbering `Automatic`, "Retain Original Voucher No." `Yes`; screenshots before and after. Leave with Ctrl+Q, never Escape. A field that is not on the screen is a stop. Read `masters` again. Marks.
4. Start the relay: `python -I relay.py 9002 9001 <new folder>\a`. Point the build at 9002. `list_companies`: EXPECT this GUID.
5. Screenshot: Tally at the Gateway, no one working in it. `build_import_xml` with `{"company_guid": <guid>, "vouchers": <c50a.json>}`: keep the whole answer. A refusal: keep it, stop.
6. `post_import` with `company_guid` and the batch id. Record each call's start and end as UTC with milliseconds. Yes by mouse as soon as the box shows, and note that time. `approval.state` `pending`: call again after `retry_after_s`. `approved`: call again at once. Each call is its own record. Never call again after an attempt is recorded.
7. Screenshot of the whole Tally window. `verify_import` with the batch id: whole answer. `egress_log`: whole answer. Marks raw on 9001: EXPECT V up by 50.
8. Stop the relay. Steps 4 to 7 again with `c50b.json` and folder `b`.
9. Marks. Shut the company by Tally's own Shut Company. Leave the 100 vouchers, the ledger and the types: this book is a fixture for this kind of row. BK is the way back.

No one keys in Tally from step 5 to step 8.

## What to report
For each post, from `relay.jsonl`, one row:

| | a | b |
|---|---|---|
| requests before the import, their bytes and their summed `answer_s` | | |
| the import line's `sent`, `first_byte_s`, `answer_s`, `bridge_left_s` | | |
| Tally's counters, from `import-N.answer` | | |
| any `reply_lost`, `tally_closed`, `relay_error` | | |
| requests after the import, bytes, seconds | | |
| each call's start, end, seconds; Yes at | | |
| result: `created`, `attempt_recorded`, `dispatch`, code, response state | | |
| `response_prepared` line in `egress_log` | | |
| `verify_import` | | |
| Tally's screen before and after | | |

Read the numbers from the file. If an import reads over 20 s or a call over 60 s, say so in the first line.

## Decides
| what the row shows | reading on #852 |
|---|---|
| both `created` 50, reply kept, `posted_verified` 50 of 50, import under 20 s, each call under 60 s | not reproduced at this scale; the issue stays open with these times |
| as above, but a call over 60 s | the reply stayed only because no host cut it: the host limit stands, and the row says which part of the call used the time |
| `bridge_left_s` set on the import, near 20 s, `reply_lost`, Tally's answer whole | Bridge's own deadline cut the reply: a deadline sized to the voucher count, with a ceiling, is the next step |
| `import_dispatch_outcome_unknown` with `bridge_left_s` null | the reply reached the relay and Bridge dropped it: keep everything, the code is next |
| `tally_closed`, or no answer in 600 s | Tally did not answer: stop, this is another issue |

`verify_import` settles the batch in every case. Nothing is sent again.

## Stop rules
Any dialog on Tally's screen. Any refusal before an attempt is recorded. Any call with no answer in 10 minutes. V not as expected after a post: report before batch b. Education mode.

## Limits
- Silver, one user, four ledgers. The failing book was a client's Gold book, and its reads were about 100 requests and 15.9 MB. This book's master reads will be far smaller, so the call may be shorter for that reason alone.
- No one works in Tally during the row. Activity alongside a post stays unmeasured.
- The relay reads each answer whole before it forwards it, so a large read reaches Bridge late. Import answers are small. Whole-call seconds run a little over a direct call.
- A reply lost to a host cancel cannot appear here: this client does not cancel.
- The batch mix is chosen, not the client's. Its split of types was not recorded.
- Two posts only. A 10-day span was chosen to stay near the failing call's read size, from the bytes per voucher of other rows: inference.

## Code admits the trigger (master 838242cb, the build's head)
- 50 is the batch cap: `MAX_BATCH_POST_VOUCHERS`, agent_import_ledger.rs:14. `post_voucher_limit` (agent_import_post.rs:432) gives it once posting is on, which is set at agent.rs:689. The retired setting (agent.rs:645) is refused at agent.rs:1411.
- The types: `LIVE_QUALIFIED_VOUCHER_TYPES` agent_import.rs:390; `PostScope::Vouchers` admits all four, agent_import_post.rs:443.
- The legs: `Lab Vol Bank` under Bank Accounts satisfies the money leg of a Payment, Receipt and Contra, and `Lab Vol Expense` and `Lab Vol Income` the other leg (agent_import.rs:352, :358, :364).
- No numbers: a numbered voucher is refused by `require_native_numbering`, agent_import_post.rs:3100, so the batches carry none. Numberless needs Automatic with retain (`admit_numbering_for_post`, called at agent_import_post.rs:1140), hence step 3.
- **Dates:** `validate_dates` (agent_import.rs:4802) refuses a date past the host's today (`tally_host_today`, agent.rs:3916). The batches of `lab/1029-large-book-posts` are dated March 2027 and would be refused, so these are new.
- The call: tool dispatch agent.rs:3334 (`build_import_xml`) and :3372 (`post_import`); `post_import` agent_import_post.rs:636. A click made in the call's own wait carries the post in that call. One too late for the post to fit returns `approved` (`ask_and_hold` agent_import_post.rs:578, `CALL_BUDGET` 40 s, agent_import_approval.rs:46).
- The send: wire lock runtime.rs:3865, the one import send `post_probe_xml` runtime.rs:3899, under `DEFAULT_REQUEST_TIMEOUT` of 20 s (bridge-tally-transport lib.rs:95, taken at runtime.rs:276). The relay of `lab/2176-r0` saw that limit end a send live.
- The two endings: `import_dispatch_outcome_unknown` agent_import_post.rs:2100 and `response_missing` :2325. `finalize_previous_attempt_reconciliation` (:2333) settles only with a clean response on file.

## Who sends each EXPECT's request, and what each count counts (838242cb)
| step | request | sent by | decided by | the count |
|---|---|---|---|---|
| 0, 2, 7, 9 | marks, raw | the operator's curl on 9001, not Bridge | the step itself | one COMPANY row; V is that company's voucher mark (ALTVCHID), so +50 is 50 vouchers created and no other voucher change |
| 2 | `seed-bank-ledger.xml` | the operator's curl on 9001 | the step itself | `CREATED` counts masters Tally created: the one ledger |
| 3 | voucher types | `masters`, agent.rs:3360; kind arm `"voucher_types"`, runtime_masters.rs:27; sent at tally/connector.rs:263 with `render_native_voucher_type_export_request` (request.rs:411) | `kind=voucher_types` in the call | none |
| 4 | the company list | `list_companies`, agent.rs:3346 | always sent by that call | one company with this GUID |
| 5 | the build's reads | `build_import_xml`, agent.rs:3334, then agent_import.rs:1193 | the call itself | none |
| 6 | reads, then the one import | `post_import`, agent.rs:3372, then agent_import_post.rs:636; the import at runtime.rs:3899 | reached only after the approval's redemption (agent_import_post.rs:1255), inside the endpoint's dispatch lease (:1259), through `post_approved_import` (:1288, runtime.rs:3493), which records the intent before any import bytes (runtime.rs:3491) | Tally's `CREATED` counts vouchers created: 50 |
| 7 | the verification reads | `verify_import`, agent.rs:3398, then agent_import.rs:2069 | the call itself | `posted_verified` counts vouchers found by their markers: 50 of 50 |

## Keep
On `lab/852-capture-answers` with `SHA256.txt`: `relay.jsonl` of both folders, every `import-N.request` and `.answer`, every tool answer whole, call times, marks, BK's name, size and sha256, seed answer, screens by name and hash. Screens stay local. A results file with the table above. Send the sha.
