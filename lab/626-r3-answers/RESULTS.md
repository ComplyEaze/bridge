# Results: #626-R3, one ledger of a CR LF pair, through Bridge's own path, 2 Nov 2026

**Pass. All four Payments landed on the named ledger: every entry's bytes equal the name sent, only that ledger moved, and its twin stayed at nil. Each dialog named both ledgers of the pair and said which is used. A twin that appeared while an approval was open was refused as `ledger_has_folded_twin`, with no attempt recorded and the voucher mark unmoved.**

Build: head fix/626-crlf-twin-review 2c37d1af, `bridge_mcp.exe` sha256 cf588596399c1bd581e9396dae22c244aff82a764ada8a0a61bd6959d5c38230, equal to the package's smoke file, before and after. Over stdio, posting on. BRIDGE TWIN R3 LAB, new, one currency, no inventory, no GST, TallyPrime 7.2 Silver, loaded alone. Approvals answered by mouse.

## The book
Seeded by direct XML, each `CREATED` 1, `ERRORS` 0.

| pair, order | name | UTF-16LE hex | group | GUID ends |
|---|---|---|---|---|
| 1, first | `Twin Lab` | 5400770069006e0020004c0061006200 | Indirect Expenses | cd |
| 1, second | `Twin Lab` + CR LF | 5400770069006e0020004c00610062000d000a00 | Indirect Incomes | ce |
| 2, first | `Twin Lab B` + CR LF | 5400770069006e0020004c0061006200200042000d000a00 | Indirect Expenses | cf |
| 2, second | `Twin Lab B` | 5400770069006e0020004c006100620020004200 | Indirect Incomes | d0 |

Also `BRIDGE Twin Bank` and `Solo Lab`.

**One addition, not in the sheet.** The first build refused `empty_book_first_import`, nothing sent: a book that has never held a voucher has no mark to attribute against. So one raw Payment on a new `Seed Expense` ledger and Cash went in first, 1.00, 2 April 2025. It touches no ledger under test.

## Steps
| step | what | outcome |
|---|---|---|
| 1 | `validate_masters` on `Twin Lab` | `importable` true, `match_state` exact, no `folded_twins` |
| 2 | Payment 101.00 to `Twin Lab` | approved, posted, V 1 to 2 |
| 3 | Payment 102.00 to `Twin Lab B` + CR LF | approved, posted, V 2 to 3 |
| 4 | Batch: 103.00 to `Twin Lab` + CR LF, 104.00 to `Twin Lab B` | approved, posted, V 3 to 5 |
| 5 | `verify_import` on each batch | `posted_verified`: 1, 1, 2; every other count 0 |
| 6 | Raw entries and closing balances | below |
| 7 | Held approval, `Solo Lab` + CR LF seeded with the box up | `ledger_has_folded_twin`, `attempt_recorded` false, V stays 5 |

[verified]

## The dialog
Whole text in `box/`. Step 2's two lines:
```
Used: "Twin Lab", no line break at its end, group "Indirect Expenses"
Not used: "Twin Lab", line break at its end, group "Indirect Incomes"
```
Step 3's:
```
Used: "Twin Lab B", line break at its end, group "Indirect Expenses"
Not used: "Twin Lab B", no line break at its end, group "Indirect Incomes"
```
Step 3 also marks its entry, `Dr 102.00  Line break  "Twin Lab B\r\n"`, and adds the line "Line break: a name that ends in a line break in Tally (shown as \r\n). No other name here has one."

Step 4 is two lines per pair, four in all. It did not go to the totals form, so one form was seen: each voucher listed, then per-ledger totals, then the four lines. The totals form is unmeasured here.

## Entries, as read
Read by structure with CR protected. `reads/rv1.xml`.

| voucher | entry `LEDGERNAME`, UTF-16LE hex | amount |
|---|---|---|
| R3-1 | 5400770069006e0020004c0061006200 | -101.00 |
| R3-2 | 5400770069006e0020004c0061006200200042000d000a00 | -102.00 |
| R3-3 | 5400770069006e0020004c00610062000d000a00 | -103.00 |
| R3-4 | 5400770069006e0020004c006100620020004200 | -104.00 |

Each equals the name sent, byte for byte, and its own ledger's name above. The other entry on each is `BRIDGE Twin Bank`. [verified]

## Closing balances
`TBALCLOSING` from a `List of Ledgers` read for the year, raw, the fields of the ledger snapshot with `CLOSINGBALANCE` beside them; Tally signs a debit negative. `evidence/closing-table.txt`.

| ledger | before | after the posts | after step 7 |
|---|---|---|---|
| `Twin Lab` | nil | -101.00 | -101.00 |
| `Twin Lab` + CR LF | nil | -103.00 | -103.00 |
| `Twin Lab B` | nil | -104.00 | -104.00 |
| `Twin Lab B` + CR LF | nil | -102.00 | -102.00 |
| `BRIDGE Twin Bank` | nil | 410.00 | 410.00 |
| `Solo Lab` | nil | nil | nil |
| `Solo Lab` + CR LF | absent | absent | nil |

Each ledger moved by its own Payment and no other, and the bank's 410.00 is their sum. So "only the named ledger" holds at the balance as well as the entry, which R2 did not read. [verified]

## Step 7
Order, from the logs: box seen 18:33:01Z; `Solo Lab` + CR LF seeded with the box up, `CREATED` 1, M 215 to 216; Yes by mouse 18:33:05Z; answer `ledger_has_folded_twin`, "No posting attempt was recorded". V 5 before and after. `reads/rv1.xml` and `rv2.xml` differ only in the header's ledger count, 22 to 23, from that seed; the voucher rows are identical. [verified]

The box at step 7 showed no Used or Not used line, as the pair did not exist when it opened.

## `posted_verified`
Each row's `marker` reads `post_span_binding`, which says how the voucher was found. In the code the status then comes from `entries_match` on that voucher (`agent_import_verification.rs`), so it is from entries; that is a reading of the code, the answer does not say it. The answers themselves show the status. [partial for the basis, verified for the status]

## Left behind
BRIDGE TWIN R3 LAB stays, with the seed voucher, four Payments and the `Solo Lab` pair. Batch `R3-5` was built and not posted. Company shut, no company loaded, one tally.exe on 9001. Client ended, exit 0.

**Confidence:** two pairs in both orders, one book, one Tally, one sitting: PARTIAL.
