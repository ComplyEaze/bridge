# Row 2162-RP: does the build's own invoice post under an unused number, and what does Tally answer when the request that created `BP/26-27/0010` is sent again, as is?

Not for merge. Part of #2162 (ADR 0004, the 2026-10-31 checklist, item 11: two invoices that differ only by their number). Written 10 Nov 2026. Two raw imports on BRIDGE PILOT LAB, each between a restore from one backup. Confidence of everything here until the row runs: unverified.

## Question
Item 11 owes, one variable each: the same figures under another number post; the same number is refused; a replay is refused; numbers that differ by case, spaces or leading zeros; a blank number's read-back. Row 2162-S4 (`lab/2162-s4-answers`) measured the second and the case and leading zeros: on `BRIDGE Sales`, a second invoice under a number in use, with a new REMOTEID, was declined in the answer (`CREATED` 0, `EXCEPTIONS` 1, no `LINEERROR`, no dialog). This row asks two things S4 did not.

- **Part A, the same figures under another number, from the build's own bytes.** Row 2162-HS (`lab/2162-hs-answers`) imported `BP/26-27/0072`, `0010`'s request with the number and the REMOTEID tail changed by `gen.py`, and Tally created it. That was an edit of a captured request. Here the request is the build's own write, rendered by master's code: the build of `0010`'s two lines under the number `BP/26-27/0040`, posted with the approval given, on the pilot lab's answers (the W18 replay route). The render test asserts it equals `0010`'s request with only the number and the REMOTEID tail changed. So A is HS's shape again, with bytes the build itself wrote.
  - **The one variable:** one invoice added, `BP/26-27/0040`, after a number read shows no voucher holds the number.
- **Part B, a replay.** `0010`'s own request, byte for byte (the build's request that created `BP/26-27/0010` on 9 Oct 2026, REMOTEID ending `0777`), sent once more to the book that holds `0010`, with `BRIDGE Sales`' guard on.
  - Section 9.14 of the voucher-writes reference saw a same-REMOTEID `ACTION="Create"` alter a voucher in place (`ALTERED` 1, same GUID). That was a Journal type with automatic numbering on TallyPrime 7.1 Silver, not a Sales type with Manual numbering and duplicates prevented.
  - **The one variable:** the same request sent again, nothing changed. Number, REMOTEID, figures, party and date all equal the stored voucher's.
- **Not asked here:** spaces and a blank number. The build cannot make either, because `InvoiceNumber::parse` (`agent_import_invoice.rs:560` at master ae38a75c) admits only non-empty ASCII letters, digits, `/` and `-`. They close by that reason, not by a row. A replay after the voucher was changed in Tally is not asked either.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2162-KC's, 2162-SC's, 2162-RU's, 2162-GF's and 2162-HS's) applies in full. Where it and this sheet differ, the stricter one holds.
- **The book is pinned by GUID.** `count15.py` holds BRIDGE PILOT LAB's GUID as `BOOK`. Its M kind prints whether the COMPANY row's GUID is the book's, and `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called. STOP before anything is sent that writes unless s1's COMPANY row is the book's.
- **Its state,** from the lab's last notes on it: rows queued before this one each end with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245.
- **BK is this row's way back.** It is taken at step 1, before anything is sent that writes. It is restored by C6 twice: at step 4, between the parts, and at step 7, after the last. Row 2162-S4 restored one backup several times in one sitting, and each restore read byte-identical.
  - The restores are not optional. Each runs after its part in every case, including a stop.
  - After each restore, every read is compared whole with the reads before the first import. A compare that is not equal stops the row.
- Write down the release shown on Tally's screen. Rows 2162-M1, 2162-B2C and 2162-HS read TallyPrime 7.2 Silver (tally.exe 1.1.7.2).

**No Bridge build.** Raw requests, one at a time, through `run.py`. The two imports are the only writes; nothing is keyed that saves.

