# Lab run sheet 692-R1 to R5: opening stock that Tally carries from a prior year, on a new synthetic two-year book (#692). SYNTHETIC COMPANY ONLY. Not for merge.

About two hours. `LAB-COMMON.md` in this folder (v3, byte copy) applies in full; where this sheet and it differ, the stricter holds. Two exceptions: the company is new, so C1's number and GUID are recorded on the lab PC only (see THE COMPANY), and C11's "every answer in full" is replaced by EVIDENCE below. The book is created and keyed on Tally's screen by the operator, and nothing is imported. ComplyEaze Bridge runs once, for a single `list_companies` call that reads the new company's GUID (setup item 2). Every request is a read.

## What it decides

`profit_and_loss` and `balance_sheet` decline any book whose Profit and Loss shows a non-empty `Opening Stock` line (`TallyClosingStock::from_profit_and_loss`, `src-tauri/src/reports/statements.rs:293`, refusing at :331). That covers every inventory book after its first year.

The proposal under ruling would establish such a book from two of Tally's figures, the `Opening Stock` (O) and `Less: Closing Stock` (C) lines. It would do so only when these relations hold exactly:
1. Tally's Current Assets less the ledgers' Current Assets equal C;
2. the carried Profit & Loss A/c gap equals O − C;
3. the ledger closings sum to −O, and the Balance Sheet export balances;
4. the `Cost of Sales :` heading equals O plus purchases plus direct expenses, less C.

The relations held on `BRIDGE TIEOUT D` and its four experiments of 27 Oct, a book whose opening stock was keyed as an item's opening, and broke where they should (its opening edit, X5). They have never been read where Tally itself carries the opening stock from the year before. That is what a real book has after its first year, and it is what this sheet makes:
- **R1:** a company whose books begin 1 April 2024, keyed through two years with no item opening, so 2025-26's opening stock is Tally's carry-forward. The questions: do the relations hold in 2025-26 and in its part-year windows, and does the Profit & Loss A/c ledger carry the first year's result as Tally's statement shows it, or as the ledgers alone give it?
- **R2 to R4:** each changes one thing on R1's book, from its backup: the costing method, negative stock, a direct expense.
- **R5:** a read of the window across both years.

