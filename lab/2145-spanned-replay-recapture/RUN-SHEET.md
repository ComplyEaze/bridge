# #2145, item 2145-R: the replays that hold only an unspanned census, recorded again with the census spanned

Not for merge. READS ONLY on two synthetic companies the lab already holds, BRIDGE AMEND LAB and BRIDGE PILOT LAB, one at
a time; Part B alone restores a backup, into a NEW folder of its own, and only on the run coordinator's word.
`LAB-COMMON.md` in this folder (v3, a byte copy from `lab/2145-spanned-census-captures` at 6a968f01, sha256
5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6) applies in full, with THE COMPANY of each part below;
where this sheet and it differ, the stricter holds. About 30 minutes for Parts 0, A and P; Part B adds about 20 minutes a
backup.

**WHY.** Since #2145 every census Bridge sends is an AlterID span, `(0, mark]`, with the mark the call read. The
maintainers' rule (4 Oct, on #899) is that an answer captured for the unspanned census may not stand in for the spanned
one. Six replay tests still hold only an unspanned census answer:
- four replay three `verify_import` calls on BRIDGE AMEND LAB: the 50-voucher batch of 1 April 2026 (recorded at the
  voucher mark 1469; one test), the same batch after one of its vouchers was cancelled (1685; two tests), and a single
  Payment of 3 April 2026 (1676; one test);
- two replay one register call recorded on another book, which this item records again on BRIDGE PILOT LAB.
Item 2145-C sent those AMEND LAB censuses spanned on 30 Oct 2026 (`lab/2145-spanned-census-captures-answers` at
993d96bc). The book had moved past every recorded mark: its voucher mark read 1829. On 1 April one voucher of the batch is
gone and three are altered or cancelled; on 3 April the payment itself is altered. So the replays cannot carry their old
answers forward, and are recorded again here: as the book is now (Part A), and, where a backup of the book from the
recorded days exists, as it was then (Part B).

## The requests (`requests/`, UTF-16LE with a byte-order mark)
- **How they were made.** Each is the request ComplyEaze Bridge itself sends, as its own code renders it at the head of
  #2145 (`2da3a53d`). A throwaway test drove each call (`verify_import`, `purchase_register`, `sales_register`) against a
  recording test double whose answers carried the marks named below, and wrote every request the code sent; no byte was
  typed by hand. Each request recorded before #2145 came out byte for byte as recorded: the company list, the AMEND LAB
  marks read and both verification reads equal the requests the replays pin, and the three Part B census requests and
  `p-marks` equal item 2145-C's (`requests/a-*_c2_spanned_*` and `p-marks` on `lab/2145-spanned-census-captures`). The
  register call sends `purchase_register` and `sales_register` the same requests.
- **Each call's order**, which these requests follow, skipping repeats (every repeat of a request in the recorded calls
  got the same answer): `verify_import` reads the company list, the marks, the window's census and the verification
  read; the register reads the company list, the book extent, the currencies, the two ledger listings, the groups, the
  window's census, the window and the marks.
- **The census requests carry the mark**: each is spanned to the voucher mark its call reads first (for AMEND LAB the
  marks read, for PILOT LAB the book extent). They are right only for the marks in the table below. **That is why each
  part's first marks read decides whether the part goes on.** The PILOT LAB ledger listings also carry dates the call
  takes from the book extent (its last voucher date and the start of its books), and the register's choice of how to read
  the ledgers depends on the master mark, so Part P also checks those before any ledger request.
- **`SHA256.txt`** has three columns (sha256, bytes, path) and does not list itself. `python -I check_requests.py
  requests` in this folder checks every file against it, and the census requests' spans: EXPECT `files: ok 22, FAIL 0`
  and `requests: ok 17, FAIL 0`; anything else = STOP, nothing sent.

