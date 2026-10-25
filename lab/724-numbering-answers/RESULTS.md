# Results: N1 (#724) numbering rows, 26 Oct 2026. PARTIAL: set up and parked before step 4 (the post), which needs the owner watching the approval window

The sheet is `lab/724-numbering-rows` @ aba5eabfdd0d3410a78b39db222a081d4403defe (`SHA256.txt` 3/3 OK). TallyPrime 7.2, Silver, not Educational mode. The build is the staged master build named in the sheet's README: `bridge_mcp.exe` sha256 **92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524** (checked before every call), run over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, port 9001, a new data folder kept for the item, `BRIDGE_TERMS_ACCEPTED=true`, redaction unset, and posting and import both unset (reads only so far). The owner authorised the lab, in the lab's own chat, to key the OWNER steps itself, one act at a time.

## THE COMPANY
`BRIDGE NUMBERADM LAB`, company number 100078, GUID **7f9b1bc0-f300-4265-81ef-4282254174e9**, books from 1-Apr-2025. Select Company showed no company of that name before it was created. The Features screen opened by itself after Create Company with TallyPrime 7.2's defaults (GST Yes, inventory Yes, bill-wise Yes); the sheet names no feature, so it was accepted unchanged.

## Backup (C5): BK1 is a finding; BK1b is the backup
- **BK1, finding.** On Tally's Backup screen (Data > Backup & Restore > Backup), with Specify Path set to a new empty folder, the company list filtered to `NUMBERADM` and THE COMPANY selected (the screen showed that one company), the screen was accepted with **Ctrl+A while the list's next-row highlight sat on another company**. TallyPrime 7.2 then backed up **every company in the data folder** into that folder (79 files, none 0 bytes) and showed no done-message. THE COMPANY's own file was among them (23,411 bytes, sha256 910f4962…). Nothing was loaded, changed or deleted; that folder is kept untouched on the lab PC and is not listed or copied anywhere (its listing is kept by hash only).
- **BK1b, the backup** (chosen by the run's review): a second new empty folder; THE COMPANY selected; the list highlight then moved to "End of List" and Enter pressed; the screen's "Accept?" answered Yes. The folder then held one file: `TDBK1800_<company number>.001`, **23,411 bytes, 01:06 IST, sha256 48edd7e348c59a7415dd786a979d962209f77381f1a07ba7aa82504db8ccf6aa**. MARKS after: ALTMSTID 207.

## Set-up masters
- Ledger `BRIDGE Lab Bank` under Bank Accounts (bank details blank).
- Ledger `BRIDGE Num Customer` under Sundry Debtors (bill-by-bill Yes, as offered; the sheet does not set it).
- Voucher type Receipt, as found: Method of Voucher Numbering **Automatic**, Numbering behaviour on insertion/deletion **Retain Original Voucher No.**, no additional numbering details (so duplicates are not prevented). Already as the sheet requires, so it was left unchanged (left with Ctrl+Q).
- MARKS: ALTMSTID 207 -> 209.

## Row 1, steps done
| Step | What | Result |
|---|---|---|
| 1 | `list_companies` | one company: BRIDGE NUMBERADM LAB, 100078, the GUID above, identity verified |
| 1 | `tally_status` | TallyPrime 7.2, silver, education false, the one company loaded. This build's `tally_status` carries no marks, so **M0 / V0 are taken from the raw MARKS read: ALTMSTID 209, no ALTVCHID** |
| 1 | `masters` `kind=voucher_types` | 24 types; Receipt: series numbering method **automatic**, sub-method **auto_retain**, duplicates **not_prevented** (as the sheet expects) |
| 2 | Receipt keyed on 20-Apr-2025: To BRIDGE Num Customer 1.00 (bill-wise On Account, the default), By BRIDGE Lab Bank 1.00 | the screen numbered it **1**; MARKS ALTVCHID 1 |
| 2 | Receipt keyed on 25-Apr-2025, the same | numbered **2**; MARKS ALTVCHID 2 |
| 3 | `vouchers` 2025-04-01 to 2025-04-30 (the before table) | complete, 2 vouchers: **No. 1, 20250420, MasterID 1**; **No. 2, 20250425, MasterID 2** |

## Not done (parked)
Steps 4 and 5 (the numberless Receipt dated 10 April 2025, its post and `verify_import`, the after table) need the owner to answer the approval window, and steps 6 to 9 (Row 2) follow Row 1 in the sheet's order. They wait for the owner. THE COMPANY was then shut so the next item could start with no company loaded; it is kept as it is (2 vouchers, marks 2 / 209) for the post.

## In this commit
`notes.txt` (UTC times, one line per step; local folder paths replaced by placeholders), `send-log.txt` (raw reads with sha256), `answers/` (company list, every MARKS answer with headers, the tool answers of steps 1 and 3 with their arguments), `requests/` (the company list and the filled MARKS request), `SCREENSHOTS.txt`, `SHA256.txt`. Screenshots and both backup folders stay on the lab PC.
