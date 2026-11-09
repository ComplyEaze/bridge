First line: **(a) is met except for one word of the sheet: a1 adds three fields, not four, because the shipped read already returns `LANGUAGENAME.LIST`. (b) is the sheet's second row: b1 was created (CREATED 1) but stored no GST applicability and an empty state-wise block, while b2 was created and equals both the sibling and R2's item. (c) is met: the alias arrives as a second name entry. The restore was checked: every read after it is byte-equal to the read before the writes. No stop, no skip, no dialog. One lab slip on the screen, corrected before the accept (S7).**

Sheet `lab/1381-r1-sheet` at `c76e23d6`, kit 13 of 13 OK by its `SHA256.txt`; `gen_r1.py`'s four requests equal it. Book BRIDGE STOCK CREATE LAB (synthetic), GUID `bac2b5e9...` (the lab's own record), loaded alone, at BK0's state (M 210). TallyPrime 7.2 Silver (ProductVersion 1.1.7.2), 1381-M's release. All times are UTC on 9 Nov 2026. Every number is read from its file, and `evidence/notes.txt` has each step. check_r1.py's whole output for each step is in `evidence/s3-check.txt` to `s8-check.txt`.

## What it reports
| step | result |
|---|---|
| S0 | step 0 met; the one company loaded alone; m0: one COMPANY row, M 210, ALTVCHID absent |
| S2 backup (C5) | `TDBK1800_<company number>.001`, 24,219 B, sha256 `612c3adc5a88606437937b89c3d4a8c45445902f2dbe52519f194f0cb9bd7aef`, in the new empty folder `C:\lab\1381-r1\backup-1`, written 06:07:31Z (the Restore list showed 9-Nov-26 11:37 am). Marks after it byte-equal to m0 |
| S3 (a) | five reads, each a collection of rows. `fetch`: same GUIDs; a0's fields all equal in a1; a1 adds `BASEUNITS`, `GSTAPPLICABLE` and `GSTTYPEOFSUPPLY`, each with a value. `LANGUAGENAME.LIST` is already in a0 and equal in a1. `diff` against 1381-M's base-items (sha256 checked against its `kept-local.txt`): unchanged. a1 is byte-equal to 1381-M's read after its restore (`rst-items`); it differs from base-items only in six count lines of the CMPINFO block, which are not rows. One name entry; g1 `SC Goods Group`, u1 `Nos`. m1 = m0 |
| S4 | `make_b.py plain` and `marked`, no refusal. b1-create 3,646 B `8f1e50a6...`, 0 marked values; b2-create 3,674 B `6961f475...`, 3 marked values |
| S5 b1 | CREATED 1, every other counter 0; M 210 to 211. `same` against the sibling and against R2's item, each: `PARENT`, `BASEUNITS`, `GSTTYPEOFSUPPLY` and `HSNDETAILS.LIST` same; `GSTAPPLICABLE` absent in the copy; `GSTDETAILS.LIST` 4 descendants in the copy against 24. The copy's block holds `APPLICABLEFROM`, `TAXABILITY`, `SRCOFGSTDETAILS` and an empty `STATEWISEDETAILS.LIST`: no `STATENAME`, no rate rows (`evidence/s6-gst-tags.txt`). No LINEERROR |
| S6 b2 | CREATED 1, every other counter 0; M 211 to 212. `same` against the sibling and against R2's item: all six fields same, 24 descendants each. b1's row unchanged by it |
| S7 (c) | Alter, Stock Item, SC Sibling Goods; alias `SC Sibling Alias` typed and the screen accepted with no prompt. M 212 to 213. `names`: two name entries, `SC Sibling Goods` and `SC Sibling Alias`. `diff` a1b2 to a1c: only SC Sibling Goods changed, in `ALTERID` (210 to 213) and `LANGUAGENAME.LIST` (the alias as a second `NAME` in the same list); the two copies unchanged |
| S8 restore (C6) | the plan written; the backup's sha256 checked, one file in its folder; THE COMPANY shut; the list showed it with its number and 9-Nov-26 (11:37 am); "Accept ?" Yes; Tally's "Company data folder (<company number>) already exists ..." Replace; no other prompt. Loaded alone; the Gateway with no dialog. Marks byte-equal to m0 (M 210); a1r, g2 and u2 byte-equal to a1, g1 and u1; `diff` of each: nothing added, removed or changed |
| S9 (C10) | Company, Shut; Tally at Select Company, none loaded; one tally.exe owning 9001; /status 200; current date as at S0. No XML request after the shut |

Every request was answered in under 0.35 s by curl's total (`evidence/send-log.txt`).

## What it decides
- **(a)** Every condition holds except the count. a0's fields are equal in a1 and a1 equals 1381-M's base-items read row for row. Of the four fields the candidate read adds to the fetch list, three come back only with it, each with a value. The fourth, `LANGUAGENAME.LIST`, comes back with the shipped read too (1 element, 3 descendants in both). The maintainers decide whether that holds as "adds the four".
- **(b)** b1 was created, but it is different against both: Tally stored it without `GSTAPPLICABLE` and without its state row and rates, and said nothing in the counters. b2 was created and is the same against both. That is the sheet's second row: the builder needs closed-set types for those three fields, written in Tally's marked form.
- **(c)** Two name entries, the second `SC Sibling Alias`, and the diff shows only that item's name entries and AlterID changed. The alias refusal rests on the name entries.

## Lab slip (S7)
The first typing of the alias put `SCSiblingAlias` in the field: Tally did not receive the two spaces. Before any accept, the field was cleared with 14 Backspaces and typed again in three parts with the space key between them. A zoom showed `SC Sibling Alias` before the accept. The read after it shows the alias as typed, 16 characters.

## Kept on the lab PC
Each read answer holds the company's GUID. These are listed by bytes and sha256 in `kept-local.txt`, as the sheet's data step says. Also listed there: the backup, the restore plan, the data-folder listings, the process and port listings and this row's stage script. The import answers (b1, b2) carry no GUID and are here whole, and every answer's headers are here. The screenshots stay on the lab PC and are listed in `SCREENSHOTS.txt`.

## Limits
One sitting, one book, one release, one copy of each form. b1's result is what Tally stored, read back; the counters alone did not show it.
