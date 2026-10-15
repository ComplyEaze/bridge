# Which ledger a voucher lands on when two names fold equal: both creation orders of a line-break pair, a name that matches neither exactly, a ledger altered on the screen, and aliases that fold equal (#626 slice 2b), on BRIDGE LINE BREAK LAB

Synthetic company `BRIDGE LINE BREAK LAB` only (the company of the 13 Oct 2026 twin run, which ended with a restore to the book as it was found), loaded alone, nobody else using this PC for Tally or this book from step 0 to the last step (35). **This sheet WRITES**: it creates nine ledgers and up to ten vouchers by gateway requests, and alters three ledgers on the screen. A backup comes first and the restore at the end removes every change. Licensed TallyPrime 7.1, Education mode No, single currency.

**Why, and what this is evidence of.** Protocol reference section 9.4e holds one run on this book: an import naming one twin's exact name posted to that twin; a name equal to neither twin, folding to both, landed on the twin WITHOUT the line break (one run, the line-break twin created first); a `Create` in another case altered the ledger instead of making a twin; the ledger screen refused a name differing by case alone and stored a pasted trailing line break without the break; a voucher naming an alias landed on the aliased ledger. That run did not measure, and this sheet does: (M1) the same fold-only name when the plain twin is created FIRST and the line-break twin second, sent twice and with the exact names sent between the two (so the result is not an effect of the order of posting), and once more on a second pair in the other order; (M3) whether the ledger screen keeps a trailing line break when a ledger that has one is altered for another reason (a person who alters such a ledger on the screen could otherwise turn it into a name that collides with a plain twin); (M4) an alias that folds equal to a ledger's name, in both orders (alias added second, ledger created second), and which ledger each spelling then reaches. The planned pure-case twins (two names differing by case alone) cannot be made on this release by either route (section 9.4e), so no step of this sheet tries again. A balance read of the ledgers by GUID (step 5) is a new request; it is read before anything is written.

**What each outcome means (decided before the sitting).** Each posting step ends in `check.py vch`, which prints the ledger the debit entry landed on (its name, GUID and group, matched byte for byte to exactly one ledger row). A fold-only name that lands on the same twin in both creation orders means the choice does not follow creation order on this book; a different twin in the two orders means it does. Either is recorded, never generalised: one run is verified for this encoding, this book and this release. Whatever the outcome, the shipped refusal (`ledger_has_folded_twin`) is not changed by this sheet; the outcome decides only which of the narrowing rules in the issue can be built.

Files: `requests/` (24), `check.py`, `selftest-626b.py` (the known-answer controls of `check.py`, built from the real answers of the 13 Oct run in `selftest-data/`; not run at the sitting), `check-request-ids.py` with `census-controls/`, `.gitattributes`, `SHA256.txt`. Check the files against `SHA256.txt` first (PowerShell: `Get-FileHash -Algorithm SHA256 FILE`, compare the 64 characters), and run `python -I selftest-626b.py` once before the sitting if Python is on the PC (it prints `all controls fired` and exits 0); a failure there is a STOP before anything is sent.

**Encodings.** The 24 requests are UTF-16LE with a byte-order mark and no final line break, sent as they are. A line break inside a ledger name is written in the request as the character references `&#13;&#10;`, never as a raw line break. Each answer is saved raw by curl (`-o`), as received (UTF-16LE without a mark is expected); never open an answer in an editor that rewrites it. `check.py` reads either and prints ASCII only: a line break inside a name prints as `<CR><LF>`, and in an option it is written `<CRLF>`. Screenshots are PNG files.

**Exit codes of `check.py`** (run it as `python -I check.py ...`; `py -3 -I check.py ...` is the same command): 0 = PASS; 1 = FINDING (an EXPECT did not hold); 3 = STOP (a completeness, identity or size rule broke); 2 = the check could not run (never a pass). Go on after a step only on exit 0, unless the step says that exit 1 is a finding to write down.

**General STOP rule.** Stop and tell us before going on if: Tally shows any dialog, message or warning that the step does not name; `check.py` exits 3 or 2; an answer is not a complete envelope, carries a `LINEERROR` that the step does not name, or has header `STATUS` 0; an answer is above its cap; an answer has not finished after five minutes (do nothing at all: do not cancel it, do not send it again, do not send anything else, do not touch Tally; tell us and wait); `GET /status` after a request is not HTTP 200 `TallyPrime Server is Running`; the process picture (step 0) differs at the last step; a screen offers a field this list does not name and requires it. **Nothing is sent while any form, list or message is on Tally's screen except the Gateway.** **Never press Escape on a screen with unsaved entries, and never on a ledger Alteration screen.** Never press Ctrl+Q at the Gateway (it quits Tally with no prompt). Save, or STOP and tell us. Leaving a screen: only when a screenshot shows that screen's own title, after its save, with Ctrl+Q once; any prompt on leaving means press no key and STOP.

**A STOP after step 8 leaves the book changed.** From the first write on, the book differs from the backup. After a STOP there: change nothing more, do not restore, and tell us; the restore is done only when the coordinator writes `RESTORE` in the thread. A STOP before step 8 leaves the book as it was. **A FINDING at a create or a control step is a STOP before the next change** (the answer is the evidence of that one change). A FINDING at a step marked MEASURE is written down and the sheet goes on, because those steps measure an outcome.

**Named prompts, not stops.** `Accept ?` (Yes) at the Backup screen and at the Restore screen, and the Replace prompt of the restore. A master save is made with Ctrl+A and showed no prompt on the 13 Oct run; an `Accept ?` straight after a Ctrl+A is answered Yes only if a screenshot shows it. At the alias Alteration screen (step 22) the red box `Error / Oops! / Name already exists. / Enter a different name.` is NAMED: it is an outcome to record; press nothing further, take a screenshot and tell us (the 13 Oct run left such a box with one click in the Name field and then Ctrl+Q once to the list and once more to the Gateway, each only after being told). Any other prompt: STOP.

