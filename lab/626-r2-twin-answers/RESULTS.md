# Results: #626 R2, twins that differ by a trailing CR LF, both creation orders, TallyPrime 7.2 Silver, 2 Nov 2026

**In both creation orders Tally kept two separate ledgers with their own GUIDs, and each of the four Payments landed on the ledger whose exact bytes it named. On this book, for a trailing CR LF, a voucher's name matched the ledger by exact bytes. Two pairs, one sitting: PARTIAL.**

Synthetic company BRIDGE TWIN R2 LAB, created on Tally's screen for this row, one currency (base INR), loaded alone, port 9001. Raw requests by curl, each write sent once, no ComplyEaze Bridge build. A backup was taken by Tally's own screen before the first write, and stays on the lab PC (23,391 B, sha256 c41a8bb7821b2bfa...).

## Ledgers
All created by request. Each answered CREATED 1, ALTERED 0, ERRORS 0. All under Indirect Incomes. GUIDs share the company's prefix; the last 2 hex digits differ.

| pair, order | name | name bytes, UTF-16LE hex | GUID ends | ALTERID |
|---|---|---|---|---|
| A, first | `Twin A` | 5400770069006e0020004100 | ce | 208 |
| A, second | `Twin A` + CR LF | 5400770069006e00200041000d000a00 | cf | 209 |
| B, first | `Twin B` + CR LF | 5400770069006e00200042000d000a00 | d0 | 210 |
| B, second | `Twin B` | 5400770069006e0020004200 | d1 | 211 |

UTF-8 forms: 5477696e2041, 5477696e20410d0a, 5477696e20420d0a, 5477696e2042. The first ledger of each pair was unchanged by the second: same GUID, name and ALTERID. In raw answers the break reads `&#13;&#10;`. Each ledger's NAME attribute and NAME element hold the same bytes. [verified]

## Imports
Each a Payment, Dr the name under test, Cr Cash, the name sent with `&#13;&#10;` for the break. Each answered CREATED 1, ERRORS 0.

| import | date | amount | name sent | ledger entry as read, UTF-16LE hex | landed on, by exact bytes |
|---|---|---|---|---|---|
| imp-a-plain | 20260804 | 711.00 | `Twin A` | 5400770069006e0020004100 | ce |
| imp-a-break | 20260805 | 712.00 | `Twin A` + CR LF | 5400770069006e00200041000d000a00 | cf |
| imp-b-break | 20260806 | 713.00 | `Twin B` + CR LF | 5400770069006e00200042000d000a00 | d0 |
| imp-b-plain | 20260807 | 714.00 | `Twin B` | 5400770069006e0020004200 | d1 |

Read by XML structure, not by text search: the `ALLLEDGERENTRIES.LIST` entry's LEDGERNAME against the LEDGER rows' NAME attribute, in the same step's reads. CR is protected before parsing, so the parser cannot normalise it; `tools/step.py` shows how. Each name matched exactly one row. [verified]

Marks: from M 207 and no voucher, to M 211 and V 4: one step for each write, none moved by a read.

## Reading for #626
- **Supports narrowing, for this one difference:** on 7.2 Silver, for a trailing CR LF in either creation order, exact bytes decided which ledger a voucher landed on. [verified on this book]
- **The row does not cover:** a name folding to both twins but equal to neither (a different question; the 13 Oct run on 7.1 saw one land on the plain twin), case twins, alias twins, twins made on Tally's screen, Gold, other releases, or a book other than this one.
- **Not read:** ledger balances or a report. "Landed on" means the entry's name equals that one ledger's bytes, not that a closing balance moved.

## Noted
Tally reads back `PARTYLEDGERNAME` as `Cash` on all four vouchers, though each file sent the twin's name there. The ledger entries, not that field, are what this row used.

## Deviations
The company is left as it ended, shut. No restore: the row did not ask for one, and it is a new synthetic book. No sheet exists for this row: its steps are the run lead's message, and the request shapes are the earlier 626-T sheet's, with the company, names, dates and amounts changed.

**Confidence:** two pairs, one sitting, one synthetic book: PARTIAL.
