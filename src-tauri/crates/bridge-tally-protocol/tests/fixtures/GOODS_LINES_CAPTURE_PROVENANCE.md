# `voucher_goods_lines_two_godowns_shape_lab_*` — provenance

A live capture of the request the vouchers read sends (`Bridge Agent Vouchers`, a `Voucher` collection with the `FETCH` list that ends `ALLLEDGERENTRIES.*,REFERENCE,ISINVOICE,PARTYGSTIN,REFERENCEDATE`), for 24 April 2025, on a synthetic lab book, with the request that was sent. That day holds a Purchase whose one goods line is split over two godowns and two batches. Protocol reference §8.2i.

## Provenance

- **Host / gateway:** TallyPrime **Silver**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance, Windows).
- **Date:** 2026-10-14, read only, one request at a time, no client time limit; HTTP 200, `RESPSTATUS: 1`, and a size equal to its `CONTENT-LENGTH` (110,156 bytes).
- **Book:** `BRIDGE SHAPE LAB`, a synthetic lab company with inventory, batches and two godowns, loaded alone; its book extent read before and after the run was byte-identical (`ALTVCHID` 112, `ALTMSTID` 289). Nothing was keyed, imported or altered in that run.
- **Source:** the lab branch `lab/missing-reads-answers` at `6ab1fffa680a9cde8bfdab99948c531a23170bef`: `answers/ans-a07.xml` and `requests/a07-vouchers-shape-20250424.xml`, each listed in that branch's `SHA256.txt` with the sha256 below; its `NOTES.md` records the run. Copied here byte for byte.
- **Sender:** `curl.exe` from the lab's run sheet; the request is the file the lab sent (UTF-16LE with a BOM, no line breaks) and equals what `render_agent_vouchers` renders for this company and day (a test compares them); the response is the file written with `-o`.
- **Encoding:** the response is BOM-less UTF-16LE with CRLF line ends, exactly as received. `.gitattributes` marks this tree `-text`.
- **What the capture shows:** two `VOUCHER` rows. The Payment carries no goods line. The Purchase (`ISINVOICE` `Yes`) carries one `INVENTORYALLOCATIONS.LIST` under its purchase ledger entry: Hydrochloric Acid 33pc, ` 666.667 Kgs` at `30.00/Kgs`, `-20000.00`, `ISDEEMEDPOSITIVE` `Yes`, with two `BATCHALLOCATIONS.LIST` rows: Main Location, H-2504-A, ` 300.000 Kgs`, `-9000.00`; Factory Floor, H-2504-B, ` 366.667 Kgs`, `-11000.00`. Counted with an XML parser.
- **Not established:** a Sales line split over godowns, a Credit or Debit Note with items, discounts, additional cost, a voucher typed on Tally's screen (this book's vouchers were made for the lab), Gold and other releases.

| file | bytes | sha256 | content |
|---|---|---|---|
| `voucher_goods_lines_two_godowns_shape_lab_request.utf16le.xml` | 1740 | `47cf36c300a5cc31297033dba27c6233729e93d3ad6e17d72022e73d28352d9c` | the request for 2025-04-24 on `BRIDGE SHAPE LAB` (UTF-16LE with a BOM) |
| `voucher_goods_lines_two_godowns_shape_lab_live.utf16le.xml` | 110156 | `393a49416675b03720b0a89700a9dc212179920326259725ed2df0523ac42c63` | its answer: a Payment and a Purchase with one goods line split over two godowns |
