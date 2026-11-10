# Row 2346-AR: what does Tally store for a Credit Note's Agst Ref against a bill-wise customer's invoice bill, against that bill once it is settled, and against a bill the customer does not hold?

Shape discovery: the three notes are requests written for this row, not requests the build renders, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2346 (0.9.0), capture 6 of the #691 design. Written 10 Nov 2026. A seeded ledger and a seeded invoice set the row up; then three Credit Notes, each sent once; then a restore from the row's own backup. Nothing is keyed in Tally but the backup and the restore. Confidence of everything here until the row runs: unverified.

## Question
Capture 6 asks for three shapes: a `New Ref` on Sales, an `Agst Ref` on a Credit Note, and a `New Ref` reusing an existing name, each read back with the chosen allocation fetch.
- **Two of the three are measured.** Row 2162-S3a (`lab/2162-s3a-answers` at 4dff8215) measured the first: a Sales invoice's New Ref, named by its number, stood as the customer's one open bill. Row 2162-S3b (`lab/2162-s3b-answers` at 7fd14241) measured the third: a New Ref named like an existing bill was stored as an Agst Ref against it, settled bills included. The build's read-back (`ALLLEDGERENTRIES.*`) returned each allocation, and the outstandings tool's Bills Receivable read returned each bill.
- **This row measures the second.** Row 2346-CD (`lab/2346-cd-answers` at fcdd4356) showed that the build's invoice request, as a Credit Note, is created and read back as sent. Its customer was not bill-wise, so it sent no allocation. #2346 needs a Credit Note's party leg to carry an `Agst Ref` against the customer's bill. It also needs to refuse an `Agst Ref` to an absent bill or a smaller one, and to know what happens if it does not.

- **The set-up**, seeded by direct XML, never evidence of its own shape:
  - one ledger, `BRIDGE Walk-in BC`, under Sundry Debtors and bill-wise. This is S3a's ledger request with the name changed.
  - one Sales invoice to it, `BP/26-27/0099`, dated 1 Aug 2026, for 1,051.00, with one New Ref named by its number. This is S3a's invoice request with the number, REMOTEID and party changed. `gen.py` asserts that the same change with S3a's values gives S3a's file byte for byte.
  - The set-up is read back before any note. If the bill does not stand as S3a's did, the row stops there.
- **The three notes**, each row 2346-CD's Credit Note request (`I-CN.xml`) with:
  - the party `BRIDGE Walk-in BC` in its seven places;
  - `REFERENCE` the set-up invoice's number;
  - a new REMOTEID tail;
  - on the party leg (Cr 1,051.00), one `Agst Ref` for 1,051.00. Its elements and their order are the ones `render_sales_invoice_xml` writes for a New Ref (`src-tauri/src/agent_import_invoice.rs:2679`), with `BILLTYPE` changed.

  | Sub-row | Agst Ref names | Sent | The one variable |
  | --- | --- | --- | --- |
  | **c1** | `BP/26-27/0099` | with the bill open for 1,051.00 | the note against an open bill of its full amount |
  | **c2** | `BP/26-27/0099` | after c1, with the bill settled, if c1 settled it | the bill's open amount, 0 where c1 had 1,051.00; c2's request differs from c1's by the REMOTEID alone |
  | **c3** | `BP/26-27/0098`, which the customer does not hold | last | the bill's name, against c1's request |

- **Why one sitting:** each sub-row is one import against a book the earlier ones left. The question of c2 *is* the state c1 leaves. c3 names a bill none of the others touch.
- **What is read:**
  - the marks (M);
  - the number read (N) for the set-up invoice's number;
  - the build's read-back (R) for the set-up invoice and, with the Credit Note type's GUID, for each note by the number Tally gives it;
  - the outstandings tool's Bills Receivable read (B).
