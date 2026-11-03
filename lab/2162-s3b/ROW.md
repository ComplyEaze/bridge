# Row 2162-S3b: what does Tally store for a Sales invoice's New Ref when the customer already has a bill of that name?

Question (#2162 items 3 and 6; ADR 0004 checklist items 1 and 6). S3a was the control: with no other bill of that name, the invoice stored one New Ref named by its number, for the party amount (`lab/2162-s3a-answers` at 4dff8215, PARTIAL). S3b changes one thing per sub-row: before the invoice, its customer already has a bill named by the invoice's number. Each sub-row has its own customer, so the sub-rows do not interact.
- **b1:** the bill comes from a Journal's New Ref.
- **b2:** the bill comes from a Journal's New Ref, settled in full by a Receipt.
- **b3:** the bill is an opening bill.

The earlier-year Sales invoice is not in this row; it moved to item 14.

For each sub-row, record which of these happens:
- (i) a second bill of that name;
- (ii) one bill holding both amounts;
- (iii) the New Ref stored as Agst Ref;
- (iv) the import refused.

Book: BRIDGE PILOT LAB (synthetic).
- One BK before the first write and one restore after the last, with the restore checked byte-identical.
- If a sub-row stops at its import (see Stop), restore BK, check that M's answer equals s1's, then run s3, the keyed setup and s4 again for the sub-rows still to run, and go on.
- Write down the release shown on Tally's screen.

The invoice imports are the row's subject. Each is the build's own request that created `BP/26-27/0010`, with four things changed: the number, the REMOTEID, the party in its seven places, and the New Ref the build writes for a bill-wise party. gen.py asserts each change. `I-0048.xml` equals S3a's `I-0047.xml` with its number, REMOTEID and party substituted.

Setup:
- The four ledgers are seeded by direct XML, as S3a's ledger was.
- The Journals, the Receipt and the opening bill are keyed on Tally's screen. Bridge writes none of those shapes on master, so the seed rule does not cover them.
- Seeded and keyed rows set the test up; they are never evidence of their own shape. Each keyed entry is read back natively (s4) before any import. If one does not hold its bill, that sub-row stops there.

No Bridge build. Raw requests, one at a time, through `run.py`.
- It sends `Content-Type: text/xml; charset=utf-16`.
- It stops at a curl failure, and at an answer that says Unknown Request. It decodes each answer as UTF-16 little-endian whether or not it starts with a byte-order mark (Tally's answers have none), and also as UTF-8.
- Count every answer with an XML parser, never a text search. Count bill allocations by content: a `BILLALLOCATIONS.LIST` with a `NAME` child is an allocation, and a whitespace-only one is counted apart.
- Close every other Bridge process first.

Files (UTF-16LE with a BOM, as sent; `SHA256.txt` lists them):
- `M-marks.xml`: the company high-water read, `lab/2162-s3a-sheet:lab/2162-s3a-sheet/M-marks.xml`, byte-equal.
- `B-bills.xml`: Bills Receivable from 1 Apr 2026 to 1 Aug 2026, `lab/2162-s3a-sheet:lab/2162-s3a-sheet/B-bills.xml`, byte-equal.
- `L-ledgers.xml`: four ledger Creates in the All Masters envelope. Setup.
  - `BRIDGE Walk-in BJ`, `BRIDGE Walk-in BS` and `BRIDGE Walk-in BO` under Sundry Debtors, `ISBILLWISEON` Yes;
  - `BRIDGE Seed Income` under Indirect Incomes, `ISBILLWISEON` No.
- `V-days.xml`: the vouchers read for 1 Jul 2026 to 2 Jul 2026, as the product renders it (the request dump `request_dump` at master 6569d3a3, file `v01-vouchers-wildcard.xml`, 1,740 bytes, sha256 137c35f1…).
- `I-0048.xml`, `I-0049.xml`, `I-0050.xml`: the invoices, each dated 1 Aug 2026, 1,051.00, with the party leg's New Ref named by the number for -1051.00:
  - `BP/26-27/0048` to `BRIDGE Walk-in BJ` (b1);
  - `BP/26-27/0049` to `BRIDGE Walk-in BS` (b2);
  - `BP/26-27/0050` to `BRIDGE Walk-in BO` (b3).
- `R-0048.xml`, `R-0049.xml`, `R-0050.xml`: the read-back, `lab/2162-s1-sheet:lab/2162-s1/R-0010.xml` with the number changed.
- `gen.py` makes `L-`, `I-` and `R-` again, from `template-import-0010.utf16le.xml` (the committed fixture `pilot-lab/pilot-lab-invoice-post-request.utf16le.xml` on master, byte-equal) and `template-r-0010.utf16le.xml` (`lab/2162-s1-sheet:lab/2162-s1/R-0010.xml`, byte-equal).

Keyed setup (after s3, on Tally's screen; write down each entry's voucher type, number and date as Tally shows them):
- **K1 (b1):** a Journal dated 1 Jul 2026: Dr `BRIDGE Walk-in BJ` 500.00, bill-wise New Ref named `BP/26-27/0048`; Cr `BRIDGE Seed Income` 500.00.
- **K2 (b2):**
  - a Journal dated 1 Jul 2026: Dr `BRIDGE Walk-in BS` 500.00, bill-wise New Ref named `BP/26-27/0049`; Cr `BRIDGE Seed Income` 500.00;
  - then a Receipt dated 2 Jul 2026: Cr `BRIDGE Walk-in BS` 500.00, bill-wise Agst Ref `BP/26-27/0049`; Dr `Cash` 500.00.
- **K3 (b3):** alter `BRIDGE Walk-in BO`: opening balance 500.00 Dr, with one opening bill named `BP/26-27/0050`, dated 1 Apr 2026, for 500.00 Dr.
- If Tally asks anything not listed here (for example a GST or a cost-centre detail), answer nothing: escape out of the entry, stop that sub-row, and write down the prompt.

Steps (`python3 -I run.py <gateway URL> <label> <files>`):
1. `run.py URL s1 M-marks.xml`. Take BK. Read the Gateway's current date and write it down. If it is before 1-Aug-2026, stop: the invoices would be post-dated, which is S1's variable.
2. `run.py URL s2 R-0048.xml R-0049.xml R-0050.xml B-bills.xml`. If any R holds a `VOUCHER` element, that number is in use: stop and report it.
3. `run.py URL s3 L-ledgers.xml`. If the answer is not CREATED 4 with ERRORS 0, stop at step 8 and report the answer.
4. Key K1, K2 and K3.
5. `run.py URL s4 V-days.xml B-bills.xml M-marks.xml`. Check the setup (Expected, s4). A sub-row whose setup does not hold stops here: it gets no import.
6. For each sub-row still running, in order b1, b2, b3:
   - b1: `run.py URL s5 I-0048.xml`, then `run.py URL s5r R-0048.xml B-bills.xml M-marks.xml`.
   - b2: `run.py URL s6 I-0049.xml`, then `run.py URL s6r R-0049.xml B-bills.xml M-marks.xml`.
   - b3: `run.py URL s7 I-0050.xml`, then `run.py URL s7r R-0050.xml B-bills.xml M-marks.xml`.
7. Capture Tally's window after each import: the Gateway, or a dialog.
8. Restore BK. Run `run.py URL s8 M-marks.xml`, and check that its answer is byte-identical to step 1's.

**Stop** at an import if:
- its answer is neither CREATED 1 with ERRORS 0, nor CREATED 0 with ERRORS 1 and a `LINEERROR`; or
- Tally's window shows a dialog.

A clean refusal (CREATED 0, ERRORS 1) is outcome (iv): record it and go on.

Record, for each answer: bytes, sha256, and the counts below. Each count says what it counts.
- **M answers:** the company's `ALTVCHID` and `ALTMSTID`, the voucher and master alteration marks.
- **s2 R:** the number of `VOUCHER` elements in each answer.
- **Every B answer:** for each of the three customers, the number of `BILLFIXED` groups whose `BILLPARTY` is that customer, and for each group its `BILLREF`, `BILLDATE` and the `BILLCL` that follows it.
- **L and every I answer:** `CREATED`, `ALTERED`, `ERRORS`, `EXCEPTIONS` and `LASTVCHID`, and any `LINEERROR` text.
- **s4 V:**
  - the number of `VOUCHER` elements;
  - for each voucher with a leg for one of the three customers: its `VOUCHERTYPENAME`, `VOUCHERNUMBER` and `DATE`;
  - on that leg: the number of `BILLALLOCATIONS.LIST` children with a `NAME` child, with each one's `NAME`, `BILLTYPE` and `AMOUNT` (its sign and value), and the number of whitespace-only ones apart.
- **s5r, s6r and s7r R:**
  - the number of `VOUCHER` elements;
  - within it, the number of `ALLLEDGERENTRIES.LIST` legs for the sub-row's customer;
  - on that leg: the number of `BILLALLOCATIONS.LIST` children with a `NAME` child, with each one's `NAME`, `BILLTYPE` and `AMOUNT`;
  - on every other leg: the allocations with a `NAME` child, and the whitespace-only ones apart;
  - `ALTERID`, `GUID` and the three `VCHGSTSTATUS` flags.

Expected:
- **s2:** each R holds 0 `VOUCHER` elements (the numbers are unused). B holds 0 groups for each of the three customers (the ledgers do not exist yet).
- **s3 L:** CREATED 4, ERRORS 0. s4's `ALTMSTID` is above s1's.
- **s4 V**, the keyed setup read natively:
  - one Journal dated 1 Jul 2026 whose `BRIDGE Walk-in BJ` leg holds 1 allocation: `NAME` `BP/26-27/0048`, `BILLTYPE` New Ref, `AMOUNT` -500.00;
  - one Journal dated 1 Jul 2026 whose `BRIDGE Walk-in BS` leg holds 1 allocation: `NAME` `BP/26-27/0049`, New Ref, -500.00;
  - one Receipt dated 2 Jul 2026 whose `BRIDGE Walk-in BS` leg holds 1 allocation: `NAME` `BP/26-27/0049`, Agst Ref, 500.00.
  - Other vouchers in the window, if any, are counted and named, not checked.
- **s4 B:**
  - `BRIDGE Walk-in BJ`: 1 group, `BP/26-27/0048`, 1-Jul-26, `BILLCL` -500.00;
  - `BRIDGE Walk-in BS`: 0 groups (the bill is settled);
  - `BRIDGE Walk-in BO`: 1 group, `BP/26-27/0050`, 1-Apr-26, `BILLCL` -500.00.
- **s5 to s7:** not predicted; this is the row's question. Each sub-row's answers show one of the outcomes:
  - (i) R's party leg holds 1 New Ref named by the number for -1051.00, and B shows two groups of that name for the customer;
  - (ii) R as in (i), and B shows one group of that name, with a `BILLCL` combining both amounts;
  - (iii) R's party leg holds an allocation of another `BILLTYPE`;
  - (iv) the import is refused (record the `LINEERROR`).
  - Anything else is recorded as seen.
- **s8:** the marks answer byte-identical to s1's.

An empty s5r, s6r or s7r R means the filter did not find the voucher. Report the number sent and the number on the screen. That is not a finding about the bill.

Decides:
- **(iii) or (iv) in a sub-row:** the read-back's allocation check (agent_import_invoice.rs:1476-1486 at e4440334) reads a difference, or nothing is created. That case fails closed.
- **(i) or (ii) in a sub-row:** the read-back reads clean while the customer's bills are not what the invoice says. That case is not caught today. Bill-wise customers then stay refused unless the Sales admission reads the customer's open bills (#1764 item 1), or b2 shows a settled bill that does not merge (#2162 item 6).
- **A setup that did not hold:** that sub-row is not a finding about the invoice. Send the answers to D66 before any code.

Code admits the trigger: each request's sender at file:line, and the branch that decides whether it is sent. Lines are at master 6569d3a3 unless marked.
- **I, the invoice:**
  - Rendered by `render_sales_invoice_xml` (agent_import_invoice.rs:2631). The New Ref is written at :2649 by `leg(party_entry, observed.party_bill_wise)` (:2661), where `party_bill_wise` is the catalogue's `ISBILLWISEON` for the party. It is sent as the native post `native_post_request` (agent_import_post.rs).
  - Whether a Sales post is sent at all is decided by `LIVE_QUALIFIED_VOUCHER_TYPES` (agent_import.rs:390). That list does not include Sales, so no release sends this request; that is why the row sends it raw, as #2162 item 3 asks.
- **R, the read-back:**
  - Rendered by `render_invoice_readback_request` (agent_import_invoice_wire.rs:484 at e4440334, #2307). Sent by `read_back_sales_invoice` (agent_import_invoice.rs), when `invoice_readback_due` (agent_import.rs) finds an invoice in the batch.
  - Its answer feeds `allocation_ok` (agent_import_invoice.rs:1476 at e4440334). The branch `leg.ledger == party && observed.party_bill_wise` (:1477) requires exactly one New Ref named by the number, for the party amount. Every other leg must hold no allocation (:1485). That check reads the `NAME`, `BILLTYPE` and `AMOUNT` leaves (agent_import_invoice_wire.rs:587-607 at e4440334), so a whitespace-only element adds no allocation.
- **B, Bills Receivable:** rendered by `render_native_bills_request` (bridge-tally-protocol native_outstandings/request.rs:131), byte-equal. Sent by the outstandings read at tally/runtime.rs:4136, from the book's start to the as-of date.
- **V, the vouchers read:** rendered by `render_agent_vouchers` (agent_read_profiles.rs:112, `FETCH` `ALLLEDGERENTRIES.*`, whose answer carries each leg's bill allocations), the unspanned entry-wildcard window read (agent_voucher_window.rs:160). The product sends that arm when a window needs no AlterID span. Here it is a native read of the keyed setup, not a claim about which tool would send it.
- **M, the marks:** `render_agent_company_high_water` (agent_read_profiles.rs:531), sent as `company_high_water_read` around every post.
- **L, the ledgers (setup):** `LedgerCreate::render_message` (bridge-tally-protocol ledger_create.rs:36) inside `render_import_envelope` (import_envelope.rs:27) with `ImportReport::AllMasters`, byte-equal for these fields. It is a seed, not a claim about which Bridge path would create these ledgers.
- **K1 to K3 (keyed):** no Bridge sender. Bridge's invoice renderer writes only a New Ref (agent_import_invoice.rs:2649). The lab-only import surface writes bill allocations on bank vouchers (`render_bill_allocations_xml`, agent_lab_import.rs:1935, feature `lab-writes`), but no Tally answer to it is on master. The keyed entries are setup only.

Row: lab/2162-s3b/
