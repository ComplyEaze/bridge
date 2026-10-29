# Results: #1709 settle row S1, second sitting, 29 Oct 2026. **Stopped at step 1.2: on this Tally, the Journal type's "Prevent creating duplicate Voucher Nos." field could not be reached, so the not-created batch could not be made. Nothing was posted; the whole book was restored from BK0 and reads back as before.**

**The findings:**
- **The Prevent field is never a cursor stop (verified, this book, TallyPrime 7.2 Silver).** After Method of Voucher Numbering was set to Manual, the form showed "Prevent creating duplicate Voucher Nos.: No", but Enter moved from Method straight to "Use effective dates for vouchers", Backspace and Up moved from there straight back to Method, and a mouse click on the field's value did not move the cursor. This held on the unsaved form and again on the saved form, reopened. The row needs Manual with duplicates prevented, so it stopped there.
- **Saving Manual empties every existing Journal's number (verified).** Ctrl+A raised a second Tally prompt the sheet does not name: "Are you sure you want to update the voucher numbers of existing vouchers? Y : Accept Master & Update Vouchers / N : Back". After Y, all 17 Journals kept their GUID, MASTERID and date, and every `voucher_number` read back as the empty string (before: 1 to 16 and 18). The master mark moved 292 to 294; the voucher mark stayed 112. `tables/renumber-after-manual-save.txt`.
- **The numbering prompt can be raised twice.** The named prompt ("Changing the method of voucher numbering will affect voucher numbers of existing transactions. Continue?") came again, word for word, when the Method field was re-entered (Backspace, then Enter) on the unsaved form. On the saved form, Enter on Method raised no prompt.
- **`masters` `voucher_types` refused on an unchanged book (a finding for #2150's build, partial).** The first `masters` call of each launch was refused, `masters_read_failed`, cause `masters_collection_changed` (answers 018 and 053), with no mark moving; the same call repeated at once answered (019 and 054). Two launches, two times out of two.

**The restore (verified).** BK0 (the first sitting's backup, a95a19ed...) restored by Tally's Restore with the two named prompts only. After it: marks 292/112 (equal to M0/V0); J-RESTORED equal to J-BEFORE in count (17), dates, numbers, MASTERIDs and GUIDs; no voucher 22 to 30 Jun 2025 and none numbered `S1709-1`; THE TYPE back to automatic, Retain Original Voucher No., duplicates not prevented, as at step 1.1.

**Who acted.** The lab, on the owner's standing word of 28 Oct 2026 and the go-ahead for #2150's build recorded in `lab/0.7.0-auto-order` ORDER.md "Row 22, second sitting" at 24a844aa. Each departure from the sheet below was taken on the run lead's recorded word. The sheet is `lab/1709-settle-row` at 8fdb4e8dc26e9f71bcefeb59e624af8e5dee66f6 (`SHA256.txt` 2 of 2 equal).

## The build
#2150 at ed0f5fc5, the build the row note names, already staged and re-hashed on 29 Oct: `bridge_mcp.exe` 32,607,744 B, sha256 e95269fb9ab2d72676902891615d733e2cb13e12cc003b483afbc0c0110d9a58, equal to `mcpb-smoke.json`'s `binary_sha256`, before the first call and after each launch; `pdfium.dll` beside it equal to the smoke file's. The tool reports bridge-tally 0.6.1.
- Posting launch (steps 0 to the stop): data folder new and empty; `BRIDGE_AGENT_ENABLE_WRITES` present; the three unset keys absent; error stream 0 B; exit 0.
- After the stop: the key removed, `launch-env.json` sha256 c735598ac53a7931c42dfa221b75e379666d8c630a4fc54148aa021a39d38999.
- Read-only launch (step 7.3): the same exe and data folder, the key absent; error stream 0 B; exit 0.
- Data folder at the end: `agent-egress.jsonl` and `terms-acceptance.jsonl` only (no import journal: nothing was built or posted).

## Steps
| step | what | reading |
|---|---|---|
| 0.1 | THE COMPANY (BRIDGE SHAPE LAB, 100021) loaded alone. One tally.exe owns 9001; `/status` 200; Tally's date Monday 20-Apr-2026. C2 raw MARKS: one COMPANY row, THE COMPANY's GUID, ALTMSTID 292, ALTVCHID 112. Posting key present. Exe hash equal | PASS |
| 0.2 | BK0: the first sitting's `backups` folder, that one file, `TDBK1800_100021.001`, 102,253 B, a95a19ed...; the Restore list showed it at 29-Oct-26 3:08 pm local = 09:38 UTC (the first sitting recorded 09:37 UTC) | PASS |
| 0.3 | `tally_status` 7.2 silver, not education; `list_companies` exactly THE COMPANY | PASS |
| 0.4 | L1 Discount Allowed (Indirect Expenses), L2 Capital A/c (Capital Account): neither form shows a "Maintain balances bill-by-bill" field, and neither asks for a bill allocation | taken as No (deviation 1) |
| 0.5 | MARKS M0 292, V0 112 (`reused` false), equal to the first sitting's. J-BEFORE: 17 Journals; the seven of 15 to 21 Jun (numbers 6 to 12) equal the first sitting's; none 22 to 30 Jun. 22 to 30 Jun window: `partial`, `empty_window_wider_read_nonempty`, no snapshot | PASS |
| 1.1 | Screen: Automatic; Retain Original Voucher No.; additional numbering details No; show unused Yes. `masters`: refused once (`masters_collection_changed`), answered on the repeat: automatic, `auto_retain`, duplicates not prevented | PASS (deviation 2) |
| 1.2 | Manual chosen; the named prompt, Yes by mouse; again on re-entry, Yes by mouse; the Prevent field not reachable; Ctrl+A raised the update-vouchers accept, Y by mouse (Manual saved, Prevent No); J-AFTER and MARKS read; reopened: Prevent still not reachable; left by Ctrl+Q without saving | **stop** (deviations 3 to 5) |
| 1.3 to 6 | not run | - |
| stop | C8 reads: MARKS 294/112; 22 Jun: no item; `masters`: THE TYPE manual, duplicates not prevented. Launch stopped; posting key removed | done |
| 7.2 | BK0 restored (Accept Yes; Replace), THE COMPANY loaded alone | PASS |
| 7.3 | read-only launch: as "The restore" above | PASS |
| 7.4 | launch stopped; exe equal; C10: THE COMPANY shut, one tally.exe owns 9001, `/status` 200, none loaded | PASS |

## Deviations (each on the run lead's recorded word)
1. 0.4: neither form shows a bill-by-bill field (company not bill-wise for non-trading ledgers, inferred); taken as No.
2. 1.1/1.3: a `masters_collection_changed` refusal on an unchanged book is recorded as a finding; the read was repeated once before any change, and the screen read-back would have stood in at 1.3.
3. 1.2: the named prompt appeared twice; the second came from re-entering the Method field with Backspace, then Enter. Both were answered Yes by mouse.
4. 1.2: after the second Yes, Up once from "Use effective dates" (landed on Method, its list open); Ctrl+A then raised Tally's update-vouchers accept; Y by mouse. J-AFTER, `renumber-after-manual-save.txt` and MARKS were read right after this save, before the form was reopened.
5. 1.2: the Prevent field still could not be reached on the reopened form: nothing more saved, the row stopped, and the sheet's whole-book restore followed.

Screen exits, each with a capture after it: ledger and voucher type forms by Ctrl+Q after a capture showed the title; each list by its Back row by mouse (the first click only highlighted the row every time; a second click left the list); the List of Masters popup by one Escape, which reached the Gateway each time (four times). A mouse click on the text of Tally's update-vouchers accept did nothing twice; a click on its "Y :" accepted it.

## Not measured
Steps 1.3 to 6: the not-created batch, the settle dialog, the settlement and what it changes. Whether the Prevent field can be set by another route (F12 options, a voucher type created new, an XML master alteration). Whether the `masters` refusal follows the first call of every launch or something else.

**Confidence:** the stop, the emptied numbers, the double prompt and the restore are directly observed (verified, this book, one sitting); the `masters` pattern is partial (two launches).

## Files
- `answers/`: every tool answer, whole, in call order (000 to 054), and `answers/raw/s0-c2marks.response` (C2's raw MARKS answer, UTF-16LE).
- `tables/`: `j-before.tsv`, `j-after-manual-saved.tsv`, `j-restored.tsv`, `renumber-after-manual-save.txt`.
- `evidence/`: step-0 and C10 checks (netstat cut to port 9001), `s0-extra.txt`, `bk0.txt`, the data-folder listings before and after the restore.
- `restore-plan.txt`, `client-log.txt`, `notes.txt` (UTC), `SCREENSHOTS.txt` (captures kept on the lab PC), `substitutions.txt`, `scripts/` (`call.py`, `reads.py`, `step0.ps1`; the stdio client stays on the lab PC).