- **The allocation fetch and its cost.**
  - The fetch is the build's own read-back, unchanged: S3a and S3b read every allocation through its `ALLLEDGERENTRIES.*`. This row records each answer's bytes and the time of each send (`run.py`'s log).
  - On S3a's book, one voucher's read-back answered 95,834 bytes.
  - B reads every bill of every customer from the book's start. Its cost on a large book is not measured here.
- **Not asked here:**
  - a part note (an Agst Ref for less than the bill);
  - a Debit Note's allocation;
  - a note with a number;
  - `GSTNATUREOFRETURN`, a tax-only note, IGST, invoice mode, GSTR-1;
  - a bill from an earlier year.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2464-GR's, 2346-CD's and 2464-GN's) applies in full. Where it and this sheet differ, the stricter one holds.

- **The book is pinned by GUID.** `count20.py` holds BRIDGE PILOT LAB's GUID as `BOOK`.
  - Its M kind prints whether the COMPANY row's GUID is the book's.
  - `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called.
  - STOP before anything is sent unless s1's COMPANY row is the book's.
- **Its state:** every row queued before this one ends with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245. Other marks at s1: stop before BK, with nothing sent.
- **It holds no bill-wise customer and no Credit Note.**
  - S3a's and S3b's customers were restored away.
  - `BRIDGE Walk-in BC` is in no file on any `lab/` branch or on master.
  - Row 2346-CD's s2 read the Credit Note type by number `1` and found none. Its Credit Note was restored away.
- **The numbers:**
  - Row 2162-S5's year read (`lab/2162-s5-answers` at ab3d2e77, `answers/a-s2-Y-all.xml`) lists the `BP/26-27/` numbers held at the restored state: 0001 to 0016, 0019, 0020, 0021 and 0041 to 0044.
  - Every blob on every `lab/` branch (655 refs, 23,580 blobs, read as UTF-16 and as UTF-8) and on master was searched. None holds `26-27/0098` or `26-27/0099`, or the REMOTEID tails `000000000d10` to `000000000d13`.
  - `0089` to `0097` were passed over, as other rows' sheets send them.
  - The tail `000000000999` was passed over: a test constant on master holds it.
  - Step 2's N and R reads stay a STOP, in case this check is wrong.
- **The Credit Note numbers:** Credit Note numbers automatically on this book. Row 2346-CD's note took `1`. The three notes here are expected to take `1`, `2` and `3` in the order sent, and each R-CN read asks for one of those. If a note takes another number, R finds nothing: record the number on Tally's screen (Display, the day book of 1 Aug 2026) and say so. That is not a finding about the bill.
- **The date:** read the Gateway's current date at step 1. If it is before 1-Aug-2026, stop: the invoice and the notes would be post-dated, which is row 2162-S1's variable. Row 2346-CD read Tuesday 15-Sep-2026.
- **BK is this row's way back.**
  - It is taken at step 1, before anything is sent, and restored at step 8 by C6.
  - The restore runs after step 3 in every case, including a stop.
  - After it, every read is compared whole with the reads before the change. Any part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. Row 2346-CD read TallyPrime with `tally.exe` 1.1.7.2, from the program's file.

**No Bridge build.** Raw requests, one at a time, through `run.py`. Nothing is keyed in Tally but BK (C5) and the restore (C6). The writes are `L-ledger.xml`, `I-0099.xml`, `I-CN-1.xml`, `I-CN-2.xml` and `I-CN-3.xml`, each sent at most once.

**The sends** are row 2464-GN's, unchanged.
- Each of these stops the run and is reported as "the request failed", never as "nothing found":
  - a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`);
  - a collection answer with no `BODY/DATA/COLLECTION`, a B answer whose root is not `ENVELOPE`, an import answer with no counters, or an answer that does not parse (`count20.py`, which prints STOP and exits non-zero).
