# Spec pack: `knock_off_candidates` (settlements through another party; may bear on Form 3CD clauses 31 and 21(d))

The goldens in this pack were produced by the reference engine at commit `742f67fc` and are the
contract; this note explains them and cites [`docs/tax-audit/parity-spec-v1.md`](../../parity-spec-v1.md)
(sections 1, 2.2, 3.1, 4, 4.1, 5, 6, 7 and 11, as relevant); where the note and a golden differ, the golden wins and the
reference's maintainers should be told on the pull request or issue.

In this note "README section N" is a section of this note and "parity spec section N" a section of
that document.

This pack is for porting the test into `src-tauri/crates/bridge-tax-audit`. It holds:

- `books/ko_*.json`: invented edge books, in the shape of the crate's
  `tests/fixtures/edge-books/*.json`. The engagement's extra party groups ride in the
  `party_identity` table those books already use (README section 2.2).
- `goldens/edge.<book>.knock_off_candidates.json`: the reference's canonical dump of this test on
  each book (parity spec section 1).
- `HASHES.md`: bytes and SHA-256 of every book and golden, what the books establish, and how the
  goldens were made.

## 1. What the test does

The test points the auditor at vouchers where one party's balance may have been cleared through a
different party. It has two lists:

- **T1**, a journal settlement: a voucher of any type with nonzero lines on two or more different
  party ledgers and no nonzero line on a bank or cash ledger.
- **T2**, a narration naming another party: a voucher with a nonzero bank or cash line whose
  narration spells out, word for word, the full name of a party ledger that has no nonzero line on
  the voucher (a zero line posts nothing). When the only place a party's name appears is next to a further name-like word, the
  voucher goes to a third list, **T2 embedded**, instead.

Every voucher that fits is listed, whatever its size or apparent purpose. The test reaches no
conclusion about any of them, and its totals are sums of candidates, not amounts to report. Its
findings carry an empty clause list; the clauses appear only as words in their titles (README section
4.5).

## 2. Inputs

### 2.1 The book

Read from the engagement's book, as the crate's `src/book.rs` holds it:

- **Ledger masters**: each ledger's name, its group chain (parent first, primary group last) and its
  GUID. The chain decides whether a ledger is a party or a bank or cash ledger (README section 3.1);
  the GUID only gives the tag in two figure ids (README section 4.7). Every ledger master whose name
  yields a token also counts when the test looks for names two masters share (README section 3.3).
- **Vouchers**: GUID, date, voucher type name, base voucher type, number, narration, status and
  lines; each line is a ledger name and an integer amount in paise, debit positive, credit negative.
  The base type is read once: to tell a Contra when a voucher's money is counted (README section 4.1;
  `knock_off_candidates.py`, `money_of`: `v.base_type == "Contra"`, line 218 at reference commit
  `742f67fc`). Nothing else in the test reads it, and the evidence label uses the type name (README section 4.2).
- **The books population**: vouchers whose status is regular (`Book::population`). Optional,
  cancelled and post-dated vouchers are never read (`ko_status`). The test always forms the
  population, so a book with any voucher of unknown status refuses (README section 10).

