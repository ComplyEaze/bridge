# `ledger_gst_details_*` — provenance

One Collection request, the compliance ledger-master read with each ledger's `GSTDETAILS.LIST` named in its `FETCH`, and its answer, read from one synthetic company on Windows. Both files are live captures, never hand-written, and no byte of either was edited.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, Windows, `http://127.0.0.1:9001` (a lab instance).
- **Date:** 9-10 October 2026. One scripted request.
- **Book:** `BRIDGE PILOT LAB`, a synthetic company. The company name, the GUIDs and every value are synthetic.
- **Sender:** a script written by an outside contributor, not `bridge_mcp`, sending the body whose bytes are committed here. The request text is the production compliance request (`render_party_ledger_master_request`) with `, GSTDETAILS.LIST, RATEOFTAXCALCULATION, ROUNDINGMETHOD, ROUNDINGLIMIT` added to its `FETCH`, `SVFROMDATE` 20260401 and `SVTODATE` 20260801; the test `the_gst_details_request_is_byte_for_byte_the_one_sent_live` asserts that `render_party_ledger_master_request_with_gst_details` produces exactly these bytes. `RATEOFTAXCALCULATION`, `ROUNDINGMETHOD` and `ROUNDINGLIMIT` were fetched only to keep the request the one that was sent; the parser skips them.
- **Encoding:** the response is **BOM-less UTF-16LE**, exactly as received. The request is UTF-16LE **with a BOM**. `.gitattributes` marks this tree `-text`.
- **What the capture shows:**
  - 14 `LEDGER` rows. Every row carries one `GSTDETAILS.LIST`. Nine hold only whitespace (an empty placeholder); five hold one dated entry each.
  - An entry holds `APPLICABLEFROM` (`20260401` on all five), `SRCOFGSTDETAILS` (`As per Company/Group` on `BRIDGE CGST 2.5%`, `Specify Details Here` on the other four) and, on four of the five, `GSTINELIGIBLEITC` (`Yes`; absent on `BRIDGE Purchase Svc 5%`) and `TAXABILITY` (`Taxable`; absent on `BRIDGE CGST 2.5%`).
  - Each entry holds one `STATEWISEDETAILS.LIST`. Its `STATENAME` is the reserved value written as the numeric reference `&#4; Any`, then five `RATEDETAILS.LIST` rows with `GSTRATEDUTYHEAD` (`CGST`, `SGST/UTGST`, `IGST`, `Cess`, `State Cess`) and `GSTRATEVALUATIONTYPE` (`Based on Value`, or `&#4; Not Applicable` on the Cess row), then a `GSTSLABRATES.LIST` holding only whitespace.
  - `GSTRATE` is a `Number` written with leading spaces (` 2.50`, ` 5`, ` 9`, ` 18`) and is **absent** on some rows: on the Cess and State Cess rows of every entry, and on all five rows of the entry of `BRIDGE CGST 2.5%`. The capture has no `GSTRATE` with no text and none equal to zero.
  - There is no HSN/SAC element anywhere in the response.
- **Not established:** an empty `GSTRATE`; a `GSTRATE` of zero; a non-empty `GSTSLABRATES.LIST`; more than one `GSTDETAILS.LIST` or more than one `STATEWISEDETAILS.LIST` on a ledger; HSN/SAC; type of supply; other releases and editions; macOS. The response headers were not kept.

| file | bytes | sha256 | content |
|---|---|---|---|
| `ledger_gst_details_request.utf16le.xml` | 1876 | `2e728e002305cbaa33c73bd36507ca8d6ab62c5dea5497b02204a07636385d88` | the request (UTF-16LE with a BOM) |
| `ledger_gst_details_live.utf16le.xml` | 51434 | `597f2b11728c39d6643ced43682b89143fce29481a803ad5f23433ee896c9847` | its answer, 14 ledgers |
