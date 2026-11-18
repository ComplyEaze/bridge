# Row 1381-O: what Tally stores when a stock-item Create carries GST details wider than the one measured copy

**Not for merge. Part of #1381. This sheet runs only after the owner grants it. About 60 minutes.** Written 18 Nov 2026. No ComplyEaze Bridge build is used: the kit writes the requests a build would send. No key is pressed on Tally's screen except C5, C6 and the company load. The rules of `LAB-COMMON.md` (v3, byte-equal to row 1381-N's copy) apply throughout. Confidence of everything here until the row runs: unverified.

The sheets it builds on are `lab/1381-n-sheet` at 3c207ab1 and `lab/1381-n-answers` at 20c9d64d.

## Question
`BRIDGE STOCK CREATE LAB` is at BK0: one stock item `SC Sibling Goods`, with one dated GST entry from 2025-04-01, one state row and five rate rows (CGST 9, SGST/UTGST 9, IGST 18, and Cess and State Cess with no rate), one dated HSN entry, one stock group `SC Goods Group`, one unit `Nos`, and marks M 210. Rows 1381-R1 and 1381-N measured `b2-create`: a Create carrying those values, with Tally's control reference on three of them, was stored equal to the sibling on all six fields.

ComplyEaze Bridge will build a copy only in that one shape. Real books hold items with more than one dated entry, and items with a cess rate. This row changes one variable per request and asks what Tally stores:

- **o1:** a second dated GST entry, after the first, dated 2025-10-01 with CGST 6, SGST/UTGST 6 and IGST 12;
- **o2:** a second dated HSN entry, after the first, dated 2025-10-01;
- **o3:** the Cess row's valuation `Based on Value` and rate 12;
- **o4:** CGST 2.5, SGST/UTGST 2.5 and IGST 5;
- **o5:** no `PARENT` element;
- **o6:** `PARENT` as `&#4; Primary`;
- **o7:** a copy made on the lab PC from the read after o1, through the product's own typed-value path.

For o1 and o2 the second entry is otherwise a byte copy of the first. The measured request names the date element `APPLICABLEFROM` and writes a rate as `GSTRATE`; the generated requests keep its element names and order.

## Book
`BRIDGE STOCK CREATE LAB` is on the lab PC at BK0's state, as row 1381-R1's checked restore left it (R1 RESULTS, S8). It has one stock item `SC Sibling Goods`, keyed with GST and HSN lists, one stock group `SC Goods Group`, one unit `Nos`, and marks M 210. It is checked against the lab's private exclusions, which do not name it; the run coordinator checks again before the sitting.

This row writes to it behind its own C5 backup and ends with a checked C6 restore. Any other company, another GUID, Education mode, or a book not at BK0's state: STOP, there is no book.

**The GUID check.** `check_r1.py marks` prints the first 8 characters of the COMPANY row's GUID. For this book they are the 8 characters R1's notes record in their S0 line (`lab/1381-r1-answers/evidence/notes.txt` on `lab/1381-r1-answers`), called **the book's 8** below; master's `scripts/merge-gate-synthetic-uuids.json` lists the GUID whole. Write the book's 8 at the top of this sitting's notes before S0. At S0, and in the marks read before every write, the printed 8 characters must equal them and the row count must be 1. At S0 also compare the whole GUID in the marks answer with the registry entry, and check that the Gateway screenshot shows `BRIDGE STOCK CREATE LAB` and no other company. Anything else: STOP, nothing is sent. Marks M 210 alone does not identify the book.

## Kit
`SHA256.txt` lists every file with its sha256 and size: check it first.

