# 2016 stock movement (#2016): the candidate stock movement request on every kind of stock voucher, the item totals against Tally, a Delivery Note alone, nested-only items, and the voucher types of stock vouchers

Not for merge. About 5 hours. This sheet WRITES by hand on Tally's screens (two stock items, one voucher type, up to three voucher type activations, and the vouchers listed below), reads, then restores the book from one backup. ComplyEaze Bridge only reads. `SHA256.txt` lists every file here with its bytes; check it before step 0.

`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy of the one in `lab/0.7.0-rc-sitting`, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds. It runs alone: it shares no state with any other sheet, and it restores its own backup.

## What it decides

#2016 asks for one read of every stock movement in a window. Captures so far (28 Oct 2026, this book, TallyPrime 7.2) hold one Stock Journal, one Credit Note, one Delivery Note and one Receipt Note, read through the shipped vouchers request (a07) and through the same request with `ALLINVENTORYENTRIES.*` (a08). The read to be built sends a08 **with the voucher-type class computes** after its FETCH. That exact request has never been sent. This sheet:
- **R1:**
  - sends that request, and its AlterID-span form, over a month holding every kind of stock voucher the read is to cover: Stock Journals of three shapes, a Sales and a Purchase with the item, two returns, an optional, a cancelled and a post-dated voucher;
  - compares the item's totals with Tally's own screens and with `stock_summary`;
  - isolates one Delivery Note on an item of its own, to decide whether a note moves the Stock Summary.
- **R2:** keys an accounting-mode Purchase with an inventory allocation, an item-less control, and a Journal with an inventory allocation, to find whether anything in the a08 answer marks items nested only under a ledger entry.
- **R3:** keys a Delivery Note, a Receipt Note, a Physical Stock and a Manufacturing Journal, to record each one's voucher-type class and reserved name in that request.

**Limit, whatever the result:** one book, one release, one voucher per shape.

## Rules and company

**Who acts.** On the owner's standing word of 27 and 28 Oct 2026, the lab does every step here, including the keying, on this synthetic company, and records each as LAB-COMMON C11 directs. The final restore waits for the run coordinator's word RESTORE.

**THE COMPANY** is `BRIDGE SHAPE LAB` (synthetic), GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, as the 28 Oct 2026 rows left it after their restores:
- marks `ALTMSTID` 292 and `ALTVCHID` 112;
- two godowns, `Main Location` and `Factory Floor`;
- the item `Hydrochloric Acid 33pc` in batches `H-2504-A` and `H-2504-B`;
- the company period 1 April 2026 to 31 March 2027, and the current date 20-Apr-2026;
- Delivery Note and Receipt Note voucher types inactive.

If it is absent, or step 1's marks differ: STOP and report (the run coordinator decides before step 2). Only THE COMPANY is loaded, alone, from step 0 to the last step. Licensed mode only; Education mode = STOP.

Differences from LAB-COMMON (the stricter rule holds everywhere else):
1. **Two kinds of read.**
   - **Tool reads** are calls of ComplyEaze Bridge's built tool, one at a time.
   - **Raw reads** by curl are allowed by the owner's decision of 26 Oct 2026 for shape discovery. Each is a file in `requests/`, written by `gen_requests.py` from the committed templates in `templates/`, and sent as C3 sets out.
   - `python -I gen_requests.py --check` must print its ok line before step 0. It proves that each file is its template with only the company, the day, the class COMPUTE block and (for the one span read) the AlterID filter changed. No other raw request is sent.
   - Every raw send of `r-m-*` and `r-span-*` is a NEW SHAPE (C3): a screenshot of Tally's whole window after each, before the next send. A modal dialog: screenshot it, copy its text exactly, press NO key, send NO request, and report (C7).
   - Each answer is under 2,000,000 bytes, else STOP.
   - An empty envelope, a 0-byte 200 answer or an in-band refusal (`STATUS` 0 with a line error) is a **result**, recorded whole; C3's "empty body" STOP is set aside for these and nothing else.
2. **Marks.** `tally_status` carries no marks. M (`ALTMSTID`) and V (`ALTVCHID`) are read with `ledger_masters` with `fields` `compliance`, first page, no `snapshot_id` (basic refuses on this book: `company_several_currency_masters`, 28 Oct): `snapshot.master_alter_id`, `snapshot.voucher_alter_id`, `snapshot.reused` false.
   - Each voucher Accept raises V by exactly 1, and so does each cancellation.
   - Each master created raises M by exactly 1, and each voucher type activated raises M by exactly 2 (28 Oct: 292, 294, 296).
   - A change of period or current date moves no mark.
   - Any other change = STOP.
   - Every group of raw reads has a marks read before and after it, and they must be equal.
3. **Backup and restore.** BK1 of THE COMPANY alone before the first change (C5), into a new empty folder `C:\bridge-lab\2016-sm\backups\bk1\`. At the end, on the word RESTORE, BK1 is restored by C6, and the book is checked against step 1. BK1 is the only backup; a restore that does not complete is a STOP.
4. **Keys.**
   - Accept a master or voucher with Ctrl+A. After it, if a creation or voucher screen is showing, leave it with ONE Ctrl+Q, and only after a screenshot shows its title.
   - **Never Ctrl+Q or Escape at the Gateway of Tally**, never two Ctrl+Q in one call, never Escape on a master or voucher screen (C7).
   - On reports, one Escape per level, with a screenshot after each.
   - In a menu, arrows, Enter and Escape only (letters typed at the Gateway act as menu keys).
   - Where a step uses a screen's own option (Optional, Post-Dated, Cancel), it is chosen from that screen's right-hand button list. The key Tally shows for it is recorded, and a screenshot shows it set before the Accept.
5. **Dates.** Tally refuses a date below the current period. Every voucher here is dated inside 1 April 2026 to 31 March 2027. Setting a voucher's date may move Tally's current date (28 Oct: it followed the voucher); record it. Step 14 sets the current date back to step 0's before the restore.
6. **One Tally.** At step 0 and at the end, as C2 a and b set out, plus `Get-NetTCPConnection -LocalPort 9001 -State Listen`: exactly one `tally.exe`, and it owns port 9001.
7. **Ledgers named from Tally's own lists.** Where a step says "the first … the list offers", the lab takes the first entry of the list Tally opens at that field, records its exact name, and uses that name everywhere after. The 28 Oct rows used `Outstation Buyer 1`, `Sales - Exempt` and `Chem Supplier 1`. EXPECT the same names offered here; a different name is recorded, not a STOP.

## Build

The published release `mcp-v0.7.0` Windows package, as the 30 October post-publish check staged it on the lab PC:
- `bridge-tally-mcp-v0.7.0-windows-x64.mcpb`, sha256 `291a4ed95c758fad9fa04cd21a66be889e90a2fd03960fbceeec2c2a7242d4ec`;
- its `bridge_mcp.exe`, sha256 `48d2f7430b26da0d8f670eb74a27df46b99b7d45e05e102504482ddf24c8ba2d`.

Record the exe's sha256 before the first call and after the last; another hash = STOP. One launch over stdio for the whole item:
- `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`;
- ONE new empty `BRIDGE_AGENT_DATA_DIR`;
- `BRIDGE_TERMS_ACCEPTED=true`;
- writes, import, batch post and redaction unset.

It holds `ledger_masters`, `list_companies` and `stock_summary` (month-end `as_of`); one absent from `tools/list` = STOP.

## Files

- `gen_requests.py` and `templates/` (three committed requests);
- `requests/` (the 18 generated reads);
- `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt`.

## Steps

### Step 0 and the book as it stands

0. LAB-COMMON C2 step 0 with THE COMPANY loaded alone (its MARKS read by rule 2). Record Tally's release, edition and current date from the Gateway.
1. `tools/list` (record the count). Then:
   - `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`, with the GUID above.
   - Marks: M0, V0. EXPECT 292 and 112.
   - `masters` `kind` `voucher_types`: record the rows of Stock Journal, Delivery Note, Receipt Note, Physical Stock, Credit Note, Debit Note, Sales and Purchase, each whole (record any that is missing).
2. **Backup BK1** (rule 3), then the marks: EXPECT M0 and V0.

### R1: the candidate request on every kind, the totals, and a Delivery Note alone

3. **Two stock items** (Gateway > Create > Stock Item), each `Under` Primary, `Units` Kgs, `Maintain in batches` Yes; a screenshot before each Accept:
   - **P:** Name `Movement Lab Item P`. Opening 100 Kgs at 10.00, allocated as `Main Location` / batch `MV-P-1` 60 and `Factory Floor` / batch `MV-P-1` 40. Expiry 31-3-2027 if Tally asks (28 Oct: an empty expiry did not pass).
   - **Q:** Name `Movement Lab Item Q`. Opening 50 Kgs at 10.00, `Main Location` / batch `MV-Q-1`, expiry as for P.

   EXPECT M0+1 after P and M0+2 after Q, and V unchanged.
4. **Delivery Note made active** (as on 28 Oct: Gateway > Alter > Voucher Type > Show Inactive > `Delivery Note` > "Activate this Voucher Type" Yes, nothing else). EXPECT M +2, V unchanged. If `Debit Note` is listed only under Show Inactive, activate it the same way (EXPECT M +2) and record it.
5. **Key on P**, in this order, a screenshot of each before its Accept; marks after each (V +1):
   1. **03-06-2026, Stock Journal SJ1:** source `Movement Lab Item P`, `Main Location`, `MV-P-1`, 10 Kgs at 10.00; destination P, `Factory Floor`, `MV-P-1`, 10 Kgs at 10.00.
   2. **03-06-2026, Stock Journal SJ2** (two items): source P `Main Location` `MV-P-1` 5 Kgs at 10.00 and `Hydrochloric Acid 33pc` `Main Location` `H-2504-A` 5 Kgs at the rate Tally offers (record it); destination P `Factory Floor` `MV-P-1` 5 Kgs at 10.00 and the acid `Factory Floor` `H-2504-A` 5 Kgs at the same rate.
   3. **03-06-2026, Stock Journal SJ3** (one side only): source P `Main Location` `MV-P-1` 4 Kgs at 10.00; no destination line. If Tally refuses a one-sided Stock Journal, record its message, take NO further key on that screen but Ctrl+Q after a screenshot, and go on (the row records it as a result).
   4. **08-06-2026, Sales S1** (Item Invoice): party the first Sundry Debtors ledger offered; sales ledger the first offered; P `Main Location` `MV-P-1` 6 Kgs at 15.00. Bill-wise: the default offered (record it).
   5. **08-06-2026, Purchase U1** (Item Invoice): party the first Sundry Creditors ledger offered; purchase ledger the first offered; P `Main Location` `MV-P-1` 20 Kgs at 10.00.
   6. **12-06-2026, Credit Note CN1** (Item Invoice): S1's party and sales ledger; P into `Main Location` `MV-P-1` 2 Kgs at 15.00.
   7. **12-06-2026, Debit Note DB1** (Item Invoice): U1's party and purchase ledger; P out of `Main Location` `MV-P-1` 3 Kgs at 10.00.
   8. **16-06-2026, Sales S2, marked Optional** (rule 4): as S1, 7 Kgs.
   9. **19-06-2026, Purchase U2:** as U1, 9 Kgs. Accept it (V +1). Then open it from the Day Book for 19-06-2026 and cancel it by the screen's own Cancel option (rule 4); answer Tally's confirmation Yes (the only prompt answered). EXPECT V +1 again.
   10. **23-06-2026, Sales S3, marked Post-Dated** (rule 4): as S1, 1 Kgs.

   Record each voucher's number as Tally gives it. EXPECT V0+1 to V0+11 in order: SJ1 113, SJ2 114, SJ3 115, S1 116, U1 117, CN1 118, DB1 119, S2 120, U2 121, U2's cancel 122, S3 123, if V0 was 112. **The span read of step 7 needs SJ1, SJ2 and SJ3 to hold AlterIDs 113, 114 and 115.** If they do not, it is not sent, and the reason is recorded.
6. **The Delivery Note alone, on Q:**
   1. Tally's Stock Summary for 1-6-2026 to 30-6-2026: the line of `Movement Lab Item Q` (quantity, rate, value). Then Q's Stock Query (Stock Summary > Q > Change View > Stock Query): Godown/Batch Details, Cost price. A screenshot of each.
   2. **26-06-2026, Delivery Note DN1:** party the first Sundry Debtors ledger offered; Q out of `Main Location` `MV-Q-1` 5 Kgs at 10.00; tracking number the default offered (record it). Screenshot before the Accept. Marks: V +1.
   3. The same two screens again, at once. Nothing else is keyed between 6.1 and 6.3.
7. **Raw reads, R1** (marks before and after the group, equal), in this order, each with C3's line and a screenshot:
   - `r-m-20260603`, `r-m-20260608`, `r-m-20260612`, `r-m-20260616`, `r-m-20260619`, `r-m-20260623`, `r-m-20260626`;
   - then `r-span-20260603-112-114`.
8. **Tool reads, R1:**
   - `stock_summary` with `as_of` 20260531, then 20260630: record each whole, with `tie_out` and `item_count_cross_check`, and the rows of P and Q.
9. **Tally's screens, R1,** for 1-6-2026 to 30-6-2026:
   - the Stock Summary lines of P and Q;
   - Stock Item Vouchers of P for June (every line: date, voucher, Inwards, Outwards, closing);
   - P's Stock Query: Godown/Batch Details.

   **EXPECT on Tally's screens:**
   - P's closing at 30-6-2026 is **108 or 109** Kgs: 100 − 4 − 6 + 20 + 2 − 3, less 1 if the post-dated S3 is counted.
     - The optional S2 and the cancelled U2 do not count.
     - By godown: Factory Floor 55, and Main Location 53 or 54.
     - If Tally did not take SJ3 (step 5.3), add 4 to P's closing and to Main Location's.
     - Record which, and whether S3 shows.
   - Q's closing at 30-6-2026 is 45 Kgs if the Delivery Note counts in the Stock Summary, 50 if it does not. Record which. Step 6's before and after lines decide it.

   A different P figure is recorded, and goes to the run coordinator before step 10.
10. **Record, without judging,** for each `r-m` and `r-span` answer, read by structure (LAB-COMMON C4; ROWS are the `VOUCHER` children of the collection):
    - **for each VOUCHER:** DATE, VOUCHERTYPENAME, VOUCHERNUMBER, ALTERID, ISOPTIONAL, ISCANCELLED, ISPOSTDATED, and the ten computed elements (`BRIDGEVCHTYPEGUID`, `BRIDGEVCHRESERVEDNAME`, the eight `BRIDGEVCHIS…`);
    - **for each `ALLINVENTORYENTRIES.LIST`:** STOCKITEMNAME, ISDEEMEDPOSITIVE, ACTUALQTY, BILLEDQTY, RATE, AMOUNT, ISSCRAP, and each `BATCHALLOCATIONS.LIST`'s GODOWNNAME, BATCHNAME, DESTINATIONGODOWNNAME, ACTUALQTY and AMOUNT;
    - whether `INVENTORYENTRIESIN.LIST` / `INVENTORYENTRIESOUT.LIST` are present, and their lines;
    - the answer's bytes;
    - **for `r-span-20260603-112-114`:** EXPECT exactly SJ1 and SJ2 (AlterIDs 113 and 114), and not SJ3. If Tally did not take SJ3, the span read is not sent (step 5's AlterIDs differ), and that is recorded;
    - **from the lines read** (excluding the optional and the cancelled voucher), P's in and out quantity per day and in total; the godown and batch of each; and the same sum against the figure of step 9;
    - the class elements of each voucher type, especially Stock Journal (`BRIDGEVCHRESERVEDNAME`, every `BRIDGEVCHIS…`) and Debit Note.

### R2: nested-only items, and the item-less control

11. **Key**, a screenshot of each before its Accept, marks after each (V +1):
    1. **06-07-2026, Purchase in accounting mode** (on the Purchase screen, change the mode to the voucher mode that takes ledgers, not items: Ctrl+H, record the mode's name): Dr the purchase ledger U1 used, 100.00, with its inventory allocation P `Main Location` `MV-P-1` 10 Kgs at 10.00 if Tally opens one; Cr U1's party. If Tally opens no inventory allocation, record that, and key the voucher without one.
    2. **07-07-2026, the control,** a Purchase in the same mode: Dr the first Indirect Expenses ledger offered, 50.00; Cr U1's party. No item. If Tally refuses a Purchase with no purchase ledger, record its message and key the same two ledgers as a Journal.
    3. **08-07-2026, Journal** with an inventory allocation: Dr the purchase ledger, 20.00, with its inventory allocation P `Main Location` `MV-P-1` 2 Kgs at 10.00 if Tally opens one; Cr U1's party. If none opens, record that.
12. **Raw reads, R2** (marks before and after, equal): `r-a07-20260706`, `r-m-20260706`, `r-a07-20260707`, `r-m-20260707`, `r-a07-20260708`, `r-m-20260708`.

    **Record:**
    - for each voucher, where its item sits in a07 (nested `INVENTORYALLOCATIONS.LIST` under a ledger entry) and in the `r-m` answer (top-level, nested, or absent);
    - **every element** of the `r-m` answer of 06-07 that is absent from or different in the `r-m` answer of 07-07: names and values, read by structure, never a text search.

    No EXPECT beyond the shape: the row's result is what the a08 answer does, or does not, carry for a voucher whose item is nested only.

### R3: the voucher types of stock vouchers

13. 1. **Receipt Note** and **Physical Stock** made active if inactive (as step 4). EXPECT M +2 each; record which were inactive.
    2. **A Manufacturing Journal type** (Gateway > Create > Voucher Type): Name `Lab Manufacturing Journal`; Select type of voucher `Stock Journal`; "Use as Manufacturing Journal" Yes; everything else as offered. Screenshot before the Accept. EXPECT M +1.
    3. **Key**, a screenshot each before its Accept, marks after each (V +1):
       - **04-08-2026, Delivery Note:** party the first Sundry Debtors ledger offered; P out of `Main Location` `MV-P-1` 1 Kgs at 10.00.
       - **05-08-2026, Receipt Note:** party the first Sundry Creditors ledger offered; P into `Factory Floor` `MV-P-1` 1 Kgs at 10.00.
       - **06-08-2026, Physical Stock:** P at `Main Location` / `MV-P-1`, the quantity Tally shows as current there (record it), so that nothing changes.
       - **07-08-2026, Lab Manufacturing Journal:** produce P `Factory Floor` `MV-P-1` 1 Kgs; consume `Hydrochloric Acid 33pc` `Main Location` `H-2504-A` 2 Kgs at the rate offered.
       
       If Tally asks for a bill of materials, choose none (the "Not Applicable" or end-of-list entry; record the screen). If it will not proceed without one, record the screen, Ctrl+Q after a screenshot, and go on without this voucher.
    4. **Raw reads, R3** (marks before and after, equal): `r-m-20260804`, `r-m-20260805`, `r-m-20260806`, `r-m-20260807`.

    **EXPECT, per voucher:**
    - Delivery Note, Receipt Note and Physical Stock: `BRIDGEVCHRESERVEDNAME` the type's own name, and all eight `BRIDGEVCHIS…` `No`;
    - Lab Manufacturing Journal: `BRIDGEVCHRESERVEDNAME` empty, and all eight `No`.

    A different value is recorded and reported before step 14; it is not a STOP. Also record each one's lines as in step 10.

### Back to the start

14. **Set back** Tally's current date to step 0's (F2 at the Gateway); record it. Marks unchanged.
15. **Restore** (C6) from BK1, on the run coordinator's explicit RESTORE. Then, in order:
    - a screenshot of the Gateway: its period and current date equal step 0's;
    - `list_companies`: exactly THE COMPANY, `verified_tuple`, the same GUID;
    - marks 292 and 112 (M0 and V0);
    - `masters` `kind` `voucher_types`: the rows of step 1, equal;
    - `stock_summary` `as_of` 20260630: no row named `Movement Lab Item P` or `Movement Lab Item Q`.

    Anything else = STOP. There is no fallback backup.
16. Hash the exe. LAB-COMMON C10: Company > Shut from the Gateway's menu (never Ctrl+Q at the Gateway); Select Company with none loaded; Tally's current date as step 0 recorded; one tally.exe; 9001 LISTENING; `GET /status` 200.

## STOP rules

LAB-COMMON's, with rules 1 to 7 above. Also: a raw answer that does not parse as XML; a marks change other than rule 2's; a voucher keyed on the wrong item, godown, batch or date (do not alter it: STOP and report what was saved); any prompt not named here (C7).

## What the book holds after each step (C8)

| after | the book |
|---|---|
| step 2 | as at step 1 (BK1 taken) |
| step 3 | plus P and Q |
| step 4 | and Delivery Note active (and Debit Note, if it was inactive) |
| step 5 | and SJ1, SJ2, SJ3 (if Tally took it), S1, U1, CN1, DB1, S2 (optional), U2 (cancelled), S3 (post-dated) |
| step 6 | and DN1 |
| step 11 | and the three July vouchers (or those Tally took) |
| step 13 | and Receipt Note and Physical Stock active (if they were inactive), Lab Manufacturing Journal, and the four August vouchers (or those Tally took) |
| step 14 | the same, with the current date set back |
| step 15 | as at BK1, the book of step 1 |

## What the row records, and what it does not

**Records:**
- the candidate request's answers on every kind of stock voucher keyed here, with each voucher's class elements;
- the AlterID-span form on one day;
- P's movement against Tally's screens and `stock_summary`;
- one Delivery Note's effect on the Stock Summary, alone;
- whether the inventory request carries anything for a voucher whose item is nested only.

**Does not:**
- a second book or release;
- 7.1;
- Gold;
- alternate or compound units;
- a client-sized book (the per-voucher bytes of a dense window);
- a narrower FETCH (a later row);
- Education mode.

## Evidence

To `lab/2016-stock-movement-answers`, after the pre-push scan and the run coordinator's yes:
- `RESULTS.md`;
- `notes.txt` (UTC, one line per step);
- `answers/` (every tool answer, whole);
- `raw/` (every raw answer with its headers, size and sha256, and a `.parsed.txt` of its structure);
- `send-log.txt`;
- `restore-plan.txt`;
- `SCREENSHOTS.txt` (the captures and BK1 by sha256; they stay on the lab PC).

THE COMPANY's GUID is already in the merge gate's registry. No figure is recorded beyond the book's own synthetic ones, and the company's number is not written in the evidence.
