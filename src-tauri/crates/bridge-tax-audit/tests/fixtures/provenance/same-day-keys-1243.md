# Fixture provenance: four edge books for same-day order and id-hash collisions under the per-voucher keys (#1243)

Every book here is invented, with plain names: no fixture is a Tally read of any real assessee, and no figure comes from one.

## What these fixtures establish, and what they do not

The per-voucher keys (`review-fixes-1243.md` and the earlier pins of this batch) left two kinds of site that no book pinned: a list of vouchers ordered by date and then by key in which vouchers of one day have keys whose order is not the books' order or the GUIDs' order, and a row id built from a hash of the voucher's GUID in which two different GUIDs have one hash. (Earlier books already hold a same-day tie in a partner's TDS seen and in the Contra rows, enough to notice a reversed tie there; none tells the key's order from the GUID's in a partner's lists, and none has a tie in the other three lists.) Each book below is a behaviour the reference shows and the port already matches: every golden passes on the crate before any source change, so each is a regression pin, and a mutant that changes the site is killed by it (the mutant ids are in `parity/mutations.json`).

In the first two books one GUID is shared by twelve or more vouchers, the blank GUID by several, and single vouchers hold GUIDs that sort before, between and after the keys of those sharers. On the first book's first day (the TDS seen, the rows of clause 21(b)(ii)(A) and of the tax deducted) and in two of the second book's three lists (a partner's TDS seen and its negative net credits) the order the books hold the vouchers in, the order of the GUIDs and the order of the keys all differ; on the first book's second day (the rows of (ii)(B)) and in the second book's third list (the TDS attributed to no one) the keys' order differs from the books' order and is the GUIDs' order.

- `edge-books/tds_payees_same_day_keys.json`: Contractor P has 22 vouchers carrying TDS, 17 of them on one day and 5 on another. The TDS seen is named by date and, within a day, by key: twelve labels and "10 more". The rows of clause 21(b)(ii)(A), of (ii)(B), of the tax deducted and of the short deductions are numbered in the same order. Contractor U has two vouchers alike in GUID, number and day, each with a GST line, which is a line other than the expense, the payee and its TDS, so neither is rate tested: the figure counting the bills not rate tested cites that one reference twice. Four payments between two lines of the bank ledger hold the shared GUID as well and touch no payee. 122 figures, 8 findings.
- `edge-books/partners_same_day_keys.json`: Partner A has sixteen vouchers carrying TDS, fourteen of them on one day, so the cut after twelve labels falls inside that day: twelve are named and "4 more". Five vouchers of one day carry a negative net credit to that partner and are named in key order. Partners C and D share an interest ledger on which thirteen vouchers of one day carry TDS attributed to no one: twelve are named and "1 more". 34 figures, 9 findings.
- `edge-books/hvr_journal_hash_collision.json`: two different voucher GUIDs whose hashes begin with the same eight hexadecimal digits (SHA-1), each held by several journals between party ledgers. Lines that would take one row id (lines of the two GUIDs on one ledger, lines of one GUID on two journals, and both lines of one journal on one ledger, debit first on one journal and credit first on another) take a suffix in date order, the books' order within a day whatever the amounts (of two journals of one day the later holds the smaller amount). A journal under the threshold has a line on each of two such ids and takes no place; a journal of a third GUID keeps its id unsuffixed. 84 figures, 22 findings.
- `edge-books/bkq_contra_hash_collision.json`: two different voucher GUIDs whose hashes begin with the same twelve hexadecimal digits (SHA-256), one on two Contra vouchers whose narration contradicts the Cash line and the other on one: the three rows take suffixes 1 to 3 in (date, key) order, so a row takes a suffix because its hash is shared though its GUID is not. Two more vouchers of those GUIDs claim no direction and are no rows. Twelve blank-GUID Contras on scrambled days, seven of them on one day, take suffixes 1 to 12 in the same order. A single voucher whose GUID sorts between the two keys of a shared GUID, on their day, keeps its id unsuffixed. 32 figures, 19 findings.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading Tally. The reference's reader of a Tally read refuses one in which a voucher has no GUID or two vouchers share one, so only a book built directly, as these are, reaches these rows.

## How they were produced

At the reference engine (a private repository), commit `ee17d80f`, from an archive of its engine package only, under Python 3.13, run from `src-tauri/crates/bridge-tax-audit`:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The four books are written as data by a small generator, every voucher balancing and each Trial Balance row summing its vouchers; three amounts of the first book were then set by hand, its Trial Balance kept equal to its vouchers. The two pairs of GUIDs whose hashes begin alike were found by search over invented texts. They are hand-chosen scenarios, not generated from any data.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `tds_payees_same_day_keys.json` | 10,930 | `8042611534a829d3a0c4f0bf09e9406424dfdfbe8ee693c0c88df61b984a8086` | `edge-books/tds_payees_same_day_keys.json` |
| `partners_same_day_keys.json` | 12,178 | `945d3832d62c084f3c4e0acb7d1b4df216068ed842a1ce9ddba1b7c6c587c68d` | `edge-books/partners_same_day_keys.json` |
| `hvr_journal_hash_collision.json` | 5,249 | `e5d7d3d62802f127642abe146ac531abaa2098e3e523a431a6ca49d682fc86e1` | `edge-books/hvr_journal_hash_collision.json` |
| `bkq_contra_hash_collision.json` | 6,841 | `f4e096bd478a65793a6a5c980413c4ff015c2bf7b98d7942a592800e4408ab91` | `edge-books/bkq_contra_hash_collision.json` |
| `edge.tds_payees_same_day_keys.tds_payees.json` | 99,206 | `db7d2da03ddc4f874a357003a2a2b0c83d4a5c7698f397298a1bd19f25b56ce5` | `golden/edge.tds_payees_same_day_keys.tds_payees.json` |
| `edge.partners_same_day_keys.partners_40b_194t.json` | 52,907 | `4eb81b5cbf9d2b75d03edb2f1483857bacd661a25224c674e6adedf7e74c110b` | `golden/edge.partners_same_day_keys.partners_40b_194t.json` |
| `edge.hvr_journal_hash_collision.high_value_register.json` | 67,341 | `623ebd8693d9927d383f7298ce584ad778745d5f287579691ea9668a34918fb1` | `golden/edge.hvr_journal_hash_collision.high_value_register.json` |
| `edge.bkq_contra_hash_collision.book_keeping_quality.json` | 37,712 | `3583acdda03079df4abdb356bb2b2e6e2e7a1ea8030170fc0c7b28feab3e23c8` | `golden/edge.bkq_contra_hash_collision.book_keeping_quality.json` |
