# Spec pack: `related_parties_cl23` (Form 3CD clause 23, s.40A(2)(b))

The goldens in this pack are the reference engine's, regenerated at its commit `ee17d80f` (see
`HASHES.md`), and are the contract; this note explains them and cites [`docs/tax-audit/parity-spec-v1.md`](../../parity-spec-v1.md)
(sections 1, 2, 2.1, 2.2, 3, 3.1, 4, 5, 6 and 11, as relevant); where the note and a golden differ,
the golden wins and the reference's maintainers should be told on the pull request or issue.

This pack is for porting the test into `src-tauri/crates/bridge-tax-audit`. It holds:

- `books/rp_*.json`: invented edge books, in the shape of the crate's
  `tests/fixtures/edge-books/*.json`, plus one key, `related_parties` (below).
- `goldens/edge.<book>.related_parties_cl23.json`: the reference's canonical dump of this test on
  each book (parity spec section 1), module check included.
- `HASHES.md`: where the goldens came from, bytes and SHA-256 of every book and golden, and how the goldens were made.

## 1. What the test does

The client confirms who its related persons are (family members of the proprietor or of a
partner, and businesses in which any of them holds a substantial interest), and which of its ledgers carry each person's
salary, rent, interest, purchases or other dealings. For each confirmed person the test reports
what moved on those ledgers in the books population, nature by nature, and, for a nature the client
says runs a payable balance, the opening and closing payable and the amounts accrued and paid
during the year, read off the Trial Balance. It then raises one computed finding listing those
figures for clause 23 and one judgement finding saying that whether the amounts are reasonable
under s.40A(2)(b) is not assessed.

With no person confirmed, it reports that clause 23 cannot be answered from the books and asks
the client for the list.

## 2. Inputs

### 2.1 The book

Read from the engagement's book, as the crate's `src/book.rs` holds it:

- **Vouchers**: GUID, date, voucher type, number, status and lines; each line is a ledger name and
  an integer amount in paise, debit positive, credit negative.
- **The books population**: vouchers whose status is regular. Optional, cancelled and post-dated
  vouchers are excluded (`Book::population` in `src/book.rs`; a book with any unknown status
  refuses to form a population). Only population vouchers are walked. With no related person
  confirmed the test never forms the population at all, so a book with an unknown status does not
  fail it (no golden can show this).
- **Trial Balance rows**, by ledger name: `opening` and `closing` (signed, debit positive),
  `debit` and `credit` (period totals, both positive). The Trial Balance is read as given; the test
  does not recompute it from the vouchers.
- **Ledger masters** are not read by the test itself. They matter only to the result-level check
  that every cited ledger exists (section 7).

Groups, chains, narrations, parties and inventory are not read.

### 2.2 The related-person table

A JSON object keyed by **person key** (any text the client uses for the person). Each value is an
object with three optional keys:

| Key | Shape | Absent means |
| --- | --- | --- |
| `relationship` | text, as the client confirmed it | `""` |
| `ledgers_by_nature` | object: nature name to a list of ledger names | no natures |
| `payable_natures` | list of nature names | no payable natures |

In the reference's pack this is the client configuration's `[related_parties]` table, passed through
verbatim; in the edge books it is the top-level key `related_parties`, and an absent key means an
empty table. The goldens cover these shapes only. A value that is not an object or a ledger list that
is not a list of text is not covered by any golden; ask before choosing a behaviour for it. A
relationship that is not text is refused (section 10).

### 2.5 Binding of the ledger names (before the test runs)

The test does not check the ledger names in `ledgers_by_nature` against the books. In the reference's
real pipeline a separate step does it first, for every list under `ledgers_by_nature`: a name that
matches no ledger master is refused with a binding error naming the location, and a ledger renamed
since the table was written is rewritten to its current name by identity. In the crate that step is
`src/binding.rs`, which binds only the locations the ported tests read, in the order the reference
binds its name locations; this location is bound at `src/binding.rs:881-924`. `rp_unknown_ledger`
shows what the test itself does with a name that has no master (it contributes 0 and the result-level
check reports each citation), which the real pipeline never reaches because it refuses the name earlier.

The position in the reference's list of ledger-name locations (42 in all, in the order it binds them): the
location `related_parties.<person>.ledgers_by_nature.<nature>` (a list) is number 25. It follows the three
`partners.<partner>` locations (number 22 `capital_ledgers`, 23 `interest_ledger`, 24 `remuneration_ledger`)
and precedes `statutory_dues.salary_expense_ledgers` (26) and `statutory_dues.nature_by_ledger` (27). In the
crate's `src/binding.rs` that is after the block that binds the `[partners]` locations and before the block that
binds the `statutory_dues` locations. The order only decides which refusal is reported first when several
names are unknown, and the order in which renamed ledgers are rewritten.

