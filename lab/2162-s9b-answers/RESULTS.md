# Row 2162-S9b: Show More offers "Regular - SEZ"; saving it adds a second dated registration entry from Tally's current date, and the first entry stays Regular

Run on 9 Nov 2026, 12:32Z to 12:46Z, on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2162-s9b-sheet` (a761e603), in a sitting of its own on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic). Every value below is M, measured by this run. Tally's current date was Tuesday 15-Sep-2026 throughout.

**Result.** The "GST Registration Types" list's Show More offers 12 types, one of which names a special economic zone: `Regular - SEZ`. It was chosen, nothing else was keyed, and the ledger was saved once with no prompt. Tally did not change the existing registration entry. It added a second dated `LEDGSTREGDETAILS.LIST` entry, `APPLICABLEFROM` 20260915 (Tally's current date; no date was asked for or keyed), with `GSTREGISTRATIONTYPE` `Regular - SEZ`, the same GSTIN and `PLACEOFSUPPLY` Rajasthan. The first entry (20260401, Regular) is unchanged. The build's compliance read (C) carries the new entry whole. In the full ledger read (K) one leaf outside `LEDGSTREGDETAILS.LIST` also changed: `CONTACTDETAILS.LIST[1]/COUNTRYISDCODE`, '' to +91. `ISSEZPARTY` and `GSTNATUREOFSUPPLY` did not change. The book was restored from this sitting's BK and reads as it did at s1 and s2, byte for byte.

## Step 5: the list, word for word
Before Show More ("GST Registration Types", `shots/s5-list-before.png`): Show More / Unknown (marked with a diamond) / Composition / Regular (highlighted) / Unregistered/Consumer.

After Show More (`shots/s5-list-show-more.png`; one page, it did not scroll), top to bottom: Show Less / Unknown (diamond) / Composition / Regular / Unregistered/Consumer / Government entity / TDS / Regular - SEZ / Regular-Deemed Exporter / Regular-Exports (EOU) / e-Commerce Operator / Input Service Distributor / Embassy/UN Body / Non-Resident Taxpayer.
- One entry names a special economic zone: `Regular - SEZ`, with a space on each side of the hyphen.
- Also offered, not chosen: `Regular-Deemed Exporter` and `Regular-Exports (EOU)` (no spaces around their hyphens), and `Embassy/UN Body`.
- While the longer list was open, the Registration type field showed an empty entry box.

## Step 5: the screens
- **Before anything was opened** (`shots/s5-screen-open.png`): Tax Registration Details read PAN/IT No. empty; Registration type Regular; GSTIN/UIN the seed's; Set/Alter additional GST details No.
- **After the choice** (`shots/s5-after-choice-before-save.png`): Name BRIDGE Customer RJ; (alias) empty; Under Sundry Debtors (Current Assets); Maintain balances bill-by-bill No; Mailing Details: Name BRIDGE Customer RJ, Address empty, State Rajasthan, Country India, Pincode empty; Banking Details: Provide bank details No; Tax Registration Details: PAN/IT No. empty, Registration type Regular - SEZ, GSTIN/UIN the seed's (the cursor there, not keyed), Set/Alter additional GST details No; Opening Balance (on 1-Apr-26) empty. No new field, no sub-screen, no date and no prompt.
- **The save:** Ctrl+A. No "Accept ?" and no other prompt. Tally went to the List of Ledgers (`shots/s5-after-save.png`).
- **On reopening** (`shots/s5-reopen.png`), with nothing keyed: Registration type Regular - SEZ, GSTIN/UIN the seed's, Set/Alter additional GST details No; the other fields as after the choice. The screen shows one registration type and no date. Ctrl+Q: no prompt; the List of Ledgers.

## What the save stored
`count7.py --leafdiff` of `s4r` against `s5r` (`evidence/leafdiff-s4r-s5r-*.txt`):
- **K** (443 to 453 leaves, 12 differ):
  - `ALTERID` 247 to 248;
  - `CONTACTDETAILS.LIST[1]/COUNTRYISDCODE` '' to +91 (the contact entry save A created, with this code empty);
  - a new `LEDGSTREGDETAILS.LIST[2]`: `APPLICABLEFROM` 20260915, `GSTREGISTRATIONTYPE` 'Regular - SEZ', `PLACEOFSUPPLY` Rajasthan, `GSTIN` (the same value as entry 1: K holds 2 GSTIN values, 1 distinct), and `CONSIDERPURCHASEFOREXPORT`, `ISCOMMONPARTY`, `ISOTHTERRITORYASSESSEE`, `ISTRANSPORTER`, `STATE` and `TRANSPORTERID` present and empty.
- **C** (17 to 21 leaves, 5 differ): `ALTERID` 247 to 248, and `LEDGSTREGDETAILS.LIST[2]` with `APPLICABLEFROM` 20260915, `GSTIN`, `GSTREGISTRATIONTYPE` 'Regular - SEZ' and `PLACEOFSUPPLY` Rajasthan.
- `LEDGSTREGDETAILS.LIST[1]` (20260401, Regular, Rajasthan, the GSTIN) is the same in both reads before and after.
- M: ALTVCHID 68, ALTMSTID 247 to 248. `s5q` (after the look) is byte-equal to `s5r`'s marks.

## Checks
| check | result |
|---|---|
| Sheet files against `SHA256.txt` (11 files) | PASS: all equal at a761e603. The ten files the sheet names are byte-equal to S9a's at e558c9f8. |
| `gen.py`, re-run in a separate folder | PASS: it rebuilt `K-rj.xml` byte for byte. |
| Step 0 of C2, no company loaded | PASS: Select Company with none loaded; one tally.exe, which alone owns 9001; `GET /status` 200. |
| C1, load alone | PASS: `PILOT` typed at Select Company, one row, Enter, no prompt. The Gateway: BRIDGE PILOT LAB alone, 1-Apr-26 to 31-Mar-27, current date Tuesday 15-Sep-2026, date of last entry 15-Sep-26. |
| Step 1, s1 | PASS: one COMPANY row, the book's GUID, ALTVCHID 68, ALTMSTID 245, byte-equal to S9a's s1. |
| Step 1, the BK (C5) | Taken in this sitting before the first write, to a new empty folder (listed empty before): Backup Data Path C:\ changed to Specify Path and that folder; BRIDGE PILOT LAB alone; "Accept ?" y; no other message. `TDBK1800_<company number>.001`, 90,051 B, sha256 `a1ebf26b6cdcd7cede05f3af645a8455cebe41bf6799f71d02718eb8b8e34e68`, 18:04 local (the Restore list showed 9-Nov-26, 6:04 pm). Marks after it (`s1bk`) equal to s1. |
| Step 2, the book as found | PASS: C 18 LEDGER rows, 0 named `BRIDGE Customer RJ`; K 0 LEDGER rows. Both byte-equal to S9a's s2. |
| Step 3, the seed | PASS: s3m check passed (68/245); `L-seed.xml` sent alone; answer 538 B, root RESPONSE, CREATED 1, every other counter 0, no LINEERROR. The capture after it showed the plain Gateway, no dialog. |
| Step 3, s3r and s3b | PASS: C 19 LEDGER rows, one `BRIDGE Customer RJ` with one `LEDGSTREGDETAILS.LIST` (20260401, Regular, Rajasthan, one GSTIN); K one LEDGER row, the same list; M 68/246. `--leafdiff` s3r against s3b: K 435 and 435 leaves, 0 differ; C 17 and 17, 0 differ. All four answers byte-equal to S9a's s3r and s3b. |
| Step 4, the location (setup) | PASS: s4m check passed (68/246). Country typed India, Enter; Shift+Tab twice to State, typed Rajasthan, Enter; nothing else keyed; Pincode left empty. Ctrl+A, no prompt. s4r M 68/247. `--leafdiff` s3r against s4r: K 435 to 443, 19 differ; C 17 and 17, 2 differ. The leaf paths equal S9a's save A in both K and C (`leafdiff-s3r-s4ar-*.txt` at e84d928e): none in one list and not the other. |
| Step 5, the variable | One save, `Regular - SEZ`, above. s5m check passed (68/247). |
| Step 6, the restore (C6) | PASS: the BK re-hashed equal, one file in its folder; `restore-plan.txt` written; the data folder listed before and after (kept local). Shut (no prompt). Restore from the BK's folder: the list showed BRIDGE PILOT LAB alone, 9-Nov-26 (6:04 pm); "Accept ?" y; "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." Replace (r); no other prompt; Select Company. Loaded alone, no prompt; the two captures showed no dialog and the Gateway with the one company. |
| Step 6, s9 | PASS: M 68/245, `--same-row` against s1 equal, byte-equal to s1; C 18 LEDGER rows, 0 named `BRIDGE Customer RJ`, byte-equal to s2; K 0 LEDGER rows, byte-equal to s2. |
| C10 | PASS: Company > Shut (Alt+K, h), BRIDGE PILOT LAB alone, Enter, no prompt; Select Company with no company loaded; one tally.exe, which alone owns 9001; `GET /status` 200. No XML request after the shut. |

## Decides
Two of the sheet's outcomes hold together.
- **Tally adds a second dated entry.** The zone is a registration type, `Regular - SEZ`, stored as a new `LEDGSTREGDETAILS.LIST` entry whose `APPLICABLEFROM` is Tally's current date at the save, not as a change to the entry in force. The compliance read carries it whole. By the sheet's reading of the build (`read_gst_registration_entry`), an invoice dated before 20260915 reaches entry 1 (Regular), and one dated on or after it reaches entry 2 (`Regular - SEZ`). The refusal's test then uses both entries, with the captured `s5r` C answer (on the lab PC, by size and sha256 in `kept-local.txt`, as it holds the seed's invented GSTIN).
- **A leaf outside `LEDGSTREGDETAILS.LIST` changed too**, in K and not in C: `CONTACTDETAILS.LIST[1]/COUNTRYISDCODE`, '' to +91. It is the dialing code of the contact entry that save A created. This row does not separate whether the type or this save changed it. `ISSEZPARTY` and `GSTNATUREOFSUPPLY` did not change.

Not measured here: the other types the list offers, F12 Configure, More Details, a zone type on an unregistered or composition customer or set by import, the tax Tally applies to an invoice for such a customer, and any other book or release.

## Deviations and notes
- **Where the save left Tally.** Step 5 says to screenshot the Gateway after the save. Both saves (A and the SEZ save) left Tally at the List of Ledgers instead, as in S9a. That screen was captured (`shots/s4-after-save.png`, `shots/s5-after-save.png`). Step 5's second opening of Ledger Alteration went from that list (Enter on `BRIDGE Customer RJ`), as did its first, as in S9a's save B. The list was left with Ctrl+Q to the Gateway before the shut.
- **Labels.** Each label was used once. One label not in the sheet: `s1bk`, the marks read after the BK that C5 asks for.
- **The restore plan's folder line** was first written with a shell `printf` that mangled the backslashes. It was rewritten in Python before the restore; the plan was checked before it was used.

## Files
- `answers/`: the marks answers, the import answer and the K answers that hold no GSTIN, byte for byte. The C answers and the K answers that hold the seed's invented GSTIN stay local, by size and sha256 (`kept-local.txt`).
- `evidence/`: the notes (UTC), each count, check and leafdiff output, and the send times. The process and port listings, the folder listings and `restore-plan.txt` stay local.
- `logs/`: `run.py`'s own logs. `scripts/`: the step 0 helper, the window capture helper and the before-write helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.
