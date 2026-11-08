# Row 2377-L, Part B: lab-made parties with marked names and a bill called "On Account" on BRIDGE BILLTRAIL LAB, and the dense book's three largest parties, through Tally's Ledger Outstandings report, by curl

Not for merge. Part of #2377. Sitting 1 writes to BRIDGE BILLTRAIL LAB, behind a backup, and ends with a restore that is checked. Sitting 2 only reads BRIDGE CORPUS DENSE. No ComplyEaze Bridge build is used. Lines are cited at master 76705cfb. The design is `lab/2377-design` at `84aebfbf` (DESIGN.md section 4, rows L5, L7 and L8). Part A is `lab/2377-sheet` at `da5c4cac`. Part B needs none of Part A's results and may run before or after it. Until the row runs, the confidence of everything here is: unverified.

## Question

- **L5.** For a party whose name holds one special character class, does the Ledger Outstandings report, sent the book's own name escaped once as XML text, answer that party's one bill, tying to the ledger's closing balance? Which classes does the master create keep exactly as sent? Classes: `&`, `<>`, `"`, `'`, `$`, `$$`, `#`, `:`, `.`, `-`, `,`, `()`, `/`, non-ASCII, a trailing space.
- **L7.** For a party with a real New Ref bill named `On Account` and a genuine On Account amount, can the answer tell them apart?
- **L8.** On the dense book, how long does each of the three parties with the most bills take, in seconds and bytes, and does each answer hold that party's bill count from the Receivable report?

## Why

- **L5.** The design admits a party name only through `LedgerScopeName`: ASCII letters, digits and single inner spaces (DESIGN.md, deviation of 8 Nov). Each class measured here may widen a separate type for this report, and only that type. A class answered as anything other than its own bill stays refused.
- **L7.** The design counts an On Account row by `BILLREF` = `On Account` (DESIGN.md section 3). A real bill with that reference would be counted as On Account.
- **L8.** One call has 20 s of request limit (`DEFAULT_REQUEST_TIMEOUT`, transport `lib.rs:95`). The only dense measurement is one party of 248 bills.

## What it decides

| this row shows | the design |
|---|---|
| a class's party created `exact` (`present`), its bill `one created`, and its answer one bill that ties (`equal` or `negated`, the word Part A's L1 shows) | that class may enter a separate admitted type for this report, in its own pull request, citing this row |
| a class's create refused, or the name not kept `exact` | Tally does not hold that name as sent. The class stays refused; recorded. |
| a class created `exact` whose answer is empty, holds another party's bills, or does not tie | the report does not read that class as text. The class stays refused, and the run coordinator hears of it before the build. |
| L7: the New Ref bill and the On Account amount answer as two rows that differ in a field other than `BILLREF` | the design keys On Account on that field, not on the text; a change to DESIGN.md section 3 |
| L7: the two answer as rows that differ only in amount and date, or as one row | Tally does not tell them apart in this report. A party with a bill named `On Account` is refused (`party_bill_named_on_account`); recorded. |
| L8: each of the three answers in under 20 s, with that party's bill count | the one-party read fits the request limit on this book; recorded with the times |
| L8: any of the three takes 20 s or more | the build needs its own limit for this request, or a refusal; it goes to the run coordinator before the build |

A result that fits no row goes to the run coordinator as it is.

## Rules and companies

`LAB-COMMON.md` here is a byte copy of v3 (sha256 5370c6d2…, as in `lab/2119-l` and Part A), and applies in full. Where it and this sheet differ, the stricter one holds. The lab acts on:
- the owner's standing word of 27 Oct 2026;
- the owner's word of 26 Oct for curl requests in the lab (shape discovery, writes included).

**Sitting 1, THE COMPANY: `BRIDGE BILLTRAIL LAB`.** Synthetic, on licensed TallyPrime 7.2 Silver.
- If it is not on the PC: STOP.
- Its marks at B1, before the backup, are this sitting's baseline. After the restore, at B12, the marks must equal them; a difference is a STOP.
- Every party and voucher this sitting makes is new. No existing master or voucher is altered or deleted.
- Only THE COMPANY is loaded, alone, from its load in B0 to the shut in B13, except during the restore at B12, when none is.
- BOOKSFROM must be on or before 1 Oct 2025, the date of every voucher here. Else: STOP before the backup.

