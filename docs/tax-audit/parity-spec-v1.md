# Parity spec v1 — canonical serialisation of a TestResult

This is the contract a Rust port of a books-based tax-audit test must follow byte-for-byte (after
JSON parsing — see "What is NOT part of the contract" at the end) so that a comparison tool can
tell a real regression from a harmless difference in how the two languages happened to build the
same JSON. `bridge-tax-audit`'s `canonical` module implements the producing side of this contract,
and its `compare` module implements the comparison in section 7.

Reference implementation: a Python engine's canonical serialiser, dump tool and diff tool. Each
rule below exists because an earlier design review found a specific way a naive
`{figure_id: value}` diff could pass silently on a real regression; the rules close those gaps one
at a time rather than trusting a single flat comparison.

## 0. Scope

v1 covers **TestResult parity for one test id, on one Book, run once** — starting with the
`cash_44ab` test this crate ports first. It does **not** cover:

- A canonical digest of the **Book** itself. Out of scope until a Rust adapter reads the same
  format the Python adapter reads through a shared, verified path; today the two languages build
  their `Book`s from separate code, so a Book-level digest would only prove a fixture was copied
  correctly, not that a real adapter agrees with the reference implementation's own parse.
- Rendering (workbook/document output) and any downstream reviewer scanning. Those consume a
  `TestResult` after it has already passed this parity check; they are a separate contract.

## 1. Top-level shape

One JSON object per (test id, Book) pair:

```
{
  "spec_version": "1.0.0",
  "test_id": str,
  "test_version": str,
  "rules_version": str,
  "population_note_sha256_16": str,
  "population_note_text": str,          // context only, see §4 — NOT a compared field
  "figures": [ Figure, ... ],           // sorted by id, §2
  "findings": [ Finding, ... ],         // sorted by id, §3
  "book_invariants_evaluated": [str],   // sorted invariant codes, §5
  "book_invariant_violations": [Violation, ...],
  "result_invariants_evaluated": [str],
  "result_invariant_violations": [Violation, ...],
  "module_invariants_evaluated": [str],
  "module_invariant_violations": [Violation, ...]
}
```

`spec_version` is this document's own version (semver-ish, currently `"1.0.0"`). A change to any
rule below that changes what a conforming dump looks like bumps it. Two dumps with different
`spec_version`s are reported as differing by this crate's `compare` (§7 step 4), so a dump written
to another version of this spec never passes as identical.

## 2. Figure

A named, typed value a test computes, with the evidence that supports it.

```
{
  "id": str,                    // "<test_id>.<name>", exactly the figure's own id
  "value": int | str | null,    // see the type rule below — never a float, never a bool
  "unit": "paise" | "bp" | "count" | "days" | "text",
  "definition_sha256_16": str,  // §4
  "definition_text": str,       // context only, NOT compared (§4)
  "evidence": [ EvidenceRef, ... ]  // sorted, §2.2
}
```

### 2.1 The value/unit type rule (never a float)

