# R0: a ledger renamed by an import `Alter`, raw capture on TallyPrime 7.2 (lab sheet, not for merge)

Part of #690. ComplyEaze Bridge is to rename one ledger through its approved path, by an `Alter` that names the
ledger and carries only its new name. That shape was measured once on TallyPrime 7.1 Gold, on one book
(`docs/tally/TALLY_PROTOCOL_REFERENCE_WRITE_RESPONSES_AND_MASTERS.md`, §9.4, "A rename by `Alter` changes only the
name"). Nothing about it is measured on 7.2. The cases that decide whether it is safe to build were never measured
for an `Alter`:

- an `Alter` that names a ledger the book does not hold;
- a ledger with an alias;
- a change of letter case only;
- a new name another ledger already holds;
- whether a renamed ledger's row `NAME` attribute keeps the old name until Tally restarts.

This sheet captures them with `curl`. Nothing is built on it until its answers are read.

Synthetic BRIDGE company only: never a client book, never REGISTER LAB. About 75 minutes.

## The company

BRIDGE LEDGERBOX CI LAB, licensed TallyPrime 7.2 Silver, loaded alone, as in L0 and L0b. The request files name it in
`SVCURRENTCOMPANY`; another company needs the requests made again, never edited by hand.

## Rules

- **Backup BK0 first.** Before step S1, back the company up into a NEW empty folder as BK0; record its name, size and
  SHA-256. No BK0, or a BK0 with no recorded hash = do not start. At step S0 also record the PID of the one
  `tally.exe` (step G1 may need it).
- **Restore at the end, and after every STOP that follows a write.** Step Z0 then Z1: the answers and log are kept,
  the company is closed in Tally, and BK0 is restored by Tally's own Restore, as in L0, on the owner's standing word
  for synthetic books. Z2 then checks it.
- One request at a time; wait for each answer before the next. Never retry, cancel or re-send a write. Before every
  request, Tally shows its Gateway with no dialog open.
- Commands below are for PowerShell. Send each file's bytes exactly as they are,
  `Content-Type: text/xml; charset=utf-16`, to `http://127.0.0.1:9001`, with `curl.exe` (never PowerShell's `curl`
  alias). Write the answer to a file with `--output`, never piped, with `--max-time 120` and
  `--write-out '%{http_code}'`.
  - Record for every request: the HTTP status, `curl`'s exit code, the time taken, the answer file's size and SHA-256.
  - An exit code other than 0, a status other than 200, or an empty answer = the request failed. That is a STOP; it
    is never read as an answer with no counters.
  - A request that reaches `--max-time`: take a screenshot of Tally, then STOP.
- Read each answer with an XML parser, never by eye or by grep, as LAB-COMMON C4 (v3, the copy on
  `lab/1713-settle-order`) says. Decode it as UTF-16LE, with or without a byte-order mark. Before parsing, drop ONLY
  the characters XML forbids, whether raw or as references: 0 to 8, 11, 12 and 14 to 31. KEEP 9, 10 and 13, since a
  name may end in a line break. Record which ones an answer carried and where (the group read's `PARENTSTRUCTURE`
  carries raw 3 and the reference 4, for example). An answer that still does not parse = STOP. Keep every answer
  file byte for byte: the dropping is done on a copy for reading only.
  - Changed 30 Oct 2026, after the first S1 stopped on a raw 3 in the group read: the sheet named only `&#4;`. The
    rule is now C4's. Requests and every STOP rule are unchanged.
- Before sending, check each request file's SHA-256 against `SHA256.txt`; a mismatch = STOP. The one file with a
  placeholder, `r0h-guid-template.utf16le.xml`, is filled only by `fill_r0h.py` (step H0), never by hand.
- **"Named"** in this sheet means a ledger's own name: the first `NAME` of the first `NAME.LIST` under its first
  `LANGUAGENAME.LIST`. The row's `NAME` attribute is recorded apart and is never what "named" means.
- **A "read"** is three requests in this order: `marks.utf16le.xml`, `ledgers.utf16le.xml`, `groups.utf16le.xml`.
  From them record:
  - the company's `ALTMSTID` and `ALTVCHID`;
  - for every ledger: the row's `NAME` attribute and `RESERVEDNAME`, and every `LANGUAGENAME.LIST` separately, with its
    `LANGUAGEID` and every `NAME` in it in order; also `GUID`, `ALTERID`, `PARENT` and `ISBILLWISEON`;
  - for every group: `NAME`, `GUID` and `ALTERID`. The group read carries no alias list, so group aliases are not
    checked.
  - **Against the previous read:** the ledger and group GUIDs added, and those gone. Compare by GUID, never by name.
- **A write's answer:** record every counter Tally gives (`CREATED`, `ALTERED`, `DELETED`, `COMBINED`, `IGNORED`,
  `ERRORS`, `CANCELLED`, `EXCEPTIONS`, `LASTMID`, `LASTVCHID`). A counter Tally leaves out is recorded as "absent",
  never as 0. Record every `LINEERROR` text too.
