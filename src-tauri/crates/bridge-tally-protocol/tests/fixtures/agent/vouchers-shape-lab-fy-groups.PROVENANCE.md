# `vouchers-shape-lab-fy.group-live-answers.json`: provenance

The answers of two live `vouchers` calls with `summarise_by` set to `group` and to `primary_group` (#1230), over the year of the synthetic book `BRIDGE SHAPE LAB`, the same year and rows as `vouchers-shape-lab-fy.rows.json`.

## Capture

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` through a recording relay, one request at a time, read-only.
- **Date:** 2026-10-07, about 14:20 IST.
- **Company:** `BRIDGE SHAPE LAB`, 67 vouchers in the year 2025-04-01 to 2026-03-31 (one more voucher, dated after the year, was in the book).
- **Build:** a debug build of the repository at the head of the pull request that adds the two modes, response budget raised to 2,000,000 (the answers were 45,220 bytes for `group` and 24,230 for `primary_group`).
- **Calls:** `vouchers` with `from` 2025-04-01, `to` 2026-03-31 and `summarise_by` `group`, then `primary_group`. Each took 64 relayed requests (34 for the window and 30 for the group modes, as counted from the code), 10.56 s and 11.12 s.
- **The file:** the stable fields of each call's `result` (`summarised_by`, `state`, `total`, `buckets`, `totals`, `vouchers_summarised`, `excluded_from_buckets`, `post_dated_included`, `post_dated_flag_absent`, `entries_counted` and, for `group`, `subtree_totals`, `subtree_totals_total`, `subtree_totals_complete`), keys sorted, no other edit. The snapshot, window timings, `basis` text and evidence of each call are left out (they change from run to run or are text of the tool).

| file | bytes | sha256 |
|---|---|---|
| `vouchers-shape-lab-fy.group-live-answers.json` | 26665 | `8964db9507e7297007fb7e76108568ee261fe57828a386792769e0d28e4885a2` |

## What it establishes

- **The two modes ran against a live Tally and completed**: `group` in 12 buckets and `primary_group` in 8, over 64 vouchers; the 3 left out are one cancelled voucher, one optional and one with no accounting entries.
- **Every bucket of both calls is what the production code gives for the committed rows and the committed ledger and group snapshots** (the tests compare the whole bucket list, the totals, the exclusions and the 18 subtree totals).
- **Independent check, run on the captures after the call:** each group bucket's debit, credit and voucher count equals the first run's ledger buckets added up by the trial balance's own parent column; the 18 subtree totals equal the trial balance rolled up the group tree read from the book's own group list; each chain, with its reserved names, equals that list's chain; each bucket's `primary_group` equals the hand-written table of the book's primary groups; each member's debit and credit equals the first run's ledger bucket and `members_total` equals the number of ledgers touching the group; `totals`, `vouchers_summarised` and the exclusions equal the first run's; each primary bucket's voucher count equals the vouchers touching it.

## What it does not establish

One synthetic book, one TallyPrime release, one build, one run per call. No large book, no ledger without a parent, no group renamed or moved while a window is read, no held later page, no foreign-currency voucher, no masking. The trial balance's parent column and the ledger list's parent come from the same collection, so the roll-up check covers the placement logic, not Tally's grouping.

## Screening

Every ledger and group name in the file is a synthetic lab name. The raw captures stay private.