| `unit` | legal `value` | illegal |
|---|---|---|
| `paise`, `bp`, `count`, `days` | a JSON integer, or JSON `null` for an explicitly undefined ratio (e.g. a percentage with a zero denominator) | a JSON number with a fractional part or exponent (`5000.0`, `5e3`); `true`/`false` (Python's `bool` is an `int` subclass — reject it explicitly) |
| `text` | a JSON string | anything else |

**Why `null` is legal and distinct from `0`.** Some ratio figures (for example, a cash-share
percentage) are computed as `None` when the denominator is zero — no activity of that kind at all.
`null` there means "undefined", not "0%": a port that substitutes `0` for the undefined case is a
real behaviour change and must fail parity.

**Why this matters more than it looks.** A JSON parser that round-trips `5000` as `5000` and
`5000.0` as `5000.0` (which is what every mainstream JSON library does) turns "the Rust side used
a floating-point type for a paise amount" into a mechanically detectable type difference, with no
need to inspect the Rust source. The comparison tool's type validation enforces this independently
on **both** loaded documents, not only at dump time.

### 2.2 Evidence refs (sorted)

```
{ "kind": str, "id": str, "label": str }
```

Sort key: `(f"{kind}:{id}", label)` — the evidence reference's own key first, `label` as a
tie-break. Both `id` and `label` are NFC-normalised (§4) before sorting or comparing. `label`
**is** a compared field, not a hash: it is not free prose, it is built by a fixed format the
porting contract already specifies (for example, a voucher label built from voucher type, number
and ISO date), so a port is expected to reproduce it exactly.

## 3. Finding

```
{
  "id": str,
  "clauses": [str, ...],           // ORDERED exactly as authored — see §3.1, NOT sorted
  "confidence": "computed" | "indicative" | "needs_document" | "judgement_required",
  "facts": [ {"name": str, "figure_id": str}, ... ],   // sorted by name
  "evidence": [ EvidenceRef, ... ],  // sorted, §2.2
  "title_sha256_16": str,          // §4
  "title_text": str,               // context only, NOT compared
  "limits_sha256_16": str,         // §4, hash of the ORDERED tuple
  "limits_text": [str, ...],       // context only, NOT compared
  "ask_client_sha256_16": str,     // §4, hash of the ORDERED tuple
  "ask_client_text": [str, ...]    // context only, NOT compared
}
```

`test_id` is not repeated per finding in the canonical form (it is implied by the enclosing
document's own `test_id`).

### 3.1 Clause order is part of the contract

`clauses` is a tuple the test author chose an order for — a test can emit a shorter or longer
clause tuple depending on which conditions it found, and the order the author chose is meaningful.
A Rust port must emit clauses in the same order, not merely the same set: the comparison tool
compares the list as-is, unsorted. If a future test genuinely does not care about clause order,
that is a decision for this spec to record explicitly (bump `spec_version`), not something the
comparison tool should guess at silently.

## 4. Hashing prose fields (definition / title / limits / ask_client)

Why a hash and not the raw string as the compared field: these are long, and some (`limits`,
`ask_client`) are ordered tuples of strings rather than one string, so embedding them raw as the
*compared* value either loses order-sensitivity (if naively joined) or bloats every diff with
paragraphs of legal text a CI log has to print on every run. The **raw text still rides along**
(the `*_text` fields above) purely so a human reading a comparison failure can see *what* changed
— those fields are explicitly excluded from equality checking, only the hash is compared.

Algorithm, identical for every hashed field:

1. NFC-normalise (Unicode Normalization Form C) every string involved. Two adapters can
   legitimately hand back the same visible ledger/party name in different normalisation forms;
   hashing the un-normalised bytes would make two semantically identical results diverge for a
   reason that has nothing to do with the engine. `definition_text`, `title_text`,
   `limits_text[i]`, `ask_client_text[i]`, and evidence `label`/`id` are all NFC-normalised in the
   canonical JSON too, not only before hashing.
2. For a single string (`definition`, `title`): `sha256(nfc(text).encode("utf-8")).hexdigest()[:16]`.
3. For an **ordered tuple** of strings (`limits`, `ask_client`): join with `U+001F` (ASCII Unit
   Separator, chosen because it cannot appear in ordinary prose and makes `["ab", "c"]` and
   `["a", "bc"]` hash differently), no trailing separator, then hash as in step 2. An empty tuple
   hashes the empty string.

16 hex characters (64 bits) is a deliberate truncation: this is a change-detector, not a security
boundary, and a shorter hash keeps the JSON readable.

### 4.1 Unicode version

The reference implementation runs on Python 3.13, whose Unicode tables are version 15.1.0
(`unicodedata.unidata_version`), and every golden is produced under it (`uv run --python 3.13`). A
port reproduces that version wherever text is transformed, not its own toolchain's:

- **Case mapping.** Rust 1.96's `char` tables are Unicode 17.0.0. Measured over every code point,
  `to_uppercase` and `to_lowercase` each differ from Python's `str.upper()` / `str.lower()` at 55
  code points; Python maps every one of them to itself. `bridge-tax-audit`'s `support::py_upper` /
  `py_lower` keep those code points unchanged. `py_lower` also applies the final-sigma rule as
  CPython does, on Python's own Cased and Case_Ignorable sets (generated tables; those properties
  moved between the versions too). The lists are generated by `parity/case_exceptions.py` and
  `parity/text_semantics.py`, and a unit test fails when the toolchain's tables move.
- **Where it is used.** The narration keys and own-account terms of `cash_book_integrity`, the
  narration-term matching of `counter_cheques_40a3`, the name words and the GSTIN's PAN segment of
  `party_identity`, the applicability source labels and the entity type, and every GUID
  normalisation: ledger tags,
  binding keys and lookups, and the company pin and its comparisons. Ledger tags and binding also
  strip GUIDs with Python's whitespace (`support::py_strip`: Rust's whitespace plus U+001C..U+001F,
  measured to be the only difference), as the reference's `.strip().lower()` does; the company pin
  and the company-GUID comparisons only lower-case, as the reference's `.lower()` does.
