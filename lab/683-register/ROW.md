# Row 683-R1: the sales and purchase registers and gst_by_counter_ledger on the several-currency lab book, behind a relay that keeps every exchange

Not for merge. Part of #683. Lines below are at master 75224fe3. Every expectation here is unverified until the row runs.

## Question
What does the build send and get back when `sales_register`, `purchase_register` and `gst_by_counter_ledger` read one day of a book whose vouchers hold currency composites? The row keeps every request and answer whole, for fixtures. It writes nothing.

## Why
#683 stays open for the registers ("ledger_movement, voucher_presence, the registers and the desktop screen read no figure out of a composite"). A register reads its vouchers in the class-entry shape, which no capture holds on this book. `vouchers-forex-unspaced-20250401` (#2276) is the plain entry shape. The two requests share the `ALLLEDGERENTRIES.*` fetch, but no answer to the class shape has been seen (P1). The register's master listing on a book with several Currency masters is not captured either.

## Book
`BRIDGE FOREXGROUP LAB` (synthetic), GUID `1bb48be5-baba-45b2-90d8-7593244915a9`. It has two Currency masters, `₹` (base) and `$`. TallyPrime 7.2 Silver, loaded alone. This is the book of `lab/683-v72-voucher-answers` at 03940570. On 1-Apr-2025 it held eleven vouchers: 3 Sales, 3 Receipt, 2 Purchase, 2 Payment and 1 Journal, eight of them with composites (`vouchers-forex-unspaced-20250401.PROVENANCE.md`).

If the book is not on the PC, stop. If opening it shows any prompt, press no key, take a screenshot, stop, and report the prompt's words.

## Build
Merge-queue run 41690252208, head 75224fe3 (master), artifact `unsigned-bundle-smoke-windows-latest`. The master push run skipped the bundle. Record the zip's sha256 against the API digest, and `bridge_mcp.exe`'s sha256 against the bundle's smoke file, before and after the row.

Launch over stdio with these settings:
- `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_TALLY_HOST=127.0.0.1` and `BRIDGE_TALLY_PORT=9002` (the relay);
- one new, empty `BRIDGE_AGENT_DATA_DIR`;
- `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_BATCH_POST` and `BRIDGE_AGENT_REDACTION` all absent.

Client: `scripts/client.py` of `lab/2074-2214-export-cut-answers` at cc28c8f9 (sha256 `85752d4a…7498`), which refuses the write settings. Record its sha256.

## Kit
| file | what |
|---|---|
| `relay.py` | The #852 row's relay with two changes: it keeps every exchange as `exchange-N.request` / `exchange-N.answer`, and it never forwards an Import Data request (it logs `import_refused` and closes that connection). |
| `check-relay.py` | 13 checks on a stand-in Tally. `python -I check-relay.py relay.py` passes all 13 on macOS. |
| `args/none.json`, `args/day.json` | `{}`, and this GUID with `from` and `to` both 20250401. |

`SHA256.txt` lists the kit. Check it first.

## Steps
One step at a time. Every answer goes to a file. A file with no `open` line in `relay.jsonl` is a failed row, not an empty one.

0. Load the book alone and take a screenshot. `GET /status` on 9001. Then read the marks raw on 9001, not through the relay.
   EXPECT: one company, this GUID. Record V and M (the voucher and master alteration marks), Tally's date and the PC's date.
1. Run `python -I check-relay.py relay.py` on the lab PC. EXPECT: 13 PASS. Any FAIL: stop.
2. Start `python -I relay.py 9002 9001 relay\` with a new, empty `relay\`, then start the client.
3. `list_companies` with `none.json`. EXPECT: exactly this company, as `verified_tuple`.
4. `vouchers` with `day.json`. This is the control.
   - Sender: `agent_vouchers.rs:727`, the plain entry shape `render_agent_vouchers`, at `agent_read_profiles.rs:121-131`.
   - EXPECT: 11 vouchers. The 8 composite ones have their rupee figure as the entry `amount`, with `foreign` apart (#2276).
   - Record the window read's request sha256. If it is `a105626a…c4f1a`, the fixture's request, also compare its answer with `92e0e054…9369`.
5. `sales_register` with `day.json`.
   - Senders, in order:
     - the master listing (`agent_register.rs:812-819`, `ComplianceRead::WithoutBalances`). On a book with two Currency masters this still sends the balance pair (`tally/connection.rs:811-813`);
     - the census and the class-entry window (`agent_register.rs:750-760`, `VoucherReadShape::ClassEntryWildcard`, `render_agent_class_vouchers_in_span` at `agent_read_profiles.rs:156-170`);
     - the marks (`agent_register.rs:762-763`).
   - EXPECT (unverified): a refusal, code `voucher_amount_invalid`. The window parse refuses a composite (`CompositePolicy::Refuse`, `agent_voucher_parse.rs:167-176`; the error at `:1022`; the window fails at `agent_voucher_window.rs:2251`). It refuses before the register's own `register_ledger_currency_excluded` (`agent_register.rs:335`) can apply.
   - Any other answer is recorded whole and is a result, not a stop.
6. `purchase_register` with `day.json`. Same senders and the same EXPECT.
7. `gst_by_counter_ledger` with `day.json`. It reads through the same window (`read_register_window`, called at `agent_gst_by_ledger.rs:460`), so it has the same senders and the same EXPECT. Its own `register_ledger_currency_excluded` is at `agent_gst_by_ledger.rs:194`.
8. Stop the client, then the relay. Read the marks raw on 9001 again. EXPECT: V and M equal step 0's.

**Stop** if `relay.jsonl` logs `import_refused`, if Tally's window shows a dialog, if the gateway stops answering, or if step 8's marks moved. In each case press no key, take a screenshot, and report to D66 before anything else.

## Record
- **For every exchange:** its number, the connection it went on, the first request line, and the bytes and sha256 of both the request and the answer (from `relay.jsonl`).
- **For each window answer** (the `vouchers` read and each register tool's class-entry read), counted with an XML parser, never a text search:
  - the number of `VOUCHER` elements;
  - the number of `AMOUNT` elements whose text holds ` @ ` (a composite), and the same count for `VATEXPAMOUNT`;
  - for each voucher, its `VOUCHERTYPENAME`, `VOUCHERNUMBER`, and the number of `ALLLEDGERENTRIES.LIST` children.
- **For each class-entry answer:** whether it holds the voucher type class elements that the computes in `agent_voucher_type_class.rs:114-127` ask for, and how many rows carry them.
- **For each master listing:** the number of ledger rows in the master answer and in the balance answer. Also count the balance rows whose currency is not the base, and the balance rows whose opening or closing text holds ` @ `.
- **For each tool answer:** the whole JSON, and for a refusal, its `code` and `cause`.

## Decides
- **The class-entry answer's entries match #2276's layout** (the same `AMOUNT` text, symbol and one space before ` = `): the register can reuse `read_composite_amount` (`agent_voucher_parse.rs:480-524`) as captured. These answers become the register's fixtures.
- **They differ:** the register's composite parse needs its own reading of this answer. That decides the design before any code.
- **The master listing:** the counts of set-aside ledgers and their master rows. These are what the register needs to classify those ledgers by group and duty head (the filter at `tally/connection.rs:3518`).
- **A dialog anywhere:** the row stops (see Stop).
