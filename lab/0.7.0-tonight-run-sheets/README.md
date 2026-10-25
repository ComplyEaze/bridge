# Run sheets for the lab sitting of 21 Oct 2026 (seven items for 0.7.0 work)

Not for merge. Synthetic BRIDGE companies only, each NEW (named in its sheet); never a client book. The PC's Tally licence lapses on 21 Oct 2026, after which Tally runs in Education mode with day-1/2/31 boundaries, so every row that posts or keys a voucher on an ordinary day can only be run licensed, tonight. Read-only rows lose less.

**Marks (26 Oct 2026).** `tally_status` carries no marks on the builds these rows run (it returns the product, release, licence, Education mode, endpoint, loaded companies and the date). Wherever a row below says "`tally_status`: Mx, Vx" (or reads a mark from it), call `tally_status` as written and ALSO `ledger_masters` (first page, no `snapshot_id`), and take the marks from its `snapshot.master_alter_id` (M) and `snapshot.voucher_alter_id` (V); record both answers whole and `snapshot.reused`, which must read false. A book that holds no voucher may give no V: record it. The stop rules on marks apply to these values.

Standing rules for every row: one request at a time and no retry; Gateway of Tally with no dialog before every call; every answer saved whole with its egress line and the data-folder snapshot; marks and counts taken structurally, never by substring, and a failed request reported as failed (never as "0 rows"); backups before any keying and a restore only on the run coordinator's explicit word; STOP at any dialog, any answer outside the stated shape, or any mark that moves where a row says it must not; skip, never improvise. The owner types Terms and posting values; the lab never does.

## Status on 26 Oct 2026 (this section wins over the rest of this file where they differ)
- Done on 21 Oct (TallyPrime 7.1): B2, B3 and B4 (with its steps b and c), evidence on `lab/0.7.0-tonight-answers`. Do not run them again.
- B5 is replaced by `lab/1039-live-rows` (G1). Do not run B5.
- A1 is NOT READY. On the build below `parse_bank_statement` reads text PDF statements only (sbi, hdfc, ubi, bob; no CSV route reaches the tools), and the "don't know" answer exists only for a cash line: an SBI "ATM WDL" withdrawal (a Payment, bank to the suspense ledger) or a UBI or BOB "BY CASH" deposit. None of the committed synthetic statement PDFs has such a line, and every balance and total of a statement must reproduce exactly. A1 runs only after a synthetic statement PDF with one such line, its figures and its `parse_bank_statement` and `build_import_xml` calls are added to this folder. Nothing is improvised.
- A2 and A3 are READY, on the build below. The licence note above is out of date: the lab runs TallyPrime 7.2, licensed (Silver), not Education mode; record the release and licence at the top of each row.
- Each row posts through the tool: its approval window opens from the exe's own process and can open behind other windows without coming to the front. A person watches for it and answers it by mouse; nothing is typed while it is open. Run A2 and A3 only while that person is at the PC.

## Build (26 Oct 2026)
The build already staged at the lab as `w1-2306d86d`: the Windows bundle of master `2306d86dfd4c86a3c5f6c19ea9ddff6818c460fb`, Bridge CI run `40077056130` (event `schedule`, conclusion `success`), artifact `12553782422` `unsigned-bundle-smoke-windows-latest` (60,880,528 bytes, zip digest `sha256:74ae2daa851397b71cb59e80373d9478a2da72c74398108e33e34ea83e56eb1e`). Its `bridge_mcp.exe` as the lab recorded it on staging: sha256 `92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524`. Before the first call hash the exe that will run; any other hash is a STOP.

Run it over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, a NEW empty `BRIDGE_AGENT_DATA_DIR` for each row (kept for the whole row, so the batch journal carries from build to post to verification), `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use), `BRIDGE_AGENT_ENABLE_WRITES=true` (posting ON) and `BRIDGE_AGENT_REDACTION` unset. "MODE: B or A" and "the owner types" in the rows below mean this setting.

## Order (about 2.5 hours; cut from the bottom if short)
| # | Sheet | Company (new) | Issue | Writes |
|---|---|---|---|---|
| 1 | `B2-1709-s3-empty-book-marks.md` | BRIDGE EMPTY LAB | #1709 | none |
| 2 | `B3-239-regroup-and-move-group-marks.md` | BRIDGE REGROUP LAB | #239 | two regroups by hand, restored |
| 3 | `A1-1761-s2-tagged-narration-readback.md` | BRIDGE TAG LAB | #1761 | 1 native post |
| 4 | `B5-1039-s1-refused-binding.md` | BRIDGE BINDING LAB | #1039 | 3 posts and a hand edit |
| 5 | `A2-1038-s2-repost-after-restore.md` | BRIDGE REPOST LAB | #1038 | 4 posts around a restore |
| 6 | `B4-1423-s2-class-entry-purchase.md` | BRIDGE CLASS PURCHASE LAB | #1423 | 1 hand-keyed purchase |
| 7 | `A3-1738-s2-find-a-rejected-native-post.md` | BRIDGE REJECT LAB | #1738 | up to 4 post tries |

Restore each company at its row's end, before the next row. Not in this set: two further sheets (#485 and #1231) wait for a request file derived from a committed capture and reviewed before it is sent.

`SHA256.txt` lists every file here. Check it before the sitting starts.
