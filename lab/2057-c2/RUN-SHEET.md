# 2057-C2 the two screens 2057-C did not open: Data > Configuration, and F12 inside a Purchase voucher (#2057), look only

Not for merge. About 45 minutes.

This sheet changes NOTHING in Tally. It opens two configuration screens and one voucher-creation screen, transcribes them, and leaves each without a save. Nothing is keyed into any field, no option is changed, no voucher is accepted, and no raw request is sent. ComplyEaze Bridge only reads, and only the marks and the company list. `SHA256.txt` lists every file here with its bytes; check it before step 0.

`LAB-COMMON.md` in this folder applies in full. It is v3, byte-equal to the one in `lab/2057-c-sheet` (sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6). Where it and this sheet differ, the stricter holds.

## What it decides

2057-C (`lab/2057-c-answers` at e36e7e30, RESULTS.md) ended at its step 2c with "C1 not run: none found on the two screens looked at", and nothing saved:
- F12 at the Gateway opened no screen;
- the Purchase voucher type, transcribed in full, has no label that ties the ledger to the item line.

TallyPrime also has a "Configuration" entry under CONFIGURE in its Data menu, and a voucher screen has its own F12 (Configure). 2057-C opened neither: they were off its route. This row transcribes both.

**It records every label, on those screens, that names a ledger per item or an accounting allocation, in the screen's own words, with its value as shown.**
- It changes no option. Which option a later row sets, if any, is decided from this row's transcriptions, by a ruling and a sheet of its own.
- No EXPECT is stated for what the screens hold. EXPECT lines remain only for identity and marks.

**Limit:** one book, one release, the screens as this book shows them.

## Rules and company

**Who acts.** On the owner's standing word of 27 and 28 Oct 2026, the lab does every step here on this synthetic company, and records each as LAB-COMMON C11 directs.

**THE COMPANY** is `BRIDGE CLASS PURCHASE LAB` (synthetic), GUID `92a7ef97-dd28-4286-b02c-c059fcb8690e`, as 2057-C left it. If it is absent, or step 1 differs from 2057-C's step 1 as step 1 sets out: STOP and report. Only THE COMPANY is loaded, alone, from step 0 to the last step. Licensed mode only; Education mode = STOP.

Differences from LAB-COMMON (the stricter rule holds everywhere else):

1. **No backup, no restore.** Nothing is written, so LAB-COMMON C5 and C6 are not run. The marks (rule 2) are the proof that nothing was written.
2. **Marks.** M (`ALTMSTID`) and V (`ALTVCHID`) are read with `ledger_masters` (basic, first page, no `snapshot_id`): `snapshot.master_alter_id`, `snapshot.voucher_alter_id` (record null as read), `snapshot.reused` false. They are read at step 1 (M0, V0) and after every step. **Either moving = STOP**, and report what was on the screen.
3. **Keys.**
   - Move on a list or a configuration screen only with the Down arrow, one press at a time, with a screenshot after each, to bring the rest of it into view.
   - Never press Enter on an option, never press Space or a letter on one, and never type into a field.
   - Enter is pressed only on a menu entry or a list entry whose label the step names.
   - Leave each screen with ONE Ctrl+Q, only after a screenshot shows its title. Never press Ctrl+Q or Escape at the Gateway of Tally, never two keys in one call, and never Escape on a master, configuration or voucher screen (C7).
   - Never Ctrl+A on any screen in this row.
