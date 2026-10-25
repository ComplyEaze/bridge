# Run sheets for lab sitting 4: write captures on fresh synthetic companies (four rows)

Not for merge. Written to LAB-COMMON v3 (C1 to C11 apply to every row in full). Each row names its company exactly, creates it NEW (STOP if the name or data folder exists), takes a backup into a new empty folder before the first change, and ends with a restore on the run coordinator's explicit word. Every action on Tally's screen is an OWNER STEP, done by the owner one at a time; no one else acts on Tally's screen. Every read is sent by the built tool, one request at a time, so no hand-written request is sent and the request-ID preflight (header ID equal to the collection name) does not apply to these four rows. Record the TallyPrime version, the licence mode and the date at the top of every row.

**Marks (26 Oct 2026).** `tally_status` carries no marks on the builds these rows run (it returns the product, release, licence, Education mode, endpoint, loaded companies and the date). Wherever a row below says "`tally_status`: Mx, Vx" (or reads a mark from it), call `tally_status` as written and ALSO `ledger_masters` (first page, no `snapshot_id`), and take the marks from its `snapshot.master_alter_id` (M) and `snapshot.voucher_alter_id` (V); record both answers whole and `snapshot.reused`, which must read false. A book that holds no voucher may give no V: record it. The stop rules on marks apply to these values.

## Build (named 26 Oct 2026, for a run before the release-candidate cut)

The build already staged at the lab as `w1-2306d86d`: the Windows bundle of master `2306d86dfd4c86a3c5f6c19ea9ddff6818c460fb`, Bridge CI run `40077056130` (event `schedule`, conclusion `success`), artifact `12553782422` `unsigned-bundle-smoke-windows-latest` (60,880,528 bytes, zip digest `sha256:74ae2daa851397b71cb59e80373d9478a2da72c74398108e33e34ea83e56eb1e`). Its `bridge_mcp.exe` as the lab recorded it on staging: sha256 `92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524`. Before the first call hash the exe that will run and compare it with that line; any other hash is a STOP. These rows capture what Tally does; they test no change of the candidate, so a run on this build stands as their capture, and the cut decides whether any row is repeated on the candidate.

Run it as the lab runs a staged exe over stdio, with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, a NEW empty `BRIDGE_AGENT_DATA_DIR` for each row (kept for the whole row), `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use), `BRIDGE_AGENT_REDACTION` unset and `BRIDGE_AGENT_ENABLE_WRITES` unset (posting OFF). A row that calls `build_import_xml` also sets `BRIDGE_AGENT_ENABLE_IMPORT=true` (the tool refuses without it); nothing is posted by the tool.

RC COMMIT: 2306d86dfd4c86a3c5f6c19ea9ddff6818c460fb  EXE SHA256: 92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524  MCPB SHA256: the `archive_sha256` of the artifact's `mcpb-smoke.json`, as staged

## Order
| # | Sheet | Company (new) | Issue | What is keyed |
|---|---|---|---|---|
| 1 | `W4-1108-rename-with-marks.md` | BRIDGE RENAMEMARKS LAB | #1108 row 5 | one ledger renamed and renamed back |
| 2 | `W2-1037-hand-import-alterid-span.md` | BRIDGE HANDIMPORT LAB | #1037 | two hand imports of a built file, a restore between |
| 3 | `W1-917-longest-names.md` | BRIDGE NAMELEN LAB (and one further company for the company-name limit) | #917 | many ledgers and groups with long names |
| 4 | `W3-691-credit-debit-note-hand-entry-reads.md` | BRIDGE NOTES LAB | #691 | two invoices and a credit and a debit note |

Restore each company at its row's end, before the next row.

## Sitting-4 items NOT written, and why (no recipe is improvised)
- **#1108 rows 1 to 4 and the partly created batch.** No post that Tally refuses after Bridge's own checks pass can be produced: a voucher carrying its own number, or naming a ledger the book does not hold, is refused by Bridge before anything is sent, and a raw import that Tally rejects cannot go through `post_import` (issue note of 24 October). Only row 5 is written (W4). The open problem is a way to make Tally refuse a post that Bridge's checks admit.
- **#1379 (item-line invoices).** Every measurement in the issue needs an item invoice imported with stock-item lines. Bridge refuses items on a Sales invoice today, so the imports would be hand-written requests of a new shape (section 9.12 holds the earlier hand imports). New request shapes need their own written, preflighted request files and an approved shape first; they are not invented in a sheet.
- **#1509 (opening bill on a ledger master).** Step 1 is a read-only export of a ledger whose opening bills were keyed by hand, for which the issue says the exact request will be posted on the issue before it is sent, and it first needs a check that the company it names is still on the lab PC. The guessed shape is never sent. Step 2 (one import in exactly the exported shape) waits on step 1.
- **#852 (batch post on a large book).** It needs a large synthetic book (the failing book is a client's and is not in the lab) and a capture proxy that times the import request; neither exists as a lab set-up. Building the large book is the sitting-5 build (#1029).

`SHA256.txt` lists every file here. Check it before the sitting starts.
