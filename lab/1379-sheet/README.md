# Lab run sheet: what a synthetic inventory book holds for an item Sales invoice, and the design of the two write rows that follow (#1379). SYNTHETIC COMPANY ONLY. PART R IS READS ONLY. Not for merge.

About 20 minutes for Part R. `LAB-COMMON.md` in this folder applies in full (v3, sha256 `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6`, byte-identical to the copy on `lab/1233-stock-census-slice`). Where this sheet and it differ, the stricter one holds. In Part R nothing is keyed, imported, backed up or restored, and no setting on Tally's screen is changed.

**WHAT IT IS FOR.** #1379 adds stock-item lines to the Sales invoice. Two of its "measure live first" rows need no Sales switch-on, because they are raw imports, not ComplyEaze Bridge posts:
- **W1:** an amount-only item line on a book with one godown and no batches. Does Tally take the shape, and does a read return the line?
- **W2:** an item whose GST details differ from the sales ledger's, one difference at a time (type of supply, rate, HSN). Which one does Tally apply, as read from the stored voucher and from GSTR-1?

**WHY PART R COMES FIRST.** A write file is fixed and hashed before it is sent, and is sent once (C3). The two write rows' files must name a stock item, its unit, a sales ledger, a customer ledger and the tax ledgers that THE COMPANY holds, with the GST details W2 varies. No captured answer of THE COMPANY's ledgers, items or GST details is committed, so those names are not known. Part R reads them through ComplyEaze Bridge's own tools and adds nothing to the book. The write files are made from its answers, in a v2 of this sheet with its own SHA256.txt and a new GO.

**WHAT THE CODE ADMITS (read at master `5680ba48`).** The product cannot send either write row, so W1 and W2 are raw imports by curl, under the lab allowance for shape discovery.
- An invoice's own fields are `voucher_type_name`, `place_of_supply`, `round_off_ledger` and `observed`, with `deny_unknown_fields` (`src-tauri/src/agent_import_invoice.rs:38-55`). There is no item-line field.
- Sales is not in `LIVE_QUALIFIED_VOUCHER_TYPES` (`src-tauri/src/agent_import.rs:419-424`).
- The product's Sales render is `render_sales_invoice_xml` (`agent_import_invoice.rs:2655-2707`). It sends `OBJVIEW="Invoice Voucher View"`, `VCHENTRYMODE` `Accounting Invoice`, `ISINVOICE` Yes and every leg as `LEDGERENTRIES.LIST`, with a New Ref only for a bill-wise party. W1 starts from that render so that its answer transfers to #1379's builder.
- `vouchers` with `stock_lines` reads stock movement for Stock Journals, Delivery Notes, Receipt Notes and Credit Notes only, and counts other kinds as not read (change-list entry for #2016). So the read-back of a Sales item line is a raw request in v2, not a tool call.

**THE COMPANY.** `BRIDGE STOCK LAB`, company number 100027, GUID `1edb9b05-8d35-4c0c-9959-dce655730463` (as listed in `scripts/merge-gate-synthetic-uuids.json`). On 29 Oct 2026 its extent read gave `ALTMSTID` 292, `ALTVCHID` 4, `NUMSTOCKITEMS` 55, `ISINVENTORYON` Yes and `BOOKSFROM` 20250401 (`lab/1233-stock-census-slice-answers`, RESULTS.md). Later sheets may have changed it, so Part R records the values and does not gate on them.

**THE BUILD.** Part R's tool calls go through the lab's `bridge_mcp` client, as the #1233 nested-groups sheet ran them. The build must be of master at or after `5680ba48`. Record its commit, the source of the binary and its sha256 at step 0. A build that is older, or whose commit cannot be named, = STOP before the first call.

## Part R: the requests and calls

- `requests/m-marks-S.utf16le.xml` (1,640 bytes): THE COMPANY's flags, counts and marks (`render_company_inventory_flags_request`, filtered to THE COMPANY's GUID). It is byte-identical to the file of the same name on `lab/1233-stock-census-slice`, which was rendered by that builder. Its text also equals the builder's format string at master `5680ba48` (`bridge-tally-protocol/src/native_stock_summary.rs:78-84`, with `COMPANY_EXTENT_V2_FETCH` from `outstandings_shared.rs:750-751`) filled for THE COMPANY, as checked by a script. Its FETCH includes `ISINVENTORYON`, `ISBATCHWISEON`, `NUMGODOWNS`, `NUMSTOCKITEMS`, `NUMUNITS` and `NUMVOUCHERTYPES`. READ BOUND 8,192 bytes. Sent by curl under C3. Check it against `SHA256.txt` before its first send; a mismatch = STOP.
- Request-ID check before the first send, in this folder: `python -I check-request-ids.py requests`. EXPECT `census: ok 1, skipped 0, FAIL 0`. Anything else = STOP, nothing sent.
- The tool calls, each with exactly these arguments and THE COMPANY's GUID as `company_guid`. Each answer is saved whole as JSON with its size and sha256. A call that returns an error is recorded with the error's code and is not repeated. A refusal is a result, not a failure of the sheet.
  - `r1`: `list_companies` `{}`.
  - `r2`: `masters` `{"kind":"godowns"}`.
  - `r3`: `masters` `{"kind":"units"}`.
  - `r4`: `masters` `{"kind":"stock_items"}`. Each item comes with its own dated HSN and GST lists.
  - `r5`: `masters` `{"kind":"voucher_types"}`.
  - `r6`: `ledger_masters` `{"group":"Sales Accounts","group_scope":"ancestry","fields":"compliance","include_gst_details":true}`.
  - `r7`: `ledger_masters` `{"group":"Sundry Debtors","group_scope":"ancestry","fields":"compliance","include_gst_details":true}`.
  - `r8`: `ledger_masters` `{"group":"Duties & Taxes","group_scope":"ancestry","fields":"compliance","include_gst_details":true}`.
  - Paged answers: where an answer says more rows follow, call again with the `offset` it gives and the `snapshot_id` it returns, until none follow. Record each page.