4. **The one prompt answered** (in addition to LAB-COMMON's) is on leaving the voucher-creation screen of step 4: a prompt whose text asks whether to quit, or to leave without saving, the voucher being created.
   - Screenshot it and copy its text exactly.
   - Answer the choice that leaves WITHOUT saving (for example "Yes" to "Quit?"). If the text does not make plain which choice discards the voucher, press no key and STOP.
   - Any other prompt, anywhere, is C7: no key, STOP.
5. **One Tally.** At step 0 and at the end, as LAB-COMMON C2 a and b set out, plus `Get-NetTCPConnection -LocalPort 9001 -State Listen`: exactly one `tally.exe`, and it owns port 9001.

## Build

2057-C's build, unchanged:
- **The bundle:** the Windows bundle of the merge-queue run of #2327, CI run 41745127138, `merge_group`, head ae9a1bb5 (`ae9a1bb5096010a28fde3eac19aac4457f49eb69`).
- **The artifact:** `unsigned-bundle-smoke-windows-latest`, id 13283891845, 59,284,798 bytes, digest `sha256:922968055bb143f1a802beffbb496bbd94795a539382f9b5ae013c4985a45685`.

Record `bridge_mcp.exe`'s sha256 before the first call and after the last. It must equal 2057-C's (`c5b75d6480202ef70a5216f74d369ea0963b4a2eedbb3d4912e4f25b385e943d`); a difference = STOP.

The launch and the client are 2057-C's:
- the client is `scripts/client.py` of `lab/2057-item-purchase-class-gst-answers` at a8bcd9f8 (sha256 `c127118d327a50f7836118f440a0da9855215ea0b55c50e5f56fd1da91b8a600`);
- ONE new empty `BRIDGE_AGENT_DATA_DIR`;
- the write settings absent.

`ledger_masters` and `list_companies` must be in `tools/list`.

## Files

- `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt`.

There are no requests: this row sends nothing raw.

## Steps

0. LAB-COMMON C2 step 0 with THE COMPANY loaded alone. Record Tally's release, edition, current period and current date from the Gateway, and screenshot the Gateway. EXPECT Current Date 1-Apr-2025 and "No Vouchers Entered", as at 2057-C's step 0 and step 15.
1. `tools/list` (count). Then:
   - `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`, with the GUID above.
   - Marks M0, V0: EXPECT 213 / null, `reused` false, as 2057-C's RESULTS records from its step 1 to its end. A difference = STOP.
2. **Data > Configuration** (look only):
   - From the Gateway, open the top menu "Data" by its key as the menu bar shows it (on 4 Nov its CONFIGURE heading held "Configuration"). Screenshot the menu whole and transcribe every heading and entry. If no "Configuration" entry is there, record the menu and go to step 4.
   - Press Enter on "Configuration" only. Screenshot the screen that opens, whole, and transcribe every entry or option with its value, in the screen's own words. Bring the rest of the list into view by rule 3. A field whose value is a folder or file path (a data or backup location) is recorded by its label and the words "a path, not recorded" (C11).
   - Open every entry whose own label names vouchers, voucher entry, invoices or accounting (for example "Voucher Entry" or "Invoicing"), one at a time, in the screen's order: Enter on that label, transcribe it the same way, then leave it with ONE Ctrl+Q back to the Configuration screen before the next. Open no other entry.
   - Leave each screen with ONE Ctrl+Q after a screenshot of its title, back to the Gateway. A prompt on leaving = C7.
   - Marks: equal to M0 and V0.
3. **Record from step 2** every label that names a ledger per item (or per stock item, or per item allocation) or an accounting allocation, with its value as shown and the screen it is on. Write "none" if there is none.
4. **F12 inside a Purchase voucher** (look only, nothing keyed):
   - Gateway > Vouchers > F9 Purchase. Screenshot the voucher screen as it opens and record its mode (Item Invoice or Accounting Invoice), its title, and the field the cursor is on.
   - If it is not in Item Invoice mode, press Ctrl+H, screenshot the mode list, and press Enter on "Item Invoice" only. Record the mode it opened in.
   - With nothing typed in any field, press F12 (Configure). Screenshot the screen that opens whole and transcribe every option with its value, in the screen's own words. Bring the rest into view by rule 3. If F12 opens nothing, record that.
   - Open no sub-screen of it, and press Enter on none of its options. Leave it with ONE Ctrl+Q after a screenshot of its title, and screenshot what follows:
     - **back on the voucher screen:** screenshot it, then ONE Ctrl+Q;
     - **rule 4's quit prompt, straight from the F12 screen:** rule 4 holds, not C7. Answer it as rule 4 says, and record that this one Ctrl+Q closed both screens;
     - **any other prompt:** C7, no key, STOP.
   - After the voucher's quit prompt is answered, screenshot the screen Tally returns to and record which it is (the Gateway, the Vouchers menu or the voucher type list). Press no further key on it until the marks are read.
   - Marks: equal to M0 and V0. A V that moved = STOP and report: a voucher may have been saved. Do not alter or delete it.
5. **Record from step 4** every label that names a ledger per item or an accounting allocation, with its value as shown. Write "none" if there is none.
6. **The end.** Gateway screenshot: "No Vouchers Entered", compared with step 0's. Then:
   - `list_companies` as in step 1;
   - the marks equal to M0 and V0;
   - hash the exe;
   - LAB-COMMON C10: Company > Shut from the Gateway's menu, by the route 2057-C's step 15 records (never Ctrl+Q at the Gateway); Select Company with none loaded; one tally.exe; 9001 LISTENING; `GET /status` 200.

## STOP rules

LAB-COMMON's, with rules 1 to 5 above. Also:
- a field that takes a typed character, or any option whose value changes on the screen;
- a marks move at any step;
- a voucher saved (do not alter or delete it: STOP and report what was saved);
- any prompt not named here (C7).

## What the book holds after each step (C8)

| after | the book |
|---|---|
| every step | as at step 1: nothing is written |

## What the row decides

- **Exactly one label, on the screens of steps 2 and 4, ties the ledger to the item line or names an accounting allocation per item, and says which value does so:** it is the candidate for a later row that changes it once. That row needs a ruling and a sheet of its own, with a backup and a set-back, as 2057-C's step 7 has.
- **More than one such label:** the transcriptions go to a ruling on which, if any, a later row changes.
- **None:** the shape is not reached through the screens these two rows looked at, on this release. `purchase_register`'s description keeps its "not measured" line, with both rows cited.

## What master admits

Nothing in this row reaches Bridge's request code beyond the marks and the company list, which are 2057-C's tool calls. The code that would carry the shape is set out in 2057-C's sheet ("What master admits, at f3bdc6d5"). This row only decides whether a keyed invoice can take that shape.

## What the row records, and what it does not

**Records:**
- the Data menu and its Configuration screens, transcribed;
- the voucher screen's own F12 configuration, in Item Invoice mode, transcribed;
- the marks before and after every step.

**Does not:**
- change any option;
- key or save any voucher;
- open any other screen;
- other releases, Gold, Education mode.

## Evidence

To `lab/2057-c2-answers`, after the pre-push scan and the run coordinator's yes:
- `RESULTS.md`: the labels of steps 3 and 5, and every EXPECT line with PASS or FINDING;
- `notes.txt`: UTC, one line per step, with each mark;
- `answers/`: every tool answer, whole;
- `transcriptions/`: each screen, in its own words;
- `SCREENSHOTS.txt`: the captures by sha256. They stay on the lab PC.

No identifier beyond the company's synthetic GUID, and no figure, is recorded. The company's number and data folder are not recorded.