**How to send.** One request at a time: `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\FILE" -o ans-NAME.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001`, each answer written with `-o`, never piped, no client time limit (an abandoned request can leave the gateway stuck). If the commands are run through a tool that has its own time limit, run each curl in the background, create an empty file `NAME.pending` before it and delete that file after it ends, and never leave a curl under that time limit. Note curl's exit code and printed line for every request, then `curl.exe -s -S -o status-NAME.txt -w "%{http_code}" http://127.0.0.1:9001/status` (EXPECT 200 and `TallyPrime Server is Running`). Every send, in full, is in the appendix. Before typing a command click into the PowerShell window, and before pressing a Tally key click Tally's title bar and look at the screen.

**Sizes.** Imports answer with 538 bytes (UTF-16LE); the marks answer is 3,402 bytes (cap 6,000); the company list is 4,000 bytes on the earlier book (cap 8,000); the ledger list was 9,616 bytes for five ledgers on 13 Oct and grows by about 1,300 bytes a ledger (cap 60,000); the voucher read was 33,256 bytes with one voucher and grows by about 6,800 bytes a voucher (cap 150,000). A different size is not a stop by itself; above the cap is. The balance read (step 5) is new: no earlier answer exists, cap 60,000.

**Loading and shutting a company.** Use only Tally's Select Company screen: pick the company from its list and press Enter to load it; to shut a loaded company use the shut action on that same screen. If more than one company is loaded, or the screen offers any choice this list does not name, STOP.

## Before anything
0. `tasklist /fi "imagename eq tally.exe"` and `netstat -ano | findstr ":9001"`; note both. EXPECT exactly one `tally.exe` line, and every LISTENING line on port 9001 (two: IPv4 and IPv6) carrying the PID of that one `tally.exe` line; client lines on 9001 (`TIME_WAIT`, `ESTABLISHED`, `CLOSE_WAIT`) are the PC's own and are ignored. Anything else: STOP. If any company other than `BRIDGE LINE BREAK LAB` is loaded, STOP; if `BRIDGE LINE BREAK LAB` is not in the company list, STOP (do not create a company). Record Tally's current date (look only). Load `BRIDGE LINE BREAK LAB` alone and leave Tally at the Gateway with no form open. Check `SHA256.txt` (above).

## Part A: reads on the book as it is (nothing is written)
1. COMPANY LIST.
   Send `r01-company-list.xml` -> `ans-a01.xml` (then the status line).
   `python -I check.py co ans-a01.xml` EXPECT PASS (the only loaded company is `BRIDGE LINE BREAK LAB`, Education mode No).
2. MARKS.
   Send `r02-marks.xml` -> `ans-a02.xml` (then the status line).
   `python -I check.py marks ans-a02.xml` EXPECT PASS. Record `ALTVCHID` and `ALTMSTID` (2 and 212 after the 13 Oct restore; other values: write them down, go on) and the company GUID (224e7f8c-a0aa-45ce-889a-ad4b7fdbe852 on 13 Oct; a different GUID: STOP).
3. LEDGER LIST.
   Send `r03-ledgers.xml` -> `ans-a03.xml` (then the status line).
   `python -I check.py ledgers ans-a03.xml --marks ans-a02.xml --before ans-a03.xml --absent "Lab Order Probe" --absent "Lab Order Probe<CRLF>" --absent "Lab Order Other" --absent "Lab Order Other<CRLF>" --absent "Lab Screen Alter" --absent "Lab Screen Alter<CRLF>" --absent "Lab Alias Holder" --absent "Lab Alias Name" --absent "Lab Alias Holder Two" --absent "LAB ALIAS TWO" --absent "Lab Alias Two"` EXPECT PASS: every row carries this book's GUID and none of the names this sheet creates is in the book (the list also shows `Cash`, which the vouchers use; STOP if `Cash` is not in it: read the printed count and the list in `ans-a03.xml`). A FINDING here is a STOP before the backup: the names are taken.
4. VOUCHERS OF AUGUST 2026.
   Send `r04-vouchers.xml` -> `ans-a04.xml` (then the status line).
   Record the count the check prints with the next step's command. EXPECT a complete answer (the 13 Oct book held one voucher here after its restore: a Journal of 3 Aug). New vouchers of this sheet are told apart from these by GUID.
5. BALANCES (a new request, read-only).
   Send `r05-balances.xml` -> `ans-a05.xml` (then the status line).
   `python -I check.py bal ans-a05.xml --ledgers ans-a03.xml` EXPECT PASS (at least one row matched to the ledger list by GUID). If Tally shows a dialog or the answer carries an error, that is a STOP before the backup: nothing has been changed; tell us. Record the opening and closing balance printed per ledger.

## The backup
6. BACKUP.
   Create a NEW EMPTY local folder on this PC's own disk (for example `C:\bridge-lab\626b\backup-pre`; not a cloud, network or synced folder). On Tally's Backup screen the "Backup Data Path" may open on cloud storage: change it to `Specify Path` and key the local folder; any cloud path, or a folder that is not new and empty, is a STOP. Choose `BRIDGE LINE BREAK LAB` only, take a screenshot of the screen before accepting, and accept (`Accept ?` Yes). An overwrite prompt is a STOP. EXPECT exactly one file in the folder, its name containing the company number shown on the Backup screen; STOP if the file is missing, 0 bytes, or if there is more than one file. Record its full path, its size, and its SHA-256 (`Get-FileHash -Algorithm SHA256`). Nothing below starts before this is recorded.
7. MARKS AFTER THE BACKUP.
   Send `r02-marks.xml` -> `ans-a02a.xml` (then the status line).
   `python -I check.py same ans-a02.xml ans-a02a.xml` EXPECT PASS: reading after the backup moved nothing. Exit 1 is a STOP: nothing is changed yet, but the backup no longer matches; tell us before the next step.

