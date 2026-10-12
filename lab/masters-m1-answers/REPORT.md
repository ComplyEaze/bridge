# Masters M1 lab sitting, phases 0-2 (12 Oct 2026)

Run sheet: lab/masters-m1-requests @ dd5135804c40f0f809dd804ec129543edf153a04 (README.md sha256 c87cd300f3539525198af9ef92cc44a5590f978964b5ee7db59c06eec587c7e7). The 21 request files were checked against SHA256.txt (21/21 OK). Only the four phase-2 reads were sent. No writes; phases 3-4 were not run.

## Phase 0
- TallyPrime 7.1, Silver, Licensed, not Education mode.
- Tally's own Connectivity settings: acts as Both, ODBC Yes, port 9001.
- Loaded at the check: one company (since shut, see phase 1).

## Phase 1 (keyed on screen)
- **Company:**
  - BRIDGE MASTERS LAB, number 100060.
  - Rajasthan, books from 1-Apr-2026, no inventory.
  - GST Regular, Monthly, with the run sheet's company GSTIN (shown here as the same-length mask ZZZZZZZZZZZZZZZ).
  - The only company loaded.
- **Ledgers.** All six are under Sundry Debtors, bill-by-bill No, no opening balance:
  - MLab Exact Target. Regular, with the run sheet's party GSTIN (mask ZZZZZZZZZZZZZZZ).
  - MLab Alias Target. Aliases MLab Alias Name and MLab Alias Two.
  - MLab Case Target.
  - MLab Space Target.
  - MLab Dash-Target.
  - MLab Slash/Target.
  - Tally pre-filled Registration type Regular on each party form. It was left that way, with GSTIN blank, on the five ledgers other than MLab Exact Target.
- **Group:** MLab Group Target under Sundry Debtors, alias MLab Group Alias.
- **Backup B0:**
  - File TDBK1800_100060.001, 24735 bytes, written 2026-10-12T14:16:25Z.
  - sha256 9ca37bd80d695f2fd93b8055a59566bfdf1f2b4a3b581a5b16c3092eb792994d.
  - Stays on the lab PC.
- **Screenshots:** the alias screens and the others stay on the lab PC. They are listed by hash in SHA256-screenshots.txt.

## Phase 2 (step id `nr`, sent in order, one at a time)
| Request | curl exit | HTTP | bytes | Rows under ENVELOPE/BODY/DATA/COLLECTION |
|---|---|---|---|---|
| R-MARKS | 0 | 200 | 3308 | COMPANY 1 |
| R-V2 | 0 | 200 | 12910 | LEDGER 8 |
| R-V3 | 0 | 200 | 13610 | LEDGER 8 |
| R-GRP | 0 | 200 | 56068 | GROUP 29 |

All four answers have Content-Type `text/xml; charset=utf-16` and were decoded by that charset.

- **R-MARKS:**
  - One COMPANY row: BRIDGE MASTERS LAB, GUID f107e523-3d6b-4516-bd24-5b1a14498fa6.
  - ALTMSTID 216.
  - **ALTVCHID is absent from the row,** although the request FETCHes it. The company has 0 vouchers, and CMPINFO VOUCHER is 0.
- **MLab Alias Target, NAME list in document order:**
  - R-V2: MLab Alias Target, MLab Alias Name, MLab Alias Two.
  - R-V3: MLab Alias Target, MLab Alias Name, MLab Alias Two.
- **R-V3 ALTERID:** yes, on 8 of 8 LEDGER rows.
  - Cash 32, Profit & Loss A/c 31.
  - MLab Exact Target 210, MLab Alias Target 211, MLab Case Target 212, MLab Space Target 213, MLab Dash-Target 214, MLab Slash/Target 215.
  - R-V2's rows carry no ALTERID.
- **MLab Group Target in R-GRP:**
  - NAME.LIST: MLab Group Target, MLab Group Alias.
  - GUID f107e523-3d6b-4516-bd24-5b1a14498fa6-000000d4.
  - ALTERID 216.
- **Parsing note:**
  - R-V2 and R-V3 each carry one `&#4;` character reference, in Profit & Loss A/c's PARENT ("&#4; Primary").
  - R-GRP carries raw 0x03 characters and 30 `&#4;` references.
  - A strict XML parser refuses these. The answers are published byte-for-byte as received.
  - phase2-parse.py replaces those characters with "?" in memory before parsing. phase2-parse.txt is its output.
- Tally's window showed no dialog after the reads (p2-tally-after-reads.jpg, by hash).

No STOP condition was met: one COMPANY row, ALTERID on every R-V3 ledger row, no transport failure, no dialog.

