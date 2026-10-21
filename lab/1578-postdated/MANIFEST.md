# lab/1578-postdated: does Tally return `ISPOSTDATED` on the read-back request the post uses? (#1578)

Read-only capture, 21 Oct 2026 (14:50-14:51 IST, 09:20-09:21 UTC), synthetic company **BRIDGE PILOT LAB** (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, the only company loaded), TallyPrime Silver 7.1, Windows. Claude Desktop and every ComplyEaze Bridge process were not running. One request at a time, none retried, by curl through a sender script that refuses to run while Claude Desktop or any bridge process runs; every answer was written straight to its file, kept whole as received (UTF-16LE, no byte order mark), unedited. Nothing was written to Tally.

Background: the post's read-back (request `352dfca9…`, answered by the lab on 10 Oct 2026) fetches `ISCANCELLED` and `ISOPTIONAL` but not `ISPOSTDATED`; #1578 merged with that limit stated. `ISPOSTDATED` was measured before only on the status-window request (12-13 Oct 2026).

Requests: D1-D4 are the lab's status-window request of 12 Oct 2026 (`68aaa000…`) with only its two dates changed, four windows of at most 93 days covering the financial year 2026-27; C0 is the read-back request as the code sends it today for `BP/26-27/0010`; R1 is C0 with `, ISPOSTDATED` appended to the field list after `ISGSTOVERRIDDEN`.

## Company marks (VERIFIED unchanged)
`ALTVCHID` 68 / `ALTMSTID` 245 in `ans-r02-1` and `ans-r02-2` (byte-identical, `8a0c4268…`). The company list (`ans-r01-1`, `ans-r01-2`, byte-identical, `0cb23618…`) holds only BRIDGE PILOT LAB.

## Findings
- **VERIFIED:** the book holds no post-dated `BRIDGE Sales` voucher. D1, D3 and D4 are empty collections; D2 lists 33 vouchers dated 1 Aug to 15 Sep 2026, each with `ISPOSTDATED` exactly once as a direct child, all `No` (one cancelled, `BP/26-27/0041`, one optional, `BP/26-27/0042`). Only that voucher type was listed.
- **VERIFIED:** R1 returns `ISPOSTDATED` once, as a direct child of the voucher, as `<ISPOSTDATED TYPE="Logical">No</ISPOSTDATED>`, placed after `VCHGSTSTATUSISEXCLUDED`. The R1 pair (94,474 bytes) and the C0 pair (94,372 bytes) are each byte-identical.
- **VERIFIED:** R1's answer with that one line (and its line break) removed equals C0's answer byte for byte; nothing else in the answer moves. The voucher reads included Yes, uncertain No, excluded No, `ISGSTOVERRIDDEN` No, AlterID 17.
- **VERIFIED:** C0 equals the lab's 10 Oct 2026 answer except five counters in the answer's header (`LEDGER`, `CURRENCY`, `TAXUNIT`, `VOUCHERNUMBERSERIES`, `VOUCHER`), which Tally fills from the objects its session has loaded.
- **UNVERIFIED (not measured):** the reading `ISPOSTDATED` Yes on the read-back request. It needs a post-dated voucher, and the book holds none.

Method: counted by structure with an XML parser (C0 character references such as `&#4;` stripped for parsing only), not by text search.

