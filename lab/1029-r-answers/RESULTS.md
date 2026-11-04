# Row 1029-R: five approved bank batches posted through Bridge on BRIDGE LARGE LAB: results

**First line: 0 of 5 refused read-backs. Every post was approved by the lab's mouse click on the box's own Yes, posted in the same call, and came back `posted_verified` with `created` equal to the batch's count. Every `verify_import` read `posted_verified` n of n. Marks went V 30,030 to 30,168 in steps of 3, 10, 25, 50 and 50, with M 211 throughout. No call took over 60 s (the longest was 43.305 s). No dialog showed on Tally's screen, and no stop rule fired.** (verified)

Sheet `lab/1029-r/ROW.md` at aa811b9b (SHA256.txt 9 of 9 equal, checked by sha256). The five batches were remade with `make-batches.py` and are byte-equal to the sheet's. Each batch's sha256 was checked again when it was used. Every step and stamp is in `notes.txt` (UTC).

## Count (the sheet's What counts)

| | r03 | r10 | r25 | r50a | r50b |
|---|---|---|---|---|---|
| refused read-back | no | no | no | no | no |
| refusal before an attempt was recorded | none | none | none | none | none |
| `import_dispatch_outcome_unknown` | none | none | none | none | none |
| call over 60 s | no | no | no | no | no |
| deadline or refused part after the send, from `egress_log` | none | none | none | none | none |

**Reading on #1029 item 3 (the sheet's Decides, first row): 0 of 5 on this book. Item 3 is measured.** (verified; the reading is the sheet's)

## Record