- STOP and report, with nothing improvised, on:
  - a second company loaded, or Education mode;
  - a failed request or an answer that does not parse, as above;
  - **after any `Alter`, a ledger or group GUID added that the previous read did not hold** (an `Alter` made a
    master);
  - a STOP line below;
  - **any dialog in Tally.** Take a screenshot first. The run coordinator decides how the dialog is cleared (the owner's
    standing word allows a restart). The row is recorded as "left a dialog", and once the dialog is cleared, one read is
    taken. Then Z0 and Z1, and nothing more is sent.
- Record: TallyPrime's release and edition, the company's GUID, BK0's hash, and a timed log of every request with its
  file name.

## Steps

S0. Record TallyPrime's release and edition and the `tally.exe` PID. BK0 as above.
S1. A read. EXPECT one company row (this company); record its `ALTMSTID` as M0. EXPECT none of these names on any
    ledger (as its own name or any name in its lists, under any case or spacing) or group: `R0 Missing Ledger`,
    `R0 Missing Renamed`, `R0 Rename A`, `R0 Rename A2`, `R0 Taken B`, `R0 Alias C`, `R0 Alias C2`,
    `R0 Alias C Other`, `R0 Case D`, `R0 CASE D`, `R0 Plain E`, `R0 Plain E2`, `R0 Not A Ledger`, `R0 Rename H`.
    One present = STOP.

### R0c first: an `Alter` naming a ledger the book does not hold

C1. **Request** `r0c-alter-missing.utf16le.xml`: one ledger `Alter` addressed to `R0 Missing Ledger`, which S1 showed
    no ledger holds, carrying only the name `R0 Missing Renamed`.
    **STOP if the answer counts `CREATED`, `ALTERED` or `COMBINED` above 0.** The read can miss a master Tally leaves
    out of the collection; the counters cover that. Record `LASTMID` as given.
C2. A read. Compare with S1.
    **STOP if any ledger or group is added**, or one is named `R0 Missing Ledger` or `R0 Missing Renamed`: the `Alter`
    made a master.
    STOP also if `ALTMSTID` moved, or any ledger's or group's `ALTERID` moved.
    After any of these STOPs: Z0, Z1 and Z2, then report. The rename slice is stopped and redesigned on that answer.
    Otherwise record the counters and `LINEERROR` exactly: they are the answer this row measures.

### Setup: five ledgers

N1. **Request** `setup-create-5.utf16le.xml`: five party ledgers under Sundry Creditors, not bill-wise, no opening,
    no GSTIN, in this order: `R0 Rename A`, `R0 Taken B`, `R0 Alias C`, `R0 Case D`, `R0 Plain E`. It is in the form
    of L0b's request. EXPECT `CREATED 5` and every other counter 0; another answer = STOP.
N2. A read. EXPECT five ledger GUIDs added, named as above, and `ALTMSTID` = M0 + 5 (L0 and L0b measured one step per
    create). Record each new ledger's GUID; the rows below call them A, B, C, D and E.

### R0a and R0b: a rename, and the rename back

