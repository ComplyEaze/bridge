# Row 2162-RU: how Tally writes a composition registration, an inactive registration, a second registration and a Sales type bound to one registration, as the build's own reads return them

Not for merge. Part of #2162 (ADR 0004, the 2026-10-31 checklist, item 16: the company's own registration, "the rest of S9"). Written 9 Nov 2026. Master alterations on Tally's screen, one variable per save, each read before and after, with a restore from the row's own backup between parts. Confidence of everything here until the row runs: unverified.

## Question
The Sales build reads the company's tax units and admits an invoice only when exactly one GST unit is in force on the invoice's date: its latest dated row on or before that date is not inactive and reads `Regular` (`parse_company_registration`, below). Section 12a.19 records the one shape ever read: one registration, one dated row, `Regular`, not inactive. It lists as not measured how Tally writes an inactive or composition registration, several registrations, and a voucher type bound to a tax unit. Each refusal in the parser for those cases was written from field names, not from an answer that held them.

This row asks, for each case, where Tally puts the change in the two answers the build reads (the tax units, U, and the voucher types, T):
- **Part A, one variable:** the registration's type, from `Regular` to `Composition`.
- **Part B, one variable:** the registration made inactive.
- **Part C, one variable per save:** a second GST registration in another state (C2, with C1 only if Tally needs a company setting first), then BRIDGE Sales bound to one of the two (C3).

What decides each part is whether the change lands in a field the parser reads (a dated row's `REGISTRATIONTYPE` or `ISINACTIVE`, a new `TAXUNIT` row with the book's GUID prefix), or somewhere it does not. **Not asked here:** GSTR-1 (rows 2162-B2C and 2162-SC look at it), and anything posted while the book is altered.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2162-KC's and 2162-SC's) applies in full. Where it and this sheet differ, the stricter one holds.
- **The book is pinned by GUID.** `count14.py` holds BRIDGE PILOT LAB's GUID as `BOOK`. Its M kind prints whether the COMPANY row's GUID is the book's, its U kind whether each tax unit's GUID begins with it, and `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called. STOP before anything is keyed unless s1's COMPANY row is the book's.
- Its state, from the lab's last notes on it: rows queued before this one each end with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245.
- **BK is this row's way back,** taken at step 1 before anything is keyed, and restored by C6 at the end of each part (three restores of the one BK).
- Write down the release shown on Tally's screen. Rows 2162-M1 and 2162-B2C read TallyPrime 7.2 Silver (tally.exe 1.1.7.2).

**No Bridge build.** Raw reads, one at a time, through `run.py`. Every change is made on Tally's screen, by hand; no request writes.

**The sends** are row 2162-SC's, unchanged. Each of these stops the run and is reported as "the request failed", never as "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`); a read answer with no `BODY/DATA/COLLECTION`, or an answer that does not parse (`count14.py`, which prints STOP and exits non-zero). Check each file's sha256 against `SHA256.txt` before its send. Count every answer with `count14.py` and copy its whole output into RESULTS.md. Close every other Bridge process first.

## Files
- **Byte-equal to row 2162-SC's file of the same name** (`lab/2162-sc-sheet` at c4881cf2, folder `lab/2162-sc/`), whose ROW.md says where each came from: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml` (the build's read-back for `BP/26-27/0010`) and `U-taxunits.xml` (the build's company registration read, `render_company_registration_request`).
- **`T-types.xml`:** byte-equal to rows 2162-KD's, 2162-FY14R's and 2162-KC's: the build's voucher-type read (`render_voucher_types_request`, `agent_import_invoice_wire.rs:267` at master ae38a75c). Row 2162-KD's answer to it (`lab/2162-keyed-dup-answers`, `a-s2-T-types.xml`) shows BRIDGE Sales holding `TAXUNITNAME` (Tally's marked `Any`), `LASTGSTTAXUNIT` `Rajasthan Registration` and `ALLOWMULTIPLETAXUNITSPERSERIES` `No`. The build reads none of them.
- **`count14.py`:** row 2162-SC's `count13.py` with three changes, each marked RU:
  - T also prints, for each named type, every non-empty leaf whose tag names a tax unit or a registration;
  - `--unit-diff` prints every leaf holding text of a unit found in one answer only, and counts its empty leaves;
  - `--type-diff A B`, on two T answers: the VOUCHERTYPE row counts, each type whose leaves differ with how many, and every differing leaf of the types named by `--type` (default BRIDGE Sales and BRIDGE Sales Keyed). Both diffs stop at 200 lines.
  - Tried on the `pilot-lab-tax-units` fixture and on row 2162-KD's `a-s2-T-types.xml`: on a copy of the unit answer with a second unit added, `--unit-diff` printed the added unit (44 leaves with text, 87 empty) and 0 differing leaves for the other two. On a copy of the type answer with BRIDGE Sales's `TAXUNITNAME` replaced, `--type-diff` printed that one leaf. On each answer against itself, it printed none.
- Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count14.py a-<label>-*`. A STOP from either script stops the row there.

**The alteration rules, for every change in Parts A, B and C.**
- Reach a screen only by Alt+G (Go To) and the words the step names. Press Enter only on a line whose words name what the step names. Write down every line the list offered.
- On the screen (shot), write down every field it shows with its value, as text, before anything is changed.
- Move between fields with Enter or the arrow keys only. Never press Esc on a master or voucher-type screen (C7).
- Change only the field the step names. Choose a value only from the list Tally offers for that field, and only the item that reads exactly as the step says. If no field or no item reads so, press nothing more: screenshot, and end the part as the step says.
- **A date.** If Tally asks for a date from which the change applies, type `1-9-2026` (after the August 2026 invoices and before Tally's current date, 15-Sep-2026, which earlier rows read). Record the field's label and every value it showed first. Any second date, or a date field the step does not name, stops the part.
- Accept with Ctrl+A (shot after). An `Accept ?` prompt is answered Yes. **Any other prompt or message:** copy it word for word, screenshot, press no key, and stop (C7). That message is a result, and the coordinator decides what is pressed next.
- If Tally returns to a list, screenshot its title, then Ctrl+Q to the Gateway (shot). Write down every screen passed.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to the new empty folder `C:\lab\2162-ru\backup-1`. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `U-taxunits.xml T-types.xml R-0010.xml M-marks.xml`.
   - EXPECT U two TAXUNIT rows, each the book's: `Default Tax Unit`, and `Rajasthan Registration` with TAXTYPE `GST`, TAXREGISTRATION and GSTREGNUMBER `08ZZZZZ0000Z1ZQ`, GSTOLDREGNUMBER empty, and one dated row `20260401`, Rajasthan, Rajasthan, Regular, not inactive. Any other: record it and stop before step 3.
   - EXPECT T one row named `BRIDGE Sales`, with the unit leaves row 2162-KD read (above). Record them all.
   - EXPECT R-0010 one row: `BRIDGE Walk-in`, included Yes and every other status flag No.
3. **Part A: the registration's type.** Before it, `sAm`: `M-marks.xml`, then `count14.py --before-write a-s1-M-marks.xml a-sAm-M-marks.xml`. It must pass.
   1. Alt+G, type `GST Registration`; Enter only on a line that alters a GST registration. A list of registrations must hold one line, `Rajasthan Registration`; Enter on it. Anything else: stop.
   2. Find the field whose label names the registration type, reading `Regular`. Choose `Composition` from its list. No such field, or no item reading `Composition`: end Part A with nothing accepted (Ctrl+Q only on the coordinator's word, C7) and record it. Then take 3.5's reads and checks without the restore, and go on to Part B.
   3. Touch no other field. Accept by the rules above.
   4. `sA`: `M-marks.xml U-taxunits.xml T-types.xml R-0010.xml`. Then `count14.py --unit-diff a-s2-U-taxunits.xml a-sA-U-taxunits.xml`, `count14.py --type-diff a-s2-T-types.xml a-sA-T-types.xml` and `count14.py --diff a-s2-R-0010.xml a-sA-R-0010.xml`. Record each whole.
   5. **Restore** BK by C6, with `restore-plan.txt` written first. Then `sAr`: `M-marks.xml U-taxunits.xml T-types.xml`, `count14.py --same-row a-s1-M-marks.xml a-sAr-M-marks.xml` (EXPECT it passes), and `--unit-diff` and `--type-diff` of s2 against sAr (EXPECT 0 leaves differ). Any check failing: send nothing more, and report before the close.
4. **Part B: the registration made inactive.** `sBm` and `--before-write` as at step 3.
   1. As 3.1.
   2. Find the field whose label says whether the registration is active or inactive (its words may be Status, Active or Inactive). Set it to inactive by the item its list offers. No such field: end Part B as 3.2 does.
   3. Touch no other field. Accept by the rules above.
   4. `sB`: as 3.4, with the diffs against s2.
   5. **Restore** and check as 3.5, labelled `sBr`.
5. **Part C: a second registration, then the Sales type bound to one.** `sCm` and `--before-write` as at step 3.
   1. **C2, the second registration.** Alt+G, type `GST Registration`; Enter only on a line that creates a GST registration. If Tally says, or the list shows, that a company setting must be turned on first (for example on F11, or a question about more than one registration), take C1 below first, then return here.
   2. Fill only these, each chosen from Tally's list where it offers one: State `Maharashtra`; registration type `Regular`; GSTIN `27ZZZZZ0000Z1ZQ` (typed, then zoomed and checked before any Enter: 15 characters, capitals); the date from which it applies, `1-4-2026`. A name field, if shown: `Maharashtra Registration`. Every other field stays as Tally fills it: write each down with its value.
   3. Accept by the rules above. `sC2`: `M-marks.xml U-taxunits.xml T-types.xml R-0010.xml`, then the three diffs against s2, recorded whole.
   4. **C3, the type bound to one registration.** Alt+G, type `Voucher Type`; Enter only on a line that alters a voucher type. Choose `BRIDGE Sales`. Find the field whose label names a GST registration or a tax unit (row 2162-KD read `TAXUNITNAME` as `Any`). Choose `Rajasthan Registration` from its list. No such field: record every field the screen showed, end C3 with nothing accepted (C7), and go to 5.6.
   5. Accept by the rules above. `sC3`: `M-marks.xml U-taxunits.xml T-types.xml R-0010.xml`, then `--type-diff a-sC2-T-types.xml a-sC3-T-types.xml` and `--unit-diff a-sC2-U-taxunits.xml a-sC3-U-taxunits.xml`, recorded whole.
   6. **Restore** and check as 3.5, labelled `sCr`.
   - **C1, only if 5.1 sends you here.** Make only the company setting Tally names, by the rules above, and write down its label and both values. Then `sC1`: `M-marks.xml U-taxunits.xml T-types.xml`, and the two diffs against s2, recorded whole, before 5.1 goes on.
6. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the alteration rules and C6 name, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. A stop inside a part leaves the book altered: say so in the report.

## Record
- Each answer's bytes and sha256, and `count14.py`'s lines.
- The `--unit-diff`, `--type-diff`, `--diff`, `--before-write` and `--same-row` outputs, whole.
- Each alteration's screenshots, listed by name, bytes and sha256. As text and word for word: every list offered, every screen's fields before the change, every key pressed, and every prompt or message with the key that answered it.
- The BK file and the three restores, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **Part A:** a second dated row on `Rajasthan Registration`, `REGISTRATIONTYPE` `Composition` from the date entered or from Tally's current date, the `20260401` row unchanged. Row 2162-S9b found that a ledger's party type, changed on screen, added a dated entry from Tally's current date; a company registration may or may not do the same.
- **Part B:** a dated row with `ISINACTIVE` Yes, the same way.
- **Part C:** a third TAXUNIT row, TAXTYPE `GST`, its GUID the book's GUID plus a suffix, with one dated row `20260401`, Maharashtra, Regular. C1 not needed. C3: `TAXUNITNAME` on BRIDGE Sales changes from `Any` to `Rajasthan Registration`.
- R-0010 unchanged by every part; each restore equal to s2.

## Decides
Each U answer the row returns becomes a captured fixture once pushed, and a test runs `parse_company_registration` on it at invoice dates on either side of any new dated row (P1). The build reads the outcome below from those tests, never from this sheet.
- **A or B lands in a dated row the parser reads** (`REGISTRATIONTYPE` other than `Regular`, or `ISINACTIVE` Yes): an invoice dated in that row's span is refused (`invoice_company_registration_not_regular`, `_inactive`), and one dated before it is admitted. No code change; the tests are what is added.
- **A or B lands anywhere else** (a unit-level field, or a field the parser does not read) **and the dated row still reads `Regular` and not inactive:** the build would admit an invoice on a composition or inactive registration. That is a build defect, a switch-on blocker for the coordinator, with the field named.
- **A or B edits the `20260401` row in place:** every invoice on the book is refused from that change on, whatever its date. Record it; the reference then says Tally keeps no history of that field.
- **C2 adds a unit whose GUID begins with the book's:** two units in force refuse every invoice as `invoice_company_registration_ambiguous`. The build then does not serve a company with two registrations. That is a scope limit to state in section 12a.19, not a defect.
- **C2 adds a unit whose GUID does not begin with the book's:** every invoice on such a book is refused as `invoice_company_registration_unbound`, whose wording blames a foreign answer. The refusal then needs its own case and words.
- **C3 shows where the binding lands** (`TAXUNITNAME` or another leaf). The build reads no binding today. A later slice could choose the unit by the type's binding instead of refusing; that needs this field, and its own row.
- **Any part changes R-0010:** a company registration change reaches a posted voucher. Record which fields, and report before the restore.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build reads both answers this row changes.** The invoice judge reads the tax units (`render_company_registration_request`, `agent_import_invoice_wire.rs:694`) and parses them with `parse_company_registration` (`:731`, called at `agent_import_invoice.rs:2338`). It reads the voucher types with `render_voucher_types_request` (`agent_import_invoice_wire.rs:267`).
- **Every case here reaches a refusal written without an answer that held it:** `invoice_company_registration_unbound` (`:751`), `_inactive` (`:802`), `_ambiguous` (`:804`) and `_not_regular` (`:807`). This row supplies the answers.

## Not measured
- A company with GST off, and the company's own GST flag (the build does not read it).
- A changed GSTIN (row 2162-SC).
- Registration types other than `Composition`, and a registration made active again.
- A second registration in the same state, or of another PAN.
- An invoice built or posted while the book is altered: no Bridge build runs.
- GSTR-1 for any of these states.
- Any other book or release.

Row: lab/2162-ru/
