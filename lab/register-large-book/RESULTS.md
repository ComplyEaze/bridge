# Results: #2265 row R4b, registers on a large book, 3 Nov 2026

**All answer. On BRIDGE CORPUS DENSE grown to 10,123 ledgers, a month's `sales_register`, `purchase_register` and `gst_by_counter_ledger` each answered `complete` in 52 to 55 s, with no refusal, no `failed_request` and no deadline. Neither `ledger_master` nor `ledger_balance` was named. The slowest of steps 1 to 4 is the balance read, 2.62 s on the first pass.**

Build: head 6b6f1f56 on fix/2265-name-the-failed-read, run 41548572803, artifact 13189868906, zip digest equal, `bridge_mcp.exe` sha256 f5478ad9236adb58758023e03e1c26bb2b1e6a5222a1ce5c79b3e2828e781964, equal to the package's smoke file, before and after. BRIDGE CORPUS DENSE, synthetic, 29,900 vouchers, 328 masters, TallyPrime 7.2 Silver, licensed, loaded alone. Posting off.

## The seed
Backup taken first: 14,661,545 B, sha256 223a2ceaef95bfae88d2e6115dcea8745401d8540c96d523ad61cb85c7c678da, local.

The first seed was stopped after the group and one import: `ADDRESS.LIST` on the ledger did not land. One probe with a dated `LEDMAILINGDETAILS.LIST` did, on the first try, so no variant was needed. The backup was restored, proved byte-identical, and the whole seed sent again. The first attempt's counters are in `evidence/seed-log-first.txt`.

The seed as run: one group, `Lab Scale Sub Debtors` under Sundry Debtors; 4,000 ledgers under Sundry Debtors, 4,000 under Sundry Creditors, 2,000 under the new group; 20 imports of 500. All 21 answered as sent: `CREATED` 500 (1 for the group), every other counter 0, no `LINEERROR`. Marks rose by exactly the created count, 328 to 10,329, V 29,900 unchanged. 127.9 s in all, 5.7 to 9.5 s an import. No Currency master was added. Sundry Debtors held 120, so 4,120 after the seed; no parent over 4,266. `evidence/seed-log.txt`. [verified]

GSTIN, PAN, name on PAN, bank holder, account and IFSC read back in the master read. The address was read back on the probe only: seeded rows use the probe's shape and were not read for it. [verified for the probe; hypothesis for the seeded rows]

Seeded rows set the test up and are no evidence of their own shape.

## Steps
| step | read | bytes | seconds | rows | repeat, seconds |
|---|---|---:|---:|---:|---:|
| 0 | Currency masters, on screen | | | 1 | |
| marks | raw | 3,418 | 0.01 | V 29,900, M 10,329 | |
| 1 | catalogue | 11,490,082 | 2.35 | 10,123 | 0.52 |
| 2 | groups | 56,016 | 0.03 | 29 | 0.01 |
| 3 | balance of every ledger | 12,667,522 | **2.62** | 10,123 | 0.65 |
| 4 | full master, `$Parent` Sundry Debtors | 10,895,042 | 2.41 | 4,120 | 0.75 |
| 4 | full master, `$Parent` Sundry Creditors | 10,755,006 | 2.17 | 4,000 | 0.73 |

Rows counted from the files. Each repeat is byte-equal to its first pass. Marks after step 7 and after step 8 are the same bytes as before step 1. Steps 1 to 4 were first sent right after the seed; why the repeats are faster was not tested. [verified]

The master and group requests are the bytes the build sends, or that template with the source's `$Parent` filter. The catalogue and balance requests are rendered from the source at this head, not captured from the build.

| step | call | seconds | Tally bytes | `isError` | state | code, cause, `failed_request` | rows |
|---|---|---:|---:|---|---|---|---|
| 5 | `sales_register`, April 2025 | 52.03 | 338,309,360 | false | complete | none | `items` 0, `total` 0 |
| 6 | `purchase_register` | 53.97 | 338,309,360 | false | complete | none | `items` 0, `total` 0 |
| 7 | `gst_by_counter_ledger` | 54.66 | 338,309,360 | false | complete | none | `counter_ledgers` 0 |
| 8 | `sales_register` again, new process and data folder | 52.83 | 338,309,360 | false | complete | none | as step 5 |

Each saw `ledger_masters_observed` 10,123 and `vouchers_observed` 2,430. No classification refusal. `sales_register` counts 2,430 under `sales_vouchers_without_duties_taxes_entry`: none of them touches a Duties & Taxes ledger, so no row. `gst_by_counter_ledger` counts 2,430 under `vouchers_without_gst_entry`. `purchase_register` read a window with no Purchase voucher, `purchase_vouchers_without_duties_taxes_entry` 0. The sheet expected about 2,460 Sales vouchers; the tools report 2,430. [verified]

## Answer bytes per ledger in the master read
| ledgers | bytes each |
|---|---:|
| 120 under Sundry Debtors before the seed | 1,856 |
| the 4,000 seeded under Sundry Debtors | 2,667 |
| the 4,000 seeded under Sundry Creditors | 2,688 |
| the 2,000 seeded under the new group | 2,659 |

These are UTF-16 bytes as Tally answered. The master's `FETCH` names `LEDADDRESS.LIST`, which stayed empty on the probe, and not the mailing list, so a mailing address adds nothing to these figures. [verified]

## What the build sent
Added beyond the sheet, both read-only, each a separate data folder, through a loopback relay that changed nothing:

**Before the seed**, all three tools once, to get the request bytes: each `complete`, 70.40, 65.88 and 73.65 s, 185,362,792 B, `ledger_masters_observed` 123. The master went as one read, `List of Ledgers` 231,300 B. The relay added its own time, so these seconds are not a clean comparison with steps 5 to 7.

**On the seeded book**, `sales_register` once more before the restore, 56.76 s, the same 338,309,360 B, to see what made them up. By request, answer bytes across all its sends:

| request | sends | bytes | largest answer |
|---|---:|---:|---:|
| catalogue | 4 | 45,960,328 | 11,490,082 |
| master, Cash-in-Hand, Sales Accounts, Sundry Debtors, Primary | 4 | 43,602,496 | 10,900,624 |
| master, Sundry Creditors | 4 | 43,020,024 | 10,755,006 |
| master, `Lab Scale Sub Debtors` | 4 | 21,281,288 | 5,320,322 |
| voucher census | 8 | 11,790,492 | 5,886,156 |
| vouchers | 62 | 172,379,040 | 2,908,786 |

So at this size the build counts by catalogue and reads the master in three parts by parent, each read four times. Group, currency, extent and mark reads make up the rest. No balance read was sent: the book holds one Currency, as step 0 shows on screen. Full table in `evidence/check.txt`. [verified]

## Restore
Restored twice from the same backup on Tally's screen, its hash checked first, with the two prompts expected. After each, marks, extent, catalogue, groups, balance and both masters are byte-identical to the reads taken before the seed: `evidence/restore-check.txt`. V 29,900, M 328, 123 ledgers. [verified]

## Limits
The seed adds ledgers with no vouchers on them, so the month's voucher read did not grow with the masters. Nothing here measures a book whose parties are in use. A second Currency, which would bring the balance read, was out of the row. The row took one book, one sitting.

## Left behind
BRIDGE CORPUS DENSE as it was. Company shut, no company loaded, one tally.exe on 9001. The backup stays on the lab PC.

**Confidence:** PARTIAL, as the limits above.
