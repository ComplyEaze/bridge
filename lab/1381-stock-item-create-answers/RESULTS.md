# 1381-M: what Tally does with a stock item Create, one variable at a time (results)

Sheet: `lab/1381-stock-item-create` at 5bbd6fb2 (base master 428fba57; SHA256.txt 19 of 19 equal by sha256 and size). LAB-COMMON v3 (5370c6d2..., byte copy). TallyPrime Silver; tally.exe ProductVersion 1.1.7.2; one tally.exe owning port 9001. THE COMPANY: `BRIDGE STOCK CREATE LAB`, created on screen in step 1; its GUID shows here as `<company guid>` and its number as `<company number>`. Row 6 (Gold) is not run, by the owner's decision of 5 Nov 2026. No ComplyEaze Bridge build ran.

## The answers to the sheet's questions

| Row | Counters (every other counter 0, LASTVCHID 0, LASTMID 0) | What the reads show |
|---|---|---|
| R1 `SC Name Unit Only`, NAME and BASEUNITS only | CREATED 1 | One item ADDED; M 210 -> 211. Tally holds PARENT `Primary`, BASEUNITS `Nos`; GSTAPPLICABLE and GSTTYPEOFSUPPLY are absent from the read; GSTDETAILS.LIST and HSNDETAILS.LIST are present and empty. |
| R1b, screen read (no write) | none | The Alter screen shows GST applicability `Applicable`, HSN/SAC Details and GST Rate Details `As per Company/Stock Group` (source Not Available), GST Rate 0 %, Type of Supply `Goods`. M unchanged (211; the marks answer byte-identical to R1's). |
| R2 `SC Copied Goods`, the keyed item's own GST details | CREATED 1 | One item ADDED; M 212. `same` against `SC Sibling Goods`: PARENT, BASEUNITS, GSTAPPLICABLE, GSTTYPEOFSUPPLY, GSTDETAILS.LIST and HSNDETAILS.LIST all **same**. |
| R3 `SC Copied Goods` again (a name that exists) | CREATED 0, **ALTERED 1** | No row added; the item kept its GUID; its ALTERID 212 -> 213 and M 212 -> 213. CHANGED: GSTDETAILS.LIST (CGST 9 / SGST/UTGST 9 / IGST 18 with Cess and State Cess heads, to CGST 2.50 / SGST/UTGST 2.50 / IGST 5 with no Cess heads: replaced by what was sent) and HSNDETAILS.LIST (39239090 to 39269099, 'Other articles of plastics'). |
| R4a `sc name unit only` (case only) | CREATED 0, **ALTERED 1** | No row added (3 items); `SC Name Unit Only` CHANGED: ALTERID 211 -> 214 and its LANGUAGENAME.LIST name became `sc name unit only`; its NAME attribute stayed `SC Name Unit Only` (17 characters). M 214. |
| R4b `SC Name  Unit Only` (spacing only) | CREATED 0, **ALTERED 1** | No row added (3 items); `SC Name Unit Only` CHANGED: ALTERID 214 -> 215 and its LANGUAGENAME.LIST name became `SC Name  Unit Only` (two spaces); NAME attribute unchanged. M 215. |
| R5a `SC Absent Unit Item`, unit `SCX` not in the book | **EXCEPTIONS 1**, LINEERROR `Unit 'SCX' does not exist!` | No item added; no unit `SCX` appeared (units against the BASELINE: unchanged); M unchanged (215). |
| R5b `SC Absent Group Item`, group `SC Absent Group` not in the book | **EXCEPTIONS 1**, LINEERROR `Stock Group 'SC Absent Group' does not exist!` | No item added; no group `SC Absent Group` appeared (groups against the BASELINE: unchanged); M unchanged (215). |

Read against the sheet's decision table:
- **R1, R1b:** the read carries no type of supply, GST applicability or GST/HSN details for a Create with a name and a unit only (the two scalars absent, the two lists empty); Tally's Alter screen shows the defaults it presents for such an item (Applicable, As per Company/Stock Group, Goods).
- **R2:** yes, a Create carrying the keyed item's own GST details is stored as sent, field for field.
- **R3:** yes, a Create on an existing item's name alters it (`ALTERED 1`), replacing the GST and HSN details sent; GUID kept.
- **R4a, R4b:** yes, a name differing only by case, or only by spacing, reaches the existing item (`ALTERED 1`, no new item); it rewrites the item's LANGUAGENAME name to the spelling sent, while the NAME attribute the read shows stays the original.
- **R5a, R5b:** Tally refuses in-band (EXCEPTIONS 1 with a LINEERROR) an item whose unit or stock group the book does not hold, and creates neither the unit nor the group.

## Run record

- **Step 0:** no `BRIDGE STOCK CREATE LAB` existed; `gen_requests.py` rewrote `requests/` byte-equal to SHA256.txt; census `ok 4, skipped 6, FAIL 0` (= EXPECT).
- **Step 1** (screens, in notes.txt):
  - The company: FY and books 1-Apr-2025, Karnataka; the creation screen has no password field.
  - Features opened with Maintain Inventory Yes and GST Yes. GST Details: Karnataka, Regular, GSTIN/UIN blank.
  - Tally's box `Warning` / `GSTIN/UIN is not provided.` / `Press any key to continue` on the blank GSTIN was a C7 stop. On the run lead's word one Enter cleared it, and Tally accepted the blank GSTIN.
  - Unit `Nos`: Simple, formal name Numbers, 0 decimals. The screen also has `Unit Quantity Code (UQC)`, kept at its offered Not Applicable.
  - Group `SC Goods Group` with the offered `As per Company/Stock Group`.
  - Item `SC Sibling Goods`: HSN/SAC `Specify Details Here`, 39239090, picked from Tally's own HSN list. Tally filled the description `Other`; the sheet's description was typed over it and **Tally's screen capitalised words in a typed HSN description**: it is stored as `Plastic Articles for Conveyance` (a stop, and accepted on the run lead's word). GST Rate Details `Specify Details Here`, Taxable; the screen's one rate field is labelled `GST Rate`: 18. No applicable-from date was asked. Type of Supply Goods.
  - Ctrl+Q on the blank form showed no `Quit ?`.