| file | what |
|---|---|
| `.gitattributes` | the kit's line-ending rule |
| `LAB-COMMON.md` | v3, byte-equal to row 1381-N's copy |
| `m-items.utf16le.xml` | 1381-M's `requests/items.utf16le.xml`, byte-equal. No step sends it; it is kept as the reference the candidate read was made from |
| `m-r1-name-unit.utf16le.xml` | 1381-M's measured name-and-unit Create, 978 bytes, byte-equal to `requests/r1-name-unit.utf16le.xml` |
| `requests/marks.utf16le.xml` | 1381-M's `requests/marks.utf16le.xml`, byte-equal |
| `requests/a1-candidate-items.utf16le.xml` | R1's candidate stock-item read, byte-equal |
| `requests/groups.utf16le.xml` | R1's stock-group read, byte-equal |
| `requests/units.utf16le.xml` | R1's unit read, byte-equal |
| `requests/b2-create.utf16le.xml` | the marked 3,674-byte Create sent in row 1381-N, sha256 beginning `6961f475`, byte-equal |
| `gen_o.py` | checks the template against `SHA256.txt`, makes the counted edits below, and writes six requests |
| `make_o7.py` | row 1381-N's `make_b.py` with its source item, its written item and its output file changed, and only the `marked` mode kept. On the lab PC, `marked` writes `requests/o7-create.utf16le.xml`, item `SO Two Dates Copied`, from the read after o1. It writes every dated GST and HSN entry of `SO Two Gst Dates`, not only the first |
| `check_r1.py` | reads rows ONLY at `./BODY/DATA/COLLECTION/<ROW>` and import counters only at `./<COUNTER>` of the RESPONSE. Modes: `marks`, `import`, `rows`, `diff`, `fetch`, `same`, `names`. `same` compares every dated entry in each complete GST and HSN list, not only the first |

| request file | name sent | the one difference from `b2-create` |
|---|---|---|
| `requests/o1-two-gst-dates.utf16le.xml` | `SO Two Gst Dates` | a second `GSTDETAILS.LIST`, dated 20251001, with CGST 6, SGST/UTGST 6 and IGST 12 |
| `requests/o2-two-hsn-dates.utf16le.xml` | `SO Two Hsn Dates` | a second `HSNDETAILS.LIST`, dated 20251001 |
| `requests/o3-cess-rate.utf16le.xml` | `SO Cess Rate` | the Cess row has `Based on Value` and `GSTRATE` 12 |
| `requests/o4-other-rates.utf16le.xml` | `SO Other Rates` | CGST 2.5, SGST/UTGST 2.5 and IGST 5 |
| `requests/o5-root-parent.utf16le.xml` | `SO Root Parent` | no `PARENT` element |
| `requests/o6-root-marked.utf16le.xml` | `SO Root Marked` | `PARENT` is `&#4; Primary` |
| `requests/o7-create.utf16le.xml` | `SO Two Dates Copied` | written from the read after o1 by `make_o7.py marked`; not in the kit before S5 |

When the sheet was written: `gen_o.py` was run twice here. Both runs printed the same six lines and wrote the same bytes; those requests are in `SHA256.txt`.

## Steps (one sitting)
Requests go one at a time by C3's line. **After every write, send `marks`** and record V and M.

**Answer files.** Each answer is saved in this sitting's own new folder `answers/` as `a-<label>.xml`, with its headers file beside it. A request with no answer in 10 minutes is C3's stop. The labels:

| when | labels |
|---|---|
| S0 | `m0` |
| S2 | `s2-marks` |
| S3 | `a1`, `g1`, `u1`, `m1` |
| each write `<w>` of S4 and S5 (`o1` to `o7`) | `<w>-pre` (the marks before), `<w>` (the import answer), `<w>-post` (the marks after), `a1-<w>` (the candidate read after) |
| S6 | `m-r`, `a1-r`, `g-r`, `u-r` |

"The previous a1 answer" is the candidate read of the write sent just before, in the order sent: for `o1` it is this sitting's `a-a1.xml`, for `o2` it is `a-a1-o1.xml`, and so on; for `o7` it is `a-a1-o6.xml`. A skipped write has no files; S5 is the last write, so no chain passes over it. Row 1381-N's own `a-a1.xml` stays in N's folder and is named by its path there; it is not copied into this sitting's `answers/`. Screenshots and each `check_r1.py` output go in this sitting's `evidence/` folder, one file per step, as N kept them.

