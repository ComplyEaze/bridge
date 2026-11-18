First line: **(a) Tally takes all four near names for the existing item: a Create named with a hyphen, a slash, a removed space or removed full stops answered ALTERED 1, added no row, and rewrote only that item's display name. (b) The folded Create rewrites the name only: on an item with GST and HSN lists, no list or other field changed. (c) Tally lets an item take a stock group's name and a unit's name, and the group and unit reads are unchanged. (d) A Create named as another item's alias is refused in-band with 'Duplicate Entry!' and changes nothing. The restore was checked: every read after it is byte-equal to the read before the writes. No stop, no skip, no dialog.**

Sheet `lab/1381-n-sheet` at `3c207ab1`, kit checked by its `SHA256.txt` (a=26, b=0 mismatched); `gen_n.py`'s fourteen requests equal it. Book BRIDGE STOCK CREATE LAB (synthetic), GUID `bac2b5e9...`, equal whole to its entry in master's `scripts/merge-gate-synthetic-uuids.json`, loaded alone, at BK0's state (M 210). TallyPrime 7.2, edition as recorded (ProductVersion 1.1.7.2). All times are UTC on 18 Nov 2026. Every number is read from its file, and `evidence/notes.txt` has each step. check_r1.py's whole output for each step is in `evidence/s0-check.txt` to `s8-check.txt` and `evidence/<write>-check.txt`.

## What it reports
| step | result |
|---|---|
| S0 | step 0 met; the one company loaded alone; m0: one COMPANY row, the book's 8, M 210, ALTVCHID absent; the whole GUID equal to the registry entry |
| S1 | kit a=26, b=0 mismatched; `gen_n.py`'s fourteen lines equal `SHA256.txt`'s as a set |
| S2 backup (C5) | `TDBK1800_<company number>.001`, 24,219 B, sha256 `8454721f6de95211afc8fa1d76054fad25356056a153ab084934c428cd48de7b`, in the new empty folder `C:\lab\1381-n\backup-1` (the Restore list showed 18-Nov-26, 1:05 pm). Marks after it byte-equal to m0 |
| S3 | a1 byte-equal to row 1381-R1's a1 (8,270 B, `92508c8e...`); one item `SC Sibling Goods` with one name entry, one group `SC Goods Group`, one unit `Nos`; m1 = m0 |
| S4 | `make_b.py marked`: b2-create 3,674 B, `6961f475...`, as the sheet expects |
| S5 t1 to t5, b2 | each CREATED 1, every other counter 0; M up by 1 each time, 210 to 216; each `diff`: exactly one row ADDED with the name sent, nothing CHANGED. After b2, `same` SC Copied Marked against SC Sibling Goods: all six fields same |
| S6 n1 to n8 | the table below |
| S7 | Alter, Stock Item, `SN Alias Item` found by the list's search; the Alteration screen showed its name and an empty alias. Alias `SN Alias Other` typed in parts, read by zoom, accepted with Ctrl+A and no prompt. M 224 to 225. `names`: two entries, `SN Alias Item` and `SN Alias Other` |
| S7 n9 | the table below |
| S8 restore (C6) | the plan written and the backup re-hashed before the shut, one file in its folder; THE COMPANY shut; the list showed it with 18-Nov-26 (1:05 pm); "Accept ?" Yes; Tally's "Company data folder (<company number>) already exists ..." Replace; no other prompt. Loaded alone; the Gateway with no dialog. m-r byte-equal to m0 (M 210); a1-r, g-r and u-r byte-equal to a1, g1 and u1; `diff` of each: nothing added, removed or changed |
| S9 (C10) | Company, Shut; Tally at Select Company, none loaded; one tally.exe owning 9001; /status 200; current date as at S0. No XML request after the shut |