## Part B: M1, the plain ledger created first, the line-break ledger second (writes from step 8)
8. CREATE the plain ledger `Lab Order Probe` (M1, order B: plain first).
   Send `c01-order-b-plain.xml` -> `ans-08.xml` (then the status line).
   `python -I check.py imp ans-08.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP (the book is changed).
   Send `r03-ledgers.xml` -> `ans-08l.xml` (then the status line).
   `python -I check.py ledgers ans-08l.xml --marks ans-a02.xml --before ans-a03.xml --add "Lab Order Probe|Indirect Incomes"` EXPECT PASS: exactly this one ledger is new, under Indirect Incomes, and every other row is as it was. Record its GUID.
9. CREATE the twin `Lab Order Probe` + CR LF (M1, order B: the line-break ledger second).
   Send `c02-order-b-crlf.xml` -> `ans-09.xml` (then the status line).
   `python -I check.py imp ans-09.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP (the book is changed).
   Send `r03-ledgers.xml` -> `ans-09l.xml` (then the status line).
   `python -I check.py ledgers ans-09l.xml --marks ans-a02.xml --before ans-08l.xml --add "Lab Order Probe<CRLF>|Indirect Expenses"` EXPECT PASS: exactly this one ledger is new, under Indirect Expenses, and every other row is as it was. Record its GUID.
   Section 9.4e measured `CREATED` 1 for this order. `ALTERED` 1 or any other counter makes `imp` exit 1: that is a STOP, and its answer is the finding.
10. POST A (fold-only name, first time): Payment 11 Aug, Dr `LAB ORDER PROBE` (capitals, no line break), 701.00 against `Cash`. M1.
   Send `p01-fold-only-b-first.xml` -> `ans-10.xml` (then the status line).
   `python -I check.py imp ans-10.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP.
   Send `r04-vouchers.xml` -> `ans-10v.xml` (then the status line).
   `python -I check.py vch ans-10v.xml --ledgers ans-09l.xml --before ans-a04.xml --amount 701.00 --allowed "Lab Order Probe" --allowed "Lab Order Probe<CRLF>"`
   MEASURE. EXPECT PASS: the voucher landed on one of the two ledgers named. **Write down the printed `LANDED on ledger` line (name, GUID, group) and the whole `VOUCHER` and `entry` lines.** Exit 1 (it landed on neither, or on more than one row) is written down; go on. If the line says the debit entry matches 0 ledger rows, Tally may have made a ledger of that name: send `r03-ledgers.xml` -> `ans-10x.xml` and tell us before the next change.
11. POST B (control): Payment 12 Aug, Dr `Lab Order Probe` (the exact plain name), 702.00.
   Send `p02-exact-b-plain.xml` -> `ans-11.xml` (then the status line).
   `python -I check.py imp ans-11.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP.
   Send `r04-vouchers.xml` -> `ans-11v.xml` (then the status line).
   `python -I check.py vch ans-11v.xml --ledgers ans-09l.xml --before ans-10v.xml --amount 702.00 --exact "Lab Order Probe"`
   CONTROL. EXPECT PASS: it landed on the ledger carrying exactly the name sent (section 9.4e). Exit 1 is a STOP: do not post anything more; tell us.
12. POST C (control): Payment 13 Aug, Dr `Lab Order Probe` + CR LF (written `&#13;&#10;` in the request), 703.00.
   Send `p03-exact-b-crlf.xml` -> `ans-12.xml` (then the status line).
   `python -I check.py imp ans-12.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP.
   Send `r04-vouchers.xml` -> `ans-12v.xml` (then the status line).
   `python -I check.py vch ans-12v.xml --ledgers ans-09l.xml --before ans-11v.xml --amount 703.00 --exact "Lab Order Probe<CRLF>"`
   CONTROL. EXPECT PASS: it landed on the ledger carrying exactly the name sent (section 9.4e). Exit 1 is a STOP: do not post anything more; tell us.
13. POST D (the fold-only name again, after the exact names were posted): Payment 14 Aug, Dr `LAB ORDER PROBE`, 704.00. M1.
   Send `p04-fold-only-b-again.xml` -> `ans-13.xml` (then the status line).
   `python -I check.py imp ans-13.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP.
   Send `r04-vouchers.xml` -> `ans-13v.xml` (then the status line).
   `python -I check.py vch ans-13v.xml --ledgers ans-09l.xml --before ans-12v.xml --amount 704.00 --allowed "Lab Order Probe" --allowed "Lab Order Probe<CRLF>"`
   MEASURE. EXPECT PASS: the voucher landed on one of the two ledgers named. **Write down the printed `LANDED on ledger` line (name, GUID, group) and the whole `VOUCHER` and `entry` lines.** Exit 1 (it landed on neither, or on more than one row) is written down; go on. If the line says the debit entry matches 0 ledger rows, Tally may have made a ledger of that name: send `r03-ledgers.xml` -> `ans-13x.xml` and tell us before the next change.
   Compare the landing of POST A and POST D (same name, same pair). Write down whether they are the same ledger.

## Part B: M1, the line-break ledger created first, the plain ledger second
14. CREATE `Lab Order Other` + CR LF (M1, order A: line break first).
   Send `c03-order-a-crlf.xml` -> `ans-14.xml` (then the status line).
   `python -I check.py imp ans-14.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP (the book is changed).
   Send `r03-ledgers.xml` -> `ans-14l.xml` (then the status line).
   `python -I check.py ledgers ans-14l.xml --marks ans-a02.xml --before ans-09l.xml --add "Lab Order Other<CRLF>|Indirect Expenses"` EXPECT PASS: exactly this one ledger is new, under Indirect Expenses, and every other row is as it was. Record its GUID.
15. CREATE the plain ledger `Lab Order Other` (M1, order A: plain second).
   Send `c04-order-a-plain.xml` -> `ans-15.xml` (then the status line).
   `python -I check.py imp ans-15.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP (the book is changed).
   Send `r03-ledgers.xml` -> `ans-15l.xml` (then the status line).
   `python -I check.py ledgers ans-15l.xml --marks ans-a02.xml --before ans-14l.xml --add "Lab Order Other|Indirect Incomes"` EXPECT PASS: exactly this one ledger is new, under Indirect Incomes, and every other row is as it was. Record its GUID.
   Section 9.4e measured `CREATED` 1 for this order. Any other counter is a STOP.
