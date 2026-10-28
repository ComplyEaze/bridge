# Results: #1650 box row, run CI, 28 Oct 2026. The one-ledger box opened whole on a 1920x1080 screen at 125 percent; No refused master_approval_declined with nothing sent; Yes created one ledger, posted_verified; a third call answered from the journal with no box

The sheet is `lab/1650-ledger-create-box-row` at adae9d8cd005d0669caa1f1a24c8061c48d5dcff (`SHA256.txt` 3 of 3 equal).
- **Build:** #1650's CI package of head 239896f8: run 40512095961, artifact 12737059797. `bridge_mcp.exe` sha256 85a003ce59ae1e0451cdcad03f6bb7391255248a54e632ac6f9b6832f49bb8a7, equal to the package's `binary_sha256`, at the first call and at the end. `initialize` bridge-tally 0.6.1; `tools/list` 35 tools, including `prepare_ledger_creates` and `post_ledger_creates`.
- **#1650's head:** it has since moved to 01b2e81e5d8c3c18bfbbb8f39ae34830f613be73; run CI used the 239896f8 package, as the sheet says.
- **Tally:** TallyPrime 7.2 (Latest), Silver Rental, Licensed (About screen; `tally_status` education_mode false).
- **Display:** 1920x1080 at 125 percent, work area 1920x1030; unchanged.
- **Company:** BRIDGE LEDGERBOX CI LAB, new: number 100092, GUID 5848d0ca-ed85-4c97-ab20-e045893a9803, books from 1 Apr 2025.
- **Settings:** posting and ledger creation were set by the lab on the owner's word of 28 Oct; batch post was absent.

## Steps
| step | what | answer |
|---|---|---|
| 1 | `list_companies` / `tally_status` / `ledger_masters` | exactly the company, `verified_tuple`; 7.2 silver; **M0 207**, V none, rows Cash and Profit & Loss A/c |
| 2 | BK1 | `TDBK1800_100092.001`, 23,406 B, sha256 7f5eeebc8a83ffeaf8a0f5dc1e4887abf9cfd3794e04e228c446bc5e237dcaa4; **M0' 207** |
| 3 | `prepare_ledger_creates` LBox Suspense, suspense | batch `bridge-107db096-234f-492c-a3e5-5fee70d18ccb`; preview of nine lines naming the company and "LBox Suspense" under Suspense A/c |
| 4 | `ledger_masters` | M 207 |
| 5 | `post_ledger_creates`: the box, answered **No** | **`master_approval_declined`**, attempt_recorded false, not_dispatched; M 207 |
| 6 | `post_ledger_creates` (same batch): the box, answered **Yes** | **`posted_verified`**, attempt_recorded true, dispatched (resent false), created 1 and every other counter 0 |
| 7 | `ledger_masters` | **M1 208** (M0' + 1); one new row LBox Suspense under Suspense A/c; the other rows unchanged |
| 7b | `post_ledger_creates` (same batch) | **no box**, 0.079 s; `posted_verified`, dispatched, resent false, `journal.response_recorded` true, `journal.counters.created` 1, `journal.verdict` `posted_verified`; M 208 |
| 8 | `egress_log` | recorded whole |
| 9 | C10 | shut; no company loaded; one tally.exe on 9001; /status 200 |

**Step 5 needed two calls.** At the first box (05:54Z) the lab's screen-control grant for this exe path was declined, so the box was not read or answered. That call returned `pending` after 40.1 s (retry_after_s 15), attempt_recorded false, not dispatched. Asked again, the owner approved the grant at the next box (08:44Z), and that call is the decline recorded above.

**Each box, as the lab read it on the screen** (captures listed by sha256):
- whole on the screen: its title bar, every line of its text, and its buttons;
- title "ComplyEaze Bridge - create one ledger?";
- the answer line "Press Y (or click Yes) to create this ledger in Tally. Press N (or click No) to cancel. Nothing is created until you answer.", wrapped;
- a blank line, then step 3's nine preview lines unchanged;
- buttons Yes, No (the default) and Cancel.

The box opened on top by itself each time, and no window was brought forward (rule 7). The owner was not watching the boxes. The lab answered them by mouse on his word of 28 Oct, and the person's mouse click is not measured.

**One Tally (rule 6):** before every `post_ledger_creates`, one tally.exe (13992) owned port 9001 (notes).

## Not established
"Its GUID beginning with THE COMPANY's GUID": the tool's `ledger_masters` (basic and compliance) returns no ledger GUIDs, and this sheet allows no raw read.

## Files
- `answers/`: every tool answer whole (000 to 015).
- `client-log.txt`.
- `notes.txt` (UTC, one line per step).
- `SCREENSHOTS.txt`: the captures and BK1 by sha256; they stay on the lab PC.
