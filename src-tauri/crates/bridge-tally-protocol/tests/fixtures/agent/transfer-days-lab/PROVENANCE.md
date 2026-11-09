# Transfer days of one book: BRIDGE TRANSFER LAB, its catalogue and 12 to 29 April 2026 (#1392)

These files are requests and answers of a read-only lab run on 2 November 2026 on the synthetic book BRIDGE TRANSFER LAB (TallyPrime 7.2, Silver, Education mode off), the book of `whole-day-read-transfer-lab`, loaded alone. The requests were rendered from the code (the catalogue by its builder; each day from that folder's 3 April census and vouchers requests with the date changed and nothing else) and published on the branch `lab/1392-transfer-days-sheet` (commit `ddac9f69`). The answers, with their headers and `log.txt`, were published on `lab/1392-transfer-days-answers` (commit `5fb43569`) with `RESULTS.md`, `evidence-report.txt` and a SHA-256 list. Each file here is byte-equal to the file there, and its SHA-256 below equals the one in that list.

The run's sheet names each request file `<name>.utf16le.xml` and each answer `evidence/<name>.response`. Here a request is `<name>.request.utf16le.xml` and an answer `<name>.utf16le.xml`. The marks request is not repeated: it is byte-equal to `whole-day-read-transfer-lab/m.request.utf16le.xml`.

## What was keyed

Before the reads, one ledger and five vouchers were keyed by hand on Tally's screen, double entry, no narration and no reference: `TLab Clearing` under `Suspense A/c`, and

| Date | Voucher | Entries |
| --- | --- | --- |
| 13 April | Contra 6 | Cash and TLab Bank A, 7000.00 |
| 20 April | Contra 7 | Cash and TLab Bank A, 4000.00 |
| 21 April | Contra 8 | TLab Bank B and TLab Bank A, 25000.00 |
| 27 April | Payment 1 | TLab Clearing and TLab Bank A, 30000.00 |
| 28 April | Receipt 1 | TLab Bank B and TLab Clearing, 30000.00 |

These are keyed rows, not imported ones.

## What the answers show

- Marks: `ALTVCHID` 7 before the keying (`m1`, not committed) and 12 after it; `m2` (after the keying) and `m3` (after every read, not committed) are byte-equal, so no read moved a mark.
- Catalogue: `cat1` (before the keying) names Cash, Profit & Loss A/c, TLab Bank A and TLab Bank B; `cat2` (after every read) names those and TLab Clearing. Every ledger name on a voucher row equals a `cat2` name exactly, by code point.
- Days: 13, 20, 21, 27 and 28 April each hold the one voucher keyed on them, and their census holds the same voucher (GUID, alteration id, date). 12, 14, 19, 22, 26 and 29 April answer with no voucher; their census and vouchers answers are one and the same envelope.
- Counting: every answer also holds a header count node, `<VOUCHER>0</VOUCHER>`, with no attribute and no child, and the marks and catalogue answers hold `<COMPANY>0</COMPANY>` or `<LEDGER>0</LEDGER>` the same way. A count of nodes named `VOUCHER` reads one too many unless it sets that node aside; the code counts the rows it parses.
- The vouchers and catalogue answers carry Tally's `&#4;` character references, as received.

## What they do not show

One book, one release (7.2), one sitting. The catalogue's spelling equalled the voucher rows' spelling on this small book; another book may differ, and the code compares names exactly so that a difference keeps a row unchecked. Nothing here shows a day Tally withheld or refused, a day of many vouchers, a census in more than one AlterID span, Gold, or another release. 22 April was read and is not committed, as no test reads it; the run's `log.txt` is not committed either.

## Files

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `cat.request.utf16le.xml` | 2,112 | `119f80c0314cb876eb81f7dbed081b3df3ed68767e9af45fa259102437b118fc` |
| `cat1.utf16le.xml` | 7,840 | `fdc7c83e372956ea9ee7b49a496e24e397de006c9efd6a83b2c351165291f644` |
| `cat2.utf16le.xml` | 9,048 | `a64e215c34c3524bb7f002b208692e5325f1bf97df5baddf9e8e1757e15e25b6` |
| `d12-c.request.utf16le.xml` | 1,442 | `997f0bf0eab2e6419ee82d18c755dda61a6af7be2f3f6ef39f6968941073ab66` |
| `d12-c.utf16le.xml` | 3,026 | `b072f28fa0b711663cb0e24fe02a44043fa7dc31678d43a1efd38551bf64f351` |
| `d12-v.request.utf16le.xml` | 1,746 | `d18374387f9beba58c1f152f2bf0b5544c1c672f8193b646f1f1554ffb37529c` |
| `d12-v.utf16le.xml` | 3,026 | `b072f28fa0b711663cb0e24fe02a44043fa7dc31678d43a1efd38551bf64f351` |
| `d13-c.request.utf16le.xml` | 1,442 | `d15d482a0adeaf351867814b6998fef12d6c952afb79d3506784a9d7a7acd599` |
| `d13-c.utf16le.xml` | 5,436 | `7d01c9c9c0fb897776d61271d7599e8d620f64009ed186fb6c45036ae38fb2e4` |
| `d13-v.request.utf16le.xml` | 1,746 | `a35da16465294dac62a3e660137c2645ab5bd926f3540cf21ec6d003a9a1257c` |
| `d13-v.utf16le.xml` | 38,972 | `77e8ab25a9e64f4e8a30e9b0bfd6c5841fd1c1cc9d059761076c6e33e0ecf509` |
| `d14-c.request.utf16le.xml` | 1,442 | `687cc6d94436d411b34d7e16a113324f2fa6bc33910fa9382ef89215ef862d47` |
| `d14-c.utf16le.xml` | 3,026 | `b072f28fa0b711663cb0e24fe02a44043fa7dc31678d43a1efd38551bf64f351` |
| `d14-v.request.utf16le.xml` | 1,746 | `ad95846de1c44862d7f56085253a8b2f43127ba9652b252d563886a1018c7aa0` |
| `d14-v.utf16le.xml` | 3,026 | `b072f28fa0b711663cb0e24fe02a44043fa7dc31678d43a1efd38551bf64f351` |
| `d19-c.request.utf16le.xml` | 1,442 | `2fe068771b7452976367b80638b9f30fa7caf6adc1465bcccbe8487b58119226` |
| `d19-c.utf16le.xml` | 3,026 | `b072f28fa0b711663cb0e24fe02a44043fa7dc31678d43a1efd38551bf64f351` |
| `d19-v.request.utf16le.xml` | 1,746 | `105459be08e733bbcb56f44625d51d33772f734d2be72ddd35bef85941804798` |
| `d19-v.utf16le.xml` | 3,026 | `b072f28fa0b711663cb0e24fe02a44043fa7dc31678d43a1efd38551bf64f351` |
| `d20-c.request.utf16le.xml` | 1,442 | `4804c672cc04e27b237539d814622a27d025049be3fd73ed925c62b1260b9ed4` |
| `d20-c.utf16le.xml` | 5,436 | `22d8ce8b2719925b760b15b2f4a6c99d37d330cec87e48a03b4e6672f8f77b0f` |
| `d20-v.request.utf16le.xml` | 1,746 | `002a99ade98e8a2828945a1c3521c0b51b26cdd65c6ea10d8f9866f441acf7e8` |
| `d20-v.utf16le.xml` | 38,972 | `c09457d783e24ecc2350760c6b0c3cc3216451a4740deaf44d8b5be324dbc896` |
| `d21-c.request.utf16le.xml` | 1,442 | `e4703c9b7398caef90cb546058d3bb345229f8e331fea606115d02352a6a224b` |
| `d21-c.utf16le.xml` | 5,438 | `54f6975fc0b4bb68d68164b11c78f0e06368b6020c90714d298687cb74c9fcf5` |
| `d21-v.request.utf16le.xml` | 1,746 | `84a7fe5933e9fa2c085d50e3c46329a770fe33851bd49e2942fdb64ba3785fc4` |
| `d21-v.utf16le.xml` | 44,674 | `61c9eed7bb078ff1c50a388af4a6715164898c6bf620557d7bee4a3b6325fd59` |
| `d26-c.request.utf16le.xml` | 1,442 | `78baa7f645f48dbf6a8cf8b22cc22b2ef129f1592f88865b0ab1a4e089ff5041` |
| `d26-c.utf16le.xml` | 3,026 | `b072f28fa0b711663cb0e24fe02a44043fa7dc31678d43a1efd38551bf64f351` |
| `d26-v.request.utf16le.xml` | 1,746 | `eb53fcf7a537dc8c3e45cc1f7913c7d9b54909912e4ce4e35c1e575983089ef5` |
| `d26-v.utf16le.xml` | 3,026 | `b072f28fa0b711663cb0e24fe02a44043fa7dc31678d43a1efd38551bf64f351` |
| `d27-c.request.utf16le.xml` | 1,442 | `fa27e79ec16606ace41ef68c7870ac96c3c55a602f47790fb8a6efa80f0c7837` |
| `d27-c.utf16le.xml` | 5,440 | `bd0fbe460173d5c450247326804c5a73098cf47fb771dce46a6324027c50829f` |
| `d27-v.request.utf16le.xml` | 1,746 | `4dc933c0eb9290d2bd4509f0ac5727db53260369f6b0de453543d6f3c79fe498` |
| `d27-v.utf16le.xml` | 39,048 | `c0e2f79688ce370e8bfdd3c1d4630e8a945ddefcfb66c927ff686aaafe615fb5` |
| `d28-c.request.utf16le.xml` | 1,442 | `c372707bf6086a27df95905e03b74fef7edcbf2fcb250f2f71eb2a5a846bd8e8` |
| `d28-c.utf16le.xml` | 5,442 | `1a58d520cf93c91f1da9e4288736104029d0f50188738e64a58153d34193b4e2` |
| `d28-v.request.utf16le.xml` | 1,746 | `9c9f23809f0a0df708efbb859b719faad3914ad1093c00245ab9b91b97a930b9` |
| `d28-v.utf16le.xml` | 39,058 | `26754f6310ecdd13fcde3c45775e0d61859d3bfc334c0cc3aabecac56109d02e` |
| `d29-c.request.utf16le.xml` | 1,442 | `ecd9d4e859f9e7f671ffbc5d91396178639fc36152d005b9c8c3d94b3946b22b` |
| `d29-c.utf16le.xml` | 3,026 | `b072f28fa0b711663cb0e24fe02a44043fa7dc31678d43a1efd38551bf64f351` |
| `d29-v.request.utf16le.xml` | 1,746 | `8c3d52138d07dfa7d8a7f54852e89bd4ea5eb8e4d0b434cdde83cd3411ff07a0` |
| `d29-v.utf16le.xml` | 3,026 | `b072f28fa0b711663cb0e24fe02a44043fa7dc31678d43a1efd38551bf64f351` |
| `m2.utf16le.xml` | 3,404 | `63ff95d5235692fabd9e21844acf5c0a0fd10355bbf0b419aec74b8836a75136` |

## Spanned census, 3 November 2026

Since #2199 the code sends every census spanned to the voucher mark it read. The ten requests below are that census for each day above, as the code on merge `b2cd7e32` renders it: each is its day's `dNN-c.request.utf16le.xml` with `$AlterID > 0 AND $AlterID <= 12` added to its filter and nothing else changed (12 is the voucher mark in `m2`). A read-only lab row sent them on 3 November 2026 to the same book, loaded alone, with nothing keyed (TallyPrime 7.2, Silver). Its answers, with their headers, `log.txt` and `RESULTS.md`, were published on `lab/1392-spanned-census-answers` (commit `a8714009`), with the SHA-256 of each request it sent, which equals the one below. Each answer here is byte-equal to the file there. The marks were read before and after the ten sends and were byte-equal, at `ALTVCHID` 12 and `ALTMSTID` 211; they are not committed, as the replays serve `m2`, whose voucher mark is the same 12.

Each answer holds the same vouchers as its day's `dNN-c.utf16le.xml` (GUID, alteration id and date), counted by structure. No answer is byte-equal to it: four header counts (`LEDGER`, `CURRENCY`, `VOUCHERNUMBERSERIES` and the bare `VOUCHER` count) read 0 on 3 November, and they are not rows. The five empty days' answers are one and the same envelope.

| File | Bytes | SHA-256 | Source |
| --- | ---: | --- | --- |
| `d12-c2.request.utf16le.xml` | 1,526 | `c60a9f1d0bfe8820c5b940c75e0856e375d8d19cec594dedc5ddbd016fa29811` | the code's own rendering, with a byte order mark, by `render_voucher_census` (`agent_read_profiles.rs:249`) on merge `b2cd7e32`, the span `(0, 12]` taken from the captured `m2`; its answer is `d12-c2.utf16le.xml` |
| `d12-c2.utf16le.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` | `evidence/d12-c2.response` on `lab/1392-spanned-census-answers` (commit `a8714009`), the answer to `d12-c2.request.utf16le.xml` |
| `d13-c2.request.utf16le.xml` | 1,526 | `548c9f004054305257c6788d9c9bb4d7eb7a51e34d1b3218b82d42f5dabc18dd` | the code's own rendering, with a byte order mark, by `render_voucher_census` (`agent_read_profiles.rs:249`) on merge `b2cd7e32`, the span `(0, 12]` taken from the captured `m2`; its answer is `d13-c2.utf16le.xml` |
| `d13-c2.utf16le.xml` | 5,432 | `084d21a00b19d82b5b9717b41cf4ee56a7820ad23a9984722b491f68551b431a` | `evidence/d13-c2.response` on `lab/1392-spanned-census-answers` (commit `a8714009`), the answer to `d13-c2.request.utf16le.xml` |
| `d14-c2.request.utf16le.xml` | 1,526 | `6472c003ab25e12009350f80278bee4cf3c3d84510e4848fd2939067a10949d4` | the code's own rendering, with a byte order mark, by `render_voucher_census` (`agent_read_profiles.rs:249`) on merge `b2cd7e32`, the span `(0, 12]` taken from the captured `m2`; its answer is `d14-c2.utf16le.xml` |
| `d14-c2.utf16le.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` | `evidence/d14-c2.response` on `lab/1392-spanned-census-answers` (commit `a8714009`), the answer to `d14-c2.request.utf16le.xml` |
| `d19-c2.request.utf16le.xml` | 1,526 | `6a555f985ab65bd7f11f89073f329fb0a320c35be0959f9aa0f8dd608627633d` | the code's own rendering, with a byte order mark, by `render_voucher_census` (`agent_read_profiles.rs:249`) on merge `b2cd7e32`, the span `(0, 12]` taken from the captured `m2`; its answer is `d19-c2.utf16le.xml` |
| `d19-c2.utf16le.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` | `evidence/d19-c2.response` on `lab/1392-spanned-census-answers` (commit `a8714009`), the answer to `d19-c2.request.utf16le.xml` |
| `d20-c2.request.utf16le.xml` | 1,526 | `2883bb5e74ac4e0ec234d0a58ccfc09c3e30393fcb74276e292e2a1359d29845` | the code's own rendering, with a byte order mark, by `render_voucher_census` (`agent_read_profiles.rs:249`) on merge `b2cd7e32`, the span `(0, 12]` taken from the captured `m2`; its answer is `d20-c2.utf16le.xml` |
| `d20-c2.utf16le.xml` | 5,432 | `a556f23d90095f959444e9d2df5e29b32054eb1a7e098b92c286203352c4465b` | `evidence/d20-c2.response` on `lab/1392-spanned-census-answers` (commit `a8714009`), the answer to `d20-c2.request.utf16le.xml` |
| `d21-c2.request.utf16le.xml` | 1,526 | `675f447e20e609e4162d43059bc15da94d4e55cf476e4dfb64fdbed13c22eec7` | the code's own rendering, with a byte order mark, by `render_voucher_census` (`agent_read_profiles.rs:249`) on merge `b2cd7e32`, the span `(0, 12]` taken from the captured `m2`; its answer is `d21-c2.utf16le.xml` |
| `d21-c2.utf16le.xml` | 5,434 | `68de428ebfa199e402b0d7ab2ef9645ee6c6c056705e17e6dc6ad266255c66e2` | `evidence/d21-c2.response` on `lab/1392-spanned-census-answers` (commit `a8714009`), the answer to `d21-c2.request.utf16le.xml` |
| `d26-c2.request.utf16le.xml` | 1,526 | `e8da4bcb139a857486776317625874f189067d2e6c4d1e273e5e88dc1d623a8d` | the code's own rendering, with a byte order mark, by `render_voucher_census` (`agent_read_profiles.rs:249`) on merge `b2cd7e32`, the span `(0, 12]` taken from the captured `m2`; its answer is `d26-c2.utf16le.xml` |
| `d26-c2.utf16le.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` | `evidence/d26-c2.response` on `lab/1392-spanned-census-answers` (commit `a8714009`), the answer to `d26-c2.request.utf16le.xml` |
| `d27-c2.request.utf16le.xml` | 1,526 | `1b5146b16c9a51d057dcfa9ea22a5ea5536f34e6a148992d67e2e9e82a8c49fb` | the code's own rendering, with a byte order mark, by `render_voucher_census` (`agent_read_profiles.rs:249`) on merge `b2cd7e32`, the span `(0, 12]` taken from the captured `m2`; its answer is `d27-c2.utf16le.xml` |
| `d27-c2.utf16le.xml` | 5,436 | `c73299e524679e21c329ae35f3bc87462167f940537ef335fdd029f5fab117f8` | `evidence/d27-c2.response` on `lab/1392-spanned-census-answers` (commit `a8714009`), the answer to `d27-c2.request.utf16le.xml` |
| `d28-c2.request.utf16le.xml` | 1,526 | `cf2cc4c5c1d89e41de69128552761278e7ca30f0c0f9660b1c7696f6eddcc12d` | the code's own rendering, with a byte order mark, by `render_voucher_census` (`agent_read_profiles.rs:249`) on merge `b2cd7e32`, the span `(0, 12]` taken from the captured `m2`; its answer is `d28-c2.utf16le.xml` |
| `d28-c2.utf16le.xml` | 5,438 | `2a9f0aa0a65ffb486a96ebfdf9fd8d46375044f8388306bd61c920736cf09ffe` | `evidence/d28-c2.response` on `lab/1392-spanned-census-answers` (commit `a8714009`), the answer to `d28-c2.request.utf16le.xml` |
| `d29-c2.request.utf16le.xml` | 1,526 | `ad68e914574c3e6a05cd3b55328097cbd718b7a04e1edbf4cd61e2cc273fac3c` | the code's own rendering, with a byte order mark, by `render_voucher_census` (`agent_read_profiles.rs:249`) on merge `b2cd7e32`, the span `(0, 12]` taken from the captured `m2`; its answer is `d29-c2.utf16le.xml` |
| `d29-c2.utf16le.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` | `evidence/d29-c2.response` on `lab/1392-spanned-census-answers` (commit `a8714009`), the answer to `d29-c2.request.utf16le.xml` |