16. POST E (fold-only name): Payment 15 Aug, Dr `LAB ORDER OTHER`, 705.00. M1, order A on a second pair.
   Send `p05-fold-only-a.xml` -> `ans-16.xml` (then the status line).
   `python -I check.py imp ans-16.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP.
   Send `r04-vouchers.xml` -> `ans-16v.xml` (then the status line).
   `python -I check.py vch ans-16v.xml --ledgers ans-15l.xml --before ans-13v.xml --amount 705.00 --allowed "Lab Order Other" --allowed "Lab Order Other<CRLF>"`
   MEASURE. EXPECT PASS: the voucher landed on one of the two ledgers named. **Write down the printed `LANDED on ledger` line (name, GUID, group) and the whole `VOUCHER` and `entry` lines.** Exit 1 (it landed on neither, or on more than one row) is written down; go on. If the line says the debit entry matches 0 ledger rows, Tally may have made a ledger of that name: send `r03-ledgers.xml` -> `ans-16x.xml` and tell us before the next change.
17. MARKS (checkpoint).
   Send `r02-marks.xml` -> `ans-17m.xml` (then the status line).
   `python -I check.py marks ans-17m.xml --prev ans-a02.xml` EXPECT PASS. Record the movement of both marks (`ALTVCHID` moved by the number of vouchers posted so far, `ALTMSTID` by the ledgers created).

## Part B: M3, a line-break ledger altered on the screen
18. CREATE `Lab Screen Alter` + CR LF (a single ledger, no plain twin).
   Send `c05-screen-alter-crlf.xml` -> `ans-18.xml` (then the status line).
   `python -I check.py imp ans-18.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP (the book is changed).
   Send `r03-ledgers.xml` -> `ans-18l.xml` (then the status line).
   `python -I check.py ledgers ans-18l.xml --marks ans-a02.xml --before ans-15l.xml --add "Lab Screen Alter<CRLF>|Indirect Expenses"` EXPECT PASS: exactly this one ledger is new, under Indirect Expenses, and every other row is as it was. Record its GUID.
19. ALTER on the screen: change only the group of `Lab Screen Alter` + CR LF.
   LIVE, on the screen. (a) Look only, first. From the Gateway: Alter, Ledger, and in the list of ledgers choose the entry `Lab Screen Alter` (the list holds one ledger of that name: its name ends in a line break and may show as the name with an empty second line). Take a screenshot of the `Ledger Alteration` screen. **The Name field must read `Lab Screen Alter`, the group `Indirect Expenses`; otherwise press nothing and post it, and wait.**
   (b) Key nothing in the Name field. Move down to the group field only with Enter (one Enter at a time, a screenshot after each), and key `Indirect Incomes` in the group field (`Under`), Enter. If the screen shows a field the list above does not name and requires it, STOP. Then wait for the coordinator to write the save, and save with Ctrl+A once; take a screenshot. If the screen returns to the list of ledgers, press Ctrl+Q once only after a screenshot of the list's own title; at the Gateway press nothing.
   (c) Read: send `r03-ledgers.xml` -> `ans-19l.xml` (then the status line).
   `python -I check.py names ans-19l.xml --marks ans-a02.xml --before ans-18l.xml` and then `python -I check.py ledgers ans-19l.xml --marks ans-a02.xml --before ans-18l.xml --moved "Lab Screen Alter<CRLF>|Indirect Incomes"`. MEASURE. EXPECT: the first prints, for the ledger's GUID, its NAME and NAME elements before and after; **write down both lines and whether the stored name still ends in <CR><LF>**. The second passes if the name kept its break and the group is now Indirect Incomes (exit 0); exit 1 is written down (the name lost its break, or something else moved) and the sheet goes on.
   Whatever the stored name is now (with or without the break) is the name the book holds from here on; it is not used again.

## Part B: M4, an alias that folds equal to a ledger's name
20. CREATE the holder `Lab Alias Holder`.
   Send `c06-alias-holder.xml` -> `ans-20.xml` (then the status line).
   `python -I check.py imp ans-20.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP (the book is changed).
   Send `r03-ledgers.xml` -> `ans-20l.xml` (then the status line).
   `python -I check.py ledgers ans-20l.xml --marks ans-a02.xml --before ans-19l.xml --add "Lab Alias Holder|Indirect Expenses"` EXPECT PASS: exactly this one ledger is new, under Indirect Expenses, and every other row is as it was. Record its GUID.
21. CREATE the ledger `Lab Alias Name`.
   Send `c07-alias-named.xml` -> `ans-21.xml` (then the status line).
   `python -I check.py imp ans-21.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP (the book is changed).
   Send `r03-ledgers.xml` -> `ans-21l.xml` (then the status line).
   `python -I check.py ledgers ans-21l.xml --marks ans-a02.xml --before ans-20l.xml --add "Lab Alias Name|Indirect Incomes"` EXPECT PASS: exactly this one ledger is new, under Indirect Incomes, and every other row is as it was. Record its GUID.
