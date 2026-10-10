# GST status fixtures

Answers Tally gave on 2026-10-10 on the synthetic company `BRIDGE PILOT LAB` (TallyPrime 7.1
Silver, licensed mode). The answers are published on the branch `lab/1342-capture-answers` (the two
reads of the narrow window at commit `e67df2c72`, the wider read at commit `4b5ad93cf`); each answer
here is byte-equal to the file there (its SHA-256 is the one recorded for that file when it was published).
The request is on `lab/1342-capture-requests` (commit `1e7317c5`).

The data is synthetic: the company, its GSTIN (of the synthetic family the fixtures may carry) and
its vouchers were made for this measurement. One book, one release. The answers are UTF-16LE with
no byte order mark, as received; the request file begins with a byte order mark.

| Fixture | Bytes | SHA-256 | Published as | Request | What it is |
| --- | ---: | --- | --- | --- | --- |
| `pilot-lab-gst-status-sales-window.utf16le.xml` | 21,030 | `504d9b06911e222b31d585a092e0192a0a263ede91b9fdc5173040438a1e1672` | `aw8-status-narrow-window-sales-byname.xml` (commit `e67df2c72` of `lab/1342-capture-answers`) | `w8-status-narrow-window-sales-byname.xml` (`6c94de0ce12aec70c33fda1a59b759bd16bb5e44179756411fa254136c56cd65`; committed here as `pilot-lab-gst-status-sales-window.request.utf16le.xml`) | six vouchers of type `BRIDGE Sales` dated 2026-08-02, selected by the type's name in the request, window 20260802 to 20260803: `BP/26-27/0012` to `0015` and `0019` read included and not accepted as they stand, `BP/26-27/0016` read included and accepted as it stands (`ISGSTOVERRIDDEN` Yes) |
| `pilot-lab-gst-status-window.utf16le.xml` | 35,928 | `bdde73bd794236fd76b4b3975f34366b6080322ec4a113cbffb871e016dcde57` | `aw8-status-narrow-window.xml` (commit `e67df2c72` of `lab/1342-capture-answers`) | `w8-status-narrow-window.xml` in `lab/1342-capture-requests` (the same window and field list with no voucher-type condition; that file is not committed here and its hash was not recorded) | the same window with no type filter: eleven vouchers, the six above, `Z1/0001` of type `BRIDGE Sales Z1` (included) and four Receipts dated 2026-08-03 whose three status flags all read No |
| `pilot-lab-gst-status-three-vouchers.utf16le.xml` | 234,936 | `ebc84c460da6123c329ff36ce2b74e13f55f8024efa33d7b2ddec7c318996cb8` | `aw7-readback-status.xml` (commit `4b5ad93cf` of `lab/1342-capture-answers`) | `w7-readback-status.xml` in `lab/1342-capture-requests` (a wider field list than the narrow one; that file is not committed here and its hash was not recorded) | three vouchers of type `BRIDGE Sales`, read with the wider field list: `BP/26-27/0002` uncertain (flags No, Yes, No; not accepted as it stands), `BP/26-27/0010` included, `BP/26-27/0016` included and accepted as it stands; Tally's `&#4;` references as received |
| `pilot-lab-gst-status-sales-window.request.utf16le.xml` | 1,776 | `6c94de0ce12aec70c33fda1a59b759bd16bb5e44179756411fa254136c56cd65` | `w8-status-narrow-window-sales-byname.xml` in `lab/1342-capture-requests` (commit `1e7317c5`) | the code renders it: `render_gst_status_window` for company `BRIDGE PILOT LAB`, window 20260802 to 20260803, type name `BRIDGE Sales`; a test compares the two character for character | the request that was answered, with a byte order mark |
