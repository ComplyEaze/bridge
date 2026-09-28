# `compliance_master_forex_live` and `trial_balance_*_forex_live`: provenance

The compliance ledger master and the native Trial Balance of a book with several Currency masters
(bridge#551), captured so the compliance and Trial Balance reads can be tested on the wire bytes
Tally returns, not on strings written to match a parser.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)** 7.1, `education_mode` false. `/status` was
  healthy before and after each request. Only synthetic lab books were loaded.
- **Date:** 2026-09-25, between 22:01 and 22:02 IST. Each request was sent once, and each answered
  in under 0.1 s.
- **Company:** `BRIDGE CORPUS FOREX` (synthetic). Base INR (currency master NAME `I₹`), plus a `$`
  master. The day's extent read, taken first, gave `BOOKSFROM` 20250401, `LASTVOUCHERDATE` 20260915,
  `ALTVCHID` 18 and `ALTMSTID` 216.
- **Requests:** the bytes Bridge's own builders emit, not written by hand, over `20250401`–`20260915`:
  - `compliance_master_forex_live`: the compliance master request (`render_party_ledger_master_request`);
  - `trial_balance_forex_live`: the native Trial Balance (`render_native_trial_balance_request`);
  - `trial_balance_currency_forex_live`: the same Trial Balance with `CURRENCYNAME` appended to its
    `FETCH` (`render_native_trial_balance_request_with_currency`). That is its only difference.
- **Encoding:** BOM-less UTF-16LE, the undecoded wire bytes.

| file | bytes | sha256 |
|---|---|---|
| `compliance_master_forex_live.utf16le.xml` | 21,714 | `36fe16f1c5f65752c4c75d84b35988f8472e45f62b0641bbfbb1098d8478817f` |
| `trial_balance_forex_live.utf16le.xml` | 15,476 | `2d797a9baf9aa7f66076ba7486db85d60a03edc0052d438c1399278aeeb32131` |
| `trial_balance_currency_forex_live.utf16le.xml` | 16,510 | `a14577d2606a0ec43e73048caca809089fe9bd4a568fb6a35ef8c7407c8c99b7` |

## What the bytes show (one book, one run: PARTIAL)

- **Compliance master:** 10 ledgers. `BRIDGE FX DEBTOR A`'s `OPENINGBALANCE` is the composite
  `-$ 500.00 @ I₹ 84/$  = -I₹ 42000.00`. The request fetches no `CURRENCYNAME`, and one row's
  `PARENT` holds the reserved `&#4;` value.
- **Trial Balance with `CURRENCYNAME`:** every one of the 10 rows carries it: `$` on 3 and `I₹` on 7.
- **A ledger's currency does not predict whether its values are composite.** Three `I₹` rows carry
  dollar composites:
  - `FX Party 01`, in `DEBITTOTALS` and `TBALCLOSING`: `-$ 100.00 @ I₹ 201/$  = -I₹ 20100.00`;
  - `FX Sales`, in `CREDITTOTALS` and `TBALCLOSING`: `$ 100.00 @ I₹ 3786/$  = I₹ 378600.00`;
  - `Profit & Loss A/c`, in `TBALCLOSING`: `$ 0.00 @ I₹ /$  = I₹ 0.00`. Its rate slot is empty.
  The rates are derived (base total divided by the dollar component), not rates any voucher used.
- **The `=` parts do not tie:** summed over all ten rows, they come to 34,500.

## The compliance source's other reads, at the same moment (2026-09-25, 22:49-22:50 IST)

The same host, taken the same way, with only synthetic books loaded. The company-extent collection
was read before and after. Both responses are byte-identical and give the values above (`ALTVCHID` 18,
`ALTMSTID` 216), so the book did not change between the compliance master above (22:01) and these
reads. Together they are one moment of one book.

- `company_extents_forex_live`: the extent collection (`CompanyBookExtentV2`), the "before" read. It
  lists every loaded company, all of them synthetic lab books.
- `balance_snapshot_forex_live`: the compliance source's balance snapshot
  (`render_native_ledger_snapshot_request`), over `20250401`–`20261001`, the runtime's closing
  boundary after `LASTVOUCHERDATE`. 10 ledgers, each with `CURRENCYNAME`.
- `group_snapshot_forex_live`: the compliance source's group snapshot
  (`render_native_group_snapshot_request`). 28 groups. Its `PARENTSTRUCTURE` values hold raw U+0003
  separators, which a strict XML parser refuses.