- A B answer that is `<ENVELOPE></ENVELOPE>` is 0 bills (row 2162-S3a's s2 answer, 46 bytes).
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with `count20.py` and copy its whole output into RESULTS.md.
- Close every other Bridge process first.

## Files
**Copied from earlier rows**
- **Byte-equal to row 2464-GN's file of the same name** (`lab/2464-gn-sheet` at 23b2e57b, folder `lab/2464-gn/`), whose ROW.md says where each came from: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml` (the build's read-back for `BP/26-27/0010`), `template-import-0010.utf16le.xml` (the build's request that created `BP/26-27/0010`, sha256 `b333f0dc`, equal to `PILOT_REQUEST_SHA256` on master) and `template-n-0001.utf16le.xml` (the build's number read).
- **Byte-equal to row 2346-CD's file of the same name** (`lab/2346-cd-sheet` at 9c2229cc, folder `lab/2346-cd/`):
  - `I-CN.xml`, its Credit Note (sha256 `705a9c57`);
  - `R-CN1.xml`, the build's read-back for the number `1` on the type `Credit Note` (`...-00000028`, sha256 `448064a4`).
- **Byte-equal to row 2162-S3a's file** (`lab/2162-s3a-sheet` at d99ef82e, folder `lab/2162-s3a-sheet/`):
  - `B-bills.xml`, the outstandings tool's Bills Receivable read from 1 Apr 2026 to 1 Aug 2026 (`render_native_bills_request`, `src-tauri/crates/bridge-tally-protocol/src/native_outstandings/request.rs:131`, as S3a's ROW.md records). It is kept under its own name.
  - `template-s3a-L-ledger.xml` and `template-s3a-I-0047.xml`, S3a's `L-ledger.xml` and `I-0047.xml`, renamed.

**`gen.py`** writes each request below, with only the change it names, each asserted to occur exactly as often as expected. Each template's sha256 is checked first. Each import is parsed, and the one allocation on the party leg is asserted.
- **`L-ledger.xml`:** S3a's with the name `BRIDGE Walk-in BC` (twice).
- **`I-0099.xml` (set-up):** the template import with:
  - the number `BP/26-27/0099` (twice);
  - the tail `000000000d10`;
  - the party `BRIDGE Walk-in BC` (seven places);
  - on the party leg (Dr 1,051.00) `<BILLALLOCATIONS.LIST><NAME>BP/26-27/0099</NAME><BILLTYPE>New Ref</BILLTYPE><AMOUNT>-1051.00</AMOUNT></BILLALLOCATIONS.LIST>`.
  - With S3a's number, tail and party the same function gives S3a's `I-0047.xml` byte for byte, and `gen.py` asserts it. The set-up is S3a's shape.
- **`I-CN-1.xml` (c1):** `I-CN.xml` with:
  - the tail `000000000d11`;
  - the party `BRIDGE Walk-in BC` (seven places);
  - `REFERENCE` `BP/26-27/0099`;
  - on the party leg (Cr 1,051.00) `<BILLALLOCATIONS.LIST><NAME>BP/26-27/0099</NAME><BILLTYPE>Agst Ref</BILLTYPE><AMOUNT>1051.00</AMOUNT></BILLALLOCATIONS.LIST>`.
  - Still no `VOUCHERNUMBER` (asserted).
- **`I-CN-2.xml` (c2):** `I-CN-1.xml` with the tail `000000000d12` and nothing else.
- **`I-CN-3.xml` (c3):** `I-CN-1.xml` with the tail `000000000d13` and the Agst Ref's `NAME` `BP/26-27/0098`.
- **`N-0099.xml`, `R-0099.xml`:** the number read and the build's read-back for `BP/26-27/0099`.
- **`R-CN2.xml`, `R-CN3.xml`:** `R-CN1.xml` with the number `2`, and `3`.

