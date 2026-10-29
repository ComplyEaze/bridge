# The 0.7.0 release-candidate sitting: one build, six rows (Part of #2056)

Not for merge. This is the one combined sheet for the release-candidate build of ComplyEaze Bridge 0.7.0: the live write pass, the masters row, and the rows promised by pull requests merged into the release. It is written before the cut commit is chosen, so it names no build: the run coordinator fills the BUILD block below by a new commit on this branch, and no row starts while any field there reads `<to fill>`.

`LAB-COMMON.md` (v3, a byte copy, sha256 in `SHA256.txt`) applies to every row in full. Where a row's own sheet and this README differ, this README holds for the build, the order, the evidence and the rows' conditions; everywhere else the stricter rule holds.

## The rows

| row | what | book | writes | blocks the tag | runs when |
|---|---|---|---|---|---|
| R1 | W1, the live write pass: one approved post and the checks that changed since 0.6.1 | BRIDGE TIE OUT LAB 3 | one voucher, then restored | **yes** | always |
| R2 | the masters row, run CUT: one ledger created through the tools and the tool's read-back verdict | a new BRIDGE LEDGERBOX CUT LAB | one ledger (kept) | **yes** | always (the build must hold #1650) |
| R3 | the person's settlement of a batch Tally answered as creating none of its vouchers (#2150) | named by its own sheet | its own sheet says | **yes, if #2150 is in the build** | only if #2150 is in the build |
| R4 | the Bills report limit of #2158, `outstandings` at 31 March on the dense book | BRIDGE CORPUS DENSE | none (reads) | **yes, if #2158 is in the build** (its release hold on #2119) | only if #2158 is in the build |
| R5 | the cost-centre summary of #2149 read through the tool | BRIDGE COST KEY LAB | none (reads) | no | always |
| R6 | part D of row 1780-H: the approval box under the desktop client as the server's parent | BRIDGE BATCH LAB | none (answered No) | no | only if #2107 is in the build, and only with the owner at the PC |

Order: R1, R2, R3, R4, R5, then R6 whenever the owner is at the PC (it may come earlier; it never interrupts a row). One row at a time, each with its own launch and its own new data folder; a row's STOP stops that row only, and the next row starts on the run coordinator's word. R4 comes after the writes because an abandoned Bills request leaves Tally working for minutes.

## BUILD (filled by the run coordinator; one build for every row)