| file | bytes | sha256 |
|---|---|---|
| `company_extents_forex_live.utf16le.xml` | 6,586 | `4e6eeaccb3f2d6e149285151942a5d088b6894248c5e86bb2df47917266eaa83` |
| `balance_snapshot_forex_live.utf16le.xml` | 15,768 | `55cfe08de04f5eec29eba85fcf7363e840af29c3992f1496c32538399404ab93` |
| `group_snapshot_forex_live.utf16le.xml` | 54,280 | `35d78956890fc78f7b019d50d1a148d0c8c40da8bf339904e0aa94aad1bda0c2` |

## The Bills reports after the dollar invoice to a rupee party (2026-09-26, saved 02:25 IST)

These were captured for bridge#642 (outstandings on a party ledger with mixed-currency values). The
requests are Bridge's own builder bytes, `render_native_bills_request`, for Receivable and Payable on
`BRIDGE CORPUS FOREX` over `20250401`–`20260915` (request SHA-256, UTF-16LE wire: Receivable
`ea98632ace0ea84a0cc224a2f270ffd131a7680af32066ab9bace1c4f1cfa4f0`, 770 bytes; Payable
`03c028f94c73796ab24976ced1fb1a6aac037a0857be256734ee0a8a5e4fc9e3`, 764 bytes). The extent collection
was read before and after, and both responses are byte-identical. They give this book `ALTVCHID` 18 and
`ALTMSTID` 216, the values of the 25 Sep reads above, so these reports describe the same moment of
the book as `balance_snapshot_forex_live`. The host's `/status` was not saved with these two captures.

- `bills_receivable_forex_post_c1_live`: 19 bills in one record shape (`BILLFIXED`, `BILLCL`,
  `BILLDUE`, `BILLOVERDUE`). Every `BILLCL` is a plain decimal, including the bills of the `$`
  ledgers. The dollar invoice on the rupee party `FX Party 01` (`FX-USD-ON-INR-1`, dated 15-Sep-26)
  is `-8600.00`, its rupee value, and not a composite. That party's five bills sum to 20,100, the
  base part of its composite closing.
- `bills_payable_forex_post_c1_live`: an empty `<ENVELOPE></ENVELOPE>`, the shape Tally gives an empty
  collection. The book has no payable bills.

| file | bytes | sha256 |
|---|---|---|
| `bills_receivable_forex_post_c1_live.utf16le.xml` | 8,650 | `213312210fc8c699845e313af43820b35fe10b814e54cb6cf3f69d624a35c6f4` |
| `bills_payable_forex_post_c1_live.utf16le.xml` | 46 | `8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3` |

## After a part-settlement at another rate (2026-09-26, 17:04 IST)

These were captured for bridge#642 and the #683 record. A Receipt was entered at the screen against `FX-USD-ON-INR-1` (FX Party 01): $40 @ 88, into Cash. The sale was $100 @ 86. The screen showed only two voucher lines, with no gain or loss line.

The requests are the runbook's builder bytes (`01b`: `render_native_ledger_snapshot_request` over `20250401`–`20261001`, SHA-256 `a7d28418e60dbea3dc9d4d502e5ce93b264e52b6ce35eb8062a56fdabb935947`; `03b`: `render_native_bills_request`, Receivable, over `20250401`–`20260915`, SHA-256 `ea98632ace0ea84a0cc224a2f270ffd131a7680af32066ab9bace1c4f1cfa4f0`). They were sent once each on port 9001, under a lab read grant. The extent collection, read before and after, is byte-identical and gives this book `ALTVCHID` 19 and `ALTMSTID` 216, so the two responses are one moment. The extent response itself is not committed: it lists every loaded company.

- `balance_snapshot_forex_post_receipt_live`: 11 ledgers. `Cash` is now a composite, `-$ 40.00 @ I₹ 948/$  = -I₹ 37920.00`, where 948 is derived: 37,920 ÷ 40. `FX Party 01` closes `-$ 60.00 @ I₹ 279.6667/$  = -I₹ 16780.00`.
- `bills_receivable_forex_post_receipt_live`: 19 bills, none composite.
  - `FX-USD-ON-INR-1` is `-5280.00`, the remaining $60 at 88. It includes 200 of forex gain that Tally computes with no voucher.
  - Three `$` bills that no voucher touched moved to 88: `FX-OPEN-1`, `FX-INV-1` and `FXU-INV-002`. `FXU-INV-001` did not move.

| file | bytes | sha256 |
|---|---|---|
| `balance_snapshot_forex_post_receipt_live.utf16le.xml` | 15,828 | `1d114fca9363930bb05002fe99a712af7cd68b433139e63eac1c8bce172697ca` |
| `bills_receivable_forex_post_receipt_live.utf16le.xml` | 8,650 | `0c7f6a6a2dcb53cf44337a031c12d30ce39a8cd126d3cafb92e6d2c152214c9a` |
