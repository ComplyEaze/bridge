# Row 2119-L: Tally's Ledger Outstandings report for one party, against the Bills Receivable report, on BRIDGE CORPUS DENSE at 31 March 2026, by curl

Not for merge. Part of #2119 and #2158. Reads only: nothing is created, altered, imported or restored, and no backup is needed. No ComplyEaze Bridge build. Lines are cited at master 187e882b. Confidence of everything here until the row runs: unverified on this book.

## Question
1. For one party, does Tally's own Ledger Outstandings report return exactly the debit bills the Bills Receivable report holds for that party, bill for bill and amount for amount? And does it equal Tally's Ledger Outstandings screen for that party?
2. How long does that one-party read take on this book, where the whole Receivable report takes about 40 s? The product gives each request 20 s (`DEFAULT_REQUEST_TIMEOUT`, `src-tauri/crates/bridge-tally-transport/src/lib.rs:95`).

## Why
`outstandings` sends the whole-book Bills Receivable report, paired, at `src-tauri/src/tally/runtime.rs:4141-4147`, and Bills Payable at `:4198-4202`, whatever its arguments. (At 4c7b8db6 these were :4135-4141 and :4192-4196.) Its `offset` and `limit` (`src-tauri/src/agent_outstandings.rs:70-72`, applied at `:99-100`) cut an answer that has already been read whole.

On this book the Receivable report took 41.5 s and 36.7 s by curl (`lab/2119-bills-read-timing-answers`). A different first day gives the same 13.7 MB (`lab/2119-bills-split-quarters-answers`), so a split by date is dead. A split by party is the one left. If one party's read is fast and ties to the report, `outstandings` can read bills by party, whatever a host's limit (row 2119-H).

## The request
`bills.py select` renders it; nothing in ComplyEaze Bridge sends it today. It is the shape already observed live, changed only in its two values:
- **The text:** `r04` of `lab/1356-party-term-and-ledger-outstandings-run-sheet` (1198 B, sha256 `f560dba1…`), sent twice on 27 Oct 2026 on TallyPrime 7.2 Silver on a small synthetic book. It is the text of the dormant probe removed by #1747 (`bills_native_outstandings_probe.rs` at bc5077bff^), checked byte for byte there.
- **What was seen** (`lab/1356-party-term-and-ledger-outstandings-answers` at b761d18e, RESULTS.md, capture 2): no dialog before or after; the two answers byte-equal; every bill equal to Tally's Ledger Outstandings screen, an opening bill included. Verified once, on that book.
- **The shape:** `TYPE` `Data`, `ID` `Ledger Outstandings`, with the static variables `SVCURRENTCOMPANY`, `LedgerName`, `SVTODATE`, `SVEXPORTFORMAT` and `EXPLODEFLAG`, in r04's order and layout. No collection, no formula, no TDL.
- **What changes:** `SVCURRENTCOMPANY` becomes `BRIDGE CORPUS DENSE`, and `LedgerName` becomes the picked party. `SVTODATE` stays `20260331`. Before it writes anything, `select` renders the template with r04's own company and party and fails unless that gives r04's bytes.
- **What comes from Tally:** only the party's name, from the step 2 report's `BILLPARTY`. It is admitted only if `LedgerScopeName::new` would admit it (`src-tauri/crates/bridge-tally-protocol/src/ledger_scope.rs:65-94`): ASCII letters and digits with single spaces, no operator word, at most 1024 characters. Then it is escaped once as XML text. It lands in a static variable, never in a formula.

**One request shape per step.** Step 4 sends only this shape. The marks read and the report are the shapes earlier rows sent on this book.

**Not in this row: the `Bill` collection candidate** (`TYPE` `Bill`, `CLEARED`, `$GUID:Ledger:$Parent`). It returns only in a later row whose sheet:
- quotes the developer reference FAQ's text verbatim;
- settles the `Bill` against `Bills` spelling against `UNIT_A_RULING_9.md` §4;
- still treats any dialog as a STOP.

A wrong object type is the one failure recorded as a modal dialog (implementation guide §5.3a).

## What it decides
The deciding branch is `runtime.rs:4141-4147` and `:4198-4202` (the whole-book pairs), with the pages at `agent_outstandings.rs:70-72`.