22. ALTER on the screen: add the alias `LAB ALIAS NAME` to `Lab Alias Holder`.
   LIVE, on the screen. (a) Look only, first. Alter, Ledger, choose `Lab Alias Holder`; take a screenshot of the `Ledger Alteration` screen. **The Name field must read `Lab Alias Holder`; otherwise press nothing and post it, and wait.** The screen shows the Name field and, below it, a line for an alias (the 13 Oct run added its alias on this screen). If there is no alias line, STOP and tell us.
   (b) Wait for the coordinator to write each key. Move to the alias line with Enter, key `LAB ALIAS NAME` (capitals: it folds equal to the ledger `Lab Alias Name` and is not equal to it), Enter, a screenshot. Then Ctrl+A once, a screenshot.
   (c) MEASURE. Two outcomes are named. ACCEPTED: the screen saved and returned to the list or the Gateway. REFUSED: the red box `Name already exists.` appeared: press nothing further, screenshot it, post it, and wait for the coordinator (the form is unsaved; the book is as before this step).
   (d) Read: send `r03-ledgers.xml` -> `ans-22l.xml` (then the status line).
   `python -I check.py names ans-22l.xml --marks ans-a02.xml --before ans-21l.xml` MEASURE. EXPECT PASS. ACCEPTED: it prints the holder's GUID with its NAME elements now (a second NAME element `LAB ALIAS NAME`); write the line down. REFUSED: it prints nothing changed; write that down.
   **If the alias was REFUSED, skip the next three POST steps (alias order 1) and go on to the CREATE of `Lab Alias Holder Two`.**
23. POST F (alias spelling, capitals): Payment 18 Aug, Dr `LAB ALIAS NAME`, 706.00. M4. Only if the alias was accepted.
   Send `p06-alias-caps-exact-alias.xml` -> `ans-23.xml` (then the status line).
   `python -I check.py imp ans-23.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP.
   Send `r04-vouchers.xml` -> `ans-23v.xml` (then the status line).
   `python -I check.py vch ans-23v.xml --ledgers ans-22l.xml --before ans-16v.xml --amount 706.00 --allowed "Lab Alias Holder" --allowed "Lab Alias Name"`
   MEASURE. EXPECT PASS: the voucher landed on one of the two ledgers named. **Write down the printed `LANDED on ledger` line (name, GUID, group) and the whole `VOUCHER` and `entry` lines.** Exit 1 (it landed on neither, or on more than one row) is written down; go on. If the line says the debit entry matches 0 ledger rows, Tally may have made a ledger of that name: send `r03-ledgers.xml` -> `ans-23x.xml` and tell us before the next change.
24. POST G (the ledger's own spelling): Payment 19 Aug, Dr `Lab Alias Name`, 707.00. M4. Only if the alias was accepted.
   Send `p07-alias-exact-ledger.xml` -> `ans-24.xml` (then the status line).
   `python -I check.py imp ans-24.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP.
   Send `r04-vouchers.xml` -> `ans-24v.xml` (then the status line).
   `python -I check.py vch ans-24v.xml --ledgers ans-22l.xml --before ans-23v.xml --amount 707.00 --allowed "Lab Alias Holder" --allowed "Lab Alias Name"`
   MEASURE. EXPECT PASS: the voucher landed on one of the two ledgers named. **Write down the printed `LANDED on ledger` line (name, GUID, group) and the whole `VOUCHER` and `entry` lines.** Exit 1 (it landed on neither, or on more than one row) is written down; go on. If the line says the debit entry matches 0 ledger rows, Tally may have made a ledger of that name: send `r03-ledgers.xml` -> `ans-24x.xml` and tell us before the next change.
25. POST H (a spelling equal to neither): Payment 20 Aug, Dr `lab alias name`, 708.00. M4. Only if the alias was accepted.
   Send `p08-alias-neither-exact.xml` -> `ans-25.xml` (then the status line).
   `python -I check.py imp ans-25.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP.
   Send `r04-vouchers.xml` -> `ans-25v.xml` (then the status line).
   `python -I check.py vch ans-25v.xml --ledgers ans-22l.xml --before ans-24v.xml --amount 708.00 --allowed "Lab Alias Holder" --allowed "Lab Alias Name"`
   MEASURE. EXPECT PASS: the voucher landed on one of the two ledgers named. **Write down the printed `LANDED on ledger` line (name, GUID, group) and the whole `VOUCHER` and `entry` lines.** Exit 1 (it landed on neither, or on more than one row) is written down; go on. If the line says the debit entry matches 0 ledger rows, Tally may have made a ledger of that name: send `r03-ledgers.xml` -> `ans-25x.xml` and tell us before the next change.
26. CREATE the holder `Lab Alias Holder Two`.
   Send `c08-alias-holder-two.xml` -> `ans-26.xml` (then the status line).
   `python -I check.py imp ans-26.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP (the book is changed).
   Send `r03-ledgers.xml` -> `ans-26l.xml` (then the status line).
   `python -I check.py ledgers ans-26l.xml --marks ans-a02.xml --before ans-22l.xml --add "Lab Alias Holder Two|Indirect Expenses"` EXPECT PASS: exactly this one ledger is new, under Indirect Expenses, and every other row is as it was. Record its GUID.
27. ALTER on the screen: add the alias `Lab Alias Two` to `Lab Alias Holder Two`.
   LIVE, on the screen. Same route and rules as the alias step above: Alter, Ledger, choose `Lab Alias Holder Two`, screenshot (the Name field must read `Lab Alias Holder Two`), then, one key at a time as the coordinator writes it, the alias `Lab Alias Two` (this spelling is the one the next CREATE will fold equal to, in capitals), Enter, screenshot, Ctrl+A once, screenshot. No ledger named `Lab Alias Two` exists, so a refusal here is not expected; if the red box appears, it is a STOP (screenshot, tell us).
   Read: send `r03-ledgers.xml` -> `ans-27l.xml` (then the status line). `python -I check.py names ans-27l.xml --marks ans-a02.xml --before ans-26l.xml` EXPECT PASS and the holder's second NAME element `Lab Alias Two`.
