# Row 2057-C: an item purchase whose purchase ledger is taken per item line: results

**First line: C1 not run: none found on the two screens looked at. At step 2a, F12 at the Gateway of Tally opened no screen. At step 2b, the Purchase voucher type holds no option whose own label ties the purchase ledger to the item line. Nothing was saved after BK4, so no restore was made. The row went from step 2c to step 15, as the sheet says. Every EXPECT of steps 0 to 2 and 15 was met; no stop, no dialog.** (verified)

Sheet `lab/2057-c-sheet` at 67b12237 (folder `lab/2057-c`; SHA256.txt 8 of 8 equal, by sha256 and bytes). `gen_requests.py --check` printed its ok line with 2 requests. LAB-COMMON v3. Every step and stamp is in `notes.txt` (UTC).

## The answers

- **(a)** C1 not run: none found on the two screens looked at. No option was changed, and the marks did not move: M 213, V null from step 1 to step 2c. (verified)
- **(b)** Not reached: C2-C was not keyed (step 8 runs only if step 7 changed an option). (verified)
- **(c)** Not reached: the raw a07 read was not sent. (verified)
- **(d)** Not reached: the raw sl read was not sent. (verified)
- **(e)** Not reached: `vouchers` and `purchase_register` were not called for C2-C. (verified)

As the sheet's "What the row decides" says for this case, the transcriptions of the two screens are the result, and nothing was saved. This does not show that no such option exists: a voucher screen's own F12 was not opened, and neither was the Data menu's "Configuration" entry, which is another route than step 2a's. Any next row waits for a decision and a sheet of its own. (verified)

## The two screens looked at

- **2a, the Gateway's configuration:** F12 was pressed once at the Gateway of Tally, and no screen opened. The Gateway's button panel lists only "F2: Date" and "F3: Company". The capture after the key is byte-equal to step 0's Gateway (`shots/s2a-gateway-after-f12.png`, sha256 a1ccfeac…). Nothing was listed to transcribe. (verified)
- **2b, the Purchase voucher type** (Alter > Voucher Type > Purchase, as it opened, `shots/s2b-purchase-type-as-opened.png`), in the screen's own words. (verified)
  - **General:** Select type of voucher: Purchase. Abbreviation: Purc. Activate this Voucher Type: Yes. Method of Voucher Numbering: Automatic. Numbering behaviour on insertion/deletion: Retain Original Voucher No. Set/Alter additional numbering details: No. Show unused vch nos. in transactions for Retain Original Voucher No. behaviour: Yes. Use effective dates for vouchers: No. Allow zero-valued transactions: No. Make this voucher type as 'Optional' by default: No. Allow narration in voucher: Yes. Provide narrations for each ledger in voucher: No. Track Additional Costs for Purchases: No. **Enable default accounting allocations: No.** WhatsApp voucher after saving: No.
  - **Printing:** Print voucher after saving: No.
  - **Name of Class:** none.
  - Nothing was changed; one Ctrl+Q, no prompt, no save.
- **2c:** the one label that speaks of allocations, "Enable default accounting allocations", names neither the item line nor which value takes the ledger per item line. So no option qualifies by the sheet's rule. (verified)

## Every EXPECT line

| step | EXPECT | result |
|---|---|---|
| 0 | Current Date 1-Apr-2025 and "No Vouchers Entered"; one tally.exe owning 9001 | PASS: Tuesday, 1-Apr-2025, "No Vouchers Entered"; tally.exe 1200 owns 9001 (IPv4 and IPv6); GET /status 200. The Gateway capture is byte-equal to 2057-B's step 0. TallyPrime 7.2 Silver. (verified) |
| build | zip = digest; `bridge_mcp.exe` = 2057-B's, unchanged | PASS: zip 922968055bb143f1a802beffbb496bbd94795a539382f9b5ae013c4985a45685; exe c5b75d6480202ef70a5216f74d369ea0963b4a2eedbb3d4912e4f25b385e943d before the first call and after the last. Client c127118d…. (verified) |
| 1 | the five tools in `tools/list` | PASS: 32 tools, all five present. (verified) |
| 1 | `list_companies` exactly THE COMPANY, `verified_tuple` | PASS: BRIDGE CLASS PURCHASE LAB, 92a7ef97-dd28-4286-b02c-c059fcb8690e, verified_tuple. (verified) |
| 1 | marks M0/V0 = 213 / null | PASS: 213 / null, reused false. (verified) |
| 1 | `ledger_masters` compliance rows equal to 2057-B's step 1 | PASS: 6 rows, each equal field by field apart from the snapshot. (verified) |
| 1 | the Purchase row of `masters` voucher_types equal to 2057-B's | PASS: 24 rows; the Purchase row equal. (verified) |
| 2 | BK4 taken; marks M0/V0 after | PASS: `backups/bk4/TDBK1800_<company number>.001`, 24,100 B, sha256 e08bd0136b05e1a525c642ac1e51d9a1b8c905cbcf06884a5f8033fb9da23041, 17:05 local (the folder holds that one file). It stays on the lab PC. Marks 213 / null. (verified) |
| 2a, 2b | marks equal to M0/V0 | PASS: 213 / null after 2b (2a opened nothing). (verified) |
| 2c | marks M0/V0 once more before step 15 | PASS: 213 / null. (verified) |
| 15 | exe unchanged; Shut Company; none loaded; current date as step 0; one tally.exe; 9001 LISTENING; /status 200 | PASS: Company > Shut from the Gateway's menu (Alt+K, Down ×4 to "SHut", Enter, Enter on the only row); Select Company, none loaded; the book's current date 1-Apr-2025 as at step 0; tally.exe 1200 owns 9001; GET /status 200. No XML request after the shut. (verified) |

Steps 3 to 14 were not run: no master, option or voucher was saved, no raw read was sent, and no restore was needed. BK4 is kept on the lab PC. (verified)

## Kept on the lab PC

No captured byte is altered. A captured file is pushed whole unless it holds the company number or a run of 10 or more digits. Two places are exempt: inside a whole 64-hex or 40-hex hash value, and the whole value of a VOUCHERKEY element. Each kept file is listed in `kept-local.txt` by size and sha256, with its reason. Screens and BK4 stay here too, listed in `SCREENSHOTS.txt` and `kept-local.txt`. (verified)
- **Six tool answers** carry the company number in their `company_number` field, so they stay here whole: `list_companies` (001), the three marks reads (002, 005, 006), `ledger_masters` compliance (003) and `masters` voucher_types (004). Their figures are quoted above from the files. The tools list (000) and the server's own logs are pushed. (verified)

The four desktop-client connector servers stayed running: no call was made to them from this sitting, and their calls were not measured.
