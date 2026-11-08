# Row 1381-R1: Bridge's candidate stock-item read, a Create re-rendered from typed values, and an item's alias. The evidence #1381's first slice needs before code

**Not for tonight.** The lab's current grant covers only the index sheet's run. This sheet runs at the next lab sitting, and only after the owner grants it.

Not for merge. Part of #1381. Written 8 Nov 2026. The design is `lab/1381-design/DESIGN.md` at 4390ef89, section 4. Lines are cited at master 1f5dc024. No ComplyEaze Bridge build is used: the kit writes the requests a build would send. The rules of `LAB-COMMON.md` (v3, byte-equal to `lab/2377-b/LAB-COMMON.md`) apply throughout. Confidence of everything here until the row runs: unverified.

## Question
- **(a)** Does the candidate read return what row 1381-M's item read returned, field for field? It is master's `stock_items` request (`native_masters.rs:121-125`, `:241-246`) with `BASEUNITS, GSTAPPLICABLE, GSTTYPEOFSUPPLY, LANGUAGENAME.LIST` added to its fetch list, and nothing else changed. It is also 1381-M's item read with master's collection name.
- **(b)** Does a Create written from the sibling's values *as Bridge reads them* store what 1381-M's R2 stored? R2 copied the answer's own characters, which Bridge may not do (P7).
  - The difference that matters: Tally sends three of the sibling's values with a leading `&#4;` reference, `GSTAPPLICABLE`, `STATENAME` and one `GSTRATEVALUATIONTYPE`. C4, and Bridge's reader (`tolerant_xml.rs:118`), drop that reference. So the values as read begin with a space.
  - **b1** writes them as read. **b2** writes exactly those three as Tally's own marked form, and nothing else differs from b1.
- **(c)** Does an alias keyed on the screen arrive as a second name entry in the candidate read? §12a.13 says so on 7.1 Silver (partial). The alias refusal of the design's slice 1 rests on it.

## Book
`BRIDGE STOCK CREATE LAB`, the company row 1381-M created empty on Tally's screen on 5 Nov 2026 and keyed by hand with invented values (sheet `lab/1381-stock-item-create` at 5bbd6fb2, step 1). It is on the lab PC at its BK0 state, as 1381-M's restore left it (`lab/1381-stock-item-create-answers` at d8948467, RESULTS.md, steps 7 and 8). It is checked against the lab's private exclusions, which do not name it; the run coordinator checks again before the sitting.

This row writes to it, behind a C5 backup, and ends with a C6 restore that is checked. Any other company, a GUID other than the lab's own record of this book, Education mode, or a book that is not at BK0's state: STOP, there is no book.

## Kit
`SHA256.txt` lists every file with its sha256 and size: check it first.

| file | what |
|---|---|
| `LAB-COMMON.md` | v3, byte-equal to `lab/2377-b/LAB-COMMON.md` |
| `m-items.utf16le.xml` | 1381-M's `requests/items.utf16le.xml`, byte-equal |
| `m-r1-name-unit.utf16le.xml` | 1381-M's `requests/r1-name-unit.utf16le.xml`, byte-equal |
| `requests/marks.utf16le.xml` | 1381-M's `requests/marks.utf16le.xml`, byte-equal |
| `gen_r1.py` | writes `requests/a0-shipped-items`, `a1-candidate-items`, `groups` and `units` from master's envelope and each kind's shape. It asserts that a1 is a0 with the fetch list changed once, and that a1 is `m-items` with the collection name changed (twice). It writes nothing if either fails |
| `requests/a0-*.xml`, `a1-*`, `groups`, `units` | gen_r1.py's output |
| `make_b.py` | on the lab PC: `make_b.py plain` and `make_b.py marked` write `requests/b1-create` (item `SC Copied Goods`) and `b2-create` (item `SC Copied Marked`) from step 3's a1 answer. It reads the sibling as C4 says, keeps the fields master's parser models, and writes them with master's escape and import envelope. It asserts the envelope is m-r1's less its message, and refuses on any field it does not model. Its docstring says exactly what differs from R2 |
| `check_r1.py` | reads rows ONLY at `./BODY/DATA/COLLECTION/<ROW>` and an import answer's counters only at `./<COUNTER>` of its RESPONSE. Modes: `marks`, `import`, `rows`, `diff`, `fetch`, `same`, `names` |

