# An invoice's e-invoice details read back: the day read and its captured answer (#2162)

These files are row 2440-ED's first request and Tally's answer to it, on the synthetic book BRIDGE EINVOICE LAB (GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`, listed as synthetic in `scripts/merge-gate-synthetic-uuids.json`). The row ran on 10 Nov 2026 (01:17 to 01:23 IST on 11 Nov), reads only, in one sitting, on TallyPrime single-user edition: the Gateway on port 9001, `tally.exe` file version 1.1.7.2. The Gateway showed no release number. No Bridge build ran, and nothing was imported, keyed or restored.

The request is the e-invoice read-back's day read: the `vouchers` window read with `include_e_invoice` (protocol reference §8.2j) over one day, 3 April 2025. Row 2440-ED rendered it with the read-back's request builder on the e-invoice change's branch, by a scratch test that was not committed, and sent it by a file. It is byte-equal to the request row 1378 sent through a build to this book on 2 Nov 2026 for §8.2j, whose relay kept only the answer's size and hash. It is kept as sent: UTF-16LE with a byte order mark.

The answer was written straight to a file by the lab's runner and is kept exactly as received: UTF-16LE without a byte order mark. It was sent twice, as every admitted read is a pair, and the two answers are byte-equal. This file is the first. Each carries the book's company identifier and the lab's invented customer GSTIN from the synthetic family, twice. There is no company number or PAN. Each row also carries `VOUCHERKEY`, Tally's internal key, as a 15-digit value.

What the answer holds, as row 2440-ED counted it with expat (M, one run):
- 3 rows, all dated 3 April 2025: Sales `1`, Sales `2` and Payment `1`. Each GUID begins with the book's.
- Sales `1` holds the three details: `IRN` of 64 characters, `IRNACKNO` 112233 and `IRNACKDATE` 20250403. They were keyed on Tally's screen, not imported.
- Sales `2` and the Payment each hold the three elements once, empty.
- 127,966 B: 124,904 B in rows and 3,062 B of the rest. 0.09 s to answer.

Evidence: `lab/2440-ed-answers` at 72023156. Row 2440-ED's sheet is `lab/2440-ed-sheet` at 9f29079d. The answers were added as captured by row 2440-ED-D, once their one GSTIN value was found to be a synthetic family member already in the public tree (`2440-ed-d/RESULTS.md` there).

Not measured: an invoice whose details were imported rather than keyed, another day or book, other releases, the multi-user edition, and the read-back sent by a built executable after an import.

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `einvoice-day-read.request.utf16le.xml` | 1,794 | `e0702cbb980e35f77e84cf48085824ef7d500d2f674e62d4c93d8d7257e95a71` | the day read for 3 April 2025, as sent; row 2440-ED's `lab/2440-ed/E1-einvoice-day.xml`; byte-exact |
| `einvoice-day-read.utf16le.xml` | 127,966 | `f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0` | its answer: three vouchers, Sales `1` with the three e-invoice details; `answers/a-s1-E1-einvoice-day.xml` of the 2440-ED evidence, byte-equal to its pair `a-s2`; byte-exact |
