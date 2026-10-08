# Fixture provenance: four edge books for the per-voucher keys of tds_payees and partners_40b_194t (#1243)

Every book here is invented, with plain names: no fixture is a Tally read of any real assessee, and no figure comes from one.

## What these fixtures establish, and what they do not

The per-voucher keys of `tds_payees` and `partners_40b_194t` (the earlier pins of this batch: `tds-payees-shared-guid-1243.md`, `partners-shared-guid-1243.md`) left sites that no book separated. Each book below is a behaviour the reference shows and the port already matches: every golden passes on the crate before any source change, so each is a regression pin, and a mutant that changes the site is killed by it (the mutant ids are in `parity/mutations.json`).

- `edge-books/tds_payees_shared_guid_listing.json`: Contractor U has three s.194C bills, each also crediting a helper and carrying TDS the books do not divide, so none is rate tested; two have a blank GUID, one number and one day, the third another number. The figure counting the bills not rate tested cites the identical pair twice (three refs), where every other figure cites a voucher once. Contractor W has one bill over the single sum and two TDS journals of unique GUIDs whose GUID order is the reverse of their dates: the TDS seen is named by date. 45 figures, 4 findings.
- `edge-books/tds_payees_shared_guid_lists.json`: the client's reversal classification names a bill GUID, and the reversal's own GUID, whose first holders in the books are a transfer between bank and cash and whose second holders are the bill and the reversal: the bill counts as credited to a payee and the reversal as named, so nothing is refused or reported as matching nothing. Contractor Y has two blank-GUID bills each crediting a helper and carrying shared TDS; only the second, with its TDS, passes the single sum. 38 figures, 2 findings.
- `edge-books/partners_shared_guid_order.json`: Partner P has two reversals of one GUID read out of date order (named by date) and thirteen interest vouchers of one GUID, descending dates, each with TDS: the TDS seen is named by date, twelve of them and "1 more". Partner Q has exactly twelve such vouchers (no remainder). Partners C and D share an interest ledger; two blank-GUID payments with TDS, read out of date order, are named by date for each. 44 figures, 11 findings.
- `edge-books/partners_shared_guid_unread.json`: an interest voucher and a remuneration voucher share a GUID and each also credits a ledger the test does not read; only the interest voucher is an interest voucher crediting one, so the not-computed finding names one ledger. 14 figures, 3 findings.
- Moved elsewhere in the same change (their own provenance files carry the rows): `hvr_journal_suffix` gains a debit-first one-ledger journal (`shared-guid-rows-1195.md`), and `bkq_shared_guid_misc` changes one amount (`bkq-shared-guid-1243.md`).
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading Tally.

## How they were produced

At the reference engine (a private repository), commit `ee17d80f`, from an archive of its engine package only, under Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The four books are written as data by a small generator, every voucher balancing and each Trial Balance row summing its vouchers; they are hand-chosen scenarios, not generated from any data.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `tds_payees_shared_guid_listing.json` | 4,305 | `8f0d315a47c86f5be0ff70b592e1144c4c4b20e897eb39ef41100b660b656660` | `edge-books/tds_payees_shared_guid_listing.json` |
| `tds_payees_shared_guid_lists.json` | 4,829 | `c2689a5185c6ac73fbb3ff6dea3e30d3a841fbdb834e2f0c91f01876d4730d6b` | `edge-books/tds_payees_shared_guid_lists.json` |
| `partners_shared_guid_order.json` | 11,241 | `c318b10511b8aa5b998a3d11a113c094d00ce6ef1bd3dbd3d580f36e0573bf2c` | `edge-books/partners_shared_guid_order.json` |
| `partners_shared_guid_unread.json` | 2,998 | `b907f17693116bceb5acf4b64e432c425091b2a76c51a29cc45a3d132e498f66` | `edge-books/partners_shared_guid_unread.json` |
| `edge.tds_payees_shared_guid_listing.tds_payees.json` | 30,854 | `16864cf70f1b6d58eb9bd2197f002f43b0a525fade02235b847b6dc7133a964c` | `golden/edge.tds_payees_shared_guid_listing.tds_payees.json` |
| `edge.tds_payees_shared_guid_lists.tds_payees.json` | 24,092 | `38faae1c4eafab63714b36ede2b09998046ee2dba57a327ab26ecf1ee64f3e55` | `golden/edge.tds_payees_shared_guid_lists.tds_payees.json` |
| `edge.partners_shared_guid_order.partners_40b_194t.json` | 53,000 | `5404286640b83590c812dca9137dd12c66c48616aad71825b858b9b9534d4504` | `golden/edge.partners_shared_guid_order.partners_40b_194t.json` |
| `edge.partners_shared_guid_unread.partners_40b_194t.json` | 15,617 | `211d75146da06cfb507b689ad1a0048ddd6d0ae73e0ad32d6c78fa03e06f2ae0` | `golden/edge.partners_shared_guid_unread.partners_40b_194t.json` |
