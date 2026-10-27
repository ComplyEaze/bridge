# Windows item GS2-L3, rerun of steps 4 to 6 (27 Oct 2026). SYNTHETIC ONLY: THE COMPANY is `BRIDGE FIRST VOUCHER LAB`, made by the first sitting of this item. READ-ONLY: nothing is keyed, posted or imported. `WINDOWS-GS2-L3.md` and LAB-COMMON v3 apply in full except where this sheet says otherwise; where they differ, the stricter holds.

Why: the first sitting stopped at step 4. `purchase_register` found the April 2025 window empty and, as it does for any empty voucher window, read it again widened by one day on each side to corroborate it (`widened_window` in `src-tauri/src/agent.rs`, called through `corroborate_empty_voucher_read` from `src-tauri/src/agent_register.rs`, at the build's commit `796e94f18e5af783b291ddf3b5e60ec19132df9a`). That read asks for 31 March to 1 May 2025. The relay at `e5dffc2b` admitted dates from 1 April 2025 only, refused it as `date_out_of_range`, and the tool answered `agent_runtime_read_failed`. The first sitting's recording folder and answers are its own evidence and stay as they are.

## One variable
The relay only: `lab/gs1-relay` at `300ea3c3d51c0ba95be761a8982ad65fa80678b6`, whose `FIRST_DATE` is `20250331` (`gs1_relay.py` 34,437 bytes, sha256 `40a9c57cf494d9f7101e5f83a1802c359b66f8d908c6999ca94e095b22cbb9a0`). The build, THE COMPANY, the arguments and the steps are the first sitting's.

## The build, the relay and the calls
- The same build as the first sitting, the lab config's line `bridge-lab-gs1` (`bridge_mcp.exe` sha256 `bf8cdf82e10bf22f693d3c5d6d89bd074b35b2dbf6dc2cbad748ac8609916f46`; hash it before the first call and after the last; another hash = STOP). Posting and ledger creation OFF. One call at a time; each answer kept whole with its size and sha256; a call in flight is never cancelled, abandoned or repeated.
- The relay at `300ea3c3`: check every file against its `SHA256.txt`, then `python -I gs1_relay.py --self-test` and EXPECT `self-test: ok (11 forwarded, 42 refused, none of the refused reached the target)` and exit 0; anything else = do not use it, STOP. It records into a NEW folder `C:\bridge-lab\gs2-l3\relay-rerun\` (never the first sitting's folder). A request it refuses = the call fails = STOP, folder kept.

## Who acts
The lab does every step itself, on the owner's standing word of 27 Oct 2026. Nothing here is keyed: the only screen steps are loading and shutting THE COMPANY.

## Steps
R0. LAB-COMMON step 0 with no company loaded, without its MARKS read (as the first sitting). Select Company's list: EXPECT `BRIDGE FIRST VOUCHER LAB` present (absent = STOP). Load it alone; screenshot the Gateway. Tally's About screen: record the release.
R3. Relay on (as above). Call `list_companies`. EXPECT one company, THE COMPANY, with the `company_guid` the first sitting recorded (another GUID = STOP).
R4. Call `purchase_register` with `company_guid`, `from` 2025-04-01, `to` 2025-04-30. RECORD the answer whole (`state` complete or a refusal; either is a result, a refusal recorded with its code, not a STOP). In the relay's `manifest.tsv`, record the line of the read whose dates are 20250331 to 20250501: EXPECT `forwarded`.
R5. Call `purchase_register` again with the same arguments.
R6. Shut THE COMPANY, load it again alone, screenshot the Gateway, and call `purchase_register` a third time with the same arguments. Relay off (Ctrl+C), the config's port back to 9001, the server restarted.
R7. LAB-COMMON C10. THE COMPANY is kept, shut, as it is.

STOP also if any answer shows a voucher in THE COMPANY: the item reads a company that has never held one.

## What is recorded (nothing is judged at the lab)
As `WINDOWS-GS2-L3.md` "What is recorded", from the new folder: the whole folder with `manifest.tsv` and its SHA256 list, every company-marks, book-extent and voucher-window answer of R4 to R6, the four tool answers whole, the release, screenshots by name, size and sha256 (never pushed), and a notes file with UTC times. Evidence on a NEW branch `lab/1709-first-voucher-rerun-answers`, after the pre-push scan and the run coordinator's yes.
