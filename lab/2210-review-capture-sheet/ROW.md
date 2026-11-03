# #2210 row note (3 Nov 2026). Not for merge.

Question: what does `verify_import` read back, with the census spanned, for a batch of 50 Journals and a single Payment that Bridge itself posted to a book nothing has touched since?
Why: #2199's move to spanned censuses turned two tests that showed a review going ahead into refusals, and six more tests still answer a spanned request with an unspanned capture. BRIDGE AMEND LAB has moved on and stays unwritten (#2210, not in scope).

Head: master at the row, green merge-queue run, artifact unsigned-bundle-smoke-windows-latest. Record `bridge_mcp.exe`'s sha256, before and after. File lines below are at master 8520f071.
Book: a NEW synthetic company, `BRIDGE REVIEW LAB`, created on Tally's screen, books from 1 April 2026, one currency, GST off, loaded alone. TallyPrime 7.2 Silver, licensed. The name is unused so far. Never any other company.
Seed, set-up only: `Cash` as Tally makes it, and one ledger `Test Expense B` under Indirect Expenses, bill-wise off. Keep Tally's own Journal and Payment voucher types. Record the company GUID, and the voucher and master marks before step 2.
Env: BRIDGE_AGENT_ENABLE_WRITES true, BRIDGE_TERMS_ACCEPTED true, BRIDGE_TALLY_HOST 127.0.0.1, BRIDGE_TALLY_PORT=9002, BRIDGE_AGENT_DATA_DIR a new lab folder.
Files here: `relay.py` sha256 44abb45146202c6132d0aa0c1d8675dfc89cb476c8538041a331946463589747 (the #2176 R0 relay, also writing each request and answer to `bodies/<sha256>`; checked on a stand-in only), `batch-vouchers.json` fce9f569…, `payment-vouchers.json` 1e0ba099…. Pass each file's list as `vouchers`, with `company_guid`. Check the sha256 of all three first.

Code admits the trigger:
- `post_import` runs with posting on: `agent.rs:3263`, `:3333`, `agent_import_post.rs:636`.
- 50 is the batch cap: `agent_import_ledger.rs:14`.
- Journal takes any balanced entries; Payment's credit needs a ledger under Cash-in-Hand or Bank, and its debit a ledger holding no money (the `voucher_type` rule in `build_import_xml`'s schema). `Cash` and `Test Expense B` meet both. D3 and WA1 posted these same shapes on 26 Sep, on 7.1.
- A batch holds Journals only or bank types only, so two builds.
- `verify_import` reads and never dispatches: `agent.rs:3358`, `agent_import.rs:1897`.

No live doubt, and why: a batch step doubt needs another voucher saved between the post's two mark reads (`agent_import_post.rs:1541-1551`, `agent_import_post_location.rs:172`). A masters doubt needs an approved name to resolve to another master after the post (`agent_import_post.rs:404-424`), and the voucher must still read back matched or the review refuses (`agent_import_ack.rs:447`). Neither happens on a plain post. The tests write the doubt themselves, as both 26 Sep captures did (`D3_BATCH_CAPTURE_PROVENANCE.md`). `acknowledge_post_review` is not run here: with no doubt it refuses, and the reads it sends are `verify_import`'s kinds (`agent_import_ack_tests.rs:1212-1217`).

Steps, one `bridge_mcp` process, nothing else touching Tally:
1. `python -I relay.py 9002 9001 OUT pass`. MARKS with curl on 9001. `list_companies`, `tally_status`.
2. `build_import_xml` with `batch-vouchers.json`, then `post_import` with its batch_id, Yes by mouse. EXPECT CREATED 50, `posted_verified`, voucher mark up by exactly 50, `batch_step` state `matched`.
3. `build_import_xml` with `payment-vouchers.json`, then `post_import`, Yes by mouse. EXPECT CREATED 1, `posted_verified`, voucher mark up by one.
4. At once, no change in Tally: `verify_import` of the batch, then of the payment. EXPECT `posted_verified` for all 51.
5. MARKS, then stop the relay.
6. Copy `agent-import-ledger.jsonl` and the two `imports/<batch_id>.xml` from the data folder, untouched. Copy nothing else from it.

Check by parsing, from `OUT/relay.jsonl` and `OUT/bodies/`:
- no `relay_error` or `tally_closed` line, and at least one `open`: a file with none is a failed row, not a clean one;
- for each `verify_import`, the request order, by kind, against `SEESESEHSHSEECSCSEEVSVSEEVSVSE`;
- each census request decoded: one request only, holding `$AlterID` bounds `(0, M]`, M the voucher mark that call read. Divided into several spans: STOP, out of scope;
- each repeat of a request has the same `answer_sha256`. A difference is a NOTE: send it, do not choose;
- the batch's import verification holds 50 VOUCHER rows and the payment's holds its one, counted by parsing elements, not by text search.

Any EXPECT missed, or a refusal: stop and send the file. Do not edit a voucher, and do not rebuild.

Keep: tool answers, `relay.jsonl`, `bodies/`, the journal, both import files, both MARKS, the company's GUID and number. Answers branch `lab/2210-review-capture-answers`, with RESULTS graded verified, partial or unverified. Narrations carry no lane name, and the journal holds no machine path: check before the push, and record any replacement.

After it, from the answers: fixtures with provenance, the two positive tests back with every assertion they had and counts from the answers, the six tests moved onto them, the two refusal tests kept.
