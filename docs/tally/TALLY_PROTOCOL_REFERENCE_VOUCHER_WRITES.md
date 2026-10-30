# Tally XML gateway protocol reference — voucher writes

> This is a section of the canonical [Tally XML gateway protocol reference](./TALLY_PROTOCOL_REFERENCE.md). Read its confidence-marker convention and legacy-link note before relying on a finding.

---

## 9.5 Identity after write

**VERIFIED.** `LASTMID` is **0** on successful master creates, so this counter does not
identify the created master. `LASTVCHID` is populated for vouchers. Non-numeric
`LASTVCHID` text is also accepted without error when parsed back.

For the implementation's readback identity policy, see `IMPLEMENTATION_GUIDE.md` §3.5
and `PROMPT_PLAYBOOK.md` Phase 4 step 4. Their prescriptions are separate from this observation.

---

### 9.8 Voucher numbering method changes everything — **use Manual**

**Scope: "use Manual" applies when ComplyEaze Bridge supplies the voucher number.** A numberless
native post is the opposite case: under Manual numbering its second voucher was given a repeated
number or refused, and under Automatic with "Retain Original Voucher No." it was numbered in
sequence; see "A numberless native post under Manual and Automatic numbering" at the end of this
section.

**VERIFIED.** The voucher type's numbering method silently determines both whether your
voucher number survives and how a failed Alter behaves.

| Numbering method | Your `<VOUCHERNUMBER>` | Failed Alter behaviour |
| --- | --- | --- |
| **Automatic** (`Auto Retain`) | **Discarded.** Tally assigns its own — we sent `BRIDGE-PROBE-VCH-001` and Tally stored `1` | **Silently creates a duplicate** (`CREATED=1`) |
| **Manual** + `PREVENTDUPLICATES=Yes` | **Preserved.** `BRIDGE-MAN-0001` stored verbatim | **Cleanly rejected** (`CREATED=0, ALTERED=0, EXCEPTIONS=1`) |

Two consequences, both significant:

1. **What was measured is the FAILED-ALTER column, and the consequences below are about that
   column.** §9.8 sent a failed `Alter`; it did not test a `Create` retry, a restart, or another
   voucher type, and its own rule below forbids carrying the observation to a different request
   identity mechanism. Read "idempotency" here as "this failure mode, under this setting".
   Voucher-number-based idempotency in that sense only works with Manual numbering (of the two
   methods in the table; no other method was put through a failed Alter). Under automatic
   numbering the client-supplied number is thrown away, so any dedupe key built on it is
   silently ineffective. (The method measured here is `Automatic`, with `Auto Retain`. On
   2026-10-13 one raw import of a Receipt had its number thrown away under `Auto Renumber` too,
   and one raw import of a Journal kept its number under the separate method `Automatic (Manual
   Override)`: the dated paragraphs below.)
   This was not obvious — the create returned `CREATED=1, ERRORS=0`
   and looked entirely successful.
2. **Manual numbering converts a dangerous failure into a safe one.** The same failed Alter
   duplicates a client's voucher under automatic numbering and is rejected under manual.

> **RULE: a flow that relies on voucher numbers for identity or duplicate
> rejection requires Manual numbering with `PREVENTDUPLICATES=Yes`.** Do not
> apply the failed-`Alter` observation to a different request identity mechanism.

**Scope clarification — verified 2026-09-06, recorded 2026-09-07.** The licensed
Journal file workflow uses `ACTION="Create"` with a stable client `REMOTEID`, as
described in [Implementation Guide §3.3a](IMPLEMENTATION_GUIDE.md#33a-remoteid-is-the-idempotency-key--supersedes-34s-conclusion).
The measured file omitted `VOUCHERNUMBER`. Its first import returned
`CREATED=1, ALTERED=0`; importing the exact file again returned
`CREATED=0, ALTERED=1`. Readback retained one voucher with the same GUID, numeric
master ID, and assigned voucher number. The file SHA-256 was
`7c02fd1b598d70157fa676e5a41ae16184034d39169793d8554e31713cd0f127`.

This qualifies that exact-file repeat on the observed licensed Journal path. It
does not establish voucher-number-based identity, the configured numbering
method, other request shapes or voucher types, restart behavior, or universal
REMOTEID semantics.

**Automatic Journal dispatch — verified 2026-09-07, bounded observation.**
On the synthetic TallyPrime Silver 7.1 instance, a macOS MCP host obtained
explicit native approval and sent one saved Journal with `ACTION="Create"`
and its generated `REMOTEID`. The response reported `CREATED=1`, with all
other import counters zero. Independent effective-voucher readback matched
the date, ledger entries and amounts. A fresh MCP process then reconciled
the original batch: the voucher identity, number and AlterID were unchanged,
and the local journal still contained one dispatch intent and one response.
The observed executable SHA-256 was
`208e6c95fb5f2a18aba4592767f6eeee9121922e02bb55371792925d42114a54`.
This establishes that single dispatch and restart reconciliation; it does
not qualify interactive Windows approval or Gold/Education live posting.

Bridge can dispatch only a locally built, saved one-Journal batch on this
source-specific path: it binds the saved endpoint, requires an independent
native approval, records one durable attempt before sending, persists any
response, and reads the original batch back. It does not automatically retry.
A timeout, missing response, dirty counters, or incomplete readback stays with
the original saved batch for read-only reconciliation. This operational guard
is not evidence for a different product, licence mode, voucher type, endpoint,
or request shape. A mandatory manual-numbering preflight would require a
separately observed voucher-type read contract; it cannot be inferred from the
failed-`Alter` case.

*Note, 2026-09-30.* A voucher-type read now exists: the `masters` tool with
`kind=voucher_types` returns each type's numbering method as Tally reported it, measured on one
synthetic book on licensed TallyPrime 7.1 Silver
([§12a.12](./TALLY_PROTOCOL_REFERENCE_MEASUREMENTS_AND_OPEN_QUESTIONS.md#12a12-master-collections-on-licensed-71)).
`Default` numbering remains unmeasured, so a numbering preflight is still not
qualified and the rule above stands until that is measured.

**Native mutation selector — verified 2026-09-07, bounded fresh-dispatch observation.**
A new native attempt uses a fresh private `REMOTEID`, separate from the selected
public file's identity. The narration keeps the original batch attribution.
The durable dispatch intent binds the exact UTF-16LE native request SHA-256 before
sending; the response must match that commitment. Older intents without this
field remain readable and are never resent. This prevents native posting from
reusing the public file's mutation selector after a manual import. It does not
prove semantic absence after arbitrary edits to an earlier business event.
Native posting refuses a supplied `VOUCHERNUMBER` until its matching precedence
is qualified.

A fresh synthetic Silver 7.1 Journal using this separate selector and attribution
returned `CREATED=1`, all other counters zero, and explicit zero exceptions.
Independent readback matched its original batch attribution, date and balanced
entries. Restarting with both generation and posting disabled reconciled the same
GUID, master ID, voucher number and AlterID with one intent and one response.
The original earlier Journal was unchanged in before/after readback. The executable
SHA-256 was `c2df9ffd76bf687e50b9b78916cb83fdacc6cddde2f317b53c46f538f1b2a96f`;
the durable native request commitment matched the actual transport request hash.
This qualifies a fresh dispatch and read-only restart for that binary and source.
**Preservation follow-up — verified 2026-09-08, bounded observation.** The
macOS desktop at `f1ffcc4`, using shared posting code at `7c3266f`, posted one
human-approved synthetic Journal in the quiet-company workflow. Its signed
executable SHA-256 was
`078e6bfe95fdd57f8f5ef905167b49a509bb0f7a3d3dd94734ae9aadc36c32ca`.
Readback found exactly one new Journal and all six prior accounting records
unchanged, including the manually edited public-file comparison voucher.
The original selected XML stayed unchanged. The saved response included all
seven counter-presence flags: one create and all other counters zero. A fresh
`7c3266f` MCP process with writes disabled reconciled the original batch without
resend; history retained one intent and one response. This closes that measured
preservation/recovery comparison, not arbitrary concurrent editing or other
platform, product-mode, or voucher-type qualification.

**Batch identity qualification — verified 2026-09-06, recorded 2026-09-07.**
A new synthetic Journal reused the earlier caller transaction label in a separate
local batch. Its `REMOTEID` and narration marker used the same batch-derived UUID;
the caller label was not the wire key. The first exact-file import returned
`CREATED=1, ALTERED=0`; its repeat returned `CREATED=0, ALTERED=1`. Both had zero
errors, exceptions and deletions. Readback retained new master ID 5, voucher number
2, and the same GUID while AlterID advanced from 9 to 10. The old voucher's GUID,
master ID 4, number 1 and AlterID 8 were unchanged in immediate before/after reads.
The new file SHA-256 was
`e39eb3c0bfe53144bdd9c0f4afcb88c3d63a2050214233ee77465d42a54245ef`.
The unchanged UTF-16LE readback is retained as
`native-namespaced-journal.utf16le.xml` in the protocol fixture tree, with capture
metadata. This is a Silver 7.1 Journal observation; it does not qualify other
profiles or deduplication after losing/rebuilding a batch.

Creating such a type over XML works: `<VOUCHERTYPE ACTION="Create">` with
`<NUMBERINGMETHOD>Manual</NUMBERINGMETHOD>` and `<PREVENTDUPLICATES>Yes</PREVENTDUPLICATES>`
returned `CREATED=1`.

Note also that `EXCEPTIONS=1` arrived with **no `LINEERROR`** — a parser must treat a
non-zero `EXCEPTIONS` as failure on its own, without waiting for an error string.

**The method is kept per number series, not on the voucher type — verified 2026-10-09 on one
book (PARTIAL).** On a synthetic book on Silver 7.1 on Windows 10, three voucher types were set on
Tally's own screens (Journal Automatic; Payment Manual with duplicates prevented; Receipt Manual
without), and one export of all 24 voucher types fetched `NUMBERINGMETHOD`, `PREVENTDUPLICATES` and
`VOUCHERNUMBERSERIES.*` (request 1,106 bytes, response 81,966 bytes; both committed as
`voucher_type_numbering_series_*` with their provenance, bridge#724).

- **VERIFIED:** each type holds exactly one `VOUCHERNUMBERSERIES.LIST`, named `Default`. The series'
  `NUMBERINGMETHOD` and `PREVENTDUPLICATES` match the screen for all three types (`Automatic`/`No`,
  `Manual`/`Yes`, `Manual`/`No`); the 21 types left alone read `Automatic`/`No`.
- **VERIFIED:** the type-level `NUMBERINGMETHOD` reads `None` for the three types set by hand and
  `Default` for the rest, and the type-level `PREVENTDUPLICATES` reads `No` on all 24, including
  Payment, whose screen says Yes. The type level is not the method and is never read for it.
- **Observed, meaning not established:** `DUPLICATECONTROL` is present and empty on all 24 series.
  `NUMBERINGSUBMETHOD` read `Auto Retain` on all 24 series, Manual ones included; what it carries
  was measured on 2026-10-13 (next paragraph), as were the texts of `Automatic (Manual Override)`
  (until then seen only on a real book on 2026-09-28, not committed) and Multi-user Auto.
- **Not established:** more than one series on a type; a single series not named `Default`; a type
  created new rather than altered; other releases; macOS. A type with no series or several is a
  typed observation, not a method, and a method outside `Automatic`, `Manual` and
  `Automatic (Manual Override)` is kept raw and never read as `Manual`
  (`native_voucher_type_numbering.rs`).

**A save of a voucher-type screen moved the master mark, and three more settings as exported:
verified 2026-10-13 on one book and one voucher type (PARTIAL).** On a synthetic book on Silver 7.1
on Windows, one voucher type that held no vouchers (Contra) was saved on Tally's own screen nine
times: first with no field changed, then with one field changed per save and each change then
undone. The company's marks (`ALTVCHID`, `ALTMSTID`) were read after every save, and the series
request above (1,106 bytes; 1,116 as sent, with only the company name changed) once before the
saves and after six of the nine (branch `lab/724-m6-master-mark-answers`, `b7761256`, bridge#724;
every marks and series answer was read with an XML parser, with its size checked against
`CONTENT-LENGTH`). The screen labels, the licence and release, which field each save changed or
left at its offered default, and that the type held no vouchers are from that branch's notes; no
screenshot is on the branch.

- **VERIFIED:** each of the nine saves moved `ALTMSTID` by 2 (213 to 231) and left `ALTVCHID`
  unchanged, including the save with no field changed. In every series answer read after a save,
  the type's own `ALTERID` equals the `ALTMSTID` read with it, and no other type's `ALTERID` moved.
  So on this book every save of that type's screen moved `ALTMSTID`; a save of another type was not
  measured in this run (the run in the next paragraph saved Receipt twice). A moved `ALTMSTID` does
  not show that a setting changed.
- **VERIFIED:** the series `NUMBERINGMETHOD` prints `Automatic (Manual Override)` and
  `Multi-user Auto` exactly as the screen names them, in plain ASCII; Multi-user Auto was offered
  on this Silver licence. A type switched to `Manual` with the screen's default left reads
  `PREVENTDUPLICATES` `No`.
- **VERIFIED:** on this type, with the method Automatic, the series `NUMBERINGSUBMETHOD` followed
  the screen's "Numbering behaviour on insertion/deletion": `Auto Retain` for "Retain Original
  Voucher No." and `Auto Renumber` for "Renumber Vouchers". It was the only child of the series
  list that differed, and `NUMBERINGMETHOD` stayed `Automatic`.
- **VERIFIED:** the type-level `NUMBERINGMETHOD` of the saved type read `Default` before the saves
  and `None` in every series answer after them, also with the method back at Automatic. Of the
  other 23 types, 22 read `Default` throughout and Payment, whose series already read `Manual` at
  the first read, reads `None` throughout. This agrees with the 9 October reading above. Not
  established: whether the save that changed nothing flips it on its own (no series answer was
  read between it and the next save).
- **Not established:** a save of a type other than Contra, Receipt and Journal (the paragraphs
  below measured saves of Receipt and of Journal, which held vouchers; each changed the type's
  numbering, and `ALTMSTID` moved by 2 at each); Gold; other releases; macOS; the same answers
  through ComplyEaze Bridge's own read.

**"Renumber Vouchers" changed the numbers of vouchers already in the book without moving their
`ALTERID`: verified 2026-10-13 on one book and one voucher type (PARTIAL).** On the same synthetic
book, the Receipt type held one voucher. Six more Receipts were entered by hand on Tally's own
screen, one per step, with no number typed, and the type's "Numbering behaviour on
insertion/deletion" was set to "Renumber Vouchers" and back between them. The company's marks and
ComplyEaze Bridge's own voucher export for July 2026 (`render_native_voucher_export_request`,
1,778 bytes as sent) were read after every act, and the series request above (1,116 bytes as
sent) at each setting (branch `lab/724-m8r-renumber-answers`, `6726047a`, bridge#724; every figure
was read from the marks, voucher and series answers with an XML parser, with each size checked
against `CONTENT-LENGTH`).

- **VERIFIED:** with the series at `Auto Retain` ("Retain Original Voucher No."), each new Receipt
  took the next number, a back-dated one included (twice), and no other voucher's number changed.
- **VERIFIED:** saving the type with "Renumber Vouchers" (the series then read `Auto Renumber`)
  put the four Receipts in the book into date order: three of them changed number. A back-dated
  Receipt entered under it took number 2 and moved three later-dated Receipts up by one; a Receipt
  dated after every other took the next number and moved nothing. Saving the type back to Retain
  changed four of the six Receipts again, into the order they were entered (`MASTERID` order).
- **VERIFIED:** in all ten changes of number the voucher's `ALTERID`, `MASTERID` and GUID stayed as
  they were. `ALTVCHID` did not move at either save of the type; it moved by one for each voucher
  entered. `ALTMSTID` moved by 2 at each save (231, 233, 235) and the type's own `ALTERID` read the
  new `ALTMSTID` (233, then 235). So on this run a voucher's `ALTERID` and `ALTVCHID` did not show a
  renumbering. The master mark and the type's `ALTERID` moved at each save, as they did at each
  save of Contra and Receipt (the paragraph above), so they do not show one either.
- **VERIFIED:** what the answers did show is which behaviour was set, in two places. The series'
  `NUMBERINGSUBMETHOD` read `Auto Retain`, then `Auto Renumber`, then `Auto Retain`. And every
  voucher row of the export carried `NUMBERINGSTYLE`: on each Receipt it read the setting in force
  when the row was read, `Auto Retain` before the first save, `Auto Renumber` while the setting was
  on (also on the two whose numbers never changed) and `Auto Retain` after the second; on the two
  Payments, whose type is Manual, it read `Manual`, and on the three Journals `Auto Retain`
  throughout. Neither value says that a number has changed.
- **VERIFIED:** no voucher of another type changed (two Payments, three Journals), and the other
  23 types' series read the same in all four series answers.
- From that branch's notes, not from an answer: that each Receipt was entered by hand with no
  number typed, and which screen setting was changed; that Tally asked two confirmations at each
  switch on this type, one on choosing the behaviour and one on accepting the screen; and that
  before accepting a Receipt the entry screen showed the number it then took.
- **VERIFIED:** the Receipt type's own type-level `NUMBERINGMETHOD` read `Default` in the first
  series answer and `None` in the three after its saves, as Contra's did in the paragraph above.
- **Not established:** a deletion; Journal, Payment and Contra by hand entry; a type holding more
  than a handful of vouchers; Gold; other releases; macOS; "Set/Alter additional numbering
  details". A voucher that arrives by import was measured later the same day (next paragraph),
  and one Journal by import after that (three paragraphs below).

**A voucher sent by import changed the numbers of vouchers already in the book under "Renumber
Vouchers": verified 2026-10-13 on one book and one voucher type (PARTIAL).** On the same synthetic
book, later the same day, three Receipts were sent by import, one per step, each without a number
and with a new `REMOTEID`, in the shape of the committed fixture `post-span-import.xml`. The
Receipt type was set to "Renumber Vouchers" and back on its screen between them. The company's
marks and the July vouchers were read after every act, and the series request above at each
setting (branch `lab/724-m9i-import-renumber-answers`, `e29cbb5f`, bridge#724; every figure was
read from the marks, voucher, series and import answers with an XML parser, with each size checked
against `CONTENT-LENGTH`).

- **VERIFIED:** with the series at `Auto Retain`, an imported Receipt dated before three existing
  ones took the next number, and no other voucher's number changed.
- **VERIFIED:** with the series at `Auto Renumber`, an imported Receipt dated 4 July took number 4,
  the number its place in date order gives, and each of the five later-dated Receipts went up by
  one; one of the five had itself been imported. No voucher's `ALTERID`, `MASTERID` or GUID
  changed, `ALTVCHID` rose by one (13 to 14) and `ALTMSTID` did not move. The import's own answer
  read `CREATED` 1, `ALTERED` 0, `ERRORS` 0. An imported Receipt dated after every other took the
  next number and moved nothing.
- **VERIFIED:** the two saves of the type renumbered as in the paragraph above: six numbers changed
  at the switch (into date order) and eight at the switch back (into `MASTERID` order, which is
  also the order of `VOUCHERRETAINKEY`: 1, 5, 9 and so on), with no `ALTERID` change either time.
  `ALTMSTID` moved by 2 at each save (235, 237, 239).
- **VERIFIED:** no voucher of another type changed, and the other 23 types' series read the same in
  all four series answers. After the book was restored from a backup taken at the start, the
  marks, the Receipt series and the twelve vouchers read as at the start.
- **VERIFIED:** `NUMBERINGSTYLE` on a voucher row read the sub-method of its type's series where
  the method was Automatic (`Auto Retain` or `Auto Renumber` on the Receipts as the series was
  switched, `Auto Retain` on the three Journals), and `Manual` on the two Payments, whose series
  reads the method `Manual` and the sub-method `Auto Retain`. This held at the four voucher reads
  that had a series read beside them, and the rows read the same at the three between, where no
  type was saved. It is recorded as a possible second reading of the setting. No ComplyEaze
  Bridge source names the field (checked in the code, not in an answer).
- So on this run nothing an import returns, and nothing in a voucher's `ALTERID` or in `ALTVCHID`,
  showed that the import had changed other vouchers' numbers. What read differently beforehand
  was the series' `NUMBERINGSUBMETHOD`, and `NUMBERINGSTYLE` on the voucher rows of that type (the
  type's own `ALTERID` and the company's `ALTMSTID` differed too, as after any measured save of a
  type).
- From that branch's notes, not from an answer: that Tally asked the same two confirmations at
  each switch, and that on the way to the behaviour field the row `Automatic`, already set, was
  selected again in "Method of Voucher Numbering".
- **Not established:** Journal, Payment and Contra; a deletion; a type holding more than a handful
  of vouchers; Gold; other releases; macOS; the same through ComplyEaze Bridge's own post. A file
  of two vouchers and a voucher that carries a number were measured later the same day (next
  paragraph), and after them one numberless Journal dated 19 July, between the Journals of the
  book, under "Renumber Vouchers" (the paragraph after that), which is all that is measured of an
  imported Journal under that setting.

**A file of two vouchers, and a voucher that carries a number, each changed numbers already in
the book under "Renumber Vouchers", and neither supplied number was kept: verified 2026-10-13 on
one book and one voucher type (PARTIAL).** On the same synthetic book, after the run above had
been restored, three more imports of Receipts were sent, one per step, in that run's shape (branch
`lab/724-m9j-batch-and-numbered-answers`, `703dd35e`, bridge#724; every figure was read from the
marks, voucher, series and import answers with an XML parser, with each size checked against
`CONTENT-LENGTH`). A numbered Receipt carried one `VOUCHERNUMBER` element after
`PARTYLEDGERNAME`, where ComplyEaze Bridge's builder places it (read in the builder's code). Both
supplied numbers were text with letters, not held by any voucher of the book.