## Files
- nr-*.response and nr-*.headers: the answers and headers exactly as saved. SHA256-evidence.txt lists them.
- log.txt: the sitting log.
- send-read.sh: the send helper (it checks the hash, then runs the run sheet's curl line).
- phase2-parse.py and phase2-parse.txt: the element parse.
- SHA256-screenshots.txt and SHA256-backup.txt: hashes only.
- Phase 3: n-*.response and n-*.headers (writes n-<case>, reads n-<case>-after, R-PARTY n-0b-party-before/-after), phase3-parse.py and phase3-parse.txt. send-read.sh is the same helper with a log label.

## Phase 3: L-N (after GO; 8 writes, 26 reads, 14:22:39Z-14:27:04Z)
tally_status at 14:22:39Z showed BRIDGE MASTERS LAB (100060) loaded alone, Silver, not Education. All 21 request hashes were re-checked. Each request was sent once, in the run sheet's order. Every send returned curl exit 0 and HTTP 200. There was no dialog in Tally after any write; a screenshot after each write stays on the PC, by hash.

Every Create uses parent Sundry Creditors and bill-wise Yes. Each target was Sundry Debtors and bill-wise No before its case (nr-R-V3).

| Case | Create name | Counters | ALTMSTID | R-V3 after: row hit |
|---|---|---|---|---|
| N-0a | MLab Fresh Party | CREATED 1 (control OK) | 216 → 217 | new row: GUID …-000000d5, ALTERID 217, Sundry Creditors, Yes, names [MLab Fresh Party] |
| N-0b | MLab Exact Target | ALTERED 1 (control OK) | 217 → 218 | same GUID …-000000ce, ALTERID 210 → 218, now Sundry Creditors, Yes, names [MLab Exact Target] |
| N-a | MLab Alias Name | ERRORS 1, LINEERROR "Duplicate Entry!", all others 0 | 218 → 218 | target unchanged: GUID …-000000cf, ALTERID 211, Sundry Debtors, No, names [MLab Alias Target, MLab Alias Name, MLab Alias Two] |
| N-b | MLAB CASE TARGET | ALTERED 1 | 218 → 219 | same GUID …-000000d0, ALTERID 219, Sundry Creditors, Yes, NAME.LIST now [MLAB CASE TARGET]; the row's NAME attribute still says "MLab Case Target" |
| N-c | MLab  Space Target (two spaces) | ALTERED 1 | 219 → 220 | same GUID …-000000d1, ALTERID 220, Sundry Creditors, Yes, NAME.LIST now [MLab  Space Target] (two spaces); attribute "MLab Space Target" |
| N-d | MLab Dash Target | ALTERED 1 | 220 → 221 | same GUID …-000000d2, ALTERID 221, Sundry Creditors, Yes, NAME.LIST now [MLab Dash Target]; attribute "MLab Dash-Target" |
| N-f | MLab Slash Target | ALTERED 1 | 221 → 222 | same GUID …-000000d3, ALTERID 222, Sundry Creditors, Yes, NAME.LIST now [MLab Slash Target]; attribute "MLab Slash/Target" |
| N-e | MLab Group Target (ledger) | CREATED 1 | 222 → 223 | new ledger row: GUID …-000000d6, ALTERID 223, Sundry Creditors, Yes, names [MLab Group Target] |

All GUIDs share the company prefix f107e523-3d6b-4516-bd24-5b1a14498fa6. Every count is by XML element, and every ALTMSTID comes from the single COMPANY row of R-MARKS (1 row each time). ALTVCHID was absent every time (0 vouchers).

- **N-0b and R-PARTY (does a Create without a GSTIN clear it?): No.**
  - Before: PARENT Sundry Debtors, ALTERID 210, TAXTYPE Others. One LEDGSTREGDETAILS.LIST (APPLICABLEFROM 20260401, GSTREGISTRATIONTYPE Regular, PLACEOFSUPPLY Rajasthan, GSTIN = the run sheet's party GSTIN).
  - After: PARENT Sundry Creditors, ALTERID 218. The same LEDGSTREGDETAILS.LIST, unchanged, with the GSTIN still present.
  - PARTYGSTIN is absent from the row both times, although FETCHed. The GSTIN is only in LEDGSTREGDETAILS.LIST.
  - MASTERID is 206 both times.
- **N-e, group row in R-GRP:**
  - Before (n-f-after) and after (n-e-after) are identical: GUID …-000000d4, ALTERID 216, PARENT Sundry Debtors, names [MLab Group Target, MLab Group Alias].
  - The group was not touched. Tally created a ledger with the group's name beside it, with no dialog.
- **Summary of the folds:**
  - Tally matched each tested variant to the existing ledger. The variants were: all capitals; a doubled space for a single one; a space where the target has "-"; a space where the target has "/". The Create then overwrote that ledger (ALTERED=1, same GUID): parent and bill-wise changed, and the NAME.LIST took the incoming spelling. The NAME attribute on the row kept the old spelling. Other separators and the reverse directions were not tested.
  - A name equal to another ledger's alias is refused whole: ERRORS=1, "Duplicate Entry!", no change, no ALTMSTID step.
  - A ledger name equal to a group's name is not a conflict: it creates.
- **CMPINFO's `<LEDGER>` count element** (not a row count) went 6, 7, 8, 8, 9, 10, 11, 12, 13 across R-MARKS nr…n-e-after. It steps on every created or altered ledger, not on the refused write. COLLECTION rows went 8 → 9 (N-0a) → 10 (N-e).
- phase3-parse.py and phase3-parse.txt hold the per-case element parse. Phase 4 was not run.

## Phase 4: L-M1 (after GO; 8 writes, 26 reads, 14:52:39Z-14:55:26Z)
- **Before the run:**
  - Tally was restarted twice between phases 3 and 4, during another item on another company. After the restart Tally showed "Application settings are lost". Connectivity was unchanged: Both, ODBC Yes, port 9001.
  - BRIDGE MASTERS LAB was reloaded alone with no prompt.
  - tally_status at 14:52:39Z showed it ALONE. 21/21 hashes were OK.
- **The run:**
  - Each request was sent once in the run sheet's order. Every send returned curl exit 0 and HTTP 200.
  - No dialog appeared after any write. A screenshot after each write stays on the PC, by hash.
  - R-MARKS returned 1 COMPANY row every time.

| Write | Counters | ALTMSTID | New row in R-V3 after (GUID, PARENT, ISBILLWISEON, names) |
|---|---|---|---|
| M1-PARTY | CREATED 1, others 0 | 223 → 224 | …-000000d7, Sundry Debtors, Yes, [MLab M1 Party] |
| M1-SUSP | CREATED 1 | 224 → 225 | …-000000d8, Suspense A/c, No, [MLab M1 Suspense] |
| M1-DUTY-CGST | CREATED 1 | 225 → 226 | …-000000d9, Duties & Taxes, No, [MLab M1 Duty CGST] |
| M1-DUTY-IGST | CREATED 1 | 226 → 227 | …-000000da, Duties & Taxes, No, [MLab M1 Duty IGST] |
| M1-DUTY-STATE | CREATED 1 | 227 → 228 | …-000000db, Duties & Taxes, No, [MLab M1 Duty State Tax] |
| M1-DUTY-SGSTUTGST | CREATED 1 | 228 → 229 | …-000000dc, Duties & Taxes, No, [MLab M1 Duty SGST UTGST] |
| M1-DUTY-UT | CREATED 1 | 229 → 230 | …-000000dd, Duties & Taxes, No, [MLab M1 Duty UT Tax] |
| M1-DUTY-CESS | CREATED 1 | 230 → 231 | …-000000de, Duties & Taxes, No, [MLab M1 Duty Cess] |

**R-PARTY at m1-end** (each field as returned; R-PARTY at m1-before had no MLab M1 rows):

| Ledger | TAXTYPE | GSTDUTYHEAD | PARTYGSTIN | OPENINGBALANCE |
|---|---|---|---|---|
| MLab M1 Party | Others | absent | **present**: the request's party GSTIN, as sent | 0.00 |
| MLab M1 Suspense | Others | absent | absent | 0.00 |
| MLab M1 Duty CGST | GST | CGST | absent | 0.00 |
| MLab M1 Duty IGST | GST | IGST | absent | 0.00 |
| MLab M1 Duty State Tax | GST | State Tax | absent | 0.00 |
| MLab M1 Duty SGST UTGST | GST | SGST/UTGST | absent | 0.00 |
| MLab M1 Duty UT Tax | GST | UT Tax | absent | 0.00 |
| MLab M1 Duty Cess | GST | Cess | absent | 0.00 |

- **All six duty heads were stored exactly as sent.** No head was dropped.
- **MLab M1 Party, LEDGSTREGDETAILS.LIST rows:** one element, empty, with no child elements: no APPLICABLEFROM, GSTREGISTRATIONTYPE, PLACEOFSUPPLY or GSTIN.
  - So a gateway Create carrying PARTYGSTIN stores the GSTIN in the top-level PARTYGSTIN field and builds no registration row.
  - This is the reverse of the screen-keyed MLab Exact Target. There the GSTIN is only inside LEDGSTREGDETAILS.LIST, and PARTYGSTIN is absent (phase 3).
- **LANGUAGEID** is 0 on every ledger created through the gateway (phases 3 and 4). It is 1033 on the ledgers keyed on screen.
- **NAME attributes after the restart:** since Tally's restart, R-V3 and R-PARTY carry the phase-3 spellings in the NAME attribute too (for example "MLAB CASE TARGET", "MLab  Space Target"). So the stale attribute seen within the phase-3 session was session state, not stored data.
- R-V3 rows went 10 → 18. phase4-parse.txt holds the per-write parse.

## Masking
The run sheet's synthetic GSTINs (company, party and M1 party) are written as the same-length mask ZZZZZZZZZZZZZZZ.
- **Text:** the mask is used in log.txt, the phase parse files and this report. The published log.txt also drops one scheduling note from its first line.
- **XML answers:** the mask is used inside the R-PARTY answers (n-0b-party-before/-after, m1-before, m1-end). It keeps their byte length and adds no markup; the answers keep Tally's own &#4; references (see the parsing note). No other answer contains a GSTIN, and they are published byte for byte.
- **Hashes:** SHA256-evidence.txt lists the lab hash and the published hash of every file.