The relations are checked by two scripts in this folder:
- `identities.py` is a byte copy from `lab/692-identity-check-requests`. It gives the right verdicts on all four D3 states (true on S, X1 and X4; false on X5's two balancing relations).
- `relations.py` is new and was tested on the same answers.

## THE COMPANY

`BRIDGE TIEOUT E`, created in step 1. Its company number, its GUID and its backup's file name (which carries the number) are written on the lab PC only, never in a committed file. The committed notes say "THE COMPANY", "BK-E" and the backup's bytes and sha256. The filled marks file (`m-marks-E.utf16le.xml`, with the GUID) stays on the lab PC. Never any company but this one. If a company of this name already exists on the lab PC: STOP before step 1.

## Requests and checks

- **The files.**
  - `requests/` is written by `gen_requests.py` from `templates/`, changing only the company and the dates (`SHA256.txt` lists every file).
  - With `BRIDGE TIEOUT D` and its dates filled in, each template gives the request the D3 sitting sent live, byte for byte (`lab/692-opening-stock-d3-requests` at `6f4f7869`). Those requests in turn equal committed fixtures under `src-tauri/crates/bridge-tally-protocol/tests/fixtures/`, as that sitting's README sets out. No new request shape is sent.
- **Request-ID census before the first send**, in this folder: `python -I check-request-ids.py requests`.
  - EXPECT `census: ok 7, skipped 20, FAIL 0`.
  - The skipped files are report requests. Check by eye that each defines no `COLLECTION`.
  - Fill `@@GUID@@` in `m-marks-TEMPLATE.utf16le.xml` with the GUID that setup item 2 reads, save it as `m-marks-E.utf16le.xml`, and run the census on it: EXPECT `ok`.
  - Anything else: STOP, nothing sent.
- **MARKS** means sending `m-marks-E.utf16le.xml` (C4's marks answer: ALTVCHID V, ALTMSTID M).
- **Bounds (C3):** the marks read 8,192 bytes, every other read 2,000,000 bytes.
- **A READ GROUP for window wN:**
  1. MARKS;
  2. `wN-pl`, `wN-bs`, `wN-ledgers`, `wN-negative-stock` and `wN-stock-summary`, in that order;
  3. MARKS. EXPECT the marks equal before and after; else STOP.
  - Each answer is saved as received, named `<row>-<request file>`, for example `r1-w2-pl.utf16le.xml`.
- **THE CHECKS for a read group**, run on the answers as received:
  - `python -I identities.py <pl> <bs> <ledgers> <groups>`;
  - `python -I relations.py window <pl> <stock-summary> <negative-stock>`;
  - where the row says so, `python -I relations.py carry <pl of w1> <ledgers of w1> <pl of w2> <ledgers of w2>`.
  - Both scripts print verdicts and counts only. Their printed lines go into the evidence whole.
  - A script that exits with "not read" is a result: record it and go on. A Python that will not start is reported as such, not as a verdict.

## Setup (step 1, the operator on Tally's screen; screenshot before each accept)

1. **Start.** Start with no company loaded (C1).
   - Create company `BRIDGE TIEOUT E`: financial year and books beginning 1-Apr-2024, base currency INR.
   - In F11: maintain inventory Yes, integrate accounts with inventory Yes, GST No, batches No, godowns No.
   - Record each screen's fields as shown. Any prompt these lines do not name: C7.
2. **Step 0 (C2)** with THE COMPANY loaded alone.
   - Read its GUID with one `list_companies` call of the lab's staged `bridge_mcp.exe`, over stdio, with a new empty `BRIDGE_AGENT_DATA_DIR` and posting and import unset. Record the exe's sha256 before and after. Send no other tool call, and stop the server after it.
   - EXPECT exactly one company, `BRIDGE TIEOUT E`; else STOP.
   - Write the GUID on the lab PC only and fill the marks file as above. MARKS: record M and V.
3. **Masters on screen**, each saved by the screen's own accept:
   - unit `Nos`;
   - stock items `Lab Item A` and `Lab Item B`, under Primary, unit Nos, no opening quantity, rate or value. Record the Costing Method the item's More Details (Ctrl+I) shows; do not change it;
   - ledgers:
     - `Lab Capital` (Capital Account), opening 50,000.00 Cr;
     - `Lab Bank` (Bank Accounts), opening 50,000.00 Dr;
     - `Lab Supplier` (Sundry Creditors) and `Lab Customer` (Sundry Debtors), each with Maintain balances bill-by-bill No;
     - `Lab Sales` (Sales Accounts) and `Lab Purchase` (Purchase Accounts), each with Tally's default for "Inventory values are affected", recorded;
     - `Lab Rent` (Indirect Expenses).
   - MARKS after the masters.
4. **The first year's vouchers**, each dated by F2 on the voucher screen:
   - Purchase, item invoice, 10-May-2024, Lab Supplier / Lab Purchase: Lab Item A 10 Nos at 100.00, Lab Item B 20 Nos at 50.00;
   - Purchase, 10-Aug-2024, the same ledgers: Lab Item A 10 Nos at 120.00, Lab Item B 10 Nos at 60.00;
   - Sales, item invoice, 10-Nov-2024, Lab Customer / Lab Sales: Lab Item A 8 Nos at 200.00, Lab Item B 15 Nos at 90.00;
   - Payment, 10-Dec-2024, Lab Bank to Lab Rent, 500.00.
   - MARKS after each voucher: EXPECT V up by one each time and M unchanged; record any difference.
   - Tally's Bank Allocations sub-screen, if it opens, is accepted with its defaults (as on 27 Oct). A Party Details or Dispatch Details sub-screen is accepted blank with its defaults. Any other sub-screen or prompt: C7.
5. **The period.**
   - At the Gateway, set the company's period to 1-4-2024 to 31-3-2026 (Alt+F2), and record the keys and what Tally shows.
   - A prompt to create, split or open a new year, or to carry balances forward: press no key, STOP. This sheet does not split the company.
6. **The second year's vouchers**, as in item 4, MARKS after each:
   - Purchase 10-May-2025: Lab Item A 5 Nos at 130.00, Lab Item B 10 Nos at 70.00;
   - Sales 10-Sep-2025: Lab Item A 10 Nos at 220.00, Lab Item B 15 Nos at 100.00;
   - Payment 10-Dec-2025, Lab Bank to Lab Rent, 500.00.
   - No item is sold beyond what is held (A holds 12 then 7, B holds 15 then 10).
7. **Tally's screens, for the record.**
   - Stock Summary for 1-4-2024 to 31-3-2025 and for 1-4-2025 to 31-3-2026: does any line show a negative quantity?
   - The Balance Sheet as at 31-3-2026: does it show a "Difference in opening balances" line? Yes or no; screenshot.
   - Set report periods by F2 inside each report.
8. **BK-E (C5).** Back up THE COMPANY to a new empty local folder. Then MARKS: M_E, V_E.

## Rows

- **692-R1, carried opening stock (on BK-E's state).**
  - Read the groups request once (`r1-groups`), then read groups w1, w2, w3 and w4.
  - Checks for each window (each with `r1-groups`), and `carry` with w1 and w2.
  - EXPECT none: the verdicts are the result.
  - What would show the proposal's premise wrong:
    - any relation false in w2, w3 or w4;
    - any relation false in w1, where the `Opening Stock` line is expected empty and read as zero by `identities.py`. A first-year book's empty line is the shipped case (§12a.21).
    - `year 2's opening stock == year 1's closing stock` false;
    - a Profit & Loss A/c opening that equals neither candidate `carry` prints.
- **692-R5, the window across both years (read-only, straight after R1).** Read group w5, then the checks with `r1-groups`. Record. The tools refuse such a window with stock today, and this read only shows what Tally's export carries for it.
- **692-R2, costing method (one change).**
  - **Change:** alter `Lab Item A`, More Details (Ctrl+I), Costing Method: from what step 1.3 recorded to the other of Avg. Cost and FIFO. MARKS: EXPECT M up by one and V unchanged.
  - **Read:** groups w1 and w2 (`r2-…`). Checks with `r1-groups`, and `carry` w1 with w2.
  - **Restore (C6):** restore BK-E. MARKS: EXPECT M_E, V_E. Read `w2-pl` once more (`r2-after-w2-pl`): EXPECT its sha256 equal to `r1-w2-pl`'s; else STOP.
- **692-R3, negative stock (one change).**
  - **Change:**
    - create stock item `Lab Item C` under Primary, unit Nos, no opening;
    - then a Sales voucher, 10-Jan-2026, Lab Customer / Lab Sales, Lab Item C 3 Nos at 150.00, with no purchase of it;
    - screenshot Tally's Stock Summary and its Negative Stock report;
    - MARKS: EXPECT M and V each up by one.
  - **Read:** group w2 (`r3-…`), and the checks with `r1-groups`.
  - **Expected from D3's X1 (one book, keyed opening):** the negative stock answer lists one row without a value, and the stock summary has one row without a value. Record what comes.
  - **Restore** as in R2, with `r3-after-w2-pl` equal to `r1-w2-pl`.
- **692-R4, a direct expense beside stock (one change).**
  - **Change:**
    - create ledger `Lab Freight In` under Direct Expenses;
    - then a Payment, 15-May-2025, Lab Bank to Lab Freight In, 200.00;
    - MARKS: EXPECT M and V each up by one.
  - **Read:** group w2 (`r4-…`), and the checks with `r1-groups`, including the Direct Expenses counts that `relations.py window` prints.
  - **Restore** as in R2, with `r4-after-w2-pl` equal to `r1-w2-pl`.
- **Close.** The company period back to what step 5 recorded before its change, if Tally allows it without a prompt (else record and leave it). Then C10. THE COMPANY and BK-E stay on the lab PC for later rows.

**PASS** requires all of these:
- every read whole within its bound;
- the marks equal around every read group;
- every check's printed lines recorded;
- each restore back at M_E and V_E, with its `w2-pl` sha256 equal to R1's;
- C10 done.

Whatever the verdicts are, they are the result. **FAIL** is any of:
- a transport failure (C3);
- a restore that does not return to BK-E;
- a prompt answered that this sheet and LAB-COMMON do not name.

## Evidence

On a new branch `lab/692-opening-stock-answers` (folder `lab/692-opening-stock-answers/`), after the pre-push scan and the run coordinator's yes. **Shapes and counts only:**
- `RESULTS.md`;
- `checks.txt`: every line the two scripts printed, under the row and window it belongs to;
- `marks.txt`: M and V at each point;
- `send-log.txt`: request file, request sha256, curl exit, status, CONTENT-LENGTH, the answer's bytes and sha256;
- `screens.txt`: the yes/no and field records of steps 1, 3, 5 and 7 and of R2 to R4, in words;
- `notes.txt`: UTC times, one line per step;
- `SHA256.txt`.

No answer file and no amount read from an answer is committed. The answers, screenshots and BK-E stay on the lab PC, listed by name, bytes and sha256 in `SHA256.txt`; names that carry the company number are listed by bytes and sha256 only.

## Not measured by this sheet

- a company split into a new year, or opening balances carried by Tally's split;
- Gold;
- another release;
- more than two items;
- a stock group;
- batches and godowns;
- GST;
- a book that is not integrated;
- the shipped statement tools' answers (they are not run: they decline any book with opening stock today).
