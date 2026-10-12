# Item 1c (partial-landing probe): NOT RUN, 12 Oct 2026

Build: bridge_mcp.exe 411f7cca80ca4a5fee543f4f44d14b911ce23e982169785b1762d8d50472e5a6 (#1516 build); enable_writes true, enable_batch_post true. Company: BRIDGE BATCH LAB (100050), loaded alone.
Result: NOT RUN. The one variable could not be set on this build's screen, so no build_import_xml or post_import was sent.
- Method of Voucher Numbering = Manual was set and saved.
- "Prevent creating duplicate Voucher Nos." is shown (No) but never takes focus. Tried: Enter from Method; Shift+Tab back from the next field; mouse click and double-click (these acted on the focused field); and reopening the alteration with Method already Manual. F12 has no option for it.

Deviation (approved by the orchestrator): unlisted Tally prompts during the alteration, answered after a pre-change backup.
1. On choosing Manual: "Changing the method of voucher numbering will affect voucher numbers of existing transactions. Continue?" (No the first time and the alteration quit unsaved; then Yes, twice).
2. On Ctrl+A: "Are you sure you want to update the voucher numbers of existing vouchers? Y: Accept Master & Update Vouchers / N: Back" (Y).

Finding (separate from 1c): switching Payment from Automatic to Manual with "Update Vouchers" blanked all 32 existing Payment voucher numbers.
- Before (08:25:47Z): numbers 1..32 (response_sha256 be5ddc3b…).
- After (08:31:02Z): all "" (response_sha256 98a153d8…).
- Unchanged: every voucher's master_id, alter_id, guid, date, amounts and narration, and the voucher mark (87). The master mark moved 224 → 226.
- So stored voucher numbers changed with no voucher AlterID change and no voucher-mark change. See s1c-pre-payment-numbers.txt and s1c-after-manual-payment-numbers.txt.

Restore (orchestrator option a):
- Pre-1c backup restored over BRIDGE BATCH LAB only, per ../restore-plan.txt.
  - Restore Data Path C:\Users\Public\TallyPrimeEditLog\data; source C:\bridge-lab\1c\backup-pre; one company selected (100050).
  - Prompts: "Accept ?" (Yes); "Company data folder (100050) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." (Replace). No repair or migrate offer.
  - Screenshots s1c-r1..r6.
- After the restore (08:34:28Z): Payment numbers are 1..32 again. The raw response is byte-identical to the pre-1c read (response_sha256 be5ddc3b…). Marks are back to master 224 / voucher 87.
- Payment is on Automatic (s1c-r7).
- B2: C:\bridge-lab\1c\backups\B2\TDBK1800_100050.001, 57,180 B, sha256 dc69ec4f32206cbfedc7afd6319fcc457cb4efe6b84420624fe024b15f555f84, 08:35:48Z.
