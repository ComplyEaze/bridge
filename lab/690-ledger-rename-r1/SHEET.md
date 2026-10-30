# R1: does an `Alter` whose GUID and name point at different ledgers refuse, and what does a refused `Alter` leave? (lab sheet, not for merge)

Part of #690. Two open questions remain from sheet R0 (`lab/690-ledger-rename-r0-answers`), one sitting on TallyPrime
7.2 Silver:

- **R1a, the GUID guard.** R0 H1 showed that an `Alter` carrying a ledger's GUID, addressed to a name no ledger holds,
  is refused, and the GUID's ledger is untouched. The name decides. R0 did not test an `Alter` carrying one ledger's
  GUID, addressed to another ledger's existing name.
  - If Tally refuses that mismatch in-band, a `GUID` element acts as a guard: the `Alter` lands only where the name
    and the GUID agree.
  - If it renames the named ledger, the GUID is ignored.
  - If it renames the GUID's ledger, the GUID addresses when a name exists.
  - A control comes first: the same `Alter` with the GUID and the name of the same ledger. It shows whether the `GUID`
    element by itself leaves a rename working.
- **R1b, what a refused `Alter` leaves.** After R0's restart the company showed "(e) Data exceptions exist" on the
  Gateway, and the BK0 restore removed it. The only refused answer before that restart was R0 C1, an `Alter` to a
  missing name (`EXCEPTIONS 1`). R0 also renamed ledgers before the restart, so the cause is not isolated. R1b first
  restarts Tally with no write (a control), then sends only that `Alter` on the restored book, restarts Tally again,
  and records the Gateway and Tally's exception report.

Raw capture with `curl`. Nothing is built on it until its answers are read. Synthetic BRIDGE company only: no company
other than the one named below. About 55 minutes.

**LAB-COMMON v3 applies in full** (the copy on `lab/1713-settle-order`); where this sheet and it differ, the stricter
holds. This sheet's own rules below add to it. Its one widening is B0 and B3: each quits Tally from its own Gateway
and answers the quit prompt, which C7 admits as a prompt the step names. Nothing in this sheet ends a process (C2).

## The company

BRIDGE LEDGERBOX CI LAB, licensed TallyPrime 7.2 Silver, loaded alone, as in R0. The request files name it in
`SVCURRENTCOMPANY`; another company needs the requests made again, never edited by hand.

## Rules

- **Backup BK0 first**, by C5, before step S1, into a NEW empty folder. Record its name, size and SHA-256. No BK0, or a
  BK0 with no recorded hash = do not start.
- **Restores.** BK0 is restored by Tally's own Restore (C6), on the owner's standing word for synthetic books, at Z1
  and Z3. After a STOP, nothing is restored without the run coordinator's word (C8); part B never runs after a STOP in
  part A. The answers and the log are kept in every case.
- One request at a time, each sent by C3's line exactly, with its headers file, its printed status and size, and its
  `CONTENT-LENGTH` recorded. No client timeout; a request with no answer in 10 minutes is never sent again: STOP and
  report (C3). Every read's bound is C3's 2,000,000 bytes. Never retry, cancel or re-send a write.
- Before every write, Tally shows its Gateway with no dialog open, and the MARKS answer just before it holds exactly
  one COMPANY row (C3). **After every write (N1, M1, A1, B1), screenshot the whole Tally window before its read**; a
  dialog there = C7, and nothing further is sent.
- Record for every request: the HTTP status, `curl`'s exit code, the time taken, the answer file's size and SHA-256.
  An exit code other than 0, a status other than 200, a size other than `CONTENT-LENGTH`, or an empty answer = the
  request failed. That is a STOP; it is never read as an answer with no counters.
- **Read each answer as LAB-COMMON C4 says** (v3, the copy on `lab/1713-settle-order`), with an XML parser, never by
  eye or by grep.
  - Decode it as UTF-16LE, with or without a byte-order mark.
  - Before parsing, drop ONLY the characters XML forbids, raw or as references: 0 to 8, 11, 12 and 14 to 31. KEEP 9,
    10 and 13. The group read carries raw 3 and the reference 4 in `PARENTSTRUCTURE`.
  - Record which characters each answer carried. An answer that still does not parse = STOP.
  - Rows are the children of `ENVELOPE/BODY/DATA/COLLECTION`. `CMPINFO`'s bare elements of the same names are not
    rows.
  - Keep every answer file byte for byte: the dropping is done on a copy, for reading only.
