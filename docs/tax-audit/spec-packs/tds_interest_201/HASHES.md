# Hashes and provenance: the `tds_interest_201` spec pack

Each book here is invented, with synthetic ledger names: no book is a Tally read of any real assessee, and no
figure, name or narration comes from one.

## What these files establish, and what they do not

- The books are written scenarios, each built to reach a rule or boundary of the reference engine's
  `tds_interest_201` test or of the assembly of its rows (the README lists which, and each book's `comment`
  says so voucher by voucher). They are regression fixtures: a port that reproduces every golden agrees with
  the reference on these books, and nothing more.
- `ti_empty`, `ti_quiet` and `ti_not_deductor` give byte-identical `tds_interest_201` goldens: with no row
  the dump does not depend on the book. They differ in why there is no row.
- The `edge.<book>.tds_payees.json` and `edge.<book>.partners_40b_194t.json` goldens are the two input
  tests' results on the same books, made in the same run. They are what this test was given. Both tests
  are already ported and have their own goldens in the crate; README section 2.6 names the two points on
  which these goldens, made at a later commit of the reference, are ahead of the crate's ports.
- The goldens cover only rows the reference's own assembly builds. The test also accepts rows it never
  builds (a TCS section, a date of deduction or deposit, a payee filing date); README section 11.1 gives
  the measurement, and no file here says anything about them.
- They prove nothing about reading Tally. The books are built directly in the edge-book shape, not read
  from a company, so they say nothing about how real vouchers, group chains or TDS ledgers reach the test.
- They say nothing about the law. The limits, rates, month counting and the reading of when a credit
  attracts TDS are the reference's, carried as they are.
- Agreement on real books is not established by this pack. It will be established separately, once the
  test is ported, by a local parity run on real reads that is never committed.

## Which rules the goldens pin

Each rule below was changed alone, in a copy of the reference's source (the reference itself was not edited),
and every book was run again; the books named are those whose `tds_interest_201` golden then changed. A port
that gets one of the first table's rules wrong fails at least one golden.

The first table, the rules of the test itself, was run again for this version of the pack, at `10717095`.
The other three tables (the assembly of the rows, the tranches and the order of the input findings, the call)
are as they were measured at `ee17d80f`, on the books as they then were, and were NOT run again: the files
those rules are in (the caller, the tranche rule and the two input tests) are byte for byte the same at
`10717095`, but the goldens they are read against have more figures now, `ti_rounding` has one voucher more
and five vouchers of `ti_194c`, `ti_base` and `ti_sections` have other dates, so a list there may be a book out.

### The test

Re-run at `10717095` on the books as they are now.

