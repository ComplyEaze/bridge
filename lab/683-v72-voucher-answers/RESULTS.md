# Results: row 683-V72 (#683), one raw vouchers answer from TallyPrime 7.2, 2 Nov 2026

**Tally answered the request with all 11 vouchers of 1 April 2025 on BRIDGE FOREXGROUP LAB, 387,856 B, saved whole. No mark moved.**

- **Book:** BRIDGE FOREXGROUP LAB (synthetic), GUID 1bb48be5-baba-45b2-90d8-7593244915a9, TallyPrime 7.2 Silver, loaded alone, as left at BK4. Nothing keyed, nothing restored.
- **Request:** one raw `Bridge Agent Vouchers` read. The bytes of `tests/fixtures/agent/vouchers-forex-composite-20260915.request.xml` from master 4cdfe448, with SVCURRENTCOMPANY set to the book and each of the four dates set to 20250401; the fetch list is unchanged. Sent as UTF-16LE with a byte-order mark, as the lab's marks request is. Both forms are in `requests/`.
- **Why raw:** master's `vouchers` tool refuses this layout and keeps no answer.

| step | answer | status | bytes = CONTENT-LENGTH | sha256 |
|---|---|---|---|---|
| marks before | `raw/m0.response` | 200 | 3404 = 3404 | 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba |
| vouchers | `raw/vouchers.response` | 200 | 387856 = 387856 | 92e0e0548596cb6103f974c7d8b44afdea09c39ee8fcaf425ddceb3ae67c9369 |
| marks after | `raw/m1.response` | 200 | 3404 = 3404 | 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba |

- **Marks:** ALTVCHID 11, ALTMSTID 214, one company row with the row's GUID, before and after; the two answers are byte-equal. EXPECT met. [verified]
- **Vouchers:** 11, by GUID, with AlterID and MasterID 1 to 11: 3 Sales, 3 Receipt, 2 Purchase, 2 Payment, 1 Journal; all dated 20250401; none cancelled or optional. EXPECT met. [verified]
- **Size:** 387,856 B; EXPECT a few hundred KB. [verified]
- **Close:** Company > Shut; Select Company with no company loaded; one tally.exe owns 9001; `GET /status` 200. [verified]
- **Count note:** a plain search for `<VOUCHER` gives 12, because Tally's header counts block holds `<VOUCHER>0</VOUCHER>`. Only 11 are voucher objects, each with REMOTEID, VCHKEY and VCHTYPE attributes.

**Confidence:** one read of one book: PARTIAL for anything beyond this book and date.

## Deviations
1. The first attempt to load the book, earlier the same sitting, was overtaken by the owner's own login to another company. Nothing reached this book, and no request was sent. The row ran after that company was shut.
2. The fixture is UTF-8 and the wire request is UTF-16LE, so the request's bytes differ from the fixture's by encoding as well as by the four dates and the company name.