The cut build is the Windows bundle `unsigned-bundle-smoke-windows-latest` of the Bridge CI run on master at RC COMMIT, the commit that holds the last change merged for 0.7.0. The lab downloads it into the new empty folder `C:\bridge-lab\rc070\build\`, checks the zip's sha256 against the artifact's digest, the `.mcpb` against `archive_sha256` and `bridge_mcp.exe` against `binary_sha256` in its `mcpb-smoke.json`, extracts the WHOLE `.mcpb` there (`pdfium.dll` beside the exe), and sends the three hashes to the run coordinator. Every row runs this one `bridge_mcp.exe`: hash it before each row's first call and after its last; another hash = STOP.

```
RC COMMIT:   <to fill>
CI RUN:      <to fill>    ARTIFACT: <to fill>    ZIP DIGEST: <to fill>
MCPB SHA256: <to fill>
EXE SHA256:  <to fill>
VERSION:     <to fill>    (as the exe's `initialize` reports it)
```

**In the build** (the run coordinator writes one line per item: `in` or `not in`, by `git merge-base --is-ancestor <merge> <RC COMMIT>`; an item merged after this sheet was written is named by its merge commit):

```
#1650 ledger create (5fa399809):                <to fill>
#1967 post_import text (0bb49bdfe):             <to fill>
#1995 held_elsewhere step (2e3ce9af1):          <to fill>
#1896 another change during a post (7f7f50e58): <to fill>
#2047 STATUS 0 floor (565ffff52):               <to fill>
#2105 dates in the boxes (b089e4963):           <to fill>
#2116 dates in the boxes, follow-up (917ec73d2): <to fill>
#2149 cost-centre summary (134f58b6a):          <to fill>
#2107 boxes system-modal:                       <to fill>
#2150 settle a batch created none:              <to fill>
#2158 Bills report 60 s:                        <to fill>
the Bills call budget (if built):               <to fill>
```

A row whose condition reads `not in` is recorded "not run: not in the build" and is not a failure.

## Who acts

The lab does every step of R1 to R5 on these synthetic companies, including the steps a source sheet marks [OWNER], switching posting and ledger creation on and off, and answering every approval box by mouse, on the owner's standing word of 28 Oct 2026; each such step is recorded as "done by the lab on the owner's standing word of 28 Oct". The run coordinator's word (a go-ahead, a RESTORE, a STOP ruling, the evidence push) comes from the run coordinator. R6, and R4's steps 5 and 6 if they run, need the owner at the PC: the desktop client is restarted for them, and the lab never restarts it.

## Common rules for this sitting

1. **One build** (above). One launch per row, over stdio, unless the row says otherwise, with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use), `BRIDGE_AGENT_REDACTION`, `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_ENABLE_BATCH_POST` unset, and a NEW empty `BRIDGE_AGENT_DATA_DIR` named per row below (listed absent or empty first, else STOP). Posting (`BRIDGE_AGENT_ENABLE_WRITES`) and ledger creation (`BRIDGE_AGENT_ENABLE_MASTER_WRITES`) are on only in the rows that say so, and removed from the launch's environment at the end of that row (the launch stopped first).
2. **One ComplyEaze Bridge at a time** calls port 9001, except where a row names a second process. Record the `bridge_mcp.exe` process list before each row; any `bridge_mcp.exe` the row did not start is recorded, and is a STOP only if it makes a call during the row.
3. **One Tally.** Before each row and before every call that writes: exactly one `tally.exe`, and it owns port 9001 (`Get-NetTCPConnection -LocalPort 9001 -State Listen`).
4. **Boxes.** From a call that can open a ComplyEaze Bridge box until its answer, nobody presses a key on the PC and nothing is clicked except the box's own button by mouse. If #2107 is in the build, every box opens system-modal: for each box record whether it is above Tally in the screenshot taken before it is answered (no click into Tally is made to test it, except in R6). A date in a box reads in Tally's form with the day not zero-padded (`30-Sep-2026`, `1-Apr-2025`) if #2105 is in the build; a date in `YYYYMMDD` form in a box on such a build is a FINDING, recorded, not a STOP.
5. **Answers.** Every tool answer is kept whole with its size and sha256. A text check reads the named field of the answer as parsed JSON, never a search of the file's bytes.
6. **Evidence** for the whole sitting goes on ONE new branch, `lab/0.7.0-rc-sitting-answers`, one folder per row (`R1/` to `R6/`), each with its `SHA256.txt`, plus `RESULTS.md` at the top (below). Screenshots, backups, statement files and data folders stay on the lab PC, listed by name, size and sha256. The pre-push scan runs first; the SHA goes to the run coordinator, and the push waits for its yes. The evidence commits carry no `Co-Authored-By` or "Generated with" line. A synthetic company's GUID that is new (R2's) goes into the merge gate's synthetic-UUID registry before the push, or the evidence shows it as `<company guid>`.

## R1: the live write pass (BLOCKING)

`src/W1/README.md` and `src/W1/W1-write-pass.md`, byte copies from `lab/0.7.0-write-pass` at 493e6c0ab6ba1e226490b1919bebf97b7b75392a, run whole, with these changes:

- **Build.** The BUILD block above replaces W1's own build line; its "In the build" lines replace W1's ancestor note. Data folder `C:\bridge-lab\rc070\r1\data\`; backup folder `C:\bridge-lab\rc070\r1\backups\BK1\`.
- **Step 0.4.** If M0 and V0 differ from 210 and 8, that STOP is reported with the marks read, and the run coordinator decides whether the book is used as it is.
- **Step 6.2, the approval window (added).** Before answering Yes, record the box's title and every line word for word, and, if #2105 is in the build, EXPECT its date line to carry `30-Sep-2026` and no `20260930` (rule 4). Record whether the box is above Tally (rule 4). A line that names a company, ledger, date, amount or batch other than step 6's = **No** by mouse, then STOP.
- **Not covered (added).** #2073 (a lost-answer ledger create settling as `posted_unattributed`) and #2082 (recording that a person checked a content-only match) are not reached by a clean post; their unit tests carry them. A post of 2 to 50 vouchers and a Sales invoice stay uncovered, as W1 says.
- **Evidence** in `R1/` (rule 6) in place of W1's own branch.

## R2: the masters row, run CUT (BLOCKING)

`src/1650/RUN-SHEET.md`, a byte copy from `lab/1650-ledger-create-box-row` at adae9d8cd005d0669caa1f1a24c8061c48d5dcff. Run CI is done (`lab/1650-ledger-create-box-row-answers` at acc5c12d49007119e5512dc1691d672e79041246). Run **CUT** only, whole (steps 0 to 9), with these changes:

- **Build.** The BUILD block above. `#1650` must read `in`; its `tools/list` must name `prepare_ledger_creates` and `post_ledger_creates` (absent = STOP). Data folder `C:\bridge-lab\rc070\r2\data\`; backup folder `C:\bridge-lab\rc070\r2\backups\bk1\`.
- **Who acts.** Rule 4 of the source sheet ("The OWNER answers the approval box") and its Build paragraph ("the owner sets ... himself") are replaced by "Who acts" above: the lab sets both keys and answers the box by mouse. The source sheet's rule 6 (one Tally) and rule 7 (click only) stand.
- **The fit check (correction).** The source sheet says the one-ledger box is measured against the screen when it opens. On master the ledger-create box is shown without that check (its dialog path does not pass through the fit check), so `master_review_too_large` can come only from the text's size. Step 5's FINDING line stands as written: such an answer, or `master_approval_unavailable`, while the box was whole on the screen, is the row's finding.
- **Step 3 and step 5 (added).** If #2105 or #2116 is in the build, EXPECT the preview's company line to read `Books from: 1-Apr-2025` (rule 4), not `20250401`.
- **Step 7 (changed).** If `ledger_masters` gives no GUID for the new row, record "GUID not shown" in place of the prefix check; it is not a STOP (run CI found the tool returns none).
- **Evidence** in `R2/` (rule 6) in place of `lab/1650-ledger-create-box-row-cut-answers`. THE COMPANY's new GUID: rule 6.

## R3: settle a batch Tally answered as creating none of its vouchers (BLOCKING if #2150 is in the build)

Runs only if `#2150` reads `in`. Its sheet is written for the build of #2150's merge and is pinned here by the run coordinator, by a new commit on this branch, as `R3 SHEET: <branch> at <full SHA>, file <path>, sha256 <hash>`, before the row starts; while that line is absent, R3 does not run.

```
R3 SHEET: <to fill>
```

That sheet runs on the BUILD block above, in place of any build it names, with data folder `C:\bridge-lab\rc070\r3\data\`; its evidence goes in `R3/` (rule 6). Everything else is that sheet's.

## R4: the Bills report limit on the dense book (BLOCKING if #2158 is in the build)

Runs only if `#2158` reads `in`. `src/2119/RUN-SHEET.md`, a byte copy from `lab/2119-bills-60s-row` at c43ef6120c62ba7a480eadf3f639023e190817a4, which ran on #2158's own build (`lab/2119-bills-60s-row-answers` at da4015419b253cb8a40144ac35ebcc68b0686274). On the release candidate it runs with these changes:

- **Build.** The BUILD block above, copied to `C:\bridge-lab\rc070\r4\bin\bridge_mcp.exe`; data folders `C:\bridge-lab\rc070\r4\data-a` and `data-b` (and `data-d` if steps 5 and 6 run).
- **Steps.** 0, 1, 2 (S1 and P1), 3 (S2), 4 and 7 run. Steps 5 (H1) and 6 (D1), and with them the desktop client's setup and restore, run only if the line "the Bills call budget" reads `in`; otherwise they are recorded "not run: no call budget in the build", rule 3's process D list is empty, and the desktop client is not touched.
- **What it decides.** Rules (a) to (c) hold the limit as written. Rules (d) to (f) and the call budget apply only if steps 5 and 6 run. The sheet's EXPECT that S1 answers in about 70 to 100 s stands; on #2158's build it answered in 76.9 s.
- **Evidence** in `R4/` (rule 6) in place of the sheet's own branch.

## R5: the cost-centre summary through the tool (reads; not blocking)

Promised by #2149: "A release-candidate row on BRIDGE COST KEY LAB for 2025-05-15..16 will check it." Runs if `#2149` reads `in`.

THE COMPANY is BRIDGE COST KEY LAB, GUID `ddf3a557-704c-4f61-ae8d-46a3380062a1`, books from 1 April 2025, Cost Centres set to Yes, with the Payment of 15 May 2025 and the Journal of 16 May 2025 keyed on 27 Oct 2026 (its captures are `voucher_cost_centre_allocations_keyed_*` on master, provenance in `VOUCHER_COST_CENTRE_KEYED_CAPTURE_PROVENANCE.md`). Absent, or a different GUID = STOP. READS ONLY: no backup, no restore. Posting off. Data folder `C:\bridge-lab\rc070\r5\data\`.

0. LAB-COMMON step 0 with no company loaded, without its MARKS request (its GET /status checks stay); load THE COMPANY alone. Hash the exe.
1. `list_companies`: EXPECT exactly THE COMPANY. `ledger_masters` (first page, basic): M0 and V0 (`snapshot.master_alter_id`, `snapshot.voucher_alter_id`, `snapshot.reused` false).
2. `vouchers` with `company_guid`, `from` 20250515 and `to` 20250516, nothing else: record it whole. EXPECT two vouchers (a Payment on 15 May, a Journal on 16 May); on each, the entry on `CK Expense` carries two `cost_centre_allocations` rows and the bank entry none, and no `cost_centre_allocations_unreadable` anywhere. Another count or shape = STOP before step 3 (the book is not as captured).
3. `vouchers` with the same three arguments and `summarise_by` `cost_centre`: record it whole. EXPECT, from the result (redaction is off when `BRIDGE_AGENT_REDACTION` is unset):
   - four `buckets`, in this order, named by `category` / `group`, each with `members` holding `CK Expense` only, `members_total` 1, `vouchers` 1 and `voucher_refs_complete` true: `Primary Cost Category` / `CK North`, debit -600.00; `Primary Cost Category` / `CK South`, debit -500.00; `CK Region` / `CK Retail`, debit -500.00; `Primary Cost Category` / `CK North East`, debit -400.00; each with credit 0;
   - `category_totals`: `Primary Cost Category` with 2 entries, debit -1,500.00, credit 0; `CK Region` with 1 entry, debit -500.00, credit 0;
   - `totals` debit -1,500.00 and credit 0; `entries_unallocated` 2 entries, debit 0, credit 1,500.00; `vouchers_summarised` 2; `entries_counted` `all_entries`.
   These are the figures the merged tests give for the two captures. A refusal is a result: record its code and cause. Any other figure is the row's FINDING (record, no STOP: nothing is written).
4. `vouchers` with the same arguments, `summarise_by` `cost_centre` and `ledger` `CK Bank`: EXPECT no buckets, `entries_counted` `selected_ledger` and `entries_unallocated` 2 entries.
5. `ledger_masters` (basic): EXPECT M0 and V0. `egress_log`: record it whole. Hash the exe. LAB-COMMON C10.

Evidence in `R5/` (rule 6). Not covered: masking (`mask_parties`), any other window or book, Gold.

## R6: part D of row 1780-H (owner at the PC; not blocking)

Runs only if `#2107` reads `in` and the owner is at the PC; otherwise "not run" with the reason. `src/1780h/RUN-SHEET.md` and `src/1780h/requests/`, byte copies from `lab/1780-system-modal-box` at 693a04f5258cb14592b197b966de5aa1e60c0572 (row 1780-H ran parts H, S and L on #2107's own build: `lab/1780-system-modal-box-answers` at 968810d4e2681c96b0f3e6dd9652eac311e0f5a3; part D was not run). With these changes:

- **Build.** BUILD-H is the BUILD block above, extracted as rule "Build" of that sheet says into `C:\bridge-lab\rc070\r6\build\`. Working folder `C:\bridge-lab\rc070\r6\`; evidence there.
- **What runs.** "Before the sitting" (P0, B1, B3, B4, B5), then S0 (the backup BK0 to `C:\bridge-lab\rc070\r6\bk0\`), then part D, then L6's end checks without the `Lab Topmost Suspense` line, WRITES OFF and the shut. Parts H, S, L and B3b are not run.
- **Part D, made concrete.** The owner points the desktop client's ComplyEaze Bridge entry at BUILD-H's `bridge_mcp.exe` with `BRIDGE_AGENT_ENABLE_WRITES` `true`, `BRIDGE_AGENT_DATA_DIR` `C:\bridge-lab\rc070\r6\data` (new, empty), and the other keys of rule 1; records the configuration's sha256 before and after; restarts the desktop client; and in a new chat asks it to call `build_import_xml` with the content of `requests/build-1591-s1.json` as its arguments (EXPECT `voucher_count` 1, as S3), and then `post_import` with that `company_guid` and the returned `batch_id`. Each tool approval is "Allow once". The lab pre-arms the trace and the probes as in S4 (title click to T0 as there), the owner makes the one click into Tally at T0+16 (±2 s) as in S6, and answers **No** by mouse before T0+38. Record `exstyle`, `topmost`, the foreground and the rectangle from the trace and probes at T0+3, +8 and +13 and at C+0 and C+6, and the call's final answer (EXPECT `import_approval_declined`, nothing sent). Then the owner removes `BRIDGE_AGENT_ENABLE_WRITES` from the entry and restarts the desktop client; WRITES OFF applies.
- **Reading it.** As the source sheet's table: topmost 1 in every read and the box above Tally after the click = the fix holds under the desktop client.
- **Evidence** in `R6/` (rule 6).

## RESULTS.md (the answers branch, at the top)

One line per row: PASS, FAIL, FINDING, STOP or "not run" (with its reason), with the build (RC COMMIT and EXE SHA256), the company, the date and the folder. R1's checks are listed as W1's "Close and evidence" lists them (1967, 1995, the post, 1896 no false line, rollback record, rollback cover), plus the box's date line. R2: the decline, the create and its verdict, the journal answer. R4: rules (a) to (c), and (d) to (f) only if steps 5 and 6 ran. R5: each EXPECT of steps 2 to 4. R6: topmost and the position after the click.

**The tag waits on** R1 and R2 passing, and on R3 and R4 where they run. A FAIL or a STOP in any of them goes to the run coordinator, who decides the fix and the rerun; a fix makes a new build, and every blocking row runs again on it.

## Files

`README.md` (this sheet), `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt` (each line: sha256, bytes, path, and for a copy its source), and under `src/` the byte copies named above. Check every file against `SHA256.txt` before the sitting.
