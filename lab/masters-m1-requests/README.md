# Lab run sheet: master name resolution and ledger-create shapes (11 Oct 2026). SYNTHETIC COMPANY ONLY. Not for merge.

**What it decides:**
- **L-NR:** the shape of a ledger alias in Bridge's ledger catalogue response, and whether the V3 catalogue returns ALTERID. These become the M1a fixtures.
- **L-N:** what Tally does with a ledger Create whose name is an alias, a case/space/`-`/`/` variant, or a group's name. This tests M1a's absence folds.
- **L-M1:** whether Tally accepts and stores M1b's exact Create bytes for a party, a Suspense ledger and the six GST duty heads. These become the M1b goldens and the admitted duty-head set.

**Who:**
- The Windows operator keys the company on screen, sends each request file exactly, and saves every response.
- The requester reads the saved files and says GO between phases. One report per phase.

**Grant:** one grant per sitting. Requests are by file name and sha256 (SHA256.txt). Count: 4 reads in phase 2, 8 writes and 26 reads in phase 3, 8 writes and 26 reads in phase 4. Hold the lab lock throughout.

## Rules
- **Company:** only BRIDGE MASTERS LAB is loaded. Close any other company (closing is not deleting). Never touch a client company. Port 9001 only.
- **One request at a time.** Never abandon a request (no client timeout). Nothing else talks to 9001 meanwhile. **A write is never re-sent**, even after an error.
- **Sending a file:** check `Get-FileHash -Algorithm SHA256` against SHA256.txt before sending. Send with:
  `curl.exe -sS --data-binary "@<ID>.utf16le.xml" -H "Content-Type: text/xml; charset=utf-16" -o "evidence\<step>-<ID>.response" -D "evidence\<step>-<ID>.headers" -w "%{http_code} %{size_download}\n" http://localhost:9001/`
  - No `--max-time`. Record curl's exit code and the `-w` line in `evidence\log.txt` with an ISO time.
  - A non-zero exit or a non-200 answer is a **transport failure**: STOP. It is never "refused" and never "0 rows".
- **Counting:** decode each response by its Content-Type charset and parse it as XML (PowerShell `[xml]` or Python). Never count with grep or substring matches.
  - Rows are the LEDGER or GROUP elements under ENVELOPE/BODY/DATA/COLLECTION only. CMPINFO also carries `<LEDGER>` and `<GROUP>` count elements; do not count those.
  - For a write, read the counters by element from the RESPONSE: CREATED, ALTERED, DELETED, COMBINED, IGNORED, ERRORS, CANCELLED, EXCEPTIONS, plus any LINEERROR text.
- **After every write,** screenshot Tally's window.

**STOP** (change nothing, save, screenshot, report) on any of:
- any dialog in Tally's own window;
- a transport failure;
- a second company loaded;
- Education mode;
- no answer within 10 minutes;
- anyone typing in Tally.

A STOP on no answer means: wait, do not kill curl, do not re-send, screenshot Tally and report. Abandoning a request has wedged Tally before.

No case is repeated within a sitting. Restoring B0 and repeating a case is a new sitting that needs a new GO and a new grant.

## Phase 0: pre-check (no Tally request)
Report: TallyPrime version, licence (Silver, not Education), the gateway port as Tally's own settings show it (9001), the companies loaded, and the 21 hashes checked. Phase 2's first R-MARKS is the first proof on the wire. Wait for GO.

## Phase 1: keying on screen (no gateway write)
1. **New company "BRIDGE MASTERS LAB":**
   - State Rajasthan, books from 1 Apr 2026, no inventory.
   - GST enabled: registration Regular, GSTIN `08ZZZZZ0000Z1ZQ`, state Rajasthan.
2. **Ledgers.** All have no opening balance, parent Sundry Debtors, and bill-wise **No**:
   - `MLab Exact Target`, which also carries party GSTIN `08ZZZZZ0000Z3ZO` (registration Regular)
   - `MLab Alias Target`, with aliases `MLab Alias Name` and `MLab Alias Two`
   - `MLab Case Target`
   - `MLab Space Target`
   - `MLab Dash-Target`
   - `MLab Slash/Target`
3. **Group:** `MLab Group Target` under Sundry Debtors, with alias `MLab Group Alias`.
4. **Screenshots** of each alias screen.
5. **Backup:** take a Tally backup B0 of this company (file name, time). A restore is only ever a new sitting (see Rules).

## Phase 2: L-NR (4 reads)
Send in order: `R-MARKS`, `R-V2`, `R-V3`, `R-GRP` (step id `nr`).

Report:
- each file's bytes and LEDGER or GROUP row count;
- for `MLab Alias Target` in R-V2 and R-V3: every NAME under every LANGUAGENAME.LIST/NAME.LIST, in document order;
- whether R-V3's rows carry ALTERID (yes or no, and on how many rows). **If any ledger row lacks ALTERID, STOP at this GO:** phase 3 then needs a different read or an explicit decision to rely on ALTMSTID alone;
- for `MLab Group Target` in R-GRP: its NAME.LIST names, GUID and ALTERID;
- ALTMSTID and ALTVCHID from R-MARKS: take the COMPANY row whose GUID is the lab company's, and report how many COMPANY rows came back (more than one means a second company is loaded: STOP).

Wait for GO.

## Phase 3: L-N (8 cases; each is 1 write then 3 reads)
Order: N-0a, N-0b, N-a, N-b, N-c, N-d, N-f, N-e.

For each case:
- send the write (step id `n-<case>`);
- then `R-MARKS`, `R-V3`, `R-GRP` (step id `n-<case>-after`).

