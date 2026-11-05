# Lab row 1381-M: what Tally does with a stock item Create, one variable at a time (#1381). SYNTHETIC COMPANY ONLY. Not for merge.

Written 5 Nov 2026 for #1381's "Measure live first" list, which comes before any code. Rows 1 to 5 of that list are
measured here. Row 6 (the same Create on a Gold book) is dropped by the owner's ruling of 5 Nov 2026 that Gold is
Silver for everything; it is not run and not replaced.

`SHA256.txt` lists every file here with its bytes; check it before step 0. About 60 minutes.

## What is known, and what this row adds

- No stock-item Create has been sent through ComplyEaze Bridge; no builder for one exists. #690's approved master path
  creates ledgers only (`src-tauri/src/tally/ledger_create_item.rs`).
- §12a.13 of the protocol reference (licensed 7.1, PARTIAL, one item): a Create on an existing stock item "changed no
  field of those read". GST fields were not among those read.
- §9.4g (ledgers, PARTIAL): a Create on an existing ledger name replaced its parent, bill-wise flag and opening balance,
  answering `CREATED 0, ALTERED 1`. §9.4b and §9.4d (ledgers): names match without regard to case and separators, and a
  Create in a folded spelling altered the existing ledger. Neither has been measured for a stock item.
- The masters-shape read of 15 Oct 2026 (`lab/hsn-masters-answers` at 46dc0a98) shows how Tally answers a keyed item's
  `GSTDETAILS.LIST` and `HSNDETAILS.LIST`. It did not fetch the type of supply or the unit.

Each write is a candidate shape, sent by `curl` to this lab book only, as the owner allowed on 26 Oct 2026 for shape
discovery. What it answers is meant to become the stock-item Create message of #1381, beside the ledger Create, in the
same masters import envelope. No ComplyEaze Bridge build runs in this row.

## Rules and company