A1. **Request** `r0a-rename.utf16le.xml`: an `Alter` addressed to `R0 Rename A`, carrying only `R0 Rename A2`.
A2. A read. Record for ledger A: its GUID (EXPECT unchanged), its row `NAME` attribute, its lists, its `ALTERID`,
    `PARENT` and `ISBILLWISEON`. Record `ALTMSTID` and `ALTVCHID`, and whether any other ledger's or group's `ALTERID`
    moved. No EXPECT on what changed: that is what this row measures.
B1. **Request** `r0b-rename-back.utf16le.xml`: an `Alter` addressed to `R0 Rename A2`, carrying only `R0 Rename A`.
    It runs only if the latest read showed ledger A named `R0 Rename A2`; otherwise it is skipped and recorded so.
B2. A read, recorded as in A2.

### R0p: a plain rename left in place

P1. **Request** `r0p-plain-rename.utf16le.xml`: an `Alter` addressed to `R0 Plain E`, carrying only `R0 Plain E2`.
P2. A read, recorded as in A2 for ledger E. E is not written again before step G2.

### R0e: a ledger with an alias

E1. **Request** `r0e-add-alias.utf16le.xml`: an `Alter` addressed to `R0 Alias C`, carrying one names list:
    `R0 Alias C`, `R0 Alias C Other`.
E2. A read. Record ledger C's lists. E3 runs only if its first list is exactly `R0 Alias C`, `R0 Alias C Other`;
    otherwise it is skipped and recorded so.
E3. **Request** `r0e-rename-aliased.utf16le.xml`: an `Alter` addressed to `R0 Alias C`, carrying only `R0 Alias C2`.
E4. A read, recorded as in A2 for ledger C. The question is whether `R0 Alias C Other` is still in any of its lists.

### R0f: letter case only

F1. **Request** `r0f-case-only.utf16le.xml`: an `Alter` addressed to `R0 Case D`, carrying only `R0 CASE D`.
F2. A read, recorded as in A2 for ledger D, with the row `NAME` attribute set beside its own name.

### R0g: after a restart

G0. The baseline for this row is, per ledger, its last read before G1: A at B2 (or A2 if B1 was skipped), E at P2,
    C at E4 (or E2), D at F2.
G1. Restart TallyPrime:
    1. Quit it from its own screen (Ctrl+Q, then Yes).
    2. If the lab cannot work the screen, end the one `tally.exe` recorded at S0 with `Stop-Process -Id <PID> -Force`,
       and no other means. Record which of the two was used.
    3. Start TallyPrime and load only this company from its data folder.
    4. Wait for its Gateway with no dialog open.

    **A repair, migrate or recovery prompt = STOP** (screenshot; then Z0 to Z2). There is no backup after the writes
    above, and none is needed: Z1 restores BK0 in every case.
G2. A read. For ledgers A, C, D and E, where each exists, record the row's `NAME` attribute, `RESERVEDNAME` and lists
    against its G0 baseline.

### R0d: a new name another ledger holds

D1. **Request** `r0d-rename-onto-taken.utf16le.xml`: an `Alter` addressed to `R0 Rename A`, carrying only
    `R0 Taken B`, the name ledger B holds. It runs only if the latest read showed ledger A named `R0 Rename A` and
    ledger B named `R0 Taken B`; otherwise it is skipped and recorded so.
D2. A read, recorded as in A2 for ledgers A and B both: each one's GUID, lists, `ALTERID` and parent, and whether
    either is gone.

### R0h, candidate only: an `Alter` carrying a GUID

Runs last, and only if all three hold:
- C1 counted no create, alteration or combination;
- C2 showed no master added and a counter or `LINEERROR` reporting the request refused;
- no row above left a dialog.

Otherwise it is skipped and recorded so.

H0. Check `fill_r0h.py` against `SHA256.txt` first; a mismatch = this row is skipped. Then fill the template with it,
    with the Python the lab used for L0's reads (`py -I` or `python -I`):
    `py -I fill_r0h.py requests/r0h-guid-template.utf16le.xml <GUID> <evidence folder>\r0h-filled.utf16le.xml`.
    `<GUID>` is ledger D's GUID copied from N2's read, the read where D was still named `R0 Case D`. The script refuses
    a template whose hash differs, a GUID not in a ledger GUID's form, or a result that is not the template with that
    one value in place. Record the GUID and the size and SHA-256 it prints. A refusal = this row is skipped.