Not read: the Trial Balance, the group masters themselves (only each ledger's chain), the party
field, the reference, inventory, and the edge book's `cash` and `bank` lists.
Bank and cash ledgers are found by group, never by those lists (`ko_groups`: `OD Account` is a bank
ledger though `bank` names only `Bank A`).

### 2.2 The extra party groups

A list of group names the engagement treats as party groups, added to the four fixed ones (README section 2.5).
In the reference's pack this is the client configuration's
`[party_identity].party_groups`, the same setting the party-identity step reads, passed to the test as
a tuple in configuration order; absent means none. In the edge books it is the same place: the
top-level `party_identity` table's `party_groups` list, which the crate's `tests/edge_books.rs`
already reads for another test; an absent table or key means `[]`. Order and repeats do not matter: a
ledger is a party when any name in either list is in its chain (`ko_groups` repeats
`Sundry Debtors`).

### 2.3 Binding of the group names (before the test runs)

The test does not check the names in `party_groups` against the books. In the reference's real
pipeline a separate step binds every name at `party_identity.party_groups` to the book's group
masters before any test runs: a name that matches no group master is refused with
`BIND-GROUP-UNKNOWN`, naming that location; a value that is not a list of text is refused with
`BIND-ID-MALFORMED`; and a group renamed since the configuration was written is rewritten to its
current name through `[group_ids]`. The match is exact, never case-folded. In the crate that step is
`src/binding.rs`, which already binds this location as `party_identity.party_groups`; the port reads
the bound list.

What the test itself does with a name that reached it unbound is shown by `ko_groups`, which the real
pipeline never reaches: `No Such Group` is in no chain, so it makes no ledger a party and changes
nothing. The test takes no ledger names from the configuration at all, so there is nothing else to
bind.

### 2.4 Rules

Only the rules version is read; it is copied into `rules_version` (`2026-09-17.1` in every golden).
`test_version` is `"1"`. The entity type is not read.

### 2.5 Fixed vocabularies (not client data)

- **Party groups**, always: `Sundry Debtors`, `Sundry Creditors`, `Loans (Liability)`,
  `Loans & Advances (Asset)`.
- **Bank and cash groups**: `Bank Accounts`, `Bank OD A/c`, `Cash-in-Hand`.
- **Separators** that cut a narration into parts: any run of the characters `-` `/` `,` `;` `|`
  `:` `(` `)` and backslash. A full stop, `&`, an apostrophe, `#` and `_` are not separators
  (`ko_vocab` pins each of these fourteen characters).
- **Words that are never name-like** (README section 3.4): `to`, `from`, `by`, `for`, `paid`, `pay`,
  `payment`, `pymt`, `neft`, `rtgs`, `imps`, `upi`, `ref`, `cr`, `dr`, `towards`, `being`, `via`,
  `and`, `of`, `the`, `in`, `on`, `at`, `with`, `mr`, `mrs`, `ms`, `inb`, `transfer`, `trf`, `chq`,
  `cheque`, `cash`, `deposit`, `withdrawal`, `settled`, `adjustment`, `bill`, `gr`, `inv`,
  `invoice`, `against`, `pvt`, `ltd`, `limited`, `llp`, `private`, `rs`, `inr`, `amt`, `amount`,
  `no`, `a`, `an` (55 words, all lower case; `ko_vocab` writes each one next to a party name).
- **The shown narration** is at most 80 characters (README section 4.4).

## 3. How vouchers are chosen

### 3.1 Party ledgers and bank or cash ledgers

A ledger master is a **party** when any party group (README section 2.5, plus `party_groups`)
appears anywhere in its chain, and a **bank or cash ledger** when any bank or cash group does. Chain
membership is an exact, case-sensitive match on the group name, so a ledger under a sub-group counts
(`ko_t1`: `Unsecured Loans` under `Loans (Liability)`, `Debtors North` under `Sundry Debtors`).

The two are not exclusive. In Tally's default groups `Bank OD A/c` sits under `Loans (Liability)`, so
an overdraft ledger is both (`ko_groups`' `OD Account`), and naming `Cash-in-Hand` in `party_groups`
makes a cash ledger both (`ko_groups`' `Petty Cash`). Such a ledger is a bank or cash ledger wherever
that is asked, and a party wherever that is asked.

A line on a ledger with no master is neither.

### 3.2 T1, the journal settlement

For each population voucher, take the distinct ledgers it has a **nonzero** line on. It is a T1 row
when two or more of them are parties and none is a bank or cash ledger. The voucher type does not
matter (`ko_t1`: a credit note and a sales voucher are listed).

- A zero-amount line counts for neither side: a zero bank line does not block T1 (`ko_t1`'s t05), and
  a party on a zero line only is not a second party (`ko_quiet`'s q06).
- Two lines on one party ledger are one party (`ko_quiet`'s q03).
- A party whose lines net to zero still counts, being on nonzero lines (`ko_t1`'s t08).
- A single bank or cash line of 1 paise blocks T1 (`ko_text`'s x10); a party line of 1 paise counts
  (`ko_text`'s x09).

### 3.3 T2, reading a narration for party names

A population voucher is read for names when it has a nonzero bank or cash line and its narration is
not the empty string. A narration of spaces only is read and yields no token (`ko_quiet`'s q08). T1
and T2 never overlap: one needs a bank or cash line and the other forbids it.

**Tokens and parts.** The narration is cut into parts at the separators (README section 2.5), on the
text as written. Each part is then decomposed (Unicode NFD), then case-folded with Python 3.13's
`str.casefold()` (the crate's `support::py_casefold`; parity spec section 4.1 on the Unicode
version), and only then are its tokens taken: the maximal runs of letters, numbers and combining
marks of any alphabet, that is of characters whose general category is a letter (`L*`), a number
(`N*`) or a mark (`M*`). Tokens are numbered across the whole narration, and each remembers its
part. While a token is read:

- a mark that follows a letter `a` to `z` is dropped, so an accented Latin letter reads as its plain
  letter: `Café`, composed or decomposed, gives `cafe`;
- a mark that would start a token is dropped, and so are U+200C, U+200D and U+00AD (the zero-width
  non-joiner and joiner and the soft hyphen), which neither end a token nor stay in it;
- every other mark stays in its token: a Devanagari vowel sign or virama (`देव ट्रेडर्स` is two
  tokens, each with its signs), an accent on a Greek letter, a mark after a digit.

Any other character ends a token: a space, punctuation, a symbol such as the trade mark sign
U+2122, the zero-width space U+200B. The sharp s folds to `ss` and the `ﬁ` ligature to `fi`; the
Kelvin sign decomposes to `K`, folded to `k`; the dotted capital I decomposes to `I` and a
combining dot, folded to `i` and the dot, which is dropped (`ko_vocab`'s v44 to v48, README
section 12). A part is cut on the text as written, so U+037E, which decomposes to `;`, ends a
token but does not cut a part (README section 11).

**The names that can be found.** Each party ledger, taken in code-point order of its name, gives a
key: the tokens of its whole name, read the same way but not cut into parts. A party is left out
when its key is empty, which a name made only of punctuation or symbols gives (no book has one). A
key is **shared** when any other ledger master, of any group, has the same tokens in the same order,
whatever its case, punctuation or spacing, or an accent on a letter `a` to `z` (`Café Corner` and
`Cafe Corner` would share one); masters are counted over every group, and a key with more than one
master is bound to no one: the narration cannot say which master it means, so it is listed for none. That other master may
be a party (`ko_t2`: `Omega Mart` and `OMEGA MART`, so b07 names neither; `Kappa Traders` and
`Kappa-Traders`, so b06 is not listed) or not (`ko_t2`: the party `Sigma Stores` and the expense
`sigma stores`). A shared key stays in the index as a name the narration holds, so a shorter name
inside it is still dropped as nested and a neighbouring name still sits beside it; `ko_shared` pins that.

**Finding them.** At every token position, every key that matches the tokens starting there, in
full, is a found name with its span of tokens. Matching runs across parts, so a name written across
a separator is found (`ko_nested`'s n18). A found span lying inside another found span, and not equal
to it, is dropped (`ko_nested`'s n01: `Rho` inside `Rho Transport`). Spans that merely overlap are
all kept (`ko_nested`'s n03 and n04). The same name found twice is one name (`ko_t2`'s b14).

**Skipping the voucher's own ledgers.** A kept span whose ledger has a nonzero line on the voucher
is not reported (`ko_nested`'s n02 and n19). A zero-amount line posts nothing, so a party on the
voucher only through one is reported like any other (`ko_t2`'s b12 is listed for `Gamma Supplies`).
A span not reported still counts as a kept span in README section 3.4.

Matching examples from the books: `CAFÉ CORNER` and `Cafe Corner` both find `Café Corner` (`ko_t2`'s
b09 and b10), as the accent on `e` is dropped from the name and the narration alike; `ZOE CRAFTS`
finds a ledger whose name is written with a combining diaeresis (`ko_tags`' k06); `Unit 7 Supplies`
is found with its digit (`ko_t2`'s b08); the Devanagari narration of `ko_t2`'s b11 finds the
Devanagari party `देव ट्रेडर्स`; `AlphaTraders` and `Alpha` alone do not find `Alpha Traders`
(`ko_quiet`'s q10). An accent on any other letter is kept, so a Greek word written with an accent and
the same word written without it are different tokens, and neither finds the other.

### 3.4 Plain or inside a longer name

A reported span is **inside** when the token just before it or the token just after it meets all four
of these:

1. it exists;
2. it is not a token of another kept span disjoint from this one. Precisely: a token fails this
   condition only when it belongs to at least one kept span and to no kept span that shares a token
   with this one (this span included). A token that belongs both to a disjoint span and to an
   overlapping span passes (`ko_nested`'s n04: `MU` before `NU XI` is in `Tau Mu`, disjoint from
   `Nu Xi`, and in `Mu Nu`, which overlaps it, so `Nu Xi` is inside);
3. it is in the same part as the span's own first token (for the token before) or the span's own
   last token (for the token after; `ko_vocab`'s v43 writes `LAMBDA-TRADERS ZED`, where the two
   differ); and
4. it is name-like: only the letters `a` to `z`, read as in README section 3.3 (so an accent on them
   is already dropped, and `ZOË` beside a name is name-like), at least two of them, and not in the
   word list of README section 2.5. A token holding a digit or any other character is never
   name-like, so a Devanagari or full-width word beside a name leaves it plain: the word list is in
   English only.

Otherwise it is **plain**. A name plain anywhere in the narration is plain for the voucher, and its
inside occurrences are forgotten (`ko_nested`'s n15). The voucher is a T2 row when it has at least one
plain name, and a T2 embedded row when it has at least one inside name that is not also plain; it can
be both (`ko_nested`'s n16).

`ko_nested` shows the cases: ZED and the name on either side of a slash (n07, plain); ZED before and
after it in one part (n08 and n09, inside); `LTD`, `X`, `42` and `B2` beside it (n10 to n13, plain);
`XY` beside it (n14, inside); two parties written back to back (n05 and n06, plain, the booked
party's span included); overlapping names (n03) and a chain of three (n04), all inside. `ko_vocab`
repeats the test for every word of the list (plain) and for every separator (plain) and
non-separator (inside) character.

### 3.5 Traps for a port

- **Take token characters by general category, not by Rust's `char::is_alphabetic`.** That method is
  the Unicode Alphabetic property, which counts some marks as letters and not others: the
  Devanagari vowel signs U+093E and U+0947 are Alphabetic, the virama U+094D and the combining acute
  U+0301 are not. Built on it, a Devanagari word breaks at its virama, and a vowel sign that starts a
  token or follows a letter `a` to `z` is kept as a letter where README section 3.3 drops it. The
  rule is `L*`, `N*` or `M*`; Rust's `char::is_numeric` is the `N*` categories, of Rust's own
  Unicode version.
- **Do not reuse Python's `\w`** (the crate's `support::py_is_word` and `support::py_isalnum`). It
  rejects every mark, so `देव` reads as two one-letter tokens, and the dotted capital I as `i` and
  `ota`. One rule built on it changes `ko_vocab` (HASHES.md).
- **Decompose, then case-fold, with Python 3.13's Unicode 15.1 tables.** Without the decomposition a
  precomposed letter keeps its accent (`ko_t2` changes). Measured over every code point, alone, after
  `a` and before U+0301: leaving the decomposition out reads 13,080 code points differently,
  decomposing with NFKD instead of NFD reads 3,575 differently, and folding before decomposing reads
  64 differently (the Greek iota subscript before another mark). Only the first is pinned by a
  golden. The crate's `unicode-normalization` dependency, used today for NFC in `canonical.rs`,
  carries Unicode 17.0 tables (`support.rs` asserts that version), not 15.1.
- **Cut the shown part from the narration as written** (README section 4.2), at the parts' own
  boundaries. The reference's boundaries are character positions in the original text, so a port
  must never apply them as byte offsets, nor take them from the decomposed or case-folded text,
  whose length differs.

## 4. Outputs

Every figure and finding id below is exactly as the goldens carry it. `<tag>` is the named ledger's
stable tag (README section 4.7).

### 4.1 Figures

Seven figures are always present, with these values when nothing is listed (`ko_quiet`, `ko_empty`);
two more come for each name found plainly.

| Figure id (`knock_off_candidates.` + ) | Unit | Value | Evidence |
| --- | --- | --- | --- |
| `t1_row_count` | count | T1 rows | one T1 ref per row |
| `t1_candidate_total_paise` | paise | over T1 rows, the sum of every positive line on a party ledger | none |
| `t2_row_count` | count | T2 rows | one T2 ref per row |
| `t2_candidate_total_paise` | paise | over T2 rows, the sum of the absolute values of every bank or cash line | none |
| `t2_named_pair_count` | count | over the names, the number of T2 rows naming each, summed | none |
| `t2_embedded_row_count` | count | T2 embedded rows | one embedded ref per row |
| `t2_embedded_candidate_total_paise` | paise | over T2 embedded rows, as for T2 | none |
| `t2_named_count_<tag>` | count | T2 rows naming this ledger plainly | one per-name ref per row |
| `t2_named_total_paise_<tag>` | paise | over those rows, as for T2 | none |

Consequences the goldens show: a T1 voucher adds every positive party line, even for a party whose
lines net to zero (`ko_t1`'s t08 adds the ₹2,000 debit line of `Alpha Traders`, whose two lines net to
`₹0`); a voucher whose every non-nil line is on a bank or cash ledger, or whose base type is Contra,
moves one amount between money ledgers and counts the larger side of its money lines once (`ko_t2`'s b04 adds its
amount once); every other voucher sums the absolute value of each of its bank and cash lines; a voucher that is both a T2 and a T2 embedded row is in both totals (`ko_nested`'s n16); a T2
row naming two parties counts once in `t2_row_count` and once in each name's figures, so
`t2_named_pair_count` exceeds `t2_row_count` (`ko_t2`: 14 and 13).

Each definition is a fixed sentence; the two per-name definitions carry the tag. The definitions of
`t2_row_count` and `t2_named_count_<tag>` say the named party has no amount on the voucher, and
those of the three T2 totals each end with one sentence saying that a Contra, or a voucher whose
every line with an amount is on a bank or cash ledger, counts the larger side of its money lines
once. Take them from the goldens: they are compared by hash (parity spec section
4).

### 4.2 Evidence labels

Every voucher ref has kind `voucher` and the voucher's GUID as id. Its label starts with the voucher
label `<voucher type> <number> on <ISO date>` (the crate's `support::voucher_label`). The voucher type
is the voucher's own type name, not its base type (`ko_t1`'s t06, `Sales GST`; `ko_text`'s x09,
`Adjustment Journal`). An empty number is replaced by the GUID's last 12 characters, or the whole GUID
when shorter (`ko_t1`'s t05 and t03). Then, with `<narration>` as in README section 4.4:

| Ref | Label after the voucher label |
| --- | --- |
| T1 (on `t1_row_count` and the `t1` finding) | `: <party> <amount>; <party> <amount>; narration: <narration>` |
| T2 (on `t2_row_count`) | `: booked to <ledgers>; narration names <names>; narration: <narration>` |
| per name (on `t2_named_count_<tag>`) | `: narration: <narration>` |
| T2 embedded (on `t2_embedded_row_count` and the finding) | `: booked to <ledgers>; the name of <names> sits inside <parts>` |

- T1 `<party> <amount>`: one entry per party of the row, in code-point order of the name (capitals
  before lower case: `ko_t1`'s t04), the amount being the sum of every line of the voucher on that
  ledger, written as in README section 4.3; entries are joined by `; `.
- `<ledgers>`: the distinct ledgers with a **nonzero** line that are not bank or cash ledgers, in
  code-point order, joined by `, ` (`ko_t2`'s b13 shows two; `ko_vocab`'s v49 leaves out its zero line
  on `Sales`, and `ko_t2`'s b12 its zero line on `Gamma Supplies`); when there is none, the text
  `no other ledger` (`ko_t2`'s b04).
- `<names>`: the row's plain names (T2) or its inside names that are not plain (embedded), in
  code-point order of the names as the book writes them, before any normalisation, joined by `, `
  (`ko_vocab`'s v51: the decomposed `Ze` + U+0301 + `ro Mart` sorts before the composed `Zéro
  Bazaar`, although after normalisation it would sort after it).
- `<parts>`: for each inside occurrence of those names (never one recorded for a name that is also
  plain on the voucher: `ko_vocab`'s v50), the narration as written from the start of the part
  holding the name's first token to the end of the part holding its last token, separators
  included, with its whitespace collapsed (README section 4.4, without the cut). The distinct texts
  are sorted in code-point order, each is rendered by Python's `repr()` on its own, and they are
  joined by `, ` (`ko_nested`'s n17: `'LAMBDA TRADERS YOD', 'ZED LAMBDA TRADERS'`; `ko_text`'s b02 in
  double quotes). A name written across a separator so shows every part it spans (`ko_nested`'s
  n18: `'ZED LAMBDA-TRADERS'`; `ko_vocab`'s v43: `'LAMBDA-TRADERS ZED'`); a name within one part
  shows that part alone (`ko_nested`'s n08: `'ZED LAMBDA TRADERS'`).

Ledger refs (on the findings only) have kind `ledger`, the ledger's name as id, and the labels given in
README section 4.5. Every label and id is NFC-normalised in the dump (parity spec section 2.2): `ko_tags`
cites the decomposed `Zoe` + U+0308 ledger as the composed `Zoë Crafts`, and `ko_vocab`'s v47 shows its
Kelvin sign as a plain `K`.

### 4.3 Amounts as text

As the crate's `support::rupees` writes them: `₹`, then the whole rupees in Indian grouping (the last
three digits, then pairs), then `.` and two digits only when the paise are not zero; a negative
amount starts with `−` (U+2212). So `₹0`, `₹0.01`, `−₹0.05`, `₹0.50`, `₹123.45`, `−₹1,500`,
`₹5,00,000` (`ko_t1`, `ko_text`).

### 4.4 The narration as shown

The narration's whitespace runs are collapsed to one space and the ends trimmed, by Python's
`str.split()` whitespace (the crate's `support::py_split`): tabs, newlines and the no-break space
are whitespace, a zero-width space is not (`ko_text`'s x03 and x06). If the result is longer than 80
characters (code points, not bytes or graphemes), it is cut to its first 79 and `…` (U+2026) is
added: 80 is shown whole, 81 is cut (`ko_text`'s x01, x02, and x07 in Devanagari). The result is
rendered by Python's `repr()` (the crate's `support::py_repr_str`): single quotes unless the text
holds a single quote and no double quote; a backslash doubled; a non-printable character written as
an escape, so x06's zero-width space appears in the golden as the six characters `\u200b` and the
label reads `'Set\u200boff समायोजन'`; the empty narration is `''` (`ko_text`'s x04, x05, x06, x08,
x09).

Names are always looked for in the whole narration, not the shown one: `ko_text`'s b01 names
`Alpha Traders` across the cut.

### 4.5 Findings

| Finding id | When | Facts (name: figure) |
| --- | --- | --- |
| `knock_off_candidates/t1` | at least one T1 row | `candidate_total`: `t1_candidate_total_paise`; `rows`: `t1_row_count` |
| `knock_off_candidates/t2` | at least one T2 row | `candidate_total`: `t2_candidate_total_paise`; `named_pairs`: `t2_named_pair_count`; `rows`: `t2_row_count` |
| `knock_off_candidates/t2_embedded` | at least one T2 embedded row | `candidate_total`: `t2_embedded_candidate_total_paise`; `rows`: `t2_embedded_row_count` |

All three have clauses `[]` (an empty list, not absent; parity spec section 3.1) and confidence
`judgement_required`.

Evidence:

- `t1`: the T1 refs, exactly as on `t1_row_count`.
- `t2`: one ledger ref per plainly named ledger, label
  `named in <k> voucher(s), <total> (a candidate total, not a reportable amount)`, where `<k>` and
  `<total>` are that ledger's two figures, the total written as in README section 4.3. No voucher ref.
- `t2_embedded`: one ledger ref per inside name, label `inside a longer name in <k> voucher(s)`,
  `<k>` being the number of embedded rows listing it, then the embedded refs as on
  `t2_embedded_row_count`.

**Texts** are compared by hash (parity spec section 4); take them from the goldens. Each finding has a
fixed title, three limits (two shared by all three findings, one of its own) and one ask; `t2` and
`t2_embedded` share their ask. Nothing in any text depends on the book. `ko_text` carries all three
findings, so every text of the test is in the goldens; there is no message no golden reaches.

### 4.6 Population note

A fixed sentence naming the party groups and the bank and cash groups (four and three, README section
2.5), the same in every golden, whatever `party_groups` holds.

### 4.7 The tag

`<tag>` is the crate's `ledger_ids::stable_ledger_tag` of the named ledger (parity spec section 11):

1. Normalise the ledger's GUID: strip surrounding whitespace, then lower-case.
2. When the result is not empty, the tag is the first 8 hex digits of the SHA-1 of its UTF-8 bytes.
3. When it is empty, the tag is the first 8 hex digits of the SHA-1 of the ledger's **name**, as
   written in the book, before any normalisation. The test never refuses a GUID-less ledger.

Every name this test tags is a ledger master, so the case of a name with no master (parity spec section 11)
does not arise. `ko_tags` covers each shape:

| Ledger | GUID in the book | Tag source | Tag |
| --- | --- | --- | --- |
| `Alpha Traders` | `edge-ko_tags-alpha` | GUID | `2062fff0` |
| `Beta Stores` | `  EDGE-KO_TAGS-BETA  ` | `edge-ko_tags-beta` | `bb158f3f` |
| `Epsilon Mills` | `EDGE-KO_TAGS-EPSILON` | `edge-ko_tags-epsilon` | `7828d566` |
| `Gamma Works` | empty | name | `755fba76` |
| `Delta Works` | three spaces | name | `64e676ab` |
| `Zoe` + U+0308 + ` Crafts` | empty | name, decomposed bytes | `b7e7b40e` |

The last row is the trap: the composed form `Zoë Crafts` would give `aeb5fdf9`. Hash the name as the
book holds it, and normalise only where the dump does (README section 4.2). Two ledgers whose GUIDs
normalise alike stop the run when the book is built, before this test (README section 10).

## 5. When nothing is listed, or the book is empty

There is no applicability gate. The population is always formed and every voucher in it is looked
at. With nothing listed, the dump has the seven figures of README section 4.1 at 0 with empty
evidence, no finding, and the fixed population note (`ko_quiet`, every voucher a near miss;
`ko_empty`, no voucher at all; the two goldens are byte-identical). An empty `party_groups` is the
same as an absent one.

## 6. The module's own checks

None. The test has no check of its own: `module_invariants_evaluated` is `[]` and
`module_invariant_violations` is `[]` in every golden. A port adds none (parity spec section 5).

## 7. Book and result checks in the dumps

Every golden carries the book-level and result-level reports of parity spec section 5. In every
edge golden every report is empty (the synthetic golden of README section 14 carries the synthetic
read's own book-level violations, MAP-1, POP-1, POP-2 and POP-3, which do not come from this test,
and an empty result-level report): each edge book's vouchers tie to its Trial Balance, every line's
ledger has a master, and the result checks cannot fire on this test's output, because every fact names one of its own
figures (REND-0), every voucher ref is a population voucher and every ledger ref a master (EVID-1,
POP-4).

## 8. Ordering

Nothing about the order in which the test walks vouchers or names shows in a dump: figures, findings,
evidence and violations are all sorted (parity spec section 6). Findings sort as `t1`, `t2`,
`t2_embedded`. What does show is the order inside a label, given in README section 4.2: parties,
booked ledgers and names in code-point order of the names as written, parts sorted. The reference
also orders the `t2` ledger refs by count and then name before the dump sorts them; that order is not
part of the contract.

## 9. What the test never does

- It reaches no conclusion on any listed voucher. Whether a row is a cash payment caught by s.40A(3)
  or saved by Rule 6DD, a loan or deposit, a trade set-off, or a client holding two ledgers, and
  whether anything is disallowed, is left to the auditor. Small, reversing and opening entries are
  listed like any other.
- It produces no clause 31 or clause 21(d) entry, puts nothing in a finding's clause list, and its
  totals are not amounts to report.
- It never reads the Trial Balance, a party's PAN, GSTIN or bill-wise details, or the party field.
  Besides the book it reads only the rules version and `party_groups`.
- It never matches a name loosely: no partial names, abbreviations, transliteration or spelling
  variants. A name in Devanagari is found only where the narration writes it in Devanagari, with the
  same letters and signs. Nothing is read alike beyond what README section 3.3 reads alike (case,
  spacing and punctuation, a character written composed or decomposed, an accent on a letter `a`
  to `z`, and the characters it drops). A name that yields no token is never found.

## 10. Not covered by any golden

Reachable in the module but in no book:

- **A voucher of unknown status.** Forming the population refuses (measured: the reference raises its
  unknown-status error before any figure). The edge-book shape cannot express an unknown status (it
  does express optional, cancelled and post-dated). The crate's `Book::population` already refuses;
  propagate that error.
- **Two ledgers whose GUIDs normalise to the same value.** The book is refused when built (parity spec section 11),
  before this test runs; the crate's `load_book` does the same.
- **Two tags alike.** A GUID-less ledger whose name equals another ledger's normalised GUID gets the
  same tag; when both are named plainly in the book, the reference stops with a duplicate figure id
  error (measured with invented ledgers). Ask before choosing an error type.
- **A `party_groups` value that is not a list of text.** The real pipeline's binding step refuses it
  (`BIND-ID-MALFORMED`) before the test runs. A port's edge-book reader should refuse a value that is
  not a list or holds anything but text; the crate's `strs()` panics on a non-text item and silently turns a value that is not a list (a bare
  string, say) into an empty list, so it is not a safe reader for this key; how to refuse is the porter's
  choice.
- **A voucher line on a ledger with no master.** It is neither a party nor a bank or cash ledger, and
  appears in `<ledgers>` when nonzero; the book checks would then report it (MAP-0). No book has one.
- **A ledger whose chain is incomplete** (`chain_complete` false): the test uses its chain as given,
  and the book checks would then report it (MAP-1). No book has one.

Rules no golden can pin: the word `a` in the never-name-like list. Removing it changes nothing,
because a one-letter word already fails the two-letter test of README section 3.4. Every other
word, every separator and non-separator, and each rule HASHES.md lists changes at least one golden
when it alone is changed (measured at reference commit `742f67fc` by changing the reference one rule
at a time and re-running every book).

Rules of README sections 3.3 and 3.4 that no book reaches, so that changing any one of them alone
changes no golden (measured the same way); a port follows them from this note:

- NFKD in place of NFD, or case-folding before decomposing (README section 3.5);
- a mark that starts a token kept, or U+200C, U+200D and U+00AD made to end a token;
- marks, or digits, not counted as token characters (no book has Devanagari words that differ only
  in their signs, nor names that differ only in a digit);
- a token in another alphabet counted as name-like.

## 11. Behaviour that may look like a defect

Reproduce these as the goldens and this note show them, and raise them on the pull request rather
than fixing them in the port. The four behaviours an earlier form of this section listed were
changed in the reference (README section 13.1); these remain:

- **An accent is ignored only on a letter `a` to `z`.** On any other letter it stays (README section
  3.3), and two names differing only by an accent on `a` to `z` share their tokens, so both are bound
  to no one.
- **U+037E, which decomposes to `;`, ends a token but does not cut a part** (README section 3.3),
  because parts are cut on the text as written. Measured on invented vouchers: `ZED`, U+037E,
  `LAMBDA TRADERS` puts the name inside, `ZED;LAMBDA TRADERS` leaves it plain; the dump shows both
  labels with `;`, as it NFC-normalises them.

## 12. The books

All are invented: synthetic names, round figures plus a few small odd amounts under ₹1,000, nothing
read from Tally. Each book's `comment` says what it reaches, voucher by voucher.

| Book | Reaches |
| --- | --- |
| `ko_quiet` | Eleven near misses, nothing listed: own party named, one party, one party on two lines, two parties with a bank or a cash line, a second party on a zero line only, a bank and an expense ledger named, a narration of spaces, a name with no bank or cash line, partial and run-together names, a zero bank line. |
| `ko_empty` | No vouchers: seven zero figures, no finding; golden byte-identical to `ko_quiet`'s. |
| `ko_t1` | T1: debtor against creditor; a ledger under a sub-group of `Loans (Liability)` against a ledger directly under `Loans & Advances (Asset)`; a credit note and a sales voucher; three parties in code-point order; a zero bank line; a party netting to zero; empty numbers with short and long GUIDs; a five-lakh amount; a cash line that blocks. |
| `ko_text` | Label text: narrations of 80, 81 and over 80 Devanagari characters, collapsed whitespace, quotes, a zero-width space, a backslash, no narration; amounts from 1 paise; a 1-paise bank line that blocks; a name found across the cut; an inside part with a quote. All three findings. |
| `ko_t2` | T2: names in parts and in lower case, two names in one narration, a contra with no other ledger, a cash payment, two booked ledgers, two bank ledgers, a name found twice, a digit in a name, an accented name written with and without its accent, a Devanagari name in a Devanagari narration, a party on a zero line only (all listed); names not read (two parties sharing a name in different case, a party and an expense master sharing one); two parties whose names differ only in punctuation (`Kappa Traders`, `Kappa-Traders`: a shared name, README section 13.2). |
| `ko_nested` | Nested, overlapping and chained names; plain and inside decided by a separator, a legal suffix, a single letter, a number, a letter-and-digit word and a two-letter word; parties written back to back; plain winning over inside; one voucher in both lists; two inside parts; a name across a separator; own party skipped. |
| `ko_vocab` | One voucher per rule: all 55 never-name-like words (two per voucher, one on the last), the nine separators and five non-separators, the token after a name spread over two parts, the sharp s in a narration and in a name, the `ﬁ` ligature, the Kelvin sign, the dotted capital I (found, its dot dropped), a zero line left out of `<ledgers>`, the parts of a plain name left out, and two names sorted as written rather than as normalised. |
| `ko_shared` | A name two masters share stays in the index: `Sigma Trading` (a creditor) and `SIGMA-TRADING` (a debtor) share their words, so the name is bound to no one, and the shorter party `Sigma` inside it is dropped as nested (s01 is not listed); s02 names only `Sigma` and is listed. |
| `ko_transfer` | A bank-to-cash transfer booked as a Payment with no line on any other ledger counts its larger side once (t01: 500,000 paise); a Journal with the same money lines and a charge line counts both money lines (t02: 990,000 paise). |
| `ko_groups` | `party_identity.party_groups` with a custom group under `Capital Account`, a repeated default, `Cash-in-Hand` and a name no group has: T1 through a partner's ledger; an overdraft ledger that is both party and bank; a cash ledger and a partner's ledger named as parties. |
| `ko_groups_off` | The same book with no `party_identity` table: only the overdraft and the debtor are found, with the same tags as in `ko_groups`. |
| `ko_tags` | Every tag shape of README section 4.7; the decomposed name is found by a narration that writes it without its mark (k06). |
| `ko_status` | Optional, cancelled and post-dated vouchers that would be listed if regular; only the regular one is. |

## 13. Running the books

The crate's `tests/edge_books.rs` does not run this test yet. A port adds that: read
`party_identity.party_groups` (absent meaning `[]`), refusing anything but a list of text (README section 10);
run the test on the book with the rules and those groups; and compare each whole dump
with its golden as the other edge books do (parity spec section 7). There is no module check to run.

The reference's side of the edge harness, `parity/edge_golden.py`, is in this repository, but it has
no runner for this test yet: the runner that produced these goldens extends it and is held by the
reference's maintainers, outside this pack (HASHES.md says how it calls the test). A porter does not
write the generator that makes the goldens, which only the reference's maintainers regenerate; a port
adds just the registration lines of README section 14, so that its own harness can name the test.

### 13.1 Re-pin note: names in any alphabet, zero lines and plainer texts (reference commit `742f67fc`)

The goldens were regenerated from reference commit `742f67fc` (Python 3.13, the pack's own books).
As a control, the same generator at reference commit `98ee6e71`, which precedes the change,
reproduced all fourteen goldens of the previous pin byte for byte. All fourteen change:

- **Texts, in every golden.** The definitions now read as README section 4.1 says. Every finding's
  first limit no longer says "the candidate total is a candidate total", and the `t2` limit is
  reworded to say what a name match misses.
- **Tokens (README section 3.3).** They were the runs of ASCII `a` to `z` and `0` to `9` after
  case-folding, so any other letter or mark cut them: `Café Corner` was keyed `caf corner`, a
  Devanagari name had no key, and the dotted capital I cut its word in two. `ko_t2` now lists b10
  (`Cafe Corner deposit`) and b11 (Devanagari), and `ko_vocab` lists v48 (`İOTA WORKS`).
- **Zero lines (README section 3.3).** A party with only a zero line on the voucher counted as on it
  for T2, so it was not reported; it now is, as T1 and `<ledgers>` already read such a line.
  `ko_t2` now lists b12.
- **The part shown (README section 4.2).** It was the part holding the name's first token, and all
  the parts sat inside one quote, joined by ` | `. It now runs to the part holding the last token,
  and each text is quoted on its own (`ko_nested`'s n17 and n18, `ko_vocab`'s v43).
- **Figures.** `ko_t2`: `t2_row_count` 13 (was 10), `t2_named_pair_count` 14 (was 11),
  `t2_candidate_total_paise` 2,880,000 (was 2,760,000); `Café Corner` named in 2 vouchers, ₹1,000
  (was 1, ₹600); `Gamma Supplies` in 5, ₹20,500 (was 4, ₹20,000); a new group for `देव ट्रेडर्स`,
  1 voucher, ₹300. `ko_vocab`: `t2_row_count` 45 (was 44), `t2_named_pair_count` 46 (was 45),
  `t2_candidate_total_paise` 4,500,000 (was 4,400,000); a new group for `Iota Works`, 1 voucher,
  ₹1,000. No other golden changes a value or the vouchers it lists.
- **Books.** None was added. The comments of `ko_t2` (b10, b11, b12) and `ko_vocab` (v48) were
  corrected to match; the goldens do not read a comment.

### 13.2 Re-pin note: the reference's fix of two behaviours (reference commit `66e842e7`)

The figures in this section are those of that commit; README section 13.1 gives what changed after it.

An earlier form of this pack documented two reference behaviours as defects. The reference has since
fixed both, and the goldens were then regenerated from it (reference commit `66e842e7`, Python 3.13, the
pack's own books; each of the eleven goldens of the earlier pack was regenerated and compared). Exactly one golden
changed, `ko_t2`; the other ten are byte-identical. Two books were added with their goldens, `ko_shared` and
`ko_transfer`, one for each rule below that no earlier book pinned.

- **A name two masters share is listed for none.** Before: masters were counted by their case-folded
  name but indexed by their token tuple, so `Kappa Traders` and `Kappa-Traders` shared one key and the
  later in code-point order won it. Now masters are counted by token tuple over every group, and a
  tuple with more than one master is bound to no one (but stays in the index as a name the narration
  holds). In `ko_t2` b06 is no longer listed, so `t2_row_count` is 10 (was 11), `t2_named_pair_count`
  is 11 (was 12), the Kappa group's count and total figures are gone, and `Alpha Traders`' named total
  is 3,000 rupees (was 5,000).
- **A voucher's money counts once.** Before: every bank or cash line was added by absolute value, so a
  Contra counted twice. Now a voucher whose every non-nil line is on a bank or cash ledger, or any
  Contra, counts the larger side of its money lines once; every other voucher is as before. In `ko_t2`
  b04 (a 2,000-rupee contra naming `Alpha Traders`) counts 2,000 (was 4,000), and
  `t2_candidate_total_paise` is 2,760,000 (was 3,360,000: 4,000 rupees less for b06, 2,000 rupees less
  for b04).
- **Known limits the reference states:** a transfer between a bank and a cash ledger booked as a Payment
  or Journal with a charge line on another ledger still counts both money lines; a name two party
  masters share, with neither on the voucher, is listed for neither (a recall loss, as a case-only
  duplicate always was).
- **The two rules no earlier book pinned are now pinned** by the two new books: `ko_shared` (a shared key
  stays in the index as a name the narration holds, rather than being dropped from it: dropping it makes `ko_shared`
  list s01 for `Sigma`) and `ko_transfer` (a voucher booked as a type other than Contra, whose every non-nil line is
  on a bank or cash ledger, counts its larger side once: counting only Contra that way changes `ko_transfer`). Each
  change was applied to a copy of the reference: the new book changes and every other golden does not; on the
  unchanged reference both new books reproduce.

## 14. Registering the test in the crate

A new registry entry needs, besides the books, a golden for the synthetic read and the two harness lines.

- **The synthetic golden.** `goldens/synthetic.knock_off_candidates.json` is the test on the crate's synthetic
  read (`tests/fixtures/synthetic-engagement.toml`), with no party groups configured. It has the seven figures
  (all zero or empty counts and totals) and no finding. It goes to
  `tests/fixtures/golden/synthetic.knock_off_candidates.json`. The registry's `min_figures` for this test is 7.
- **The harness lines** (the reference's maintainers keep and regenerate the harness; these are the lines a port
  adds so its own harness can name the test). In `parity/python_golden.py`, a runner and its `RUNNERS` entry
  (the `RUNNERS` list is kept sorted by test id):

      def _knock_off_candidates(c):
          from tae.audit_tests import knock_off_candidates
          return knock_off_candidates, knock_off_candidates.run(
              c.eng, c.rules, tuple(c.cfg.get("party_identity", {}).get("party_groups", ())))

      "knock_off_candidates": _knock_off_candidates,

  In `parity/edge_golden.py`, the `runners` table gets the same call with the book's `party_identity` table
  (`tuple(spec.get("party_identity", {}).get("party_groups", ()))`). A port only needs these to keep the two
  sides' test lists in step (the crate has tests that compare the Rust registry with the harness's list).
