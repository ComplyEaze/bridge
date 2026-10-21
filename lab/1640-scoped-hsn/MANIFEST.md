# lab/1640-scoped-hsn: Tally's answer to the rates request the merged code sends (#1331 + #1525 + #1640)

Read-only capture, 21 Oct 2026 (14:02-14:03 IST, 08:32-08:33 UTC), synthetic company **BRIDGE PILOT LAB** (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, the only company loaded), TallyPrime Silver 7.1, Windows. Claude Desktop and every ComplyEaze Bridge process were not running. Each request was sent once, one at a time, by curl through a sender script that refuses to run while Claude Desktop or any bridge process runs; no request was retried; every answer was written straight to its file. Nothing was written to Tally. The answers are kept whole, as received (UTF-16LE, no byte order mark), and have not been edited.

The request `requests/req-1640-q1-hsn-rates-four-parents.utf16le.xml` is the lab's own four-parent rates request of 10 Oct 2026 with the one edit that turns the lab's unrestricted request into its 12 Oct HSN one (`ROUNDINGLIMIT</FETCH>` becomes `ROUNDINGLIMIT, HSNDETAILS.LIST, GSTTYPEOFSUPPLY, GSTAPPLICABLE</FETCH>`); the same edit applied to the lab's unrestricted request gives the 12 Oct request (`03de8fa7…`). Until now the repository served a derived answer for it.

## Company marks (VERIFIED unchanged)
`ALTVCHID` 68 / `ALTMSTID` 245 in `ans-r02-1` and `ans-r02-2` (byte-identical, `8a0c4268…`). The company list (`ans-r01-1`, `ans-r01-2`, byte-identical, `0cb23618…`) holds only BRIDGE PILOT LAB.

## Findings
- **VERIFIED:** `ans-q1hsn-1` and `ans-q1hsn-2` are byte-identical (63,016 bytes = Content-Length, ends `</ENVELOPE>`); likewise the catalogue pair (25,010 bytes).
- **VERIFIED:** 13 `<LEDGER NAME=` rows, counted by structure; every `PARENT` is one of `Duties & Taxes`, `Indirect Expenses`, `Sales Accounts`, `Sundry Debtors`; every row carries `BRIDGECOMPANYGUID` equal to the company GUID; the rows' names and GUIDs are exactly the catalogue's ledgers under those four parents (the catalogue holds 18 ledgers).
- **VERIFIED:** every row carries an `HSNDETAILS.LIST` element; `GSTTYPEOFSUPPLY` is present on 7 rows and `GSTAPPLICABLE` on 7; six rows have an empty `HSNDETAILS.LIST` element.
- **VERIFIED:** row for row, every child element including `ALTERID` equals the 12 Oct whole-book HSN answer filtered to the same four parents, in the same order. Against the 10 Oct scoped answer (no HSN fields) the 11 common rows differ only in the three new fields; the two extra rows are `BRIDGE Svc 4D 5%` and `BRIDGE Svc INH 5%`, added since.
- **VERIFIED:** the answers contain the character reference `&#4;`, which a strict XML parser refuses; the analysis stripped C0 references only for parsing.
- **UNVERIFIED:** nothing here is on the W18 book state (marks 60 / 243); this is the live book at 68 / 245.

