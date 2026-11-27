# Party bills fixtures: BRIDGE BILLTRAIL LAB (#2377)

Live answers to the `Ledger Outstandings` report for one party, and to the `List of Ledgers` snapshot each is tied
to, on the synthetic company `BRIDGE BILLTRAIL LAB` loaded alone on TallyPrime 7.2 Silver, port 9001, Windows lab,
8 Nov 2026. Each request was a fixed file sent with curl, one at a time, and each answer was written to a file as
received (BOM-less UTF-16LE; the fixture tree's attributes keep them `-text`). The company, its ledgers and its vouchers
are synthetic. Part A only read; Part B made its parties and bills behind a backup and restored the book after.

- Part A: evidence branch `lab/2377-l-answers` at `a2be6d96` (its `SHA256.txt` lists each file with the same hash).
- Part B: evidence branch `lab/2377-b-answers` at `44e44b28`.
- Row 2377-NB: evidence branch `lab/2377-nb-answers` at `61a5c1bf`, on another synthetic company, `BRIDGE PILOT LAB`,
  loaded alone on TallyPrime 7.2, 9 Nov 2026 UTC, 10 Nov in India, reads only. Its bill-wise payable party
  `BRIDGE Supplier RJ` to
  31 July 2026, before its first entry, answered no bytes twice, with headers byte-equal to
  `ledger-outstandings-unknown-party.headers.txt`, and the snapshot to the same date carries the party, bill-wise, with
  an empty `CLOSINGBALANCE` (`ledger-snapshot-supplier-rj-20260731.utf16le.xml`). That snapshot request was
  `render_native_ledger_snapshot_request` from the book's first day, 1 April 2026 (its books begin then, as
  `agent/pilot-lab-register/pilot-lab-register-book-extent.utf16le.xml` records), to the same date: the period the
  party-bills read builds. Only that snapshot tells this party
  from a name no ledger carries (protocol reference §12a.27).
- The Ledger Outstandings request: `<ID>Ledger Outstandings</ID>`, `SVCURRENTCOMPANY`, `LedgerName`, `SVTODATE`,
  `SVEXPORTFORMAT` `$$SysName:XML` and `EXPLODEFLAG` `Yes`; no `SVFROMDATE`, no fetch list. The snapshot request is
  `render_native_ledger_snapshot_request` from the books' first day, 1 April 2025, to the same to-date.
- An unknown name answered no bytes twice, with a healthy `/status` between. A name differing only in letter case or
  a doubled inner space answered byte-equal to the exact name's answer (not kept here: the same bytes as
  `ledger-outstandings-p1-20260331.utf16le.xml`).

Tests: `src/native_outstandings/party_bills_tests.rs` parses each answer and ties it to its ledger's closing; the parse
record keeps what each parser returns.

Confidence, with the markers of `docs/tally/TALLY_PROTOCOL_REFERENCE.md`: PARTIAL for every finding here. Each was
observed live, with request and answer captured, but on two synthetic books and one release, in sittings on 8 Nov
2026 and on 9 Nov 2026 UTC, 10 Nov in India; Bridge's own transport did not send these requests.

Protocol behaviour these captures record, for the declared reference part (P9), each PARTIAL on the scope above. The
answer to a `LedgerName` no ledger carries (`ledger-outstandings-unknown-party.*`) is recorded in §12a.27 of
`docs/tally/TALLY_PROTOCOL_REFERENCE_MEASUREMENTS_AND_OPEN_QUESTIONS.md`. These two are not yet in a declared part of
`docs/tally/TALLY_PROTOCOL_REFERENCE.md`, which is a pinned file and is changed in a pull request of its own:
- A ledger made with a trailing space in its name is held without it, and the `List of Ledgers` snapshot names it so
  (`ledger-snapshot-trailing-space.utf16le.xml`).
- Of two On Account rows on one party, the second leaves `BILLDUE` and `BILLOVERDUE` empty
  (`ledger-outstandings-two-on-account.utf16le.xml`).