| this row shows | reading |
|---|---|
| p1's debit bills tie to the report (every bill matched, equal, none only on one side); the screen equals the answer; the two sends are byte-equal; and each send takes under 2 s (ten parties inside the 20 s limit). | A read by party is admitted for the design note. The next row measures parties in turn, one request each, on the same shape. Then the row 2119-H result picks the build (#2119 plan, row 3). |
| They tie, but a send takes 2 s or more, or the time is near the report's. | One party is not cheap on this book. Record the seconds and the bills per second; no build on it. |
| The debit bills do not tie: a bill on one side only, or an amount that differs. | The report does not stand in for the Receivable report on this book. No build on it. #2119 then rests on row H alone, or closes as a known limit. |
| They tie, and the answer also holds credit bills or an On Account row that the Receivable report does not. | As on 27 Oct (r04 held ADV-1, CN-1 and On Account). Record their count and sum; a builder would state which rows it keeps. |
| Every debit amount has the opposite sign, and everything else ties. | It ties up to its sign. Record it; a builder would state the sign. |
| The answer holds 0 bills where the report holds bills for that party. | The name or the date was not read as asked. Record it; that is not "no bills". |
| The first send raises a dialog on Tally's screen, or answers with a STATUS or a LINEERROR. | STOP (C7). Nothing further is sent. |

## Rules and company
`LAB-COMMON.md` here is a byte copy of v3 (sha256 5370c6d2…, as in `lab/2119-bills-60s-row`), and applies in full. Where it and this sheet differ, the stricter one holds. The lab acts on:
- the owner's standing word of 27 Oct 2026;
- the owner's word of 26 Oct for curl requests in the lab (shape discovery);
- the owner's word of 4 Nov 2026 for this row.

**THE COMPANY** is `BRIDGE CORPUS DENSE`, synthetic, GUID begins `d45bc1b0` (whole in `list_companies` and in `lab/2119-bills-60s-row`). Licensed TallyPrime 7.2 Silver. The lab's last notes on it (`lab/register-large-book-answers`, 3 Nov 2026) say:
- it was restored from its backup and proved byte-identical: V 29,900, M 328, 123 ledgers;
- it was shut, with no company loaded, and one `tally.exe` on 9001.

Books from 1 April 2025. Tally's current date was 31 March 2026 in every #2119 row. If THE COMPANY is not on the PC, or step 1's marks differ from V 29,900 and M 328: STOP. Record the difference; the run coordinator decides. Only THE COMPANY is loaded, alone, from its load in step 0 to the shut in step 7.

**Keys in Tally.**
- Two screens are used: the Gateway, and at step 5 the Ledger Outstandings display with the menus that lead to it. Nothing is keyed on any other screen.
- Load THE COMPANY from Select Company with the arrow keys and Enter. Press Enter only once a screenshot shows the highlighted row reads `BRIDGE CORPUS DENSE`.
- Step 5 goes from the Gateway to Display More Reports, Statements of Accounts, Outstandings, Ledger, then the party, as on 27 Oct. Use the arrow keys and Enter, each Enter only once a screenshot shows the highlighted row reads the next name. In the ledger list, typing the party's name to narrow the list is allowed, and Enter only once the highlighted row reads the party exactly.
- No F2, Alt+F2 or other period key. The report is read at the date it opens with.
- Leave the display with Escape, one press at a time, with a screenshot before each. Each press is made only while the screenshot shows the display or one of those menus, never the Gateway. Stop pressing once the Gateway shows. These are reports and menus, not master or voucher screens, so C7's Escape rule does not apply to them.
- Shut THE COMPANY at step 7 by the top menu's Company, then Shut. A confirmation that names THE COMPANY is answered by clicking Yes once, after a screenshot.
- No other key or click goes to Tally. Never Ctrl+Q or Escape at the Gateway.
- Tally's current date is never changed. A date other than 31 March 2026 at step 0 = STOP.

**Every dialog and prompt this row may meet, and its answer.** Anything else, in Tally, Windows or a shell, is C7: screenshot, its text exactly, no key, report.

| where | what shows | answer |
|---|---|---|
| Tally, step 0 | Select Company, with no company loaded | the expected screen: load THE COMPANY as above |
| Tally, step 5 | the menus named under Keys, the ledger list, the Ledger Outstandings display | the expected screens: navigate as under Keys |
| Tally, step 7 | a confirmation to shut that names THE COMPANY | Yes, once, by mouse, after a screenshot |
| Tally, any step | any other dialog, message box or half-entered screen; a save prompt of any kind (none can arise, since nothing is altered); a "quit" prompt | STOP (C7) |
| Windows | a security or permission prompt for curl or python | STOP, nothing allowed |

