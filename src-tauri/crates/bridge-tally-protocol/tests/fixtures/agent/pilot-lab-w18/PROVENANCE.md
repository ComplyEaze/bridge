# Pilot lab W18 replay fixtures

Answers Tally gave on 2026-10-12 on the synthetic company `BRIDGE PILOT LAB` (TallyPrime 7.1 Silver,
licensed mode, the company loaded alone, GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`), sent by the
Windows lab, reads only, one request at a time, each answer written straight to a file with
`curl.exe -o` and never piped. The data is synthetic: the company, its GSTIN (of the synthetic family
the fixtures may carry) and its ledgers were made for these measurements.

Together they are the reads a Sales invoice build sent when they were taken, from one book state (the
book that holds the four-digit sales ledger `BRIDGE Svc 4D 5%`), so a build on that ledger can be
replayed from them. The code has since added three reads that the lab was not asked on this book: the
replay serves their answers DERIVED (see "Three answers the replay serves DERIVED" below). They come from two sittings on that day, tied together by the company's marks, and one on 13 Oct for the two listings whose to-dates the build asks differently (below):

- **W18** (15:58 UTC), reads only, no backup taken: every read of the build but the verification
  window. The answers are on `lab/1342-w18-answers` (commits `0f475e40d` and `43a6de41e`), with the
  lab's hash table, and each file here is byte-equal to the file there. Its two marks reads (first and
  last of the sitting) are byte-equal, SHA-256 `0a2d7f08...`: the book did not move.
- **W18b** (18:24 UTC): the verification window, which W18 did not take, read from W18's book state
  restored from the backup `W19-C` (taken before W19's first write; W19's first marks equal W18's).
  Its marks, read before and after the window, are byte-equal to W18's, SHA-256 `0a2d7f08...`, which
  is what ties the two sittings: the window was read from W18's book. The answers are on
  `lab/1342-w18b-answers` (commit `c9fca2a36`), with the lab's hash table; the file here is byte-equal to the file
  there. The book was put back as it was after the read (the lab's marks `8a0c4268...`, equal to the
  previous sitting's last).

The lab's files and the published copies of the compliance listing and the tax units differ, by a
same-length marker over one synthetic GSTIN in the published copy. The two fixtures here are the lab's
own bytes (the `answers-raw` files of `lab/1342-w18-answers`), as in the 12 Oct group of
`../pilot-lab/PROVENANCE.md`.

Three answers of the same sittings are byte-equal to files already in `../pilot-lab/` and are read from
there, not copied: `pilot-lab-company-extent.utf16le.xml` (SHA-256 `0cb23618...`),
`pilot-lab-currencies.utf16le.xml` (`fe2dee55...`) and `pilot-lab-ledger-rates-hsn.utf16le.xml`
(`fe04e3fb...`). The number read here is also byte-equal to `pilot-lab-number-new-invoice.utf16le.xml`
(`947fbf36...`): it is the answer to a different request (the number `BP/26-27/0040`), so it is kept
under its own name.

The requests are on `lab/1342-capture-requests` (commit `7fb92a528`): the `r` files in its top folder
and, in `workorder/`, `w14-company-object.xml`, `w16-ledger-rates-hsn.xml` and `w18-number-0040.xml`
(`r09-number-new-invoice.xml` with the number changed and nothing else). The replay test checks, at
every position, the SHA-256 of the request ComplyEaze Bridge sent against the request the lab answered.

| Fixture | Bytes | SHA-256 | Published as | Request (SHA-256 of the request file) | What it is |
| --- | ---: | --- | --- | --- | --- |
| `w18-marks.utf16le.xml` | 3,394 | `0a2d7f088c55bd6c1d0e827e040b62e88a28f242b7e7388c11259ecce7763a6d` | `answers/aw18-marks-0.xml` (W18; `aw18-marks-1.xml` and, in W18b, `aw18b-marks-0.xml` and `aw18b-marks-1.xml` are byte-equal) | `r02-marks.xml` (`81892a744009b6cf462a6f8d2fd505e650f8ad022e1462b23414cf2b8b3d2d7c`) | the company's change counters |
| `w18-book-extent.utf16le.xml` | 3,836 | `e5b36a5317e349297b81498507be1ed8993c56469dbc3b19ff53904ef439991c` | `answers/aw18-book-extent.xml` (W18) | `r03-book-extent.xml` (`eff1280ac2e8244c49855d4b178d3006fc31ad4e517d17941fcc25080a783c49`) | the book's counters and its last voucher date |
| `w18-ledgers-compliance.utf16le.xml` | 38,278 | `4976bbece519394b53b37b616b169f12d3438b625445d90a3380994393e07216` | `answers-raw/aw18-ledgers-compliance.xml` (W18; the published copy in `answers/` has one GSTIN masked); byte-equal to `answers/aw18c-compliance.xml` (W18c), the answer to the request the build sends | `w18c-ledgers-compliance-to-20260915.xml` (`b0c02fc724502e10dc23a9d6b430c9cbb20c191df948082f06fab9d7e4c8f9bc`; `r05-ledgers-compliance.xml`, `c283b511...`, with `SVTODATE` 20260915) | the 18 ledgers' GST details |
| `w18-ledgers-paired.utf16le.xml` | 25,518 | `fc6f1d6c04ebb4d912113d6c6db1499a014869a2fbbcf992ba63abf7482a84ce` | `answers/aw18c-paired.xml` (W18c; W18's own answer to `r06` with `SVTODATE` 20260801 was 25,480 bytes, `ffe23742...`, and differs: see below) | `w18c-ledgers-paired-to-20261001.xml` (`41b2de3ed4488e5943d25fe36b2c2727f397ca01bae4082d7af36f787bb4b4b5`; `r06-ledgers-paired.xml`, `71326bb0...`, with `SVTODATE` 20261001) | each of the 18 ledgers with its group |
| `w18-groups.utf16le.xml` | 55,954 | `8360cacb9c47d6f14516c524ae5b62e7111a317718388a963b3652fb0c4d8823` | `answers/aw18-groups.xml` (W18) | `r07-groups.xml` (`20d147b548e716b876a013ec58593327a6b5b52ce20d68747c45d878dc98699f`) | the 29 account groups, `BRIDGE Sales Sub` among them |
| `w18-voucher-types.utf16le.xml` | 293,130 | `3b087e281b61a589abe6c1308c787bc8f73c059d1028a0a1943a0ff22ef6737d` | `answers/aw18-voucher-types.xml` (W18) | `r08-voucher-types.xml` (`c351d6c62a09ce5173355f3998ef12746f01aba253ea8bdc42664b1b9766fb00`) | the voucher types |
| `w18-number-0040.utf16le.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` | `answers/aw18-number-0040.xml` (W18) | `workorder/w18-number-0040.xml` (`84f6b7228f3e8c94d028c302c289ed7f99d9076eb45b4b0676adcbc67dc497d8`) | vouchers numbered `BP/26-27/0040`: none |
| `w18-tax-units.utf16le.xml` | 22,636 | `e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989` | `answers-raw/aw18-tax-units.xml` (W18; the published copy in `answers/` has two GSTINs masked) | `r16-tax-units.xml` (`c763f19d79b1e1736ecd17578fe5f353f9b1c4d781382073286484749f181a38`) | the company's tax units |
| `w18-company-object.utf16le.xml` | 86,692 | `98afd2f9d8ae01a3344e49c519bac8909eef9431ef382a180cfbd1ed30d2fbd2` | `answers/aw18-company-object.xml` (W18) | `workorder/w14-company-object.xml` (`72d8cd921a2a89aa781cbd768d013e58f167c2dc5072249fa92df5bdad301648`) | the company object: its GUID, HSN/SAC rows and settings |
| `w18-ledger-catalogue-v2.utf16le.xml` | 25,010 | `873803dd0ab3eab2e0a4dc315afb109f40e11fe3b81cf96fc915c04a8df97a6c` | `answers/aw18-ledger-catalogue-v2.xml` (W18) | `r13-ledger-catalogue-v2.xml` (`28a47c26a29fa799856c112b71e19076946d4c0f85a6dcc65f5cdafe0e1e1bd0`) | the 18 ledgers with identities and bill-wise flags |
| `w18b-verification-window.utf16le.xml` | 87,884 | `b42a98cd6268f7ee4f4d589b68c777d96d7243a0657b20126d90cd1ac6128093` | `answers/aw18b-verification-window.xml` (W18b) | `r15-verification-window.xml` (`a40f220e631d1c1d0ad0028e1f52f651288af6098f8960bff0aa5e24f3ca02e7`) | the 1-Aug-2026 window: eleven vouchers, `BP/26-27/0001` to `0011` |

## The to-date of two listings (measured, W18c)

The lab was first asked `r05` and `r06` with `SVTODATE`
  20260801, the last voucher date of the 9 Oct book. On this book the last voucher date is 20260915 (the
  book-extent answer above), so the build asks the compliance listing with `SVTODATE` 20260915 and the
  paired listing with 20261001. W18c (13 Oct 2026, `lab/1342-w18c-answers` commit `0c283834e`) asked
  those two exact requests from the same book state (the `W19-C` backup restored; its first and second
  marks equal W18's, `0a2d7f08...`): the compliance answer is byte-equal to W18's (38,278 bytes), and the
  paired answer is not: the same 18 ledgers, `CLOSINGBALANCE` different on 13 of them and nothing else
  different, so its closing balances follow the to-date. The paired fixture here is therefore W18c's
  answer. The replay test names the SHA-256 of each request the build sends; every request is one the lab
  answered, byte for byte, at every position (status probes aside), except the three kinds in the next
  section.

## Three answers the replay serves DERIVED

The code now sends three reads the lab was not asked on this book, and the replay serves a derived
answer for each (named DERIVED where it is built, in `agent_import_tests.rs` and
`agent_import_w18_replay_tests.rs`). None is Tally's bytes.

- **`Q`, the rates read restricted to the invoice's ledgers by GUID, with the three HSN/SAC fields (#1331).** The request is
  the code's, SHA-256 `a7ced791a7af67b3bcda6de614970891ebfb1ff82da8b9a580fafc6e12b654b2`; it names this invoice's six
  ledgers and was never sent to the lab. The lab answered the same request naming five other ledgers of this book on
  31 Oct 2026 (`../pilot-lab/pilot-lab-ledger-rates-guid.request.utf16le.xml`), and a test derives this request's
  SHA-256 from that one with only its GUIDs changed. The answer is the lab's 21 Oct 2026 answer to the request
  restricted to the four parents the invoice's ledgers sit under
  (`../pilot-lab/pilot-lab-ledger-rates-scoped-hsn.utf16le.xml`, whose catalogue of the same minute is byte-equal to
  this directory's `w18-ledger-catalogue-v2`, and which the lab answered byte for byte the same on 31 Oct 2026), cut to
  the invoice's six ledgers by `rates_cut_to`: each other row removed with the whitespace after it, in answer order.
  On 31 Oct 2026 the lab's GUID answers for five and for four ledgers were, byte for byte, that answer cut this way;
  a six-term filter was not measured.
- **`X` and `Y`, the voucher-type numbering reads.** The requests are the code's, `229f54c9...` and
  `e83a1df1...`; the lab answered them on 16 Oct 2026 (`../pilot-lab/pilot-lab-numbering-masters.utf16le.xml`
  and `...-series.utf16le.xml`), when its book stood at master mark 245, not at W18's 243. The replay serves
  those answers with the rows above 243 removed: one row, `BRIDGE Sales Two` (AlterID 245), counted and
  named by the test; the rows left must equal this directory's voucher-types read in name, AlterID and
  GUID. The series-level fields of the rows left have no W18 capture, so for them the inference is not
  checked.

Captures of the numbering reads on this book are owed (`CAPTURE-1640` and its W18 addendum, written for the next lab sitting). A
capture of the numbering reads at W18's state needs the book restored from the backup the W18b and W18c
sittings used; on the live book, whose mark is now 245, it would be the numbering of a different state.

## What is not measured

- **A post.** No write was sent in either sitting.
