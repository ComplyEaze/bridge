# Row 2377-L, Part A: one party's bills through Tally's Ledger Outstandings report, against the ledger's closing balance, on BRIDGE BILLTRAIL LAB, by curl

Not for merge. Part of #2377. Reads only: no write, no import, no key outside the screens named below, no ComplyEaze Bridge build. Lines are cited at master 76705cfb. The design is `lab/2377-design` at `cac89a4a` (DESIGN.md, section 4). Confidence of everything here until the row runs: unverified.

## Question

On a second book, does one party's Ledger Outstandings answer tie to that ledger's closing balance in master's ledger snapshot, at two dates? And what does the report answer for:
- a party ledger with no open bills;
- a credit-balance party;
- a name that is no ledger;
- a name that differs from a ledger's only in case, or only in a doubled space?

## Why

The design releases a party's bills only when the sum of `BILLCL`, On Account included, equals the ledger's `CLOSINGBALANCE` (DESIGN.md section 3), and an empty answer only when that balance is zero. Neither has been measured for this report. Row L3 decides the design: if an unknown name gives an error line, that becomes a refusal of its own; if it gives an empty envelope, the tie is the only guard.

## What it decides

| this row shows | the design |
|---|---|
| L1 ties at both dates (`equal` or `negated`, the same word at both), L2 is empty with a zero balance, L3 is an error line or an empty envelope, and L4's answers are byte-equal to L1's or empty | stands as written. The sign the tie uses is the word L1 shows. |
| L1 `differ` at either date | the tie cannot gate a release. The build waits for a design change; nothing is built. |
| L2 holds bills while its balance is zero, or L3 answers with another ledger's bills | an answer cannot be trusted by name alone. The design returns to the run coordinator. |
| L4 answers with P1's bills for either variant | Tally folds names here. The builder sends only the book's own name, which the design already says; recorded. |

A result that fits no row goes to the run coordinator as it is.

## Rules and company

`LAB-COMMON.md` here is a byte copy of v3 (sha256 5370c6d2…, as in `lab/2119-l`), and applies in full. Where it and this sheet differ, the stricter one holds. The lab acts on:
- the owner's standing word of 27 Oct 2026;
- the owner's word of 26 Oct for curl requests in the lab (shape discovery).

**THE COMPANY** is `BRIDGE BILLTRAIL LAB`, synthetic, on licensed TallyPrime 7.2 Silver, as read on 27 Oct (`lab/1356-party-term-and-ledger-outstandings-answers` at `b761d18e`).
- If it is not on the PC: STOP.
- Step S1's marks are recorded as the row's baseline. They must be equal at S1 and S8. A difference is a STOP; the run coordinator decides.
- Only THE COMPANY is loaded, alone, from its load in S0 to the shut in S9.
- Its current date and period are as the lab finds them. They are recorded, not changed.

**Keys in Tally.** Only the Gateway and Select Company are used. Load THE COMPANY from Select Company with the arrow keys and Enter. Press Enter only once a screenshot shows the highlighted row reads `BRIDGE BILLTRAIL LAB`. No F2, Alt+F2, Escape or Ctrl+Q. Shut THE COMPANY at S9 by the top menu's Company, then Shut, as in `lab/2119-l`.

**Every dialog this row may meet.** Only one is expected: at S9, a confirmation to shut that names THE COMPANY, answered Yes once, by mouse, after a screenshot. Any other window or message in Tally or Windows: STOP, screenshot, copy its text, press no key.

## Files

| file | what |
|---|---|
| `ROW.md` | this sheet |
| `LAB-COMMON.md` | v3, byte copy |
| `l2377.py` | writes the requests; reads BOOKSFROM; picks L2's and L6's parties from the snapshot; reads each answer; ties it |
| `bills.py` | byte copy of `lab/2119-l` (`9145f6cd`); its `screen` command prints an answer as Tally's screen shows it |

`SHA256.txt` lists them. Check it before S0. Run the scripts with `python -I`, from a folder of their own, with the answers in another.

`l2377.py fixed` refuses unless three of its requests equal the 27 Oct files of `lab/1356-party-term-and-ledger-outstandings-run-sheet` byte for byte:

| request | bytes | sha256 | sender on master |
|---|---|---|---|
| `marks` | 984 | `501c3094…` (r01) | the marks read of earlier rows |
| `books` | 968 | `d2e54607…` | none: the marks collection with `FETCH GUID,BOOKSFROM`, a field of master's `AUDIT_COMPANY_FETCH` (`xml_read_profiles.rs:490`) |
| `lo-p1-0331` | 1198 | `f560dba1…` (r04) | none: the design's candidate (DESIGN.md section 1) |
| `lo-p1-0731` | 1198 | `7c093656…` (r05) | none |
| `lo-unknown` | 1198 | `5ed21185…` | none: the name `Lab Party Q9` |
| `lo-case` | 1198 | `5507e5cd…` | none: `lab party p1` |
| `lo-space` | 1200 | `f0990a5a…` | none: `Lab  Party P1`, two spaces |

