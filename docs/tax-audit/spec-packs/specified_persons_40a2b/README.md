# Spec pack: `specified_persons_40a2b` (Form 3CD clause 23 family, s.40A(2)(b) and s.40(b))

The goldens in this pack were produced by the reference engine at commit `ed9a29af` and are the
contract; this note explains them and cites [`docs/tax-audit/parity-spec-v1.md`](../../parity-spec-v1.md)
(sections 1, 2, 2.2, 3, 3.1, 4, 4.1, 5, 6 and 7, as relevant); where the note and a golden differ, the
golden wins and the reference's maintainers should be told on the pull request or issue.

This pack is for porting the test into `src-tauri/crates/bridge-tax-audit`. It holds:

- `books/sp_*.json`: invented edge books, in the shape of the crate's
  `tests/fixtures/edge-books/*.json`, plus the key `related_parties` (section 2.2) and, where it
  matters, `entity_type` (section 2.4).
- `goldens/edge.<book>.specified_persons_40a2b.json`: the reference's canonical dump of this test on
  each book (parity spec section 1), module check included.
- `goldens/edge.<book>.related_parties_cl23.json`: the canonical dump of the related-party test on
  the same book, from the same run. It is this test's input, shown so that a porter can see every
  amount the findings below rest on (section 2.1).
- `HASHES.md`: bytes and SHA-256 of every book and golden, what the books establish, and how the
  goldens were made.

## 1. What the test does

`related_parties_cl23` reports, for each person the client has confirmed as related, what moved on
that person's ledgers, nature by nature. This test takes that result and, for every nature with a
nonzero amount, names the outside evidence the auditor needs before that amount can be weighed
under s.40A(2)(b): a market figure suited to the nature (a prevailing salary, a market rent, an
unrelated lender's rate, an arm's-length price, or a comparable for any other nature). There is one
exception: when the engagement's entity rules apply s.40(b) and the client has recorded the person
as a partner, the person's salary and interest are reported as falling under s.40(b) instead, for
the client to confirm, and no market figure is asked for them.

It does no arithmetic on the amounts, applies no limit, and offers no view on whether an amount is
too high. Every finding it raises is a question for the auditor or the client.

## 2. Inputs

### 2.1 The related-party test's result

The test is given the `related_parties_cl23` result computed on the same book with the same
related-person table, and reads from it only the figures `related_parties_cl23.amount_<nature>_<tag>`:
each one's id, value and evidence. It reads no other figure of that result (not the payable figures,
not `amount_total_<tag>`, not the relationship figure) and none of its findings.

A port obtains that result by running its own port of `related_parties_cl23` first and passing the
result in, as the reference's pack does. The contract for that test is its own spec pack,
`../related_parties_cl23/` (its README and goldens), where that pack is present. This test needs that
test's result as input: a port either has its port of `related_parties_cl23` working first, or its
harness reads the input from the golden `edge.<book>.related_parties_cl23.json` of the same book (its
amount figures carry the id, value and evidence this test reads). Each book here also names `related_parties_cl23`, and its golden is the exact input this
test saw on that book. In short, from that pack: for a person and a vocabulary nature whose ledger
list names at least one ledger, there is always an `amount_<nature>_<tag>` figure, whose value is the
net of the population vouchers on the nature's ledger set (zero-net vouchers dropped) and may be
zero or negative, and whose evidence is one `ledger` ref per name in the set and one `voucher` ref per
entry.

### 2.2 The related-person table

The same table `related_parties_cl23` reads, passed unchanged to both tests: a JSON object keyed by
**person key**, each value an object with the optional keys `relationship` (text),
`ledgers_by_nature` (nature name to a list of ledger names) and `payable_natures`. In the reference's
pack it is the client configuration's `[related_parties]` table; in the edge books it is the
top-level key `related_parties`, and an absent key means an empty table.

This test reads:

- the person keys (for the gate, the order and the tag);
- `relationship`, absent meaning `""` (section 3.3);
- of `ledgers_by_nature`, only whether each vocabulary nature's list is present and non-empty. It
  never reads a ledger name itself.

It never reads `payable_natures` (`sp_core` and `sp_shapes` list some; no finding depends on them).

