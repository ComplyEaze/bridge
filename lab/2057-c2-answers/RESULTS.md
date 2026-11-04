# Row 2057-C2: Data > Configuration, and F12 inside a Purchase voucher, look only: results

**First line: every EXPECT met; no stop, no prompt, nothing written (marks 213 / null at every step). One label names an allocation per item: "Select common Ledger Account for Item Allocation", value Yes, on the Purchase voucher screen's own F12 "Voucher Configuration" (Item Invoice mode, nothing keyed). Data > Configuration holds none. Rule 4's quit prompt did not appear: Ctrl+Q left the blank voucher straight to the Gateway.** (verified)

Sheet `lab/2057-c2-sheet` at 8715d5df (folder `lab/2057-c2`; SHA256.txt 3 of 3 equal, by sha256 and bytes). LAB-COMMON v3. Every step and stamp is in `notes.txt` (UTC); each screen, in its own words, is in `transcriptions/`.

## The labels (steps 3 and 5)

| step | screen | label, in the screen's own words | value as shown |
|---|---|---|---|
| 3 | Data > Configuration ("Data Configuration", List of Configurations) | none | – |
| 5 | Purchase voucher screen > F12 ("Voucher Configuration"), Item Invoice mode | Select common Ledger Account for Item Allocation | Yes |

- One label, on the screens of steps 2 and 4, names an allocation per item. Its value is shown as Yes.
- This row records the label and its value only. Which value, if any, a later row sets is for a decision and a sheet of their own (the sheet's "What the row decides", first row).
- "Show more configurations : No" on that screen says more options exist that it does not show. It was not changed, so they were not seen.
- On the Data menu: COMPANY DATA (Backup & Restore, Split), TROUBLESHOOTING (RepAir, Migrate, All Exceptions, greyed), CONFIGURE (Configuration). On Data Configuration: three path fields (not recorded) and "Allow working on data during migration : No"; a "Show More" button, not opened (its label names no voucher, invoice or accounting entry). (verified)

## Every EXPECT line

| step | EXPECT | result |
|---|---|---|
| 0 | Current Date 1-Apr-2025, "No Vouchers Entered", as at 2057-C's step 0 and 15; one tally.exe owning 9001 | PASS: Tuesday, 1-Apr-2025, "No Vouchers Entered"; the capture is byte-equal to 2057-C's step 0. tally.exe 1200 owns 9001; GET /status 200. TallyPrime 7.2 Silver. (verified) |
| build | `bridge_mcp.exe` = 2057-C's, before the first call and after the last | PASS: c5b75d6480202ef70a5216f74d369ea0963b4a2eedbb3d4912e4f25b385e943d both times. Client c127118d…. (verified) |
| 1 | `ledger_masters` and `list_companies` in `tools/list` | PASS: 32 tools, both present. (verified) |
| 1 | `list_companies` exactly THE COMPANY, `verified_tuple` | PASS: BRIDGE CLASS PURCHASE LAB, 92a7ef97-dd28-4286-b02c-c059fcb8690e, verified_tuple. (verified) |
| 1 | marks M0/V0 = 213 / null, reused false | PASS. (verified) |
| 2 | marks equal to M0/V0 | PASS: 213 / null. (verified) |
| 4 | marks equal to M0/V0 (no voucher saved) | PASS: 213 / null. (verified) |
| 6 | the Gateway "No Vouchers Entered", compared with step 0's | PASS: "No Vouchers Entered", 1-Apr-2025. Its capture differs from step 0's only in the menu highlight (on Vouchers after step 4, not Create). (verified) |
| 6 | `list_companies` as in step 1; marks M0/V0 | PASS: the same company and GUID, verified_tuple; 213 / null. (verified) |
| 6 | exe hashed; Company > Shut; none loaded; one tally.exe; 9001 LISTENING; /status 200 | PASS: Company > Shut (Alt+K, Down ×4 to "SHut", Enter, Enter on the only row); Select Company, none loaded; tally.exe 1200 owns 9001; GET /status 200. No XML request after the shut. (verified) |

## How step 4 went

- Gateway > Vouchers opened directly on Purchase No. 1 (Tally's last-used type; "F9: Purchase" shown greyed), in Item Invoice mode, the cursor on Supplier Invoice No. F9, pressed as the sheet says, changed nothing. No Ctrl+H was needed. (verified)
- F12 opened "Voucher Configuration" with the whole list in view. No key was pressed on any option. (verified)
- One Ctrl+Q went back to the voucher screen with no prompt. A second Ctrl+Q went straight to the Gateway with no prompt: rule 4's quit prompt did not appear, and no other prompt did. (verified)

## Kept on the lab PC

No captured byte is altered. A captured file is pushed whole unless it holds the company number or a run of 10 or more digits. Two places are exempt: inside a whole 64-hex or 40-hex hash value, and the whole value of a VOUCHERKEY element. Each kept file is listed in `kept-local.txt` by size and sha256, with its reason. Screens stay here too, listed in `SCREENSHOTS.txt`. (verified)
- **Six tool answers** carry the company number in their `company_number` field, so they stay here whole: the two `list_companies` answers (001, 005) and the four marks reads (002, 003, 004, 006). Their values are quoted above from the files. The tools list (000) and the server's own logs are pushed. (verified)

The four desktop-client connector servers stayed running: no call was made to them from this sitting, and their calls were not measured.