When the sheet was written: gen_r1.py was run here, and its four requests are in SHA256.txt. make_b.py was run in both modes on an answer built from 1381-M's R2 request:
- b2 equalled R2 element for element, apart from R2's `TYPE` attributes and whitespace;
- b1 differed from b2 in exactly the three marked values.

check_r1.py was run on answers built the same way, with a CMPINFO block of count lines that were not counted as rows.

## Steps (one sitting)
Requests go one at a time by C3's line. **After every write, send `marks`** and record V and M.
- **S0.** C1, then C2 (step 0). Record Tally's release: 1381-M ran on TallyPrime Silver 7.2, ProductVersion 1.1.7.2. Another release: STOP, because (a) and (b) compare with 1381-M's answers. Load THE COMPANY alone. `marks` (`m0`): EXPECT one COMPANY row and M 210, BK0's mark (1381-M's M0′). Any other M: STOP.
- **S1.** Check the kit against SHA256.txt. Run `python3 -I gen_r1.py`: EXPECT its four lines to equal SHA256.txt's for those files.
- **S2. Backup (C5)** to a new empty local folder `C:\lab\1381-r1\backup-1`. Record its name, folder, bytes and sha256, and the time the Restore list will show. Then `marks`: EXPECT `m0`.
- **S3. (a), reads only.** Send `a0-shipped-items` (`a0`), `a1-candidate-items` (`a1`), `groups` (`g1`), `units` (`u1`) and `marks` (`m1`, EXPECT `m0`). An answer that is not a collection of rows, or an Unknown Request: record it whole, skip S4 to S7, and go to S8. Then:
  - `check_r1.py fetch a-a0.xml a-a1.xml`: EXPECT the same GUIDs, a0's fields equal in a1, and a1 adding the four fields;
  - `check_r1.py diff <1381-M base-items> a-a1.xml`: EXPECT no row added, removed or changed. 1381-M's `answers/base-items` is kept on the lab PC; check its sha256 against that branch's `kept-local.txt` first;
  - `check_r1.py names a-a1.xml "SC Sibling Goods"`: EXPECT one name entry;
  - `check_r1.py rows` on a1, g1 and u1: EXPECT the one item `SC Sibling Goods`, the group `SC Goods Group` and the unit `Nos`.
- **S4.** `python3 -I make_b.py plain a-a1.xml` and `python3 -I make_b.py marked a-a1.xml`. Add both requests and their sha256 to the notes. A refusal: record its words, send nothing, and go to S8.
- **S5. b1, one send, watched.** `marks` (`b1pre`, EXPECT `m1`). Send `b1-create`. Screenshot Tally's window as soon as the answer is written. Then `check_r1.py import`, `marks` (`b1post`), and `a1-candidate-items` again (`a1b1`).
  - EXPECT CREATED 1 with every other counter 0, and M up by 1.
  - EXCEPTIONS with a LINEERROR is an answer, not a stop: record it, and EXPECT M unchanged.
  - Then `check_r1.py same a-a1b1.xml "SC Copied Goods" a-a1b1.xml "SC Sibling Goods"`, and `same a-a1b1.xml "SC Copied Goods" <1381-M r2-items> "SC Copied Goods"`. The second compares with the item R2 stored by copying bytes; check that file's sha256 against `kept-local.txt` first.
