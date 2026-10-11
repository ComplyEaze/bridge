# Spec pack: `narration_payees` (payees named only in the bank narration, against the s.194C limits)

The goldens in this pack were produced by the reference engine at commit `2b329354` and are the
contract; this note explains them and cites [`docs/tax-audit/parity-spec-v1.md`](../../parity-spec-v1.md)
(parity spec sections 1, 2.1, 2.2, 3, 3.1, 4, 4.1, 5, 6, 7, 10 and 11, as relevant); where the note and a
golden differ, the golden wins and the reference's maintainers should be told on the pull request or issue.

In this note "README section N" is a section of this note and "parity spec section N" a section of
that document.

This pack is for porting the test into `src-tauri/crates/bridge-tax-audit`. It holds:

- `books/np_*.json`: invented edge books, in the shape of the crate's
  `tests/fixtures/edge-books/*.json`. They carry one key the crate's edge-book reader does not know
  yet, `narration_payee_ledgers` (README section 2.3), beside keys it already reads (`bank`, and the
  TDS payee test's `nature_by_ledger`).
- `goldens/edge.<book>.narration_payees.json`: the reference's canonical dump of this test on each
  book (parity spec section 1), module check included.
- `goldens/edge.<book>.tds_payees.json`: the canonical dump of the TDS payee test on the same book,
  from the same run. That result is this test's input (README section 2.4); the TDS payee test is
  already ported (`src/tds_payees.rs`) and these goldens add nothing to its contract.
- `goldens/synthetic.narration_payees.json`: the test on the crate's synthetic read (README section 16).
- `HASHES.md`: bytes and SHA-256 of every book and golden, what the books establish, which rule
  changes each golden catches, and how the goldens were made.

A port needs three things the crate does not have: a reader that takes a payee's name and UPI handle
out of a bank narration (README section 3, specified here in full and pinned by `np_forms`,
`np_unread`, `np_text`, `np_handles` and `np_cut`), the configuration key and its binding (README
sections 2.3 and 2.5), and the evidence kind `payee_name` (README section 6.6).

## 1. What the test does

Some expense ledgers (wages, loading, cartage) are paid to many people, and the books name none of
them: the voucher debits the expense ledger and credits the bank. The only place a payee appears is
the narration, where the bank's own statement text was pasted. This test reads that text.

It takes the bank payments debited to a set of ledgers, reads the payee the bank printed on each,
joins payments that print the same name or the same UPI handle into one payee, and compares each
payee's largest payment and total with the two s.194C limits. It lists beside that: payments whose
narration names no one, debits with no bank payment behind them, payees paid more than once on one
day, payees whose payments booked to other ledgers would take them over a limit, and payees that a
UPI handle printed cut short might join.

It reaches no conclusion. Whether tax was deductible depends on things the books do not hold (whether
the assessee was a deductor, whether a payee was a contractor or an employee), so every finding is a
question.

## 2. Inputs

### 2.1 The book

Read from the engagement's book, as the crate's `src/book.rs` holds it:

- **Vouchers**: GUID, date, voucher type name, base type, number, narration, status and lines; each
  line is a ledger name and an integer amount in paise, debit positive, credit negative.
  In an edge book an absent `number` is the voucher's GUID, an absent `vtype` its base type, an absent
  `status` `regular` and an absent `narration` the empty text; the crate's `tests/edge_books.rs`
  already reads them so, and the labels in the goldens (`np_single`: `Payment s02 on 2025-05-03`) follow from it.
- **Ledger masters**: only each ledger's group chain, and only to decide whether a debit on a
  payee's payment is to a bank, cash or duties ledger (README section 6.4). Ledger GUIDs are not read.
- **The books population**: vouchers whose status is regular (`Book::population`). Optional,
  cancelled and post-dated vouchers are never read (`np_quiet`'s q01 to q03). The test always forms
  the population, so a book with any voucher of unknown status refuses (README section 12).

Not read: the Trial Balance, the books period (`np_rows`' r14 is dated after it and is read), the
party field, the voucher's reference field, inventory, the group masters themselves, and the edge book's
`cash` list.
The base type is read in one place only (a `Contra`, README section 6.4); everywhere else a voucher of
any type is treated alike (`np_rows`' r09 and r10).

### 2.2 The bank ledgers

A set of ledger names. A line counts as a bank line when its ledger's name is in the set, compared
exactly as written.

- **In the reference's pack** the set is every ledger under one of the groups in `[roles].bank_groups`,
  the set the other tests take; in the crate, `book.ledgers_under_any(&bound.bank_groups)`.
- **In the edge books** it is the `bank` list, as the crate's `tests/edge_books.rs` already reads it.
  The books use that to separate two things the real pipeline cannot: a ledger under a bank group
  that is not in the set (`np_rows`' r03, `np_elsewhere`'s e05 and e15) and a ledger in the set that
  is under no bank group (`np_elsewhere`'s `Wallet Float`). A name in the set that matches no master
  still makes a bank line (`np_names`' m03), and a name that differs only in case does not
  (`np_names`' m05).

### 2.3 The ledgers the client lists

`[roles].narration_payee_ledgers` in the client configuration: a list of ledger names, the ledgers
whose payees are named only in the bank narration. The key is optional; absent means none. The caller
turns the list into a set, so a name listed twice is one (`np_rows` lists `Wages` twice and its
count is 2).

In the edge books it is the top-level key `narration_payee_ledgers`, a list of text, absent meaning
`[]`. The crate's `tests/edge_books.rs` does not read it yet; a port's reader should refuse anything
but a list of text, as the reference's side of the edge harness does (README section 16).

The test compares these names with line ledgers exactly as written. `np_names`, which the real
pipeline never reaches (README section 2.5), shows what an unbound name does: a name with no master
is still read when a line carries it (m01), a name that differs in case reads nothing (m04), a name
with a trailing space is a different name and counts apart, and each distinct name counts in
`configured_ledgers_count` whether or not it matches anything.

### 2.4 The ledgers added from the TDS payee test's result

Besides the listed ledgers the test reads some ledgers the client did not list. The caller works them
out from the result of the TDS payee test (`tds_payees`) on the same book, run first with the same
configuration:

1. Find the finding the TDS payee test raises for payments of an expense mapped to 194C that it could
   tie to no named payee. Its id is `tds_payees/194C_<h>`, where `<h>` is the first 8 hex digits of the
   SHA-1 of `194C:` followed by the ledger tag of the name `(payee not named)`
   (`tds_payees::PAYEE_NOT_NAMED`; the tag is the crate's `ledger_ids::stable_ledger_tag`, parity
   spec section 11). No book has a ledger of that name, so wherever the finding exists its id is
   `tds_payees/194C_f81067e8`. The port builds this id inside `tds_payees::run`; this test must
   look for exactly it. Other findings of that test (a named payee over the limit, as in
   `np_quiet`'s q12, or any id with further path segments) add nothing.
2. When there is no such finding, nothing is added. Whether it exists is the TDS payee test's own
   rule and is not restated here (`np_quiet`'s q08 to q11 sit exactly at both of its s.194C limits,
   raise none and add nothing).
3. Otherwise take the ids of the finding's evidence refs of kind `voucher`: they are voucher GUIDs.
4. For every population voucher whose GUID is one of them, every line with an amount above zero on a
   ledger that `[tds].nature_by_ledger` maps to exactly `194C` adds that ledger.

So a ledger is added only by a debit (`np_added`'s `Hamali` is credited on a cited voucher and stays
out), only when mapped `194C` (`Consultancy`, mapped `194J`, stays out), and only from a cited voucher
(`Loading Charges`, debited on a purchase voucher the TDS payee test does not pool, stays out). The
match is by GUID, not by voucher: on a book where vouchers share a GUID, a ledger debited on another
voucher with the cited GUID is added too (`np_shared_guid`'s `Job Work`), but not from a voucher
outside the population (`Hamali` there).

The added set is passed to the test apart from the listed set. The test reads both alike;
`added_ledgers_count` counts the added ledgers that are not also listed (`np_added`: `Wages` is both,
and the count is 2). The module check is given the union of the two sets.

The `edge.<book>.tds_payees.json` golden beside each book shows the result the test was given.
In the edge books the TDS payee test's own inputs are the keys the crate's `tds_config(&s)` and
`tds_inputs(&s)` already read (`nature_by_ledger` and the rest, each absent in most books, and
`entity_type`, absent meaning `individual`).

### 2.5 Binding (before the test runs)

In the reference's real pipeline a separate step binds every configured ledger name to the book's
masters before any test runs. `roles.narration_payee_ledgers` is one of its ledger-name locations, a
list of names. Measured on the reference with an invented book:

- a name that matches no ledger master and has no `[ledger_ids]` entry is refused with
  `BIND-NAME-UNKNOWN`, naming the location `roles.narration_payee_ledgers`; the match is exact, so a
  name that differs only in case is refused too;
- a value that is not a list of text is refused with `BIND-ID-MALFORMED`;
- a label bound in `[ledger_ids]` is rewritten to the master's current name; the list keeps its
  order and its repeats, and the caller's set removes the repeats.

The crate's `src/binding.rs` does not bind this location yet, and its `Engagement` has no field for
the key: a port adds both, binding the list as `roles.round_off_ledgers` is bound, and reads the
bound list. `[tds].nature_by_ledger`'s keys and `[roles].bank_groups` are bound already.

The edge books go round binding, as every edge book does, which is how `np_names` can show unbound
names.

### 2.6 Rules

`[s194c].single_sum_paise` (the single-sum limit, 3,000,000 paise in the rules both sides carry) and
`[s194c].aggregate_paise` (the aggregate limit, 10,000,000 paise); in the crate, the `s194c` field of
`Rules`. The rules version is copied into `rules_version` (`2026-09-17.1` in every golden).
`test_version` is `"1"`. The entity type is not read by this test (the TDS payee test reads it).

### 2.7 Fixed values (not client data)

- **The two placeholders** of the second UPI layout: `BANKACC` and `PHONEPE`, compared after
  upper-casing and removing spaces (README section 3.2).
- **The two refused cheque beginnings**: `CASH PAID TO` and `SELF`, as words (README section 3.2).
- **The channel words** that are no name in forms 1 to 3: `UPI`, `IMPS`, `NEFT`, `RTGS` (README section 3.1).
- **The channel names**: `UPI`, `IMPS`, `NEFT`, `RTGS`, `CHEQUE_COUNTER`.
- **The groups whose ledgers are not "other ledgers"**: `Bank Accounts`, `Bank OD A/c`,
  `Cash-in-Hand`, `Duties & Taxes` (README section 6.4).
- **The base type** `Contra` (README section 6.4).
- **The nature** `194C` and the name `(payee not named)` (README section 2.4).
- **The key prefix** `upi:` (README section 5.3).

## 3. Reading a payee out of a narration

This is the part a port has to write from this note. The reference reads two things from a
narration: a payee (a name and a channel), or the fact that none can be read; and, for a UPI payment
with a payee, a handle. The crate has no regular-expression dependency; each form below can be
matched by walking the text once.

### 3.1 Preparing the text

1. An absent narration is the empty text (`np_unread`'s u81).
2. Upper-case the whole text with Python 3.13's `str.upper()` (the crate's `support::py_upper`,
   parity spec section 4.1). This is the full Unicode mapping, so it can lengthen the text: the sharp
   s becomes `SS`, the fi ligature U+FB01 `FI`, the long s `S`, the dotless i `I` (`np_text`'s t01
   to t08). The Kelvin sign U+212A is already upper case and stays what it is, which is not `K`
   (t09).
3. Split on Python whitespace and join the pieces with one space (`support::py_split`): leading and
   trailing whitespace goes, and every run inside becomes one space. Python's whitespace includes
   the no-break space, U+2028, U+0085, U+3000 and U+001C to U+001F (`np_text`'s t13 to t17); it does
   not include the zero-width space U+200B or U+FEFF (t18, t19).

Everything below is said of this prepared text. Every form must start at its first character
(`np_unread`'s u09, u12, u38, u52 and u74 put text before a form). Nothing is required after a form's
last character: whatever follows is ignored.

Three character sets are used:

- **A to Z** is the 26 ASCII capitals. An accented letter, a full-width letter or a letter of
  another script is not one (`np_text`'s t10 to t12).
- **0 to 9** is the ten ASCII digits (`np_text`'s t33 to t35 and `np_handles`' h43 and h44 put a
  Devanagari digit where only these are read).
- **A decimal digit** is any character Python's `\d` matches: a decimal digit of any script
  (`support::py_is_decimal`). Devanagari and full-width digits are; a superscript two and a circled
  digit are not (`np_text`'s t21 to t24).

A **name** in the first three forms is: one character A to Z, then zero or more characters each of
which is A to Z, 0 to 9, a space, a full stop, `&` or `'`, then a hyphen. The hyphen is not part of
the name and must be there. Since a name cannot hold a hyphen it ends at the first one after it
starts (`np_forms`' f04), and a character outside the set before that hyphen means the form does not
match at all (`np_unread`'s u06 to u08). A space left at the end of the name is removed (`np_forms`'
f03, f14 and f25); nothing else is.

A name that is exactly `UPI`, `IMPS`, `NEFT` or `RTGS`, once that space is removed, is no name: the
form fits, the text names no one and no other form is tried (`np_unread`'s u84 to u88; u85 has the
space). A name that only begins with one of these words is a name (`np_forms`' f59 and f60). Forms 4
and 5 have no such rule: a name field `UPI` and a cheque named `UPI` are both the payee `UPI`
(`np_forms`' f61 and f62).

### 3.2 The five forms

They are tried in this order and the first that fits is used. A text can fit two: `UPI-CHQ PAID-X` fits
the first and the fifth, and is read by the first. No book has one.

| # | Form | The text is | Result |
| --- | --- | --- | --- |
| 1 | first UPI layout | `UPI-`, a name | the name, channel `UPI` |
| 2 | IMPS | `IMPS-`, one or more decimal digits, `-`, a name | the name, channel `IMPS` |
| 3 | NEFT or RTGS | `NEFT` or `RTGS`, an optional space, `DR-`, one or more characters A to Z or 0 to 9, `-`, a name | the name; the channel is the word, `NEFT` or `RTGS` |
| 4 | second UPI layout | `TO TRANSFER-`, an optional space, `UPI/DR/`, a reference number, `/`, a name field, `/` | see below |
| 5 | cheque paid at the counter | an optional prefix, a cheque name, an optional space, `-`, an optional space, `CHQ PAID` | see below |

**Form 4.** The reference number is one or more characters, each a decimal digit or a space; it may be
spaces only (`np_forms`' f33) and may not be empty (`np_unread`'s u43). The name field is one or more
characters other than `/`, and the `/` after it must be there (`np_unread`'s u46). Remove every space
from the field. Then:

- if what is left is exactly `BANKACC` or `PHONEPE`, there is no payee and the payment is a
  **placeholder** (`np_unread`'s u40 and u41; `PHONEPE1` and `BANKACCT` are payees, `np_forms`' f35
  and f36);
- otherwise, if it holds at least one character A to Z, it is the name, channel `UPI`. Any other
  character may be in it, a hyphen, an ampersand or a letter of another script included
  (`np_forms`' f34, `np_text`'s t26);
- otherwise (digits only, another script only, nothing left) there is no payee (`np_unread`'s u42 and
  u51, `np_text`'s t27 and t28).

The placeholders belong to this form alone: in the first UPI layout `PHONEPE` is a payee like any
other (`np_forms`' f08).

**Form 5.** The prefix is `WITHDR`, an optional `A`, `W`, an optional `A` and `L BY`: that is
`WITHDRAWAL BY`, `WITHDRWAL BY`, `WITHDRAWL BY` or `WITHDRWL BY` (`np_forms`' f42 to f45), not
followed by a character A to Z (`WITHDRAWAL BYWAY` is no prefix, `np_forms`' f55); any spaces after
it are part of it. A cheque name is one character A to Z followed by zero or more characters
each A to Z, a space or a full stop; no digit, `&` or `'` (`np_unread`'s u65 to u67). It is followed
by an optional space, a hyphen, an optional space and `CHQ PAID`; as the name cannot hold a hyphen,
that hyphen is the first one in it (`np_unread`'s u68). The name does not take in the optional space
before the hyphen: `EPSILON CARRIERS - CHQ PAID` names `EPSILON CARRIERS` (`np_forms`' f40). `CHQ PAID`
need not end a word (`np_forms`' f47).

- A text that starts with the prefix is read only after it. When what follows cannot be read as a
  cheque name and the rest, the text names no one; it is not read again from its start
  (`np_unread`'s u72 and u89 to u91: `WITHDRAWAL BY - CHQ PAID`, the same with no space round the
  hyphen, and a full stop after `BY`). Text that only resembles the prefix is a name from the start
  (`np_forms`' f53 and f55).
- A name that starts with `SELF` or `CASH PAID TO`, followed by a character other than A to Z or
  by nothing, is refused: no payee (`np_unread`'s u60, u61, u63 and u92, the last `SELF.`). A name
  that only begins with those characters is a payee: `SELFMADE MILLS` and `CASH PAID TOWER`
  (`np_forms`' f57 and f58), as are `MYSELF` and `CASH PAID` (f50 and f51).
- Otherwise the name is the payee, channel `CHEQUE_COUNTER`.

A text that fits none of the five has no payee.

### 3.3 The UPI handle

Only a payment with a payee of channel `UPI` (form 1 or form 4) can have a handle. A handle is text
and a flag: **whole**, or **cut** (the layout printed only its beginning).

- **Form 1.** Directly after the name's closing hyphen: one or more characters each A to Z, 0 to 9,
  a full stop or `_`; then optionally one hyphen followed by one or more decimal digits; then `@`.
  The handle is everything before the `@`, the optional suffix included, and is whole. Anything else
  between the hyphen and the `@` means no handle: a space, an ampersand, a letter outside A to Z, a
  digit of another script, a second suffix, a suffix of letters, a suffix with nothing before it,
  nothing at all, or a full-width `@` in place of the `@` (`np_handles`' h10 to h23 and h41 to
  h44). Nothing need follow the `@` (h24).
- **Form 4.** After the name field's closing `/` come a field of zero or more characters other than
  `/` (it may be empty, `np_handles`' h36), a `/`, the handle field of one or more characters other
  than `/`, and a `/` that must be there (h34). Remove every space from the handle field. If it
  holds an `@`, the handle is the text before the first `@` and is whole (h30, h39); if it holds
  none, the handle is the whole field and is cut. An empty result is no handle (h38).

### 3.4 Traps for a port

- **Upper-case with Python's tables, then match ASCII.** Upper-casing only `a` to `z` leaves
  `np_text`'s t01, t03, t05 and t07 unread; taking `char::is_alphabetic` or `char::is_uppercase`
  for A to Z reads names among t10 to t12 that the reference does not.
- **Two kinds of digit.** The IMPS reference, the form 4 reference and the handle's suffix take any
  decimal digit; names, the NEFT code and the body of a handle take 0 to 9 only.
- **Whitespace is Python's.** Rust's `char::is_whitespace` lacks U+001C to U+001F (`np_text`'s t15);
  use `support::py_split`.
- **Hash the name as read, normalise only in the dump.** `np_text`'s t29 and t30 are two payees whose
  names differ only in how an accent is written; the dump shows both names alike (README section
  6.6), and their tags differ (README section 5.3).

## 4. Which vouchers are read

Let the **ledgers read** be the listed ledgers and the added ledgers together. For each population
voucher:

1. Its **amount** is the sum of its lines that are on a ledger read and above zero. Credits on those
   ledgers are not set off (`np_rows`' r02), lines on two ledgers read add up (r01), and a voucher
   whose amount is zero is not read at all (`np_quiet`'s q05 and q06).
2. It has a **bank leg** when at least one of its lines is on a bank ledger with an amount below
   zero. A nil bank line and a bank debit are not one (`np_rows`' r06 and r07). A population voucher
   with a bank leg is an **outgoing bank payment**, whatever it debits and whatever its type.
3. With an amount and no bank leg, the amount goes to `not_through_bank_total` and the voucher is
   cited there. Nothing is read from its narration (`np_rows`' r03 to r07, all of which carry a
   narration that names a payee).
4. With an amount and a bank leg, the payee is read (README sections 3 and 5). With none, the amount
   goes to `unresolved_bank_total`; if the narration was a placeholder it goes to
   `placeholder_payee_total` as well. With one, the voucher is a **payment** of that payee's row.

The amount is always the voucher's whole debit on the ledgers read, also when only part of it left
by bank (`np_rows`' r08).

## 5. Joining payments into payees

### 5.1 What is joined

Joining looks at every outgoing bank payment of the population whose narration gives a payee, also
those that debit no ledger read. Receipts (a bank debit) take no part (`np_handles`' h62).

Each such payment carries its printed name and at most one handle for joining:

- a whole handle counts as printed;
- a cut handle counts as a whole one only when it is **completed** (README section 5.2); otherwise
  the payment carries no handle for joining.

Two payments are the same payee when they print the same name (the same characters, compared after
reading, so the names of two channels join: `np_forms`' f09 and f56) or carry the same handle, and
the relation is carried through chains: one handle under two names, one name on two handles, and
any chain of them, is one payee (`np_handles`' h01 to h07 and h50 to h56).

Payments are told apart by voucher, never by GUID: two vouchers sharing a GUID are two payments and
may be two payees (`np_shared_guid`'s dup-a).

### 5.2 Completing a cut handle

Take the set of whole handles printed on any outgoing bank payment with a payee. For a payment with a
cut handle `C` and name `N`:

1. the candidates are the whole handles that start with `C` (one equal to `C` is a candidate,
   `np_cut`'s c18; one that merely holds `C` is not, c25);
2. the payment is completed, to that handle, when there is exactly one candidate and at least one
   name printed with that handle, on any payment, agrees with `N`: with the spaces removed from
   both, one starts with the other (`np_cut`'s c02 one way, c04 the other, c16 against the second
   of two names).

With no candidate (c05 to c08), with several (c11, c24), or with one under names that do not agree
(c13), it is not completed.

### 5.3 The key and the tag

A payee's **key** is `upi:` followed by its smallest handle, in code-point order, when any payment
joined into it carries a handle; otherwise its name. "Any payment" includes outgoing bank payments
that debit no ledger read (`np_outside`'s o04 is keyed `upi:TK.7`, a handle only o05 prints), and
"smallest" is over all the payee's handles (`np_handles`' h50 and h51 are keyed `upi:ABE.9`).

The **tag** in figure ids is the first 8 hex digits of the SHA-1 of the key's UTF-8 bytes
(`support::hash8`):

| Key | Tag |
| --- | --- |
| `upi:ALPHA.T` | `7dfd0200` |
| `BETA STORES` | `9fab0428` |
| `upi:ABE.9` | `8c6f8789` |
| `TAU` followed by U+0302 | `667e309b` |
| `TA` followed by U+00DB | `98500d24` |

The key is the text as read: upper-cased, not normalised.

### 5.4 A payee's row

A payee has a **row** when at least one of its payments has an amount (README section 4). The row
holds only those payments: their count, their total, the largest of them, the set of their channels,
and the set of names printed on them. A payee whose payments all debit other ledgers has no row and
appears nowhere (`np_outside`'s o09, `np_elsewhere`'s e20).

The row's **names shown** are its names sorted in code-point order and joined by ` / `; names printed
only on payments outside the row are not shown (`np_elsewhere`'s e09, `np_outside`'s o08 and o12). Its
**channels** are sorted and joined by `, `.

The row is **over** when its largest payment is above the single-sum limit or its total is above the
aggregate limit. At a limit is not over (`np_single`, `np_aggregate`).

## 6. Outputs

Every figure and finding id below is exactly as the goldens carry it. `<tag>` is a payee's tag.
Each definition is fixed text, compared by hash (parity spec section 4); take them from the goldens.
One definition carries a value: that of a same-day figure holds its date, as
`Bank payments to one payee on <ISO date>, together: two or more payments, their total over the s.194C single-sum limit.`

### 6.1 The fifteen figures every book has

| Figure id (`narration_payees.` + ) | Unit | Value | Evidence |
| --- | --- | --- | --- |
| `configured_ledgers_count` | count | distinct listed names | none |
| `added_ledgers_count` | count | added ledgers not also listed | none |
| `payees_count` | count | rows | none |
| `payees_over_194c_count` | count | rows that are over | none |
| `payees_over_194c_total` | paise | the totals of the rows that are over | their payments |
| `resolved_total` | paise | the totals of all rows | none |
| `unresolved_bank_total` | paise | README section 4, step 4 | the vouchers |
| `placeholder_payee_total` | paise | README section 4, step 4 | the vouchers |
| `not_through_bank_total` | paise | README section 4, step 3 | the vouchers |
| `same_day_count` | count | same-day rows (README section 6.3) | none |
| `same_day_total` | paise | their day totals | their payments |
| `elsewhere_over_count` | count | crossing payees (README section 6.4) | none |
| `elsewhere_over_total` | paise | their debits to other ledgers | the vouchers of those debits (not the rows' own payments; the finding adds those) |
| `cut_handle_unjoined_count` | count | README section 6.5 | the vouchers |
| `possible_same_count` | count | groups (README section 6.5) | none |

### 6.2 Per row

| Figure id | Unit | Value | Evidence |
| --- | --- | --- | --- |
| `payee_channel_<tag>` | text | the channels | the name ref |
| `payee_count_<tag>` | count | the payments | the name ref |
| `payee_total_<tag>` | paise | their total | the payments, the name ref |
| `payee_max_<tag>` | paise | the largest | the name ref |
| `payee_over_194c_<tag>` | text | `yes` or `no` | the name ref |

### 6.3 Same day

For each row, group its payments by voucher date. A day with two or more payments whose amounts
together are above the single-sum limit gives one figure:

| Figure id | Unit | Value | Evidence |
| --- | --- | --- | --- |
| `same_day_<tag>_<YYYY-MM-DD>` | paise | the day's total | the day's payments, the name ref |

One payment never makes a same-day row, however large (`np_same_day`'s d01). A row that is over is
listed like any other (d08 to d11), one row can have several days (d04 to d07), and only the row's
own payments count: not a cash payment, a payment whose narration is not read, or a payment booked
to a ledger that is not read (d16 to d20). The limit itself is not over (`np_single`'s s04 and s05).

### 6.4 Debits to other ledgers

For every outgoing bank payment whose payee has a row (the row's own payments included), take the
voucher's **other debits**: zero when the voucher's base type is exactly `Contra`; otherwise the sum
of its lines that are above zero and on a ledger that

- is not a ledger read,
- is not a bank ledger (the set of README section 2.2), and
- has no master whose chain holds `Bank Accounts`, `Bank OD A/c`, `Cash-in-Hand` or
  `Duties & Taxes`. A ledger with no master at all passes this test and is counted (`np_names`'
  m02).

A payee with at least one voucher whose other debits are above zero gets:

| Figure id | Unit | Value | Evidence |
| --- | --- | --- | --- |
| `payee_elsewhere_<tag>` | paise | those vouchers' other debits | those vouchers; one `ledger` ref per ledger so debited on them; the name ref |

`np_elsewhere` has one voucher for each clause above. The payee is **crossing** when its row is not
over and either its row total plus this figure is above the aggregate limit, or the other debits of
one of those vouchers alone are above the single-sum limit. The second test is per voucher: two
lines of one voucher add up (`np_else_agg`'s g42), two vouchers do not (g52 and g53). A row that is
already over is never crossing (g31 and g32).

### 6.5 Cut handles that were not completed

Look again at the outgoing bank payments whose cut handle was not completed (README section 5.2),
grouped by the cut handle's text.

- `cut_handle_unjoined_count` is the number of those payments that are in a row; it cites them. A
  payment that debits no ledger read is not counted (`np_cut`'s c21).
- For each cut handle, collect the payees of (a) its uncompleted payments that are in a row and (b)
  every row payment that carries, printed or by completion, one of that cut handle's candidates.
  When that is two or more payees, they are a **group**:

| Figure id | Unit | Value | Evidence |
| --- | --- | --- | --- |
| `possible_same_<h>` | paise | the group's row totals added | every payment of those rows; one name ref |

`<h>` is the first 8 hex digits of the SHA-1 of the cut handle's text alone (no `upi:` before it).
The group's name ref is the printed names of all its rows, each name once, sorted in code-point order
and joined by ` / `. Two payees
are never joined by this; the figure says what their total would be if they were one.

`np_cut` has a group from no candidate (c05, c06), from two candidates (c09 to c11), from one
candidate under another name (c12, c13), a cut handle under one name that makes no group (c07, c08),
and a cut payment that its name joins to one of its own candidates (c22 to c24).

### 6.6 Evidence refs and labels

- **A voucher ref** has kind `voucher`, the voucher's GUID as id, and the label
  `<voucher type name> <number> on <ISO date>`. The number is used as it is: an empty number leaves
  two spaces in the label (`np_text`'s t41) and a number of spaces is kept (t42). This is not the
  crate's `support::voucher_label`, which puts the GUID's tail in place of an empty number. The type
  is the voucher's own type name, not its base type (t40).
- Among the vouchers a figure or finding cites, two with the same GUID and the same label are one
  ref, and two with the same GUID and different labels are two (`np_shared_guid`'s dup-d and dup-c).
- **A name ref** has kind `payee_name`, and both its id and its label are the row's names shown (for
  a group, the group's). The crate emits no ref of this kind yet (parity spec section 11 lists it
  among the kinds whose id is a name); it needs no resolving in the result checks.
- **A ledger ref** has kind `ledger`, the ledger's name as id and an empty label.

Ids and labels are NFC-normalised in the dump (parity spec section 2.2): `np_text`'s t43 and
`np_elsewhere`'s e16 are written with combining marks and shown composed.

### 6.7 Findings

| Finding id (`narration_payees/` + ) | When | Clauses | Confidence | Facts (name: figure) | Evidence |
| --- | --- | --- | --- | --- | --- |
| `recipient_not_read` | any payment with no payee read | `s.194C` | `needs_document` | `amount`: `unresolved_bank_total`; `placeholders`: `placeholder_payee_total` | those vouchers |
| `s194c_candidates/all` | any row over | `s.194C`, `s.40(a)(ia)` | `judgement_required` | `payees`: `payees_over_194c_count`; `amount`: `payees_over_194c_total`; `not_through_bank`: `not_through_bank_total` | the payments of the rows that are over |
| `same_day/all` | any same-day figure | `s.194C`, `s.40(a)(ia)` | `judgement_required` | `payee_days`: `same_day_count`; `amount`: `same_day_total` | the payments of every same-day figure |
| `elsewhere/all` | any crossing payee | `s.194C`, `s.40(a)(ia)` | `judgement_required` | `payees`: `elsewhere_over_count`; `amount`: `elsewhere_over_total` | for each crossing payee, the vouchers of its other debits and its row's payments |
| `possible_same/all` | any group | `s.194C` | `judgement_required` | `groups`: `possible_same_count` | every payment of every group's rows |

The clause lists are in the order shown (parity spec section 3.1). Every fact names a figure of this
test. **Texts** are compared by hash (parity spec section 4); take them from the goldens. Nothing in
any title, limit or item to ask depends on the book. `np_same_day` carries the first three findings,
`np_else_agg` the fourth and `np_cut` the fifth, so every text of the test is in the goldens.

### 6.8 Population note

One fixed sentence, the same in every golden.

## 7. When nothing is read

There is no applicability gate, and no "not applicable" form. The population is always formed. With
no ledger listed and none added, or with no voucher that debits a ledger read, the dump has the
fifteen figures of README section 6.1 and no finding: `np_empty` (no voucher) and `np_quiet` (twelve
near misses) give byte-identical goldens, with `configured_ledgers_count` at 2 and everything else
at 0 with no citation. The synthetic dump (README section 16) is the same with that count at 0.

## 8. The module's own check (NP-1 to NP-7)

`module_invariants_evaluated` is `["narration_payees.check_invariants"]` in every golden. Each
violation is `{"invariant": "narration_payees.check_invariants", "subject": "narration_payees",
"detail": <text>}`. The check is given the book, the test's result, the bank ledgers and the ledgers
read.

Its messages are below, `<fid>` being a figure's full id and `<tag>` the part of a `payee_total_`
figure's id after that prefix. Two things shape them:

- The check finds a cited voucher by its GUID, and where population vouchers share a GUID it takes
  the **last** of them in book order.
- It visits the `payee_total_` figures in the order the test made them: rows by total, largest
  first, ties by key in code-point order.

**NP-1** walks the population again with the rules of README sections 4 and 5 and compares.

- `NP-1: unresolved_bank_total = <value> but a fresh population walk finds <its sum>`
- `NP-1: not_through_bank_total = <value> but a fresh population walk finds <its sum>`
- `NP-1: <fid> names a payee a fresh population walk does not find at all`
- `NP-1: <fid> = <value> but a fresh population walk of the same payee finds <its sum>`
- `NP-1: a fresh population walk finds payee tag <tag> with no payee_total_<tag> figure`
- `NP-1: resolved_total = <value> != sum of payee_total_* figures (<their sum>)`

None of the six can be produced by the test's own output: the walk repeats the test's own rules on
the same inputs. Measured on 160,000 random invented books, half of them with shared and blank
GUIDs, the check never returned one; no golden shows one.

**NP-2 and NP-3** take each voucher ref of each `payee_total_` figure.

- `NP-2: <fid> evidence voucher <GUID> is not in the books population` cannot fire: the test cites
  population vouchers only.
- `NP-2: <fid> evidence voucher <GUID> carries no bank leg` is never seen: when the voucher found has
  no bank leg, the reference stops with an internal error straight after forming this message, and
  there is no dump (README section 13).
- `NP-2: <fid> evidence voucher <GUID> narration names payee tag <got>, not <tag>` fires when the
  voucher found belongs to another payee; `<got>` is that payee's tag, or the word `none` when its
  narration gives no payee. `np_shared_guid` has both (dup-a and dup-b).
- `NP-3: voucher <GUID> is evidence for both payee tag <earlier> and <tag>` fires when a figure
  visited earlier cited the same GUID under another tag; `<earlier>` is the tag of the figure that
  cited it last before this one (`np_shared_guid`'s dup-a).

**NP-4 to NP-6** read each row's payments again, with the narration reader alone. A row's payments
here are the vouchers its `payee_total_` figure's voucher refs resolve to, one per ref.

- `NP-4: the printed name <name> is in <n> payee rows (<tags>)`, for each name printed in more than
  one row, names in code-point order; `<name>` is written as Python's `repr()` writes it (the crate's
  `support::py_repr_str`) and `<tags>` are sorted and joined by `, `.
- `NP-5: the whole UPI handle <handle> is in <n> payee rows (<tags>)`, likewise for whole handles;
  `<handle>` is written as Python's `repr()` writes it too, so it stands in quotes (`np_shared_guid`:
  `'BETA.S'`).
- `NP-6: payee row <tag> joins payments that no shared printed name or UPI handle links`, for each
  row of two or more payments that are not all connected. Two payments are linked when they print
  the same name; or both print the same whole handle; or one prints a whole handle and the other a
  cut handle that the whole one starts with, and their two names, spaces removed, are both non-empty
  and one starts with the other. Two cut handles never link, even when they are the same text
  (`np_outside`'s o13 and o14).

NP-4 and NP-5 fire only where vouchers share a GUID (`np_shared_guid`). NP-6 also fires on a book
with no shared GUID, whenever a row was joined through a payment that is not in it: the check sees
only the row's own payments (`np_outside`, four times). Measured on 40,000 random invented books
with a GUID of its own for every voucher, NP-6 was the only message of the check that fired; on
40,000 more in which every payment was of a ledger read, none fired.

**NP-7** takes each `payee_elsewhere_` figure.

- `NP-7: <fid> = <value> but its evidence vouchers debit other ledgers by <its sum>`: the sum of
  the other debits (README section 6.4, the `Contra` rule included) of the vouchers its refs resolve
  to. It fires only where vouchers share a GUID (`np_shared_guid`'s dup-c).

## 9. Book and result checks in the dumps

Every golden carries the book-level and result-level reports of parity spec section 5.

- `np_shared_guid`: POP-5 at book level for each of the five GUIDs two regular vouchers share and
  once for the blank GUID; POP-4 at result level once for every ref that cites a GUID a cancelled
  voucher also carries (four for `dup-e`, one for `dup-f`), subject `narration_payees`.
- `np_names`: MAP-0 at book level for each of the three ledgers with no master; EVID-1 at result
  level, `unresolved ledger:Ghost Advances`, for the `ledger` ref of a debit to a ledger with no
  master.
- REND-0 cannot fire: every fact names a figure of this test.
- Every other report of every edge golden is empty: each book's vouchers tie to its Trial Balance.
- The synthetic dump carries the synthetic read's own four book-level violations, as every synthetic
  golden of the crate does.

The `tds_payees` goldens carry the same book-level reports for the same books, and for
`np_shared_guid` that test's own POP-4 reports.

## 10. Ordering

Figures, findings, facts, evidence and violations are all sorted in a dump (parity spec section 6),
so the order in which the test walks vouchers does not show. What does show:

- the order of names in a row's names shown, of channels, and of tags inside NP-4 and NP-5: sorted
  in code-point order;
- which handle keys a payee: the smallest;
- which voucher of a shared GUID the check reads (the last), and which tag NP-3 names first (README
  section 8).

## 11. What the test never does

- It never says tax was deductible, never computes tax, and never reads whether tax was deducted.
- It never reads a payee from anything but the narration: not the party field, not a ledger name.
- It never reads the narration of a voucher with no bank leg (a cash payment, a journal to a
  creditor), and never reads a receipt.
- It never joins two payees on a likeness: not on a part of a name, not on a cut handle that is not
  completed. Two spellings of one person with no handle in common are two payees.
- It never splits a payee: everyone the bank prints under one name is one row.
- It never adds a debit to another ledger, or a same-day total, into a row's total or its `over`
  answer.
- It never matches a ledger name loosely.

## 12. Not covered by any golden

Reachable in the module but in no book:

- **A voucher of unknown status.** Forming the population refuses before any figure. The edge-book
  shape cannot express an unknown status. The crate's `Book::population` already refuses; propagate
  that error.
- **Rules with no `[s194c]` table.** The reference stops with an internal error on the missing
  table; the TDS payee test, which runs first, stops on the same table, and the crate's port of it
  refuses. The crate's edge-book reader has no `rules_without` arm for this table.
- **A book the TDS payee test refuses.** The run stops before this test, in the reference's pack and
  in the edge harness alike.
- **Two keys with one tag**, or a cut handle whose tag equals another's. The reference stops with a
  duplicate figure id error; the crate's `TestResult::fig` returns `DuplicateFigureId`. Not
  constructed.
- **A ledger named `(payee not named)`.** The id of README section 2.4 then takes that ledger's tag
  on both sides. No book has one.
- **A listed ledger that is itself a bank or cash ledger.** No book lists one.
- **A ledger whose chain is incomplete** (`chain_complete` false): the chain is used as given.
- **The binding refusals of README section 2.5.** They were measured on the reference; the edge
  books go round binding, and the crate does not bind the location yet.
- **The check's internal error** (README section 13), and the eight messages README section 8 says
  cannot be seen (the six of NP-1 and two of NP-2).

Rules no golden can pin, because no input tells the two readings apart:

- in form 4, whether an empty name field fits the form: it gives no payee either way;
- in completing a cut handle, removing the spaces from the cut payment's name: form 4 has already
  removed them.

Boundaries: each limit has a book at the limit and one paisa either side of it (`np_single` for one
payment and for a day's payments, `np_aggregate`, `np_else_agg`, `np_else_single`).

Every rule HASHES.md lists changes at least one golden when it alone is changed (measured by changing
a copy of the reference one rule at a time and running every book again: the rows HASHES.md marks † at
commit `2b329354`, the others at commit `ee17d80f`, before `np_forms` and `np_unread` changed).

## 13. Behaviour that may look like a defect

Reproduce these as the goldens show them, and raise them on the pull request rather than fixing them
in the port.

- **The check stops with an internal error on some shared GUIDs.** When a `payee_total_` figure
  cites a GUID whose last population voucher has no bank leg, the check forms the "carries no bank
  leg" message and then fails looking that voucher up among the outgoing bank payments; the run
  produces no dump. Smallest case measured: two regular vouchers with one GUID, the first a bank
  payment of a listed ledger with a payee, the second a journal. `np_shared_guid` keeps the last
  voucher of every GUID a bank payment for this reason. Ask before choosing what a port does there.
- **NP-6 fires on a correct row** whenever the join ran through a payment outside the row
  (`np_outside`): the row is as the rules of README section 5 make it, and the check, which reads
  the row alone, cannot see the link.
- **The check and the test disagree on shared GUIDs** (`np_shared_guid`): the test tells payments
  apart by voucher, the check finds them by GUID, so NP-2 to NP-5 and NP-7 fire on a book defect
  that POP-5 already reports. At this commit the reference's own reader refuses a read in which two
  vouchers share a GUID or a voucher has none, before any test runs, so its real pipeline reaches
  neither this nor the internal error above; the edge harness builds the book directly and does.
- **A payment's amount is the whole debit**, also when only part of it was paid by bank (`np_rows`'
  r08), and a credit on the same ledger in the same voucher does not reduce it (r02).
- **A channel word is no name only in forms 1 to 3, and only as the whole name**: in forms 4 and 5
  `UPI` is the payee `UPI` (`np_forms`' f61 and f62), and a name such as `UPI.`, with its full stop, is
  a name in every form (no book has one).
- **Two payees can show one name.** A name written with a combining accent and the same name
  written composed are two keys and two rows, and the dump normalises both names to the same text
  (`np_text`'s t29 to t31). Only form 4 can read such a name.
- **A row can be keyed by a handle none of its payments prints**, and joined by a payment that is
  not in it (`np_outside`).
- **A citation's label keeps an empty number as it is**, unlike the label other tests build (README
  section 6.6).
- **A debit to a ledger with no master counts as a debit to another ledger** and its `ledger` ref
  then fails EVID-1 (`np_names`).
- **A same-day row and an over row are independent**: a payee over the aggregate limit is listed
  again for a day's pair (`np_same_day`'s d08 to d11).
- **The added ledgers follow GUIDs, not vouchers** (README section 2.4, `np_shared_guid`'s dup-e).
- **The bank ledgers and the bank groups are two tests.** A bank payment is one that credits a
  ledger of the set; a debit is left out of the other debits when its ledger is in the set or under
  a bank group. In the real pipeline the set is the ledgers under `[roles].bank_groups`, so the two
  tests differ only when that list is not exactly `Bank Accounts` and `Bank OD A/c`.

## 14. The books

All are invented: synthetic ledger names, payee names from the Greek alphabet, from trees or built
from the reader's own words, made-up bank-style tokens, nothing read from Tally or from a statement.
Each book's `comment` says what it reaches, voucher by voucher. Small books use amounts under
100,000 paise with irregular paise; the books at a limit use multiples of 10,000 paise and the limit's
own neighbours.
Every book also runs `tds_payees`, whose golden is beside it.

| Book | Reaches |
| --- | --- |
| `np_empty` | No voucher: the fifteen figures, no finding; golden byte-identical to `np_quiet`'s. |
| `np_quiet` | Twelve near misses, nothing read: optional, cancelled and post-dated payments; a ledger not read; a receipt; a nil line; a ledger not listed; 194C payments pooled exactly at both limits (nothing added); a named payee over the limit in the TDS payee test (nothing added). |
| `np_forms` | Every form that names a payee, with its variants: case, spacing, optional spaces, one-character names and codes, the closing character last, the cheque prefixes, names near the placeholders, the refused beginnings and the channel words, text that only resembles the prefix; a name joined across two channels; the channel word UPI as a name in the second UPI layout and on a cheque. Twenty-three payees. |
| `np_unread` | Sixty-six narrations that name no one: one per rule of the five forms, a channel word as the name, the cheque prefix with no name after it; the two placeholders; an empty, an absent and a blank narration. No payee row. |
| `np_text` | Upper-casing that lengthens or changes a letter; letters and digits of other scripts; Python whitespace and characters that are not whitespace; two spellings of one accented name; labels with another voucher type, an empty number, a number of spaces and combining marks. |
| `np_handles` | What is and is not a handle in both UPI layouts; one handle under two names, one name on two handles, a chain; the key's smallest handle; a receipt that joins nothing. Thirty-six payees. |
| `np_cut` | Cut handles: completed (each way the names can agree, an equal handle, a whole handle from the second layout) and not completed (no candidate, two, one under another name); the count and the four groups; a cut payment outside the rows. |
| `np_outside` | Joins, a key and completions that come from payments of a ledger not read; a payee with no row; NP-6 four times. |
| `np_single` | The single-sum limit at, one over and one under, for one payment and for a day's payments. |
| `np_aggregate` | The aggregate limit at, one over and one under. |
| `np_same_day` | Same-day rows: one payment, two days, two rows for one payee, a row that is also over, two names joined, two payees, and second payments that do not count. Three findings. |
| `np_elsewhere` | Which debits are debits to other ledgers: one voucher per rule, the bank set against the bank groups, a Contra, a nil line, a credit, a payee with no row, a ledger name with a combining mark. |
| `np_else_agg` | Crossing by the aggregate at, one over and one under; a row already over; two lines of one voucher against two vouchers. The elsewhere finding. |
| `np_else_single` | Crossing by the single-sum limit at, one over and one under. |
| `np_rows` | What makes a bank payment and its amount: two ledgers read, a credit not set off, an overdraft ledger outside the set, a journal, cash, a nil bank line, a bank debit, part by bank, a Journal and a Contra with a bank credit, a date after the period; a ledger listed twice. |
| `np_added` | Ledgers added from the TDS payee test's finding, and each way a 194C-mapped ledger stays out; a ledger both listed and added. |
| `np_names` | Listed and bank names with no master, or differing in case or by a space; a debit to a ledger with no master. MAP-0 and EVID-1. |
| `np_shared_guid` | Vouchers sharing a GUID and with none: rows by voucher, citations by GUID and label, a ledger added through a shared GUID, POP-5 and POP-4; NP-2 (both forms), NP-3, NP-4, NP-5 and NP-7. |

## 15. Running the books

The crate's `tests/edge_books.rs` does not run this test yet. A port adds that, for each book naming
`narration_payees`:

1. read `narration_payee_ledgers` (absent meaning none; anything but a list of text refused) and the
   `bank` list, as sets;
2. run the TDS payee port on the book as the existing `tds_payees` arm does (the entity type, with
   `tds_config(&s)` and `tds_inputs(&s)`), and work out the added ledgers from its result and
   `nature_by_ledger` (README section 2.4);
3. run the test with the book, the rules, the bank ledgers, the listed set and the added set, and
   its module check with the book, the result, the bank ledgers and the two sets together;
4. compare the whole dump with `edge.<book>.narration_payees.json`, as the other edge books do
   (parity spec section 7).

Every book also names `tds_payees` in `tests`, so the existing arm compares the `tds_payees` goldens
too, once the books and goldens are copied into the crate's fixtures. Copied files need byte rows
there as well: `tests/provenance_rows.rs` requires one four-cell row per golden and edge book in a
Markdown file under `tests/fixtures` (`PROVENANCE.md` or a batch file under `provenance/`), with the
crate-relative path (`edge-books/np_forms.json`, `golden/edge.np_forms.narration_payees.json`); the
byte and SHA-256 cells can be copied from HASHES.md, whose own rows, under `docs/`, do not count.

The reference's side of the edge harness, `parity/edge_golden.py`, is in this repository, but it has
no runner for this test yet; README section 16 gives the lines. A porter does not regenerate goldens:
that is done only by the reference's maintainers.

## 16. Registering the test in the crate

A new registry entry needs, besides the books, a golden for the synthetic read, a `min_figures`
value, the configuration key with its binding, and the two harness lines.

- **The synthetic golden.** `goldens/synthetic.narration_payees.json` is the test on the crate's
  synthetic read (`tests/fixtures/synthetic-engagement.toml`), with the bank ledgers of its
  `bank_groups`, no ledger listed (the engagement has no `narration_payee_ledgers` key) and nothing
  added (the TDS payee test's result on that read, the crate's committed `synthetic.tds_payees.json`,
  has no finding for payments with no payee named). It has the fifteen figures at 0, no finding, no
  violation of the module check, and the read's four book-level violations. It goes to
  `tests/fixtures/golden/synthetic.narration_payees.json`. It is not an empty dump, so the
  comparer's refusal of two empty results is not tripped. Until it is in `tests/fixtures/golden`,
  the crate's registry test that every registered test has a synthetic golden fails.
- **`min_figures`**: 15. The test publishes those fifteen figures on every book, so fewer is a broken
  dump (the minimum figure count of parity spec section 7; the steps of adding a test are parity
  spec section 10).
- **The configuration key.** `Engagement` gains `[roles].narration_payee_ledgers` (optional, a list
  of ledger names) and `src/binding.rs` gains the location `roles.narration_payee_ledgers`, bound as
  a list of ledger names (README section 2.5), so that `BIND-ID-UNUSED` also counts a label used
  only there.
- **The registry entry** takes no caller data. A `narration_payees_on` binds the engagement, takes
  the bank ledgers from the bound `bank_groups`, reads the bound list, runs the TDS payee port as
  `tds_payees_on` does (so it needs `[client].entity_type` and a `[tds]` table and refuses without
  them, as the reference's pack does), works out the added ledgers, then runs this test and its
  check.
- **The evidence kind.** `payee_name` is new to the crate. Parity spec section 11 already says its
  id is a name; the sentence there that the crate does not emit it yet goes when the test is ported.
- **The harness lines** (the reference's maintainers keep and run the harness; these are the lines a
  port adds so that its harness names the test). In `parity/python_golden.py`, a runner and its
  `RUNNERS` entry (the list is kept sorted by test id):

      def _narration_payees(c):
          from tae.audit_tests import narration_payees
          from tae.config import tds_config
          configured = frozenset(c.cfg.get("roles", {}).get("narration_payee_ledgers", []))
          added = narration_payees.unnamed_194c_ledgers(c.eng.book, _tds_payees(c)[1], tds_config(c.cfg)[0])
          module = SimpleNamespace(TEST_ID=narration_payees.TEST_ID, check_invariants=lambda eng, result:
                                   narration_payees.check_invariants(eng, result, c.bank, configured | added))
          return module, narration_payees.run(c.eng, c.rules, c.bank, configured, added_ledgers=added)

      "narration_payees": _narration_payees,

  In `parity/edge_golden.py`, a runner beside `tds_payees_run` and its entry in the `runners` table:

      def narration_payees_run():
          from tae.audit_tests import narration_payees
          configured = frozenset(typed(spec, "narration_payee_ledgers",
                                       lambda x: isinstance(x, list) and all(isinstance(t, str) for t in x),
                                       "a list of text", absent=[], nullable=False))
          added = narration_payees.unnamed_194c_ledgers(book, tds_payees_run()[1], dict(spec.get("nature_by_ledger", {})))
          module = SimpleNamespace(TEST_ID=narration_payees.TEST_ID, check_invariants=lambda e, res:
                                   narration_payees.check_invariants(e, res, bank, configured | added))
          return module, narration_payees.run(eng, rules, bank, configured, added_ledgers=added)

      "narration_payees": narration_payees_run,

  Both import the test from the reference's audit tests and run the TDS payee runner of the same
  file first, as the reference's pack runs that test first and passes its result on; both wrap the
  module so that the canonical dump can call its check with the bank ledgers and the ledgers read,
  as `_party_monthly` and the `party_monthly` entry do for theirs. A port only needs them to keep
  the two sides' test lists in step (the crate has tests that compare the Rust registry and the edge
  dispatch with the harness's lists), and `parity/edge_golden.py`'s list of spec keys gains
  `narration_payee_ledgers`.
