# Fixture provenance: the `applicability_44ab` re-pin (2 Oct 2026)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What changed in the reference, and what these fixtures establish

The reference engine's `applicability_44ab` gained, after the last pin, a question under s.44AB(b): a profession's tax audit turns on its gross receipts, which the books' sales figure is not, and whether the client carries on a profession is configuration (`[deductor].activity`). The test takes a new `deductor_activity` argument and, when the client is recorded as a profession or both, or when the activity is not recorded and s.44AB(a) is not already "yes", it adds one finding, `applicability_44ab/profession_44ab_b` (judgement required; a question only; the limit read from the rules table, `[s44ab].profession_gross_receipts_paise`). Nothing else in the dump changed: no figure, no figure definition and no other finding differs from the earlier golden.

- `golden/synthetic.applicability_44ab.json` (regenerated) gains that finding: the synthetic engagement records no activity and s.44AB(a) is "no".
- `ap_profession`: activity "profession", turnover between the two thresholds, cash share within 5% (s.44AB(a) undetermined): the question names the profession and the limit; a full-coverage GSTR-1 turnover is differenced; presumptive history supplied.
- `ap_both`: activity "both", turnover over the highest threshold (s.44AB(a) yes): the question is still asked, with the receipts-not-split wording; a partial-coverage GSTR-1 turnover is stated and not differenced; the cash share is not supplied; no history.
- `ap_unrecorded_no`: activity not recorded, s.44AB(a) "no": the question is asked, with the not-recorded wording.
- `ap_unrecorded_yes`: activity not recorded, s.44AB(a) "yes": no question.
- `ap_unrecorded_undetermined`: activity not recorded, turnover between the two thresholds and the cash share within 5% (s.44AB(a) undetermined): the question is asked with the not-recorded wording, which only the undetermined case reaches without a recorded profession.
- `ap_business`: activity "business", a profession-looking entity type, turnover not supplied: no question, and the s.44ADA flag is "yes".
- The rules table: `profession_gross_receipts_paise` (50 lakh rupees, `status = "verified"` in the reference) is vendored as its own verbatim block of `rules/ay2026-27.s44ab.toml`; the reference rules file is unchanged since the last vendoring (same SHA-256), so the source constants stand.
- Regression fixtures only: they prove the port and the reference agree on the same inputs, and nothing about reading Tally. The evidence for real books is local parity on real reads, never committed.

## How they were produced

At the reference engine (a private repository), commit `c62a4ab4` (the last change to its `tae/` and `selftest/`), under Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.applicability_44ab.json \
        --test applicability_44ab --turnover-inputs tests/fixtures/synthetic-turnover-inputs.json
    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/ap_NAME.json tests/fixtures/golden

The edge books are hand-written scenarios, not generated from any data. The synthetic golden's own byte row stays in `PROVENANCE.md`.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `ap_profession.json` | 877 | `180d506bf42da24e76dac5abf648571d38108a741fed04483bde36a97ae4957c` | `edge-books/ap_profession.json` |
| `edge.ap_profession.applicability_44ab.json` | 11,295 | `e94fef99cd968521befbaa351b99417846ae19e345e848a0500c6fc3e313afe4` | `golden/edge.ap_profession.applicability_44ab.json` |
| `ap_both.json` | 770 | `e484221da54ab99487de5af09b5a4e9746cb50859e38479a4f18578223568da0` | `edge-books/ap_both.json` |
| `edge.ap_both.applicability_44ab.json` | 10,201 | `3a8e99495b6b6fa21618a3109ea587a5b610e9df8162f66f9998d4525c0d1c96` | `golden/edge.ap_both.applicability_44ab.json` |
| `ap_unrecorded_no.json` | 601 | `ab1d9b9cb2a5bdac7bd237601d7217b9090a669a8cea280cd36dd19e0b4f7f8f` | `edge-books/ap_unrecorded_no.json` |
| `edge.ap_unrecorded_no.applicability_44ab.json` | 9,810 | `af4312ba550dbf1f0b7462142d621ccaa6dffdfdf578ff8a9ee0ac09a0e7e9cd` | `golden/edge.ap_unrecorded_no.applicability_44ab.json` |
| `ap_unrecorded_yes.json` | 446 | `28e399f78ced2142e2cf55e8dbfc533ab3200ab294419c7c6d45123b58241e0b` | `edge-books/ap_unrecorded_yes.json` |
| `edge.ap_unrecorded_yes.applicability_44ab.json` | 8,582 | `e2e8481ff54ce5ed97bf25e9e8f7b269812cb8b2a5a847610eef1574948d85e4` | `golden/edge.ap_unrecorded_yes.applicability_44ab.json` |
| `ap_unrecorded_undetermined.json` | 695 | `519d9b1022b0bae1c6add8f89b2ce275090458f5ae0d42e301f6357884e52119` | `edge-books/ap_unrecorded_undetermined.json` |
| `edge.ap_unrecorded_undetermined.applicability_44ab.json` | 10,161 | `001f11de272f7e7e0bacb33a7fe3153f7c73da7dab214af5812e09c75208c91b` | `golden/edge.ap_unrecorded_undetermined.applicability_44ab.json` |
| `ap_business.json` | 567 | `e62128b0e9432973e01e1b1eed5c3873c01b6403b9d5c9821bd7ea00b5309171` | `edge-books/ap_business.json` |
| `edge.ap_business.applicability_44ab.json` | 8,218 | `0523df029b682b27f144345ecbfafb90e6d289dd035b6e56790bdf3a9056630e` | `golden/edge.ap_business.applicability_44ab.json` |
