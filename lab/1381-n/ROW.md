# Row 1381-N: what Tally does with a stock-item Create whose name is near an existing name. The evidence #1381's name check rests on

**Not for merge. Part of #1381. This sheet runs only after the owner grants it. About 90 minutes.** Written 17 Nov 2026. No ComplyEaze Bridge build is used: the kit writes the requests a build would send. The rules of `LAB-COMMON.md` (v3, byte-equal to row 1381-R1's copy) apply throughout. Confidence of everything here until the row runs: unverified.

The sheets it builds on are `lab/1381-stock-item-create` at 5bbd6fb2 (row 1381-M) and `lab/1381-r1-sheet` at c76e23d6, with answers on `lab/1381-stock-item-create-answers` at d8948467 and `lab/1381-r1-answers` at 7bda13fc.

## Question
ComplyEaze Bridge refuses a new stock-item name that folds to an existing item's name or alias under any of three keys in `bridge-tally-core/src/master_name_index.rs` (`collision_keys`): the absence key, the canonical key, and the letters-and-digits key that drops every space and symbol. For stock items, only case and a doubled space are measured. The rest is a rule chosen because a false refusal costs less than an overwrite. This row turns each into a measured fact, or records that Tally keeps the names apart.

- **(a) n1 to n4:** Does a Create reach the existing item when its name differs by a hyphen for a space, a slash for a space, a removed space, or removed full stops?
- **(b) n5 and n6:** What does a name-and-unit-only Create under a folded name do to an item that HAS GST and HSN lists? Row 1381-M's R4a and R4b measured case and a doubled space on an item whose lists were empty.
- **(c) n7 and n8:** May an item take the name of a stock group or of a unit?
- **(d) n9:** Does a Create named as another item's alias reach that item, after the alias is keyed on the screen?

For ledgers, the write reference (`docs/tally/TALLY_PROTOCOL_REFERENCE_WRITE_RESPONSES_AND_MASTERS.md`, section 9.4d) records on licensed 7.1 Silver that a hyphen and a slash matched for a space, and an underscore and an en dash were rejected. Those are ledger facts, not item facts.

## Book
`BRIDGE STOCK CREATE LAB` is on the lab PC at BK0's state, as row 1381-R1's checked restore left it (R1 RESULTS, S8). It has one stock item `SC Sibling Goods`, keyed with GST and HSN lists, one stock group `SC Goods Group`, one unit `Nos`, and marks M 210. It is checked against the lab's private exclusions, which do not name it; the run coordinator checks again before the sitting.

This row writes to it behind its own C5 backup and ends with a checked C6 restore. Any other company, another GUID, Education mode, or a book not at BK0's state: STOP, there is no book.

## Kit
`SHA256.txt` lists every file with its sha256 and size: check it first.

| file | what |
|---|---|
| `.gitattributes` | the kit's line-ending rule |
| `LAB-COMMON.md` | v3, byte-equal to row 1381-R1's copy |
| `m-items.utf16le.xml` | 1381-M's `requests/items.utf16le.xml`, byte-equal |
| `m-r1-name-unit.utf16le.xml` | 1381-M's measured name-and-unit Create, 978 bytes, byte-equal to `requests/r1-name-unit.utf16le.xml` |
| `requests/marks.utf16le.xml` | 1381-M's `requests/marks.utf16le.xml`, byte-equal |
| `requests/a1-candidate-items.utf16le.xml` | R1's candidate stock-item read, byte-equal |
| `requests/groups.utf16le.xml` | R1's stock-group read, byte-equal |
| `requests/units.utf16le.xml` | R1's unit read, byte-equal |
| `gen_n.py` | checks the template against `SHA256.txt`, replaces only its two item-name occurrences, and writes the fourteen requests below |
| `make_b.py` | on the lab PC, `marked` writes `requests/b2-create.utf16le.xml`, item `SC Copied Marked`, from S3's a1 answer |
| `check_r1.py` | reads rows ONLY at `./BODY/DATA/COLLECTION/<ROW>` and import counters only at `./<COUNTER>` of the RESPONSE. Modes: `marks`, `import`, `rows`, `diff`, `fetch`, `same`, `names` |

