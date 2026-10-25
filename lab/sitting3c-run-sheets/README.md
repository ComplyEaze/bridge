# Run sheets for lab sitting 3, the third part: #681 and #1231 (raw requests rendered by the product)

Not for merge. Written to LAB-COMMON v3 (C1 to C11 apply in full). Each row names its company exactly, creates it NEW (STOP if the name or data folder exists), takes a backup into a new empty folder before the first change, marks every screen action as an OWNER STEP, and ends with a restore on the run coordinator's explicit word. Record the TallyPrime version, the licence mode and the date at the top of each row. The build is not named: the run coordinator fills the marks at the cut. RC COMMIT: ______  EXE SHA256: ______

## The requests
The request files in `requests-681/` and `requests-1231/` were not written by hand and were not edited. They are the unedited output of a test-only dump of the product's own request builders (it formats strings and writes files; nothing in it reaches Tally), run for the company name and window each sheet names (1 to 30 April 2025). Each is BOM-prefixed UTF-16LE with no final line break, and each was checked when written: the header ID equals the name of the one collection the request defines (a request whose ID names no collection raises a modal error in Tally that blocks the gateway). `MANIFEST.txt` in each folder lists each file's sha256 and size; check them and `SHA256.txt` before the sitting. Send as LAB-COMMON C3 says, one at a time.
- `v01-vouchers-wildcard.xml`: the shipped voucher read (`ALLLEDGERENTRIES.*`).
- `a01-vouchers-audit-profile.xml`: the tax-audit voucher profile (named fields, no bill-allocation path), the baseline #681 names.
- `c01-census-unspanned.xml`: the voucher census for the window (kept for the census row, #899; not used by these two rows).

## Order
| # | Sheet | Company (new) | Issue | What is keyed |
|---|---|---|---|---|
| 1 | `X2-681-bill-allocation-read-cost.md` | BRIDGE BILLCOST LAB | #681 | about 24 bill-wise vouchers |
| 2 | `X3-1231-bank-reconciliation-and-cost-centre-allocations.md` | BRIDGE BANKREC LAB | #1231 | 6 vouchers, bank reconciliation, two cost centres |

Restore each company at its row's end, before the next row.

## Not here
- A named-field voucher read with the allocation field added (the measurement #681 most wants). It is a new request shape; it is measured through the dispatch path, with its deadline, size cap and window checks, once a lab-only candidate builder exists. This row only records the baseline and the wildcard's allocation content.

## Still not written
- **#899 (spanned census captures).** It waits on the owner's decision about the 5 October bytes (kept private) and on which book to span; the unspanned census request is in the folders above, and a spanned one is produced by the same dump with an AlterID span once the book's mark is known.

`SHA256.txt` lists every file here (including the manifests).