### 2.3 The fixed nature vocabulary

`salary`, `rent`, `interest`, `purchases`, `other`, processed in that order. Any other name, in
`ledgers_by_nature` or in `payable_natures`, is ignored without a word (`rp_core`: `commission` and
`bonus`).

### 2.4 Rules

Only the rules version is read; it is copied into `rules_version` (`2026-09-17.1` in every golden).
`test_version` is `"1"`. The entity type is not read.

## 3. Outputs

Every figure and finding id below is exactly as the goldens carry it. `<tag>` is the person key's
tag: the first 8 hex digits (lowercase) of the SHA-1 of the key's UTF-8 bytes **as written**: no
trimming, no case folding and no Unicode normalisation. Section 11 of the parity spec (ledger tags
by GUID) does not apply to it. `rp_many` holds keys that differ only in case, and two keys that are
the same text in composed and decomposed form: each is its own person with its own tag. `<nature>`
is a vocabulary name.

### 3.1 The applicability gate

`applicable` is `"yes"` when the table has at least one person key, whatever that person's value
holds (`rp_partial`: a person with an empty object still counts). It is `"no"` for an absent or
empty table.

When `"no"` (`rp_none`): the dump has exactly one figure, `related_parties_cl23.applicable`, and
one finding, `related_parties_cl23/not_confirmed` (clauses `["3CD-23", "s.40A(2)(b)"]`, confidence
`judgement_required`, no facts, no evidence, one limit, one ask). The population note is the empty
string. Nothing is inferred from ledger names: `rp_none` has ledgers named like a relative's salary
and rent that move, and the gate is still `"no"`.

### 3.2 Figures when applicable

Each id is prefixed `related_parties_cl23.` (parity spec section 2).

| Figure | Value | Unit | Evidence |
| --- | --- | --- | --- |
| `applicable` | `"yes"` | text | none |
| `related_person_count` | number of person keys | count | none |
| `relationship_<tag>` | the `relationship` text, byte for byte (not normalised; `rp_many`) | text | none |
| `amount_<nature>_<tag>` | section 4 | paise | the set's ledgers, then its entries' vouchers |
| `payable_opening_<nature>_<tag>` | minus the sum of the set's TB `opening` | paise | the set's ledgers |
| `payable_closing_<nature>_<tag>` | minus the sum of the set's TB `closing` | paise | the set's ledgers |
| `payable_accrued_<nature>_<tag>` | the sum of the set's TB `credit` | paise | the set's ledgers |
| `payable_paid_<nature>_<tag>` | the sum of the set's TB `debit` | paise | the set's ledgers |
| `amount_total_<tag>` | the sum of the person's `amount_<nature>` figures | paise | none |

Per person, always: `relationship_<tag>` and `amount_total_<tag>`. For each vocabulary nature whose
list in `ledgers_by_nature` names at least one ledger: `amount_<nature>_<tag>`. A nature with an
empty list or no entry gets no figure (`rp_partial`). For each such nature that is also in
`payable_natures`: the four `payable_*` figures. A payable nature with no ledgers gets none
(`rp_core`: `purchases`; `rp_partial`: `salary`).

The **ledger set** of a nature is the set of names in its list: duplicates collapse (`rp_walk` lists
one ledger twice). A ledger named under two natures counts in both, and twice in the total
(`rp_many`, a negative total). A ledger shared by two persons counts for each (`rp_many`).

Amounts are integers in paise and can pass 32 bits (`rp_walk`); use `i64`.

**Evidence** (parity spec section 2.2): a `ledger` ref per name in the set, id the name, label
`""`; and for `amount_<nature>` a `voucher` ref per distinct (GUID, label) pair among its entries, id the voucher GUID, label
`"<voucher type> <number> on <ISO date>"`, where an empty number is replaced by the GUID's last 12
characters (`rp_walk`). The voucher type is the voucher's own type, not its base type (`rp_walk`:
`Rent Payment`). The crate's `support::voucher_label` builds this label. Two vouchers with one GUID,
number, type and day are two entries, counted twice in the amount, and cited once
(`rp_shared_guid_places`: the blank-GUID pair); two with one GUID and different labels are cited
twice. The reference's own readers refuse a read in which a voucher has no GUID or two vouchers
share one, so only a book built directly, as these edge books are, reaches these rows. A name in
the set with no ledger master is still cited (section 7). Sorting is by `"<kind>:<id>"`, then label, so ledger refs
come before voucher refs.