### 2.3 Binding of the ledger names (before either test runs)

In the reference's real pipeline a separate step binds every list under `ledgers_by_nature` to the
book's ledger masters before either test runs: a name that matches no ledger master is refused with
a binding error naming the location, and a ledger renamed since the table was written is rewritten
to its current name by identity. In the crate that step is `src/binding.rs`; the location is the
one the related-party port adds there, and this test needs nothing more, because it reads no ledger
name. The `relationship` text is not bound or checked by anything: it is read as written.

The position of that location in the reference's list of ledger-name locations (42 in all, in the order it
binds them): `related_parties.<person>.ledgers_by_nature.<nature>` (a list) is number 25, after the three
`partners.<partner>` locations (22 to 24) and before the two `statutory_dues` locations (26 and 27). In the
crate's `src/binding.rs` that is after the block that binds the `[partners]` locations and before the block
that binds `statutory_dues`.

What this test does with a ledger that reached it unbound is shown by `sp_unknown_ledger`, which the
real pipeline never reaches: the related-party amount still nets the set's other ledgers, the
finding repeats that figure's evidence, the unknown ledger included, and the result-level check
EVID-1 reports it (section 5).

### 2.4 Rules and the entity type

- The rules version is copied into `rules_version` (`2026-09-17.1` in every golden). `test_version`
  is `"1"`.
- The **s.40(b) rule** is open when the rules' entity value `s40b_interest_rate_bp` for the
  engagement's entity type is greater than 0, an absent value reading as 0. In the rules at this
  commit: `firm` and `llp` carry 1200 (open); `individual` carries 0; `company` has no such value;
  any other type, `huf` among them, has no `[entity.<type>]` table (all closed). The crate's
  `Rules::s40b_interest_rate_bp(entity_type)` gives exactly this number. The rate itself is not
  used; only whether it is above 0.
- In the edge books the entity type is the key `entity_type`, absent meaning `individual`, as the
  existing edge books already use it. The test reads the entity type only through this rule.

### 2.5 The book

The test does not read the book at all: no voucher, Trial Balance row, ledger master or group. It
never forms the books population, whether the table is empty or not. Everything it knows of the book
comes through the related-party figures. The book matters to a golden only through those figures and
through the book-level and result-level checks every dump carries (section 5).

### 2.6 Fixed vocabularies (not client data)

- **Natures**, the related-party test's: `salary`, `rent`, `interest`, `purchases`, `other`,
  processed in that order. Any other key under `ledgers_by_nature` is ignored, and the match is
  exact: `Salary` with a capital S is not `salary` (`sp_shapes`).
- **Partner labels**: `partner`, `partner in the firm`, `working partner`.
- **Natures s.40(b) governs**: `interest` and `salary`.
- **The comparable for each nature**: a fixed phrase per nature that ends up in the
  comparable-required finding's first ask. `sp_core` carries all five (section 3.4).

## 3. Outputs

Every figure and finding id below is exactly as the goldens carry it. `<tag>` is the person key's
tag, built exactly as the related-party test builds it: the first 8 hex digits (lowercase) of the
SHA-1 of the key's UTF-8 bytes as written, with no trimming, case folding or normalisation (the
crate's `support::hash8`; `sp_keys`). `<nature>` is a vocabulary name.

### 3.1 The gate and the only figure

The test has exactly one figure, `specified_persons_40a2b.applicable`: `"yes"` when the table has at
least one person key, whatever else the values hold (`sp_empty`: one key whose value is an empty
object makes it `"yes"`, with no finding), unless the table is refused first (two keys with one tag,
or a relationship that is not text: section 8), and `"no"` for an absent or empty table (`sp_none`). Unit
`text`, no evidence; its definition is a fixed sentence (`definition_text` in any golden). The gate
reads the table, not the related-party test's own gate; the two agree because both read the same
table.

When `"no"`: no finding at all, the population note is the empty string, and the module check is
still listed as evaluated (`sp_none`). The test reports no count of persons and no per-person figure
in either case: those belong to the related-party test.

### 3.2 Which (person, nature) pairs are looked at

