# `sales-day/`: one Sales item invoice, read two ways

Four files captured on one day window that holds exactly one voucher, a Sales item invoice.
They are here for the reads that come next (a sales register, and stock item movement). No test
reads them yet, and no sales-register or movement tool produced them.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`,
  `http://127.0.0.1:9001` (a lab instance).
- **Date:** 2026-10-01, 16:52–16:54 IST.
- **Book:** `BRIDGE STOCK LAB`, a synthetic lab company with inventory and batch-wise details on.
  Its stock items' opening values were entered with the sign reversed (a positive opening value is
  what Tally's Stock Summary screen shows as negative stock value; protocol reference §12a.13), so
  no figure in this book says anything about a real trader's stock.
- **The day:** 2025-04-20 holds one voucher: `Sales`, invoice view, a party ledger debit of 24.00
  and one goods line, 2 units of one stock item at 12.00, with a batch allocation naming Tally's
  default godown and default batch. The voucher was imported for this purpose; it was not typed on
  Tally's screen.
- **`tally_status`** was read before and after each call, and the loaded companies were the same.

## The two pairs

**`register_window_sales_day_*` are wire bytes.**
- Sender: `bridge_mcp`, a debug build of commit `9d3d92f2745c908ef7883bea43a8bd7ee8572c2e` (the
  purchase register's branch), making one `purchase_register` call for the single day. A serial
  read-only relay sat between the binary and the gateway and kept the exact request and response
  bytes; 96 requests were relayed, none refused.
- **The read path is the purchase register's.** The tool answered `complete` with no rows and
  `vouchers_observed: 1`, because a Sales voucher with no entry under Duties & Taxes is not a
  purchase-register row. These files are the class-shaped voucher window that call sent and received.
- The request is UTF-16LE with a byte-order mark, exactly as sent. The response is BOM-less
  UTF-16LE, exactly as received; its length equalled the declared `Content-Length`; the call's two
  paired reads returned identical bytes. No transformation.

**`inventory_window_sales_day_*` are not wire bytes.**
- Sender: `bridge_mcp`, a debug build of commit `4b3871cad358522f335812f47b2ec64235924de4` with the
  `lab-writes` feature, making one `lab_read_inventory` call for the single day. That tool refuses
  any endpoint port but the gateway's own, so no relay could sit in front of it.
- The files are what the tool itself persists for each data read: the request and the response
  **as the text Bridge held** (UTF-8), named by their SHA-256. They are byte-exact copies of those
  persisted files, not of the bytes on the wire.
- The request's `FETCH` asks for `ALLINVENTORYENTRIES.*`. It is a lab-only shape; no shipped tool
  sends it.

## What the captures show

- **Ledger-entries window.** `VCHTYPE="Sales"`, `ISINVOICE` `Yes`, the class predicate for sales
  `Yes`, voucher number `1` (a number supplied on import was not kept; numbering is automatic), and
  `REMOTEID` equal to Tally's own GUID. Two `ALLLEDGERENTRIES.LIST`: the party (`-24.00`, deemed
  positive `Yes`) and the sales ledger (`24.00`, `No`). **The goods line is nested under the sales
  ledger's entry**, as `INVENTORYALLOCATIONS.LIST` with its `BATCHALLOCATIONS.LIST` inside.
- **Inventory-entries window.** The goods line is at the top level (`ALLINVENTORYENTRIES.LIST`):
  deemed positive `No`, rate `12.00/U.`, actual and billed quantity ` 2 U.`, amount `24.00`, with
  its `BATCHALLOCATIONS.LIST` and an `ACCOUNTINGALLOCATIONS.LIST` to the sales ledger. An outward
  line reads back with a positive amount.

## What they do not settle

Tax lines (the book has no GST ledgers); credit notes; a Stock Journal; optional, cancelled or
post-dated vouchers; a line listed twice; a sale typed on Tally's screen; a real batch; stock items
under a stock group inside a stock group; other releases. One voucher, one company, one day.

## Files

| file | bytes | sha256 | content |
|---|---|---|---|
| `register_window_sales_day_request.utf16le.xml` | 3088 | `c266e0139725fc954f7427a5082633891598178634358480e162ee1b02948b42` | captured request, class-shaped voucher window, wire bytes |
| `register_window_sales_day_live.utf16le.xml` | 49988 | `3b6f2bae39000b8cf3aa7e40ee8b1d674e792edd1a688bd783736bf9a48ef0f7` | captured response, one Sales voucher, wire bytes |
| `inventory_window_sales_day_request.utf8.xml` | 815 | `4bfec60199ef88746403d4407301e2aa89faa36c894fd79e4b33b426e077277f` | request as persisted text, inventory-entries window |
| `inventory_window_sales_day_response.utf8.xml` | 17890 | `06cbaee9df3a4a7648a62497895c0b587d83a72500b7c801459f2b6d68b2f37d` | response as persisted text, one Sales voucher |