**Definitions** are fixed sentences with the tag and the nature substituted. Their exact text is in
the goldens (`definition_text`; only its hash is compared, parity spec section 4): `rp_core` carries
every figure kind.

### 3.3 Findings when applicable

For each person with **any nonzero `amount_<nature>`** (the total does not matter: `rp_partial`'s
Person D nets to a total of 0 and still gets both), two findings:

| Finding id | Clauses (ordered, section 3.1) | Confidence |
| --- | --- | --- |
| `related_parties_cl23/clause23/<tag>` | `["3CD-23", "s.40A(2)(b)"]` | `computed` |
| `related_parties_cl23/40a2b_reasonableness/<tag>` | `["s.40A(2)(b)"]` | `judgement_required` |

Neither carries evidence. Both carry the same facts (sorted by name, section 3):

| Fact name | Figure |
| --- | --- |
| `<nature>` | `amount_<nature>_<tag>`, for every nature with a figure, zero or not |
| `<nature>_payable_opening` | `payable_opening_<nature>_<tag>` |
| `<nature>_payable_closing` | `payable_closing_<nature>_<tag>` |
| `<nature>_accrued` | `payable_accrued_<nature>_<tag>` |
| `<nature>_paid` | `payable_paid_<nature>_<tag>` |
| `total` | `amount_total_<tag>` |

The clause 23 finding has one limit and no ask; the reasonableness finding has one limit and one
ask. A person whose every nature nets to zero gets figures and no finding (`rp_quiet`; `rp_partial`'s
Person C).

**Titles, limits and asks** are compared by hash (section 4). Take the text from the goldens:
`rp_none` for `not_confirmed`, `rp_core` for the two per-person findings (titles carry the tag).

### 3.4 Population note

Empty when not applicable; otherwise one fixed sentence, the same in every applicable golden
(`population_note_text`).

## 4. The per-person, per-nature amount

For a person and a nature with a ledger set:

1. Walk the population vouchers in book order.
2. For each voucher, add up its lines whose ledger is in the set (two lines on one ledger, or lines
   on two ledgers of the set, are all summed: `rp_walk`). That is the voucher's net on the set.
3. A voucher whose net is zero is dropped: it is no entry and is not cited (`rp_walk`'s
   reclassification between two ledgers of one set; `rp_quiet`).
4. Every other voucher is its own entry. Its key is its GUID when no other population voucher has
   that GUID; otherwise the GUID together with the voucher's place among the population vouchers
   that share it, in book order. Two vouchers that share a GUID are therefore both counted, and
   one that nets to zero is still no entry (`rp_shared_guid`, where the book's own GUID check,
   POP-5, also fires). The evidence does not follow the entries one for one: it is the distinct
   (GUID, label) pairs of the entries. Two entries with the same GUID and different labels are both
   cited; two with the same GUID, number, type and day are two entries (both in the amount) and one
   citation (`rp_shared_guid_places`).
5. `amount_<nature>_<tag>` is the sum of the entries' nets, and may be negative (`rp_core` salary,
   `rp_walk` minus 37 paise in one entry).

Vouchers outside the population never count, whatever they touch (`rp_core`, `rp_walk`,
`rp_quiet`). A ledger with no Trial Balance row is walked like any other.

## 5. The payable block

For a payable nature, the four `payable_*` figures read the Trial Balance rows of the set's
ledgers, summed across the set:

- opening payable = minus the summed `opening`; closing payable = minus the summed `closing` (payables
  usually carry credit balances, which are negative in the Trial Balance, so these come out
  positive);