| request file | name sent | kind |
|---|---|---|
| `requests/t1-hyphen-target.utf16le.xml` | `SN Hyphen Item` | target |
| `requests/t2-slash-target.utf16le.xml` | `SN Slash Item` | target |
| `requests/t3-space-target.utf16le.xml` | `SN Space Item` | target |
| `requests/t4-stops-target.utf16le.xml` | `S.N. Stops Item` | target |
| `requests/t5-alias-target.utf16le.xml` | `SN Alias Item` | target |
| `requests/n1-hyphen.utf16le.xml` | `SN Hyphen-Item` | probe against t1: a hyphen where it has a space |
| `requests/n2-slash.utf16le.xml` | `SN Slash/Item` | probe against t2: a slash where it has a space |
| `requests/n3-space.utf16le.xml` | `SN SpaceItem` | probe against t3: one space removed |
| `requests/n4-stops.utf16le.xml` | `SN Stops Item` | probe against t4: its two full stops removed |
| `requests/n5-case-gst.utf16le.xml` | `sc sibling goods` | probe against `SC Sibling Goods`: case only |
| `requests/n6-spacing-gst.utf16le.xml` | `SC Copied  Marked` | probe against `SC Copied Marked`: a doubled space |
| `requests/n7-group-name.utf16le.xml` | `SC Goods Group` | probe: the name of the book's stock group |
| `requests/n8-unit-name.utf16le.xml` | `Nos` | probe: the name of the book's unit |
| `requests/n9-alias.utf16le.xml` | `SN Alias Other` | probe against t5 after S7 keys that alias on it |

When the sheet was written: `gen_n.py` was run twice here. Both runs printed the same fourteen lines and wrote the same bytes; those requests are in `SHA256.txt`.

## Steps (one sitting)
Requests go one at a time by C3's line. **After every write, send `marks`** and record V and M.
- **S0.** C1, then C2 (step 0). Record Tally's release: it must be TallyPrime Silver 7.2, ProductVersion 1.1.7.2. Another release: STOP. Load THE COMPANY alone. `marks` (`m0`): EXPECT one COMPANY row and M 210, BK0's mark. Any other M: STOP.
- **S1.** Check the kit against `SHA256.txt`. Run `python3 -I gen_n.py`: EXPECT its fourteen lines to equal `SHA256.txt`'s for those files.
- **S2. Backup (C5)** to a new empty local folder `C:\lab\1381-n\backup-1`. Record its name, folder, bytes and sha256, and the time the Restore list will show. Then `marks`: EXPECT `m0`.
- **S3. Baseline reads.** Send `a1-candidate-items` (`a1`), `groups` (`g1`), `units` (`u1`) and `marks` (`m1`, EXPECT `m0`). An answer that is not a collection of rows, or an Unknown Request: record it whole and go to S8. Then:
  - `check_r1.py rows` on a1, g1 and u1: EXPECT the one item `SC Sibling Goods`, the one group `SC Goods Group` and the one unit `Nos`;
  - `check_r1.py names a-a1.xml "SC Sibling Goods"`: EXPECT one name entry. R1's restore removed the alias;
  - `check_r1.py diff` of row 1381-R1's `answers/a-a1.xml` against this a1: EXPECT no row added, removed or changed. That answer is 8270 bytes, sha256 `92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4`, on the lab PC and on `lab/1381-r1-answers`; check the sha256 first.
