# Results: #2068 control on an integrated book, 28 Oct 2026. trial_balance returns no stock line, and Tally's own ledger-wise Trial Balance shows Opening Stock Dr 1,000.00. The tool's opening and closing sums are each 1,000 away from zero, which is exactly that line

The sheet is `lab/2068-integrated-control` at c5a10d79335c8a8a8cdc65e28e610efaaafbacde (`SHA256.txt` 3 of 3 equal). It is read only, on BRIDGE TIEOUT D (synthetic, integrated): number 100075, GUID 6a9518b2-a88c-417c-a700-40243f9d9119.
- **Build:** master 609776ae (`bridge_mcp.exe` sha256 4c9bace3...4544, the same before and after), over stdio with writes off.
- **Tally:** TallyPrime 7.2, Silver, Licensed.
- **Marks:** before 213 / 4, after 213 / 4.

## `trial_balance`, 1 Apr 2025 to 31 Mar 2026 (one call, limit 100)
| ledger | parent | opening | debit | credit | closing | ties |
|---|---|---|---|---|---|---|
| Cash | Cash-in-Hand | 0.00 | empty | empty | empty | yes |
| Customer | Sundry Debtors | 0.00 | -2850.00 | empty | -2850.00 | yes |
| Lab Capital | Capital Account | 1000.00 | empty | empty | 1000.00 | yes |
| Profit & Loss A/c | (root) | 0.00 | empty | empty | 0.00 | yes |
| Purchases | Purchase Accounts | 0.00 | -2600.00 | empty | -2600.00 | yes |
| Sales | Sales Accounts | 0.00 | empty | 2850.00 | 2850.00 | yes |
| Supplier | Sundry Creditors | 0.00 | empty | 2600.00 | 2600.00 | yes |

- **Stock-in-Hand:** no ledger under it is returned, and the book holds none.
- **`totals`** (`totals_scope` `all_returned_ledgers_numeric_observations_only`): opening **1000**, debit -5450, credit 5450, closing **1000**.
- **Debit and credit** cancel. The opening and closing sums are each 1,000 away from zero.
- **Snapshot:** reused false, master 213, voucher 4.

## Tally's own Trial Balance (view only; period offered 1-Apr-25 to 15-Oct-25, the last voucher being 15 Oct 2025)
| view | lines | Grand Total |
|---|---|---|
| group-wise | Capital Account Cr 1,000.00; Current Liabilities Cr 2,600.00; **Current Assets Dr 3,850.00**; Sales Accounts Cr 2,850.00; Purchase Accounts Dr 2,600.00 | 6,450.00 / 6,450.00 |
| ledger-wise (F5) | **Opening Stock Dr 1,000.00**; Customer Dr 2,850.00; Lab Capital Cr 1,000.00; Purchases Dr 2,600.00; Sales Cr 2,850.00; Supplier Cr 2,600.00 | 6,450.00 / 6,450.00 |

## Comparison
- **The missing line:** Tally's ledger-wise Trial Balance names an **Opening Stock** line, Dr 1,000.00, that `trial_balance` does not return.
- **It equals the gap:** that 1,000.00 is exactly the gap in the tool's opening and closing sums.
- **Group-wise:** the same 1,000.00 sits inside Current Assets (3,850.00 = Customer 2,850.00 + 1,000.00).
- **Closing Stock:** Tally's Trial Balance shows no line on this integrated book; it shows the opening stock.
- **Status:** one observation, PARTIAL until repeated.

## Files
- `answers/` (000 to 005).
- `client-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures by sha256; they stay on the lab PC.