- Before sending, check each request file's SHA-256 against `SHA256.txt` (Get-FileHash); a mismatch = STOP. The two
  files with a placeholder, `r1m-guid-matched-template.utf16le.xml` and `r1a-guid-mismatch-template.utf16le.xml`, are
  filled only by `fill_r1.py` (step A0), never by hand; each filled file is checked against the size and SHA-256 the
  script printed for it.
- **"Named"** means a ledger's own name: the first `NAME` of the first `NAME.LIST` under its first
  `LANGUAGENAME.LIST`. The row's `NAME` attribute is recorded apart and is never what "named" means.
- **A "read"** is three requests in this order: `marks.utf16le.xml`, `ledgers.utf16le.xml`, `groups.utf16le.xml`.
  From them record:
  - the company's `ALTMSTID` and `ALTVCHID`;
  - for every ledger: the row's `NAME` attribute and `RESERVEDNAME`, every `LANGUAGENAME.LIST` with its `LANGUAGEID`
    and every `NAME` in it in order, and its `GUID`, `ALTERID`, `PARENT` and `ISBILLWISEON`;
  - for every group: `NAME`, `GUID` and `ALTERID`;
  - **against the previous read:** the ledger and group GUIDs added and those gone, compared by GUID, never by name,
    and every `ALTERID` that moved.
- **A write's answer:** record every counter Tally gives (`CREATED`, `ALTERED`, `DELETED`, `COMBINED`, `IGNORED`,
  `ERRORS`, `CANCELLED`, `EXCEPTIONS`, `LASTMID`, `LASTVCHID`). A counter Tally leaves out is recorded as "absent",
  never as 0. Record every `LINEERROR` text exactly.
- **Tally's screens in R1b** are read only. Follow LAB-COMMON C7 throughout:
  - never press Escape on a master or voucher screen;
  - never open an entry of a report;
  - never choose an item whose name holds Resolve, Rewrite, Repair, Migrate, Delete, Alter or Accept;
  - never press a key that would accept, resolve, alter or delete anything.

  A screen that offers to repair, rewrite, migrate or resolve = press no key, screenshot, STOP.
- STOP and report, with nothing improvised, on:
  - a second company loaded, or Education mode;
  - a failed request or an answer that does not parse, as above;
  - **after any `Alter`, a ledger or group GUID added that the previous read did not hold** (an `Alter` made a
    master);
  - a STOP line below;
  - **any dialog in Tally** other than the quit prompt B0 and B3 name. Take a screenshot first. The run coordinator
    decides how the dialog is cleared. The row is recorded as "left a dialog", and once the dialog is cleared, one
    read is taken (C8). A restore follows only on the run coordinator's word.
- Record: TallyPrime's release and edition, the company's GUID, BK0's hash, and a timed log of every request with its
  file name.

## Steps

S0. Step 0 by C2, a to d, with this company loaded alone (C1): the whole-window screenshot first, `tasklist` (record
    the `tally.exe` PID), `netstat`, GET /status, the Gateway screenshot and a MARKS read. Record TallyPrime's release
    and edition (Silver 7.2 only; Gold, another release or Education = STOP) and the date the Gateway shows as Tally's
    current date. Record whether the Gateway or the company's line shows "(e)" or "Data exceptions exist". If it does,
    part B is not run: its question needs a company without that mark. Then BK0 by C5.
S1. A read. EXPECT one company row (this company); record its `ALTMSTID` as M0. EXPECT none of these names on any
    ledger (as its own name or any name in its lists, under any case or spacing) or group: `R1 Guard X`,
    `R1 Guard X2`, `R1 Guard Y`, `R1 Guard Y2`, `R0 Missing Ledger`, `R0 Missing Renamed`. One present = STOP.

### Part A: R1a, the GUID guard

N1. **Request** `setup-create-2.utf16le.xml`: two party ledgers under Sundry Creditors, not bill-wise, no opening, no
    GSTIN, in this order: `R1 Guard X`, `R1 Guard Y`. It is R0's setup request with two ledgers. EXPECT `CREATED 2`
    and every other counter 0; another answer = STOP.