**`count20.py`** is row 2464-GN's `count19.py` with two changes, each marked AR.
- The R kind also prints, for each leg, every `BILLALLOCATIONS.LIST` that holds a `NAME`: its `NAME`, `BILLTYPE`, `AMOUNT`, `BILLDATE`, `BILLCREATIONDATE` and `BILLID`. The whitespace-only ones are counted apart, by S3b's rule.
- A kind B: the `BILLFIXED` groups, then for `--ledger` (default `BRIDGE Walk-in BC`) each group's `BILLREF` and `BILLDATE` with the `BILLCL` after it.
- Tried once on a captured answer of every kind this row reads, each exit 0:
  - B: S3b's s2 (`<ENVELOPE></ENVELOPE>`: 0 groups), s4 and s7r, the last giving `BRIDGE Walk-in BO` one group, `BP/26-27/0050`, `31-Mar-26`, `-1551.00`, as S3b's RESULTS reads;
  - R with bills: S3b's s5r and s6r, the first giving the party leg one `Agst Ref` `BP/26-27/0048` `-1051.00` dated `20260701`, and five whitespace-only lists on the other legs;
  - R on a note: row 2346-CD's s2 (no row) and s3r R-CN1 (one row, six legs, no named allocation);
  - I and L: CD's s3, S3b's s5 and s3;
  - M: S3b's s1 and s8, and `--same-row` on the two;
  - N, T, U and V: the answers row 2464-GN tried;
  - W: CD's s2.

`SHA256.txt` lists every file with its size. Requests are UTF-16LE with a BOM, as sent.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count20.py a-<label>-*`. A STOP from either script stops the row there.

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, each with the key named.
- For any other prompt, message or dialog: copy it word for word and screenshot it first.
- Then answer No, or Esc if it offers no No. Press nothing more and send nothing.
- Then restore BK by C6, compare as step 8, and close (C10).
- That prompt is this row's result.

**The wait rule.** After each import's send: if 5 minutes pass with no answer and no message:
- send nothing, press nothing, screenshot the whole window, and report to the run coordinator;
- a read sent while Tally is busy can hold its gateway.
- With the Gateway showing and no dialog, run C2's status check (`GET /status` 200, one tally.exe owning 9001) before the next read.

**Each import's answer** is one of these:
- `CREATED` 1 with `ERRORS` 0;
- `CREATED` 0 with `ERRORS` 1 and a LINEERROR;
- `CREATED` 0 with `EXCEPTIONS` 1.

Any other answer, or a dialog after it: the prompt rule. Never send an import file twice (C3).

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down. Before 1-Aug-2026: stop (the Book's date bullet).
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: stop here, before BK.
   - **Take BK** by C5, to the new empty folder `C:\lab\2346-ar\backup-1`. Row 2346-CD's step 1 is the path.
     - Tally may show the folder as `C:\lab\2346-Ar\backup-1`.
     - Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `N-0099.xml R-0099.xml R-CN1.xml R-CN2.xml R-CN3.xml B-bills.xml M-marks.xml`.
   - EXPECT N-0099 0 rows, R-0099 0 rows, and R-CN1, R-CN2 and R-CN3 0 rows each. A row in any: stop before step 3, with each row recorded whole.
   - EXPECT B 0 groups for `BRIDGE Walk-in BC`. Record how many groups it holds in all, and its bytes.
3. **The set-up.**
   - `s3m`: `M-marks.xml`, then `count20.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. It must pass.
   - `s3`: `L-ledger.xml`, sent once. EXPECT `CREATED` 1, `ERRORS` 0. Anything else: send nothing more, and go to step 8.
   - `s4`: `I-0099.xml`, sent once. EXPECT `CREATED` 1, `ERRORS` 0. Anything else: send nothing more, and go to step 8.
   - `s4r`: `N-0099.xml R-0099.xml B-bills.xml M-marks.xml`. **The set-up holds** only if all of these do:
     - N-0099 one row;
     - R-0099 one row whose `BRIDGE Walk-in BC` leg holds one bill with `NAME`: `BP/26-27/0099`, `New Ref`, `-1051.00`;
     - B one group for `BRIDGE Walk-in BC`: `BP/26-27/0099`, `1-Aug-26`, `BILLCL` `-1051.00`.
     - These are as S3a's s5 read.
   - If the set-up does not hold: no note is sent. Record the answers whole and go to step 8.
