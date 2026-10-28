# `company_marks_empty_company_no_voucher_mark_live`: provenance

The company marks of a company that has never held a voucher, captured as received.

- **Host / gateway:** TallyPrime **Silver (licensed)** 7.2, not Educational mode.
- **Date:** 2026-10-26, read-only, one request at a time.
- **Company:** the synthetic book `BRIDGE HANDIMPORT LAB` (company number 100081, GUID
  `9b5b6d54-4cdb-4b90-9600-cab45ee70c3a`, books from 1-Apr-2025), created in the lab that day and
  loaded alone. It held only Tally's own groups and ledgers plus three set-up ledgers, and no
  voucher.
- **Request:** the GUID-filtered `BridgeCompanyBookExtentV2` collection. Its decoded text equals
  what `render_company_inventory_flags_request` renders for that company and GUID (compared as
  text after decoding); it was sent with a UTF-16LE byte-order mark, kept here.
- **Encoding:** the request is UTF-16LE with a BOM; the response is BOM-less UTF-16LE, the undecoded
  wire bytes. `STATUS 1`, one `COMPANY` row.
- **Copied from:** the lab record branch `lab/sitting4-w2-answers` at `9ed022285`
  (`answers/s0-marks-after-bk1.xml` and `requests/c1-marks-w2.utf16le.xml`), byte for byte; the git
  blob ids are the ones on that branch.

| file | bytes | sha256 |
|---|---|---|
| `company_marks_empty_company_no_voucher_mark_live.utf16le.xml` | 4,818 | `30ad5317b6fe566b420e435df73274286c21e365934227bc53cfcd8a232a7e88` |
| `company_marks_empty_company_no_voucher_mark_request.utf16le.xml` | 1,650 | `7ec76ad8d35e7c931ce920b400a275fba07e99de3d9b7d7d8f9d2da4c69577b4` |

What it shows (verified): the row carries `ALTMSTID 210` and **no `ALTVCHID` element**, while
`LASTVOUCHERDATE` is reported (`20250401`, the books-from date). The `CMPINFO` counters describe the
current company, not the collection, and no parser reads them.

Not committed here: an answer to the `Bridge Agent Company High Water` request the voucher reads
send (that request fetches the same two marks from the same `Company` object). The same lab record
(`lab/sitting4-w2-answers` at `9ed022285`) shows it answering the same way on this book, through two
tool calls: `answers/s2-build_import_xml.json` was refused `empty_book_first_import`, which the code
returns only when that request's answer carries no `ALTVCHID`, and `answers/s1-vouchers.json` reports
`company_has_no_vouchers` with `voucher_alter_id` 0 (the value this change shows as null).

Not established: the same answer from any other TallyPrime release.