- **Word characters, whitespace and literal regular expressions.** Python's `\w` is `isalnum()` or
  an underscore; `support::py_isalnum` is Rust's `is_alphanumeric` minus 295 generated ranges (the
  difference is one-way, measured over every code point). `support::py_isspace` / `py_strip` /
  `py_split` use Python's whitespace. `support::py_re_search` searches the reference's literal
  patterns (ASCII letters, `\b`, `\s?`, `|`) with `re.I` as `sre` applies it to a literal letter:
  its two cases plus U+0130/U+0131 for I, U+212A for K and U+017F for S, and never an expanding
  form such as ß. `cash_payments_40a3` and `depreciation` pass the reference's pattern text to it
  verbatim. `tests/fixtures/text-probes.json`, Python's own results on the acceptance set (Latin,
  Latin-1/Extended, Devanagari and the other Indic scripts, punctuation, NBSP and control
  whitespace), is replayed in CI.
- **`repr()` of a str.** `support::py_repr_str` builds the reference's `f"{x!r}"` as CPython's
  `unicode_repr` does: `'` quotes unless the text holds a `'` and no `"`; the quote and backslash
  escaped; `\t`, `\n`, `\r`; `\xhh` for the other controls and U+007F; other ASCII kept; non-ASCII
  kept when printable, else `\xhh`, `\uhhhh` or `\Uhhhhhhhh`. `support::py_isprintable` is Python
  3.13's `str.isprintable()` on Unicode 15.1.0 (Cc, Cf, Cs, Co, Cn, Zl, Zp and Zs other than the
  space are not printable), from 713 ranges generated by `parity/text_semantics.py` over every code
  point. Measured in Python with Python's own `isprintable`, the rule reproduced `repr()` on every
  code point, surrogates included, in five strings each (alone, and beside quotes and a backslash):
  5,570,560 strings. An independent review compared `py_repr_str` itself with `repr()` on every
  Rust `char` (a `char` is never a surrogate) in the same five strings: 5,560,320 strings, no
  difference. The probe file replays it on the acceptance set and on every code point either side
  of each non-printable range.
- **NFC.** The `unicode-normalization` crate (0.1.25, its own tables at Unicode 17.0.0, pinned by a
  unit test) and Python 3.13's `unicodedata.normalize("NFC", ...)` agree on every single code
  point (measured). On multi-code-point sequences they do not always agree: an independent review
  found 20 of 13,253 decomposed sequences that recompose differently, all in scripts added after
  15.1 (Tulu-Tigalari, for example). That is a documented limit.

This does not change what a conforming dump looks like, so `spec_version` is unchanged.

## 5. Invariant reports: name what was EVALUATED, not only what fired

This is the one rule in this spec that exists purely to prevent a false pass, so it gets its own
section: **"invariant list" has to mean invariants evaluated, each with its outcome, not
"violations found." Otherwise an invariant never implemented in Rust reads as "no violations."**

Every invariant report is a pair: `<name>_invariants_evaluated` (a sorted list of invariant
**codes**, e.g. `["ID-1", "MAP-0", "MAP-1", "POP-0", "POP-1", "POP-2", "POP-3", "POP-5"]`) and
`<name>_invariant_violations` (a sorted list of `{"invariant": code, "subject": str, "detail": str}`,
NFC-normalised, empty when nothing fired). The comparison tool compares **both** lists, as sets for
`*_evaluated` and as sorted lists for `*_violations`. A code present in `book_invariants_evaluated`
on one side and absent on the other is a parity failure **even if neither side reports any
violation for it** — that absence is exactly the silent gap this section closes.

Three reports, one dump:

| Report | Source | Codes |
|---|---|---|
| `book_*` | the reference engine's book-level invariant functions, run on the Book | `ID-1`, `POP-0`, `POP-1`, `POP-2`, `POP-3`, `POP-5`, `MAP-0`, `MAP-1` |
| `result_*` | result-level checks: findings cite real figures, evidence resolves, no non-accounting evidence leaks in | `REND-0`, `EVID-1`, `POP-4` |
| `module_*` | the test module's own invariant check, if it defines one | `<test_id>.check_invariants`, or nothing at all if the module has none |

