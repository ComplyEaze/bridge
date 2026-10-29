# Run sheet for 0.7.0: the live write pass on the cut build

Not for merge. LAB-COMMON v3 (C1 to C11, `LAB-COMMON.md` here, a byte copy from `lab/1211-tie-out-rows-run-sheet` at 539f191d) applies in full; where the row and it differ, the stricter holds. One row, `W1-write-pass.md`, on a synthetic company the lab already holds. Every call is made by the built tool, one at a time; no request file is sent by curl, so the request-ID preflight finds no request to check. Record the TallyPrime version, the licence mode and the date.

**WHY.** Release 0.7.0 owes one live pass of what ComplyEaze Bridge writes, on the cut build, before the tag (#2056, step 5): one approved post on a synthetic company, plus the checks that changed since 0.6.1 in what a write says or is checked against. The 0.6.1 control post (R3 step 6, `lab/1714-skip-at-send-answers` at aeae741b) was not completed: its approval window timed out unanswered. This row runs that post again, on the same book and statement, and adds the changed checks:
- #1967 (merged): the `post_import` tool text no longer tells the assistant that two kinds of row are entered in Tally by hand.
- #1995 (merged): the `held_elsewhere` step names both outcomes of sending a batch that holds a skipped row.
- #2047 (#2035): an answer that reported `STATUS` 0 counts no voucher as landed in the rollback floor. Neither case it changes can be produced here: a native post answers with a `<RESPONSE>` root, which reports no status, and for a bound post the floor comes from its binding, not from the answer. The row records the answer's status (step 6) and runs the rollback floor and record on the cut build as a regression check that holds whether or not #2047 is in it (step 7); see NOT COVERED.
- #1896 (merged): a binding refused for the mark's step or the span's count adds a line on another change made during the post. The row shows only that a clean post carries no such line, which 0.6.1 also showed (see NOT COVERED).
- #2030 (merged): no step. Its four checks read dates from Tally's own answers (a Sales invoice's HSN and rate dates, a company context field, the desktop sync), never from a tool argument, and this book has no Sales invoice; see NOT COVERED.

**Marks.** `tally_status` carries no marks. MARKS means `ledger_masters` (first page, no `snapshot_id`): M = `snapshot.master_alter_id`, V = `snapshot.voucher_alter_id`; `snapshot.reused` must read false. Record each answer whole. The stop rules on marks apply to these values.

## WHO ACTS

Steps marked [OWNER] are as `lab/0.7.0-auto-order/ORDER.md` at 6a69df744c73e96db777fac1d04f2d140a4e727c sets out under "WHO ACTS". The run coordinator's word, where the row asks for it (go-ahead, RESTORE, the evidence push), comes from the run coordinator.

## Build (the 0.7.0 cut build; BLOCKING for 0.7.0)

The cut build is the Windows bundle `unsigned-bundle-smoke-windows-latest` of the Bridge CI run on master that holds the last change merged for 0.7.0. The lab downloads it into a new empty folder, checks the zip's sha256 against the artifact's digest, the `.mcpb` against `archive_sha256` and `bridge_mcp.exe` against `binary_sha256` in its `mcpb-smoke.json`, and sends those three hashes to the run coordinator, who fills the line below by a new commit on this branch. Before filling it, the run coordinator checks that the merges of #1967, #1995, #1896 and #2047 are ancestors of RC COMMIT, and writes the result as one BUILD NOTES line; a text check (1967, 1995) whose change is not in the build is recorded "not in the build", never PASS or FAIL. The post (step 6) and the rollback regression (step 7) are judged on any build. While any field below reads `<to fill>`, this row does not run: STOP. Before the first call hash the exe that will run and compare it with the line; any other hash is a STOP. Record the version the tool reports.

RC COMMIT: <to fill>  CI RUN: <to fill>  ARTIFACT: <to fill>  ZIP DIGEST: <to fill>  EXE SHA256: <to fill>

Run it as the lab runs a staged exe over stdio, ONE launch from step 0 to the close (never relaunched in between), with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, ONE new empty `BRIDGE_AGENT_DATA_DIR` for the row (`C:\bridge-lab\070-write-pass\data\`, never a folder an earlier row used), `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use), and `BRIDGE_AGENT_REDACTION`, `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_ENABLE_BATCH_POST` unset. [OWNER] Posting ON: `BRIDGE_AGENT_ENABLE_WRITES=true` in the launch configuration, set before the launch; the lab records at step 0 only that it is present. No other ComplyEaze Bridge may make a call against port 9001 during the row: record the process list at step 0, and record any other `bridge_mcp.exe` that is running (on 27 Oct the desktop app's own entries stayed running and idle: a deviation recorded, not a STOP, if no call is made through them).

**THE APPROVAL WINDOW** opens from the exe's own process and may open behind other windows (look for it on the taskbar). While it is open nobody presses a key on the PC and no tool is called, except step 6's own repeat `post_import` with B. [OWNER] answers it by mouse, within its time. Record the time it was seen open and the time it was answered.

`SHA256.txt` lists every file here.
