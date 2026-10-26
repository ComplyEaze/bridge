# 1949 (#1949, Part of #697): a lab build keeps an admitted audit read on the lab PC, for the lab's target company only (READ-ONLY in Tally)

Not for merge. Written 26 Oct 2026 from pull request #1949's own body (its scope, security notes and "Not measured: a live run on the lab computer; how the lab computer gets a lab-writes build"). `SHA256.txt` lists every file here with its bytes; check it before step 0.

In a build with the `lab-writes` feature and lab writes on, `audit_test_run` first passes the lab guards (port 9001, a well-formed `BRIDGE_LAB_TARGET_GUID`, the deny list), refuses unless the company it reads is the lab's target, and then writes the read the crate admitted to `<data dir>\lab\audit-reads\<manifest sha256>\` (`manifest.json` and `parts\<name>`) and adds one line to `<data dir>\lab\lab-manifest.jsonl`. Its answer to the caller does not change. This row is the first live run.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6.

THE COMPANY is `BRIDGE SHAPE LAB`, GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, synthetic, and listed in the merge gate's synthetic-UUID registry (`scripts/merge-gate-synthetic-uuids.json`). The same `audit_test_run` call ran on it on 25 Oct 2026 (`lab/697-0.7.0-answers`, file B1: completed in 7.8 s, 37 Tally requests). If the company list shows another GUID: STOP.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **No raw requests.** The operator sends no request file. Every read is a call of ComplyEaze Bridge's built tool, one at a time. C2 step 0 runs WITHOUT its MARKS request; its GET /status checks still run.
2. **Marks.** `tally_status` carries no marks. THE COMPANY holds several Currency masters, so a basic `ledger_masters` read is refused on it (`company_several_currency_masters`, by design). M (ALTMSTID) and V (ALTVCHID) are read with `ledger_masters` with `fields` `compliance` (first page, no `snapshot_id`): `snapshot.master_alter_id` and `snapshot.voucher_alter_id`; record the whole answer and `snapshot.reused`, which must read false. If that is refused too, use `trial_balance` from `2025-04-01` to `2026-03-31` (offset 0) and its `snapshot` instead, and record which tool gave the marks.
3. **Nothing is written to Tally:** no backup and no restore. A mark that moves is a STOP.
4. **The kept read stays on the lab PC.** The folder `<data dir>\lab\audit-reads\` holds the book's raw answers. Nothing in it is committed or copied off the PC by this row: the evidence lists its files by name, size and sha256 only. Whether this synthetic book's read becomes a committed fixture is decided later, after the privacy scan and the registry check, and is not part of this row.
5. **Tools.** With lab writes on, the tool list also carries the lab write tools (`lab_import_masters`, `lab_import_vouchers`): record the tool list at step 0 and call none of them. Only the tools these steps name are called.
6. **Who acts.** The operator alone: it loads and shuts THE COMPANY and makes the calls. No owner act is needed, apart from the build (below) if the run coordinator asks the owner for it.
7. **Grant.** Reads only, about 12 tool calls, one at a time.

Licensed mode only; Education mode = STOP.

## Build
A `lab-writes` build of master at or after the merge of #1949, built on the lab PC (no CI workflow builds the `lab-writes` feature). In a clean checkout of the commit the run coordinator names: `cargo build --locked --release --manifest-path src-tauri/Cargo.toml -p bridge --bin bridge_mcp --features lab-writes`. Record the commit (`git rev-parse HEAD`; it must contain #1949's merge) and the sha256 of `src-tauri\target\release\bridge_mcp.exe` before the first call. A commit that does not contain #1949 = STOP.

Run it over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, ONE new empty `BRIDGE_AGENT_DATA_DIR` for the row (record its full path on the lab PC only, never in the evidence), `BRIDGE_TERMS_ACCEPTED=true` (the owner allowed the lab to set it on 25 Oct 2026, the person responsible for the lab accepting the Terms of Use), `BRIDGE_AGENT_ENABLE_TAX_AUDIT=true`, `BRIDGE_LAB_WRITES=1`, `BRIDGE_LAB_TARGET_GUID=3a6bd6e1-b835-4bff-89dd-8a6af138c346`, and `BRIDGE_LAB_DENY_GUIDS`, `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_IMPORT` and `BRIDGE_AGENT_REDACTION` unset. The capture is kept only for the target company (the build checks the company's GUID against `BRIDGE_LAB_TARGET_GUID`); that only one company is loaded is LAB-COMMON C1's rule, checked at step 0.

**Timeouts.** The built tool has its own 20-second deadline for each request it sends and abandons a request that passes it (the request keeps running in Tally). Any answer holding `request_deadline_exceeded` or `response_size_limit_exceeded`, any call with no answer within 10 minutes, and any dialog on Tally's screen is a STOP: nothing is re-sent, no key is pressed, the screen is photographed, and the answer is reported whole.

## Steps
0. Load THE COMPANY alone (Select Company), then C2 step 0 without its MARKS request. `list_companies`: EXPECT exactly THE COMPANY with the GUID above. `tally_status`: record release, licence and Education mode. Marks (Difference 2): M0, V0. The tool list (`tools/list`): record it whole (Difference 5). A listing of the data folder (names and sizes): EXPECT no `lab` folder yet.
1. **The capture.** `audit_test_run` with `company_guid` `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, `from` `2025-04-01`, `to` `2025-09-30`, `test` `cash_44ab`, `cash_groups` `["Cash-in-Hand"]`, `bank_groups` `["Bank Accounts"]` (the call of 25 Oct's B1, unchanged). Time it. Record the answer whole and its `egress_log` lines.
   EXPECT: no refusal; the answer's state `completed_with_failed_book_checks` as on 25 Oct. Compare its figures and finding ids with `lab/697-0.7.0-answers` B1 and with that run's screen totals, and record every difference: the build is newer than 25 Oct's and the book may have moved since, so a difference here is a CONFOUND to report, not a finding against #1949 (this row tests the keeping, not the engine).
2. **What it kept.** List `<data dir>\lab\` recursively (names, sizes, sha256 of every file). EXPECT:
   - exactly one folder under `audit-reads\`, named by a 64-hex sha256;
   - in it `manifest.json` and a `parts\` folder of plain files, and nothing else;
   - `lab-manifest.jsonl` with exactly one line, holding `tool` `audit_test_run`, `at`, `read_id`, `company_guid` = THE COMPANY's GUID, `manifest_sha256` = the folder's name, and `parts` = the number of files in `parts\`.
   Record the sha256 of `manifest.json` and say whether it equals the folder's name. Record the folders' permissions with `icacls` (the build's own rule on Windows: they inherit the data folder's ACL; owner-only `0700` folders and `0600` files are its rule on Unix, not here). Copy nothing out of the folder.
3. `local_data_report`: record it whole. EXPECT its `lab` class to count the files and bytes of step 2's listing.
4. **The same read again.** At least two seconds after step 1, `audit_test_run` with exactly the arguments of step 1. The kept manifest carries the read's time (`created_at`, `read_at`, to the second), so a later read has another manifest sha256. EXPECT: no refusal; a SECOND folder under `audit-reads\` and a second line in `lab-manifest.jsonl`, the first folder untouched (its files' sha256 as at step 2); and the two `manifest.json` files differing only in `created_at` and `read_at` (a parsed comparison; record any other difference). A first folder changed or overwritten is the FINDING: report it whole.
5. **Not the target.** Quit the program. Start it again with the same settings except `BRIDGE_LAB_TARGET_GUID` set to `6b43e498-430c-4d5c-bfef-d32e2ab93c85` (BRIDGE PILOT LAB, which is NOT loaded), THE COMPANY still loaded alone and a NEW empty data folder. `audit_test_run` with the arguments of step 1. EXPECT a refusal `lab_audit_read_not_target` before any book request (the company check comes after one company-list read), the call's `egress_log` showing at most the company-list read, and no `lab\audit-reads\` folder in the new data folder.
6. Marks as Difference 2 says (with the settings of step 1 again): M1, V1, EXPECT equal to M0, V0. LAB-COMMON C10: shut THE COMPANY; no company loaded.

PASS = steps 0 to 6 recorded whole and every EXPECT checked. A step 1 whose figures differ from 25 Oct's, a kept folder or manifest line other than described, a `local_data_report` that does not count it, a changed first folder in step 4, or a capture for a non-target in step 5 is a FINDING for #1949: report it whole.

## Not measured by this row
A client book (this row never sets the target to one); a deny list; a write that fails part way; a Unix lab host; whether the kept read loads in the crate's `local_parity` example on the lab PC (a separate check, after the fixture decision).

## Evidence
Every tool answer whole with its size and sha256, the build's commit and exe sha256, the listings of `<data dir>\lab\` (names, sizes, sha256; paths shown relative to the data folder), the `icacls` output with user and machine names replaced by `<user>` and `<machine>`, the marks, and a notes file with UTC times and one line per step. NOT in the evidence: the kept read's `manifest.json` and `parts\` (they stay on the lab PC). Evidence on a new branch `lab/1949-audit-read-capture-answers` with `SHA256.txt`; send the SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.
