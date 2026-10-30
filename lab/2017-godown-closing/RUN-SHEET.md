# 2017 closing stock by godown (#2017): Tally's Godown Summary and Stock Summary with and without the explode flag, and Stock Query, on the synthetic book as it stands and after a third godown under another, two items under the root and one transfer between godowns are keyed, then restored

Not for merge. About 3.5 hours. Part A sends reads only. Part B keys three masters (one godown, two stock items) and one voucher between a backup and a restore (C5, C6). `SHA256.txt` lists every file here with its bytes; check it before step 0.

## What it decides
#2017 asks for closing quantity **and value** per item per godown, with godown totals that tie to the Stock Summary. Two things are captured already:
- Tally's own Godown Summary, requested by that name (`lab/2017-2018-stock-by-godown-batch-answers` at `5b793065`). It answers one closing line per godown (quantity, rate, amount) and nothing per item, and its amounts sum to the Stock Summary's total.
- Stock Query (`lab/2018-stock-query-items-periods-answers` at `a5b369bf`, and the Stock Query pull requests of #2018), which answers quantity per godown and batch for one item, with no value.

No captured answer carries a value per item per godown. This sheet measures the one variable already observed on a stock report, the explode flag:
- Section 12a.13 of the protocol reference saw `<EXPLODEFLAG>Yes</EXPLODEFLAG>`, on TallyPrime 7.1, expand each top-level line of the **Stock Summary** by **one** level: a stock group by its items.
- There, an item directly under the root was also followed by a batch and godown line (`SSBATCHNAME`, `SSGODOWN`) with its figures. No such line was recorded under an item inside a group.
- The flag has never been sent with the Godown Summary.

It also measures four things no capture has:
- a godown under another godown;
- a transfer between godowns;
- the Godown Summary at a date other than the year end;
- whether Tally's screen title on a godown's drill-down is another report.

**Limit, whatever the result.** Every item of this book sits in a stock group. Part A shows what the explode flag gives for those items; Part B adds items directly under the root. If Part B shows a value per godown for a root item, that says nothing about an item inside a group: those stay unmeasured by this sheet, and the issue's acceptance line on values is not met for them.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds.

**Who acts.** On the owner's standing word of 27 Oct 2026, the lab does every step here on the synthetic company and records each as LAB-COMMON C11 directs. C11's rule on names applies to what is pushed.

**THE COMPANY** is `BRIDGE SHAPE LAB` (synthetic), as it was last read on 30 Oct 2026 (`lab/2018-stock-query-groups-batches-answers` at `0678520c`) after the restores before it:
- marks `ALTMSTID` 292 and `ALTVCHID` 112;
- two godowns, `Main Location` and `Factory Floor`, both directly under the root;
- eleven stock items in three stock groups (`Packaging`, `Raw Chemicals`, `Finished Kits`), none directly under the root;
- the Godown Summary at 31 Mar 2026 as the 28 Oct capture shows it (Factory Floor `316.667 Kgs`, Main Location with an empty quantity, the two amounts summing to the Stock Summary's);
- the Gateway's current period 1-Apr-26 to 31-Mar-27 and current date 20-Apr-2026.

If it is absent, or step 1's marks differ (record the difference; the run coordinator decides before step 2): STOP and report. Only THE COMPANY is loaded, alone, from step 0 to the last step.

Differences from LAB-COMMON (the stricter rule holds everywhere else):
1. **Two kinds of read.**
   - **Tool reads** are calls of ComplyEaze Bridge's built tool, one at a time.
   - **Raw reads** by curl are allowed by the owner's decision of 26 Oct 2026 for shape discovery. Each is a committed file in `requests/`, written by `gen_requests.py` from the committed Stock Summary request (`templates/`), and sent as C3 sets out. No other raw request is sent, except the drill-down candidates of step 4, which `make_report_request.py` writes in the plain shape only.
   - Each raw request differs from the template only in the report ID, the period end and at most one added variable:
     - `EXPLODEFLAG` `Yes` on a Stock Summary or Godown Summary;
     - `STOCKITEMNAME` on a Stock Query, naming one of the two items Part B creates.
   - A file's name says its ID, period end and variable. `--check` reads each of those from the name's text alone and from the file by parsing it, requires them to agree, refuses requests outside the set, and pins the template and all 21 requests by sha256.
   - `a-00`, `a-01` and `a-04` are byte-equal to the 28 Oct sitting's `reqs/stock-summary.xml`, `reqs/godown-1.xml` and `reqs/control.xml` (`5b793065`).
   - The two Stock Query requests have the shape this gateway answered on 28 Oct (`a5b369bf`, `requests/r-07`), with only the item and the period end changed.
2. **No ID or variable is guessed.**
   - The report IDs are `Stock Summary`, `Godown Summary` and `Stock Query`, each already answered by this gateway, and `Bridge Probe Unknown` as the refusal control.
   - A drill-down title (step 4) is sent only by the 28 Oct sheet's rule 2: candidate (1) the title as shown, (2) `&` as `and` with `/` dropped, (3) spaces removed. Each candidate is plain, sent once, in that order, and the first one that answers with data ends the list.
   - No variable names a godown, a group or an item for those reports, and none is added to scope a report. A drill-down answer is compared with a-00 and a-01: if its lines are the company's, it is recorded as **the default view, not scoped**, never as the drill-down's figures.
3. **Every raw send is a NEW SHAPE.** C3's screenshot of Tally's whole window is taken after each, before the next send. A modal dialog at any step: screenshot it, copy its text exactly, press NO key, send NO request, and report (C7).
   - These are **results**, not STOPs: a 0-byte 200 answer, an empty envelope (`<ENVELOPE></ENVELOPE>`), and an in-band refusal (`STATUS` 0 with a line error). C3's "empty body" STOP is set aside for them and nothing else.
4. **Marks.** `tally_status` carries no marks. M (`ALTMSTID`) and V (`ALTVCHID`) are read with `ledger_masters` (`fields` `compliance`, first page, no `snapshot_id`: basic refuses on this book, 28 Oct): `snapshot.master_alter_id`, `snapshot.voucher_alter_id`, `snapshot.reused` false.
   - Every group of raw reads, drill-down candidates included, has a marks read before and after it, and they must be equal.
   - Step 7 states exactly which mark rises at each keyed Accept, and by how much.
5. **One Tally.** At step 0 and at the end, as C2 a and b set out, plus `Get-NetTCPConnection -LocalPort 9001 -State Listen`: exactly one `tally.exe`, and it owns port 9001.
6. **Keys on reports.**
   - In a menu, use arrows, Enter and Escape only (letters typed at the Gateway act as menu keys).
   - In a report, use:
     - arrows;
     - Enter on a godown, group or item line (never on a voucher line);
     - F2 to set the period;
     - the report's own configuration (F12, or Alt+F5 for Detailed) only where a step names it, setting each changed option back and taking a screenshot of the reset before leaving;
     - on an item's monthly summary, the side bar's `Change View` button, then `Stock Query` from its list (the path the 28 Oct sitting used; the H key alone did nothing there);
     - at the Gateway, only the keys a step names (Alt+F2 and F2 in steps 7.4 and 7.6).
   - At the Godown Summary's `Select Godown` prompt, the first Escape clears the field and the second leaves it.
   - Leave a report with one Escape per level, with a screenshot after each. **Never press Ctrl+Q or Escape at the Gateway of Tally.**
7. **Keys on the master and voucher screens of step 7:** C7 holds. Never press Escape.
   - Every field not named in step 7 is left as Tally offers it.
   - A prompt other than the screen's own `Accept ?`: no key, screenshot, STOP.
   - A field that does not appear as step 7 expects (for example, no godown column in the opening allocation): no key, screenshot, STOP. Nothing is improvised on a master or voucher screen.
   - **The order after each Accept:** Tally opens a new blank creation (or voucher) screen. Take a screenshot showing its title, press ONE Ctrl+Q to leave it, take a screenshot of the screen it returns to, and only then read the marks.
   - On a blank voucher screen, Ctrl+Q left with no prompt in the 27 Oct sitting on this book (`lab/2015-goods-lines-notes-answers`). A `Quit ?` prompt anywhere: no key, screenshot, STOP.
8. **Size bound** 2,000,000 bytes per answer. The 28 Oct godown answers were under 2 KB, the largest Stock Query answer seen is 3,152 B, and the stock-items collection with seven added items was 27,118 B.
9. **The scripts run with `python -I` only**, never `-O`, which strips their checks.

Licensed mode only; Education mode = STOP.

## Build
`bridge_mcp.exe` of the 0.7.0 release, or a master build at or after `4f450ce1`. It must hold `list_companies`, `ledger_masters`, `masters` and `stock_summary`; absent from `tools/list` = STOP.
- Record its sha256 before the first call and after the last; another sha256 = STOP.
- Run it over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, a new empty `BRIDGE_AGENT_DATA_DIR` (`C:\bridge-lab\2017\data`), and `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026), with redaction unset.
- Writes stay off. Every key in Part B is pressed on Tally's own screens.

## Files
- `gen_requests.py`, `requests/` (21 requests) and `templates/stock_summary_request.utf16le.xml` (the fixture `stock_summary_report_fy_request`, byte for byte).
- `make_report_request.py`, a byte copy from `lab/2017-2018-stock-by-godown-batch` at `30e0ded5`, for the step 4 candidates only.
- `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt`.

## Before the sitting
1. Check every file here against `SHA256.txt`.
2. `python -I gen_requests.py --check`: EXPECT exactly `gen_requests: ok (21 requests, each the Stock Summary request with only the ID, the period end and at most one variable changed, as its name says)`.
3. `python -I make_report_request.py --check`: EXPECT exactly `make_report_request: ok (the Stock Summary request is reproduced byte for byte; only ID, company and dates change)`.
4. There is no request-ID census: every request is a TYPE Data report that defines no collection (`check-request-ids.py` skips them), and there is no script that sends.
5. **Backup folder** `C:\bridge-lab\2017\bk1`: absent or empty (C5).
6. **Fallback:** the backup the 28 Oct sitting of `lab/2018-stock-query-names-units-dates` took (`TDBK1800_<number>.001`, 102,253 bytes, sha256 `05f476f74e41efb87adcf53afc7a8c88823e89f22aac6d326a86cbe58ca1ca6b`, the book at 292/112; it stays on the lab PC). It is restored only on the run coordinator's word, never by this sheet's own steps.

## Steps

### Part A: the book as it stands (reads only)
0. LAB-COMMON step 0 (C2) with no company loaded. Step 1 is C2 d's MARKS read, as in the earlier rows.
   - Take the screenshot, the process list and port 9001 (rule 5), and `GET /status`.
   - On Tally's About screen, record the release (7.2 expected) and the licence. Hash the exe.
   - Load THE COMPANY alone, with arrow keys.
   - Take a screenshot of the Gateway showing the Current Period and Tally's current date. EXPECT 1-Apr-26 to 31-Mar-27 and 20-Apr-2026; record what is shown. Step 7 changes both and sets them back to what is recorded here.
1. `list_companies`: EXPECT exactly THE COMPANY, with `identity_state` `verified_tuple`; record its `guid`. Read `ledger_masters` for M0 and V0 (EXPECT 292 and 112).
2. **Tool reads.**
   - `masters` kind `godowns`: EXPECT two godowns, `Factory Floor` and `Main Location`. Record each one's name, parent and master_id (the 28 Oct sitting read 212 and 99).
   - `stock_summary` with `as_of` 20260331, then with 20250831: record each answer whole (size, sha256), `totals`, `tie_out` and `item_count_cross_check`.
3. **Raw reads, Part A.** Read the marks (EXPECT M0, V0). Send, one at a time with C3's line and a screenshot after each, saving the answer and headers, in this order:
   1. `a-00-20260331-stock-summary` (EXPECT three stock groups and the same total as step 2; record whether it is byte-equal to the 28 Oct `raw/stock-summary.xml`, sha256 `862c3206e5f22922cea932380da07d2170591085da7ef8227f4c911452bfe9a4`);
   2. `a-01-20260331-godown-summary` (record whether it is byte-equal to the 28 Oct `raw/godown-1.xml`, sha256 `92c3908ffe3916e5a7f9a86e7a4f84c912a77c926e3a1e4ab2f76f808ca67e01`);
   3. `a-02-20260331-stock-summary-explode`;
   4. `a-03-20260331-godown-summary-explode`;
   5. `a-04-20260331-control-unknown` (EXPECT `STATUS` 0 and "Could not find Report"; record whether it is byte-equal to the 28 Oct `raw/control.xml`, sha256 `09b30fb7a073de8f2373d0589e1275f7da3db9779d41e7a42e200893cb78e744`);
   6. `a-05-20250831-stock-summary`;
   7. `a-06-20250831-godown-summary`;
   8. `a-07-20250831-stock-summary-explode`;
   9. `a-08-20250831-godown-summary-explode`.

   Then read the marks (EXPECT M0, V0).

   The screens are read after the raw reads (step 4), so no screen format set on a report can be in force when these answers are taken.
4. **Tally's screens, view only**, period 1-4-2025 to 31-3-2026 set with F2, then again with 1-4-2025 to 31-8-2025. Record the period shown on leaving each report and again at the Gateway (EXPECT the Gateway still at 1-Apr-26 to 31-Mar-27: a report's F2 does not carry to it).
   - **Godown Summary** (Display More Reports > Inventory Books > Godowns > Select Godown > `Primary`, the 28 Oct path). Record:
     - every line as shown;
     - then, with Detailed (Alt+F5) and, where F12 offers it, every level expanded, every line again: each godown, the group or item under it, quantity, rate and value.
     - Set the format back, and take a screenshot of the reset.
   - Enter on `Factory Floor`. Record the title **exactly as shown** (TITLE-D1) and every line. Enter on its stock group `Raw Chemicals`; record that screen's title (TITLE-D2) and every line. Leave by Escapes.
   - **Stock Summary**, the same periods, Detailed with every level expanded: record every line, then set it back and take a screenshot of the reset.
   - **Drill-down candidates.** For each of TITLE-D1 and TITLE-D2 that is **not** exactly `Godown Summary`, `Stock Summary` or a title already sent:
     - read the marks;
     - write its candidates with `make_report_request.py --id "<candidate>" --to 20260331 --out reqs-d\<n>.xml` and send them as rule 2 says; a title the script refuses is recorded, not sent;
     - read the marks again (equal).
5. **Read each answer by structure** (C4: parse; never search the text). For each answer, record:
   - the sequence of top-level element names;
   - each line's name and its `DSPCLQTY`, `DSPCLRATE` and `DSPCLAMTA` (or the elements found instead), with empty values counted, never read as zero;
   - for an exploded answer, which lines are godowns, groups or items, as far as position and the screens of step 4 tell it, and any `SSGODOWN`, `SSBATCH` or other element not in the plain answer;
   - for a drill-down answer, its lines against the drill-down screen and against a-00 and a-01 (rule 2).

   Compare each answer with the screen of step 4 for the same report and period. Record, without judging: do the top-level amounts sum to the same period's Stock Summary total, and does any line carry an item's value in one godown?

### Part B: keyed, then restored
6. **Backup** (C5) to `C:\bridge-lab\2017\bk1`, THE COMPANY only. Record the file's name, bytes and sha256, and the time. Read the marks (EXPECT M0, V0).
7. **Key, one object at a time.** After each Accept, follow rule 7's order, then read the marks. At each Accept exactly one mark rises, by exactly one, as the sub-step says; a period or date change moves no mark; any other change = STOP.

   C8's statement of what the book holds after each sub-step: 7.1 the godown; 7.2 and the split item; 7.3 and the single item; 7.4 the current period and date changed, with nothing else; 7.5 and the Stock Journal; 7.6 the period and date set back. Step 12's restore removes all of it.
   1. **Godown** (Gateway > Create > Godown): Name `Godown Lab C`; Under `Main Location` (from the list Tally offers). Accept. EXPECT M 293, V 112.
   2. **Stock item** (Create > Stock Item): Name `Godown Split Item`; Under `Primary`; Units `Nos`; Maintain in batches `No` if asked.
      - In the Opening Balance, type the quantity `50`. The godown allocation screen must open on it, with columns Godown, Quantity, Rate, per and Amount, as the 28 Oct sitting saw.
      - Row 1: `Main Location`; type `30` over the 50 it offers; rate `10`.
      - Row 2: `Godown Lab C`; `20`; rate `12.50`. The two rates differ on purpose, so that a value per godown can show whether Tally values a godown's stock at that godown's cost or at the item's average (550.00 for 50 Nos, 11.00 per Nos).
      - EXPECT the screen to close by itself at 50 Nos, as the 28 Oct sitting saw for one row. If it does not, and the cursor is on an empty row: Enter once on it.
      - Accept. EXPECT M 294, V 112.
   3. **Stock item:** Name `Godown Single Item`; Under `Primary`; Units `Nos`; Maintain in batches `No` if asked. Opening quantity `15`, allocated as one row: `Factory Floor` `15` at `10`. Accept. EXPECT M 295, V 112.
   4. **The current period and date**, as the 27 Oct sitting on this book did before keying a voucher dated in 2025: Tally refuses a date below the current period ("Date cannot be below the current period").
      - At the Gateway, Alt+F2 Change Period: From `1-4-2025`, To `31-3-2027` (typed, whatever is shown). Screenshot. Read the marks (EXPECT unchanged: 295, 112).
      - F2 Change Current Date: `15-9-2025`. Screenshot. Read the marks (EXPECT unchanged).
   5. **Stock Journal** (Gateway > Vouchers > F10 Other Vouchers > `Stock Journal`). Its title must read Stock Journal, and its date 15-Sep-2025, before anything is typed.
      - Source (Consumption): `Godown Split Item`, godown `Main Location`, `8` Nos. Record the rate Tally offers, then type `11`.
      - Destination (Production): `Godown Split Item`, godown `Factory Floor`, `8` Nos. Record the rate Tally offers, then type `11`.
      - Narration `Lab transfer 2017`. Accept. EXPECT M 295, V 113.
   6. **Set back** the period and date, as the 27 Oct sitting did:
      - Alt+F2 Change Period: From `1-4-2026`, To `31-3-2027`. The current date may move to 1-Apr-2026 by itself; record it. Read the marks (EXPECT 295, 113).
      - F2 Change Current Date: the date step 0 recorded (20-4-2026 expected). Read the marks (EXPECT unchanged).
      - Take a screenshot of the Gateway: its period and date must equal step 0's, else STOP.
   7. `masters` kind `godowns`: EXPECT three. Record each one's name, parent and master_id; `Godown Lab C`'s parent, as written, is a result.
8. **Raw reads, Part B** (marks before and after the group, equal, at 295 and 113). Send in file order, `b-01` to `b-12`, with C3's line and a screenshot after each. Read every answer by structure as in step 5.
9. **Tally's screens** at 1-4-2025 to 31-8-2025 and at 1-4-2025 to 31-3-2026:
   - the Godown Summary and the Stock Summary, each plain and then Detailed with every level expanded, every line recorded as in step 4, each format set back with a screenshot of the reset;
   - in the Godown Summary, Enter on `Main Location`: record whether `Godown Lab C` appears under it, and the lines of each;
   - at Select Godown, record whether `Godown Lab C` is offered, and the Godown Summary for it;
   - Tally's Stock Query screen for `Godown Split Item` (Stock Summary > `Godown Split Item` > Change View > Stock Query, rule 6), with F2 on that screen to each of the two periods (it opens on the current period), and its Godown/Batch Details lines.

   EXPECT **on the Stock Query screen** (Godown/Batch Details):
   - `Godown Split Item`: at 31-8-2025, Main Location 30 and Godown Lab C 20; at 31-3-2026, Main Location 22, Godown Lab C 20 and Factory Floor 8.
   - `Godown Single Item` (the same path): Factory Floor 15 at both dates.

   A difference there is recorded and goes to the run coordinator before step 10. Other screens have no EXPECT: their lines are the results.
10. **Record, without judging,** for each period end:
    - each root item's lines per godown in the exploded answers, with any value beside them, and its value per godown against the two readings of step 7.2: the godown's own cost (Main Location 22 at 10.00 = 220.00 at 31-3-2026), or the item's average (22 × 11.00 = 242.00);
    - whether the Stock Query lines (b-05, b-06, b-11, b-12) equal the Stock Query screen of step 9;
    - whether the top-level godown amounts sum to the same period's Stock Summary total (b-01 and b-07);
    - whether `Godown Lab C` is its own line, or folded into `Main Location`, in the plain report and in the exploded report. The arithmetic that tells these apart: each godown's amount in Part B minus its amount in Part A at the same date (a-06 against b-02 at 31-8-2025, a-01 against b-08 at 31-3-2026) is the new items' value in that godown, with the wire sign (held stock negative). Main Location's difference includes Godown Lab C's value only if the parent folds in the child. At 31-8-2025 a folded Main Location differs by 550.00 under both valuation readings (300 + 250, or 330 + 220), so that date tells folding apart but not the reading; 31-3-2026, a Godown Lab C line of its own and the exploded item lines tell the reading. Under the godown's own cost the godown values at 31-3-2026 total 558.00, not the item's 550.00, so a tie that fails there is a result, not a fault;
    - how the transfer shows: the split item's lines at 31-8-2025 (before it) and 31-3-2026 (after it).
11. **Tool reads after the keying.**
    - `stock_summary` with `as_of` 20250831 and then 20260331: record each whole, with `tie_out` and `item_count_cross_check`.
    - `masters` kind `godowns` once more.
12. **Restore** (C6) from step 6's backup. Then, in order:
    - a screenshot of the Gateway: its period and date must equal step 0's;
    - `list_companies`: exactly THE COMPANY, `verified_tuple`, the same `guid` as step 1;
    - marks 292 and 112 (M0 and V0);
    - `masters` kind `godowns`: the two of step 2, with the same names, parents and master_ids;
    - `a-00-20260331-stock-summary` and `a-01-20260331-godown-summary`, sent once more, each byte-equal to its step 3 answer;
    - marks again, equal.

    Anything else = STOP. The fallback is restored only on the run coordinator's word.
13. Hash the exe. LAB-COMMON C10: Company > Shut from the Gateway's menu (never Ctrl+Q at the Gateway); Select Company with none loaded; Tally's current date as step 0 recorded; one tally.exe; 9001 LISTENING; `GET /status` 200.

## STOP rules
- C7 and C8 of LAB-COMMON.
- Any dialog (rule 3), and any prompt on a master or voucher screen other than its own `Accept ?` (rule 7).
- A second `tally.exe`.
- A mark that moves during a group of reads.
- A keyed Accept, or a period or date change, after which the marks are not exactly what step 7 says.
- An answer above its bound, or a status other than 200. An empty or refused answer is a result (rule 3).
- A master or voucher screen that is not as step 7 describes (rule 7).
- A Gateway period or date after step 7.6 or step 12 that differs from step 0's.
- A backup or restore outside C5 and C6.
- Ctrl+Q at the Gateway. That is an incident (C9): report it, restart Tally from its shortcut, shut any company that loads at startup, read the marks, and do not go on until the run coordinator says.

## What the row records, and what it does not
For each report and date, it records:
- the answer's structure and amounts, beside Tally's screen;
- the sum of the top-level godown amounts against the Stock Summary total;
- whether any answer carries a value per item per godown, in which shape, and on which valuation reading;
- whether a child godown is a line of its own, or folded into its parent;
- how a transfer between godowns shows, before and after its date.

It does not decide a read, a tool name or an estimate.

**Confidence:** one observation, PARTIAL until repeated.

**Not covered:**
- an item inside a stock group in more than one godown, through anything but Stock Query (the limit above);
- more than one level of godowns;
- a batch-kept item split over godowns;
- a compound unit;
- a costing method other than the book's own;
- another release;
- the tax-audit book.

## Evidence
- Every tool answer whole, with its size and sha256.
- Every raw request (with its sha256) and every answer as received, with headers.
- The build's sha256 before and after.
- The text of every screen recorded above, with titles exactly as shown, and the rates Tally offered in step 7.5.
- The backup's name, bytes and sha256, `restore-plan.txt`, and the directory listings C6 asks for.
- Screenshots listed by name, size and sha256, never pushed.
- A notes file with UTC times and one line per step, kept as C11 sets it out (no roles and no people).
- THE COMPANY's GUID is already in the merge gate's registry. No figure is recorded beyond the book's own synthetic ones, and the company's number is not written in the evidence.
- Evidence goes on a new branch, `lab/2017-godown-closing-answers`, with `SHA256.txt`. Send its SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes (C11).
