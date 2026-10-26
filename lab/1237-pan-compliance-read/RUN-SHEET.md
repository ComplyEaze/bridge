# #1237: is a party's PAN returned by the compliance listing, and what does a party without one return

Not for merge. THREE LEDGERS ARE CREATED AND ONE ALTERED on Tally's own screen, on one synthetic company the lab already holds, between a backup and a restore; every request is a read. `LAB-COMMON.md` in this folder (v3, a byte copy from `lab/1331-scoped-listing-run-sheet` at 36e2d37e, itself a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15) applies in full, with BRIDGE PILOT LAB as THE COMPANY; where this sheet and it differ, the stricter holds. About 30 minutes.

**WHY.** The books health check (#1237) would list party ledgers that carry no PAN. ComplyEaze Bridge reads a party's PAN only through the compliance listing (`ledger_masters` with `fields=compliance`), whose request asks Tally for `INCOMETAXNUMBER`. Every committed answer of that request omits `INCOMETAXNUMBER` on every ledger (four answers, 48 ledgers, none with a PAN keyed), and no answer has ever carried one. So it is not known whether a keyed PAN comes back, nor whether a party with no PAN answers no element (the same as a field Tally did not send) or an empty one. Until one book answers both, "no PAN" cannot be told from "not returned", and nothing may be built on it (AGENTS.md P1).

**THE REQUESTS** are committed requests, unchanged (check each against `SHA256.txt` before every send, C3):
- `p01_marks.request.utf16le.xml`: the company marks of BRIDGE PILOT LAB (`Bridge Agent Company High Water`), the same file `lab/1331-scoped-listing-run-sheet` sends.
- `p02_compliance_listing.request.utf16le.xml`: the compliance listing of BRIDGE PILOT LAB from 1 April to 15 September 2026, byte for byte the request `lab/1342-w18c-answers` sent (`requests/w18c-ledgers-compliance-to-20260915.xml`). The build's own replay test (`src-tauri/src/agent_import_w18_replay_tests.rs`, request `L`) pins its SHA-256 as one the build sends. Its to-date follows the book's last voucher date; a different last date changes only that date, and the PAN does not depend on it.

**REQUESTS BY CURL (allowed by the owner's decision of 26 Oct 2026 that requests sent by curl may be used to discover shapes; both requests only read).** Both are existing builders' requests, used as instruments; no new builder is made from them: `p01` is the company marks builder (`render_agent_company_high_water` in `src-tauri/src/agent_read_profiles.rs`) and `p02` is the request the compliance listing behind `ledger_masters` with `fields=compliance` sends (pinned as request `L` above). Before the first send: `python -I check-request-ids.py requests` in this folder; EXPECT `census: ok 2, skipped 0, FAIL 0`; anything else = STOP, nothing sent. A marks answer's size bound is 8,192 bytes.

**DIALOGS.** Any dialog or message box on Tally's screen that a step does not name = STOP (C7): press no key, send nothing more, screenshot, copy its text, report.

**OWNER steps** are marked [OWNER].

## Before the sitting
1. [OWNER] Tally licensed. Educational mode = STOP (C1). Record the release and edition from Tally's About screen (the lab PC runs 7.2 since 25 Oct 2026; step 5's expected label was seen on 7.1).
2. Check every file here against `SHA256.txt`.
3. [OWNER] Go-ahead.

## The steps (THE COMPANY = BRIDGE PILOT LAB, synthetic; STOP if absent)
1. Load THE COMPANY alone (C1). C2 step 0. Then `p01` (answer `m1`). EXPECT GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`. Record ALTVCHID and ALTMSTID.
2. Look at THE COMPANY's ledger list on Tally's screen (Chart of Accounts, Ledgers). If any ledger is named `BRIDGE PAN Party A`, `BRIDGE PAN Party B` or `BRIDGE PAN Party C`: STOP.
3. `p02` (answer `c0`, size bound 2,000,000 bytes). This is the book before any change.
4. C5 BACKUP to the new empty folder `C:\bridge-lab\1237-pan\backup`. Record the file, its bytes, its sha256 and the time the Restore list will show.
5. [OWNER] Before this step the lab asks the owner in its own chat whether to key the three ledgers, and keys nothing until the owner says yes. Then create, on Tally's ledger creation screen, a ledger named `BRIDGE PAN Party A` under `Sundry Debtors`. In the field Tally labels for the PAN (on 7.1 expected as `PAN/IT No.`; record the label and the section it sits in exactly), key `ZZZZZ0000Z`. Key nothing else: no GST registration details, no opening balance, no address; leave every other field as Tally shows it. Screenshot the screen before accepting; accept.
6. Create `BRIDGE PAN Party B` under `Sundry Debtors` the same way, with the PAN field left untouched (never keyed). Screenshot before accepting; accept.
7. Create `BRIDGE PAN Party C` under `Sundry Debtors` the same way, with `ZZZZZ0000Z` in the PAN field. Accept. Then alter `BRIDGE PAN Party C`: go to the PAN field, clear it to blank with Backspace (never Escape, C7), screenshot, accept.
8. `p01` (answer `m2`). Record ALTVCHID and ALTMSTID. EXPECT ALTVCHID equal to step 1's; record ALTMSTID whatever it is (an expectation of 4 more than step 1, three creates and one alter, is unverified).
9. `p02` (answer `c1`, size bound 2,000,000 bytes).
10. `p02` again (answer `c2`). Then `p01` (answer `m3`): EXPECT both marks equal to step 8's.
11. If a ComplyEaze Bridge build is connected on the lab PC (the build staged as `w1-2306d86d`, `bridge_mcp.exe` sha256 `92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524`, hashed first; over stdio with `BRIDGE_TALLY_PORT=9001`, a new empty `BRIDGE_AGENT_DATA_DIR`, `BRIDGE_TERMS_ACCEPTED=true`, and posting, import and redaction unset): call `tally_status`, then `ledger_masters` with THE COMPANY's `company_guid`, `fields` = `compliance`, `group` = `Sundry Debtors`. For each of the three ledgers copy, exactly as returned, its row's `compliance.income_tax_number`, `party_gstin`, `party_gstin_status` and `snapshot.master_alter_id`. If no build is connected: skip this step and say so.
12. C6 RESTORE from step 4's backup. Then `p01` (answer `m4`): EXPECT both marks equal to step 1's (the book back as it was found). Then `p02` (answer `c3`): EXPECT byte-equal to `c0` (`fc /b`, or equal SHA-256); if not, the first differing offset and both sizes.
13. C10.

## What the book holds after each step
- Steps 1 to 4: as found.
- Step 5: plus `BRIDGE PAN Party A`. Step 6: plus Party B. Step 7 onwards: plus Party C, its PAN cleared.
- Step 12 onwards: as found (restored).

## Reading the answers (C4, structure only)
For each of `c0`, `c1`, `c2` and `c3`: parse it as XML and list every `LEDGER` row (children of `ENVELOPE/BODY/DATA/COLLECTION` only) with its `NAME`, `PARENT` and, for `INCOMETAXNUMBER`, exactly one of: `absent` (no such child element), `empty` (the element with no text, either form), or its text. Count the rows. Count, across all rows, the `INCOMETAXNUMBER` elements present. A substring count is not accepted. Also list the same three states for `PARTYGSTIN` and `NAMEONPAN` on the three new ledgers.

## EXPECT (recorded; the lab judges only the STOP rules)
- `c0`: no row named `BRIDGE PAN Party A`, `B` or `C`.
- `c1` and `c2`: byte-equal (a difference is itself the finding: report the first differing offset and both sizes); three more rows than `c0`, each under `Sundry Debtors`.
- The `INCOMETAXNUMBER` state of Party A, Party B and Party C in `c1` is the measurement; there is no expected value. Report it as read.
- `c3`: byte-equal to `c0`.

## PASS / FAIL
- PASS: every answer received in full (C3), the marks as EXPECTed at steps 8, 10 and 12, the three ledgers made as the steps say, the book restored, and the tables above reported. The sheet passes whatever `INCOMETAXNUMBER` holds.
- FAIL: a transport failure, a dialog not named here, a mark that moved when it should not have, a restore that did not return the marks, or a count made by substring.

## Close and evidence
- **C10** at the end.
- **C11:** evidence to a new branch `lab/1237-pan-compliance-answers`, after the pre-push scan and the run coordinator's yes, under `lab/1237-pan-compliance-read/answers/`. It holds: every answer, whole, with headers, size, sha256 and its send and answer times; the tables of "Reading the answers"; the step 11 rows as returned (or the skip); the notes; the backup's file name, bytes and sha256 (the backup itself stays on the lab PC). Screenshots stay on the lab PC, listed by name, size and sha256. Commit messages carry no attribution trailer.

## Report, one message
The release and edition; the marks of steps 1, 8, 10 and 12; per answer its size, sha256 and row count; the three ledgers' `INCOMETAXNUMBER`, `PARTYGSTIN` and `NAMEONPAN` states in `c1`; the PAN field's label and section from steps 5 to 7; step 11's rows or the skip; the `c1`/`c2` and `c0`/`c3` comparisons; any STOP. A command that errors is reported as an error, never as "absent" or "unchanged".