**Requests.** Each is sent by C3's line, with the timing fields added, `-w "%{http_code} %{size_download} %{time_starttransfer} %{time_total}"`, one at a time and never with `--max-time`.
- Check each request file's sha256 against the line `bills.py` printed for it before sending it.
- Size bounds: 20,000,000 bytes for `ref-receivable`, and 2,000,000 for the others, `lo-p1` included.
- After each `lo-p1` send, take a screenshot of Tally's whole window before the next request: this book has not been sent this shape (C3, a NEW SHAPE). Take one every 30 s while any request runs.
- An answer is read only by `bills.py`.
- No request is sent while step 5's screens are open. Step 5 starts after step 4's last send and ends at the Gateway.

**Two sends.** `lo-p1` is sent twice in a row, as the product pairs its reads. The two answers are compared by sha256. A difference is recorded, and the comparison uses the first.

## Files
| file | what |
|---|---|
| `ROW.md` | this sheet |
| `LAB-COMMON.md` | v3, byte copy |
| `bills.py` | writes the fixed requests; picks the party and writes `lo-p1`; reads each answer; prints the screen's figures; compares bill for bill |

`SHA256.txt` lists them. Check it before step 0. Run the script with `python -I`, from a folder of its own, with the answers in another.

`bills.py fixed` writes two requests. `ref-receivable` must be byte for byte the 770-byte r-4 of `lab/2119-bills-read-timing`.

| request | bytes | sha256 | sender on master |
|---|---|---|---|
| `marks.utf16le.xml` | 982 | `040ac47754c40a4e2e736f8be2b3f37cac924213ea66c8c4f0074222f9a00280` | the marks read of earlier rows |
| `ref-receivable.utf16le.xml` | 770 | `6520d94ec6d013e3e44d9af62ee9c9830a65085b4cb5b54710340df0d03c750f` | `render_native_bills_request` (`request.rs:131-144`), sent at `runtime.rs:4141-4147` |

`lo-p1`'s bytes depend on the party picked at step 3, so its hash is the one `select` prints. It has no sender on master: its only renderer is `bills.py`, and the branch it would stand in for is `runtime.rs:4141-4147`.

## Steps
One at a time. Every answer goes to a file, whole; a file with no answer in it is a failed step, not an empty one. All times are in UTC.

0. **Step 0 (C2), with no company loaded.** Screenshot; the process list; port 9001; `GET /status`. Load THE COMPANY alone. Screenshot the Gateway: EXPECT THE COMPANY alone, and the current date 31 March 2026. Record Tally's release.
1. **Marks.** `bills.py fixed --out req`; check the two lines against the table. Send `marks`. EXPECT one COMPANY row, this GUID, ALTVCHID 29900 and ALTMSTID 328 (C4).
2. **The report.** Send `ref-receivable` once.
   - `bills.py ref` on its answer. EXPECT 29,900 rows, and the sum of R4 on 29 Oct (`lab/2119-bills-read-timing-answers`, RESULTS.md).
   - EXPECT a time of about 33 to 42 s (R4 to R6, A1 to A6).
   - Sender `runtime.rs:4141-4147`; the report's grammar is the one `native_outstandings/wire.rs` reads.
3. **The pick.** No request.
   - `bills.py select <ref answer> --out lo`. EXPECT a `parties` line, then one `lo-p1.utf16le.xml` line with its size and sha256.
   - A FAIL = STOP. That covers a template that does not render r04's bytes, and no party in the admitted alphabet.
   - Read `lo-p1` by eye: `ID` `Ledger Outstandings`, `SVCURRENTCOMPANY` THE COMPANY, `LedgerName` the name in `party.txt`, `SVTODATE` `20260331`, no `COLLECTION` and no `TDL`.
   - `party.txt` stays on the lab PC: the name is pushed only as "p1", with its bill count.
