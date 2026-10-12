# W16: the ledger, group and company reads of the HSN/SAC check (Windows lab, 12 Oct 2026)

Book: synthetic BRIDGE PILOT LAB, company number 100033, loaded alone, as W15 left it. TallyPrime 7.1 Silver.
- **Reads only.** No backup was taken.
- Request set: lab/1342-capture-requests @ b426aeb7. W16-README.md sha256 e92e7ceb….
- Request hashes, each checked against workorder/SHA256.txt or the top-level SHA256.txt:
  - w7-ledgers-rates.xml 2e728e00
  - w16-ledger-rates-hsn.xml 03de8fa7
  - r07-groups.xml 20d147b5
  - w16-groups-hsn.xml 237bb6c3
  - w14-company-object.xml 72d8cd92
  - r02-marks.xml 81892a74
- Sends: 8, one at a time, each with curl exit 0. These are the 6 README requests plus 2 extra marks reads, one before step 1 and one after the screens.
- Every answer is a complete ENVELOPE with no LINEERROR. Each is under 100 KB and was answered in under 1 s.
- Not a STOP: the two group answers contain raw `\x03` separators inside `PARENTSTRUCTURE` text. Both end with `</ENVELOPE>`, and they are parsed with those characters replaced (`analyse.py`, `analyse-output.txt`).
- Note: at 13:28Z another company (BRIDGE PARTIAL LAB) was loaded; at 13:37Z BRIDGE PILOT LAB was loaded alone. That change was not made by this run.

| # | Request | Answer | Bytes | sha256 (first 8) |
|---|---|---|---|---|
| (x) | r02-marks.xml | aw16-x-marks-before.xml | 3,394 | 0a2d7f08 |
| 1 | w7-ledgers-rates.xml (control) | aw16-ledgers-rates-0.xml | 73,024 | d9f0c44b |
| 2 | w16-ledger-rates-hsn.xml | aw16-ledgers-rates-hsn-0.xml | 82,392 | fe04e3fb |
| 3 | r07-groups.xml (control) | aw16-groups-0.xml | 55,954 | 8360cacb |
| 4 | w16-groups-hsn.xml | aw16-groups-hsn-0.xml | 59,686 | e9811021 |
| 5 | w14-company-object.xml | aw16-company-object-0.xml | 85,890 | ab9b1f51 |
| 6 | r02-marks.xml | aw16-marks-0.xml | 3,394 | 0a2d7f08 |
| (x) | r02-marks.xml | aw16-x-marks-after-screens.xml | 3,394 | 0a2d7f08 |

## Results against the pre-registered expectations
**(a) Ledgers: met.**
- The new request returns the same 18 ledgers as the control.
- New child tags appear: `HSNDETAILS.LIST`, `GSTAPPLICABLE` (with Tally's control-character prefix, e.g. "¤ Applicable") and `GSTTYPEOFSUPPLY`.
- Every ledger carries an `HSNDETAILS.LIST`. Ledgers with no code (tax, party, cash ledgers) carry an empty row.
- Sales ledgers, as returned:

| Ledger | HSNDETAILS.LIST rows |
|---|---|
| BRIDGE Svc 998313 5% | 20260401 998313, Specify Details Here |
| BRIDGE Svc 998314 5% | 20260401 998314, Specify Details Here |
| BRIDGE Svc 998315 5% | 20260401 "As per Company/Group" (no code); 20260805 998315 |
| BRIDGE Svc 4D 5% | 9983, Specify Details Here |
| BRIDGE Goods 18% | 847130, Specify Details Here |
| BRIDGE Svc INH 5% | 20260401 998344; 20260901 998345 (both Specify Details Here) |
| BRIDGE Svc INH2 5% | 20260401 "As per Company/Group" (no code) |

  - All sales ledgers carry `GSTTYPEOFSUPPLY` Services (Goods for BRIDGE Goods 18%).

**(b) Groups: met.**
- The new request returns the same 29 groups as the control. The only new child tag is `HSNDETAILS.LIST`.
- Sales Accounts: 20260401 998342, Specify Details Here.
- BRIDGE Sales Sub: 20260401 998343, Specify Details Here.
- **Finding:** every group without a code carries one EMPTY `HSNDETAILS.LIST` row. None is missing the list, and none carries a placeholder text.

**(c) Company object: met.**
- `HSNDETAILS.LIST`: 20260401 998341 "Geological and geophysical consulting services", Specify Details Here.
- `CMPGSTDETAILS.LIST`: as in W14 (HSNSOURCEPRIORITY Ledger, IGNOREROUNDINGTYPE "Greater or Lesser than Computed Tax", HSNCODELENGTH 4).
- CMPTAXAMOUNTTOLERENCE 0.

**(d) Marks: met.** Marks before step 1, after step 6 and after the screens are byte-identical: ALTVCHID 60, ALTMSTID 243.

## HSN/SAC codes as Tally's screens show them (look only)
The README forbids opening a master for alteration. In Chart of Accounts, Enter opens alteration, so it was left unused. The codes were read in **Go To > GST Rate Setup**, a report, using its F4 Group filter only; nothing was typed or changed.
- **Company: 998341.** Every group under Primary shows "(998341)" in italic, inherited from the company. This matches the company object read, and W14's Company GST Details screen.
- **Sales Accounts: 998342** (from 1-Apr-26).
- **BRIDGE Sales Sub: 998343** (from 1-Apr-26).
- **BRIDGE Svc INH 5%: 998345.** The screen shows the row applicable from 1-Sep-26, because the report period ends 15-Sep-26. The ledger also has 998344 from 1-Apr-26 in the read.
- **BRIDGE Svc INH2 5%: "(998343)"** in italic, from its group BRIDGE Sales Sub.

The screens agree with the reads at every level.

## Differences from the README
- Two extra marks reads: one before step 1, as the README invites, and one after the screens.
- The screens were the GST Rate Setup report rather than each master's GST screen, because those open only in alteration mode.
- Screenshots stay on the lab PC, listed by hash.
