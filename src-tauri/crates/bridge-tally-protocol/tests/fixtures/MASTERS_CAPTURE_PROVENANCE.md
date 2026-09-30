# `masters_*` — provenance

Voucher types, godowns, units and stock groups, each read as one native collection with the company GUID computed onto every row. These are the fixtures for the `masters` read (#724 step 1 for voucher types). Every file is a live capture, never hand-written.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance).
- **Date:** 2026-09-30, between 19:14:34 and 19:14:37 IST, one request at a time. `tally_status` was healthy before the first request and after every request. The company was confirmed loaded before anything was sent.
- **Book:** `BRIDGE SHAPE LAB`, a synthetic lab company. Its book extent (`ALTMSTID` 289, `ALTVCHID` 111) was read before and after the four requests and was equal.
- **Encoding:** responses are **BOM-less UTF-16LE**, exactly as received. Requests are the exact bytes sent: **UTF-16LE with a BOM**. `.gitattributes` marks this tree `-text`.
- **Request shape:**
  - Voucher types: the production `render_native_voucher_type_export_request` text with `ISACTIVE, ISOPTIONAL, NUMBERINGMETHOD` added to its `FETCH`, and the `BRIDGECOMPANYGUID` compute that the group and ledger snapshots already send.
  - Godowns, units and stock groups: a collection of that type (`ISMODIFY="No"`) with the fields in the table, and the same compute.
  - No filter, no dates, no `$$` function other than `$$SysName:XML`.
- **What the captures show:**
  - Every row of every kind carries `BRIDGECOMPANYGUID` equal to the company's GUID.
  - A collection with rows is present as one `COLLECTION` element.
  - `NUMBERINGMETHOD` is a direct child of every voucher-type row. Its values here are `Default` (24), `Automatic` (1) and `Manual` (1).

| file | bytes | sha256 | kind | rows |
|---|---|---|---|---|
| `masters_voucher_types_request.utf16le.xml` | 1188 | `4359b5366ea77cb57bbbd72c6c1fee3b2facef7cb6dc1c079492a2988fc628db` | request, voucher types | n/a |
| `masters_voucher_types_shape_lab_live.utf16le.xml` | 48456 | `91c7e56cc3810b8b266ac21b17ca2f08cfce3d0174db16149bcf55a4c9d4e4d1` | voucher types | 26 |
| `masters_godowns_request.utf16le.xml` | 1102 | `a2a63164752bd2c438de7a27ca4cb854cb0bf5b1a60db3723986370140f051f3` | request, godowns | n/a |
| `masters_godowns_shape_lab_live.utf16le.xml` | 5190 | `2ecfd80804b3691776888982187f03401b14ddbf0e48a6a4823dd45b93a5ab9a` | godowns | 2 |
| `masters_units_request.utf16le.xml` | 1160 | `c03da62e706f063bcd88fb2a9d9d5df11ec3347a305fc6de5bf5395d283de7ed` | request, units | n/a |
| `masters_units_shape_lab_live.utf16le.xml` | 6732 | `50c606821031ddfe15740b5a318596cbeb164fff77bad710131dee3c6da52d6e` | units | 4 |
| `masters_stock_groups_request.utf16le.xml` | 1130 | `8f8737ee8f899a2c052d31c57522a9db41a4eeda9162b17ca51b50bf7d602838` | request, stock groups | n/a |
| `masters_stock_groups_shape_lab_live.utf16le.xml` | 6166 | `032564c60fcbd54db5231b87ed20c9fb5b28b116ce146133b15b9841113f4dc8` | stock groups | 3 |
