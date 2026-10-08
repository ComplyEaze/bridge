# Spec pack: `tds_interest_201` (the s.201(1A) interest range on TDS that two other tests find undeducted)

The goldens in this pack were produced by the reference engine at commit `10717095` and are the
contract; this note explains them and cites [`docs/tax-audit/parity-spec-v1.md`](../../parity-spec-v1.md)
(parity spec sections 1, 2.1, 2.2, 3, 3.1, 4, 5, 6, 7, 10 and 11, as relevant); where the note and a
golden differ, the golden wins and the reference's maintainers should be told on the pull request or
issue.

In this note "README section N" is a section of this note and "parity spec section N" a section of
that document.

This pack is for porting the test into `src-tauri/crates/bridge-tax-audit`. It holds:

- `books/ti_*.json`: fourteen invented edge books, in the shape of the crate's
  `tests/fixtures/edge-books/*.json`. They use only keys the crate's edge-book reader already has
  for `tds_payees` and `partners_40b_194t`; one book's `rules_without` names two tables that reader
  does not map yet (README section 14).
- `goldens/edge.<book>.tds_interest_201.json`: the reference's canonical dump of this test on each
  book (parity spec section 1), module check included.
- `goldens/edge.<book>.tds_payees.json` and `goldens/edge.<book>.partners_40b_194t.json`: the
  canonical dumps of the two tests this test's rows are assembled from, on the same book, from the
  same run. This test's rows are assembled from those two tests (README section 2.2). Both tests are already in the crate,
  ported from an earlier commit of the reference; README section 2.6 says where these input goldens
  are ahead of the crate.
- `goldens/synthetic.tds_interest_201.json`: the test on the crate's synthetic read (README
  section 15).
- `HASHES.md`: bytes and SHA-256 of every book and golden, what the books establish, which rule
  changes each golden catches, and how the goldens were made.

## 1. What the test does

`tds_payees` lists the payees a book credits over a TDS section's limit, and `partners_40b_194t`
lists the partners credited over the s.194T limit. For each of them this test takes the tax on the
credits that attract TDS, split into tranches by the day each became deductible, and publishes for
every tranche a range of interest under s.201(1A):

- a **minimum** of 0, because no date of deduction or of deposit is known;
- a **still-not-deducted amount**, at the pre-deduction rate (1% in the rules) for every month or
  part of a month from the day the tax became deductible to a fixed as-of date, as if the tax were
  still not deducted;
- a **maximum**, the largest amount over every possible date of deduction from that day to the as-of
  date: the pre-deduction rate up to the date of deduction and the post-deduction rate (1.5% in the
  rules) from it, as if the tax had been deducted and not deposited;
- both of these again with each credit of the tranche run from that credit's own date instead of
  from the tranche's date: the **own-date** still-not-deducted amount and the **own-date maximum**.

It then raises one finding per payee and rate, and totals the rows in two rate scenarios that are
never added together. It concludes nothing: every finding has confidence `needs_document`, and no
figure is called interest payable.

The test itself reads no voucher and no ledger. Everything it knows arrives as a list of rows and a
date, and those are assembled by the test's caller (in the reference, the code that runs every test
of an engagement). No row reaches the test any other way, so the assembly is part of this pack's
contract: README section 2 specifies it, and every book drives it through a real invented book.

## 2. Inputs

### 2.1 What the test takes

- **The rows** (README section 2.2), in order. The order matters: a row's position is hashed into
  its tag (README section 3.1).
