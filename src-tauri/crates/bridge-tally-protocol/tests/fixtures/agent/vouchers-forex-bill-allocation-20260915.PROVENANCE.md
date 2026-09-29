# `vouchers-forex-bill-allocation-20260915`: provenance

A second live `vouchers` window read of the synthetic several-currency lab book (#674). It holds the first capture's voucher and, besides it, a Receipt whose foreign amount is carried by a bill allocation.

## Capture

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance).
- **Date:** 2026-09-27, 20:38 IST.
- **Company:** `BRIDGE CORPUS FOREX`, a synthetic lab book. Its voucher mark (ALTVCHID) was 19.
- **Window:** 2026-09-15 to 2026-09-15, one request, one part.
- **Request:** byte for byte the first capture's request, `vouchers-forex-composite-20260915.request.xml` (858 bytes, sha256 `dd2352aff9aa5b0b4bed50870d8d4bd27886b1067aa107e7b428e73fde95b324`). It was sent as UTF-16LE with a BOM, as Bridge's own builders emit every request.
- **Response:** BOM-less UTF-16LE, exactly as received (HTTP 200, 6.96 s).
- `tally_status` was read before and after, with the same seven loaded synthetic companies and no client book.

| file | bytes | sha256 |
|---|---|---|
| `vouchers-forex-bill-allocation-20260915.utf16le.xml` | 68784 | `4bc80bf36efe4dc9c9815917e415ade21a286ef6ccadea9f014ed7df9054ac4b` |

## What it establishes

- **Two vouchers** (counted by parsing the response, not by substring). Both carry voucher number `1`.
- **The Sales voucher, ALTERID 18.** Its voucher element is byte-identical to the first capture's (`vouchers-forex-composite-20260915`).
- **The Receipt, ALTERID 19.** It was entered as `$40 @ 88` against the Sales voucher's bill.
  - Tally stores `$ 40.00 @ I₹ 88/$  = I₹ 3520.00` as the AMOUNT of the party entry and of its bill allocation (BILLTYPE `Agst Ref`).
  - It stores `-$ 40.00 @ I₹ 88/$  = -I₹ 3520.00` as the AMOUNT of the cash entry.
  - The receipt is signed opposite to the sale; each voucher's composites agree in sign within it.
- **Six AMOUNT elements**, all composites: three on each voucher.
- `&#4;` occurs ten times (for example `<GSTCLASS>&#4; Not Applicable</GSTCLASS>`), as in the first capture. It is Tally's reserved-name marker, which the parsers sanitise before reading.

## Screening

- Every name in the response is a synthetic lab name (`FX Party 01`, `FX Sales`, `Cash`), and the one non-empty narration is `C1 Synthetic test`.
- `PARTYGSTIN` is present and empty. The response carries no PAN, contact or address value.
- The company name appears only in the request.

## Limits

- One release (7.1), Silver, one book, one run, one currency (`$`), two rates (86 and 88).
- Receipts against a bill are the only settlement shape captured. A payment, a partial settlement, and a forex gain or loss entry are not.
