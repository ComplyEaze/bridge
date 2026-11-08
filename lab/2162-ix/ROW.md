# Rows 2162-L1 to L5: what moves the company's marks, whether an AlterID term serves the number read, and whether repeated reads and reloads are stable. The evidence the invoice-number index needs before code

Not for merge. Part of #2162. Written 8 Nov 2026. The design is `lab/2162-index-design/DESIGN.md`, section 5, on this branch. Lines are cited at master 9a4cf1f7. No ComplyEaze Bridge build is used. Requests go by `timed.py`, one at a time. The rules of `LAB-COMMON.md` (v3, byte-equal to `lab/2377-b/LAB-COMMON.md`) apply throughout. Confidence of everything here until the rows run: unverified.

## Question
The index is trusted only while the company's marks, V (ALTVCHID) and M (ALTMSTID), equal the marks it was built at. It is updated locally only when V moved by exactly this session's own post. Five things are not measured:
- **L1.** Does every screen action that can change an invoice's number, date or existence move V? Gateway writes were measured: create +1, alter +1, cancel +1, re-date +1, delete +2 (§11c.5's table). Screen actions were measured once.
- **L2.** Does changing a voucher type's parent move M, and does it change the class of that type's vouchers?
- **L3.** Does `$AlterID > K` as a term of the number read's filter return exactly the rows whose ALTERID exceeds K, and what does it cost?
- **L4.** Do the marks survive a company reload and a restart of Tally?
- **L5.** Are twelve sends of one dense month, with marks between them, all element for element equal?

**Correction to the design (8 Nov 2026).** Y0's fetch list is `DATE, VOUCHERNUMBER, VOUCHERTYPENAME, ISCANCELLED, IRN, IRNACKNO, IRNACKDATE`. It carries no ALTERID, so section 1's "its `ALTERID`" is not available from Y0. L3a adds that one field and nothing else; L3b then adds the one filter term. The index's freshness rests on the company marks, not on row AlterIDs, so the rest of the design stands.

## Books
Synthetic only: `BRIDGE BILLTRAIL LAB` (Part 1, which writes, behind a backup and a checked restore), `BRIDGE PILOT LAB` (Part 2, reads only) and `BRIDGE CORPUS DENSE` (Part 3, reads only). No client book. Any other company, another GUID than the lab's own record of each book, or Education mode: STOP, that part has no book.

## Kit
`SHA256.txt` lists every file with sha256 and size: check it first.

| file | what |
|---|---|
| `LAB-COMMON.md` | v3, byte-equal to `lab/2377-b/LAB-COMMON.md` |
| `timed.py` | S6's, byte-equal |
| `template-y-pilot.utf16le.xml` | S6's template, byte-equal (S5's `Y-all.xml`) |
| `M-pilot.xml`, `M-dense.xml` | S5's and S6's `M-marks.xml`, byte-equal |
| `M-billtrail.xml` | `M-dense.xml` with the company name changed, once |
| `gen8.py` | writes every request below from the template and `M-dense.xml`, asserting each change occurs exactly as often as expected. `gen8.py k <marks answer>` writes `L3b-pilot.xml` and `k.txt` from that answer's ALTVCHID, read at `./BODY/DATA/COLLECTION/COMPANY`, digits only |
| `A-Y0-year.xml`, `B-Y0-month.xml` | S7's files of those names, byte-equal (Y0, pilot year; Y0, dense April 2025) |
| `L3a-pilot.xml` | `A-Y0-year.xml` with `, ALTERID` at the end of its fetch list: one variable |
| `L3b-pilot.xml` | written in step P4: `L3a-pilot.xml` with ` AND $AlterID > K` at the end of its filter, K = the step's ALTVCHID less 10 |
| `L3b-dense.xml` | the same shape for the dense book, 1 Apr 2025 to 31 Mar 2026, with K = 29800. **A shape Tally has not been sent** |
| `Y0-bt-20251001.xml`, `Y0-bt-20251002.xml` | Y0 for BRIDGE BILLTRAIL LAB over one day each |
| `count8.py` | S7's `count7.py` reading rows only at `./BODY/DATA/COLLECTION/<ROW>` (CMPINFO's bare count lines are never rows), with this row's comparisons |

count8.py was run, when the sheet was written, on S6's captured answers renamed to this sheet's names: the dense marks (GUID, 29900, 328) and two April sends (2,430 rows each, equal).