**The sends** are row 2162-HS's, unchanged.
- Each of these stops the run and is reported as "the request failed", never as "nothing found":
  - a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`);
  - a read answer with no `BODY/DATA/COLLECTION`, or an answer that does not parse (`count15.py`, which prints STOP and exits non-zero).
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with `count15.py` and copy its whole output into RESULTS.md.
- Close every other Bridge process first.
- **C3's once-only rule holds for each import file in this row.** `I-0010.xml`'s bytes were sent to this book once before, on 9 Oct 2026, and that send created `0010`. Sending them again is part B's whole question. This row sends the file once, at step 5. It is never sent again, whatever its answer.

## Files
- **Byte-equal to row 2162-HS's file of the same name** (`lab/2162-hs-sheet` at b6767b87, folder `lab/2162-hs/`), whose ROW.md says where each came from: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count15.py`, `M-marks.xml`, `R-0010.xml` (the build's read-back for `BP/26-27/0010`), `template-import-0010.utf16le.xml` (the build's request that created `BP/26-27/0010`, sha256 equal to `PILOT_REQUEST_SHA256` on master) and `template-n-0001.utf16le.xml` (the build's number read).
- **`I-0040.xml`: the build's own write,** made by `src-tauri/src/agent_lab_2162_rp_render_tests.rs`, a test ignored by default. It and its one `mod` line in `agent_import_w18_replay_tests.rs` live on this branch only and are never part of a pull request.
  - The test builds `0010`'s entries (1,051.00 to `BRIDGE Walk-in`; 600.40 and 400.40 on the two sales ledgers; 25.02 each head; 0.16 round off) on the W18 and pilot lab answers. The number read is answered by W18's `w18-number-0040`: no voucher under `BP/26-27/0040`.
  - It then posts with the approval given and a scripted REMOTEID ending `0940`, and keeps the one write.
  - It asserts that write equals the `pilot-lab` fixture of `0010`'s request with only the number (twice) and the REMOTEID tail changed, then writes it here.
  - Run on master ae38a75c: `RP_OUT=<this folder> cargo nextest run --locked --run-ignored only -E 'test(rp_the_build_renders_0010)'`.
- **`gen.py`** checks `I-0040.xml` against the template with those two changes, and writes:
  - `I-0010.xml`: the template, copied byte for byte (part B);
  - `N-0040.xml` and `N-0010.xml`: the number read for each number. `N-0040.xml` comes out byte-equal to the request W18 sent for the same number (`workorder/w18-number-0040.xml`, sha256 `84f6b722...`, as the `pilot-lab-w18` PROVENANCE.md records it);
  - `R-0040.xml`: the read-back with the number `BP/26-27/0040`.
  - Every change is asserted to occur exactly as often as expected. Neither the number `0040` nor the REMOTEID tail `0940` appears in any earlier lab row's folder. W18 read `0040` and sent no write. The files are committed as generated; run `python3 -I gen.py` only to check them, and compare the hashes it gives with `SHA256.txt`.
- Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count15.py a-<label>-*`. A STOP from either script stops the row there.

**The prompt rule, at every step.** The only prompts answered are C5's and C6's. For any other prompt, message or dialog, copy it word for word and screenshot it first. Then answer No, or Esc if it offers no No. Press nothing more and send nothing. Then restore BK by C6 and compare as step 7, then close (C10). That prompt is this row's result.

**The wait rule, after each import.** Press nothing and send nothing until the import's answer is in, and the window after it is screenshotted.
- If 5 minutes pass after a send with no answer: send nothing, press nothing, screenshot the whole window, and report to the run coordinator. This is stricter than C3's 10 minutes. A read sent while Tally is busy can hold its gateway.
- With the answer in and the window showing the Gateway with no dialog, go on to C2's status check (`GET /status` 200, one tally.exe owning 9001) before the next read.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to the new empty folder `C:\lab\2162-rp\backup-1`. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `N-0040.xml N-0010.xml R-0010.xml M-marks.xml`.
   - EXPECT N-0040 0 rows: no voucher holds the number. One row or more: stop before step 3.
   - EXPECT N-0010 one row, Sales class, `BRIDGE Sales`. Record its GUID.
   - EXPECT R-0010 one row: `BRIDGE Walk-in`, included Yes, every other status flag No, 6 legs. Record its GUID and ALTERID.
3. **Part A, the build's own invoice under an unused number.** `s3m`: `M-marks.xml`, then `count15.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. It must pass; then no key is pressed in Tally before the send.
   - `s3`: `I-0040.xml`, sent once. Then the wait rule. EXPECT CREATED 1 and every other counter 0, no LINEERROR. The window after it (shot): the Gateway, no dialog. Anything else: record it, then go to step 4.
   - `s3r`: `N-0040.xml R-0040.xml N-0010.xml R-0010.xml M-marks.xml`.
     - EXPECT N-0040 one row, Sales class, `BRIDGE Sales`.
     - EXPECT R-0040 one row, `BRIDGE Walk-in`, the 6 legs as sent, included Yes and every other status flag No.
     - EXPECT N-0010 and R-0010 as at s2.
     - EXPECT ALTVCHID one above s3m's.
   - `count15.py --diff a-s3r-R-0010.xml a-s3r-R-0040.xml`. Record the whole output. EXPECT only the number and identity fields to differ, and `legs: 6 -> 6`.
4. **Restore** BK by C6, with `restore-plan.txt` written first. Then `s4`: `M-marks.xml N-0040.xml N-0010.xml R-0010.xml`, and compare whole:
   - `count15.py --same-row a-s1-M-marks.xml a-s4-M-marks.xml`: EXPECT it passes.
   - N-0040 0 rows, and its sha256 equal to s2's.
   - N-0010's and R-0010's sha256 equal to s2's. `count15.py --diff a-s2-R-0010.xml a-s4-R-0010.xml`: EXPECT only `lists compared` and `legs: 6 -> 6`.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
5. **Part B, the replay.** `s5m`: `M-marks.xml`, then `count15.py --before-write a-s1-M-marks.xml a-s5m-M-marks.xml`. It must pass; then no key is pressed in Tally before the send.
   - `s5`: `I-0010.xml`, sent once. Then the wait rule. Record every counter and any LINEERROR, word for word. The window after it (shot): the Gateway, no dialog. A dialog: the prompt rule.
   - `s5r`: `N-0010.xml R-0010.xml M-marks.xml`.
     - Record how many rows N-0010 holds, with each row's GUID.
     - Record R-0010's rows, each with its GUID and ALTERID.
     - Record ALTVCHID and ALTMSTID against s5m's.
   - If R-0010 holds one row: `count15.py --diff a-s2-R-0010.xml a-s5r-R-0010.xml`. Record the whole output. If it holds two: `count15.py --guid <s2's GUID> a-s5r-R-0010.xml`, and record both rows whole.
   - Any answer at s5 is a result. Record it and go on to step 6. Never send `I-0010.xml` again (C3).
6. **The look, read only.** Nothing is keyed here; it is a screenshot only. At the Gateway, write down whether anything on it changed. Then go on.
7. **Restore** BK by C6, with `restore-plan.txt` written again. Then `s7`: `M-marks.xml N-0040.xml N-0010.xml R-0010.xml`, and compare as step 4, with s7 in place of s4.
   - **Any of these not equal stops the row:** send nothing more, and report before the close.
8. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those C5 and C6 name, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. Say in the report what the book then holds:
- a stop between steps 3 and 4 leaves `0040` on the book;
- a stop between steps 5 and 7 leaves whatever s5 did to `0010`.

## Record
- Each answer's bytes and sha256, and `count15.py`'s lines.
- The `--diff`, `--guid`, `--before-write` and `--same-row` outputs, whole.
- s3's and s5's whole answers, every counter and LINEERROR word for word.
- The screenshots, listed by name, bytes and sha256. As text and word for word: every prompt, message or dialog, with the key that answered it and its time (UTC).
- The BK file and both restores, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **s2:** N-0040 0 rows; N-0010 and R-0010 as row 2162-HS's s2.
- **s3:** CREATED 1, stored as sent, included, 6 legs. That is what `0010` and HS's `0072` did.
- **s5:** `ALTERED` 1, `CREATED` 0, every other counter 0, no LINEERROR, no dialog.
  - N-0010 one row, with s2's GUID.
  - R-0010 one row, with s2's GUID, the same 6 legs and status flags, and its ALTERID moved to the book's next mark.
  - ALTVCHID one above s5m's.
  - This is section 9.14's finding carried to a Sales type. Duplicates prevented guards a number against another voucher, not a voucher against itself. That last point is an estimate.
- **s4 and s7:** equal to s2 and s1.

## Decides
**Part A.**
- **s3 created `0040` as sent:** the build's own bytes post under an unused number. With row 2162-HS's edited request, that is two runs of one shape. Item 11's first sub-item is measured, PARTIAL (one book, one release). No build change.
- **s3 did anything else:** the build's own request differs in effect from HS's edit. The two are asserted byte-equal apart from the number and the REMOTEID tail, so the coordinator decides from the answer.

**Part B. The guard's expected answer is no refusal.** Tally takes the replay as an alteration of `0010` itself (`ALTERED` 1, `CREATED` 0), and the guard does not fire, because no second voucher is made. Each outcome decides:
- **`ALTERED` 1, `CREATED` 0, one voucher under `0010` with s2's GUID, entries unchanged:** a byte-identical replay is not refused on Sales. It re-states the stored invoice in place, and makes no duplicate.
  - Item 11's "a replay is refused" then needs rewording: Tally does not refuse it, it alters.
  - Bridge's defence is that it never sends a REMOTEID twice. `RemoteIds::mint` (`agent_import_post.rs:3044`) mints a fresh random one per voucher at dispatch, recorded with the dispatch intent (#582).
  - The reference records it beside section 9.14's design consequence. A resend overwrites the stored invoice with the request's content: harmless here, where the two are equal, and not measured after an edit made in Tally.
- **`CREATED` 0, `ALTERED` 0, and `EXCEPTIONS` 1 or `ERRORS` 1:** the replay is refused in the answer. Record the LINEERROR, or that none came. Item 11's "a replay is refused" is then measured on Sales, PARTIAL. An `EXCEPTIONS` count without a LINEERROR is again the case section 9.2's success rule exists for.
- **`CREATED` 1, two vouchers under `0010`:** the guard does not hold against a same-REMOTEID replay. That is a switch-on blocker. Sales posting stays off until the build has a defence that does not rely on Tally. The coordinator decides with the owner.
- **A dialog after s5:** the replay fails inside Tally's screen, not in its answer (P7). That decides against any path that could resend, before any count does.
- **`ALTERED` 1 with an entry, a flag or the number changed:** record exactly what moved. A replay that changes a stored invoice is a finding for the reference, and it goes to the coordinator before anything else.

## Code admits the trigger
Lines are at master ae38a75c.
- **Part A is the build's write.** The render test runs the build and the post through `build_import_xml` and `post_import`, with only the REMOTEID scripted. The write it keeps is the one the post dispatches. The number check passes because the W18 number read answers no voucher under `BP/26-27/0040`.
- **Part B is not a path a release sends.** A native post mints its REMOTEIDs fresh at dispatch (`RemoteIds::mint`, `agent_import_post.rs:3044`) and never derives or reuses one. So a replay of `0010`'s request can only come from outside the build. The row sends it raw to measure what Tally would do with one, because the build's safety rests on never sending it.
- **The guard the build relies on:** `resolve_voucher_type` admits a Sales type only with Manual numbering and `PREVENTDUPLICATES` Yes (`agent_import_invoice_wire.rs:383-387`). Row 2162-S4 measured that guard against a second voucher. No row has measured it against the same voucher.

## Not measured
- A replay after `0010` was altered or cancelled in Tally, or deleted (section 9.14 saw a resend after a delete create a new voucher on a Journal type).
- A replay with the same REMOTEID and another number, or another figure.
- A number with spaces, or a blank number: the build cannot make either.
- An invoice posted through a Bridge build.
- Any other book or release.

Row: lab/2162-rp/
