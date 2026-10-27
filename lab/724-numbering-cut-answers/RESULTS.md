# Results: #724 N1 numbering rows on the 0.6.1 cut build 67f647e1 and TallyPrime 7.2, 27 Oct 2026. Row 1 PASS, row 2 PASS, with one Tally finding at step 6

The sheet is `lab/724-numbering-rows/N1-724-numbering-admission-rows.md` and `README.md` at d258aed7.

## Build and setting
- The cut build: RC commit 67f647e1a292e8d7815053b5c08289cba6402795, CI run 40456677783, artifact 12712717489; `bridge_mcp.exe` sha256 6f520338b7f9a25180977c40a5f1d9b03dfa5774df5b5d9a6e65b9c2a050728d (= the sheet's line) at the launch. It reports `bridge-tally` 0.6.0, as the sheet's build notes expect for this cut; `tools/list` holds 33 tools with posting on.
- One launch over MCP stdio for the whole row pair, 09:01:32 to 09:24:33 UTC (`client-log.txt`), with `BRIDGE_TERMS_ACCEPTED`, host 127.0.0.1, port 9001, a new empty data folder and `BRIDGE_AGENT_ENABLE_WRITES`; `_ENABLE_BATCH_POST`, `_MAX_BYTES` and `_REDACTION` absent. The server's error stream was kept and stayed empty.
- TallyPrime 7.2, Silver, not Educational mode (`tally_status`). The book: BRIDGE NUMBERADM LAB (company number 100078, GUID 7f9b1bc0-f300-4265-81ef-4282254174e9), loaded alone.
- R0: backup BK1b of 26 Oct (`TDBK1800_100078.001`, 23,411 bytes, sha256 48edd7e348c59a7415dd786a979d962209f77381f1a07ba7aa82504db8ccf6aa, hashed before each restore: equal) restored over the book. It stays on the lab PC.

## R0 and step 1
- After the restore: one company; marks M 207 and V null (no voucher); no voucher in FY 2025-26.
- Two ledgers created on Tally's screen: BRIDGE Num Customer (Sundry Debtors, "Maintain balances bill-by-bill" No) and BRIDGE Lab Bank (Bank Accounts). M 209, V null.
- Receipt type as found: `series_numbering` method `automatic`, sub_method `auto_retain`, duplicates `not_prevented`.
- Step 1b: `build_import_xml` of one Receipt (1.00, 10 Apr 2025, no number) on the book with no voucher: tool error `empty_book_first_import`, whose remediation states the cause and that nothing was sent to Tally, with no next step. V still null, no voucher in April.

## Row 1: an approved post into Automatic with Retain
- Step 2: two Receipts keyed on Tally's screen (1.00 each, BRIDGE Num Customer to BRIDGE Lab Bank): Tally numbered them 1 (20 Apr 2025) and 2 (25 Apr 2025).
- Step 4: B = 2. The Receipt of step 1b built (batch bridge-4e2976c3-8826-44ae-9ebf-e2125c646c0e) and posted. The first call (018) answered `pending` after 40 s with the window open; the second (019) answered `posted_verified`, `post_location.state` `target_only`, `target_voucher_step` before 2 after 3 step 1, `matches_created` true, no `after_read_failure`. The window was answered with the Y key.
- The approval window, captured once while open with no key or click (`s4-approval-window-dpi.png`, 1920x1080): titled "ComplyEaze Bridge — post this voucher?", above Tally, first line "Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to cancel. Nothing is posted until you answer."
- Step 5: `verify_import` `posted_verified`, `alter_id_delta` from `pre_post_mark` before 2 after 3, `post_span_binding` `bound`; no `rollback_recorded` and no `voucher_mark_contradiction`. M1 209, V1 3.

| voucher | date | MasterID | number before | number after |
|---|---|---|---|---|
| keyed | 20 Apr 2025 | 1 | 1 | 1 |
| keyed | 25 Apr 2025 | 2 | 2 | 2 |
| posted | 10 Apr 2025 | 3 | — | 3 |

The posted voucher took the next number in sequence; no earlier voucher's number changed. **Row 1 PASS.**

After the post, Tally showed the Gateway's Create list with nothing entered; it was closed with Escape and the marks were unchanged.

## Row 2: a build into a Manual type is refused
- Step 6: Receipt altered on Tally's screen (Alter > Voucher Type > Receipt) from Automatic to Manual. Tally asked two things the sheet did not name: "Changing the method of voucher numbering will affect voucher numbers of existing transactions. Continue?" (Yes), then on the save "Are you sure you want to update the voucher numbers of existing vouchers?" with "Y : Accept Master & Update Vouchers" and "N : Back" (Y). The saved screen reads Manual, "Prevent creating duplicate Voucher Nos." No.
- Step 7: `series_numbering` method `manual`, no sub_method. The baseline was read again after the alteration (V1' 3, M1' 211) and steps 8 and 9 are compared with it.
- Step 8: `build_import_xml` of one Receipt (1.00, 12 Apr 2025, no number): an ordinary answer, `result.state` `refused`, `reason` `import_voucher_numbering_not_automatic`, `refused_types` Receipt observed `manual`, `batch_id` null, `next_step` recorded whole. V 3 = V1', the same three vouchers.
- Step 9: Receipt set back to Automatic with "Retain Original Voucher No." (the same two prompts, Y): `automatic` / `auto_retain` read back. V2 3 = V1'; M2 213.

**Row 2 PASS.**

## Finding (Tally 7.2, step 6 and step 9)
"Accept Master & Update Vouchers" on the change to Manual emptied the voucher number of all three Receipts (each reads `voucher_number` ""; Tally's Day Book shows an empty Vch No.). Each voucher's AlterID stayed the same (3, 1, 2) and the voucher mark V stayed 3; only the master mark moved (209 to 211). Setting the type back to Automatic with Retain, with the same answer, put the numbers 3, 1 and 2 back, again without a change in any AlterID or in V (M 213). A change of voucher numbers made through the voucher type's screen is therefore not seen in the voucher mark.

## AFTER
BK1b restored over the book (Replace). One company; M 207, V null; no voucher in FY 2025-26: the book of R0.

## Files
- `answers/NNN-*.json`: every call's answer whole; each sha256 is in `client-log.txt` and `SHA256.txt`. The `egress_log` answers (013, 014, 021, 031) are recorded as data.
- `notes.txt`: one line per step with UTC times.
- Kept on the lab PC, by name, bytes and sha256: r0-ledger-bank.jpg 105426 31106fce1636ad95854785f12ea0bf5356c27864ca591901a8032c2bb84e3c45; r0-ledger-customer.jpg 110490 cc19113a6fa7297a354d6d1ef4f18b24a8d08b1cf3504820c26cf11a5c8093df; s2-receipt-1-filled.jpg 103948 cec457e7aadc5e5c6ecda7942678a907946e9bdd305f698f5889562f9ded0b5c; s2-receipt-2-filled.jpg 105340 0595cad29260c4899b4dbb64e73a8ed8c50ac52caac67e46d46f269e14ad5427; s4-approval-window-dpi.png 405636 5e22ce795dbaa1dffc58c8f5900bc2a7967cb5cdf81b0fb311d92b8c6f4b654d; s5-after-post-tally-screen.jpg 25898 e81b986eda7ccd457fd0b2faa717775d2b77341914728f0e817edeb79e1cddde; s6-receipt-type-before.jpg 123368 e123634e2595c2fa1282ad754216151ae76a6b7ea709d60826e88acd7fd9aa39; s6-prompt-continue.jpg 4426 d988d3432eff57bd4cc55f478e03d06d4c61522042dc6391b3be788ffa01d646; s6-manual-set-unsaved.jpg 115588 7cdb3ae663728439ccefaf67b12598ed64d4e02a0c5897af95a99b0650250aa7; s6-prompt-update-vouchers.jpg 117096 4791dc3b9635f10b577d8c50fcdc36c68992e9fe109dc9f6110e02f205fceb55; s6-after-update-list.jpg 96601 26e9fa466d81db22ecdbf1372d77c59651c6de13d6a291af7ce9eafe63ec4d73; s6-receipt-type-saved-manual.jpg 116636 0d822f98764f410b602b56604a06ebf4cb4ec8ff0b798f84ae459b93129806e3; s6-daybook-25apr-blank-number.jpg 109422 6c6056f212264c99fe0ef41643c4112c412efb3885451cd05e48e9a6538cc4d3; s9-prompt-continue.jpg 26027 0bd36978e9d777f08131e0fa9495cbf8d0f1c8ed0561759496a164397dd02b60; s9-numbering-behaviours.jpg 123205 4abad98f451d35359c5cc77db7f537aad2cbc33e8d6b8f063d8bf9c2eb2665e7; s9-prompt-update-vouchers.jpg 121027 b04717ca103add94584afa0bafc5d291ccae952a1baf696da1a6abafe0249c44; s9-receipt-type-saved-automatic.jpg 120450 3f41d9e39e9f843882c4115853a7f1a1a1a926d96cc793773019b56b123cad3a; BK1b (above); the data folder.
