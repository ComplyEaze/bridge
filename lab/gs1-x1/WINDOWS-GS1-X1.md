# Windows item GS1-X1 v1: what TallyPrime 7.2's "Data for GSTR-9" export fills. (25 Oct 2026.) SYNTHETIC ONLY: THE COMPANY is BRIDGE GST RECON LAB (company number: the `COMPANYNUMBER` of the committed capture `src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-book-extent.utf16le.xml`; GUID ae1490be-52c5-4544-9ffc-4b7da85f9797). LAB-COMMON v3 applies in full, except where "Differences" says otherwise; where the two differ, the stricter holds. Runs BEFORE GS1-L1.

Why: TallyPrime 7.2's help page for GSTR-9 (Release 7.2 and later, last updated 14 Oct 2026) says the export writes the books' data into a copy of GSTN's GSTR-9 Excel utility, but names no table and says nothing about uncertain vouchers or blank tables. This item records, cell by cell, what the export fills and what it leaves blank on one synthetic book. No ComplyEaze Bridge program is used. Nothing is keyed in the book; the only acts are a report screen, an export to a file and two reads.

## Differences from LAB-COMMON
- No backup and no restore: nothing in the book is changed by a step. If the marks move at the export, that is the finding of step 5 (a GST return export has been seen to raise the voucher mark), not a STOP.
- C3: the operator's only requests are MARKS, the file `../gs1-l1/requests/marks.utf16le.xml` (its sha256 in `../gs1-l1/SHA256.txt`), size bound 8,192 bytes.
- C7: on the GST Annual Computation report and the export screens, the only keys pressed are the ones the steps name. Never press Accept, Alt+J (Stat Adjustment), Ctrl+J (exceptions), or anything that resolves, accepts or ignores a voucher. Leave a report with Ctrl+Q after a screenshot of its title.

## Folders
`C:\bridge-lab\gs1x1\` with `tool\`, `out-all\`, `out-hsn\` and `evidence\`, each absent or empty BEFORE (a listing kept), else STOP.

## Steps
0. LAB-COMMON step 0. Also: Tally's version from its About screen (EXPECT Release 7.2; anything else = STOP) and licence Silver, not Educational mode; `x1-cells.py` against `SHA256.txt`; `python -I x1-cells.py --self-test` (EXPECT `self-test: ok`, exit 0; else STOP).
1. MARKS -> `ans-x1-marks-1.xml`. EXPECT one COMPANY row with THE COMPANY's GUID. Record ALTVCHID and ALTMSTID.
2. The utility (the lab's act under the run coordinator's GO): from the GST portal's public Downloads page (no login), download the GSTR-9 Excel utility listed there. Record the page URL, the file name, its size and sha256, and the tool version the page or the file states. Unzip it into `tool\`. Make two copies of its workbook: `template-blank.xlsm` (never opened, never edited, never given to Tally) and `template-for-tally.xlsm`. Do not open either in Excel and do not change any Excel setting. Record both copies' sha256 (EXPECT equal).
3. LOOK ONLY: Gateway > Alt+G (Go To) > GST Annual Computation. F2: period 1-4-2025 to 31-3-2026. If the screen asks for a registration (F3), record the list it offers and choose THE COMPANY's only one; none offered, or a message that GST details are missing = screenshot, STOP ("no registration on the book": a later item seeds one). Screenshot each section the screen shows (Total Vouchers with its included, HSN-incomplete, not relevant and Uncertain counts; Liability; Input Tax Credit; ITC Reversal and Ineligible ITC; HSN/SAC Summary; Outward Supplies on Which Tax is not Payable). Record the counts and totals as printed. Open no voucher.
4. Export, all sections: on that report, Alt+E > Data for GSTR-9 > C (Configure). Set Sections to export: All Sections; template file: `C:\bridge-lab\gs1x1\tool\template-for-tally.xlsm`; output folder: `C:\bridge-lab\gs1x1\out-all\`. Screenshot the configuration, then Ctrl+A, then E, then C as the screen asks. A message naming Excel's "personal information" or file-properties setting = screenshot, STOP (an Excel setting is the owner's act, not this item's). Any other prompt or message the step does not name = C7. Record the output file's name, size, sha256 and time; a listing of `out-all\`.
5. MARKS -> `ans-x1-marks-2.xml`. Record both marks; EXPECT one COMPANY row. A mark different from step 1 is the FINDING "the export moved ALTVCHID/ALTMSTID from a to b": record it, go on.
6. `python -I x1-cells.py tool\template-blank.xlsm out-all\<file> > evidence\cells-all.txt`, then its exit code. EXPECT exit 0 (exit 3 = the export is not a copy of this template: STOP; exit 2 = could not read: STOP, never "nothing filled"). Keep the file whole.
7. Export, HSN only: as step 4 with Sections to export: HSN/SAC Summary, a FRESH copy of `template-blank.xlsm` named `template-for-tally-hsn.xlsm` as the template, output folder `out-hsn\`. Then MARKS -> `ans-x1-marks-3.xml` (as step 5) and `x1-cells.py tool\template-blank.xlsm out-hsn\<file> > evidence\cells-hsn.txt`.
8. LAB-COMMON C10.

## Evidence (hand back)
The utility zip's name, size, sha256 and version; both exported workbooks and both blank copies (synthetic data only); `cells-all.txt`, `cells-hsn.txt` and their exit codes; the three MARKS answers; the step 3 counts and totals as printed; screenshots by name, size and sha256 (never pushed); the notes file. Mark each finding verified, partial or unverified.

## What the result is for
Which GSTR-9 tables and cells Tally 7.2 fills from these books and which it leaves blank (each sheet of the utility is one table or a group of tables), whether uncertain vouchers reach the export, and whether the export moves the book's marks. It is compared afterwards with what `gst_by_counter_ledger` reads, so Bridge claims nothing Tally already does.