For each person, and for each vocabulary nature whose list under `ledgers_by_nature` is present and
non-empty:

1. Take the related-party figure `related_parties_cl23.amount_<nature>_<tag>`. If the result passed
   in has no such figure, the test stops with an error rather than computing an amount (section 8).
2. If its value is 0, nothing is reported for the pair. Zero is exact: there is no tolerance. It
   covers a nature whose vouchers net to zero, a nature touched only by vouchers outside the
   population, a set whose only voucher moves money between two of its own ledgers, a ledger that
   never moves, and a set naming only a ledger the book does not have (`sp_zero`,
   `sp_unknown_ledger`).
3. Any other value, negative included, gives exactly one finding: `s40b_governs` when the rule of
   section 3.3 holds, `comparable_required` otherwise. A negative amount is not skipped
   (`sp_core`'s `other`, `sp_firm`'s Partner B interest, `sp_zero`'s minus 1 paise); 1 paise is
   enough (`sp_zero`).

A person with no `ledgers_by_nature`, with only empty lists, or with only non-vocabulary natures gets
nothing (`sp_shapes`). A ledger listed twice in one nature is still one nature (`sp_shapes`'s
Partner T). The same ledger under two natures of one person, or under natures of two persons, gives
one finding per (person, nature), each with the full amount (`sp_firm`).

### 3.3 The s.40(b) rule

A pair is governed by s.40(b) when all three hold:

1. the s.40(b) rule is open for the engagement's entity type (section 2.4);
2. the person's `relationship`, after stripping leading and trailing whitespace and lower-casing, is
   exactly one of the three partner labels;
3. the nature is `interest` or `salary`.

The stripping and lower-casing are Python 3.13's `str.strip()` and `str.lower()` (parity spec 4.1 on
the Unicode version), which the crate already reproduces as `support::py_strip` and
`support::py_lower`. Two consequences the goldens show (`sp_labels`):

- Whitespace is Python's: besides the usual spaces, tabs and newlines, the no-break space U+00A0,
  the ideographic space U+3000 and the control characters U+001C to U+001F are stripped. The last
  four are not whitespace to Rust's `str::trim`. A zero-width space U+200B is not whitespace to
  either, so it blocks the match. Whitespace inside the text is kept: `working  partner` with two
  spaces does not match.
- Lower-casing is full Unicode lower-casing: the Kelvin sign U+212A lower-cases to `k`, so
  `WORKING PARTNER` written with it matches; U+0130 lower-cases to two characters, so a label written
  with it does not; full-width letters stay full-width and do not match.

Anything else does not match: `sleeping partner`, `partners`, `partner's son`, an absent
relationship (read as `""`). With the rule closed, a recorded partner's salary and interest ask for a
comparable like any other pair (`sp_core`'s Person B, `sp_company`, `sp_huf`). With it open, a
partner's `rent`, `purchases` and `other` still ask for a comparable, even on the very ledger whose
`salary` amount is governed (`sp_firm`: Partner A's remuneration ledger is listed under both
`salary` and `other`).

### 3.4 Findings

| Finding id | Clauses (ordered, parity spec section 3.1) | Confidence |
| --- | --- | --- |
| `specified_persons_40a2b/comparable_required/<tag>/<nature>` | `["3CD-23", "s.40A(2)(b)"]` | `judgement_required` |
| `specified_persons_40a2b/s40b_governs/<tag>/<nature>` | `["s.40A(2)(b)", "s.40(b)"]` | `judgement_required` |

Both kinds carry:

- **one fact**, named `amount`, whose figure id is the related-party figure
  `related_parties_cl23.amount_<nature>_<tag>`: a figure of the other test, not of this one (see
  REND-0 in section 5);
- **the evidence of that figure, unchanged** (parity spec section 2.2): its `ledger` refs, label
  `""`, one per name in the set, a name with no ledger master included; and its `voucher` refs, id
  the voucher GUID, label `"<voucher type> <number> on <ISO date>"` with an empty number replaced by
  the GUID's last 12 characters (the crate's `support::voucher_label`; `sp_core`'s
  `Rent Payment ore-rent-p02 on 2025-06-10`). A port copies the evidence from the figure; it does not
  rebuild it.

