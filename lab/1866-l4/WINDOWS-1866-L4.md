# Windows item 1866-L4 v1: an item invoice with a round-off ledger, typed on Tally's screen and read through `gst_by_counter_ledger` (Part of #1866)

Not for merge. About one hour. `SHA256.txt` lists every other file here with its bytes; check it before step 0.

**Writes to the book: YES, to THE COMPANY only, undone at the end by restoring BK-1866L4.**
- One ledger, `GS1 Round Off`, is created on Tally's Ledger Creation screen.
- One Purchase in Item Invoice mode is typed on Tally's voucher screen, dated 7 December 2025.
- Nothing is imported. No ComplyEaze Bridge write tool is called.

## Why
- **The last item of #1866's acceptance list.** #1866 asks for an item invoice "with and without round-off" to be typed on Tally's screen and read through the tool. GS1-L1 typed two item invoices, the second with a freight ledger, and neither with a round-off ledger.
- **What the code says it must do.** A round-off ledger sits under Indirect Expenses, one of the tool's counter groups.
  - If Tally lists the purchase ledger among the invoice's ledger entries, as it did for GS1-L1's item invoices, the invoice has two counter ledgers. It is set aside as `several_counter_ledgers`.
  - If Tally does not list it, the entries do not sum to zero, and the invoice is set aside as `entries_do_not_balance`.
  - Either way, the invoice's tax must never be attributed to the round-off ledger. That is the safety property this item measures.
- **What it gives the repository:** the day's raw answers, recorded by the relay, as the fixtures of one more captured-day test (P1).

