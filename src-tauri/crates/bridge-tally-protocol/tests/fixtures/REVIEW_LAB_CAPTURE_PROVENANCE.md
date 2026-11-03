# `review-lab-*` — provenance

Bridge's own posts of a 50-voucher batch and a single Payment on the synthetic company `BRIDGE REVIEW LAB`, and one
`verify_import` of each, with every census spanned to the voucher mark the call read and sent (#2210). A window read sends
no census on a book whose voucher mark is 170 or less (`skips_census`, `agent_voucher_window.rs`), so the row raised the
book's mark first. These captures replace the D3 and WA1 captures of 26 Sep 2026 wherever a test needs a batch or a
payment reading back whole and matched (`D3_BATCH_CAPTURE_PROVENANCE.md`, `WA1_PAYMENT_CAPTURE_PROVENANCE.md`).

## Provenance

- **The #2210 census row, 3 Nov 2026.** Run sheet `lab/2210-census-sheet` at `1a381f08`; evidence
  `lab/2210-census-answers` at `b52cd5ec` (`RESULTS.md`, `out/relay.jsonl`, `out/bodies/`, `evidence/check.txt`,
  `copied/`). TallyPrime 7.2 Silver (licensed), `BRIDGE REVIEW LAB` (GUID `63292d7c-d541-42e5-aa9d-2ab5f8b993ff`) loaded
  alone. One `bridge_mcp` process built from master `8520f071`, behind the sheet's relay in pass mode, which kept every
  request and answer as sent. 486 requests over two connections; no relay error.
- **The book.** It stood at voucher mark 52. A set-up import of 148 Journals dated 1 April 2026, sent once with curl,
  raised it to 200. No set-up Journal falls in either window below.
- **The batch.** `build_import_xml` and `post_import` of 50 Journals, `C-001` to `C-050`, dated 2 April 2026, amounts
  1.00 to 50.00, Test Expense B Dr and Cash Cr, approved by a person in the post dialog. Tally reported creating 50; the
  post read them back verified at ALTERIDs 201 to 250, its step matched, 200 to 250.
- **The payment.** One Payment, `C-PAY`, dated 4 April 2026, 1.00, Test Expense B Dr and Cash Cr, posted the same way at
  ALTERID 251.
- **The reads.** Each window fixture is the answer to the `verify_import` of its batch, on the second connection, after
  both posts, with the marks at 251: the census request spanned `(0, 251]` (`770156b3` for 2 April, `e64bc76b` for 4
  April) and the verification read (`9e067c38` for 2 April, `a3c8f435` for 4 April). The extent (`9df2a53f`) and marks
  (`388c28bc`) answers are those calls' too. Each request sent twice in a row was answered alike both times
  (`evidence/check.txt`).
- **The catalogue** is the answer to the post path's ledger catalogue request (`215735da`), as sent during the batch's
  build and post on the first connection (`review-lab-ledger-catalogue-v2.json`).
- **The company number.** The evidence branch writes the company number as `X` at equal length in its bodies and
  journal. The extent fixture and the journal hold Tally's value again, the six digits whose answer has the SHA-256 the
  lab recorded for the answer as received (each body's file name). The extent fixture's SHA-256 equals it; the journal
  differs from the lab's copy only in its two `company_number` fields.
- **The journal and saved files** are the data folder's `agent-import-ledger.jsonl` and `imports/<batch_id>.xml`,
  copied untouched by the lab. Their origin is the relay's, `http://127.0.0.1:9002`; the tests move it to the simulator.

## What it establishes

- On this book, `verify_import` reads the batch as 50 verified and the payment as 1 verified, every census sent and
  spanned, each call in the order `SEESESEHSHSEECSCSEEVSVSEEVSVSEEHSHSE` (36 exchanges, by the relay's times; the
  lab's per-call split in `evidence/check.txt` files the payment post's last status probe and extent read under the
  batch's call, which it lists as 38).
- Tally's read-back of a native post carries no Bridge marker in a narration, and holds each voucher's own GUID as its
  `REMOTEID`. So it cannot stand for a batch imported by hand from the saved file, whose narrations keep the marker:
  `a_hand_imported_batch_with_a_duplicate_reads_unverified_in_the_markdown` stays on the D3 capture.
- Tally returns the Payment's debit as `-1.00`, which the review must show as the post dialog did, `Dr 1.00` (#730).

## What it does not establish

- Any doubt. The post read its step as matched; each test writes the doubt it reviews as a local record, stated where it
  is written.
- Any other book, release or edition, or a book whose census needs more than one span. One sitting: PARTIAL.

| file | bytes | sha256 |
|---|---|---|
| `agent/review-lab-company-extent.utf16le.xml` | 4020 | `bfdf843eb115e73a4a1c41cbdd5f61a2674a46d9f19a9f601c4c12dae4f9af77` |
| `agent/review-lab-company-high-water.utf16le.xml` | 3414 | `e470bf1c5493725cd212ced59e9e4dc455d165d33340fa4ed7259f85103c6077` |
| `agent/review-lab-batch-voucher-census.utf16le.xml` | 124242 | `61d85a9b8bc3bb0efb137b825b26af89c05ff1032e1580e5d8de7bbfaafe49c3` |
| `agent/review-lab-batch-import-verification.utf16le.xml` | 248988 | `a8a8403d83bbb942aa081bc7d37e97662ccaa543cfb99b8ebfb6e8538bd3364a` |
| `agent/review-lab-payment-voucher-census.utf16le.xml` | 5458 | `99014b3efe3748a5b6ecefb3dc26e18d9fca59fd89962176894620d44fc5807c` |
| `agent/review-lab-payment-import-verification.utf16le.xml` | 7960 | `99b619afdeb06a34eff2e033abb5cf7ed27327351d536b8827fe2423901faef6` |
| `agent/review-lab-ledger-catalogue-v2.utf16le.xml` | 6658 | `652e419a35ccf772d05adb25a3e36bca6b8fc9a966d3243b9dcf4d09623c9338` |
| `agent/review-lab-journal.jsonl` | 26244 | `37d3db93d18979e965ca0a99b190859d0a12fdf2714c098f86630f7a8c01a4ad` |
| `agent/review-lab-batch-import.xml` | 31103 | `5af715f8ae796a68a6bd9368810c24eb2d59737dd76ed37b58d8ba0e8f72cdab` |
| `agent/review-lab-payment-import.xml` | 1037 | `d736c66b4fa875c740920e8425f6901f5adde668660de34eccf788be1990cab2` |
