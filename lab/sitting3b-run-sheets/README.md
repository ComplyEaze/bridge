# Run sheets for lab sitting 3, the second half: one row written, three not (and why)

Not for merge. Written to LAB-COMMON v3 (C1 to C11 apply in full). The row names its company exactly, creates it NEW (STOP if the name or data folder exists), takes a backup into a new empty folder before the first change, marks every screen action as an OWNER STEP, and ends with a restore on the run coordinator's explicit word. Every read is sent by the built tool, one at a time, so no hand-written request is sent and the request-ID preflight does not apply to it. Record the TallyPrime version, the licence mode and the date at the top of the row. The build is not named: the run coordinator fills the marks at the cut. RC COMMIT: ______  EXE SHA256: ______

## Written
| Sheet | Company (new) | Issue | What is keyed |
|---|---|---|---|
| `X1-683-forex-digit-grouping.md` | BRIDGE FOREXGROUP LAB | #683 | four vouchers (two foreign-currency Sales, a Receipt at another rate, a rupee Journal); the digit-grouping setting changed and changed back |

## Not written, and why (nothing is improvised)
- **#681 (the cost of bill allocations in a named-field voucher profile).** The measurement compares the per-voucher wire bytes of one voucher profile with and without `ALLLEDGERENTRIES.BILLALLOCATIONS.*`. No built tool sends either variant with a switch, so the two requests would be hand-written files of a new shape, and a request that fails inside Tally's UI blocks the gateway (the header ID must equal the collection name; each file must be decoded and checked first). They have to be rendered by the product's own request renderer, with that one field added, and then preflighted. That generator run and its check are not done; the sheet follows them. Needs a lab book with a few hundred bill-wise vouchers, which is also not set up.
- **#899 (spanned census captures).** It waits on an owner decision (the 5 October bytes were kept private and the issue says committed captures need that decision first), and on raw census requests rendered by the product for a spanned and an unspanned window. Neither exists as a request file.
- **#1231 (a populated, reconciled bank entry; cost-centre allocations on vouchers).** No built tool returns the bank-allocation fields or the voucher-level cost-centre allocations (the second is #1707, still a draft), so the capture has to be the raw answer to the voucher read the product already sends, which again needs the product's renderer for the request and its check. The two master reads that exist (`masters` with `kind=cost_centres`, and `company_features`) can be run against Tally's own screen in a later short row; they are not the missing capture.

`SHA256.txt` lists every file here. Check it before the sitting starts.