| Rule changed | Goldens that change |
| --- | --- |
| the pre-deduction rate read from the post-deduction key | every book |
| the post-deduction rate read from the pre-deduction key | every book |
| the test's own default pre-deduction rate doubled | `ti_rules_default` |
| the test's own default post-deduction rate changed | `ti_rules_default` |
| the test's own default s.206C(7) rate changed | `ti_rules_default` |
| the rules' `[s201_1a]` table ignored, the test's own values always used | every book but `ti_rules_default` |
| the default-rate limit never added | `ti_rules_default` |
| the rules' `[s206c_7]` table ignored, the test's own value always used | every book but `ti_rules_default` |
| a single-rate row counted in the lower scenario only | every book with a row but `ti_194c`, `ti_rounding` |
| a single-rate row counted in neither scenario | every book with a row but `ti_194c`, `ti_rounding` |
| the row tag without the row's position | every book with a row |
| the row tag without the section | every book with a row |
| tags of 10 hexadecimal characters | every book with a row |
| months counted without the part month | every book with a row |
| the same day counted as one month | `ti_194c`, `ti_month_edges`, `ti_partners`, `ti_placeholder`, `ti_rules_default`, `ti_shared_guid` |
| months counted as periods of 30 days or part | `ti_194c`, `ti_month_edges`, `ti_partners`, `ti_placeholder`, `ti_rounding`, `ti_rules_default`, `ti_sections`, `ti_shared_guid` |
| months counted as whole months, with one more only when the day of the month is later | `ti_194c`, `ti_month_edges`, `ti_names`, `ti_partners`, `ti_placeholder`, `ti_rules_default`, `ti_shared_guid` |
| interest rounded down | `ti_194c`, `ti_rounding` |
| interest rounded half to even | `ti_194c`, `ti_rounding` |
| interest rounded up | `ti_rounding` |
| the maximum left at the pre-deduction rate to the as-of date (the reference's rule before `10717095`) | every book with a row |
| the maximum taken as deducted on the deductible date, without the search | every book with a row |
| the maximum taken as deducted the day after the deductible date, without the search | `ti_month_edges`, `ti_rounding` |
| the search for the maximum starts the day after the deductible date | `ti_month_edges` |
| the two legs of the maximum rounded together | `ti_rounding` |
| the two rates of the maximum the other way round | `ti_194c`, `ti_month_edges`, `ti_partners`, `ti_placeholder`, `ti_rules_default`, `ti_shared_guid` |
| the month of deduction counted at the post-deduction rate only | every book with a row |
| the still-not-deducted figure never published, nor its fact and its limit | every book with a row |
| the still-not-deducted figure at the post-deduction rate | every book with a row |
| the post-deduction months figure left at 0 | every book with a row |
| the minimum runs to the as-of date too | every book with a row |
| the own-date maximum never published | every book with a row |
| the own-date maximum runs each credit from the tranche's date | `ti_194c`, `ti_partners`, `ti_sections`, `ti_shared_guid` |
| the own-date maximum left at the pre-deduction rate to the as-of date | every book with a row |
| the own-date maximum with the two rates the other way round | `ti_194c`, `ti_month_edges`, `ti_partners`, `ti_placeholder`, `ti_rules_default`, `ti_shared_guid` |
| the own-date still-not-deducted figure taken from the row, not its credits | `ti_194c`, `ti_partners`, `ti_rounding`, `ti_sections`, `ti_shared_guid` |
| the own-date still-not-deducted figure rounded once, on the credits together | `ti_rounding` |
| a scenario's maximum summing the rows' maxima instead of their own-date maxima | `ti_194c`, `ti_partners`, `ti_rounding`, `ti_sections`, `ti_shared_guid` |
| the one-off total taking the own-date maximum for a one-off row too | `ti_194c`, `ti_partners` |
| the one-off total taking the plain maximum for every row | `ti_194c`, `ti_partners` |
| the one-off total treating an unclassified row as one-off | `ti_194c`, `ti_partners` |
| rows without a filing date not counted | every book with a row |
| the unclassified count also counts foreseeable rows | `ti_194c`, `ti_names`, `ti_partners`, `ti_sections` |
| the one-off figures published on every book | `ti_base`, `ti_empty`, `ti_month_edges`, `ti_not_deductor`, `ti_placeholder`, `ti_quiet`, `ti_rounding`, `ti_rules_default`, `ti_sections`, `ti_shared_guid`, `ti_status_unknown` |
| the combined one-off figure published on every book | `ti_base`, `ti_empty`, `ti_month_edges`, `ti_not_deductor`, `ti_placeholder`, `ti_quiet`, `ti_rounding`, `ti_rules_default`, `ti_sections`, `ti_shared_guid`, `ti_status_unknown` |
| the one-off count also counts foreseeable rows | `ti_sections` |
| the combined ceiling is the lower scenario's maximum | `ti_194c`, `ti_rounding`, `ti_sections` |
| the combined one-off ceiling is the lower scenario's | `ti_194c` |
| the scenario tax total also adds each row's tax to the other scenario | `ti_194c`, `ti_rounding`, `ti_sections` |
| a total published for the still-not-deducted reading | every book |
| the two-readings limit left off | every book with a row |
| the two-readings limit without the as-of date | every book with a row |
| the quarterly-statement limit left off | every book with a row |
| the no-filing-date limit left off | every book with a row |
| the no-filing-date limit keeps the clause the reference had before `10717095` | every book with a row |
| the basis left off the limits | every book with a row |
| the basis left off the tax definition | every book with a row |
| the rate note left off the tax definition | `ti_194c`, `ti_rounding`, `ti_sections` |
| the unclassified limit also on a foreseeable payee | `ti_194c`, `ti_names`, `ti_partners`, `ti_sections` |
| the unclassified limit left off | every book with a row |
| the one-off limit left off | `ti_194c`, `ti_names`, `ti_partners` |
| the deductor-status limit left off | `ti_placeholder`, `ti_status_unknown` |
| the deductor-status limit on every row | every book with a row but `ti_status_unknown` |
| the basis put before the no-filing-date limit | every book with a row |
| a group's limits not de-duplicated across its tranches | `ti_194c`, `ti_month_edges`, `ti_partners`, `ti_rounding`, `ti_sections`, `ti_shared_guid` |
| the undated maximum's definition left as the dated one's | every book with a row |
| the finding id is the first row's tag, not the group's | every book with a row |
| the group id without the section | every book with a row |
| a finding for every group, also when no maximum is above 0 | `ti_month_edges` |
| a finding only when every tranche's maximum is above 0 | `ti_month_edges`, `ti_rounding` |
| a finding lists only tranches whose maximum is above 0 | `ti_month_edges`, `ti_rounding` |
| a finding lists every tranche, also one with no tax | `ti_rounding` |
| the tranche count always plural | every book with a row but `ti_month_edges` |
| the tranche count is the group's rows, not the listed ones | `ti_rounding` |
| the own-date fact left off a finding | every book with a row |
| the still-not-deducted fact left off a finding | every book with a row |
| the own-date still-not-deducted fact left off a finding | every book with a row |
| the still-not-deducted fact points at the maximum | every book with a row |
| tranches numbered from 0 | every book with a row |
| the rate phrase left off the title | `ti_194c`, `ti_rounding`, `ti_sections` |
| clause 34(c) left off | every book with a row |
| the clause order changed | every book with a row |
| confidence `judgement_required` | every book with a row |
| a finding carries no evidence | every book with a row |
| a finding carries only its first tranche's evidence | `ti_194c`, `ti_month_edges`, `ti_partners`, `ti_rounding`, `ti_sections`, `ti_shared_guid` |
| the tax figure carries no evidence | every book with a row |
| the still-not-deducted figure carries no evidence | every book with a row |
| the rate figure left off | every book with a row |
| the module check recomputes the maximum at the pre-deduction rate to the as-of date | every book with a row |
| the module check tries only the deductible date | every book with a row |
| the module check recomputes the still-not-deducted figure at the post-deduction rate | every book with a row |
| the module check counts months without the part month | every book with a row |
| the module check rounds down | `ti_rounding` |
| the module check counts the same day as a month | `ti_194c`, `ti_month_edges`, `ti_partners`, `ti_placeholder`, `ti_rules_default`, `ti_shared_guid` |
| the module check reads the as-of date as one day later | every book with a row |
| no module check | every book |
| the search for the maximum tries only the deductible date and the day after it | `ti_rounding` |
| the module check does not try the first of the later months | `ti_rounding` |

### The assembly of the rows

As measured at `ee17d80f`; not run again (see above).

| Rule changed | Goldens that change |
| --- | --- |
| a non-deductor's payees get rows too | `ti_not_deductor` |
| the payees of an assessee of `unknown` status given no row either | `ti_status_unknown` |
| a finding with no `credited` fact not passed by | `ti_quiet` (the reference then refuses `ti_quiet`) |
| s.194H payees passed by | `ti_month_edges`, `ti_names`, `ti_partners`, `ti_placeholder`, `ti_sections`, `ti_shared_guid`, `ti_status_unknown` |
| the s.194C rates swapped | `ti_194c`, `ti_rounding` |
| the s.194-I rates swapped | `ti_sections` |
| the s.194J pair swapped (royalty and s.28(va)) | `ti_sections` |
| professional fees at the technical rate | `ti_base`, `ti_rules_default`, `ti_sections` |
| technical fees at the professional rate | `ti_sections` |
| a known s.194J category gets both rates like the others | `ti_base`, `ti_rules_default`, `ti_sections` |
| s.194H at two rates | `ti_month_edges`, `ti_names`, `ti_partners`, `ti_placeholder`, `ti_sections`, `ti_shared_guid`, `ti_status_unknown` |
| the higher-rate row before the lower-rate row | `ti_194c`, `ti_rounding`, `ti_sections` |
| the row's tax rounded down | `ti_194c`, `ti_rounding` |
| each credit's tax rounded down | `ti_rounding` |
| a row cites all of its payee's vouchers, not its tranche's | `ti_194c`, `ti_month_edges`, `ti_partners`, `ti_rounding`, `ti_sections`, `ti_shared_guid` |
| a row cites its tranche's vouchers by GUID alone | `ti_shared_guid` |
| the payee key without the tranche's date | every book with a row |
| one group per payee, not per payee and rate | every book with a row |
| the deductor flag never set | `ti_placeholder`, `ti_status_unknown` |
| the row's rate not passed | every book with a row |
| the basis text of s.194C used for every section | `ti_base`, `ti_month_edges`, `ti_names`, `ti_partners`, `ti_placeholder`, `ti_rules_default`, `ti_sections`, `ti_shared_guid`, `ti_status_unknown` |
| the credits with their own dates not passed | every book with a row |
| foreseeability never read | `ti_194c`, `ti_names`, `ti_partners`, `ti_sections` |
| a payee's flag passed for a partner too | `ti_placeholder` |
| a partner's foreseeability never read | `ti_partners` |
| a payee's foreseeability never read | `ti_194c`, `ti_names`, `ti_sections` |
| the s.194T rate fixed at the s.194H rate | `ti_partners`, `ti_placeholder`, `ti_rules_default`, `ti_shared_guid` |
| the s.194T rate read from the rules only (no default) | `ti_rules_default` (the reference then refuses `ti_rules_default`) |
| partners' rows before payees' rows | `ti_partners`, `ti_placeholder`, `ti_rules_default`, `ti_shared_guid` |
| the payees' tranches read without the TDS-payable ledgers | `ti_base` |
| the payees' tranches read without the GST-by-agreement list | `ti_base` |
| the payees' tranches read without the GST ledgers | `ti_base` |
| the partners' tranches read without the TDS-payable ledgers | `ti_partners` |

### The tranches and the order of the input findings

As measured at `ee17d80f`; not run again (see above).

| Rule changed | Goldens that change |
| --- | --- |
| a credit at exactly the single-sum limit is over it | `ti_194c` |
| a total at exactly the aggregate limit is over it | `ti_194c`, `ti_quiet`, `ti_sections`; also the `tds_payees` golden of `ti_sections` |
| the earlier credits keep their own dates at a crossing | `ti_194c`, `ti_partners`, `ti_sections`, `ti_shared_guid` |
| a reversal lowers the running total | `ti_partners` |
| credits read in book order, not date order | `ti_194c` |
| credits read in order of voucher key alone | `ti_194c` |
| a credit after the crossing is dated on the crossing day | `ti_194c`, `ti_month_edges`, `ti_partners`, `ti_rounding`, `ti_shared_guid` |
| the single-sum test ignored | `ti_194c`, `ti_rounding`; also the `tds_payees` golden of `ti_194c`, `ti_rounding` |
| s.194-I read against the monthly limit over the whole year | `ti_sections`; also the `tds_payees` golden of `ti_sections` |
| s.194J read with the s.194C single-sum test too | `ti_sections` |
| a tranche's credits merged by date | `ti_rounding` |
| one tranche per credit, not per day | `ti_194c`, `ti_partners`, `ti_rounding`, `ti_sections`, `ti_shared_guid` |
| the payees of a section taken in name order | `ti_194c`, `ti_month_edges`, `ti_names`, `ti_rounding` |
| the sections taken in another order | `ti_sections` |
| the s.194J categories taken in another order | `ti_sections` |
| the partners taken in the order the table lists them | `ti_partners` |
| the partner tranche table reading a total of exactly the s.194T limit as over it | `ti_quiet` |

### The call

As measured at `ee17d80f`; not run again (see above).

| Rule changed | Goldens that change |
| --- | --- |
| the as-of date taken from the rules' return due date | every book |
| the flag set by an `unknown` status only | `ti_placeholder` |
| the flag set by a placeholder turnover only | `ti_status_unknown` |
| the foreseeability list not passed to the assembly | `ti_194c`, `ti_names`, `ti_partners`, `ti_sections` |

That is 152 changes. The 98 of the first table each changed at least one golden at `10717095`; the 54 of the
other three tables each changed at least one at `ee17d80f`. Eight more changed no golden at `10717095`; README
section 11.3 says why:

- the group tax sums only the listed tranches.
- the combined floor is the higher scenario's minimum.
- no section is read as s.206C.
- the module check's two comparisons (a bound below 0; a minimum above the maximum) are removed.
- the module check does not look at the still-not-deducted amount.
- a group's finding is gated on the larger of its rows' two maxima.
- a group's finding is gated on its rows' still-not-deducted amounts.
- the search for the maximum stops the day before the as-of date.

Any change to a fixed text (a title, a limit, an item to ask, a definition, a basis, the population note) or
to a clause tag changes every golden that carries it.

## How the goldens were produced

At the reference engine (a private repository), commit `10717095`, under Python 3.13, with the crate's
`parity/edge_golden.py`, which is in this repository under its Apache-2.0 licence and has no runner for
this test. The goldens were made with that file extended by the runner README section 15 gives as text;
the extended file is held by the reference's maintainers and is not part of this pack. The runner runs the
file's own `tds_payees` runner and the `partners_40b_194t` test on the book, hands both results to the
reference's own assembly of the rows with the same inputs those two took, and runs the test with the rows
and the rules' audit-report date, as the reference runs the three in one pass. The input goldens come from
the file's two existing runners, because every book also names those two tests. Run from
`src-tauri/crates/bridge-tax-audit`, once per book:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python EDGE_GOLDEN_WITH_RUNNER ENGINE \
        BOOK.json OUTDIR

Each run writes the three goldens of the book. Running every book a second time, into a new directory,
reproduced every golden byte for byte.

This pack was first made at `ee17d80f`. At `10717095` the reference's maximum for a row with no date of
deduction is the largest over every date of deduction, and the amount it held before is published under its
own name (README section 3.2); `ti_rounding` gained the voucher x11, two comments were rewritten, and five
vouchers of `ti_194c`, `ti_base` and `ti_sections` were given other dates (c06, b03, s04, s05 and s09). All 43
goldens were made again: 28 came out byte for byte as they were (the three goldens with no row, the synthetic
one and 24 of the 28 input goldens), and 15 changed (the `tds_interest_201` goldens of the eleven books with
a row, and the `tds_payees` goldens of the four books whose vouchers changed). As a control before that, the 43 goldens
as first committed were reproduced byte for byte at the reference's commit `6f44ad82`, which has the earlier
rule, with the same extended file.

As a control at `ee17d80f`, not repeated, the unchanged `parity/edge_golden.py` was first run on the crate's
25 committed edge books for the two input tests: 24 goldens came out byte for byte as committed, and
`edge.partners_tds_mixed.partners_40b_194t.json` differed in one limit's wording (README section 2.6).
The unchanged `parity/python_golden.py` reproduced the crate's committed `synthetic.tds_payees.json` and
`synthetic.partners_40b_194t.json` byte for byte.

`synthetic.tds_interest_201.json` was made with the crate's `parity/python_golden.py` extended by the
runner README section 15 gives, with
`ENGINE tests/fixtures/synthetic-engagement.toml OUT.json --test tds_interest_201`; made twice, it was
byte-identical both times.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `ti_194c.json` | 7,729 | `cc8bbe8621bfe1999bfddbe95d73df3f51032db6ccf7a146e5b5f58849e77fb6` | `books/ti_194c.json` |
| `ti_base.json` | 4,217 | `23a4fb76f1563984b3bfd94747ca03b05cf1ea526e969dcf87e55340d4bbc93d` | `books/ti_base.json` |
| `ti_empty.json` | 1,157 | `8d8e025ae55a27f600f04fd620a855f81d00aca96b69bfe39bcee7679c9ed658` | `books/ti_empty.json` |
| `ti_month_edges.json` | 5,632 | `81ab147904931865aba55abd9b6c2cb11f7c7f172b34cd1d7ae3e9a1ebdeac92` | `books/ti_month_edges.json` |
| `ti_names.json` | 5,713 | `ff67ecd43e922d327406c7047e7f651537d7a5fd98ae7439dd089c3b1da59145` | `books/ti_names.json` |
| `ti_not_deductor.json` | 2,242 | `d9267c16d487b16c607a04ed903d7ffaa8f39df6d3d2b5336944e129efd90569` | `books/ti_not_deductor.json` |
| `ti_partners.json` | 6,446 | `5a863baa2585a92850ef5035ef42218dbf8380cb2567046b92f0a89dfb4f8fe2` | `books/ti_partners.json` |
| `ti_placeholder.json` | 3,019 | `38ba09a18e3c8e4d21fe18f18167d1b8603156dc3fe00d199a3504c39b9cb75e` | `books/ti_placeholder.json` |
| `ti_quiet.json` | 12,300 | `dd15f0d3f021eeb22463cce7ddf32ab9abd6a530e9c9c8d66adf5fd67b72c2e4` | `books/ti_quiet.json` |
| `ti_rounding.json` | 6,350 | `edf59f1928958b2630dc4eda5aa4a93929de8d6fd5e9c0f50e6e618f1509e834` | `books/ti_rounding.json` |
| `ti_rules_default.json` | 3,131 | `a38f3ad636cfa7e9c37b6555002c16b1fa14f7eabe78fefc73755d6a00d1ff84` | `books/ti_rules_default.json` |
| `ti_sections.json` | 7,792 | `f13cafc09e12307be644fe0e022dce691b30999efbb1feef8df2d9f726e5cb81` | `books/ti_sections.json` |
| `ti_shared_guid.json` | 5,020 | `a7d41444d5d4e108d1de37a4d137c886a710d8dc4066f5fa0a487bb8c17381ef` | `books/ti_shared_guid.json` |
| `ti_status_unknown.json` | 2,032 | `44be4bfc33adbd970a36ac8b2c8e08cbc8e04979a8fedf2dce93c1be95687ef0` | `books/ti_status_unknown.json` |
| `edge.ti_194c.partners_40b_194t.json` | 4,646 | `0b4799bc763a34b4b4abbaf06d5d88a2c31ad44796fa704bdb69f8cd90807828` | `goldens/edge.ti_194c.partners_40b_194t.json` |
| `edge.ti_194c.tds_interest_201.json` | 149,233 | `0288de263380fb8e14149633eb9f34edca4183239111a6a0a5eab0bde6fd63b6` | `goldens/edge.ti_194c.tds_interest_201.json` |
| `edge.ti_194c.tds_payees.json` | 41,788 | `75724079407d27fce5f03289b2915ccf64e76ceddb3299271b67c5ef05b0aea5` | `goldens/edge.ti_194c.tds_payees.json` |
| `edge.ti_base.partners_40b_194t.json` | 4,646 | `0b4799bc763a34b4b4abbaf06d5d88a2c31ad44796fa704bdb69f8cd90807828` | `goldens/edge.ti_base.partners_40b_194t.json` |
| `edge.ti_base.tds_interest_201.json` | 41,811 | `16311347fea2a8017695fbf2edf71fbd3448ceb3c8586f69243f670e521a94a2` | `goldens/edge.ti_base.tds_interest_201.json` |
| `edge.ti_base.tds_payees.json` | 33,077 | `9a0d32a2299e955881116df94827b08816971e44a66a2c72e0a5ae1eec1687b0` | `goldens/edge.ti_base.tds_payees.json` |
| `edge.ti_empty.partners_40b_194t.json` | 1,049 | `d03a931902fd87eb43fcf832cc2bb5297e03da9e34d7dfb0b3a4c51c57677e73` | `goldens/edge.ti_empty.partners_40b_194t.json` |
| `edge.ti_empty.tds_interest_201.json` | 7,615 | `6e5d8ba61f1b9c35cd64d37ab10df04c686456b4b31809900e37c278b9b73692` | `goldens/edge.ti_empty.tds_interest_201.json` |
| `edge.ti_empty.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `goldens/edge.ti_empty.tds_payees.json` |
| `edge.ti_month_edges.partners_40b_194t.json` | 4,646 | `0b4799bc763a34b4b4abbaf06d5d88a2c31ad44796fa704bdb69f8cd90807828` | `goldens/edge.ti_month_edges.partners_40b_194t.json` |
| `edge.ti_month_edges.tds_interest_201.json` | 79,925 | `5f4259d2832a235a4e7d76a59af7bbab42bfebad10ab2b71ecb9ca4eef57a4e8` | `goldens/edge.ti_month_edges.tds_interest_201.json` |
| `edge.ti_month_edges.tds_payees.json` | 34,566 | `5d8a1b39bd8f4ebf93b5c9ecb4645d9454cc64c7f0a0ba36c1ca891de27bdd14` | `goldens/edge.ti_month_edges.tds_payees.json` |
| `edge.ti_names.partners_40b_194t.json` | 4,646 | `0b4799bc763a34b4b4abbaf06d5d88a2c31ad44796fa704bdb69f8cd90807828` | `goldens/edge.ti_names.partners_40b_194t.json` |
| `edge.ti_names.tds_interest_201.json` | 76,953 | `3c6caf33451615e2f8376a7155737bb64bddbffe123e1597ef5a1c74865ef726` | `goldens/edge.ti_names.tds_interest_201.json` |
| `edge.ti_names.tds_payees.json` | 41,489 | `4d492d99518ddf5d408c2f40a55edc91e54b5981d9a244f82c9ba3ed3e4aca75` | `goldens/edge.ti_names.tds_payees.json` |
| `edge.ti_not_deductor.partners_40b_194t.json` | 1,049 | `d03a931902fd87eb43fcf832cc2bb5297e03da9e34d7dfb0b3a4c51c57677e73` | `goldens/edge.ti_not_deductor.partners_40b_194t.json` |
| `edge.ti_not_deductor.tds_interest_201.json` | 7,615 | `6e5d8ba61f1b9c35cd64d37ab10df04c686456b4b31809900e37c278b9b73692` | `goldens/edge.ti_not_deductor.tds_interest_201.json` |
| `edge.ti_not_deductor.tds_payees.json` | 19,730 | `f2094db83c9735882f46e271cf80d8b8e2821c5225f83741fc77c7d620bfaab0` | `goldens/edge.ti_not_deductor.tds_payees.json` |
| `edge.ti_partners.partners_40b_194t.json` | 31,725 | `e8680861abd2a149ce577c41e9d1268cc575605db59d65a3192e80da77a32cdb` | `goldens/edge.ti_partners.partners_40b_194t.json` |
| `edge.ti_partners.tds_interest_201.json` | 62,570 | `0b6182dfe57efdb7db48c7b1dcade7b8aefcfa8a85b2e1354e132073c8e56505` | `goldens/edge.ti_partners.tds_interest_201.json` |
| `edge.ti_partners.tds_payees.json` | 21,449 | `b067bed2b47a641dfa0e7ca6b6d754e45b78dbaf97e83c94326eb9924c583a18` | `goldens/edge.ti_partners.tds_payees.json` |
| `edge.ti_placeholder.partners_40b_194t.json` | 12,668 | `df10681c3d2a59fedba9469f3c14e30e2a586940fa4faac2f59e44af700b94ed` | `goldens/edge.ti_placeholder.partners_40b_194t.json` |
| `edge.ti_placeholder.tds_interest_201.json` | 30,287 | `f49578d86086c0278a379e0c1928e8e7874972064f8bd05b3f05a23785bedbbc` | `goldens/edge.ti_placeholder.tds_interest_201.json` |
| `edge.ti_placeholder.tds_payees.json` | 20,978 | `5ab216b5ed00a9a4fd22310e3eeb7a4aedcbcf3f2cf5db12a7418c052f422a92` | `goldens/edge.ti_placeholder.tds_payees.json` |
| `edge.ti_quiet.partners_40b_194t.json` | 12,579 | `b8a052d5d1b69bf3c425d100ea7882d0ec8f59500cb8ea6249fad53d497297b2` | `goldens/edge.ti_quiet.partners_40b_194t.json` |
| `edge.ti_quiet.tds_interest_201.json` | 7,615 | `6e5d8ba61f1b9c35cd64d37ab10df04c686456b4b31809900e37c278b9b73692` | `goldens/edge.ti_quiet.tds_interest_201.json` |
| `edge.ti_quiet.tds_payees.json` | 30,023 | `e503a5192336b490e8b72d548d9d5729c0fef9abee8c2a2f55977d5dc093b72a` | `goldens/edge.ti_quiet.tds_payees.json` |
| `edge.ti_rounding.partners_40b_194t.json` | 4,646 | `0b4799bc763a34b4b4abbaf06d5d88a2c31ad44796fa704bdb69f8cd90807828` | `goldens/edge.ti_rounding.partners_40b_194t.json` |
| `edge.ti_rounding.tds_interest_201.json` | 174,404 | `a8dfb5ceaf6603b08d8476a23671cebbe8c4dd777fc23c8685c9ac087c9bd085` | `goldens/edge.ti_rounding.tds_interest_201.json` |
| `edge.ti_rounding.tds_payees.json` | 34,585 | `92789aa4e46e82e7665064b5f26fd01e8b99565ffa5ad7e631e217b0e97a8a6d` | `goldens/edge.ti_rounding.tds_payees.json` |
| `edge.ti_rules_default.partners_40b_194t.json` | 12,866 | `497dd0c98f76c6b1cf18817e2f5490358dd0ed94e41125071a42e8d28c2c32ca` | `goldens/edge.ti_rules_default.partners_40b_194t.json` |
| `edge.ti_rules_default.tds_interest_201.json` | 30,420 | `730ef71e3c3cf9c44c30e89851a5f3dfb42c8eef06a36253371ac8596a79310a` | `goldens/edge.ti_rules_default.tds_interest_201.json` |
| `edge.ti_rules_default.tds_payees.json` | 19,657 | `2ebac257ff1008723c99ec0668a4cb0de96ffc45064935e53812cba441beba54` | `goldens/edge.ti_rules_default.tds_payees.json` |
| `edge.ti_sections.partners_40b_194t.json` | 4,646 | `0b4799bc763a34b4b4abbaf06d5d88a2c31ad44796fa704bdb69f8cd90807828` | `goldens/edge.ti_sections.partners_40b_194t.json` |
| `edge.ti_sections.tds_interest_201.json` | 128,795 | `f74557300e08a183d86fb8f5260c83858c036a820e5ccd40f09cb5318d801a88` | `goldens/edge.ti_sections.tds_interest_201.json` |
| `edge.ti_sections.tds_payees.json` | 45,329 | `474f93e55618496279867586190457e97dedea73de5a7c8e9ac7baab230df5e9` | `goldens/edge.ti_sections.tds_payees.json` |
| `edge.ti_shared_guid.partners_40b_194t.json` | 13,165 | `7f25df4e10b3c7b4e3faeaa22a711a46aaadf1e6ce2993bed5ddda95855f1a95` | `goldens/edge.ti_shared_guid.partners_40b_194t.json` |
| `edge.ti_shared_guid.tds_interest_201.json` | 55,880 | `7708b2e0ef5976dd6932da236d411664f9def854cb65cc2c1729211a99ae3d18` | `goldens/edge.ti_shared_guid.tds_interest_201.json` |
| `edge.ti_shared_guid.tds_payees.json` | 25,151 | `fbc26dc48d5bb4e2e3a7efdae737a8bbf6dc4b8afb2336a25c40fd08d2c846ec` | `goldens/edge.ti_shared_guid.tds_payees.json` |
| `edge.ti_status_unknown.partners_40b_194t.json` | 1,049 | `d03a931902fd87eb43fcf832cc2bb5297e03da9e34d7dfb0b3a4c51c57677e73` | `goldens/edge.ti_status_unknown.partners_40b_194t.json` |
| `edge.ti_status_unknown.tds_interest_201.json` | 19,264 | `71eed038dbee27f57befb23ca608983cf01a874137cdaf11a56b0705cea1cc8d` | `goldens/edge.ti_status_unknown.tds_interest_201.json` |
| `edge.ti_status_unknown.tds_payees.json` | 22,082 | `9929d7841620a470a28c84875d0c697197e9b9de644c8c11028929e707e7008b` | `goldens/edge.ti_status_unknown.tds_payees.json` |
| `synthetic.tds_interest_201.json` | 8,171 | `ca3a7cc458dd4ed2596fbb0f80d2af516e8b09a5d3848dafa5161426a9f09440` | `goldens/synthetic.tds_interest_201.json` |
