# Fixture provenance: TDS inside a repayment, and nil lines in the loan rows (#1259 items 2 and 3)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

The reference made two changes to `loans_interest` after item 1 of #1259 (`loans-nil-two-loans-1259.md`).

Item 2. A repayment booked net of TDS (Loan Dr 10,000 / TDS Payable Cr 1,000 / Bank Cr 9,000) holds its TDS in its loan line, so the repaid total already has it. LOAN-1 added the TDS deducted on the loan to the expected movement as well and fired on every such voucher (and on a loan taken carrying a TDS line). The reference now publishes `tds_in_principal_<tag>`, the part of `tds_on_loan_<tag>` on the vouchers counted in the loan's taken or repaid rows, only where there is some, citing those vouchers; LOAN-1 subtracts it, and first re-derives it from the vouchers the figure cites (by GUID, among the population, not Contra, with a line with an amount on the loan), adding a LOAN-1 line where the re-derivation differs. `tds_on_loan_<tag>` itself does not move.

Item 3. The ledgers of a voucher's nil (zero-amount) lines are no longer part of its "other ledgers" in `compute_loan_rows` (an interest journal beside a nil line was read as a loan taken; a nil cash line made a bank loan a cash one; a nil line named a ledger among a row's counter ledgers), in LOAN-2 (D)'s stray-ledger set and in LOAN-3's re-derivation of a paired loan's interest journals. Three sites; the other places the reference builds a voucher's other ledgers stay as they were.

