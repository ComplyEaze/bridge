# BRIDGE DEMO TRADERS: synthetic demo book (FY 2025-26)

Everything is invented (parties, items, amounts). GSTINs are only of the gate's synthetic family
(`<state>ZZZZZ0000Z<c>Z<c>`). 292 vouchers, 13 parties (9 customers, 4 suppliers), 8 stock items, one bank ledger.
`python3 -I gen_demo_book.py <dir>` writes the same bytes every run (seeded, stdlib only).

## Create the company first (not in these files)
Name `BRIDGE DEMO TRADERS`; books and financial year from 1 Apr 2025; currency INR; bill-wise entry ON; maintain
inventory ON; **batch-wise OFF** (a goods line on a batch-wise company needs a batch allocation); GST ON with state
**Maharashtra**, regular registration, GSTIN `27ZZZZZ0000Z1Z5`. Stock valuation: leave Tally's default.
Customers in Maharashtra (5) are intra-state (CGST+SGST); Karnataka, Gujarat and Delhi customers are inter-state (IGST).

## Import order (one file per step; import the `...-1-first-voucher-only.xml` of each kind ALONE and read it back before its `-2-rest.xml`)
1. `import/00-masters.xml` (unit, 8 stock items, 13 party ledgers, bank, capital, sales/purchase ledgers, 3 output and 3 input GST ledgers, 8 expense ledgers)
2. `10-sales`, 3. `20-purchases`, 4. `30-capital-and-other-receipts` (one voucher), 5. `40-contra`, 6. `50-receipts-against-bills`, 7. `60-supplier-payments`, 8. `70-expense-payments`.
Expect CREATED to equal the voucher count of each file and ERRORS=0 (masters: 1 unit + 8 stock items + 31 ledgers):

| file | vouchers |
|---|---|
| import/10-sales-1-first-voucher-only.xml | 1 |
| import/10-sales-2-rest.xml | 90 |
| import/20-purchases-1-first-voucher-only.xml | 1 |
| import/20-purchases-2-rest.xml | 39 |
| import/30-capital-and-other-receipts-1-first-voucher-only.xml | 1 |
| import/40-contra-1-first-voucher-only.xml | 1 |
| import/40-contra-2-rest.xml | 6 |
| import/50-receipts-against-bills-1-first-voucher-only.xml | 1 |
| import/50-receipts-against-bills-2-rest.xml | 78 |
| import/60-supplier-payments-1-first-voucher-only.xml | 1 |
| import/60-supplier-payments-2-rest.xml | 27 |
| import/70-expense-payments-1-first-voucher-only.xml | 1 |
| import/70-expense-payments-2-rest.xml | 45 |

Every voucher carries a fixed REMOTEID, so a bad import can be deleted by it. Voucher numbers are included but Tally may ignore them;
no answer depends on a voucher number (bills are identified by their reference, e.g. `DT/25-26/012`).

## What is NOT proven (state this to anyone relying on it)
- The XML shapes follow captured fixtures in the repo (item invoice with bill allocation and tax ledgers; accounting vouchers with
  `Agst Ref`). The **purchase** invoice shape is the mirror image of the captured sales shape and has not been imported anywhere; payments,
  receipts against bills and contra use the proven accounting-voucher shape. Nothing here was imported into Tally by me.
- GP and NP are **expected, confirmed only by a read from the live book in Tally**: closing stock is valued at each item's one fixed
  purchase rate (so FIFO, average and last-cost agree); the negative item (Packing Tape Roll, -40 Nos) is taken at 0 because Tally sent no value
  for such a line when measured (that is an assumption); `expected.json` also gives the figures if it is valued at its rate.
  Two items (LED Bulb 9W, Cotton Floor Mop) sell out exactly and show closing quantity 0.
- The gate's text scan passes for the party list, masters, expected answers, receipt and the two checkers. It reports one finding class on
  the voucher files and the generator: `idshape:guid` (the fixed REMOTEIDs and the generator's namespace id, all derived from fixed text, none real).
  The real-figures layer flagged three round amounts in a first version; the numbers were changed and every amount now passes.

## Answers (see `expected.json`; computed twice)
`gen_demo_book.py` computes them from its model. `verify_from_xml.py` was written without sight of the generator or `expected.json` and recomputes
them from the import XML only (`python3 -I verify_from_xml.py import > recomputed.json`); `python3 -I compare_answers.py` compares: 17 answers, 0 differ.
Overdue means a sales bill dated more than 90 days before 31 Mar 2026 with an unpaid balance after receipts; there is no credit period (due date = bill date).
No bill is dated 28 Dec to 3 Jan or 26 Feb to 3 Mar, so no answer sits on the 30/31 or 90/91-day boundary.
The cash pair is not a released MCP tool (`cash_payments_40a3` is the tax-audit crate only): show it through a vouchers or ledger read of
Ganga Metal Works on 14 Nov 2025 (two cash payments of Rs 12,000, our numbers PAY/25-26/019 and PAY/25-26/020; Tally may number them differently).

## (c) The receipt to post live (`receipt-c.json`)
"Record a receipt of Rs 25,000 from Saraswati Paper Depot into Demo Bank Current Account dated 28 March 2026."
Saraswati has no bill older than 90 days, so the over-90 answer cannot change; its open balance goes from before to after as in `receipt-c.json`.
If the receipt posts On Account, an ageing read lists it as an advance beside the open bills instead of reducing a bill; the over-90 total is unchanged either way.
`fallback-receipt-c.xml` is the same receipt as an import file. Do not import it before recording: it is the voucher the live post is meant to create.