- **The as-of date**: the rules' `[due_dates].audit_report`, `2026-09-30` in the vendored rules
  (the crate's `Rules.due_date_audit_report`, a field). The test takes no other date and reads no clock.
- **The rules**: the version, copied into `rules_version` (`2026-09-17.1` in every golden); the
  `[s201_1a]` table (`rate_before_deduction_bp`, `rate_after_deduction_bp`, `authority`, `status`)
  and the `[s206c_7]` table (`rate_bp`, `authority`, `status`), each replaced by the test's own
  values when the rules have no such table (README section 2.7). The crate's `Rules` already holds
  both as `Option`s (`s201_1a`, `s206c_7`).
- **The book**: passed in, never read, by the test or by its module check. The entity type is not
  read either. `test_version` is `"1"`.

### 2.2 The rows, as the caller assembles them

The caller runs after `tds_payees` and `partners_40b_194t` have run on the same book with the same
configuration, and uses their results as they stand.

1. **The flag.** "Deductor status unresolved" is true when the figure `tds_payees.deductor_status`
   has the value `unknown`, or when `[tds].previous_year_turnover_status` is `placeholder`
   (whatever the status then is). `ti_status_unknown` reaches the first, `ti_placeholder` the second.
2. **Payee rows.** When `tds_payees.deductor_status` is `not_deductor`, there are none
   (`ti_not_deductor`). Otherwise the caller walks the findings of `tds_payees` in the order that
   test raised them (README section 2.4). For each one:
   - the row id is the finding's id after `tds_payees/`, and the section is the row id's text before
     its first underscore;
   - the finding is skipped unless the section is one of `194C`, `194I`, `194J`, `194H` and the
     finding has a fact named `credited`. That skips every finding that is not a payee row (their
     ids begin with another word), a payee under a s.194J ledger with no category (its fact is named
     `unmapped_category_credited`) and a payee that is only possibly over (`possibly_over_credited`);
     `ti_quiet` has one of each;
   - the rates are those of the table below;
   - the tranches are the row id's entry in the payee tranche table (README section 2.3);
   - for each tranche in date order, and for each rate in the order of the table, there is one row.
3. **Partner rows**, after all payee rows. The caller walks the findings of `partners_40b_194t` in
   the order raised (README section 2.4) and takes those whose id begins
   `partners_40b_194t/s194t/` and that have a fact named `tds_expected`. The rest of the id is the
   partner's tag, used as the row id. The section is `194T`; there is one row per tranche, with the
   rate tag `single_rate` and one rate, the rules' `[s194t].rate_bp`, or 1000 when the rules have no
   `[s194t]` table (`ti_rules_default`); the
   tranches are the tag's entry in the partner tranche table; the flag of step 1 is never set on a
   partner's row (`ti_placeholder`).

| Section | Rows per tranche, in this order | Rate, from `[tds_rates]` unless the line says otherwise |
| --- | --- | --- |
| `194C` | `lower_rate`, then `higher_rate` | `s194c_individual_huf_bp`, then `s194c_other_bp` |
| `194I` | `lower_rate`, then `higher_rate` | `s194i_plant_machinery_bp`, then `s194i_land_building_bp` |
| `194J`, category `professional` | `single_rate` | `s194j_professional_bp` |
| `194J`, category `technical` | `single_rate` | `s194j_technical_bp` |
| `194J`, category `royalty` or `28va` | `lower_rate`, then `higher_rate` | `s194j_technical_bp`, then `s194j_professional_bp` |
| `194H` | `single_rate` | `s194h_bp` |
| `194T` (a partner's row) | `single_rate` | not from `[tds_rates]`: `[s194t].rate_bp`, or 1000 (step 3) |

The s.194J category is the second underscore-separated part of the row id
(`194J_professional_<tag>`). `ti_sections` has every payee line of the table but the first, which
`ti_194c` and `ti_rounding` have; `ti_partners` has the `194T` line.

Each row carries:

| Field | Value |
| --- | --- |
| section | `194C`, `194I`, `194J`, `194H` or `194T`, as text |
| payee key | `<row id>_<rate tag>_<tranche date, ISO>` |
| group | `<row id>_<rate tag>` |
| tax, in paise | the tranche's paise times the rate in basis points, divided by 10000 and rounded half up: `(paise * rate + 5000) div 10000` |
| deductible date | the tranche's date |
| deducted date, paid date | none, on every row |
| flag | step 1 for a payee's row; never set for a partner's |
| rate tag, rate | as the table gives them |
| basis | the section's fixed text: five texts, one per section, each in the goldens (README section 3.2) |
| credits | each credit of the tranche as its own date and its own tax, `(credit paise * rate + 5000) div 10000`, each rounded on its own |
| foreseeability | the value `[tds_payees.foreseeability]` gives the payee, or `unclassified` |
| evidence | those of the finding's voucher refs that are among the tranche's refs: the tranche's own vouchers |

**Foreseeability** is looked up by exact text. For a payee's row the name is the payee entity: the
ledger's name, or the label `[tds].payee_aliases` gives the ledger, or the fixed text
`(payee not named)` for the row `tds_payees` makes of cash and bank credits. For a partner's row it
is the partner's key in `[partners]`. The values are `foreseeable` and `one_off`; a name the list
does not hold is `unclassified`. `ti_names` shows what "exact" means: a name whose letters are
composed is not the same name decomposed, case and spaces count, and nothing is trimmed.

### 2.3 The tranches

A tranche is the credits of one payee, or of one partner, that became deductible on one day: its
date, its total in paise, the refs of its vouchers, and its credits each with its own date and
amount.

**The credits.** For a payee row they are the row's credits exactly as `tds_payees` tests the row
against its limit: one credit per voucher, the voucher's net credit to the payee after that test's
own adjustments, and only credits above 0. The adjustments: TDS on the voucher, on a ledger classified as
TDS payable, is added back; and GST is left out where the configuration records that the payee's
agreement states it separately. Each is made only when the payee is the only party the voucher
credits (`ti_quiet`'s q17: a voucher crediting two payees leaves its TDS out of both, so that payee
has no tranche). `ti_base` has one payee for each adjustment. For a partner they are the credits
`partners_40b_194t` builds its s.194T base from: each voucher's credit to the partner's capital,
taken as interest or as remuneration according to which of the partner's two ledgers the voucher
has a non-zero line on, plus the TDS on the same voucher (its lines on ledgers classified as TDS payable) when
this holds: take the side of the voucher the TDS is on (credited with a deduction, debited with its
reversal) and leave the TDS-payable ledgers out; every ledger left on that side is one of that
partner's capital ledgers (also when none is left). `ti_partners`' p04 credits the capital and the TDS ledger, so its TDS is
added back. Otherwise the TDS is not added. A voucher's credit can be negative. A voucher is one credit even when it shares its GUID with another
voucher (`ti_shared_guid`).

**The rule**, the same for every section (the crate's `tds_tranches::crossing_tranches` already
implements it):

1. Take the credits in order of date, and within a date in order of voucher key (the GUID; for
   vouchers sharing a GUID, their order in the book). Book order plays no other part
   (`ti_194c`'s c08).
2. A credit of 0 or less is skipped: it lowers nothing (`ti_partners`' p05).
3. Keep a running total. A credit **over** the single-sum limit, where the section has one, is
   deductible on its own date.
4. The first credit that takes the running total **over** the aggregate limit marks the crossing.
   Every earlier credit not yet deductible becomes deductible on the crossing day, with the crossing
   credit; every later credit is deductible on its own date. A crossing is never undone.
5. The credits deductible on one day are one tranche. Its refs are the distinct pairs of GUID and
   label of its vouchers; its credits are sorted by own date, then amount, and two credits of one
   date stay two.

"Over" is strict in both tests: a credit of exactly the single-sum limit waits, and a total of
exactly the aggregate limit has not crossed (`ti_194c`'s c02 and c04).

| Section | Single-sum limit | Aggregate limit |
| --- | --- | --- |
| `194C` | `[s194c].single_sum_paise` | `[s194c].aggregate_paise` |
| `194I` | none | `[s194i].per_month_per_payee_paise`, applied to each calendar month's credits on their own; the months' tranches are then put in date order |
| `194J` | none | `[s194j].aggregate_paise`, or 5000000 paise when the rules have no `[s194j]` table; per payee and category, as `tds_payees` forms its rows |
| `194H` | none | `[s194h].aggregate_paise` |
| `194T` | none | `[s194t].limit_paise`, or 2000000 paise when the rules have no `[s194t]` table |

**The two tables.** The payee tranche table has an entry for every payee row `tds_payees` tests
against a limit (so not for its goods-invoice bucket and not for a s.194J ledger with no category),
keyed by the same row id that test puts in its figure and finding ids. The partner tranche table has
an entry for every partner in `[partners]`, keyed by the partner's tag. An entry can be empty: a
payee `tds_payees` lists as over the limit only when TDS it cannot divide is counted (`ti_quiet`'s
q17), and a partner whose finding is raised for another reason than its total (`ti_quiet`'s q19),
have no tranche and so no row.

A finding that qualifies under README section 2.2 and whose row id is in neither table is refused by
the reference, not skipped. No book reaches that: both tables are built from the rows the two
tests themselves build, and it did not happen on any of the books of README section 11.

### 2.4 The order of the rows

The dumps of `tds_payees` and `partners_40b_194t` are sorted, so their goldens do not show the order
in which those tests raise findings. This test depends on it, because a row's position is part of
its tag. The order is:

1. payee rows before partner rows;
2. payee rows by section, `194C`, `194I`, `194J`, `194H`, and within `194J` by category,
   `professional`, `technical`, `royalty`, `28va`;
3. within a section (and category), payees in ascending order of the first 8 hexadecimal characters
   of the SHA-1 of the payee entity's name (the name as README section 2.2 gives it, in UTF-8);
4. partners in ascending order of their key in `[partners]`, by code point, whatever order the table
   lists them in (`ti_partners`);
5. for one payee or partner, tranches by date, and for one tranche the lower rate before the higher.

On origin/master the crate's `tds_payees` walks sections and payees in this order
(`NATURES`, and a sort by the same hash); a port must keep the order through to the rows.

### 2.5 Binding (before the test runs)

This test adds no location to the binding step. Every ledger name it depends on reaches it through
the two input tests, whose locations the reference's binding and the crate's `src/binding.rs`
already bind before any test runs: the keys of `[tds].nature_by_ledger`, of `[tds].payee_aliases`
and of `[tds_payees].s194j_category_by_ledger`, each partner's ledgers in `[partners]`, the keys of
`[statutory_dues].nature_by_ledger` and the ledgers of `[roles].tax_ledgers`.

Three things this test's rows depend on are **not** bound, in the reference or in the crate: the
names in `[tds_payees.foreseeability]`, the names in `[tds_payees].gst_separate_by_agreement`, and
the partners' keys. They are payee labels and keys, matched as written. So a ledger the binding step
rewrites to its name in the book is classified only if the foreseeability list uses that same name.
A binding refusal anywhere in the configuration stops this test too, as it stops every test.

### 2.6 What the crate has, and what a port adds

On origin/master:

- **The rule of README section 2.3** is in `src/tds_tranches.rs`. Its `Tranche` holds the date, the
  total, the vouchers' GUIDs and the credits; it does not hold the vouchers' refs (GUID and label),
  which the caller needs for a row's evidence. The five basis texts are not ported; that file says
  so.
- **`src/tds_payees.rs`** uses the rule privately, for clause 21(b), and keeps only the GUIDs. It
  has no payee tranche table and no map from a row id to its payee entity; a port adds both, built
  from the same rows and limits the test itself uses. (A row id's hash is made from the ledger tag of
  parity spec section 11, not from the name, so neither the entity nor the order of the rows can be
  read off the ids.)
- **`src/partners_40b_194t.rs`** builds the s.194T credits privately and has no partner tranche
  table; a port adds it.
- **The foreseeability list**: the crate's `ConfigLists` and `TdsConfig` keep only the names
  (`foreseeability_names`), which is all `tds_payees` needs. A port keeps each name's value too.
- **The rules**: `Rules` already has `s201_1a`, `s206c_7`, `tds_rates`, `s194t` and
  `due_date_audit_report`.
- **Vouchers that share a GUID.** The reference at `ee17d80f` keeps one credit per voucher in both
  input tests and in the tranches. The crate's two ports key a row's credits by voucher GUID. The
  three goldens of `ti_shared_guid` are the reference's; a port of this test needs the two input
  ports brought to that reading first (issue #1243).
- **One limit text of `partners_40b_194t`** was reworded in the reference after the crate's port:
  regenerating the crate's committed edge goldens of the two input tests at `ee17d80f` reproduced
  24 of 25 byte for byte, and `edge.partners_tds_mixed.partners_40b_194t.json` differs in that one
  text. A partner with a voucher of negative net credit gets that limit, so
  `edge.ti_partners.partners_40b_194t.json` carries the newer wording.

The input goldens in this pack add nothing to the contract of the two input tests beyond those two
points. They give the input tests' findings, facts and evidence. They do not give everything the rows
need: the two tranche tables (built from the book, the configuration and the rules), the payee names
and partner keys behind the hashed ids, the foreseeability values, and the order in which the
findings were raised (a dump is sorted) are not in them. A port builds those from the book (README
sections 2.3, 2.4 and 14).

### 2.7 Rules keys and fixed values

Read by the test: `[s201_1a]` and `[s206c_7]` (README section 2.1). Read by the caller:
`[due_dates].audit_report`, `[tds_rates]` (the seven keys of it that README section 2.2's table names; the
vendored table holds two more, `s194t_bp` and `s194a_bp`, and the caller reads neither),
`[s194t].rate_bp`, and through the two tranche tables the limits of README section 2.3.

Without `[tds_rates]` the caller refuses, on every book, also one with no row. Without `[s194t]`
or `[s194j]` it falls back as README sections 2.2 and 2.3 say. Without `[s201_1a]` or `[s206c_7]`
the test uses its own values:

| Table absent | Values used |
| --- | --- |
| `[s201_1a]` | `rate_before_deduction_bp` 100, `rate_after_deduction_bp` 150, authority `s.201(1A)`, status `confirm` |
| `[s206c_7]` | `rate_bp` 100, authority `s.206C(7)`, status `confirm` |

`ti_rules_default` runs without all four optional tables.

Fixed in the test, not client data: the three clause tags, the confidence, the population note, the
two items to ask, the texts of README section 4.3, the two values of the deducted and paid date
figures (`not supplied`), and the length of a tag (8 hexadecimal characters).

## 3. From a row to its figures

Every figure and finding id below is exactly as the goldens carry it, after the prefix
`tds_interest_201.`.

### 3.1 The tag

A row's tag is the first 8 hexadecimal characters of the SHA-1 of the UTF-8 text
`<section>:<payee key>:<position>`, the position counted from 0 over the whole list of rows. For the
first row of `ti_placeholder` that text is `194H:<row id>_single_rate_2025-07-14:0`, with the row id
its `tds_payees` golden shows.

### 3.2 The fifteen figures of a row

| Figure id | Unit | Value | Evidence |
| --- | --- | --- | --- |
| `section_<tag>` | text | the section | none |
| `tax_<tag>` | paise | the row's tax | the row's |
| `rate_bp_<tag>` | bp | the row's rate | none |
| `deductible_date_<tag>` | text | the deductible date, ISO | none |
| `deducted_date_<tag>` | text | `not supplied` | none |
| `paid_date_<tag>` | text | `not supplied` | none |
| `months_stage1_min_<tag>` | count | 0 | none |
| `months_stage1_max_<tag>` | count | months from the deductible date to the as-of date | none |
| `months_stage2_min_<tag>` | count | 0 | none |
| `months_stage2_max_<tag>` | count | the same count as `months_stage1_max_<tag>` | none |
| `interest_min_<tag>` | paise | 0 | the row's |
| `interest_if_not_deducted_<tag>` | paise | interest on the row's tax at the pre-deduction rate for `months_stage1_max` months | the row's |
| `interest_max_<tag>` | paise | the maximum (below) on the row's tax from the deductible date | the row's |
| `interest_if_not_deducted_own_date_<tag>` | paise | the sum, over the row's credits, of interest on the credit's tax at the pre-deduction rate for the months from the credit's own date to the as-of date | the row's |
| `interest_max_own_date_<tag>` | paise | the sum, over the row's credits, of the maximum (below) on the credit's tax from the credit's own date | the row's |

**Months** ("month or part of a month") from a start date to an end date: 0 when the end is on or
before the start; otherwise the number of calendar months the span touches,
`(end year - start year) * 12 + (end month - start month) + 1`. Days are not counted: 29 September
to 30 September is 1 month, 31 August to 30 September is 2, and a span that starts on the as-of date
itself or on any later date is 0 (`ti_month_edges`).

**Interest** on a tax at a rate for a number of months is
`(tax * rate * months + 5000) div 10000` paise, rounded half up, and 0 when the tax, the rate or the
months is 0 or less (`ti_rounding`). The pre-deduction rate is the rules' `rate_before_deduction_bp`
and the post-deduction rate their `rate_after_deduction_bp`.

**The maximum** on a tax from a start date is found by trying every day D from the start date to the
as-of date, both included: the interest on the tax at the pre-deduction rate for the months from the
start date to D, plus the interest on it at the post-deduction rate for the months from D to the
as-of date, each of the two rounded on its own. The maximum is the largest of those sums, and 0 when
the start date is after the as-of date. A month that both periods touch is counted in both, so the
largest is not always at the start date:

- `ti_month_edges`' m04, tax Rs 20.00 from 1 September 2026: deducted that day, 1 month at 1.5%,
  30 paise; deducted on the 2nd, September at 1% and again at 1.5%, 50 paise, the maximum;
- m03, from 31 August 2026, a month end: deducted that day, 2 months at 1.5%, 60 paise; deducted on
  1 September, 2 months at 1% and 1 at 1.5%, 70 paise, the maximum;
- m05, from 29 September 2026: deducted that day 30 paise, the maximum; the only later day is the
  as-of date, 1 month at 1% and nothing after, 20 paise, which is also its still-not-deducted
  amount;
- `ti_rounding`'s x11, tax 9 paise from 22 January 2026: 1 paisa if deducted that day or the day
  after, and 2 paise if deducted on 1 June, where 6 months at 1% and 4 months at 1.5% are 0.54 paise
  each and each rounds up. A port that tries only the start date and the day after it fails that
  golden.

The maximum is never below the still-not-deducted amount: deducted on the as-of date itself gives
that amount.

Each credit's part of an own-date figure is rounded on its own, so where every credit of a tranche
has the tranche's date an own-date figure can still differ from the plain one by a paisa, either way
(`ti_rounding`'s Oak Transit and Palm Freight).

**Definitions** are compared by hash (parity spec section 4); take the fixed words from the
goldens. They carry these variable parts:

- the row's position and tag, in every row figure; `section_<tag>` and `tax_<tag>` also carry the
  payee key, written inside single quotes as Python's `repr()` writes it (the key is always plain
  ASCII letters, digits, underscores and hyphens);
- `tax_<tag>`: after its fixed sentence, a space and the row's basis, then for a `lower_rate` or
  `higher_rate` row one more fixed sentence saying which of the two rates it is. The five basis
  texts are in `ti_194c` (194C), `ti_sections` (194I, 194J, 194H) and `ti_partners` (194T);
- `interest_if_not_deducted_<tag>`, `interest_max_<tag>` and the two own-date figures: the as-of
  date, ISO;
- the three rate figures of README section 5: the table's `authority` and `status`.

### 3.3 Evidence labels

Every ref has kind `voucher` and the voucher's GUID as id; refs are sorted in the dump (parity spec
section 2.2). The label is `<voucher type> <number> on <ISO date>`, the crate's
`support::voucher_label`: the voucher's type name, not its base type, and for an empty number the
last 12 characters of the GUID, or the whole GUID when it is shorter (`ti_names`' n04 and n05).
Labels are NFC-normalised in the dump (`ti_names`' n02). Two vouchers that share a GUID give two
refs with one id unless their labels are equal too (`ti_shared_guid`). In an edge book a voucher with no `vtype`
takes its base type as its type name and one with no `number` takes its GUID, as the crate's
`tests/edge_books.rs` already reads them (`ti_quiet`'s tds_payees golden: `Journal ti-q17 on 2025-10-20`).

## 4. Groups and findings

### 4.1 Groups

Rows are gathered by their group, in the order groups first appear. A group's id is the first 8
hexadecimal characters of the SHA-1 of `<section>:<group>`. So one payee at one rate is one group,
whatever the number of its tranches, and a payee under two rates is two groups.

A group collects, in row order: its rows; their limits, each text kept once; their evidence, each
ref kept once.

### 4.2 When a group gets a finding

A group gets a finding only when the plain maximum, `interest_max_<tag>`, of at least one of its rows is
above 0; the own-date maximum and the still-not-deducted amounts play no part. A group whose
tranches all fall on or after the as-of date has its row figures and counts in the totals, and no
finding and no group figure (`ti_month_edges`' m08 and m09).

The finding lists the group's rows whose tax is above 0, in row order, numbered from 1. A row with
a tax of 0 is left out (`ti_rounding`'s x04 at both rates and x03 at the lower rate), while a row
with tax and a maximum of 0 is listed (`ti_rounding`'s x03 at the higher rate; `ti_month_edges`'
m06 and m07).

With the finding comes one more figure, `group_tax_<group id>` (paise, no evidence): the sum of the
tax of every row of the group. Its definition carries the group id and, for a two-rate group, which
rate.

### 4.3 The finding

- **id**: `tds_interest_201/<group id>`.
- **clauses**, in this order: `s.201(1A)`, `3CD-34(a)`, `3CD-34(c)`.
- **confidence**: `needs_document`.
- **facts**: `tax` names `group_tax_<group id>`; then for the n-th listed row
  `tranche_<n>_deductible_date`, `tranche_<n>_tax`, `tranche_<n>_interest_min`,
  `tranche_<n>_interest_max`, `tranche_<n>_interest_if_not_deducted`,
  `tranche_<n>_interest_max_own_date` and `tranche_<n>_interest_if_not_deducted_own_date`, each
  naming that row's figure.
- **evidence**: the group's.
- **title**:
  `Possible s.201(1A) interest on a <section> shortfall for one payee<rate phrase> -- <n> tranche<s> -- range pending deposit evidence`,
  where `<n>` is the number of listed rows, `<s>` is `s` when that is more than 1, and the rate
  phrase is empty for a single-rate group and a fixed parenthesis for each of the two rates (the
  goldens of `ti_194c` have both).
- **limits**, in this order, each a fixed text taken from the goldens:
  1. dates need documents (every finding);
  2. tie the range to the quarterly statement (every finding);
  3. the rate is the test's own default: only when the rules have no `[s201_1a]` table
     (`ti_rules_default`);
  4. the date of deduction is not known, so the interest is computed two ways (every finding); this
     text carries the as-of date, ISO, in four places;
  5. no payee filing date, so no Form 26A figure (every finding);
  6. the row's basis;
  7. for an `unclassified` payee one text, for a `one_off` payee another, for a `foreseeable` payee
     none (`ti_194c` and `ti_partners` have all three);
  8. the deductor status is unresolved: only when the row's flag is set.
- **ask the client**: two fixed items.

### 4.4 Population note

One fixed sentence, the same in every golden.

## 5. Totals and counts

Seventeen figures are on every dump, whatever the rows:

| Figure id | Unit | Value |
| --- | --- | --- |
| `as_of` | text | the as-of date, ISO |
| `rate_before_deduction_bp`, `rate_after_deduction_bp` | bp | the `[s201_1a]` rates |
| `rate_206c_bp` | bp | the `[s206c_7]` rate |
| `total_tax_<sc>_rates_paise` | paise | the sum of the tax of the scenario's rows |
| `total_interest_min_<sc>_rates_paise` | paise | the sum of their minima: 0 |
| `total_interest_max_<sc>_rates_paise` | paise | the sum of their **own-date** maxima |
| `total_interest_26a_relief_<sc>_rates_paise` | paise | 0 |
| `total_interest_min_paise` | paise | the lower scenario's minimum |
| `total_interest_max_paise` | paise | the higher scenario's maximum |
| `foreseeability_unclassified_row_count` | count | rows whose foreseeability is `unclassified` |
| `s201_1a_defaults_with_26a_relief_date_count` | count | 0 |
| `s201_1a_defaults_without_26a_relief_date_count` | count | the number of rows |

`<sc>` is `lower` and `higher`. The **lower scenario** holds every `lower_rate` row and every
`single_rate` row; the **higher scenario** every `higher_rate` row and every `single_rate` row. A
single-rate row is in both, and the two scenarios are never added.

Three more figures appear only when at least one row is `one_off` (`ti_194c`, `ti_partners`,
`ti_names`): `total_interest_max_one_off_at_crossing_<sc>_rates_paise` for each scenario, the same
sum as `total_interest_max_<sc>_rates_paise` but taking the plain maximum, `interest_max_<tag>`, for
the `one_off` rows; and `total_interest_max_one_off_at_crossing_paise`, the higher scenario's.
`ti_sections` has a `foreseeable` payee and no `one_off` one, and does not have them.

Every total of interest other than the minimum is built from maxima. No total is published for the
still-not-deducted amounts; the rows carry them.

None of these figures has evidence.

## 6. When there is no row

There is no gate. With no row the dump is the seventeen figures of README section 5, every total
and count 0, and no finding, and it does not depend on the book: `ti_empty` (no voucher),
`ti_quiet` (nineteen vouchers of near misses) and `ti_not_deductor` (a payee over the limit of an assessee that
is not a deductor) give byte-identical goldens. The test never forms the books population; the two
input tests and the two tranche tables do, so a book they refuse stops the run before this test.

## 7. The module's own check (TDSI-1)

`module_invariants_evaluated` is `["tds_interest_201.check_invariants"]` in every golden, and
`module_invariant_violations` is empty in every golden. Each violation would be
`{"invariant": "tds_interest_201.check_invariants", "subject": "tds_interest_201", "detail": <text>}`.

The check reads only the published figures. For every tag that has an `interest_min_<tag>` figure,
in ascending order of tag, it reads that row's `tax`, `section`, `deductible_date`, `deducted_date`,
`paid_date`, `interest_min`, `interest_max` and `interest_if_not_deducted` figures, and the `as_of`
and three rate figures, and recomputes them with its own copies of the months and interest rules of
README section 3.2. A `deducted_date_<tag>` or `paid_date_<tag>` figure that is absent, or has no
value, is read as `not supplied`. For a row whose deducted date is `not supplied` the recomputed minimum is 0, the
recomputed still-not-deducted amount is the interest on the tax at `rate_before_deduction_bp` for
the months from the deductible date to the as-of date, and the recomputed maximum is the largest of
the sums of README section 3.2. The reference's check does not try every day for that: it tries the
deductible date, the day after it when the as-of date is later, and the first day of every later
calendar month up to the as-of date's. A day's two month counts depend only on its calendar month
unless it is the deductible date or the as-of date, and the as-of date is one of the days tried or
has an earlier day of its own month among them, which counts the same first period and a second. So
the two methods give one maximum, and a port may use either in its check. The check does not look at
the own-date figures, the months figures, the group figures or the totals.

Its six messages, exactly, each beginning `TDSI-1: <tag>: `:

- `missing a figure needed for independent recomputation`, when the row's tax, section, deductible
  date, minimum or maximum figure is absent or has no value (the check takes its tags from the
  minimum figures, so the minimum can have no value but cannot be absent); the row is then not
  checked further;
- `negative bound (min=<minimum>, max=<maximum>)`;
- `min <minimum> > max <maximum>`;
- `published interest_min <minimum> != independently recomputed <value>`;
- `published interest_max <maximum> != independently recomputed <value>`;
- `published interest_if_not_deducted <amount> != independently recomputed <value>`, where the
  amount is `None` when a row with no deducted date lacks the figure, and the value is `None` when
  a row that should have no such figure carries one.

**None of them can fire on a row the caller builds**, and no golden shows one. The test always
publishes the figures the first message looks for; no bound is below 0; the minimum of a caller's
row is 0; and the recomputation repeats the rule that produced the figures. That was also measured.
At the commit this pack was first made from (`ee17d80f`), on the rows of README section 11's books,
16,699 rows, the first five messages never fired. At this commit the check was run on every row of
this pack's books and on 126,672 invented rows with no deducted date (deductible dates up to four
years before the as-of date, leap days and month ends, taxes from 1 paisa up, thirteen pairs of
rates), each compared with a search of every day: no violation and no difference. The goldens still pin the
check's own arithmetic: a port whose check counted months or rounded differently from its test
would fire on these books and fail them (HASHES.md).

## 8. Book and result checks in the dumps

Every golden carries the book-level and result-level reports of parity spec section 5.

- `ti_shared_guid`, in all three of its goldens: POP-5 at book level on each of the three GUIDs that
  two regular vouchers share (`2 in-books vouchers share this GUID`). In its `tds_interest_201`
  golden POP-4 fires seven times at result level, subject `tds_interest_201`, detail
  `voucher:ti-g-void`: once for each of the six figures of the CJ/05 row that carry evidence and
  once for the finding, because that voucher's GUID is also a cancelled voucher's. Its `tds_payees`
  golden has the same POP-4 three times.
- Every other report of every edge golden is empty: each book's vouchers tie to its Trial Balance
  and every line's ledger has a master.
- The synthetic dump of README section 15 carries the synthetic read's own four book-level
  violations, as every synthetic golden of the crate does.

## 9. Ordering

Figures, findings, facts, evidence and violations are all sorted in a dump (parity spec section 6),
and clause lists keep their authored order (parity spec section 3.1). The order of the rows still
shows, in three places: a row's tag holds its position (README sections 2.4 and 3.1); a finding's
tranches are numbered in row order; and a group's limits are hashed in the order its rows gave them
(parity spec section 4).

## 10. What the test never does

- Who the payees are, which section each falls under, what each was credited and whether a limit was
  passed are all settled by the two input tests; this test takes them as given.
- It never reads a voucher, a ledger, the Trial Balance, a narration or the period, and never forms
  the books population.
- It never uses a date of deduction or of deposit, and never reads the TDS the books show as
  deducted: the caller supplies neither date for any row.
- It never states an amount of interest payable; it states a range, and its confidence is always
  `needs_document`.
- It never adds the two rate scenarios together.
- It never reads a clock: the only date besides the rows' is the rules' audit-report date.

## 11. Not covered by any golden

### 11.1 Inputs the caller never builds

The test accepts rows that no run of the reference can give it. They were looked for by running the
caller on the fourteen books of this pack, on 24 of the crate's 25 committed edge books for the two
input tests (the 25th, `tds_payees_gross_gst`, names a partner with no ledgers, which
`partners_40b_194t` refuses) and on 2,000 randomly built invented books: 2,038 books, 16,699 rows.
That run was made at `ee17d80f`, on the books as they then were; the caller, the tranche rule and
the two input tests are byte for byte the same at `10717095`, and it was not repeated.
On every one of those rows the section was one of the five of README section 2.2, the deducted date
and the paid date were absent, there was no payee filing date, and the group, the rate, the basis
and at least one credit were present. So no golden shows, and this pack does not specify:

- **a s.206C (TCS) row**, which the test would price at the `[s206c_7]` rate with two other months
  figures in place of the four of README section 3.2. The rate figure `rate_206c_bp` is published
  all the same;
- **a row with a deducted date**, with or without a paid date, which the test would price in two
  legs, the second at `rate_after_deduction_bp`, and would give no still-not-deducted amount and no
  own-date figure;
- **a row with a payee filing date** (the Form 26A route), which would add three figures to the row
  and a fact to its finding, and move the two counts and the two relief totals of README section 5
  off the values given there;
- **a row with no group**, whose finding would take the row's own tag as id and other fact names;
  a row with no rate, no credits or no basis; a rate tag or a foreseeability value outside the
  three each has, which the test refuses.

A port whose row type cannot hold these inputs agrees with every golden. Nothing in this pack is
evidence for how they behave.

### 11.2 Reachable, but in no book

- **A refusal.** A dump is not produced, so there is no golden: rules without `[tds_rates]` (the
  caller refuses on every book); a `[tds_payees.foreseeability]` value other than `foreseeable` or
  `one_off`, or a `[tds].previous_year_turnover_status` other than `placeholder` or `confirmed`
  (the configuration readers refuse); a ledger mapped to `194H` on rules without `[s194h]`, a
  `[partners]` entry the partners test refuses, or a voucher of unknown status (the input tests
  refuse first).
- **Two payees whose names hash to the same 8 characters**, or two rows or two groups whose tags
  collide. Rows with one tag would overwrite each other's figures.

### 11.3 Rules no golden pins

Each of these was changed alone in a copy of the reference at `10717095` and no golden changed:

- **the group tax summed over the listed rows instead of all rows**: an unlisted row has a tax
  of 0;
- **the combined floor taken from the higher scenario instead of the lower**: both are 0;
- **the test for a s.206C section removed**: no row has one;
- **the module check's two comparisons removed**: they never fire (README section 7);
- **the module check not looking at the still-not-deducted amount**: its message never fires
  either;
- **a group's finding gated on the larger of its rows' two maxima instead of the plain maximum**
  (README section 4.2): no book has a group in which the two differ;
- **a group's finding gated on its rows' still-not-deducted amounts instead of their maxima**: the
  two differ only for a group whose every row has a still-not-deducted amount of 0 and one of whose
  rows has a maximum above 0, and no book has one (no row in any book has the one at 0 and the other
  above it);
- **the search for the maximum stopping the day before the as-of date**: on no row of any book does
  trying the as-of date add to the maximum.

And these cannot be told apart by any book:

- the order of two credits of one date within a tranche's reading (README section 2.3 step 1): both
  end in the same tranche;
- the caller's filter on evidence of kind `voucher`: the two input tests cite vouchers only on
  these findings;
- passing by a payee that is only possibly over: it has no tranche either way;
- where the s.194T rate is read from: `[s194t].rate_bp` (which the caller reads), `[tds_rates].s194t_bp`
  (which it does not) and the value used without an `[s194t]` table are all 1000 in the vendored
  rules, and a book can drop a rules table but cannot change a value;
- where the s.194J technical rate is read from: `[tds_rates].s194j_technical_bp` (which the caller
  reads) and `[s194j].technical_services_rate_bp` (which it does not) are both 200.

Every rule in the first table of HASHES.md changed at least one golden when it alone was changed at
`10717095`. Its other three tables are as measured at `ee17d80f`; HASHES.md says what was not run
again.

## 12. Behaviour that may look like a defect

Reproduce these as the goldens show them, and raise them on the pull request rather than fixing
them in the port:

- **The whole tax is priced even when the books show it deducted.** `ti_base`'s b01 credits the
  payee net of TDS on a ledger classified as TDS payable; `tds_payees` reports that TDS as seen,
  and the row's tax is still the full amount, priced with no date of deduction as every row is. The
  minimum of 0 is the only sign of it.
- **A firm can be told its deductor status is unresolved.** `ti_placeholder`: the status is
  `deductor`, and the payee's finding still carries the limit about an unresolved status, because
  the turnover is marked a placeholder. That limit points to a finding of `tds_payees` which does
  not exist on that book.
- **An own-date figure can differ from the plain one by a paisa, either way,** when every credit
  has the tranche's date (`ti_rounding`): each credit is rounded on its own.
- **The maximum counts the month of deduction twice.** Deducted the day after the tax became
  deductible, that month is counted at the pre-deduction rate and again at the post-deduction rate,
  so the maximum is above the post-deduction rate for the whole span (`ti_month_edges`' m04: 50
  paise, not 30).
- **A maximum can be reached months after the deductible date** on a tax of a few paise, where each
  period's interest rounds up on its own (`ti_rounding`'s x11).
- **No total is published for the still-not-deducted amounts.** Every total of interest but the
  minimum is built from maxima.
- **A limit says a count by periods of thirty days may give a higher figure,** and the test computes
  none.
- **The post-deduction months figure is the whole span** on every row, although no date of
  deduction is known: either period can be the whole span.
- **A payee with no name can be classified.** `ti_names`' n06: the foreseeability list names the
  text `(payee not named)`, `tds_payees` reports the entry as matching nothing, and the caller
  applies it.
- **A payee `tds_payees` lists as over a limit can have no row** (`ti_quiet`'s q17), and so can a
  partner whose finding states a tax expected (`ti_quiet`'s q19): neither has a tranche.
- **Figures without a finding.** A payee whose every tranche is on or after the as-of date has its
  figures and its tax in the totals and no finding (`ti_month_edges`' m08 and m09).
- **A tranche with no tax is in the figures and not in the finding**, so the two rate groups of one
  payee can list different numbers of tranches (`ti_rounding`).
- **Tags move with position.** A payee added earlier in the order of README section 2.4 changes the
  tag of every later row, and so its figure ids, although nothing about that row changed.
- **Every finding says no payee filing date was supplied**, and a count gives the number of rows
  without one: no row can have one (README section 11.1).
- **The section is printed as `194I` in a title** while the basis text writes s.194-I.
- **Months are calendar months touched**, so 31 August to 30 September is 2 months and 1 September
  to 30 September is 1 (`ti_month_edges`' m03 and m04).

## 13. The books

All are invented: synthetic ledger names, nothing read from Tally. Each book's `comment` says what
it reaches, voucher by voucher. The period is the default, 1 April 2025 to 31 March 2026, except in
`ti_month_edges`. Every book names `tds_payees`, `partners_40b_194t` and `tds_interest_201` in
`tests`. A firm is always a deductor, so most books are a firm's.

| Book | Rows | Findings | Reaches |
| --- | ---: | ---: | --- |
| `ti_empty` | 0 | 0 | No voucher, an individual with status `unknown`: the seventeen figures alone. |
| `ti_quiet` | 0 | 0 | Near misses at exactly each limit (194C single sum and aggregate, 194-I month, 194J, 194H, 194T); optional, cancelled and post-dated vouchers; a goods invoice; a Contra; a nil line; the input findings that give no row. Golden byte-identical to `ti_empty`'s. |
| `ti_not_deductor` | 0 | 0 | A payee over the limit of an individual that is not a deductor. Golden byte-identical to `ti_empty`'s. |
| `ti_194c` | 12 | 8 | Two rates; single-sum, crossing and later tranches; exactly at each limit and a paisa over; credits read in date order; an alias joining two ledgers; a payee not named; one-off, foreseeable and unclassified payees; the one-off totals. |
| `ti_sections` | 11 | 9 | 194-I by month; 194J professional, technical, royalty and s.28(va); 194H; the order of sections and categories; a foreseeable payee without one-off totals. |
| `ti_base` | 3 | 3 | The tax follows the credit as `tds_payees` reads it: gross of TDS, GST left out by agreement, GST kept. |
| `ti_partners` | 5 | 4 | Partners after payees and in key order; a crossing and a later tranche; TDS added back; a reversal that lowers nothing; one-off and foreseeable partners. |
| `ti_placeholder` | 2 | 2 | The flag from a placeholder turnover, on a payee and not on a partner. |
| `ti_status_unknown` | 1 | 1 | The flag from an `unknown` status. |
| `ti_rules_default` | 2 | 2 | Rules without `[s201_1a]`, `[s206c_7]`, `[s194j]` and `[s194t]`. |
| `ti_month_edges` | 9 | 1 | 0, 1, 2, 12 and 13 months; the as-of date and each side of it; a deductible date on a month end, on the first of a month and on the day before the as-of date; figures with no finding. |
| `ti_rounding` | 18 | 6 | Half-up rounding of tax and of interest, at the half and just under it; a tranche with no tax; a maximum found only by trying every date of deduction; an own-date figure a paisa above and a paisa below the plain one. |
| `ti_shared_guid` | 5 | 2 | Vouchers sharing a GUID as separate credits, in two tranches and in one; identical twins cited once; a GUID shared with a cancelled voucher; POP-5 and POP-4. |
| `ti_names` | 6 | 6 | Exact matching of foreseeability names (composed and decomposed letters, Devanagari, double spaces, case); labels with Unicode, padded and empty numbers; a classified payee with no name. |

## 14. Running the books

The crate's `tests/edge_books.rs` does not run this test yet. A port adds that, for each book naming
`tds_interest_201`:

1. run the `tds_payees` port and the `partners_40b_194t` port on the book, as the two existing arms
   do from the same keys, and keep their results;
2. set the flag of README section 2.2 from the first result and from the spec's
   `previous_year_turnover_status`, build the two tranche tables from the same inputs the two runs
   took, read the spec's `foreseeability` table with its values, and assemble the rows;
3. run the test with the rules (after `rules_without`), the rows and the rules' audit-report date,
   and its module check with the result;
4. compare the whole dump with `edge.<book>.tds_interest_201.json`, as the other edge books do
   (parity spec section 7).

`ti_rules_default` drops four tables: `s201_1a`, `s206c_7`, `s194j` and `s194t`. The reader's `rules()`
already maps the last two; it maps neither `s201_1a` nor `s206c_7` yet and stops on a table it does
not map, so the port adds both there.

Every book also names the two input tests, so the two existing arms compare the input goldens too,
once the books and goldens are copied into the crate's fixtures. The crate was not built or run for
this pack. From reading origin/master (README section 2.6), the input goldens of `ti_shared_guid`
and the `partners_40b_194t` golden of `ti_partners` are the ones its two ports are not expected to
reproduce until they are brought to `ee17d80f`.

Copied files need byte rows there as well: `tests/provenance_rows.rs` requires one four-cell row per
golden and edge book in a Markdown file under `tests/fixtures` (`PROVENANCE.md` or a batch file
under `provenance/`), with the path relative to `tests/fixtures` (`edge-books/ti_base.json`,
`golden/edge.ti_base.tds_payees.json`; HASHES.md writes `books/` and `goldens/`); the byte and SHA-256
cells can be copied from HASHES.md, whose own rows, under `docs/`, do not count.

The reference's side of the edge harness, `parity/edge_golden.py`, is in this repository, but it
has no runner for this test yet; README section 15 gives the lines. A porter does not regenerate
goldens: that is done only by the reference's maintainers.

## 15. Registering the test in the crate

A new registry entry needs, besides the books, a golden for the synthetic read, a `min_figures`
value and the two harness lines. All of them are here, so that a port does not write a golden by
hand.

- **The synthetic golden.** `goldens/synthetic.tds_interest_201.json` is the test on the crate's
  synthetic read (`tests/fixtures/synthetic-engagement.toml`), with the rows assembled from the two
  input tests on the same read. It goes to `tests/fixtures/golden/synthetic.tds_interest_201.json`.
  On that read `tds_payees` lists no payee over a limit and `partners_40b_194t` raises no finding,
  so there is no row: the dump has the seventeen figures of README section 5, no finding, no TDSI-1
  violation, and the read's four book-level violations (as every synthetic golden of the crate
  does). It is not an empty dump, so the comparer's refusal of two empty results is not tripped.
  Until it is in `tests/fixtures/golden`, the crate's registry test that every registered test has
  a synthetic golden fails.
- **`min_figures`**: 17. The test publishes those seventeen figures on every book, so fewer is a
  broken dump (the minimum figure count of parity spec section 7; the steps of adding a test are
  parity spec section 10).
- **The binding position**: none (README section 2.5).
- **The registry entry** takes no caller data: like `tds_payees_on` and `partners_40b_194t_on`, a
  `tds_interest_201_on` binds the engagement, runs the two input ports on the bound configuration,
  assembles the rows and runs this test with the rules' audit-report date. It needs what those two
  runners need, and nothing else from the configuration but the values of
  `[tds_payees.foreseeability]`.
- **The harness lines** (the reference's maintainers keep and run the harness; these are the lines
  a port adds so that its harness names the test). In `parity/python_golden.py`, a runner beside
  `_tds_payees` and its `RUNNERS` entry (the list is kept sorted by test id):

      def _tds_interest_201(c):
          from tae.audit_tests import tds_interest_201, tds_payees
          from tae import config as tc
          from tae.pack import _tds_interest_defaults
          tax_ledgers = tc.tax_ledgers_by_head(c.cfg) if "tax_ledgers" in c.cfg.get("roles", {}) else {}
          nature_by_ledger, payee_aliases, _turnover, s194j_category_by_ledger = tc.tds_config(c.cfg)
          payees, partners_result = _tds_payees(c)[1], _partners_40b_194t(c)[1]
          uncertain = (payees.figures[f"{tds_payees.TEST_ID}.deductor_status"].value == "unknown"
                       or tc.turnover_is_placeholder(c.cfg))
          defaults = _tds_interest_defaults(
              c.eng, c.rules, payees, partners_result, uncertain,
              (nature_by_ledger, payee_aliases, s194j_category_by_ledger), tc.partners_config(c.cfg)[0],
              tc.tds_payees_reversals(c.cfg), tc.tds_payees_foreseeability(c.cfg), tc.tds_payees_gst_separate(c.cfg),
              frozenset(tax_ledgers), tc.tds_payable_ledgers(c.cfg))
          return tds_interest_201, tds_interest_201.run(c.eng, c.rules, defaults, c.rules["due_dates"]["audit_report"])

      "tds_interest_201": _tds_interest_201,

  In `parity/edge_golden.py`, a runner beside `tds_payees_run` and its entry in the `runners`
  table:

      def tds_interest_201_run():
          from tae import config as tc
          from tae.audit_tests import tds_interest_201
          from tae.pack import _tds_interest_defaults
          cfg = {"tds": {k: spec[k] for k in ("previous_year_turnover_status",) if k in spec},
                 "tds_payees": {k: spec[k] for k in ("reversals", "gst_separate_by_agreement", "foreseeability")
                                if k in spec}}
          payees = tds_payees_run()[1]
          partners = {k: dict(v) for k, v in spec.get("partners", {}).items()}
          tds_payable = frozenset(spec.get("tds_payable_ledgers", []))
          partners_result = partners_40b_194t.run(eng, rules, partners, spec.get("deed"), tds_ledgers=tds_payable)
          uncertain = (payees.figures[f"{tds_payees.TEST_ID}.deductor_status"].value == "unknown"
                       or tc.turnover_is_placeholder(cfg))
          defaults = _tds_interest_defaults(
              eng, rules, payees, partners_result, uncertain,
              (dict(spec.get("nature_by_ledger", {})), dict(spec.get("payee_aliases", {})),
               dict(spec.get("s194j_category_by_ledger", {}))), partners,
              tc.tds_payees_reversals(cfg), tc.tds_payees_foreseeability(cfg), tc.tds_payees_gst_separate(cfg),
              frozenset(spec.get("gst_ledgers", [])), tds_payable)
          return tds_interest_201, tds_interest_201.run(eng, rules, defaults, rules["due_dates"]["audit_report"])

      "tds_interest_201": tds_interest_201_run,

  Both import the test and the reference's own assembly of the rows, run the two input runners of
  the same file first, and pass the same inputs those took, as the reference runs the three in one
  pass; both give the canonical dump the test module itself, whose module check takes the
  engagement and the result. A port only needs them to keep the two sides' test lists in step (the
  crate has tests that compare the Rust registry with the harness's list).