4. **p1, the one send of this step's shape.**
   - Send `lo-p1` once, then the screenshot. EXPECT no dialog (verified on 27 Oct on another book; unverified here).
   - `bills.py lo` on it. A FAIL = STOP: that covers a parse failure, a STATUS, an element outside r04's grammar, and an empty answer.
   - Send `lo-p1` again, then the screenshot, and compare the two sha256.
   - `bills.py compare <ref answer> <first lo answer> party.txt`. EXPECT (unverified):
     - `matched` equal to `ref (this party)`'s rows, with `differ 0 only_ref 0 only_lo_debit 0`;
     - credit bills and On Account counted on their own line, any number.
   - EXPECT each send under 2 s (inference: a party holds about one in 120 of the bills).
   - No sender on master; the branch it would stand in for is `runtime.rs:4141-4147`.
5. **The screen.** No request.
   - `bills.py screen <first lo answer>`: it prints the row count, the total pending, and the first and last five rows.
   - Open the Ledger Outstandings display for the party in `party.txt` as under Keys. Screenshot its first page, its last page (Page Down; a navigation key, which alters nothing), and its total.
   - EXPECT the date it reads to be 31-Mar-26 (inference: Tally's date). Another date is recorded, and the tie is then not made; no period key is pressed.
   - EXPECT the total, and each of the ten rows printed (date, reference, opening, pending with Dr or Cr, due date, overdue days), to equal the screen. Type the screen's figures into `screen-tie.txt` beside the printed lines.
   - Leave with Escape to the Gateway, as under Keys.
6. **Marks again.** Screenshot the Gateway first: EXPECT THE COMPANY alone. Send `marks`: EXPECT V 29900 and M 328.
7. **Close (C10).** Screenshot the Gateway (the date unchanged). Shut THE COMPANY as under Keys. One `tally.exe` on 9001; `GET /status` 200; no company loaded. No XML request is sent after the shut.

## What to report
| step | result |
|---|---|
| Tally's release, date; V and M at steps 1 and 6 | |
| the report: rows, sum, seconds, bytes | |
| the pick: parties in the report, in the alphabet; p1's bills in the report; `lo-p1`'s size and sha256 | |
| `lo-p1`: each send's first-byte and total seconds, bytes, sha256 equal or not | |
| `compare` and `screen` outputs, whole | |
| the screen tie: equal or not, row by row and the total | |
| bills per second, report against `lo-p1` | |
| screenshots: any change on Tally's screen | |

Read every number from its file. Put any STOP or EXPECT not met in the first line. Name the "What it decides" row the result meets.

## Stop rules
C7 and C8 of LAB-COMMON. Also a STOP:
- any dialog in Tally that the dialog table does not name;
- a second `tally.exe`;
- marks that move;
- a request with no answer in 10 minutes (C3: never sent again);
- a request file whose sha256 is not the one printed;
- an answer over its bound;
- Ctrl+Q or Escape at the Gateway;
- any `bills.py` FAIL.

After a STOP nothing more is sent. The state reads (marks) may be sent once, as C8 allows. If a STOP comes while step 5's screens are open, they are left as they are, and the run coordinator decides.

## Limits
- One sitting, one book, one party, one date (the books' last day), TallyPrime 7.2 Silver. Gold is not covered.
- Whether the report follows an earlier `SVTODATE` on this book is not measured (r05 measured it on 27 Oct, on the small book).
- Parties in turn, and the time of many one-party reads, are the next row's question, not this one's.
- A name outside the admitted alphabet is not read. Whether a name that matches no ledger answers in band is not established.
- A real book's party count and bill spread are not measured. The heaviest real book refused in band at 96.9 s (`lab/held-book-reads-answers`).
- The Payable side has no bills on this book.
- A party named by two ledgers is not detected. The report's `BILLPARTY` is a name, and the product would bind by GUID.

## Keep
On `lab/2119-l-answers`, with `SHA256.txt`:
- every `bills.py` output whole;
- the request files `marks` and `ref-receivable` (`lo-p1` by size and sha256 only, since it holds the name), and the C3 lines with their times;
- the answers' headers;
- `screen-tie.txt`.

The answers themselves stay on the lab PC by size and sha256: the report is 13.7 MB, and both hold party names. So do `party.txt`, `lo-p1` and the screenshots. Also keep a `RESULTS.md` holding the table above. Notes carry no user names and no machine paths beyond `C:\bridge-lab\2119-l\`. Commits carry no co-author or generated-with line. Send the sha first; push on the run coordinator's yes.
