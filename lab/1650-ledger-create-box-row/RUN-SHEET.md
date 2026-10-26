# 1650 box row (#1650, Part of #690 and #1780): the one-ledger approval box, with its screen-fit check, shown whole and answered on a real Windows screen

Not for merge. Written 26 Oct 2026 from the pull request body of the box-fit change stacked on #1650 (branch `fix/1780-ledger-create-box-fit-check`), whose "Not measured" line asks for exactly this: "a one-ledger box shown and answered normally (the fit check lets it through), and the box under a 1366x768 screen at 150% to confirm it still fits". `SHA256.txt` lists every file here with its bytes; check it before step 0.

On that build the Windows one-ledger box (`show_create_ledger`) is measured when it opens and cancelled if it is not wholly on the screen, as the post and review boxes are (#1879); a box the check cannot measure answers `master_approval_unavailable` and a box that does not fit answers `master_review_too_large`, each with nothing sent. This row checks that a box that fits is shown and works: declined once, accepted once, one ledger created.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6.

THE COMPANY is `BRIDGE LEDGERBOX LAB`. It does not exist yet: step 1 creates it. If a company of this name exists on the PC, STOP and report it.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **No raw requests.** The operator sends no request file. Every read and write is a call of ComplyEaze Bridge's built tool, one at a time. C2 step 0 runs WITHOUT its MARKS request; its GET /status checks still run.
2. **Marks.** `tally_status` carries no marks. M (ALTMSTID) and V (ALTVCHID) are read with `ledger_masters` (first page, no `snapshot_id`): `snapshot.master_alter_id` and `snapshot.voucher_alter_id`; record the whole answer and `snapshot.reused`, which must read false. A book with no voucher may give no V: record it.
3. **Backup.** One backup, BK1, of THE COMPANY at step 2, before the first ledger write (C5), into the new empty folder `C:\bridge-lab\1650box\backups\bk1\`. No restore is planned: THE COMPANY is this row's own and is kept, shut, at the end.
4. **Who acts.** The operator creates THE COMPANY (step 1) and makes the tool calls. The OWNER answers the approval box by mouse in steps 5 and 6. On this build the box opens on top of every window and takes the keyboard focus, so a key typed anywhere on the PC (in Tally, in the chat) can answer it: from the moment a `post_ledger_creates` call is made until its answer arrives, nobody presses a key on the PC. Its buttons are Yes, No and Cancel; No is the default; Cancel and Esc decline as No does. The box closes by itself 120 seconds after it opens, with nothing sent. This row runs only while the owner is at the PC.
5. **Grant.** The creation of THE COMPANY on Tally's screen (step 1) and one ledger created by ComplyEaze Bridge (`post_ledger_creates`, step 6); nothing else written. Tool calls: about 15, one at a time.

Licensed mode only; Education mode = STOP.

## Build
The `bridge_mcp` program built on the lab PC from `fix/1780-ledger-create-box-fit-check` at `6f7e98bc068302b3a13386471b2d7d07758a76c7` (the box-fit change on #1650's head `a71f416c9f94254e9786818965c93f37c5fb7863`); no CI run builds that branch's bundle. In a clean checkout of exactly that commit: `cargo build --locked --release --manifest-path src-tauri/Cargo.toml -p bridge --bin bridge_mcp`. Record the commit (`git rev-parse HEAD`) and the sha256 of `src-tauri\target\release\bridge_mcp.exe` before the first call. If the branch's head is no longer `6f7e98bc` when the row starts, STOP before the first call and ask the run coordinator which commit to build.

Run it over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, ONE new empty `BRIDGE_AGENT_DATA_DIR` for the row (the masters journal must carry from the prepare to the posts), `BRIDGE_TERMS_ACCEPTED=true` (the owner allowed the lab to set it on 25 Oct 2026, the person responsible for the lab accepting the Terms of Use) and `BRIDGE_AGENT_REDACTION` unset. POSTING and LEDGER CREATION are the owner's acts: the owner sets `BRIDGE_AGENT_ENABLE_WRITES=true` and `BRIDGE_AGENT_ENABLE_MASTER_WRITES=true` in the launch configuration himself; the lab never sets, writes or copies them, and records only that they are present. `BRIDGE_AGENT_ENABLE_BATCH_POST` must be absent. Exactly one ComplyEaze Bridge with ledger creation on may be running; record the process list.

## The ledger
Name `LBox Suspense`, kind `suspense` (under Suspense A/c). No ledger, alias or group of THE COMPANY may carry that name, or one that differs from it only in case, spaces or symbols, before step 6 (a new company has none).

## Steps
0. LAB-COMMON step 0 with no company loaded, without its MARKS read. Tally's About screen: record the release (7.2 expected). EXPECT no company named `BRIDGE LEDGERBOX LAB` in Select Company's list. Record the screen's resolution and scale (Settings, Display) and the work area: this row runs on the PC's own display setting; change nothing.
1. Create Company `BRIDGE LEDGERBOX LAB`: financial year and books beginning 1-4-2025, no password, every other field as the screen offers; if the Features screen opens, accept it unchanged and record every value it shows (on 7.2 it has been seen to open with GST already Yes). Screenshot before accepting. Load it alone. `list_companies`: EXPECT exactly THE COMPANY with `identity_state` `verified_tuple`; record its `guid` (the `company_guid` of every later call) and the company number. `tally_status`: record release, licence and Education mode. `ledger_masters`: M0, V0, and the ledger rows (EXPECT no `LBox Suspense`).
2. Backup BK1 (C5) into the folder above: its name, size and sha256. `ledger_masters`: M0′ (a mark that moved across the backup is recorded, as C5 says, not a STOP). Every later comparison is against M0′.
3. `prepare_ledger_creates` with `company_guid` and `ledger` = `{"name": "LBox Suspense", "kind": "suspense"}`. EXPECT a `batch_id` and a `preview` of nine lines naming THE COMPANY and the ledger. Record the answer whole.
4. `ledger_masters`: EXPECT M = M0′ (a prepare writes nothing to Tally).
5. **Decline.** `post_ledger_creates` with the `batch_id`. A box titled `ComplyEaze Bridge — create one ledger?` opens. The OWNER: confirms by eye that the WHOLE box is on the screen (its title bar, every line of its text and its buttons), that its text is the answer line ("Press Y (or click Yes) to create this ledger in Tally ...", wrapped at 100 characters), a blank line, then step 3's nine preview lines unchanged, and its three buttons; a screenshot of it if the lab's screenshots show it (else the owner's words, recorded verbatim); then presses **No**. A call waits about 40 seconds for the answer: if it answers `approval.state` `pending` (the check took longer), the owner still presses No, and `post_ledger_creates` is called once more with the same `batch_id`. EXPECT the final answer to refuse `master_approval_declined` with nothing sent (`attempt_recorded` false). An answer `master_review_too_large` or `master_approval_unavailable` while the box was whole on the screen, or no box at all, is the FINDING of this row: record it whole and STOP (nothing was sent). `ledger_masters`: M = M0′.
6. **Accept.** `post_ledger_creates` with the SAME `batch_id`. The box opens again: the OWNER checks it as in step 5 and presses **Yes** within its 120 seconds. If an answer says the approval is `pending` or `approved` without a send, call `post_ledger_creates` again with the same `batch_id` after its `retry_after_s` (the tool's own flow, not a retry). EXPECT `state` `posted_verified`, `attempt_recorded` true, `dispatch.state` `dispatched`, `counters` `created` 1 and every other counter 0. Record every answer whole. Then a screenshot of the whole Tally window (C3; the read-back runs inside the call). Any other final state: record it whole, do NOT call again, do NOT key the ledger by hand, and STOP.
7. `ledger_masters`: M1, V1 and the ledger rows. EXPECT M1 = M0′ + 1 and one new row, `LBox Suspense` under `Suspense A/c`, its GUID beginning with THE COMPANY's GUID; every other row unchanged.
8. `egress_log`: record it whole (each call's `response_prepared` line).
9. LAB-COMMON C10: shut THE COMPANY; it is kept as it is.

### Not in this row: the box on a 1366x768 screen at 150 percent
It needs a second ledger (a grant of its own), a display change the owner allows, and Tally and the program restarted after the sign-out Windows asks for. It is a later row.

STOP RULES: C7 and C8; any dialog before a call; a box other than the one named; two such boxes at once; `master_approval_timed_out`; any answer outside the stated shape; a mark that moves outside the steps that change it.

## Not measured by this row
A box that does not fit (its about twelve short lines fit a screen of this size; the fit refusal is covered by the branch's tests); the box at 1366x768 and 150 percent; a box when the fit check cannot measure it; the macOS box; a name too long for its dialog line; Gold.

## Evidence
Every tool answer whole with its size and sha256, the build's commit and exe sha256, the display setting before and after, the owner's words for each box, BK1's name, size and sha256, screenshots by name, size and sha256 (never pushed), and a notes file with UTC times and one line per step. THE COMPANY's GUID is new: before the evidence is pushed it goes into the merge gate's synthetic-UUID registry, or the evidence shows it as `<company guid>`. Evidence on a new branch `lab/1650-ledger-create-box-row-answers` with `SHA256.txt`; send the SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.