## Steps (Part R)

0. C2 a, b and c with NO company loaded (a company already loaded = STOP). Load THE COMPANY alone by Select Company. C2 d: the MARKS read is `m-marks-S`. Record Tally's current date, the period and the last entry the Gateway shows. Record THE BUILD.
1. Send `m-marks-S` (name `m-a`). One COMPANY row with THE COMPANY's GUID (C4, else STOP). Record, trimmed: `ALTMSTID`, `ALTVCHID`, `ISINVENTORYON`, `ISBATCHWISEON`, `ISINTEGRATED`, `NUMGODOWNS`, `NUMSTOCKITEMS`, `NUMUNITS`, `NUMVOUCHERTYPES`, `BOOKSFROM` and `LASTVOUCHERDATE`. STOP, and send nothing further, if `ISINVENTORYON` is not `Yes`.
2. The calls `r1` to `r8`, in order, one at a time, with paging as above. `r1` must list THE COMPANY with THE COMPANY's GUID, else STOP.
3. Send `m-marks-S` again (`m-b`). `ALTMSTID` and `ALTVCHID` must equal `m-a`'s, else STOP and report.
4. C10 close.

**Record by parser** (C4 for `m-a` and `m-b`; the JSON as returned for `r1` to `r8`). These go into a table file, and no name from the book goes into a text line other than that table:
- `r2`: the number of godowns and their names. Is there exactly one, Tally's default?
- `r3`: each unit's name, its decimal places, and whether it is simple.
- `r4`: for each stock item, its name, parent, base unit, and its dated HSN and GST lists as returned (taxability, type of supply, HSN or SAC, each rate). Count the items with a GST rate, with an HSN, and with neither.
- `r5`: each voucher type of the Sales class, with `numbering_method` and `series_numbering` as returned.
- `r6`: each sales ledger's GST details as returned (taxability, type of supply, HSN or SAC, rates), and whether it is set to affect stock if that is returned.
- `r7`: each customer ledger's registration type, GSTIN status, state and bill-wise setting where returned.
- `r8`: each tax ledger's `gst_duty_head` and its own rate details.
- **The verdict for v2:** can W1 and W2 be built on this book as it is? That needs one godown, no batch-wise items, a Sales-class voucher type, a customer ledger that is not bill-wise, CGST and State Tax ledgers, and one stock item and one sales ledger whose GST details are set. For W2 they must differ in exactly one attribute per row, or be made to by keying in v2. Say which masters v2 would have to key, as a list of fields.