## Part 1: BRIDGE BILLTRAIL LAB, one sitting, writes behind a backup (L1, L2)
Keys and the backup and restore follow `lab/2377-b/ROW.md` C5 and C6 exactly. Every voucher is dated 1 Oct 2025: BOOKSFROM must be on or before it, or STOP before the backup. **After every key step, send `M-billtrail.xml`** (label `bm<n>`) and record V and M.
- **B0.** Load THE COMPANY alone. `bm0`: the baseline.
- **B1. Backup** (C5) to a new empty folder. Record its name, bytes and sha256. `bm1` = `bm0`.
- **k1, create.** By hand, Accounting Invoice mode: one Sales voucher dated 1 Oct 2025, number `IX-0001`, a party and a sales ledger the book already holds, one line of 100.00. If the book holds no Sales type or no such ledgers: STOP. Then send `Y0-bt-20251001.xml` (`by1`).
- **k2, narration only.** Open `IX-0001`, add the narration `ix`, save. `bm`, then `by2`.
- **k3, number change.** Change its number to `IX-0002`, save. `bm`, then `by3`.
- **k4, re-date.** Change its date to 2 Oct 2025, save. `bm`, then send `Y0-bt-20251002.xml` (`by4`).
- **k5, cancel.** Cancel it (Tally's own Cancel on the voucher). `bm`, then `by5` with the 2 Oct read.
- **k6, delete.** Delete it. `bm`, then `by6` with the 2 Oct read.
- **k7, L2.** Create a voucher type `IX SALES` under Sales, Automatic numbering. `bm`. Key one voucher in it dated 1 Oct 2025, number `IX-0003`, as k1. `bm`, then `by7`.
- **k8, L2.** Alter `IX SALES`: set its parent ("Select type of voucher") to Journal. If Tally refuses or the field is not editable, record its words and screenshot, and that is the result. `bm`, then `by8`.
- **B10 to B12. Restore** (C6) of B1's backup, checked as 2377-b's B12: the marks equal `bm0`, and `by9` (the 1 Oct read) holds no `IX-` number.

## Part 2: BRIDGE PILOT LAB, reads only (L3, L4)
- **P1.** Load alone. Send `M-pilot.xml` (`pm1`).
- **P2.** `A-Y0-year.xml` (`p2`).
- **P3.** `L3a-pilot.xml` (`p3`). An answer that is not a collection of rows, or an Unknown Request: record it whole, and skip P4 and P5.
- **P4.** `python3 -I gen8.py k a-pm1-M-pilot.xml`, then add `L3b-pilot.xml` and `k.txt` to the notes with their sha256.
- **P5.** `L3b-pilot.xml` (`p5`), **one send, watched**. Screenshot Tally's window as soon as the answer is written. Any dialog: STOP.
- **P6, L4.** Shut the company (Shut Company). Load it again. `pm6`. Then close Tally by its own Quit, start it again, and load the company alone. `pm7`.

## Part 3: BRIDGE CORPUS DENSE, reads only (L3 cost, L5)
- **D1.** Load alone. `M-dense.xml` (`dm1`): EXPECT the book's GUID, V 29,900 and M 328. Other marks: STOP, because L3b-dense's K is fixed for this mark.
- **D2.** `L3b-dense.xml` (`d2`), one send, watched, as P5.
- **D3, L5.** Twelve times: `B-Y0-month.xml` (labels `l501` to `l512`), each followed by `M-dense.xml` (`dm2` to `dm13`).
- **D4.** Copy S6's kept `run/a-s6-Y0-year.xml` beside count8.py as `s6-Y0-year.xml`. count8 checks its sha256 (1db925db…, `lab/2162-s6-answers` kept-local.txt).
- **D5.** Shut by Shut Company. One tally.exe owns 9001.

## Stop rules
LAB-COMMON v3's rules, and these:
- any dialog or prompt that this sheet does not name;
- a marks read that fails or lacks a COMPANY row. That is no answer, never "unchanged";
- a restore whose marks differ from `bm0`;
- no answer in 15 minutes. Press nothing and kill nothing. A read given up by its client keeps the gateway busy.

## Record
- **Every send:** bytes, sha256, rows, and curl's first-byte and total seconds.
- **The marks after each key step:** V and M, and their difference from the step before.
- **Every key step:** Tally's own words, and the screen where one was asked for.
- **count8.py's comparison lines.**

## Decides
- **L1.** Every action k1 to k6 moves V by at least 1, so the index's equality check is sound. An action that changes a number, date or existence and moves neither mark makes the index unusable as designed. Record which.
- **L2.** k8 moves M, and `by8`'s class for `IX-0003` changes or not. If k8 moves M, the rule "M unchanged" covers a class change. If a class change moves neither mark, the design changes before code.
- **L3.**
  - p3 equals p2 less ALTERID, with one numeric ALTERID on every row: the fetch change serves.
  - p5 equals p3's rows above K: the term serves. With a dialog, or not equal, the term is not used.
  - d2's rows all lie above 29800 and are found in S6's year, and its seconds are the delta read's cost on a dense book.
- **L4.** `pm6` and `pm7` equal `pm1`: the marks survive a reload and a restart. That alone does not make the index survive one; the design still drops it at a reload.
- **L5.** All twelve equal, with `dm2` to `dm13` equal to `dm1`: the evidence for the pair ruling.

## Keep
On `lab/2162-index-answers`, with `SHA256.txt`:
- each answer of 10 MB or less, whole; larger answers by name, size and sha256 in `kept-local.txt`;
- count8.py's whole output after each part;
- the notes in UTC, the backup and restore records, and the screenshots by name, size and sha256.

Send the sha to the run coordinator before any push.

Row: lab/2162-ix/