In the reference, the invariant **code** for a book-level or result-level function is read from
its own docstring's leading `CODE-n:` token — not hardcoded a second time in the serialiser, so a
new invariant that follows the convention is picked up automatically. This crate has no docstring
to read and writes each code by hand: where a violation is raised, and again in the list of codes
evaluated (`src/invariants.rs:178` for the book, `:244` for the result). What keeps the crate
in step is this section's comparison of the evaluated code sets: an invariant added to the
reference fails parity until the crate lists it.

**Known gap, inherited, not hidden:** as of this spec's writing, not every ported test has a
module-level invariant check yet. Until one is added for a given test, `module_invariants_evaluated`
**must be the empty list on both sides** for that test — that is the correct, honest state, not a
placeholder. The day a module-level check is added to either language's port of a test before the
other, this field's set-equality check turns that asymmetry into an immediate, named parity
failure — which is the point.

A polarity observation the reference engine tracks separately from its book invariants is excluded
from every report here: it is documented as an observation, not a gate, so it carries no pass/fail
meaning in this contract.

## 6. Sort order, generally

Every sorted list in this document (figures by id, findings by id, evidence by key, facts by name,
invariant violations by `(invariant, subject, detail)`) sorts by plain lexicographic order on the
UTF-8 byte sequence of the sort key. This is deliberately the same thing as "sort by Unicode scalar
value" for any valid UTF-8 string (UTF-8's byte encoding is order-preserving with respect to code
points), so `str::cmp` in Rust and Python's default string `<` agree without either side needing a
locale-aware collation library. **Do not use a locale-sensitive or case-insensitive sort anywhere
in this pipeline.**

The comparison tool does not trust that a producer already sorted correctly — it re-sorts every
list before comparing. A dump whose lists are in a different but internally consistent order must
still pass; a dump is free to write its JSON in whatever order is convenient to produce, as long as
the content is complete and correct.

## 7. What the comparison checks, in order

1. **Type validation on both sides independently** (§2.1), before anything else. A violation here
   stops the comparison outright — a badly-typed document cannot be meaningfully diffed further.
2. **Refuse empty-vs-empty.** If both sides report zero figures, that is itself a failure
   regardless of anything else being equal. An empty result must never look like success.
3. **Structure of each side**, a check this crate's `compare` adds and the reference's own
   `tae/parity/compare.py` does not make. Each side is checked for unknown keys (in the dump, a
   figure, a finding or an evidence ref) and for a figure or finding id that appears more than
   once (`src/compare.rs:165-197`, called at `:346-347`); the reference's tool ignores unknown keys
   and keeps one of two entries that share an id. These findings are collected with the rest and
   do not stop the comparison.
4. **`spec_version`, `test_id`, `test_version` and `rules_version` equality.** The reference
   implementation's own `tae/parity/compare.py` checks `test_id` and `rules_version` only; this
   crate's `compare` also checks the two versions, so a port that bumps a test's version (or
   writes another spec version) without the reference doing the same is reported, not passed.
5. **Minimum figure count**, per test id. In this crate's `compare` the floor is held per test in
   the registry (`min_figures` in `src/registry.rs`, explained in `src/compare.rs:15-67`): the
   fewest figures a run of that test that did its work publishes, set from the code, low enough
   that a quiet real book passes and high enough that an empty dump does not. The crate does not
   anchor it to the synthetic fixture's own figure count: for most tests the floor is below that
   count. The reference's own comparison tool anchors a floor to its fixture for two tests only
   (`cash_44ab` 7, `cash_payments_40a3` 28) and holds a floor of 1 for every other test. A real
   client's count is measured separately, in the local run of section 9 below that compares the two
   engines; it is never committed as a CI gate value, consistent with the rule that no client data
   enters this repository or its CI.
6. **Figure key-set equality** (the full symmetric difference is reported, not just the
   intersection — comparing only the intersection would let a comparison pass on nothing shared).
7. **Per common figure:** unit, value, `definition_sha256_16`, evidence (§2.2).
8. **Finding key-set equality**, same symmetric-difference treatment.
9. **Per common finding:** clauses (ordered), confidence, facts, evidence, and the three prose
   hashes (§4).
10. **`population_note_sha256_16` equality.**
11. **`*_invariants_evaluated` set equality and `*_invariant_violations` list equality** (§5), for
    all three reports.

