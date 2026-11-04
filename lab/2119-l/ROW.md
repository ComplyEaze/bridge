# Row 2119-L: a candidate read of open bills by party, against the Bills Receivable report, on BRIDGE CORPUS DENSE at 31 March 2026, by curl

Not for merge. Part of #2119 and #2158. Reads only: nothing is created, altered, imported or restored, and no backup is needed. No ComplyEaze Bridge build. Lines are cited at master 187e882b. Confidence of everything here until the row runs: unverified.

## Question
1. Tally documents a collection of uncleared bills. With a filter on each bill's ledger GUID, does it return exactly the rows the Bills Receivable report holds for those parties, bill for bill and amount for amount? The same is asked of the collection unfiltered against the whole report.
2. Does it answer a page of 1, 10 or 100 parties well within the 20 s each request is given, where the whole report takes about 40 s on this book?

## Why
`outstandings` sends the whole-book Bills Receivable report, paired, at `src-tauri/src/tally/runtime.rs:4141-4147`, and Bills Payable at `:4198-4202`, whatever its arguments. (At 4c7b8db6 these were :4135-4141 and :4192-4196.) Its `offset` and `limit` (`src-tauri/src/agent_outstandings.rs:70-72`, applied at `:99-100`) cut an answer that has already been read whole.

On this book the Receivable report took 41.5 s and 36.7 s by curl (`lab/2119-bills-read-timing-answers`). A different first day gives the same 13.7 MB (`lab/2119-bills-split-quarters-answers`), so a split by date is dead. A split by party is the one left. If a page of parties is fast and ties to the report, `outstandings` can read bills by party page, whatever a host's limit (row 2119-H).

## The candidate request
`bills.py` renders it; nothing in ComplyEaze Bridge does. Every name in it comes from Tally's documentation or from master's own requests, never from trying names (implementation guide §5.3a):
- **`TYPE` `Bill`, `CLEARED` `No`:** the developer reference FAQ on a collection of bills, both cleared and uncleared.
- **`$GUID:Ledger:$Parent`:** the method-of-another-object form master sends as `$GUID:Company:##SVCurrentCompany` (`src-tauri/crates/bridge-tally-protocol/src/native_outstandings/request.rs:203`).
- **The filter:** `ledger_guid_filter_parts`'s (`request.rs:225-245`), one `SYSTEM` formula and `FILTERS`, with each term on the bill's ledger.
- **The fetch list:** `NAME, PARENT, BILLDATE, CLOSINGBALANCE`, plus that GUID as a `COMPUTE`, so that each row proves its own ledger.
- **What comes from Tally:** only GUIDs, admitted by `LedgerGuid::parse`'s check (`src-tauri/crates/bridge-tally-protocol/src/ledger_guid_part.rs:62-73`: ASCII hex digits and the hyphen, 1 to 64 characters). Each GUID must also begin with this company's GUID. The company, dates, type, fetch list and formula are fixed in the script.
- **Term counts:** a `$GUID` filter has been sent live with five terms only (`ledger_guid_part.rs:30-37`). The 10 and 100 terms here are new counts. Parent filters of 200 terms have been sent (protocol reference §11e).

Not established before this row: whether `Bill` rows carry these four fields, whether `$Parent` on a bill is its ledger, and whether the collection follows `SVTODATE`. This row reads it at Tally's current date, the books' last day, so that last question is not tested.

## What it decides
The deciding branch is `runtime.rs:4141-4147` and `:4198-4202` (the whole-book pairs), with the pages at `agent_outstandings.rs:70-72`.

| this row shows | reading |
|---|---|
| p1, p10 and p100 each tie to the report (every bill matched, equal, none only on one side); each send of p100 takes under 10 s; cand-all ties to the whole report. | A read by party page is admitted for the design note: the pages choose the parties, each page's request stays inside the limit, and the totals come from the ledger snapshot already read. Then the row 2119-H result picks the build (#2119 plan, row 3). |
| They tie, but p100 takes 10 s or more, or the time grows with the bills as the report's does. | The page size is the open question. Record the times per bill; no build until a smaller page is measured. |
| A page does not tie: a bill on one side only, an amount that differs, or a row whose ledger GUID was not asked for. | The candidate does not stand in for the report. No build on it. #2119 then rests on row H alone, or closes as a known limit. |
| Every amount has the opposite sign, and everything else ties. | The candidate ties up to its sign convention. Record it; a builder would state the sign. |
| A filtered page returns 0 rows where the report holds bills for those parties. | The filter shape is not the one admitted: `$Parent` is not the ledger here, or the term fails. Record it. That is not "no bills". |
| The first candidate send raises a dialog on Tally's screen, or answers with a STATUS or LINEERROR. | STOP (C7). The shape is refused, and nothing further is sent. |

