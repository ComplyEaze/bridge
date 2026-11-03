# Row 683-B: the three registers through a build of #2323, on the several-currency lab book, behind the keeping relay

Not for merge. Part of #683. **Read only:** no write, and the relay refuses any import. Lines are at master a4b4adff.

## Question
The same row as `lab/683-register-answers` (32f896c7): the same book, day, relay, client and calls, now through a build that holds #2323. Do the three registers read the day's foreign-currency vouchers at their rupee figures, as #2323's replay test says? Does each register call send the requests that test replays, in that order? Until this row runs, #2323 rests on the earlier build's refusal and on replayed captures.

## Book
`BRIDGE FOREXGROUP LAB` (synthetic), GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, TallyPrime 7.2 Silver.

The lab's last state note for it (`lab/683-register-answers` RESULTS.md, "The book now"):
- V 11, M 214;
- Tally's current date 1-Apr-2025, period 1-Apr-25 to 31-Mar-26;
- eleven vouchers on 1-Apr-2025, eight with composites.

No row has written to it since.

## Build
Merge-queue run 41717087257 (`merge_group`, head a4b4adff, which is master after #2323), artifact `unsigned-bundle-smoke-windows-latest`, id 13270783263, digest sha256 `f8a6388b2271696a0fc4ae1d2cfacf03909325f65859bcbe96129c9b23fe43e4`.
- The master push run at a4b4adff skips the bundle job (`Determine bundle scope`), so it has no build to take.
- Record the zip's sha256 against that digest. Record the sha256 of `bridge-tally.mcpb`, `bridge_mcp.exe` and `pdfium.dll` against the bundle's smoke file, before and after the row.
- The server should report a version; record it.

Launch exactly as the earlier row did (`lab/683-register-answers` `run/launch-env.json`):
- `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9002` (the relay);
- one new, empty `BRIDGE_AGENT_DATA_DIR`;
- `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_BATCH_POST` and `BRIDGE_AGENT_REDACTION` all absent.

Client: `scripts/client.py` of `lab/2074-2214-export-cut-answers` at cc28c8f9 (sha256 `85752d4a…7498`). It refuses the write settings. Record its sha256.

## Kit
`SHA256.txt` lists the kit. Check it first.

| file | what |
|---|---|
| `relay.py`, `check-relay.py`, `args/none.json`, `args/day.json` | `lab/683-register-sheet` (0adb5b76), `lab/683-register/`, byte-equal. The relay keeps every exchange and never forwards an Import Data request. |
| `marks.utf16le.xml` | The marks request of `lab/683-register-answers`, byte-equal. It is also the request the build sends as its marks read (sha256 `5b447d03…b4b7`). Send it raw on 9001 with that row's `raw.py` (sha256 `4ca5215d…9534`). |
| `count.py` | `python3 -I count.py <run folder> <relay folder>`. Prints shapes and counts from the answers and `relay.jsonl` only (see Record). |

Record any other Bridge process. None of them may take a call.

## Prompts this row expects
**None.** Every one of these is a stop, with no key pressed (see Stop):
- **Loading the book:** Select Company, the book by its name filter, then Enter. The earlier row loaded it this way with no prompt.
  - If another company is loaded, shut it first, with no change to it.
  - Any question on shutting or loading is a prompt.
- **Bringing Tally to the front:** use the running window only. In the earlier row, a desktop "open application" call started a second `tally.exe` (`lab/683-register-answers` `evidence/notes.txt`). If a second one starts, stop it by its PID before any request, as that row did, and record it.
- **During the calls:** reads alone have never raised a Tally dialog on this book. A Windows prompt about the unsigned `bridge_mcp.exe` is also a stop. The earlier row's build of the same artifact name raised none.

## Steps
One step at a time. Every answer goes to a file. A call with no exchange in `relay.jsonl` is a failed row, not an empty one.

0. Load the book alone, as above, and take a screenshot. Send `GET /status` on 9001. Then run `raw.py m0`.
   EXPECT: one company, this GUID, V 11, M 214. Record Tally's date and period, and the PC's date. Any other V or M: stop, since the book has changed since its last note.
1. Run `python -I check-relay.py relay.py`. EXPECT: 13 PASS. Any FAIL: stop.
2. Start `python -I relay.py 9002 9001 relay\` with a new, empty `relay\`, then the client. Send `tools/list` first; it sends nothing to Tally.
3. `list_companies` with `none.json`. EXPECT: this company, `verified_tuple`.
4. `vouchers` with `day.json`, the control. EXPECT: 11 vouchers, `complete`, 8 with a `foreign` amount, as in the earlier row.
5. `sales_register` with `day.json`.
6. `purchase_register` with `day.json`.
7. `gst_by_counter_ledger` with `day.json`.
8. Stop the client, then the relay. Run `raw.py m8`. EXPECT: V 11, M 214, and an answer byte-identical to m0's, as in the earlier row.

Then run `python3 -I count.py run relay`.

## Expected, steps 5 to 7
From `a_window_of_foreign_currency_composites_is_read_at_its_rupee_figures_by_all_three_tools` (agent_register_server_tests.rs:1282). That test replays one register call on this book from the earlier row's captures. Unverified until this row runs.
- **Each register call:**
  - `isError` false, `state` `complete`, no `withheld_total`;
  - `vouchers_observed` 11, `ledger_masters_observed` 8.

  The book keeps no GST ledger, so no voucher is a register row:
  - `sales_register`: `total` 0, and `sales_vouchers_without_duties_taxes_entry.total` 3;
  - `purchase_register`: `total` 0, and `purchase_vouchers_without_duties_taxes_entry.total` 2;
  - `gst_by_counter_ledger`: `vouchers_without_gst_entry` 11.
- **The requests of each register call:** 140 exchanges, in the test's order and with its request fingerprints. `count.py` prints them as letters and says whether they equal it.
  - The first 64 are the earlier row's exchanges 49 to 112.
  - Then the base read for the composites (22).
  - Then the marks and the masters again (54).
- **No** `import_refused` and no `reply_lost`.

Any other answer is a result, not a stop: record it in full.

## Stop
Press no key, take a screenshot, and report before anything else is pressed or sent, if:
- any prompt appears (see Prompts this row expects);
- `relay.jsonl` logs `import_refused`;
- the gateway stops answering;
- step 0's marks are not V 11, M 214;
- step 8's marks moved.

## Record
- **Shapes and counts only, all of them printed by `count.py`:**
  - the relay's exchange, import-refusal, reply-lost and connection counts;
  - each call's tool, `isError`, seconds, and the code and cause of a refusal;
  - each call's `state`, `reason`, `total`, `withheld_total`, `vouchers_observed`, `ledger_masters_observed` and the without-entry totals;
  - for `vouchers`, the number of items and how many carry a `foreign` amount;
  - for each call, its exchange count and request letters, and for each register, whether its letters equal the replayed call's.
- **Also:**
  - m0 and m8's V and M, and whether m8 is byte-identical to m0;
  - the build and client sha256 above;
  - the Tally and PC dates.
- **Kept on the lab PC only, by sha256 and bytes in `kept-local.txt`:** the relay exchanges, the tool answers and the screenshots.

## Decides
- **All three as expected, sequences equal:** #2323 holds through a build on this book. The register rows of #683's progress comment become measured at tool level, for one book, one day and one currency.
- **A register refuses or answers `partial`:** record its code, reason and letters. The first leg that differs from the replayed call says where the build parts from the test.
- **Sequences differ but the answers are as expected:** record the first difference. The replay test's order then needs the build's order before the captures can serve as its fixtures.
