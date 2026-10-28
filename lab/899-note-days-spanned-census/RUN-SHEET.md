# #899 part 2: the spanned voucher census on the three note-day records' book and days

Not for merge. READS ONLY, on one company the lab already holds. `LAB-COMMON.md` in this folder (v3, a byte copy from `lab/485-ledger-filtered-vouchers-run-sheet` at 4e769a9c, itself from `lab/626-1038-1050-run-sheets` at 8c602c15) applies in full, with THE COMPANY below; where this sheet and it differ, the stricter holds.

**WHY.** #899 part 2 sends a census whose voucher mark fits one request as the AlterID span `(0, mark]`, never unspanned. The maintainers' rule (4 Oct) is that the change is built only on captures of the spanned request: an answer captured for the unspanned census may not stand in for it. Three register tests replay live calls recorded on 1 Oct 2026 on BRIDGE SHAPE LAB (`src-tauri/crates/bridge-tally-protocol/tests/fixtures/note-days/`), and each record holds the unspanned census of its day. After the change, the code sends that day's census spanned to the mark the record read, which was 111. This item captures exactly those three spanned requests, beside the unspanned ones as the control, so that the three replays can carry real answers.

**THE COMPANY:** BRIDGE SHAPE LAB, GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, synthetic. STOP if it is absent, or if the marks answer at step 1 does not name exactly this GUID. On 1 Oct its voucher mark (ALTVCHID) read 111.

**THE REQUESTS** (`requests/`, UTF-16LE with a byte-order mark):
- `m_marks`: the company marks read, a byte copy of `note-days/shape_lab_company_marks_request.utf16le.xml`, the request the records sent.
- `dDDDD_c1_unspanned`, one per day: a byte copy of that day's recorded census request (`debit_note_day_…` for 28 Apr 2025, `credit_note_day_…` for 29 Apr 2025, `cancelled_purchase_day_…` for 3 Jul 2025). It is the control.
- `dDDDD_c2_spanned_0_111`, one per day: the same request with ` AND $AlterID &gt; 0 AND $AlterID &lt;= 111` added to its filter, as the code writes an AlterID span, and nothing else changed. Its shape is the census sent spanned on 26 Oct to two other synthetic books (#2061, `agent/spanned-census-two-books/`), and each byte of it is checked against the code's own rendering before it is used in a test.

**REQUESTS BY CURL** are allowed by the owner's decision of 26 Oct 2026 that requests sent by curl may be used to discover shapes. Each request is sent exactly as LAB-COMMON C3 says, one at a time. Before the first send, run `python -I check-request-ids.py requests` in this folder. EXPECT `census: ok 7, skipped 0, FAIL 0`; anything else = STOP, nothing sent. Size bounds: 8,192 bytes for the marks, and 100,000 bytes for each census; the recorded answers were 5,434 to 5,454 bytes.

**OWNER steps** are marked [OWNER]. The spanned census is a known shape (two books on 26 Oct, no dialog), so a person at the machine is not required for it. The [OWNER] steps are the owner's, or the lab's on the owner's recorded word, as the run coordinator's row note says.

## Before the sitting
1. Check every file here against `SHA256.txt`.
2. [OWNER] Go-ahead. Tally licensed; Educational mode = STOP (C1). Record the release (the About screen, or the build's own `tally_status`).

## The reads
1. Load THE COMPANY alone (C1). C2 step 0. Then `m_marks`.
   - **The GUID check:** EXPECT one COMPANY row, GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346`. Another GUID, or more than one row = STOP.
   - Record ALTVCHID and ALTMSTID. ALTVCHID below 111 = STOP: the book is not the one recorded (it may have been restored to an older state). At 111 or above, go on, recording the value.
2. For each day in order (28 Apr 2025, 29 Apr 2025, 3 Jul 2025): send `dDDDD_c1_unspanned`, then `dDDDD_c2_spanned_0_111`.
3. A dialog, a hang, a non-200 or an answer that does not parse = STOP (C3, C7, C9). The gateway is not probed again.
4. `m_marks` again. EXPECT the marks of step 1, since the reads write nothing. A difference = record it and STOP.
5. C10 (Shut Company).

## EXPECT and the comparison (recorded; the lab judges only the STOP rules)
For each day, run `python -I compare899.py RECORDED_SHA256 c1-answer c2-answer`, with the recorded answer's SHA-256 from `note-days/PROVENANCE.md`:

| Day | Record | RECORDED_SHA256 |
| --- | --- | --- |
| 28 Apr 2025 | `debit_note_day_voucher_census_response.utf16le.xml` | `8bf751237defd84bdfe4dc7ae98ad9cb584be3f983694caefe0de359e1876e9d` |
| 29 Apr 2025 | `credit_note_day_voucher_census_response.utf16le.xml` | `54ca29c0d72ffdd17272f6bf872e4de7740ce0a975a1d6a7f1833f0dfff26206` |
| 3 Jul 2025 | `cancelled_purchase_day_voucher_census_response.utf16le.xml` | `71c527b65edcb2aaee81561d581528ce3f620a10deaca764f6bd050fd2842973` |

It parses both answers as XML after dropping only what C4 drops, never by a text search, and prints each answer's rows as (GUID, ALTERID, DATE). Keep its whole output. It returns three verdicts:
- **the unspanned answer equals the recorded answer**, byte for byte. EXPECT it to hold: the day is unchanged since 1 Oct. If it does not hold, the captures cannot stand in for the record: record it, with both row lists. That is the finding, not a STOP.
- **the spanned answer equals the unspanned answer**, byte for byte. EXPECT it to hold, as it did on the two books on 26 Oct.
- **every row is inside the span** (ALTERID 111 or below). EXPECT it to hold: each recorded day holds one voucher, and the 28 Apr one has ALTERID 10.

Exit 0: all hold. Exit 1: one does not. Exit 2: an error, never a verdict.

## The book
It does not change; every request is a read. The marks are read at the start and the end, and must be equal.

## Close and evidence
- **C10.**
- **C11:** evidence to a new branch `lab/899-note-days-spanned-census-answers`, after the pre-push scan and the run coordinator's yes. It holds:
  - every answer, whole, with headers, size, sha256 and its send and answer times;
  - each `compare899.py` output;
  - the notes, with UTC times and one line per step.

  Screenshots stay on the lab PC, listed by name, size and sha256.

## Not covered
- A day where the span leaves a voucher out: every recorded day's vouchers are inside `(0, 111]`.
- The note-day replays' other requests: only their census legs change with #899.
- The code itself: the three replays are rebuilt on these answers in the #899 change. Each spanned request here is checked there against the code's own rendering, byte for byte.