- **VERIFIED:** with the series at `Auto Retain`, a Receipt that carried the number `M9J-C1`,
  dated before three existing ones, read back number 8, the next number, and no other voucher's
  number changed. This is the control, on the book as found; it agrees with the table at the head
  of this section, which was measured on another book.
- **VERIFIED:** with the series at `Auto Renumber`, one file of two Receipts without numbers, the
  later-dated first, answered `CREATED` 2, `ALTERED` 0. Both were numbered by their dates (6 July
  took 5, 8 July took 7), and four Receipts already in the book changed number: the one dated
  between the two went up by one and the three later-dated ones by two. `MASTERID` and `ALTERID`
  followed the order of the file (14 for the first voucher in it, 15 for the second).
- **VERIFIED:** with the series at `Auto Renumber`, a Receipt that carried the number `M9J-N1`,
  dated 4 July, read back number 4, the number its place in date order gives, and each of the
  seven later-dated Receipts went up by one. Its answer read `CREATED` 1, `ALTERED` 0.
- **VERIFIED:** no voucher row read either supplied number at any of the seven voucher reads. No
  existing voucher's `ALTERID`, `MASTERID` or GUID changed at any step. `ALTVCHID` rose only by
  the vouchers created (12, 13, 15, 16), and `ALTMSTID` moved by 2 at each save of the type (235,
  237, 239) and at no import.
- **VERIFIED:** the two saves of the type changed six numbers and then ten, into date order and
  back into `MASTERID` order. No voucher of another type changed, and the other 23 types' series
  read the same in all four series answers. After the book was restored from a backup taken at
  the start, the marks, the Receipt series and the twelve vouchers read as at the start.
- So on this run, under `Auto Renumber`, a supplied voucher number did not keep a voucher, or the
  vouchers already in the book, from being renumbered, and a file of two vouchers renumbered as a
  single voucher did. Under `Auto Retain` nothing already in the book was renumbered.
- From that branch's notes, not from an answer: that the two switches were made on Tally's own
  Receipt type screen, and that Tally asked the same two confirmations at each.
- **Not established:** a file of more than two vouchers; a file that mixes numbered and numberless
  vouchers; a supplied number made of digits only; a number that an existing voucher already holds;
  Journal, Payment and Contra (a numbered Journal, or a file of two, under "Renumber Vouchers"
  included); a deletion; a type holding more than a handful of vouchers; Gold; other releases;
  macOS; the same through ComplyEaze Bridge's own post. A supplied number under Automatic (Manual
  Override), and one numberless Journal under "Renumber Vouchers", were measured later the same day
  (next paragraph), and a supplied number under Manual with duplicates allowed in a later run (the
  paragraph after that).

**"Automatic (Manual Override)" kept a supplied number and gave a numberless voucher none, and a
back-dated import renumbered Journals under "Renumber Vouchers": verified 2026-10-13 on one book
and one voucher type (PARTIAL).** On the same synthetic book, restored again, the Journal voucher
type's numbering was changed three times and three Journals were sent by import, one per step,
each voucher in the shape of a voucher of the committed `post-span-journal-import.xml` with a new
`REMOTEID` (branch `lab/724-m9k-manual-override-and-journal-answers`, `34db4f7e`, bridge#724;
every figure was read from the marks, voucher, series and import answers with an XML parser, with
each size checked against `CONTENT-LENGTH`). The numbered Journal carried one `VOUCHERNUMBER`
element after `VOUCHERTYPENAME`, where ComplyEaze Bridge's builder places it for a Journal (read
in the builder's code). Twice in the run, between its two halves and at the end, the marks, the
Journal series and the twelve vouchers read as at the start again; that the three changes were
saves of Tally's own Journal type screen, and that the book was put back by restoring one backup,
are from that branch's notes. This method is a value of its own in the series answer, so the row
"Automatic" of the table at the head of this section does not cover it.

- **VERIFIED, the switch to the method:** saving the Journal type with the method "Automatic
  (Manual Override)" left each of the three Journals in the book with an empty `VOUCHERNUMBER`
  element (they had read 1, 2 and 3). Their `ALTERID`, `MASTERID`, GUID and `VOUCHERRETAINKEY`
  did not change, `ALTVCHID` did not move and `ALTMSTID` moved by 2. The Journal series then read
  `NUMBERINGMETHOD` `Automatic (Manual Override)`, `NUMBERINGSUBMETHOD` `Auto Retain` and
  `PREVENTDUPLICATES` `No`. In the voucher answers, `NUMBERINGSTYLE` on each Journal row read
  `Automatic (Manual Override)`.
- **VERIFIED, a voucher without a number under that method:** an imported Journal answered
  `CREATED` 1, `ALTERED` 0 and read back with an empty `VOUCHERNUMBER` element: Tally gave it no
  number. No other voucher changed.
- **VERIFIED, a voucher that carries a number under that method:** an imported Journal with one
  `VOUCHERNUMBER` element reading `M9K-J1` answered `CREATED` 1, `ALTERED` 0 and read back
  `M9K-J1`. No other voucher changed. Both imports were dated after every other Journal.
- **VERIFIED, a Journal under "Renumber Vouchers":** after the restore, saving the Journal type
  (method Automatic) with "Renumber Vouchers" put the three Journals into date order; all three
  changed number. A Journal imported without a number, dated 19 July, between them, read back 2,
  the number its place in date order gives; the two later-dated Journals each went up by one and
  the earlier-dated one kept 1. The answer read `CREATED` 1, `ALTERED` 0. Saving the type back to
  "Retain Original Voucher No." changed all four numbers, into `MASTERID` order.
- **VERIFIED:** no voucher's `ALTERID`, `MASTERID` or GUID changed at any step. `ALTVCHID` rose by
  one at each import and at no save; `ALTMSTID` moved by 2 at each of the three saves and at no
  import. The nine vouchers that are not Journals read as at the start at every one of the nine
  voucher reads, and the other 23 types' series read the same in all six series answers.
- So on this run a voucher sent without a number into a type on "Automatic (Manual Override)"
  ended with no number at all, and one sent with a number kept it; the switch to that method
  blanked the numbers of that type's three vouchers, as the switch to Manual blanked a type's
  numbers on 12 October (below). The renumbering under "Renumber Vouchers" also happened on the
  Journal type.
- From that branch's notes, not from an answer: that Tally asked the same two confirmations at each
  of the three saves; and that at the first save, once the method was chosen, the type screen no
  longer showed "Numbering behaviour on insertion/deletion", showed "Prevent creating duplicate
  Voucher Nos." with No in its place, and showed "Show unused vch nos. in transactions for Retain
  Original Voucher No. behaviour" gone from Yes to greyed out, reading No.
- **Not established:** a back-dated voucher WITHOUT a number under "Automatic (Manual Override)"; a
  supplied number made of digits only; a number that an existing voucher already holds, with
  duplicates prevented (both were measured under Manual in a later run, below, and not under this
  method); a voucher entered by hand under that method; under "Renumber Vouchers", a
  Journal that carries a number, a Journal dated after every other, and a file of several; a
  Journal imported raw under `Auto Retain` with an earlier date; Multi-user Auto; Payment and
  Contra; a deletion; Gold; other releases; macOS; the same through ComplyEaze Bridge's own post. A
  back-dated numbered voucher, and a number that another voucher already holds, were measured under
  this method in a later run, with duplicates allowed (next paragraph).

**A voucher's own number was kept under Manual with duplicates allowed and under "Automatic
(Manual Override)", also when back-dated and when another voucher already held it: verified
2026-10-14 on one book (PARTIAL).** On the same synthetic book, restored again, six vouchers were
sent by import, one per step, each carrying one `VOUCHERNUMBER` element where ComplyEaze Bridge's
builder places it (read in the builder's code) and a new `REMOTEID` (branch
`lab/724-m9l-held-and-backdated-number-answers`, `8788d89f`, bridge#724; every figure was read
from the marks, voucher, series and import answers with an XML parser, with each size checked
against `CONTENT-LENGTH`). The first three were Payments, into the Payment type as it stood: its
series read `Manual` with `PREVENTDUPLICATES` `No`, and it held two Payments that both read
number 1. The last three were Journals, after the Journal type's series had been changed to
`Automatic (Manual Override)`, also with `PREVENTDUPLICATES` `No`. That the change was a save of
Tally's own Journal type screen, and that the book was put back between the halves and at the
end by restoring one backup, are from that branch's notes; the answers show the marks, the series
and the twelve vouchers reading as at the start both times.

- **VERIFIED, Manual with duplicates allowed (Payment):** a Payment carrying `M9L-P1`, dated after
  both Payments of the book, read back `M9L-P1`. One carrying `M9L-P2`, dated before both, read
  back `M9L-P2`. One carrying `M9L-P1` again was created as a second voucher reading `M9L-P1`,
  with its own `MASTERID`, and the first was unchanged. Each answer read `CREATED` 1, `ALTERED` 0.
- **VERIFIED, "Automatic (Manual Override)" with duplicates allowed (Journal):** the same three
  cases gave the same three results: `M9L-J1` was kept; `M9L-J2`, dated before every Journal of
  the book, was kept; `M9L-J1` again was created as a second voucher beside the first. Each answer
  read `CREATED` 1, `ALTERED` 0. As on the run above, the change of method left the three Journals
  of the book with an empty `VOUCHERNUMBER` element.
- **VERIFIED:** at no import did a voucher already in the book change its number, `ALTERID`,
  `MASTERID` or GUID. `ALTVCHID` rose by one at each import. `ALTMSTID` moved at no import; it
  moved by 2 at the change of method. All 24 series rows read the same before and after the three
  Payment imports, and at every voucher read the vouchers of a type other than the one being
  imported into read as at the start.
- **VERIFIED, one entry in a series row:** after the three Journal imports the Journal type's
  series held, inside `RESTARTFROMLIST.LIST/LASTNUMBERLIST.LIST`, a `DATE` reading `20260401` and a
  `LASTNUMBER` reading `0`. The same request sent again returned the same bytes, and no other
  element of the 24 rows differed from the read before the imports. The inner list,
  `LASTNUMBERLIST.LIST`, was an empty element in the series answer read after the change of method
  and before the imports, in the one read straight after the three Payment imports under Manual,
  and in every series answer of the five earlier runs on this book cited in this section; none of
  those earlier answers was read straight after an import, so they do not show what an import under
  `Auto Retain` or `Auto Renumber` leaves there. **Not established:** which of the three imports
  wrote it (no series answer was read between them); whether a voucher without a number writes it
  (the run above read no series after its imports under this method); what Tally uses it for.
- So on this run, with duplicates allowed, both methods stored the number an import carried,
  whether it was dated after or before every other voucher of its type, and a number that another
  voucher already held gave a second voucher with the same number, with nothing in the import's
  answer to show it.
- From that branch's notes, not from an answer: that the save of the type screen asked the same two
  confirmations as on the runs above, and that the same three lines of that screen changed with the
  method as on the run above.
- **Not established:** "Prevent creating duplicate Voucher Nos." set to Yes under "Automatic
  (Manual Override)", with a number already held (under Manual it was measured on a Journal in a
  later run, below, as was a number of digits only); a Journal under Manual with duplicates
  allowed and a Payment under "Automatic (Manual Override)" (a supplied number was measured under
  each method on one voucher type); a supplied number made of digits only under "Automatic
  (Manual Override)" or under Manual with duplicates allowed; a voucher without a number under
  Manual with duplicates allowed, back-dated; Multi-user Auto; a file of several vouchers that
  carry numbers; a deletion; Gold; other releases; macOS; the same through ComplyEaze Bridge's
  own post. A voucher without a number under Manual, by a raw import, was measured in a later
  run (next paragraph).

**A voucher imported without a number under Manual numbering was numbered 1 each time: verified
2026-10-14 on one book and two voucher types (PARTIAL).** On the same synthetic book, restored
again, five imports were sent, one per step, none carrying a `VOUCHERNUMBER` element and every
voucher with a new `REMOTEID` (branch `lab/724-m9m-numberless-under-manual-answers`, `9dbd42d5`,
bridge#724; every figure was read from the marks, voucher, series and import answers with an XML
parser, with each size checked against `CONTENT-LENGTH`). A series answer was read after every
import and compared, row by row, with the one read before it. The first three imports were
Contras, after the series of the Contra type, which held no voucher, had been changed to
`Manual`. The last two were Receipts, after the series of the Receipt type, which held seven
Receipts numbered 1 to 7, had been changed to `Manual`. Both series read `PREVENTDUPLICATES`
`No`; the elements named `NUMBERINGMETHOD` and `PREVENTDUPLICATES` directly on each of the two
type rows, outside its series list, read `None` and `No` at every read, before and after the
change. That each change was a save of Tally's own type screen, and that the book was put back
between the halves and at the end by restoring one backup, are from that branch's notes; the
answers show the marks, the series and the twelve vouchers reading as at the start both times.

- **VERIFIED, a type that held no voucher (Contra):** an imported Contra read back number 1. A
  second, dated a day later, read back 1 as well, and the first was unchanged. One file of two
  Contras, the later-dated first, answered `CREATED` 2, and both read 1; their `MASTERID`
  followed the order of the file.
- **VERIFIED, the change of method on a type that held vouchers (Receipt):** it left each of the
  seven Receipts with an empty `VOUCHERNUMBER` element. Their `ALTERID`, `MASTERID`, GUID and
  `VOUCHERRETAINKEY` did not change, `NUMBERINGSTYLE` on them read `Manual`, `ALTVCHID` did not
  move and `ALTMSTID` moved by 2.
- **VERIFIED, a type that held seven vouchers with emptied numbers (Receipt):** an imported
  Receipt dated after all seven read back number 1, and a second, dated a day later, read back 1
  as well. The seven stayed empty.
- **VERIFIED:** every import answer read `CREATED` equal to the vouchers of its file, `ALTERED` 0,
  `LASTVCHID` the `MASTERID` of the last voucher created, and every other counter 0. No voucher
  already in the book changed at any import, and the vouchers of a type other than the one being
  imported into read as at the start at every voucher read. `ALTVCHID` rose only by the vouchers
  created; `ALTMSTID` moved at no import.
- **VERIFIED, the series rows:** after each of the five imports all 24 rows equalled the series
  answer read before that import. The inner list `LASTNUMBERLIST.LIST`, which gained an entry
  under "Automatic (Manual Override)" in the run above, stayed an empty element under `Manual`.
- So on this run, with duplicates allowed, Manual numbering gave every voucher that came without
  a number the number 1, whether its type held no voucher or seven with emptied numbers, and
  whether it came alone or as one of two in a file; under "Automatic (Manual Override)" such a
  voucher got no number (above). This agrees with the posts of 12 October through ComplyEaze
  Bridge on a Payment type (below).
- From that branch's notes, not from an answer: that the save of the Contra type asked nothing,
  and that the save of the Receipt type asked the two confirmations quoted below.
- **Not established:** duplicates prevented on these two types (on 12 October a second numberless
  Journal was refused whole under it, below); a voucher without a number under Manual dated
  before a voucher its type already holds; a file of more than two vouchers; Payment and Journal
  by a raw import; Multi-user Auto; a deletion; Gold; other releases; macOS; the same through
  ComplyEaze Bridge's own post on a Receipt or a Contra.

**Under Manual numbering with duplicates prevented, an imported Journal kept the number it carried,
and a number a Journal already held was refused whole by an answer that names no cause: verified
2026-10-14 on one book and one voucher type (PARTIAL).** On a second synthetic book, four imports
were sent raw, one per step, each of one Journal that carried a `VOUCHERNUMBER` element and a new
`REMOTEID` (branch `lab/724-m9n-held-number-duplicates-prevented-answers`, `85fd51e5`, bridge#724;
every figure was read from the marks, voucher, series and import answers with an XML parser, with
each size checked against `CONTENT-LENGTH`). As found, the book held one Receipt, one Payment and
one Journal, each numbered 1, and the series of its Journal type read `Manual`, `Auto Retain`,
`PREVENTDUPLICATES` `Yes`, as did the series of its Payment type; the elements named
`NUMBERINGMETHOD` and `PREVENTDUPLICATES` directly on each of the two type rows, outside its
series list, read `None` and `No`. A series answer was read after every import and compared, row
by row, with the one read before it. That no type screen was saved in this run, and that the
book was put back at the end by restoring a backup, are from that branch's notes; `ALTMSTID`
read the same at every read, and the marks, series and voucher answers read after the restore
are byte-identical to the first ones.

- **VERIFIED, a number no Journal held:** a Journal numbered `M9N-J1`, dated 20 July, read back
  that number, and a Journal numbered `7`, dated 22 July, read back the one character `7`. Each
  answer read `CREATED` 1, `LASTVCHID` the new voucher's `MASTERID`, and every other counter 0.
  `NUMBERINGSTYLE` on both read `Manual`.
- **VERIFIED, a number a Journal held:** a second Journal numbered `M9N-J1`, dated 21 July, sent
  after the first had been created, and a Journal numbered `1`, dated 23 July, were each refused
  whole: `CREATED` 0, `EXCEPTIONS` 1, every other counter 0, no `LINEERROR`. After each,
  everything under `COLLECTION` in the voucher answer equalled the answer before it, and
  `ALTVCHID` did not move. Only a Journal held `M9N-J1`. The number 1 was held by the Journal of
  the book as found and also by its Receipt and its Payment, so the second refusal alone does
  not show which voucher's number it met. A later run on this book sent a Payment a number that
  only a Journal held (next paragraph).
- **VERIFIED:** the two refusal answers are byte-identical: 538 bytes, SHA-256 `09a713e2…`, the
  size and hash recorded below for the refused second numberless Journal of 12 October and for
  whole refusals of zero-value vouchers. So the answer does not tell a number already held from
  those causes; only the counters show the refusal.
- **VERIFIED, the series rows:** after each of the four imports, the two refused ones included,
  all 24 rows equalled the series answer read before that import. The Journal row's
  `LASTNUMBERLIST.LIST` stayed an empty element.
- **VERIFIED, the marks:** `ALTVCHID` read 3 as found, then 4, 4, 5 and 5 after the four imports;
  `ALTMSTID` did not move. No voucher already in the book changed at any import.