The reads after a case are the "before" for the next one. Every Create uses parent Sundry Creditors and bill-wise **Yes**, so an overwrite of a target (Sundry Debtors, No) is visible.

| Case | Create name | Target it may hit | Expected |
|---|---|---|---|
| N-0a | `MLab Fresh Party` | none | CREATED=1 (control) |
| N-0b | `MLab Exact Target` | the same name | overwrite, ALTERED=1 (control; measured before). Also send `R-PARTY` just before this write and after it (step ids `n-0b-party-before`, `n-0b-party-after`), and report the target's PARTYGSTIN and GST registration rows from both: does a Create without a GSTIN clear it? |
| N-a | `MLab Alias Name` | alias of `MLab Alias Target` | unknown |
| N-b | `MLAB CASE TARGET` | `MLab Case Target` | unknown |
| N-c | `MLab  Space Target` (two spaces) | `MLab Space Target` | unknown |
| N-d | `MLab Dash Target` | `MLab Dash-Target` | unknown |
| N-f | `MLab Slash Target` | `MLab Slash/Target` | unknown |
| N-e | `MLab Group Target` (ledger) | the group of that name | unknown |

N-e is the write most likely to raise a dialog in Tally (a ledger named after a group). Its screenshot is mandatory; the STOP-on-dialog rule applies.

Report per case:
- the counters;
- the ALTMSTID step;
- from R-V3 after: every ledger row whose names include the created name or the target's names, each with GUID, ALTERID, PARENT, ISBILLWISEON and all NAME.LIST names;
- for N-e: the group row from R-GRP, before and after.

If a control does not behave as expected, STOP. Wait for GO after phase 3.

## Phase 4: L-M1 (8 writes; each is 1 write then 3 reads)
Order: M1-PARTY, M1-SUSP, then M1-DUTY-CGST, -IGST, -STATE, -SGSTUTGST, -UT, -CESS.

After each write, send `R-MARKS`, `R-V3` and `R-GRP` (step id `m1-<id>-after`).

Send `R-PARTY` (the party ledger master read; it fetches TAXTYPE, GSTDUTYHEAD and PARTYGSTIN) once before M1-PARTY (step id `m1-before`) and once after the last write (step id `m1-end`). This read-back is mandatory for every duty ledger, because Tally drops an unknown head silently while reporting CREATED. From `m1-end`, report each new ledger's TAXTYPE, GSTDUTYHEAD, PARTYGSTIN and OPENINGBALANCE, each as returned or absent.

Report per write: the counters, the ALTMSTID step, and the new row's GUID, PARENT, ISBILLWISEON and names.

## Phase 5: L-NP (5 writes; each is 1 write then 3 reads). A new sitting after phase 4, with its own GO and grant
It asks whether a ledger Create folds away a removed space or a removed full stop. The run sheet's name fold does neither, so either answer changes what the name check must refuse.

Send it only on the company as phase 4 left it: `MLab Fresh Party` exists (created by N-0a: Sundry Creditors, bill-wise Yes, never renamed), `MLab Exact Target` is on Sundry Creditors with bill-wise Yes (after N-0b), `MLab Alias Target` is on Sundry Debtors with bill-wise No and keeps its two aliases, and no ledger is named like `MLab Probe Pvt Ltd`.

Tally restarted with its application settings reset after phase 3, so Q-0 repeats phase 3's exact-name control first: an exact-name Create must still alter the existing ledger, or nothing after it is comparable with phase 3.

Order: Q-0, Q-r, Q-a, Q-c0, Q-c. After each write, send `R-MARKS`, `R-V3`, `R-GRP` (step id `q-<case>-after`). Before Q-a, send the same three reads once (step id `q-before`).

| Case | Create name | Target it may hit | Expected |
|---|---|---|---|
| Q-0 | `MLab Exact Target` (Sundry Debtors, bill-wise No) | the same name | ALTERED=1 (control) |
| Q-r | `MLab Alias Target` with `REMOTEID="MLAB-Q-R-REMOTEID-0001"`, a REMOTEID no master holds (Sundry Creditors, bill-wise Yes) | the same name | unknown: does a Create carrying an unknown REMOTEID still alter a master found by name, or is it refused? |
| Q-a | `MLab FreshParty` (Sundry Debtors, bill-wise No) | `MLab Fresh Party` | unknown |
| Q-c0 | `MLab Probe Pvt. Ltd.` (Sundry Creditors, bill-wise Yes) | none | CREATED=1 (control) |
| Q-c | `MLab Probe Pvt Ltd` (Sundry Debtors, bill-wise No) | `MLab Probe Pvt. Ltd.` | unknown |

Q-0, Q-r, Q-a and Q-c use a parent and bill-wise unlike their target's, so an overwrite shows in the row as well as in the counters. For Q-r, also report whether the target row carries a REMOTEID after it.

Report per case: the counters and any LINEERROR text; the ALTMSTID step; from R-V3 after, every ledger row whose names include the created name or the target's names, each with GUID, ALTERID, PARENT, ISBILLWISEON, the NAME attribute and all NAME.LIST names.

If Q-0 is not ALTERED=1, STOP before Q-r. If Q-c0 is not CREATED=1, STOP before Q-c. The other STOP rules apply as in every phase.

Grant: 5 writes and 18 reads.

## Evidence
- Everything goes in `C:\bridge-lab\masters-m1\evidence\`: each response and its headers, log.txt, and the screenshots.
- Zip it with its SHA256 for the requester.
- Nothing from a client book is in it.