## Rules and company
`LAB-COMMON.md` here is a byte copy of v3 (sha256 5370c6d2…, as in `lab/2119-bills-60s-row`), and applies in full. Where it and this sheet differ, the stricter one holds. The lab acts on the owner's standing word of 27 Oct 2026, the owner's word of 26 Oct for curl requests in the lab (shape discovery), and the owner's word of 4 Nov 2026 for this row.

**THE COMPANY** is `BRIDGE CORPUS DENSE`, synthetic, GUID begins `d45bc1b0` (whole in `list_companies` and in `lab/2119-bills-60s-row`). Licensed TallyPrime 7.2 Silver. The lab's last notes on it (`lab/register-large-book-answers`, 3 Nov 2026) say:
- it was restored from its backup and proved byte-identical: V 29,900, M 328, 123 ledgers;
- it was shut, with no company loaded, and one `tally.exe` on 9001.

Books from 1 April 2025. Tally's current date was 31 March 2026 in every #2119 row. If THE COMPANY is not on the PC, or step 1's marks differ from V 29,900 and M 328: STOP. Record the difference; the run coordinator decides. Only THE COMPANY is loaded, alone, from its load in step 0 to the shut in step 9.

**Keys in Tally.**
- The Gateway is the only screen used.
- Load THE COMPANY from Select Company with the arrow keys and Enter, and press Enter only once a screenshot shows the highlighted row reads `BRIDGE CORPUS DENSE`.
- Shut it at step 9 by the top menu's Company, then Shut. A confirmation that names THE COMPANY is answered by clicking Yes once, after a screenshot.
- No other key or click goes to Tally. Never Ctrl+Q or Escape at the Gateway.
- Tally's current date is never changed. A date other than 31 March 2026 at step 0 = STOP.

**Every dialog and prompt this row may meet, and its answer.** Anything else, in Tally, Windows or a shell, is C7: screenshot, its text exactly, no key, report.

| where | what shows | answer |
|---|---|---|
| Tally, step 0 | Select Company, with no company loaded | the expected screen: load THE COMPANY as above |
| Tally, step 9 | a confirmation to shut that names THE COMPANY | Yes, once, by mouse, after a screenshot |
| Tally, any step | any other dialog, message box or half-entered screen; a save prompt of any kind (none can arise, since nothing is altered) | STOP (C7) |
| Windows | a security or permission prompt for curl or python | STOP, nothing allowed |

**Requests.** Each is sent by C3's line, with the timing fields added, `-w "%{http_code} %{size_download} %{time_starttransfer} %{time_total}"`, one at a time and never with `--max-time`.
- Check each request file's sha256 against the line `bills.py` printed for it before sending it.
- Size bounds: 20,000,000 bytes for `ref-receivable` and every `cand-*` answer, and 2,000,000 for the others.
- After every candidate send, take a screenshot of Tally's whole window before the next request (C3, a NEW SHAPE). Take one every 30 s while any request runs.
- A candidate answer is read only by `bills.py`.

**Two sends each.** Each candidate is sent twice in a row, as the product pairs its reads. The two answers are compared by sha256. A difference is recorded, and the comparison uses the first.

## Files
| file | what |
|---|---|
| `ROW.md` | this sheet |
| `LAB-COMMON.md` | v3, byte copy |
| `bills.py` | writes the fixed requests; picks the parties and writes the candidates; reads each answer; compares bill for bill |

`SHA256.txt` lists them. Check it before step 0. Run the script with `python -I`, from a folder of its own, with the answers in another.

`bills.py fixed` writes three requests. `ref-receivable` must be byte for byte the 770-byte r-4 of `lab/2119-bills-read-timing`.

| request | bytes | sha256 | sender on master |
|---|---|---|---|
| `marks.utf16le.xml` | 982 | `040ac47754c40a4e2e736f8be2b3f37cac924213ea66c8c4f0074222f9a00280` | the marks read of earlier rows |
| `ref-receivable.utf16le.xml` | 770 | `6520d94ec6d013e3e44d9af62ee9c9830a65085b4cb5b54710340df0d03c750f` | `render_native_bills_request` (`request.rs:131-144`), sent at `runtime.rs:4141-4147` |
| `ledgers.utf16le.xml` | 1158 | `f3971d396a5a1416ab11ee831ff9d212dd1989d5bd5c5df7e85faecc40b1fc9e` | `render_native_ledger_export_request` (`request.rs:258-269`, `:387-407`), sent at `runtime.rs:3169` |

The candidates' bytes depend on the GUIDs read at step 3, so their hashes are the ones `select` prints.

## Steps
One at a time. Every answer goes to a file, whole; a file with no answer in it is a failed step, not an empty one. All times are in UTC.

