# The 0.7.0 release-candidate sitting: one build, seven rows (Part of #2056)

Not for merge. This is the one combined sheet for the release-candidate build of ComplyEaze Bridge 0.7.0: the live write pass, the masters row, and the rows promised by pull requests merged into the release. It is written before the cut commit is chosen, so it names no build: the run coordinator fills the BUILD block and the row notes below by new commits on this branch. No row starts while the BUILD block, or that row's own note, holds a field reading `<to fill>`.

`LAB-COMMON.md` (v3, a byte copy, sha256 in `SHA256.txt`) applies to every row in full. Where a row's own sheet and this README differ, this README holds for the build, the order, who acts, the folders, the evidence, the rows' conditions and what decides the tag; everywhere else the stricter rule holds.

## The rows

| row | what | book | writes | blocks the tag | runs when |
|---|---|---|---|---|---|
| R1 | W1, the live write pass: one approved post and the checks that changed since 0.6.1 | BRIDGE TIE OUT LAB 3 | one voucher, then restored | **yes** | always |
| R2 | the masters row, run CUT: one ledger created through the tools and the tool's read-back verdict, on the corrected verdict (#2165) | a new BRIDGE LEDGERBOX CUT LAB | one ledger (kept) | **yes** | always; the build must hold #1650 and #2166 |
| R3 | RC-SETTLE: a native post into a period locked in Tally, settled by the person if Tally refuses it whole (#2150) | named in the R3 NOTE | one Journal at most, then restored | **yes, if #2150 is in the build** | only if #2150 (with its settle box system-modal) is in the build |
| R4 | the Bills report limit of #2158, `outstandings` at 31 March on the dense book | BRIDGE CORPUS DENSE | none (reads) | **yes, if #2158 is in the build** (its release hold on #2119) | only if #2158 is in the build |
| R5 | the cost-centre summary of #2149 read through the tool | BRIDGE COST KEY LAB | none (reads) | no | always |
| R6 | part D of row 1780-H: the approval box under the desktop client as the server's parent | BRIDGE BATCH LAB | none (answered No) | no | only with the owner at the PC |
| R7 | a ledger asked for by its alias (#1688) | BRIDGE LINE BREAK LAB | one alias on Tally's screen, then restored | no | always |

Order: R1, R2, R3, R4, R5, R7, then R6 whenever the owner is at the PC (it may come earlier; it never interrupts a row). One row at a time, each with its own launch and its own new data folder; a row's STOP stops that row only, and the next row starts on the run coordinator's word. R4 comes after the writes because an abandoned Bills request leaves Tally working for minutes.

**A rerun on a new build** (see "What the tag waits on") uses the folder `C:\bridge-lab\rc070-<n>\` in place of `C:\bridge-lab\rc070\` (n = 2, 3, ...), and R2 creates `BRIDGE LEDGERBOX CUT LAB <n>`.

## BUILD (filled by the run coordinator; one build for every row)

The cut build is the Windows bundle `unsigned-bundle-smoke-windows-latest` of the Bridge CI run on master at RC COMMIT, the commit that holds the last change merged for 0.7.0; a master CI bundle may be downloaded by the lab (the owner's word of 29 Oct 2026). The lab downloads it into the new empty folder `C:\bridge-lab\rc070\build\`, checks the zip's sha256 against the artifact's digest, the `.mcpb` against `archive_sha256` and `bridge_mcp.exe` against `binary_sha256` in its `mcpb-smoke.json`, extracts the WHOLE `.mcpb` there (so the exe is `C:\bridge-lab\rc070\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe`, with `pdfium.dll` beside it), and sends the three hashes to the run coordinator. Every row runs this one exe, copied where a row says so: hash it before each row's first call and after its last; another hash = STOP.

```
RC COMMIT:   <to fill>
CI RUN:      <to fill>    ARTIFACT: <to fill>    ZIP DIGEST: <to fill>
MCPB SHA256: <to fill>
EXE SHA256:  <to fill>
VERSION:     <to fill>    (as the exe's `initialize` reports it)
```

**In the build** (the run coordinator writes `in` or `not in` on each line, by `git merge-base --is-ancestor <merge> <RC COMMIT>`; an item merged after this sheet was written is named by its merge commit):

```
#1650 ledger create (5fa399809):                     <to fill>
#2166 the master-mark step of a ledger create (#2165): <to fill>
#1967 post_import text (0bb49bdfe):                  <to fill>
#1995 held_elsewhere step (2e3ce9af1):               <to fill>
#1896 another change during a post (7f7f50e58):      <to fill>
#2047 STATUS 0 floor (565ffff52):                    <to fill>
#2105 dates in the boxes (b089e4963):                <to fill>
#2116 dates in the boxes, follow-up (917ec73d2):     <to fill>
#2107 boxes system-modal (621233407):                <to fill>
#2149 cost-centre summary (134f58b6a):               <to fill>
#2150 settle a batch created none, with its settle box system-modal: <to fill>
#2158 Bills report 60 s:                             <to fill>
```

A row whose condition reads `not in` is recorded "not run: not in the build"; what that means for the tag is under "What the tag waits on".

## Who acts

The lab does every step of R1, R2, R3, R4 (steps 0 to 4 and 7), R5 and R7 on these synthetic companies, including the steps a source sheet marks [OWNER] or gives to the owner: Tally's own screens, backups and restores (a restore still only on the run coordinator's RESTORE), switching posting and ledger creation on and off, and answering every approval and settle box by mouse. It does so on the standing word of 28 Oct 2026, and records each such step as "done by the lab on the standing word of 28 Oct". The run coordinator's word (a go-ahead, a RESTORE, a STOP ruling, the evidence push) comes from the run coordinator. R6, and R4's steps 5 and 6 if they run, need the owner at the PC: the desktop client is reconfigured and restarted for them by the owner, outside the lab's own session, never by the lab.

## Common rules for this sitting

1. **One build** (above). One launch per row, over stdio, unless the row says otherwise, with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use), `BRIDGE_AGENT_REDACTION`, `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_ENABLE_BATCH_POST` unset, and a NEW empty `BRIDGE_AGENT_DATA_DIR` named per row below (listed absent or empty first, else STOP).
2. **Writes off.** Posting (`BRIDGE_AGENT_ENABLE_WRITES`) and ledger creation (`BRIDGE_AGENT_ENABLE_MASTER_WRITES`) are on only in the rows that say so. At the end of such a row, and at once on any STOP or FAIL in it: any open ComplyEaze Bridge box gets **No** by mouse; the launch is stopped by closing its input (nothing is ended by name or number: C2); both keys are removed from the launch's environment and its sha256 recorded. This sends nothing to Tally.
3. **One ComplyEaze Bridge calling port 9001** at a time, except where a row names a second process. Record the `bridge_mcp.exe` process list, with each process's parent, before each row. A `bridge_mcp.exe` the row did not start, whose parent is the desktop client, is recorded and is not a STOP while the row's own record shows no call from it (R4 says how its source rule reads them); any other is a STOP.
4. **One Tally.** Before each row and before every call that writes: exactly one `tally.exe`, and it owns port 9001 (`Get-NetTCPConnection -LocalPort 9001 -State Listen`).
5. **Boxes.** Before the first call of R1, R2, R3 and R6, the lab's screen tool must be able to see and click a window of the exe the row runs: record that its grant covers that exe's path (a grant declined for a new path left a box unanswered for 40 s in the CI run of the masters row). From a call that can open a ComplyEaze Bridge box until its answer, nobody presses a key on the PC and nothing is clicked except the box's own button by mouse. With #2107 in the build every box opens system-modal, above Tally, and is never reached through the taskbar (a taskbar activation ended unanswered boxes, #2161; while one is open the taskbar may show no buttons). A box not seen 20 s after its call: a screenshot of the whole screen, no click, wait for the call's own answer (`pending`), record it, and STOP that row. For each box record, from the screenshot taken before it is answered, whether it is above Tally; its lines may be transcribed from that screenshot after it is answered.
6. **Dates in boxes.** With #2105 in the build, a box's date lines (`Date:`, `Books from:` and any line that labels a date) read in Tally's form, with the day not zero-padded (`30-Sep-2026`, `1-Apr-2025`). Such a line in `YYYYMMDD` form is a FINDING, recorded, not a STOP. Digits inside a reference, narration or batch id are not date lines.
7. **Answers.** Every tool answer is kept whole with its size and sha256. A text check reads the named field of the answer as parsed JSON, never a search of the file's bytes. Amounts are compared as numbers (`-1500.00` equals `-1500`); this sheet writes them with no digit grouping.
8. **Evidence** for the whole sitting goes on ONE new branch, `lab/0.7.0-rc-sitting-answers`, one folder per row (`R1/` to `R7/`), each with its `SHA256.txt`, plus `RESULTS.md` at the top (below). Where a source sheet names its own answers branch or its own push step, the row's evidence is committed into its folder here instead, and pushed with the sitting's (rule 8's push). Screenshots, backups, statement files and data folders stay on the lab PC, listed by name, size and sha256. Before the push: the pre-push scan; any machine path in an answer replaced by `<DATA_DIR>` or `<LAB_DIR>`, each replacement recorded; R2's new company GUID added to the merge gate's synthetic-UUID registry, or shown as `<company guid>`. The SHA goes to the run coordinator and the push waits for its yes. The evidence commits carry no `Co-Authored-By` or "Generated with" line.

## R1: the live write pass (BLOCKING)

`src/W1/README.md` and `src/W1/W1-write-pass.md`, byte copies from `lab/0.7.0-write-pass` at 493e6c0ab6ba1e226490b1919bebf97b7b75392a, run whole, with these changes:

- **Build.** The BUILD block above replaces W1's own build line; its "In the build" lines replace W1's ancestor note. Data folder `C:\bridge-lab\rc070\r1\data\`; backup folder `C:\bridge-lab\rc070\r1\backups\BK1\`.
- **The window (W1 README, THE APPROVAL WINDOW).** "Look for it on the taskbar" is replaced by rule 5: with #2107 in the build the box is expected above Tally and is never reached through the taskbar.
- **Step 0.4.** If M0 and V0 differ from 210 and 8, that STOP is reported with the marks read, and the run coordinator decides whether the book is used as it is.
- **Step 6.2, the approval window (added).** Record the box's title and every line word for word (rule 5), and, with #2105 in the build, EXPECT its date line to carry `30-Sep-2026` (rule 6); this holds for every box step 6 opens. Record whether the box is above Tally. A line that names a company, ledger, date, amount or batch other than step 6's = **No** by mouse, then STOP.
- **Not covered (added).** #2073 (a lost-answer ledger create settling as `posted_unattributed`) and #2082 (recording that a person checked a content-only match) are not reached by a clean post; their unit tests carry them.
- **Evidence** in `R1/` (rule 8).

## R2: the masters row, run CUT (BLOCKING)

`src/1650/RUN-SHEET.md`, a byte copy from `lab/1650-ledger-create-box-row` at adae9d8cd005d0669caa1f1a24c8061c48d5dcff. Run CI is done (`lab/1650-ledger-create-box-row-answers` at acc5c12d49007119e5512dc1691d672e79041246). Run **CUT** only, whole (steps 0 to 9), with these changes:

- **Build.** The BUILD block above. `#1650` and `#2166` must both read `in`; otherwise R2 does not start and the run coordinator decides (the verdict the row checks is the corrected one, #2165). Its `tools/list` must name `prepare_ledger_creates` and `post_ledger_creates` (absent = STOP). Data folder `C:\bridge-lab\rc070\r2\data\`; backup folder `C:\bridge-lab\rc070\r2\backups\bk1\`.
- **Who acts.** As "Who acts" above: the lab sets both write keys and answers the box by mouse. The source sheet's rule 6 (one Tally) and rule 7 (click only) stand.
- **The fit check (correction).** The source sheet says the one-ledger box is measured against the screen when it opens. On master the ledger-create box is shown without that check (its dialog path does not pass through the fit check), so `master_review_too_large` can come only from the text's size. Step 5's FINDING line stands as written: such an answer, or `master_approval_unavailable`, while the box was whole on the screen, is the row's finding.
- **Steps 3 and 5 (added).** With #2105 in the build, EXPECT the preview's company line to read `Books from: 1-Apr-2025` (rule 6).
- **Step 6 (added).** The verdict is `posted_verified` only for a master-mark step of exactly one (#2165). EXPECT `posted_verified` as the sheet says; step 7 checks M1 = M0′ + 1. Any other final state is recorded whole, as the sheet says.
- **Step 7 (changed).** If `ledger_masters` gives no GUID for the new row, record "GUID not shown" in place of the prefix check; it is not a STOP (run CI found the tool returns none).
- **Evidence** in `R2/` (rule 8). THE COMPANY's new GUID: rule 8.

## R3: RC-SETTLE, settle a native post Tally refused whole (BLOCKING if #2150 is in the build)

Runs only if `#2150` reads `in`, and only from a build holding #2150's forward commit that makes the settle box system-modal (the BUILD line says so). `src/2150/RC-SETTLE.md`, a byte copy from `lab/2150-rc-settle` at 544a26a570590ce73f90f36cd70ccd034140a0fe, runs whole, with these changes:

- **The row note.** RC-SETTLE asks for a row note in the auto order; for this sitting the note is the R3 NOTE block below, filled by the run coordinator. Its build fields are the BUILD block above, and its download OK is the owner's word of 29 Oct 2026 on master CI bundles.
- **Who acts.** As "Who acts" above, which covers the [OWNER] steps, the lock on Tally's screen and the settle dialog.
- **Data folder** `C:\bridge-lab\rc070\r3\data\`, backup folder `C:\bridge-lab\rc070\r3\bk0\`, Part 2's throwaway folder `C:\bridge-lab\rc070\r3\part2-data\`. **Evidence** in `R3/` (rule 8), in place of `lab/2150-rc-settle-answers`.

```
R3 NOTE
THE COMPANY:   <to fill: a synthetic book with exactly ONE Currency master; its GUID and number>
L1, L2:        <to fill: two ledgers that take a balanced Journal, neither bill-wise, cash nor bank>
THE LOCK:      <to fill: the period locked on Tally's screen, its first day; its last day is THE DATE>
THE DATE:      <to fill>
MARKS WINDOW:  <to fill: the vouchers window of every MARKS read>
C2 RAW MARKS:  <to fill: the request file, its sha256, and the GET /status command>
PART 2:        <to fill: the data folder of the 7 Oct 2026 rehearsal and the company it posted into, or "not run">
GO-AHEAD:      <to fill>
```

## R4: the Bills report limit on the dense book (BLOCKING if #2158 is in the build)

Runs only if `#2158` reads `in`. #2158 stays in 0.7.0 only on the owner's measurement of how long the desktop client's chat waits for one tool call on Windows, which the run coordinator records here:

```
R4 HOST: <to fill: the desktop chat's measured limit on Windows, how and when measured, and whether steps 5 and 6 run>
```

`src/2119/RUN-SHEET.md`, a byte copy from `lab/2119-bills-60s-row` at c43ef6120c62ba7a480eadf3f639023e190817a4, which ran on #2158's own build (`lab/2119-bills-60s-row-answers` at da4015419b253cb8a40144ac35ebcc68b0686274: S1 answered in 76.9 s; the Windows Code tab host cut every call at 60 s, so that sheet's rules (d) and (e) held the limit for that host). On the release candidate it runs with these changes:

- **Folders.** Every path the source names under `C:\bridge-lab\2119-60s\` is read under `C:\bridge-lab\rc070\r4\` (`bin`, `data-a`, `data-b`, `data-d`, `sleep`, `sleep-log`, the configuration copy); the desktop entry keeps its name `bridge-2119-60s`. The exe is the BUILD block's, copied to `C:\bridge-lab\rc070\r4\bin\bridge_mcp.exe`.
- **Steps.** 0, 1, 2 (S1 and P1), 3 (S2), 4 and 7 run. Steps 5 (H1) and 6 (D1), with the desktop client's setup and restore, run only if R4 HOST says so; otherwise they are recorded "not run: R4 HOST" and the desktop client is not touched.
- **Process D (source rule 3).** If steps 5 and 6 do not run, process D's set is the `bridge_mcp.exe` processes whose parent is the desktop client at step 0, recorded with their PIDs (it may be empty). Rule 3's checks before steps 1 and 7 compare against that set: a change in it is a STOP, as the source says; the set itself is not.
- **Step 8.** The evidence is committed into `R4/` and pushed with the sitting's (rule 8). If steps 5 and 6 ran, the owner's restore of the desktop configuration follows R4's evidence commit, not the sitting's push.
- **What it decides.** The source's rules (a) to (c) are judged on S1. The limit then **passes** only if (a) to (c) do not hold it AND either steps 5 and 6 ran and the source's rules (d) to (f) do not hold it, or R4 HOST records a desktop chat limit of at least 90 seconds on Windows. Otherwise R4's result is **HELD**, not PASS: it goes to the run coordinator with the 29 Oct finding, and the tag waits for the decision on #2158.
- **Evidence** in `R4/` (rule 8).

## R5: the cost-centre summary through the tool (reads; not blocking)

Promised by #2149: "A release-candidate row on BRIDGE COST KEY LAB for 2025-05-15..16 will check it." Runs if `#2149` reads `in`.

THE COMPANY is BRIDGE COST KEY LAB, GUID `ddf3a557-704c-4f61-ae8d-46a3380062a1`, books from 1 April 2025, Cost Centres set to Yes, with the Payment of 15 May 2025 and the Journal of 16 May 2025 keyed on 27 Oct 2026 (its captures are `voucher_cost_centre_allocations_keyed_*` on master, provenance in `VOUCHER_COST_CENTRE_KEYED_CAPTURE_PROVENANCE.md`). Absent, or a different GUID = STOP. READS ONLY: no backup, no restore. Posting off. Data folder `C:\bridge-lab\rc070\r5\data\`.

0. LAB-COMMON step 0 with no company loaded, without its MARKS request (its GET /status checks stay); load THE COMPANY alone. Hash the exe.
1. `list_companies`: EXPECT exactly THE COMPANY. `ledger_masters` (first page, basic): M0 and V0 (`snapshot.master_alter_id`, `snapshot.voucher_alter_id`, `snapshot.reused` false).
2. `vouchers` with `company_guid`, `from` 20250515 and `to` 20250516, nothing else: record it whole. EXPECT two vouchers (a Payment on 15 May, a Journal on 16 May); on each, the entry on `CK Expense` carries two `cost_centre_allocations` rows and the bank entry none, and no `cost_centre_allocations_unreadable` anywhere. Another count or shape = STOP before step 3 (the book is not as captured).
3. `vouchers` with the same three arguments and `summarise_by` `cost_centre`: record it whole. EXPECT, from the result (redaction is off when `BRIDGE_AGENT_REDACTION` is unset; amounts compared as numbers, rule 7):
   - four `buckets`, in this order, named by `category` / `group`, each with `members` holding `CK Expense` only, `members_total` 1, `vouchers` 1 and `voucher_refs_complete` true: `Primary Cost Category` / `CK North`, debit -600.00; `Primary Cost Category` / `CK South`, debit -500.00; `CK Region` / `CK Retail`, debit -500.00; `Primary Cost Category` / `CK North East`, debit -400.00; each with credit 0;
   - `category_totals`: `Primary Cost Category` with 2 entries, debit -1500.00, credit 0; `CK Region` with 1 entry, debit -500.00, credit 0;
   - `totals` debit -1500.00 and credit 0; `entries_unallocated` 2 entries, debit 0, credit 1500.00; `vouchers_summarised` 2; `entries_counted` `all_entries`.
   These are the figures the merged tests give for the two captures. A refusal is a result: record its code and cause. Any other figure is the row's FINDING (record, no STOP: nothing is written).
4. `vouchers` with the same arguments, `summarise_by` `cost_centre` and `ledger` `CK Bank`: EXPECT no buckets, `entries_counted` `selected_ledger` and `entries_unallocated` 2 entries.
5. `ledger_masters` (basic): EXPECT M0 and V0. `egress_log`: record it whole. Hash the exe. LAB-COMMON C10.

Evidence in `R5/` (rule 8). Not covered: masking (`mask_parties`), any other window or book, Gold.

## R6: part D of row 1780-H (owner at the PC; not blocking)

Runs only with the owner at the PC (#2107 is merged, so its line should read `in`; if not, "not run"). `src/1780h/RUN-SHEET.md` and `src/1780h/requests/`, byte copies from `lab/1780-system-modal-box` at 693a04f5258cb14592b197b966de5aa1e60c0572 (row 1780-H ran parts H, S and L on #2107's own build, `lab/1780-system-modal-box-answers` at 968810d4e2681c96b0f3e6dd9652eac311e0f5a3; part D was not run). With these changes:

- **Folders.** Every path the source names under `C:\bridge-lab\1780h\` is read under `C:\bridge-lab\rc070\r6\`: the `requests\` files are copied there from `src/1780h/requests/` and checked against `SHA256.txt`; BUILD-H is the BUILD block's build, extracted into `C:\bridge-lab\rc070\r6\build\` (the exe at `build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe`); evidence in `C:\bridge-lab\rc070\r6\evidence\`.
- **What runs.** "Before the sitting" P0, B1, B3, B4 and B5 (B3b is not run: it needs part H's child launch); then S0, the backup BK0 to `C:\bridge-lab\rc070\r6\bk0\`; then the marks and part D below; then WRITES OFF and the shut. Parts H, S and L are not run.
- **Marks, by the lab's own launch.** After BK0 and before the owner's setup, the lab starts its own launch of the RC exe (writes off, data folder `C:\bridge-lab\rc070\r6\lab-data\`): `list_companies` (EXPECT BRIDGE BATCH LAB alone) and `ledger_masters` (basic): M0 and V0. It stops that launch before part D. After part D and the owner's restore, it starts it again on the same folder: `ledger_masters` (basic): EXPECT M0 and V0 (a moved mark = FAIL, as the source's "A write in the book" says), then stops it.
- **Part D, made concrete.** Before the lab's session for part D starts, the owner points the desktop client's ComplyEaze Bridge entry at the R6 exe with `BRIDGE_AGENT_ENABLE_WRITES` `true`, `BRIDGE_AGENT_DATA_DIR` `C:\bridge-lab\rc070\r6\data` (new, empty) and the other keys of rule 1; records the configuration's sha256 before and after; and restarts the desktop client. Then, in a new chat of the desktop client, the owner asks it to call `build_import_xml` with the content of `requests\build-1591-s1.json` as its arguments (EXPECT `voucher_count` 1, as S3), and then `post_import` with that `company_guid` and the returned `batch_id`. In that chat each tool approval is "Allow once", and only for exactly those two calls; any other call is denied and recorded. The lab pre-arms the trace and the probes as in S4 (the title click to T0 as there); the owner makes the one click into Tally at T0+16 (±2 s) as in S6, and answers **No** by mouse before T0+38. Record `exstyle`, `topmost`, the foreground and the rectangle from the trace and probes at T0+3, +8 and +13 and at C+0 and C+6, and the call's final answer (EXPECT `import_approval_declined`, nothing sent). After the lab's part D notes are written, the owner removes `BRIDGE_AGENT_ENABLE_WRITES` from the entry and restarts the desktop client.
- **Reading it.** As the source sheet's table: topmost 1 in every read and the box above Tally after the click = the fix holds under the desktop client.
- **Evidence** in `R6/` (rule 8).

## R7: a ledger asked for by its alias (#1688; not blocking)

Runs always. #1688's alias handling (#1937, #2028) is built but has never been read through the tool. Through ComplyEaze Bridge this row is READS ONLY: the tool sends no alias and writes nothing. It needs a ledger that carries an alias, and no lab book is known to hold one now, so the row adds one alias on Tally's own ledger screen, between a backup and a restore.

THE COMPANY is BRIDGE LINE BREAK LAB (GUID `224e7f8c-a0aa-45ce-889a-ad4b7fdbe852`), a synthetic book. L is its ledger `Lab Plain Ledger`; A is the alias `Lab Plain Alias`, as on 13 Oct 2026 (`lab/626-twin-answers` at a283cd3cd4, step t6, where adding it moved the master mark by one). Absent, or another GUID = STOP. Posting off. Data folder `C:\bridge-lab\rc070\r7\data\`; backup folder `C:\bridge-lab\rc070\r7\bk0\`.

0. LAB-COMMON step 0 with no company loaded, without its MARKS request (its GET /status checks stay); load THE COMPANY alone. Hash the exe. `list_companies`: EXPECT exactly THE COMPANY. `ledger_masters` (first page, basic): M0, V0 and the ledger rows. EXPECT L listed, and no ledger or group whose name differs from A only in case, spaces or symbols; otherwise STOP.
1. Backup BK0 (C5) into the folder above, then `ledger_masters` (basic): M0′ (a mark moved across the backup is recorded, as C5 says). On Tally's screen: Gateway of Tally, Alter, Ledger, L; add A as its alias; accept the screen (C7 for leaving it); a screenshot of the Gateway with no dialog. A message at the alias field = C7, STOP (nothing was saved). `ledger_masters` (basic): EXPECT M = M0′ + 1 and V unchanged, and L still listed under its own name; another step is recorded, not a STOP.
2. `validate_masters` with `company_guid` and `ledgers` `["Lab Plain Alias", "Lab Plain Ledger", "Lab Alias Control Absent"]` (the third is a name no ledger, alias or group of THE COMPANY carries). Record it whole. EXPECT, in `result.masters`, in that order:
   - `Lab Plain Alias`: `match_state` `missing` or `near_miss`, and `alias_of` `Lab Plain Ledger`. No `alias_of`, or one naming another ledger, is the row's FINDING (recorded; not a STOP: nothing is written).
   - `Lab Plain Ledger`: `match_state` `exact`, with no `alias_of`.
   - the control: `match_state` `missing`, with no `alias_of`.
3. `ledger_masters` (basic): EXPECT M and V as read after step 1 (the read wrote nothing). `egress_log`: record it whole. Hash the exe.
4. Restore BK0 (C6) on the run coordinator's RESTORE. Then MARKS (C6) and `ledger_masters` (basic): EXPECT M0′ and V0 and the ledger rows of step 0; on Tally's screen, L's Alter form (read, then left by Ctrl+Q after a screenshot of its title) shows no alias. Then C10.

Evidence in `R7/` (rule 8). Not covered: the twin check through an alias (Tally's screen refused every alias twin tried on 21 Oct 2026, so one cannot be made there), a build naming the alias (refused as `masters_not_exact`, nothing sent), aliases of groups, Gold.

## RESULTS.md (the answers branch, at the top)

One line per row: PASS, FAIL, FINDING, HELD (R4 only), STOP or "not run" (with its reason), with the build (RC COMMIT and EXE SHA256), the company, the date and the folder. R1's checks are listed as W1's "Close and evidence" lists them (1967, 1995, the post, 1896 no false line, rollback record, rollback cover), plus the box's date line. R2: the decline, the create and its verdict, the journal answer. R3: RC-SETTLE's own outcome lines. R4: rules (a) to (c), then (d) to (f) or R4 HOST, and PASS or HELD. R5: each EXPECT of steps 2 to 4. R6: topmost and the position after the click. R7: each EXPECT of step 2.

## What the tag waits on

R1 and R2 passing on the cut build. R3 passing if `#2150` reads `in`: an R3 not run while #2150 is in the build holds the tag, whatever the reason. R4 passing if `#2158` reads `in`: a HELD R4, or an R4 not run while #2158 is in the build, holds the tag until the run coordinator decides on #2158. A FAIL or a STOP in any of them goes to the run coordinator, who decides the fix and the rerun; a fix makes a new build, and every blocking row runs again on it (see "A rerun on a new build"). R5, R6 and R7 do not hold the tag; their results go into the release notes as run or not run.

## Files

`README.md` (this sheet), `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt` (each line: sha256, bytes, path, and for a copy its source), and under `src/` the byte copies named above. Check every file against `SHA256.txt` before the sitting.