`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy of the one on `lab/692-opening-stock` at 991c3ec4,
sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6.

THE COMPANY is `BRIDGE STOCK CREATE LAB`. It does not exist yet: step 1 creates it. If a company of this name exists on
the PC, STOP and report.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **Step 0 before the company exists** runs without its MARKS read (C2 d): the Select Company list stands in for the
   Gateway. Every later step 0 is in full.
2. **Import answers.** C4's EXPECT for an import (`CREATED` equal to the objects sent, every other counter 0) is
   replaced in rows R1 to R5 by "record every counter and each `LINEERROR`": the counters are what these rows measure.
   A counter missing, or an answer that does not parse, is still a STOP.
3. **Size bound** for every read: 200,000 bytes.
4. **Grant.** The company and its masters (step 1); BK0 (step 4); the seven writes R1, R2, R3, R4a, R4b, R5a and R5b,
   each sent once; the screen read of R1b; the reads; the restore of BK0 on the run coordinator's RESTORE. Nothing else.

Licensed mode only; Education mode = STOP.

## The files

- `requests/` is written by `gen_requests.py` (run `python3 -I gen_requests.py` here; it prints `10 request files`).
  - `marks.utf16le.xml` is the marks request of `lab/690-batch-ledger-create-l0` at 83a5c989, with this company's name.
  - The six write files keep that row's import envelope (`create-a.utf16le.xml`) byte for byte. Only the messages inside
    `REQUESTDATA` are this row's.
  - `items`, `groups` and `units` are this row's reads. Each is a Collection export of fixed fields:
    - `items`: NAME, PARENT, GUID, MASTERID, ALTERID, BASEUNITS, GSTAPPLICABLE, GSTTYPEOFSUPPLY, HSNDETAILS.LIST,
      GSTDETAILS.LIST and LANGUAGENAME.LIST;
    - `groups`: NAME, PARENT, GUID, MASTERID and ALTERID;
    - `units`: NAME, GUID, MASTERID and ALTERID.

    They are NEW SHAPES (C3: a screenshot before the next request).
- `templates/` holds the two byte copies the requests are made from.
- `make_r2.py` renders `requests/r2-copied.utf16le.xml` on the lab PC at step 3, from step 2's answer. It is the only
  request not in `SHA256.txt`; step 3 records its bytes and sha256.
- `check_1381.py` reads every answer as C4 says and prints what this sheet records. Its modes: `marks`, `import`,
  `rows`, `diff BEFORE AFTER` and `same ANSWER NAME-A NAME-B`. A read that failed prints `NOT READ` and exits 1. That
  is never "nothing" or "no change".
- `check-request-ids.py`: run `python3 -I check-request-ids.py requests` before the first send. EXPECT `census: ok 4,
  skipped 6, FAIL 0` (the four reads defining their own collection, and the six writes). After step 3, EXPECT `ok 4,
  skipped 7, FAIL 0`.

## The writes

Each is one `STOCKITEM` with `ACTION="Create"`, and each asks one question. R4a, R4b, R5a and R5b each differ from R1
in one thing; R3 differs from R2 in that its name now exists, and in the details it sends so that an alteration shows.

| Row | Item name sent | What it carries | The one variable |
|---|---|---|---|
| R1 | `SC Name Unit Only` | NAME and BASEUNITS `Nos` only | #1381 row 1: what Tally fills by itself |
| R2 | `SC Copied Goods` | PARENT, BASEUNITS, GSTAPPLICABLE, GSTTYPEOFSUPPLY and every GSTDETAILS.LIST and HSNDETAILS.LIST, copied by `make_r2.py` from step 2's answer for `SC Sibling Goods` | row 2: the GST details as our own export of a keyed item shows them |
| R3 | `SC Copied Goods` again | PARENT `SC Goods Group`, BASEUNITS `Nos`, HSN `39269099` and IGST 5 (CGST and SGST/UTGST 2.50) from 1-Apr-2025 | row 3: a Create on a name that exists, on an item made for this test |
| R4a | `sc name unit only` | as R1 | row 4: the name differs from R1's in case only |
| R4b | `SC Name  Unit Only` (two spaces before `Unit`) | as R1 | row 4: the name differs from R1's in spacing only |
| R5a | `SC Absent Unit Item` | NAME and BASEUNITS `SCX`, a unit the book does not hold | row 5: a unit that is not in the book |
| R5b | `SC Absent Group Item` | PARENT `SC Absent Group`, a group the book does not hold, and BASEUNITS `Nos` | row 5: a stock group that is not in the book |

## Steps

0. LAB-COMMON step 0 with no company loaded, without its MARKS read (Difference 1). EXPECT no company named `BRIDGE
   STOCK CREATE LAB` in the Select Company list. Record TallyPrime's release and edition as the screen shows them. Run
   the census and check every file against `SHA256.txt`.

1. **Keying on Tally's screen**, by the operator, one act at a time, with a screenshot before each accept (Ctrl+A).
   Record each screen's path and each field's exact name and value. A screen that differs from this text (a field
   missing, a value not offered): STOP, screenshot and report. Nothing is improvised.

   a. Create Company `BRIDGE STOCK CREATE LAB`:
      - financial year and books beginning 1-4-2025; no password; state Karnataka;
      - every other field as the screen offers.

      If a Features screen opens:
      - Maintain Inventory: Yes.
      - Enable Goods and Services Tax (GST): Yes. On its details screen: state Karnataka, registration type Regular,
        GSTIN left blank, every other field as offered.
      - If Tally will not accept a blank GSTIN: STOP and report the screen. No GSTIN is typed.
      - Record every value the Features screen shows.

      Load THE COMPANY alone.

   b. Unit `Nos`: Simple, formal name `Numbers`, decimal places 0.

   c. Stock group `SC Goods Group`, under Primary, every other field as offered.

   d. Stock item `SC Sibling Goods`:
      - under `SC Goods Group`, unit `Nos`;
      - GST applicability `Applicable`, if the screen shows it;
      - type of supply `Goods`;
      - HSN/SAC details `Specify Details Here`, HSN/SAC `39239090`, description `Plastic articles for conveyance`;
      - GST rate details `Specify Details Here`, taxability `Taxable`, IGST rate 18, applicable from 1-Apr-2025 if the
        screen asks;
      - leave every head Tally fills by itself as filled, and record what it filled.

      After the save, Tally shows a blank creation screen. Leave it with Ctrl+Q. If `Quit ?  Yes or No` shows, answer
      Yes: on 15 Oct 2026 Yes only closed the empty form (`lab/hsn-masters-answers`, NOTES.md).

2. **The first reads.** Send `marks`, `items`, `groups` and `units`, each followed by `check_1381.py`. Use `marks` for
   the first and `rows` for the other three.
   - `marks`: EXPECT one COMPANY row. Record its GUID, V0 and M0.
   - `items`: EXPECT exactly one row, `SC Sibling Goods`. Record its whole `rows` output, and whether `GSTAPPLICABLE`,
     `GSTTYPEOFSUPPLY` and `BASEUNITS` arrive at all.
   - `groups`: EXPECT `SC Goods Group` among the rows, with what else Tally holds there.
   - `units`: EXPECT `Nos`.
   - Every GUID begins with THE COMPANY's, else STOP.

3. **Render R2.** Run `python3 -I make_r2.py <step 2's items answer>` and record its whole output. Then run the census
   (EXPECT `ok 4, skipped 7, FAIL 0`) and record r2's bytes and sha256 in the notes.
   - If `make_r2.py` refuses (`not rendered: ...`), record it. R2 and R3 are skipped, because R3 needs R2's item, and
     the run goes on to step 4. That is a result, not a STOP.

