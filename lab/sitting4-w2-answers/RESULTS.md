# Results: W2 (#1037), a hand import through Tally's Import menu, 26 Oct 2026. PASS both runs: each import filled its own consecutive AlterID span in file order; the UI import shows no counters

The sheet is `lab/sitting4-run-sheets` W2, at 955b1f5a43bacc908dace4c96883bb8f48e142be (`SHA256.txt` 6/6 OK). TallyPrime 7.2, Silver, Licensed, not Educational mode. The build is the staged `w1-2306d86d`: `bridge_mcp.exe` sha256 **92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524**, checked before every call. It ran over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, port 9001, a new data folder kept for this row, `BRIDGE_TERMS_ACCEPTED=true`, redaction unset, `BRIDGE_AGENT_ENABLE_IMPORT=true` (as the README says, for `build_import_xml`), and `BRIDGE_AGENT_ENABLE_WRITES` unset. Nothing was posted by the tool. The owner authorised the lab, in the lab's own chat, to key the OWNER steps, one act at a time. For the restore, the owner was asked separately and chose that the lab keys it. From the set-up incident below onwards, every screen act was one key per action with a screenshot after it.

## THE COMPANY
`BRIDGE HANDIMPORT LAB`, company number 100081, GUID **9b5b6d54-4cdb-4b90-9600-cab45ee70c3a**, books from 1-Apr-2025. Select Company showed no company of that name before it was created. Features were accepted as offered (TallyPrime 7.2 defaults).

Masters:
- `Lab Bank` under Bank Accounts, with bank details blank.
- `BRIDGE Party A` and `BRIDGE Party B` under Sundry Debtors, with **bill-by-bill set to No**. This is a choice the run's review accepted: `build_import_xml` refuses a bill-wise party unless the person approves On Account entries, and this row does not measure bill-wise behaviour.
- The Receipt voucher type was left as found: Automatic, Retain Original Voucher No., no additional numbering details.

Day D is **15-May-2025**. The party names and day D were accepted by the run's review.

## Set-up incident (lab error), recorded
While leaving the Receipt voucher type list, the lab sent two Ctrl+Q in one batch. The first went straight back to the Gateway, and the second **quit TallyPrime**. No prompt was seen. PID 18504 stopped listening on 9001 at once and exited at 04:19:02Z.

At that point:
- the company and all its masters had been accepted;
- no W2 read had been sent to Tally;
- BK1 had not been taken.

TallyPrime was restarted by the lab (one instance, PID 29692). It loaded the demo company by itself, and that company was shut. C2 was run again, THE COMPANY was loaded alone, and Chart of Accounts showed exactly the set-up: 28 groups and 5 ledgers.

## Backups (C5)
- **BK1** (before any voucher): `TDBK1800_<company number>.001`, 24,041 bytes, sha256 **cdb8f6f30442733e90a9f384130745bcd4e55b6d8a28775696486b55510623b6**. Kept untouched.
- **BK1b** (after the seed, the restore point for step 5): one file, 26,215 bytes, sha256 **425c120b597b7f61f4226244d92553cf4a74d14c0795f76feb673e5913934445**.

Both were taken with the safe sequence: Specify Path to a new empty folder, select the one company, move to End of List, Enter, answer Accept? Yes.

## Step 2 first attempt: refused by design, then a seed voucher
On the empty book, `build_import_xml` was **refused with `empty_book_first_import`**: "This company has never held a voucher, so Tally reports no voucher high-water mark…". This is the designed rule. No file was written.

The run's review ruled that one seed voucher is keyed by hand on another day. It was keyed as Receipt No. 1 on 1-May-2025: To BRIDGE Party B Cr 1.00, By Lab Bank Dr 1.00, narration `SEED`. Lab Bank's Bank Allocations sub-form was accepted as offered (Cheque/DD, no instrument number). Raw MARKS after the seed: ALTMSTID 210, ALTVCHID 1.

## The two runs
Marks come from `ledger_masters`' first-page snapshot (`reused` false every time). Each is shown beside the raw LAB-COMMON MARKS read, and all agree.

| | Run 1 | Run 2 (after RESTORE of BK1b) |
|---|---|---|
| M0 / V0 (snapshot; raw) | 210 / 1; 210 / 1 | 210 / 1; 210 / 1 (after the restore) |
| N0: `vouchers` on day D | 0 items, **state partial, reason `empty_uncorroborated`, no snapshot** | not read again (the restore returned the book to BK1b) |
| Batch | `bridge-12c87c35-f7fe-4571-9cfc-4b43852a0844` | `bridge-796edde5-3999-4a12-9032-334510a85f48` |
| Import file (2,419 B each) | sha256 462e6f8202fce76f271c013b87bc582db34a0ddcde9015feb9a7b257c3334ff2 | sha256 ee3bcb13209a93c609b5a0782f16e1864422f7cc1b79cf8aac6360637d515bed |
| Order in the file (read from the file) | S1, S2, S3 | S3, S2, S1 |
| Raw MARKS just before the import | 210 / 1 | 210 / 1 |
| Import screen counters | **none**: "Import completed successfully." / "Press any key to continue" | **none**, the same text |
| M1 / V1 (snapshot; raw) | 210 / 4; 210 / 4 | 210 / 4; 210 / 4 |
| AlterIDs (MasterID, number and GUID suffix the same) | S1 = 2, S2 = 3, S3 = 4 | S3 = 2, S2 = 3, S1 = 4 |

