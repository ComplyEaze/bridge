# Hashes and provenance: the `narration_payees` spec pack

Each book here is invented, with synthetic ledger names, payee names taken from the Greek alphabet,
from trees or built from the reader's own words, and made-up bank-style tokens: no book is a Tally
read of any real assessee, no narration is a line of any bank statement, and no figure or name comes
from either.

## What these files establish, and what they do not

- The books are hand-written scenarios, each built to reach a rule or boundary of the reference
  engine's `narration_payees` test and of the narration reader it uses (the README lists which, and
  each book's `comment` says so voucher by voucher). They are regression fixtures: a port that
  reproduces every golden agrees with the reference on these books, and nothing more.
- `np_empty` and `np_quiet` give byte-identical `narration_payees` goldens: with nothing read the
  dump does not depend on the book. They differ in why nothing is read (no voucher at all, against
  twelve near misses).
- The eighteen `edge.<book>.tds_payees.json` goldens are the TDS payee test's result on each book,
  made in the same run. They are the input this test was given; they add nothing to the TDS payee
  test's own contract, which is already ported and has its own goldens in the crate. 13 of them
  (`np_aggregate`, `np_cut`, `np_else_agg`, `np_else_single`, `np_empty`, `np_forms`, `np_handles`, `np_outside`, `np_rows`, `np_same_day`, `np_single`, `np_text`, `np_unread`) are byte-identical: on a book with no ledger mapped to a TDS nature
  and clean book checks that test's dump does not depend on the vouchers.
- `np_shared_guid` and `np_names` are books the reference's real pipeline does not let through: its
  reader refuses a read in which two vouchers share a GUID or a voucher has none, and its binding
  refuses a listed name that matches no ledger. They show what the test and its check do when such
  a book is built directly, as the edge harness builds it.
- They prove nothing about what a bank prints. The narrations are built from the reader's rules to
  reach each of them; they pin how the reference reads a text, not which texts banks produce, how
  often a payee can be read from a real narration, or how often two people share a printed name.
- They prove nothing about reading Tally. The books are built directly in the edge-book shape, not
  read from a company.
- Agreement on real books is not established by this pack. It will be established separately, once
  the test is ported, by a local parity run on real reads that is never committed.
- The goldens carry, as every dump does, short hashes of the reference's fixed texts
  (`definition_sha256_16` and the like). Those are hashes of sentences, not figures.

## Which rules the goldens pin