**PASS (Part R ran)** = the gate held, every call answered (an error code recorded is an answer), and the marks did not move. **FAIL** = a transport failure, a count by substring, or a mark that moved during the reads.

**EVIDENCE** on a NEW branch `lab/1379-sheet-answers` (C11), after the pre-push scan and the run coordinator's yes. It holds `m-a` and `m-b` raw with headers, sizes and sha256, each tool answer as JSON, a send log, the parser's table and a notes file with UTC times. Screenshots stay on the lab PC, listed by name, bytes and sha256.

## Part W: the two write rows (design for review; files in v2)

Not sent from this sheet. Each runs on THE COMPANY under C5 (a backup before the first change) and C6 (restored after the last read). Each file is sent once. Each import is preceded by a MARKS read holding THE COMPANY alone and is followed by a screenshot before the next request (a NEW SHAPE).
- **W1, an amount-only item line.** It is `render_sales_invoice_xml`'s shape with two changes, and no other:
  1. the sales ledger's `LEDGERENTRIES.LIST` is replaced by one `ALLINVENTORYENTRIES.LIST` (`STOCKITEMNAME`, `ISDEEMEDPOSITIVE` No, `AMOUNT`, and one `ACCOUNTINGALLOCATIONS.LIST` naming the sales ledger with the same amount), with no `RATE`, `ACTUALQTY` or `BILLEDQTY` (the service-line form of protocol reference §9.12a, observation 2);
  2. `VCHENTRYMODE` reads `Item Invoice` in place of `Accounting Invoice`.

  The party is a customer that is not bill-wise, so there is no New Ref. CGST and State Tax are at the item's rate on the line amount, and the whole voucher sums to zero. **What it decides:** the import's counters, and a raw read of the stored voucher with `ALLINVENTORYENTRIES.*` and `LEDGERENTRIES.*` (a NEW SHAPE, written in v2). Compare it field by field: the item, the amount, the sales ledger in the allocation, the party leg, both tax legs, no quantity, and the godown Tally put on the line. Then the invoice read-back request the product renders, with `ISINVOICE` and the GST status fields, as a control. **Scope, stated:** two elements change against the product's render. §9.12's own observation was a desktop import of a different shape on another release. So the result is a shape that worked or did not, recorded PARTIAL, never one variable isolated.
- **W2, item against ledger GST details, one difference at a time.** Three invoices with the same figures, the same customer and the same sales ledger, each with one item line on a different stock item. Each item agrees with the sales ledger in every GST detail but one:
  - W2a: the type of supply (Goods against Services);
  - W2b: the rate;
  - W2c: the HSN or SAC.

  The tax legs are sent at the sales ledger's rate in every one, so the figures stay constant and only the item's detail varies. **What it decides:** for each, the stored inventory entry's GST details, from the v2 raw read; Tally's GST status of the voucher; and Tally's GSTR-1 view for the month, captured if Tally exports it, else transcribed and marked PARTIAL. Record which of the two details each shows. A control invoice whose item agrees with the ledger in everything is posted first. If the book holds no item that differs from the ledger in exactly one attribute, v2 names the keyed item and each field value, and that keying is a master change made after the backup.
- **After both rows:** C6 restore from the backup taken before W1, then `m-marks-S` and `r4` to `r8` again. Each must equal Part R's answer, else STOP and report.

**NOT MEASURED** (by either part): quantity and rate lines, a batch-wise company (section 12a.13), more than one godown, a stock item that is not in the book (#1379's refusal row), the tax rule when per-line rounding and rounding on the total differ, ComplyEaze Bridge's own post of an item invoice (that needs Sales switched on), and any other book or release.
