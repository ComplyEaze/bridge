# `voucher_contra_hand_keyed_transfer_lab_*` — provenance

A live capture of the request the vouchers read sends (`Bridge Agent Vouchers`, a `Voucher` collection with the `FETCH` list that ends `ALLLEDGERENTRIES.*,REFERENCE,ISINVOICE,PARTYGSTIN,REFERENCEDATE`), for 1 to 5 April 2026, on a synthetic lab book of five Contras keyed by hand between two bank ledgers, with the request that was sent. Protocol reference §8.2h.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance, Windows).
- **Date:** 2026-10-12, attended, read only, one request at a time, no client time limit; HTTP 200 and `RESPSTATUS: 1`.
- **Book:** `BRIDGE TRANSFER LAB`, a synthetic lab company (books from 2026-04-01, no inventory, no GST), loaded alone. Two ledgers under Bank Accounts, `TLab Bank A` and `TLab Bank B`, no opening balance; five Contras of 50,000.00 typed on Tally's screen, as the run sheet lists them.
- **Sender:** `curl.exe` from the lab's run sheet; the request is byte for byte the file the lab sent (UTF-16LE with a BOM, no line breaks), and the response is byte for byte the file written with `-o`.
- **Encoding:** the response is BOM-less UTF-16LE with CRLF line ends, exactly as received. `.gitattributes` marks this tree `-text`.
- **What the capture shows:** five `VOUCHER` rows, each with two ledger entries and one `BANKALLOCATIONS.LIST` on each entry. Voucher 2 (bank allocation only, type `Same Bank Transfer`) carries `INSTRUMENTNUMBER` `900000000002` on both entries; voucher 5 carries a cheque number `123456` (type `Cheque` on one entry, `Cheque/DD` on the other); vouchers 1, 3 and 4 carry no instrument number (the reference of 1 and 4 was typed in the narration only, and 3 has none). The voucher-level `REFERENCE` is empty on every voucher.
- **Not established:** other bank-allocation types (NEFT, RTGS, IMPS), whether a UTR is kept in `TRANSACTIONID` under any type (it was empty here), a cash deposit or withdrawal, Gold, other releases, and a book with several currencies.

| file | bytes | sha256 | content |
|---|---|---|---|
| `voucher_contra_hand_keyed_transfer_lab_request.utf16le.xml` | 1746 | `38f572e26ce010db638754aafe152d67999dd670de2bf9869e54628905694375` | the request for 2026-04-01 to 2026-04-05 on `BRIDGE TRANSFER LAB` (UTF-16LE with a BOM) |
| `voucher_contra_hand_keyed_transfer_lab_live.utf16le.xml` | 210796 | `e67c0abfe375351e8b60282efa46754ad029999e6d00335077034575651f4add` | its answer: five Contras, ten entries |