- **S4.** `python3 -I make_b.py marked a-a1.xml` writes `requests/b2-create.utf16le.xml`, item `SC Copied Marked`. EXPECT 3674 bytes, sha256 `6961f4754673d5359c7bf597dd272e11c28b64cbe74f9e27064875f1709c526c`: the request R1 sent at its S6. The sibling is unchanged, so the render is the same. A refusal or another sha256: record it, skip the b2 send and n6, and go on.
- **S5. The targets, six sends, each watched.** In this order: `t1`, `t2`, `t3`, `t4`, `t5`, then `b2-create`. For each:
  - `marks` just before, EXPECT the marks of the step before. Send once. Screenshot Tally's window as soon as the answer is written. Then `check_r1.py import`, `marks`, and `a1-candidate-items` again;
  - `check_r1.py diff <the previous a1 answer> <this one>`. EXPECT CREATED 1 with every other counter 0, M up by 1, and exactly one row ADDED with the name sent and nothing CHANGED. Name-and-unit Creates were measured so in 1381-M's R1, and b2 in R1's S6;
  - after b2, also `check_r1.py same <read> "SC Copied Marked" <read> "SC Sibling Goods"`: EXPECT all six fields same.
  - A target or b2 that is not CREATED 1: record it and skip its probe. t1 owns n1, t2 owns n2, t3 owns n3, t4 owns n4, t5 owns n9, and b2 owns n6. An `ALTERED` above 0 on a target or b2 is a STOP.
- **S6. The probes n1 to n8, one send each, watched, in order.** Use S5's loop: `marks` just before, one send, a screenshot, `check_r1.py import`, `marks`, `a1-candidate-items`, and `diff` against the previous a1 answer. **There is no EXPECT on the counters.** Record every counter, each `LINEERROR`, M's difference, and every line of the diff. `EXCEPTIONS` with a `LINEERROR` is an answer, not a stop. Extra reads:
  - after n5, `check_r1.py names <read> "SC Sibling Goods"`;
  - after n6, `check_r1.py names <read> "SC Copied Marked"`. 1381-M found that a folded Create rewrote the language name and left the `NAME` attribute, so `names` is asked by the original name;
  - after n7, send `groups` and `diff` it against `g1`;
  - after n8, send `units` and `diff` it against `u1`.
- **S7. The alias, on the screen, then n9.** This step needs keys on Tally's screen. If the sitting has no one to key it, skip S7 whole, say so in the notes, and go to S8. Otherwise:
  - Open `SN Alias Item` in Alter (Gateway, Alter, Stock Item). Screenshot the screen. Type the alias `SN Alias Other` in its alias field. R1's notes record that Tally did not receive spaces typed in one run: type the alias in parts with the space key between them, and read the field in a zoomed screenshot before accepting. Screenshot, accept the screen, then leave by C7's Ctrl+Q;
  - any prompt other than the screen's own accept: C7;
  - `marks`, then `a1-candidate-items`, `check_r1.py names <read> "SN Alias Item"`, and `diff` against the read before. EXPECT two name entries, the second `SN Alias Other`; otherwise skip n9;
  - then n9, by S6's loop, with `check_r1.py names <read> "SN Alias Item"` after it.
- **S8. Restore (C6)** of S2's backup, as R1's S8, only on the run coordinator's RESTORE word; it may be given in the GO. Load THE COMPANY alone. Then `marks` (EXPECT `m0`), `a1-candidate-items`, `groups` and `units`. `check_r1.py diff` of each against `a1`, `g1` and `u1`: EXPECT nothing added, removed or changed. If S3 stopped early, compare the marks only. Any difference: STOP; the way back is S2's backup, restored again only on the run coordinator's word.
- **S9.** C10.

## Stop rules
LAB-COMMON v3's rules, and these:
- a release other than TallyPrime Silver 7.2, ProductVersion 1.1.7.2, or `m0` other than 210;
- any dialog or prompt that this sheet does not name;
- an `ALTERED` counter above 0 on a target or on b2;
- a marks read that fails or lacks the COMPANY row. That is no answer, never "unchanged";
- a restore whose marks or rows differ (S8);
- no answer in 15 minutes. Press nothing and kill nothing;
- a probe's answer is never a stop by its counters, only by a dialog.