4. **c1: a Credit Note against the open bill.**
   - `s5m`: `M-marks.xml`, then `--before-write` against s1. It must pass.
   - `s5`: `I-CN-1.xml`, sent once. Then the wait rule, and record every counter and any LINEERROR word for word. The window after it (shot).
   - `s5r`: `R-CN1.xml R-0099.xml B-bills.xml M-marks.xml`.
     - **c1's answer:**
       - R-CN1's row: its number, date, GUID, ALTERID and status flags;
       - each leg, and every bill on the `BRIDGE Walk-in BC` leg;
       - B's groups for `BRIDGE Walk-in BC`.
     - `count20.py --diff a-s4r-R-0099.xml a-s5r-R-0099.xml`: record whether the invoice moved, its allocation included.
     - Record ALTVCHID and ALTMSTID against s5m's.
   - **c2 is sent only if c1 settled the bill.** That means:
     - c1 was created;
     - R-CN1's `BRIDGE Walk-in BC` leg holds one bill, `BP/26-27/0099`, `Agst Ref`, `1051.00`;
     - B holds no group `BP/26-27/0099` for the customer.
   - Otherwise record why, skip c2, and go to step 6.
5. **c2: the same note again, against the settled bill.**
   - `s6m`: `M-marks.xml`, then `--before-write` against s1.
   - `s6`: `I-CN-2.xml`, sent once. Record as step 4.
   - `s6r`: `R-CN2.xml R-CN1.xml R-0099.xml B-bills.xml M-marks.xml`.
     - **c2's answer:** R-CN2's row and bills as c1's, and B's groups for the customer.
     - `count20.py --diff a-s5r-R-CN1.xml a-s6r-R-CN1.xml` and `--diff a-s5r-R-0099.xml a-s6r-R-0099.xml`: record whether c1's note or the invoice moved.
6. **c3: a Credit Note against a bill the customer does not hold.**
   - `s7m`: `M-marks.xml`, then `--before-write` against s1.
   - `s7`: `I-CN-3.xml`, sent once. Record as step 4.
   - `s7r`: `R-CN3.xml R-CN1.xml R-0099.xml B-bills.xml M-marks.xml`.
     - If c2 was skipped, the third note may take `2`. Then also send `R-CN2.xml` in `s7r`, and record which one holds c3.
     - **c3's answer:** its row and bills, and B's groups for the customer, `BP/26-27/0098` among them.
     - `--diff` of R-CN1 and of R-0099 against their last earlier answers: record whether either moved.
7. *(Reserved: nothing between c3 and the restore.)*
8. **Restore** BK by C6, with `restore-plan.txt` written first. Row 2346-CD's step 5 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace).
   - Then `s8`: `M-marks.xml N-0099.xml R-0099.xml R-CN1.xml R-CN2.xml R-CN3.xml B-bills.xml`, and compare whole:
     - `count20.py --same-row a-s1-M-marks.xml a-s8-M-marks.xml`: EXPECT it passes.
     - N-0099, R-0099 and each R-CN: 0 rows. B: 0 groups for `BRIDGE Walk-in BC`.
     - Record each answer's sha256 against s2's; EXPECT each equal.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
9. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule and C6 name, or if the gateway stops answering.
- Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.
- Say in the report what the book then holds: from step 3 the ledger, from step 3's import the invoice and its bill, and from each note whatever it did.

## Record
- Each answer's bytes and sha256, and `count20.py`'s lines.
- `run.py`'s log for each label (each send's start time, file and bytes): the allocation fetch's cost on this book.
- The `--diff`, `--before-write` and `--same-row` outputs, whole.
- The window after each import (shot), listed by name, bytes and sha256.
- Every prompt, message and busy screen, word for word, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **s2:** as the Book and step 2 say.
- **The set-up:** as S3a's: the ledger created, the invoice created with its New Ref, one open bill of 1,051.00.
- **c1:** created as Credit Note `1`, its party leg holding the Agst Ref as sent, and the bill gone from B (settled). A receipt's Agst Ref settled a Journal's bill this way in S3b's set-up (its K2).
- **c2 and c3:** not estimated. That is the row's question.
- **s8:** equal to s2 and s1.

