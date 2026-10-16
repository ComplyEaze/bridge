# Pilot lab fixtures

Answers Tally gave on 2026-10-09 on the synthetic company `BRIDGE PILOT LAB` (TallyPrime 7.1
Silver, licensed mode), sent by an outside contributor from a machine of their own, one request at a
time, each answer written straight to a file with `curl.exe -o` and never piped. The company was
the only one loaded.

The data is synthetic: the company, its GSTIN (of the synthetic family the fixtures may carry)
and its ledgers were made for this measurement. One book, one release, one day.

## The invoice round (2026-10-09)

The one-invoice round of #1342, sent by the same contributor on the same company, the same way: one request at a time, each answer written straight to a file with `curl.exe -o`. The answers are on `lab/1342-capture-answers` (one second-round read in `fixed2/` at commit `4bceb5d0`, the third-round reads in `fixed3/` at commit `0b85a64c`, the invoice round in `post/` at commit `01cbee0b`); each file here is byte-equal to the file there, and its SHA-256 equals the one in that branch's hash table. The book is the one the third-round reads describe: ten keyed vouchers dated 2026-08-01 (the third round's voucher read on that branch, `fixed3/vouchers-fixed3.xml`).

| Fixture | Bytes | SHA-256 | Published as | Request (SHA-256 of the request file) | What it is |
| --- | ---: | --- | --- | --- | --- |
| `pilot-lab-company-extent.utf16le.xml` | 4,000 | `0cb23618073bc5eeddbc3441fd4f0da9e84c55699cd206b66d1410b304f8d147` | `a01-company-extent-fixed3.xml (commit 0b85a64c)` | `r01-company-extent.xml` (`9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8`) | the loaded companies: one, BRIDGE PILOT LAB |
| `pilot-lab-marks-before-post.utf16le.xml` | 3,394 | `449097451bbce612c7deff74d2847fdf71ea428bbcfb50f84462d14e49623ee8` | `post/a02-marks-pre.xml (commit 01cbee0b)` | `r02-marks.xml` (`81892a744009b6cf462a6f8d2fd505e650f8ad022e1462b23414cf2b8b3d2d7c`) | the company's change counters before the invoice was sent: voucher mark 16, master mark 223 |
| `pilot-lab-book-extent.utf16le.xml` | 3,836 | `aa419ce1f6b133fd054a483b39cd1c435b90f845e1fefaae882c2acdacc54cda` | `post/a03-book-extent-pre.xml` | `r03-book-extent.xml` (`eff1280ac2e8244c49855d4b178d3006fc31ad4e517d17941fcc25080a783c49`) | the book's counters and last voucher date 2026-08-01 |
| `pilot-lab-currencies.utf16le.xml` | 3,400 | `fe2dee55160e8af6d3a61264cb0c4daff70dbfa680212f4a90b39909b921bb96` | `a04-currencies-fixed3.xml` | `r04-currencies.xml` (`56e9306e2a5128da625ed35be222f790d69cc30ce34f5d76cb2ffd98c8727661`) | the base currency |
| `pilot-lab-ledgers-compliance.utf16le.xml` | 24,472 | `8185d7e748518810ac5b8b09e05decec3c1f323372b1b8a491d8a734a2e2f9eb` | `a05-ledgers-compliance-fixed3.xml` | `r05-ledgers-compliance.xml` (`c283b511ac42ae19df3a1d23c21e41504afde8ae75f73a8666f4df7a9bfc6386`) | each of the 11 ledgers' GST details |
| `pilot-lab-ledgers-paired.utf16le.xml` | 16,704 | `faef4bd14d89ac104e5d2d879664c12a45c82a6a67dab24210ba14b3ed335283` | `a06-ledgers-paired-fixed3.xml` | `r06-ledgers-paired.xml` (`71326bb05d54449f8aea5a6966450da70f4fc261f480e17a74feeee718ed8f0f`) | each ledger with its group |
| `pilot-lab-groups.utf16le.xml` | 54,280 | `d6b15cfa27b93e7da75c1f59e7a265d178991e531c6aa7e9977fc7b5543c92b6` | `a07-groups-fixed3.xml` | `r07-groups.xml` (`20d147b548e716b876a013ec58593327a6b5b52ce20d68747c45d878dc98699f`) | the account groups |
| `pilot-lab-voucher-types.utf16le.xml` | 264,678 | `de940b6d6d1714cb3b6a8a2cf341b4c656318bfdd0de9dc81526ba6a30782844` | `a08-voucher-types-fixed3.xml` | `r08-voucher-types.xml` (`c351d6c62a09ce5173355f3998ef12746f01aba253ea8bdc42664b1b9766fb00`) | the voucher types, with the Manual series of BRIDGE Sales |
| `pilot-lab-number-new-invoice.utf16le.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` | `a09-number-new-invoice-fixed3.xml` | `r09-number-new-invoice.xml` (`aa6e7a3ae50bd13d6eff60ffac89bd713f084a49512667edcc6d40fee0ca3985`) | vouchers numbered BP/26-27/0010 before the import: none |
| `pilot-lab-number-in-use.utf16le.xml` | 5,612 | `e7a427b6392d3a7ada1d2427937400be8a574001d86ebaa24c713e3e2f317568` | `fixed2/a10-number-control-k0-fixed2.xml (commit 4bceb5d0)` | `r10-number-control-k0.xml` (`90d188e7089852c411516ef36617dfc3a157df76cfc1787b1985804833ff31e6`) | vouchers numbered BP/26-27/0001 in 2026-27: one, the book's first invoice (BRIDGE Sales, 2026-08-01, Sales class); read in the second round, whose marks are 15 and 223: between the two rounds one voucher was keyed (BP/26-27/0011, alteration id 16), and this voucher's alteration id is 2 in both rounds' voucher reads (`fixed2/vouchers-fixed2.xml`, `fixed3/vouchers-fixed3.xml`); its header counts (`CMPINFO`) are not zero, as the third round's are, and no parser reads them |
| `pilot-lab-company-state.utf16le.xml` | 3,418 | `0b4abb57fbfc888ef9d3d84d9b8847b9cd6f27f9c1de4aff36e36045b213286b` | `a11-company-state-fixed3.xml` | `r11-company-state.xml` (`40f91b3fdb131279b413910c60f86a3b56989d8646def9bbe6b1fbdc97b6a68b`) | the company's state: one row, BRIDGE PILOT LAB, Rajasthan |
| `pilot-lab-ledger-catalogue-v2.utf16le.xml` | 16,404 | `0ed443c9d8c9b120d8877df14cd68a8ea1b6c0ddd9fd09ef0aab0836542e5298` | `post/a13-ledger-catalogue-v2-pre.xml` | `r13-ledger-catalogue-v2.xml` (`28a47c26a29fa799856c112b71e19076946d4c0f85a6dcc65f5cdafe0e1e1bd0`) | the 11 ledgers with identities and bill-wise flags; the walk-in row is spelled BRIDGE Walk-in |
| `pilot-lab-marks-after-post.utf16le.xml` | 3,396 | `5836730f49e712ad3cd68d5a2cf1cb8981fe06b63b049f4489c882de5014b057` | `post/a02-marks-post.xml` | `r02-marks.xml` (`81892a744009b6cf462a6f8d2fd505e650f8ad022e1462b23414cf2b8b3d2d7c`) | the same read after the import: voucher mark 17, master mark 223 |
| `pilot-lab-invoice-readback-after-post.utf16le.xml` | 93,812 | `8bb2b0000841f2469f34c19bbb3f872dc63790af70062d469877e81609f0f0fa` | `post/a12-invoice-readback-post.xml` | `r12-invoice-readback.xml` (`dc03b35f075e638acf6ca0a1a8503311ee501c5ee6f739e31d4240c4d22bf5f7`) | the read-back of BP/26-27/0010 after the import: one voucher |
| `pilot-lab-invoice-readback-before-post.utf16le.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` | `post/a12-invoice-readback-pre.xml` | `r12-invoice-readback.xml` (`dc03b35f075e638acf6ca0a1a8503311ee501c5ee6f739e31d4240c4d22bf5f7`) | the read-back of `BP/26-27/0010` before the import: a complete envelope with a `COLLECTION` and no voucher |