N2. A read. EXPECT two ledger GUIDs added, named as above, and `ALTMSTID` = M0 + 2. Record each one's GUID; the rows
    below call them X and Y. Keep N2's ledgers answer file: A0 reads X's GUID from it.
A0. Check `fill_r1.py` against `SHA256.txt` first; a mismatch = M1 to A3 are skipped and recorded so; go to Z1. Then
    fill both templates from N2's ledgers answer, with the Python the lab used for R0 (`py -I` or `python -I`):
    - `py -I fill_r1.py requests/r1m-guid-matched-template.utf16le.xml <N2 ledgers answer> <evidence folder>\r1m-filled.utf16le.xml`
    - `py -I fill_r1.py requests/r1a-guid-mismatch-template.utf16le.xml <N2 ledgers answer> <evidence folder>\r1a-filled.utf16le.xml`

    The script takes X's GUID from the answer, never typed: exactly one ledger whose own name is `R1 Guard X`. It
    refuses a template whose hash differs, a GUID not in a ledger GUID's form, an output file that exists, or a result
    that is not the template with that one value in place. Record the GUID, size and SHA-256 each run prints. EXPECT
    the GUID equal to X's GUID recorded at N2. A refusal or a different GUID = M1 to A3 are skipped and recorded so;
    go to Z1.
M1. **Request** the filled `r1m` file, the control: an `Alter` addressed to `R1 Guard X`, carrying X's own `GUID` and
    only the name `R1 Guard X2`.
M2. A read. Record for X and Y: GUID, own name, row `NAME` attribute, lists, `ALTERID`, `PARENT`, `ISBILLWISEON`.
    Record `ALTMSTID`, and every other `ALTERID` that moved.
    - R1a runs only if M1 counted `ALTERED 1` and every other counter 0, and M2 shows X (by GUID) named `R1 Guard X2`
      and Y unchanged.
    - Otherwise A1 to A3 are skipped and recorded so, and the result goes to the run coordinator before Z1 (C11): a
      `GUID` element that stops a matched rename makes R1a's answer unreadable as a guard.
A1. **Request** the filled `r1a` file: an `Alter` addressed to `R1 Guard Y`, Y's existing name, carrying **X's**
    `GUID` and only the name `R1 Guard Y2`.
A2. A read, recorded as in M2 for X and Y both, against M2 (where X is already `R1 Guard X2`). The question is which
    of these it is:
    - (i) refused in-band (a counter or `LINEERROR` reporting it) with X and Y unchanged against M2 and no mark moved;
    - (ii) Y renamed to `R1 Guard Y2`, X unchanged against M2;
    - (iii) X renamed to `R1 Guard Y2`, Y unchanged against M2;
    - (iv) anything else, including Y's GUID changed, X's or Y's GUID gone, or two ledgers sharing a GUID.

    Record the counters, `LINEERROR` and both ledgers exactly; no EXPECT, since that is what this row measures. Any
    master added = STOP (the rule above).
A3. Screenshot the Gateway (no key pressed). Record whether it shows "(e)" or "Data exceptions exist".
Z1. Copy the answers so far into the evidence folder. Close the company in Tally, restore BK0, and load the restored
    company alone.
Z2. As C6 says, before any request: a screenshot of the whole Tally window and one of the Gateway showing this company
    as the one company. Record whether the Gateway or the company's line shows "(e)" or "Data exceptions exist"; if
    it does, part B is not run. Then a read. EXPECT the ledger and group GUIDs and `ALTERID`s of S1 exactly, and none
    of the `R1` or `R0` names listed at S1. A difference = STOP and report; part B is not run. If part B is not run
    for any reason here or at S0, go to Z5: the company is already restored.

### Part B: R1b, what a refused `Alter` leaves

Runs only if S0 and Z2 both showed the Gateway without "(e)" and "Data exceptions exist", and part A ended with no
STOP.

**The restart used in B0 and B3.**
1. Screenshot the Gateway with no dialog open. At the Gateway, press Escape once (not Ctrl+Q, which C7 forbids
   there). EXPECT Tally to ask whether to quit: screenshot the prompt, then press `Y` to answer Yes. In R0, Ctrl+Q
   closed Tally with no prompt; record what this Tally shows.