**Expected:** one of the two readings above. Any attribution of the invoice's tax to a counter ledger is a FINDING of the first rank (see "What is recorded").

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/2214-mailing-cut-drain` at 3ba77c56, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step, on the owner's standing words, and records each as C11 directs, naming the word:
- master and voucher writes on a synthetic book, typed on Tally's screen: 28 Oct 2026;
- the backup (C5);
- the restore (C6), on the run coordinator's RESTORE word for this item.

The go-ahead and any STOP ruling come from the run coordinator. The book is the one the issue 628 purchase items also use, so the coordinator places this item where none of them is open.

**THE COMPANY** is `BRIDGE GST LEDGER LAB` (synthetic), the company GS1-L1 created, with the GUID GS1-L1 step 1 recorded. It should be as item 1867-L3 left it:
- marks M 228 and V 71;
- 19 ledgers, no ledger named `GS1 Round Off`, and the stock item `GS1 Item` (unit Nos);
- in December 2025, Purchase 57, Purchase 58, Purchase 60 (optional), Purchase 59 (cancelled), Journal 5 and Payment 3, and nothing dated 7 December.

If any item has changed it since, record what the notes say and STOP. The coordinator rules.

## Differences from LAB-COMMON
- **C2 (d) and C3:** the operator sends NO request of its own. Every read is a call of ComplyEaze Bridge's own tools.
- **MARKS** means a direct call (port 9001, no relay) of `ledger_masters` with THE COMPANY's `company_guid` (first page, no `snapshot_id`), as in 1867-L3. Record `snapshot.master_alter_id`, `snapshot.voucher_alter_id` and `snapshot.reused`. `reused` must read false; true = STOP.
- **Writes.** A write is a screen accept, never a request. Before each, MARKS is read with the Gateway shown and no screen open; then only the keys that open the step's screen are pressed, and nothing is typed but the step's entries before the accept. After each: MARKS.
- **Tool answers.** An answer above 1,000,000 bytes = STOP. So is any call that answers `request_deadline_exceeded` or `response_size_limit_exceeded`, or gives no answer within 10 minutes: nothing is sent again, no key is pressed, a screenshot is taken and the answer kept whole.

## The build, the relay and the calls
- **The build** is the `bridge_mcp` program the lab config's line `bridge-lab-gs1` runs: GS1-L1's build of pull request 1867 at head `796e94f18e5af783b291ddf3b5e60ec19132df9a`, `bridge_mcp.exe` sha256 `bf8cdf82e10bf22f693d3c5d6d89bd074b35b2dbf6dc2cbad748ac8609916f46`. Hash the exe before the first call; another hash = STOP.
  - It is the build of every capture this tool's tests replay (`gst-ledger-l1`, and #2220's `gst-ledger-l3`), and its decision for this voucher is the merged tool's.
  - Posting and ledger creation stay OFF. Exactly one ComplyEaze Bridge is connected, under that server name. Calls go one at a time, with exactly the arguments the step gives. Each answer is kept whole as a file in the evidence folder, named by step, with its size and sha256. A call in flight is never cancelled, abandoned or repeated.
- **The relay:** relayed calls go through `lab/gs1-relay` at `e5dffc2b`, checked against its SHA256.txt and self-tested as its README says. It records into `C:\bridge-lab\1866-l4\relay\` (a new folder).
  - Start the relay, set the `bridge-lab-gs1` line's Tally port to 9011, and restart that server. After step 7, Ctrl+C the relay, set the port back to 9001 and restart the server.
  - A request the relay refuses means the call fails = STOP, with the folder kept.

## Keying
- **Screens.** Every master and voucher is typed on Tally's own screen. The invoice of step 5 is keyed in **Item Invoice** mode, not As Voucher: reach it with Ctrl+H (Change Mode) on the Purchase voucher screen, as GS1-L1 and 628-C0 recorded. Item lines first, then the three ledger lines. The party's amount and the total are Tally's: they are checked against the step, never typed.
- **Sub-forms.** This is the first item invoice on this book since it was registered. The Receipt Details and Party Details sub-forms that 628-C0 met for a registered supplier are accepted as offered, left blank where 628-C0 left them blank, with a screenshot of each. Any other sub-form or prompt (a GST Details or HSN prompt among them), or a warning that the reference "has been used earlier", is C7: press no key.
- **Before every accept** (Ctrl+A), take a screenshot of the whole screen.
- **Leaving without saving:** Ctrl+Q, only after a screenshot shows the screen's title. Never Escape on a master or voucher screen, and never Ctrl+Q or Escape at the Gateway (C7).
- **Back to the Gateway after each Ctrl+A** (steps 4 and 5): take a screenshot, press Escape once and take a screenshot, then press Escape once more and take a screenshot, which must show the Gateway. On the 1688 row, Tally went from the list after the accept to Master Alteration's List of Masters on the first Escape, and reached the Gateway only on the second. If a screenshot already shows the Gateway, press nothing more: never Escape at the Gateway itself. A screen other than these = STOP.
- **If Tally fills a tax or round-off amount by itself,** that is, any non-empty amount on a tax or round-off line the operator has not typed (a line that opens empty or as 0.00 is not one): accept nothing. Take a screenshot, leave the voucher unsaved with Ctrl+Q, record "Tally computed an amount", key nothing further, and go to step 9. GS1-L1's item invoices opened their tax lines empty.
- **The only prompts answered** are each screen's own accept, and the two prompts C6 quotes. Any other box is C7.

## Folders
The working folder is `C:\bridge-lab\1866-l4\`, with evidence in `C:\bridge-lab\1866-l4\evidence\` and BK-1866L4 in the new empty folder `C:\bridge-lab\1866-l4\bk\`.

## Steps
0. **Start.**
   - Load THE COMPANY alone by Select Company (C1), then LAB-COMMON step 0 without its MARKS request.
   - Hash every file of this folder against `SHA256.txt`.
1. **Look only:** by Display, never Alter, take a screenshot of each.
   - The Day Book for December 2025: EXPECT exactly 1867-L3's six vouchers, and nothing on 7 December.
   - The List of Ledgers under Indirect Expenses: EXPECT no `GS1 Round Off`.
   - Anything else = STOP.
2. **MARKS** -> `ans-1866l4-marks-0.json`. EXPECT M 228 and V 71. Another value = STOP, and the notes say what changed the book.
3. **Backup** BK-1866L4 by C5. Then MARKS (ms0).
4. **The ledger.** On Tally's Ledger Creation screen, create `GS1 Round Off`, group Indirect Expenses.
   - Leave every other field as the screen opens it. If the screen offers a "Type of Ledger", leave it at its default; if that default is not "Not Applicable", screenshot it, leave the screen unsaved with Ctrl+Q after a screenshot of its title, read MARKS (EXPECT M 228), and STOP. A ledger typed for invoice rounding would let Tally fill the amount itself.
   - Take a screenshot, then Ctrl+A, then back to the Gateway as Keying says.
   - Then MARKS: EXPECT M +1 and V unchanged.
5. **The invoice.** On Tally's Purchase voucher screen, in Item Invoice mode, dated **7 December 2025**:
   - **Party:** `SYN GS1 Supplier B`, supplier invoice number `GS1/P/0005`, dated 7 December 2025.
   - **Item:** `GS1 Item`, 1 Nos at 4,999.60, purchase ledger `Purchase - Goods` (4,999.60).
   - **Tax:** `Input CGST` 449.96 and `Input SGST` 449.96.
   - **Round-off:** `GS1 Round Off`, 0.48 typed in the Amount column, which adds to the total. Leave Rate (%) empty: GS1-L1's freight line, typed in Rate (%) first, was computed by Tally. A minus amount would lower the total.
   - **Total:** Tally's total must read 5,900.00, the party's Cr, with one bill allocation, New Ref `GS1/P/0005`, 5,900.00. If Tally's default for the allocation is Agst Ref or On Account, change it to New Ref, as GS1-L1 did.
   - If the voucher screen shows any other total, or will not balance: screenshot, leave it unsaved with Ctrl+Q, and STOP.
   - Take a screenshot, then Ctrl+A, then back to the Gateway as Keying says. Then MARKS: EXPECT V +1 and M unchanged.
6. **Look only.** Take a screenshot of the Day Book for 7 December 2025: EXPECT the one Purchase.
7. **Relay on** (as above).
   - **7a.** Call `list_companies`. EXPECT THE COMPANY with the GUID GS1-L1 recorded.
   - **7b.** Call `gst_by_counter_ledger` with `company_guid`, `from` 2025-12-07 and `to` 2025-12-07 -> `ans-1866l4-day.json`. A refusal is a result: record its code and go on.
   - **7c.** Call `gst_by_counter_ledger` with `company_guid`, `from` 2025-12-01 and `to` 2025-12-31 -> `ans-1866l4-dec.json`. A refusal is a result: record its code and go on.
   - Relay off, port back to 9001, server restarted.
8. **MARKS** -> `ans-1866l4-marks-1.json`. EXPECT both marks equal to step 5's: the calls wrote nothing. Any change is a FINDING of the first rank: nothing more is sent, and step 9's restore follows on the RESTORE word.
9. **Restore BK-1866L4 by C6,** on the run coordinator's RESTORE word. Then:
   - MARKS: EXPECT ms0;
   - by Display, the Day Book for December 2025: EXPECT step 1's six vouchers and nothing on 7 December;
   - the List of Ledgers under Indirect Expenses: EXPECT no `GS1 Round Off`, and the List of Ledgers: 19 ledgers.

   Anything else = STOP: the book was not restored.
10. **The date, then C10.** A voucher dated 7 December may move Tally's current date (GS1-L1 saw it move). Before the shut, set the Gateway's current date back to the one step 0 recorded (8-Dec-2025 by 1867-L3's notes) with F2, never Escape or Ctrl+Q, and take a screenshot. Then C10.

STOP RULES:
- C7 and C8;
- any dialog but the accepts and C6's prompts;
- a second `tally.exe`;
- a mark that moves other than as a step says;
- an exe hash that is not the one named;
- a call answering `request_deadline_exceeded` or `response_size_limit_exceeded`, or giving no answer within 10 minutes;
- a request the relay refuses;
- step 1, 2 or 9's check;
- Tally computing an amount (go to step 9);
- Ctrl+Q or Escape at the Gateway (an incident: press no key, take a screenshot, STOP).

## What is recorded (nothing is judged at the lab)
- **The relay's recording folder,** whole, with `manifest.tsv` and its SHA256 list: every answer of step 7.
- **The tool answers whole,** the three MARKS answers, and the notes file (UTC, one line per step).
- **Screenshots,** listed by name, size and sha256, never pushed.

**The requester reads the recorded day:**

| reading of 7b | meaning |
|---|---|
| `set_aside.several_counter_ledgers`, 1 voucher, `debit_side_tax` `cgst` −449.96 and `sgst_utgst` −449.96 | Tally lists the purchase ledger among an item invoice's entries, so the round-off ledger makes two counter ledgers; the tax stays set aside. |
| `set_aside.entries_do_not_balance`, 1 voucher | Tally keeps this invoice's purchase ledger only in its inventory allocation; the tax stays set aside. |
| the invoice's tax attributed to `GS1 Round Off` or to `Purchase - Goods` | a FINDING of the first rank: report it at once, with the answers. |
| anything else | record it whole and infer nothing. |

7c shows December's other vouchers read as 1867-L3 recorded them, with this one added. EXPECT `vouchers_observed` 7; `counter_ledgers` as 1867-L3 (Purchase - Goods 2 vouchers at −1800 per head, GS1 Rent 1 at −450 per head); one voucher set aside, as in 7b; `vouchers_cancelled_optional_or_post_dated` 2; `vouchers_without_gst_entry` 1. A difference is recorded, not judged.

**Confidence:** one voucher on one synthetic book, one release (TallyPrime 7.2 Silver), so PARTIAL.

**Not covered:**
- a round-off ledger typed for invoice rounding, whose amount Tally computes;
- a round-off on the credit side;
- item invoices of other shapes;
- Gold.

## Evidence
Everything but the screenshots and the backup goes on a new branch, `lab/1866-l4-answers`, with `SHA256.txt` over the committed files. The data is synthetic. Send its head to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.