Not established: Bridge's own transport against a live Tally for this report (curl sent it); a payable party with
a non-empty answer captured in this folder (§12a.27 records one on another book); a party with bill-wise off; a
foreign-currency ledger; a real bill whose reference is the text
`On Account`; a party with entries and no open bills (row 2377-NB above and §12a.27 record one with no entry by its
date); other releases, editions, a client book.

| Fixture | Bytes | SHA-256 | Source (lab file) | What it is |
| --- | ---: | --- | --- | --- |
| `ledger-outstandings-p1-20260331.utf16le.xml` | 5,878 | `063b3c7684b4fe416a0749c114e3b454fe59d71eb6e9bd0e01c9db25bb86de5b` | Part A `answers/s3-lo-p1-0331-a.answer.xml` | `Lab Party P1` at 31 March 2026: five bills and one On Account row, Σ BILLCL -26000.00; a second send was byte-equal |
| `ledger-outstandings-p1-20260731.utf16le.xml` | 3,926 | `f505ba966c106ecd6674bffb390187e6a1e8c52ffc33d173ed87bbdd3ec88091` | Part A `answers/s3-lo-p1-0731.answer.xml` | the same party at 31 July 2026: four bills and one On Account row, Σ BILLCL -37000.00 |
| `ledger-snapshot-p1-20260331.utf16le.xml` | 10,378 | `e111b013092484701c172917728a4e3dfdd90be6f5acab10070396ca4b490cf9` | Part A `answers/s2-snap-0331.answer.xml` | `List of Ledgers` from 1 April 2025 to 31 March 2026; `Lab Party P1` closes at -26000.00 |
| `ledger-snapshot-p1-20260731.utf16le.xml` | 10,378 | `829ee265067cdc7d3b18c20fb276ac4d8d5e565a370bfbc13fe1c8f84328e419` | Part A `answers/s2-snap-0731.answer.xml` | the same to 31 July 2026; `Lab Party P1` closes at -37000.00 |
| `ledger-outstandings-unknown-party.utf16le.xml` | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Part A `answers/s6-lo-unknown.answer.xml` | a name no ledger carries: no bytes at all |
| `ledger-outstandings-unknown-party.headers.txt` | 77 | `547c40c5c5f4e011efd4f95175768b2ae81f0dfd351f7e36d0b3b9821a584e0b` | Part A `answers/s6-lo-unknown.headers.txt` | that answer's headers: HTTP 200, `RESPSTATUS: 1`, `CONTENT-LENGTH: 0` |
| `ledger-outstandings-ampersand.utf16le.xml` | 846 | `75b96699bb2902437a3eb5040eb857ce3edc26912c7cf641c0b9b3d19a6564c8` | Part B `answers/b5-amp-r-lo.answer.xml` | `L5 Amp & Co` (sent escaped): one bill, -100.00 |
| `ledger-snapshot-ampersand.utf16le.xml` | 12,822 | `6d8fbc6d496f94c0c653a1ee302bd19d9d4bc0f18e34c2a7cc852208eeb4e6ce` | Part B `answers/b5-amp-snap.answer.xml` | the snapshot after that party was made; it closes at -100.00 |
| `ledger-outstandings-trailing-space.utf16le.xml` | 852 | `72b8f02a2aae65f8a8125ce66a04d8d4fda1df5e4e9e8bcbd01000a3e8b7a086` | Part B `answers/b5-trail-r-lo.answer.xml` | `L5 Trail ` sent with its trailing space: the party's one bill, -100.00 |
| `ledger-snapshot-trailing-space.utf16le.xml` | 29,934 | `5f78ea690df025c61bdceaf3b73b568b3fc06c81de67805a5b1df1ca57720123` | Part B `answers/b5-trail-snap.answer.xml` | the whole book's snapshot after it, 22 ledgers: Tally holds the party as `L5 Trail`, without the space |
| `ledger-outstandings-two-on-account.utf16le.xml` | 1,636 | `00b538866cc8327666694432575889e787dde4d5c6c496d799d3b1914406477c` | Part B `answers/b6-r-lo.answer.xml` | `L7 Party` with two On Account rows, -100.00 and -50.00; the second leaves BILLDUE and BILLOVERDUE empty |
| `ledger-snapshot-supplier-rj-20260731.utf16le.xml` | 25,298 | `552bc05e5ff566f3b51372b3bd5e9f3ce56e1b49b40aa12e4b0585dfeaac423f` | Row 2377-NB `answers/a-s2-S-snap-20260401-20260731.xml` | `BRIDGE PILOT LAB`'s `List of Ledgers` from its first day, 1 April 2026, to 31 July 2026: `BRIDGE Supplier RJ` bill-wise with an empty `CLOSINGBALANCE`; every closing in it is empty |