2. If Escape instead shuts the company and leaves Tally at Select Company, or Tally does not close, or shows any other
   prompt: press no further key, screenshot, STOP for the run coordinator's word. Nothing ends a process (C2).
3. When `tasklist` shows no `tally.exe`, open Tally once by its usual shortcut. As C9 says, shut any company that loads
   by itself (record its name and the menu path), then load this company alone.
4. Step 0 by C2, a to d (a new PID is recorded). Any repair, migrate or recovery prompt, at any point = press no key,
   screenshot, STOP.

B0. The control: the restart above with no write since Z2. Then record whether the Gateway or the company's line
    shows "(e)" or "Data exceptions exist", with the exact text. If it does, B1 to B6 are not run: a restart alone
    marks the company, and that is the answer. Go to Z3.
B1. **Request** `r0c-alter-missing.utf16le.xml`: R0 C1's file, byte for byte: one ledger `Alter` addressed to
    `R0 Missing Ledger`, which S1 and Z2 showed no ledger holds, carrying only `R0 Missing Renamed`.
    **STOP if the answer counts `CREATED`, `ALTERED` or `COMBINED` above 0** (R0 C1 answered `EXCEPTIONS 1`).
B2. A read. Compare with Z2 and with B0's MARKS read (step 0 after the restart). Any master added, any `ALTERID`
    moved, or `ALTMSTID` moved = STOP; the restore follows on the run coordinator's word.
    Screenshot the Gateway (no key pressed); record whether it shows "(e)" or "Data exceptions exist".
B3. The restart above.
B4. Screenshot the Gateway and the company's line in Select Company or the company list (no key pressed beyond
    reaching them). Record whether either shows "(e)" or "Data exceptions exist", and the exact text.
B5. Tally's exception report, read only:
    1. Open Go To (Alt+G), type `Exception`, and screenshot the list Tally offers before choosing anything.
    2. Open only the items that list shows as reports, whose name says exception and holds none of the words the rules
       above forbid, one at a time. Screenshot each and copy every line of its text exactly: its title, each entry and
       each count. If a report has no entries, record that.
    3. Never open an entry within a report. Leave each report only as C7 allows; a screen that is a master or voucher
       screen = press no key, screenshot, STOP.
    4. If the Gateway's "Data exceptions exist" line is itself a menu item, record its exact text and a screenshot,
       and open it only on the run coordinator's word, by the same rules.
    5. Return to the Gateway with no dialog open, and screenshot it.
B6. GET /status, then a read. Compare with B2. Any change = record it exactly (a restart is not expected to change any
    master).
Z3. Copy every answer file, the filled R1 requests that exist, the screenshots' names, sizes and SHA-256s, and the
    log into the evidence folder. Close the company, restore BK0, and load the restored company alone.
Z4. As C6 says, before any request: a whole-window screenshot and one of the Gateway. Record whether "(e)" or "Data
    exceptions exist" shows. Then a read. EXPECT the ledger and group GUIDs and `ALTERID`s of S1 exactly. A difference
    = STOP and report.
Z5. As LAB-COMMON C10: the company shut, Select Company with none loaded, one `tally.exe` owning port 9001,
    `/status` 200.

## What R1 decides

| Question | Read from | What depends on it |
| --- | --- | --- |
| Does a `GUID` element in an `Alter` leave a matched rename working? | M1, M2 | whether R1a's answer can be read as a guard at all |
| Does Tally refuse an `Alter` whose `GUID` names one ledger and whose `NAME` attribute names another, existing one? | A1, A2 | #690 slice 2's window: a guard (i) lets a reviewed candidate shape close it in-band; (ii) needs the owner's acceptance in ADR 0004's words; (iii) stops the slice for a redesign |
| Does a restart alone, with no write, mark the company with data exceptions? | B0 | whether B4's answer can be put on B1 |
| Does a refused `Alter` to a missing name leave the company marked with data exceptions, before or after a restart, and what does Tally's exception report hold? | B2, B4, B5 | what a refused rename's answer may say, and whether such a refusal leaves the person something to clear in Tally |

Not covered: Gold, other releases, other master types, and an `Alter` carrying any field other than a name list and a
`GUID`. Confidence of every answer: one sitting, one synthetic book: partial.
