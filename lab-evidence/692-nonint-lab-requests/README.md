# Lab request: a new synthetic stock company whose inventory is NOT integrated with accounts (SETUP by hand, then READ-ONLY reads)

Why: every BRIDGE book read so far is integrated with accounts. The statements code decides from the company's integration flag and from the stock lines of Tally's own Profit and Loss; what a book with inventory on and integration off looks like in those reports, and in the ledger collection, has never been captured. One small book, built by hand, answers it.

## Rules
- Synthetic company only, NEW, named exactly `BRIDGE NONINT LAB`. Never any other loaded company; no real names or numbers.
- Hand-keyed only: no import, no Bridge tool posting, nothing that writes. The files in `requests/` are reads.
- One step at a time; save every answer as received (BOM-less UTF-16LE) with size and sha256; a failed request is reported as failed, never as an empty answer. Request files are UTF-16LE with a BOM exactly as here (MANIFEST.sha256; check with `shasum -c`); do not edit them except `m-marks-TEMPLATE` (company identifier at `@@GUID@@`).
- Marks: send `m-marks` (the company row with ALTVCHID and ALTMSTID) before and after every key step and read group; equal around a read group, changing only around a key step and only for `BRIDGE NONINT LAB`. Take the company list and the data-folder listing before K0 and after the last read: every other company unchanged.
- STOP (stop, write down the screen, report, change nothing more): the name already exists (do not pick another); a prompt these steps do not describe; another company's marks or data folder changed; a read fails. After a STOP restore BK1 only if another company changed.

## K0
K0a: confirm `BRIDGE NONINT LAB` is not in the company list. K0b: BK1, a backup of the Tally data as the lab usually takes it, with name, size, sha256; restored only under the STOP rule.

## Build
K1: create the company: books beginning 1-4-2025, financial year to 31-3-2026, INR. Features: accounting and inventory ON, **Integrate accounts with inventory: NO**, GST off, batches off, cost centres off. Screenshot of the Features screen. Send `m-marks` with the company's identifier.
K2: unit `Nos`. K3: ledgers `Customer` (Sundry Debtors), `Supplier` (Sundry Creditors), `Sales` (Sales Accounts), `Purchases` (Purchase Accounts), `Lab Capital` (Capital Account, opening 1,000.00 Cr), and `Closing Stock` under the Stock-in-Hand group with opening balance 1,000.00 Dr.
K4: stock item `Lab Gadget` under Primary, unit Nos, opening 10 Nos at 100.00 (1,000.00).
K5: accounting vouchers, keyed the way this mode asks for them (no inventory allocation is needed): Purchase 5-5-2025 Supplier / Purchases 1,200.00; Sales 20-6-2025 Customer / Sales 1,600.00.
K6: closing stock for the year as a firm without integration records it: a Journal dated 31-3-2026 that moves the Stock-in-Hand ledger `Closing Stock` from 1,000.00 to 1,400.00 Dr against Profit & Loss A/c (or the method Tally's own screens offer for a closing-stock adjustment in this mode; write down exactly what was keyed). Also key the inventory side so that the item shows 12 Nos at 100.00 = 1,200.00 if Tally allows it without integration (a Stock Journal or a Purchase with an item); if it does not, say what the Stock Summary shows instead.
Screenshots: Tally's Profit & Loss and Balance Sheet for 1-4-2025 to 31-3-2026 and the Stock Summary as on 31-3-2026. Send `m-marks` after each key step.

## Reads (marks before and after the group)
n-r1 Profit and Loss (year), n-r2 Balance Sheet (year), n-r3 ledger collection (year), n-r4 group tree, n-r5 Stock Summary (year); the integration flags are the `m-marks` rows (ISINTEGRATED must read No).
Record, against what the screens show: whether the P&L answer has "Opening Stock" and "Less: Closing Stock" lines and from which source; whether the ledger collection carries the `Closing Stock` ledger with its balance; whether the Balance Sheet export balances; whether the Stock Summary closing value equals the Stock-in-Hand ledger. Not a pass/fail beyond: every read answered, HTTP 200, saved with size and sha256, marks equal.

## At the end
Company list and data-folder listing again; report the company's identifier. Leave the company in place.
Not measured: GST, batches, godowns, costing methods other than the default, a window other than the year.
