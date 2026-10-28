# #1356: the `outstandings` party detail read by its ledger, on a large book, through the pull request's build

Not for merge. READS ONLY, through the tools of one build, on one company the lab already holds. `LAB-COMMON.md` in this folder (v3, a byte copy from `lab/1356-ledger-filtered-allocations-run-sheet` at fd02e22f) applies in full, with THE COMPANY below. Where this sheet and it differ, the stricter holds. No request is sent by curl in this item: every read is a tool call of BUILD-T.

**WHY.** The pull request for #1356 changes the `outstandings` party detail only where it is refused today. The detail reads every company voucher from the start of the books to `as_of`, and it refuses when that window needs more requests than one call may spend (about 5,376 vouchers). With the change, after that refusal, when the party's ledger name holds only letters, digits and single spaces and no TDL operator word, it does this:
1. It counts the party's vouchers by a ledger-scoped census, in AlterID spans.
2. It reads them by ledger-scoped reads, in date parts the planner sets from that count (42 vouchers a part at its first figure), each admitted against the census voucher by voucher.
3. If that read finds nothing, is not admitted, or would need one day divided by AlterID, the company window's refusal stands.

Every window the company read answers is answered as before; nothing in this item tests that.

Both request shapes were captured by curl. The tests replay them byte for byte. This item is the first run of the whole path through the build, on a live book. It is also the first read of a party that holds a small share of its window: about one voucher in 120 of this book. The captures of 28 Oct were all of windows in which every voucher carried the ledger.

**THE COMPANY:** BRIDGE CORPUS DENSE, GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, synthetic, about 30,000 vouchers. STOP if it is absent from Select Company, or if `list_companies` does not name exactly this GUID. On 28 Oct its voucher mark (ALTVCHID) read 29900 and its master mark (ALTMSTID) 328.

**THE PARTY:** `DN Party 001`. By the ledger census captured on 28 Oct (#485), it holds 61 vouchers from 1 April 2025 to 30 June 2025, and 248 to 31 March 2026. Its first voucher is dated 2 April 2025. The company holds about 82 vouchers a day, so its window to 30 June 2025 is about 7,400 vouchers, past what one call may spend.

## Build and inputs (hash before the first call and at the end; a different hash = STOP)
| item | source | bytes | sha256 |
|---|---|---|---|
| `bridge-tally.mcpb`, the bundle BUILD-T comes in | #2115 at head 73289a9b094a570e7d70d65dbaa4f90ec04a9486: CI run 40758082390, artifact 12846649341 `unsigned-bundle-smoke-windows-latest` (expires 4 Nov 2026 20:04 UTC), file `bridge-tally.mcpb`; its sha256 equals the artifact's `mcpb-smoke.json` `archive_sha256` | 15,355,579 | e500bf153df74c73ce79a91b7457a847930fbbf02da016eca3d715c32201559f |
| `bridge_mcp.exe`, BUILD-T | `bin/x86_64-pc-windows-msvc/bridge_mcp.exe` inside that `.mcpb`; its sha256 equals `mcpb-smoke.json` `binary_sha256`, and its server reports version 0.6.1 | 31,879,680 | 7a3c07f01bcfccff2f44e1547709260632ccebac974ceddf67a9f81cf34ff556 |

- Posting stays OFF for this launch: `BRIDGE_AGENT_ENABLE_WRITES` absent or `false`. Record the launch file's key names (not values) and its sha256.
- Working folder: `C:\bridge-lab\1356b\`. Evidence goes in `C:\bridge-lab\1356b\evidence\`.
- Check every file here against `SHA256.txt` before step P0.

## Before the sitting
- P0. Tally's own Select Company list. If BRIDGE CORPUS DENSE is not there: STOP. Otherwise load it alone (C1), then C2 step 0.
- P1. Start BUILD-T. Then:
  - `tally_status` and `list_companies`: EXPECT BUILD-T, and THE COMPANY as the one company with its GUID. Record its `books_from`. If it is not `20250401`, the counts in D1 to D3 may differ: record that, and go on.
  - `ledger_masters` (basic): record both marks. Voucher mark below 29900 = STOP (not the book of 28 Oct). At 29900 or above, go on, recording it.
  - Every later compare is against these marks.

## The calls (one at a time; a call in flight is never cancelled or repeated)
Each call is `outstandings` with `company_guid` THE COMPANY's GUID, `party` `DN Party 001`, and the `as_of` and `detail` given. Keep each answer whole as a file, with its size and sha256. A call that does not answer in 15 minutes: STOP, leave it, report (C9).

- D1. `as_of` `20250630`, `detail` `bill_trail`.
  - EXPECT a result, not a refusal: the company window is refused before any data request (its census shows it), and the party is then read by its ledger.
  - EXPECT `detail.window` to hold `party_vouchers_read` 61, and no `company_vouchers_read`.
  - EXPECT `detail.ledger_match.matched` `exact`.
  - Record `detail.state`, and for each bill its `state`. A bill whose `state` is `tied` has its signed allocations equal to Tally's own balance for that bill: the voucher read and Tally's bills report agree on it.
  - Any bill `trail_does_not_tie`: record both numbers. That is the finding, not a STOP.
  - A refusal, or `company_vouchers_read`, is the finding, not a STOP: keep it whole, and record its code and `reads`.
- D2. `as_of` `20250630`, `detail` `unadjusted`.
  - EXPECT `party_vouchers_read` 61.
  - Record `detail.state`, `residual`, `on_account_sum` and the row count.
- D3. `as_of` `20260331`, `detail` `bill_trail`.
  - EXPECT a result: 248 vouchers, read by the ledger in several date parts.
  - EXPECT `party_vouchers_read` 248, and no `company_vouchers_read`.
  - Record `detail.state` and each bill's `state`, as in D1. A refusal is the finding, not a STOP: keep it whole, with its code and `reads`.
- D4. `egress_log` (the newest 200). Keep it whole. From it, for each of D1 to D3, count the reads by their request fingerprints between the call's start and end. Record the counts only.
- D5. `ledger_masters` (basic) again. EXPECT the marks of P1, since the calls write nothing. A difference = record it and STOP.
- D6. Optional, on the run coordinator's word: Tally's own Ledger Outstandings screen for `DN Party 001` as on 30 June 2025. Screenshot it, and record each bill's name and pending amount beside D1's.

## The book
It does not change; every call is a read. The marks are read at the start and the end, and must be equal.

## Close and evidence
- C10, then stop BUILD-T.
- C11: evidence to a new branch `lab/1356-party-detail-by-ledger-answers`, after the pre-push scan and the run coordinator's yes. It holds:
  - the build line (pull request, head, CI run, artifact, exe sha256);
  - the launch file's key names and sha256;
  - each answer whole, with its size, sha256 and UTC call times;
  - the D4 counts;
  - the notes, one line per step.

  Screenshots stay on the lab PC, listed by name, size and sha256.

## Not covered
- The bytes of the requests and answers. No relay sits in between, so no fixture comes from this item. The curl captures of 28 Oct are the byte record, and the tests replay them.
- A party name with any other character. It takes the old path by design.
