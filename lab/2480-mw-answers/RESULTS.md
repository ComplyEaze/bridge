# Row 2480-MW: the set-up passed as row 2480-MX's did; the shape step passed, and each of the eleven names, in both voucher reads in the `##LedgerName` form, read its own ledger's one voucher as the book-wide read shows it

Run on 11 Nov 2026, 01:40:08Z to 02:02:05Z (07:10 to 07:32 IST), on TallyPrime single-user edition: the Gateway on 9001, `tally.exe` file version 1.1.7.2 (`evidence/tally-exe.txt`, equal to row 2480-MX's record; the Gateway shows no release number). The sheet was `lab/2480-mw-sheet` (a7bf1de3), with all 110 files equal to its `SHA256.txt`; every file byte-equal to row 2480-MX's sheet but `ROW.md`, `wx.py` and `SHA256.txt`. Before the row, MW's `wx.py` read MX's kept `a-k1-Vm-u-pl.xml` as `ANSWER: VOUCHERS 1` (exit 0), where MX's `wx.py` had stopped. One book, `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic, in the synthetic-uuid registry), loaded alone, in a sitting of its own with a Tally start of its own. No Bridge build ran and no Bridge tool was called. Each request file's sha256 and size were checked against the sheet's `SHA256.txt` just before its send (`evidence/sends-utc.txt`).

The set-up and the voucher reads are judged apart, as the sheet says: each section is written from its own answers.

## The set-up (steps 1 to 5)

**Confidence: M, measured by this run, in one run, for each line below; PARTIAL as the sheet's Decides says (one book, one release, one sitting, master's request templates sent by curl, without the build's own parse). The set-up decides nothing new: it is MX's, and MX's part A results stand.** Every send of the set-up was answered: each with curl exit 0 and HTTP 200, with bytes. No dialog, message or prompt showed but the two of C5's and C6's own (none at the loads or the shuts). Every capture after a send was the Gateway, the same image as s1's.

**Step 1.** The Gateway: the book alone; period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026. s1's marks were 68/245, as the sheet asks, before the backup.

**Step 2, the book before.** Each N read 0 rows; B-bills no group for any of the eleven names; the snapshot 18 LEDGER rows and "no row" for each name; `st.py` (the before option) `FOLD: CLEAR`; `st.py --names` `NAMES: EQUAL`; the marks 68/245. **None of MX's set-up was in the book:** the snapshot answer was byte-equal to MX's own s2 answer, and both `st.py` outputs equal to MX's.

**Step 3, the set-up.** MX's 22 import files, byte for byte, each sent once, in table order, each after a marks read that passed the before-write check: every answer `CREATED` 1 and the other counts 0, with no LINEERROR, so `st.py --import` gave `IMPORT: CREATED ONE` on each. The invoices' `LASTVCHID` ran from 45 (pl) to 55 (al) in table order, as at MX's step 4. **No import was refused, so `rf.py` was never run** and the run lead's exception (c) was not needed.

**Step 4, the book-wide reads: the intended state.** s6m 79/256. The snapshot, sent twice: 39,750 B each, byte-equal, 29 LEDGER rows, a row for each of the eleven, each `Sundry Debtors`, bill-wise Yes, opening 0.00, closing -1051.00; `st.py --names` `NAMES: EQUAL`, each ledger's name list its own name alone. B-bills, sent twice: 5,396 B each, byte-equal, 11 BILLFIXED groups, one for each name, `BP/26-27/0281` to `BP/26-27/0291` in table order, each dated 1-Aug-26 and -1051.00, none of any other party. Both answers were byte-equal to MX's s6sa and s6ba. **CREATED: `pl,ga,ss,ms,qd,pp,hd,ad,ca,xs,al`**, all eleven.

**Step 5.** m1's marks were equal to s6m's (79/256): step 4 only read. No m1r was sent.

**The set-up whole, before k0** (the sheet's step 6, written in `evidence/notes.txt` before k0, each with the line that shows it): no stop at any step from 1 to 5; every send answered; m1 equal to s6m, with no m1r sent; every before-write check passed, and no refusal; the control `pl` in CREATED and in the intended state (its S row, bill-wise Yes, closing the invoice's amount, one B group `BP/26-27/0281`, `nm.py --same` `EQUAL` on both pairs). All five held, so the voucher reads started.

**What the set-up does not say.** Anything MX's part A did not say: the set-up only made the book again as MX made it, and checked it as MX did.

### The set-up's deviations from the sheet

- **`python` for `python3`.** This PC has no `python3` command. Every script ran as `python -I` (Python 3.10), unchanged, from the row folder, through the helpers in `scripts/`. Once, at step 2, `st.py` was typed with `python3`: that call ran nothing, and both of its output files were written again by `python` (in the notes).
- **The run lead's three words for this row** (in the queue entry, as in item 80): (a) an empty `L-lo-` answer with HTTP 200 and `CONTENT-LENGTH: 0` is not a stop (no `L-lo-` file is sent in this row); (b) the same for a `Vm-v-` or `Vw-v-` answer, recorded as `wx.py` reads it; (c) an import that `rf.py` reads as refused, with an equal marks read after it, is that id's result. `scripts/vr.sh` carried (b), which was not needed; `scripts/w.sh` carried (c), which was not needed.
- **The start-up company** loaded by itself when Tally started. It was shut (K: Company, Shut, Enter on its one row, no prompt) before the book was loaded, on the run lead's standing word.
- **The BK path field.** At C5's Specify Path, the typed path went in ahead of the field's old `C:\`, so the field read the path with `C:\` after it. End and three Backspaces took that off before Enter, and the field read `C:\lab\2480-mw\backup-1` alone; the BK went to that folder (one file, `evidence/bk.txt`).
- **The three Bridge processes** that ran before the row (two `bridge_mcp.exe` and `BridgeCommunication.exe`) were left running, as the sheet says; no process is ended on this PC. None was called by the row.
- **Sends run in the background, with a 300 s watch.** From step 2 on, each block of sends was started by `scripts/bg.sh`, which writes the block's output and exit to `evidence/run-<name>.out` and `.rc`, beside `scripts/wd.sh`, which watched the same block for 5 minutes and ended when it did. So no tool's time limit could end a curl that waited (the wedge rule: never end curl). Each block ended inside the watch; the longest, step 3, took 147 s. s1 (one marks read) was sent in the foreground.
- **One id a command, looped.** Each id's four write labels were sent by one command, `scripts/wid.sh <id>`, which runs `scripts/w.sh` for each in the recipe's order and ends at the first exit other than 0. Step 3 ran `wid.sh` for the eleven ids by one loop, `scripts/s4.sh`, which went on only on exit 0 (or 4 or 5, a refusal, which did not occur). Step 4 was sent by one loop, `scripts/s5.sh`.
- **The Gateway images.** `evidence/gateway-refs.txt` carries the three Gateway images of row 2480-NX, each looked at there, as in MX. This row's s1 capture was byte-equal to the first. Every capture after a send in this row was that same image (`ca58f95e…`), as was s9's.
- **The notes' stamps.** The lines of steps 1 to 5 and of the set-up check were first written with rounded stamps, some later than the line itself. Each was set to the time the line was written (the start of the next background block, within a second of it); no other text changed, and the notes say so.

## The voucher reads (steps 6 and 7)

**Confidence: M, measured by this run, in one run, for each line below; PARTIAL as the sheet's Decides says (one book, one release, one sitting, master's request templates sent by curl, without the reads Bridge sends around them, and without the build's own parse).** Every send of steps 6 and 7 was answered: each with curl exit 0 and HTTP 200, with bytes; no answer was empty, so exception (b) was never needed. No dialog showed. Every capture after a send was the Gateway image above.

**Step 6, the shape step, the control alone.** Each send was checked before the next, by `scripts/k8.sh`:

| label | file | answer | check | gave |
| --- | --- | --- | --- | --- |
| k0 | `M-marks.xml` | 79/256 | `--same-row` against m1 | SAME MARKS |
| k1 | `Vm-u-pl.xml` | 11,954 B | `wx.py` | `ANSWER: VOUCHERS 1` |
| k2 | `Vm-uv-pl.xml` | 11,954 B | `wx.py --same` against k1 | `SAME: EQUAL` |
| k3 | `Vm-v-pl.xml` | 11,954 B | `wx.py --same` against k1 | `SAME: EQUAL` |
| k4 | `Vw-u-pl.xml` | 94,752 B | `wx.py` | `ANSWER: VOUCHERS 1` |
| k5 | `Vw-uv-pl.xml` | 94,752 B | `wx.py --same` against k4 | `SAME: EQUAL` |
| k6 | `Vw-v-pl.xml` | 94,752 B | `wx.py --same` against k4 | `SAME: EQUAL` |
| k7 | `Vb-book.xml` | 1,821,796 B | `wx.py`; `--match pl movement` (k1); `--match pl window` (k4) | `ANSWER: VOUCHERS 22`; `EQUAL`; `EQUAL` |
| k8 | `M-marks.xml` | 79/256 | `--same-row` against k0 | SAME MARKS |

k1 and k4 each held the control's one invoice, `BP/26-27/0281`, dated 20260801, ALTERID 69, 6 entries. k1's answer was byte-equal to MX's kept k1 answer: the same answer MX's reader stopped on, which MW's reader read. It holds one bill allocation with an `AMOUNT` alone (and five empty ones), the shape the sheet expects from master's movement capture and the one MW's reader admits; k4's party line holds the named bill, `BP/26-27/0281 New Ref -1051.00`. k7, the book-wide read of 1-Aug-2026, was 1,821,796 B, under the 2,000,000 B bound and near the sheet's estimate of about 1,800,000 B, with 22 vouchers, each dated 20260801. `MATCH pl movement: EQUAL` (1 voucher, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal) and `MATCH pl window: EQUAL` (its line Yes -1051.00; bill `BP/26-27/0281` New Ref -1051.00; 14 own fields equal). Once k8 had passed, `wx.py` on k2, k3, k5 and k6 each gave `ANSWER: VOUCHERS 1`, for the record. No k0r or k8r was sent. **The shape step passed.**

**So, by the sheet's Decides, the shape step:** on this book and release, each data-carrying ledger-scoped read in the `##LedgerName` form gave master's bytes for the control (k3 equal to k1, k6 equal to k4), the added variable changed nothing in master's form (k2, k5), master's form held what the book-wide read of the day holds for that ledger (k1, k4 against k7), and these reads left the marks as they were (k0 to k8). The ten were then measured.

**Step 7, the ledger reads.** For each id of CREATED, in table order, four sends, each read by `wx.py` before the next: `q<n>a` and `q<n>b` (`Vm-v-<id>.xml`), then `w<n>a` and `w<n>b` (`Vw-v-<id>.xml`), then the two `--match` lines against k7. MX's part A result for each id stands beside this row's:

| n | id | name | MX part A: step 3, report, against | MW set-up: ledger, invoice; name list | movement: q a, q b | movement `--same` | window: w a, w b | window `--same` | `MATCH` movement | `MATCH` window |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | pl | `BRIDGE Mx Plain` | NO BYTES, told, EQUAL | CREATED ONE, CREATED ONE; its own | VOUCHERS 1, VOUCHERS 1 (11,954 B each) | EQUAL | VOUCHERS 1, VOUCHERS 1 (94,752 B each) | EQUAL | EQUAL | EQUAL |
| 2 | ga | `BRIDGE Mx Ak, Am & Co` | NO BYTES, told, EQUAL | CREATED ONE, CREATED ONE; its own | VOUCHERS 1, VOUCHERS 1 (11,994 B each) | EQUAL | VOUCHERS 1, VOUCHERS 1 (94,792 B each) | EQUAL | EQUAL | EQUAL |
| 3 | ss | `BRIDGE Mx Bk & Sons (Bm), Bn` | NO BYTES, told, EQUAL | CREATED ONE, CREATED ONE; its own | VOUCHERS 1, VOUCHERS 1 (12,022 B each) | EQUAL | VOUCHERS 1, VOUCHERS 1 (94,820 B each) | EQUAL | EQUAL | EQUAL |
| 4 | ms | `BRIDGE Mx M/s. Ck_Cm, Cn` | NO BYTES, told, EQUAL | CREATED ONE, CREATED ONE; its own | VOUCHERS 1, VOUCHERS 1 (11,990 B each) | EQUAL | VOUCHERS 1, VOUCHERS 1 (94,788 B each) | EQUAL | EQUAL | EQUAL |
| 5 | qd | `BRIDGE Mx "Dk" Dm & Dn` | NO BYTES, told, EQUAL | CREATED ONE, CREATED ONE; its own | VOUCHERS 1, VOUCHERS 1 (12,038 B each) | EQUAL | VOUCHERS 1, VOUCHERS 1 (94,836 B each) | EQUAL | EQUAL | EQUAL |
| 6 | pp | `BRIDGE Mx Ek 50% + Em` | NO BYTES, told, EQUAL | CREATED ONE, CREATED ONE; its own | VOUCHERS 1, VOUCHERS 1 (11,978 B each) | EQUAL | VOUCHERS 1, VOUCHERS 1 (94,776 B each) | EQUAL | EQUAL | EQUAL |
| 7 | hd | `BRIDGE Mx Fk #2, Fm-Fn` | NO BYTES, told, EQUAL | CREATED ONE, CREATED ONE; its own | VOUCHERS 1, VOUCHERS 1 (11,982 B each) | EQUAL | VOUCHERS 1, VOUCHERS 1 (94,780 B each) | EQUAL | EQUAL | EQUAL |
| 8 | ad | `BRIDGE Mx Gk @ Gm.` | NO BYTES, told, EQUAL | CREATED ONE, CREATED ONE; its own | VOUCHERS 1, VOUCHERS 1 (11,966 B each) | EQUAL | VOUCHERS 1, VOUCHERS 1 (94,764 B each) | EQUAL | EQUAL | EQUAL |
| 9 | ca | `BRIDGE Mx Hk: Hm's (Hn)` | NO BYTES, told, EQUAL | CREATED ONE, CREATED ONE; its own | VOUCHERS 1, VOUCHERS 1 (12,006 B each) | EQUAL | VOUCHERS 1, VOUCHERS 1 (94,806 B each) | EQUAL | EQUAL | EQUAL |
| 10 | xs | `BRIDGE Mx Jk! Jm *` | NO BYTES, told, EQUAL | CREATED ONE, CREATED ONE; its own | VOUCHERS 1, VOUCHERS 1 (11,966 B each) | EQUAL | VOUCHERS 1, VOUCHERS 1 (94,766 B each) | EQUAL | EQUAL | EQUAL |
| 11 | al | `BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly's` | NO BYTES, told, EQUAL | CREATED ONE, CREATED ONE; its own | VOUCHERS 1, VOUCHERS 1 (12,258 B each) | EQUAL | VOUCHERS 1, VOUCHERS 1 (95,058 B each) | EQUAL | EQUAL | EQUAL |

Each `MATCH` line names that id's own invoice, `BP/26-27/0281` (pl) to `BP/26-27/0291` (al) in table order, dated 20260801 and matched by VOUCHERKEY, its line on the ledger Yes -1051.00, with 10 own fields equal (movement) or 14 own fields equal and the bill `BP/26-27/<its number>` New Ref -1051.00 (window). The whole outputs are below. Then k9 (`Vb-book.xml`, the file k7 sent): `ANSWER: VOUCHERS 22`, `SAME: EQUAL` against k7 (1,821,796 B each), so every `MATCH` line was judged against a book-wide read that did not change during step 7. k10's marks were equal to k8's (79/256): step 7 only read. No k10r was sent.

**So, by the sheet's Decides:** for each of the eleven names of CREATED, in each of the two reads, `MATCH: EQUAL` with the pair `SAME: EQUAL`: on this book and release, the movement read and the window read in the `##LedgerName` form each read that ledger's vouchers as the book-wide read shows them. With rows 2480-CV and 2480-MX, that is evidence that each read can take each of these names as data. **The shape step passed and every name of CREATED was `EQUAL` in both reads.** By the sheet, `ledger_movement` and the `outstandings` party detail could then send their ledger-scoped reads with the name as the static variable, for every name the static-variable type admits, on the evidence of one book and release; the build of that change is #2480's and #485's, and the choice is the maintainers'.

**What the voucher reads do not say.** Whether the build's own parse accepts these answers; the reads Bridge sends around them; any other book, release or day; a window of more than one day, an AlterID span or a book of real size; any name inside `CHILDOF` itself; a ledger with more than one voucher or bill, a settled or part-settled bill, a voucher with two lines on the ledger, a note or a voucher after the invoice; a book with GST on; the census in this form for these names. `wx.py`'s empty-answer and `STATUS` paths met no real answer here.

### The voucher reads' deviations from the sheet

- **`scripts/k8.sh`** sent k0 to k8 one at a time, each check run before the next send, and would have ended at the first check that did not give what the sheet names; none did.
- **Step 7 in two runs.** To keep each block inside the 300 s watch, step 7 ran as two blocks of `scripts/q9.sh`: `pl` to `pp` (125 s), then `hd` to `al` (106 s), each with its own watch. `q9.sh` numbers each id by its place in the table, so the labels are the sheet's. No id was left unread. Each send went by `scripts/vr.sh`, which ends with exit 3 when the answer came with bytes (its answered path, on every send here) and carries exception (b) for an empty one. k9 and k10 were sent by `scripts/k9.sh`.
- **Captures after q<n>b and w<n>b.** `vr.sh` captures the window after each of its sends, so each `b` send has a capture as well as each `a` send. k10 has none: `k9.sh` captures after k9 alone.

## The way back

**Step 8.** BK was restored by C6, with only the two expected prompts. Then s9 sent every file of s2, in s2's order: every answer byte-equal to its s2 answer (`evidence/s9-against-s2.txt`, 14 of 14), each N read 0 rows, no group and no row for any of the eleven names, and the marks equal to s1's (68/245; `count20.py --same-row`). So the book holds again what it held at step 1.

**The side files.** Before Tally started, four of the seven files that the restores of rows 2480-NX, 2480-CV and 2480-MX left out were in the book's data folder (`CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`). At load, six were: `TCMPSCHMESSAGE.TSF` and `TINTMSG.TSF` were written at this load, with `TCMPSCHACCESS.TSF` and `TCMPSCHSTATE.TSF` written again; `Index.1800` was absent. Before the restore the folder held all seven (24 entries); this row's restore left out the same seven (17 entries after), and at the load after it the same six were back and `Index.1800` absent, as in those rows. The listings stay on the lab PC.

**Timings.** Every read sent without a window capture after it returned within 1 s of its send line (0.399 to 0.625 s), wall clock, python's start included, but s2 and s9: s2's fourteen reads took 2.047 s together and s9's 4.905 s. The labels sent with a capture took 2.926 to 6.032 s, the capture included: the imports 3.043 to 4.421 s, k0 to k9 2.926 to 6.032 s (k9, the book-wide read, the longest; k7 4.675 s), and q1a to w11b 3.023 to 4.842 s (`evidence/durations.txt`).

## Steps
- Step 0 (01:40:08Z): the sheet's files checked; the book's data folder found once by name and GUID; the excluded GUID absent (`evidence/bookcheck.txt`). The backup folder made new and empty. No `tally.exe` ran.
- Tally started from its desktop shortcut (01:41:25Z). The start-up company was shut (the set-up's deviations). `BRIDGE`, space, `PILOT`, space, `LAB` typed in Select Company's filter; Enter only after a capture showed the one row left (`s1-select`).
- Gateway: the book alone; period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026; last entry 15-Sep-26. Step 0 C2 (01:42:45Z): one `tally.exe`, which alone owns 9001; GET /status status=200, bytes=51. s1: the book's GUID, ALTVCHID 68, ALTMSTID 245.
- BK by C5 (capture before Accept 01:44:30Z): Data, Backup & Restore, Backup; Backspace to the path, Specify Path, `C:\lab\2480-mw\backup-1` (the field mended as above); the book the one company; End of List; Accept y; back at the Gateway with no message. One file of 90,051 B, at 07:14 IST, 11 Nov (`evidence/bk.txt`).
- Step 2 (01:45:07Z to 01:45:16Z), step 3 (01:45:49Z to 01:48:16Z), step 4 (01:48:49Z to 01:48:59Z), step 5 (01:49:16Z to 01:49:18Z), step 6 (01:49:39Z to 01:50:27Z) and step 7 (01:51:37Z to 01:56:37Z) as above, each in the notes.
- Step 8 C6 (prep 01:57:01Z: the BK re-hashed equal, its folder holding that file alone, `restore-plan.txt` written, the data folder listed): the book shut by K: Company, Shut, Enter, no prompt; Data, Backup & Restore, Restore; Restore Data Path Tally's data folder; the list opened at the drive's root; Specify Path, the backup-1 folder; the list showed the book, its number and 11-Nov-26 (7:14 am), as the plan; the book selected alone; End of List; Accept ? y (01:58:31Z); the folder-exists question with the book's number (01:58:43Z), r Replace; back at Select Company with no other prompt. The data folder listed again (the side files above). The book loaded alone by the name filter; Gateway captured, byte-equal to s1's, date and period unchanged; then s9 (01:59:50Z to 02:00:02Z), equal to s2 throughout.
- Step 9, C10 (02:00:49Z): the book shut by K: Company, Shut, Enter on its one row, no prompt; Select Company with none loaded; one `tally.exe` owning 9001; GET /status status=200, bytes=51. No request after the shut. Tally then quit by its own Quit, no prompt; then no `tally.exe` ran and nothing listened on 9001 (the first tasklist and netstat call passed `cmd` its switch in a form it did not take, and ran nothing; both were run again). The BK, listed again, one file, its sha256 equal to `evidence/bk.txt`.

## Files
- `answers/`: every answer and its headers file, whole, byte for byte (`kept-local.txt` lists any that stay on the lab PC).
- `logs/`: `run.py`'s log of each label.
- `evidence/`: notes (UTC, one line per step); every `count20.py`, `nm.py`, `st.py` and `wx.py` output, whole, with its exit; each send's UTC time and request check; each background block's output and exit; each id's write run; the Gateway images' list; the per-label wall clock; the book check; `tally.exe`'s file data. Process, port, folder and backup listings and the restore plan stay on the lab PC.
- `scripts/`: the sitting's helpers. `run.py`, `count20.py`, `nm.py`, `st.py`, `rf.py` and `wx.py` are the sheet's files, byte for byte, and are not repeated.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## The outputs, whole

### s1
```
== a-s1-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s2 (count20.py)
```
== a-s2-N-0281.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0282.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0283.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0284.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0285.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0286.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0287.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0288.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0289.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0290.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0291.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s2 (nm.py)
```
== a-s2-B-bills.xml
  BILLFIXED groups: 0
  pl 'BRIDGE Mx Plain': 0 groups
  ga 'BRIDGE Mx Ak, Am & Co': 0 groups
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': 0 groups
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': 0 groups
  qd 'BRIDGE Mx "Dk" Dm & Dn': 0 groups
  pp 'BRIDGE Mx Ek 50% + Em': 0 groups
  hd 'BRIDGE Mx Fk #2, Fm-Fn': 0 groups
  ad 'BRIDGE Mx Gk @ Gm.': 0 groups
  ca "BRIDGE Mx Hk: Hm's (Hn)": 0 groups
  xs 'BRIDGE Mx Jk! Jm *': 0 groups
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': 0 groups
  groups of any other party: 0
== a-s2-S-snap-20260401-20260801.xml
a-s2-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Mx Plain': no row
  ga 'BRIDGE Mx Ak, Am & Co': no row
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': no row
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': no row
  qd 'BRIDGE Mx "Dk" Dm & Dn': no row
  pp 'BRIDGE Mx Ek 50% + Em': no row
  hd 'BRIDGE Mx Fk #2, Fm-Fn': no row
  ad 'BRIDGE Mx Gk @ Gm.': no row
  ca "BRIDGE Mx Hk: Hm's (Hn)": no row
  xs 'BRIDGE Mx Jk! Jm *': no row
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': no row
exit 0
```

### s2 st.py, the before option
```
a-s2-S-snap-20260401-20260801.xml: LEDGER rows 18
FOLD: CLEAR
exit 0
```

### s2 st.py --names
```
a-s2-S-snap-20260401-20260801.xml: LEDGER rows 18
  pl 'BRIDGE Mx Plain': no row
  ga 'BRIDGE Mx Ak, Am & Co': no row
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': no row
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': no row
  qd 'BRIDGE Mx "Dk" Dm & Dn': no row
  pp 'BRIDGE Mx Ek 50% + Em': no row
  hd 'BRIDGE Mx Fk #2, Fm-Fn': no row
  ad 'BRIDGE Mx Gk @ Gm.': no row
  ca "BRIDGE Mx Hk: Hm's (Hn)": no row
  xs 'BRIDGE Mx Jk! Jm *': no row
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': no row
NAMES: EQUAL
exit 0
```

### s4plm count20.py (the before-write option)
```
== a-s4plm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4pl st.py --import
```
a-s4pl-LM-pl.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5plm count20.py (the before-write option)
```
== a-s5plm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5pl st.py --import
```
a-s5pl-I-pl.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 45, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4gam count20.py (the before-write option)
```
== a-s4gam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ga st.py --import
```
a-s4ga-LM-ga.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5gam count20.py (the before-write option)
```
== a-s5gam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ga st.py --import
```
a-s5ga-I-ga.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 46, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4ssm count20.py (the before-write option)
```
== a-s4ssm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ss st.py --import
```
a-s4ss-LM-ss.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5ssm count20.py (the before-write option)
```
== a-s5ssm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ss st.py --import
```
a-s5ss-I-ss.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 47, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4msm count20.py (the before-write option)
```
== a-s4msm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ms st.py --import
```
a-s4ms-LM-ms.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5msm count20.py (the before-write option)
```
== a-s5msm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ms st.py --import
```
a-s5ms-I-ms.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 48, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4qdm count20.py (the before-write option)
```
== a-s4qdm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4qd st.py --import
```
a-s4qd-LM-qd.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5qdm count20.py (the before-write option)
```
== a-s5qdm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5qd st.py --import
```
a-s5qd-I-qd.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 49, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4ppm count20.py (the before-write option)
```
== a-s4ppm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4pp st.py --import
```
a-s4pp-LM-pp.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5ppm count20.py (the before-write option)
```
== a-s5ppm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5pp st.py --import
```
a-s5pp-I-pp.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 50, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4hdm count20.py (the before-write option)
```
== a-s4hdm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4hd st.py --import
```
a-s4hd-LM-hd.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5hdm count20.py (the before-write option)
```
== a-s5hdm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5hd st.py --import
```
a-s5hd-I-hd.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 51, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4adm count20.py (the before-write option)
```
== a-s4adm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'75'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ad st.py --import
```
a-s4ad-LM-ad.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5adm count20.py (the before-write option)
```
== a-s5adm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'75'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ad st.py --import
```
a-s5ad-I-ad.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 52, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4cam count20.py (the before-write option)
```
== a-s4cam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ca st.py --import
```
a-s4ca-LM-ca.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5cam count20.py (the before-write option)
```
== a-s5cam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ca st.py --import
```
a-s5ca-I-ca.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 53, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4xsm count20.py (the before-write option)
```
== a-s4xsm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'77'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4xs st.py --import
```
a-s4xs-LM-xs.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5xsm count20.py (the before-write option)
```
== a-s5xsm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'77'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5xs st.py --import
```
a-s5xs-I-xs.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 54, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4alm count20.py (the before-write option)
```
== a-s4alm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'78'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4al st.py --import
```
a-s4al-LM-al.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5alm count20.py (the before-write option)
```
== a-s5alm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'78'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5al st.py --import
```
a-s5al-I-al.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 55, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s6m
```
== a-s6m-M-marks.xml: 3406 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '79' ALTMSTID '256'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s6sa nm.py
```
== a-s6sa-S-snap-20260401-20260801.xml
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 29
  pl 'BRIDGE Mx Plain': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ga 'BRIDGE Mx Ak, Am & Co': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  qd 'BRIDGE Mx "Dk" Dm & Dn': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  pp 'BRIDGE Mx Ek 50% + Em': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  hd 'BRIDGE Mx Fk #2, Fm-Fn': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ad 'BRIDGE Mx Gk @ Gm.': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ca "BRIDGE Mx Hk: Hm's (Hn)": PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  xs 'BRIDGE Mx Jk! Jm *': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
exit 0
```

### s6ba nm.py
```
== a-s6ba-B-bills.xml
  BILLFIXED groups: 11
  pl 'BRIDGE Mx Plain': 1 groups
    BILLREF 'BP/26-27/0281'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ga 'BRIDGE Mx Ak, Am & Co': 1 groups
    BILLREF 'BP/26-27/0282'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': 1 groups
    BILLREF 'BP/26-27/0283'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': 1 groups
    BILLREF 'BP/26-27/0284'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  qd 'BRIDGE Mx "Dk" Dm & Dn': 1 groups
    BILLREF 'BP/26-27/0285'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  pp 'BRIDGE Mx Ek 50% + Em': 1 groups
    BILLREF 'BP/26-27/0286'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  hd 'BRIDGE Mx Fk #2, Fm-Fn': 1 groups
    BILLREF 'BP/26-27/0287'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ad 'BRIDGE Mx Gk @ Gm.': 1 groups
    BILLREF 'BP/26-27/0288'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ca "BRIDGE Mx Hk: Hm's (Hn)": 1 groups
    BILLREF 'BP/26-27/0289'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  xs 'BRIDGE Mx Jk! Jm *': 1 groups
    BILLREF 'BP/26-27/0290'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': 1 groups
    BILLREF 'BP/26-27/0291'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  groups of any other party: 0
exit 0
```

### s6sa/s6sb nm.py --same
```
  a-s6sa-S-snap-20260401-20260801.xml: 39750 bytes, sha256 3f6f6c67ff1730c09dc16c9a0904a28440e5ae69d4b38a40f2bbe69ec972dfff
  a-s6sb-S-snap-20260401-20260801.xml: 39750 bytes, sha256 3f6f6c67ff1730c09dc16c9a0904a28440e5ae69d4b38a40f2bbe69ec972dfff
SAME: EQUAL
exit 0
```

### s6ba/s6bb nm.py --same
```
  a-s6ba-B-bills.xml: 5396 bytes, sha256 a7fe98942b9af1f1e5e955a8d2d62f620d811632b2d27e03d307275e0827709c
  a-s6bb-B-bills.xml: 5396 bytes, sha256 a7fe98942b9af1f1e5e955a8d2d62f620d811632b2d27e03d307275e0827709c
SAME: EQUAL
exit 0
```

### s6 st.py --names
```
a-s6sa-S-snap-20260401-20260801.xml: LEDGER rows 29
  pl 'BRIDGE Mx Plain': names ['BRIDGE Mx Plain']
  ga 'BRIDGE Mx Ak, Am & Co': names ['BRIDGE Mx Ak, Am & Co']
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': names ['BRIDGE Mx Bk & Sons (Bm), Bn']
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': names ['BRIDGE Mx M/s. Ck_Cm, Cn']
  qd 'BRIDGE Mx "Dk" Dm & Dn': names ['BRIDGE Mx "Dk" Dm & Dn']
  pp 'BRIDGE Mx Ek 50% + Em': names ['BRIDGE Mx Ek 50% + Em']
  hd 'BRIDGE Mx Fk #2, Fm-Fn': names ['BRIDGE Mx Fk #2, Fm-Fn']
  ad 'BRIDGE Mx Gk @ Gm.': names ['BRIDGE Mx Gk @ Gm.']
  ca "BRIDGE Mx Hk: Hm's (Hn)": names ["BRIDGE Mx Hk: Hm's (Hn)"]
  xs 'BRIDGE Mx Jk! Jm *': names ['BRIDGE Mx Jk! Jm *']
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': names ['BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s']
NAMES: EQUAL
exit 0
```

### m1 count20.py --same-row against s6m
```
== a-m1-M-marks.xml against a-s6m-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'256'")
  equal
exit 0
```

### k0 count20.py --same-row against m1
```
== a-k0-M-marks.xml against a-m1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'256'")
  equal
exit 0
```

### k1 wx.py (Vm-u-pl.xml)
```
a-k1-Vm-u-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### k2 wx.py --same against k1
```
SAME: EQUAL (11954 B sha256 b8eab7b26d95eccb; 11954 B sha256 b8eab7b26d95eccb)
exit 0
```

### k3 wx.py --same against k1
```
SAME: EQUAL (11954 B sha256 b8eab7b26d95eccb; 11954 B sha256 b8eab7b26d95eccb)
exit 0
```

### k4 wx.py (Vw-u-pl.xml)
```
a-k4-Vw-u-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### k5 wx.py --same against k4
```
SAME: EQUAL (94752 B sha256 f158857ba662a10f; 94752 B sha256 f158857ba662a10f)
exit 0
```

### k6 wx.py --same against k4
```
SAME: EQUAL (94752 B sha256 f158857ba662a10f; 94752 B sha256 f158857ba662a10f)
exit 0
```

### k7 wx.py (Vb-book.xml)
```
a-k7-Vb-book.xml: ANSWER: VOUCHERS 22
  BP/26-27/0001 DATE 20260801 ALTERID 2 entries 4
  BP/26-27/0002 DATE 20260801 ALTERID 11 entries 4
  BP/26-27/0003 DATE 20260801 ALTERID 50 entries 4
  BP/26-27/0004 DATE 20260801 ALTERID 13 entries 5
  BP/26-27/0005 DATE 20260801 ALTERID 54 entries 5
  BP/26-27/0006 DATE 20260801 ALTERID 48 entries 4
  BP/26-27/0007 DATE 20260801 ALTERID 8 entries 5
  BP/26-27/0008 DATE 20260801 ALTERID 9 entries 5
  BP/26-27/0009 DATE 20260801 ALTERID 10 entries 7
  BP/26-27/0011 DATE 20260801 ALTERID 16 entries 4
  BP/26-27/0010 DATE 20260801 ALTERID 17 entries 6
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
  BP/26-27/0282 DATE 20260801 ALTERID 70 entries 6
  BP/26-27/0283 DATE 20260801 ALTERID 71 entries 6
  BP/26-27/0284 DATE 20260801 ALTERID 72 entries 6
  BP/26-27/0285 DATE 20260801 ALTERID 73 entries 6
  BP/26-27/0286 DATE 20260801 ALTERID 74 entries 6
  BP/26-27/0287 DATE 20260801 ALTERID 75 entries 6
  BP/26-27/0288 DATE 20260801 ALTERID 76 entries 6
  BP/26-27/0289 DATE 20260801 ALTERID 77 entries 6
  BP/26-27/0290 DATE 20260801 ALTERID 78 entries 6
  BP/26-27/0291 DATE 20260801 ALTERID 79 entries 6
exit 0
```

### k7 wx.py --match pl movement (k1)
```
MATCH pl movement: EQUAL (1 voucher, BP/26-27/0281 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### k7 wx.py --match pl window (k4)
```
MATCH pl window: EQUAL (1 voucher, BP/26-27/0281 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0281 New Ref -1051.00; 14 own fields equal)
exit 0
```

### k8 count20.py --same-row against k0
```
== a-k8-M-marks.xml against a-k0-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'256'")
  equal
exit 0
```

### k2 wx.py, for the record
```
a-k2-Vm-uv-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### k3 wx.py, for the record
```
a-k3-Vm-v-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### k5 wx.py, for the record
```
a-k5-Vw-uv-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### k6 wx.py, for the record
```
a-k6-Vw-v-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### q1a (pl) wx.py
```
a-q1a-Vm-v-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### q1b wx.py
```
a-q1b-Vm-v-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### q1 wx.py --same
```
SAME: EQUAL (11954 B sha256 b8eab7b26d95eccb; 11954 B sha256 b8eab7b26d95eccb)
exit 0
```

### w1a (pl) wx.py
```
a-w1a-Vw-v-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### w1b wx.py
```
a-w1b-Vw-v-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### w1 wx.py --same
```
SAME: EQUAL (94752 B sha256 f158857ba662a10f; 94752 B sha256 f158857ba662a10f)
exit 0
```

### pl wx.py --match movement (q1a against k7)
```
MATCH pl movement: EQUAL (1 voucher, BP/26-27/0281 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### pl wx.py --match window (w1a against k7)
```
MATCH pl window: EQUAL (1 voucher, BP/26-27/0281 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0281 New Ref -1051.00; 14 own fields equal)
exit 0
```

### q2a (ga) wx.py
```
a-q2a-Vm-v-ga.xml: ANSWER: VOUCHERS 1
  BP/26-27/0282 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### q2b wx.py
```
a-q2b-Vm-v-ga.xml: ANSWER: VOUCHERS 1
  BP/26-27/0282 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### q2 wx.py --same
```
SAME: EQUAL (11994 B sha256 b678a7fef67a9651; 11994 B sha256 b678a7fef67a9651)
exit 0
```

### w2a (ga) wx.py
```
a-w2a-Vw-v-ga.xml: ANSWER: VOUCHERS 1
  BP/26-27/0282 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### w2b wx.py
```
a-w2b-Vw-v-ga.xml: ANSWER: VOUCHERS 1
  BP/26-27/0282 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### w2 wx.py --same
```
SAME: EQUAL (94792 B sha256 20c26f96821bfa50; 94792 B sha256 20c26f96821bfa50)
exit 0
```

### ga wx.py --match movement (q2a against k7)
```
MATCH ga movement: EQUAL (1 voucher, BP/26-27/0282 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### ga wx.py --match window (w2a against k7)
```
MATCH ga window: EQUAL (1 voucher, BP/26-27/0282 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0282 New Ref -1051.00; 14 own fields equal)
exit 0
```

### q3a (ss) wx.py
```
a-q3a-Vm-v-ss.xml: ANSWER: VOUCHERS 1
  BP/26-27/0283 DATE 20260801 ALTERID 71 entries 6
exit 0
```

### q3b wx.py
```
a-q3b-Vm-v-ss.xml: ANSWER: VOUCHERS 1
  BP/26-27/0283 DATE 20260801 ALTERID 71 entries 6
exit 0
```

### q3 wx.py --same
```
SAME: EQUAL (12022 B sha256 7c1ea25bdeb07437; 12022 B sha256 7c1ea25bdeb07437)
exit 0
```

### w3a (ss) wx.py
```
a-w3a-Vw-v-ss.xml: ANSWER: VOUCHERS 1
  BP/26-27/0283 DATE 20260801 ALTERID 71 entries 6
exit 0
```

### w3b wx.py
```
a-w3b-Vw-v-ss.xml: ANSWER: VOUCHERS 1
  BP/26-27/0283 DATE 20260801 ALTERID 71 entries 6
exit 0
```

### w3 wx.py --same
```
SAME: EQUAL (94820 B sha256 6c2d49e8267a4a2f; 94820 B sha256 6c2d49e8267a4a2f)
exit 0
```

### ss wx.py --match movement (q3a against k7)
```
MATCH ss movement: EQUAL (1 voucher, BP/26-27/0283 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### ss wx.py --match window (w3a against k7)
```
MATCH ss window: EQUAL (1 voucher, BP/26-27/0283 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0283 New Ref -1051.00; 14 own fields equal)
exit 0
```

### q4a (ms) wx.py
```
a-q4a-Vm-v-ms.xml: ANSWER: VOUCHERS 1
  BP/26-27/0284 DATE 20260801 ALTERID 72 entries 6
exit 0
```

### q4b wx.py
```
a-q4b-Vm-v-ms.xml: ANSWER: VOUCHERS 1
  BP/26-27/0284 DATE 20260801 ALTERID 72 entries 6
exit 0
```

### q4 wx.py --same
```
SAME: EQUAL (11990 B sha256 fe370ab141c89bac; 11990 B sha256 fe370ab141c89bac)
exit 0
```

### w4a (ms) wx.py
```
a-w4a-Vw-v-ms.xml: ANSWER: VOUCHERS 1
  BP/26-27/0284 DATE 20260801 ALTERID 72 entries 6
exit 0
```

### w4b wx.py
```
a-w4b-Vw-v-ms.xml: ANSWER: VOUCHERS 1
  BP/26-27/0284 DATE 20260801 ALTERID 72 entries 6
exit 0
```

### w4 wx.py --same
```
SAME: EQUAL (94788 B sha256 f0f23981eb074253; 94788 B sha256 f0f23981eb074253)
exit 0
```

### ms wx.py --match movement (q4a against k7)
```
MATCH ms movement: EQUAL (1 voucher, BP/26-27/0284 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### ms wx.py --match window (w4a against k7)
```
MATCH ms window: EQUAL (1 voucher, BP/26-27/0284 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0284 New Ref -1051.00; 14 own fields equal)
exit 0
```

### q5a (qd) wx.py
```
a-q5a-Vm-v-qd.xml: ANSWER: VOUCHERS 1
  BP/26-27/0285 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### q5b wx.py
```
a-q5b-Vm-v-qd.xml: ANSWER: VOUCHERS 1
  BP/26-27/0285 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### q5 wx.py --same
```
SAME: EQUAL (12038 B sha256 a85d084b2729fc8b; 12038 B sha256 a85d084b2729fc8b)
exit 0
```

### w5a (qd) wx.py
```
a-w5a-Vw-v-qd.xml: ANSWER: VOUCHERS 1
  BP/26-27/0285 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### w5b wx.py
```
a-w5b-Vw-v-qd.xml: ANSWER: VOUCHERS 1
  BP/26-27/0285 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### w5 wx.py --same
```
SAME: EQUAL (94836 B sha256 e4c224348d107f2f; 94836 B sha256 e4c224348d107f2f)
exit 0
```

### qd wx.py --match movement (q5a against k7)
```
MATCH qd movement: EQUAL (1 voucher, BP/26-27/0285 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### qd wx.py --match window (w5a against k7)
```
MATCH qd window: EQUAL (1 voucher, BP/26-27/0285 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0285 New Ref -1051.00; 14 own fields equal)
exit 0
```

### q6a (pp) wx.py
```
a-q6a-Vm-v-pp.xml: ANSWER: VOUCHERS 1
  BP/26-27/0286 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### q6b wx.py
```
a-q6b-Vm-v-pp.xml: ANSWER: VOUCHERS 1
  BP/26-27/0286 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### q6 wx.py --same
```
SAME: EQUAL (11978 B sha256 7793b8210b2901c3; 11978 B sha256 7793b8210b2901c3)
exit 0
```

### w6a (pp) wx.py
```
a-w6a-Vw-v-pp.xml: ANSWER: VOUCHERS 1
  BP/26-27/0286 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### w6b wx.py
```
a-w6b-Vw-v-pp.xml: ANSWER: VOUCHERS 1
  BP/26-27/0286 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### w6 wx.py --same
```
SAME: EQUAL (94776 B sha256 25ff5fc742a622f1; 94776 B sha256 25ff5fc742a622f1)
exit 0
```

### pp wx.py --match movement (q6a against k7)
```
MATCH pp movement: EQUAL (1 voucher, BP/26-27/0286 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### pp wx.py --match window (w6a against k7)
```
MATCH pp window: EQUAL (1 voucher, BP/26-27/0286 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0286 New Ref -1051.00; 14 own fields equal)
exit 0
```

### q7a (hd) wx.py
```
a-q7a-Vm-v-hd.xml: ANSWER: VOUCHERS 1
  BP/26-27/0287 DATE 20260801 ALTERID 75 entries 6
exit 0
```

### q7b wx.py
```
a-q7b-Vm-v-hd.xml: ANSWER: VOUCHERS 1
  BP/26-27/0287 DATE 20260801 ALTERID 75 entries 6
exit 0
```

### q7 wx.py --same
```
SAME: EQUAL (11982 B sha256 c5067d431c3e1c72; 11982 B sha256 c5067d431c3e1c72)
exit 0
```

### w7a (hd) wx.py
```
a-w7a-Vw-v-hd.xml: ANSWER: VOUCHERS 1
  BP/26-27/0287 DATE 20260801 ALTERID 75 entries 6
exit 0
```

### w7b wx.py
```
a-w7b-Vw-v-hd.xml: ANSWER: VOUCHERS 1
  BP/26-27/0287 DATE 20260801 ALTERID 75 entries 6
exit 0
```

### w7 wx.py --same
```
SAME: EQUAL (94780 B sha256 f190b8a4b4593b0e; 94780 B sha256 f190b8a4b4593b0e)
exit 0
```

### hd wx.py --match movement (q7a against k7)
```
MATCH hd movement: EQUAL (1 voucher, BP/26-27/0287 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### hd wx.py --match window (w7a against k7)
```
MATCH hd window: EQUAL (1 voucher, BP/26-27/0287 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0287 New Ref -1051.00; 14 own fields equal)
exit 0
```

### q8a (ad) wx.py
```
a-q8a-Vm-v-ad.xml: ANSWER: VOUCHERS 1
  BP/26-27/0288 DATE 20260801 ALTERID 76 entries 6
exit 0
```

### q8b wx.py
```
a-q8b-Vm-v-ad.xml: ANSWER: VOUCHERS 1
  BP/26-27/0288 DATE 20260801 ALTERID 76 entries 6
exit 0
```

### q8 wx.py --same
```
SAME: EQUAL (11966 B sha256 d1a44c0313af45c4; 11966 B sha256 d1a44c0313af45c4)
exit 0
```

### w8a (ad) wx.py
```
a-w8a-Vw-v-ad.xml: ANSWER: VOUCHERS 1
  BP/26-27/0288 DATE 20260801 ALTERID 76 entries 6
exit 0
```

### w8b wx.py
```
a-w8b-Vw-v-ad.xml: ANSWER: VOUCHERS 1
  BP/26-27/0288 DATE 20260801 ALTERID 76 entries 6
exit 0
```

### w8 wx.py --same
```
SAME: EQUAL (94764 B sha256 aaa8f807445890187fdba4f969cf7dc6a9ca1f90c187447c35abcf5fcb6e8ed9; 94764 B sha256 aaa8f807445890187fdba4f969cf7dc6a9ca1f90c187447c35abcf5fcb6e8ed9)
exit 0
```

### ad wx.py --match movement (q8a against k7)
```
MATCH ad movement: EQUAL (1 voucher, BP/26-27/0288 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### ad wx.py --match window (w8a against k7)
```
MATCH ad window: EQUAL (1 voucher, BP/26-27/0288 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0288 New Ref -1051.00; 14 own fields equal)
exit 0
```

### q9a (ca) wx.py
```
a-q9a-Vm-v-ca.xml: ANSWER: VOUCHERS 1
  BP/26-27/0289 DATE 20260801 ALTERID 77 entries 6
exit 0
```

### q9b wx.py
```
a-q9b-Vm-v-ca.xml: ANSWER: VOUCHERS 1
  BP/26-27/0289 DATE 20260801 ALTERID 77 entries 6
exit 0
```

### q9 wx.py --same
```
SAME: EQUAL (12006 B sha256 cb0fc2dd8f3290ff; 12006 B sha256 cb0fc2dd8f3290ff)
exit 0
```

### w9a (ca) wx.py
```
a-w9a-Vw-v-ca.xml: ANSWER: VOUCHERS 1
  BP/26-27/0289 DATE 20260801 ALTERID 77 entries 6
exit 0
```

### w9b wx.py
```
a-w9b-Vw-v-ca.xml: ANSWER: VOUCHERS 1
  BP/26-27/0289 DATE 20260801 ALTERID 77 entries 6
exit 0
```

### w9 wx.py --same
```
SAME: EQUAL (94806 B sha256 e1709841e70aadd4; 94806 B sha256 e1709841e70aadd4)
exit 0
```

### ca wx.py --match movement (q9a against k7)
```
MATCH ca movement: EQUAL (1 voucher, BP/26-27/0289 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### ca wx.py --match window (w9a against k7)
```
MATCH ca window: EQUAL (1 voucher, BP/26-27/0289 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0289 New Ref -1051.00; 14 own fields equal)
exit 0
```

### q10a (xs) wx.py
```
a-q10a-Vm-v-xs.xml: ANSWER: VOUCHERS 1
  BP/26-27/0290 DATE 20260801 ALTERID 78 entries 6
exit 0
```

### q10b wx.py
```
a-q10b-Vm-v-xs.xml: ANSWER: VOUCHERS 1
  BP/26-27/0290 DATE 20260801 ALTERID 78 entries 6
exit 0
```

### q10 wx.py --same
```
SAME: EQUAL (11966 B sha256 ea4019e3547cfac2; 11966 B sha256 ea4019e3547cfac2)
exit 0
```

### w10a (xs) wx.py
```
a-w10a-Vw-v-xs.xml: ANSWER: VOUCHERS 1
  BP/26-27/0290 DATE 20260801 ALTERID 78 entries 6
exit 0
```

### w10b wx.py
```
a-w10b-Vw-v-xs.xml: ANSWER: VOUCHERS 1
  BP/26-27/0290 DATE 20260801 ALTERID 78 entries 6
exit 0
```

### w10 wx.py --same
```
SAME: EQUAL (94766 B sha256 e555279f4bfebbdf; 94766 B sha256 e555279f4bfebbdf)
exit 0
```

### xs wx.py --match movement (q10a against k7)
```
MATCH xs movement: EQUAL (1 voucher, BP/26-27/0290 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### xs wx.py --match window (w10a against k7)
```
MATCH xs window: EQUAL (1 voucher, BP/26-27/0290 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0290 New Ref -1051.00; 14 own fields equal)
exit 0
```

### q11a (al) wx.py
```
a-q11a-Vm-v-al.xml: ANSWER: VOUCHERS 1
  BP/26-27/0291 DATE 20260801 ALTERID 79 entries 6
exit 0
```

### q11b wx.py
```
a-q11b-Vm-v-al.xml: ANSWER: VOUCHERS 1
  BP/26-27/0291 DATE 20260801 ALTERID 79 entries 6
exit 0
```

### q11 wx.py --same
```
SAME: EQUAL (12258 B sha256 129c54dfd3c3a18a; 12258 B sha256 129c54dfd3c3a18a)
exit 0
```

### w11a (al) wx.py
```
a-w11a-Vw-v-al.xml: ANSWER: VOUCHERS 1
  BP/26-27/0291 DATE 20260801 ALTERID 79 entries 6
exit 0
```

### w11b wx.py
```
a-w11b-Vw-v-al.xml: ANSWER: VOUCHERS 1
  BP/26-27/0291 DATE 20260801 ALTERID 79 entries 6
exit 0
```

### w11 wx.py --same
```
SAME: EQUAL (95058 B sha256 65bc7a5de6e7ff72; 95058 B sha256 65bc7a5de6e7ff72)
exit 0
```

### al wx.py --match movement (q11a against k7)
```
MATCH al movement: EQUAL (1 voucher, BP/26-27/0291 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### al wx.py --match window (w11a against k7)
```
MATCH al window: EQUAL (1 voucher, BP/26-27/0291 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0291 New Ref -1051.00; 14 own fields equal)
exit 0
```

### k9 wx.py (Vb-book.xml)
```
a-k9-Vb-book.xml: ANSWER: VOUCHERS 22
  BP/26-27/0001 DATE 20260801 ALTERID 2 entries 4
  BP/26-27/0002 DATE 20260801 ALTERID 11 entries 4
  BP/26-27/0003 DATE 20260801 ALTERID 50 entries 4
  BP/26-27/0004 DATE 20260801 ALTERID 13 entries 5
  BP/26-27/0005 DATE 20260801 ALTERID 54 entries 5
  BP/26-27/0006 DATE 20260801 ALTERID 48 entries 4
  BP/26-27/0007 DATE 20260801 ALTERID 8 entries 5
  BP/26-27/0008 DATE 20260801 ALTERID 9 entries 5
  BP/26-27/0009 DATE 20260801 ALTERID 10 entries 7
  BP/26-27/0011 DATE 20260801 ALTERID 16 entries 4
  BP/26-27/0010 DATE 20260801 ALTERID 17 entries 6
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
  BP/26-27/0282 DATE 20260801 ALTERID 70 entries 6
  BP/26-27/0283 DATE 20260801 ALTERID 71 entries 6
  BP/26-27/0284 DATE 20260801 ALTERID 72 entries 6
  BP/26-27/0285 DATE 20260801 ALTERID 73 entries 6
  BP/26-27/0286 DATE 20260801 ALTERID 74 entries 6
  BP/26-27/0287 DATE 20260801 ALTERID 75 entries 6
  BP/26-27/0288 DATE 20260801 ALTERID 76 entries 6
  BP/26-27/0289 DATE 20260801 ALTERID 77 entries 6
  BP/26-27/0290 DATE 20260801 ALTERID 78 entries 6
  BP/26-27/0291 DATE 20260801 ALTERID 79 entries 6
exit 0
```

### k9 wx.py --same against k7
```
SAME: EQUAL (1821796 B sha256 1b0e20f5545faff5; 1821796 B sha256 1b0e20f5545faff5)
exit 0
```

### k10
```
== a-k10-M-marks.xml: 3406 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '79' ALTMSTID '256'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### k10 count20.py --same-row against k8
```
== a-k10-M-marks.xml against a-k8-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'256'")
  equal
exit 0
```

### s9 (count20.py)
```
== a-s9-N-0281.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0282.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0283.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0284.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0285.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0286.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0287.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0288.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0289.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0290.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0291.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s9 (nm.py)
```
== a-s9-B-bills.xml
  BILLFIXED groups: 0
  pl 'BRIDGE Mx Plain': 0 groups
  ga 'BRIDGE Mx Ak, Am & Co': 0 groups
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': 0 groups
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': 0 groups
  qd 'BRIDGE Mx "Dk" Dm & Dn': 0 groups
  pp 'BRIDGE Mx Ek 50% + Em': 0 groups
  hd 'BRIDGE Mx Fk #2, Fm-Fn': 0 groups
  ad 'BRIDGE Mx Gk @ Gm.': 0 groups
  ca "BRIDGE Mx Hk: Hm's (Hn)": 0 groups
  xs 'BRIDGE Mx Jk! Jm *': 0 groups
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': 0 groups
  groups of any other party: 0
== a-s9-S-snap-20260401-20260801.xml
a-s9-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Mx Plain': no row
  ga 'BRIDGE Mx Ak, Am & Co': no row
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': no row
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': no row
  qd 'BRIDGE Mx "Dk" Dm & Dn': no row
  pp 'BRIDGE Mx Ek 50% + Em': no row
  hd 'BRIDGE Mx Fk #2, Fm-Fn': no row
  ad 'BRIDGE Mx Gk @ Gm.': no row
  ca "BRIDGE Mx Hk: Hm's (Hn)": no row
  xs 'BRIDGE Mx Jk! Jm *': no row
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': no row
exit 0
```

### s9 against s2, byte for byte
```
N-0281.xml EQUAL 3022 B
N-0282.xml EQUAL 3022 B
N-0283.xml EQUAL 3022 B
N-0284.xml EQUAL 3022 B
N-0285.xml EQUAL 3022 B
N-0286.xml EQUAL 3022 B
N-0287.xml EQUAL 3022 B
N-0288.xml EQUAL 3022 B
N-0289.xml EQUAL 3022 B
N-0290.xml EQUAL 3022 B
N-0291.xml EQUAL 3022 B
B-bills.xml EQUAL 46 B
S-snap-20260401-20260801.xml EQUAL 25480 B
M-marks.xml EQUAL 3394 B
```

### s9 count20.py --same-row against s1
```
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```
