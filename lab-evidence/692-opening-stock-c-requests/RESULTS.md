# Results: BRIDGE TIEOUT C ledger collection and group tree (read-only, 21 Oct 2026)

Run on the Windows lab PC: TallyPrime 7.1 Silver (licensed mode), XML gateway on port 9001, BRIDGE TIEOUT C (company number 100069, GUID 824929d5-39e3-4ab7-a67d-a4efc503e24d) loaded alone. Requests sent byte-exact from `requests/` (MANIFEST.sha256 3/3 checked before sending), one at a time, POST with `Content-Type: text/xml; charset=utf-16`. Answers saved as received (BOM-less UTF-16LE) in `answers/`; sizes and sha256 in `ANSWERS.sha256`. Every answer had response header `RESPSTATUS: 1`.

Voucher mark before: 3 (master 213), read through the installed Bridge extension 0.6.0 (`vouchers` for 20250620, snapshot). After all three: 3 (master 213). Not moved.

| Request | HTTP | Bytes | Result |
|---|---|---|---|
| c1 List of Ledgers, 1 Apr 2025 to 31 Mar 2026 | 200 | 10,030 | PASS: root ENVELOPE, 6 ledger rows of this book |
| c2 List of Groups | 200 | 54,280 | PASS: 28 group rows, each with BRIDGECOMPANYGUID 824929d5-39e3-4ab7-a67d-a4efc503e24d |
| c3 List of Ledgers, 1 Jul to 30 Sep 2025 | 200 | 9,962 | PASS: root ENVELOPE, 6 ledger rows of this book |

c1 rows (opening / debit totals / credit totals / closing, as sent; empty = tag empty):
- Cash (Cash-in-Hand): 0.00 / - / - / -
- Lab Customer (Sundry Debtors): 0.00 / -1600.00 / - / -1600.00
- Lab Purchases (Purchase Accounts): 0.00 / -2600.00 / - / -2600.00
- Lab Sales (Sales Accounts): 0.00 / - / 1600.00 / 1600.00
- Lab Supplier (Sundry Creditors): 0.00 / - / 2600.00 / 2600.00
- Profit & Loss A/c (reserved Primary, sent as `&#4; Primary`): 0.00 / - / - / 0.00

c3 rows: Cash 0.00 / - / - / -; Lab Customer -1600.00 / - / - / -1600.00; Lab Purchases 0.00 / - / - / -; Lab Sales 0.00 / - / - / -; Lab Supplier 2600.00 / - / - / 2600.00; Profit & Loss A/c 640.00 / - / - / 640.00.

Observations, not interpretations: neither ledger answer has a Stock-in-Hand ledger row; the book's opening stock (10 @ 100) does not appear as a ledger in this collection. In the July window Profit & Loss A/c opens at 640.00 while Lab Sales and Lab Purchases open at 0.00.
Nothing was keyed, so no restore. No Education mode or licence dialog appeared.