| file | part | what it reads | READ BOUND (bytes) |
|---|---|---|---:|
| `company-list` | A, B, P | the companies loaded (Bridge's company list) | 65,536 |
| `a-marks` | A, B | AMEND LAB's marks (`Bridge Agent Company High Water`) | 16,384 |
| `a-0401_census_spanned_0_1829` | A | the census of 1 April 2026, `(0, 1829]` | 1,000,000 |
| `a-0401_verification` | A, B | the verification read of the 1 April batch | 1,000,000 |
| `a-0403_census_spanned_0_1829` | A | the census of 3 April 2026, `(0, 1829]` | 1,000,000 |
| `a-0403_verification` | A, B | the verification read of the 3 April payment | 1,000,000 |
| `b-0401_census_spanned_0_1469` | B | the census of 1 April 2026, `(0, 1469]` | 1,000,000 |
| `b-0401_census_spanned_0_1685` | B | the census of 1 April 2026, `(0, 1685]` | 1,000,000 |
| `b-0403_census_spanned_0_1676` | B | the census of 3 April 2026, `(0, 1676]` | 1,000,000 |
| `p-marks` | P | PILOT LAB's marks | 16,384 |
| `p-book-extent` | P | the book extent | 16,384 |
| `p-currencies` | P | the currencies | 16,384 |
| `p-ledgers-compliance` | P | the ledgers, with their duty heads | 1,000,000 |
| `p-ledgers-paired` | P | the ledgers, paired listing | 1,000,000 |
| `p-groups` | P | the groups | 1,000,000 |
| `p-0804-0805_census_spanned_0_68` | P | the census of 4 to 5 August 2026, `(0, 68]` | 1,000,000 |
| `p-0804-0805_window` | P | the vouchers of 4 to 5 August 2026 | 1,000,000 |

**REQUESTS BY CURL** are allowed by the owner's decision of 26 Oct 2026 that requests sent by curl may be used to discover
shapes. Each request is sent exactly as LAB-COMMON C3 says, one at a time; each is a read. **After every request**, once
its answer is written, send `GET /status` (C3's line, without the data) and record its status and time. None is a NEW
SHAPE: every one is a shape Bridge has sent before, and the spanned census was sent on 26, 28 and 30 Oct 2026 with no
dialog. `p-0804-0805_window` is the register's window read with the field list Bridge has sent since REFERENCEDATE was
added; these exact bytes have not been sent before, so take a screenshot of the whole Tally window after it, before the
next request (a dialog there = C7). **Time:** each request has 600 s to answer (C3). If no answer comes, or the
`GET /status` after it has no answer 60 s after it was sent, the gateway is held: STOP, leave the curl running, press no
key, send nothing more, report, and wait (no state read and no shut until the run coordinator says).

## The books and the marks each part needs
| Part | THE COMPANY (GUID) | needed at its first marks read | its window(s) |
|---|---|---|---|
| A | BRIDGE AMEND LAB (`17a10910-773c-42c6-bd66-7bba9a392536`) | ALTVCHID **1829** (read 30 Oct 2026; ALTMSTID then 245) | 1 April 2026; 3 April 2026 |
| P | BRIDGE PILOT LAB (`6b43e498-430c-4d5c-bfef-d32e2ab93c85`) | ALTVCHID **68** (read 30 Oct 2026; ALTMSTID then 245) | 4 to 5 August 2026 |
| B | a restored copy of BRIDGE AMEND LAB (the same GUID) | (ALTVCHID, ALTMSTID) **exactly** one of (1469, 222), (1676, 229), (1685, 232) | by the marks: below |

**At a part's first marks read:** another GUID, or more than one COMPANY row = STOP. **An ALTVCHID other than the one
needed** = record it and STOP that part before its next request: its census requests were rendered for that mark, and
a new set is rendered for the mark read. In Part A an ALTMSTID other than 245 is recorded and the part goes on: no
request `verify_import` sends depends on it. In Part P an ALTMSTID other than 245 = STOP, as for the voucher mark. In
Part B both marks must match exactly, because each replay also holds the master mark of its recorded state.
**Any STOP ends the sitting**, in this part and before every later one: C8, then C10 for the book
loaded (except a held gateway, rule above), and no later part is run without the run coordinator's word.

## Before the sitting
1. Copy this folder to `C:/bridge-lab/2145R/sheet/` and work from there.
2. `python -I check_requests.py requests` in that folder. EXPECT `files: ok 22, FAIL 0` and `requests: ok 17, FAIL 0`.
3. Tally licensed; Educational mode = STOP (C1). Record the release and licence mode only.
4. The answers go to `C:/bridge-lab/2145R/answers/` (absent or empty before, else STOP), one folder per part (`A`, `P`,
   `B1`, `B2`, ...).
5. The data path Tally's Select Company screen shows (Tally's own data folder) is recorded, with a screenshot, the first
   time that screen is open (Part A step 1). The lab is left on it.

## Part 0: the backups of BRIDGE AMEND LAB (no request, nothing loaded, nothing changed)
1. In PowerShell, list every Tally backup file under the lab folder:
   `Get-ChildItem -Path C:\bridge-lab -Recurse -File -Filter 'TDBK*' -ErrorAction Stop | Select-Object FullName,Length,LastWriteTime | Format-List`
   then `$?` on its own line, both recorded; do the same for any other folder the lab's own notes name as holding
   backups. An error, or `$?` False, is "the command failed", never "none found" (C2). Keep the whole output as
   `answers/part0-backups.txt`.
2. BRIDGE AMEND LAB's company number is read from the Select Company list when Part A loads it (Part A step 1). Then mark
   each listed file whose name carries that number, and record each one's name, folder, bytes, sha256 (Get-FileHash) and
   LastWriteTime, and any lab note saying when and why it was taken.
3. **Report the list to the run coordinator at the end of Part A.** Part P does not wait for the answer. Part B runs only
   for a backup the run coordinator names from this list.

## Part A: BRIDGE AMEND LAB as it is now (reads only)
1. Load THE COMPANY alone (C1): on the Select Company screen, before choosing it, screenshot the list and record the data
   path it shows and BRIDGE AMEND LAB's company number. C2 step 0. Then `a-marks`: the GUID check and the ALTVCHID check
   above (needed: 1829).
2. In this order, each once: `company-list`, `a-0401_census_spanned_0_1829`, `a-0401_verification`,
   `a-0403_census_spanned_0_1829`, `a-0403_verification`.
3. A dialog, a hang, a non-200 or an answer that does not parse = STOP (C3, C7, C9); after such a STOP nothing more is
   sent, `GET /status` included, until the run coordinator says (C8).
4. A screenshot of the whole Tally window, then `a-marks` again. EXPECT the marks of step 1, since the reads write
   nothing. A difference = record it and STOP.
5. C10 (Shut Company). Then Part P.

## Part P: BRIDGE PILOT LAB, the register call (reads only)
1. Load THE COMPANY alone (C1). C2 step 0. Then `p-marks`: the GUID check and the ALTVCHID check above (needed: 68).
2. `company-list`, then `p-book-extent`. **The extent check:** its one COMPANY row has THE COMPANY's GUID, ALTVCHID 68,
   ALTMSTID 245, LASTVOUCHERDATE 20260915 and BOOKSFROM 20260401 (`summarise2145r.py` prints them). Anything else = STOP
   before the next request: the ledger and census requests were rendered for those values.
3. Then, in this order, each once: `p-currencies`, `p-ledgers-compliance`, `p-ledgers-paired`, `p-groups`,
   `p-0804-0805_census_spanned_0_68`, `p-0804-0805_window` (the screenshot after it, above).
4. As Part A step 3.
5. A screenshot of the whole Tally window, then `p-marks` again. EXPECT the marks of step 1. A difference = record it
   and STOP.
6. C10. Then Part B if the run coordinator has named a backup; otherwise close.

## Part B: a backup of BRIDGE AMEND LAB from the recorded days, restored apart (only on the run coordinator's word)
Each backup the run coordinator names is restored into a NEW folder of its own, never into Tally's data folder, so the
book Part A read is not touched. B1 is the first backup named, B2 the second, and so on.
1. **First, once:** a backup of the book as it is now, by C5, with BRIDGE AMEND LAB loaded alone, to
   `C:/bridge-lab/2145R/backup-amend-now/` (absent or empty before). Then shut it (C10's Shut Company only). The restores
   below never write to Tally's data folder; this backup is the way back if one does, restored only on the run
   coordinator's word (C6).
2. **The source.** Copy the backup file named into a new folder of its own, `C:/bridge-lab/2145R/source-Bn/` (absent or
   empty before), and check the copy's sha256 against the one Part 0 recorded; the original is not moved or changed. The
   restore reads from that folder, so C6's "that folder holds that one file only" holds.
3. **The restore**, by C6, with these changes, which are stricter: the Restore Data Path is
   `C:/bridge-lab/2145R/restored-Bn/` (a listing shows it absent or empty before, else STOP), not Tally's data folder;
   so the only prompt expected is "Accept ?" (Yes). The "Company data folder ... already exists" prompt appearing = press
   no key, screenshot, STOP: the path is not the new folder. restore-plan.txt names that folder as the Restore Data Path.
   A listing of the new folder is kept before and after. A backup from an older release may offer to upgrade or migrate
   its data: that is C6's "offer to repair, rewrite or migrate" = no key, STOP.
4. **Load the restored copy alone:** in Select Company, change the data path to `C:/bridge-lab/2145R/restored-Bn/`
   (screenshot the path before accepting) and load BRIDGE AMEND LAB from it. BEFORE any request: a screenshot of the
   whole Tally window and one of the Gateway showing it as the one company. C2 step 0.
5. `a-marks`: the GUID check (EXPECT BRIDGE AMEND LAB's GUID). Then, by (ALTVCHID, ALTMSTID):
   - **(1469, 222)**, the 50-voucher batch as recorded: `company-list`, `b-0401_census_spanned_0_1469`, `a-0401_verification`;
   - **(1676, 229)**, the payment as recorded: `company-list`, `b-0403_census_spanned_0_1676`, `a-0403_verification`;
   - **(1685, 232)**, the batch after the cancel as recorded: `company-list`, `b-0401_census_spanned_0_1685`, `a-0401_verification`;
   - **anything else**: record it, and nothing more is sent to this copy. That is a result, not a STOP.
6. As Part A steps 3 and 4.
7. Shut the copy (Shut Company). Change Select Company's data path back to Tally's own data folder (recorded in Part A
   step 1), screenshot it, and load nothing. Then the next backup named, from step 2, or close.
**A STOP in Part B** follows C8, and its route back, once the run coordinator says to end, also sets Select Company's
data path back to Tally's own data folder before C10.

## EXPECT
For each answer, once it is written, run `python -I summarise2145r.py ANSWER COMPANY_GUID [M]`, with THE COMPANY's GUID
and, for a census answer, M (the number its request ends in), and keep its whole output. It prints the answer's size,
sha256, BODY digest (CMPINFO removed) and rows. Its exit 2 (an answer that does not parse, or a row of another company)
is C3's or C4's STOP. Any other result that differs from an EXPECT line below, a census row above its M (exit 1)
included, goes to the run coordinator before the next request (C11), with the whole output; nothing is re-sent.
- **Part A, 1 April census:** 49 rows, none above 1829, with the BODY digest
  `ce95e4cfae8ce67438c0faa0eee1c2d8dffc5ffcce0d25ce0c7ec7900ed9653f` that 2145-C's answers for that day had on 30 Oct.
- **Part A, 3 April census:** 4 rows, none above 1829, with 2145-C's BODY digest for that day,
  `ee381b99270576d658297f756dba70bc84a21ea60084f7f382a4064d3e3e885d`.
- **Part A, the two verification reads:** whatever the book holds; nothing is predicted.
- **Part P, census:** 3 rows: two dated 20260804 with ALTERIDs 31 and 32, and one dated 20260805 with ALTERID 35, as
  the year's audit read of this book recorded them. **The window:** the same 3 vouchers, two Purchase and one
  BRIDGE Sales.
- **Part B:** for a copy whose marks match, the answers are expected to have the BODY digests the replay's own answers
  have (the recorded census answer for the census, the recorded verification answer for the verification read). That is
  recorded, not judged.
Nothing is edited in a book.

## The books
- Parts A and P change nothing: every request is a read, and the marks are read at the start and the end of each part.
- Part B writes one backup file of BRIDGE AMEND LAB to a new folder and restores each named backup into a new folder of
  its own. Tally's data folder and the book in it are not touched; the new folders are left in place.

## Close and evidence
- **C10** for each part; Tally's Select Company is left on its own data folder, with no company loaded.
- **C11:** evidence to a new branch `lab/2145-spanned-replay-recapture-answers`, after the pre-push scan and the run
  coordinator's yes. It holds every answer, whole, with headers, size, sha256 and its send and answer times; each
  `summarise2145r.py` output; the `check_requests.py` output; Part 0's listing; for Part B, restore-plan.txt and the
  folder listings; and the notes, with UTC times and one line per step. Backup files and screenshots stay on the lab PC,
  listed by name, size and sha256.

## What each replay can carry (decided on #2145, not by the lab)
- **From Part B**, for a copy whose marks match a recorded state: that replay keeps its meaning, with its census answer
  recorded spanned.
- **From Part A** otherwise. The two cancelled-voucher tests and the payment test are re-pinned to the book as it is now,
  with every count they assert taken from these answers, never edited by hand. One voucher of the 1 April batch is gone
  and the payment has been altered since it was recorded, so their counts and what they show may change. The 50-voucher
  batch test needs the batch before any change; the book no longer holds it, so without a matching Part B copy it is
  rewritten to what the book now holds or retired. Its requests and the cancelled tests' are byte-identical, so one
  Part A capture serves both.
- **From Part P:** both register tests move to BRIDGE PILOT LAB. The window holds a sale as well as the two purchases, so
  the sales register lists one row and names the two purchases apart, where the old record listed none.

## Not covered
- A register call that refuses, or a census divided into several parts: both windows here fit one request.
- Any other book, release or edition.
