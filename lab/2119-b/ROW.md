# Row 2119-B: one party's Ledger Outstandings answer against the Bills Payable report and the ledger's closing balance, on BRIDGE BILLTRAIL LAB, by curl

Not for merge. Part of #2119. Reads only: nothing is created, altered, imported or restored, and no backup is needed. No ComplyEaze Bridge build. Lines are cited at master 1400175e. Confidence of everything here until the row runs: unverified.

## Question
Row 2119-L (`lab/2119-l-answers` at c6899415) tied one party's Ledger Outstandings answer to the Bills Receivable report on the dense book. That party had no credit bill and no On Account row. The #2119 design note reads a named party's bills by this report alone, and withholds the call unless the answer's total equals the ledger's closing balance. Before any code, three things must hold on a party that has all three kinds of row:
1. Are the answer's credit bills exactly the party's rows in the Bills Payable report, and how do their signs relate?
2. Does the answer's total equal the party's `CLOSINGBALANCE` in the ledger snapshot Bridge reads, and does its On Account row equal closing balance less the named bills, the residual Bridge computes today?
3. Does 2 still hold at an earlier date? The report takes only a to-date, while the snapshot takes the books' first day and the as-of date (`src-tauri/crates/bridge-tally-protocol/src/native_outstandings/request.rs:158-165`).

## Why this book
`BRIDGE BILLTRAIL LAB` is synthetic, GUID begins `9ab5002c`. Its party `Lab Party P1` holds, at 31 March 2026, three debit bills (OB-1, INV-1, INV-2), two credit bills (ADV-1, CN-1) and an On Account row. That was read on 27 Oct 2026 (`lab/1356-party-term-and-ledger-outstandings-answers` at b761d18e, r04), and every bill equalled Tally's screen. Both Bills reports scope by sign, not by group (protocol reference §12a.1). §12a.14 records that a party's advance is listed in the report of the opposite direction. So ADV-1 and CN-1 are expected among Bills Payable's rows. That is inferred, not yet read against this report.

## What it decides
The deciding branch is the whole-book pairs at `src-tauri/src/tally/runtime.rs:4141-4147` and `:4198-4202`, which the design note would replace for a call with `party`. The residual it would keep is computed after the ledger snapshot at `:4224-4225`.

| this row shows | reading |
|---|---|
| Step 6: `debit_vs_receivable` and `credit_vs_payable` each match every row, with none on one side only, and the amounts are all `equal` or all `opposite_sign`; both `tie` lines equal. Step 9: both `tie` lines equal. | The design note (d78b8e71) stands. Its sign rule is written from what `credit_vs_payable` shows. Code waits on the run coordinator's word. |
| `credit_vs_payable` does not match: a row on one side only, or amounts that are neither all equal nor all of opposite sign. | The answer's credit side is not the Payable side. The design note is revised, on a new path, before any code. |
| `closing_vs_lo_total` is NOT EQUAL at step 6. | The withhold check would refuse this party. The party read is not built as designed. #2119 rests on row H. |
| Step 6 ties, but `closing_vs_lo_total` is NOT EQUAL at step 9. | The report and the snapshot differ at an earlier date. The party read would be admitted only at the books' last day, or not at all. Record it; the design note is revised. |
| `residual_vs_on_account` is NOT EQUAL while `closing_vs_lo_total` is equal. | The two reads name different bills. The match lines say which. Record it. |
| Any send raises a dialog, or a `tie.py` FAIL. | STOP (C7). Nothing further is sent. |

## Rules and company
`LAB-COMMON.md` here is a byte copy of v3 (sha256 5370c6d2…, as in `lab/2119-l-sheet`), and applies in full. Where it and this sheet differ, the stricter one holds. The lab acts on:
- the owner's standing word of 27 Oct 2026;
- the owner's word of 26 Oct for curl requests in the lab (shape discovery);
- the owner's word of 4 Nov 2026 for this row.

**THE COMPANY** is `BRIDGE BILLTRAIL LAB`. The lab's last notes on it (`lab/1356-ledger-filtered-allocations-answers`, 28 Oct 2026) say:
- licensed TallyPrime 7.2 Silver;
- ALTVCHID 7 and ALTMSTID 212, the marks answer byte-equal to 27 Oct's;
- current period 1 April 2025 to 31 March 2026; current date Monday, 1 September 2025;
- shut at the end, with no company loaded, and one `tally.exe` on 9001.

If THE COMPANY is not on the PC, or step 1's marks differ from 7 and 212: STOP. Record the difference; the run coordinator decides. Only THE COMPANY is loaded, alone, from its load in step 0 to the shut in step 11.