## Names with marks in `LedgerName` (#2480)

Live answers to the same `Ledger Outstandings` request, with party names that hold marks, from four rows on the
synthetic company `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, in the synthetic-UUID registry),
loaded alone on TallyPrime single-user edition (`tally.exe` file version 1.1.7.2; the Gateway names no release),
10 and 11 November 2026 UTC. Each row made eleven bill-wise ledgers under `Sundry Debtors`, each with one invoice of
-1051.00, behind a backup, read them, and restored the book; the reads after the restore were byte-equal to those
before. Each request was a fixed file sent with curl, one at a time, its sha256 checked before the send; each answer
was written to a file as received. Every file below is byte-equal to its line in its branch's `SHA256.txt`.

- Row 2480-NS: answers `lab/2480-ns-answers` at `24951ffb`, sheet `lab/2480-ns-sheet` at `6e44a3dc`. One mark a name:
  `-`, `.`, `&`, `/`, `(`, `)`, `'`, the word `and`, digits with a dash, a Devanagari word.
- Row 2480-NX: answers `lab/2480-nx-answers` at `ea691186`, sheet `lab/2480-nx-sheet` at `ea84bcdf`. One mark a name:
  `,`, `_`, `"` as a pair round a word, `+`, `@`, `#`, `:`, `%`, `*`, `!`.
- Row 2480-NC: answers `lab/2480-nc-answers` at `2ce5b6ea`, sheet `lab/2480-nc-sheet` at `8eb5012c`. Mixes of NS's
  marks; two of its eleven names are kept here.
- Row 2480-MX: answers `lab/2480-mx-answers` at `e88cd036`, sheet `lab/2480-mx-sheet` at `080692b7`, part A. Mixes of
  the seventeen marks; its name holding all seventeen is kept here.

What was observed, in each row: before its ledger existed, each name was answered with no bytes (the answer
`ledger-outstandings-unknown-party.*` records); every import was `CREATED` 1 with every other counter 0; each
ledger's name list was its own name alone; no two names of a row, and no name and a ledger the book already held, had
the same letters-and-digits key; and each name, sent twice, was answered with the same bytes, one row, that ledger's
own bill, which equals that ledger's group in the row's Bills Receivable read and sums to its closing in the snapshot
kept here. The answers hold no party name, so a test ties each to its name's row of the snapshot and checks its bill
number. The control of each row, a plain name, is not kept.

The request files are the lab's, as sent: UTF-16LE with a byte-order mark. The answers and snapshots are as received,
without one. The tests strip the mark before comparing a request with the one Bridge builds.

Kept: twenty-three marked names, NS's ten and NX's ten, two of NC's eleven names and one of MX's eleven (each row's
eleven being ten mixes and a plain control), each with its request, one answer and the row's snapshot. Not kept: NS's
and NX's controls, NC's other nine names and MX's other ten (their controls among them), the answers before each
ledger existed, the second answer of each pair, and the Bills Receivable reads.

