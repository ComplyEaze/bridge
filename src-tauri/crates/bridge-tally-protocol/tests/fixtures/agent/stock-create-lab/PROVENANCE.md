# Creating one stock item copied from a sibling: captured reads, creates and answers (#1381)

These files are row 1381-R1's requests and Tally's answers on the synthetic book BRIDGE STOCK CREATE LAB, created on Tally's screen for row 1381-M on 5 Nov 2026 with invented names. The release was TallyPrime 7.2, single-user edition (`tally.exe` ProductVersion 1.1.7.2), run on 9 Nov 2026, in one sitting. The book was taken from a backup before the first write and restored from it after the last. Every read after the restore was byte-equal to the read before the writes.

The lab's runner wrote each answer straight to a file. Each is kept exactly as received: UTF-16LE without a byte order mark. The reads carry the book's company identifier, which `scripts/merge-gate-synthetic-uuids.json` lists as synthetic (#2419). There is no GSTIN, company number or PAN in any of them.

Evidence: `lab/1381-r1-answers` at 7bda13fc. The sheet is `lab/1381-r1-sheet` at c76e23d6. The import answer and the two requests were pushed with the row at 3b50a15e. The seven read answers were kept on the lab PC until #2419 listed the identifier, then checked and pushed by row 1381-R1P (`1381-r1p/RESULTS.md` there: PUSH 7 of 7).

## The row

- **The reads.** Each read is sent by a file, not by a build: master's stock-item, stock-group, unit and company-marks requests, as `gen_r1.py` on the sheet branch writes them. The stock-item reads are the candidate read, master's `stock_items` request with `BASEUNITS`, `GSTAPPLICABLE` and `GSTTYPEOFSUPPLY` added to the fetch list. `LANGUAGENAME.LIST` was already returned by master's read.
- **The two creates.** `make_b.py` on the lab PC wrote both from the candidate read's row for `SC Sibling Goods`, a Goods, Taxable item in `SC Goods Group` with the unit `Nos`. Each is one `STOCKITEM ACTION="Create"` copying that item's parent, unit, GST applicability, type of supply, GST details list and HSN details list:
  - **b1, the plain form:** creates `SC Copied Goods`, with the three constant texts written plain;
  - **b2, the marked form:** creates `SC Copied Marked`, with those three texts in the marked form.
- **The answers.** Each create was answered CREATED 1, with every other counter 0, and the two answers are byte-equal, so one file stands for both. Each create raised the company's master mark by 1 (210 to 211 to 212), as the row's `RESULTS.md` records (`lab/1381-r1-answers` at 7bda13fc, steps S5 and S6). Only the first marks read, at 210, is a fixture here.
- **What the reads after them show.** After b1, Tally stored `SC Copied Goods` without `GSTAPPLICABLE` and with an empty state-wise block in its GST details list: 4 descendants against the sibling's 24. After b2, `SC Copied Marked` equals the sibling on all six fields. The counters did not show the difference; only the read did (PARTIAL: one sitting, one book, one release).
- **The alias.** `SC Sibling Alias` was keyed as an alias of `SC Sibling Goods` on Tally's Alter screen (mark 212 to 213, step S7 of the same `RESULTS.md`). The read after it returns two name entries for that item in `LANGUAGENAME.LIST`, and differs from the read before only in that item's name entries and AlterID.

Not measured: other releases, the multi-user edition, a Services or non-taxable sibling, or a create sent by a build. A build's candidate read and builder are tested against these files in later pull requests; row 1381-B repeats the create through ComplyEaze Bridge's own tools.

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `stock-create-lab-items-before.utf16le.xml` | 8,270 | `92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4` | R1 `a-a1`: the candidate stock-item read before the creates, one row (`SC Sibling Goods`) |
| `stock-create-lab-items-after-plain-create.utf16le.xml` | 10,758 | `2ae12ef647f24b9645231b868f733f978b09d3a8508aab5bf6dfa2564ccbffbd` | R1 `a-a1b1`: the same read after b1, two rows; `SC Copied Goods` has no `GSTAPPLICABLE` and an empty state-wise block |
| `stock-create-lab-items-after-marked-create.utf16le.xml` | 15,922 | `f58ab8debba4d819c53e3d58df3d84b1bbfb050e35b78a9d506ee1cd74fc2ba3` | R1 `a-a1b2`: the same read after b2, three rows; `SC Copied Marked` equals the sibling on the six fields |
| `stock-create-lab-items-with-alias.utf16le.xml` | 15,998 | `3bdb313e443c8de87fa6f9c593611307833f93e29dc2f348133e76f00c5b800c` | R1 `a-a1c`: the same read after the alias was keyed, `SC Sibling Goods` with two name entries |
| `stock-create-lab-groups.utf16le.xml` | 4,170 | `b57cf4dc5311ee18bc767d9a3a8f6fa1220cdae9f72461066a85964b5d84602a` | R1 `a-g1`: the stock-group read, one row (`SC Goods Group`) |
| `stock-create-lab-units.utf16le.xml` | 4,040 | `cb9d693327fb6b8b27c9449ee9f841cc12f8a4afe2598b38692a0f825f0d4f39` | R1 `a-u1`: the unit read, one row (`Nos`) |
| `stock-create-lab-marks.utf16le.xml` | 3,318 | `ed6554bcc9af34d808d6ee4df477803a8f22266c036aa9dd246abc909e4027e1` | R1 `a-m0`: the company-marks read before the row, one COMPANY row, master mark 210, no voucher mark |
| `stock-create-lab-plain-create.request.utf16le.xml` | 3,646 | `8f1e50a6f4a996adf951b6b30cee4f0b1366e9925f6bc3e6f016ecb2d8b28f93` | R1 `requests/b1-create`: the plain-form create, as sent |
| `stock-create-lab-marked-create.request.utf16le.xml` | 3,674 | `6961f4754673d5359c7bf597dd272e11c28b64cbe74f9e27064875f1709c526c` | R1 `requests/b2-create`: the marked-form create, as sent |
| `stock-create-lab-create-answer.utf16le.xml` | 538 | `1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010` | R1 `a-b1`, byte-equal to `a-b2`: the import answer to each create, CREATED 1, every other counter 0 |