| | r03 | r10 | r25 | r50a | r50b |
|---|---|---|---|---|---|
| build: batch id | bridge-e89c3f7e-c946-42af-9949-e8333562bee4 | bridge-27187d2c-6f8a-4b3a-8790-aa065e519499 | bridge-53c11cc1-04a4-4773-91a3-6c2f7dac3bb0 | bridge-ed60d374-3506-432e-b24e-1d73fb9aed28 | bridge-a8705431-49c2-4f91-8857-0b991b2858a9 |
| build: `voucher_count`, seconds | 3, 11.416 s | 10, 15.044 s | 25, 15.509 s | 50, 14.933 s | 50, 18.487 s |
| build: warnings | 7 (the same seven texts each time, whole in the answer); `hsn_warnings` [], `statement_ledger_warnings` [], `suspense_lines` count 0 | the same | the same | the same | the same |
| build: total debit = total credit | 1,521 | 5,525 | 16,875 | 46,875 | 64,375 |
| call 1: start (UTC) | 07:33:13.362 | 07:36:08.791 | 07:37:49.771 | 07:39:35.409 | 07:41:09.490 |
| call 1: end (answer written), seconds | 07:33:45.240, 31.493 s | 07:36:46.436, 37.634 s | 07:38:33.512, 43.305 s | 07:40:17.067, 41.261 s | 07:41:52.379, 42.535 s |
| box shown at; Yes at (seen to click) | 07:33:26.861; 07:33:28.348 (1.5 s) | 07:36:24.326; 07:36:25.455 (1.1 s) | 07:38:08.626; 07:38:09.784 (1.2 s) | 07:39:51.541; 07:39:52.791 (1.3 s) | 07:41:25.383; 07:41:26.752 (1.4 s) |
| box title; box check | 'post 3 vouchers?'; 6 of 6 PASS | 'post 10 vouchers?'; 6 of 6 PASS | 'post 25 vouchers?'; 6 of 6 PASS | 'post 50 vouchers?'; 6 of 6 PASS | 'post 50 vouchers?'; 6 of 6 PASS |
| box types line | 1 Payment, 2 Receipt | 1 Contra, 4 Payment, 5 Receipt | 2 Contra, 10 Payment, 13 Receipt | 5 Contra, 20 Payment, 25 Receipt | 5 Contra, 20 Payment, 25 Receipt |
| calls per batch | 1 | 1 | 1 | 1 | 1 |
| `approval.state` | no `approval` member (the click came inside the call's wait; the post went in that call) | the same | the same | the same | the same |
| `attempt_recorded`; `dispatch.state`; `response_state`; `resent` | no `attempt_recorded` member; posted_verified; response_clean; false | the same | the same | the same | the same |
| Tally's counters | created 3; altered, cancelled, deleted, errors, exceptions, ignored, line_error_count 0 | created 10; every other 0 | created 25; every other 0 | created 50; every other 0 | created 50; every other 0 |
| `error` | none | none | none | none | none |
| the post's own counts; `unrelated_duplicates_in_window`; `post_location` | posted_verified 3, every other 0; []; target_only | posted_verified 10; []; target_only | posted_verified 25; []; target_only | posted_verified 50; []; target_only | posted_verified 50; []; target_only |
| `egress_log`: the post's `response_prepared` present; its request trail | yes; 200 sent, 0 failed, every outcome answered | yes; 212 sent, 0 failed, all answered | yes; 212 sent, 0 failed, all answered | yes; 212 sent, 0 failed, all answered | yes; 212 sent, 0 failed, all answered |
| `egress_log`: records returned, `omitted_for_size` | 12, absent | 20, absent | 25, 3 | 25, 11 | 25, 19 |
| `verify_import`: status counts, seconds | posted_verified 3, every other 0; 9.775 s | posted_verified 10; 11.357 s | posted_verified 25; 13.087 s | posted_verified 50; 11.576 s | posted_verified 50; 15.241 s |
| marks after: V, M | 30,033, 211 | 30,043, 211 | 30,068, 211 | 30,118, 211 | 30,168, 211 |
| Tally's screen after | the Gateway, BRIDGE LARGE LAB alone, no dialog (`shots/r03-after.png`) | the same (byte-equal capture) | the same (byte-equal capture) | the same (byte-equal capture) | the same (byte-equal capture) |

All times are 4 Nov 2026 UTC. Start is when the request reached the client's inbox, and the seconds are the client's own round trip. The client checks its inbox every 0.5 s. The `egress_log` calls asked for `limit` 256. Its size bound cut the oldest lines from r25 on (the `omitted_for_size` row). Each batch's own post line was among those returned. The one "refus" match in r10's log is the field name `result.refusal_reason` in tally_status's prepared-field list, not a refusal. (verified)

## The rest of the Record

- **Step 0** (verified):
  - Tally showed Select Company with no company loaded. BRIDGE LARGE LAB was loaded alone with no prompt.
  - Marks: one COMPANY row, GUID 2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab, **V 30,030, M 211** (EXPECT met).
  - TallyPrime release 7.2, Silver, education mode false (from `tally_status`). Tally's Current Date: Wednesday 31-Mar-2027. The PC's date: 4-Nov-2026, which is on or after 7-Oct-2026.
- **BK** (verified):
  - Tally's own Backup into a new empty folder. The one prompt, "Accept ?", was answered y.
  - Result: one file, `TDBK1800_<company number>.001`, 9,111,833 B, sha256 798342cb72ed569717a98205874212c7a963763d9413a6b199ae57fc7cf5a55e.
  - Marks after: V 30,030, M 211.
- **Step 2** (verified):
  - `list_companies`: this GUID, `books_from` 20260401, verified_tuple.
  - `masters` with `kind=voucher_types` (24 types): Payment, Receipt and Contra were each `series_numbering.method` automatic, `sub_method` auto_retain. No type was altered.
  - Marks: V 30,030, M 211.
- **Step 4** (verified):
  - Marks: **V 30,168, M 211** (EXPECT met).
  - `bridge_mcp` was stopped by the client (exit code 0).
  - Shut Company by Tally's own menu: a click on the title band, Alt+K, Down ×4 to "SHut", Enter, then Enter on BRIDGE LARGE LAB, the only row. Tally then showed Select Company with no company loaded.
  - One tally.exe (pid 1200) owns 9001 (`evidence/s4-c2-*`).
- **The client** (verified):
  - It is the 852-C1 inbox stdio client (`run/client.py`). It never cancels and has no call limit of its own. The sheet's 10-minute stop was timed by the lab and never came near.
  - Launch: `BRIDGE_AGENT_ENABLE_WRITES=true`, `BRIDGE_TERMS_ACCEPTED=true`, host 127.0.0.1, port 9001 (no relay), and one new empty data folder. `BRIDGE_AGENT_ENABLE_BATCH_POST` was absent and `BRIDGE_AGENT_REDACTION` unset.
  - One `bridge_mcp` process ran for the whole row (bridge-tally 0.7.0).
- **The build** (verified):
  - BUILD-M is CI run <CI run id>, artifact <CI artifact id>. Its zip sha256 1940bd9736edc2aa76d811b3bcd0d6229ee69b5e01f5f59321a46b6ca28f691a equals the digest.
  - `bridge_mcp.exe` was 9bda7144f54e5ef68477b5ca30be33a17759de2e1ad756f840785c67bcab2dc0 before the first call and after the last, equal to `mcpb-smoke.json`'s.
- **Who acted** (verified):
  - The lab answered each box itself. `box.ps1` watched for the box, and `boxcheck.py` read its text, wrote Yes only when the text named BRIDGE LARGE LAB, this GUID, the batch's id and its voucher count, and otherwise wrote No.
  - `box.ps1` then clicked the box's own Yes button by mouse (SetCursorPos and mouse_event). No key was pressed.
  - Nobody keyed in Tally from the first build to the last check.
- **A lab slip, not a product result** (verified):
  - After r10's build, the lab's `batch.sh` halted on its own read of the count: a carriage return in the helper's output, not anything in the answer.
  - It was fixed, and r10's post used that build's saved answer. Nothing was rebuilt or resent.
  - Separately, the tools list's answer file is named `000-000-tools.json` because its label carried its own number.

## Kept on the lab PC (the run lead's standing rules)

No captured byte is altered. A captured file is pushed whole unless it holds the company number, or a digit run of 10 or more outside VOUCHERKEY. Each such file stays on the lab PC, listed in `kept-local.txt` by size and sha256. Screens stay local, listed in `SCREENSHOTS.txt`. Each kept file's structure is below with no values (`evidence/kept-structure.txt`). (verified)

- **The tool answers that carry the company number** (18): `list_companies`, `tally_status`, `masters` and, for each batch, `build_import_xml`, `post_import` and `verify_import`.
  - Each is JSON with top-level keys arguments, call_sent_utc, response, round_trip_seconds, server_info, server_pid and tool. The result keys are in `evidence/kept-structure.txt`.
  - Every field the Record table quotes was read from these files by the lab's `fields.py` (pushed).
  - Each `post_import` and `verify_import` result member is also hashed in `evidence/kept-canonical.txt`, which gives its size and sha256. The hash is taken as `json.dumps(obj, sort_keys=True, separators=(",",":"), ensure_ascii=False).encode("utf-8")`, leaving out company, evidence, read_at and truncated. Those bytes are written to no file.
- **The five `egress_log` answers, the tools list, `initialize`, the marks reads** (with their headers) and **the data folder's `agent-egress.jsonl` and `terms-acceptance.jsonl`** hold neither the company number nor a 10+ digit run. They are pushed whole.
- **The data folder's import ledger** (`agent/agent-import-ledger.jsonl`): 35 lines, made up of 15 verification_status, 5 dispatch_intent, 5 dispatch_response, 5 post_span_verdict and 5 build records. It carries the company number.
- **The 15 saved proofs** (`agent/imports/*.proof.*.json`, three per batch) carry the company number, so they stay local. Their markdown twins and the other batch files without it are pushed.
- **Two saved import files** (`agent/imports/bridge-ed60d374-….xml` and `bridge-a8705431-….xml`; r50a's and r50b's): XML with ENVELOPE/HEADER/TALLYREQUEST and ENVELOPE/BODY/IMPORTDATA. Each holds a voucher REMOTEID whose last UUID group is all digits (12), and that run appears again in a narration marker, outside VOUCHERKEY.
- **The five box logs** (`box/<batch>-p1-box.txt`, 40 to 51 lines each) carry the box's "Company number" line. The checks `box/<batch>-p1-check.out` (title, Create line, GUID, batch, types) and the answer files are pushed.

## The book now

BRIDGE LARGE LAB holds V 30,168, M 211, with the 138 vouchers of this row (1 to 7 Oct 2026) kept as a fixture. It was not restored; BK is the way back on the run lead's word. It was left shut, with one tally.exe on 9001.

The four desktop-client connector servers stayed running: no call was made to them from this sitting, and their calls were not measured.
