# Lab run sheet: an invoice's masters read by ledger, on BRIDGE PILOT LAB as it is and then grown to 10,000 and 20,000 extra ledgers (#1331). SYNTHETIC COMPANY ONLY. READS, THEN WRITES, THEN RESTORED. Not for merge.

About 90 minutes. `LAB-COMMON.md` in this folder (v3, sha256 `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6`) applies in full; where this sheet and it differ, the stricter holds. Part A is reads only. Part B backs THE COMPANY up (BK0), imports plain ledgers into it, reads, backs the grown book up (BK1, kept), and restores BK0. Nothing is keyed in Tally beyond the Backup and Restore screens and loading the company. No ComplyEaze Bridge build is used: every request is sent raw by curl.

**WHAT IT DECIDES.** A Sales invoice build reads the whole ledger compliance listing at each of its checks, so the build refuses a book whose master mark is above 5,000 unless it holds at most 2,000 ledgers (#1331). The ledger-rate read is already narrowed to the parents of the invoice's ledgers, but on a book with thousands of customers the customer's own parent (Sundry Debtors) is itself too large for one read, so a parent filter does not admit such a book. This sheet measures the alternative: a filter that names the invoice's ledgers themselves. It records:
1. **The shape.** Whether Tally answers a `$GUID = "…" OR …` filter, and a `$Name = "…" OR …` filter, on the ledger-rate request and on the compliance master request, with exactly the named ledgers, each row equal to the same ledger's row from the parent-filtered request already measured.
2. **A missing ledger.** How a filter naming a GUID or a name the book does not hold answers: the other rows only, and an empty collection when it names nothing that exists.
3. **Case.** Whether `$Name` matches a name spelled with another letter case on this request.
4. **Cost on a large book.** Bytes and seconds of each filtered read with 10,018 and 20,018 ledgers in the book, against the same read with 18. The filter is evaluated on every ledger, so its time can grow with the book even though the answer does not.
5. **The whole reads every check still sends.** Bytes and seconds of the import's ledger catalogue and of the group list at each size: the catalogue is read whole at every check of every import, so it, not the listing, bounds how large a book can post.

**THE COMPANY.** `BRIDGE PILOT LAB`, GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85` (as listed in `scripts/merge-gate-synthetic-uuids.json`).
- **At the read of 30 October 2026** (`lab/2145-spanned-replay-recapture-answers`): `ALTMSTID` 245, `ALTVCHID` 68, 18 ledgers.
- **The five ledgers of the lab invoice** the filtered requests name, with their GUID suffix after the company GUID and their parent:

| ledger | GUID suffix | parent |
| --- | --- | --- |
| `BRIDGE Walk-in` | `-000000d0` | `Sundry Debtors` |
| `BRIDGE Goods 18%` | `-000000d3` | `Sales Accounts` |
| `BRIDGE CGST 9%` | `-000000d5` | `Duties & Taxes` |
| `BRIDGE SGST 9%` | `-000000d7` | `Duties & Taxes` |
| `BRIDGE Round Off` | `-000000d8` | `Indirect Expenses` |

- Other marks than those of 30 October are recorded, not a STOP. **If step A2's catalogue does not hold each of these five with that GUID and that parent, or holds any ledger whose name begins `Lab Scale Debtor`: STOP.** The book is not the one this sheet was written for.

**THE REQUESTS** (in `requests/`; check each against `SHA256.txt` before its first send; a mismatch = STOP). Each is UTF-16LE with a byte-order mark. Each is a request already sent to this book, unchanged or with ONE part changed, written by `make-1331-sheet.py` (in this folder) from those files; no byte was typed by hand.
- **Controls, sent to this book before, unchanged** (each byte-identical to the file named):
  - `m-marks.xml` (976 bytes): the marks read (`lab/1342-capture-requests` `r02`). READ BOUND 8,192 bytes.
  - `k-catalogue-v2.xml` (2,106 bytes): the import's ledger catalogue, the read every import check sends whole (`r13`). READ BOUND: Part A 2,000,000 bytes; at 10,018 ledgers 16,000,000 bytes; at 20,018 ledgers 32,000,000 bytes.
  - `g-groups.xml` (1,066 bytes): the group list (`r07`). READ BOUND 2,000,000 bytes.
  - `c-master-whole.xml` (1,736 bytes): the compliance master request, whole book (`r05`). READ BOUND 2,000,000 bytes. **Part A only: never sent once an import has been sent.**
  - `p1-rates-parents-four.xml` (2,404 bytes): the ledger-rate request filtered to the four parents of the lab invoice, byte-identical to the committed `pilot-lab-ledger-rates-scoped-hsn.request` fixture. READ BOUND 2,000,000 bytes. **Part A only: its parents include Sundry Debtors, so it is never sent once an import has been sent.**
- **`e-extent.xml` (1,640 bytes):** THE COMPANY's counts and marks (`NUMLEDGERS`, `NUMGROUPS`, `ALTMSTID`). It is `lab/1233-large-stock-book`'s `m-marks-S` with only the company name and GUID changed. READ BOUND 8,192 bytes.
- **New filter shapes** (each is `p1` with only its `SYSTEM` formula and `FILTERS` name changed, or `c-master-whole` with that pair added; the fetch list and the window are unchanged):
  - `g1-rates-guid-five.xml`: the rate request, `$GUID = "…" OR …` naming the five ledgers.
  - `g2-rates-guid-four-and-absent.xml`: the first four of them and the GUID suffix `-0000ffff`, which no ledger of this book carries.
  - `g3-rates-guid-absent-only.xml`: only that GUID.
  - `g4-master-guid-five.xml`: the compliance master request with the `g1` filter.
  - `n1-rates-name-five.xml`: the rate request, `$Name = "…" OR …` naming the five by their catalogue spelling.
  - `n2-rates-name-case-changed.xml`: `$Name = "BRIDGE WALK-IN"` alone (the book spells it `BRIDGE Walk-in`).
  - `n3-rates-name-absent-only.xml`: `$Name = "BRIDGE No Such Ledger"` alone.
  - `p2-rates-parents-three.xml`: `p1` without Sundry Debtors: the parent filter that stays small on the grown book, for comparing its time with `g1`.
  - READ BOUND 200,000 bytes for each of these (five rows of the rate request were about 4.4 KB each when measured).
- **The imports, `imports/i001-debtors.xml` to `imports/i040-debtors.xml`** (191,666 bytes each): each is one "All Masters" import of 500 ledgers under `Sundry Debtors`, bill-wise off, no opening, named `Lab Scale Debtor` and a six-letter code of consonants (`BBBBBB` onward, a few codes skipped so that no name contains a word of the pre-push scan's lists; 20,000 distinct names in all, none spelled like another in any case). The ledger message is the captured Create shape of `crates/bridge-tally-protocol/src/ledger_create.rs`; the envelope is `lab/1233-large-stock-book`'s. Each WRITES and is sent ONCE in this item's life (C3). READ BOUND 65,536 bytes for each answer.
- **Request-ID check before the first send**, in this folder: `python -I check-request-ids.py requests`. EXPECT `census: ok 14, skipped 40, FAIL 0`; the imports are skipped (check by eye that each is `Import Data`, report `All Masters`). Anything else = STOP, nothing sent.
- **How they are sent:** by curl under C3, one at a time, with ONE difference from C3's line: the `-w` format is `"%{http_code} %{size_download} %{time_total}"`, so each send also records its seconds. Every send of a new filter shape (`g1` to `g4`, `n1` to `n3`) is followed by a screenshot of the whole Tally window before the next request (C3).

**READING THE FILTERED ANSWERS** (C4 applies): for each answer, a table of its `LEDGER` rows with `NAME` (printed exactly, with its length), `GUID` and `PARENT`, and the count; the bytes and the seconds. "Equal to the control's row" means the same element paths with the same values, compared by parsing both, as #1331's capture of 10 October did; a difference is listed by element path, never by value.

## Part A: the book as it is (reads only)

A0. **Start.** C1, then C2 with THE COMPANY loaded alone. Run the request-ID check. GET /status.

A1. `m-marks` (**A-m0**): one COMPANY row, THE COMPANY's GUID; record `ALTMSTID` and `ALTVCHID`. Then `e-extent`: record `NUMLEDGERS` (EXPECT 18) and `NUMGROUPS`.

A2. `k-catalogue-v2`: the table of every row (name, GUID, parent, bill-wise flag). EXPECT the five ledgers as in THE COMPANY's table, and no name beginning `Lab Scale Debtor` (else STOP, as above). EXPECT as many rows as `NUMLEDGERS`.

A3. `g-groups`: record the row count and bytes.

A4. `c-master-whole` (control for `g4`), then `p1-rates-parents-four` (control for `g1` to `n3`). EXPECT `p1`'s rows to be exactly the catalogue's ledgers under the four parents.

A5. `g1-rates-guid-five`. EXPECT exactly the five rows, each equal to the same GUID's row of `p1`, no `LINEERROR`.

A6. `g2-rates-guid-four-and-absent`. EXPECT exactly the four rows `-000000d0`, `-000000d3`, `-000000d5`, `-000000d7`, each equal to its row of `p1`, and no `LINEERROR`.

A7. `g3-rates-guid-absent-only`. EXPECT a complete envelope whose `COLLECTION` holds no `LEDGER` row, and no `LINEERROR`. Record its bytes and sha256: this is how "nothing matched" answers, and it must be told apart from a request that failed (C3 still applies).

A8. `g4-master-guid-five`. EXPECT exactly the five rows, each equal to the same GUID's row of `c-master-whole`.

A9. `n1-rates-name-five`. EXPECT the same five GUIDs as A5, each row equal to A5's.

A10. `n2-rates-name-case-changed`. RECORD the rows: one row (`BRIDGE Walk-in`, spelled as the book spells it) means `$Name` folds case on this request; none means it does not. Either is a result.

A11. `n3-rates-name-absent-only`. EXPECT as A7: a complete envelope, no row, no `LINEERROR`.

A12. `p2-rates-parents-three`. EXPECT exactly the catalogue's ledgers under those three parents.

A13. `m-marks` (**A-m1**). EXPECT the marks of A-m0 (reads move no mark).

**Part B starts only if** every Part A answer parsed, held no `LINEERROR`, and no dialog appeared. A row count that differs from an EXPECT line above is recorded and does not hold Part B back; a STOP does.

## Part B: the book grown to 10,018 and then 20,018 ledgers

B0. **BK0** under C5, to a new empty local folder `1331-bk0`. Then `m-marks`.

B1. **Imports `i001` to `i020`**, in order. For each: `m-marks` first (C3: one COMPANY row, THE COMPANY's), then the import, then a screenshot of the whole Tally window. EXPECT each import's counters `CREATED` 500 and every other counter 0, no `LINEERROR` (C4); record `LASTMID` and the seconds. Anything else = STOP.

B2. **Reads at 10,018 ledgers (S10)**, in this order:
- `m-marks`: record the marks. EXPECT `ALTMSTID` above A-m1's by 10,000 (one alteration per Create); another rise is recorded, not a STOP.
- `e-extent`: EXPECT `NUMLEDGERS` 10,018 (A1's plus 10,000). Another count = STOP: the imports did not do what their counters said.
- `k-catalogue-v2` (bound 16,000,000 bytes): EXPECT `NUMLEDGERS` rows, the five ledgers unchanged. Record bytes and seconds.
- `g-groups`: EXPECT the rows of A3.
- `g1-rates-guid-five`, then `g1-rates-guid-five` again: EXPECT each byte-identical to A5's answer (the rows do not depend on the rest of the book). A difference outside `CMPINFO` is listed by element path.
- `n1-rates-name-five`: EXPECT byte-identical to A9's.
- `g3-rates-guid-absent-only`: EXPECT byte-identical to A7's.
- `g4-master-guid-five`: EXPECT byte-identical to A8's.
- `p2-rates-parents-three`: EXPECT byte-identical to A12's.
- `m-marks`: EXPECT the marks of this step's first read.
- **NEVER SENT from here on:** `c-master-whole` and `p1-rates-parents-four`. Each reads every ledger under Sundry Debtors, far past its bound.
- **Gate to go on:** if the catalogue answered more than 15,000,000 bytes or took more than 30 seconds, or any read of this step took more than 120 seconds, do not send `i021` to `i040` or B4: go to B5 and record why.

B3. **Imports `i021` to `i040`**, as B1.

B4. **Reads at 20,018 ledgers (S20)**: the reads of B2 in the same order, `e-extent` EXPECTING `NUMLEDGERS` 20,018, the catalogue's bound 32,000,000 bytes, the same byte-identity EXPECT lines against Part A.

B5. **BK1** under C5, to a new empty local folder `1331-bk1`: the grown book. **BK1 is KEPT on the lab PC** (folder, name, bytes and sha256 in the notes) for a later sitting that posts an invoice on a large book; it is not restored in this item. Then `m-marks`.

B6. **Restore BK0** under C6 (the plan names `1331-bk0`, its file, bytes and sha256 as recorded at B0). Then `m-marks` (**R-m**), `k-catalogue-v2` (**R-k**) and `g1-rates-guid-five` (**R-g1**). EXPECT R-m equal to A-m1, R-k byte-identical to A2's answer, R-g1 byte-identical to A5's; a difference inside `CMPINFO` only is recorded, not a failure. **ANY OTHER DIFFERENCE: STOP**, report "the restore check differs" with both sizes and the first differing offset, and send nothing further. The book is then not as found.

B7. C10 close.

**PASS (the sheet ran)** means: Part A answered whole and was recorded; BK0 and BK1 are recorded; every import's counters are as expected; the S10 reads (and the S20 reads, unless the B2 gate held them) answered whole and were recorded with their times; the restore check is byte-identical.

**FAIL** means any of: a transport failure; a count by substring; a write sent twice; a mark that moves during the reads of one step; `c-master-whole` or `p1` sent after an import; the restore check differs.

**THE FINDING it records**, on one synthetic book:
- whether a filter naming ledgers by GUID, and one naming them by name, answers exactly the named ledgers on the rate and the compliance master requests;
- how a filter that names a ledger the book does not hold answers;
- whether `$Name` folds case on the rate request;
- the bytes and seconds of those filtered reads with 18, 10,018 and 20,018 ledgers;
- the bytes and seconds of the whole catalogue and group reads at those sizes.

**NOT MEASURED:**
- a book whose ledgers carry GST registrations, addresses or long names (the added ledgers carry none, so the catalogue's bytes per ledger here are a lower bound);
- a book whose mark is high from alterations rather than ledgers;
- any parent filter on Sundry Debtors once it holds thousands of ledgers (never sent: far past its bound);
- the ComplyEaze Bridge build or post itself on the grown book (BK1 is kept for that);
- any other book, release or edition.

**EVIDENCE** on a NEW branch `lab/1331-scoped-masters-answers` (C11), after the pre-push scan and the run coordinator's yes. It holds: every raw answer with headers, sizes and sha256 (the catalogue answers at 10,018 and 20,018 ledgers may stay on the lab PC, listed by name, bytes and sha256, if their size makes a push unwise); every import answer; the send log with times; the parser's tables; the restore plan and the restore check; a notes file with UTC times. Screenshots and both backups stay on the lab PC, listed by name, bytes and sha256.