**Sitting 2, THE COMPANY: `BRIDGE CORPUS DENSE`.** Synthetic: the book of every #2119 row. It is never REGISTER LAB or any other book.
- Books from 1 April 2025. Tally's current date was 31 March 2026 in every #2119 row.
- At D1 and D5 the marks are V 29900 and M 328, as in `lab/2119-l`; else: STOP.
- Only THE COMPANY is loaded, alone, from its load in D0 to the shut in D6.
- It is a separate sitting: sitting 1 ends with its C10 close before sitting 2's step 0.

**Keys in Tally.** Only the Gateway, Select Company, Shut, and the Backup and Restore screens of C5 and C6 are used.
- Load a company from Select Company with the arrow keys and Enter. Press Enter only once a screenshot shows the highlighted row reads that company's name.
- No F2, Alt+F2, Escape or Ctrl+Q.
- Shut a company by the top menu's Company, then Shut, as in `lab/2119-l`.

**Every dialog this row may meet.** Answer each by mouse, once, after a screenshot:
- the shut confirmation that names THE COMPANY: Yes;
- C5's message that the backup is done: accept;
- C6's two restore prompts, `Accept ?` and the `Company data folder` prompt: Yes and Replace, as C6 quotes them.

Any other window or message in Tally or Windows: STOP, screenshot, copy its text, press no key. That includes any message during an import.

**Size bounds.** 20,000,000 bytes for `ref-receivable` (13,679,490 in `lab/2119-l-answers`). 2,000,000 bytes for every other answer.

## Files

