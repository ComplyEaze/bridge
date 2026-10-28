- `gst_by_counter_ledger` totals, for a period, the GST the books record on ledgers under Duties & Taxes per counter
  ledger (the one Purchase Accounts, Direct Expenses, Indirect Expenses or Fixed Assets ledger on a purchase, debit
  note, journal or payment with a supplier on it, named with its GUID), debit and credit side apart, per head, and sets
  every other voucher with GST aside under a named reason with its own totals (a voucher with two such ledgers, a
  reverse-charge or set-off journal, a sale or credit note, a tax payment with no supplier, a duty ledger not typed GST,
  TDS or TCS, a ledger under another group). Books only: not a return figure, and no eligibility or availment decision.
  Measured on fourteen vouchers typed on TallyPrime 7.2's screen in a synthetic book, item invoices among them. A
  second synthetic book on 7.2, with a registered company and party GSTIN, was read with an earlier build of this tool
  that did not yet admit Direct Expenses or Fixed Assets ledgers or name the ledger GUID: for December's six vouchers the
  registration changed no decision, and a supplier invoice entered twice (the same reference, date and bill) was
  totalled twice with nothing flagged, so the answer does not detect a duplicate invoice. Gold, other releases and a
  registered book on the build of this release are not measured (part of #1866).
