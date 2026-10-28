# Run sheet for #1039: reproduce one refused span binding on a single-user book

Not for merge. LAB-COMMON v3 (C1 to C11, `LAB-COMMON.md` here, a byte copy from `lab/1211-tie-out-rows-run-sheet` at 539f191d) applies in full; where the row and it differ, the stricter holds, except for the one named deviation D1 in `R1-1039-refusal.md`. One row, `R1-1039-refusal.md`, run beside G1 row A of `lab/1039-live-rows` (the 0.7.0 order, row 17), on the same synthetic company. Every call is made by the built tool, one at a time; no request file is sent by curl, so the request-ID preflight finds no request to check. Record the TallyPrime version, the edition, the licence mode and the date.

**WHY.** A refused span binding has never been observed live. #1039's next build (a person's "reviewed, not attributed" record) may merge only after a live refused batch has been shown in its dialog (AGENTS.md P1). This row makes one on purpose and keeps it.

**How a refusal arises (read from the code, not measured).** Read at master 565ffff5. The builds this row may run on (those G1 names) differ from it in these paths only by #1896 (a summary line) and #2047 (the rollback floor), neither of which touches the step check.
- After the person approves, the post reads the company's voucher mark (the aim read, the last request before the POST), sends the POST, and reads the mark again just after it. Its own readback then binds the batch to the AlterIDs between the two marks.
- A voucher saved in Tally between those two reads makes the step one more than Tally's `CREATED`, and the binding is refused as `span_step_not_created`, for good.
- A voucher saved before the aim read only moves the mark the post starts from. One saved after the second read lies above the post's span. Neither refuses.
- So the row makes the window long (a batch of 50 vouchers, the most a post sends) and saves one prepared Payment on Tally's screen a set time after the approval. The saved voucher's own AlterID then says whether it landed before, inside or after the post's span, and the next attempt's delay is corrected from that.
- No second request is sent to Tally beside ComplyEaze Bridge's own: the second writer is Tally's screen. Sending while a voucher sits unsaved on that screen is a named deviation (D1 in `R1-1039-refusal.md`), run only on the run coordinator's recorded yes.

**Marks.** `tally_status` carries no marks. MARKS means `ledger_masters` (first page, no `snapshot_id`): M = `snapshot.master_alter_id`, V = `snapshot.voucher_alter_id`; `snapshot.reused` must read false. Record each answer whole.

## WHO ACTS

Steps marked [OWNER] are as `lab/0.7.0-auto-order/ORDER.md` at 6a69df744c73e96db777fac1d04f2d140a4e727c sets out under "WHO ACTS". The run coordinator's word, where the row asks for it (go-ahead, RESTORE, the evidence push), comes from the run coordinator.

## Build

The build G1 row A ran on, as `lab/1039-live-rows` README names it and the row A notes record (commit and exe sha256). Hash the exe before the first call and at the end; any other hash is a STOP. Record the version the tool reports. Run it over stdio, ONE launch for the row, with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, ONE new empty `BRIDGE_AGENT_DATA_DIR` for this row (`C:\bridge-lab\1039-refusal\data\`, never G1's folder), `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use), and `BRIDGE_AGENT_REDACTION`, `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_ENABLE_BATCH_POST` unset. [OWNER] Posting ON: `BRIDGE_AGENT_ENABLE_WRITES=true` in the launch configuration, set before the launch; the lab records at step 0 only that it is present.

**THE APPROVAL WINDOW** opens from the exe's own process and may open behind other windows (look for it on the taskbar). While it is open nobody presses a key on the PC and no tool is called, except the attempt's own repeat `post_import`. [OWNER] answers it by mouse.

**The data folder is kept.** The refused batch, its data folder and the book are the input of the later dialog row for #1039: nothing in this row restores the book or clears the folder unless the run coordinator says RESTORE.

**Row R2**, `R2-1039-review-dialog.md`, runs after R1 on R1's kept batch, data folder and book, on a later build that its own section names (not the build above). It shows the refused batch in the content review dialog and records the review; it writes nothing to Tally.

`SHA256.txt` lists every file here.