- **S6. b2, one send, watched,** exactly as S5, with `b2-create`, item `SC Copied Marked`, labels `b2pre`, `b2post` and `a1b2`. Its first `same` is against `SC Sibling Goods` in `a1b2`, and its second against R2's `SC Copied Goods`.
- **S7. (c), on the screen.**
  - Open `SC Sibling Goods` in Alter (Gateway, Alter, Stock Item). Screenshot the screen. Type the alias `SC Sibling Alias` in its alias field, and accept the screen. Screenshot, then leave by C7's Ctrl+Q.
  - Any prompt other than the screen's own accept: C7.
  - `marks` (`c1`): record M's difference. Then `a1-candidate-items` (`a1c`), `check_r1.py names a-a1c.xml "SC Sibling Goods"`, and `check_r1.py diff a-a1b2.xml a-a1c.xml`.
- **S8. Restore (C6)** of S2's backup: the plan, the restore by the operator, and the listings, all as C6 says. Load THE COMPANY alone. Then `marks` (EXPECT `m0`), `a1-candidate-items` (`a1r`), `groups` (`g2`) and `units` (`u2`). `check_r1.py diff` of a1 against a1r, g1 against g2 and u1 against u2: EXPECT nothing added, removed or changed. If S3 stopped early, compare the marks only. Any difference: STOP; the way back is S2's backup, restored again only on the run coordinator's word.
- **S9.** C10.

## Stop rules
LAB-COMMON v3's rules, and these:
- a release other than 1381-M's, or `m0` other than 210;
- any dialog or prompt that this sheet does not name;
- an `ALTERED` counter above 0 on b1 or b2: the item existed, which S3 said it did not;
- a marks read that fails or lacks the COMPANY row. That is no answer, never "unchanged";
- a restore whose marks or rows differ (S8);
- no answer in 15 minutes. Press nothing and kill nothing.

## Record
- **Every send:** bytes, sha256, rows, and curl's first-byte and total seconds.
- **The marks after each write:** V and M, and their difference from the step before.
- **Every screen step:** Tally's own words, and the screenshots.
- **check_r1.py's whole output** for every command run.

## Decides
- **(a)** a0's fields are equal in a1, a1 adds the four with a value each, and a1 equals 1381-M's base-items read: the candidate read returns what the slice needs, and it may be reviewed as a release shape. Otherwise, the field that differs is the result.
- **(b)**
  - **b1 created and same against both:** a builder may write the values as Bridge's reader leaves them.
  - **b1 refused or different, and b2 created and same:** the builder needs closed-set types for those three fields, written in Tally's marked form. The design's `SiblingGst` gains them, and master's `escape_text` (`xml_text.rs:41-50`) is not enough on its own.
  - **Both refused or different:** a re-render from typed values is not yet a shape Bridge may send. The design stops there, and a later row isolates the `TYPE` attributes and the whitespace, one at a time.
- **(c)** Two name entries, the second `SC Sibling Alias`, with the diff showing only that item's name entries and AlterID changed: the alias refusal rests on the name entries. Otherwise the alias check needs another read, measured first.

## The data step (after the row, not on Tally)
- Every read here carries the company's GUID. Until that GUID is in master's synthetic-uuid registry, each read answer stays on the lab PC, listed by bytes and sha256 in `kept-local.txt`. That covers this row's reads and 1381-M's kept reads.
- The import answers carry no GUID and are kept whole.
- The run coordinator receives the GUID by the private channel and checks it against the exclusions. Then a data pull request lists it, with lab-capture manifest entries for the answers the fixtures will use, as #2380 and #2382 did. The kept reads are pushed to the answers branch only after that merges.

## Keep
On `lab/1381-r1-answers`, with `SHA256.txt`:
- each import answer, whole;
- the read answers by name, size and sha256 in `kept-local.txt` (above);
- check_r1.py's whole output for each step, with every GUID cut to its first 8 characters as the script prints it;
- the notes in UTC, the backup and restore records, and the screenshots by name, size and sha256.

Send the sha to the run coordinator before any push.

Row: lab/1381-r1/