| file | what |
|---|---|
| `ROW.md` | this sheet |
| `LAB-COMMON.md` | v3, byte copy |
| `l2377.py` | Part A's script, byte copy (`da5c4cac`): `fixed` (`marks`, `books`), `books`, `snapshot`, `lo`, `tie` |
| `l2377b.py` | this part's requests, and its `present`, `tie`, `counters` and `top3` checks |
| `bills.py` | byte copy of `lab/2119-l` (`9145f6cd`): `fixed` (DENSE's `marks` and `ref-receivable`), `ref`, `screen` |

`SHA256.txt` lists them. Check it before B0. Run the scripts with `python -I`, from a folder of their own, with the answers in another. Every request file's sha256 goes in the send log before it is sent. A special-character name is never typed in a shell: `l2377b.py tie` and `present` take it from the script.

**The requests of sitting 1.**
- `l2377.py fixed --out req1` refuses unless `marks` equals r01 of `lab/1356-party-term-and-ledger-outstandings-run-sheet` byte for byte. Only `marks` and `books` are used here. `lo-p1-0331` is used once, at B12.
- `l2377.py snapshot <BOOKSFROM> --out req1` writes `snap-0331`. That is master's unfiltered ledger snapshot (`render_native_ledger_snapshot`, `native_outstandings/request.rs:197-210`) to 20260331. `snap-0731` is not used.
- `l2377b.py writes --out req1` refuses unless its Ledger Outstandings template renders r04's 1198 bytes (sha256 `f560dba1…`). It writes:

| file | one change |
|---|---|
| `w0-income` | a ledger `L5 Income` under Indirect Incomes, bill-wise off |
| `w1-party-<class>` | a ledger under Sundry Debtors, bill-wise on, named as in `l2377b.py`'s `CLASSES` |
| `w2-bill-<class>` | one Journal on 20251001: that party Dr 100.00 with one New Ref bill `L5-<class>`, and `L5 Income` Cr 100.00 |
| `r-lo-<class>` | that party's Ledger Outstandings to 20260331 |
| `w1-party-l7` | a ledger `L7 Party` under Sundry Debtors, bill-wise on |
| `w2-bill-l7` | one Journal on 20251001: `L7 Party` Dr 100.00 with one New Ref bill named `On Account`, and `L5 Income` Cr 100.00 |
| `w3-onacct-l7` | one Journal on 20251001: `L7 Party` Dr 50.00 allocated On Account, and `L5 Income` Cr 50.00 |
| `r-lo-l7` | `L7 Party`'s Ledger Outstandings to 20260331 |

- **The classes**, one a party, each the only mark in its name: `amp` `&`, `angle` `<` and `>`, `dquote` `"`, `squote` `'`, `dollar` `$`, `dollar2` `$$`, `hash` `#`, `colon` `:`, `dot` `.`, `dash` `-`, `comma` `,`, `paren` `(` and `)`, `slash` `/`, `nonascii` `é`, `trail` a trailing space. The marks `. , - ( ) /` are the ones DESIGN.md section 1 proposed and its deviation left for this row.
- **How a name is sent.** Every value is escaped once by master's `escape_text` rule (`xml_text.rs:41-50`: `&`, `<`, `>`, `"`, `'`, CR and LF), in text and in the `NAME` attribute alike. No name is in a formula.
- **Senders on master.**
  - `w0` and `w1`: master's `LedgerCreate` message (`bridge-tally-protocol/src/ledger_create.rs:43-46`, no opening balance, GSTIN, tax type or duty head) in master's import envelope (`import_envelope.rs:30`), with that company and name.
  - `w2` and `w3`: none. The Journal is `lab/1108-a2-partial-landing`'s `templates/b-cash-journal`, with this company, date, ledgers and amounts and a REMOTEID of its own. One bill allocation is added inside the party's entry, as master's invoice writer places one (`agent_import_invoice.rs:2679`). A Journal with a bill allocation is a NEW SHAPE (C3).
  - `r-lo`: none; the design's candidate (DESIGN.md section 1).

**The requests of sitting 2.**
- `bills.py fixed --out req2` refuses unless `marks` (982 B, `040ac477…`) and `ref-receivable` (770 B, `6520d94e…`) are byte-equal to `lab/2119-l`'s.
- `l2377b.py top3 <ref answer> --out req2` writes `lo-top1` to `lo-top3`, for BRIDGE CORPUS DENSE to 20260331, and `top3.txt`. The three are the parties with the most bills in the Receivable answer whose names `LedgerScopeName` would admit; ties go by the order of the name's UTF-8 bytes.

## Sitting 1: BRIDGE BILLTRAIL LAB

One step at a time. Every answer goes to a file, whole, with its response headers (C4). Each send waits for the one before to end. All times are in UTC.

**A write** means these five acts, in order, and nothing else:
1. Send `marks`: EXPECT one COMPANY row, THE COMPANY's (C3).
2. Send the write file, once in this item's life (C3).
3. Screenshot the whole Tally window: EXPECT no dialog.
4. `l2377b.py counters <answer>`: EXPECT `one created` and `lineerrors -`.
5. Send `marks` again. Record ALTVCHID and ALTMSTID.

A `counters` FAIL, or `not one created`, on a `w1-party-<class>` write is a finding, not a STOP. Record its line, keep the answer whole, skip that class's `w2` and `r-lo`, and go on to the next class. On any other write it is a STOP.

- **B0. Before (C2), no company loaded.** Screenshot; the process list; port 9001; `GET /status`. Load THE COMPANY alone. Screenshot the Gateway: EXPECT THE COMPANY alone. Record Tally's release, its current date and period.
- **B1. Marks, books and the plan.**
  - `l2377.py fixed --out req1`: EXPECT exit 0.
  - Send `marks`, then `books`. EXPECT one COMPANY row each, the same GUID. Record ALTVCHID and ALTMSTID: this is the baseline.
  - `l2377.py books`: EXPECT one YYYYMMDD, on or before 20251001; else STOP.
  - `l2377.py snapshot <BOOKSFROM> --out req1`, then `l2377b.py writes --out req1`: EXPECT exit 0 from each. Record every line.
- **B2. The snapshot before.** Send `snap-0331`. `l2377b.py present <snap>`: EXPECT every line `absent`. A line not `absent` is a STOP before the backup: the book already holds one of these names.
- **B3. Backup (C5).** To a new empty local folder `C:\lab\2377-b\backup-1`. Record the file's name, folder, bytes, sha256, and the time the Restore list will show. Then `marks`: EXPECT the baseline.
  - Book after B3: as found.
- **B4. W0.** A write of `w0-income`.
  - Book after B4: `L5 Income`.
- **B5. L5, one class at a time,** in the order of the classes above (`amp` first, `trail` last). For each class:
  - (a) a write of `w1-party-<class>`;
  - (b) a write of `w2-bill-<class>`;
  - (c) send `r-lo-<class>` once. Then `l2377.py lo <answer>`. A `lo` FAIL is a finding, not a STOP: record it and keep the answer whole;
  - (d) send `snap-0331`, then `l2377b.py present <snap>` and `l2377b.py tie <class> <lo answer> <snap>`. Record both. A `tie` FAIL is a finding.
  - Book after each class: what came before, and that class's party and its bill.
- **B6. L7.**
  - A write of `w1-party-l7`.
  - A write of `w2-bill-l7`.
  - A write of `w3-onacct-l7`.
  - Send `r-lo-l7` once. Then `l2377.py lo`, `bills.py screen <answer>`, and each `BILLFIXED` group with the fields that follow it, as `screen` prints them.
  - Send `snap-0331`, then `l2377b.py tie l7 <lo answer> <snap>`.
  - EXPECT a sum of −150.00 if both rows answer. Record the rows whatever they are.
  - Book after B6: B5's, `L7 Party` and its two vouchers.
- **B7. Marks after the writes.** Send `marks`. Record ALTVCHID and ALTMSTID.
- **B8.** Nothing is sent. Copy every answer of B4 to B7 into the evidence folder. Check each file's sha256 against the send log.
- **B9. Shut.** Screenshot the Gateway. Shut THE COMPANY as under Keys. EXPECT no company loaded.
- **B10. Restore plan (C6).** Check the backup file's sha256 against B3's, and that its folder holds that one file only. Write `restore-plan.txt`. List THE COMPANY's own data folder.
- **B11. Restore (C6),** of B3's backup, by the operator, exactly as C6 says. Then list THE COMPANY's data folder again. Load THE COMPANY alone. Screenshot the whole Tally window, and the Gateway: EXPECT THE COMPANY alone, no dialog.
- **B12. The restore checked.**
  - Send `marks`: EXPECT B1's GUID, ALTVCHID and ALTMSTID, the baseline.
  - Send `snap-0331`: `l2377b.py present`: EXPECT every line `absent`.
  - Send `lo-p1-0331` once: `l2377.py lo`: EXPECT bills 6, on_account 1, sum −26000.00, as on 27 Oct.
  - Any one not met: STOP. The way back is B3's backup, restored again only on the run coordinator's word.
  - Book after B12: as found at B0.
- **B13. Close (C10).** Screenshot the Gateway. Shut THE COMPANY as under Keys. One `tally.exe` on 9001; `GET /status` 200; no company loaded. Nothing is sent after the shut.

## Sitting 2: BRIDGE CORPUS DENSE, reads only

- **D0. Step 0 (C2), no company loaded.** As B0, for this company. EXPECT the current date 31 March 2026.
- **D1. Marks.** `bills.py fixed --out req2`: EXPECT exit 0 and the two lines above. Send `marks`: EXPECT one COMPANY row, V 29900 and M 328.
- **D2. The Receivable report.** Send `ref-receivable` once, with its 20,000,000-byte bound. Record its first-byte and total seconds, bytes and sha256. `bills.py ref <answer>`: record the line.
- **D3. The three parties.** `l2377b.py top3 <ref answer> --out req2`: EXPECT exit 0. Record its lines. `top3.txt` stays on the lab PC.
- **D4. L8, one party a send.** For k = 1, 2, 3: send `lo-top<k>` once by C3's line with the timing fields added, as `lab/2119-l` does: `-w "%{http_code} %{size_download} %{time_starttransfer} %{time_total}"`. Record its first-byte and total seconds, bytes and sha256. Then `l2377.py lo <answer>`: EXPECT bills equal to `top3.txt`'s count for that party. Screenshot the whole Tally window after each send: EXPECT no dialog.
- **D5. Marks again.** Screenshot the Gateway: EXPECT THE COMPANY alone. Send `marks`: EXPECT V 29900 and M 328.
- **D6. Close (C10).** As B13.

## What to report

| step | result |
|---|---|
| B0, D0 | release, date, period; the process and port lines |
| B1 | the script lines; GUID, ALTVCHID, ALTMSTID; BOOKSFROM |
| B2 | `present`'s lines |
| B3 | the backup's name, folder, bytes, sha256, time; the marks |
| B4, B6 writes | each `counters` line and the marks after it |
| B5 | one line per class: create `counters`; bill `counters`; `lo`; `present` for that class; `tie` (or each FAIL line) |
| B6 | the rows of `r-lo-l7` as `screen` prints them; `lo`; `tie` |
| B7 | the marks |
| B10 to B12 | the restore plan, the two folder listings, the marks, `present`, `lo` of `lo-p1-0331` |
| B13, D6 | the close lines |
| D1, D5 | the marks |
| D2 | seconds, bytes, sha256, `ref` line |
| D3 | `top3`'s lines (names stay in `top3.txt` on the lab PC) |
| D4 | per party k: seconds, bytes, sha256, `lo` line, and its count against `top3.txt` |

Put any STOP, skip, finding or EXPECT not met in the first line. Fill the "What it decides" row this result meets, per class for L5. Every number is read from its file.

## Stop rules

C7 and C8 of LAB-COMMON. Also a STOP:
- any dialog or message in Tally or Windows other than those listed above;
- a second `tally.exe`;
- a write sent when the `marks` read just before it did not hold exactly THE COMPANY's row;
- marks that move where the sheet says they do not, or that do not return at B12;
- a request with no answer in 10 minutes (C3: never sent again);
- an answer over its bound;
- a request file whose sha256 is not the one printed;
- any script FAIL, except where a step names it a finding.

After a STOP nothing more is sent, except the state reads once, as C8 allows. A STOP between B4 and B8 leaves the book holding the writes made so far, as each step's "Book after" says. The route back is B9 to B13, on the run coordinator's word.

## Limits

- One release, Silver. Each book is read in one sitting.
- **One mark a class.** A name with two classes is not measured. Neither is a doubled inner space (Part A's L4), a leading space, or a line break.
- **The answers are this report's only.** How a class behaves in a TDL formula, `CHILDOF` among them, is not measured. A class this row admits widens nothing that takes a formula.
- **The bills are Journal bills.** Sales or receipt vouchers, a payable party, and bill-wise off are not in this part.
- **L8's three parties are the largest by bill count in the Receivable report only.** A party with many bills on the payable side is not chosen.

## Keep

On `lab/2377-b-answers`, with `SHA256.txt`:
- every request file sent, except `lo-top1` to `lo-top3`, which hold dense party names and are listed by size and sha256 only;
- every answer of sitting 1, whole, with its headers. THE COMPANY is synthetic and every party in it was made here;
- sitting 2's `marks` answers whole. `ref-receivable` and `lo-top<k>` answers are listed by size and sha256 only, as `lab/2119-l-answers` keeps them;
- the script lines, the send log, `restore-plan.txt`, the folder listings, and the screenshots listed by name, size and sha256;
- a `RESULTS.md` holding the table above.

Notes carry no user names and no machine paths beyond the lab folder. Commits carry no co-author or generated-with line. Send the sha first; push on the run coordinator's yes.

## Deviation, 8 Nov 2026

Part A's first sitting stopped at `pick`: the shared `ledgers()` of `l2377.py` walked the whole answer and reached the bare LEDGER count line under BODY/DESC/CMPINFO, which is not a row. `l2377.py` here is now byte-equal to the corrected copy on `lab/2377-sheet` at 3e865846, so every `tie` reads only BODY/DATA/COLLECTION/LEDGER under an ENVELOPE root. `present()` in `l2377b.py` reads the same rows: before, the count line counted as a name `""`, which met the "no LEDGER in the snapshot" guard when there was no row. No request changes. `SHA256.txt` carries the new hashes of `l2377.py`, `l2377b.py` and this file.