In both comparison tools, any violation from steps 3–11 (in the reference's tool, from those of
the steps it makes) is collected and reported together (not fail-fast) so a single run shows every
difference, not just the first one found; the process still exits non-zero if the list is
non-empty. Steps 1–2 are the only ones that stop early, because a badly-typed or empty document
cannot be diffed meaningfully at all.

## 8. What is NOT part of the contract

- **File formatting.** JSON key order within an object, indentation, and trailing whitespace in
  the `.json` file itself are not compared — the comparison tool parses both files and compares
  the resulting data structures, never the raw bytes.
- **`definition_text`, `title_text`, `limits_text`, `ask_client_text`.** Present for human
  readability only; only their hashes are compared (§4).
- **`population_note_text`.** Same treatment as the prose fields above — only
  `population_note_sha256_16` is compared.
- **A canonical Book digest** (§0) — deferred.

## 9. Real-world validation

Beyond the committed synthetic golden fixture that CI compares on every run, this crate's parity
with the reference implementation has been checked locally against several real client books, with
zero differences reported by the comparison in section 7. That run, and the client data it reads,
are never committed: results are kept privately, consistent with the rule that no client name,
figure or byte enters this repository or its CI.

## 10. Extending this spec to a new test

1. Add the new test module to the reference implementation's dump tool, mirroring how its own pack
   builder already calls that module's `run()`.
2. Add a fixture-builder branch (or a new fixture module) covering that test's own interesting
   rows and boundaries, invented names only.
3. Generate and commit a golden file for the new test. In this crate, give the test's registry
   entry its `min_figures` (§7 step 5): the fewest figures a run of the test that did its work
   publishes, set from the code; the golden's own figure count is not the anchor.
4. Add the new golden to the reference implementation's own parity-golden test.
5. If the module has (or gains) a module-level invariant check, nothing else changes here — §5's
   generic handling already covers it; only its own code appearing on both sides is new
   information.
6. Type a test's own configuration lazily. Loading a client config (`Engagement::from_toml`) and
   binding it (`Engagement::bind`) run for EVERY test, so a malformed value in one test's own
   table must not refuse the others. That is also how the reference's parity harness behaves
   (`parity/edge_golden.py` and `parity/python_golden.py`): it calls a test's table reader only
   inside that test's own runner, so a malformed table fails that test alone. The reference's full
   run is different: it always runs every test and calls the readers of several tables (the TDS,
   depreciation, partner and 26AS tables among them) before the first test, so there a malformed
   table stops the run.
   - Binding checks only the shape of the name locations the reference's binding reads (a list of
     names, a table keyed by names), refusing `BIND-ID-MALFORMED` there as the reference's
     `names_at` does.
   - Every other value is kept as written and typed in the test's own `*_on` runner, with the
     reference's own coercion (for example Python's `int()`).
   - A test's table that is present but is not a table is skipped by binding (the reference's
     `_expand` skips it) and refused when that test runs.
   `CreditorAgeingConfig` and `StatutoryDuesConfig` in `src/lib.rs` are the pattern. A test in the
   crate's registry must have a unit test showing that a malformed value in its table fails that
   test and leaves the others running.

   Known exceptions: this crate's `Engagement::from_toml` still refuses these when the config
   loads, so a malformed value there refuses every test. They stay that way until they are brought
   in line, which is a change inside the crate.
   - `[tds]` and `[tds_payees]` (`src/lib.rs:822-890`): `tds_payees` (batch C1) was written before
     this convention. With them go two values the same test reads elsewhere, `[client].state` and
     `[deductor]` with its `activity` (`src/lib.rs:957-958`, `src/tds_payees.rs:383-417`).
   - `[depreciation]`, when a required key is missing or an entry is wrongly typed
     (`src/lib.rs:670-756`).
   - `[tds_tcs_26as]`, when it is not a table or a key has the wrong type (`src/lib.rs:770-821`).
   - `[partners]`, when it or a partner is not a table, or an `interest_ledger` is not text
     (`src/lib.rs:891-915`).
   - `[presumptive_history]`, when it is not a table (`src/lib.rs:967-974`), although the bullet
     above says such a table is refused only when its test runs.

## 11. Ledger tags: figure/finding ids keyed by GUID, not by name

A per-ledger figure or finding id is not built from the ledger's display NAME — a
read-shape/serialisation artifact can re-case a name between two reads of the same unchanged
master (`ROUND OFF` -> `Round Off`) with no edit having happened, and a name-hash id would churn
for no reason connected to the books. Both engines derive this id from the ledger's Tally GUID
instead (the reference engine's ledger-tag module; this crate's `src/ledger_ids.rs`):