4. **BK0** (C5), into the new empty folder `C:\bridge-lab\1381sc\backups\bk0\`. Record its name, bytes and sha256.
   Then `marks` (M0′; a move across the backup is recorded, not a STOP), `items`, `groups` and `units`. These four
   answers are the BASELINE.

5. **The writes, in table order.** For each one:
   1. `marks`, just before the write (C3);
   2. send the write once;
   3. take a screenshot of the whole Tally window;
   4. run `check_1381.py import` on its answer;
   5. send `marks` and `items`;
   6. run `check_1381.py diff <the previous items answer> <this one>`.

   Record every line. There is no EXPECT on the counters (Difference 2). There is also:
   - **R1.** Record what Tally holds for the new item in every field the read returns: PARENT, BASEUNITS,
     GSTAPPLICABLE, GSTTYPEOFSUPPLY, GSTDETAILS.LIST and HSNDETAILS.LIST.
   - **R1b, a screen read with no write.** Open the item `SC Name Unit Only` on Tally's Alter screen. Screenshot it
     and record what it shows for the type of supply, GST applicability, HSN/SAC and the rate. Change nothing. Leave
     with Ctrl+Q; if `Quit ?  Yes or No` shows, answer Yes. Then `marks`: EXPECT M unchanged since R1's read. A
     moved M = STOP. This tells a default that the read does not carry from one that is not there.
   - **R2** (only if step 3 rendered it). Then run `check_1381.py same <items answer> "SC Sibling Goods" "SC Copied
     Goods"` and record each field as same or DIFFERENT.
   - **R3** (only if R2's `diff` showed `ADDED` for `SC Copied Goods`; otherwise skipped and recorded). Record:
     - whether `SC Copied Goods` kept its GUID;
     - whether its ALTERID and M moved;
     - each field the `diff` shows CHANGED, from what to what;
     - whether any row was ADDED.
   - **R4a and R4b.** Record:
     - whether a row was ADDED, or `SC Name Unit Only` CHANGED;
     - the `NAME` attribute and the LANGUAGENAME.LIST names after each;
     - the item count.
   - **R5a.** Send `units` too, and `diff` it against the BASELINE's. Record whether a unit `SCX` appeared, and whether
     an item was ADDED.
   - **R5b.** Send `groups` too, and `diff` it against the BASELINE's. Record whether a group `SC Absent Group`
     appeared, and whether an item was ADDED.

6. **The last reads.** Send `marks`, `items`, `groups` and `units`, and `diff` each against the BASELINE. Record every
   line.

7. **Restore BK0** (C6), only on the run coordinator's explicit RESTORE. Then `marks`, `items`, `groups` and `units`.
   EXPECT:
   - the COMPANY row's GUID, V and M equal to the BASELINE's;
   - each `diff` against the BASELINE showing no row ADDED, REMOVED or CHANGED.

   Anything else = STOP.

8. LAB-COMMON C10.

STOP RULES:
- C7 and C8;
- any dialog after a write, or after a NEW SHAPE read;
- an import answer that does not parse or lacks a counter;
- a MARKS answer with other than one COMPANY row;
- a GUID that does not begin with THE COMPANY's;
- a mark that moves in R1b or in any step that sends no write.

## What the result decides

Recorded by the coordinator, each with its confidence. Each line names the part of #1381's acceptance it settles.

| Question | Read from | If yes | If no |
|---|---|---|---|
| Does a Create with a name and a unit only give the item a type of supply, GST applicability or GST or HSN details that were not sent? | R1, R1b | The build sends every GST field from its named source, always. The read-back (acceptance 5) counts a field Tally filled as a difference. | The build still sends every field; the read-back shows nothing filled. |
| Is a Create carrying the keyed item's own GST details stored as sent, field for field? | R2 | That shape is the builder's; acceptance 2 can copy from a sibling item. | Each DIFFERENT field is a field the builder cannot copy as it reads; the design names it. |
| Does a Create on an existing item's name alter it (`ALTERED 1`, fields replaced)? | R3 | Acceptance 1 is a must before any window, as it is for ledgers (§9.4g). | Recorded as the 7.2 answer beside §12a.13. Acceptance 1 stays, because a Create is never sent for a name that exists. |
| Does a name differing only by case, or only by spacing, reach the existing item? | R4a, R4b | The name check folds case and spacing, as §9.4b says for ledgers. | The name check is exact for stock items, and the design says so with this row's evidence. |
| Does Tally refuse in-band an item whose unit or stock group the book does not hold? | R5a, R5b | The build's own refusal stands first. Tally's is the second line. | If Tally creates the unit or group by itself, the build must read both before any window. A silent create would otherwise write two masters under one approval (acceptance 4). |

ComplyEaze Bridge builds the stock-item Create only on the shapes this row answers. A result unlike every column here
changes the design before any code.

## Not measured by this row

- a Gold book (by the ruling above);
- an item whose type of supply is Services;
- aliases;
- a Create carrying an opening balance, a rate or a godown;
- more than one item in one request;
- TallyPrime 7.1;
- a book with vouchers.

## Evidence

- Every answer whole, with its size and sha256, and every `check_1381.py` output.
- The screens' paths and field names, and the screenshots by name, size and sha256 (never pushed).
- BK0's name, bytes and sha256, `restore-plan.txt`, and a notes file with UTC times and one line per step.

THE COMPANY's GUID is new: it goes into the merge gate's synthetic-UUID registry when a pull request first cites it.
Until then the evidence shows it as `<company guid>`, and the company number as `<company number>`.

Evidence goes on a new branch, `lab/1381-stock-item-create-answers`, with `SHA256.txt`. Send its sha to the run
coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.
