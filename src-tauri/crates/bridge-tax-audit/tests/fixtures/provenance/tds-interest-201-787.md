# Fixture provenance: `tds_interest_201` (#787)

The fourteen edge books, their 42 goldens and the synthetic golden come unchanged, byte for byte, from the
spec pack `docs/tax-audit/spec-packs/tds_interest_201/` (#1353), whose `HASHES.md` lists the same bytes and
SHA-256 for each file. Every book there is invented, with synthetic ledger names: no book is a Tally read of
any real assessee, and no figure, name or narration comes from one.

## What these fixtures establish, and what they do not

- The books are written scenarios, each built to reach a rule or boundary of the reference engine's
  `tds_interest_201` test or of the assembly of its rows; the pack's README lists which, and each book's
  `comment` says so voucher by voucher. They are regression fixtures: the port reproduces every golden, so
  it agrees with the reference on these books, and nothing more.
- `ti_empty`, `ti_quiet` and `ti_not_deductor` give byte-identical `tds_interest_201` goldens: with no row
  the dump does not depend on the book. They differ in why there is no row.
- Each book also names `tds_payees` and `partners_40b_194t`. Their goldens here are the two input tests'
  results on the same books, made in the same run: what this test was given. The two ports are compared
  with them too; they add nothing to those tests' own contracts.
- The goldens cover only rows the reference's own assembly builds. The test also accepts rows no caller
  builds (a TCS section, a date of deduction or deposit, a payee filing date); this port's row type cannot
  hold them, and no file here says anything about them.
- They prove nothing about reading Tally: the books are built directly in the edge-book shape, not read
  from a company. They say nothing about the law either: the limits, rates, month counting and the reading
  of when a credit attracts TDS are the reference's, carried as they are. Agreement on real books is
  established separately, by a local parity run on real reads that is never committed.
- `golden/synthetic.tds_interest_201.json` is the test on the crate's synthetic read, with the rows
  assembled from the two input tests on the same read: no row, so the seventeen totals and counts, no
  finding, and the read's own four book-level violations.

## How they were produced

At the reference engine (a private repository), at the commit the pack's README names in its first line, under Python 3.13, with
`parity/edge_golden.py` extended by the runner the pack's README gives (section 15), which this change adds
to the crate's copy, with its `parity/python_golden.py` line. The runner runs the file's `tds_payees` runner
and the `partners_40b_194t` test on the book, hands both results to the reference's own assembly of the
rows with the same inputs those two took, and runs the test with the rows and the rules' audit-report date.
Run from `src-tauri/crates/bridge-tax-audit`, once per book (each run writes the book's three goldens):

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The synthetic golden is the same test on the crate's synthetic engagement, through
`parity/python_golden.py`'s runner for it:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.tds_interest_201.json \
        --test tds_interest_201

Running every book a second time, into a new directory, reproduced every golden byte for byte; the
synthetic golden, made twice, was byte-identical both times. The goldens are regenerated only by the
reference's maintainers.

## Declared behaviour differences

No golden reaches any of these. Each is in the module's own documentation, and each names the test that
covers it.

- Two rows whose tags coincide, or two groups whose group tags coincide, are refused, as the crate refuses a
  repeated figure id, where the reference's later row or group overwrites the earlier one's figures. A tag is
  8 hex digits, so whether a book reaches this grows with its size: about n² in 2³³ for n rows, near 1% at
  10,000 rows. No test in the module makes two tags coincide; the refusal is `TestResult::fig`'s, covered by
  `a_repeated_figure_id_is_refused_with_its_typed_error` in `src/findings.rs`.
- A s.194H row when the rules' `[tds_rates]` has no `s194h_bp` is refused; the pack does not say what the
  reference does there (`assembly_refuses_without_tds_rates_a_missing_entry_and_194h_without_a_rate`). A
  s.194J row of a category other than the four is refused too (`s194j_categories_take_their_rates`).
- An as-of date that is not ISO is refused at the start of the run; the pack does not say what the reference
  does with one [unverified] (`a_row_with_no_credits_and_a_non_iso_as_of_date_are_refused`).
- The module check (TDSI-1) recomputes only a row whose deducted date is not supplied, the only kind the
  caller builds; for a row with one it checks only that no still-not-deducted amount is published
  (`each_tdsi_1_message_fires_on_its_own_tampering`).
- The module check refuses a result that has no `as_of` figure or no rate figure, where the reference raises
  (`an_absent_deducted_date_reads_as_not_supplied_and_tags_come_in_order`).

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `ti_194c.json` | 7,729 | `cc8bbe8621bfe1999bfddbe95d73df3f51032db6ccf7a146e5b5f58849e77fb6` | `edge-books/ti_194c.json` |
| `ti_base.json` | 4,217 | `23a4fb76f1563984b3bfd94747ca03b05cf1ea526e969dcf87e55340d4bbc93d` | `edge-books/ti_base.json` |
| `ti_empty.json` | 1,157 | `8d8e025ae55a27f600f04fd620a855f81d00aca96b69bfe39bcee7679c9ed658` | `edge-books/ti_empty.json` |
| `ti_month_edges.json` | 5,632 | `81ab147904931865aba55abd9b6c2cb11f7c7f172b34cd1d7ae3e9a1ebdeac92` | `edge-books/ti_month_edges.json` |
| `ti_names.json` | 5,713 | `ff67ecd43e922d327406c7047e7f651537d7a5fd98ae7439dd089c3b1da59145` | `edge-books/ti_names.json` |
| `ti_not_deductor.json` | 2,242 | `d9267c16d487b16c607a04ed903d7ffaa8f39df6d3d2b5336944e129efd90569` | `edge-books/ti_not_deductor.json` |
| `ti_partners.json` | 6,446 | `5a863baa2585a92850ef5035ef42218dbf8380cb2567046b92f0a89dfb4f8fe2` | `edge-books/ti_partners.json` |
| `ti_placeholder.json` | 3,019 | `38ba09a18e3c8e4d21fe18f18167d1b8603156dc3fe00d199a3504c39b9cb75e` | `edge-books/ti_placeholder.json` |
| `ti_quiet.json` | 12,300 | `dd15f0d3f021eeb22463cce7ddf32ab9abd6a530e9c9c8d66adf5fd67b72c2e4` | `edge-books/ti_quiet.json` |
| `ti_rounding.json` | 6,350 | `edf59f1928958b2630dc4eda5aa4a93929de8d6fd5e9c0f50e6e618f1509e834` | `edge-books/ti_rounding.json` |
| `ti_rules_default.json` | 3,131 | `a38f3ad636cfa7e9c37b6555002c16b1fa14f7eabe78fefc73755d6a00d1ff84` | `edge-books/ti_rules_default.json` |
| `ti_sections.json` | 7,792 | `f13cafc09e12307be644fe0e022dce691b30999efbb1feef8df2d9f726e5cb81` | `edge-books/ti_sections.json` |
| `ti_shared_guid.json` | 5,020 | `a7d41444d5d4e108d1de37a4d137c886a710d8dc4066f5fa0a487bb8c17381ef` | `edge-books/ti_shared_guid.json` |
| `ti_status_unknown.json` | 2,032 | `44be4bfc33adbd970a36ac8b2c8e08cbc8e04979a8fedf2dce93c1be95687ef0` | `edge-books/ti_status_unknown.json` |
| `edge.ti_194c.partners_40b_194t.json` | 4,646 | `0b4799bc763a34b4b4abbaf06d5d88a2c31ad44796fa704bdb69f8cd90807828` | `golden/edge.ti_194c.partners_40b_194t.json` |
| `edge.ti_194c.tds_interest_201.json` | 149,233 | `0288de263380fb8e14149633eb9f34edca4183239111a6a0a5eab0bde6fd63b6` | `golden/edge.ti_194c.tds_interest_201.json` |
| `edge.ti_194c.tds_payees.json` | 41,788 | `75724079407d27fce5f03289b2915ccf64e76ceddb3299271b67c5ef05b0aea5` | `golden/edge.ti_194c.tds_payees.json` |
| `edge.ti_base.partners_40b_194t.json` | 4,646 | `0b4799bc763a34b4b4abbaf06d5d88a2c31ad44796fa704bdb69f8cd90807828` | `golden/edge.ti_base.partners_40b_194t.json` |
| `edge.ti_base.tds_interest_201.json` | 41,811 | `16311347fea2a8017695fbf2edf71fbd3448ceb3c8586f69243f670e521a94a2` | `golden/edge.ti_base.tds_interest_201.json` |
| `edge.ti_base.tds_payees.json` | 33,077 | `9a0d32a2299e955881116df94827b08816971e44a66a2c72e0a5ae1eec1687b0` | `golden/edge.ti_base.tds_payees.json` |
| `edge.ti_empty.partners_40b_194t.json` | 1,049 | `d03a931902fd87eb43fcf832cc2bb5297e03da9e34d7dfb0b3a4c51c57677e73` | `golden/edge.ti_empty.partners_40b_194t.json` |
| `edge.ti_empty.tds_interest_201.json` | 7,615 | `6e5d8ba61f1b9c35cd64d37ab10df04c686456b4b31809900e37c278b9b73692` | `golden/edge.ti_empty.tds_interest_201.json` |
| `edge.ti_empty.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `golden/edge.ti_empty.tds_payees.json` |
| `edge.ti_month_edges.partners_40b_194t.json` | 4,646 | `0b4799bc763a34b4b4abbaf06d5d88a2c31ad44796fa704bdb69f8cd90807828` | `golden/edge.ti_month_edges.partners_40b_194t.json` |
| `edge.ti_month_edges.tds_interest_201.json` | 79,925 | `5f4259d2832a235a4e7d76a59af7bbab42bfebad10ab2b71ecb9ca4eef57a4e8` | `golden/edge.ti_month_edges.tds_interest_201.json` |
| `edge.ti_month_edges.tds_payees.json` | 34,566 | `5d8a1b39bd8f4ebf93b5c9ecb4645d9454cc64c7f0a0ba36c1ca891de27bdd14` | `golden/edge.ti_month_edges.tds_payees.json` |
| `edge.ti_names.partners_40b_194t.json` | 4,646 | `0b4799bc763a34b4b4abbaf06d5d88a2c31ad44796fa704bdb69f8cd90807828` | `golden/edge.ti_names.partners_40b_194t.json` |
| `edge.ti_names.tds_interest_201.json` | 76,953 | `3c6caf33451615e2f8376a7155737bb64bddbffe123e1597ef5a1c74865ef726` | `golden/edge.ti_names.tds_interest_201.json` |
| `edge.ti_names.tds_payees.json` | 41,489 | `4d492d99518ddf5d408c2f40a55edc91e54b5981d9a244f82c9ba3ed3e4aca75` | `golden/edge.ti_names.tds_payees.json` |
| `edge.ti_not_deductor.partners_40b_194t.json` | 1,049 | `d03a931902fd87eb43fcf832cc2bb5297e03da9e34d7dfb0b3a4c51c57677e73` | `golden/edge.ti_not_deductor.partners_40b_194t.json` |
| `edge.ti_not_deductor.tds_interest_201.json` | 7,615 | `6e5d8ba61f1b9c35cd64d37ab10df04c686456b4b31809900e37c278b9b73692` | `golden/edge.ti_not_deductor.tds_interest_201.json` |
| `edge.ti_not_deductor.tds_payees.json` | 19,730 | `f2094db83c9735882f46e271cf80d8b8e2821c5225f83741fc77c7d620bfaab0` | `golden/edge.ti_not_deductor.tds_payees.json` |
| `edge.ti_partners.partners_40b_194t.json` | 31,725 | `e8680861abd2a149ce577c41e9d1268cc575605db59d65a3192e80da77a32cdb` | `golden/edge.ti_partners.partners_40b_194t.json` |
| `edge.ti_partners.tds_interest_201.json` | 62,570 | `0b6182dfe57efdb7db48c7b1dcade7b8aefcfa8a85b2e1354e132073c8e56505` | `golden/edge.ti_partners.tds_interest_201.json` |
| `edge.ti_partners.tds_payees.json` | 21,449 | `b067bed2b47a641dfa0e7ca6b6d754e45b78dbaf97e83c94326eb9924c583a18` | `golden/edge.ti_partners.tds_payees.json` |
| `edge.ti_placeholder.partners_40b_194t.json` | 12,668 | `df10681c3d2a59fedba9469f3c14e30e2a586940fa4faac2f59e44af700b94ed` | `golden/edge.ti_placeholder.partners_40b_194t.json` |
| `edge.ti_placeholder.tds_interest_201.json` | 30,287 | `f49578d86086c0278a379e0c1928e8e7874972064f8bd05b3f05a23785bedbbc` | `golden/edge.ti_placeholder.tds_interest_201.json` |
| `edge.ti_placeholder.tds_payees.json` | 20,978 | `5ab216b5ed00a9a4fd22310e3eeb7a4aedcbcf3f2cf5db12a7418c052f422a92` | `golden/edge.ti_placeholder.tds_payees.json` |
| `edge.ti_quiet.partners_40b_194t.json` | 12,579 | `b8a052d5d1b69bf3c425d100ea7882d0ec8f59500cb8ea6249fad53d497297b2` | `golden/edge.ti_quiet.partners_40b_194t.json` |
| `edge.ti_quiet.tds_interest_201.json` | 7,615 | `6e5d8ba61f1b9c35cd64d37ab10df04c686456b4b31809900e37c278b9b73692` | `golden/edge.ti_quiet.tds_interest_201.json` |
| `edge.ti_quiet.tds_payees.json` | 30,023 | `e503a5192336b490e8b72d548d9d5729c0fef9abee8c2a2f55977d5dc093b72a` | `golden/edge.ti_quiet.tds_payees.json` |
| `edge.ti_rounding.partners_40b_194t.json` | 4,646 | `0b4799bc763a34b4b4abbaf06d5d88a2c31ad44796fa704bdb69f8cd90807828` | `golden/edge.ti_rounding.partners_40b_194t.json` |
| `edge.ti_rounding.tds_interest_201.json` | 174,404 | `a8dfb5ceaf6603b08d8476a23671cebbe8c4dd777fc23c8685c9ac087c9bd085` | `golden/edge.ti_rounding.tds_interest_201.json` |
| `edge.ti_rounding.tds_payees.json` | 34,585 | `92789aa4e46e82e7665064b5f26fd01e8b99565ffa5ad7e631e217b0e97a8a6d` | `golden/edge.ti_rounding.tds_payees.json` |
| `edge.ti_rules_default.partners_40b_194t.json` | 12,866 | `497dd0c98f76c6b1cf18817e2f5490358dd0ed94e41125071a42e8d28c2c32ca` | `golden/edge.ti_rules_default.partners_40b_194t.json` |
| `edge.ti_rules_default.tds_interest_201.json` | 30,420 | `730ef71e3c3cf9c44c30e89851a5f3dfb42c8eef06a36253371ac8596a79310a` | `golden/edge.ti_rules_default.tds_interest_201.json` |
| `edge.ti_rules_default.tds_payees.json` | 19,657 | `2ebac257ff1008723c99ec0668a4cb0de96ffc45064935e53812cba441beba54` | `golden/edge.ti_rules_default.tds_payees.json` |
| `edge.ti_sections.partners_40b_194t.json` | 4,646 | `0b4799bc763a34b4b4abbaf06d5d88a2c31ad44796fa704bdb69f8cd90807828` | `golden/edge.ti_sections.partners_40b_194t.json` |
| `edge.ti_sections.tds_interest_201.json` | 128,795 | `f74557300e08a183d86fb8f5260c83858c036a820e5ccd40f09cb5318d801a88` | `golden/edge.ti_sections.tds_interest_201.json` |
| `edge.ti_sections.tds_payees.json` | 45,329 | `474f93e55618496279867586190457e97dedea73de5a7c8e9ac7baab230df5e9` | `golden/edge.ti_sections.tds_payees.json` |
| `edge.ti_shared_guid.partners_40b_194t.json` | 13,165 | `7f25df4e10b3c7b4e3faeaa22a711a46aaadf1e6ce2993bed5ddda95855f1a95` | `golden/edge.ti_shared_guid.partners_40b_194t.json` |
| `edge.ti_shared_guid.tds_interest_201.json` | 55,880 | `7708b2e0ef5976dd6932da236d411664f9def854cb65cc2c1729211a99ae3d18` | `golden/edge.ti_shared_guid.tds_interest_201.json` |
| `edge.ti_shared_guid.tds_payees.json` | 25,151 | `fbc26dc48d5bb4e2e3a7efdae737a8bbf6dc4b8afb2336a25c40fd08d2c846ec` | `golden/edge.ti_shared_guid.tds_payees.json` |
| `edge.ti_status_unknown.partners_40b_194t.json` | 1,049 | `d03a931902fd87eb43fcf832cc2bb5297e03da9e34d7dfb0b3a4c51c57677e73` | `golden/edge.ti_status_unknown.partners_40b_194t.json` |
| `edge.ti_status_unknown.tds_interest_201.json` | 19,264 | `71eed038dbee27f57befb23ca608983cf01a874137cdaf11a56b0705cea1cc8d` | `golden/edge.ti_status_unknown.tds_interest_201.json` |
| `edge.ti_status_unknown.tds_payees.json` | 22,082 | `9929d7841620a470a28c84875d0c697197e9b9de644c8c11028929e707e7008b` | `golden/edge.ti_status_unknown.tds_payees.json` |
| `synthetic.tds_interest_201.json` | 8,171 | `ca3a7cc458dd4ed2596fbb0f80d2af516e8b09a5d3848dafa5161426a9f09440` | `golden/synthetic.tds_interest_201.json` |
