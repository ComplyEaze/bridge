# Lab order for 0.7.0, no-owner re-cut (the sheet preparer, 27 Oct 2026 16:04 IST). Runs after the 0.6.1 post-publish check.

Replaces  Heads come from ls-remote at 15:55 IST, and every head is the full SHA on origin. Order per the coordinator's instruction of 27 Oct: contributor-unblocking rows first, then inventory parity, then #1423, then #1738, then #1867 L3 with 628-C0/C0b ahead of it, then the rest, and #1713 O1 last.

## WHO ACTS: the owner's standing word of 27 Oct 2026 (the sheets stay as they are on origin)
- The owner confirmed this to the lab directly. On a synthetic company, the lab itself does every step a sheet marks [OWNER] or "OWNER STEP" or says "the owner keys / approves / restores": it keys masters and vouchers, answers ComplyEaze Bridge's approval window, takes backups and restores, and edits the row's launch file for posting.
- A sheet's "the lab asks the owner in its own chat ... until the owner says yes" is met by that standing word. The lab asks nobody.
- The lab records each such step in its notes as "done by the lab on the owner's standing word of 27 Oct".
- The run coordinator's word, where a sheet asks for it (go-ahead, RESTORE, the evidence push), still comes from the run coordinator.
- The column "screen steps the sheet gives the owner" lists where those lines are, so the lab can find them.
- Three rows carry changes that override their sheet. Each is set out under its row number in "Row notes" below: row 7 (#1423 control 2a skipped), row 12 (#1317 timing rule) and row 18 (#683 X1 toggle kept).

## Builds already staged at the lab (exe sha256; hash before the first call and at the end)
| name | commit | `bridge_mcp.exe` sha256 |
|---|---|---|
| CUT (0.6.1) | 67f647e1a292e8d7815053b5c08289cba6402795, run 40456677783, artifact 12712717489 | 6f520338b7f9a25180977c40a5f1d9b03dfa5774df5b5d9a6e65b9c2a050728d |
| FD6 | fd6f7c0d032ec51f9d9de9c80e8bcd33d930ad77, run 40362694215, artifact 12668144368 | 63641231787492201133443cb1a887705c7137f495a9205ffd33c2d0068f7484 |
| 609 | 609776ae9f5cec48e6563f165480d3fe1adae445, run 40366984202, artifact 12670984633 (expires 2 Nov 17:27 UTC) | 4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44 |
| W1-2306 (`w1-2306d86d`) | 2306d86dfd4c86a3c5f6c19ea9ddff6818c460fb, run 40077056130, artifact 12553782422 | 92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524 |
| gs1 (config line `bridge-lab-gs1`) | #1867 head 796e94f18e5af783b291ddf3b5e60ec19132df9a, run 40167319272, artifact 12589749758 | bf8cdf82e10bf22f693d3c5d6d89bd074b35b2dbf6dc2cbad748ac8609916f46 |
"curl" means the row's reads are the committed request files sent by LAB-COMMON C3's line, and the sheet uses no build except where it names one.

## Done since the sheet preparer's order (shape PASS on record)
#1392 S2c (226c53b3), #899 part 2 book O (6c8ff7c7), #2027 (f9c94b0b), rd21 part B (7fe6cf57: B1-B5; B6 not run because the sheet names no ledgers for it; B4's sheet expectation was wrong, and the book is at its baseline), #485 part T (a63b57f9), #1233 mid-year (ba3b55cf). 0.6.1 rows: W1 b486f2f4, 724 N1 c437ee21, R3 aeae741b.

## The order
| # | row | branch @ full head | company | build | screen steps the sheet gives the owner (the lab does them: WHO ACTS) |
|---|---|---|---|---|---|
| 1 | **#1709 first-voucher captures** (first-voucher L3; feeds the empty-book first post; the contributor's #1709 text slices need no lab) | lab/1709-first-voucher-captures @ 576a08afad2fcf98697c1acab235145fb9fb06db | BRIDGE FIRST VOUCHER LAB (new) | gs1; reads only, posting and ledger create OFF | lab/gs2-l3/WINDOWS-first-voucher L3.md: header ("created by the owner"), C1, "OWNER STEPS", steps 1-2 "OWNER" (create company, two ledgers) |
| 2 | **#1233 nested-group capture** (owner decision 11 YES rests on it) | lab/1233-nested-groups-capture @ 23c676dda4ba99027304d84adfe7d83c400ee710 | BRIDGE NESTED STOCK LAB (new) | 609 + curl reads n1-n3 | none: company "by whoever the run coordinator names", masters by the operator after the coordinator's yes |
| 3 | **#692 two-year window** | lab/692-two-year-window @ 006c2cd2f2906bafe1aaacb5d7ee885ca4e8a3dd | BRIDGE TIEOUT B | FD6 or 609 (same statement code) | none: operator keys J0/J1 after the coordinator's yes |
| 4 | **#692 D3** (five experiments X1-X5, BK3 restore between them; real-book conditions, not on/off isolation) | lab/692-opening-stock-d3-requests @ 6f4f7869909bbff7081ac29f6faa3ebdc3eebfdb | BRIDGE TIEOUT D (also rd21's book: available now) | curl | README line 9 "The owner's yes before keying"; line 17 "your own keys" |
| 5 | **#692 NONINT** | lab/692-nonint-lab-requests @ f22e68d9a482be557445eb6744c696cbadba42b9 | BRIDGE NONINT LAB (new) | curl | README line 9 "The owner's yes before keying" |
| 6 | **#2015 goods lines of keyed notes** (unblocks draft #2038) | lab/2015-goods-lines-notes @ 6e3744804d8bbff83003248dc27a79a2ce667948 | BRIDGE SHAPE LAB | FD6 (marks and list only) + curl a07 | none: operator keys K1-K3 after the coordinator's yes |
| 7 | **#1423 duty-head mark** (same book as row 6: run 6 then 7, each with its own BK0/restore) | lab/1423-duty-head-mark @ b4427246463104711164f44e254c13fb1112e544 | BRIDGE SHAPE LAB | 609 | none (operator after the coordinator's yes). SKIP step 2a: see Row notes, row 7 |
| 8 | **#1738 S1 abandon/rebuild** | lab/1738-s1-abandon-rebuild @ 4dd8fb8de5bc8c609c694f2644996b97c25ae8d7 | BRIDGE REBUILD LAB (new, five ledgers + seed Journal) | CUT, posting ON | README line 26 "the owner answers the approval box"; line 3 "with the person's approval" |
| 9 | **628-C0 then 628-C0b** (current sheets v1.3; NOT lab/628-purchase-c0-run-sheet db22b1c5, which is the stale v1) | lab/1904-purchase-identity-capture @ e1e579c0b6ede0162e91d644bbecacae8f82d96c | BRIDGE GST LEDGER LAB | gs1 + curl (C0: 2 requests; C0b: 4) | lab/628-c0: header "from the owner", "OWNER STEPS ... by the owner himself", steps 1b, 4, 5 "OWNER". lab/628-c0b: "OWNER STEPS ... by the owner himself", Grant line, steps 4-8 "OWNER" (K1-K5, incl. K3 cancel) |
| 10 | **#1867 L3** Dec + year (needs 628-C0b's close) | lab/1867-registered-book-reads @ 37afa39e57a403fc5f2a219047f58f0d59281675 | BRIDGE GST LEDGER LAB | gs1, reads only | none |
| 11 | **#1356 party term + Ledger Outstandings** | lab/1356-party-term-and-ledger-outstandings-run-sheet @ 1f97cdef8d0f69c0d087847de90d057e4cb58dda | BRIDGE BILLTRAIL LAB | curl | RUN-SHEET "OWNER steps are marked [OWNER]"; step 1 "[OWNER] Tally is LICENSED"; before-the-sitting step 3 "[OWNER] Go-ahead" (the run coordinator's); Steps step 3 "[OWNER] ... asks the owner ... whether to key" |
| 12 | **#1317 request gap** (four driver runs) | lab/1317-request-gap-run-sheet @ 5167f849604b55d606874d68150007b7150e7c1a | BRIDGE CORPUS DENSE | curl driver | Title "(the owner at the Tally screen)"; THE SCREEN; steps 1, 4 [OWNER]; "Between runs ... the owner back at the Gateway"; RESPONSE-FORM "the owner fills this in". The lab fills the form by the TIMING RULE in Row notes, row 12, not by eye |
| 13 | **#485 part D (bounds fix) then part E**, same DENSE load, straight after row 12 | lab/485-ledger-filtered-vouchers-run-sheet @ 4e769a9c9dc1eaf875e1bf397c0c4859510a90ab; lab/485-ledger-census-spans @ 97fdc0635a436c28fa5811cc5f47619eaf3f4a13 | BRIDGE CORPUS DENSE | curl | D: "OWNER steps ... The owner is AT THE LAB MACHINE for every NEW SHAPE" (a modal dialog "needs a person"), steps 1, 3 [OWNER], step 5 "a gateway that stops answering is for the owner to restart". E: WHEN "with the owner still at the machine", step 4 [OWNER] |
| 14 | **#1231 reconciled bank entry** (TRANSFER LAB sitting is done, so it is available) | lab/1231-bank-reconciliation-run-sheet @ 4a36e7e592d5710765efc3f9d4415ffd9f6ca863 | BRIDGE TRANSFER LAB | W1-2306 + curl V-CONTRA | none found |
| 15 | **#1231 keyed cost-centre allocations** | lab/1231-cost-centre-keyed-allocation @ 23bca8b913f75e79ef5129e631e1ffac89e2faa7 | BRIDGE COST KEY LAB (new) | 609 + curl | RUN-SHEET WHO ACTS "the lab asks the owner in its own chat"; step 1 "OWNER'S YES first" |
| 16 | **#1633 span-mark questions** (deletes, GSTR-1 export) | lab/1633-span-mark-questions @ 0c0ed340e77f3e0fb0cbb9bb71190e5ce5d1c349 | BRIDGE SPAN MARK LAB (new; GSTIN 08ZZZZZ0000Z1ZQ) | gs1, posting OFF | lab/1633-l4/WINDOWS-1633-L4.md: header, C1, "OWNER STEPS", steps 1-12 "OWNER" |
| 17 | **#1039 G1 row A** (rows B/C STOP on Silver: they need Gold and a second client) | lab/1039-live-rows @ 85cb14b74bfa10c96bf885a1febc6addc03762e0 | BRIDGE CONCUR LAB (new) | CUT (its build line's first choice, now filled), else FD6, else 609; posting ON | G1: RULES "the owner answers the approval window by mouse"; A2 "(owner approves)"; A3 "OWNER STEP" (edit narration); B1, B2, AFTER "OWNER STEP"; README "owner steps marked" |
| 18 | **#683 X1 forex digit grouping** | lab/sitting3b-run-sheets @ dbf5b2d9fa059afdee8ab67e6795b19507f13167 | BRIDGE FOREXGROUP LAB (new) | W1-2306, posting OFF | X1: "KEYING (OWNER STEPS ...)", steps 1, 3, 4, 5, AFTER "OWNER STEP"; README "marks every screen action as an OWNER STEP". Steps 3-5 STAY: see Row notes, row 18 |
| 19 | **#1713 O1 abandoned ledger create, LAST** (holds the gateway about 4 h) | lab/1713-abandoned-ledger-create @ f86d4f1a3b34c18b44a38223ef6c7c83cb2b7e06 | BRIDGE ABANDON LAB (new) | none (scripts, curl writes) | README lines 10, 134, 230: "the owner's yes ... recorded in notes.txt before S1" |
| 21 | **#1039 refusal reproduction** (one refused span binding on the Silver book; runs straight after row 17's G1 row A, before row 19) | lab/1039-refusal-row @ 08bb7491d9eef797d9975fdca46630ff1b71cbb1 | BRIDGE CONCUR LAB (row 17's company, as row A left it) | the build row 17 ran on | R1-1039-refusal.md and README: the steps marked [OWNER] follow WHO ACTS above. See Row notes, row 21 |

## Not runnable, so not in the order
- lab/1738-remove-duplicate-contra 7bb106f1: its cancel/delete rows are P7-blocked until there is a candidate cancel/delete builder (contributor issue draft , for the coordinator to file Wednesday).
- lab/1650-ledger-create-box-row b7f69670 pins build 6f7e98bc, but its branch is at 3578917d.
- lab/1761-s1-cash-day-end-run-sheet bef604bb has never been preflighted.
- lab/626-2b 82c73845 (hand-edited requests, P7) and A2/A3 lab/0.7.0-tonight-run-sheets 7622e10f: parked. #1108: retired.
- rd21 B6: the sheet names no ledgers. rd21 B4's two-dated item needs its own keyed step, and there is no sheet for it.
- lab/1867-client-book-set-aside-share 68a1ec46 needs a real book, so it sits outside this synthetic order.

## Row notes (these override the sheet where they differ; the lab records each as a deviation in its notes)
None of these 0.7.0 sheets toggles a setting only to isolate a variable. Rows 7 and 18 were the two near cases, and they are ruled here.

**Row 7, #1423 control 2a: SKIP, because it is not the thing measured.**
- What the row decides: whether a duty-head change moves the master mark, so that `ledger_snapshot_drifted` can rely on the mark.
- A duty-head change is always saved, so the move D1-D4 record is the answer whatever its cause. Step 2a, a save with no change, only tells "moved by saving" apart from "moved by the change", and neither the product nor the decision needs that split.
- So the lab skips step 2a and takes no `m1a`. In step 3 it records **M2 − M1** in place of M2 − M1a.
- Note in `notes.txt`: "2a skipped: a no-change save is not what #1423 measures (owner's word of 27 Oct, no isolation steps)".

**Row 18, #683 X1 digit grouping: STEPS 3-5 STAY, because the toggle is the thing measured.**
- The row asks what `trial_balance` and `outstandings` return when the digit-grouping setting changes. Its FAIL line is "a different code for the same book at the two settings". The two settings are the experiment, not a control around it.
- Step 5 also puts the setting back. It may be a Tally-wide setting rather than a company one, so the closing restore of BK1 might not undo it.

**Row 12, #1317: TIMING RULE (the lab measures screen lag in place of a person's judgement).** The driver runs as written. The lab does RESPONSE-FORM's eight actions itself through its screen tool, and times each one.
1. **Clock.** One clock for everything, read as UTC with milliseconds (for example PowerShell `(Get-Date).ToUniversalTime().ToString('HH:mm:ss.fff')`). The lab writes t0 immediately before sending the action's key, and the time of every screenshot taken after it.
2. **Done means seen.** After the key, the lab takes screenshots back to back, nothing else in between, until the screen shows the action's end state. t1 is the time of the first screenshot that shows it, and the latency is t1 − t0.
   - Actions 1, 4, 6: the report's title (Day Book, Balance Sheet, Profit & Loss A/c) and its first rows.
   - Actions 3, 5, 7: the Gateway of Tally.
   - Action 8: the Trial Balance title, then, timed separately as 8b, the Gateway.
   - Action 2: t0 at the first Page Down. t1 at the first screenshot after the fifth key whose top row differs from the screen before the keys.
3. **Resolution.** In the baseline pass, the lab records R, the median time between two back-to-back screenshots. No latency is finer than R, and the form says so.
4. **Baseline pass B0**, before run A1, with the driver NOT running and nothing sent: the eight actions once, timed the same way, giving the latency base(a) for each action. It costs no owner time.
   - It is a reference, not an on/off toggle. A latency through a screen tool includes the tool's own time, and only the difference from B0 is Tally's lag.
   - Its column is added to the form as "B0 (idle)".
5. **In each run**, the actions start once the first request has gone, as the form says. For each action, excess E = latency − base(a). The cell gets:
   - **I** if E ≤ max(0.5 s, R);
   - **L** if E is between that and 3 s;
   - **F** if E > 3 s, with E in seconds.
   - Every cell also holds t0, t1, the latency and E, so the class can be checked from the numbers.
6. **STOP** if any action has still not reached its end state 10 s after t0. The lab creates the `STOP` file at once, presses no key, takes a screenshot and reports. This is the form's freeze over 10 s.
7. **Between runs**, at least two minutes with no request and the screen at the Gateway. This replaces "the owner back at the Gateway".
8. **Evidence.** The filled form, with B0 and the four runs, each cell's numbers and R. The screenshot times stay in `notes.txt`, and the screenshots stay on the lab PC, listed by name, size and sha256.
9. **Reading it (for the requester, not the lab).** The screen counts as responsive at 250 ms if no B run has an F, and the B runs' L count is not above the A runs'.

**Row 21, #1039 refusal reproduction: runs after row 17's G1 row A, on the same company and build.**
- The sheet is `lab/1039-refusal-row` at 08bb7491d9eef797d9975fdca46630ff1b71cbb1 (one row, `R1-1039-refusal.md`). It starts only after row 17's step A4 is recorded. If row 17 did not run, or THE COMPANY of row 17 is not on the PC, the row does not run and nothing is created in its place.
- Deviation D1 of the sheet (sends made while one unsaved voucher sits on Tally's screen) has the run coordinator's recorded yes, in the coordinator's log of 28 Oct 2026. The lab records in `notes.txt`: "D1 used with the run coordinator's recorded yes of 28 Oct".
- The sheet defers row 17's AFTER restore and its C10 until the #1039 dialog row has finished: the refused batch, its data folder and the book are that row's input. The lab does not restore or shut row 17's company after row A; it does so only when the run coordinator says RESTORE, after the dialog row.
- Row 19 (the #1713 row) stays last: the gateway is held for about four hours there.

**Row 22, #1709 settle row S1 (added 29 Oct 2026): runs on its own data folder and build, after the rows above.**
- The sheet is `lab/1709-settle-row` at bc6b5ea8e4484fa8ae5a27dbbcec30fc4b35577c (one row, `S1-1709-settle.md`, with its `LAB-COMMON.md` v3 copy).
- The build: #2150 at head ed0f5fc5, Bridge CI run 40834023243, artifact 12880706328 (`unsigned-bundle-smoke-windows-latest`), `bridge_mcp.exe` sha256 e95269fb9ab2d72676902891615d733e2cb13e12cc003b483afbc0c0110d9a58.
- The owner's go-ahead is recorded twice: in the run coordinator's chat on 29 Oct 2026, "yes download the relevant builds on lab; yes on ci bundle merge gate", and in the lab's own chat the same day, "Yes, download #2150's unsigned-bundle-smoke-windows-latest artifact from its CI run at head ed0f5fc5 and run #1709-settle."
- THE COMPANY: BRIDGE SHAPE LAB (books from 1 April 2025), whose ledgers are listed in `lab/2018-batch-report-routes-answers` at e56a7305, answer `001-s1-ledger_masters.json`.
- L1 = `Discount Allowed` (Indirect Expenses), L2 = `Capital A/c` (Capital Account), as that answer names them; neither is a party, bank or cash ledger.
- The date for steps 2.1 and 2.2: 15 June 2025. At step 0 the lab reads `vouchers` for that day; if a Journal is already on it, the lab uses the next day with none and records which.
- Not established: whether L1 or L2 is bill-wise (the build refuses a bill-wise ledger it did not approve, which would show at step 2.1, a STOP).
