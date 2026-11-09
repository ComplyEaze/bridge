# `einvoice-lab`: provenance

Tally's answers to the invoice-number read with `IRN, IRNACKNO, IRNACKDATE` added to its fetch list
(`render_invoice_number_request`), on the synthetic e-invoicing lab book (#2162 item 8).

## Capture

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.2, one loaded company.
- **Date:** 2026-11-04 (the lab PC's date). Each request was sent from a file with curl, not through a
  build: `NI-1.xml` and `NI-2.xml` of `lab/2162-irn-read-sheet` at `8b1573e8`, folder
  `lab/2162-irn-read/`. Their sha256 are the ones `the_number_read_is_the_one_the_lab_answered_with_the_e_invoice_words`
  pins for the code's render. Source: `answers/a-r1-NI-1.xml` and `answers/a-r1-NI-2.xml` on
  branch `lab/2162-irn-read-answers` at `4ded3a06`, results at `9a20049b`, R1.
- **Company:** `BRIDGE EINVOICE LAB`, a synthetic lab book with e-invoicing on in its GST registration,
  GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`. It held three vouchers dated 2025-04-03: Sales `1` with
  invented e-invoice details keyed by hand, Sales `2` without them, and a Payment `1`. Nothing reached
  the e-invoice portal.
- **Window:** 2025-04-01 to 2026-03-31, the financial year.
- **Answers:** each file is the answer's body, BOM-less UTF-16LE, as received. Each row carries
  `VOUCHERKEY`, Tally's internal key, as a 15-digit value; no company number, GSTIN or PAN.
- **What R1 measured:** each row carried each of the three elements once, and each answer was otherwise
  the same read's answer without them (`N-1.xml`, `N-2.xml`, kept on the lab PC). One book, one
  release, one sitting (protocol reference §8.2j).

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `number-read-irn-1.utf16le.xml` | 8,734 | `bc0a594d61ebc09c170f2f9a4cca9987b7f506d1baa5f1b22c31e59badbead0a` | the read for number `1`: Sales `1`, Sales class, `IRN` of 64 characters, `IRNACKNO` and `IRNACKDATE` set; and Payment `1`, not Sales class, the three empty |
| `number-read-irn-2.utf16le.xml` | 5,794 | `87ede2a7617036b876bb3972a7eef4fc0c2cb7e9673faf16db2945d5292454f2` | the read for number `2`: Sales `2`, Sales class, the three empty |
