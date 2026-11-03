# Row 2162-S3a: what does Tally store for a Sales invoice's New Ref to a bill-wise customer, with no other bill of that name?

Question (#2162 item 3; ADR 0004, #1764 item 1 and #2162 item 6): for a bill-wise customer, the build writes the party leg with one New Ref named by the invoice number. No committed capture holds a Sales invoice to a bill-wise customer. This is the control for S3b: with no other bill of that name on the ledger, does the import store one New Ref, named by the number, for the party amount? Does it then stand as the customer's one open bill of that name and amount?

Book: BRIDGE PILOT LAB (synthetic). Grant: BK before the first write, restore after the last, restore check byte-identical, as S1 and S2 did. Write down the release shown on Tally's screen.

The invoice import is the row's subject. It is the build's own request that created `BP/26-27/0010` (S2's template). Four things change, and gen.py asserts each: the number, the REMOTEID, the party ledger in its seven places, and the New Ref the build writes for a bill-wise party. The ledger is setup, seeded by direct XML under the owner's seed rule. It is never evidence of its own shape.

No Bridge build. Raw requests, one at a time, through `run.py`. It sends the header every answered sheet since S1 used, `Content-Type: text/xml; charset=utf-16`. It stops at a curl failure, and at an answer that says Unknown Request. Count every answer with an XML parser, never a text search. Close every other Bridge process first.

Files (UTF-16LE with a BOM, as sent; `SHA256.txt` lists them):
- `M-marks.xml`: the company high-water read, `lab/1578-postdated-answers:lab/1578-postdated/requests/r02-marks.xml`, byte-equal.
- `L-ledger.xml`: one ledger Create, `BRIDGE Walk-in BW` under Sundry Debtors with `ISBILLWISEON` Yes, in the All Masters envelope. Setup.
- `I-0047.xml`: the invoice, `BP/26-27/0047`, dated 1 Aug 2026, 1,051.00 to `BRIDGE Walk-in BW`, with its party leg's New Ref `BP/26-27/0047` for -1051.00.
- `R-0047.xml`: the read-back, `lab/2162-s1-sheet:lab/2162-s1/R-0010.xml` with the number changed.
- `B-bills.xml`: Bills Receivable from 1 Apr 2026 to 1 Aug 2026.
- `gen.py NUMBER` makes `I-` and `R-` for another number, and makes `L-` and `B-` again unchanged.

Steps (`python3 -I run.py <gateway URL> <label> <files>`):
1. `run.py URL s1 M-marks.xml`. Take BK. Read the Gateway's current date and write it down. If it is before 1-Aug-2026, stop: the invoice would be post-dated, which is S1's variable.
2. `run.py URL s2 R-0047.xml B-bills.xml`. If R holds a VOUCHER row, the number is in use: run `python3 -I gen.py <next number>` and repeat this step with its files.
3. `run.py URL s3 L-ledger.xml`. If the answer is not CREATED 1 with ERRORS 0, a ledger of that name exists or the create was refused: stop at step 6, and report the answer.
4. `run.py URL s4 I-0047.xml`.
5. `run.py URL s5 R-0047.xml B-bills.xml M-marks.xml`.
6. Restore BK. `run.py URL s6 M-marks.xml`, and check that answer is byte-identical to step 1's.

Record, for each answer: bytes, sha256, and the counts below. Each count says what it counts.
- s1, s5 and s6 marks: the company's `ALTVCHID` and `ALTMSTID`, the voucher and master alteration marks.
- s2 R: the number of `VOUCHER` elements in the answer.
- s2 B and s5 B: the number of `BILLFIXED` groups whose `BILLPARTY` is `BRIDGE Walk-in BW`. For each, its `BILLREF`, `BILLDATE` and the `BILLCL` that follows it.
- s3 L and s4 I: `CREATED`, `ALTERED`, `ERRORS`, `EXCEPTIONS` and `LASTVCHID`, and any `LINEERROR` text.
- s5 R:
  - the number of `VOUCHER` elements;
  - within it, the number of `ALLLEDGERENTRIES.LIST` elements whose `LEDGERNAME` is `BRIDGE Walk-in BW`;
  - within that leg, the number of `BILLALLOCATIONS.LIST` direct children, with each one's `NAME`, `BILLTYPE` and `AMOUNT`;
  - the number of `BILLALLOCATIONS.LIST` elements under every other leg;
  - `ALTERID`, `GUID` and the three `VCHGSTSTATUS` flags.

Expected:
- s2: R holds 0 `VOUCHER` elements (the number is unused). B holds 0 groups for `BRIDGE Walk-in BW` (the ledger does not exist yet).
- s3 L: CREATED 1, ERRORS 0. s5's `ALTMSTID` is above s1's.
- s4 I: CREATED 1, ERRORS 0, EXCEPTIONS 0. s5's `ALTVCHID` is above s1's.
- s5 R:
  - 1 `VOUCHER`;
  - 1 leg for `BRIDGE Walk-in BW`;
  - on it, 1 `BILLALLOCATIONS.LIST`: `NAME` `BP/26-27/0047`, `BILLTYPE` New Ref, `AMOUNT` -1051.00;
  - 0 under every other leg;
  - included Yes, uncertain No, excluded No.
- s5 B: 1 group for `BRIDGE Walk-in BW`: `BILLREF` `BP/26-27/0047`, dated 1-Aug-26, `BILLCL` -1051.00.
- s6: the marks answer byte-identical to s1's.

An empty s5 R means the filter did not find the voucher: report the number sent and the number on the screen. That is not a finding about the bill.

Decides:
- **As expected:** the read-back's allocation check holds for a bill-wise customer on a book with no other bill of that name. S3b then changes one thing at a time: the same bill name already open on the ledger from an earlier-year Sales invoice, an opening bill, a Journal, and a settled bill.
- **Any other allocation:** two rows, another type or name, or an amount not the party's. The check at agent_import_invoice.rs:1476 would refuse every bill-wise invoice. Bill-wise customers stay refused, and S3b is not needed for that decision.
- **No bill, or a bill the B report does not show:** stop, and send the answers to D66 before any code.

Code admits the trigger. Each request's sender at file:line, and the branch that decides whether it is sent. Lines are at master a0d0ead0 unless marked.
- **I, the invoice:**
  - Rendered by `render_sales_invoice_xml` (agent_import_invoice.rs:2631), called from agent_import.rs:5445 for the native post `native_post_request` (agent_import_post.rs:3063, sent at :945).
  - The New Ref is written by `leg(party_entry, observed.party_bill_wise)` (agent_import_invoice.rs:2661). `party_bill_wise` is the catalogue's `ISBILLWISEON` for the party, read at agent_import_invoice.rs:2244.
  - Whether a Sales post is sent at all is `LIVE_QUALIFIED_VOUCHER_TYPES` (agent_import.rs:390). It does not list Sales, so no release sends this request. That is why the row sends it raw, as #2162 item 3 asks.
- **R, the read-back:**
  - Rendered by `render_invoice_readback_request` (agent_import_invoice_wire.rs:484 at e4440334, #2307). Sent by `read_back_sales_invoice` (agent_import_invoice.rs:1677) from agent_import.rs:2380, when `invoice_readback_due` (agent_import.rs:2374) finds an invoice in the batch.
  - Its answer feeds the allocation check `allocation_ok` (agent_import_invoice.rs:1476). The branch `leg.ledger == party && observed.party_bill_wise` (:1477) requires exactly one New Ref named by the number, for the party amount.
- **B, Bills Receivable:** rendered by `render_native_bills_request` (bridge-tally-protocol native_outstandings/request.rs:131, byte-equal). Sent by the outstandings read at tally/runtime.rs:4136, from the book's start (1 Apr 2026 on this book) to the as-of date, after the as-of is admitted (:4124).
- **M, the marks:** the company high-water read, `render_agent_company_high_water` (agent_read_profiles.rs:531), sent as `company_high_water_read` around every post.
- **L, the ledger (setup):** `LedgerCreate::render_message` (bridge-tally-protocol ledger_create.rs:36) inside `render_import_envelope` (import_envelope.rs:27) with `ImportReport::AllMasters`, byte-equal for these fields. It is a seed, not a claim about which Bridge path would create this ledger.

Row: lab/2162-s3a/