**Texts** are compared by hash (parity spec section 4); take them from the goldens:

- `comparable_required`: the title is a fixed sentence with the nature (in single quotes) and the
  tag substituted; the limits are one fixed sentence; the asks are two, the first a fixed frame
  around the nature's comparable phrase, the second a fixed sentence. `sp_core` carries all five
  natures.
- `s40b_governs`: the title is a fixed sentence with the tag substituted; the asks are one fixed
  sentence; the limits are one sentence with the tag and the **relationship as written** substituted:
  split on whitespace and joined with single spaces (so padding goes, and each run of spaces, tabs,
  newlines and other whitespace becomes one space), in single quotes, not lower-cased and with no
  escaping (`'Working Partner'`, `'Partner in the firm'`, `'partner'` in `sp_labels`, where the raw
  texts carry tabs, newlines, no-break and ideographic spaces and the control characters U+001F and
  U+001C). The split is Python's `str.split()`, so the whitespace set is the one of section 3.3,
  **including U+001C to U+001F**: a join over Rust's `split_whitespace()` differs on those four
  characters. The crate's `support::py_split` splits on Python's whitespace set; build the join from
  it, not from `split_whitespace()`.
- Every text then goes through the dump's NFC normalisation before it is written and hashed (parity
  spec section 4), so the Kelvin sign in a relationship appears as a plain `K` in `limits_text`
  (`sp_labels`).

### 3.5 Population note

Always the empty string, applicable or not.

## 4. The module's own check (SPD-1)

`module_invariants_evaluated` is `["specified_persons_40a2b.check_invariants"]` in every golden,
including `sp_none`. The check takes the same table, rules and related-party result as the test, plus
the test's result. For each person in key order and each vocabulary nature with a non-empty list, it
takes the related-party amount figure, skips the pair when that figure is missing (where the test
itself stops with an error) or its value is 0, re-applies the rule of section 3.3, and looks for the
two possible finding ids in the result. It never fires on the test's own output, so no golden shows a
violation. Each violation is `{"invariant": "specified_persons_40a2b.check_invariants", "subject":
"specified_persons_40a2b", "detail": <text>}`.

The conditions are tried in this order and the first that holds gives the message; each message names
what was emitted. `<key>`, `<nature>` and `<relationship>` are Python `repr()` renderings (quoted; the
`repr()` of the unmodified relationship text, not the whitespace-joined form of section 3.4; the
crate's `support::py_repr_str` renders it), `<governed id>` and `<comparable id>` are the two finding
ids for the pair:

a. both ids present:
   `SPD-1: <key> nature <nature> has BOTH <governed id> and <comparable id> -- exactly one must be emitted`
b. the rule holds and the governed id is absent: the message begins
   `SPD-1: <key> nature <nature> should be governed by s.40(b) (relationship <relationship>, entity rules gate open) but `
   and ends `a comparable-required finding <comparable id> was emitted instead` when the comparable id is
   present, else `<governed id> is missing`
c. the rule does not hold and the comparable id is absent: the message begins
   `SPD-1: <key> nature <nature> should ask for a comparable but `
   and ends `a s.40(b)-governs finding <governed id> was emitted instead` when the governed id is
   present, else `<comparable id> is missing`

Each message describes a result that has drifted from the rule; none fires on the test's own output,
so no golden shows one. The check does not look at findings for pairs it skips, so it does not notice a
finding raised for a zero amount or for a nature with no list.

## 5. Book and result checks in the dumps

Every golden carries the book-level and result-level reports of parity spec section 5. The
book-level reports are empty in every golden: each book's vouchers tie to its Trial Balance.

- **REND-0 fires once per finding**, in every golden that has findings: subject the finding id,
  detail `fact amount -> missing figure related_parties_cl23.amount_<nature>_<tag>`. The dump
  evaluates the result-level checks with this test's result alone (the crate's
  `invariants::result_invariants` does the same), and the fact names a figure of the other test.
  A port reproduces this exactly; it is not a defect in the port. `sp_none` has none.
