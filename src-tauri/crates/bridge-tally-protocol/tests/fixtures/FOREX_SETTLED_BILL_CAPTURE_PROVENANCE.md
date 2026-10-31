# The `forex_settled` Bills and rupee-control captures: provenance

The Bills reports, ledger snapshot and group snapshot of a book whose parties are kept in rupees
but hold bills entered in dollars, as TallyPrime 7.2 sent them (bridge#2071). A bill settled in
full in dollars at a rate other than its own is sent in the other side's report with an empty
`BILLCL` (TALLY_PROTOCOL_REFERENCE §8.2d).

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)** 7.2, `education_mode` false, one loaded
  company.
- **Company:** `BRIDGE FOREXGROUP LAB`, the synthetic book of
  `FOREX_72_UNSPACED_CAPTURE_PROVENANCE.md`.
  - Its first sitting kept four vouchers: two dollar sales, a dollar receipt against the second,
    and a rupee Journal.
  - The second sitting added, on screen:
    - stage 1: a rupee customer with a part-settled rupee bill (R1);
    - stage 2: a dollar supplier with two purchases (P1, P2), a payment for each, and a part receipt against the first sale.
  - Every keyed rate was a whole number.
- **Requests:** the bytes Bridge's own builders emit, for books from 1 April 2025, as of
  30 April 2025, each sent once with curl and the answer kept byte for byte:
  - the Bills Receivable and Bills Payable reports (`render_native_bills_request`);
  - the ledger snapshot (`render_native_ledger_snapshot_request`);
  - the group snapshot (`render_native_group_snapshot_request`).
- **Sources:**
  - `bills_payable_forex_settled_empty_closing`: `raw/bills-payable.response` on
    `lab/2071-forex-rerun-answers` at `0d7eb0bf` (2026-10-29, 02:17 UTC). The same 476 bytes
    were answered three more times in the second sitting, in rounds A, B1 and B2.
  - `bills_receivable_forex_rupee_control`, `ledger_snapshot_forex_rupee_control` and
    `group_snapshot_forexgroup_lab`: `raw/b1-bills-receivable.response`,
    `raw/b1-ledger-snapshot.response` and `raw/b1-group-snapshot.response` on
    `lab/2071-forex-reads-answers` at `ea28bde0` (2026-10-30, 19:56 UTC, after stage 1).
  - `bills_receivable_forex_settled_supplier`: `raw/b2-bills-receivable.response` on the same
    branch (2026-10-30, 20:11 UTC, after stage 2).
- **Encoding:** BOM-less UTF-16LE, the undecoded wire bytes. Each sha256 equals the lab's own
  `SHA256.txt` line. Merge-gate manifest entries: the five `native/` paths below (bridge#2216).

| file | bytes | sha256 |
|---|---|---|
| `native/bills_payable_forex_settled_empty_closing.utf16le.xml` | 476 | `b5a4bd6e0ea384076feba7c1157630a6fe0d2c09af00fc29444d845d470b6320` |
| `native/bills_receivable_forex_rupee_control.utf16le.xml` | 944 | `3bea534121ca1d9582db902a0cd2f69b6a1b3de45157e82acaaae086fcbf85c6` |
| `native/bills_receivable_forex_settled_supplier.utf16le.xml` | 1,372 | `54b8a1b77e425f8c7879b8987d904ab7d6a8049e4018c1d38a2257690a5377cc` |
| `native/ledger_snapshot_forex_rupee_control.utf16le.xml` | 10,730 | `6cde4e9d2b01e8f298833a25d04c846e36d34de1a7bc44b2a07e771f1e35fb9a` |
| `native/group_snapshot_forexgroup_lab.utf16le.xml` | 54,280 | `de8ad96bc29a90ca10dc82f206e5988c74ebd5e3a82ca86ab8c8d34dfbbeccfc` |

## What the bytes show (one book, two sittings: PARTIAL)

- **Bills Payable:** one row, the dollar customer's bill 2, settled in dollars at another rate,
  with `BILLCL` empty.
- **After stage 1, Bills Receivable:** R1 (the rupee customer, a plain amount) and the dollar
  customer's open bill 1 (a plain rupee amount).
- **After stage 2, Bills Receivable:** R1, bill 1, and the supplier's P1, paid in full at another
  rate, with `BILLCL` empty. P2, paid at its own rate, is in neither report.
- **The ledger snapshot after stage 1:** composites on every ledger a dollar entry touched, each
  with a whole or empty rate. The rupee customer and Cash are plain.
- **Not committed:** the snapshot after stage 2, whose Profit & Loss balance carries a rate with
  decimals (bridge#2071's follow-up).
