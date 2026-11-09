# `voucher_contra_hand_keyed_bankref_lab_*` — provenance

A live capture of the request the vouchers read sends (`Bridge Agent Vouchers`, a `Voucher` collection with the `FETCH` list that ends `ALLLEDGERENTRIES.*,REFERENCE,ISINVOICE,PARTYGSTIN,REFERENCEDATE`), for 1 to 5 April 2026, on a synthetic lab book of five Contras keyed by hand in double entry between two bank ledgers, with the request that was sent. Protocol reference §8.2h.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.2, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance, Windows).
- **Date:** 2026-10-28, attended, read only, one request at a time, no client time limit; HTTP 200 and `RESPSTATUS: 1`, and a size equal to its `CONTENT-LENGTH` (211,536 bytes).
- **Book:** `BRIDGE BANKREF LAB`, a synthetic lab company (books from 2026-04-01, no inventory, no GST), loaded alone. Two ledgers under Bank Accounts, `TLab Bank A` and `TLab Bank B`, no opening balance; five Contras of 50,000.00 keyed on Tally's screen in double entry (a debit entry and a credit entry, each with its own bank allocation), as the run sheet lists them. Voucher 2's bank allocation was chosen on screen as e-Fund Transfer, which Tally stores as the transaction type `Same Bank Transfer`.
- **Why a second capture:** an earlier run of the same five vouchers (2026-10-12, TallyPrime 7.1, `BRIDGE TRANSFER LAB`) used references that the repository's privacy scan reads as identifiers, so its parse record, tests and notes could not name them, and a capture is never edited. This run keyed the same shapes again, in a new company, with references of one repeated digit. The first run's capture is not committed.
- **Source:** the lab branch `lab/2006-bankref-recapture-answers` at `e49f2d0823a417c002fee29afdad7deb2cb5d515`: `evidence/ref-contra.response` and `lab/2006-bankref-recapture/V-CONTRA.utf16le.xml`, each listed in that branch's evidence with the sha256 below. Copied here byte for byte.
- **Sender:** `curl.exe` from the lab's run sheet; the request is byte for byte the file the lab sent (UTF-16LE with a BOM, no line breaks) and is the vouchers read's own request with only the company name changed, and the response is byte for byte the file written with `-o`.
- **Encoding:** the response is BOM-less UTF-16LE with CRLF line ends, exactly as received. `.gitattributes` marks this tree `-text`.
- **What the capture shows:** five `VOUCHER` rows, each with two ledger entries and one `BANKALLOCATIONS.LIST` on each entry, counted with an XML parser (ten allocations, every one with children). Voucher 2 (bank allocation only, type `Same Bank Transfer`) carries `INSTRUMENTNUMBER` `222222222222` on both entries; voucher 5 carries a cheque number `123456` on both entries (type `Cheque` on the first, `Cheque/DD` on the second); vouchers 1, 3 and 4 carry no instrument number (the reference of 1 and 4 was typed in the narration only, and 3 has none). The voucher-level `REFERENCE` is empty on every voucher. On all ten allocations `BANKERSDATE` is empty, `STATUS` is `No`, `BANKRECONSTATUS` is `Available Only in Books`, `PAYMENTMODE` is `Transacted` and `TRANSACTIONID` is empty.
- **Not established:** other bank-allocation types (NEFT, RTGS, IMPS), whether a UTR is kept in `TRANSACTIONID` under any type (it was empty here), a cash deposit or withdrawal, a reconciled entry, Gold, other releases, and a book with several currencies.

| file | bytes | sha256 | content |
|---|---|---|---|
| `voucher_contra_hand_keyed_bankref_lab_request.utf16le.xml` | 1744 | `d93946038f2ae6a40b48eb53d4e567352434533fc8b593a5aa766935ecbcc080` | the request for 2026-04-01 to 2026-04-05 on `BRIDGE BANKREF LAB` (UTF-16LE with a BOM) |
| `voucher_contra_hand_keyed_bankref_lab_live.utf16le.xml` | 211536 | `c5b465b3ebfec9df6950b8a520003e08de71c1be4438c95d1d2f64d410d1cc92` | its answer: five Contras, ten entries |