Tests: `src/native_outstandings/party_name_tests.rs` builds each name, compares Bridge's request with the lab's,
parses the answer and ties it to that name's row of the snapshot. Its test of the rule reads every kept request's
`LedgerName` and requires the type's tables to equal what those names show: each mark's placings, that no placing
appears twice, each mark's most per name, the pairs of neighbouring word kinds, `and` and the Devanagari word at most
once, and the marked-name length cap (72 characters, the longest kept marked name). The pairing of quotes and
brackets is checked against those names but chosen, not derived from them.

Confidence, with the markers of `docs/tally/TALLY_PROTOCOL_REFERENCE.md`: PARTIAL for every finding here (protocol
reference §12a.34). Each was observed live, request and answer captured, on one synthetic book, one release, in four
sittings; Bridge's own transport did not send these requests.

Not established: a name whose letters-and-digits key another ledger shares, through this report, for any mark but
`'` and `_` (rows 2480-SK and 2480-SC, not kept here); a mark placed other than in a kept name (first, beside another
mark or beside Devanagari, or between other neighbours), a name of marks only, a marked name starting with a digit or
longer than 72 characters; `and` first, last, beside a mark or in another case; Devanagari other than the whole word
read, or first; any mark or formula word not named above; groups and other masters sharing a key, and look-alike
characters in other ledgers' names; any other book, release or edition; Bridge's own transport.