`pilot-lab-number-new-invoice.utf16le.xml` and `pilot-lab-invoice-readback-before-post.utf16le.xml` are the same bytes: two requests whose answer was an empty collection of this company, which reads the same whatever the request.

Not in these files: the screenshots, the other reads of the first and second rounds (`fixed2` and before), the round's other answers, and the reads made after the screen work, which stay on the branch.

## The tax units, typed by hand (2026-10-09)

| Fixture | Bytes | SHA-256 | What it is | Request |
| --- | ---: | --- | --- | --- |
| `pilot-lab-tax-units-typed-request.utf8.xml` | 11,152 | `f66ed7c325edc01e6d5d627e6c3490b6f7c3edf573ddf670d936e0783cbb9e40` | the company's tax units, all fields, read after the registration was entered and before any voucher was keyed: the Default Tax Unit and one GST registration, `Rajasthan Registration`, with one dated row from 2026-04-01 (Regular, not inactive); UTF-8 with CRLF line ends, as received; Tally's `&#4;` references as received | typed by hand from the text in the request list of #1342 (`b-taxunit.xml`, 426 bytes, SHA-256 `e5bbb2ab3e16838d8cb3a39c48d0d5683d5a8d565e099783af0c240d58d30081`, sent as UTF-8): the same collection, type and `NATIVEMETHOD *` as `render_company_registration_request`, under another collection name and in UTF-8, where the code sends UTF-16. An answer to the code's own request is owed (it was asked for in the same thread). |
