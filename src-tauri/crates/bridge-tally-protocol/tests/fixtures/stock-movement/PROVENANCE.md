# `stock-movement/`: three vouchers that move stock, read through the inventory-entries window

Seven files: the same lab read of one day, taken for three days that each hold exactly one voucher:
a Sales item invoice, a Sales item invoice with tax, and a Stock Journal that moves one item from
one godown to another. They are here for the stock item movement read that comes next. No test
reads them yet, and no movement tool produced them.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`,
  `http://127.0.0.1:9001` (a lab instance).
- **Sender:** `bridge_mcp`, a debug build of commit `4b3871cad358522f335812f47b2ec64235924de4` with
  the `lab-writes` feature (binary sha256
  `38370d5cfa0bbaff9f48d8e9ef868dc93a3358db5b6b526910850dd054f4137f`), making one
  `lab_read_inventory` call for each single day, **directly on the gateway**. That tool refuses any
  endpoint port but the gateway's own, so no recording relay could sit in front of it.
- **None of these files is wire bytes.** The request and response files are what the tool itself
  persists for each read: the text Bridge held, UTF-8, each named by its SHA-256 in the tool's own
  record. They are byte-exact copies of those persisted files, not of the bytes on the wire.
- The request's `FETCH` asks for `ALLINVENTORYENTRIES.*`. It is a lab-only shape; no shipped tool
  sends it.
- `tally_status` was read before and after each call, and the loaded companies were the same.

### The two sales days

- **Date:** 2026-10-01, 16:52 to 16:54 IST (the first day) and 17:12 to 17:14 IST (the second).
- **Book:** `BRIDGE STOCK LAB`, a synthetic lab company with inventory and batch-wise details on.
  Its stock items' opening values were entered with the sign reversed (protocol reference
  §12a.13), so no figure in this book says anything about a real trader's stock.
- **2025-04-20** holds one voucher: `Sales`, invoice view, a party debit of 24.00 and one goods
  line, 2 units of one stock item at 12.00, with a batch allocation naming Tally's default godown
  and default batch. Imported for this purpose, not typed on Tally's screen.
- **2025-04-21** holds one voucher of the same kind with tax: a party debit of 118.00, one goods
  line of 1 unit at 100.00, and two credit entries of 9.00 on ledgers under Duties & Taxes. The
  company's GST flag reads `Yes`; what registration it holds was not established, and no GSTIN,
  place of supply, HSN or rate is on the voucher or the item.
- These four files were first committed beside the sales register's captures and are moved here
  unchanged: the same bytes, the same hashes.

### The Stock Journal day

- **Date:** 2026-10-01, 23:34 IST.
- **Book:** `BRIDGE SHAPE LAB`, a synthetic company seeded by this project: 11 stock items, 3
  stock groups, 2 godowns, 4 units.
- **2025-04-30** holds one voucher: `Stock Journal` no. 1, which moves 50.000 Kgs of one item from
  one godown to another at 25.00 a Kg. Tally's own Day Book shows it as 50.000 Kgs outwards.
- `stock_journal_day_answer.json` is the tool's answer to that call: the masters lists and the
  one voucher, as the tool returned them. It is not a capture of Tally's bytes.

## What the captures show

- **A sale.** The goods line is at the top level (`ALLINVENTORYENTRIES.LIST`): deemed positive
  `No`, rate `12.00/U.`, actual and billed quantity ` 2 U.`, amount `24.00`, with its
  `BATCHALLOCATIONS.LIST` and an `ACCOUNTINGALLOCATIONS.LIST` to the sales ledger. An outward line
  reads back with a positive amount.
- **A taxed sale.** One `ALLINVENTORYENTRIES.LIST` (1 unit at `100.00/U.`, amount `100.00`, its
  batch allocation and its accounting allocation to the sales ledger) and three
  `LEDGERENTRIES.LIST`: the party and the two tax ledgers. The sales ledger is not among them in
  this fetch; it is the goods line's accounting allocation.
- **A Stock Journal, in the tool's answer.** Two entries for the same item with the same unsigned
  quantity (` 50.000 Kgs`) and rate. One has amount `1250.00` and a batch allocation in godown
  `Factory Floor`; the other has amount `-1250.00` and a batch allocation in godown
  `Main Location`. The godown is on the batch allocation; the entry's own godown is null. In the
  answer, **only the sign of the amount tells the two sides apart**: positive is the outward side,
  negative the inward side, as for every other stock value in the reference.
- **A Stock Journal, in Tally's response.** Tally says which side is which, and the tool's answer
  does not carry it over. Beside the two `ALLINVENTORYENTRIES.LIST` the voucher holds one
  `INVENTORYENTRIESOUT.LIST` (amount `1250.00`, `ISDEEMEDPOSITIVE` `No`) and one
  `INVENTORYENTRIESIN.LIST` (amount `-1250.00`, `ISDEEMEDPOSITIVE` `Yes`), although the request
  fetched only `ALLINVENTORYENTRIES.*`. The two `ALLINVENTORYENTRIES.LIST` arrive out first, then
  in, and each carries the same `ISDEEMEDPOSITIVE` as its side. A `DESTINATIONGODOWNNAME` element
  is present on each entry and empty: the destination is the other entry's batch godown. The
  voucher's view is `Consumption Voucher View`. The amounts net to zero.

## What they do not settle

- A manufacturing journal (different items in and out); several items in one journal; a batch
  other than the default; a scrap or by-product line; a journal with additional cost.
- A purchase through this window; a credit or debit note with goods; optional, cancelled or
  post-dated vouchers; a line listed twice; stock items under a stock group inside a stock group.
- Whether the in and out lists appear when `ALLINVENTORYENTRIES.*` is not fetched, and whether
  they appear on voucher types other than a Stock Journal (the two sales here carry neither).
- Three vouchers, two synthetic companies, one run each, one release.

## Files

| file | bytes | sha256 | content |
|---|---|---|---|
| `inventory_window_sales_day_request.utf8.xml` | 815 | `4bfec60199ef88746403d4407301e2aa89faa36c894fd79e4b33b426e077277f` | request as persisted text, inventory-entries window, 2025-04-20 |
| `inventory_window_sales_day_response.utf8.xml` | 17890 | `06cbaee9df3a4a7648a62497895c0b587d83a72500b7c801459f2b6d68b2f37d` | response as persisted text, one Sales voucher |
| `inventory_window_stock_journal_day_request.utf8.xml` | 815 | `742a90fc8b200a45e4534d094148d007c1520fb8bddd2dd3c1c651ede4b6e023` | request as persisted text, inventory-entries window, 2025-04-30 |
| `inventory_window_stock_journal_day_response.utf8.xml` | 18361 | `845b238df83c7d2004cf0e69fac6b31cc9ed16cb53150fbf4df9c519e98aa2e7` | response as persisted text, one Stock Journal voucher |
| `inventory_window_taxed_sales_day_request.utf8.xml` | 815 | `b7f3d1fa62553ddbdb7a85ae10bc4f759b6e185231dd58f2b3416f2b58a77798` | request as persisted text, inventory-entries window, 2025-04-21 |
| `inventory_window_taxed_sales_day_response.utf8.xml` | 18736 | `22b1fcb5fbfaf1fbedb2bd43e41dce3a53fa2cbebaef3a10940ff64c7d59b6db` | response as persisted text, one taxed Sales voucher |
| `stock_journal_day_answer.json` | 5939 | `4759b11f514bf785d1bbdfe92cf5788ac31e04d5dbbd6bd7de257aa6c959c608` | TOOL ANSWER, not a capture of Tally's bytes: the structured content the tool returned |
