# `voucher_cost_centre_allocations_keyed_*` — provenance

Two live captures of the request the vouchers read sends (`Bridge Agent Vouchers`, a `Voucher` collection with the `FETCH` list that ends `ALLLEDGERENTRIES.*,REFERENCE,ISINVOICE,PARTYGSTIN,REFERENCEDATE`), one day each, on a synthetic lab book with Cost Centres on, with the requests that were sent. Protocol reference §8.2g.

## Provenance

- **Host / gateway:** TallyPrime **Silver**, 7.2, `http://127.0.0.1:9001` (a lab instance, Windows).
- **Date:** 2026-10-27, attended, read only, one request at a time, no client time limit; HTTP 200, `RESPSTATUS: 1`, and `CONTENT-LENGTH` equal to the bytes received.
- **Book:** `BRIDGE COST KEY LAB`, a synthetic company created in the lab that day (books from 2025-04-01), loaded alone.
  - Its Cost Centres setting was set to Yes on Tally's screen.
  - Two categories and four centres were made on the screen: one centre under another, and one centre in the second category.
  - Two vouchers were keyed on the screen: a Payment on 2025-05-15 and a Journal on 2025-05-16.
- **Sender:** `curl.exe` from the lab's run sheet. Each request is byte for byte the file the lab sent (UTF-16LE with a BOM, no line breaks), and each response is byte for byte the file written with `-o`. Lab branches `lab/1231-cost-centre-keyed-allocation` (the sheet and requests) and `lab/1231-cost-centre-keyed-allocation-answers` (the answers).
- **Encoding:** each response is BOM-less UTF-16LE with CRLF line ends, exactly as received. `.gitattributes` marks this tree `-text`.
- **What the captures show:**
  - The Payment's expense entry carries one `CATEGORYALLOCATIONS.LIST` with two `COSTCENTREALLOCATIONS.LIST` rows, one of them for the centre that sits under the other.
  - The Journal's expense entry carries two `CATEGORYALLOCATIONS.LIST`, one per category. Each holds centres that add up to the whole entry.
  - Each voucher's bank entry carries none.
- **Not established:** quantities, foreign-currency amounts, other voucher types, an item invoice, an entry whose allocations do not add up to it, Gold, and other releases.

| file | bytes | sha256 | content |
|---|---|---|---|
| `voucher_cost_centre_allocations_keyed_payment_request.utf16le.xml` | 1746 | `55ab4f02676e509232c833d2c8946509b50cc537cc5e06bf835b961aae29afdb` | the request for 2025-05-15 on `BRIDGE COST KEY LAB` (UTF-16LE with a BOM) |
| `voucher_cost_centre_allocations_keyed_payment_live.utf16le.xml` | 42960 | `87924f5a91974bbd333c351a97b989175f1f9bfcea4d59a331b5516b47ed81fb` | its answer: one Payment, two entries, one category with two centres |
| `voucher_cost_centre_allocations_keyed_journal_request.utf16le.xml` | 1746 | `0daaa90c969c286a5b4e47531eb0a1305848f5299ab09f79703c8d2b2a7a4357` | the request for 2025-05-16 on `BRIDGE COST KEY LAB` (UTF-16LE with a BOM) |
| `voucher_cost_centre_allocations_keyed_journal_live.utf16le.xml` | 43256 | `e6406e157365b11930940e8362b6e99ac181905d4b69df844519b175221ed29a` | its answer: one Journal, two entries, two categories on one entry |
