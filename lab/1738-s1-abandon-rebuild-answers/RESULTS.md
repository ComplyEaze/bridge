# Results: #1738 S1. A statement build abandoned unsent, then built again, sent and verified, on the 0.6.1 cut and TallyPrime 7.2, 27 Oct 2026 (UTC)

The sheet is `lab/1738-s1-abandon-rebuild/README.md` at 4dd8fb8de5bc8c609c694f2644996b97c25ae8d7 (SHA256.txt 3 of 3 equal; LAB-COMMON v3, 5370c6d2). The inputs are from `lab/transfer-pairing-tp-inputs` at 9318b08d: `tlab-bank-a.pdf` 2,678 B 7d9eaa09…, `tlab-bank-b.pdf` 3,116 B 1d084035…, `ROWS.md` 8e594bc0…, each equal to the sheet.

## Verdict
**PASS (the sheet ran).** Every call was answered and kept. X1 was never sent. X2 was sent once and verified. August holds exactly two vouchers, the two Contras, beside the seed Journal of 1 April. S0 and S1 are recorded. Every EXPECT line and every prediction the sheet states was met.

| step | what | answer |
|---|---|---|
| 1 | X1 built, never sent | batch X1 `bridge-19aef23b-adde-4c37-8249-58d91b8df811`, 2 Contras, 7,000 Dr = 7,000 Cr; August vouchers 0 |
| 2 | B's skip while only X1 exists | `skips_recorded`, 2 rows; E1-B and E2-B both `open` **other_side_unverified** with X1 |
| 3 | the same rows built again | a NEW batch X2 `bridge-21332db7-304c-488e-858f-8ed0803fb3fc`, **not** refused; E1-B and E2-B each `open` **ambiguous**, once with X1 and once with X2 |
| 4 | X2 sent and verified | `posted_verified`, created 2, errors 0; `verify_import` `posted_verified`; Contra No. 1 (3 Aug, 5,000.00) and No. 2 (4 Aug, 2,000.00); V 1 → 3, M 213 unchanged |
| 5 | B's skip after X2 | `skips_paired` **1** (E1-B, by its reference); E2-B `open` **matched_unconfirmed** with X2 |
| 6 | what X1 is now | `verify_import` X1: **verification_incomplete**, both rows `matching_content_observed`, attribution `not_established`, diffs `narration`; a build from X1's parse: refused **import_txn_already_posted**, `blocking_batch_id` X2, no file |
| 7 | row ids, wider window | 5 A rows over 1 to 31 Aug; E1 and E2 carry p1's ids; all 5 equal `ROWS.md` |
| 8 | the book at the end | the same 2 Contras, no other; M 213, V 3 (equal to step 4); S1 taken |

## The finding for #1738
- **While only the abandoned build exists (step 2)**, the other account's skip of both shared rows reads `other_side_unverified`, naming X1. Its next step says to verify every batch named and not to post the row.
- **An unsent build does not hold its rows (step 3).** Building the same rows again gave a new batch, not `import_txn_already_posted`. Until X2 was sent, both rows read `ambiguous`, with X1 and X2 each listed as a candidate.
- **After X2 is posted and verified (step 5)**, E1-B pairs by its reference (`skips_paired` 1). E2-B, with no reference, stays `open` as `matched_unconfirmed` with X2. Its next step says: "A statement build abandoned unsent, never sent to Tally and never verified, keeps its days uncovered, and no step inside ComplyEaze Bridge clears one yet." This is the blocking S1 would lift.
- **The abandoned batch afterwards (step 6).** `verify_import` gives an answer, not a tool error. It reads `verification_incomplete`, with both of X1's rows `matching_content_observed` and attribution `not_established`: X2's Contras match X1's by content, with only the narration different. A new build of X1's parse is refused `import_txn_already_posted`, naming X2.
- **The row ids are stable.** Parsing the same statement twice (p1, p3) gave the same two ids, with different proposals sha256 (a2eb2dcc… and dde8eb9c…). The full-month window (p7) gave the same ids for E1 and E2. All five A rows and both B rows equal `ROWS.md` (`row-ids.txt`).

