# `stock_movement_*_shape_lab_*` — provenance

Live captures of stock-carrying vouchers keyed by hand on a synthetic lab book, each read through two requests, for #2016 (reading stock movement). The two requests:
- **the vouchers request**: the one `vouchers` sends (`Bridge Agent Vouchers`, a `Voucher` collection whose `FETCH` list ends `ALLLEDGERENTRIES.*,REFERENCE,ISINVOICE,PARTYGSTIN,REFERENCEDATE`);
- **the inventory request**: the same request with `ALLLEDGERENTRIES.*` replaced by `ALLINVENTORYENTRIES.*` and nothing else changed. No shipped builder sends it; it is the shape a stock-movement read is to be built from, kept here as captured evidence before any parser exists.

Each request is a committed template with only the company and the day changed (each run sheet's `gen_requests.py --check` reproduced it byte for byte), sent one at a time.

## Provenance

- **Host / gateway:** TallyPrime **7.2**, Silver, licensed, `http://127.0.0.1:9001` (a lab instance, Windows). Every answer was HTTP 200, a size equal to its `CONTENT-LENGTH`, read without a client time limit.
- **Date:** 2026-10-28, two sittings on the same book.
- **Book:** `BRIDGE SHAPE LAB`, a synthetic lab company with inventory, batches and two godowns (Main Location, Factory Floor), loaded alone. Each sitting took a backup before keying and restored it afterwards; the book read the same before and after (marks 292 / 112).
- **What was keyed** (on Tally's own screens, one voucher a day):
  - 11 May 2026, Stock Journal SJ1: 10 Kgs of one item from Main Location to Factory Floor, batch H-2504-A;
  - 14 May 2026, Credit Note CN1 in item-invoice mode: 1 Kgs of the same item into Main Location, batch H-2504-A;
  - 12 May 2026, Delivery Note DN1: 1 Kgs out of Main Location, batch H-2504-A;
  - 13 May 2026, Receipt Note RN1: 1 Kgs into Factory Floor, batch H-2504-B.
  Delivery Note and Receipt Note were inactive voucher types on this book; the second sitting activated each by "Activate this Voucher Type" before keying.
- **Sources** (copied here byte for byte; each listed with this sha256 in its branch's `SHA256.txt`):
  - SJ1 and CN1: the lab branch `lab/2016-stock-movement-capture-answers` at `fbbdf3f2d212e4e7a605a1c0f7f55cee01263939`, `raw/a07-<day>.xml` and `raw/a08-<day>.xml`; requests from `lab/2016-stock-movement-capture` at `f85ed60ff87f3bc891e237fdff2ab85d3d759042`, `requests/r-a07-<day>.utf16le.xml` and `requests/r-a08-<day>.utf16le.xml`.
  - DN1 and RN1: `lab/2016-delivery-receipt-note-capture-answers` at `a41171d8d1a9fbbb386587274beae1a942f30ac1`, `raw/a07-<day>` and `raw/a08-<day>`; requests from `lab/2016-delivery-receipt-note-capture` at `fc993a31021595e14760ffa386033f13b1d4966f`.
- **Sender:** `curl.exe` from each run sheet; the request is the file the lab sent, and the response the file written with `-o`.
- **Encoding:** BOM-less UTF-16LE, exactly as received. `.gitattributes` marks this tree `-text`.
- **Direction:** on every stock line the direction is `ISDEEMEDPOSITIVE` (Yes into stock, No out of it), in both requests. Counted with an XML parser.
- **Not established:** a Stock Journal with several items or one side only; Sales, Purchase or Debit Note with an item in the inventory request on 7.2; Physical Stock or a Manufacturing Journal; a voucher type created under these; optional, cancelled or post-dated stock vouchers; whether a Delivery or Receipt Note moves Tally's Stock Summary (the two notes moved the same item 1 Kgs out and 1 Kgs in); other releases and Gold.

| file | bytes | sha256 | content |
|---|---|---|---|
| `stock_movement_stock_journal_vouchers_shape_lab_request.utf16le.xml` | 1740 | `29b26ce987b52fb28a5715d100aa26306ba55981fd2d7f891a1b4e2b2713a52e` | the shipped vouchers request (`ALLLEDGERENTRIES.*`) for that day on `BRIDGE SHAPE LAB` (UTF-16LE) |
| `stock_movement_stock_journal_vouchers_shape_lab_live.utf16le.xml` | 6368 | `b419dd9ab5eee12ea1eb7d4addea2a1c7dd97962bafba166754e2063d2317122` | its answer, 11 May 2026: the Stock Journal SJ1, whose `ALLLEDGERENTRIES.LIST` is empty (#388); no item, quantity, godown or batch |
| `stock_movement_stock_journal_inventory_shape_lab_request.utf16le.xml` | 1746 | `6311e6fbdd627d1be001a4252414ac5f7ac060bd05c181e8485ccccce525d897` | the lab's inventory request for that day: the vouchers request with `ALLLEDGERENTRIES.*` replaced by `ALLINVENTORYENTRIES.*`, nothing else changed on `BRIDGE SHAPE LAB` (UTF-16LE) |
| `stock_movement_stock_journal_inventory_shape_lab_live.utf16le.xml` | 36986 | `0d484496d5fa80932c484eb1e558dc05473fd6f30bf58fb70405e867335976ee` | its answer: SJ1 with two `ALLINVENTORYENTRIES.LIST` (IN first: ISDEEMEDPOSITIVE Yes, 10 Kgs, RATE and AMOUNT present and empty; then OUT: No, 10 Kgs, 30.00/Kgs, 300.00), each with one batch row (GODOWNNAME Factory Floor / Main Location, BATCHNAME H-2504-A, DESTINATIONGODOWNNAME Factory Floor), plus `INVENTORYENTRIESIN.LIST` and `INVENTORYENTRIESOUT.LIST` whose batch rows carry no GODOWNNAME |
| `stock_movement_credit_note_vouchers_shape_lab_request.utf16le.xml` | 1740 | `a14716276f75252a17cfaaeeb5c98c1bc998a77a394fb444feb002836261ff18` | the shipped vouchers request (`ALLLEDGERENTRIES.*`) for that day on `BRIDGE SHAPE LAB` (UTF-16LE) |
| `stock_movement_credit_note_vouchers_shape_lab_live.utf16le.xml` | 50088 | `a422ad9da84e1cd484b291f95c350e0888925020d12d9a9973dc8690e4360c0e` | its answer, 14 May 2026: the Credit Note CN1, its item nested under the sales ledger entry (`INVENTORYALLOCATIONS.LIST`, ISDEEMEDPOSITIVE Yes, 1 Kgs at 30.00/Kgs, -30.00; batch Main Location / H-2504-A) |
| `stock_movement_credit_note_inventory_shape_lab_request.utf16le.xml` | 1746 | `f97be461016a6449fea761894794305a938281a067ce18dd15106e5d1d538a3e` | the lab's inventory request for that day: the vouchers request with `ALLLEDGERENTRIES.*` replaced by `ALLINVENTORYENTRIES.*`, nothing else changed on `BRIDGE SHAPE LAB` (UTF-16LE) |
| `stock_movement_credit_note_inventory_shape_lab_live.utf16le.xml` | 36262 | `7bd4af7a4b3471909e4e6f914df601f8a273d347a485f316122a5a0ef1d83630` | its answer: CN1's item as an `ALLINVENTORYENTRIES.LIST` (ISDEEMEDPOSITIVE Yes) with an `ACCOUNTINGALLOCATIONS.LIST`, plus the party line |
| `stock_movement_delivery_note_vouchers_shape_lab_request.utf16le.xml` | 1740 | `9027b61e1d92f0945400ab59e944f024a63da055f3631ab3e2b112e4df917725` | the shipped vouchers request (`ALLLEDGERENTRIES.*`) for that day on `BRIDGE SHAPE LAB` (UTF-16LE) |
| `stock_movement_delivery_note_vouchers_shape_lab_live.utf16le.xml` | 49934 | `161b12649f74d3613a919751a47bd0bdc8d407a913b3a1a18db24893d1526b39` | its answer, 12 May 2026: the Delivery Note DN1, its item nested under a ledger entry (ISDEEMEDPOSITIVE No, 1 Kgs at 30.00, Main Location / H-2504-A) |
| `stock_movement_delivery_note_inventory_shape_lab_request.utf16le.xml` | 1746 | `b89787fe6d834a8aaa1fe952fc3ca6827cd39dbf6860cb7c5c7a66b67bd4b0f6` | the lab's inventory request for that day: the vouchers request with `ALLLEDGERENTRIES.*` replaced by `ALLINVENTORYENTRIES.*`, nothing else changed on `BRIDGE SHAPE LAB` (UTF-16LE) |
| `stock_movement_delivery_note_inventory_shape_lab_live.utf16le.xml` | 37356 | `6ffabc13d9f3faf9b9ac78505375f2508064cac6362b80efda3680f35791b8a0` | its answer: DN1's item as an `ALLINVENTORYENTRIES.LIST` (ISDEEMEDPOSITIVE No), no IN/OUT lists |
| `stock_movement_receipt_note_vouchers_shape_lab_request.utf16le.xml` | 1740 | `7bed0f954e4aed8a2f4885bb62fa4a787d64c0696f685b7056884337a602345a` | the shipped vouchers request (`ALLLEDGERENTRIES.*`) for that day on `BRIDGE SHAPE LAB` (UTF-16LE) |
| `stock_movement_receipt_note_vouchers_shape_lab_live.utf16le.xml` | 50542 | `f1b52ec8d70b6039ef27ebeead0bf6de85734e8f55fca4b38b93164aed56e2a0` | its answer, 13 May 2026: the Receipt Note RN1, its item nested under a ledger entry (ISDEEMEDPOSITIVE Yes, 1 Kgs at 30.00, Factory Floor / H-2504-B) |
| `stock_movement_receipt_note_inventory_shape_lab_request.utf16le.xml` | 1746 | `ef0c7878a490a80b9bde3f48c8029cd736a53d9242a6c59409536d408c2fce90` | the lab's inventory request for that day: the vouchers request with `ALLLEDGERENTRIES.*` replaced by `ALLINVENTORYENTRIES.*`, nothing else changed on `BRIDGE SHAPE LAB` (UTF-16LE) |
| `stock_movement_receipt_note_inventory_shape_lab_live.utf16le.xml` | 37964 | `f0506eb7b2ccb1dc832b9f5dee0ae0a07d875051d449d7330c35e2ce9d3db364` | its answer: RN1's item as an `ALLINVENTORYENTRIES.LIST` (ISDEEMEDPOSITIVE Yes), no IN/OUT lists |