0. **Step 0 (C2), with no company loaded.** Screenshot; the process list; port 9001; `GET /status`. Load THE COMPANY alone. Screenshot the Gateway: EXPECT THE COMPANY alone, and the current date 31 March 2026. Record Tally's release.
1. **Marks.** `bills.py fixed --out req`; check the three lines against the table. Send `marks`. EXPECT one COMPANY row, this GUID, ALTVCHID 29900 and ALTMSTID 328 (C4).
2. **The report.** Send `ref-receivable` once.
   - `bills.py ref` on its answer. EXPECT 29,900 rows, and the sum of R4 on 29 Oct (`lab/2119-bills-read-timing-answers`, RESULTS.md).
   - EXPECT a time of about 33 to 42 s (R4 to R6, A1 to A6).
   - Sender `runtime.rs:4141-4147`; the report's grammar is the one `native_outstandings/wire.rs` reads.
3. **The ledgers and the pick.**
   - Send `ledgers`.
   - `bills.py select <ref answer> <ledgers answer> --out cand`. EXPECT a `parties` line, then four candidate lines with sizes and sha256. Fewer than 100 parties writes no `cand-p100`; that is recorded, not a STOP. A FAIL (a party that is not exactly one ledger, or a GUID outside the check) = STOP.
   - `parties.txt` stays on the lab PC: names and GUIDs are pushed only by count.
4. **p1, the first send of the shape.**
   - Send `cand-p1` once, then the screenshot. EXPECT (unverified) no dialog, and an answer that parses.
   - Then `bills.py cand` on it. A FAIL that names the missing fields is recorded whole. If it is a parse failure, a STATUS or a LINEERROR: STOP.
   - If it parses: send `cand-p1` again, compare the two sha256, and run `bills.py compare <ref> <p1 answer> parties.txt p1`.
   - EXPECT (unverified):
     - `cand rows whose ledger GUID is not one asked for: 0`;
     - for both keys, `only_ref 0 only_cand 0 differ 0`;
     - `matched` equal to the report's rows for that party.
   - No sender on master (the candidate's only renderer is `bills.py`). The branch it would stand in for is `runtime.rs:4141-4147`.
5. **p10.** `cand-p10` twice, then `compare … p10`. EXPECT as at step 4.
6. **p100.** `cand-p100` twice, then `compare … p100`. EXPECT as at step 4, and each send under 10 s (the "What it decides" table).
7. **All.** `cand-all` twice, then `compare … all`.
   - EXPECT `only_ref 0 only_cand 0 differ 0` against the whole report. The report holds debit-balance bills, and Bills Payable on this book held none on 29 Oct (R7, 46 bytes). So an `only_cand` row is a bill of a credit side: record its count, not a STOP.
   - EXPECT a time near the report's (inference).
8. **Marks again.** Send `marks`: EXPECT V 29900 and M 328.
9. **Close (C10).** Screenshot the Gateway (the date unchanged). Shut THE COMPANY as under Keys. One `tally.exe` on 9001; `GET /status` 200; no company loaded. No XML request is sent after the shut.

## What to report
| step | result |
|---|---|
| Tally's release, date; V and M at steps 1 and 8 | |
| the report: rows, sum, seconds, bytes | |
| the pick: parties in the report; the four candidate sizes and sha256 | |
| p1, p10, p100, all: each send's first-byte and total seconds, bytes, sha256 equal or not | |
| each `compare` output, whole | |
| bills per second, report against each candidate | |
| screenshots: any change on Tally's screen | |

Read every number from its file. Put any STOP or EXPECT not met in the first line. Name the "What it decides" row the result meets.

## Stop rules
C7 and C8 of LAB-COMMON. Also a STOP:
- any dialog in Tally;
- a second `tally.exe`;
- marks that move;
- a request with no answer in 10 minutes (C3: never sent again);
- a request file whose sha256 is not the one printed;
- an answer over its bound;
- Ctrl+Q or Escape at the Gateway;
- a `bills.py` FAIL other than the field list of step 4.

After a STOP nothing more is sent. The state reads (marks) may be sent once, as C8 allows.

## Limits
- One sitting, one book, one date (the books' last day), TallyPrime 7.2 Silver. Gold is not covered.
- Whether the candidate follows `SVTODATE` at an earlier date is not measured.
- A real book's party count and bill spread are not measured. The heaviest real book refused in band at 96.9 s (`lab/held-book-reads-answers`).
- The Payable side has no bills on this book.
- A party whose name is used by two ledgers stops the pick. The product would bind by GUID from the start, not by the report's names.

## Keep
On `lab/2119-l-answers`, with `SHA256.txt`:
- every `bills.py` output whole;
- the request files and the C3 lines with their times;
- the answers' headers.

The answers themselves stay on the lab PC by size and sha256: the report is 13.7 MB, and the candidates hold party names. So do `parties.txt` and the screenshots. Also keep a `RESULTS.md` holding the table above. Notes carry no user names and no machine paths beyond `C:\bridge-lab\2119-l\`. Commits carry no co-author or generated-with line. Send the sha first; push on the run coordinator's yes.
