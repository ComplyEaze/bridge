# `company_features_*` — provenance

Live captures of one Collection request (`BridgeCompanyFeaturesV1`, a `Company` collection filtered to one GUID) on two synthetic lab books, each with the request that was sent. The request fetches the company's identity fields, `CURRENCYNAME` and 17 `IS...ON` flags; the parser reads three of the flags and the symbol (protocol reference §12a.18).

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance).
- **Date:** 2026-10-07, about 14:18 IST, attended, one request at a time.
- **Books:** `BRIDGE SHAPE LAB` and `BRIDGE CORPUS FOREX`, synthetic lab companies (books from 2025-04-01). The company's setting screen (F11) was looked at by a maintainer for the Cost Centres setting on these two books and a third (not committed): it read No on both of these, equal to the captured `ISCOSTCENTRESON` (recorded, §12a.17). The three settings were later compared with the screen on three other synthetic books, through the built tool (§12a.18); that run is not a capture and commits no bytes.
- **Sender:** a lab capture script, not `bridge_mcp`. It sent the exact request bytes tabled below (UTF-16LE with a BOM); a test asserts that `render_company_features_request` produces them.
- **Encoding:** responses are **BOM-less UTF-16LE**, exactly as received. `.gitattributes` marks this tree `-text`.
- **What the captures show:** one `COMPANY` row under `ENVELOPE/BODY/DATA/COLLECTION`, with the identity fields, `CURRENCYNAME` (the symbol, not an ISO code) and each flag as `<IS...ON TYPE="Logical">Yes|No</...>`. The `CMPINFO` block carries a bare `<COMPANY>0</COMPANY>` counter that is not a row. Across the two books `ISBATCHWISEON` differs (Yes, No); a third synthetic book (recorded, not committed) read `ISCOSTCENTRESON` Yes and `ISGSTON` No. The other 14 flags read the same on every book read.
- **Not established:** that any flag other than the three the tool returns follows its F11 setting (those three were compared with the screen, §12a.18), Education mode, other releases, a book that has several companies sharing a GUID.

| file | bytes | sha256 | content |
|---|---|---|---|
| `company_features_shape_lab_request.utf16le.xml` | 1888 | `4551fc700d0d7070a39f37b812dad9a4b0a30456bedc205ddd77ed7e7bd31caa` | the request for `BRIDGE SHAPE LAB` (UTF-16LE with a BOM) |
| `company_features_shape_lab_live.utf16le.xml` | 5822 | `862bf17e9934f8569e6e3dd412ebb5e46e1b100c82ecec522c735894680ffb29` | its answer: cost centres No, GST Yes, batch-wise Yes |
| `company_features_corpus_forex_request.utf16le.xml` | 1894 | `f859194c1956d94f5c6d70d8c59c6a69038c561c776e64814c67a6311ae0de6f` | the request for `BRIDGE CORPUS FOREX` (UTF-16LE with a BOM) |
| `company_features_corpus_forex_live.utf16le.xml` | 5830 | `e0195996da57416c4f70d80f8e02b5fa8bed5efd2825a55e5687154581214d5a` | its answer: cost centres No, GST Yes, batch-wise No |