## Observed product behaviour: the password-file guard
The first parse (`006-p1.json`) was refused with `statement_password_file_permissions`: "The password file is not private to you … so ComplyEaze Bridge did not read the password and parsed nothing." The empty password file sat in the row's inputs folder. That folder inherits the lab folder's entries, which let other local accounts read the file (Users: read) and change it (Authenticated Users: modify). This is the designed guard working. It refused before reading the password or the statement, and nothing was built or sent.

The guard's remediation was then followed. A new folder was made beside the row folder with inheritance removed, granting full control to the signed-in user and SYSTEM only. A NEW empty file (0 B) was created in it, not moved there. The parse was run again with that file (`007-p1b.json`, the sheet's p1). The first copy was left in place and not used again. The refusal was not part of the sheet's expectations, so the lab stopped and reported it, and resumed only after a decision on how to continue.

## The post (step 4)
Before the post, the lab checked that one tally.exe owned port 9001. The first `post_import` (`014`) waited 40.1 s and answered approval `pending`, `retry_after_s` 15, with dispatch `not_dispatched` (nothing sent). The approval box was fully on screen (`x2-box.png`). The lab answered it Yes with the mouse (synthetic company), and no key was pressed while it could be open. The second `post_import` (`015`) on the same batch was not a re-send. It answered dispatch `posted_verified`, `resent` false, created 2, errors 0, `response_clean`.

## Setting
- TallyPrime 7.2 Silver, licensed. One tally.exe listened on 9001 at step 0 and at the close, and GET /status returned 200 both times.
- `BRIDGE REBUILD LAB` was created new for this sheet by the lab on Tally's screen (synthetic company): company number 100087, GUID f3ca7068-a6cd-4888-9300-4769bab0a058, books from 1-Apr-26, no GST, no inventory, no password. It has the five ledgers the sheet names (no opening balances) and one seed Journal on 1-Apr-26: Dr LAB Seed Expense 1.00, Cr LAB Seed Capital 1.00, "LAB seed". At step 0: M0 213, V0 1. Contra numbering was `auto_retain`. August held no vouchers (`partial`, `empty_uncorroborated`: an empty window answers partial on this build).
- The tool was the 0.6.1 cut, master 67f647e1. `bridge_mcp.exe` sha256 was 6f520338b7f9a25180977c40a5f1d9b03dfa5774df5b5d9a6e65b9c2a050728d before the first call, at step 8 and after the last call (bridge-tally reports 0.6.0). It ran over stdio in one launch with a new empty data folder, terms true, import on and the posting setting on. The retired batch-post setting, redaction and max bytes were absent. The server exited 0 with an empty error stream. The posting setting was removed from the launch file at the end of the row, which returned the file to its hash from before it was added.
- Backups (they stay on the lab PC): S0 `TDBK1800_100087.001` 25,318 B, sha256 0d28cb94b7643ead354e9ce341a6a50a5af67427e082d2457d2f4d5e3a107339; S1 `TDBK1800_100087.001` 26,576 B, sha256 d46feb3ee97b59173a7783748019a80c48f7e67e0134e9c1bffd18dc3bb8f601. The book was left as it ended, not restored. It holds the seed Journal and the two Contras.

## Not measured
The items the sheet lists: two downloads of one period from a real bank; a build imported by hand with posting off; a rejected send; `transfer_double_booked` and `posted_against_skip`; a voucher cancel or delete; more than two rows; TallyPrime 7.1; Gold.

## Files
- `tool-answers/`: every answer whole, as received (000 to 026). `006` is the refused first parse. `014` and `015` are the two `post_import` calls on X2.
- `batches/`: X1's file (1,808 B, sha256 07ea5fc810f9c1a718686ab383bfe79392b8fb6c34723645c95943ddbd20e9a5) and X2's file (1,808 B, sha256 3c6833c541b3b9ad23ab3c0591d7eab879b6e848ac86d38b7c49cdae485c5022), byte for byte. X1's was never sent or imported.
- `proposals/`: the five proposals files the parses wrote, read only.
- `transfers.txt`: the `transfers` section of steps 1 to 6, verbatim.
- `row-ids.txt`: the ids of p1, p2, p3, p5 and p7 beside `ROWS.md`.
- `client-log.txt`: each call with its time, round trip and saved sha256. `notes.txt`: one line per step, in UTC.
- `SCREENSHOTS.txt`: the screen captures and both backups by name, bytes and sha256.