Each rule below was changed alone, in a copy of the reference test or of its narration reader held in
memory (the reference itself was not edited), and every book was run again; the books named are those
whose `narration_payees` golden then changed. The unchanged copies reproduced every golden first. No
`tds_payees` golden changed under any row. A port that gets one of these rules wrong fails at least one
golden. 226 rules are listed. The 26 rows marked † (the cheque prefix, the two refused cheque
beginnings, a channel word in a name's place, and two older rows whose books this change moved) were
measured at commit `2b329354` on the books as they are now; every other row was measured at commit
`ee17d80f`, before `np_forms` and `np_unread` changed (README sections 3.1 and 3.2), and was not run
again.

### Reading a name out of a narration

| Rule changed | Goldens that change |
| --- | --- |
| narration not upper-cased | `np_forms`, `np_handles`, `np_text`, `np_unread` |
| whitespace runs not collapsed | `np_forms`, `np_text` |
| only ASCII whitespace collapsed | `np_text` |
| first UPI layout found anywhere in the narration, not only at its start † | `np_text`, `np_unread` |
| IMPS found anywhere in the narration | `np_unread` |
| NEFT found anywhere in the narration | `np_unread` |
| second UPI layout found anywhere in the narration | `np_unread` |
| cheque found anywhere in the narration | `np_text`, `np_unread` |
| only the letters a to z upper-cased | `np_text` |
| UPI and IMPS name: a space before the closing hyphen kept † | `np_forms`, `np_unread` |
| NEFT name: a space before the closing hyphen kept | `np_forms` |
| UPI name: an underscore allowed | `np_unread` |
| UPI name: digits not allowed | `np_forms` |
| UPI name: a full stop not allowed | `np_forms` |
| UPI name: an ampersand not allowed | `np_forms` |
| UPI name: an apostrophe not allowed | `np_forms` |
| UPI name: a space not allowed | every book but `np_empty`, `np_quiet`, `np_unread` |
| UPI name: may start with a digit | `np_unread` |
| UPI name: may start with a space | `np_unread` |
| UPI name: needs two characters | `np_forms`, `np_handles` |
| UPI name: no closing hyphen needed | `np_unread` |
| UPI name: digits of any script | `np_text` |
| IMPS name: digits of any script | `np_text` |
| NEFT: digits of any script in the code | `np_text` |
| handle: digits of any script before the suffix | `np_handles` |
| UPI name: any letter of any script | `np_text`, `np_unread` |
| UPI: a space allowed before the first hyphen | `np_unread` |
| IMPS: ASCII digits only | `np_text` |
| IMPS: any numeric character | `np_text` |
| IMPS: the reference may be empty | `np_unread` |
| IMPS: spaces allowed in the reference | `np_unread` |
| IMPS: letters allowed in the reference | `np_unread` |
| IMPS: a space in place of the first hyphen | `np_unread` |
| IMPS name: may start with a digit | `np_unread` |
| IMPS name: no closing hyphen needed | `np_unread` |
| NEFT: the space before DR required | `np_forms` |
| NEFT: RTGS not read | `np_forms` |
| NEFT: CR read too | `np_unread` |
| NEFT: DR not needed | `np_unread` |
| NEFT: a space allowed before the hyphen | `np_unread` |
| NEFT: any word in place of NEFT | `np_unread` |
| NEFT: the code may be empty | `np_unread` |
| NEFT: a space allowed in the code | `np_unread` |
| NEFT: the code skipped | `np_unread` |
| NEFT name: no closing hyphen needed | `np_unread` |
| NEFT and RTGS reported as one channel | `np_forms` |
| first UPI layout and IMPS: a channel word read as a name † | `np_unread` |
| NEFT and RTGS: a channel word read as a name † | `np_unread` |
| channel words: UPI not one † | `np_unread` |
| channel words: IMPS not one † | `np_unread` |
| channel words: NEFT not one † | `np_unread` |
| channel words: RTGS not one † | `np_unread` |
| first UPI layout and IMPS: a name that begins with a channel word refused † | `np_forms` |
| NEFT and RTGS: a name that begins with a channel word refused † | `np_forms` |
| first UPI layout and IMPS: the channel word compared before the space before the hyphen is removed † | `np_unread` |
| second UPI layout: no space allowed after TO TRANSFER- | `np_forms` |
| second UPI layout: a space allowed before the hyphen | `np_unread` |
| second UPI layout: CR read too | `np_unread` |
| second UPI layout: any channel word | `np_unread` |
| second UPI layout: no spaces in the reference | `np_cut`, `np_forms`, `np_handles`, `np_outside`, `np_text`, `np_unread` |
| second UPI layout: ASCII digits only in the reference | `np_text` |
| second UPI layout: the reference may be empty | `np_unread` |
| second UPI layout: the reference needs a digit | `np_forms` |
| second UPI layout: letters allowed in the reference | `np_unread` |
| second UPI layout: no closing slash needed | `np_unread` |
| second UPI layout: spaces kept in the name | `np_cut`, `np_forms`, `np_handles`, `np_unread` |
| second UPI layout: Bank Acc read as a payee | `np_unread` |
| second UPI layout: PhonePe read as a payee | `np_unread` |
| second UPI layout: a name starting with a placeholder is one | `np_forms` |
| a placeholder reported as not read at all | `np_unread` |
| the placeholders applied to the first UPI layout too | `np_forms` |
| second UPI layout: a name with no letter read | `np_text`, `np_unread` |
| second UPI layout: a letter of any script is enough | `np_text` |
| second UPI layout: a name with a character outside ASCII not read | `np_text` |
| second UPI layout: every character must be a letter | `np_forms`, `np_text` |
| second UPI layout: a channel word refused as the name too † | `np_forms` |
| cheque: the WITHDRAWAL BY prefix kept as part of the name † | `np_forms`, `np_unread` |
| cheque: only the full spelling WITHDRAWAL † | `np_forms` |
| cheque: the prefix must be followed by a space † | `np_unread` |
| cheque: the prefix may be followed by a letter † | `np_forms` |
| cheque: a full stop after the prefix taken as part of a word † | `np_unread` |
| cheque: read again from the start when nothing readable follows the prefix † | `np_unread` |
| cheque name: digits allowed | `np_unread` |
| cheque name: an ampersand allowed | `np_unread` |
| cheque name: an apostrophe allowed | `np_unread` |
| cheque name: a full stop not allowed | `np_forms` |
| cheque name: may start with a full stop | `np_unread` |
| cheque name: a hyphen allowed (the last one before CHQ PAID ends it) | `np_unread` |
| cheque name: needs two characters | `np_forms` |
| cheque name: any letter of any script | `np_forms`, `np_text` |
| cheque: spaces required round the hyphen | `np_forms` |
| cheque: the hyphen not needed | `np_unread` |
| cheque: CHQ PAID must end a word | `np_forms` |
| cheque: CHQ and any word | `np_unread` |
| cheque: a name starting with SELF read † | `np_unread` |
| cheque: a name starting with CASH PAID TO read † | `np_unread` |
| cheque: only the exact names SELF and CASH PAID TO refused † | `np_unread` |
| cheque: SELF and CASH PAID TO refused by their first characters, not as words † | `np_forms` |
| cheque: CASH PAID refused without TO † | `np_forms` |
| cheque: SELF refused anywhere in the name † | `np_forms` |
| cheque: a full stop after SELF or CASH PAID TO taken as part of a word † | `np_unread` |
| cheque: a channel word refused as the name too † | `np_forms` |
| cheque channel named CHEQUE | `np_forms` |

### Reading a UPI handle

| Rule changed | Goldens that change |
| --- | --- |
| handle: an underscore not allowed | `np_handles` |
| handle: a full stop not allowed | `np_cut`, `np_elsewhere`, `np_handles`, `np_outside`, `np_same_day`, `np_shared_guid` |
| handle: digits not allowed | `np_cut`, `np_handles`, `np_outside` |
| handle: a space allowed | `np_handles` |
| handle: an ampersand allowed | `np_handles` |
| handle: any letter of any script | `np_handles` |
| handle: no -digits suffix | `np_handles` |
| handle: the suffix dropped from the handle | `np_handles` |
| handle: several -digits suffixes | `np_handles` |
| handle: a suffix of letters | `np_handles` |
| handle: ASCII digits only in the suffix | `np_handles` |
| handle: may be empty | `np_handles` |
| handle: the full-width @ read | `np_handles` |
| handle: something must follow the @ | `np_handles` |
| handle: no @ needed | `np_added`, `np_aggregate`, `np_else_agg`, `np_else_single`, `np_elsewhere`, `np_forms`, `np_handles`, `np_names`, `np_outside`, `np_rows`, `np_same_day`, `np_shared_guid`, `np_single`, `np_text` |
| second layout handle: the third field read | `np_cut`, `np_forms`, `np_handles`, `np_outside` |
| second layout handle: the bank field may not be empty | `np_handles` |
| second layout handle: no closing slash needed | `np_handles` |
| second layout handle: spaces kept | `np_cut`, `np_handles` |
| second layout handle: the text after the @ kept | `np_cut`, `np_handles` |
| second layout handle: cut at the last @ | `np_handles` |
| second layout handle: always cut short | `np_cut` |
| second layout handle: always whole | `np_cut`, `np_outside` |
| second layout handle: an empty one kept | `np_handles` |

### The ledgers added from the TDS payee test's result

| Rule changed | Goldens that change |
| --- | --- |
| added ledgers: any nature mapped | `np_added` |
| added ledgers: a credit line adds its ledger too | `np_added` |
| added ledgers: every 194C-mapped ledger of the books, with the finding | `np_added` |
| added ledgers: every 194C-mapped ledger of the books, finding or not | `np_added`, `np_quiet` |
| added ledgers: any 194C finding of the TDS payee test | `np_quiet` |
| added ledgers: vouchers outside the population too | `np_shared_guid` |
| added ledgers: only a cited voucher that itself credits cash or bank | `np_shared_guid` |
| added count: a listed ledger counted again | `np_added` |
| added ledgers not read | `np_added`, `np_shared_guid` |

### Which vouchers are read, and a payee's row

| Rule changed | Goldens that change |
| --- | --- |
| every voucher read, not the population | `np_elsewhere`, `np_quiet`, `np_shared_guid` |
| a payment's amount is its net on the ledgers read | `np_rows` |
| a payment's amount is its largest debit line | `np_rows` |
| a payment's amount is its bank credit | `np_added`, `np_elsewhere`, `np_rows`, `np_shared_guid` |
| a voucher with a nil debit read | `np_quiet`, `np_rows` |
| a bank debit counts as a bank leg | `np_handles`, `np_rows` |
| a nil bank line counts as a bank leg | `np_rows` |
| a bank ledger is one under a bank group, not one given | `np_added`, `np_elsewhere`, `np_names`, `np_rows` |
| a voucher with no bank leg dropped | `np_added`, `np_names`, `np_rows`, `np_same_day`, `np_shared_guid` |
| a placeholder left out of the unread total | `np_unread` |
| over: the single-sum limit reached, not passed | `np_aggregate`, `np_single` |
| over: the aggregate limit reached, not passed | `np_aggregate` |
| over: both limits needed | `np_added`, `np_aggregate`, `np_else_agg`, `np_rows`, `np_same_day`, `np_shared_guid`, `np_single` |
| over: the single-sum limit alone | `np_aggregate`, `np_same_day` |
| over: the aggregate limit alone | `np_added`, `np_else_agg`, `np_rows`, `np_same_day`, `np_shared_guid`, `np_single` |
| over: the total tested against the single-sum limit | `np_aggregate`, `np_else_agg`, `np_same_day`, `np_single` |
| the row tag hashes the names shown, not the key | `np_cut`, `np_elsewhere`, `np_handles`, `np_outside`, `np_same_day`, `np_shared_guid` |
| the names shown are not sorted | `np_cut`, `np_handles`, `np_outside`, `np_same_day` |
| the names shown joined by a comma | `np_cut`, `np_handles`, `np_outside`, `np_same_day` |
| the channels not sorted | `np_forms`, `np_handles`, `np_same_day` |
| the payee total cites no voucher | every book but `np_empty`, `np_quiet`, `np_unread` |
| the unread finding only when a placeholder was printed | `np_rows`, `np_same_day`, `np_shared_guid`, `np_text` |
| the unread finding always raised | `np_added`, `np_aggregate`, `np_cut`, `np_else_agg`, `np_else_single`, `np_elsewhere`, `np_empty`, `np_forms`, `np_handles`, `np_names`, `np_outside`, `np_quiet`, `np_single` |
| the over finding always raised | `np_cut`, `np_else_single`, `np_elsewhere`, `np_empty`, `np_forms`, `np_handles`, `np_names`, `np_outside`, `np_quiet`, `np_text`, `np_unread` |

### Joining payments into payees

| Rule changed | Goldens that change |
| --- | --- |
| a payee with a handle keyed by its largest handle | `np_handles` |
| a payee with a handle keyed without the upi: prefix | `np_cut`, `np_elsewhere`, `np_handles`, `np_outside`, `np_same_day`, `np_shared_guid` |
| a payee with a handle keyed by its name | `np_cut`, `np_elsewhere`, `np_handles`, `np_outside`, `np_same_day`, `np_shared_guid` |
| payments never joined by a handle | `np_cut`, `np_elsewhere`, `np_handles`, `np_outside`, `np_same_day`, `np_shared_guid` |
| payments never joined by a name alone | `np_cut`, `np_elsewhere`, `np_handles`, `np_outside`, `np_same_day`, `np_shared_guid` |
| receipts join too | `np_handles` |
| only payments on the ledgers read join | `np_else_agg`, `np_else_single`, `np_elsewhere`, `np_names`, `np_outside`, `np_same_day` |
| a cut handle joins without an agreeing name | `np_cut` |
| a cut handle joins the first of several full handles | `np_cut` |
| a cut handle joins only when the full handle's name starts with the cut name | `np_cut` |
| a cut handle joins only when the cut name starts with the full handle's name | `np_cut`, `np_outside` |
| a cut handle joins only when the two names are equal | `np_cut`, `np_outside` |
| the full handle's names compared with their spaces | `np_cut`, `np_outside` |
| a cut handle must be shorter than the full one | `np_cut` |
| a cut handle completed by a full handle that holds it anywhere | `np_cut` |

### Same day

| Rule changed | Goldens that change |
| --- | --- |
| same day: one payment is enough | `np_added`, `np_else_agg`, `np_rows`, `np_same_day`, `np_shared_guid`, `np_single` |
| same day: three payments needed | `np_same_day` |
| same day: the limit reached, not passed | `np_single` |
| same day: tested against the aggregate limit | `np_same_day`, `np_single` |
| same day: a payee already over is left out | `np_same_day` |
| same day: a payee counted once however many days | `np_same_day` |
| same day: the finding always raised | every book but `np_same_day`, `np_single` |

### Debits to other ledgers

| Rule changed | Goldens that change |
| --- | --- |
| elsewhere: a contra counted | `np_elsewhere`, `np_shared_guid` |
| elsewhere: the ledgers read counted too | every book but `np_empty`, `np_quiet`, `np_unread` |
| elsewhere: a bank ledger given counted | `np_elsewhere` |
| elsewhere: credits counted too | `np_added`, `np_elsewhere` |
| elsewhere: a Bank Accounts ledger counted | `np_elsewhere` |
| elsewhere: a Bank OD A/c ledger counted | `np_elsewhere` |
| elsewhere: a Cash-in-Hand ledger counted | `np_elsewhere` |
| elsewhere: a Duties & Taxes ledger counted | `np_elsewhere` |
| elsewhere: a ledger with no master left out | `np_names` |
| elsewhere: a payee with no row listed | `np_added`, `np_cut`, `np_elsewhere`, `np_names`, `np_outside`, `np_quiet` |
| elsewhere: the register's own vouchers left out | `np_added`, `np_elsewhere`, `np_shared_guid` |
| elsewhere: a payee over a limit can cross again | `np_else_agg` |
| elsewhere: the aggregate limit reached, not passed | `np_else_agg` |
| elsewhere: the single-sum limit reached, not passed | `np_else_agg`, `np_else_single` |
| elsewhere: the other debits alone tested against the aggregate | `np_else_agg` |
| elsewhere: all the other debits together tested against the single-sum limit | `np_else_agg` |
| elsewhere: a single line tested, not a voucher | `np_else_agg` |
| elsewhere: no aggregate test | `np_else_agg` |
| elsewhere: no single-sum test | `np_else_agg`, `np_else_single` |
| elsewhere: the finding cites only the other payments | `np_else_agg`, `np_else_single` |
| elsewhere: the ledgers debited not cited | `np_added`, `np_else_agg`, `np_else_single`, `np_elsewhere`, `np_names`, `np_outside`, `np_same_day`, `np_shared_guid` |

### Cut handles that were not completed

| Rule changed | Goldens that change |
| --- | --- |
| cut handles: a payment outside the rows counted | `np_cut` |
| cut handles: a group of one listed | `np_cut` |
| cut handles: a group needs three | `np_cut` |
| cut handles: the full handles' payees left out of a group | `np_cut` |
| cut handles: the group tag hashes upi: and the handle | `np_cut` |
| cut handles: the finding always raised | every book but `np_cut` |

### Citations

| Rule changed | Goldens that change |
| --- | --- |
| a citation's label replaces an empty number by the GUID | `np_text` |
| a citation's label uses the base type | `np_text` |
| a citation's label trims the number | `np_text` |
| one citation per GUID, whatever the label | `np_shared_guid` |
| one citation per voucher, even when two are alike | `np_shared_guid` |

### The module's own check

| Rule changed | Goldens that change |
| --- | --- |
| no module check | `np_outside`, `np_shared_guid` |
| check: no NP-2 on the payee a citation names | `np_shared_guid` |
| check: no NP-3 | `np_shared_guid` |
| check: no NP-4 | `np_shared_guid` |
| check: no NP-5 | `np_shared_guid` |
| check: no NP-6 | `np_outside` |
| check: NP-6 links a cut handle to its full one whatever the names | `np_outside` |
| check: NP-6 never links a cut handle to a full one | `np_cut` |
| check: NP-6 links two cut handles that are alike | `np_outside` |
| check: NP-6 does not link by a shared name | `np_aggregate`, `np_cut`, `np_else_agg`, `np_forms`, `np_handles`, `np_rows`, `np_same_day`, `np_shared_guid`, `np_single`, `np_text` |
| check: no NP-7 | `np_shared_guid` |
| check: NP-7 counts a contra | `np_shared_guid` |

Any change to a fixed text (a title, a limit, an item to ask, a definition, the population note), a
clause list, a confidence or a fact name changes every golden that carries it.

Two further changes were tried and move no golden, because no input tells the two readings apart
(README section 12): letting the name field of the second UPI layout be empty, and keeping the spaces
in a cut payment's name when a cut handle is completed.

## How the goldens were produced

At the reference engine (a private repository), commit `2b329354`, under Python 3.13, with the
crate's `parity/edge_golden.py`, which is in this repository under its Apache-2.0 licence and has no
runner for this test. The goldens were made with that file extended by the runner README section 16
gives as text; the extended file is held by the reference's maintainers and is not part of this
pack. The runner reads `narration_payee_ledgers` and `bank` from the book, runs the file's own
`tds_payees` runner on the book and works the added ledgers out of its result, as the reference's
pack does, then runs the test and hands the canonical dump a check bound to the bank ledgers and the
ledgers read. The `tds_payees` goldens come from the file's existing `tds_payees` runner, because
every book also names `tds_payees`. Run from `src-tauri/crates/bridge-tax-audit`, once per book:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python EDGE_GOLDEN_WITH_RUNNER ENGINE \
        BOOK.json OUTDIR

Each run writes `OUTDIR/edge.<book>.tds_payees.json` and `OUTDIR/edge.<book>.narration_payees.json`.
Running every book a second time, into a new directory, reproduced every golden byte for byte. The pack
was first made at commit `ee17d80f`. Commit `2b329354` changed the narration reader (README sections
3.1 and 3.2), and `np_forms` and `np_unread` were rebuilt for it; at `2b329354` the other 35 goldens are
byte-identical to those first made. As a
control, the extended file reproduced the crate's committed `edge.tds_payees_goods_and_cash.tds_payees.json`
and `edge.cc_shared_guid.counter_cheques_40a3.json` byte for byte.

`synthetic.narration_payees.json` was made with the crate's `parity/python_golden.py` extended by the
runner README section 16 gives, with
`ENGINE tests/fixtures/synthetic-engagement.toml OUT.json --test narration_payees`; made twice, it was
byte-identical both times, and the same extended file reproduced the crate's committed
`synthetic.tds_payees.json` byte for byte.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `np_added.json` | 6,864 | `c90b4eb8faeeba038dbe96cb9e5ff6a32ed35e92f2bb5e151ffd4940b48ab527` | `books/np_added.json` |
| `np_aggregate.json` | 4,706 | `a594cf05912d84734d8bb74fa5f0f251c1b01b81490b51beafe57d7e3835e1b0` | `books/np_aggregate.json` |
| `np_cut.json` | 8,759 | `e56d8d7e365c5fdffc3feb52490524f78525cc1d08a5542f2e85bdeb8812cff1` | `books/np_cut.json` |
| `np_else_agg.json` | 8,012 | `0dfb69b696bfe51698378e55b075f73e7351b1e5e9484fbacc2cabc914420e10` | `books/np_else_agg.json` |
| `np_else_single.json` | 3,334 | `7f478c33d09089d054036ece23c2f8399dd06dd621509fd9785dc90c36b8ff2b` | `books/np_else_single.json` |
| `np_elsewhere.json` | 10,061 | `e982da7454037ac9d01def4416c358ba37104e2ebf6a33c6500918de28e7f930` | `books/np_elsewhere.json` |
| `np_empty.json` | 1,890 | `24bda688017fc655b0c6bb08634f782023f2a8cef25d95b2e1ff01d265c9b71d` | `books/np_empty.json` |
| `np_forms.json` | 14,738 | `7383987cfe5cad2323c50ad670d7a9d7d8fb7b71ea13c9abf695bad82ddd417f` | `books/np_forms.json` |
| `np_handles.json` | 14,592 | `3e5f06e621ec6da32da26e6618c0b338729a81c63aecb9cfe0908d8da399480d` | `books/np_handles.json` |
| `np_names.json` | 4,224 | `314657c4f0d8fc67745f1c87ccc53e2e5cee72a0d62bab9d61d425cf8e5e7339` | `books/np_names.json` |
| `np_outside.json` | 6,097 | `62ee16b431d9e3cc9601b50007d8cbae361e1e916d774a8a7c1421f9a530c815` | `books/np_outside.json` |
| `np_quiet.json` | 6,483 | `3ce716b16d8f9b636ee94ec5234e284fa3b3cd2305187c3d2cc7002f271440fb` | `books/np_quiet.json` |
| `np_rows.json` | 6,739 | `111937d1af6248c3fde224457ec1ef9cc243381ba7e66f04851ff7cf0efaf717` | `books/np_rows.json` |
| `np_same_day.json` | 7,138 | `5ca0b7db3b2c2e76587ef42fa6e1444d6b9ab8de2fab4071ed7c1c44d4e2753e` | `books/np_same_day.json` |
| `np_shared_guid.json` | 7,865 | `1054b54261cb8285fdb1aa69a79afe44078ae2b1cd8a91e0e30da00789f614e1` | `books/np_shared_guid.json` |
| `np_single.json` | 4,190 | `9f3e0290c19d31fea979348e2264df6b5d72c3c8b8dda8bf90f81475bc83c189` | `books/np_single.json` |
| `np_text.json` | 11,905 | `51aafced4b7f66c6c3329caa36d215702d534df6d6ddbc8288dab3521bb91653` | `books/np_text.json` |
| `np_unread.json` | 17,781 | `d5abfdf691c70b40de67f33460e605b61f1bfd3ca75add2d0f3f133183336577` | `books/np_unread.json` |
| `edge.np_added.narration_payees.json` | 17,492 | `06c065fc9c84fe4553117d708bfe5e0d2701bc5265b275c5ef62be85b4544e2a` | `goldens/edge.np_added.narration_payees.json` |
| `edge.np_added.tds_payees.json` | 23,458 | `c9f268881dc9ccd667f1d7428fca26634ec2879d6069c9d98b26719f827bdb0b` | `goldens/edge.np_added.tds_payees.json` |
| `edge.np_aggregate.narration_payees.json` | 16,499 | `609c3ada203169cec24b8619b79ec21f90e8d34af4d303ca6614005de65e0463` | `goldens/edge.np_aggregate.narration_payees.json` |
| `edge.np_aggregate.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `goldens/edge.np_aggregate.tds_payees.json` |
| `edge.np_cut.narration_payees.json` | 46,830 | `ae9b3ab826b41e437fc41fc5533c9c77acc1da7c034e351116b4a6e239bd85ad` | `goldens/edge.np_cut.narration_payees.json` |
| `edge.np_cut.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `goldens/edge.np_cut.tds_payees.json` |
| `edge.np_else_agg.narration_payees.json` | 29,712 | `41c4af58d6a44b99399c884f369ffe04d2b1d87b5af4d0692157d923782f1bc5` | `goldens/edge.np_else_agg.narration_payees.json` |
| `edge.np_else_agg.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `goldens/edge.np_else_agg.tds_payees.json` |
| `edge.np_else_single.narration_payees.json` | 16,158 | `06f5dd1a141a20107cded3870b6cee2678e888f544fdd7caae66e91998703d54` | `goldens/edge.np_else_single.narration_payees.json` |
| `edge.np_else_single.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `goldens/edge.np_else_single.tds_payees.json` |
| `edge.np_elsewhere.narration_payees.json` | 14,751 | `6c6c30a8c5cc8e7a228fba4a83e2434233f55133dc893cef2df25c99d98d5c6c` | `goldens/edge.np_elsewhere.narration_payees.json` |
| `edge.np_elsewhere.tds_payees.json` | 16,342 | `10e473aaafd51d2074c50a3d20dbb11e4e657120c59278ca6655b147665ef65f` | `goldens/edge.np_elsewhere.tds_payees.json` |
| `edge.np_empty.narration_payees.json` | 5,861 | `58cb45bbb2afd1ccd63b9b94efc81dd009301cacb7f067dd641e5619b083a1b5` | `goldens/edge.np_empty.narration_payees.json` |
| `edge.np_empty.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `goldens/edge.np_empty.tds_payees.json` |
| `edge.np_forms.narration_payees.json` | 56,666 | `dcd5451e6254837a49cc0043bdb47b57af7d77dd54d1c5bce3fd363efe56be85` | `goldens/edge.np_forms.narration_payees.json` |
| `edge.np_forms.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `goldens/edge.np_forms.tds_payees.json` |
| `edge.np_handles.narration_payees.json` | 81,889 | `9a354cad474091d979eae7556f632a1bc7719f40da6a6a74cdd19405d2dcbbbe` | `goldens/edge.np_handles.narration_payees.json` |
| `edge.np_handles.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `goldens/edge.np_handles.tds_payees.json` |
| `edge.np_names.narration_payees.json` | 11,606 | `6cb3e66dd32794662f82d3ea99ccd54443c08cad586d383107e03598a54c2307` | `goldens/edge.np_names.narration_payees.json` |
| `edge.np_names.tds_payees.json` | 16,728 | `71bc9df62e4bde59f09f1dd672446bb5ba80ee6d6dc898142aef674d14826d1e` | `goldens/edge.np_names.tds_payees.json` |
| `edge.np_outside.narration_payees.json` | 21,828 | `30bcbe0fbbc93b3a01df51b9168c158bf56816aff625f4b3046e88aa4e218a0b` | `goldens/edge.np_outside.narration_payees.json` |
| `edge.np_outside.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `goldens/edge.np_outside.tds_payees.json` |
| `edge.np_quiet.narration_payees.json` | 5,861 | `58cb45bbb2afd1ccd63b9b94efc81dd009301cacb7f067dd641e5619b083a1b5` | `goldens/edge.np_quiet.narration_payees.json` |
| `edge.np_quiet.tds_payees.json` | 20,246 | `2a3fa0d8c054f2628c0ec564e55440bb1c7603e6a24557ac250747ffda9413d1` | `goldens/edge.np_quiet.tds_payees.json` |
| `edge.np_rows.narration_payees.json` | 20,912 | `d2f00f6ebb60910260df52ee60a44c4d7ee03695a384b97163873ad753365bf8` | `goldens/edge.np_rows.narration_payees.json` |
| `edge.np_rows.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `goldens/edge.np_rows.tds_payees.json` |
| `edge.np_same_day.narration_payees.json` | 36,999 | `e8b09f4b3181fdca62b0376d051ce818bc5d6213b912d12ecc5cb02490bf8c3e` | `goldens/edge.np_same_day.narration_payees.json` |
| `edge.np_same_day.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `goldens/edge.np_same_day.tds_payees.json` |
| `edge.np_shared_guid.narration_payees.json` | 29,838 | `c3e745493f1b8683ec976b24e8b881d5095c2b73f1c2a5d8940ae8e32b9f67ad` | `goldens/edge.np_shared_guid.narration_payees.json` |
| `edge.np_shared_guid.tds_payees.json` | 20,803 | `8b7b799ac88aa2cf3e23066bb92bc00a4dc2e93f578416a9b9a386d1a7ea6dc3` | `goldens/edge.np_shared_guid.tds_payees.json` |
| `edge.np_single.narration_payees.json` | 23,879 | `dd34c6ccf935256b2ca2656815aebc7589ca54bd19f1c7c5c67fc01f7664a7a0` | `goldens/edge.np_single.narration_payees.json` |
| `edge.np_single.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `goldens/edge.np_single.tds_payees.json` |
| `edge.np_text.narration_payees.json` | 34,529 | `8d8e495fee0e27c45fd8fea63e5b76eeb1cccd3cffa0f329e7616c2e78d0f63b` | `goldens/edge.np_text.narration_payees.json` |
| `edge.np_text.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `goldens/edge.np_text.tds_payees.json` |
| `edge.np_unread.narration_payees.json` | 23,176 | `2b6d79a211bd70be8a38f562656027da425470bced6042c7d88b00ef09859afd` | `goldens/edge.np_unread.narration_payees.json` |
| `edge.np_unread.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `goldens/edge.np_unread.tds_payees.json` |
| `synthetic.narration_payees.json` | 6,417 | `a3cb127263b6c64e381d0a7e576f6976a98a598e85063d40b02d597739350ef6` | `goldens/synthetic.narration_payees.json` |