**Keys in Tally.**
- The Gateway is the only screen used. No report is opened on Tally's screen in this row.
- Load THE COMPANY from Select Company with the arrow keys and Enter. Press Enter only once a screenshot shows the highlighted row reads `BRIDGE BILLTRAIL LAB`.
- Shut it at step 11 by the top menu's Company, then Shut. The list or confirmation that names THE COMPANY alone is answered by one click on it, after a screenshot.
- No other key or click goes to Tally. Never Ctrl+Q or Escape at the Gateway.
- Tally's current date is never changed. Every request carries its own dates.

**Every dialog and prompt this row may meet, and its answer.** Anything else, in Tally, Windows or a shell, is C7: screenshot, its text exactly, no key, report.

| where | what shows | answer |
|---|---|---|
| Tally, step 0 | Select Company, with no company loaded | the expected screen: load THE COMPANY as above |
| Tally, step 11 | the Shut list or a confirmation that names THE COMPANY alone | one click on it, after a screenshot |
| Tally, any step | any other dialog, message box or half-entered screen; a save prompt of any kind (none can arise, since nothing is altered); a "quit" prompt | STOP (C7) |
| Windows | a security or permission prompt for curl or python | STOP, nothing allowed |

**Requests.** Each is sent by C3's line, through the send helper of `lab/2119-l-answers` (`scripts/c3-send-t.ps1`), unchanged:
- the header `Content-Type: text/xml; charset=utf-16`;
- the timing fields `-w "%{http_code} %{size_download} %{time_starttransfer} %{time_total}"`;
- one at a time, and never with `--max-time`.

The helper checks each request file's sha256 against the line `tie.py` printed for it before sending, and refuses to overwrite an answer. Every answer's size bound is 2,000,000 bytes. After every send, take a screenshot of Tally's whole window before the next request. A dialog there is C7.

**One request shape per step.** Each step sends one request, once. Steps 4 and 7 send the same shape at two dates, as do steps 5 and 8. No request is sent twice except marks, at steps 1 and 10. An answer is read only by `tie.py`.

## Files
| file | what |
|---|---|
| `ROW.md` | this sheet |
| `LAB-COMMON.md` | v3, byte copy |
| `tie.py` | writes the seven requests; reads each answer; prints the ties |

`SHA256.txt` lists them. Check it before step 0. Run the script with `python -I`, from a folder of its own, with the answers in another.

`tie.py fixed` writes seven requests, and fails unless the three already sent on 27 Oct render byte for byte. The other four are master's renderers' text, checked against the Rust literals at 1400175e.

| request | bytes | sha256 | sender on master |
|---|---|---|---|
| `marks.utf16le.xml` | 984 | `501c3094407612a082bd1518518276bcdd97af77ddcc358d4b7da2387fc84910` | the marks read; r01 of 27 Oct |
| `receivable-0331.utf16le.xml` | 772 | `c83acd7e8782a0ca196e4f3bddb8506c3af133b3b410ce45d15e9e197b68ee62` | `render_native_bills_request` (`request.rs:131-144`), sent at `runtime.rs:4141-4147` |
| `payable-0331.utf16le.xml` | 766 | `d5d1e6c5b2e379be42d0b6579e15d5061cd0a1162388d8eba0aaa0d1ba60913c` | the same renderer, Payable, sent at `runtime.rs:4198-4202` |
| `ledgers-0331.utf16le.xml` | 1292 | `a5d027fd09426d3f43e8a2155ffef675121d30a3ff13c363af1db17eae0de054` | `render_native_ledger_snapshot_request` (`request.rs:169-174`, `:197-208`), sent at `runtime.rs:4224-4225` |
| `lo-0331.utf16le.xml` | 1198 | `f560dba1651ca4b4d89359feffd8e4f6058ee15ae380c05929cca072313d95dc` | none; r04 of 27 Oct, the shape of `lab/2119-l-sheet` |
| `ledgers-0731.utf16le.xml` | 1292 | `90836c009748b851b52fdf21e30dc6c5838813ee076b8756f5262ff77f76d897` | as `ledgers-0331`, to 31 July 2025 |
| `lo-0731.utf16le.xml` | 1198 | `7c093656c55f14dd9f0164d6b9cd4806aafb3e5139a7d315f79794a3263942ad` | none; r05 of 27 Oct |

## Steps
One at a time. Every answer goes to a file, whole; a file with no answer in it is a failed step, not an empty one. All times are in UTC. Each step's EXPECT is checked before the next step; an EXPECT not met is recorded, and a STOP only where the step says so.

0. **Step 0 (C2), with no company loaded.** Screenshot; the process list; port 9001; `GET /status`. Load THE COMPANY alone. Screenshot the Gateway. EXPECT THE COMPANY alone. Record its current date and Tally's release.
1. **Marks.** `tie.py fixed --out req`; check the seven lines against the table (any difference = STOP). Send `marks`, then `tie.py marks`.
   - EXPECT `ALTVCHID 7 ALTMSTID 212`, GUID beginning `9ab5002c`, and the answer byte-equal to 27 Oct's (3,400 B, sha256 0fe9f835…). Other marks = STOP.