H1. **Request** the filled file: an `Alter` addressed to `R0 Not A Ledger`, which no ledger holds, carrying ledger D's
    `GUID` and only the name `R0 Rename H`.
H2. A read. STOP if any master is added (the rule above). Record whether ledger D (by GUID) is now named `R0 Rename H`,
    and its `ALTERID`.

### The end

Z0. Copy every answer file, the filled R0h request if any, and the log into the evidence folder.
Z1. Close the company in Tally, then restore BK0 by Tally's own Restore, and load the restored company alone.
Z2. A read. EXPECT the ledger and group GUIDs and `ALTERID`s of S1 exactly, and none of the `R0` names. A difference =
    STOP and report; nothing more is sent.

## What R0 decides

| Question | Read from | What depends on it |
| --- | --- | --- |
| Does an `Alter` naming a ledger the book does not hold refuse in-band, make nothing and move no mark? | C1, C2 | the whole rename slice: if it makes a ledger, the slice stops |
| Does a rename count `ALTERED 1` and nothing else, keep the GUID, and leave parent and bill-wise unchanged? | A1, A2, B1, B2, P1, P2 | the verdict's counters, identity and fields checks |
| Does `ALTMSTID` step by one, and does the renamed ledger's `ALTERID` become the new mark? | A2, B2, P2 | the verdict's mark step and its first-change-after-the-last-read rule |
| Does `ALTVCHID` stay unmoved? | A2, B2, P2 | whether a rename can disturb a voucher census |
| Does a one-name list drop an existing alias? | E2, E4 | whether a ledger with an alias can ever be renamed this way |
| What does a change of letter case do to the lists and the row `NAME` attribute? | F1, F2 | whether a case-only rename stays refused |
| Does a renamed ledger's row `NAME` attribute keep the old name, and does a restart change it? ComplyEaze Bridge's name index counts that attribute as a name the book may still answer to. | P2 and G2 for E (A, C and D as support) | whether the row attribute keeps the old name; no request is sent at the old name here, so whether the book still answers to it is not tested |
| What does a rename onto a name another ledger holds do: refuse, merge or rename? | D1, D2 | what the approved path's remaining window could do |
| Does an `Alter` carrying a GUID address the ledger by GUID? | H1, H2 | whether that window can be closed (candidate only) |

## How each row reaches Tally, and what ComplyEaze Bridge would do

Every request here is sent by `curl`, so no check in ComplyEaze Bridge stands between it and Tally; that is what lets
the sheet measure what the approved path will refuse before sending. In the slice as designed:

- C1, D1 and H1 are refused before anything is sent: a target that no longer resolves to the bound ledger, or a new
  name another master holds. They could reach Tally only in the window between the approved path's last read and its
  request. These rows measure what that window would do.
- E3 and F1 are refused before anything is saved: a target with an alias, a change of case only. These rows measure
  whether a later slice can lift those refusals.
- A1, B1 and P1 are the shape the slice would send.

## The requests

- `marks.utf16le.xml`, `ledgers.utf16le.xml` and `groups.utf16le.xml` are L0's and L0b's read requests, byte for byte
  (the same company).
- Every other request is one `Import Data` envelope (`REPORTNAME` `All Masters`), in the form of L0b's
  `create-10.utf16le.xml`: the same envelope bytes, UTF-16LE with a byte-order mark.
  - `setup-create-5` holds five ledger `Create` messages, each in the form L0b's request holds.
  - Each `Alter` request holds one message: `<LEDGER NAME="..." ACTION="Alter">` and
    `<LANGUAGENAME.LIST><NAME.LIST TYPE="String">` with the name or names, then `<LANGUAGEID>1033</LANGUAGEID>`.
    That is §9.4's measured shape, with no other element. The exception is the R0h template, which adds one
    `<GUID>` element.
- These are candidate shapes, sent here to the lab book only.
- `fill_r0h.py` fills the R0h template and checks the result. It reads and writes local files only.