28. CREATE a ledger named `LAB ALIAS TWO` (capitals, under `Indirect Incomes`) while the alias `Lab Alias Two` exists. M4, the other order.
   Send `c09-caps-of-alias-two.xml` -> `ans-28.xml` (then the status line).
   `python -I check.py imp ans-28.xml --expect record` MEASURE. EXPECT PASS: the answer is recorded and classified. Section 9.4d: a Create named exactly like an alias answered `ERRORS` 1 with `Duplicate Entry!`; this name differs by case, so CREATED, ALTERED or ERRORS are all possible. **Write down the printed counters and any LINEERROR text exactly.** A LINEERROR here is NAMED (it is the measurement), not a stop; `imp --expect record` exits 3 on an answer that is not a complete RESPONSE.
   Send `r03-ledgers.xml` -> `ans-28l.xml` (then the status line).
   `python -I check.py names ans-28l.xml --marks ans-a02.xml --before ans-27l.xml` EXPECT PASS: it prints what changed against the earlier list: CREATED reads `NEW GUID ... NAME 'LAB ALIAS TWO'`; ALTERED reads a `CHANGED GUID` line for an existing ledger (write down which, with its NAME elements before and after); ERRORS reads nothing.
   **If the answer was CREATED, do the two POST steps below. If it was ALTERED or ERRORS, skip them and go on to the closing reads.**
29. POST I (alias spelling): Payment 21 Aug, Dr `Lab Alias Two`, 709.00. M4. Only if the CREATE answered CREATED.
   Send `p09-alias-two-exact-alias.xml` -> `ans-29.xml` (then the status line).
   `python -I check.py imp ans-29.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP.
   Send `r04-vouchers.xml` -> `ans-29v.xml` (then the status line).
   `python -I check.py vch ans-29v.xml --ledgers ans-28l.xml --before ans-25v.xml --amount 709.00 --allowed "Lab Alias Holder Two" --allowed "LAB ALIAS TWO"`
   (If POST F to H were skipped because the alias was refused, `--before` in the command above is the last voucher read taken, `ans-16v.xml`, not the one printed.)
   MEASURE. EXPECT PASS: the voucher landed on one of the two ledgers named. **Write down the printed `LANDED on ledger` line (name, GUID, group) and the whole `VOUCHER` and `entry` lines.** Exit 1 (it landed on neither, or on more than one row) is written down; go on. If the line says the debit entry matches 0 ledger rows, Tally may have made a ledger of that name: send `r03-ledgers.xml` -> `ans-29x.xml` and tell us before the next change.
30. POST J (the new ledger's spelling): Payment 22 Aug, Dr `LAB ALIAS TWO`, 710.00. M4. Only if the CREATE answered CREATED.
   Send `p10-alias-two-exact-ledger.xml` -> `ans-30.xml` (then the status line).
   `python -I check.py imp ans-30.xml --expect created` EXPECT PASS: CREATED 1, every other counter 0. Any other exit is a STOP.
   Send `r04-vouchers.xml` -> `ans-30v.xml` (then the status line).
   `python -I check.py vch ans-30v.xml --ledgers ans-28l.xml --before ans-29v.xml --amount 710.00 --allowed "Lab Alias Holder Two" --allowed "LAB ALIAS TWO"`
   MEASURE. EXPECT PASS: the voucher landed on one of the two ledgers named. **Write down the printed `LANDED on ledger` line (name, GUID, group) and the whole `VOUCHER` and `entry` lines.** Exit 1 (it landed on neither, or on more than one row) is written down; go on. If the line says the debit entry matches 0 ledger rows, Tally may have made a ledger of that name: send `r03-ledgers.xml` -> `ans-30x.xml` and tell us before the next change.

## Closing reads, then the restore and the proof of it
31. CLOSING READS.
   Send `r02-marks.xml` -> `ans-31m.xml`; `python -I check.py marks ans-31m.xml --prev ans-a02.xml` EXPECT PASS (record the movement).
   Send `r05-balances.xml` -> `ans-31b.xml`; `python -I check.py bal ans-31b.xml --ledgers ans-28l.xml` EXPECT PASS. **Write down the closing balance of each twin and of the holders**: this is the read by GUID that section 9.4e did not have. The vouchers posted debit a ledger, so a ledger that received a Payment shows its amount; compare with the `LANDED` lines.
32. RESTORE. Only after the steps above are read back (or when the coordinator writes `RESTORE` after a STOP).
   Shut `BRIDGE LINE BREAK LAB`, then restore from the backup of the backup step with Tally's own restore: on the Restore screen confirm that the source is the path recorded there, that its size and SHA-256 are the values recorded there (check both on the file again), that the company named is `BRIDGE LINE BREAK LAB` with the company number recorded, and that the destination is the existing `BRIDGE LINE BREAK LAB` data folder. If the file's size or SHA-256 is not the recorded value, STOP before accepting anything. Take a screenshot before accepting. The prompts expected are `Accept ?` (Yes) and then the one saying the company data folder already exists and that Replace will overwrite it: choose Replace. If the Replace prompt does NOT appear, STOP at once: the restore may have gone to a new place and the changed book may still be in place; do not retry. STOP before choosing anything if the screen names another company or number, offers a new folder or a new number, or offers to repair, rewrite or migrate anything. After the restore the company must be back in the company list and must open; if it does not, STOP (do not retry). Load it alone as at step 0.
33. PROOF OF THE RESTORE.
   Send `r01-company-list.xml` -> `ans-r01.xml` (then the status line). `python -I check.py co ans-r01.xml` EXPECT PASS.
   Send `r02-marks.xml` -> `ans-r02.xml` (then the status line). `python -I check.py same ans-a02.xml ans-r02.xml` EXPECT PASS (identical, or equal row by row).
   Send `r03-ledgers.xml` -> `ans-r03.xml` (then the status line). `python -I check.py same ans-a03.xml ans-r03.xml` EXPECT PASS.
   Send `r04-vouchers.xml` -> `ans-r04.xml` (then the status line). `python -I check.py same ans-a04.xml ans-r04.xml` EXPECT PASS.
   Send `r05-balances.xml` -> `ans-r05.xml` (then the status line). `python -I check.py same ans-a05.xml ans-r05.xml` EXPECT PASS.
   A difference under the COLLECTION (`same` exit 1) at any of these is a STOP: do nothing further, leave the lab as it is, send nothing, and report.
34. (look only) Tally's current date and the process picture.
   EXPECT the step-0 value. Leave the company loaded as step 0 left it. Repeat the two commands of step 0 (`tasklist /fi "imagename eq tally.exe"` and `netstat -ano | findstr ":9001"`). EXPECT the step-0 picture (compare the `tally.exe` line apart from its memory column, and the LISTENING lines).
35. HAND BACK the folder.
   Every `ans-*`, every `status-*`, every `check.py` line, the curl lines, the screenshots, the backup's path, size and SHA-256, and a table of file, bytes and sha256. Mark each finding verified, partial or unverified, and give these first: (a0) the whole thread of the sitting (every instruction and every post, with times); (a) the `LANDED` lines of POST A, D, E (fold-only) with the creation order of each pair; the two control lines of POST B and C; (b) the stored name after the screen alteration (with or without the line break) and the alias outcomes (accepted or refused, with the exact box text; the landing of POST F to J); (c) the answer to the CREATE of `LAB ALIAS TWO`; (d) the balances by GUID and whether they agree with the `LANDED` lines; (e) the `ALTMSTID` and `ALTVCHID` movement at the checkpoints; (f) whether the restore removed every change (the proof step). The book is synthetic, so the answers are handed back whole; nothing is posted publicly by the PC.

## Where each request comes from
| request | source | the one change |
|---|---|---|
| r01 | `w01-company-list.xml` of the ledger input tax credit write sheet (the seeding sheet's `s01`) | none, byte for byte (it names no company) |
| r02, r03, r04 | `marks`, `ledgers` and `vch-aug` of the 13 Oct 2026 twin run, which Tally answered | none, byte for byte (986, 2,238 and 1,786 bytes) |
| r05 | new: a `Collection` export of `Ledger` with `FETCH` `GUID,OPENINGBALANCE,CLOSINGBALANCE`, in the form of `r02` | not sent before; the census below shows its header `ID` names the collection it defines |
| c01 to c09 | `led1`, `led2` and `led3` of the 13 Oct run (`All Masters`, one `LEDGER` with `ACTION="Create"`, the name as the `NAME` attribute and the `NAME` element, one `PARENT`) | the company, the name and the group; a line break is written `&#13;&#10;` (section 9.4e) |
| p01 to p10 | `imp1` of the 13 Oct run (a Payment, `ACTION="Create"`, one debit entry and one credit to `Cash`, deemed-positive Yes and No) | the `REMOTEID` (a new UUID each), the date (11 to 22 Aug 2026), the ledger name (written as the step says) and the amount (701.00 to 710.00, unique) |

