# #2210 hand-import row note (3 Nov 2026). Not for merge.

Question: what does `verify_import` read back, with the census spanned and sent, for a batch of 50 Journals that Bridge built and that was then imported from its saved file without Bridge, with an unmarked copy of one of its vouchers entered after it?
Why: #2210's second acceptance line says no test serves a census answer captured for another request. One test still does: `a_hand_imported_batch_with_a_duplicate_reads_unverified_in_the_markdown` (`src-tauri/src/agent_import_ack_tests.rs:1923` at 94bf10d8, #2308) answers the spanned census request with the D3 capture's unspanned census of 26 Sep (`:1924`), with a copy of one voucher derived in memory (`with_d3_005_copy`, `:1886`). The census row's capture cannot replace it: its batch was a native post, whose read-back has no marker in a narration (`lab/2210-census-answers` at 8ce3397b, Fix-forward), so nothing there is matched by tag. This row makes the case the test describes, on the same book.

Head: the same build as the census row, master 8520f071, run and artifact as its RESULTS give them, `bridge_mcp.exe` sha256 bb8454bc6b52fe800a61ba7faf553427ebae1948a25771a787ed0dad77ba195f. Record it before and after. File lines below are at master 8520f071 unless a commit is named.
Book: `BRIDGE REVIEW LAB` as the census row left it, its GUID as the census row's RESULTS give it, V 251, M 207, loaded alone. TallyPrime 7.2 Silver, licensed. Never any other company.
Env: BRIDGE_AGENT_ENABLE_IMPORT true, BRIDGE_TERMS_ACCEPTED true, BRIDGE_TALLY_HOST 127.0.0.1, BRIDGE_TALLY_PORT=9002, BRIDGE_AGENT_DATA_DIR a new lab folder. BRIDGE_AGENT_ENABLE_WRITES is NOT set: this row posts nothing through Bridge.
Files here, check each sha256 first against `SHA256.txt`: `relay.py` (the census row's, unchanged), `batch-vouchers.json`, `copy-voucher.utf16le.xml`, and `scripts/make_files.py`, which wrote the other two.

Code admits the trigger:
- `build_import_xml` needs only the import setting, which `BRIDGE_AGENT_ENABLE_IMPORT` gives without posting (`agent.rs:589`, checked at `:3296`). With posting off, `post_import` cannot send (`agent.rs:3333`, `agent_import_post.rs:636`), so the journal holds no dispatch record.
- The build records the book's voucher mark before the import (`agent_import.rs:1447`). A tagged row at or below it is not attributed (`agent_import_verification.rs:529`). So import the file only after the build, and enter the copy after the import.
- The saved file's narrations carry the batch's `[BRIDGE:…]` tag (`agent_import.rs:5295`). A native post sends them without it (`agent_import_post.rs:824`). So the batch must go in from the saved file, byte for byte, not by `post_import`.
- With no dispatch record there is no pre-POST mark, so `verify_import` attributes by tag (`agent_import.rs:2240`; `Attribution::Tag`, `agent_import_verification.rs:292`).
- A Journal is keyed by content, not by number or narration (`agent_import_verification.rs:263`). A row of the batch's content that no tag claims is a duplicate (`batch_duplicate_sets`, `:1216`; `duplicates`, `:1422`). The copy has H-005's date, type and entries, and no tag.
- Every `verify_import` reads its window through `read_verification_window` (`agent_import.rs:2718`; `verify_import` at `:1897` reaches it at `:2097`). That read sends a census at a voucher mark of 171 or more (`skips_census`, `agent_voucher_window.rs:2577`; `small_books` `Skip`, `:1075`). One span `(0, M]` covers it (`census_spans`, `:2741`).
- `Cash` and `Test Expense B` meet the Journal rules: both posted in the census row.

Two imports go to Tally with curl on 9001, as the census row sent its set-up:
- the saved file, `DATA/imports/<batch_id>.xml`, exactly as `build_import_xml` wrote it. It is the request a person's import of that file makes, sent with curl so that Tally's counters come back in the answer (P7). 50 Journals in the shape the D3 slice imported, on this company.
- `copy-voucher.utf16le.xml`: the census row's set-up request shape with one voucher, the count the first #2210 row's set-up Journal had. It is dated 6 April with 5.00, no REMOTEID, narration `Hand row 5`.

Counts below: **V** and **M** are the `ALTVCHID` and `ALTMSTID` of the one COMPANY row in a MARKS answer; **CREATED** and **ERRORS** are the counters in Tally's answer to an import; **rows** are VOUCHER elements carrying a DATE, counted by parsing (each answer's dateless VOUCHER element is the header's counter); **exchanges** are request lines in `relay.jsonl` whose `utc` falls within one tool call, by its start and end in the client log.

Steps, one `bridge_mcp` process; nothing else touches Tally except the two curl imports and the MARKS reads:
0. MARKS (sent by the lab with curl on 9001, as the census row did): EXPECT V 251, M 207. Any other figure: stop and send the file.
1. `python -I relay.py 9002 9001 OUT pass`. `list_companies`, `tally_status`.
2. `build_import_xml` with `batch-vouchers.json` and `company_guid`. EXPECT a batch_id, `voucher_count` 50 (the vouchers in the saved file, `agent_import.rs:1730`), and no refusal. Record the batch_id and the saved file's sha256. Do not call `post_import`.
3. MARKS: EXPECT V 251, as the build writes nothing. Send `DATA/imports/<batch_id>.xml` with curl on 9001. EXPECT `CREATED 50`, `ERRORS 0`. MARKS: EXPECT V 301.
4. Send `copy-voucher.utf16le.xml` with curl on 9001. EXPECT `CREATED 1`, `ERRORS 0`. MARKS: EXPECT V 302.
5. At once, no other change in Tally: `verify_import` of the batch. EXPECT, in its structured result:
   - `counts.posted_verified` 50, the batch's rows whose status is `posted_verified`;
   - no `dispatch` key;
   - `duplicates`, an array, holds exactly one entry, with kind `accounting_fingerprint` and two ids in its `voucher_ids`;
   - `verification_status` `verification_incomplete`.
6. MARKS, then stop the relay.
7. Copy `agent-import-ledger.jsonl` and the whole `imports/` folder (the saved file and the JSON and Markdown proofs) from the data folder, untouched. Copy nothing else from it.

Who sends each request of the verify (step 5), and what decides it:
- **S and E:** every read Bridge sends is bracketed by a status probe and the company identity read, before and after it, and is itself sent as a pair (`fetch_admitted_agent_read`, `tally/runtime.rs:195`: opening bracket `:201`, the pair `:207`, closing bracket `:211`). These are always sent. The call opens with the import profile (`observe_import_profile`, `agent_import.rs:2084`) and the company check (`verified_company`, `:2085`).
- **H, the marks:** `read_marks` (`agent_voucher_window.rs:2501`, request `:2511`). It is sent because the verify passes no known marks (`known_marks: None`, `agent_import.rs:2101`; branch `agent_voucher_window.rs:2540`).
- **C, the census:** `census_window` (`agent_voucher_window.rs:2601`, request `voucher_census_read` at `:2625`). It is sent unless `skips_census` holds (branch `:2551`, rule `:2577`); at V 302 it does not hold.
- **V, the import verification:** the window's part read (`voucher_window_part_read`, `agent_voucher_window.rs:2174`, sent at `:2181`), always. A second V is the corroborating replay (`agent_import.rs:2141`), sent when `SecondRead::of` returns `Replay` (`:2135`; `agent_voucher_window.rs:429`). It replays the parts only, with no marks or census.

Check by parsing, from `OUT/relay.jsonl` and `OUT/bodies/`:
- no `relay_error` or `tally_closed` line, and at least one `open`: a file with none is a failed row, not a clean one;
- **the central EXPECT:** the verify's census requests (C above), each decoded, hold `$AlterID` bounds `> 0 AND <= 302` and the date 6 April. Two or more census requests that differ in their bounds: STOP, out of scope. No census: STOP and send the file. The build's own census (`read_verification_window`, called at `agent_import.rs:1539`, the same branch) is expected at `<= 251`;
- the verify's census answer holds 51 rows on 6 April, and so does its import verification answer. Of the 51 narrations in the V answer, 50 end in the batch's `[BRIDGE:…]` tag and one, `Hand row 5`, has none. No row is dated 1, 2 or 4 April;
- for each of the 51 V rows, the `REMOTEID` attribute against its `GUID`, and for the 50 tagged rows against the saved file's REMOTEIDs. Report what Tally kept; this is a NOTE, not an EXPECT;
- the verify's request order by kind, as the senders above. It is expected to be `SEESESEHSHSEECSCSEEVSVSEEVSVSE`, 30 exchanges: the order the D3 batch's verify sent on 26 Sep with no dispatch record. The census row's calls, which had a dispatch record, sent `EHSHSE` more at the end. Report the order seen; a difference is a NOTE: send it, do not choose;
- each request sent twice in a row has the same `answer_sha256` both times. A difference is a NOTE;
- the Markdown proof holds the lines `this report does not confirm posting`, `- Verification status: `verification_incomplete``, and `- Duplicates in this batch: 1`.

Any EXPECT missed, or a refusal: stop and send the file. Do not edit or delete a voucher, do not rebuild, and do not import either file twice.

Keep: tool answers, `relay.jsonl`, `bodies/`, both curl answers, every MARKS, the journal, the `imports/` folder, the company GUID. Answers branch `lab/2210-hand-import-answers`, with RESULTS graded verified, partial or unverified. Narrations are the files' own (`Hand row N`); the journal holds no machine path: check before the push, and record any replacement.

After it, from the answers: the duplicate test replays this capture's extent, marks, census and verification, with the copy as Tally returned it (no longer derived) and this row's journal. The test keeps every assertion it has, and its requests are pinned.

The book afterwards: V 302, holding the 50 Journals and the copy on 6 April beside the census row's vouchers. It is not restored.