- accrued = the summed `credit`; paid = the summed `debit`;
- a ledger in the set with no Trial Balance row contributes 0 to all four (`rp_core`'s
  `Person A Advance`, `rp_quiet`'s interest set, `rp_unknown_ledger`).

They are reported as read, consistent or not. On a consistent row, paid = opening payable +
accrued - closing payable (`rp_core`, both payable natures).

## 6. The module's own checks

`module_invariants_evaluated` is `["related_parties_cl23.check_invariants"]` in every golden,
including `rp_none`. Each violation is `{"invariant": "related_parties_cl23.check_invariants",
"subject": "related_parties_cl23", "detail": <text>}`; with invariant and subject fixed, the dump's
order is by detail text (parity spec section 5). The detail texts are in the goldens that fire.
Both checks use a tolerance of 100 paise and fire only when it is exceeded: every comparison in them,
including the two that compare a published opening or closing payable with the recomputed one
(under XCL-1 below), is `more than 100`, never an exact match.

**SUM-1** (fires in `rp_sum_check`). For each `amount_<nature>` figure: the Trial Balance movement
of its set is the sum, over the ledgers that figure cites, of `closing - opening` (0 for a
ledger with no row). The check fires when the absolute figure value exceeds the absolute movement
by more than 100. In `rp_sum_check`: each rent ledger is exactly 100 over, so the book's voucher
tie (POP-1) is silent on both, but the set is 200 over and SUM-1 fires; the rent rows' `debit`
columns are left at the voucher totals, so only `closing - opening` makes it fire; the salary
ledger is exactly 100 over and is silent; the interest ledger's movement has the opposite sign to
its population net, and since magnitudes are compared SUM-1 is silent (POP-1 fires there and on
`Cash`). Figures without ledger evidence (`amount_total`) are skipped.

**XCL-1** (fires in `rp_payable_break`). For each `payable_paid_<nature>_<tag>`: opening and
closing payable are recomputed from the Trial Balance rows of the ledgers that figure cites; paid
and accrued are the published `payable_paid` and `payable_accrued` values; the check fires when
paid differs from opening payable + accrued - closing payable by more than 100. In `rp_payable_break`: the salary row has its debit and credit transposed (fires); the rent
row's debit is 100 over (silent); the interest row's is 101 over (fires). The check has three more
messages: a paid figure with no matching accrued figure (`XCL-1: <paid id> has no matching <accrued id>`), a paid figure
with no ledger evidence (`XCL-1: <paid id> carries no ledger evidence to verify against the Trial Balance`),
and published opening or closing payable that differs from the recomputed value by more than 100 paise (`XCL-1: <id> = <value>p but the TB
itself gives opening payable <value>p for the same ledger set`, and the same with `closing`). The test's own
output cannot reach them (it always emits the four figures together, with the same ledger
evidence, from the same rows), so no golden shows them.

## 7. Book and result checks in the dumps

Every golden carries the book-level and result-level reports of parity spec section 5. They are
empty except where a book is built to trip them:

- `rp_shared_guid`: POP-5 on both shared GUIDs.
- `rp_sum_check`: POP-1 on `Sum Interest` and `Cash`.
- `rp_unknown_ledger`: EVID-1 five times, the same text each time, once per citation of the
  missing ledger (the amount figure and the four payable figures). The violations list keeps all
  five.

## 8. Ordering

- Persons are processed in key order by code point; natures in vocabulary order. Neither order
  shows in a dump, because figures, findings, facts, evidence and violations are all sorted
  (parity spec section 6), but key order decides which person's refusal is named when more than one
  relationship is not text (section 10).
- Clause lists keep their authored order (section 3.1).

## 9. What the test never does

- It never infers a related person, a relationship or a nature from a ledger, group or party name.
  Only the client's table names them.
- It never judges whether an amount is reasonable, at fair market value or needed for the business;
  that is the judgement finding's question to the client.
- It never reads ledgers outside a configured set (`rp_core`: `Person A Loan` is the person's but
  is not configured, and `Commission` sits under an ignored nature).
- It never reads vouchers outside the books population, and never changes the Trial Balance.

## 10. Not covered by any golden

- **Two person keys with the same tag.** For a non-empty table the reference refuses it with a typed
  error that names the tag and every key sharing it, sorted (measured with the invented keys
  `Person PXD` and `Person ACOW`, which share tag `773442d1`). The test returns no result and no
  golden is possible. A port must refuse such a table rather than merge or drop a person; the exact
  refusal form is the port's own.
- **A relationship that is not text.** For a non-empty table the reference refuses it with a typed
  error that names the person key. The keys are checked in sorted order, each for text before the
  tags are compared, so a table with both a non-text relationship and a tag collision gets the
  non-text refusal. A person whose `relationship` key is absent has the text `""`, not a refusal.
  The test returns no result and no golden is possible; the exact refusal form is the port's own.
  (An empty table is not refused: the test returns `applicable` `no` first.)
- A table value of the wrong shape (section 2.2), and a voucher of unknown status (the edge-book
  shape cannot express one).

## 11. The books

All are invented: plain names, round figures, nothing read from Tally. Each book's `comment` says
what it reaches.

| Book | Reaches |
| --- | --- |
| `rp_none` | Gate `"no"` and its finding; ledgers named like a relative's do not matter. |
| `rp_core` | One person, four natures; two payable natures whose rows tie (one set holding a ledger with no TB row); an excluded optional accrual; ignored nature and payable names; a payable nature with no ledgers; a negative nature. |
| `rp_walk` | The voucher walk: duplicate name in a list; two ledgers of a set in one voucher; a zero-net voucher dropped; two lines on one ledger; cancelled, optional and post-dated vouchers excluded; an amount past 32 bits; an empty voucher number; a voucher type other than its base type; entries of 1, 4,321 and minus 37 paise. |
| `rp_quiet` | Every figure, no finding: zero-net and excluded vouchers only; a payable balance with no movement; a payable nature whose ledger has no TB row. |
| `rp_partial` | No relationship key; an empty nature list; an unconfigured payable nature; an empty person (counted, gate `"yes"`); natures netting to a total of 0 with findings raised. |
| `rp_many` | Six persons: case-only differences, composed and decomposed keys, Devanagari key and relationship, a decomposed relationship; a shared ledger; one ledger under two natures. |
| `rp_payable_break` | XCL-1: transposed debit and credit (fires); 100 over (silent); 101 over (fires). |
| `rp_sum_check` | SUM-1: two ledgers each at the tolerance, the set over it (fires, POP-1 silent); one ledger exactly at it (silent); opposite signs (silent; POP-1 fires). |
| `rp_unknown_ledger` | A set naming a ledger the book has no master or TB row for; EVID-1 per citation. |
| `rp_shared_guid` | Population vouchers sharing a GUID: each is its own entry, so both count; a zero-net one is still no entry. |
| `rp_shared_guid_places` | Four population vouchers on one GUID and a blank-GUID pair: each is its own entry (the fourth, netting to zero on the rent set, is none); the rent figure cites the distinct (GUID, label) pairs sorted by GUID then label, so the blank-GUID pair is cited once and the shared GUID's labels sort `Payment 10`, `2`, `3`. |

## 12. Running the books

The crate's `tests/edge_books.rs` reads `related_parties` (absent meaning `{}`), runs the test on the
book with it and compares the whole dump with the golden, as it does for the other edge books (parity
spec section 7); a port written from this pack does the same. The side of the harness that runs the
reference is the crate's own `parity/edge_golden.py`, which already runs this test (`HASHES.md` gives
the command), so a porter does not write one. Running it needs the reference, which is private: a porter
compares with the goldens in this pack and cannot regenerate them. When the reference's behaviour
changes, its maintainers regenerate them and say so in `HASHES.md`.