## Files
| File | What it is | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `.gitattributes` | byte-exact: no line-ending normalisation | 66 | `d13bfb7ad438ed990b12090ee49e38d6d751345e951ff6048bc7ae5136a74149` |
| `answers/ans-q1hsn-1.headers.txt` | HTTP response headers of ans-q1hsn-1 | 135 | `2199868e5b3dde4854764e3555d14445fc729158785829349bab4abfd89a5f6c` |
| `answers/ans-q1hsn-1.xml` | **rates request, first send: Tally's answer** | 63,016 | `a13520cf11d79e09914088674da78254bb57dcac8916cdd9818c1809e57fe3cf` |
| `answers/ans-q1hsn-2.headers.txt` | HTTP response headers of ans-q1hsn-2 | 135 | `2199868e5b3dde4854764e3555d14445fc729158785829349bab4abfd89a5f6c` |
| `answers/ans-q1hsn-2.xml` | rates request, second send (identical to the first) | 63,016 | `a13520cf11d79e09914088674da78254bb57dcac8916cdd9818c1809e57fe3cf` |
| `answers/ans-r01-1.headers.txt` | HTTP response headers of ans-r01-1 | 134 | `24c5ec3f724e89f460c44b9215b8003f9be39e7704b34fc013dbf72feb6535eb` |
| `answers/ans-r01-1.xml` | company list, before (1st read) | 4,000 | `0cb23618073bc5eeddbc3441fd4f0da9e84c55699cd206b66d1410b304f8d147` |
| `answers/ans-r01-2.headers.txt` | HTTP response headers of ans-r01-2 | 134 | `24c5ec3f724e89f460c44b9215b8003f9be39e7704b34fc013dbf72feb6535eb` |
| `answers/ans-r01-2.xml` | company list, after (identical to the 1st) | 4,000 | `0cb23618073bc5eeddbc3441fd4f0da9e84c55699cd206b66d1410b304f8d147` |
| `answers/ans-r02-1.headers.txt` | HTTP response headers of ans-r02-1 | 134 | `a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0` |
| `answers/ans-r02-1.xml` | marks, before | 3,394 | `8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6` |
| `answers/ans-r02-2.headers.txt` | HTTP response headers of ans-r02-2 | 134 | `a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0` |
| `answers/ans-r02-2.xml` | marks, after (identical to before) | 3,394 | `8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6` |
| `answers/ans-r13-1.headers.txt` | HTTP response headers of ans-r13-1 | 135 | `541c0125aa0c233851015d6c8fc3fc78cb85f189c7a30e3dd3e94554630a34c4` |
| `answers/ans-r13-1.xml` | ledger catalogue v2, first send | 25,010 | `873803dd0ab3eab2e0a4dc315afb109f40e11fe3b81cf96fc915c04a8df97a6c` |
| `answers/ans-r13-2.headers.txt` | HTTP response headers of ans-r13-2 | 135 | `541c0125aa0c233851015d6c8fc3fc78cb85f189c7a30e3dd3e94554630a34c4` |
| `answers/ans-r13-2.xml` | ledger catalogue v2, second send (identical) | 25,010 | `873803dd0ab3eab2e0a4dc315afb109f40e11fe3b81cf96fc915c04a8df97a6c` |
| `answers/s00-status.body.txt` | GET /status body | 51 | `655415972de8e54d65743a548e00c2218810683fc0c4cab76cf6ea97ff1d3800` |
| `answers/s00-status.headers.txt` | HTTP response headers of s00-status | 130 | `103743ec081980e5be713e310cf850a1997b40d82ce213812df437627d5d9827` |
| `answers/s01-status.body.txt` | GET /status body | 51 | `655415972de8e54d65743a548e00c2218810683fc0c4cab76cf6ea97ff1d3800` |
| `answers/s01-status.headers.txt` | HTTP response headers of s01-status | 130 | `103743ec081980e5be713e310cf850a1997b40d82ce213812df437627d5d9827` |
| `answers/s02-status.body.txt` | GET /status body | 51 | `655415972de8e54d65743a548e00c2218810683fc0c4cab76cf6ea97ff1d3800` |
| `answers/s02-status.headers.txt` | HTTP response headers of s02-status | 130 | `103743ec081980e5be713e310cf850a1997b40d82ce213812df437627d5d9827` |
| `answers/s03-status.body.txt` | GET /status body | 51 | `655415972de8e54d65743a548e00c2218810683fc0c4cab76cf6ea97ff1d3800` |
| `answers/s03-status.headers.txt` | HTTP response headers of s03-status | 130 | `103743ec081980e5be713e310cf850a1997b40d82ce213812df437627d5d9827` |
| `answers/s04-status.body.txt` | GET /status body | 51 | `655415972de8e54d65743a548e00c2218810683fc0c4cab76cf6ea97ff1d3800` |
| `answers/s04-status.headers.txt` | HTTP response headers of s04-status | 130 | `103743ec081980e5be713e310cf850a1997b40d82ce213812df437627d5d9827` |
| `requests/r01-company-extent.xml` | company list (the lab's r01), UTF-16LE with BOM | 2,524 | `9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8` |
| `requests/r02-marks.xml` | change counters (the lab's r02), UTF-16LE with BOM | 976 | `81892a744009b6cf462a6f8d2fd505e650f8ad022e1462b23414cf2b8b3d2d7c` |
| `requests/r13-ledger-catalogue-v2.xml` | ledger catalogue v2 (the lab's r13), UTF-16LE with BOM | 2,106 | `28a47c26a29fa799856c112b71e19076946d4c0f85a6dcc65f5cdafe0e1e1bd0` |
| `requests/req-1640-q1-hsn-rates-four-parents.utf16le.xml` | **the request as sent**: the lab's four-parent rates request plus `HSNDETAILS.LIST, GSTTYPEOFSUPPLY, GSTAPPLICABLE` (UTF-16LE, BOM) | 2,404 | `3f5151473ba2c7a69a19fd7c0373fbed13b7dc890744a66976dfc961a3d2bcdc` |
| `send-log.txt` | one line per request, UTC, written from the sender log with host, user and folder names removed | 3,336 | `bc8d088b5ff51d403e21a56ac2f6036d0ed6eb7d9726d2bbdb2f9cd599f1b33a` |

`send-log.txt` was written from the sender's log; it is the only file not produced by Tally or taken from the lab's request files.
