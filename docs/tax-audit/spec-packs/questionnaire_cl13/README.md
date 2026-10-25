# Spec pack: `questionnaire_cl13` (Form 3CD clause 13 questions, with the clause 14 stock link)

The goldens in this pack were produced by the reference engine at commit `4df1cc43` and are the
contract; this note explains them and cites [`docs/tax-audit/parity-spec-v1.md`](../../parity-spec-v1.md)
(parity spec sections 1, 2.1, 2.2, 3, 3.1, 4, 5, 6, 7 and 10, as relevant); where the note and a golden
differ, the golden wins and the reference's maintainers should be told on the pull request or issue.

In this note "README section N" is a section of this note and "parity spec section N" a section of
that document.

This pack is for porting the test into `src-tauri/crates/bridge-tax-audit`. It holds:

- `books/qc_*.json`: invented edge books, in the shape of the crate's
  `tests/fixtures/edge-books/*.json`. Two of them also carry the stock test's inputs
  (`stock_opening`, `stock_closing`), which the crate's edge-book reader already knows.
- `goldens/edge.<book>.questionnaire_cl13.json`: the reference's canonical dump of this test on each
  book (parity spec section 1), module check included.
- `goldens/edge.<book>.stock.json`: for the two books with Stock Summaries, the canonical dump of the
  stock test on the same book, from the same run. That result is this test's input
  (README section 2.2); the stock test itself is already ported (`src/stock.rs`,
  `src/stock_read.rs`) and these goldens add nothing to its contract.
- `HASHES.md`: bytes and SHA-256 of every book and golden, what the books establish, which rule
  changes each golden catches, and how the goldens were made.

## 1. What the test does

The reference puts four questions under Form 3CD clause 13, the last one linked to clause 14: the
the accounting method, whether it differs from the previous year's (and the effect), whether the profit
computation follows each notified ICDS (and the effect of any deviation), and the method used to
value closing stock and whether that changed. The clause each question is filed under is the
reference's mapping (README section 2.5), carried as data. None of the answers can be read off the
books.

The test therefore raises the four questions as four findings, each with its own answer figure that
always holds the text `not answered`, for the auditor to replace with the client's confirmed
answer. Two of the findings also point at something in the books that the auditor may want to look
at first:

- the method-of-accounting finding points at a count this test makes itself: the regular journals
  dated on the last day of the books period that touch a ledger under `Direct Expenses` or
  `Indirect Expenses` (README section 3);
- the closing-stock finding, when the stock test's result is supplied, points at that test's count
  of vouchers posting to a Stock-in-Hand ledger: the fact names that figure, and its value is never
  read (README section 2.2).

The test reaches no conclusion: it never states a method, never reads the count as evidence of one,
and never fills an answer.

## 2. Inputs

### 2.1 The book

Read from the engagement's book, as the crate's `src/book.rs` holds it:

- **Ledger masters**: each ledger's name and group chain (parent first, primary group last). The
  chain decides which ledgers are expense ledgers (README section 3). Ledger GUIDs are not read.