| Fixture | Bytes | SHA-256 | Source (lab file) | What it is |
| --- | ---: | --- | --- | --- |
| `ledger-snapshot-2480-ns-20260801.utf16le.xml` | 39,154 | `6338e8c0eca34346065898d02fce949b988917aad5a4a3bd6283dfe911d05462` | Row 2480-NS `answers/a-s6sa-S-snap-20260401-20260801.xml` | `List of Ledgers` from 1 April 2026 to 1 August 2026 after the row's set-up, the first of two byte-equal sends: a row for each of its eleven names, each bill-wise, closing -1051.00 |
| `ledger-outstandings-2480-ns-hy-20260801.utf16le.xml` | 896 | `9106fcd53bad280111db32938332c7da1f1a8831ac5371cc4e78523bf29ef1b4` | Row 2480-NS `answers/a-r2a-L-lo-hy-20260801.xml` | `BRIDGE Ns Hk-Hm` (`-`): one bill, `BP/26-27/0222`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-ns-hy-20260801.request.utf16le.xml` | 820 | `96a30cfec7f48c6cc1bdbc5208a108e7ca96fd9ad543804305a53c13907f21c3` | Row 2480-NS sheet `L-lo-hy-20260801.xml` | the request the lab sent for `BRIDGE Ns Hk-Hm` (`-`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-ns-dt-20260801.utf16le.xml` | 896 | `81c33e352aaa4a406041e112373a6b91a3d52a25da0a811939adb07dae63da34` | Row 2480-NS `answers/a-r3a-L-lo-dt-20260801.xml` | `BRIDGE Ns Dk.Dm` (`.`): one bill, `BP/26-27/0223`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-ns-dt-20260801.request.utf16le.xml` | 820 | `8cbdddd9e41f685d1e6e05862f1a7bb502d07abd7001559117e66265a14d5064` | Row 2480-NS sheet `L-lo-dt-20260801.xml` | the request the lab sent for `BRIDGE Ns Dk.Dm` (`.`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-ns-am-20260801.utf16le.xml` | 896 | `7105759d757e21ba5281ea1ef859545698446ce4f77b047034021da348fac98b` | Row 2480-NS `answers/a-r4a-L-lo-am-20260801.xml` | `BRIDGE Ns Ak&Am` (`&`): one bill, `BP/26-27/0224`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-ns-am-20260801.request.utf16le.xml` | 828 | `6614d844e8afea67cf5baeb1223641c149f20b46d70adae73db186cbc693f580` | Row 2480-NS sheet `L-lo-am-20260801.xml` | the request the lab sent for `BRIDGE Ns Ak&Am` (`&`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-ns-sl-20260801.utf16le.xml` | 896 | `59ef6d45929487f74b4c85e653bbfc9526133927e026e593a6c09be27f17e2ec` | Row 2480-NS `answers/a-r5a-L-lo-sl-20260801.xml` | `BRIDGE Ns Sk/Sm` (`/`): one bill, `BP/26-27/0225`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-ns-sl-20260801.request.utf16le.xml` | 820 | `249fe7bf1a9843ccdcd939f0a7aefc95fac7a456da7e4631749597d65917f8cf` | Row 2480-NS sheet `L-lo-sl-20260801.xml` | the request the lab sent for `BRIDGE Ns Sk/Sm` (`/`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-ns-lp-20260801.utf16le.xml` | 896 | `dfca8eb42cf152e96ba8c6c75de5c385f53cb9461a1bcf61235349270fa971b4` | Row 2480-NS `answers/a-r6a-L-lo-lp-20260801.xml` | `BRIDGE Ns Lk(Lm` (`(`): one bill, `BP/26-27/0226`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-ns-lp-20260801.request.utf16le.xml` | 820 | `a9b9d905d53332f3eaa3b2e781a921a5f220cc5276dfd901de362d0ddaf08d6d` | Row 2480-NS sheet `L-lo-lp-20260801.xml` | the request the lab sent for `BRIDGE Ns Lk(Lm` (`(`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-ns-rp-20260801.utf16le.xml` | 896 | `6b1e1ad206f91b0e9a2dd37e812f79ceeb30b8bdac90ea4f2d33c36d773bfa84` | Row 2480-NS `answers/a-r7a-L-lo-rp-20260801.xml` | `BRIDGE Ns Rk)Rm` (`)`): one bill, `BP/26-27/0227`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-ns-rp-20260801.request.utf16le.xml` | 820 | `e23c683e1e41f5cb06bbc5acdecd806d845a6e86c94bd713e28d2b2303f8d42e` | Row 2480-NS sheet `L-lo-rp-20260801.xml` | the request the lab sent for `BRIDGE Ns Rk)Rm` (`)`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-ns-ap-20260801.utf16le.xml` | 896 | `099644837f6f566193215369d20a2ef0eb40beabe1f0e7b94d84ddc8f40e3a3d` | Row 2480-NS `answers/a-r8a-L-lo-ap-20260801.xml` | `BRIDGE Ns Pk'Pm` (`'`): one bill, `BP/26-27/0228`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-ns-ap-20260801.request.utf16le.xml` | 830 | `dcfff48069d1960e1895a4eb700d939c2889b63cadac43d8b8fe75e6b36843b6` | Row 2480-NS sheet `L-lo-ap-20260801.xml` | the request the lab sent for `BRIDGE Ns Pk'Pm` (`'`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-ns-op-20260801.utf16le.xml` | 896 | `15fbc04f4a9f48a0467dd405d869fe59cf16b4fc4fb6106fe61f3d7f10f1787d` | Row 2480-NS `answers/a-r9a-L-lo-op-20260801.xml` | `BRIDGE Ns Ok and Om` (the word `and`): one bill, `BP/26-27/0229`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-ns-op-20260801.request.utf16le.xml` | 828 | `940d4308878b55efa99586101b45b3628c809662a3bf7bde2d499f02c153d0f0` | Row 2480-NS sheet `L-lo-op-20260801.xml` | the request the lab sent for `BRIDGE Ns Ok and Om` (the word `and`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-ns-dd-20260801.utf16le.xml` | 896 | `2b4b8b82e2d97c5e9723498a52cf88ea3ded2c4a2466a796b348c37284dc3c95` | Row 2480-NS `answers/a-r10a-L-lo-dd-20260801.xml` | `BRIDGE Ns 2023-24` (digits with a dash): one bill, `BP/26-27/0230`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-ns-dd-20260801.request.utf16le.xml` | 824 | `2b364be4cecccac7ccf79ad07c0eef993b11d268bb3e2d6cc72d6871a6ffdddc` | Row 2480-NS sheet `L-lo-dd-20260801.xml` | the request the lab sent for `BRIDGE Ns 2023-24` (digits with a dash), as sent, with its byte-order mark |
| `ledger-outstandings-2480-ns-sc-20260801.utf16le.xml` | 896 | `b9ca8c484b1fcb6ab4b1de9f3acc527a524a2c90b39bdf8c0aaf135b54955465` | Row 2480-NS `answers/a-r11a-L-lo-sc-20260801.xml` | `BRIDGE Ns उदाहरण` (a Devanagari word): one bill, `BP/26-27/0231`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-ns-sc-20260801.request.utf16le.xml` | 822 | `e019f0ab21f2714a24d9491f83bf8312b34971b3dbecb766875a84b38c2dfb81` | Row 2480-NS sheet `L-lo-sc-20260801.xml` | the request the lab sent for `BRIDGE Ns उदाहरण` (a Devanagari word), as sent, with its byte-order mark |
| `ledger-snapshot-2480-nx-20260801.utf16le.xml` | 39,182 | `ee6c660ecf8b292374de172cfbe729fc6e7613e5b47f0926cbdc78b150528e65` | Row 2480-NX `answers/a-s6sa-S-snap-20260401-20260801.xml` | `List of Ledgers` from 1 April 2026 to 1 August 2026 after the row's set-up, the first of two byte-equal sends: a row for each of its eleven names, each bill-wise, closing -1051.00 |
| `ledger-outstandings-2480-nx-cm-20260801.utf16le.xml` | 896 | `ffc0820068ca9fee973dcaa17b38fe2f6161848faba94c321dfcc622f5725b58` | Row 2480-NX `answers/a-r2a-L-lo-cm-20260801.xml` | `BRIDGE Nx Ak, Am` (`,`): one bill, `BP/26-27/0262`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-nx-cm-20260801.request.utf16le.xml` | 822 | `d1fc1f8f1951f373ca1168a2c66b333ac9ced1927edc8f19dd16cc7ea6ca6244` | Row 2480-NX sheet `L-lo-cm-20260801.xml` | the request the lab sent for `BRIDGE Nx Ak, Am` (`,`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-nx-us-20260801.utf16le.xml` | 896 | `d7abfed5a7e2352a1a423a20392355c981e7f9f7360ce782d3dd140770ffe900` | Row 2480-NX `answers/a-r3a-L-lo-us-20260801.xml` | `BRIDGE Nx Bk_Bm` (`_`): one bill, `BP/26-27/0263`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-nx-us-20260801.request.utf16le.xml` | 820 | `cd11c44291b6f80101d99793ab45b1c2c67fb962dfadc6ef877b6027feaa73bf` | Row 2480-NX sheet `L-lo-us-20260801.xml` | the request the lab sent for `BRIDGE Nx Bk_Bm` (`_`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-nx-dq-20260801.utf16le.xml` | 896 | `3b313034c1e95ea0f726009ff4b0d3a059a9206159d8bc2de5235deb61d5519d` | Row 2480-NX `answers/a-r4a-L-lo-dq-20260801.xml` | `BRIDGE Nx "Ck" Cm` (`"` as a pair): one bill, `BP/26-27/0264`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-nx-dq-20260801.request.utf16le.xml` | 844 | `20c5052fd517c39dde0f7de860ad8e242cc0e3f1ff3c491adefc75af02a64ff5` | Row 2480-NX sheet `L-lo-dq-20260801.xml` | the request the lab sent for `BRIDGE Nx "Ck" Cm` (`"` as a pair), as sent, with its byte-order mark |
| `ledger-outstandings-2480-nx-ps-20260801.utf16le.xml` | 896 | `d908e7593f001f4f8fa6c92ef63a3f7f3226edd8f5c8118731645c7e9d93b5f9` | Row 2480-NX `answers/a-r5a-L-lo-ps-20260801.xml` | `BRIDGE Nx Dk + Dm` (`+`): one bill, `BP/26-27/0265`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-nx-ps-20260801.request.utf16le.xml` | 824 | `03d689746bb523fb0e08005b93b47e99b17d9c7477fbec64d1e1cfa67e9bda1b` | Row 2480-NX sheet `L-lo-ps-20260801.xml` | the request the lab sent for `BRIDGE Nx Dk + Dm` (`+`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-nx-at-20260801.utf16le.xml` | 896 | `22ad6aa26c058043816ded8324ae45dfe7a7cf8aa93af9ad698d3777e23564a7` | Row 2480-NX `answers/a-r6a-L-lo-at-20260801.xml` | `BRIDGE Nx Ek @ Em` (`@`): one bill, `BP/26-27/0266`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-nx-at-20260801.request.utf16le.xml` | 824 | `956f16a1e08a21ffd7016e863f443d71a35af44b0c3f1b6c437d6ce8cc33a5cf` | Row 2480-NX sheet `L-lo-at-20260801.xml` | the request the lab sent for `BRIDGE Nx Ek @ Em` (`@`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-nx-hs-20260801.utf16le.xml` | 896 | `01707a6b3dbc5bf469c72408552fead2bd353d82b004c1a3d73e7e55c09ff3df` | Row 2480-NX `answers/a-r7a-L-lo-hs-20260801.xml` | `BRIDGE Nx Fk #12` (`#`): one bill, `BP/26-27/0267`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-nx-hs-20260801.request.utf16le.xml` | 822 | `203e451fa88bfc8de5fb6a52809ac46da9c02d72e1c14d5649d5a916e2c5f5b1` | Row 2480-NX sheet `L-lo-hs-20260801.xml` | the request the lab sent for `BRIDGE Nx Fk #12` (`#`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-nx-cl-20260801.utf16le.xml` | 896 | `cd3946a9310c5f5b02988bc6c678601a321902674c769e2300ffc1b1d689e73d` | Row 2480-NX `answers/a-r8a-L-lo-cl-20260801.xml` | `BRIDGE Nx Gk: Gm` (`:`): one bill, `BP/26-27/0268`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-nx-cl-20260801.request.utf16le.xml` | 822 | `53402b3e0a2a1c3ec629be3ad24594ddf19ba00e53203035c370f4d3bfe040fc` | Row 2480-NX sheet `L-lo-cl-20260801.xml` | the request the lab sent for `BRIDGE Nx Gk: Gm` (`:`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-nx-pc-20260801.utf16le.xml` | 896 | `f3973ccc1acb2708af11ed361d494d760eb0dd07059b5586bc9928f2da4bf42a` | Row 2480-NX `answers/a-r9a-L-lo-pc-20260801.xml` | `BRIDGE Nx Hk 15% Hm` (`%`): one bill, `BP/26-27/0269`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-nx-pc-20260801.request.utf16le.xml` | 828 | `aaccaf538744f01cb344b08b08cc20ab0992808724ffe3977c4556b82829660b` | Row 2480-NX sheet `L-lo-pc-20260801.xml` | the request the lab sent for `BRIDGE Nx Hk 15% Hm` (`%`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-nx-st-20260801.utf16le.xml` | 896 | `beaea943a92fb4c5544137fa5edac1b836def53d56199e40ad076d092751bbf3` | Row 2480-NX `answers/a-r10a-L-lo-st-20260801.xml` | `BRIDGE Nx Jk * Jm` (`*`): one bill, `BP/26-27/0270`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-nx-st-20260801.request.utf16le.xml` | 824 | `0e8e0517a400f680b68f9d9be17a32e2efdf315e75636d76f1afb27f5673bd20` | Row 2480-NX sheet `L-lo-st-20260801.xml` | the request the lab sent for `BRIDGE Nx Jk * Jm` (`*`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-nx-ex-20260801.utf16le.xml` | 896 | `388027a631c89db43765e121c70e8c4b5c26ce3de1d82d00ff5e71772c49c34d` | Row 2480-NX `answers/a-r11a-L-lo-ex-20260801.xml` | `BRIDGE Nx Kk!` (`!`): one bill, `BP/26-27/0271`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-nx-ex-20260801.request.utf16le.xml` | 816 | `99d2bc332f205a2d4fde687088b68e69dd84f3d3f309b8326051e8d2b9cd47e6` | Row 2480-NX sheet `L-lo-ex-20260801.xml` | the request the lab sent for `BRIDGE Nx Kk!` (`!`), as sent, with its byte-order mark |
| `ledger-snapshot-2480-nc-20260801.utf16le.xml` | 39,634 | `08b7edd1249e2d60739c4d41b04c63792fac32ebbf11e98698af9c344de47b42` | Row 2480-NC `answers/a-s6sa-S-snap-20260401-20260801.xml` | `List of Ledgers` from 1 April 2026 to 1 August 2026 after the row's set-up, the first of two byte-equal sends: a row for each of its eleven names, each bill-wise, closing -1051.00 |
| `ledger-outstandings-2480-nc-sm-20260801.utf16le.xml` | 896 | `53d5013677dbf904a9fd53ac89531dcecae4c683ac4ba6889f2304c7fd303683` | Row 2480-NC `answers/a-r8a-L-lo-sm-20260801.xml` | `BRIDGE Nc उदाहरण & Gk.` (a Devanagari word, `&` and `.`): one bill, `BP/26-27/0248`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-nc-sm-20260801.request.utf16le.xml` | 842 | `014bbfe48101e5b46a8139f4f20fed06598a448cd5bc179b507f8284a4f581b9` | Row 2480-NC sheet `L-lo-sm-20260801.xml` | the request the lab sent for `BRIDGE Nc उदाहरण & Gk.` (a Devanagari word, `&` and `.`), as sent, with its byte-order mark |
| `ledger-outstandings-2480-nc-al-20260801.utf16le.xml` | 896 | `ade384af03dad64a8c69fecc00e1aa54f29683575f18756546d6149fbfbb20a7` | Row 2480-NC `answers/a-r10a-L-lo-al-20260801.xml` | `BRIDGE Nc Jk-Jm. & Jn/Jp (Jq) Jr's and 2023-24` (`- . & / ( ) '`, `and` and digits with a dash): one bill, `BP/26-27/0250`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-nc-al-20260801.request.utf16le.xml` | 900 | `ebe7473a88961ac74f64eaeff5fce6d937f10a7bb7ee4bfd37de2ca7d829fe76` | Row 2480-NC sheet `L-lo-al-20260801.xml` | the request the lab sent for `BRIDGE Nc Jk-Jm. & Jn/Jp (Jq) Jr's and 2023-24` (`- . & / ( ) '`, `and` and digits with a dash), as sent, with its byte-order mark |
| `ledger-snapshot-2480-mx-20260801.utf16le.xml` | 39,750 | `3f6f6c67ff1730c09dc16c9a0904a28440e5ae69d4b38a40f2bbe69ec972dfff` | Row 2480-MX `answers/a-s6sa-S-snap-20260401-20260801.xml` | `List of Ledgers` from 1 April 2026 to 1 August 2026 after the row's set-up, the first of two byte-equal sends: a row for each of its eleven names, each bill-wise, closing -1051.00 |
| `ledger-outstandings-2480-mx-al-20260801.utf16le.xml` | 896 | `7a04c15692e5c806e3a267b56c5b5b4b0ebf03cf35264840acc2d0b08cf10219` | Row 2480-MX `answers/a-r11a-L-lo-al-20260801.xml` | `BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly's` (all seventeen marks): one bill, `BP/26-27/0291`, -1051.00; the second send was byte-equal |
| `ledger-outstandings-2480-mx-al-20260801.request.utf16le.xml` | 972 | `ae355b8f8639f04a79e4a3010559df92ed9b505c5fbad5c7d06a64f1d446a498` | Row 2480-MX sheet `L-lo-al-20260801.xml` | the request the lab sent for `BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly's` (all seventeen marks), as sent, with its byte-order mark |