## Files
| File | What it is | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `.gitattributes` | byte-exact: no line-ending normalisation | 66 | `d13bfb7ad438ed990b12090ee49e38d6d751345e951ff6048bc7ae5136a74149` |
| `answers/ans-c0-1.headers.txt` | HTTP response headers of ans-c0-1 | 135 | `4518b42593e569f3ff133f1272d7e3cd688913c61747447f80e33e2426790d58` |
| `answers/ans-c0-1.xml` | C0, first send | 94,372 | `a02dae7c6774e8da781ef9bdfcddeb560a993404e8858701e0e885ba2e6e502a` |
| `answers/ans-c0-2.headers.txt` | HTTP response headers of ans-c0-2 | 135 | `4518b42593e569f3ff133f1272d7e3cd688913c61747447f80e33e2426790d58` |
| `answers/ans-c0-2.xml` | C0, second send (identical) | 94,372 | `a02dae7c6774e8da781ef9bdfcddeb560a993404e8858701e0e885ba2e6e502a` |
| `answers/ans-d1.headers.txt` | HTTP response headers of ans-d1 | 134 | `d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf` |
| `answers/ans-d1.xml` | D1: empty collection | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` |
| `answers/ans-d2.headers.txt` | HTTP response headers of ans-d2 | 136 | `ac7f3cc1fb9e245b57efa5bc922f2ff1d46a61cd4eac1c08e8d6879607acbd3d` |
| `answers/ans-d2.xml` | D2: 33 `BRIDGE Sales` vouchers, 1 Aug to 15 Sep 2026 | 111,754 | `067b2d0e3a7aef2cef96aa5c5e319c14e13905abf1ffe1a9f562907fe84c73ea` |
| `answers/ans-d3.headers.txt` | HTTP response headers of ans-d3 | 134 | `d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf` |
| `answers/ans-d3.xml` | D3: empty collection | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` |
| `answers/ans-d4.headers.txt` | HTTP response headers of ans-d4 | 134 | `d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf` |
| `answers/ans-d4.xml` | D4: empty collection | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` |
| `answers/ans-r01-1.headers.txt` | HTTP response headers of ans-r01-1 | 134 | `24c5ec3f724e89f460c44b9215b8003f9be39e7704b34fc013dbf72feb6535eb` |
| `answers/ans-r01-1.xml` | company list, before | 4,000 | `0cb23618073bc5eeddbc3441fd4f0da9e84c55699cd206b66d1410b304f8d147` |
| `answers/ans-r01-2.headers.txt` | HTTP response headers of ans-r01-2 | 134 | `24c5ec3f724e89f460c44b9215b8003f9be39e7704b34fc013dbf72feb6535eb` |
| `answers/ans-r01-2.xml` | company list, after (identical) | 4,000 | `0cb23618073bc5eeddbc3441fd4f0da9e84c55699cd206b66d1410b304f8d147` |
| `answers/ans-r02-1.headers.txt` | HTTP response headers of ans-r02-1 | 134 | `a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0` |
| `answers/ans-r02-1.xml` | marks, before | 3,394 | `8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6` |
| `answers/ans-r02-2.headers.txt` | HTTP response headers of ans-r02-2 | 134 | `a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0` |
| `answers/ans-r02-2.xml` | marks, after (identical) | 3,394 | `8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6` |
| `answers/ans-r1-1.headers.txt` | HTTP response headers of ans-r1-1 | 135 | `9523815f0cd64ed1063c0719c79a4d451fc55ee775ac2bde38a267dbea6dbc73` |
| `answers/ans-r1-1.xml` | **R1, first send** | 94,474 | `b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5` |
| `answers/ans-r1-2.headers.txt` | HTTP response headers of ans-r1-2 | 135 | `9523815f0cd64ed1063c0719c79a4d451fc55ee775ac2bde38a267dbea6dbc73` |
| `answers/ans-r1-2.xml` | R1, second send (identical) | 94,474 | `b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5` |
| `answers/s00-status.body.txt` | GET /status body | 51 | `655415972de8e54d65743a548e00c2218810683fc0c4cab76cf6ea97ff1d3800` |
| `answers/s00-status.headers.txt` | HTTP response headers of s00-status | 130 | `103743ec081980e5be713e310cf850a1997b40d82ce213812df437627d5d9827` |
| `answers/s01-status.body.txt` | GET /status body | 51 | `655415972de8e54d65743a548e00c2218810683fc0c4cab76cf6ea97ff1d3800` |
| `answers/s01-status.headers.txt` | HTTP response headers of s01-status | 130 | `103743ec081980e5be713e310cf850a1997b40d82ce213812df437627d5d9827` |
| `answers/s02-status.body.txt` | GET /status body | 51 | `655415972de8e54d65743a548e00c2218810683fc0c4cab76cf6ea97ff1d3800` |
| `answers/s02-status.headers.txt` | HTTP response headers of s02-status | 130 | `103743ec081980e5be713e310cf850a1997b40d82ce213812df437627d5d9827` |
| `answers/s03-status.body.txt` | GET /status body | 51 | `655415972de8e54d65743a548e00c2218810683fc0c4cab76cf6ea97ff1d3800` |
| `answers/s03-status.headers.txt` | HTTP response headers of s03-status | 130 | `103743ec081980e5be713e310cf850a1997b40d82ce213812df437627d5d9827` |
| `requests/r01-company-extent.xml` | company list (the lab's r01), UTF-16LE with BOM | 2,524 | `9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8` |
| `requests/r02-marks.xml` | change counters (the lab's r02), UTF-16LE with BOM | 976 | `81892a744009b6cf462a6f8d2fd505e650f8ad022e1462b23414cf2b8b3d2d7c` |
| `requests/req-1578-c0-readback-0010-control.utf16le.xml` | **C0**: the read-back request the post sends, voucher `BP/26-27/0010` (the lab's 10 Oct request file, unchanged) | 2,252 | `352dfca912a53499f058643e61db4febc301f6b53b753de54d7f2aac475db9c1` |
| `requests/req-1578-d1-status-20260401-20260630.utf16le.xml` | **D1**: the status-window request (type `BRIDGE Sales`, by name) for 20260401 to 20260630: the lab's 12 Oct request with only the two dates changed | 1,846 | `fff8978802e3ebefcd4d7917b6ceb95aed99bc53267fcd9ef6c6c01167864225` |
| `requests/req-1578-d2-status-20260701-20260930.utf16le.xml` | **D2**: the status-window request (type `BRIDGE Sales`, by name) for 20260701 to 20260930: the lab's 12 Oct request with only the two dates changed | 1,846 | `02f28d479813561a32a3612b0bde1642da19cd8e29e8eb341a47720d042cd350` |
| `requests/req-1578-d3-status-20261001-20261231.utf16le.xml` | **D3**: the status-window request (type `BRIDGE Sales`, by name) for 20261001 to 20261231: the lab's 12 Oct request with only the two dates changed | 1,846 | `860d7dbc902758b982b71c6df654718b3d95b4e81aeeab6ac88f0ebcf9ee5064` |
| `requests/req-1578-d4-status-20270101-20270331.utf16le.xml` | **D4**: the status-window request (type `BRIDGE Sales`, by name) for 20270101 to 20270331: the lab's 12 Oct request with only the two dates changed | 1,846 | `5002b3193da0c051c05a01540bfef45db9f3a1448006a921ed595c734e3fd3bc` |
| `requests/req-1578-r1-readback-0010-ispostdated.utf16le.xml` | **R1**: C0 plus `, ISPOSTDATED` after `ISGSTOVERRIDDEN` (the one edit) | 2,278 | `10f9711cc936c9c4d0f70bccb42b54cc10fbd5e7de11287737329cfe7ae56c21` |
| `send-log.txt` | one line per request, UTC, written from the sender log with host, user and folder names removed | 4,343 | `9a8bad81d59a105b86d1f5bb6ab2a344b2cc5c2847e14fe9605c4f43af96bd8f` |

`send-log.txt` was written from the sender's log; it is the only file not produced by Tally or taken from the lab's request files.
