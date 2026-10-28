# Results: #2068 write control, 28 Oct 2026. On a book integrated with inventory, a Stock-in-Hand ledger can be created and given a closing balance. trial_balance and the raw ledger collection still keep its closing at its opening (-1000.00) and show the 400.00 only as a debit total, as on the non-integrated book. Tally's own reports value the stock from the item (1,200.00) and leave the ledger out

The sheet is `lab/2068-integrated-write-control` at a60c01299e1ed9c39d6d9e935b01d7bee44fc5ee (`SHA256.txt` 17 of 17; `gen_requests.py --check` ok; census ok 3, skipped 3).
- **Company:** BRIDGE INTEGRATED CTRL LAB, new: number 100095, GUID 3fc2a3c7-1da8-4bc4-a472-44028cdebca7. Integrate Accounts with Inventory Yes, GST No.
- **Build:** master 609776ae (`bridge_mcp.exe` sha256 4c9bace3...4544, the same before and after).
- **Tally:** TallyPrime 7.2, Silver, Licensed.
- **BK1:** `TDBK1800_100095.001`, 23,030 B, sha256 06c9478ffeaa805a602cc1c093141fcdac0b448f12d2b6069a319f1e0691dca0.
- **Raw marks:** ISINTEGRATED Yes, ISINVENTORYON Yes.

## What Tally did at each keying step (integrated book)
| step | what | Tally | M / V after |
|---|---|---|---|
| K1 | company; Features: GST Yes to No, the rest as offered | accepted | 206 / absent |
| K2 | Unit Nos | accepted | 207 |
| K3 | Customer, Supplier, Sales, Purchases, Lab Capital (1,000.00 Cr) | accepted; Sales and Purchases show "Type of Ledger" and no inventory-values field | 208 to 212 |
| K3 | **Closing Stock under Stock-in-Hand, opening 1,000.00 Dr** | **accepted**: the group is offered, an Opening Balance field is shown (reached by a click), Dr offered, no message | 213 |
| K3 | the Total Opening Balance line after the last ledger | 1,000.00 Cr, Difference 1,000.00 Cr (the Stock-in-Hand ledger's 1,000.00 Dr does not offset it) | |
| K4 | Lab Gadget, 10 Nos at 100.00 | accepted, no message about opening stock | 214 |
| K5a | Purchase 5-5-2025, Accounting Invoice, Supplier, Purchases 1,200.00 | no inventory allocation opened; Bill-wise New Ref | 214 / 1 |
| K5b | Sales 20-6-2025, Customer, Sales 1,600.00 | the same | 214 / 2 |
| K6a | **Alter Closing Stock: a "Closing Balance" field is offered**; 1,400.00 Dr dated 31-3-2026 keyed | accepted, no prompt | 215 / 2 |
| K6b | Stock Journal 31-3-2026, Destination Lab Gadget 2 Nos at 100.00 (offered), Source empty | accepted, no prompt | 215 / 3 |

## Tally's own reports, 1-Apr-25 to 31-Mar-26
- **Profit & Loss A/c:**
  - Opening Stock 1,000.00; Purchase Accounts 1,200.00; Gross Profit c/o 600.00 (2,800.00);
  - Sales Accounts 1,600.00; **Closing Stock 1,200.00**;
  - Nett Profit 600.00.
- **Balance Sheet** as at 31-Mar-26:
  - Capital Account 1,000.00; Current Liabilities 1,200.00; Profit & Loss A/c 600.00; total 2,800.00;
  - Current Assets 2,800.00; total 2,800.00;
  - no "Difference in opening balances" line.
- **Stock Summary:** Lab Gadget 12 Nos at 100.00 = **1,200.00**.
- **Trial Balance:**
  - group-wise: Capital Account Cr 1,000.00; Current Liabilities Cr 1,200.00; Current Assets Dr 2,600.00; Sales Accounts Cr 1,600.00; Purchase Accounts Dr 1,200.00; Grand Total 3,800.00 each side;
  - ledger-wise: **Opening Stock Dr 1,000.00**; Customer Dr 1,600.00; Lab Capital Cr 1,000.00; Purchases Dr 1,200.00; Sales Cr 1,600.00; Supplier Cr 1,200.00; 3,800.00 each side;
  - **no Closing Stock ledger line**.

## The reads (marks 215 / 3 before and after, raw and tool)
- **Raw ledger collection (n-r3), Closing Stock:** TBALOPENING **-1000.00**, TBALCLOSING **-1000.00**, DEBITTOTALS **-400.00**, CREDITTOTALS empty. The other ledgers' closings equal opening + debit + credit.
- **`trial_balance` (20250401 to 20260331, limit 100), 8 rows:**

| ledger | opening | debit | credit | closing | ties |
|---|---|---|---|---|---|
| Cash | 0.00 | empty | empty | empty | yes |
| **Closing Stock** (Stock-in-Hand) | -1000.00 | **-400.00** | empty | **-1000.00** | **no** (opening + debit = -1400.00) |
| Customer | 0.00 | -1600.00 | empty | -1600.00 | yes |
| Lab Capital | 1000.00 | empty | empty | 1000.00 | yes |
| Profit & Loss A/c | 0.00 | empty | empty | 0.00 | yes |
| Purchases | 0.00 | -1200.00 | empty | -1200.00 | yes |
| Sales | 0.00 | empty | 1600.00 | 1600.00 | yes |
| Supplier | 0.00 | empty | 1200.00 | 1200.00 | yes |

- **`totals`:** opening 0, debit -3200, credit 2800, closing 0. The closing totals tie; the debit and credit sums are 400 apart.
- **Balance Sheet export (n-r2):** Capital 1000.00, Current Liabilities 1200.00, Profit & Loss 600.00 against Current Assets -2800.00. It **balances**.
- **Stock Summary export (n-r5):** Lab Gadget 12 Nos at 100.00, -1200.00. That is **not** the ledger's closing: -1000.00 as returned, against 1,400.00 keyed.
- **Profit and Loss export (n-r1):** Sales 1600.00; Opening Stock -1000.00; Purchases -1200.00; Closing Stock -1200.00.

## Set against the non-integrated book (`lab/692-nonint-lab-answers`)
- **The same:** the Stock-in-Hand ledger's closing is kept at its opening (-1000.00) and the closing-balance change (400.00) appears only as a debit total, in the raw collection and in `trial_balance`. So the ledger's row does not tie on either book.
- **Different here:**
  - Tally's own reports take the stock from the item (1,200.00 = 12 Nos), not from the ledger's 1,400.00.
  - Tally's Trial Balance shows an "Opening Stock" line and no Closing Stock ledger line.
  - The Balance Sheet export balances.
- **Status:** one observation, PARTIAL until repeated.

## Files
- `answers/` (tool, 000 to 005).
- `raw/`: every raw answer with its headers, and the marks reads m01 to m16.
- `marksreq/`: the filled marks request.
- `send-log.txt`.
- `client-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures and BK1 by sha256; they stay on the lab PC.
