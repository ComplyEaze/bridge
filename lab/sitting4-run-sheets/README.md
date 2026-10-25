# Run sheets for lab sitting 4: write captures on fresh synthetic companies (four rows)

Not for merge. Written to LAB-COMMON v3 (C1 to C11 apply to every row in full). Each row names its company exactly, creates it NEW (STOP if the name or data folder exists), takes a backup into a new empty folder before the first change, and ends with a restore on the run coordinator's explicit word. Every action on Tally's screen is an OWNER STEP, done by the owner one at a time; no one else acts on Tally's screen. Every read is sent by the built tool, one request at a time, so no hand-written request is sent and the request-ID preflight (header ID equal to the collection name) does not apply to these four rows. Record the TallyPrime version, the licence mode and the date at the top of every row.

The build is not named: the run coordinator fills the marks below at the cut and runs the rows on that build.
RC COMMIT: ______  EXE SHA256: ______  MCPB SHA256: ______

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
