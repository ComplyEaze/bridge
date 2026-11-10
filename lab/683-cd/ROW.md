# Row 683-CD: on the several-currency lab book, does Tally's current date alone change the Trial Balance to 1 April

Not for merge. Part of #683. **Reads only:** no voucher or master is keyed, so there is no BK and no restore. The one thing changed is Tally's current date, by F2 at the Gateway, and it is set back before the close. Lines are at master ae38a75c.

## Question
Row 683-TW sent the Trial Balance from 1 April 2025 to 1 April 2025 (`tb3`, request sha256 `1853d0df…1ad1a`) on this book. It answered 14,140 B, byte-equal to its Trial Balance to 30 April, with six ledgers MIXED (each reading the dollar composite). Row 683-LD sent the same request after two vouchers dated 30 April 2025 were saved. It answered 13,528 B, with every ledger PLAIN: each of the six read its rupee figure alone, and every rupee figure was the same as before (`lab/683-ld-answers` e38d2b82, `RESULTS.md`). Two things differed between those sittings:
- the two vouchers dated 30 April, which row 683-LD then restored away;
- Tally's current date: 1-Apr-2025 in row 683-TW's sitting, 30-Apr-2025 in row 683-LD's (moved by K1's voucher date).

Row 683-LD does not say which of the two changed the answer. This row changes the second alone (P6). The book is as row 683-TW read it (row 683-LD's restore returned it to 11/214 and its book-scoped window to the bytes found). The request is the same file. Only Tally's current date moves, from 1-Apr-2025 to 30-Apr-2025, and then back to 1-Apr-2025.

**Q1:** with the book as found and the current date at 30-Apr-2025, does `tb3` answer as row 683-TW's did (six MIXED) or as row 683-LD's did (all PLAIN)? `tb.py --sets` answers it in one line against each.

**The controls:**
- **`tb3a`,** the same request at the current date row 683-TW had (1-Apr-2025), before the date is touched. EXPECT byte-equal to row 683-TW's `tb3` answer. Then a difference in `tb3d` is about the date, not the book, the sitting or the script.
- **`tb3r`,** the same request again after the date is set back to 1-Apr-2025. A `tb3r` that differs from `tb3a` means the date's return did not return the answer, and `tb3d` then decides nothing until that is explained (P5: an anomaly is repeated before it is believed).

Sending master's own builder raw is shape discovery on a lab book, which the owner allows. The request is not a new shape: it is the file rows 683-TW and 683-LD sent, byte for byte.

## Book
`BRIDGE FOREXGROUP LAB` (synthetic), GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, TallyPrime 7.2. THE COMPANY in `LAB-COMMON.md` means this book.
- **Synthetic:** it is in master's registry, `scripts/merge-gate-synthetic-uuids.json` line 201 ("created in the lab on 28 October 2026 with invented masters and vouchers keyed by hand").
- **Not an excluded book:** its GUID is not the register lab book's (`REGISTER_LAB_GUID`, `captured_parse_record.rs:302`), and not the one the private exclusion list names.
- **Pinned by GUID.** The `BOOK` of `lm.py` and `tb.py` is this GUID. m0 must print one COMPANY row, `the book's: yes`, or the row STOPs before any other send.
- **Its last state** (row 683-LD, `RESULTS.md`): restored from its backup, then m2 byte-equal to m0 (V 11, M 214), and the book-scoped window byte-equal to the one found. At its step 9 the Gateway showed current date Tuesday, 1-Apr-2025, period 1-Apr-25 to 31-Mar-26, last entry 1-Apr-25. No row has written to it since. `a-ldm2-M-marks.xml` here is that m2.
- **The period:** do not change it.

## The current date, and why changing it writes nothing
F2 at the Gateway opens **Change Current Date**. Rows 2015 and 2016 used it with the marks read before and after, and each found them unchanged. Row 1039 set the date back the same way at its close. This row reads the marks before (`m0`), after the change (`m1`) and after the set-back (`m2`). Each must be SAME MARKS with m0. A difference is a STOP (below), because the row then wrote something it did not mean to.

**The path.** At the Gateway, with no report open:
1. F2. EXPECT a box titled `Change Current Date` with one date field. Screenshot it and record the date it offers.
2. Type the date as `d-m-yyyy` (`30-4-2025`, then later `1-4-2025`), Enter.
3. EXPECT the Gateway again, showing the new current date with its weekday: Wednesday, 30-Apr-2025, then Tuesday, 1-Apr-2025. Screenshot it. The period and the last entry date must read as before (1-Apr-25 to 31-Mar-26, last entry 1-Apr-25).

**The path rule.** If what opens is not that box (another title, a second field, a list, a master or voucher screen), or the box does not take the date (it stays, or a message comes up), then:
1. stop, and press Esc once to leave the box only (never from a master or voucher screen, C7);
2. send nothing;
3. read the Gateway's current date and record it;
4. report to the run coordinator, who decides whether the date is set back and the book closed by C10.

## Kit
Check every file against `SHA256.txt` first, and check each send's request file again just before it is sent (Get-FileHash, C3). If any file differs: STOP, and nothing is sent.

| file | what, and where its bytes come from |
|---|---|
| `M-marks.xml` | The marks request (GUID, ALTVCHID, ALTMSTID). Byte-equal to rows 683-TW's and 683-LD's. |
| `T-tb3-20250401-20250401.xml` | Row 683-TW's `tb3` request, byte-equal (sha256 `1853d0df…1ad1a`): `render_native_trial_balance_request_with_currency` (`native_trial_balance/mod.rs:170`), from 20250401 to 20250401. Row 683-LD sent the same file. All three sends of this row use it. |
| `a-twtb3-T-tb3-20250401-20250401.xml` | Not sent. Row 683-TW's `tb3` answer (`lab/683-tw-answers` 4f31b925, sha256 `1c4638e8…b7b7`, 14,140 B; six MIXED). |
| `a-ldtb3-T-tb3-20250401-20250401.xml` | Not sent. Row 683-LD's `tb3` answer (`lab/683-ld-answers` e38d2b82, sha256 `a12217b8…7705`, 13,528 B; all PLAIN). |
| `a-ldm2-M-marks.xml` | Not sent. Row 683-LD's m2, the book's marks after its restore (sha256 `3d75dd1a…afb7`, 3,404 B; byte-equal to row 683-TW's m0). |
| `run.py`, `LAB-COMMON.md`, `.gitattributes`, `lm.py`, `tb.py` | Rows 683-TW's and 683-LD's, byte-equal. |

## Screens and prompts
This row opens no master, voucher or report screen, and expects no prompt. The screens it opens are the Gateway's F2 box and the company screens rows 683-TW and 683-LD used with no prompt:
- **Load:** Select Company, the book typed into its name filter, the one row, Enter.
- **A company loaded at the start:** C1 says STOP. If the run coordinator's word is to shut it, shut it by the K: Company label, Shut, Enter on it, as row 683-LD shut the start-up company, with no change to it.
- **Close (C10):** the K: Company label, Shut, Enter on the book.
- **Bring Tally to the front only by its running window.** A second `tally.exe` is a STOP.

## The wedge rule
A read that Tally is still building keeps the gateway busy for every company, and giving up on it does not stop Tally's work. So:
- One request at a time, by `run.py`. It never abandons a request: curl waits for its answer.
- This book is small (11 vouchers, 8 ledgers). This request has answered twice, at 14,140 B and 13,528 B; row 683-LD's took about a second. If any send has not answered **5 minutes** after it began:
  - press nothing, and do not end curl;
  - send nothing more, not even `GET /status`;
  - screenshot the whole Tally window, and report to the run coordinator (C9).

  Nothing is re-sent on this row's own judgement.
- `run.py`'s own stops (a curl exit other than 0, Unknown Request, an empty answer, an answer above 2,000,000 B) and `lm.py`'s or `tb.py`'s STOP are failed requests, never "nothing found". Each stops the row there.

## Steps
Run each send as `python3 -I run.py http://127.0.0.1:9001 <label> <file>`, and write down each send's start time from its log. **Each label is used once.** A repeated send takes a new label (`m1r`), never an earlier one's. The answer of send `<label>` is `a-<label>-<request file>`.

0. **C2's step 0, with no company loaded:**
   - the screenshot first;
   - one `tally.exe` owning 9001;
   - `GET /status` 200 (record its bytes).

   Load the book alone, as above. Read the Gateway's current date, period and last entry date, and write them down. Record the release and the licence mode, as row 683-LD recorded them, and the PC's date.
   - EXPECT current date Tuesday, 1-Apr-2025, period 1-Apr-25 to 31-Mar-26, last entry 1-Apr-25.
   - If the current date is not 1-Apr-2025, or the period differs: record it and STOP before any send. Press nothing to change it. The control assumes row 683-TW's date, and the run coordinator decides.
1. **`m0`:** `M-marks.xml`, then `python3 -I lm.py --same-row a-ldm2-M-marks.xml a-m0-M-marks.xml`. EXPECT SAME MARKS: one COMPANY row, the book's, with ALTVCHID 11 and ALTMSTID 214.
   - Any other GUID or row count: STOP.
   - MARKS DIFFER: record it and STOP before step 2. The book changed after its last note.
2. **`tb3a`** (the control, at 1-Apr-2025): `T-tb3-20250401-20250401.xml`. Record whether its answer is byte-equal to `a-twtb3-T-tb3-20250401-20250401.xml` (sha256). Then:
   - `python3 -I tb.py a-tb3a-T-tb3-20250401-20250401.xml`, recorded whole;
   - `python3 -I tb.py --sets a-twtb3-T-tb3-20250401-20250401.xml a-tb3a-T-tb3-20250401-20250401.xml`, recorded whole. EXPECT SETS SAME.

   If it is not as EXPECT, record it and go on: it is a result, and Decides says what it means.
3. **The date to 30 April:** by the path above, F2, `30-4-2025`, Enter. Record the box's offered date and the Gateway after it (two screenshots). Then **`m1`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m0-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS. On MARKS DIFFER:
   - send `M-marks.xml` once more as `m1r` (P5, C8) and record it;
   - send nothing else, and STOP. The date is left as it is until the run coordinator's word.
4. **`tb3d`** (at 30-Apr-2025): `T-tb3-20250401-20250401.xml`. Then, at once, screenshot the whole Tally window: this is the first Trial Balance on this book sent with the current date after every voucher. A dialog = Stop. Record whether the answer is byte-equal to `a-tb3a-…`, and whether it is byte-equal to `a-ldtb3-…`. Then:
   - `python3 -I tb.py a-tb3d-T-tb3-20250401-20250401.xml`, recorded whole;
   - `python3 -I tb.py --sets a-tb3a-T-tb3-20250401-20250401.xml a-tb3d-T-tb3-20250401-20250401.xml`, recorded whole;
   - `python3 -I tb.py --sets a-ldtb3-T-tb3-20250401-20250401.xml a-tb3d-T-tb3-20250401-20250401.xml`, recorded whole.
5. **The date back to 1 April:** by the path above, F2, `1-4-2025`, Enter. Record the box's offered date and the Gateway after it (two screenshots). EXPECT current date Tuesday, 1-Apr-2025, the period and last entry as at step 0.
6. **`tb3r`** (at 1-Apr-2025 again): `T-tb3-20250401-20250401.xml`. Record whether the answer is byte-equal to `a-tb3a-…`. Then:
   - `python3 -I tb.py a-tb3r-T-tb3-20250401-20250401.xml`, recorded whole;
   - `python3 -I tb.py --sets a-tb3a-T-tb3-20250401-20250401.xml a-tb3r-T-tb3-20250401-20250401.xml`, recorded whole.
7. **`m2`:** `M-marks.xml`, then `python3 -I lm.py --same-row a-m0-M-marks.xml a-m2-M-marks.xml`. EXPECT SAME MARKS, and m2 byte-equal to m0 (record both sha256). On MARKS DIFFER: `m2r` once, as at step 3, then STOP.
8. **Close** by C10: read the Gateway's current date and period once more and record them, then shut the book. `GET /status` after the last request (C11). One `tally.exe` owns 9001.

Steps 2, 4 and 6 never stop the row on an answer that differs from EXPECT or from the estimates: each is a result, recorded in full. Only the STOPs named here stop it.

## Stop
Press no key, screenshot the whole window, and report before anything else is pressed or sent, if:
- a dialog or prompt appears outside the F2 box, or a second `tally.exe` starts;
- the gateway stops answering, or a send passes the wedge rule's 5 minutes;
- `run.py`, `lm.py` or `tb.py` stops. Their STOPs are listed in their headers and in rows 683-LM's and 683-TB's sheets;
- step 0's current date or period is not as EXPECT;
- m0 is not SAME MARKS with `a-ldm2`;
- m1 or m2 differs from m0;
- the path rule applies.

Nothing is keyed, so there is never anything to restore. If a STOP leaves the current date at 30-Apr-2025, it is set back (step 5's path) only on the run coordinator's word.

## Record
- Each answer's bytes and sha256, and each request's sha256 (as `SHA256.txt` lists; any other is the Stop of C3).
- `lm.py`'s lines for every marks read, and the whole output of each `tb.py` call.
- Whether `tb3a` is byte-equal to `a-twtb3`, whether `tb3d` is byte-equal to `tb3a` and to `a-ldtb3`, and whether `tb3r` is byte-equal to `tb3a`. Either way it is recorded, not judged.
- Each send's start time and how long it took, from `log-<label>.txt`.
- Tally's current date, period and last entry date at steps 0, 3, 5 and 8, each with the date the F2 box offered; the release, the licence mode, and the PC's date.
- The marks at m0, m1 and m2.
- Every screenshot by name, bytes and sha256 (screenshots are never pushed).

**Staging the evidence**, as for rows 683-LM's, 683-TW's and 683-LD's:
- **The start-up check:** a hit is allowed only where it lies inside the amount `$1000000.00`. List each hit with 12 characters either side. A hit anywhere else waits for the run coordinator.
- **The check for 10 or more digits:** it sets aside this book's registered GUID matched whole, and nothing else.

## Expected (estimates, not EXPECTs)
- `tb3a` byte-equal to `a-twtb3`, six MIXED, as on row 683-TW.
- `tb3d`: not known. That is the question.
- `tb3r` byte-equal to `tb3a`.
- m0, m1 and m2 byte-equal.

## Decides
- **`tb3a` as row 683-TW's, `tb3d` all PLAIN (SETS DIFFER against `tb3a`, the six MIXED to PLAIN), `tb3r` as `tb3a`:** Tally's current date alone changes what the Trial Balance to 1 April reads for the dollar ledgers, with the book unchanged, on this book and release: PARTIAL. Row 683-LD's PLAIN figures then came from the current date. Record whether `tb3d` is byte-equal to `a-ldtb3`; where it is not, record which fields differ, since row 683-LD's book had two more vouchers.
- **`tb3a` and `tb3r` as row 683-TW's, `tb3d` SETS SAME against `tb3a`:** the current date alone does not change this answer. Row 683-LD's change then came with the vouchers dated 30 April, alone or together with the date. Separating those two needs the vouchers and a current date of 1 April in one sitting, which is a write behind a backup. That is another row, drafted only on the run coordinator's word.
- **`tb3d` neither:** some ledgers MIXED and some PLAIN, other text, or any UNREAD. Record it whole; it decides nothing until the run coordinator has read it.
- **`tb3r` not as `tb3a`:** setting the date back did not return the answer. `tb3d` decides nothing until that is explained.
- **`tb3a` not as row 683-TW's:** the book or the sitting reads differently from row 683-TW's at the same date. `tb3d` and `tb3r` are recorded, but they decide nothing until that is explained.

Whichever way it falls, any answer that becomes a fixture is parsed again by master's own parser first, before any code relies on it.

## Not in this row
- **The two vouchers dated 30 April with a current date of 1 April.** That needs a write behind a backup (see Decides).
- Any Trial Balance other than the one to 1 April, any movement window or opening export, any current date other than 1 April and 30 April 2025, and any change to the period.
- `trial_balance` or `ledger_movement` through Bridge, another book, another release, and any write.
