# Results: #690 R0, a ledger renamed by an import `Alter`, raw capture on TallyPrime 7.2 Silver (BRIDGE LEDGERBOX CI LAB, synthetic), 30 Oct 2026 (UTC). **R0c: an `Alter` naming a ledger the book does not hold is refused in-band and makes nothing (EXCEPTIONS 1, LINEERROR "Group Name not given!"; no master added; no mark or ALTERID moved). A rename counts `ALTERED 1` and nothing else, keeps the GUID, parent and bill-wise flag, steps `ALTMSTID` by one, gives the ledger the new mark as its ALTERID, and leaves `ALTVCHID` unmoved (A1, B1, P1). A one-name list drops an existing alias (E3). A case-only change renames the ledger's own name but leaves the row `NAME` attribute at the old case until Tally restarts (F2, G2). Other renames update the row `NAME` attribute at once. A rename onto another ledger's name is refused (ERRORS 1, "Ledger 'R0 Taken B' already exists!"; nothing changed). An `Alter` carrying the ledger's GUID but addressed to a name no ledger holds does not address it by GUID: it is refused as R0c was. BK0 restored; Z2 equals S1.**

## Rows (`evidence/log.txt` has every request with its times and sha256s; `evidence/<label>.json` every read and answer by structure)
| row | request | counters (absent = not given) | LINEERROR | what the read after it shows |
|---|---|---|---|---|
| C1 / C2 | `r0c-alter-missing` | CREATED 0, ALTERED 0, DELETED 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, **EXCEPTIONS 1**, LASTMID 0, LASTVCHID 0 | "Group Name not given!" | no ledger or group added or gone, no ALTERID moved, ALTMSTID 208 -> 208: no stop |
| N1 / N2 | `setup-create-5` | **CREATED 5**, every other 0 | none | five ledgers added (A ...d0, B ...d1, C ...d2, D ...d3, E ...d4, ALTERIDs 209 to 213), ALTMSTID 208 -> 213 = M0 + 5 |
| A1 / A2 | `r0a-rename` (A -> `R0 Rename A2`) | **ALTERED 1**, every other 0 | none | A: same GUID; own name and row `NAME` both `R0 Rename A2`; one list, LANGUAGEID 1033; ALTERID 209 -> 214; parent and bill-wise unchanged; ALTMSTID 213 -> 214; no other ALTERID moved |
| B1 / B2 | `r0b-rename-back` | **ALTERED 1** | none | A: own name and row `NAME` `R0 Rename A`; ALTERID 214 -> 215; ALTMSTID 215 |
| P1 / P2 | `r0p-plain-rename` (E -> `R0 Plain E2`) | **ALTERED 1** | none | E: own name and row `NAME` `R0 Plain E2`; ALTERID 213 -> 216; ALTMSTID 216 |
| E1 / E2 | `r0e-add-alias` | **ALTERED 1** | none | C: one list `R0 Alias C`, `R0 Alias C Other`; ALTERID 211 -> 217; so E3 ran |
| E3 / E4 | `r0e-rename-aliased` (C -> `R0 Alias C2`) | **ALTERED 1** | none | C: one list `R0 Alias C2` only: **`R0 Alias C Other` is gone**; ALTERID 217 -> 218 |
| F1 / F2 | `r0f-case-only` (D -> `R0 CASE D`) | **ALTERED 1** | none | D: own name `R0 CASE D`, **row `NAME` attribute still `R0 Case D`**; ALTERID 212 -> 219; ALTMSTID 219 |
| G1 / G2 | restart | - | - | after the restart, D's row `NAME` attribute is `R0 CASE D`; A, C and E as at G0; no ALTERID or mark moved |
| D1 / D2 | `r0d-rename-onto-taken` (A -> `R0 Taken B`) | CREATED 0, ALTERED 0, **ERRORS 1**, every other 0 | "Ledger 'R0 Taken B' already exists!" | A and B unchanged (names, GUIDs, ALTERIDs 215 and 210); nothing gone; ALTMSTID 219 |
| H0 / H1 / H2 | the filled R0h request (D's GUID ...d3, addressed to `R0 Not A Ledger`, name `R0 Rename H`) | CREATED 0, ALTERED 0, **EXCEPTIONS 1**, every other 0 | "Group Name not given!" | nothing added; D (by GUID) still `R0 CASE D`, ALTERID 219; ALTMSTID 219 |
| Z1 / Z2 | BK0 restored | - | - | the ledger and group GUIDs and ALTERIDs of S1 exactly; ALTMSTID 208; no `R0` name: PASS |

### What R0 decides (the sheet's table)
| question | answer |
|---|---|
| Does an `Alter` naming a ledger the book does not hold refuse in-band, make nothing and move no mark? | **Yes**: EXCEPTIONS 1 with "Group Name not given!", no master, no mark moved |
| Does a rename count `ALTERED 1` and nothing else, keep the GUID, and leave parent and bill-wise unchanged? | **Yes** (A1, B1, P1) |
| Does `ALTMSTID` step by one, and does the renamed ledger's ALTERID become the new mark? | **Yes**, each time |
| Does `ALTVCHID` stay unmoved? | **Yes** (absent throughout: the book holds no voucher) |
| Does a one-name list drop an existing alias? | **Yes** (E4) |
| What does a change of letter case do to the lists and the row `NAME` attribute? | the list and own name take the new case; the row `NAME` attribute keeps the old one until a restart |
| Does a renamed ledger's row `NAME` attribute keep the old name, and does a restart change it? | for a real rename, **no**: the attribute is new at once (A2, P2, E4); for a case-only change it keeps the old case, and the restart brings it to the new case |
| What does a rename onto a name another ledger holds do? | **refused**: ERRORS 1, "already exists", nothing changed |
| Does an `Alter` carrying a GUID address the ledger by GUID? | **No**: refused as R0c (the name decides) |

## Checks
- **Sheet:** `lab/690-ledger-rename-r0` at edb22bfef24bb6d4ff0309d6d5456e0083c083d7; `SHEET.md` sha256 b31f8549...; `SHA256.txt` 15 of 15 equal; each request's sha256 checked again before its send; `fill_r0h.py` equal to SHA256.txt; the filled R0h file 1,222 B, sha256 da25624a092f89787c131a682c3b3067a567f9e9b6fe18daae232f8f5925e90b.
- **S0:** TallyPrime 7.2 Silver (licensed; About read earlier the same night); tally.exe PID 13252. **BK0:** `TDBK1800_100092.001`, 23,613 B, sha256 d8037fadfc1c68515a486adababcdd75e3ace6c5bc0d799a427dbd8c313cded4, 00:52 UTC (the Restore list showed 30-Oct-26 6:22 am local); equal before the restore.
- **Before every write:** Tally's Gateway with no dialog (looked at on screen). No dialog at any step.
- **G1:** Ctrl+Q from Tally's own screen: Tally closed at once (no Yes prompt appeared), no tally.exe left. Started again (PID 26300); it loaded the demo company on start, which was shut; THE COMPANY loaded alone; no repair, migrate or recovery prompt. The Gateway then showed THE COMPANY with "(e)" and "Data exceptions exist"; after the BK0 restore that mark was gone.
- **Z2:** marks and ledgers answers byte-identical to S1's; the groups answer has other bytes (sha256 af2a2d64... against 7658e77e...) with the same GUIDs and ALTERIDs.
- **C10:** THE COMPANY shut; Select Company with none loaded; one tally.exe (PID 26300) owns 9001; `/status` 200.

## Deviations
1. **Answers parsed by LAB-COMMON C4:** the sheet omitted C4; the parser applied C4 to answers only (dropping only the characters XML forbids, recorded per answer with their code points), on the run lead's yes at about 01:00 UTC. Every request went out byte for byte as the sheet pins it, and every stop rule stood. The first S1 read stopped on raw U+0003 characters in the groups answer (82 of them, with 30 `&#4;` references); after the yes the sheet restarted from a fresh S1. Before that, an earlier S1 attempt stopped on the lab helper's own reader (it took CMPINFO's empty COMPANY element as a row; fixed to the COLLECTION's children). Nothing but reads was sent before the restart.
2. **The Gateway check before every request** was made before every write; the three reads of each read were sent as a run.
3. **The restart** used Ctrl+Q (the sheet's step 1); Tally showed no Yes prompt.
4. The LANGUAGEID of a ledger list reads `0` for ledgers Tally created and `1033` once an `Alter` wrote the list: recorded as read.

**Confidence:** one sitting, one synthetic book, TallyPrime 7.2 Silver: PARTIAL.

**Who acted.** The lab, on the owner's standing word for synthetic books (the writes, the restart and the restore).