## Decides
Each is PARTIAL: one import of each shape, one sitting, one book, one release.

**c1, the note against an open bill**
- **Stored as sent and the bill settled:** a sales-side Credit Note's Agst Ref, in the build's allocation form, settles the invoice's bill on this release. #2346's builder can write it, and its read-back check can require one Agst Ref named by the bill for the party amount, as `allocation_ok` requires one New Ref for Sales (`src-tauri/src/agent_import_invoice.rs:1478` to `:1498`).
- **Stored as another bill type, or the bill left open:** record it whole. The builder must not judge a note's allocation by the request alone, and the coordinator decides the next shape.
- **Refused:** record the words. This allocation form does not carry over to a note on this release. Each next shape is its own row.
- **The invoice moved** (R-0099 differs other than by the note's settlement): record which field.

**c2, the same note against the settled bill**
- **Created, with a group for `BP/26-27/0099` back in B, or the bill otherwise holding a credit:** Tally takes an Agst Ref for more than the bill holds, with no error. The read-back reads the note as sent, so after the post nothing can tell. #2346 must therefore read the customer's bills before the post, settled ones included, and refuse an Agst Ref larger than the bill's open amount. This is the acceptance's "smaller bill" refusal, and it is the only guard.
- **Refused:** Tally guards an over-settled Agst Ref on this release. The build's refusal is still built, since it acts before the approval rather than at the post.
- **Stored as another bill type** (for example `Advance` or `On Account`): record it. The read-back's check would read it as divergent, after the post. The build's refusal before the post is still needed.

**c3, a note against a bill the customer does not hold**
- **Created, with a new group `BP/26-27/0098` in B:** Tally makes the bill the note names, with no error. The build must refuse an Agst Ref to a bill it did not read. This is the acceptance's "absent bill" refusal, and it is the only guard.
- **Refused, or stored as another type:** as for c2.

**What these answers are for.** They may inform #2346's design and its builder. They are never a regression fixture for the build's output, because the build renders no note.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build has no note type.** `VoucherType` (`src-tauri/src/agent_import.rs:330`) has Payment, Receipt, Journal, Contra and Sales, so no build sends a note.
- **The build refuses a bill-wise customer** before any write (`invoice_party_bill_wise_not_supported`, `src-tauri/src/agent_import_invoice.rs:883`). So no build reaches this row's set-up either: the set-up invoice is S3a's raw request.
- **The allocation the build writes and checks.** `render_sales_invoice_xml` writes a bill-wise party's New Ref (`src-tauri/src/agent_import_invoice.rs:2679`). The read-back's check requires exactly one New Ref named by the number on the party leg (`:1478`), and reads anything else as `bill_allocation` (`:1498`). The notes here use that element form with `BILLTYPE` `Agst Ref`.
- **The bill read exists.** `render_native_bills_request` (`src-tauri/crates/bridge-tally-protocol/src/native_outstandings/request.rs:131`) is the outstandings tool's request, sent at `src-tauri/src/tally/runtime.rs:4145`. `B-bills.xml` is S3a's copy of it. #2346 names it as the component a note's pre-post bill check would reuse.

## Not measured
- A part note, a note for more than the bill when part is still open, and two notes against one bill.
- A Debit Note's allocation, and a purchase-side note.
- A note with a number, `GSTNATUREOFRETURN`, or a registration written by the request.
- A bill from an earlier year, and an opening bill.
- The bill read's cost on a large book.
- GSTR-1 for any note.
- A note built and posted through ComplyEaze Bridge.
- Any other book or release.

Row: lab/2346-ar/
