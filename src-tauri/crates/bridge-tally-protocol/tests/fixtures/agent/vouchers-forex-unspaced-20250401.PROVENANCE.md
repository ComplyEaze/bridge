# `vouchers-forex-unspaced-20250401`: provenance

A raw `vouchers` window read of the synthetic several-currency lab book on TallyPrime 7.2, the first voucher answer in that release's composite layout (#683).

## Capture

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.2, one loaded company.
- **Date:** 2026-11-02. Sent once; the answer kept byte for byte. Source: `raw/vouchers.response` on branch `lab/683-v72-voucher-answers` at `03940570`.
- **Company:** `BRIDGE FOREXGROUP LAB`, a synthetic lab book, GUID `1bb48be5-baba-45b2-90d8-7593244915a9`. Base currency master NAME `₹`, plus a `$` master. Its marks were equal before and after the read.
- **Window:** 2025-04-01 to 2025-04-01, one request, one part.
- **Request:** `vouchers-forex-unspaced-20250401.request.xml`, the bytes of `vouchers-forex-composite-20260915.request.xml` with this company and date; the fetch list is unchanged. It was sent as UTF-16LE with a byte-order mark.
- **Response:** BOM-less UTF-16LE, exactly as received.

| file | bytes | sha256 |
|---|---|---|
| `vouchers-forex-unspaced-20250401.utf16le.xml` | 387856 | `92e0e0548596cb6103f974c7d8b44afdea09c39ee8fcaf425ddceb3ae67c9369` |
| `vouchers-forex-unspaced-20250401.request.xml` | 860 | `a105626a9f3ab64988c1c0041715f4a593086d07e87efba599ff8fa9407c4f1a` |

## What it establishes

- **Eleven vouchers:** 3 Sales, 3 Receipt, 2 Purchase, 2 Payment, 1 Journal. Eight hold composites, each on both of its entries; the other three are plain.
- **The layout.** A voucher entry and its bill allocation hold `-$100.00 @ ₹ 86/$ = -₹ 8600.00`: the layout 7.2 wrote into ledger balances, with the foreign amount against its symbol and one space before ` = `. Twenty-eight `AMOUNT` elements hold a composite, and twelve `VATEXPAMOUNT` elements; the voucher parser does not read the second.
- **A decimal rate.** `-$100.00 @ ₹ 86.4125/$ = -₹ 8641.25` on one Sales voucher.
- **Netting.** Each voucher's entries net to zero on the amounts after `=`. On every composite here that amount equals the foreign amount times the rate exactly.

## Limits

- One release (7.2), Silver, one book, one read, one date, one currency (`$`). No rounding case: foreign amount times rate is a whole paisa on every entry. Partial.