2. **Bills Receivable to 31 March 2026.** Send `receivable-0331`, then `tie.py report`.
   - EXPECT the party's rows 3, summing -25000.00 (inference: r04's debit bills).
   - A FAIL = STOP.
3. **Bills Payable to 31 March 2026.** Send `payable-0331`, then `tie.py report`.
   - EXPECT the party's rows 2, ADV-1 and CN-1 (inference, §12a.14). Their sign is not expected either way.
   - A FAIL = STOP. A row count other than 2 is recorded, not a STOP.
4. **The ledger snapshot to 31 March 2026.** Send `ledgers-0331`, then `tie.py ledgers <answer> <marks answer>`.
   - EXPECT one row for the party, with a `CLOSINGBALANCE`.
   - A FAIL = STOP: that covers a row whose company GUID is not the marks answer's, a STATUS other than 1, and no row for the party.
5. **Ledger Outstandings to 31 March 2026.** Send `lo-0331`, then `tie.py lo`.
   - EXPECT `rows 6 sum -26000.00 voucher_lines 9`, and the answer byte-equal to 27 Oct's (5,878 B, sha256 063b3c76…). Not byte-equal is recorded, not a STOP.
   - A FAIL = STOP.
6. **The ties at 31 March 2026.** No request. Run `tie.py tie0331 <marks> <receivable> <payable> <ledgers-0331> <lo-0331>`. EXPECT (unverified):
   - `debit_vs_receivable matched 3 equal 3 … differ 0 only_report 0 only_lo 0`;
   - `credit_vs_payable matched 2`, with `differ 0 only_report 0 only_lo 0`, and either `equal 2` or `opposite_sign 2`;
   - `tie closing_vs_lo_total equal`;
   - `tie residual_vs_on_account equal`.

   A NOT EQUAL is a result, not a STOP. Record the output whole.
7. **The ledger snapshot to 31 July 2025.** Send `ledgers-0731`, then `tie.py ledgers`. EXPECT as at step 4; a FAIL = STOP.
8. **Ledger Outstandings to 31 July 2025.** Send `lo-0731`, then `tie.py lo`.
   - EXPECT `rows 5 sum -37000.00 voucher_lines 5`, and the answer byte-equal to 27 Oct's r05 (3,926 B, sha256 f505ba96…). Not byte-equal is recorded.
   - A FAIL = STOP.
9. **The ties at 31 July 2025.** No request. Run `tie.py tie0731 <marks> <ledgers-0731> <lo-0731>`. EXPECT (unverified) both `tie` lines equal. Record the output whole.
10. **Marks again.** Screenshot the Gateway first: EXPECT THE COMPANY alone. Send `marks`: EXPECT the answer byte-equal to step 1's.
11. **Close (C10).** Screenshot the Gateway (the date unchanged). Shut THE COMPANY as under Keys. One `tally.exe` on 9001; `GET /status` 200; no company loaded. No XML request is sent after the shut.

## What to report
| step | result |
|---|---|
| Tally's release; the current date; V and M at steps 1 and 10 | |
| each send: first-byte and total seconds, bytes, sha256; byte-equal to 27 Oct or not (steps 1, 5, 8) | |
| `tie.py` outputs of steps 2 to 9, whole | |
| the "What it decides" row met | |
| screenshots: any change on Tally's screen | |

Read every number from its file. Put any STOP or EXPECT not met in the first line.

## Stop rules
C7 and C8 of LAB-COMMON. Also a STOP:
- any dialog in Tally that the dialog table does not name;
- a second `tally.exe`;
- marks other than 7 and 212, or marks that move;
- a request with no answer in 10 minutes (C3: never sent again);
- a request file whose sha256 is not the one printed;
- an answer over its bound;
- Ctrl+Q or Escape at the Gateway;
- any `tie.py` FAIL.

After a STOP nothing more is sent. The state reads (marks) may be sent once, as C8 allows.

## Limits
- One sitting, one small synthetic book, one party, two dates, TallyPrime 7.2 Silver. Gold is not covered.
- A creditor party, a party with a foreign or mixed currency, and a party whose name is outside ASCII letters, digits and single spaces are not read.
- The cost on a large or real book is not measured here. Row 2119-L measured one party of 248 bills on the dense book.
- The Bills reports and the snapshot are read once each, not paired as the product reads them.

## Keep
On `lab/2119-b-answers`, with `SHA256.txt`:
- every `tie.py` output whole;
- the request files and the C3 lines with their times;
- the answers and their headers. The book is synthetic and small, and every answer is under the bound.

Notes carry no user names and no machine paths beyond `C:\bridge-lab\2119-b\`. Commits carry no co-author or generated-with line. Send the sha first; push on the run coordinator's yes.