## 13. Registering the test in the crate

The test is in the crate's registry. Besides the books, its entry has a golden for the synthetic read and a
runner in each of the harness's two scripts, described here as they are.

- **The synthetic golden.** `goldens/synthetic.related_parties_cl23.json` is the test on the crate's synthetic
  read (`tests/fixtures/synthetic-engagement.toml`). That engagement has no related-person table, so the dump is
  the unconfirmed one: one figure (`applicable` = "no") and the `not_confirmed` finding. The crate holds it,
  byte for byte, as `tests/fixtures/golden/synthetic.related_parties_cl23.json`. The registry's `min_figures` for
  this test is 1.
- **The harness runners** (the reference's maintainers keep the harness, run it against the reference and
  regenerate the goldens). In `parity/python_golden.py`, the runner and its `RUNNERS` entry (the `RUNNERS` list
  is kept sorted by test id):

      def _related_parties_cl23(c):
          from tae.audit_tests import related_parties_cl23
          from tae.config import related_parties_config
          return related_parties_cl23, related_parties_cl23.run(c.eng, c.rules, related_parties_config(c.cfg))

      "related_parties_cl23": _related_parties_cl23,

  In `parity/edge_golden.py`, in the `runners` table, with the book's `related_parties` key (absent meaning
  `{}`):

      "related_parties_cl23": lambda: (related_parties_cl23, related_parties_cl23.run(
          eng, rules, related_parties_config({"related_parties": spec.get("related_parties", {})}))),

  Both runners import the module from the reference's audit tests and the reader from its configuration module,
  and are run against the reference by the reference's maintainers. In the crate, `tests/registry.rs` checks that
  the `RUNNERS` list of `parity/python_golden.py` names exactly the tests of the Rust registry.