- So on this run a Journal's supplied number was kept under Manual with duplicates prevented,
  as a Payment's was with duplicates allowed on the other book (above); and a Journal sent with
  a number a Journal held came back refused, with nothing that says why, where a Payment sent
  with a held number, duplicates allowed, was created as a second voucher (above). No one
  voucher type was measured under both settings in these runs; a Payment refused for a number a
  Payment held, duplicates prevented, is in the paragraph after next.
- From that branch's notes, not from an answer: that nothing appeared on Tally's screen at any of
  the four imports.
- **Not established:** duplicates prevented under "Automatic (Manual Override)"; a number with a
  leading or inner space; a Receipt or a Contra sent a number that a voucher of its own type
  holds; a file of several vouchers, one with a held number; Multi-user Auto; a deletion; Gold;
  other releases; macOS; the same through ComplyEaze Bridge's own post. A back-dated voucher with
  a number, a number with a leading zero, a held number in small letters, a number held only by a
  voucher of another type, and a Payment that carries a new number were sent in a later run on
  this book (next paragraph); a Payment sent a number a Payment holds, and a number with an end
  space, in the run after it (the paragraph after that).

**Under Manual numbering with duplicates prevented, a number held only by a Journal did not block a
Payment, a held number in small letters was refused, and `01` was created beside a held `1`:
verified 2026-10-14 on one book and two voucher types (PARTIAL).** On the book of the run above,
holding the same three vouchers, six imports were sent raw, one per step, each of one voucher that
carried a `VOUCHERNUMBER` element and a new `REMOTEID`: four Journals and two Payments (branch
`lab/724-m9o-how-a-held-number-is-matched-answers`, `ec1c39fa`, bridge#724; every figure was read
from the marks, voucher, series and import answers with an XML parser, with each size checked
against `CONTENT-LENGTH`). The series of the Journal type and of the Payment type each read
`Manual`, `Auto Retain`, `PREVENTDUPLICATES` `Yes` at every read. A series answer was read after
every import and compared, row by row, with the one read before it. That no type screen was saved
in this run, and that the book was put back at the end by restoring a backup, are from that
branch's notes; `ALTMSTID` read the same at every read, and the marks, series and voucher answers
read after the restore are byte-identical to the first ones.

- **VERIFIED, two controls:** a Journal numbered `M9O-A1`, dated 20 July, and a Payment numbered
  `M9O-P1`, dated 21 July, numbers that no voucher held, each read back its number.
- **VERIFIED, a number held only by a voucher of another type:** a Payment numbered `M9O-A1`,
  dated 22 July, sent while the only voucher that held that number was the Journal (in the
  voucher answer read before it, which returns the vouchers of July 2026: five vouchers with
  `ALTERID` 1 to 5, where `ALTVCHID` read 5), was created and read back `M9O-A1`. The Journal
  was unchanged, and both read that number at every later read until the restore. So on this
  run a number held by a Journal did not stop a Payment from taking it.
- **VERIFIED, a held number in small letters:** a Journal numbered `m9o-a1`, dated 23 July, was
  refused whole: `CREATED` 0, `EXCEPTIONS` 1, every other counter 0, no `LINEERROR`, in an answer
  byte-identical to the refusals of the run above (538 bytes, SHA-256 `09a713e2…`). Everything
  under `COLLECTION` in the voucher answer equalled the answer before it. No number in small
  letters that lacked a held twin was sent in this run or in the two before it, so this refusal
  alone does not show that Tally compares numbers without regard to letter case.
- **VERIFIED, a leading zero:** a Journal numbered `01`, dated 24 July, was created and read back
  the two characters `01`. The Receipt, the Payment and the Journal of the book as found still
  read `1`.
- **VERIFIED, a back-dated voucher:** a Journal numbered `M9O-B1`, dated 10 July, before the
  Journal of the book as found (15 July), was created and read back its number. No other voucher
  changed.
- **VERIFIED:** each of the five answers to a created voucher read `CREATED` 1, `LASTVCHID` the new
  voucher's `MASTERID` (4 to 8 in the order sent), and every other counter 0. No voucher already
  in the book changed at any import. `ALTVCHID` rose by one at each created voucher and did not
  move at the refusal. `NUMBERINGSTYLE` on the five read `Manual`; `VOUCHERRETAINKEY` read 5, 9
  and 13 on the three Journals and 5 and 9 on the two Payments, in the order created, where the
  vouchers as found read 1.
- **VERIFIED, the series rows:** after each of the six imports, the refused one included, all 24
  rows equalled the series answer read before that import. `LASTNUMBERLIST.LIST` in the Journal
  row and in the Payment row stayed an empty element.
- From that branch's notes, not from an answer: that nothing appeared on Tally's screen at any of
  the six imports, and that a line reading "(e) Data exceptions exist" stood on Tally's Gateway
  screen before the first import, after each import and after the restore; it was not opened.
- **Not established:** a number with a leading zero that has no held twin; Receipt and Contra;
  a number with a leading or inner space; duplicates prevented under "Automatic (Manual
  Override)"; a file of several vouchers, one with a held number; Multi-user Auto; a deletion;
  Gold; other releases; macOS; the same through ComplyEaze Bridge's own post. A number in small
  letters with no held twin, a Journal whose number only a Payment holds, a Payment sent a number
  another Payment holds, and a number with an end space were sent in the next run (next
  paragraph).

**Under Manual numbering with duplicates prevented, the held-number check did not cross Journal and
Payment in either direction, a number in small letters was not refused in itself, and a number's end
space did not come back: verified 2026-10-15 on one book and two voucher types (PARTIAL).** On the
book of the two runs above (licensed TallyPrime 7.1 Silver, Windows), holding the same three
vouchers, six imports were sent raw, one per step, each of one voucher that carried a
`VOUCHERNUMBER` element and a new `REMOTEID`: four Journals and two Payments (branch
`lab/724-m9p-matching-within-a-type-answers`, `3434d39e`, bridge#724; every figure was read from the
marks, voucher, series and import answers with an XML parser, with each size checked against
`CONTENT-LENGTH`; each import file on the branch has the SHA-256 the run sheet gives it). The series
of the Journal type and of the Payment type each read `Manual`, `Auto Retain`, `PREVENTDUPLICATES`
`Yes` at every read, and no series row differed from the first series answer at any read. That no
type screen was saved in this run, and that the book was put back at the end by restoring a backup,
are from that branch's notes; the voucher answer read after the restore is byte-identical to the
first one.

- **VERIFIED, a Payment against a Payment:** a Payment numbered `M9P-P1`, dated 20 July, which no
  voucher held, was created and read back its number; a second Payment numbered `M9P-P1`, dated
  21 July, was refused whole: `CREATED` 0, `EXCEPTIONS` 1, every other counter 0, no `LINEERROR`, in
  an answer byte-identical to the refusals of the two runs above (538 bytes, SHA-256 `09a713e2…`),
  which names no cause. Everything under `COLLECTION` in the voucher answer equalled the answer
  before it.
- **VERIFIED, a number held only by a voucher of another type:** a Journal numbered `M9P-P1`,
  dated 22 July, sent while only that Payment held the number, was created and read back
  `M9P-P1`; the Payment was unchanged. With the run above, where a Payment took a number only a
  Journal held, the check did not cross these two types in either direction on this book.
- **VERIFIED, small letters with no held twin:** a Journal numbered `m9p-x1`, dated 23 July, when no
  voucher held that number in any letter case, was created and read back `m9p-x1` in small
  letters. So the run above's refusal of `m9o-a1` beside a held `M9O-A1` is best explained as a
  match made without regard to letter case; that refusal names no cause, and only one pair was
  sent, small letters against a held number in capitals (ASCII letters only).
- **VERIFIED, an end space:** a Journal numbered `M9P-S1 ` (seven characters, one space at the end),
  dated 25 July, was created; the voucher answer returns `M9P-S1`, six characters, with no space.
  A Journal numbered `M9P-S1`, dated 26 July, was then refused in the same 538-byte answer. Whether
  Tally drops the space when it imports the number or when it exports it is not told apart.
- **VERIFIED:** each of the four answers to a created voucher read `CREATED` 1, `LASTVCHID` the new
  voucher's `MASTERID` (4 to 7 in the order sent), and every other counter 0. No voucher already in
  the book changed at any import.
- From that branch's notes, not from an answer: that nothing appeared on Tally's screen at any of
  the six imports, and that the line reading "(e) Data exceptions exist" stood on Tally's Gateway
  screen before the first import, after each import and after the restore; it was not opened.
- **Not established:** a leading or inner space, or other white space; letters outside ASCII; a
  number sent with an end space while the same number without it is held; a number with a leading
  zero that has no held twin; Receipt and Contra; two child types of one parent, and a type with
  several series; duplicates allowed; duplicates prevented under "Automatic (Manual Override)"; a
  file of several vouchers, one with a held number; Multi-user Auto; a deletion; Gold; other
  releases; macOS; the same through ComplyEaze Bridge's own post.

**Switching a type that holds vouchers to Manual blanks their numbers: verified 2026-10-12, one
observation.**
- Setup: a licensed TallyPrime on Windows, a synthetic company. Its Payment type, holding 32
  Payments numbered 1 to 32, was altered from Automatic to Manual on the screen.
- Tally asked twice before saving:
  - "Changing the method of voucher numbering will affect voucher numbers of existing
    transactions. Continue?"
  - then, on saving, "Are you sure you want to update the voucher numbers of existing vouchers?
    Y: Accept Master & Update Vouchers / N: Back".
- **VERIFIED:** after Y, all 32 stored voucher numbers read back blank (`""`).
  - Unchanged: every voucher's `MASTERID`, `ALTERID`, `GUID`, date, amounts and narration.
  - The company's voucher mark (`ALTVCHID`) did not move, at 87.
  - Only the master mark (`ALTMSTID`) moved, 224 to 226.
  - Restoring the backup taken before the change brought the numbers back. The read after the
    restore was byte-identical to the read before the change.
- **VERIFIED, this sitting only:** on that type the screen showed "Prevent creating duplicate
  Voucher Nos." as No, and the field never took focus (keyboard or mouse). So Manual with
  duplicates prevented could not be set by hand there. The capture above shows the same field
  set on a Payment screen elsewhere, so this is not a rule.
- A switch to "Automatic (Manual Override)" also blanked the numbers of a type's vouchers, on
  2026-10-13 (three Journals, another synthetic book; the paragraph above), again with no
  `ALTERID` change and `ALTMSTID` moving by 2.
- **Consequence:** a stored voucher field can change while neither the voucher's `ALTERID` nor
  `ALTVCHID` moves.
  - **Checked in the code, 2026-10-12:** no ComplyEaze Bridge read reuses a number across such a
    change. The held `vouchers` pages compare both company marks, so this switch's `ALTMSTID`
    move refuses or re-reads them; the mirror re-reads and hashes content; and the
    `changed_since` cursor is disabled.
  - A change that moves neither mark would still be served from held pages until they expire,
    as already documented for them.
  - A native post is unaffected: it supplies no voucher number, and its read-back compares the
    number only when the batch supplied one.
  - **Known limit:** an amendment's build-time check treats an unchanged `ALTERID` as "not
    altered since verified", and compares the number only when the batch supplied one. So a
    blanked number does not stop such a build. Posting an amendment natively is refused, so no
    post follows from it.
- Evidence: the never-merged branch `lab/1c-numbering-evidence` (`s1c-RESULT.md`,
  `s1c-pre-payment-numbers.txt`, `s1c-after-manual-payment-numbers.txt`).

**A numberless native post under Manual and Automatic numbering: verified 2026-10-12, one post per
setting (PARTIAL; VERIFIED applies to each bullet as marked).** The rule above is about a flow that supplies the voucher number. A native post
supplies none: it refuses a supplied `VOUCHERNUMBER` (see "Native mutation selector" above), so Tally
numbers every voucher it creates.
- Setup: licensed TallyPrime 7.1 Silver on Windows, new synthetic companies, each loaded alone; one
  ComplyEaze Bridge build, `bridge_mcp` SHA-256 `411f7cca…`; single native posts with no voucher
  number. Each type's setting was set or read on Tally's own screen.
- **VERIFIED:** the first numberless voucher of a type is created and numbered `1`, with Manual
  numbering and duplicates prevented (a Payment and a Journal) or allowed (a Payment).
- **VERIFIED, Manual with duplicates allowed (Payment):** the second numberless voucher was created
  and numbered `1` again. Both read `posted_verified`, because a post's readback compares the number
  only when the batch supplied one.
- **VERIFIED, Manual with duplicates prevented (Journal):** the second numberless voucher was refused
  whole: `CREATED` 0, `EXCEPTIONS` 1, every other counter 0, no `LINEERROR`. Tally's 538-byte answer
  has the same SHA-256 (`09a713e2…`) as earlier whole refusals of zero-value vouchers
  (`lab/1342-capture-answers`), so it names no cause; that the cause is the repeated number is
  inferred. On 2026-10-14 a Journal that carried a number already held drew an answer of the same
  size and hash (above).
- **VERIFIED, Automatic with "Retain Original Voucher No." on insertion and deletion (Journal):** a
  second numberless voucher was numbered `2`, and a third dated before both was numbered `3`. The
  first two kept their numbers and AlterIDs, and a later read-back still read them `posted_verified`.
- Not established: Receipt and Contra under Manual numbering by a native post (raw imports of
  both, and a file of two Contras, were measured on 2026-10-14 with duplicates allowed, above:
  each was numbered 1); a batch by a native post; Multi-user Auto; Gold; a file imported through
  Tally's Import menu. Automatic (Manual Override) was measured on
  2026-10-13 by a raw import, not by a native post: a Journal without a number read back with
  none (above). Automatic set to renumber on
  insertion was measured on 2026-10-13 for two voucher types, not by a native post (above):
  Receipt, by hand entry and by raw imports (one voucher, a file of two, and a voucher carrying a
  number), and Journal, by one raw import of a voucher without a number.
- Evidence: the never-merged branches `lab/724-m-manual-numbering-answers` (`2fd954f9`),
  `lab/724-m3-second-voucher-answers` (`6ab6a667`) and `lab/724-m4-m5-numbering-answers`
  (`3684c58c`, under `m5/`); bridge#724.

### 9.13 Payment, Receipt and Contra — the bank-statement voucher shapes

**VERIFIED 2026-09-10 (licensed TallyPrime 7.1 Gold; five files imported by hand through
Gateway of Tally → Import → Vouchers).** **157 vouchers** in total, counted from the retained
files themselves: **147** of the three bank types across four files — a 1-voucher pilot, 61
Payments with 54 Receipts, 3 Contras, and 20 Payments with 8 Receipts — plus **10** reallocation
Journals in a fifth. `CREATED` equalled the voucher count on every file with zero errors and zero
exceptions, and each affected bank ledger reproduced, on readback, the debit total, credit total
and closing balance its own statement printed.

*(An earlier summary headlined 148. That figure does not reconcile with its own per-file table
or with the artifacts, both of which give 157; the count above is taken from the files.)*

**A bank statement cannot be expressed as Journals.** Booking bank lines as Journals reconciles
arithmetically and misfiles every one of them: wrong voucher register, wrong day book grouping,
and visibly unlike the book's existing entries. The type decides which side holds the money:

| statement line | voucher | entries |
| --- | --- | --- |
| withdrawal | **Payment** | Dr party, Cr bank |
| deposit | **Receipt** | Dr bank, Cr party |
| own-account or cash movement | **Contra** | both legs cash/bank |

The imported element shape, per voucher:

```xml
<VOUCHER VCHTYPE="Payment" ACTION="Create" OBJVIEW="Accounting Voucher View" REMOTEID="...">
  <DATE>20260801</DATE>
  <EFFECTIVEDATE>20260801</EFFECTIVEDATE>
  <VOUCHERTYPENAME>Payment</VOUCHERTYPENAME>
  <PARTYLEDGERNAME>...</PARTYLEDGERNAME>       <!-- omitted on Contra -->
  <NARRATION>...</NARRATION>
  <ALLLEDGERENTRIES.LIST>
    <LEDGERNAME>...</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>-100000.00</AMOUNT>
  </ALLLEDGERENTRIES.LIST>
  ...
</VOUCHER>
```

Four properties of it are not guessable, and each was measured:

1. **A debit is `ISDEEMEDPOSITIVE Yes` with a NEGATIVE `AMOUNT`.** The flag and the sign say the
   same thing and must agree; they are not independent fields.
2. **`EFFECTIVEDATE` accompanied `DATE` on every voucher**, always equal to it. Whether these
   types import without it was not tested, so omitting it is outside this measurement.
3. **No `<VOUCHERNUMBER>`.** These types numbered automatically in the observed book, and §9.8
   established that the method `Automatic` discards a supplied number in silence (under the
   method `Automatic (Manual Override)` one raw import of a Journal kept its number,
   2026-10-13). Tally assigned
   its own. The bank's reference goes in the narration, which survives.
4. **`REMOTEID` on every voucher.** Delete by the client-supplied `REMOTEID` is the correction
   path, and §9.12b confirms it on a live book — the value you sent stays addressable as a delete
   key even though the export shows Tally's own. Use Delete + Create because it is the confirmed
   path, not because `Alter` is known to fail: §9.12b is explicit that `Alter` is unverified on a
   licensed profile rather than ruled out. A batch imported without a `REMOTEID` has no correction
   path at all, which is the point of sending one.

   **But the readback does not echo it in that attribute, and that is a trap.** A `Voucher`
   collection returns a `REMOTEID` attribute holding *Tally's own* `<company GUID>-<master id>`
   identifier, not the value the client sent. From the committed live capture
   `fixtures/agent/native-namespaced-journal.utf16le.xml`, a readback of a voucher Bridge
   imported:

   ```
   REMOTEID attribute : <TALLY-COMPANY-GUID>-<MASTER-ID>
   NARRATION          : ... [BRIDGE:<CLIENT-BATCH-KEY>]
   ```

   The values above are redacted illustrative placeholders; they preserve the observed distinction between Tally's company-GUID/master-id attribute and the client batch key in narration.

   **The client key is not absent from the response — it is in the narration.** §9.8 records
   that this batch used the same batch-derived UUID for its `REMOTEID` and its narration marker,
   and the marker came back intact while the attribute did not. The rule is field-specific and
   more useful stated that way: **Tally overwrites the attribute it owns, and preserves a marker
   placed in a field it does not.** That is why Bridge attributes readback by the narration tag —
   a deliberate choice of a durable carrier, not a workaround for a missing one. (Since
   2026-10-02 a native post carries no tag and is attributed by its own AlterID span instead;
   see §9.15. The hand-import file keeps the tag.)

   A verifier that compares the observed attribute against the value it sent therefore refuses
   every legitimate readback. The client value is still *stored*, still matches for a
   byte-identical repeat (§9.3), and still deletes (§9.12b); it is only unreadable through this
   attribute.

§9.1b applies unchanged and bites hardest here: a single unescaped `&` in a counterparty name
rejects the whole file with no field hint.

**Correcting a posted batch — reallocation Journals, not delete-and-recreate.** Lines whose
counterparty could not be identified were booked against a suspense ledger and corrected later
by a Journal moving the amount off suspense onto the real ledger. That leaves the bank side
untouched, keeps the correction auditable, and sidesteps the no-Alter restriction entirely; 10
such Journals were verified the same day.

**Naming the company does not aim the write.** The generated envelope carries
`<SVCURRENTCOMPANY>`, and §9.11d records a *verified* case of a mismatched value posting into the
**loaded** company with `CREATED=1, ERRORS=0, EXCEPTIONS=0`. **Which** mismatches behave that way
is UNVERIFIED: a separate measurement had an existing-but-unloaded name fail closed, but the
silent case's instance was never enumerated, so "the name matched nothing" is an inference from
its shape. Do not reason about which *kind* of wrong name is dangerous — there is a verified
silent case and no rule saying when it applies. Bridge cannot guard a hand import it never sees,
so the check is the operator's: immediately before importing, read the company and compare the
**complete identity tuple** `(canonical_origin, COMPANYNUMBER, GUID, NAME, BOOKSFROM)` — not the
GUID alone, because §9.11b is verified that a year-end split gives the child its parent's GUID.
`verify_import` takes a GUID as its argument but already enforces the rest: it refuses with
`company_identity_mismatch` unless the name, GUID, company number and books-from recorded at
build time all still match what the instance reports, so a readback aimed at a split sibling
fails closed. Per §9.11d, neither check closes the window between the check and the send; it is
narrowed only by running on an instance where no other company is loaded.

