# `voucher_type_numbering_series_*` — provenance

One Collection request, the voucher-type export with the numbering fields at both levels, and its answer, read from one synthetic company on Windows. Every file is a live capture, never hand-written, and no byte of either was edited.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, Windows 10 Home Single Language 22H2 (build 19045), `http://127.0.0.1:9001`.
- **Date:** 2026-10-09. One request, no retry; `GET /status` read before and after, both answered `TallyPrime Server is Running` with identical headers.
- **Book:** `Bridge Lab Win Numbering`, a synthetic company created for this read. It was the only company loaded and was not opened or changed between this read and an earlier one (24 voucher types; the three altered types kept their AlterIDs 207, 217 and 219). Three voucher types were set on Tally's own screens: Journal (Automatic), Payment (Manual, duplicates prevented), Receipt (Manual, duplicates not prevented). The company name, the GUIDs and every value are synthetic.
- **Sender:** a script written by the contributor who ran it (outside contributor, bridge#724), not `bridge_mcp`, sending the body whose bytes are committed here. The request text is the production `render_native_voucher_type_export_request` text with `NUMBERINGMETHOD, PREVENTDUPLICATES, VOUCHERNUMBERSERIES.*` added to its `FETCH` and the company name filled in; the test `the_request_is_byte_for_byte_the_one_sent_live` asserts that the renderer in `native_voucher_type_numbering.rs` produces exactly these bytes.
- **Encoding:** the response is **BOM-less UTF-16LE**, exactly as received (`Content-Type: text/xml; charset=utf-16`, 81,966 bytes, `Unicode: Yes`, `RESPSTATUS: 1`, answered in 0.011 s). The request is UTF-16LE **with a BOM**, 1,106 bytes. `.gitattributes` marks this tree `-text`.
- **What the capture shows:**
  - 24 `VOUCHERTYPE` rows; every row holds exactly one `VOUCHERNUMBERSERIES.LIST`, named `Default`.
  - Series level: Journal `Automatic` / duplicates `No`; Payment `Manual` / `Yes`; Receipt `Manual` / `No`; the other 21 `Automatic` / `No`. Each of the three matched the screen.
  - Type level: `NUMBERINGMETHOD` reads `None` for the three altered types and `Default` for the rest; `PREVENTDUPLICATES` reads `No` on all 24, including Payment. Neither is the method.
  - `NUMBERINGSUBMETHOD` reads `Auto Retain` on all 24 series, including the two Manual ones; `DUPLICATECONTROL` is present and empty on all 24 series. Neither is read.
- **Not established:** more than one series on a type; a single series not named `Default`; Automatic (Manual Override) and Multi-user Auto on a captured response (the first was seen on a real book on 28 September 2026, not committed); a voucher type created new rather than altered; other releases and editions; macOS.

| file | bytes | sha256 | content |
|---|---|---|---|
| `voucher_type_numbering_series_request.utf16le.xml` | 1106 | `70f1d9cefbfd00bda76fcdad35dc24707ba76d11a921e2ef63d8be317fc61934` | the request (UTF-16LE with a BOM) |
| `voucher_type_numbering_series_live.utf16le.xml` | 81966 | `2b6c9fb02e842f5c342a56d5cf5bef6e96c8a14b1df3fb3115546f5783597613` | its answer, 24 rows |

The response headers (135 bytes, `fc08d9a5a107902be042ea8331c464f74a499ca324ced5a01fdd4d30ddb9993e`) and the two `GET /status` answers were kept by the contributor and are not committed; only the header hash is recorded here; the two `GET /status` answers (`TallyPrime Server is Running`, 51 bytes each, identical headers) are described in the contributor's report on bridge#724.