**Census of the request ids.** `python -I check-request-ids.py requests` -> `census: ok 5, skipped 19, FAIL 0`, exit 0; `python -I check-request-ids.py --controls` -> `controls: 3 files, 0 did not fire`, exit 0. The five collection requests (r01 to r05) each name the collection they define; the 19 skipped are imports.

## Appendix: every send, in full (each answer with `-o`, then the status line)
One at a time, in this order; the step is the step of this sheet that asks for it.

1. (step 1) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r01-company-list.xml" -o ans-a01.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-a01.txt -w "%{http_code}" http://127.0.0.1:9001/status`
2. (step 2) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r02-marks.xml" -o ans-a02.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-a02.txt -w "%{http_code}" http://127.0.0.1:9001/status`
3. (step 3) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r03-ledgers.xml" -o ans-a03.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-a03.txt -w "%{http_code}" http://127.0.0.1:9001/status`
4. (step 4) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r04-vouchers.xml" -o ans-a04.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-a04.txt -w "%{http_code}" http://127.0.0.1:9001/status`
5. (step 5) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r05-balances.xml" -o ans-a05.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-a05.txt -w "%{http_code}" http://127.0.0.1:9001/status`
6. (step 7) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r02-marks.xml" -o ans-a02a.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-a02a.txt -w "%{http_code}" http://127.0.0.1:9001/status`
7. (step 8) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\c01-order-b-plain.xml" -o ans-08.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-08.txt -w "%{http_code}" http://127.0.0.1:9001/status`
8. (step 8) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r03-ledgers.xml" -o ans-08l.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-08l.txt -w "%{http_code}" http://127.0.0.1:9001/status`
9. (step 9) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\c02-order-b-crlf.xml" -o ans-09.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-09.txt -w "%{http_code}" http://127.0.0.1:9001/status`
10. (step 9) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r03-ledgers.xml" -o ans-09l.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-09l.txt -w "%{http_code}" http://127.0.0.1:9001/status`
11. (step 10) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\p01-fold-only-b-first.xml" -o ans-10.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-10.txt -w "%{http_code}" http://127.0.0.1:9001/status`
12. (step 10) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r04-vouchers.xml" -o ans-10v.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-10v.txt -w "%{http_code}" http://127.0.0.1:9001/status`
13. (step 11) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\p02-exact-b-plain.xml" -o ans-11.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-11.txt -w "%{http_code}" http://127.0.0.1:9001/status`
14. (step 11) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r04-vouchers.xml" -o ans-11v.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-11v.txt -w "%{http_code}" http://127.0.0.1:9001/status`
15. (step 12) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\p03-exact-b-crlf.xml" -o ans-12.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-12.txt -w "%{http_code}" http://127.0.0.1:9001/status`
16. (step 12) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r04-vouchers.xml" -o ans-12v.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-12v.txt -w "%{http_code}" http://127.0.0.1:9001/status`
17. (step 13) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\p04-fold-only-b-again.xml" -o ans-13.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-13.txt -w "%{http_code}" http://127.0.0.1:9001/status`
18. (step 13) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r04-vouchers.xml" -o ans-13v.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-13v.txt -w "%{http_code}" http://127.0.0.1:9001/status`
19. (step 14) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\c03-order-a-crlf.xml" -o ans-14.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-14.txt -w "%{http_code}" http://127.0.0.1:9001/status`
20. (step 14) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r03-ledgers.xml" -o ans-14l.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-14l.txt -w "%{http_code}" http://127.0.0.1:9001/status`
21. (step 15) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\c04-order-a-plain.xml" -o ans-15.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-15.txt -w "%{http_code}" http://127.0.0.1:9001/status`
22. (step 15) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r03-ledgers.xml" -o ans-15l.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-15l.txt -w "%{http_code}" http://127.0.0.1:9001/status`
23. (step 16) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\p05-fold-only-a.xml" -o ans-16.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-16.txt -w "%{http_code}" http://127.0.0.1:9001/status`
24. (step 16) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r04-vouchers.xml" -o ans-16v.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-16v.txt -w "%{http_code}" http://127.0.0.1:9001/status`
25. (step 17) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r02-marks.xml" -o ans-17m.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-17m.txt -w "%{http_code}" http://127.0.0.1:9001/status`
26. (step 18) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\c05-screen-alter-crlf.xml" -o ans-18.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-18.txt -w "%{http_code}" http://127.0.0.1:9001/status`
27. (step 18) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r03-ledgers.xml" -o ans-18l.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-18l.txt -w "%{http_code}" http://127.0.0.1:9001/status`
28. (step 19) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r03-ledgers.xml" -o ans-19l.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-19l.txt -w "%{http_code}" http://127.0.0.1:9001/status`
29. (step 20) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\c06-alias-holder.xml" -o ans-20.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-20.txt -w "%{http_code}" http://127.0.0.1:9001/status`
30. (step 20) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r03-ledgers.xml" -o ans-20l.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-20l.txt -w "%{http_code}" http://127.0.0.1:9001/status`
31. (step 21) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\c07-alias-named.xml" -o ans-21.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-21.txt -w "%{http_code}" http://127.0.0.1:9001/status`
32. (step 21) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r03-ledgers.xml" -o ans-21l.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-21l.txt -w "%{http_code}" http://127.0.0.1:9001/status`
33. (step 22) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r03-ledgers.xml" -o ans-22l.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-22l.txt -w "%{http_code}" http://127.0.0.1:9001/status`
34. (step 23) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\p06-alias-caps-exact-alias.xml" -o ans-23.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-23.txt -w "%{http_code}" http://127.0.0.1:9001/status`
35. (step 23) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r04-vouchers.xml" -o ans-23v.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-23v.txt -w "%{http_code}" http://127.0.0.1:9001/status`
36. (step 24) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\p07-alias-exact-ledger.xml" -o ans-24.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-24.txt -w "%{http_code}" http://127.0.0.1:9001/status`
37. (step 24) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r04-vouchers.xml" -o ans-24v.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-24v.txt -w "%{http_code}" http://127.0.0.1:9001/status`
38. (step 25) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\p08-alias-neither-exact.xml" -o ans-25.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-25.txt -w "%{http_code}" http://127.0.0.1:9001/status`
39. (step 25) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r04-vouchers.xml" -o ans-25v.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-25v.txt -w "%{http_code}" http://127.0.0.1:9001/status`
40. (step 26) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\c08-alias-holder-two.xml" -o ans-26.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-26.txt -w "%{http_code}" http://127.0.0.1:9001/status`
41. (step 26) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r03-ledgers.xml" -o ans-26l.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-26l.txt -w "%{http_code}" http://127.0.0.1:9001/status`
42. (step 27) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r03-ledgers.xml" -o ans-27l.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-27l.txt -w "%{http_code}" http://127.0.0.1:9001/status`
43. (step 28) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\c09-caps-of-alias-two.xml" -o ans-28.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-28.txt -w "%{http_code}" http://127.0.0.1:9001/status`
44. (step 28) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r03-ledgers.xml" -o ans-28l.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-28l.txt -w "%{http_code}" http://127.0.0.1:9001/status`
45. (step 29) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\p09-alias-two-exact-alias.xml" -o ans-29.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-29.txt -w "%{http_code}" http://127.0.0.1:9001/status`
46. (step 29) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r04-vouchers.xml" -o ans-29v.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-29v.txt -w "%{http_code}" http://127.0.0.1:9001/status`
47. (step 30) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\p10-alias-two-exact-ledger.xml" -o ans-30.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-30.txt -w "%{http_code}" http://127.0.0.1:9001/status`
48. (step 30) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r04-vouchers.xml" -o ans-30v.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-30v.txt -w "%{http_code}" http://127.0.0.1:9001/status`
49. (step 31) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r02-marks.xml" -o ans-31m.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-31m.txt -w "%{http_code}" http://127.0.0.1:9001/status`
50. (step 31) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r05-balances.xml" -o ans-31b.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-31b.txt -w "%{http_code}" http://127.0.0.1:9001/status`
51. (step 33) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r01-company-list.xml" -o ans-r01.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-r01.txt -w "%{http_code}" http://127.0.0.1:9001/status`
52. (step 33) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r02-marks.xml" -o ans-r02.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-r02.txt -w "%{http_code}" http://127.0.0.1:9001/status`
53. (step 33) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r03-ledgers.xml" -o ans-r03.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-r03.txt -w "%{http_code}" http://127.0.0.1:9001/status`
54. (step 33) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r04-vouchers.xml" -o ans-r04.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-r04.txt -w "%{http_code}" http://127.0.0.1:9001/status`
55. (step 33) `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\r05-balances.xml" -o ans-r05.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001` then `curl.exe -s -S -o status-r05.txt -w "%{http_code}" http://127.0.0.1:9001/status`