**Scope and limits.** One company, one build, two statement layouts, and files imported through
the UI rather than dispatched by Bridge. Bill-wise allocation was never exercised — every party
amount landed On Account, which is **not** established as correct for a book that reconciles
bills. This qualifies the three file shapes. It does not qualify a Bridge dispatch of them,
which remains one unnumbered Journal (§9.8).

**Which mixes are qualified.** One file may carry more than one voucher type — two of the
measured files did, 61 Payments with 54 Receipts and 20 with 8, both importing clean. The three
Contras and the ten reallocation Journals each went in on their own file, so:

| file contents | basis |
| --- | --- |
| Payment + Receipt | **observed** |
| Contra alongside either | **inferred** — Contra renders a strict subset of the Payment shape (same envelope and elements, minus the party), and a statement carrying a transfer line is the ordinary composition |
| Journal alongside any of the three | **refused** — a Journal renders no `EFFECTIVEDATE`, names no party, may carry a number and a reference, and comes from §9.8's separate lineage. Holding both citations is not evidence for their union |

**What Bridge builds from it.** `build_import_xml` renders exactly this shape for Payment,
Receipt and Contra, and leaves the Journal shape byte-identical to the file §9.8's own
measurement ran on. Each of the three is admitted with two or more entries, at least one debit
and one credit, no ledger on both sides, and every leg classified (bridge#466: the multi-entry
rule and party choice are the owner's decisions of 2026-09-22; one Bridge-built three-entry Receipt
was imported over the gateway and verified live, and Tally read the bank ledger back as its party,
but no multi-entry Payment or Contra has been, and none of the three, including that Receipt, through Tally's Import menu), carrying neither a supplied voucher number nor a `REFERENCE`: no file carrying a `REFERENCE` has been
imported and read back on these types. A Receipt carrying a voucher number was, raw, on 2026-10-13
(§9.8: Tally discarded the number under `Auto Retain` and under `Auto Renumber`); `verify_import`
compares accounting entries rather than those annotations, so nothing downstream would notice
Tally dropping or rewriting one.

Both legs are classified, not just the funding one:

| leg | requirement | refused by |
| --- | --- | --- |
| Payment credit, Receipt debit, both Contra legs | must be a **admitted** money group | anything else, **including "could not be established"** — a positive fact is required and absent |
| the counterparty leg of a Payment or Receipt | must be **established as holding no money** | any known money group, **and "could not be established"** |

Both legs need a positive fact; they differ only in which one. An earlier version made the
counterparty rule the looser of the two — refusing only a leg established *as* money — on the
reasoning that an unclassifiable counterparty is not evidence of a disguised Contra. Both halves
of that were weaker than they sounded. An ordinary party never lands unclassified: one under
`Sundry Debtors` resolves directly, and one under a user-created group walks up to its reserved
ancestor, so only anomalies reach that state. And the consequences are not symmetric — a
misjudged money leg makes Tally reject the import, which is loud, while a misjudged counterparty
files a Contra into the Payment register, which is silent and found later. The silent failure
earns the stricter rule.

The two columns also ask different questions of the same group, and that gap matters.

**Admitted** means a captured ledger was observed sitting under a captured group — the whole
edge the classifier walks, not just its far end. A group row proves the identity exists; it does
not show a ledger's `PARENT` resolving to it. Three identities clear that bar: `Bank Accounts`,
`Cash-in-Hand` and `Bank OD A/c`.

**Known money** is wider, and covers one more:

| identity | group row captured | ledger under it captured | admitted |
| --- | --- | --- | --- |
| `Bank Accounts`, `Cash-in-Hand` | yes | yes | yes |
| `Bank OD A/c` | yes | yes — 2026-09-17 | yes |
| `Bank OCC A/c` | **no** | no | no |

> **Scoped correction, 2026-09-17 — `Bank OD A/c` is admitted.** This row used to read "ledger
> under it captured: **no**", and an overdraft or cash-credit book could not be imported through
> Bridge. The promoting read was taken from the synthetic `BRIDGE SHAPE LAB` on licensed TallyPrime
> 7.1 Silver: the `StandardLedgerCatalogV1` request and the native `List of Groups` request the
> build renders, each sent twice with byte-identical responses. The catalogue carries `HDFC CC`
> with `PARENT` `Bank OD A/c`, and the group collection carries that group with `RESERVEDNAME`
> `Bank OD A/c`. Both are committed as
> `tests/fixtures/agent/native-shape-lab-{ledger-catalogue,groups}.utf16le.xml`. One company, one
> release: a book whose overdraft ledger sits under a user group beneath `Bank OD A/c` walks the same
> ancestry and is classified the same way, but that shape was not itself captured.

A ledger under the unadmitted identity is refused on a money leg for want of an observed edge,
and refused on a counterparty leg because it plainly holds money. Both refusals are the same
ignorance pointed in the safe direction; reading "not admitted" as "not money" would wave through
exactly the bank-to-bank Payment the counterparty rule exists to catch.

Classification walks the ledger's group ancestry through `RESERVEDNAME` per §8.2b, so a renamed
predefined group still classifies. A book whose money ledger sits under a group the Group
collection does not carry at all is refused the same way.

**A Journal between two bank ledgers was accepted at a new company's own settings. Measured 2026-10-12 on Windows, one new synthetic company loaded alone, one voucher posted, one run. Confidence: PARTIAL, and the run's answers are not committed and are on no evidence branch, so this paragraph cannot be re-checked from the repository.** The release and licence, TallyPrime 7.1 Silver and licensed, are from the status reads of another run in the same lab that day (14:07Z to 14:59Z in the log of `lab/masters-m1-answers`, `e783e7cb`). The run's own record states neither, and no committed file shows that the two runs used one installation. The company's voucher types were as Tally made them: no voucher-type or company setting was changed. It held one other voucher, a Receipt keyed by hand (by the run's instructions, on another date and naming neither bank ledger). One Journal of 1.00, debiting one ledger under `Bank Accounts` and crediting another, was built and posted through ComplyEaze Bridge. The post reported one voucher created and `posted_verified`, and the Journal read back with the debit on one bank ledger and the credit on the other.
- So Tally did not refuse money on both sides of a Journal at these settings. The leg rules above are ComplyEaze Bridge's rules for the Payment, Receipt and Contra shapes it builds; this run does not show whether Tally enforces any of them for those types. A transfer between two of a book's bank ledgers can therefore sit in the book as a Journal, as well as the Contra the table at the top of this section names.
- **Not established:** a Payment or a Receipt between two bank ledgers, which ComplyEaze Bridge refuses to build (the counterparty rule above) and which was not tried another way in this run; a Journal type with other settings; a second voucher or run; Gold and Education; other releases.

**What that check does not cover**, written here because a passing verdict invites being read
for more than it proves:

- It is a **group** check, not a ledger-suitability check. A bill-wise party, a foreign-currency
  bank account and a ledger requiring cost-centre allocation all classify identically to one
  needing none of that. It answers where the ledger sits, not whether Tally can use it here.
- It is true **as of the read**. The build reads the masters twice and refuses if they moved,
  which establishes stability across the build and nothing after it. The file is imported by
  hand later and Bridge never observes that import, so a ledger regrouped in between — an
  ordinary operation — leaves a stale verdict with no later gate. `verify_import` compares
  entries and would not notice a party that has since become a bank ledger.
- An **incomplete read refuses rather than admits**: a group missing from a truncated collection
  reads as unresolvable ancestry. The failure mode of a partial read is a rejected batch, never
  an accepted one.

**What it still does not do.** Two gaps, both stated here rather than left to be discovered.

A party ledger configured for bill-wise accounting is now detected (6 Oct 2026, #1234): the ledger
catalogue request carries one more native method and returns each ledger's `ISBILLWISEON`
(§12a.15, live-captured), so the build refuses a batch naming such a ledger until each party is
approved, and the post and the queue read the flag again. Every party amount on a bill-wise
ledger still lands On Account, exactly as the measured import did; the approval only records that
a person was asked, and a hand import of the file is not checked. *(Written before that change: "Nothing
detects a party ledger configured for bill-wise accounting; every build naming a counterparty says so in its
warnings.")*

**Two written elements are not verified: `EFFECTIVEDATE` and `PARTYLEDGERNAME`.** *(Written before
#467 and #469. `EFFECTIVEDATE` is now verified and `PARTYLEDGERNAME` is still not; see the two
scoped corrections below.)* The
verification collection of §9.8 fetches neither, so `verify_import` compares the date, voucher
type and signed entries and cannot see whether Tally kept, rewrote or dropped either — nor
whether an operator later edited them. A readback with a wrong effective date, or a party
silently dropped, still reports `posted_verified`.

The two are not equally unknown, and the difference decides how to close them:

| element | is it returned by a voucher collection? |
| --- | --- |
| `PARTYLEDGERNAME` | **yes, observed** — a captured `Sales` readback carries it populated. Whether the three bank types echo it is not observed |
| `EFFECTIVEDATE` | **unobserved** — no captured response in this tree carries it |

Neither may be fetched and *required* on that basis alone. Requiring an element that a
response does not return refuses every legitimate verification, which is a worse failure than
the one it guards — the same trap as comparing `REMOTEID` above, where the field comes back
carrying Tally's value rather than the client's. Both close with one live read that adds them to
the `FETCH` list and looks at what arrives.

> **Scoped correction, 2026-09-17 — both were read back, and they differ.** The table above
> predates this read. Licensed TallyPrime 7.1 Silver, synthetic `BRIDGE SHAPE LAB`: the §9.8
> verification collection's request, with `EFFECTIVEDATE` and `PARTYLEDGERNAME` appended to its
> `FETCH`, sent once over `20250422..20250423`. The window held six vouchers. Three were written by
> earlier lab runs with narrations of their own (a Contra, a Journal and a Sales voucher). Their
> written values were not recorded here, so they are not tabulated, but each also returned
> `EFFECTIVEDATE` equal to `DATE`. The three Bridge-built vouchers, found by their `[BRIDGE:…]`
> markers, are the rows below. The Receipt and Payment were built by
> `bridge_mcp`, imported from its file and amended from a Bridge-built file; the Payment was last
> re-imported from a copy of that file with only its amount edited. The Contra was built by
> `bridge_mcp` and imported from its file. Every file wrote the `PARTYLEDGERNAME` and
> `EFFECTIVEDATE` shown here:
>
> | voucher | written `DATE` / `EFFECTIVEDATE` | returned `EFFECTIVEDATE` | written `PARTYLEDGERNAME` | returned `PARTYLEDGERNAME` |
> | --- | --- | --- | --- | --- |
> | Receipt | `20250422` / `20250422` | `20250422`, `TYPE="Date"` | the counterparty (`Shape Buyer 5`) | **the bank ledger** (`Bank of Baroda CA`) |
> | Payment | `20250422` / `20250422` | `20250422`, `TYPE="Date"` | the counterparty (`Power Charges`) | **the bank ledger** (`Bank of Baroda CA`) |
> | Contra | `20250423` / `20250423` | `20250423`, `TYPE="Date"` | none written | the debit bank ledger (`HDFC CC`) |
>
> - **`EFFECTIVEDATE` is returned, equal to `DATE`**, on all three types. Comparing it is now
>   possible. `verify_import` did not fetch it when this read was taken; #469 closed that, see the
>   next correction.
> - **`PARTYLEDGERNAME` is returned but does not echo what was written.** On these vouchers it held
>   a cash or bank ledger that was on the voucher, not the counterparty Bridge wrote. **Do not
>   compare it with the written value:** that comparison would refuse every one of these legitimate
>   vouchers, which is the `REMOTEID` trap again. Whether a readback can detect a silently dropped
>   counterparty is still open. The entries comparison already requires the counterparty's ledger
>   among the signed entries.
>
> One read, one company, one release, three Bridge-built vouchers in a six-voucher window. It does not establish
> what a Tally UI edit to either field returns.

> **Scoped correction, 2026-09-17 — `EFFECTIVEDATE` is now verified (#469).** A live read
> (#467: licensed 7.1 Silver, synthetic lab) returned `EFFECTIVEDATE` with `TYPE="Date"`, equal
> to `DATE`, on a Bridge-built Receipt, Payment and Contra. The committed capture
> `native-three-vouchers.utf16le.xml` also carries it on two of its three vouchers. The §9.8
> verification read now appends `EFFECTIVEDATE` to its `FETCH`. For Payment, Receipt and Contra, a
> returned value that differs from the written date is an `effective_date` diff, in `verify_import`
> and in the amendment compare-and-swap. An absent or empty element is **not** a diff, because
> requiring it would refuse every verification on a release that does not return it; the row
> reports `"not_observed": ["effective_date"]` instead. A Journal is written without the element
> and is neither compared nor flagged. The public voucher tools do not fetch it or return it.
> `PARTYLEDGERNAME` stays unfetched: on the same read it held a bank ledger, not the written
> counterparty.

> **Scoped correction, 2026-09-25 — Bridge-built files and `verify_import` on licensed Gold.**
> The 2026-09-10 evidence above was a hand import of files Bridge did not build. Two later field
> runs used Bridge's own output. Both were on production books (not synthetic), on licensed
> TallyPrime 7.1 Gold, with the owner's authorisation. Each file was built by `build_import_xml`
> against the live book and then sent **unchanged** over the XML gateway by a script. They were not
> sent by `post_import`, and there was no native approval.
>
> - **VERIFIED 2026-09-16, one book (Bridge master `f25fc91a`).** A statement's 68 lines went in
>   as a one-voucher pilot, then a batch of 67 (Payment, Receipt and Contra). The responses were
>   `CREATED=1` and `CREATED=67`, with every other counter zero and no `LINEERROR`. `verify_import`
>   returned `posted_verified` for 1/1 and 67/67, with no duplicates, and `alter_id_delta` equal to
>   the voucher count. The bank ledger's closing balance equalled the statement's to the paisa.
> - **PARTIAL 2026-09-17, a second book (Bridge master `d1327a92`).** `verify_import` returned
>   `posted_verified` on pilot batches of two to four vouchers. On the same book it failed with
>   `agent_runtime_read_failed` once the batch window held a few hundred vouchers, and at about
>   4,900 vouchers `ledger_movement` failed the same way (bridge#485). The larger batches were
>   proven by a full voucher readback instead.
>
> So a live monetary read of a Gold book (`verify_import` over vouchers with amounts) and the
> gateway import of Bridge-built Payment, Receipt and Contra files are both observed on Gold. Two
> things are not:
> - **Native posting.** `post_import` with its approval had not been observed on Gold, on Windows
>   or on Education when this correction was written (2026-09-25). A native post (a batch with a
>   recorded `dispatch_intent`) was later captured on Gold on 2026-09-28 (§9.14's recorded-list item:
>   one client book, a debug build). The approval step was not recorded.
>   Windows and Education remain unobserved.
> - **Reads at scale.** Reads of books of thousands of vouchers are the open failure in bridge#485.
>
> Neither run exercised a second Tally user writing during the read. That is the case Gold adds
> over Silver (bridge#239). This correction promotes no claim.

### 9.9 Bulk import throughput

**VERIFIED.** One import request may carry many `<VOUCHER>` elements; the counters aggregate.

| Objects per request | Elapsed | Rate |
| --- | --- | --- |
| 1 | 0.53 s | 2/s |
| 5 | 0.57 s | 9/s |
| 25 | 0.83 s | 30/s |
| 100 | 2.08 s | 48/s |

Per-request overhead dominates at small batch sizes. Extrapolating, 10,000 vouchers is
roughly 3–4 minutes and 100,000 roughly 35 minutes — bulk test-corpus generation is
practical.

**This is for test-data generation only.** Bridge's production write path remains
batch-of-one, because import counters are unattributable at N>1: a request carrying 100
objects that returns `CREATED=99` gives no way to identify which one failed.

### 9.7 Operation support matrix — **vouchers cannot be modified**

> **Qualified by §9.14 and §9.5 (2026-09-25):** on licensed TallyPrime 7.1 Silver, a same-`REMOTEID`
> `ACTION="Create"` alters a voucher in place (`ALTERED=1`, same GUID), and `ACTION="Cancel"` with
> the full body cancelled in place (observed once). The matrix below records the earlier instance.

**VERIFIED.** Every cell tested directly; "verified" below means the resulting state was read
back and confirmed, not merely that a counter moved.

| Object | Create | Alter | Cancel | Delete |
| --- | --- | --- | --- | --- |
| **Master** (`Ledger`) | ✓ `CREATED=1` | ✓ `ALTERED=1` — keyed by **name**; parent change confirmed by readback | n/a | ✓ `DELETED=1` — keyed by name; absence confirmed by readback |
| **Voucher** | ✓ `CREATED=1` | ✗ **`CREATED=1`** — a duplicate is made, target untouched | ✗ **`CREATED=1`** — a new cancelled voucher is made, target untouched | ✓ `DELETED=1` — keyed by `REMOTEID` |

**Voucher Alter was tested with four different keys** — `MASTERID` element, `GUID` element,
`REMOTEID` element, and `REMOTEID` attribute combined with `MASTERID`. All four returned
`CREATED=1, ALTERED=0`. Each produced a duplicate voucher.

`REMOTEID` *is* a valid match key — Delete succeeds with it. So the failure is specific to
the Alter and Cancel operations, not to identification.

**Tally can report a failed match correctly** when it chooses to: deleting a non-existent
master returned `ERRORS=1` with `LINEERROR="Item does not exist!"`. Voucher Alter and Cancel
do not take that path; they create.

**Consequences for the plan's write design:**

1. **§3.1.2's ruling that Cancel is the compensation primitive is inverted by the evidence.**
   Cancel does not work; Delete does.
2. **The "Alter-by-GUID with Cancel+Create fallback saga" has no working leg** on this
   instance. Neither Alter nor Cancel functions.
3. The only working voucher-correction path here is **Delete + Create** — precisely what the
   plan sought to avoid, because it destroys the original rather than superseding it.
4. Masters behave as the plan assumes: name-keyed, alterable, deletable.

**UNVERIFIED and now critical:** whether this is Education-mode behaviour, Edit Log SKU
behaviour, or general. A SKU whose entire purpose is an immutable audit trail plausibly
refuses XML-driven voucher alteration by design. **Qualifying Alter/Cancel on licensed
standard TallyPrime is now a Phase 4 gate, not a nicety.**

**Every one of these failures is caught by the §9.2 rule** and by nothing weaker: the
intended counter (`ALTERED`, `CANCELLED`) never increments, while `ERRORS` and `EXCEPTIONS`
stay at zero. That rule has now caught three distinct silent-failure modes.

### 9.6 `ACTION="Cancel"` by `REMOTEID` creates a new voucher — it does not cancel — **TRAP**

> **Qualified by §9.14 (2026-09-25):** on licensed TallyPrime 7.1 Silver, `ACTION="Cancel"` with
> the full voucher body cancelled in place (`ALTERED=1`, `CANCELLED=0`; observed once). The result
> below is from an earlier Education/Edit Log instance with a different request.

**VERIFIED.** A Cancel request naming an existing voucher's `REMOTEID` returned
`CREATED=1, CANCELLED=0, ERRORS=0, EXCEPTIONS=0` and **created a new voucher** carrying
`ISCANCELLED=Yes`. The targeted voucher was untouched — same `AlterID`, still
`ISCANCELLED=No`.

Two consequences:

1. **The correct identifier or request shape for Cancel is not yet established.** The plan
   designates Cancel as the write-path compensation primitive; until this is solved, Cancel
   cannot be relied on for that role. A compensation that silently *creates* data is worse
   than no compensation.
2. **This validates the §9.2 success rule.** A check of `ERRORS==0 && EXCEPTIONS==0` would
   report success. Only the "intended counter incremented" clause catches it, because
   `CANCELLED` stayed at 0 while `CREATED` moved.

**UNVERIFIED:** whether `ACTION="Cancel"` works when keyed by `GUID`, `MASTERID`, or with a
fuller voucher body. Not yet tested.

**Lead found 2026-07-30.** Tally's official Sample XML page
(`help.tallysolutions.com/sample-xml/`) publishes canonical samples for **"Voucher
Alteration"** and **"Voucher Cancellation"** among its 20 request examples. Our failing
attempts were hand-built. **Before concluding Alter/Cancel is broken on this SKU, retry using
the official sample shapes verbatim** — the §9.5/§9.6 failures may be a request-shape defect
on our side rather than an Edit Log restriction. This is the cheapest next step on the write
path and it should be taken before the Phase 4 licensed-Tally gate.

### 9.17 Tally's GSTR-1 flag expects a Sales invoice's tax per sales line, rounded half-up, and flags a figure that differs from it

**PARTIAL, and narrow: one synthetic company, licensed TallyPrime 7.1 Silver (the earlier rounds on that
lab record the release and licence; these answers do not show them), CGST plus state tax within the
company's own state, 5% services lines (one 18% goods line on a bill whose figures are exact), the invoice
view, one book, one run per voucher. A "line" is a sales-ledger line in an Accounting Invoice
(`VCHENTRYMODE`), with no stock items; the buyer is Unregistered/Consumer and the state Rajasthan on every
voucher. The "Tax entered" column comes from the sitting's own record of how each voucher was made, not
from these answers: the export cannot show who filled a figure.** Sixteen Sales vouchers were read back in one full voucher export;
Tally's own GSTR-1 flag is in that export (`VCHGSTSTATUSISINCLUDED`, `VCHGSTSTATUSISUNCERTAIN`,
`VCHGSTSTATUSISEXCLUDED`) and was checked against the uncertain list on Tally's screen, which named the
same six vouchers with the reason "Mismatch between Expected Tax Amount and Modified Tax Amount in
transaction". "Per line" is `half-up(line x rate / 2)` to two decimals for each sales line, summed per rate,
for each of the two heads.

| Voucher | Sales lines | CGST / SGST | Per line | Per bill | Tally flag | Tax entered |
|---|---|---|---|---|---|---|
| 0001 | 1000.00 | 25.00 / 25.00 | 25.00 | 25.00 | included | typed |
| 0002 | 1001.00 | 25.03 / 25.02 | 25.03 | 25.03 | **uncertain** | typed |
| 0003 | 1001.00 | 25.02 / 25.03 | 25.03 | 25.03 | **uncertain** | typed |
| 0004 | 1001.00 | 25.03 / 25.03, round off -0.01 | 25.03 | 25.03 | included | typed |
| 0005 | 1001.00 | 25.02 / 25.02, round off +0.01 | 25.03 | 25.03 | **uncertain** | typed |
| 0006 | 1000.00 | 25.01 / 25.01 | 25.00 | 25.00 | **uncertain** | typed |
| 0007 | 600.00 + 400.00 | 25.00 / 25.00 | 25.00 | 25.00 | included | typed |
| 0008 | 601.00 + 400.00 | 25.03 / 25.02 | 25.03 | 25.03 | **uncertain** | typed |
| 0009 | 600.00 at 5% + 700.00 at 18% | 15.00, 63.00 / 15.00, 63.00 | same | same | included | typed |
| 0010 | 600.40 + 400.40 | 25.02 / 25.02, round off +0.16 | 25.02 | 25.02 | included | imported |
| 0011 | 1001.00 | 25.03 / 25.03 | 25.03 | 25.03 | included | Tally filled |
| 0012 | 100.10 + 100.10 | 5.00 / 5.00 | 5.00 | **5.01** | included | Tally filled |
| 0013 | 100.20 + 100.20 | 5.02 / 5.02 | 5.02 | **5.01** | included | Tally filled |
| 0014 | 100.60 + 100.60 | 5.04 / 5.04 | 5.04 | **5.03** | included | Tally filled |
| 0015 | 100.20 + 100.20, the same ledger on two lines | 5.02 / 5.02 | 5.02 | **5.01** | included | Tally filled |
| 0016 | 100.20 + 100.20 | 5.01 / 5.01 | **5.02** | 5.01 | **uncertain** | typed, the per-bill figure |

- **VERIFIED (this book): for all sixteen vouchers, Tally's flag is "included" exactly when both heads equal
  the per-line figure, and "uncertain" otherwise.** Every flagged voucher was 0.01 off the per-line figure
  on at least one head; nothing larger was seen. The per-bill figure disagrees with Tally on four
  included vouchers (0012 to 0015) and agrees with the one it flagged (0016).
- **VERIFIED (this book, positive amounts, 2.5% heads): the rounding is half-up per line.** `0013`'s lines
  are 2.505 each and Tally's figure is 5.02 (half-even would give 5.00); `0011`'s 25.025 gave 25.03
  (half-even 25.02). `0014`'s 2.515 does not discriminate: half-even gives the same.
- **VERIFIED (this book): per line, not per ledger or per bill.** `0015` carries one ledger on two lines
  and Tally's figure is 5.02 (a ledger total would give 5.01).
- **VERIFIED (this book): a difference of one paisa is flagged.** `0016` differs from the per-line figure by one
  paisa; `0002`, `0003` and `0008` carry heads that differ from each other by one paisa and are flagged.
  No tolerance showed: the four on-screen tax ledgers read `ROUNDINGMETHOD` Not Applicable and
  `ROUNDINGLIMIT` 0 (the one tax ledger created by import reads an empty `ROUNDINGMETHOD` and 0, and is on none of
  these vouchers), and the two company screens read, which show no setting that ignores a difference.
- **A round off line does not decide the flag** (`0004`, `0010` are included with one).
- **VERIFIED (this book): Accept As Is on `0016` alters the voucher and overrides the flag; it does not
  change an amount.** Before: uncertain, `ISGSTOVERRIDDEN` No. After: included, `ISGSTOVERRIDDEN` Yes,
  `ISGSTOVRDNCRC` 0 to a number, `ALTERID` 27 to 28, `ALTVCHID` 27 to 28; every leg and amount unchanged.
- **What follows for a check on an invoice's tax (a reading of this table, not a further measurement).** A
  check that admits the per-bill figure, or two heads that differ by a paisa, or a figure one paisa from
  either head, admits vouchers this table shows Tally flags (the `0002` and `0016` kinds). Whether and how to
  change a given check is a separate decision.
- **Not measured:** inter-state tax, cess, inventory or discount lines, a registered buyer, a rate other than
  5% with a tie, more than one rate with a tie, a sales ledger whose GST rate disagrees with the tax posted
  (the rate sits on the sales ledger; the vouchers carry no rate element), Gold. Do not read the rule beyond what the table covers.

Evidence: the answers (`workorder/vouchers-w1.xml`, `vouchers-w2-before.xml`, `vouchers-w2-after.xml`,
`aw2-marks-*.xml`, `ledgers-w1.xml`) are held on the repository's `lab/1342-capture-answers` branch at
`d78906775`, listed by SHA-256 in its `SHA256.txt`.

### 9.18 A zero-value Sales voucher is refused unless its voucher type allows zero-valued transactions

**PARTIAL: the same book, one run each, over the XML gateway with hand-built files and one file rendered by
Bridge's code.** The voucher type setting is "Allow zero-valued transactions".

- **VERIFIED (this book): all three zero-value Sales imports into `BRIDGE Sales` were refused.** The
  accounting view, the invoice view and the one Bridge's code renders (three ids, each a single voucher
  with a customer leg and one sales leg at 0.00) each answered `EXCEPTIONS=1`, `CREATED=0`, **with no
  `LINEERROR`**, and the voucher mark did not move (28, 28, 28). A refusal with no `LINEERROR` and
  `ERRORS=0` is the §9.2 trap in a new form.
- **VERIFIED (this book): the setting decides.** A voucher type created with the setting at its default (No)
  refused the same zero voucher the same way. A type created with it Yes answered `CREATED=1`
  (`LASTVCHID` 21) and the voucher was found by the next read (accounting view, both legs 0.00).
- **VERIFIED (screen, one try, accounting view): Tally's own data entry refused a zero line in `BRIDGE Sales`**
  ("No Entries in Voucher!"); the invoice view was not tried.
- **VERIFIED (screen): the GSTR-1 document summary lists a voucher type's zero voucher as an issued
  document** (a line for the zero-value type, count 1). The counts for `BRIDGE Sales` fit "included
  documents of a type": 11 before (16 numbered, five uncertain) and 12 after the normal invoice that
  followed. That reading is an inference: the zero voucher's own GSTR-1 flag was not read.
- **Not measured:** the GSTR-1 flag of the zero voucher (the voucher type was not in the export); a zero
  invoice in the invoice view under the setting Yes; a zero line beside a non-zero one; Gold.

Evidence: the requests (`w3-zero-accounting.xml`, `w3-zero-invoice.xml`, `w3-zero-code.xml`,
`w3-zero-type-default.xml`, `w3-zero-type-yes.xml`, `w3-valid-after-refused.xml`) are on
`lab/1342-capture-requests` at `5cc927e5` under `workorder/`; the answers `workorder/aw3-*.xml`,
`vouchers-w3.xml`, `aw3-window-0802-extra.xml` and the screens
`w3-0018-zero-keyed-error`, `w3-voucher-type-z0`, `w3-voucher-type-z1`, `w3-document-summary-before` and
`w3-document-summary-after` on the same branch and commit.

### 9.19 A keyed Purchase invoice, as the voucher export reads it

**PARTIAL: the same book, two Purchase invoices keyed by hand in invoice mode, one run.** This is the shape
Tally stored, not a measurement of Bridge posting a Purchase.

- **VERIFIED from the export:** type `Purchase`, numbered by Tally (1 and 2), the supplier's invoice number
  `SUP/0001` dated 3-Aug-2026 on both (`REFERENCE`, `REFERENCEDATE`), the voucher dated 4-Aug-2026,
  `ISINVOICE` Yes, entry mode Accounting Invoice, view Invoice Voucher View. The supplier leg reads
  1050.00, `ISDEEMEDPOSITIVE` No (a credit); the purchase leg -1000.00 and the CGST and SGST legs -25.00
  each, `ISDEEMEDPOSITIVE` Yes. The header carries the supplier's GSTIN, `STATENAME` and `PLACEOFSUPPLY`
  Rajasthan, `GSTREGISTRATIONTYPE` Regular, the company as the buyer name and its GSTIN as the consignee's.
- **VERIFIED: Tally saved the second invoice with the same supplier invoice number and date.** The screen read
  was taken after the save, so whether a warning appeared before it is not recorded.
- **VERIFIED (screen): the bill reference is `SUP/0001`, and the two invoices share it.** The supplier's
  pending bills show one row, opening 1,050.00 Cr and pending 2,100.00 Cr. **NOT established through
  a read:** the export's bill allocation carries only an amount, and why is not established.
- **Both vouchers read uncertain in GSTR (`VCHGSTSTATUSISUNCERTAIN` Yes). The cause is UNVERIFIED:** a
  duplicate supplier invoice number is unlikely to explain the first one (its `ALTERID` is 31 and the
  second's 32, so it was not re-saved, but no read was taken between the two saves and the flag is
  stored), and the supplier's GSTIN is of the synthetic family, which Tally may not accept as valid.
- **Not measured:** a purchase imported over the gateway, any GSTR-2 view, a Purchase in the accounting-voucher view, Gold.

Evidence: `workorder/purchases-w4.xml` and the screens `w4-*` on the same branch and commit.

### 9.14 A `REMOTEID` upsert re-states the voucher: cancel, optional, delete and recreate

**Scope of every item below.** TallyPrime 7.1 Silver, licensed, not Education, on a synthetic
INR-only company. Journal vouchers from one gateway batch, with Automatic numbering (Auto
Retain, duplicates not prevented). Path: raw gateway requests (G), not Bridge's post path. **One
run per item, one voucher each (2026-09-25); per P5 an anomaly is repeated before it is believed, so
each is PARTIAL until it is.** Counters were read from the saved responses and state
from a Voucher collection readback, not from the counters alone.

**Exception, dated 2026-09-28.** The recorded-list item and the Payment, Receipt and Contra block
below were measured on licensed TallyPrime 7.1 Gold, on one client book. The recorded-list item
used Bridge's native post; the upsert block used the gateway. Each states its own scope.

- **`ACTION="Cancel"` with the full voucher body cancels in place. PARTIAL — observed once (G).**
  - The response reported `ALTERED=1` and `CANCELLED=0`, with all other counters zero.
  - The readback showed the same GUID with `ISCANCELLED=Yes` and its entries gone: one empty entry list, with no ledger and no amount.
  - On this run the `CANCELLED` counter stayed 0; when, or whether, it is ever non-zero is unmeasured. A success rule keyed on it (§9.2) would have called this cancel a failure. Only the readback showed it took effect, so key a cancel on `ALTERED` and the readback.
  - This differs from §9.6, which was measured on an earlier Education/Edit Log instance with a hand-built request. Neither the instance nor the request shape is held constant between the two, so which one causes the difference is **UNVERIFIED**.
- **An upsert (`ACTION="Create"`, same `REMOTEID`) onto a cancelled voucher un-cancels it and restores its entries. PARTIAL — observed once (G).**
  - The response reported `ALTERED=1`.
  - The readback showed the same GUID and number, `ISCANCELLED=No`, and both legs carrying the upserted amounts.
- **`ISOPTIONAL=Yes` by upsert makes the voucher optional and renumbers it. PARTIAL — observed once (G); where the numbers come from is not established.**
  - The response reported `ALTERED=1`, and the voucher number moved from 5 to 815.
  - A later upsert that omitted `ISOPTIONAL` left the voucher optional (omitted fields merge, §9.14 last item) and moved its number from 815 to 816.
  - Where 815 and 816 come from is not established, and the renumbering was seen only under Automatic numbering with Auto Retain. The number cannot be relied on to identify a voucher across an optional change.
- **`ACTION="Delete"` by `REMOTEID` deletes, and an upsert of the same `REMOTEID` then creates a new voucher. PARTIAL — observed once (G).**
  - The delete reported `DELETED=1`, and the voucher was absent from the readback.
  - The upsert reported `CREATED=1`, with a **new GUID**, a **new MASTERID** (7 before, 855 after) and the number 815.
  - A resend therefore undoes a delete, and neither the GUID nor the MASTERID survives it. Anything keyed on either (a baseline, a binding) must treat the re-created voucher as new. The REMOTEID is the only link, and after a delete that link re-creates the voucher rather than restoring it.
  - **Current behaviour:** Bridge's native post sends a fresh random REMOTEID for every post and records it with the dispatch intent (#582), so it never resends one.
  - **The recorded list is in batch voucher order. VERIFIED on one run (licensed TallyPrime 7.1 Gold, 2026-09-28).** For 9 of 9 batches whose import request was captured, `dispatch_intent.native_remote_ids` equalled the captured `REMOTEID` attributes, in order. So the recorded list can address each voucher of a batch. It is VERIFIED rather than PARTIAL because the equality was checked request by request over 9 captured requests, not inferred from one. It is one session on one client book, with the request captured for 9 of the 10 batches (one was not captured), on a debug build of master 4857aff6, so a second book would strengthen it.
  - **Design consequence, not current behaviour:** a batch-posting design must refuse to resend a REMOTEID Bridge has already sent, because a resend after a delete re-creates the voucher under a new GUID.
- **Manual numbering with duplicates prevented refuses, and two of the three refusals are silent. PARTIAL — observed once each (G).** The same Journal type was switched to Manual with Prevent Duplicates, then restored.
  - An upsert (same `REMOTEID`) with no `VOUCHERNUMBER` reported `ERRORS=1` with `LINEERROR` "Voucher Number cannot be left BLANK!", and no create or alter counter moved.
  - A new voucher with no number reported `EXCEPTIONS=1` with **no `LINEERROR`**, and no create counter moved.
  - A second new voucher reusing a number reported `EXCEPTIONS=1` with **no `LINEERROR`**, and a readback held one voucher under that number.
  - An `EXCEPTIONS` count without any line error is the case §9.2's four-part success rule exists for: only the counters show the refusal.
- **Related, same block:**
  - An upsert omitting `REFERENCE` kept the stored value: omitted fields merge. **VERIFIED for a `REFERENCE` written over the gateway**, licensed 7.1 Silver. There are two independent runs: this block's (G), and a 2026-09-26 repeat through Bridge's own `build_import_xml` amendment file, sent unchanged (`ALTERED 1`, reference kept; bridge#239 comment of that date). A `REFERENCE` typed in Tally's own screens is **not measured**.
  - Each upsert moved the voucher's ALTERID to the book's next mark. PARTIAL — observed across several upserts on one book (G).

**Not measured in the Journal block above (see the Gold block below for Payment, Receipt and Contra):** other voucher types, Gold concurrency, an edit in Tally's
own screens, and repeatability beyond one run (except the `REFERENCE` merge above, repeated
once on 2026-09-26). A gateway delete of a master (a ledger or a group) is **not qualified** on any build: do not send one without maintainer approval, and add no Bridge path that does.

**Payment, Receipt and Contra on Gold: an upsert alters in place. VERIFIED on one run (licensed
TallyPrime 7.1 Gold, one client book, 2026-09-28).**
- **What was sent.** The request shape was §9.13's, with the same `REMOTEID` a voucher was created
  with, `ACTION="Create"`, and only the `NARRATION` changed. It was byte-identical to Bridge's own
  native request apart from `NARRATION`: a renderer reproduced every one of the captured native vouchers
  (about 260) exactly.
- **Pilot.** One Payment returned `ALTERED=1`, `CREATED=0`. A vouchers read of that day showed the
  same GUID, the same `VOUCHERNUMBER`, the same amounts and an unchanged day total. Only `NARRATION`
  and `ALTERID` had changed.
- **Then about 300 more** Payment, Receipt and Contra vouchers, in 13 requests, each returned `ALTERED` equal to
  its voucher count and `CREATED=0`.
- **Read-back.** A full vouchers read of the window (about 2,000 vouchers, paged) found each of the
  roughly 300 target vouchers once. Their type, amounts, cancelled and optional flags were unchanged, and none
  still carried the narration tag the upsert removed.
- **What it extends.** It extends the Journal-on-Silver upsert above to Payment, Receipt and Contra
  on Gold, for a narration change only. A changed amount, ledger or date was not sent, and
  neither was a repeat run on a second book.
- **Why VERIFIED.** §9.14 marks a single observation PARTIAL. This one differs because about 300
  vouchers were read back in full. It is still one session on one client book, not repeated on a
  second, so treat it as **Confidence: PARTIAL** beyond that book.

### 9.15 An import fills its own AlterID span in request order

**VERIFIED 2026-10-02, one run (licensed TallyPrime 7.1 Silver, synthetic `BRIDGE AMEND LAB`, raw gateway).**
One `Import Data` request carried 10 Payment, Receipt and Contra vouchers in Bridge's native shape, with print-ready narrations and no `[BRIDGE:…]` tag. Two of the vouchers were identical in every element but the REMOTEID. The capture and its provenance are `tests/fixtures/POST_SPAN_CAPTURE_PROVENANCE.md`.

| Fact | Observed |
| --- | --- |
| Counters | `CREATED=10`, every other counter 0, no `LINEERROR`, all seven present, `LASTVCHID=1733` |
| Company `ALTVCHID`, read just before and just after the POST | 1795 → 1805, a step of exactly 10; `ALTMSTID` unchanged; the other loaded companies unchanged |
| Read of `$AlterID > 1795 AND $AlterID <= 1805` within the day | exactly the 10 vouchers, ALTERIDs 1796–1805 **in request order** for every voucher that can be told apart, the identical pair at 1797 and 1798 (the two are identical in content and no sent REMOTEID comes back, so their own order cannot be observed); the whole-day read was byte-identical, so the day held only these 10 and this capture cannot show the span excluding anything (a single-writer lab) |
| MasterIDs | 1724–1733 in request order for every voucher that can be told apart (the identical pair as above); the last equals `LASTVCHID` |
| Content | type, date, `EFFECTIVEDATE`, signed entries and the untagged narration as sent |
| GUID | the company GUID, `-`, then the MasterID in 8 hex digits (`…-000006bc` = 1724); the `REMOTEID` attribute equals it (§9.3) |
| `$GUID = "<guid>"` added to the day's formula | one row for a present GUID (a GUID of the identical pair returned that voucher alone, not both); an empty collection for an absent one |

The span and order for Journals were already in a capture of Bridge's own post path, tagged: 50 Journals at ALTERIDs 1,420–1,469 in batch order, mark 1,419 → 1,469 (`D3_BATCH_CAPTURE_PROVENANCE.md`). One import of 50, 200 and 500 Journals each stepped the mark by exactly its count over the raw gateway, and 50 and 200 did so through Bridge's own post path with contiguous AlterIDs ([§11c.5](./TALLY_PROTOCOL_REFERENCE_MEASUREMENTS_AND_OPEN_QUESTIONS.md#11c5-live-evidence-2026-09-21), one run each).

**Untagged Journals (VERIFIED, one run, 2026-10-02, raw gateway).** One import of 5 untagged Journals in Bridge's own Journal render (no `EFFECTIVEDATE`, no party, the caller's entry order), including an identical pair and a three-leg Journal, gave `CREATED=5` and a mark step of exactly 5 (1805 → 1810). The span held exactly those 5 at ALTERIDs 1806–1810 in request order (the identical pair cannot be told apart in this read, so its order is assigned by position), MasterIDs 1734–1738 ending on `LASTVCHID`, with narration and entries as sent; Tally filled `EFFECTIVEDATE` with the date (`POST_SPAN_CAPTURE_PROVENANCE.md`, `post-span-journal-*`).

**What Bridge builds on it.** A native post no longer writes the narration tag. It is attributed by its position inside its own POST's span (`agent_import_span_identity.rs`). The dispatch intent records the target's voucher mark from the last read before the POST. Binding requires all of these:
- a clean response with `CREATED=N` and `LASTVCHID`;
- a step equal to `CREATED` when the mark after the POST was read;
- exactly N rows in the span, row k at mark + k, with the content sent as voucher k;
- MasterIDs that end on `LASTVCHID`, and a GUID that is the company GUID plus the MasterID.

The bound GUIDs are journaled and verified first. Content and `LASTVCHID` only refuse. The hand-import file keeps its tag, because Bridge never sees that import.

**A rolled-back book (PARTIAL: the check is derived; one restore was captured on 2026-10-13, next paragraph).** Every voucher change measured gives the voucher an AlterID above the company's mark (§11c.5), so after a create of N the mark should never again read below the pre-POST mark plus N unless the book was rolled back (a backup restored, or another copy put in its place). A restore was captured on 2026-10-13 (next paragraph): the mark read lower after it. Bridge reports such a book as `book_rolled_back`, never as `not_found`, and binds nothing in it; a company holding no voucher omits its mark, which reads as 0 for this check. The check sees a rollback only while the mark reads below that floor: a book keyed past it again after a restore before any check is not seen as rolled back, and since a restore lets new vouchers take the MasterIDs the post's vouchers had (next paragraph), a bound GUID would then name another voucher: `posted_divergent`, or `posted_verified` (a false verified) if the fields verification compares are the same and it is effective. Once a check has seen the mark below a floor (an omitted mark read twice counts as a company holding no voucher), Bridge records it in its import journal, a second restore too, and no native post of that company that was sent before and had proved a floor above the mark read reads verified again (bridge#1050, first slice): what Tally does is unchanged, only what Bridge remembers of it.

**After a restore, the next voucher took the removed voucher's identity, and a batch was numbered in file order (PARTIAL: one synthetic book whose voucher rows all read `NUMBERINGSTYLE` `Auto Retain`, licensed TallyPrime 7.1 Silver on Windows, 2026-10-13, one run; branch `lab/1038-import-after-restore-answers`, `f3b07ab6`, bridge#1038; read from the answers with an XML parser).**
- **VERIFIED:** the book held two Receipts. After a backup holding only the first was restored on Tally's own screen, the marks read `ALTVCHID` 1 (2 before) with `ALTMSTID` unchanged, and the voucher read returned the first Receipt alone.
- **VERIFIED:** a Receipt then sent by import without a number answered `CREATED` 1 and read number `2`, `MASTERID` 2, `ALTERID` 2 and the GUID ending `-00000002`: the four values the removed voucher had held.
- By the results file of an earlier run on this book (12 Oct 2026, branch `lab/1050-identity-answers`, `bbc36d8a`, bridge#1050; not re-derived here), that removed voucher had itself been keyed by hand after a restore, in place of one posted through ComplyEaze Bridge, and had read the posted one's number, `MASTERID`, `ALTERID` and GUID. So a hand entry and an import each took the identity.
- **VERIFIED:** two Receipts in one import file, the later-dated first, took `MASTERID`, `ALTERID` and number in the order of the file: 3 for the first, 4 for the second. The type's series was not read in this run. Under `Auto Renumber` an imported voucher was numbered by its date (§9.8), and a batch under that setting is not measured. The ten vouchers of the capture above share one date, so they could not show this.
- So no identifier a read returns tells a voucher that was imported, or keyed again, after a restore from the voucher it replaced. **INFERRED, not run:** the same vouchers sent again in another order after a restore would give a removed voucher's identity to a different one.
- **One read shape of the Company object showed no value that marked a restore (PARTIAL: the same book and day; branch `lab/1050-company-object-answers`, `b2d51e60`, bridge#1050, which carries the result table and the diffs but not the raw answers, so this bullet is from that table).** The Company object, read as a collection of type Company with every native method (32,986 bytes, 414 flattened paths), was the same bytes after five idle minutes, after a reload, after a backup, and after that backup was restored over the same content. After an older backup was restored one path differed, `AUDITEDUPTO`, whose two values were the dates of the last voucher the book then held; with the first backup restored again the answer was the first bytes again. That shape returned no `ALTVCHID` or `ALTMSTID`. Not covered: a value of the company after the book is keyed past the restore, which is the state the check above does not see; fields that shape does not return.
- **Not established:** a restore that also removes masters; a restore on another machine or into a new folder; Tally with security on; Gold; other releases; other voucher types.

**Known limit (by design, 2026-10-02).** A binding refusal is final for the batch: one voucher edited in Tally before a deferred bind refuses the binding of all of them, whose vouchers are then matched by content only and stay `reconciliation_required`. The person checks them in Tally; a review inside Bridge is bridge#1039.

**Not established:**
- another Gold client process writing during the post;
- whether a voucher keeps its GUID and MasterID through a screen edit, and its MasterID through an upsert that changes more than the narration (§9.3 measured the GUID kept through a changed amount and date; the MasterID was not read);
- a concurrent save tripping the step;
- Payment, Receipt and Contra batches other than 10;
- a `$GUID` filter without a date clause;
- any other release or licence tier;
- whether a data rewrite reuses AlterIDs or MasterIDs (a restore does: above).

### 9.16 A Sales invoice: what was measured, what Bridge posts, what is not established

A Sales invoice is an accounting invoice: the invoice view, `LEDGERENTRIES.LIST` (never `ALLLEDGERENTRIES.LIST`, §9.12), the party leg first with a New Ref when the party is bill-wise, and credit legs for the sales ledgers (one to three) and the two tax heads. Its number is the document number a GSTR-1 reports.

**Measured by reads (PARTIAL, 6 Oct 2026, TallyPrime 7.1 Silver, observed by reads on one real book; its answers are held privately and are not in this repository; no write).**
- **Hand-keyed invoices.** The invoices of one month, all of one user type filed under the predefined Sales type: `ISINVOICE` Yes, the voucher number equal to `REFERENCE`, `REFERENCEDATE` equal to the date, `STATENAME` and `PLACEOFSUPPLY` set, `GSTREGISTRATIONTYPE` `Unregistered/Consumer` with no `PARTYGSTIN` for an unregistered customer, the company's own GSTIN and state on the voucher, four or five legs (party, sales, state tax and central tax, with a round off on some of them and an IGST leg in place of the two heads on one), and no bill allocation on any leg because the customers were not bill-wise.
- **Voucher types.** A collection of the voucher types with no field list returns each type's `RESERVEDNAME` as an attribute, `PARENT`, `GUID`, a top-level `NUMBERINGMETHOD` and, inside `VOUCHERNUMBERSERIES.LIST`, the series-level method. The book held two Sales types (the predefined one, whose top level read `Automatic (Manual Override)` and whose series read `Manual`, and a user type under it whose top level read `None` and whose series read `Manual` with the series' duplicate-number guard on). The series-level method is the one that decides what an import's supplied number does (§9.8), and the existing masters read returns only the top-level one.
- **Ledgers.** The compliance read gives each ledger's reserved group chain, duty head and the GSTIN in force on a date; it does not return `ISBILLWISEON`, which the V2 ledger catalogue (§12a.15) and a `Ledger` row return as Yes or No. Tax ledgers can be named by rate (`CGST 12%`), so only the duty head says what one is; this book spelled the state head `SGST/UTGST`. The sales ledger carried no HSN and every rate at 0, so tax was keyed as ledger legs.
- **Company state (VERIFIED, 6 Oct 2026, one company on the lab's Tally).** A `Company` collection fetching `NAME`, `GUID` and `STATENAME` returns one row for each loaded company, whatever the current company is, each with its `GUID`; that company's row carried its `STATENAME`, and some rows carried none. The row is therefore chosen by `GUID`.
- **Hand imports on a Gold book (10 Sep 2026, PARTIAL).** The same shape without `STATENAME`, `PLACEOFSUPPLY`, `PARTYGSTIN` or a reference, accepted by Tally; no read-back by Bridge was made.

**Measured by one rehearsal (PARTIAL unless marked, 7 Oct 2026, TallyPrime 7.1 Silver, licensed mode, one company on the lab's Tally; a local build of this pull request's commit `5905768c5` with Sales added to the qualified list, never published).** Most of its answers are held privately, not in this repository (six were once committed as `sales-rehearsal` fixtures and removed in `4a3f2bb6`), and the GSTR-1 screen was read and transcribed and not captured; this reference marks an observation whose answers are held privately PARTIAL, as it does for the 6 October reads above, so VERIFIED is not claimed for them. None of its answers is committed as a fixture now; the invoice tests run on BRIDGE PILOT LAB's captured answers (9 Oct 2026, the `pilot-lab` fixtures). The observations below (the refused build, the two posts' counters and verdicts, the later verification, the native reads, the open bills, the declined import, and the GSTR-1 screen, which was read and transcribed and not saved) rest on answers that are not committed.
- **The voucher type.** A user type keyed under Sales with Manual numbering and duplicates prevented reads with `PARENT` Sales and one series whose method is `Manual` (its top-level method reads `None`), and resolves to the Sales class. Tally wrote two raw U+0005 characters inside that type's class name; the answer is read as it is.
- **The duplicate-number read.** For a number one invoice carries it returned that one row, Sales class Yes; for a number no voucher carries, status 1 and a collection with no row; for a number a Purchase and a Sales voucher share, both rows, the class telling them apart. A build with a number in use was refused before any file was written.
- **Two invoices posted.** One to a registered, bill-wise customer (a GSTIN beside a New Ref, with a credit round off) and one to an unregistered customer that is not bill-wise. Tally answered each import with one voucher created and every other counter zero and kept the supplied number; each read back as built, by the standard readback and then by the invoice read-back, and the first again in a later verification. Every field the invoice read-back fetches came back on both (`PARTYGSTIN` as an empty element on the unregistered one).
- **Posted beside keyed.** Read natively, each posted invoice and the hand-keyed one with the same customer and figures carried the same legs, the same GST header fields, the same company registration (`CMPGSTSTATE` equal to the company's `STATENAME`; every voucher read carried the one registration) and the same GST status flags. At the rehearsal only the keyed ones carried the buyer and consignee block (`BASICBUYERNAME`, `PARTYMAILINGNAME` and `CONSIGNEEMAILINGNAME` each equal to the customer ledger's name, `CONSIGNEESTATENAME` the customer's state, `CONSIGNEECOUNTRYNAME` India, and `VATDEALERTYPE` Regular on the unregistered customer as on the registered one); only the posted ones carried `REFERENCE` and `REFERENCEDATE` (the keyed invoices of this book carry neither). The GSTR-1 screen for the month was opened once (PARTIAL: the screen was read and transcribed, not saved), when the first posted invoice and three keyed ones were the month's vouchers: it counted four vouchers, all four as uncertain for one reason, which the screen gave as the company's GST registration details being invalid or not specified, and none under B2B or B2C; the list of vouchers under that count was not opened. The native read of the month was byte-identical before and after that look. The second invoice was posted after it, and its status flags, read natively, equal the others'.
- **A posted New Ref** stood as its own open bill, as a keyed invoice's does (read through the outstandings tool before and after; that answer is not committed).
- **An import Tally declines (PARTIAL).** An invoice dated in the next financial year, numbered like a voucher of the same type from this year and carrying a New Ref named like an open bill, was answered with nothing created and one exception, and no error text; nothing reached the book, the post was reported as not created and was not sent again. Three things differed at once, so which of them Tally declined is not established. A verification made later reported that voucher as sent and not attributed, as a later verification does by design (ADR 0018: only the post's own readback reads Tally's answer), and the batch stayed open.
- **The period on a native voucher read (PARTIAL, one observation).** A `Voucher` collection read natively with `SVFROMDATE` and `SVTODATE` set to one day did not return a voucher dated the next day, and returned it when the two were set to the month. The invoice's own reads carry a formula on literal dates and do not rest on this. It is one observation, beside the licensed-mode record in §5.3 of period days being honoured; the rule of §5.1 stands, that the period is not a filter to rely on.

**Measured by one import of the request the committed code renders (VERIFIED unless marked, 9 Oct 2026, TallyPrime 7.1 Silver, licensed mode, the synthetic company `BRIDGE PILOT LAB`, an outside contributor's machine; one invoice, one run).** The build ran on the contributor's own answers, rendered the request, and it was sent once, after reads of the book in the same sitting. The rehearsal's two invoices (above) were posted by a local build and carried no buyer and consignee block; this is the first import of the committed request with it, and the first whose answers are committed. The answers and the request are the `pilot-lab` fixtures (the provenance table beside them), and a test builds, posts and verifies the invoice on them, with Sales treated as qualified for that test alone (the two voucher-numbering reads that the build and the post make were answered by captures of 16 Oct 2026, not of 9 Oct, and the test serves them derived, with the voucher types made after the invoice round removed; the provenance table says so): the live list is unchanged, and every Sales build is still refused (below).
- **The import.** One Sales invoice in the invoice view, in a type with one Manual series that prevents duplicates, dated 2026-08-01, six legs (the party, two sales ledgers, a central and a state tax ledger, and a credit round off of 0.16): `CREATED` 1 and `ALTERED`, `DELETED`, `IGNORED`, `ERRORS`, `CANCELLED` and `EXCEPTIONS` 0, `LASTVCHID` equal to the voucher's `MASTERID`. It was accepted on the first send; no refusal came first. The company's voucher mark stepped by one (16 to 17) and its master mark did not move (223); a read made after the contributor had looked at the voucher and at GSTR-1 is byte-equal to the read made right after the import.
- **Stored as sent.** Of the 23 elements sent before the legs (the empty `NARRATION` among them), all 23 read back as sent from a native read of the whole book (the `pilot-lab` export fixture): the date, the effective date, the reference date and `REFERENCE`, the type name, the number, the party ledger's name and the party, base-party, mailing, buyer and consignee-mailing names, `STATENAME`, `CONSIGNEESTATENAME` and `PLACEOFSUPPLY` Rajasthan, `GSTREGISTRATIONTYPE` `Unregistered/Consumer`, `VATDEALERTYPE` Regular, both country elements India, `PERSISTEDVIEW`, `VCHENTRYMODE` and `ISINVOICE`. The six legs read as sent, under both `LEDGERENTRIES.LIST` and `ALLLEDGERENTRIES.LIST` in that read. **The creation `REMOTEID`:** in the whole-book read the voucher's `REMOTEID` attribute is the one the request supplied, beside a `SENDERID` equal to the voucher's own GUID; in the field-listed read-back the `REMOTEID` attribute reads the voucher's own GUID instead. So Tally keeps the supplied id, and which read shows it differs (one run). The ten vouchers keyed earlier read field for field as before. The voucher was filed in the Manual series (`NUMBERINGSTYLE` Manual, series `Default`) under the number supplied.
- **Read back by ComplyEaze Bridge's code.** The invoice read-back request returned a complete envelope with no voucher before the import and one voucher after (both answers are fixtures); the code's own comparison of that answer with the invoice as built found no difference and took its `ALTERID` and `GUID`. The verification of the whole post, run on these answers, reached `posted_verified`. This is one run on one book; it does not show a refused import, a debit round off or an unequal-head invoice.
- **GSTR-1 (read from screenshots by a person; PARTIAL).** The invoice was listed as included. The five keyed invoices that carry typed amounts stayed uncertain, and the count of vouchers went from ten to eleven. This invoice's tax is exact whichever way it is rounded (15.01 and 10.01 per line, 25.02 on the total), so it does not tell per-line from per-bill rounding.
- **Tally's own fill (VERIFIED, one keyed invoice).** For 1,001.00 at 5 percent with the tax left to Tally, it filled 25.03 on each tax head and a customer total of 1,051.06, with no round off: each head rounded half-up from 25.025.
- **The `VATDEALERTYPE` of a keyed invoice (PARTIAL).** The nine invoices keyed first read `Unknown`; the one keyed after them read `Regular`, as the imported one did. What changed between them is not known. In the voucher export, the nine that read `Unknown` carry `BASICBUYERNAME` `Bridge Walk-in`, the ledger's earlier spelling, and the two that read `Regular` carry `BRIDGE Walk-in`; the ledger's case-only rename is a candidate cause, **UNVERIFIED** (nothing was varied one at a time).
- **The walk-in ledger's spelling (PARTIAL).** The customer ledger's row carried one spelling and the party's own name another until the contributor renamed it (a change of letter case only). ComplyEaze Bridge keys on the row spelling, so after the rename the row and the name agree and the earlier spelling is refused as a master that is not exact (a test pins this). The rename moved neither company counter in this book (the ledger's `ALTERID` read 215 in an earlier listing, on the contributor's branch and not committed here, and in the committed compliance listing after the rename; the master mark read 223 throughout); how the rename was made is not known.

**What Bridge posts, once Sales is qualified.** Sales is not in the qualified list at this change: every Sales build is refused as `import_voucher_type_unqualified` before the first request, and no file is written (ADR 0004, the 6 Oct 2026 amendment). What follows is what the code builds; the rehearsal above ran it on two invoices. One Sales invoice per build and per post, in the shape above, with `REFERENCE` and `REFERENCEDATE` equal to the number and the date, as the keyed vouchers of the real book observed on 6 Oct carried them (the keyed invoices of the synthetic lab company carry neither). The build reads the ledger compliance listing, the same listing with each ledger's GST rate and rounding, the voucher types, the party's bill-wise flag from the V2 ledger catalogue (§12a.15), the number's absence among the Sales vouchers of its financial year (the vouchers carrying the number are read with a `$Date` formula on literal dates and a number equality, both of which ran in the rehearsal, because the date variables are not relied on to limit a collection (§5.1; one native read above was limited by its period, as §5.3 records for licensed mode), and the Sales class is decided in the program from the measured per-row `$$IsSales` compute, across every type), and the company's GST registration in force on the invoice date (its tax units, §12a.19), whose state is the supplier's state; a book whose voucher mark is above 25,000 is refused, because the number read (twice: the invoice's number and its control) and the read-back each walk the book's vouchers (a provisional bound: formula-filtered voucher reads have been timed in seconds on books of a few thousand vouchers, and this read only on the rehearsal's book of under a hundred vouchers); a book whose master mark is above 5,000 is refused unless the company's own ledger count is 2,000 or fewer, because the whole ledger compliance listing is read at least four times per invoice (the build, before the dialog, after it, and in the endpoint queue, #1337; a redeemed approval enters again from the top and repeats the two re-reads); it refuses unless the named type resolves to the Sales class through its parent chain with exactly one Manual series that prevents duplicates (the guard of §9.8, PARTIAL: seen once, on a Journal type, never on a Sales type; `invoice_voucher_type_duplicates_allowed` otherwise), the customer is registered (a valid GSTIN in force, on an entry that says Regular, whose state is the place of supply) or positively unregistered (an entry in force that names no GSTIN and says so), the supply is made in that registration's state, one or two Sales Accounts ledgers (a third waits for a measured three-line bill; the taxable value is their sum, each ledger at the rate its own dated GST row gives, read in a listing of its own, the compliance listing with four fields added (10 Oct 2026), in which a ledger row with no name fails the read (`invoice_ledger_rates_row_unnamed`): one row in force, made of the elements the lab's rows carried, ledger-specified and taxable, one all-states block, CGST and state tax equal and IGST twice one of them, no cess, the same rate on every sales ledger; the approval window holds six legs in all when no ledger is approved On Account and the names are not long, and a longer invoice is refused as too large, never cut) and one central tax and one state-tax ledger (by duty head, each carrying that rate's half as its own rate and rounding nothing; one the rate listing does not return is refused as its rate not known, `invoice_tax_ledger_rate_unknown`, and one returned with another rate as `invoice_tax_ledger_rate_mismatch`) are named. Each head must equal the tax Tally's GSTR-1 expects, the per-line figure §9.17 measured: the sum over the sales lines of half-up(line x rate / 2) to the paisa, the rate being the sales ledger's own (§9.17: on sixteen vouchers of one book, Tally's flag read included exactly when both heads equalled that figure, and every voucher it flagged was one paisa off it on at least one head), so two heads that differ are never admitted; any other figure is refused (`invoice_tax_head_not_expected`) with the one expected. The round off is not part of that figure (§9.17: it did not decide the flag). Only 2.5 percent was measured with a line whose tax is not a whole paisa, so at any other rate such a line is refused. Any round off is under one rupee. The round-off ledger's own GST row must be the one empty row the lab's round-off ledger carried (PARTIAL: one ledger, under Indirect Expenses; whether a group-level GST setting reaches a ledger whose own row is empty was not measured); a ledger with no row, several rows or a row that says anything, an attribute included, is refused. Its own rate of tax and rounding limit must read 0 and it must carry no rounding method element, as the lab's round-off ledger did (`invoice_round_off_ledger_tax_fields` otherwise; an empty field or an absent one is not a 0). IGST, cess, items, credit and debit notes, new ledgers, a reference and amendments are refused.

**Read back.** After a post, the standard window readback (date, type, number, entries, narration) is followed by a read of the invoice by type and number over its financial year, and `posted_verified` is kept only when the party, the GST header fields, the reference, every leg and the bill allocation read back as built, and Tally's own GSTR-1 status lists the invoice as included and not accepted as it stands. A post-dated mark is not read: the read-back does not fetch `ISPOSTDATED` (the request the lab answered on 10 Oct 2026 does not carry it), so an invoice that a person marks post-dated in Tally after the post still reads included and stays `posted_verified`; fetching it changes that request and needs one more lab answer, and the follow-up then decides the state with the status read's `status_of_voucher`.

**The GSTR-1 status in the read-back (10 Oct 2026, TallyPrime 7.1 Silver, the lab's synthetic company, an unregistered buyer).** The read-back request carries the four status fields of §9.20 (`VCHGSTSTATUSISINCLUDED`, `VCHGSTSTATUSISUNCERTAIN`, `VCHGSTSTATUSISEXCLUDED`, `ISGSTOVERRIDDEN`) after its other fields, and the flags are read by the GST status read's own two readings. Every other state is `posted_divergent` under its own code, and several can stand together: `gst_status_uncertain`, `gst_status_excluded`, `gst_status_not_in_return` (all three flags No), `gst_status_overridden` (accepted as it stands), `gst_status_unreadable` (a combination or a value that states nothing) and `gst_status_not_reported` (the fields did not come back), so the company's later invoices stop until a person releases it.
- **VERIFIED: the request, byte for byte as the code renders it, returned the four fields on one voucher read by type and number.** `BP/26-27/0010` read included and not accepted (the `pilot-lab-invoice-readback-status` fixture, AlterID 17, unchanged since its import), and the same request for `BP/26-27/0021` read uncertain. `0010` was read after later vouchers were keyed, not straight after its import, so no capture shows the status read at the moment the code reads it, straight after a post. If Tally has not yet set the flags then, every post ends in a stop, never a false pass (`gst_status_not_reported` if the flags do not come back, `gst_status_not_in_return` if all three read No, `gst_status_unreadable` for any other combination); a read straight after a post is owed before Sales is qualified.
- **VERIFIED, one voucher: correcting a master did not change the status of an invoice already saved.** `BP/26-27/0021` read uncertain after the HSN/SAC of its sales ledger was set, the voucher untouched (AlterID 34); after the voucher was opened and saved with no change, it read included (AlterID 35). Saving it again changed its AlterID.
- **PARTIAL: uncertain and accepted as it stands were measured with the identical field list on other filters** (the same request for three numbers at once, the `three-vouchers` fixture of §9.20: `BP/26-27/0002` uncertain, `0016` included and accepted as it stands) and on 0021 above; no invoice the build posted has been read uncertain or accepted as it stands.
- **NOT established:** a voucher flagged excluded (never observed); a registered buyer; that no field carries the reason for a status (a negative measured only on the field lists tried; Tally shows the reason in its GSTR-1 view); what a verification reads after a person saves a posted invoice again, which changes its AlterID.

**The HSN/SAC of each sales line in the read-back (the same three captures, 10 Oct 2026).** Bridge writes no HSN/SAC on the voucher; Tally fills it from the masters when the voucher is saved. The unchanged read-back request returns, inside every `ALLLEDGERENTRIES.LIST`, `HSNSOURCETYPE`, `HSNLEDGERSOURCE`, `HSNGROUPSOURCE`, `GSTHSNNAME` and `GSTHSNINFERAPPLICABILITY`. Each element is read as absent, empty or a value. Only the legs the build classified as sales lines are judged (the build records their ledgers in the saved batch; a batch saved before that is `invoice_not_observed`): a sales line whose `GSTHSNNAME` or `HSNSOURCETYPE` did not come back or came back empty makes the invoice `posted_divergent` under `invoice_line_hsn_unresolved`. The code's digits and value are not judged. The response shows, for every sales line, the code and the source Tally used (`invoice_lines`).
- **VERIFIED: the five elements come back on every leg.** On the three `pilot-lab-invoice-readback-status` captures the party, tax and round-off legs carry them empty (`<GSTHSNNAME/>`). On `BP/26-27/0010` (two sales lines) and on `BP/26-27/0021` once saved again (three), each sales line reads `HSNSOURCETYPE` Ledger, `HSNLEDGERSOURCE` the line's own ledger, `HSNGROUPSOURCE` empty, `GSTHSNINFERAPPLICABILITY` As per Masters/Company, and a `GSTHSNNAME`. The three sibling ledgers `BRIDGE Svc 998313 5%`, `BRIDGE Svc 998314 5%` and `BRIDGE Svc 998315 5%` carry three different codes: 998313, 998314 and 998315.
- **VERIFIED, one voucher: the elements are what the saved voucher holds.** On `BP/26-27/0021` read while uncertain (AlterID 34, after the HSN/SAC of `BRIDGE Svc 998315 5%` was set and the voucher was not touched), that line carries `HSNSOURCETYPE`, `HSNLEDGERSOURCE`, `HSNGROUPSOURCE` and `GSTHSNNAME` empty, and `GSTHSNINFERAPPLICABILITY` As per Masters/Company; it is the only voucher of the three with an empty sales line. After the voucher was saved again with no change (AlterID 35) the line reads Ledger, its ledger and 998315.
- **PARTIAL: the rehearsal's sales lines.** The three `sales-rehearsal` read-backs (7 Oct 2026, another synthetic lab company; their fixtures were removed in `4a3f2bb6` and are no longer in the tree, so this rests on them as read at `4a3f2bb6^`) carry the four source and code elements empty on their one sales line `Sales - Goods`, applicability As per Masters/Company. Their request asked for no GSTR-1 status, so no status was read beside them; under this check those invoices would read `invoice_line_hsn_unresolved` (inferred: no test runs the check on them now).
- **NOT established on this request:** a code Tally takes from a group or from the company, a ledger row dated after the voucher, and a four-digit code were measured with other read requests (below), not with these captures.

**The HSN/SAC check before a post (lab request sets W11 and W14, 12 Oct 2026, one synthetic book, TallyPrime 7.1 Silver, evidence on the `lab/1342-w11-answers` and `lab/1342-w14-answers` branches).** Tally decides a sales line's HSN/SAC when the voucher is saved, so the build resolves it the same way before the post and refuses an invoice whose sales ledger resolves to none (`invoice_sales_ledger_hsn_unresolved`).
- **VERIFIED, one book, one voucher each: the order.** The sales ledger's own row in force on the voucher date, then the nearest group with a code (a sub-group before its parent), then the company. The read-back names the level: `HSNSOURCETYPE` Ledger, Group or Company, with `HSNLEDGERSOURCE` or `HSNGROUPSOURCE` and `GSTHSNNAME`. A ledger row dated after the voucher was not used for it (a row from 1 Sep on a 7 Aug voucher: the earlier row's code was used). A four-digit code was accepted and the voucher included; the company's GST details read a minimum length of 4, based on annual turnover.
- **VERIFIED, one book: the code is fixed when the voucher is saved.** Two vouchers saved before any code existed stayed uncertain after a company and a group code were set and Tally's master-change re-check ran for the current period (no field of either voucher changed); only saving a voucher again repairs it (above, `BP/26-27/0021`).
- **VERIFIED, one book: where each level is read.** A ledger's rows and a group's come back as `HSNDETAILS.LIST` (`APPLICABLEFROM`, `HSNCODE`, `HSN`, `HSNCLASSIFICATIONNAME`, `SRCOFHSNDETAILS`): a row with its own code reads `Specify Details Here`, a row that defers reads `As per Company/Group` with an empty code, and a group never given a code carried one empty `HSNDETAILS.LIST`. The company's code is not in the company collection; it is in the company object read with every field (about 42 KB), as `HSNDETAILS.LIST`, beside `CMPGSTDETAILS.LIST` (`HSNSOURCEPRIORITY` Ledger) and `ISGSTCLASSIFON` No.
- **What the build follows.** Only those shapes, and only on a company whose settings below read as the lab's did. The check runs after the rest of the admission has passed, and reads the company object each time: unless its `ISGSTCLASSIFON` reads No and its `CMPGSTDETAILS.LIST` row in force reads `HSNSOURCEPRIORITY` Ledger, `HSNAPPLICABLESECTION` All Sections and a numeric `HSNCODELENGTH`, every sales ledger is refused as `invoice_sales_ledger_hsn_source_not_measured` (what the other values do is not measured, so none is described here). So is a row in force that says something else, a deferring row on a master that has other rows, several rows on one date, no row in force, a row with an element not listed above or a classification name, a sales ledger carrying only the empty element (seen on ledgers with no code; whether Tally then takes a group or company code for a sales line was not measured), a code that would have to come from further up than two groups, a level that came back without `HSNDETAILS.LIST`, and a code shorter than `HSNCODELENGTH` (whether Tally flags one is not measured). The build reads the groups only when a sales ledger's own row defers; the queue always reads them, and the judge parses the answer whether or not it is needed, so a groups answer that cannot be read fails the post in the queue only. A code that resolves is never refused for its digits: it is warned about (`hsn_warnings`: a character that is not a digit; not six digits; by the ledger's type of supply, a code starting 99 on a goods ledger or not starting 99 on a services ledger; the type of supply not judged when it is neither Goods nor Services). The warnings are recorded with the build, shown to the person approving (a count in the dialog, the list in the responses), and compared on both re-reads, so a change in them between the build and the post refuses the post as `import_invoice_masters_changed` (cause `hsn_warnings`). The rules are ComplyEaze Bridge's own, not Tally's behaviour: whether Tally flags a code that is too short or not digits was not measured. Bridge never chooses or suggests a code.
- **VERIFIED, one book: the requests (lab request set W16, 12 Oct 2026, evidence on `lab/1342-w16-answers`).** The ledger rates listing with `HSNDETAILS.LIST, GSTTYPEOFSUPPLY, GSTAPPLICABLE` added returned the same 18 ledgers as before, each with one or more `HSNDETAILS.LIST`: a row with its own code carries `APPLICABLEFROM`, `HSNCODE`, `HSN` and `SRCOFHSNDETAILS`; a deferring row carries only `APPLICABLEFROM` and `SRCOFHSNDETAILS`; a ledger with no code at all (the tax, party and cash ledgers) carries one empty element. The group listing with `HSNDETAILS.LIST` added returned the same 29 groups, the two with a code as dated rows and every other one with one empty element. The company object read is the one W14 answered, with the same rows. Read by the code's own parsers, the three answers resolve each sales ledger to the code and level Tally's own GST rate setup screens showed in that run (a test pins it). Tally writes raw control characters inside `PARENTSTRUCTURE` in the group answers; the parser reads past them.
- **VERIFIED, one book: the four-parent rates request (21 Oct 2026, read-only, same lab company, marks 68 and 245).** From #1331's first change until the ledger read below, the code restricted the rates read to the ledgers under the parents of the ledgers the invoice names, so the request it sent was the four-parent request (10 Oct 2026) with the three HSN/SAC fields added to its field list (SHA-256 `3f5151473ba2c7a69a19fd7c0373fbed13b7dc890744a66976dfc961a3d2bcdc`). The lab had answered the two halves separately (the four-parent request without the fields on 10 Oct, and W16, the unrestricted request with them, on 12 Oct); on 21 Oct it answered this request itself, twice, byte-identically (evidence on `lab/1640-scoped-hsn-answers`, commit `fe62bf959934857dd270a0cf6a4ecb5c88700718`, directory `lab/1640-scoped-hsn/`): 13 ledgers, exactly the catalogue's ledgers under the four parents (the catalogue read in the same minute), each with its `HSNDETAILS.LIST`, and row for row, element for element, equal to W16's answer cut to those parents (the `pilot-lab-ledger-rates-scoped-hsn` fixture, whole as received; the bytes are also exactly those of that cut, so what separates the capture from a cut is the published pair and the lab's record of the send). The replays of the build and the post now serve this answer cut to their invoice's six ledgers (below). It is one book on one day: the filter on a book with other parents or a case-twin is as in §11e. The replay of the build on the book of the W18 sittings (the one with the four-digit ledger) serves the same answer cut to that invoice's six ledgers (DERIVED: that book's catalogue is byte-equal to the one read with this answer on 21 and 31 Oct), and its two voucher-numbering answers are the 16 Oct 2026 answers with the rows above that book's master mark (243) removed (DERIVED); captures of the numbering reads on the W18 book need the book restored from its backup and are owed (`CAPTURE-1640`, its W18 addendum).
- **VERIFIED, one book: the rates request the code sends (31 Oct 2026, read-only, same lab company, marks 68 and 245, TallyPrime 7.2 Silver).** The code now names the invoice's ledgers themselves, by the GUID the catalogue gave each: the four-parent request with its formula replaced by `$GUID = "…" OR …` (formula `BridgeInvoiceLedgers`), nothing else changed. Asked for the five ledgers of the Goods 18% invoice, the lab answered exactly those five rows, byte for byte the four-parent answer of the same sitting (itself byte-equal to the 21 Oct answer above) with every other row removed, with the whitespace after it, in the same order. A filter naming four of them and a GUID the book does not hold answered the four, cut the same way; one naming only that GUID answered a complete envelope with an empty `COLLECTION` and no `LINEERROR` (a `$Name` filter for an absent name answered the same bytes). The book was then grown to 10,018 and 20,018 ledgers (plain Sundry Debtors ledgers, restored afterwards): the five-ledger answer kept the same rows, in 0.11 to 0.31 s against 0.06 s at 18 ledgers, while the whole ledger catalogue the import reads at each check grew to 12.4 MB (1.2 s) and 24.7 MB (2.0 s). The same filter on the compliance listing's master and balance requests answered exactly the five rows: at 18 ledgers each equal to its row of the whole read (the balance rows, which carry no GUID, matched by name), and at 10,018 and 20,018 ledgers equal to the 18-ledger answers, the whole reads not being sent at those sizes (not yet used by the code). Evidence: sheet `lab/1331-scoped-masters` (commit `09ce612e`), answers `lab/1331-scoped-masters-answers` (commit `1deab810`); the `pilot-lab-ledger-rates-guid` fixtures. The replays serve the four-parent answers cut to their invoice's ledgers the same way (DERIVED; a test holds the cut equal to the captured GUID answers).
- **NOT established:** goods ledgers and stock items (a Sales invoice with items is refused); the GST classification source; any other value of the company settings above, and the company fields not read (among them `GSTR1HSNBASIS`, `BASISOFCALC`, `GSTSOURCEPRIORITY`, `THRESHOLDLIMIT` and the HSN summary switches); a ledger or group with several rows whose row in force defers; a deeper chain; whether Tally takes a group or company code for a sales ledger that carries only the empty element; a company with no code at all in the object read (the placeholder a group with no code carried is expected and read as no code, so `invoice_sales_ledger_hsn_unresolved` rests on that expectation; any other answer is refused as not measured); whether Tally flags a code that is too short or not digits.

**Not established.**
- The duplicate-number read outside the release and licence mode of the rehearsal: there it found a known invoice and answered an absent number with an empty collection (an answer with no `COLLECTION` element is refused, never read as no rows). After a post the invoice is read back by type and number, and a second voucher of that type with that number is reported as `invoice_number_not_unique`; a twin in another Sales type is caught only by the build's class-wide read, not after the post.
- What the number read and the read-back cost on a large book: each walks the book's vouchers (the number read at build, at both re-reads (a redeemed approval repeats them) and in the endpoint queue, the read-back after the post and at every later verification; each read is sent as a pair), and neither has been timed on a large book. The 25,000 voucher-mark bound is provisional until a lab run times them on a larger book.
- A customer whose ledger holds its GSTIN only in the flat field, with no dated registration entry (some debtors of one company on the lab's Tally read this way): refused, because no entry says what kind of registration it is. Whether saving the ledger's GST details in Tally adds the entry is not measured.
- An unregistered customer's own state: the ledger's state is not read, so the company's state is written as the voucher's `STATENAME` and `CONSIGNEESTATENAME` on the caller's place of supply alone (a walk-in customer from another state is printed with the company's state).
- Among the elements written that no read-back compares (the list is not exhaustive): `PARTYNAME`, `BASICBASEPARTYNAME`, `COUNTRYOFRESIDENCE`, `PERSISTEDVIEW`, `VCHENTRYMODE`, the six of the buyer and consignee block, and the `OBJVIEW` attribute.
- A bill of the same name: the party leg of a bill-wise customer is written as a New Ref named by the invoice number, and the customer's existing bills are not read. Whether Tally merges it with an open bill of that name on the same ledger (last year's invoice of the same number, or an opening bill) is not measured: the one such invoice the rehearsal sent was declined by Tally (above) and never reached the book. A read of the customer's bills before the build is the likely guard.
- A company with more than one GST registration: no company registration is written on the voucher and none is read back, so which registration Tally files the invoice under is not measured.
- The tax check's reach (10 Oct 2026): it rests on one book, TallyPrime 7.1 Silver, CGST and state tax inside the company's own state, 5% services lines (one 18% goods line only on an exact bill), the invoice view, an unregistered buyer, one or two lines (§9.17). A tax ledger whose own rate field reads 0 or is absent is refused, because the measured ledgers carried the rate there; a book whose tax ledgers carry it only in their GST rows is therefore refused, which is not measured as wrong. Only the two rates measured on this path are admitted on a sales ledger, 5 and 18 percent (§9.17: 5 percent on every voucher of its table, 18 percent on one line of one exact bill, 0009); a ledger at another GST slab (12, 28 or 40 percent) is refused as not measured (`invoice_sales_ledger_rate_not_measured`), and one at a rate that is no slab (3 percent, say) as such (`invoice_sales_ledger_rate_not_a_slab`), though the invoice and the ledger agree. A sales ledger with more than one dated GST row is refused (only one row was read); so is one whose rate comes from its group or the company, one valued on anything but value, one with state-wise rows, and any cess. A row with an element the lab's rows did not carry (a slab-rate list that is not empty, a reverse-charge or per-unit field) is refused. How the build reads a rate row that changes during the year is not measured. The new listing is every ledger of the book with the GST rows and is read in each admission (three times for one invoice when this was written; the queue's read, #1337, makes it at least four, and since #1331 the read names only the invoice's ledgers, by GUID); its cost at 20,018 ledgers is above (0.3 s), and the cost of the whole compliance listing beside it on a large book is not measured (the lab's answer was about 3.7 KB a ledger on the wire, so about 7 MB a read at the 2,000 ledgers the existing size gate admits, and each read is sent as a pair, so about twice that on the wire). The HSN/SAC is checked before the post (above). It is joined to the compliance listing by ledger name; a ledger deleted and made again under the same name between the two reads would take the new ledger's rates, and the post-time identity checks are what catch it.
- What the company's `STATENAME` is in a company with several registrations: its address state, which can differ there from the state of the GST registration a voucher is filed under (the voucher's `CMPGSTSTATE`). It decides CGST plus state tax against IGST. The two were equal on the rehearsal's company, where every voucher read carried the same registration.
- Two of the element sets: a registered customer that is not bill-wise, and a round off on the debit side. The rehearsal posted the other two (a registered customer with a New Ref and a credit round off; an unregistered customer).
- The buyer and consignee block: written since 8 Oct 2026 with the values the keyed invoices read (above), and imported once on 9 Oct 2026, when Tally stored all of it as sent (above). The three names are the customer ledger's name, because its mailing name is not read, so a ledger with a mailing name of its own gets its ledger name there, on the printed invoice too, and nothing shows it (on the rehearsal's book the two were the same, so which one Tally writes is not established); `VATDEALERTYPE` Regular for an unregistered customer rests on one keyed invoice and the one import (nine other keyed invoices of that book read `Unknown`). None of the six is fetched or compared by the read-back, so a value Tally drops or rewrites still reads as posted. Owed before Sales is qualified: on a company whose own registration is valid (on the rehearsal's company every voucher, keyed or posted, read as uncertain for that alone), one variable, whether an accounting invoice with the block and one without it read differently in Tally's GST status and its GSTR-1 view, for a registered and an unregistered customer, with the six read natively after the import; the party's mailing name read in a read of its own (the measured compliance listing unchanged), or the reference saying why the ledger name stands; and the six added to the read-back, or the gap accepted here. What it changes on a printed invoice or an e-way bill is not measured.
- Several sales legs on one pair of tax heads: two lines are admitted, and keyed vouchers on two ledgers (0007, 0008, 0012 to 0014) and one voucher imported with the same ledger on two lines (the lab's second list, its step W6: stored as two lines, included in GSTR-1) were measured on one book (§9.17). A two-ledger bill rendered by the build was imported once (BP/26-27/0010, with a round off) and read back as built; three lines were not measured and are refused. Still owed before Sales is qualified, one variable: how Tally's GSTR-1 view splits the taxable value by heading code when two sales ledgers share one pair of duty ledgers.
- What the rate read asks for (#1331): the ledgers the invoice names, by `$GUID = "g1" OR $GUID = "g2"`, each GUID taken from the catalogue read at that check, where each name resolves exactly on the catalogue's spelling. A GUID is of a closed alphabet (hexadecimal digits and hyphens), so the filter holds nothing the invoice supplied. The answer is believed only when its rows are exactly those ledgers, one each, with the catalogue's name and parent. The compliance listing is not narrowed yet: it is still read whole at each check, and its 5,000 master mark and 2,000 ledger limits stand until it reads the same ledgers. The queue repeats both under its lock (#1337), sending the rates request planned at the last re-read; because a GUID request does not change when a ledger is regrouped, the plan also carries each named ledger's parent, and a named ledger replaced under its name or regrouped since then refuses the post as `import_invoice_masters_changed` with the cause `ledger_scope`.
- A blended bill (sales legs at two rates on one pair of tax ledgers) is refused (`invoice_sales_ledgers_rates_differ`), since each sales ledger's own rate is read. A bill at two rates with a pair of tax ledgers per rate is not admitted either: the pair cannot be tied to its sales legs without a rate the caller declares, and how Tally attributes a rate to a ledger-leg bill is not measured (the keyed voucher at two rates read as included on exact figures only).
- Unequal tax heads: refused. The lab's GSTR-1 flagged every voucher whose two heads differed by a paisa as uncertain (§9.17), so an odd total tax is not split; each head carries the per-line tax and the two are equal. The earlier reading, that Tally might take unequal heads, is replaced by that measurement.
- Two invoices that differ only by their number: the rehearsal's build refused the second, because before a post the book is read and a voucher with the same date, type and entries as one already there was refused whatever its number. The check now recognises an invoice by its number (ADR 0004, the paragraph of 7 Oct 2026): a voucher of its type with the same number is the same invoice whatever its figures, one with no number is the same invoice when its figures are, and one with another number is another invoice. None of that has run against Tally. Owed before Sales is qualified, one variable each: the same figures under another number post; the same number is refused; a replay is refused; how Tally compares numbers that differ by case, by spaces or by leading zeros; how a blank number reads back.
- A number used in an earlier financial year: the number read is by financial year, so such a number is admitted, and a voucher type whose series prevents duplicates may then decline the import (the PARTIAL observation above). Owed before Sales is qualified: establish which, and refuse at the build what Tally would decline.
- A control beside the duplicate-number read: "not in use" is status 1, a collection and no Sales-class row, and a formula that Tally evaluates to false for every row, or rows nested one level deeper, reads the same. Built since 8 Oct 2026, never run against Tally: the answer stands only beside a control. The same read, for the newest invoice of the company that ComplyEaze Bridge's journal holds as verified posted (one of the invoice's financial year first), must return it, with its date and as Sales class, or the build is refused (`invoice_number_control_missing`). With no such invoice, the build is admitted only when the book's voucher mark is 0 or when no invoice of the company stands as sent (none was sent, or a person released each one sent: ADR 0004, slice 4), and the post then stands on the type's duplicate guard and its read-back; otherwise it is refused until the earlier post is verified or released (`invoice_number_control_unavailable`). A released invoice is never the control, whatever the read at its release saw, so after a release a number read that matches nothing on this Tally is not detected: each invoice is then posted, fails its read-back and stops the company again. Not caught: a break that depends on the number's own text, a wrong class for another Sales type, a twin in another Sales type on the first post, and a journal that was reset. Owed before Sales is qualified, one variable each: that a Sales type with the guard on declines a reused number; the first post on a company with no invoice sent, and a post after a release; a second build controlled by it; the controlling invoice deleted by hand (refused as missing); a company with no voucher (mark 0); the cost of the extra read on a larger book. Measured earlier, towards a control from the year's rows: the same request without its number term returned every voucher of the year with its class (about 2.6 KB a voucher on the wire), and with the Sales class as a term of the formula, read after the two posts, 20 vouchers, every one Sales-class, where the first read had counted 18 before them, so the number can be compared outside Tally at that cost; a year with no Sales voucher still returns no row to stand as a control.
- The customer's party type: it is not read, so a customer in a special economic zone in the company's own state, or a deemed export, would be taken as an ordinary intra-state supply. Owed before Sales is qualified: read it and refuse anything but the ordinary type.
- The company's own registration on the invoice date: read since the build reads the company's tax units (§12a.19), and an invoice is admitted only when exactly one Regular, active GST registration is in force on its date, whose valid GSTIN belongs to the registration's state; a composition taxpayer or a company with no such registration is refused. Still owed before Sales is qualified: the company's GST flag is not read (a company with GST off that still holds a Regular unit in force is admitted), several registrations, how Tally writes an inactive or composition registration and a changed GSTIN (the refusals rest on the field names, and the tests edit the captured bytes), and a voucher type bound to one tax unit.
- E-invoicing (an invoice reference number for a registered customer) is outside what is posted.
- Where Tally's GSTR-1 places a posted invoice (B2B for a registered customer, B2C for an unregistered one): Tally's report counted the four vouchers the month then held as uncertain, giving the company's GST registration details as invalid or not specified, so it showed only that the first posted invoice was counted as the keyed ones were (the list under the count was not opened, and the second invoice was posted after the look). HSN is not carried. On a company whose registration is valid, one imported invoice was listed as included (above); a registered customer was not tried there.
- The invoice's masters inside the endpoint queue (#1337): built since 10 Oct 2026, never run against Tally. After the approval, inside the identity brackets of the POST and before any dispatch intent, the queue first judges the company's currency and the book's size limits on the marks and currencies it has read (the ledger count is read only for a book over the master mark), then sends the reads of the invoice's admission again (the compliance listing, the rates, the voucher types, the number and its control, the company's registration) and the admission's own judge decides on the answers; an observation that differs from the build's refuses the post as `import_invoice_masters_changed`, with the first field that differs as its cause (`ledger_scope` for the rates request, which the catalogue's parents decide). What stays outside it: a master changed after the queue's last invoice read is seen only through the company's master mark (`post_masters_moved`), and a number taken after the queue's number read only by the voucher type's own duplicate guard. Not measured: the time the extra reads add on a real book, and that every master change moves the master mark.
- Any release or licence tier other than the one measured.

### 9.20 The GSTR-1 status of one voucher type in a window, read by the type's name with a narrow field list

**PARTIAL: one synthetic company, licensed TallyPrime 7.1 Silver, one run per request, the unregistered buyer of
its Sales invoices.** This is the read that §9.17's flags can be taken through without fetching whole vouchers.
The answers are committed byte for byte with their hashes (the `gst-status` fixtures and their provenance file).

- **VERIFIED: a request that selects the vouchers of one type by the type's name, in a window of two days,
  with only the date, number, type name, `MASTERID`, `ALTERID`, the three status flags
  (`VCHGSTSTATUSISINCLUDED`, `VCHGSTSTATUSISUNCERTAIN`, `VCHGSTSTATUSISEXCLUDED`) and `ISGSTOVERRIDDEN`,
  returns those fields without the ledger entries.** An answer cost about 3.0 KB per voucher and 3.2 KB once;
  the full voucher export of the same book is 78 to 94 KB per voucher.
- **VERIFIED: selecting the type by its name and by its GUID gave byte-identical answers, and neither
  selected another type.** The by-GUID answer is `aw8-status-narrow-window-sales.xml` on the lab's answers branch
  (commit `313f5439b`), with the same SHA-256 as the by-name answer committed as a fixture, so it is not committed
  twice. A second Sales type (`Z1`) in the same window was not selected by the first type's name.
- **VERIFIED: the three flags read No, No, No on the four Receipts in the window.** For Receipts that is
  Tally not listing the voucher in the return. It is **not measured** for any other type, and the two keyed Purchase
  invoices of §9.19 read uncertain, so "all three No" is not a general meaning.
- **VERIFIED: the flags were read as No, Yes, No on an uncertain voucher (the tax figure of §9.17 that differed),
  and an Accept As Is voucher read included with `ISGSTOVERRIDDEN` Yes.** The uncertain voucher is dated 1-Aug-2026,
  outside the narrow two-day window, so it was shown by the wider voucher read-back (the `three-vouchers` fixture),
  not by the narrow request. The Accept As Is voucher was read by both.
- **VERIFIED: the narrow request answers uncertain vouchers, and shows an accept between two reads** (lab set W13,
  12 Oct 2026, the same company, evidence on `lab/1342-w13-answers`, answers `aw13-status-window-0` and `-1`, the
  request for 1 to 31 August 2026 and type `BRIDGE Sales`, byte for byte what the code rendered that day). Of the 28 vouchers
  each read returned, six read No, Yes, No in the first. `BP/26-27/0005` was then accepted as it stands on Tally's
  screen, alone, and in the second read it was the only voucher that differed: included, not uncertain,
  `ISGSTOVERRIDDEN` Yes, with a higher `ALTERID`.
- **VERIFIED: a cancelled voucher reads the included flag Yes and an optional one reads all three No, and the
  request answers whether a voucher is either once `ISCANCELLED` and `ISOPTIONAL` are added** (lab sets W19 and
  W20, 12 Oct 2026, the same company; evidence on `lab/1342-w19-answers`, answer `aw19-c`, and on
  `lab/1342-w20-answers`, answer `aw20-b`). `BP/26-27/0041`, saved and then
  cancelled, reads `ISCANCELLED` Yes and the three flags Yes, No, No. `BP/26-27/0042`, saved as optional, reads
  `ISOPTIONAL` Yes and No, No, No. With `,ISCANCELLED,ISOPTIONAL` appended to the field list of the request the
  code rendered on 12 Oct 2026 for 1 to 31 August 2026 and `BRIDGE Sales`, each of the 32 vouchers carried both exactly once, and
  every other voucher read No and No. Every voucher's three flags, `ISGSTOVERRIDDEN` and `ALTERID` equalled those
  of the same read without the two names, and the company marks were byte-identical before and after. So a reader
  that takes the three flags alone reports a cancelled voucher as included and an optional one as not listed:
  decide the two fields first. One voucher each.
- **VERIFIED: the request carries `ISPOSTDATED` once per voucher once the name is added, and Tally returns
  `ISDELETED` and `ISDELETEDVCHRETAINED` once per voucher without being asked** (lab sets W21 and W21b, 12 Oct
  2026, the same company; evidence on `lab/1342-w21-answers`, answer `aw21-b`, and on
  `lab/1342-w21b-answers`, answer `aw21b-b`). The request is the one above with `,ISPOSTDATED` appended (request
  file SHA-256 `68aaa000`). Read by structure and by a second independent parse, each of the 33 vouchers of
  `aw21b-b` carried each of `ISCANCELLED`, `ISOPTIONAL`, `ISPOSTDATED`, `ISDELETED` and `ISDELETEDVCHRETAINED`
  exactly once, as a direct child, none nested, and the two deleted flags have no `TYPE` attribute. The deleted
  flags read No on all 33. The 32 vouchers of the earlier read (`aw21-b`) came back unchanged beside the new one.
- **VERIFIED, one voucher: a voucher marked post-dated by hand reads `ISPOSTDATED` Yes and the included flag Yes
  when Tally's current date is the voucher's date** (W21b; `lab/1342-w21b-answers`, answer `aw21b-b`). `BP/26-27/0045` was keyed as a `BRIDGE Sales` invoice
  dated 25-Aug-2026 and marked post-dated on the voucher screen (Ctrl+T). The read was made with Tally's current
  date (the date set with F2 at the Gateway, which is not the calendar's; the calendar date was 12 Oct 2026) set
  to 25-Aug-2026, from the lab's send log. So a reader that takes the three flags alone reports it as included:
  decide `ISPOSTDATED` first, as for the two fields above.
- **VERIFIED, one voucher: the same read, made with Tally's current date before the voucher's date and again
  after it, returned the same bytes, and the GSTR-1 screen counted the voucher at both dates** (W21c, 13 Oct
  2026, the same company; evidence on `lab/1342-w21c-answers`, answers `aw21c-b` and `aw21c-c`, SHA-256
  `5cc5d249…` for both). `BP/26-27/0045` was keyed as a `BRIDGE Sales` invoice dated 25-Aug-2026 and marked
  post-dated. With the current date set at the Gateway to 10-Aug-2026 and then to 26-Aug-2026 the two answers
  (111,748 bytes) are byte-identical: `ISPOSTDATED` Yes once, the included flag Yes, uncertain No, excluded No,
  `ISGSTOVERRIDDEN` No. The read before the voucher was keyed (current date 10-Aug-2026) is byte-identical to
  the earlier capture `aw21-b`, so the current date alone did not change the read. The GSTR-1 screen for August
  showed 39 vouchers, 27 included, 7 not relevant and 5 uncertain before the voucher and 40, 28, 7 and 5 at both
  dates after it (1,000.00 more taxable value), as the lab read them off the screen (the screenshots stay on the
  lab PC); the Day Book of 25-Aug-2026 listed the voucher at both dates.
- **VERIFIED, one book: adding `PARTYLEDGERNAME` and `AMOUNT` to the field list returns the party and the invoice
  total once per voucher, as direct children, and changes no flag** (lab set W21d, 13 Oct 2026, the same company;
  evidence on `lab/1342-w21d-answers` (`d1070dea`), answer `aw21d-b`, request SHA-256 `cb037151…`). Read by structure,
  each of the 32 vouchers carries one `PARTYLEDGERNAME` and one `AMOUNT` directly under the voucher. `AMOUNT` is the
  invoice total and is **signed negative** on 31 of them (`BP/26-27/0001` -1050.00, `0003` -1051.04, `0016` -210.42,
  the totals the lab read off the voucher screen, party `BRIDGE Walk-in`); the 32nd, the cancelled `BP/26-27/0041`, reads an
  empty party and an empty amount, while the optional `BP/26-27/0042` reads its party and -1050.00. Asking for the
  amount makes Tally return each voucher's ledger entries as well, and `AMOUNT` then also appears under them: only
  the voucher's own direct child is the invoice total. The answer is 242,632 bytes against 108,442 for the narrow
  request, about 7.6 KB a voucher; the three status flags, `ISGSTOVERRIDDEN` and `ALTERID` of every voucher equal
  those of the narrow read.
- **VERIFIED, one voucher: one Accept As Is, made on Tally's own GSTR-1 screen on that voucher alone, changed that
  voucher and no other** (W21d, answers `aw21d-b` before and `aw21d-c` after, 242,636 bytes). `BP/26-27/0003` went
  from uncertain to included with `ISGSTOVERRIDDEN` Yes, `ALTERID` 50 to 69, `AMOUNT` unchanged; no other voucher's
  XML differs between the two answers. The screen's counts for August went from 39 vouchers, 27 included, 7 not
  relevant and 5 uncertain to 39, 28, 7 and 4, as the lab read them. The screen grouped the five uncertain vouchers
  under two reasons, two for an invalid, mismatched or missing HSN/SAC and three (`BP/26-27/0002`, `0003`, `0008`)
  for a mismatch between the expected and the modified tax amount, which is the reason no field carries. The accept
  asked one question, which was answered Yes; the screen offered nothing to undo it and none was tried. The company
  was restored from a backup taken before the accept, and the marks answer after the restore is byte-identical to the
  first. The before-read already held five vouchers with `ISGSTOVERRIDDEN` Yes (`BP/26-27/0005`, `0006`, `0016`,
  `0043`, `0044`) whose origin is not recorded, so that nothing but Accept As Is sets the flag is not measured.
- **NOT established:**
  - a voucher flagged excluded (never observed);
  - a registered buyer;
  - the reason for a status: no field in the two lists tried carries it, which is a negative measured on those
    lists only;
  - a post-dated voucher whose date lies in a later return period, one that is also uncertain, another voucher
    type, and Tally started afresh between two current dates: the one measured voucher is one Sales invoice, and
    the finding rests on it;
  - a Yes on either deleted flag: on this book the Edit Log was not seen to be on, and the one voucher deleted
    left the answer altogether, so a deleted voucher kept by an Edit Log is unmeasured;
  - more than one of `ISCANCELLED`, `ISOPTIONAL` and `ISPOSTDATED` Yes on one voucher;
  - the party and the amount on an item invoice, a registered buyer, a credit note or another Sales-class type, a
    foreign-currency voucher or a post-dated voucher, and an accept of several vouchers at once: the party and
    amount reads above rest on one book of unregistered-buyer Sales invoices;
  - a voucher both cancelled and optional, and whether Tally lists a cancelled voucher anywhere in the return,
    such as among the documents issued;
  - the cost of the selection on a large book: it was timed on about thirty vouchers, so any size bound a
    program applies is its own provisional choice and not a Tally limit;
  - another release or licence tier.
