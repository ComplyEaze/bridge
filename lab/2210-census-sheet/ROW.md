# #2210 census row note (3 Nov 2026). Not for merge.

Question: what does `verify_import` read back, with the census spanned and sent, for a batch of 50 Journals and a single Payment that Bridge itself posted?
Why: the first #2210 row (`lab/2210-review-capture-answers` at e707b1f8) sent no census. That is the code's rule, not a fault: a window read sends no census when the book's voucher mark times 96 KiB fits the 16 MiB budget, so at a voucher mark of 170 or less. That book stood at V 52. #2210 needs the census path tested live, so this row raises the mark first.

Head: the same build as the first row, master 8520f071, run and artifact as its RESULTS give them, `bridge_mcp.exe` sha256 bb8454bc6b52fe800a61ba7faf553427ebae1948a25771a787ed0dad77ba195f. Record it before and after. File lines below are at master 8520f071.
Book: `BRIDGE REVIEW LAB` as the first row left it, GUID 63292d7c-d541-42e5-aa9d-2ab5f8b993ff, V 52, M 207, loaded alone. TallyPrime 7.2 Silver, licensed. Never any other company.
Env, as the first row: BRIDGE_AGENT_ENABLE_WRITES true, BRIDGE_TERMS_ACCEPTED true, BRIDGE_TALLY_HOST 127.0.0.1, BRIDGE_TALLY_PORT=9002, BRIDGE_AGENT_DATA_DIR a new lab folder.
Files here, check each sha256 first against `SHA256.txt`: `relay.py` (the first row's, unchanged), `setup-vouchers.utf16le.xml`, `batch-vouchers.json`, `payment-vouchers.json`, and `scripts/make_files.py`, which wrote the other three. Pass each list as `vouchers`, with `company_guid`.

Code admits the trigger:
- Every `verify_import`, and each post's own read-back, reads its window through `read_verification_window` (`agent_import.rs:2718`, called at `:1539`, `:2097`, `:2142`), with `WindowReadLimits::for_shape(ImportVerification)`, so `small_books` is `Skip` (`agent_voucher_window.rs:1075`).
- The census is skipped when `skips_census` holds (`agent_voucher_window.rs:2551`, rule `:2577`): voucher mark × 96 KiB (`:86`) within the budget, half of 32 MiB (`:34`). So a mark of 171 or more sends one.
- One census span `(0, M]` holds up to 8,192 vouchers (`census_capacity`, `:1128`; `census_spans`, `:2741`). At a mark near 250 there is one census request, spanned.
- `post_import` runs with posting on: `agent.rs:3263`, `:3333`, `agent_import_post.rs:636`. 50 is the batch cap: `agent_import_ledger.rs:14`. A batch holds Journals only or bank types only, so two builds. `verify_import` reads and never dispatches: `agent.rs:3358`, `agent_import.rs:1897`.
- `Cash` and `Test Expense B` meet the Journal and Payment rules, as both posted in the first row.

The set-up import is a NEW SHAPE only in its count: the first row's set-up Journal request, sent and answered `CREATED 1`, with 148 of its vouchers in one request in place of one (about 147 KB). Same company, ledgers, date and amounts.

Steps, one `bridge_mcp` process, nothing else touching Tally:
0. MARKS with curl on 9001: EXPECT V 52, M 207. Send `setup-vouchers.utf16le.xml` with curl on 9001, as the first row sent its set-up Journal. EXPECT `CREATED 148`, `ERRORS 0`. MARKS: EXPECT V 200, M 207. Any other figure: stop and send the file.
1. `python -I relay.py 9002 9001 OUT pass`. `list_companies`, `tally_status`.
2. `build_import_xml` with `batch-vouchers.json`, then `post_import` with its batch_id, Yes by mouse. EXPECT CREATED 50, `posted_verified`, V up by exactly 50 to 250, `batch_step` state `matched`.
3. `build_import_xml` with `payment-vouchers.json`, then `post_import`, Yes by mouse. EXPECT CREATED 1, `posted_verified`, V 251.
4. At once, no change in Tally: `verify_import` of the batch, then of the payment. EXPECT `posted_verified` for all 51.
5. MARKS, then stop the relay.
6. Copy `agent-import-ledger.jsonl` and the two `imports/<batch_id>.xml` from the data folder, untouched. Copy nothing else from it.

Check by parsing, from `OUT/relay.jsonl` and `OUT/bodies/`:
- no `relay_error` or `tally_closed` line, and at least one `open`: a file with none is a failed row, not a clean one;
- **the central EXPECT:** every window read of steps 2 to 4, the posts' own read-backs included, sends census requests, and each census request decoded holds `$AlterID` bounds `(0, M]`, M the voucher mark that call read. Two or more spans in one read: STOP, out of scope. No census in a read: STOP and send the file;
- each census answer, counted by parsing elements: the batch's window, 2 April, holds 50 rows; the payment's, 4 April, holds 1. The 149 set-up Journals are all on 1 April and must not appear;
- for each `verify_import`, the request order by kind (S status, E extent, H marks, C census, V import verification). The first row sent `SEESESEHSHSEEVSVSEEVSVSEEHSHSE`; this row is expected to add a census pair before the first V pair. Report the order seen; any other difference is a NOTE: send it, do not choose;
- each repeat of a request has the same `answer_sha256`. A difference is a NOTE;
- the batch's import verification holds 50 VOUCHER rows and the payment's holds its one, counted by parsing elements, not by text search.

Any EXPECT missed, or a refusal: stop and send the file. Do not edit a voucher, and do not rebuild.

Keep: tool answers, `relay.jsonl`, `bodies/`, the set-up answer, the journal, both import files, every MARKS, the company GUID. Answers branch `lab/2210-census-answers`, with RESULTS graded verified, partial or unverified. Narrations are the files' own (`set-up N`, `Census row N`, `Census row payment`); the journal holds no machine path: check before the push, and record any replacement.

After it, from the answers: fixtures with provenance, the two positive tests back with every assertion they had and counts from the answers, the six tests moved onto them, the two refusal tests kept.
