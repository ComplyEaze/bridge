# Results: W4 (#1108, row 5), a ledger rename with the marks read before and after, 26 Oct 2026. PASS on the marks; the identity part is NOT OBSERVABLE ON THIS BUILD

The sheet is `lab/sitting4-run-sheets` W4 at eae6a82ab0a4f37ccead721e23860eab84801b33 (`SHA256.txt` 6/6 OK). The marks were read the way the run's review ruled before step 1. The next sheet commit, 955b1f5a43bacc908dace4c96883bb8f48e142be, writes that same rule into the README: take M and V from `ledger_masters`' first-page `snapshot`, since `tally_status` carries none. This commit is on top of 955b1f5a (`SHA256.txt` 6/6 OK there too). TallyPrime 7.2, Silver, not Educational mode. The build is the staged `w1-2306d86d`, with `bridge_mcp.exe` sha256 **92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524**, checked before every call. It ran over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, port 9001, a new data folder kept for this row, `BRIDGE_TERMS_ACCEPTED=true`, redaction unset, and posting and import both unset. The owner authorised the lab, in the lab's own chat, to key the OWNER steps itself, one act at a time. Every screen act below was done that way, with no Escape on any master screen.

## THE COMPANY
`BRIDGE RENAMEMARKS LAB`, company number 100080, GUID **cc23473c-e7dc-4771-a486-6e19b5d8d5ba**, books from 1-Apr-2025. Select Company showed no company of that name before it was created. Features were accepted as offered. Three ledgers were created under Sundry Debtors: BRIDGE Alpha, BRIDGE Beta and BRIDGE Gamma, with bill-by-bill Yes as offered (the sheet does not set it). There are no vouchers.

## Backup (C5)
BK1 went into a new empty folder. The company was selected in the filtered list, the highlight moved to "End of List", Enter pressed and "Accept?" answered Yes. The folder then held one file: `TDBK1800_<company number>.001`, **23,990 bytes, 06:41 IST, sha256 c994fdeee7e5e0e4b5ecaa00a9a2e741c80ad854b7c0c20eea088fd59024ac08**. The file stays on the lab PC, untouched. The run's review said to keep it as the before-state after the sitting, so no RESTORE was asked for or done.

## The three states
Each mark is taken from `ledger_masters` (first page, no `snapshot_id`, the same arguments each time): `snapshot.master_alter_id` gives M and `snapshot.voucher_alter_id` gives V. `snapshot.reused` read **false** every time. Beside each, the raw LAB-COMMON MARKS read (ALTMSTID / ALTVCHID) is shown; all three agree.

| State | Screen act (OWNER STEP keyed by the lab) | `ledger_masters` snapshot M / V | Raw MARKS ALTMSTID / ALTVCHID | The ledger's row |
|---|---|---|---|---|
| Step 1 (before) | none | **210 / null** | 210 / none | `BRIDGE Beta`, parent Sundry Debtors |
| Step 2 | Alter > Ledger > BRIDGE Beta; Name field: End, typed `2`; screen read `BRIDGE Beta2`; Ctrl+A | **211 / null** | 211 / none | `BRIDGE Beta2`, parent Sundry Debtors |
| Step 3 | Alter > Ledger > BRIDGE Beta2; Name field: End, Backspace; screen read `BRIDGE Beta`; Ctrl+A | **212 / null** | 212 / none | `BRIDGE Beta`, parent Sundry Debtors |

Deltas: **M1 - M0 = +1**, **M2 - M1 = +1**. **V1 - V0 and V2 - V1: no voucher mark at any point** (the book holds no voucher, so V is null throughout; the voucher mark did not appear or move). The name changed and changed back, in both the tool's rows and the List of Ledgers on screen. BRIDGE Alpha and BRIDGE Gamma were unchanged in every read.

`tally_status` was called at each step as the sheet says. It returned TallyPrime 7.2, silver, Education mode false, and the one company (100080, the GUID above), with no marks.

## The identity part: NOT OBSERVABLE ON THIS BUILD
The sheet asks for each ledger's GUID, MasterID and AlterID. On this build `ledger_masters` returns no GUID, MasterID or AlterID (its `fields` are `basic` or `compliance`), and `masters` has no ledgers kind. Before step 2 the run's review ruled that only the built tools are used, so this part is recorded as not observable. Whether the ledger keeps its GUID and MasterID and gains a new AlterID is therefore **not established** here. No hand-written request was sent.

## Observations (recorded, not findings for this row)
- **Mailing name.** Before the step 2 accept, the alteration screen's Mailing Details Name still read `BRIDGE Beta`. When the ledger was opened again for step 3, it read `BRIDGE Beta2`: the accept had carried the new name into the mailing name. The step 3 accept was made the same way. The mailing name after step 3 was not opened again.
- **`ledger_masters` text for Tally's own ledger.** In all three reads, `Profit & Loss A/c` has `parent` returned as U+FFFD followed by `#4; Primary`. Tally's own answer for this ledger's parent holds a `&#4;` reference before "Primary". This is the same in all three answers, so it does not bear on the rename.
- The lab's client crashed printing the step 2 `ledger_masters` text to the console (cp1252 cannot show U+FFFD). The answer had already been saved whole. The later calls ran with UTF-8 output.

## C10
Company > Shut (Alt+K, h, Enter on the one company). Select Company then showed with no company loaded. tasklist: one tally.exe (exit 0). netstat: 9001 LISTENING with that PID, IPv4 and IPv6. GET /status: 200, 51 bytes. No XML request was sent after the shut.

## In this commit
- `notes.txt`: UTC times, one line per step. It includes a correction: one hold was logged as "PC locked" from a process check that was wrong; the owner says the screen was never locked.
- `send-log.txt`: every raw read, with request sha256, bytes and answer sha256.
- `answers/`:
  - the tool list;
  - `list_companies`, `tally_status` and `ledger_masters` for each step, with arguments;
  - the company list and every MARKS answer, with headers;
  - the C10 observations and the /status answer.
- `requests/`: the company list request, and the MARKS request filled from c1-marks-TEMPLATE (sha256 4bcdcfd4…; only `@@COMPANY@@` and `@@GUID@@` replaced).
- `SCREENSHOTS.txt` and `SHA256.txt`.

Text files are stored with LF line ends. The `.headers.txt` files were received from curl with CRLF; the UTF-16 Tally answers are byte for byte as received and match `send-log.txt`.

Not committed: the screenshots and the BK1 folder. Both stay on the lab PC.