- `edge-books/loans_interest_tds_in_principal.json` (a firm, four loans): Loan Repaid, a repayment net of TDS (`tds_in_principal` 100000, LOAN-1 silent); Loan Taken, a loan taken carrying a TDS line (100000, silent); Loan Mixed, an interest journal with TDS beside a repayment net of TDS (`tds_on_loan` 300000, `tds_in_principal` 200000, the repayment's part only); Loan Journal, TDS only on an interest journal (no `tds_in_principal` figure). No module check line. 66 figures, 3 findings.
- `edge-books/loans_interest_two_sided_tds.json` (one loan): a voucher both crediting and debiting the loan and carrying TDS is listed, not counted in taken or repaid, so there is no `tds_in_principal` and LOAN-1 still fires (expected 700000, moved 600000).
- `edge-books/loans_interest_shared_guid_tds.json` (three loans; its repayments stay under the amount at which a clause 31 row is listed, so no row repeats an id): Loan Pair, two repayments net of TDS on one GUID, one date and one voucher type (`tds_in_principal` 200000, the re-derivation by GUID sums both, no line); Loan Clash, a repayment net of TDS and a second voucher carrying TDS (Loan Dr 200000, TDS Payable Cr 200000, no bank line; not counted in taken or repaid) on one GUID, date and type: `tds_in_principal` 100000 but the re-derivation by GUID takes the journal's TDS too, so the reference writes `LOAN-1: ... tds_in_principal_<tag> says 100000p but the lines on the TDS ledgers of the vouchers it cites on this loan are 300000p`; the tie itself holds. The line LOAN-1 prints for Loan Clash is the reference's own output on a GUID shared with a voucher outside the taken and repaid rows; the port prints it for parity, and a Tally read cannot hold a repeated GUID, so only a book built directly reaches it. Loan Wide, a repayment net of TDS sharing its GUID with a TDS journal that has no line on the loan, a journal whose only line on the loan is nil and a Contra with a line on the loan and a TDS line (the Trial Balance rows leave the Contra out): none is read, the re-derivation gives 100000 as the figure does and there is no line. 42 figures. Two rows on one GUID that reach clause 31 make the reference refuse the whole test with a duplicate figure id, so the book keeps them out of the window.
- `edge-books/loans_interest_tds_in_principal_untied.json` (one loan): a repayment net of TDS beside a Trial Balance debit 500000 above its vouchers (a voucher dropped): LOAN-1 fires and its text reads `+ TDS 0p`, the repayment's TDS being left out of the TDS it adds.
- `edge-books/loans_interest_nil_line_interest_journal.json` (three loans): Loan One's interest journal beside a nil line on the other configured loan (Loan One B), and Loan Two's beside a nil bank line (Bank A), are interest (`interest_total` 1000000 and 2000000, no taken row, no clause 31 total).
- `edge-books/loans_interest_nil_line_taken_rows.json` (two loans): a loan taken by bank beside a nil cash line stays a bank loan (mode `bank`); a bank repayment and a bank loan taken a day apart, one amount and one narration with a bank reference, each beside a nil line on Suspense, are a possible bank debit and its return (`clause31_reversal_pair_count` 1), because Suspense is no longer among the rows' counter ledgers.
- `edge-books/loans_interest_nil_line_misposted.json` (two loans, one shared interest ledger): a voucher posting to Loan Q against only Loan P's interest ledger, beside a nil line on Suspense, is a possible misposted interest entry, and LOAN-2 (D) does not name Suspense as a stray ledger (no module check line).
- `edge-books/loans_interest_nil_line_shared.json` (one loan, its interest ledger declared shared): an interest journal beside a nil line on Suspense is an interest journal of the paired loan, in LOAN-3's re-derivation as in the figures (no module check line).
- Not reached: the behaviours the reference still has (`charge_shaped` counting a nil bank line; a nil-only ledger printed among `interest_tds_ledgers` and the misposted finding's tags; LOAN-1 on TDS on two loans on one side; a loan taken with a TDS-payable debit; LOAN-3 on an undeclared shared ledger silenced by a nil line).
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading Tally. The evidence for real books is local parity on real reads, never committed.

## How they were produced

At the reference engine (a private repository), commit `ee17d80f` (it holds both changes), under Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The books are hand-written scenarios, not generated from any data. Each golden was regenerated under four different hash seeds with identical bytes. The 18 committed edge goldens of `loans_interest` regenerate byte-identical at that commit, so none moves.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `loans_interest_tds_in_principal.json` | 4,998 | `5b6a7886a9dba76a1c32a79517d2a49ca000405dc5b4085d4e14bf6ddcb5cd6a` | `edge-books/loans_interest_tds_in_principal.json` |
| `loans_interest_two_sided_tds.json` | 2,143 | `cec9b83fdb2fe5eb4e11f771d93e7587e4f5382363d2e2620d14e0d8f07bd7f6` | `edge-books/loans_interest_two_sided_tds.json` |
| `loans_interest_shared_guid_tds.json` | 5,055 | `7b018663b9555eb6aafb9a17b5c3fcc73be921557585543fb8dc1fec4993b988` | `edge-books/loans_interest_shared_guid_tds.json` |
| `loans_interest_tds_in_principal_untied.json` | 2,245 | `d422042444fed48e114f528b9cf7ce9e6ae01b60c187c5d56f6f8ddc20b2c728` | `edge-books/loans_interest_tds_in_principal_untied.json` |
| `loans_interest_nil_line_interest_journal.json` | 3,754 | `7f2ef8e684039f281ea77f92295b0baa08602637563cfcdf48c51e09f7b41ccc` | `edge-books/loans_interest_nil_line_interest_journal.json` |
| `loans_interest_nil_line_taken_rows.json` | 3,422 | `2d789afba12b3ae3688da261f3736d3c7569aa818c1b8ba41754395bfc6a2833` | `edge-books/loans_interest_nil_line_taken_rows.json` |
| `loans_interest_nil_line_misposted.json` | 3,353 | `386c92a5d9ad0b763296320c51617cb753645ef07087cfc18b65f44baf02ace4` | `edge-books/loans_interest_nil_line_misposted.json` |
| `loans_interest_nil_line_shared.json` | 2,896 | `7dfdeed55ea0af1fd1eafedfe3cde6c775fc53f2337b53080a5188140f6fa751` | `edge-books/loans_interest_nil_line_shared.json` |
| `edge.loans_interest_tds_in_principal.loans_interest.json` | 37,046 | `c9903a1a6f11970fb0570bd2e87cf5bd2a9f323ceeb84208c40b2b8ac382e823` | `golden/edge.loans_interest_tds_in_principal.loans_interest.json` |
| `edge.loans_interest_two_sided_tds.loans_interest.json` | 18,647 | `eae20bf1610f8ada66c5aff34a2d5c9424fdf6439d56d43c12316675cf655eb3` | `golden/edge.loans_interest_two_sided_tds.loans_interest.json` |
| `edge.loans_interest_shared_guid_tds.loans_interest.json` | 21,585 | `3f1026bf463d99c8cad40d4009287e397fb8fda67959c710d66ad90180b04a2d` | `golden/edge.loans_interest_shared_guid_tds.loans_interest.json` |
| `edge.loans_interest_tds_in_principal_untied.loans_interest.json` | 14,491 | `c8cdb469e3280c144449ce87bbcbf1e70c9c91595d0e80b2408bc5da1fe07e40` | `golden/edge.loans_interest_tds_in_principal_untied.loans_interest.json` |
| `edge.loans_interest_nil_line_interest_journal.loans_interest.json` | 22,045 | `b3c1058972761c14df5f9f1b04019c6a01cb84a0240eb9f8c0de606ee2299493` | `golden/edge.loans_interest_nil_line_interest_journal.loans_interest.json` |
| `edge.loans_interest_nil_line_taken_rows.loans_interest.json` | 26,557 | `744d6573dae8a0c93ef3a25a5ed6ab5982b24d5d058ffefdd23ebeb6f3271728` | `golden/edge.loans_interest_nil_line_taken_rows.loans_interest.json` |
| `edge.loans_interest_nil_line_misposted.loans_interest.json` | 24,372 | `fbdb82d981bed19cb42b17f5bd94c227448ebe6f96c3692d835097298b96a812` | `golden/edge.loans_interest_nil_line_misposted.loans_interest.json` |
| `edge.loans_interest_nil_line_shared.loans_interest.json` | 13,827 | `1455c091edff41f89f7abe1cee3726d999711b92df82cdd8dd7a8937f82aa982` | `golden/edge.loans_interest_nil_line_shared.loans_interest.json` |
