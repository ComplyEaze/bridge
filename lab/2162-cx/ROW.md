# Row 2162-L2b: does changing a voucher type's parent, and so its class, move V or M? The open row of the index design's section 5

**Not for tonight.** It runs at a later lab sitting, on the run coordinator's word.

Not for merge. Part of #2162. Written 8 Nov 2026. The design is `lab/2162-index-design/DESIGN.md` at cf240434, section 5 and the deviation "screen actions and the marks". The rules of `LAB-COMMON.md` (v3, byte-equal to `lab/2162-ix/LAB-COMMON.md`) apply throughout. Confidence of everything here until the row runs: unverified.

## Question
Part 1 of the index sheet (`lab/2162-index-answers` at 2cf5df51, k8) could not change `IX SALES`'s parent: in Voucher Type Alteration, Return and Shift+Tab both passed over the field, and neither mark moved. So the question is still open. Two attempts, each changing one thing from k8:
- **c1, on the screen, in a type that holds no voucher.** k8's type held one voucher. Whether the field is reachable then is the one difference.
- **c2, through the Gateway, in the type that holds one voucher, as k8's did.** One All Masters Alter of `IX SALES` with its `PARENT` set to Journal, in Bridge's import envelope. The route is the one difference.

For each attempt the row records: whether the change was made, V and M before and after, the type's `PARENT` as read, and, for c2, whether its voucher's `BRIDGEVCHISSALES` (Y0's computed flag) changed.

## Book
`BRIDGE BILLTRAIL LAB`, as Part 1 used it, at its `bm0` state (V 7, M 212), to which Part 1's restore returned it. Any other company, a GUID other than the lab's own record of this book, Education mode, or other marks: STOP, there is no book. The row writes to it behind a C5 backup and ends with a checked C6 restore.

## Kit
`SHA256.txt` lists every file with its sha256 and size: check it first.

| file | what |
|---|---|
| `LAB-COMMON.md` | v3, byte-equal to `lab/2162-ix/LAB-COMMON.md` |
| `M-billtrail.xml` | the marks read, byte-equal to `lab/2162-ix/M-billtrail.xml` |
| `Y0-bt-20251001.xml` | Y0 for 1 Oct 2025, byte-equal to `lab/2162-ix/Y0-bt-20251001.xml`. Its fetch list carries `VOUCHERTYPENAME` and the computed `BRIDGEVCHISSALES` |
| `gen_c.py` | writes `vt-read.xml` and `c2-alter.xml` (below) and checks the two copies by sha256 |
| `vt-read.xml` | master's voucher-type read (`native_masters.rs:95-102`, `:241-246` at 1f5dc024): `List of VoucherTypes`, fetch `NAME, PARENT, GUID, MASTERID, ALTERID, ISACTIVE, ISOPTIONAL, NUMBERINGMETHOD`. Equal, when written, to `lab/1381-r1/gen_r1.py`'s envelope for that kind |
| `c2-alter.xml` | one All Masters import: `VOUCHERTYPE NAME="IX SALES" ACTION="Alter"` holding only `NAME` and `PARENT` Journal. Its envelope was equal, when written, to `lab/1381-r1/m-r1-name-unit.utf16le.xml` less its message. **A shape Tally has not been sent** |
| `check_c.py` | `lab/1381-r1/check_r1.py` with `types` and `vouchers` modes added; `marks` fails unless the answer holds exactly one COMPANY row |

When the sheet was written, gen_c.py was run and its two lines are in SHA256.txt. check_c.py was run on answers built by hand: a VOUCHERTYPE row with a CMPINFO count line, a voucher row, an import answer, a voucher row without a number (refused, exit 1), and a marks read without a COMPANY row (refused, exit 1).

## Steps (one sitting)
Requests go one at a time by C3's line. **After every write, send `M-billtrail.xml`** and run `check_c.py marks` on the answer.
- **S0.** C1, then C2 (step 0). Record Tally's release: Part 1 ran on TallyPrime 7.2 Silver. Another release: STOP. Load THE COMPANY alone. Marks (`cm0`): EXPECT V 7, M 212.
- **S1.** Check the kit against SHA256.txt. Run `python3 -I gen_c.py`: EXPECT its two lines to equal SHA256.txt's.
- **S2. Backup (C5)** to a new empty local folder `C:\lab\2162-cx\backup-1`. Record its name, bytes and sha256. Marks: EXPECT `cm0`.
- **S3. The type, as Part 1 made it.**
  - Create the voucher type `IX SALES` under Sales, Manual numbering, duplicates prevented. Marks (`cm1`): EXPECT M +2, V +0, as Part 1.
  - Key one voucher in it on 1 Oct 2025, number `IX-0003`, `Lab Party P1`, `Lab Sales` 100.00, bill New Ref `IX-0003`, as Part 1's k7. Marks (`cm2`): EXPECT V +1.
  - Send `vt-read.xml` (`vt1`) and `Y0-bt-20251001.xml` (`y1`). Run `check_c.py types` on vt1 and `vouchers` on y1. EXPECT `IX SALES` under Sales, and one row `IX-0003` with its BRIDGEVCHISSALES as received.
- **S4. c1, on the screen.**
  - Create a voucher type `IX EMPTY` under Sales, Manual numbering. Marks (`cm3`). Key no voucher in it.
  - Open it in Alter (Gateway, Alter, Voucher Type). Screenshot. Move to "Select type of voucher" by Return, then try Shift+Tab. **No mouse:** in Part 1 a click left an alteration screen without saving or a prompt.
  - If the field is reached, set it to Journal and accept the screen. Screenshot. Marks (`cm4`). Then `vt-read.xml` (`vt2`) and `check_c.py types`.
  - If it is not reached, record what the cursor did, then leave by Escape. Answer Tally's own "Quit ?" Yes, as k8. Marks (`cm4`): EXPECT `cm3`.
- **S5. c2, through the Gateway, one send, watched.**
  - Marks (`cm5`): EXPECT `cm4`.
  - Send `c2-alter.xml`. Screenshot Tally's window as soon as the answer is written. Run `check_c.py import` on the answer.
  - A LINEERROR, or ERRORS or EXCEPTIONS above 0, is an answer, not a stop: record its words.
  - Then marks (`cm6`), `vt-read.xml` (`vt3`) and `Y0-bt-20251001.xml` (`y3`). Run `check_c.py types` on vt3, `vouchers` on y3, `diff vt1 vt3`, and `diff vt2 vt3` if S4 sent vt2.
- **S6. Restore (C6)** of S2's backup: the plan, the restore by the operator, and the listings, all as C6 says. Load THE COMPANY alone. Then marks (EXPECT V 7, M 212), `vt-read.xml` (`vt4`) and `Y0-bt-20251001.xml` (`y4`): EXPECT no `IX SALES` and no `IX EMPTY` in vt4, and no `IX-` row in y4. Any difference: STOP; the way back is S2's backup, restored again only on the run coordinator's word.
- **S7.** C10.

## Stop rules
LAB-COMMON v3's rules, and these:
- a release other than Part 1's, or `cm0` other than V 7, M 212;
- any dialog or prompt that this sheet does not name, including one raised by c2's send;
- a marks read that fails or lacks its COMPANY row. That is no answer, never "unchanged";
- a restore whose marks or reads differ (S6);
- no answer in 15 minutes. Press nothing and kill nothing.

## Record
- **Every send:** bytes, sha256, rows, and curl's first-byte and total seconds.
- **The marks after each write:** V and M, and their difference from the step before.
- **Every screen step:** Tally's own words, where the cursor went, and the screenshots.
- **check_c.py's whole output** for every command run.

## Decides
- **c1 or c2 changes the parent** (vt shows `IX SALES` or `IX EMPTY` under Journal):
  - if it moved M, or V, the design's check of both marks sees a class change;
  - if it moved neither, a class change is invisible to the marks. Section 2 must then carry the voucher types, with their parents, in what the index keeps, and compare them at each check.

  For c2, y3's BRIDGEVCHISSALES says whether the type's voucher changed class with it.
- **Neither changes the parent:** a class change can be made by neither route on this release. That is recorded, with each refusal's words, and the row stays open for another release.

## Keep
On `lab/2162-cx-answers`, with `SHA256.txt`:
- every answer whole. The book is synthetic, and its GUID is in master's synthetic-uuid registry only if the run coordinator says so; otherwise the reads carrying it stay on the lab PC, listed by bytes and sha256 in `kept-local.txt`;
- check_c.py's whole output for each step;
- the notes in UTC, the backup and restore records, and the screenshots by name, size and sha256.

Send the sha to the run coordinator before any push.

Row: lab/2162-cx/
