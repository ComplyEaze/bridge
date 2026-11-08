# Rows 2162-L1 to L5: results

First line: no stop rule met. **L1**: each of the six screen actions (create, narration, number change, re-date, cancel, delete) moved V (ALTVCHID) by exactly 1 and M (ALTMSTID) by 0. **L2**: in Voucher Type Alteration Tally does not let the cursor reach "Select type of voucher": Return and Shift+Tab both pass over it, with no message. So the parent could not be changed; M did not move, and `by8` is byte-equal to `by7` (IX-0003 still in IX SALES). Creating the type IX SALES moved M by 2. **L3**: the fetch change and the `$AlterID > K` term both serve on the pilot book; on the dense book the delta read answered 100 rows, all above K, in 15.08 s. **L4**: the marks survived a reload and a restart. **L5**: twelve dense month reads were byte-equal, and the thirteen marks reads byte-equal. The book of Part 1 was restored and checked: the marks equal `bm0` and the 1 Oct read holds no `IX-` number.

**Deviation, as measured.** BRIDGE BILLTRAIL LAB's Sales type numbers vouchers by itself, and its number field cannot be reached on the screen, so `IX-0001` could not be typed there. On the run lead's decision, the empty Sales voucher was left unsaved (Tally's own "Quit ?", Yes), and k1 to k7 ran in a child type, IX SALES, made under Sales with Manual numbering before k1. With Manual, Tally shows no prefix, number of digits or prefill fields; "Prevent creating duplicate Voucher Nos." was set to Yes. So k8 would have moved a type that also held the k1 to k6 history, not only k7's voucher.

Sheet: `lab/2162-index-sheet` at `bdd16647`, kit `lab/2162-ix/`, `SHA256.txt` 16 of 16 OK. TallyPrime 7.2 Silver (tally.exe FileVersion 1.1.7.2). Books, all synthetic: BRIDGE BILLTRAIL LAB (Part 1, writes behind a backup), BRIDGE PILOT LAB (Part 2, reads only), BRIDGE CORPUS DENSE (Part 3, reads only). Parts ran in the order 2, 3, 1. All times are UTC, 8 Nov 2026. Every number below is read from its file in this folder; `notes.txt` has each step's lines and screens.

## Part 2: BRIDGE PILOT LAB (L3, L4)

| step | result |
|---|---|
| P1 | Loaded alone; current date Tuesday 15-Sep-2026. `pm1`: 3,394 B, one COMPANY row, GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, ALTVCHID 68, ALTMSTID 245. |
| P2 | `A-Y0-year.xml`: 117,992 B, HTTP 200, first byte 0.121 s; 41 rows. |
| P3 | `L3a-pilot.xml`: 121,512 B, 0.078 s; 41 rows. count8: `rows 41 and 41; rows with one numeric ALTERID 41; equal less ALTERID: yes`. |
| P4 | `gen8.py k a-pm1-M-pilot.xml`: `ALTVCHID 68; K 58`. `L3b-pilot.xml` 1,722 B, sha256 `bbd55cf9...`; `k.txt` 19 B. |
| P5 | `L3b-pilot.xml`, one send, watched: 17,524 B, 0.075 s; capture after it byte-equal to the Gateway, no dialog. count8: `rows 5 and 5; element for element equal: yes` against P3's rows with ALTERID above 58. |
| P6 | Shut by Company, SHut, then loaded again: `pm6` byte-equal to `pm1`. Tally then quit by its own Quit ("Quit ?", Yes) and was started again from its own folder. At the restart Tally opened its start-up company on its own; nothing was sent while it was loaded; it was shut by Company > Shut. The three captures of that time stay on the lab PC by sha256 only. BRIDGE PILOT LAB loaded alone: `pm7` byte-equal to `pm1` (V 68, M 245). |
| P8 | Shut by Company, SHut. |

## Part 3: BRIDGE CORPUS DENSE (L3 cost, L5)

| step | result |
|---|---|
| D1 | Loaded alone. `dm1`: 3,406 B, GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, ALTVCHID 29900, ALTMSTID 328: met. |
| D2 | `L3b-dense.xml`, one send, watched: 292,626 B, HTTP 200, first byte 15.082 s, total 15.090 s; capture after it byte-equal to the Gateway, no dialog. count8: `rows 100, with ALTERID above 29800 100`; `rows found in S6's Y0-year, less ALTERID: 100 of 100`. All 100 rows Sales, dated 30 and 31 Mar 2026. |
| D3 | `d3.py`: twelve `B-Y0-month.xml` sends (`l501` to `l512`), each 6,792,322 B, HTTP 200, first byte 1.75 s to 3.09 s, all twelve byte-equal (sha256 `9262740a...`); after each, `M-dense.xml` (`dm2` to `dm13`), each byte-equal to `dm1`. count8: all 66 pairs `rows 2430 and 2430; element for element equal: yes`. |
| D4 | S6's kept `run/a-s6-Y0-year.xml` copied beside count8.py as `s6-Y0-year.xml`, 83,742,646 B, sha256 `1db925db...`, as count8 checks. |
| D5 | Shut by Company, SHut. One `tally.exe` on 9001; GET /status 200, 51 B. |

## Part 1: BRIDGE BILLTRAIL LAB (L1, L2)

Each key step is followed by a marks read `bm<n>` (`M-billtrail.xml`), then the day read the sheet names.

| step | what was keyed | Tally's prompts | marks after (V, M) | change | day read |
|---|---|---|---|---|---|
| B0 | Loaded alone; BOOKSFROM 20250401 | none | `bm0` 7, 212 | baseline | |
| B1 | Backup to the new empty folder `C:\lab\2162-ix\backup-1`: one file, 33,477 B, sha256 `fbfa30cc...` | "Accept ?" Yes | `bm1` = `bm0` | | |
| k0 | Sales voucher (No. field unreachable) left empty, Escape | "Quit ?" Yes | `bm2` = `bm1` | 0, 0 | |
| type | IX SALES created under Sales, Manual numbering, duplicates prevented | none | `bm3` 7, 214 | V 0, M +2 | |
| k1 | IX SALES `IX-0001`, 1 Oct 2025, Lab Party P1, Lab Sales 100.00, bill New Ref `IX-0001` | none | `bm4` 8, 214 | V +1 | `by1` (1 Oct): one row, IX-0001, MASTERID 8 |
| k2 | narration `ix` | none | `bm6` 9, 214 | V +1 | `by2`: row unchanged (Y0 fetches no narration) |
| k3 | number to `IX-0002` | none | `bm7` 10, 214 | V +1 | `by3`: IX-0002, same GUID |
| k4 | date to 2 Oct 2025 | none | `bm8` 11, 214 | V +1 | `by4` (2 Oct): IX-0002, 20251002 |
| k5 | Cancel Vch (Alt+X) | "Cancel ?" Yes | `bm9` 12, 214 | V +1 | `by5`: ISCANCELLED Yes |
| k6 | Delete (Alt+D) | "Delete ?" Yes | `bm10` 13, 214 | V +1 | `by6`: no row |
| k7 | IX SALES `IX-0003`, 1 Oct 2025, as k1 | none | `bm11` 14, 214 | V +1 | `by7` (1 Oct): IX-0003, MASTERID 9 |
| k8 | Alter IX SALES: the parent field is passed over by Return and by Shift+Tab; nothing changed; left by Escape | "Quit ?" Yes | `bm12` = `bm11` | 0, 0 | `by8` byte-equal to `by7` |
| B10 | Backup sha256 equal to B1's, its folder holds that one file; `restore-plan.txt`; the company folder held 24 entries | | | | |
| B11 | Shut by Company, SHut; Data, Backup & Restore, Restore; source by Specify Path; the list held BRIDGE BILLTRAIL LAB alone, 8-Nov-26 (4:55 pm) | "Accept ?" Yes; `Company data folder (<company number>) already exists ...` Replace | | | |
| B12 | Loaded alone; Gateway, no dialog | none | `bm13` = `bm0` (7, 212): met | | `by9` (1 Oct): no row, no `IX-`: met |
| B13 | Shut by Company, SHut. One `tally.exe` on 9001; /status 200, 51 B | | | | |

Notes to the table:
- A mouse click on the narration area during k2 left the alteration screen without saving and with no prompt; `bm5` was byte-equal to `bm4`. k2 was then keyed by Return.
- Before k5, Page Up did not open IX-0002, dated after the screen's date; F2 set the screen's date to 2 Oct. Page Up then gave Tally's own "Quit ?" for the empty entry, the prompt k0 met: No once, then Yes, and IX-0002 opened.
- A delete on the screen moved V by 1; the Gateway delete measured in the design (§11c.5) moved it by 2.
- The restore folder afterwards held 17 entries, all written 12:03:37Z. One extra read, `by9b` (the 2 Oct day), was sent after B12: no row, byte-equal to `by9`.

## What it decides

- **L1**: every action k1 to k6 moved V by at least 1 (each by exactly 1), so the index's equality check is sound for these screen actions, as measured in a child type of Sales.
- **L2**: the parent of a voucher type could not be changed in alteration on this release, and the attempt moved neither mark. Creating a type moved M by 2. Whether a class change moves a mark is not measured here: Tally gave no way to make one from the screen. Recorded as measured, for the run lead to place.
- **L3**: P3 equals P2 less ALTERID, with one numeric ALTERID on every row: the fetch change serves. P5 equals P3's rows above K: the term serves, with no dialog. D2's rows all lie above 29800 and are found in S6's year; its 15.08 s is the delta read's cost on the dense book.
- **L4**: `pm6` and `pm7` equal `pm1`: the marks survive a reload and a restart.
- **L5**: all twelve month reads are equal, and `dm2` to `dm13` equal `dm1`.

## Files

- `run/`: every answer `a-<label>-<request>` whole (each is 10 MB or less), `L3b-pilot.xml` and `k.txt` written in P4, each send's log line, count8's P5 output, D2's run lines and the `rows.py` helper. The requests and the kit's scripts are the sheet's, byte-equal to `lab/2162-ix/`.
- `evidence/`: the C2 reads (netstat cut to the port-9001 lines), /status bodies, the backup folder listings of B1, the company folder listings of B10 and B11, `d3-run.txt`, and count8's whole output after each part (`count8-part2.txt`, `count8-part3.txt`, `count8-part1.txt`).
- `notes.txt`, `restore-plan.txt`, and the helper scripts.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt` lists what the pushed copies of the lab's own text changed. `kept-local.txt` lists what stays on the lab PC and why. `SHA256.txt` lists every file.
