# 1029-P: `voucher_presence` on BRIDGE CORPUS DENSE through a type the book numbers Manual (results)

Sheet: `lab/1029-p-presence` at 2a645696 (base 187e882b; SHA256.txt 7 of 7 equal by sha256). LAB-COMMON v3 (5370c6d2..., byte copy). THE COMPANY: BRIDGE CORPUS DENSE, GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, its number shown as `<company number>`. TallyPrime 7.2 Silver, licensed, education_mode false; one tally.exe owning port 9001. Build: BUILD-M, zip sha256 1940bd9736edc2aa76d811b3bcd0d6229ee69b5e01f5f59321a46b6ca28f691a (= the digest), `bridge_mcp.exe` 9bda7144f54e5ef68477b5ca30be33a17759de2e1ad756f840785c67bcab2dc0 (= mcpb-smoke.json's) before and after. Client: the 852-C1 inbox stdio client (sha256 0ed0bb03...), writes, batch post and redaction unset, one new empty data folder, one `bridge_mcp` process for steps 3 to 8.

## C1, the question (step 6): every EXPECT met

`voucher_presence` with `calls/c1-presence.json`: 18.646 s, 6,053 B, one page (offset 0, total 3, no `next_offset`).

| Proposal | presence | basis | differences |
|---|---|---|---|
| P1 `LPM-1`, 2025-04-03, 2500.00 | **present** | `manual_voucher_number` | none (`[]`) |
| P2 `LPM-2`, 2025-04-05, 3750.00 | **present** | `manual_voucher_number` | exactly one: `amount`, proposed `3750`, observed `3700` |
| P3 `LPM-3`, 2025-04-06, 99999.00 | **absent** | | |

- `numbering_read`: state `read`, `read_before_window` true, master mark 331. `Lab Manual Sales` was declared manual; the book reads it observed, manual, prevented; `agrees` true.
- `window`: 20250401 to 20250407, read `complete`, reason null. The book side counts 576 window vouchers (the 574 of 1029-T's read plus K1 and K2), with 0 duplicate number groups, 0 unbalanced and 0 unmatched book vouchers.
- Each proposal's party is bound to the catalogue's `Lab Presence Party`, with `voucher_type_observed` true.
- Totals: present 2, possibly_present 0, absent 1, requested 3.

Read against the sheet's table: **row A's presence half is measured live on this book.** Each of the three answers was seen once, through the Manual admission, on about 30,000 vouchers: P1 present with no difference, P2 present with only the amount, P3 absent, the window complete and the type agreeing.

## C0, the control (step 5): EXPECT met

`voucher_presence` with `calls/c0-control.json`: 1.941 s, 3,285 B. It was refused with `presence_numbering_method_differs_from_book`. Its one candidate: `Sales`, declared `manual`, book observed `automatic`, `auto_retain`, `not_prevented`, `agrees` false. The check did not let a type the book numbers automatic through.

## The other steps

| step | what | result |
|---|---|---|
| 0 | m0 | one COMPANY row, this GUID, V 29900, M 328 (= EXPECT); 3,406 B, byte-equal to 1029-T's m2 of 4 Nov. Tally's date Tuesday 31-Mar-2026; PC date 5 Nov 2026; release 7.2 (tally.exe 1.1.7.2), Silver. |
| 1 | BK-P | `TDBK1800_<company number>.001`, 14,661,545 B, sha256 fdfdd659b87f2acadd492c949fc3ae756666a0eeb62a535a2b7fe1fc90fd0328, 5-Nov-26 1:25 pm. m0b byte-equal to m0. |
| 2 | keying | `Lab Manual Sales` (Sales, Manual, Prevent creating duplicate Voucher Nos set Yes from the offered No); `Lab Presence Party` (Sundry Debtors, bill-by-bill No, no opening); K1 and K2 (Accounting Invoice, by F8's list of types). m1: V 29902 (= EXPECT), M 331. |
| 3 | `list_companies`, `tally_status`, `masters` voucher_types | one company, this GUID, books_from 20250401, verified_tuple; release 7.2, licence silver, education_mode false. 25 types on one page. `Lab Manual Sales` observed, manual, prevented; `Sales` observed, automatic, auto_retain, not_prevented (= EXPECT); the other 23 the same as Sales. |
| 4 | m2 | byte-equal to m1 (= EXPECT) |
| 7 | C2, `vouchers` | state complete, total 2 (= EXPECT). K1: 20250403, LPM-1, Lab Presence Party -2500.00 (is_deemed_positive Yes) and DN Sales 2500.00 (No); GUID `d45bc1b0-…-000074cd`, MasterID 29901, AlterID 29901; not cancelled, not optional. K2: 20250405, LPM-2, -3700.00 (Yes) and 3700.00 (No); GUID `…-000074ce`, MasterID 29902, AlterID 29902; not cancelled, not optional. |
| 8 | m3, `egress_log` | m3 byte-equal to m1 (= EXPECT; no mark moved in steps 3 to 8). `egress_log` once, whole (57,491 B). `bridge_mcp` stopped by the client's stop, exit code 0; exe sha256 unchanged. |
| 9 | restore BK-P, m4 | the two expected prompts only. m4: GUID, V 29900 and M 328 equal to m0b's (= EXPECT), the answer byte-equal to m0b's. |
| 10 | C10 | done: one tally.exe owning 9001, `/status` 200, none loaded; Tally's date on the book back at 31-Mar-2026. |

## Step 2's screens, with the run lead's words on the three that stopped

- **The ledger's GST box.** The ledger screen shows Tax Registration Details, with Registration type offered `Regular` and GSTIN/UIN empty. Enter on the empty GSTIN raised `Warning` / `GSTIN/UIN not specified` / `Press any key to continue` (a stop). On the run lead's word one Enter cleared it. Note: **Registration type Regular kept as offered, GSTIN empty.**
- **The Sales Party Details panels.** After the party, each of K1 and K2 opened `Party Details` with a `Dispatch Details` box, all empty, then a buyer `Party Details` panel: Buyer and Mailing Name Lab Presence Party, State Not Applicable, Country India, GST Registration type Regular, GSTIN/UIN empty, Place of Supply Not Applicable. K1's was a stop. On the run lead's word each was accepted with Ctrl+A (Dispatch Details blank, the buyer panel as offered); K2's were pre-approved.
- **Every other field** is recorded in notes.txt as the screen showed it, with a shot before each accept.

Not measured, as the sheet says: a Gold book, a narration-marker identity, other numbering, a party cut short, more than one call, TallyPrime 7.1. The four desktop-client connector servers were left untouched: no call was made to them from this sitting, and their calls were not measured. Nothing ended a process.

## Files

- `agent-run/answers/`: every tool answer whole (000 to 006), with `client-log.txt` (round trips, saved sha256), `launch-env.json` and `step0/initialize.json`. Any that carry the company number or a 10+ digit run stay on the lab PC (`kept-local.txt`).
- `answers/`: the raw marks answers m0, m0b, m1 to m4.
- `evidence/`: `marks.txt`, `send-log.txt`, `bkp.txt`, `restore-plan.txt`, `company-folder.txt`, the C2 records (netstat as an excerpt), the exe hashes and the calls' printed lines.
- `notes.txt`: UTC, one line per step, step 2's keying screen by screen.
- `SCREENSHOTS.txt`, `kept-local.txt`, `substitutions.txt`, `scripts/` and `SHA256.txt`.
