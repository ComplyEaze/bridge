First line: **Every EXPECT met. All seven probes answered CREATED 1 with every other counter 0 and no LINEERROR, and each stored row holds every leaf sent: o1 to o4 and o7 are the first column of Decides. o4's rates come back as 2.50 for the 2.5 sent, equal in value. o5 (no PARENT) and o6 (`&#4; Primary`) both put the item under the root, and both read PARENT as `&#4; Primary`. The restore was checked: every read after it is byte-equal to the read before the writes. No stop, no skip, no dialog.**

Sheet `lab/1381-o-sheet` at `54eb636f`, kit 20 of 20 OK by its `SHA256.txt`; `gen_o.py`'s six requests equal it. Book BRIDGE STOCK CREATE LAB (synthetic), GUID `bac2b5e9...` (master's `scripts/merge-gate-synthetic-uuids.json`), loaded alone, at BK0's state (M 210). TallyPrime 7.2 Silver (ProductVersion 1.1.7.2). All times are UTC on 19 Nov 2026. Every number is read from its file, and `evidence/notes.txt` has each step. check_r1.py's whole output for each step is in `evidence/s3-checks.txt`, `evidence/o1-checks.txt` to `o7-checks.txt` and `evidence/s6-checks.txt`.

## What it reports
| step | result |
|---|---|
| S0 | step 0 met; the one company loaded alone; m0: one COMPANY row, the book's GUID, M 210, ALTVCHID absent |
| S1 | `SHA256.txt` 20 of 20 OK, checked again before S2 |
| S2 backup (C5) | `TDBK1800_<company number>.001`, 24,219 B, sha256 `d23f881a6bac8e1f9bf53035e7f21a72234f9f57c3672a965bc23a3dd76d44eb`, in the new empty folder `C:\lab\1381-o\backup-1`, accepted 10:44:02Z (the Restore list showed 19-Nov-26, 4:14 pm). Marks after it byte-equal to m0 |
| S3 | a1, g1, u1 each a collection of rows: a1 one item `SC Sibling Goods` with one name entry, g1 `SC Goods Group`, u1 `Nos`. a1 is 8,270 B, sha256 `92508c8e...`, byte-equal to row 1381-N's `a-a1.xml`; `diff` against it: unchanged. m1 = m0 |
| S4 o1 to o6 | each sent once by `probe.py`: marks before, the write, a window capture, `import`, marks after, a1, `diff`, `rows`. Each answered CREATED 1; ALTERED, DELETED, COMBINED, IGNORED, ERRORS, CANCELLED and EXCEPTIONS 0; no LINEERROR. M went 210 to 216, one per write; V absent throughout. Each diff: one item added, every earlier row unchanged. Every capture after a write was byte-equal to the Gateway capture (no dialog) |
| S5 o7 | `make_o7.py marked answers/a-a1-o1.xml` printed `written: 4 scalars, 2 GST entries (2 state rows), 1 HSN entries, 5 marked values`; `o7-create` 5,752 B, sha256 `8b1c39ce...`. CREATED 1, other counters 0, no LINEERROR; M 216 to 217; diff: `SO Two Dates Copied` added, seven rows unchanged. `same` against `SO Two Gst Dates`: `PARENT`, `BASEUNITS`, `GSTAPPLICABLE` and `GSTTYPEOFSUPPLY` same; `GSTDETAILS.LIST` same, 2 elements and 48 descendants each; `HSNDETAILS.LIST` same, 1 element each |
| S6 restore (C6) | the plan written; the backup's sha256 checked, one file in its folder; THE COMPANY shut; the list showed it with 19-Nov-26 (4:14 pm); "Accept ?" Yes; Tally's "Company data folder (<company number>) already exists ..." Replace; no other prompt. Loaded alone; the Gateway with no dialog. m-r, a1-r, g-r and u-r byte-equal to m0, a1, g1 and u1; `diff` of each: nothing added, removed or changed |
| S7 (C10) | Company, Shut; Tally at Select Company, none loaded; one tally.exe owning 9001; /status 200; current date as at S0. No XML request after the shut |

38 requests in all, each answered in under 0.4 s by curl's total (`evidence/send-log.txt`).

## What was stored, probe by probe
`evidence/s5-sent-vs-stored.txt` looks for every leaf each request sent in the item's stored row in `a-a1-o7.xml` (a sent `NAME` is looked for in `LANGUAGENAME.LIST`, where Tally keeps it). Every probe: no leaf missing. The whole stored rows are in `evidence/s4-rows-a1-o6.txt` and `evidence/s5-rows-o7-and-o1-item.txt`.

| probe | item | sent leaves, each kept | stored |
|---|---|---|---|
| o1 | SO Two Gst Dates | 43 | two `GSTDETAILS.LIST`: from 20250401 CGST 9, SGST/UTGST 9, IGST 18; from 20251001 CGST 6, SGST/UTGST 6, IGST 12 |
| o2 | SO Two Hsn Dates | 30 | two `HSNDETAILS.LIST`, from 20250401 and from 20251001, each HSN 39239090 |
| o3 | SO Cess Rate | 27 | Cess `Based on Value`, rate 12 |
| o4 | SO Other Rates | 26 | CGST 2.50, SGST/UTGST 2.50, IGST 5 (sent 2.5, 2.5, 5) |
| o5 | SO Root Parent | 25 | no PARENT sent; PARENT reads `&#4; Primary` |
| o6 | SO Root Marked | 26 | PARENT `&#4; Primary` sent; PARENT reads `&#4; Primary` |
| o7 | SO Two Dates Copied | 43 | equal to o1's item in every field `same` compares |

## What it decides
- **o1** A copy of an item with more than one dated GST entry is measured once, with both entries stored as sent.
- **o2** A copy of an item with more than one dated HSN entry is measured once, with both entries stored as sent.
- **o3** A Cess valuation of `Based on Value` and rate 12 are stored as sent.
- **o4** CGST 2.5, SGST/UTGST 2.5 and IGST 5 are stored as sent; Tally writes the two halves back as 2.50.
- **o5 and o6** Both forms put the item directly under the root, and PARENT reads `&#4; Primary` for both. Leaving PARENT out and sending the marked `&#4; Primary` address the root alike.
- **o7** A copy of an item with more than one dated entry is measured once through the product's own read-and-write path; every entry is stored as read.

## Kept on the lab PC
Each read answer (marks, a1, groups, units) holds the company's GUID. These are listed by bytes and sha256 in `kept-local.txt`, as the sheet's data step says. Also listed there: the backup, the restore plan, the backup record, the company data-folder listings, the process and port listings and this row's stage script. The import answers (o1 to o7) carry no GUID and are here whole, and every answer's headers are here. The screenshots stay on the lab PC and are listed in `SCREENSHOTS.txt`.

## Limits
One sitting, one book, one release, one copy of each form. What was stored is what Tally read back through the row's own a1 read; a field that read does not fetch shows only in M.
