# `vouchers-reference-date`: provenance

A live `vouchers` read of the synthetic book `BRIDGE SHAPE LAB` with `REFERENCEDATE` added to the voucher read's field list (#1257), to see what Tally returns for it.

## Capture

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` through a recording relay, one request at a time, read-only.
- **Date:** 2026-10-07, about 14:20 IST.
- **Company:** `BRIDGE SHAPE LAB`, a synthetic lab book of 67 vouchers in the year 2025-04-01 to 2026-03-31 (plus one voucher keyed for this capture, dated outside that year).
- **Build:** a debug build of the repository at the pull request head with exactly one word, `REFERENCEDATE`, appended to the voucher field list (one variable).
- **Reads:** the whole year (two data parts, each read twice by the paired read: 134 data rows for 67 vouchers) and one day (one voucher, read twice).
- **Request file:** `vouchers-reference-date-20260420.request.xml` is the day read's data request as sent (the request body decoded from UTF-16LE with its byte-order mark, saved as UTF-8; the sha256 of the UTF-16LE bytes as sent is `91abfe080fec2056de2e40b171e7df1362dae77774e2dc62fc85ad9f9b17b715`). It is what `render_agent_vouchers` renders today for this company and day.
- **Response file:** `vouchers-reference-date-year-rows.utf16le.xml` is a TRIMMED copy of the first data part of the year read: the bytes before its first voucher row, then two of its 42 voucher rows, then the bytes after its last row, BOM-less UTF-16LE. The kept rows are the first row whose `REFERENCEDATE` is empty and the one row whose `REFERENCEDATE` is populated; they are cut out by that rule and not edited. The untrimmed part was 1,736,780 bytes with sha256 `8962b162704b850ab1f02f76ff91a4d80efbab94971cb69b39f319d7ea8987de`; the raw captures stay private.

| file | bytes | sha256 |
|---|---|---|
| `vouchers-reference-date-20260420.request.xml` | 869 | `149ddf64fdfd8e6ed991ac08951f00697d3f31cc2b2b05c480a74e9efc336392` |
| `vouchers-reference-date-year-rows.utf16le.xml` | 84584 | `710f96688c6da7537b4bd96f7686401d183fde986f8911f1ae2a96b483a111d8` |

## What it establishes

- **The element comes back on every voucher.** 134 of 134 data rows of the year read, and the one row of the day read, carry `REFERENCEDATE` with `TYPE="Date"`.
- **Two shapes.** Empty: `<REFERENCEDATE TYPE="Date"></REFERENCEDATE>` on 132 rows (66 vouchers). Populated: `<REFERENCEDATE TYPE="Date">YYYYMMDD</REFERENCEDATE>` on two rows (one voucher, a Purchase whose reference date is its own voucher date and which carries a `REFERENCE`) and on the day read's voucher (reference date five days before the voucher date).
- **One variable.** The data requests are byte-equal to the same reads without the word once the word is removed, and the data responses are byte-equal to the same reads without the word once the `REFERENCEDATE` elements and the company counter block (`CMPINFO`) are removed. The word adds 50 bytes a voucher.
- **Not shown.** A reference date on any other voucher type, a voucher with a reference and no reference date, another Tally release, or a book other than this one.

## Screening

- Every name in the kept rows is a synthetic lab name (two ledgers: a supplier and a purchase ledger).
- The day read's own response is not committed: its voucher was keyed for this capture, and the year rows already carry both shapes.
- The company name appears in the request and in the response header.