- **EVID-1** fires in `sp_unknown_ledger`, once per citation of the ledger with no master in this
  test's evidence: the one finding cites `Ghost Rent` once (`unresolved ledger:Ghost Rent`, subject
  the test id). `Ghost Interest` is cited only by a zero amount, which raises no finding, so this
  dump does not report it; the related-party dump does.
- POP-4 never fires: the related-party evidence cites population vouchers only.

## 6. Ordering

Persons are processed in key order by code point and natures in vocabulary order. Neither shows in a
dump, because findings, facts, evidence and violations are all sorted (parity spec section 6):
findings by id, so all `comparable_required` findings come before all `s40b_governs` findings, each
group in tag order. Clause lists keep their authored order (parity spec section 3.1).

## 7. What the test never does

- It never computes, recomputes or adjusts an amount: every amount is the related-party figure's,
  and a missing figure stops the test with an error instead of being worked out from the book.
- It never reads the book, the Trial Balance or any ledger, and never forms the books population.
- It never decides who is a specified person or a partner: only the client's table names them, and
  partner status is the client's recorded relationship, never a ledger, group or party name, nor the
  entity type alone.
- It never reads the partnership deed or checks the s.40(b) limits; the governed finding asks the
  client to confirm the treatment.
- It never compares an amount with anything and offers no view on its level.

## 8. Not covered by any golden

- **A related-party result that lacks a figure the table implies.** The reference stops with an
  error (measured by calling the test with an empty related-party result: it raises rather than
  returning a result). The reference's pack cannot reach this, because both tests get the same
  table, and the edge harness cannot either. A port should refuse with a typed error; the exact form
  is the port's own.
- **Two person keys with the same tag.** For a non-empty table the reference refuses it, in both
  tests, with a typed error that names the tag and every key sharing it, sorted (measured with the
  invented keys `Person PXD` and `Person ACOW`, which share tag `773442d1`). The test returns no
  result and no golden is possible. The exact refusal form is the port's own.
- **A relationship that is not text.** For a non-empty table the reference refuses it, in both this
  test and the related-party test, with a typed error that names the person key (a person whose
  `relationship` key is absent has the text `""`, not a refusal). The keys are checked in sorted
  order, each for text before the tags are compared, so a table with both a non-text relationship
  and a tag collision gets the non-text refusal. No golden is possible; the exact refusal form is
  the port's own. (An empty table is not refused: both tests return `applicable` `no` first.)
- **A table value of the wrong shape** (a person value that is not an object, a nature value that is
  not a list: a non-empty string would count as a non-empty list here).
- **Rules with no `[entity]` table at all.** For a non-empty table the reference looks the rate up
  once, after the tag-collision and non-text refusals and before it reads any person's natures, so such rules fail on
  every such run, whatever the amounts; an empty table returns `applicable` `no` without looking
  anything up. The rate is looked up again for each person and nature with a nonzero amount, and by
  SPD-1. The crate's `Rules::s40b_interest_rate_bp` refuses on any call, so a port must make that
  call only where the reference does.
- Any SPD-1 message (section 4).

## 9. The books

All are invented: plain names, round figures, nothing read from Tally. Each book's `comment` says what
it reaches, and each names both tests.

