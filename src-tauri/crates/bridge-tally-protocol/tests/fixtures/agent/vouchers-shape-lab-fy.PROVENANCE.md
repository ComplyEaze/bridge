# `vouchers` search and summaries: the rows and the answers of one live run (#1230)

Covers `vouchers-shape-lab-fy.rows.json` and `vouchers-shape-lab-fy.live-answers.json`.

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `vouchers-shape-lab-fy.rows.json` | 39022 | `8e6471d4bf7b19a514a54c74e4ecd8c052221890132175d03c27e6bc4b613e47` |
| `vouchers-shape-lab-fy.live-answers.json` | 37178 | `b6ced2d614c4dd7cd81a21c02f200cfb8c96f192c55767d1bd174c382cf2260d` |

Captured, not authored: the values below come from one supervised read-only run, with
only the fields named under "Transformation" kept. They are parser output and tool
answers, not Tally wire bytes.

## What was read

- **Date and host:** 2026-10-06, 18:09 to 18:13 IST; TallyPrime 7.1, licensed Silver,
  education mode off; port 9001 through a recording relay, one request at a time.
- **Book:** the synthetic company `BRIDGE SHAPE LAB` (67 vouchers in the year
  2025-04-01 to 2026-03-31, 44 ledgers, 33 groups, voucher mark 111). No client data.
- **Build:** a debug build of `bridge_mcp` from origin/master 4c30f3f9f (not in a published
  build), with default settings except the response budget, raised to 2,000,000; the largest
  result was 86 KB, which fits the default.
- **Run:** 15 steps, 16 tool calls, 550 relayed requests, none refused and none over two
  seconds; `vouchers` for the whole year plain, with `summarise_by` ledger, month and
  voucher type, with `ledger` set and `summarise_by` month, with each search argument,
  with an amount search plus a ledger summary, and two pages of one window; then
  `trial_balance` for the same year. A summary with `voucher_class` set was also run and is
  not kept in the fixture.

## Transformation

- `rows.json`: the `items` of the plain listing of the year, exactly as the tool
  returned them (67 vouchers, no redaction), keys sorted, no other edit. They are used as the window rows the search and the summary run over.
- `live-answers.json`: from the later calls, the fields a test compares: `buckets`,
  `total`, `totals`, `vouchers_summarised`, `excluded_from_buckets`,
  `post_dated_included`, `post_dated_flag_absent`, `entries_counted`, `summarised_by`
  and `state` for each summary; for each search the arguments, `total`, `state`, the
  GUIDs of the items and their `matched`; and from `trial_balance` each ledger's
  `debit` and `credit` as the tool returned them (two decimal places, where a bucket drops
  trailing zeros: compare them as numbers). Keys sorted.
- The ledger-selected summary and the search plus summary were run on the vouchers the
  selector or search kept; the answers hold what the tool returned for them.

## What it establishes

On this book, in this run: every bucket the tool returned equals the sums over the 67
rows; the three summaries have the same totals; the 30 ledger buckets equal
`trial_balance`'s period debit and credit for those ledgers, and the other 14 ledgers
of the trial balance had no movement in the year; each search returned exactly the
vouchers the same criterion selects from the rows (a number that no voucher has, 0,
from a window the tool called `complete`); a voucher Tally flagged post-dated was
counted; a cancelled voucher (exported with no entries), an optional voucher and an
entry-less Stock Journal were left out of the buckets and counted. Two entries (both
`Round Off`, +0.50 and -0.50) carry a deemed-positive flag that disagrees with the sign of
their amount; the buckets follow the sign, each of those two vouchers balances only by the
sign, and the trial-balance tie is the same by either rule, so it does not tell them apart.

## What it does not establish

One synthetic book of 67 vouchers, one TallyPrime release, one build, one run. The tie to the trial
balance is on the period columns only (the opening columns were not compared). No
memorandum, reversing journal, voucher withheld for a foreign-currency amount or non-INR
book, and no zero-amount entry; no masking or narration
withholding; nothing about a large book, a window the read refuses, or another
edition. The summaries are not shown against any report but `trial_balance`.

The raw responses are retained privately by the maintainer and are not part of this
repository.