- **Step 2:** marks one COMPANY row, M0 210, V blank. items: exactly `SC Sibling Goods`; GSTAPPLICABLE, GSTTYPEOFSUPPLY and BASEUNITS all arrive. The GST rows arrive as CGST 9, SGST/UTGST 9, IGST 18, Cess Not Applicable, State Cess with no rate, from 20250401. groups: `SC Goods Group` alone; units: `Nos`. Every GUID begins with THE COMPANY's.
- **Step 3:** `make_r2.py` rendered r2 (5,030 B, sha256 48d08a89c7debb59f14b4101eacf5109406f421b1c183e5520fff8cb5674d128); census `ok 4, skipped 7, FAIL 0` (= EXPECT).
- **Step 4:** BK0 `TDBK1800_<company number>.001`, 24,219 B, sha256 b9db73a363be08f5df908c8e517ca20028a007fdfc9b5949f89bf137696b1549. The BASELINE (marks M0′ 210, items, groups, units) was byte-identical to step 2's.
- **Step 5:** each write was sent once, after a MARKS read holding exactly THE COMPANY's row; every capture after a write showed no box.
- **Step 6:** against the BASELINE: items ADDED `SC Name Unit Only` and `SC Copied Goods`; groups and units unchanged; M 215.
- **Step 7** (the run lead's RESTORE was in the GO): BK0 was restored by C6, with the two expected prompts only. Then the COMPANY row's GUID, V and M equal the BASELINE's, and items, groups and units show no row ADDED, REMOVED or CHANGED (= EXPECT). The four answers differ in bytes from the BASELINE's after the fresh load; their rows are equal.
- **Step 8:** C10 done: one tally.exe owning 9001, GET /status 200, none loaded. THE COMPANY (at BK0's state) and BK0 stay on the lab PC. The four desktop-client connector servers were left untouched: no call was made to them from this sitting, and their calls were not measured. Nothing ended a process.

## Files

- `answers/`: every import answer, byte for byte. The reads (marks, items, groups, units) carry THE COMPANY's GUID or number and stay on the lab PC, listed in `kept-local.txt` by bytes and sha256.
- `evidence/chk-*.txt`: every `check_1381.py` output, with the GUID as `<company guid>`. Also `send-log.txt` (every send: request sha256, curl exit, status, CONTENT-LENGTH, answer bytes and sha256), `make_r2.txt`, `bk0.txt`, `restore-plan.txt`, `company-folder.txt` and the C2 records (netstat as an excerpt).
- `requests/r2-copied.utf16le.xml`: the rendered write.
- `notes.txt`: UTC, one line per step.
- `SCREENSHOTS.txt`, `kept-local.txt`, `substitutions.txt` (every pushed text that differs from the lab copy), `scripts/` (the lab helpers) and `SHA256.txt`.