| Book | Entity | Reaches |
| --- | --- | --- |
| `sp_none` | `firm` | No table: gate `"no"`, one figure, no finding; ledgers named like a partner's remuneration and interest move and do not matter. |
| `sp_core` | `individual` | Five natures of one non-partner, each a `comparable_required` finding with its own comparable phrase; a negative `other`; an ignored non-vocabulary nature that moves; a recorded partner whose salary and interest ask for a comparable because the rule is closed; a voucher label with an empty number and a voucher type other than its base type. |
| `sp_firm` | `firm` | The rule open: a partner's salary and interest governed, rent, purchases and other not; one ledger under two natures; a ledger shared by a partner and a non-partner (governed for one, comparable for the other); a negative governed amount (`working partner`); a non-partner's salary under an open rule. |
| `sp_labels` | `llp` | Fourteen relationship texts on a moving salary: six match (case, padding, tab and newline, no-break and ideographic spaces, U+001F and U+001C, the Kelvin sign), eight do not (inner double space, zero-width space, `sleeping partner`, `partners`, U+0130, absent, full-width, `partner's son`); the limits text shows the relationship as written, whitespace joined. |
| `sp_zero` | `firm` | Zero amounts with no finding (vouchers only outside the population, netting to zero across two vouchers, a zero-net transfer inside one set, a ledger that never moves, a person with nothing nonzero); 1 paise governed; minus 1 paise asking for a comparable. |
| `sp_shapes` | `firm` | Persons with no `ledgers_by_nature`, an empty object, only `payable_natures`, only non-vocabulary or capitalised natures and an empty list (nothing for any of them); one ledger listed twice (one finding). |
| `sp_empty` | `firm` | One key with an empty object: gate `"yes"`, no finding, although a ledger named for that person moves. |
| `sp_unknown_ledger` | `individual` | A set naming a ledger with no master beside one that moves (finding, EVID-1 once); a set naming only such a ledger (zero amount, no finding, no EVID-1 in this dump). |
| `sp_keys` | `individual` | Keys differing only in case, one name composed and decomposed, a Devanagari key: five tags, five findings. |
| `sp_company` | `company` | A recorded partner under a rules table with no `s40b_interest_rate_bp`: salary and interest ask for a comparable. |
| `sp_huf` | `huf` | The same under an entity type with no rules table at all. |

In `sp_labels` the persons are keyed `Person L01` to `Person L13` and `Person L15`, in the order
the comment lists their relationships (`Person L01` also has a governed interest nature).

## 10. Running the books

The crate's `tests/edge_books.rs` reads `entity_type` already but not `related_parties`, and runs
neither test yet. A port adds that: read `related_parties` (absent meaning `{}`); run the
related-party test on the book; run this test with the same table, the rules for the book's entity
type and that result; run the module check with the same three inputs and this test's result; and
compare each whole dump with its golden as the other edge books do (parity spec section 7). The side
of the harness that runs the reference, `parity/edge_golden.py`, is kept by the reference's
maintainers, together with its runners for both tests; a porter does not write one. The goldens are
regenerated only by the reference's maintainers.

## 11. Registering the test in the crate

A new registry entry needs, besides the books, a golden for the synthetic read and the two harness lines.

- **The synthetic golden.** `goldens/synthetic.specified_persons_40a2b.json` is the test on the crate's synthetic
  read (`tests/fixtures/synthetic-engagement.toml`). That engagement has no related-person table, so the gate is
  "no": the dump has one figure (`applicable` = "no") and no finding, and its result-level and module checks are
  empty. It goes to `tests/fixtures/golden/synthetic.specified_persons_40a2b.json`. The registry's
  `min_figures` for this test is 1. (The related-party test's own synthetic golden is in the first pack.)
- **The harness lines** (the reference's maintainers keep and regenerate the harness; these are the lines a port
  adds so its own harness can name the test). In `parity/python_golden.py`, a runner and its `RUNNERS` entry
  (the `RUNNERS` list is kept sorted by test id); the test takes the related-party result as input, so the
  runner runs that test first, as the reference's pack does:

      def _specified_persons_40a2b(c):
          from types import SimpleNamespace
          from tae.audit_tests import related_parties_cl23, specified_persons_40a2b
          from tae.config import related_parties_config
          rp = related_parties_config(c.cfg)
          cl23 = related_parties_cl23.run(c.eng, c.rules, rp)
          mod = SimpleNamespace(TEST_ID=specified_persons_40a2b.TEST_ID, check_invariants=lambda e, res:
                                specified_persons_40a2b.check_invariants(e, c.rules, rp, cl23, res))
          return mod, specified_persons_40a2b.run(c.eng, c.rules, rp, cl23)

      "specified_persons_40a2b": _specified_persons_40a2b,

  In `parity/edge_golden.py`, the `runners` table gets the same call with the book's `related_parties` key
  (absent meaning `{}`: `rp = related_parties_config({"related_parties": spec.get("related_parties", {})})`).
  A port only needs these to keep the two sides' test lists in step (the crate has tests that compare the Rust
  registry with the harness's list); the harness is run against the reference by the reference's maintainers.