## Record
- **Every send:** bytes, sha256, rows, and curl's first-byte and total seconds.
- **The marks after each write:** V and M, and their difference from the step before.
- **Every screen step:** Tally's own words, and the screenshots.
- **check_r1.py's whole output** for every command run.

## Decides
For n5 and n6, the fields to read in the diff's CHANGED lines are `PARENT`, `GSTAPPLICABLE`, `GSTTYPEOFSUPPLY`, `GSTDETAILS.LIST`, `HSNDETAILS.LIST` and `LANGUAGENAME.LIST`.

| Probe | If ALTERED 1 and no row added | If CREATED 1 and a row added | If EXCEPTIONS with a LINEERROR |
|---|---|---|---|
| n1 | Tally takes that name for the existing item, so the refusal of that fold is a Tally fact for stock items. | Tally keeps the two names apart; the refusal stays as a chosen rule (a person cannot tell the names apart), and the reference says it is not a Tally fold. | Record the words. |
| n2 | Tally takes that name for the existing item, so the refusal of that fold is a Tally fact for stock items. | Tally keeps the two names apart; the refusal stays as a chosen rule (a person cannot tell the names apart), and the reference says it is not a Tally fold. | Record the words. |
| n3 | Tally takes that name for the existing item, so the refusal of that fold is a Tally fact for stock items. | Tally keeps the two names apart; the refusal stays as a chosen rule (a person cannot tell the names apart), and the reference says it is not a Tally fold. | Record the words. |
| n4 | Tally takes that name for the existing item, so the refusal of that fold is a Tally fact for stock items. | Tally keeps the two names apart; the refusal stays as a chosen rule (a person cannot tell the names apart), and the reference says it is not a Tally fold. | Record the words. |
| n5 | Read every CHANGED line for the six fields. Lists emptied or removed: a folded Create destroys GST details, not only the name. Lists unchanged: it rewrites the name only. | Read every CHANGED line for the six fields. Lists emptied or removed: a folded Create destroys GST details, not only the name. Lists unchanged: it rewrites the name only. | Record the words and every CHANGED line for the six fields; apply the same list finding. |
| n6 | Read every CHANGED line for the six fields. Lists emptied or removed: a folded Create destroys GST details, not only the name. Lists unchanged: it rewrites the name only. | Read every CHANGED line for the six fields. Lists emptied or removed: a folded Create destroys GST details, not only the name. Lists unchanged: it rewrites the name only. | Record the words and every CHANGED line for the six fields; apply the same list finding. |
| n7 | ALTERED, or the group read changed, is the finding; record every line. | With the group read unchanged, Tally lets an item share a group's name. Bridge's item index, which holds stock items only, misses no collision Tally enforces. | Tally refuses it in-band; record the words. |
| n8 | ALTERED, or the unit read changed, is the finding; record every line. | With the unit read unchanged, Tally lets an item share a unit's name. Bridge's item index, which holds stock items only, misses no collision Tally enforces. | Tally refuses it in-band; record the words. |
| n9 | ALTERED with `SN Alias Item` changed: a Create named as an alias reaches the aliased item, so the alias refusal is a Tally fact. | Tally lets a new item take another item's alias as its name; the refusal stays as a rule. | Record the words. |

Each answer is one book, one release, one sitting (partial).

## Keep
On `lab/1381-n-answers`, with `SHA256.txt`:
- each import answer, whole;
- each read answer, whole. This book's GUID is listed in master's `scripts/merge-gate-synthetic-uuids.json`, so the reads may be pushed;
- `check_r1.py`'s whole output for each step;
- the notes in UTC, and the backup and restore records;
- the screenshots by name, size and sha256, never pushed.

Send the sha to the run coordinator before any push.

Row: lab/1381-n/