- **Vouchers**: GUID, date, voucher type name, base type, number, status and lines. Of a line only
  its ledger name is read; its amount, sign and size do not matter (`qc_core`'s h04 and h05).
- **The books period**: only its last day. In the edge books it is `period[1]` (default
  `2026-03-31`); in the crate it is the `to` of the period `Window`, which ported tests such as
  `ledger_scrutiny` and `party_monthly` already take (`tests/edge_books.rs` builds it with
  `period(&s)`, with the same default).
- **The books population**: vouchers whose status is regular (`Book::population`). Optional,
  cancelled and post-dated vouchers are not counted (`qc_quiet`'s m09 to m11). The test always
  forms the population, so a book with any voucher of unknown status refuses (README section 10).

Not read: the Trial Balance, narrations, the party field, references, inventory lines, group
masters (only each ledger's chain), ledger GUIDs, and the edge book's `cash` and `bank` lists.

### 2.2 The stock test's result

The test is given the `stock` result computed on the same book, or nothing. It reads one thing
from it: whether its figures include the id `stock.stock_in_hand_voucher_count`. It never reads
that figure's value or evidence, any other figure, or any finding.

- **In the reference's pack** the stock test always runs before this one and its result is always
  passed. The stock test publishes `stock.stock_in_hand_voucher_count` on every book it accepts, so
  in the real pipeline the closing-stock finding always carries the pointer (README section 4.3).
  When the stock test cannot run, the whole run stops before this test: when its inputs cannot be
  read (an engagement with no usable `[stock]` table or stock read refuses there, in the reference
  and in the crate's `stock_read::stock_inputs` alike) or when it refuses the book (for example a
  goods inventory line with no quantity field). So in the real pipeline there is never a "no
  result" case.
- **No result.** The test itself also accepts no stock result, and then the closing-stock finding
  has its answer fact only. Only the edge harness reaches this: the edge books express it by
  carrying neither `stock_opening` nor `stock_closing` (`qc_empty`, `qc_quiet`, `qc_period`,
  `qc_shared_guid`). A book that carries exactly one of the two is refused when this test's runner
  reaches it. A port's real pipeline must not turn a missing `[stock]` table into "no result"; it
  refuses, as above.
- **A result without that figure** cannot come from the stock test; the reference treats it like no
  result (README section 10).

The stock value does not change anything: `qc_core` (stock count 0, its Stock-in-Hand ledger
posted to by no voucher) and `qc_stock` (stock count 2) give the closing-stock finding the same
fact. A port runs its own stock port first on the same book, as the crate's `stock_on` does, and
passes the result in; the two `edge.<book>.stock.json` goldens show exactly what it was given.

### 2.3 Binding (before the test runs)

This test takes nothing from the engagement's configuration: no ledger name, no group name, no
table. The two group names it looks for and the base type it compares are fixed in the test
(README section 2.5), not configured. The reference's binding step runs before any test and refuses,
among other things, a configured ledger or group name that matches no master; it has no location
for this test: none of the 42 ledger-name locations and none of the 4 group-name locations in its
two binding lists belongs to it, and the crate's `src/binding.rs` needs no new location. Nothing of
this test is bound or rewritten. A binding refusal elsewhere in the configuration still stops this
test too, as it stops every test, because the run binds the whole engagement first
(README section 14).

What the test does with the fixed names: it matches them exactly, case included, against the
chain as the book holds it. A ledger under a group spelt `indirect expenses` is not an expense
ledger (`qc_quiet`'s m07), and a ledger merely named `Indirect Expenses` is not one either unless
its chain says so (`qc_quiet`'s m08). A book whose primary groups were renamed in Tally has no
expense ledger for this test, and nothing rewrites the names.

The stock test's own inputs are read and checked by its port, as today.

### 2.4 Rules

Only the rules version is read; it is copied into `rules_version` (`2026-09-17.1` in every golden).
`test_version` is `"1"`. The entity type is not read.

### 2.5 Fixed values (not client data)

These constants decide the output; they are listed as data:

- **Expense groups**: `Direct Expenses`, `Indirect Expenses`.
- **Base type**: `Journal`.
- **The borrowed figure id**: `stock.stock_in_hand_voucher_count`.
- **The answer value**: `not answered` (unit `text`).
- **The four questions**, by key, with their clause lists in the order the dump carries them:

  | Key | Clauses | Pointer |
  | --- | --- | --- |
  | `method_of_accounting` | `3CD-13(a)`, `3CD-13(b)` | the accrual-journal count |
  | `change_in_method` | `3CD-13(c)`, `3CD-13(d)` | none |
  | `icds_deviation` | `3CD-13(e)`, `3CD-13(f)` | none |
  | `closing_stock_valuation` | `3CD-13(f)`, `3CD-14(a)`, `3CD-14(b)` | the stock figure, when supplied |

  `3CD-13(f)` is in two lists; that is how the goldens carry it.
  The clause tags are the reference's own mapping, carried as data: they need not follow Form 3CD's own
  sub-clause wording item by item (for instance 13(f) sits on both the ICDS and the stock question), and a port
  reproduces them as the goldens show them; whether the mapping should change is a question for the reference.

## 3. The accrual-journal count

A population voucher is counted when all three hold:

1. **its date equals the last day of the books period.** Not the day before or the day after
   (`qc_quiet`'s m01 and m02). The day is the book's own: in `qc_period`, whose period is the
   calendar year 2025, 31 December counts (p01); 31 March does not, inside the period (p02) or
   after it (p03), and neither does the period's first day (p04). 31 March plays no part unless it
   is the book's own last day.
2. **its base type is exactly `Journal`.** The voucher type name does not matter: an
   `Accrual Journal` whose base type is `Journal` counts (`qc_core`'s h03), a `Payment` whose type is
   named `Journal` does not (`qc_quiet`'s m04), and neither does a base type written `journal`
   (m05), a `Stock Journal` (m12, with a line on a Direct Expenses ledger) or a `Payment` (m03).
3. **at least one of its lines is on an expense ledger**: a ledger whose master's chain contains
   `Direct Expenses` or `Indirect Expenses` anywhere, so a ledger under a sub-group counts
   (`qc_core`'s h02). Any line counts, whatever its amount: a debit (h01), a credit (h05), a zero
   line beside non-expense lines (h04). Lines on other ledgers only do not count (`qc_quiet`'s
   m06). A line on a ledger with no master is never an expense line (README section 10).

Vouchers are counted **once per distinct GUID**. Two counted vouchers with one GUID make one entry,
and its evidence label is the later voucher's, in book order (`qc_shared_guid`: `JV-A1` and `JV-A2`
share a GUID; the count includes it once and the label reads `JV-A2`). Only counted vouchers are
merged in this way; the three conditions are applied first, voucher by voucher. In
`qc_shared_guid`, the journal `JV-B1` shares its GUID with a later cancelled voucher and the
journal `JV-C1` with a later regular payment, `PV-C2`; both journals are counted and labelled as
themselves.

The count is reported whatever its size, 0 included, and nothing else is derived from it.

## 4. Outputs

Every figure and finding id below is exactly as the goldens carry it.

### 4.1 Figures

Five figures, on every book (`qc_empty` has no voucher at all):

| Figure id (`questionnaire_cl13.` + ) | Unit | Value | Evidence |
| --- | --- | --- | --- |
| `hint_accrual_journal_count` | count | the count of README section 3 | one voucher ref per counted GUID |
| `answer_method_of_accounting` | text | `not answered` | none |
| `answer_change_in_method` | text | `not answered` | none |
| `answer_icds_deviation` | text | `not answered` | none |
| `answer_closing_stock_valuation` | text | `not answered` | none |

Each definition is fixed text; take them from the goldens, as they are compared by hash (parity
spec section 4). The answer figures are the only `text` values, and always text
(parity spec section 2.1). The four answer definitions are one sentence pattern that differs only by the
question key, written inside single quotes as Python's `repr()` writes a plain key (for example
`'icds_deviation'`). No answer figure ever holds anything but `not answered`.

### 4.2 Evidence labels

Every ref on the count has kind `voucher` and the voucher's GUID as id; refs are sorted in the dump
(parity spec section 2.2). Its label is
`<voucher type> <number> on <ISO date>`, the crate's `support::voucher_label`. The voucher type is
the voucher's own type name, not its base type: `qc_core`'s h03 reads
`Accrual Journal AJ-1 on 2026-03-31`. An empty number is replaced by the GUID's last 12
characters, or the whole GUID when it is shorter: `qc_core`'s h06 reads
`Journal on-entry-h06 on 2026-03-31` and h07 reads `Journal h07 on 2026-03-31`.

### 4.3 Findings

Four findings on every book, one per question, ids `questionnaire_cl13/<key>`, with the clause lists
of README section 2.5 and confidence `judgement_required` (the finding shape of
parity spec section 3).

| Finding | Facts (name: figure id) | Evidence |
| --- | --- | --- |
| `method_of_accounting` | `answer`: its answer figure; `hint_accrual_journal_count`: `questionnaire_cl13.hint_accrual_journal_count` | the count's refs, exactly as on the figure |
| `change_in_method` | `answer` only | none |
| `icds_deviation` | `answer` only | none |
| `closing_stock_valuation` | `answer`; and, only when a stock result with that figure was supplied, `hint_closing_stock_typed_in`: `stock.stock_in_hand_voucher_count` | none |

The `hint_accrual_journal_count` fact is there on every book, also when the count is 0 (`qc_quiet`,
`qc_empty`). The `hint_closing_stock_typed_in` fact names the stock test's figure id, not a figure
of this test, and nothing of that figure is copied into this result (README section 7).

**Texts** are compared by hash (parity spec section 4); take them from the goldens. Each finding has
a fixed title (the question), one limit, shared word for word by all four, and two items to ask the
client. Nothing in any text depends on the book, and every golden carries every text of the test.

### 4.4 Population note

One fixed sentence, the same in every golden.

## 5. When nothing is counted, nothing is supplied, or the book is empty

There is no applicability gate. The population is always formed, and the result always has the five
figures and four findings of README section 4. With no voucher counted and no stock result, the
dump does not depend on the book: `qc_empty` (no voucher) and `qc_quiet` (twelve near misses) give
byte-identical goldens, with the count at 0 and no evidence, and the closing-stock finding with its
answer fact only. There is no other "nothing to report" form.

## 6. The module's own check (QCL-1)

`module_invariants_evaluated` is `["questionnaire_cl13.check_invariants"]` in every golden. Each
violation is `{"invariant": "questionnaire_cl13.check_invariants", "subject": "questionnaire_cl13",
"detail": <text>}`.

The check makes a count of its own. Over the book's vouchers of regular status it applies the three
conditions of README section 3, finding the expense ledgers again from the ledger masters' chains,
and adds one for **every voucher** that passes, a repeated GUID included. It then compares that
number with the value of the published count figure.

Its two messages, exactly:

- `QCL-1: questionnaire_cl13.hint_accrual_journal_count = <published value> but an independent re-walk of the books population gives <its count>`
  fires in `qc_shared_guid`, where the figure is 3 and the walk finds 4, because two counted
  vouchers share a GUID (`JV-A1` and `JV-A2`). A counted voucher that shares its GUID with an
  uncounted one adds nothing to the difference (`JV-B1`, `JV-C1`). On a book whose counted vouchers
  all have GUIDs of their own the two numbers agree and the check is silent (every other golden).
- `QCL-1: questionnaire_cl13.hint_accrual_journal_count missing from the result`
  cannot be produced by the test's own output, which always has the figure; no golden shows it.

## 7. Book and result checks in the dumps

Every golden carries the book-level and result-level reports of parity spec section 5.

- **REND-0 fires once whenever the stock pointer is attached** (`qc_core`, `qc_stock`, and the
  synthetic dump of README section 14): subject `questionnaire_cl13/closing_stock_valuation`, detail
  `fact hint_closing_stock_typed_in -> missing figure stock.stock_in_hand_voucher_count`. The dump
  evaluates the result-level checks with this test's result alone (the crate's
  `invariants::result_invariants` does the same), and the fact names a figure of the stock test. A
  port reproduces this exactly; it is not a defect in the port. Without a stock result it does not
  fire.
- `qc_shared_guid`: POP-5 at book level on each of the two GUIDs that two regular vouchers share
  (`2 in-books vouchers share this GUID`), and POP-4 twice at result level, `voucher:qc-dup-b`, once
  for the count figure's ref and once for the finding's, because the counted journal's GUID is also
  a cancelled voucher's.
- Every other report of every edge golden is empty: each book's vouchers tie to its Trial Balance and
  every line's ledger has a master.
- The synthetic dump of README section 14 also carries the synthetic read's own four book-level
  violations, as every synthetic golden of the crate does.

## 8. Ordering

Nothing about the order in which the test walks vouchers shows in a dump: figures, findings, facts,
evidence and violations are all sorted (parity spec section 6). Findings sort as
`change_in_method`, `closing_stock_valuation`, `icds_deviation`, `method_of_accounting`. Clause lists
keep their authored order (parity spec section 3.1). Book order matters in one place only: which of
two same-GUID vouchers gives the label (README section 3).

## 9. What the test never does

- It never answers a question and never sets an answer figure to anything but `not answered`;
  whatever the books hold, the four findings come out the same apart from the two pointers and the
  count's evidence.
- It never treats the accrual-journal count as showing that the books are kept on any particular
  basis, and never treats the stock pointer as showing how closing stock was valued.
- It never reads the Trial Balance, amounts, narrations or inventory, and never reads the stock
  figure's value.
- It never takes a calendar date of its own: the only date it compares is the book's period end.

## 10. Not covered by any golden

Reachable in the module but in no book:

- **A voucher of unknown status.** Forming the population refuses before any figure. The edge-book
  shape cannot express an unknown status. The crate's `Book::population` already refuses; propagate
  that error.
- **A stock result that lacks `stock.stock_in_hand_voucher_count`.** The pointer is then not
  attached, as with no result. The stock test always publishes the figure, so neither the
  reference's pack nor the edge harness can produce this.
- **A stock test that refuses the book.** The reference's pack stops before this test; so does the
  edge harness. There is nothing for this test to produce.
- **A voucher line on a ledger with no master.** It is never an expense line; the book checks would
  report it (MAP-0). No book has one.
- **A ledger whose chain is incomplete** (`chain_complete` false): the chain is used as given; the
  book checks would report it (MAP-1). No book has one.
- **Counted vouchers with a blank GUID.** They share the GUID `""` and so make one entry, labelled by
  the last of them; POP-5 would report them, and QCL-1 would fire whenever there are two or more,
  as in README section 6. No book has one.
- The second QCL-1 message (README section 6).

Rules no golden pins:

- **Where in the chain an expense group sits.** The reference looks for `Direct Expenses` and
  `Indirect Expenses` anywhere in the chain; every book has them as the primary group (last in the
  chain), so a port that looked only at the primary group would pass. Match anywhere in the chain.
- **A number of spaces only.** The reference replaces a number only when it is empty; a number of
  spaces is kept as it is. No book has one; the crate's `support::voucher_label` already behaves
  this way.

Every rule HASHES.md lists changes at least one golden when it alone is changed (measured by
changing a copy of the reference test one rule at a time and running every book again).

## 11. Behaviour that may look like a defect

Reproduce these as the goldens show them, and raise them on the pull request rather than fixing them
in the port:

- **REND-0 on every dump with the stock pointer** (README section 7): the figure the fact names exists
  among the results of a whole run, but the dump checks this test's result alone, so it fires.
- **The count and its check disagree on shared GUIDs** (`qc_shared_guid`): the figure counts GUIDs,
  QCL-1 counts vouchers, so the check fires on a book defect that POP-5 already reports.
- **The later of two same-GUID vouchers supplies the label** (`qc_shared_guid`'s `JV-A2`).
- **A GUID shared with an excluded voucher** makes POP-4 report a citation of a voucher that is in
  the population (`qc_shared_guid`'s `JV-B1`).
- **The stock pointer carries no information in the real pipeline**: it is attached whenever a stock
  result is supplied, whatever the stock count, and the reference's pack always supplies one. Its
  fact name speaks of stock typed in, but the figure it names is a voucher count; only a count of 0
  suggests that, and the auditor has to look the count up in the stock result.
- **A zero line counts** (`qc_core`'s h04): a journal whose only touch on an expense ledger is a line
  of 0 is counted.
- **The shared limit** speaks of a figure cited above in every finding, including those that cite
  only their answer figure (two in the real pipeline, three when no stock result is passed).

## 12. The books

All are invented: synthetic ledger names, round figures, nothing read from Tally. Each book's
`comment` says what it reaches, voucher by voucher. The period is the default, 1 April 2025 to
31 March 2026, except in `qc_period`. In the two books with Stock Summaries the opening one is
dated the day before the period starts and the closing one on its last day; the stock test shows
the opening date in one figure's definition only, and does not read the closing date.

| Book | Stock result | Reaches |
| --- | --- | --- |
| `qc_empty` | none | No voucher: count 0, five figures, four findings; golden byte-identical to `qc_quiet`'s. |
| `qc_quiet` | none | Twelve near misses: the day before and the day after the period's last day; a Payment; a Payment typed `Journal`; base type `journal`; a `Stock Journal` touching Direct Expenses; a non-expense journal; a lower-case group; a ledger named like the group; optional, cancelled and post-dated journals. |
| `qc_core` | given, count 0 (typed-in finding in its stock golden) | Seven counted: directly under Indirect Expenses, under a sub-group of Direct Expenses, an `Accrual Journal`, a zero expense line, a credit to an expense ledger, empty numbers with a long and a short GUID; one receipt not counted. REND-0 once. |
| `qc_stock` | given, count 2 | The pointer attached as with count 0; one counted journal that credits a Direct Expenses ledger on the last day. REND-0 once. |
| `qc_period` | none | A calendar-year period: 31 December counted; 31 March inside and after the period, and the period's first day, not counted. |
| `qc_shared_guid` | none | Two counted journals sharing a GUID (one entry, later label, POP-5); a counted journal sharing its GUID with a later cancelled one (POP-4 twice); a counted journal sharing its GUID with a later regular payment (still counted, POP-5); QCL-1 fires, 3 against 4. |

## 13. Running the books

The crate's `tests/edge_books.rs` does not run this test yet. A port adds that, for each book naming
`questionnaire_cl13`:

1. when the book carries both `stock_opening` and `stock_closing`, run the stock port on it (as the
   existing `stock` arm does, from `stock_inputs(&s)`) and keep the result; when it carries neither,
   there is no stock result; when it carries one only, refuse the book;
2. run the test with the book, the rules, the period and that result, and its module check with the
   book, the period (the check needs the period's last day; in the crate a book carries no period, so the
   check takes it as a separate argument, as `party_monthly::check_invariants` does) and the test's result;
3. compare the whole dump with `edge.<book>.questionnaire_cl13.json`, as the other edge books do
   (parity spec section 7).

`qc_core` and `qc_stock` also name `stock` in `tests`, so the existing `stock` arm compares their
stock goldens too, once the books and goldens are copied into the crate's fixtures. Copied files
need byte rows there as well: `tests/provenance_rows.rs` requires one four-cell row per golden and
edge book in a Markdown file under `tests/fixtures` (`PROVENANCE.md` or a batch file under
`provenance/`), with the crate-relative path (`edge-books/qc_core.json`,
`golden/edge.qc_core.questionnaire_cl13.json`); the byte and SHA-256 cells can be copied from
HASHES.md, whose own rows, under `docs/`, do not count.

The reference's side of the edge harness, `parity/edge_golden.py`, is in this repository, but it
has no runner for this test yet; README section 14 gives the lines. A porter does not regenerate
goldens: that is done only by the reference's maintainers.

## 14. Registering the test in the crate

A new registry entry needs, besides the books, a golden for the synthetic read, a `min_figures` value
and the two harness lines. All of them are here, so that a port does not write a golden by hand.

- **The synthetic golden.** `goldens/synthetic.questionnaire_cl13.json` is the test on the crate's synthetic
  read (`tests/fixtures/synthetic-engagement.toml`), with the stock result of the same read passed in, as the
  reference's pack passes it (that engagement has a `[stock]` table). It goes to
  `tests/fixtures/golden/synthetic.questionnaire_cl13.json`. It has the five figures (the count is 1, citing the
  read's voucher `Journal J/3 on 2026-03-31`), the four findings with the stock pointer, REND-0 once, no QCL-1
  violation, and the read's four book-level violations (as every synthetic golden of the crate does). It is not an
  empty dump, so the comparer's refusal of two empty results is not tripped. Until it is in `tests/fixtures/golden`,
  the crate's registry test that every registered test has a synthetic golden fails.
- **`min_figures`**: 5. The test publishes exactly five figures on every book, so fewer is a broken
  dump (the minimum figure count of parity spec section 7; the steps of adding a test are parity
  spec section 10).
- **The registry entry** takes no caller data: like `stock_on`, a `questionnaire_cl13_on` binds the
  engagement, reads the stock inputs and runs the stock port, then runs this test with that result.
- **The harness lines** (the reference's maintainers keep and run the harness; these are the lines a
  port adds so that its harness names the test). In `parity/python_golden.py`, a runner beside
  `_stock` and its `RUNNERS` entry (the list is kept sorted by test id):

      def _questionnaire_cl13(c):
          from tae.audit_tests import questionnaire_cl13
          return questionnaire_cl13, questionnaire_cl13.run(c.eng, c.rules, _stock(c)[1])

      "questionnaire_cl13": _questionnaire_cl13,

  In `parity/edge_golden.py`, a runner beside `stock_run` and its entry in the `runners` table:

      def questionnaire_cl13_run():
          from tae.audit_tests import questionnaire_cl13
          has = ("stock_opening" in spec, "stock_closing" in spec)
          if has[0] != has[1]:
              raise SystemExit(f"{spec_path.name}: stock_opening and stock_closing go together")
          stock_result = stock_run()[1] if has[0] else None
          return questionnaire_cl13, questionnaire_cl13.run(eng, rules, stock_result)

      "questionnaire_cl13": questionnaire_cl13_run,

  Both import the test from the reference's audit tests and run the stock runner of the same file
  first (the edge runner only for a book with Stock Summaries), as the reference's pack runs the
  stock test first; both give the canonical dump the test module itself, whose module check in the reference takes
  the book (with its period) and the result. A port only needs them to keep the two sides' test
  lists in step (the crate has tests that compare the Rust registry with the harness's list).