| probe | name sent | counters | M | what the diff shows |
|---|---|---|---|---|
| n1 | `SN Hyphen-Item` | ALTERED 1, others 0 | 216 to 217 | no row added; `SN Hyphen Item` CHANGED in `ALTERID` and `LANGUAGENAME.LIST` only: its name entry now reads `SN Hyphen-Item` |
| n2 | `SN Slash/Item` | ALTERED 1, others 0 | 217 to 218 | no row added; `SN Slash Item` CHANGED in `ALTERID` and `LANGUAGENAME.LIST` only: `SN Slash/Item` |
| n3 | `SN SpaceItem` | ALTERED 1, others 0 | 218 to 219 | no row added; `SN Space Item` CHANGED in `ALTERID` and `LANGUAGENAME.LIST` only: `SN SpaceItem` |
| n4 | `SN Stops Item` | ALTERED 1, others 0 | 219 to 220 | no row added; `S.N. Stops Item` CHANGED in `ALTERID` and `LANGUAGENAME.LIST` only: `SN Stops Item` |
| n5 | `sc sibling goods` | ALTERED 1, others 0 | 220 to 221 | no row added; `SC Sibling Goods` CHANGED in `ALTERID` and `LANGUAGENAME.LIST` only (name `sc sibling goods`, `LANGUAGEID` 1033 kept). No CHANGED line for `PARENT`, `BASEUNITS`, `GSTAPPLICABLE`, `GSTTYPEOFSUPPLY`, `GSTDETAILS.LIST` or `HSNDETAILS.LIST`. `names` by the original name: one entry, `sc sibling goods` |
| n6 | `SC Copied  Marked` | ALTERED 1, others 0 | 221 to 222 | no row added; `SC Copied Marked` CHANGED in `ALTERID` and `LANGUAGENAME.LIST` only (name `SC Copied  Marked`, 17 characters). No CHANGED line for the six fields. `names` by the original name: one entry, the doubled-space spelling |
| n7 | `SC Goods Group` | CREATED 1, others 0 | 222 to 223 | one stock item `SC Goods Group` ADDED, nothing CHANGED; g-n7 against g1: the one group unchanged |
| n8 | `Nos` | CREATED 1, others 0 | 223 to 224 | one stock item `Nos` ADDED, nothing CHANGED; u-n8 against u1: the one unit unchanged |
| n9 | `SN Alias Other` | ERRORS 1, others 0; LINEERROR `Duplicate Entry!` | 225, unchanged | a1-n9 byte-equal to a1-c; `names` on `SN Alias Item` still two entries |

No probe raised a dialog; the screen was checked after every write. Every request was answered in under 0.49 s by curl's total (`evidence/send-log.txt`, 74 sends).

## What it decides
- **(a) n1 to n4.** Each is the sheet's "ALTERED 1 and no row added" column: Tally takes a hyphen for a space, a slash for a space, a removed space and removed full stops as the existing item's name. The refusal of those folds is a Tally fact for stock items on this release. Tally then stores the spelling sent as the item's display name; the item's `NAME` attribute in the read stays the old one.
- **(b) n5 and n6.** On an item holding GST and HSN lists, a name-and-unit-only Create under a folded name (case only; a doubled space) changed only `ALTERID` and the name entry. The lists, `PARENT`, `BASEUNITS`, `GSTAPPLICABLE` and `GSTTYPEOFSUPPLY` are unchanged: it rewrites the name only. The stored name becomes the spelling sent.
- **(c) n7 and n8.** With the group and unit reads unchanged, Tally lets an item share a stock group's name and a unit's name. An item index holding stock items only misses no collision Tally enforces here.
- **(d) n9.** Tally refuses in-band, ERRORS 1 with `Duplicate Entry!`, and nothing changes (M unchanged, the read byte-equal). A Create named as another item's alias does not reach the aliased item and does not make a new one.

## Also seen (S7)
The alias was the only field keyed on `SN Alias Item`, an item created with a name and a unit only. After Ctrl+A the read shows more than the second name entry. `GSTAPPLICABLE` went from absent to `Applicable`, `GSTTYPEOFSUPPLY` from absent to `Goods`, and `GSTDETAILS.LIST` and `HSNDETAILS.LIST` from empty to filled ("As per Company/Stock Group", from 2025-04-01, five rate heads). The Alteration screen had shown GST applicability Applicable and "As per Company/Stock Group" before any key. Saving the screen stores the values it shows. M rose by 1.

## Kept on the lab PC
Listed by bytes and sha256 in `kept-local.txt`: the backup, the restore plan, the data-folder listings, the process and port listings, and this row's stage script. Any captured file holding the company number or a run of ten or more digits would be listed there too, unedited. The screenshots stay on the lab PC and are listed in `SCREENSHOTS.txt`. The C10 capture differs in bytes from step 0's only by window focus (title bands grey at step 0, blue at C10); both show Select Company with no company loaded.

## Limits
One sitting, one book, one release, one spelling of each kind. For n1 to n6 the `NAME` attribute in a1-n1 to a1-n6 keeps the old spelling, and the List of Stock Items at S7 showed the old spellings. Whether other Tally screens or reports show the new spelling was not read.
