# `voucher_cost_centre_allocations_*` — provenance

A live capture of the request the vouchers read sends (`Bridge Agent Vouchers`, a `Voucher` collection with the `FETCH` list that ends `ALLLEDGERENTRIES.*,REFERENCE,ISINVOICE,PARTYGSTIN,REFERENCEDATE`), for one day, on a synthetic lab book, with the request that was sent. Protocol reference §8.2g.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance, Windows).
- **Date:** 2026-10-14, attended, read only, one request at a time, no client time limit; HTTP 200 and `RESPSTATUS: 1`.
- **Book:** `BRIDGE SHAPE LAB`, a synthetic lab company (books from 2025-04-01), loaded alone. The voucher is a Journal dated 2025-06-05 with two ledger entries.
- **Sender:** `curl.exe` from the lab's run sheet; the request is byte for byte the file the lab sent (UTF-16LE with a BOM, no line breaks), and the response is byte for byte the file written with `-o`.
- **Encoding:** the response is BOM-less UTF-16LE with CRLF line ends, exactly as received. `.gitattributes` marks this tree `-text`.
- **What the capture shows:** one `VOUCHER` row; the first ledger entry (`Power Charges`, -1000.00) carries `CATEGORYALLOCATIONS.LIST` (`Business Line`) with two `COSTCENTREALLOCATIONS.LIST` rows (`Trading` -600.00, `Assembly` -400.00); the second entry has no such element.
- **Not established:** several categories, a centre with a parent, quantities, foreign-currency amounts, a book whose Cost Centres setting is Yes, other voucher types, other releases.

| file | bytes | sha256 | content |
|---|---|---|---|
| `voucher_cost_centre_allocations_shape_lab_request.utf16le.xml` | 1740 | `783a89f96e6c28ee9b575a1d871fbdcadcada8f7a44122b153d001a167ee71a8` | the request for 2025-06-05 on `BRIDGE SHAPE LAB` (UTF-16LE with a BOM) |
| `voucher_cost_centre_allocations_shape_lab_live.utf16le.xml` | 35860 | `2cd0c800f810c4a8fabdce06612784baff6e710a2787758be39ef60ae81d46fb` | its answer: one Journal, two entries, two cost-centre rows |