1. **Normalise.** Trim surrounding whitespace, then lowercase, the same way on both sides (§4.1):
   the reference trims with Python's `str.strip()` and lower-cases with `str.lower()` (Python's
   whitespace and full Unicode lower-casing, on its Unicode 15.1 tables), and this crate
   reproduces both (`src/ledger_ids.rs:38-40`). Two engines, or two Tally exports of the same GUID
   in different casing, must agree on the same tag; the binding logic elsewhere in this stack
   already treats GUIDs as case-insensitive, so the tag has to match that, not hash the raw,
   differently-cased bytes.
2. **Hash.** `sha1(normalised_guid.encode("utf-8")).hexdigest()[:8]`.
3. **Fallback to a name hash** — `sha1(name.encode("utf-8")).hexdigest()[:8]`, no GUID involved —
   in two cases:
   - `name` does not name a real ledger in the Book at all (a client-config alias, a synthetic
     sentinel bucket, an orphaned Trial Balance row): never read fresh from Tally on every
     capture, so it carries none of the rename-churn risk above, and hashing the string itself is
     already stable.
   - `name` **does** name a real Book ledger, but that ledger's own GUID is blank. The
     `tally-read-v1` format does not require ledger GUIDs, so refusing here would turn a
     previously-working run into a hard failure on an otherwise-valid read. The id for such a
     ledger is stable only **within one read** — two reads of the same company can still rename
     it, and the id will move, because there is no Tally identity left to anchor it to.
4. **Duplicate GUIDs refuse.** Two different ledgers in the same Book that normalise (step 1) to
   the same non-blank GUID is a corrupt read, not a legitimate case — Tally does not hand out one
   GUID to two masters. Handing both ledgers the same figure id would silently merge their rows
   under one id, which is worse than a refusal. Both engines check this once, over every ledger in
   the Book, at Book construction/load: the reference engine's `Book.__post_init__`, and this
   crate's `load_book` (`src/book.rs`) — both calling the same `check_no_duplicate_ledger_guids`.

This fallback (step 3) applies to LEDGERS only. A blank GROUP GUID still refuses (no consumer
relying on a successful group tag over a GUID-less group has been found), and this spec does not
extend the fallback to groups without that same measurement.

**Evidence refs name what they cite.** An evidence ref's id is the identifier of the thing cited, not
a tag. The crate emits ten kinds: `voucher` and `excluded_voucher` (the voucher's GUID),
`document_row` (`{document id}#{row}`, a locator the extractor chose, not a name), `row` (a fixed
locator for a computed row), `rule` (a fixed key), `config` (the configuration entry cited: a fixed
key such as `presumptive_history`, or, for a deductor alias, the deductor's TAN), `pan` (the entity's
PAN, as recorded, set in the engagement's party table or derived from a GSTIN; bridge#1145 item 9),
`ledger` and `stock_item`, whose id is the NAME as the books write it (compared NFC-normalised,
§2.2), and `payee_name` (`narration_payees`: a payee's names as the bank printed them in the
narration, upper-cased, joined by ` / `), whose id is a name too. This section's tag rule does not
apply to any of them. The reference engine's renderer declares the kinds whose id is a name in
`NAME_KINDS` (`ledger`, `stock_item` and `payee_name`) and every other kind in `NOT_NAME_KINDS`,
with a selftest that every kind a test constructs is in exactly one list.

A consumer that redacts or masks must therefore treat the id of a `ledger`, `payee_name` or
`stock_item` ref as a name, and the id of a `pan` ref as a tax identifier; a `config` id may be a tax
identifier (a TAN) too, so treat it as one unless the key is one the consumer knows is fixed. For any
new kind it must settle, before using it, whether its id is a name or a tax identifier.

This paragraph is about ids only. A ref's label is a compared field (§2.2) and may carry names or
other text of its own: a `row` ref's label quotes the names its vouchers print, a `document_row`
label may be the first characters of a statement narration or a 26AS row's TAN and section, and a
`ledger` or `stock_item` label may repeat the name. A consumer that masks names or tax identifiers
masks labels by their own rule.

Any future change to this algorithm must be made in both engines in the same change, or parity
between them breaks silently. §7 does not compare ids directly — a stable id is a downstream
consumer's contract, not a `TestResult` field — but a hashed prose field or a figure/finding's
identity keyed off this id will still diverge if the two engines disagree on how it is computed.