`snapshot` writes `snap-0331` and `snap-0731`: master's unfiltered ledger snapshot (`render_native_ledger_snapshot`, `native_outstandings/request.rs:197-210`), from BOOKSFROM to each date. `pick` writes `lo-z` and `lo-c`. Their hashes are the ones the script prints. Every request file's sha256 goes in the send log before it is sent.

## Steps

One at a time. Every answer goes to a file, whole, with its response headers (C4). Each send waits for the one before to end. All times are in UTC.

- **S0. Before (C2), no company loaded.** Screenshot; the process list; port 9001; `GET /status`. Load THE COMPANY alone. Screenshot the Gateway: EXPECT THE COMPANY alone. Record Tally's release, its current date and period.
- **S1. Marks and books.** `l2377.py fixed --out req`: EXPECT exit 0 and the table's lines. Send `marks`, then `books`. EXPECT one COMPANY row each, the same GUID. Record ALTVCHID and ALTMSTID. `l2377.py books`: EXPECT one YYYYMMDD; else STOP.
- **S2. The snapshot.** `l2377.py snapshot <BOOKSFROM> --out req`. Send `snap-0331`, then `snap-0731`. `l2377.py pick <snap-0331> --out req`: record its lines and `picks.txt`. A FAIL is a STOP: Q9 exists, or P1 does not.
- **S3. L1, P1 at two dates.**
  - Send `lo-p1-0331` twice: EXPECT byte-equal answers.
  - Send `lo-p1-0731` once.
  - For each: `l2377.py lo`, `bills.py screen`, and `l2377.py tie <lo> <snap of the same date> "Lab Party P1"`. Record the word.
  - EXPECT (27 Oct): 6 bills, 1 On Account, sum −26000.00 at 0331; 5 bills, 1 On Account, sum −37000.00 at 0731.
- **S4. L2, a bill-wise party with a zero balance.** If `picks.txt` says `z none`: skipped, recorded. Else send `lo-z` once, then `lo` and `tie` against `snap-0331`.
- **S5. L6, a credit-balance party.** If `c none`: skipped, recorded. Else send `lo-c` once, then `lo`, `screen` and `tie` against `snap-0331`.
- **S6. L3, a name that is no ledger.** Send `lo-unknown` once.
  - Record the answer's bytes, its sha256, its response headers (`RESPSTATUS` among them) and `l2377.py lo`.
  - A `lo` FAIL here is a finding, not a STOP: record its line, and keep the answer whole.
- **S7. L4, two spellings.** Send `lo-case`, then `lo-space`, once each. For each: `l2377.py lo`, and whether the answer is byte-equal to S3's first `lo-p1-0331` answer.
- **S8. Marks again.** Screenshot the Gateway: EXPECT THE COMPANY alone. Send `marks`: EXPECT S1's ALTVCHID and ALTMSTID.
- **S9. Close (C10).** Screenshot the Gateway. Shut THE COMPANY as under Keys. One `tally.exe` on 9001; `GET /status` 200; no company loaded. Nothing is sent after the shut.

## What to report

| step | result |
|---|---|
| S0 | release, date, period; the process and port lines |
| S1 | the `fixed` lines; GUID, ALTVCHID, ALTMSTID; BOOKSFROM |
| S2 | the snapshot sizes and sha256; `pick`'s lines |
| S3 (L1) | each answer's size, sha256, `lo` and `tie` lines; the two 0331 answers byte-equal or not |
| S4 (L2), S5 (L6) | the same, or "skipped: none" |
| S6 (L3) | size, sha256, headers, `lo` line or its FAIL |
| S7 (L4) | `lo` lines; byte-equal to L1's or not |
| S8 | the marks, equal or not |
| S9 | the close lines |

Put any STOP, skip or EXPECT not met in the first line. Fill the "What it decides" row this result meets. Every number is read from its file.

## Stop rules

C7 and C8 of LAB-COMMON. Also a STOP:
- any dialog or message in Tally or Windows other than S9's shut confirmation;
- a second `tally.exe`;
- marks that move;
- a request with no answer in 10 minutes (C3: never sent again);
- a request file whose sha256 is not the one printed;
- any `l2377.py` or `bills.py` FAIL outside S6.

After a STOP nothing more is sent, except the marks once, as C8 allows.

## Limits

- One book, one sitting, Silver, one release.
- **Not in Part A.** Each of these needs its own sheet:
  - L5, names with special characters, and L7, a real bill whose reference is `On Account`. Both need lab-made parties and vouchers, so a sheet with writes, a backup and a restore.
  - L8, the dense book's largest parties. It needs a second company, and a whole Receivable report to choose them.
- L2 and L6 run only if the book already holds such a party. A skip is reported, not made up.
- The snapshot's start is the book's BOOKSFROM. Whether the report's own period starts there is not measured; the tie tests it only indirectly.

## Keep

On `lab/2377-l-answers`, with `SHA256.txt`:
- every request file sent, and every answer, whole, with its headers. THE COMPANY is synthetic.
- the script lines, the send log, the screenshots listed by name, size and sha256;
- a `RESULTS.md` holding the table above.

Notes carry no user names and no machine paths beyond the lab folder. Commits carry no co-author or generated-with line. Send the sha first; push on the run coordinator's yes.