In both runs:
- V1 = V0 + 3;
- the three AlterIDs are consecutive (2..4), start above V0's mark and **increase in file order**;
- no other voucher sits inside the span;
- the master mark did not move.

**PASS** for both runs. Comparing the spans: the same span 2..4 each time, assigned by position in the file, not by narration. Each narration in the file and in the book ends in the shipped `[BRIDGE:<remoteid>]` tag.

## How the import was keyed on TallyPrime 7.2 (exact prompts and options)
- Alt+O opens Import: Masters, Transactions, Exceptions, Bank Details, Bank Statement, GST Returns, Manage, Configuration. 7.2 has **no "Import Data" or "Vouchers" item**, so Transactions was used.
- The form "Import Transactions" has these fields: File Format (XML (Data Interchange)), File Path, File to Import, Backup Company Data before Import (No), Backup Data Path. It has **no behaviour option**: no "Add as new", combine or replace.
- The form **remembered** the previous import's folder and file. Before run 1 they belonged to an earlier, unrelated item; before run 2 they were run 1's file. Both were changed and read on screen before the last Enter.
- Enter on the last field imports at once, with no Accept prompt.

## RESTORE of BK1b (between the runs)
Run's review approved; the owner chose that the lab keys it. The steps:
1. Company > Shut.
2. Data > Backup & Restore > Restore. The Restore Data Path read as the live data folder.
3. The backup list was pointed with Specify Path at BK1b. It held one company, 100081, 26-Oct-26 10:06 am.
4. End of List, Enter, Accept? Yes.
5. Tally asked: "Company data folder (100081) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." Replace was chosen.
6. The company was reloaded alone. Marks: 210 / 1.

## Findings and observations for the read tools
1. On the empty company, `vouchers` gave `snapshot.voucher_alter_id` **0**, while `ledger_masters` gave **null**.
2. On the one-voucher company with nothing on day D, `vouchers` answered **partial / `empty_uncorroborated`** with no snapshot.
3. Each `vouchers` item's `party` field reads **"Lab Bank"**, not the party ledger.

## AFTER and C10
The run's review chose **no closing restore**: the owner's word covered the BK1b restore only, and W3 and W1 use their own new companies. End marks just before C10: `ledger_masters` snapshot **210 / 4** (`reused` false), and raw MARKS 210 / 4 with LastVoucherDate 20250515. The book holds the seed and run 2's three Receipts.

Both backups are kept on the lab PC:
- BK1: `<item folder>\BK1`, sha256 cdb8f6f30442733e90a9f384130745bcd4e55b6d8a28775696486b55510623b6.
- BK1b: `<item folder>\BK1b`, sha256 425c120b597b7f61f4226244d92553cf4a74d14c0795f76feb673e5913934445.

C10:
1. Company > Shut (Alt+K, h, Enter, one key each).
2. Select Company showed no company loaded.
3. tasklist: one tally.exe, PID 29692 (exit 0).
4. netstat: 9001 LISTENING with that PID, on IPv4 and IPv6.
5. GET /status: 200, 51 bytes.

No XML request was sent after the shut.

## In this commit
- `notes.txt`: UTC times, one line per step, including the incident.
- `send-log.txt`: every raw read, with request sha256, bytes and answer sha256.
- `answers/`: every tool answer with its arguments, every MARKS and company-list answer with headers, and the C2 observations.
- `requests/`: the company list request, and the MARKS request filled from c1-marks-TEMPLATE (sha256 4bcdcfd4…; only `@@COMPANY@@` and `@@GUID@@` replaced).
- `imports/`: the two import files exactly as built.
- `SCREENSHOTS.txt` and `SHA256.txt`.

Local folder paths are replaced by placeholders (`<item folder>`, `<lab folder>`, `<Tally data folder>`, `<an earlier item's import folder>`) in three files (and the demo company's name in `notes.txt`). As saved on the lab PC, their sha256 are:
- `notes.txt`: 0340fba37478ba03de68c38547699d3a25e9092805d87266b8f8851e28f89fb5
- `answers/s2b-build_import_xml.json`: 127f09449dcb8cea200d8a2e6242df3085103bed2fbd4311aefb4b3b677c5cb5
- `answers/s5-build_import_xml.json`: 02453e0914a61ccfb7845d725a8abff8dd123a124794f0ae3a6cacae2c71a41e

Every other file is byte for byte as saved, except that text files are stored with LF line ends. The Tally answers are UTF-16 and unchanged, and match `send-log.txt`.

Not committed: the screenshots, BK1, BK1b and the client's data folder. All stay on the lab PC.