- **S0.** C1, then C2 (step 0). Record Tally's release: it must be TallyPrime Silver 7.2, ProductVersion 1.1.7.2. Another release: STOP. Load THE COMPANY alone. Send `marks` (`m0`) and run `check_r1.py marks answers/a-m0.xml`: EXPECT one COMPANY row, the book's 8, and M 210, BK0's mark. Then the GUID check above. Any other M or GUID: STOP.
- **S1.** Check the kit against `SHA256.txt`. Run `python3 -I gen_o.py`: EXPECT its six lines to equal `SHA256.txt`'s for those files, compared as a set: `gen_o.py` prints in the order of the table above and `SHA256.txt` is sorted by path.
- **S2. Backup (C5)** to a new empty local folder `C:\lab\1381-o\backup-1`. Record its name, folder, bytes and sha256, and the time the Restore list will show. Then `marks`: EXPECT `m0`.
- **S3. Baseline reads.** Send `a1-candidate-items` (`a1`), `groups` (`g1`), `units` (`u1`) and `marks` (`m1`, EXPECT `m0`). An answer that is not a collection of rows, or an Unknown Request: record it whole and go to S6. Then:
  - `check_r1.py rows` on a1, g1 and u1: EXPECT the one item `SC Sibling Goods`, the one group `SC Goods Group` and the one unit `Nos`;
  - `check_r1.py names answers/a-a1.xml "SC Sibling Goods"`: EXPECT one name entry. R1's restore removed the alias;
  - require a1 to be byte-equal to row 1381-N's `answers/a-a1.xml` on `lab/1381-n-answers`. That answer is 8,270 bytes, sha256 `92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4`; check its sha256 first. Also run `check_r1.py diff <N's folder>/answers/a-a1.xml answers/a-a1.xml`, N's a1 first and this sitting's second: EXPECT no row added, removed or changed.
- **S4. The six probes o1 to o6, one send each, watched, in order.** Send `requests/o1-two-gst-dates.utf16le.xml`, `requests/o2-two-hsn-dates.utf16le.xml`, `requests/o3-cess-rate.utf16le.xml`, `requests/o4-other-rates.utf16le.xml`, `requests/o5-root-parent.utf16le.xml`, then `requests/o6-root-marked.utf16le.xml`. For each:
  - `marks` just before, EXPECT the marks of the step before, one COMPANY row and the book's 8. Send once. Screenshot Tally's window as soon as the answer is written. Then `check_r1.py import`, `marks`, and `a1-candidate-items` again;
  - `check_r1.py diff <the previous a1 answer> <this one>` and `check_r1.py rows <this one>`;
  - **there is no EXPECT on the counters or on what is stored.** Record every counter, each `LINEERROR`, M's difference, every line of the diff, and the new item's whole row from the read, every dated entry and value included. `ERRORS` or `EXCEPTIONS` with a `LINEERROR` is an answer, not a stop. A dialog on Tally's screen is a STOP (C7);
  - for o1 and o2, the whole row is the evidence for every dated entry. Do not use `same` against the sibling as proof: the sent dates or rates deliberately differ. For o3 and o4, read each rate and valuation in the new row. For o5 and o6, read whether the item is directly under the root and what its `PARENT` reads as.
- **S5. o7, only if o1 answered CREATED 1.** Otherwise record why o7 was skipped and go to S6. Run `python3 -I make_o7.py marked answers/a-a1-o1.xml`, which writes `requests/o7-create.utf16le.xml`, item `SO Two Dates Copied`, from `SO Two Gst Dates`. Record its printed counts and sha256; the counts must say how many GST entries, state rows, HSN entries and marked values were written, with no count assumed beforehand. If it prints a line beginning `not rendered:` instead, record that line whole, send nothing for o7 and go to S6: the refusal is the answer. Send o7 once by S4's loop, including `diff` against `a-a1-o6.xml` and `rows` on `a-a1-o7.xml`. There is no EXPECT. Then run `check_r1.py same answers/a-a1-o7.xml "SO Two Dates Copied" answers/a-a1-o1.xml "SO Two Gst Dates"`: it compares every dated entry of both complete lists. Record its whole output and the two items' whole rows. `diff` and the two whole rows, not a first-entry-only comparison, are the evidence if `same` reports a difference.
- **S6. Restore (C6)** of S2's backup, as 1381-N's S8, only on the run coordinator's RESTORE word; it may be given in the GO. Load THE COMPANY alone. Then `marks` (EXPECT `m0`), `a1-candidate-items`, `groups` and `units`. Require `m-r`, `a1-r`, `g-r` and `u-r` to be byte-equal respectively to `m0`, `a1`, `g1` and `u1`, and run `check_r1.py diff` on each pair: EXPECT nothing added, removed or changed. If S3 stopped early, compare the marks only. Any byte difference: STOP; the way back is S2's backup, restored again only on the run coordinator's word.
- **S7.** C10.

## Stop rules
LAB-COMMON v3's rules, and these:
- a release other than TallyPrime Silver 7.2, ProductVersion 1.1.7.2, or `m0` other than 210;
- any dialog or prompt that this sheet does not name;
- a marks read that fails or lacks the COMPANY row. That is no answer, never "unchanged";
- a restore whose marks or rows differ in bytes (S6). The three reads fetch fixed field lists; a change to a field they do not fetch shows only in M;
- no answer in 10 minutes (C3). Press nothing, kill nothing, send nothing again;
- a probe's answer is never a stop by its counters, only by a dialog.

## Record
- **Every send:** bytes, sha256, rows, and curl's first-byte and total seconds.
- **The marks after each write:** V and M, and their difference from the step before.
- **Every screen step:** Tally's own words, and the screenshots.
- **check_r1.py's whole output** for every command run.
- **Every probe:** its request file and item name, every counter, each `LINEERROR`, every diff line, and the new item's whole row; for o7, both whole rows and the complete `same` output.

## Decides
| Probe | If CREATED 1 and the stored row holds everything sent | If CREATED 1 and something sent is missing or changed | If ERRORS or EXCEPTIONS |
|---|---|---|---|
| o1 | A copy of an item with more than one dated GST entry is measured once, with both entries stored as sent. | Such a copy silently loses data; the stored row says which entry, rate or value is missing or changed. | Record every counter, each `LINEERROR`, M's difference and the whole row if one was stored. |
| o2 | A copy of an item with more than one dated HSN entry is measured once, with both entries stored as sent. | Such a copy silently loses data; the stored row says which entry or value is missing or changed. | Record every counter, each `LINEERROR`, M's difference and the whole row if one was stored. |
| o3 | A Cess valuation of `Based on Value` and rate 12 are stored as sent. | The stored row says whether Tally changed the valuation, dropped or changed the rate, or stored neither. | Record every counter, each `LINEERROR`, M's difference and the whole row if one was stored. |
| o4 | CGST 2.5, SGST/UTGST 2.5 and IGST 5 are stored as sent. | The stored row says which rates Tally dropped or changed, and what it stored instead. | Record every counter, each `LINEERROR`, M's difference and the whole row if one was stored. |
| o5 | This form puts the item directly under the root; record what `PARENT` reads as. | Record whether a parent was supplied or changed, whether the item is under the root, and what `PARENT` reads as. | Record every counter, each `LINEERROR`, M's difference and the whole row if one was stored. |
| o6 | This form puts the item directly under the root; record what `PARENT` reads as. Together with o5, this says which form addresses the root. | Record whether the marked parent was changed, whether the item is under the root, and what `PARENT` reads as. | Record every counter, each `LINEERROR`, M's difference and the whole row if one was stored. |
| o7 | A copy of an item with more than one dated entry is measured once through the product's own read-and-write path; every entry is stored as read. | Such a copy silently loses data; the two stored rows and `same` say what is missing or changed. | Record every counter, each `LINEERROR`, M's difference and the whole row if one was stored. |

Each answer is one book, one release, one sitting (partial).

## Keep
On `lab/1381-o-answers`, with `SHA256.txt`:
- each import answer, whole;
- each read answer, whole. This book's GUID is listed in master's `scripts/merge-gate-synthetic-uuids.json`, so the reads may be pushed;
- `check_r1.py`'s whole output for each step;
- `make_o7.py`'s printed counts and sha256, and the generated o7 request;
- the notes in UTC, and the backup and restore records;
- the screenshots by name, size and sha256, never pushed.

Send the sha to the run coordinator before any push.

Row: lab/1381-o/
