# Transfer days of one book: BRIDGE TRANSFER LAB, its catalogue and 12 to 29 April 2026 (#1392)

These files are requests and answers of a read-only lab run on 2 November 2026 on the synthetic book BRIDGE TRANSFER LAB (TallyPrime 7.2, Silver, Education mode off), the book of `whole-day-read-transfer-lab`, loaded alone. The requests were rendered from the code (the catalogue by its builder; each day from that folder's 3 April census and vouchers requests with the date changed and nothing else) and published on the branch `lab/1392-transfer-days-sheet` (commit `ddac9f69`). The answers, with their headers and `log.txt`, were published on `lab/1392-transfer-days-answers` (commit `5fb43569`) with `RESULTS.md`, `evidence-report.txt` and a SHA-256 list. Each file here is byte-equal to the file there, and its SHA-256 equals the one in that list; the 8-character prefixes below are the start of those.

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

- Marks: `ALTVCHID` 7 before the keying (`m1`, `bb303e14`, not committed) and 12 after it; `m2` (after the keying) and `m3` (after every read, `63ff95d5`, not committed) are byte-equal, so no read moved a mark.
- Catalogue: `cat1` (before the keying) names Cash, Profit & Loss A/c, TLab Bank A and TLab Bank B; `cat2` (after every read) names those and TLab Clearing. Every ledger name on a voucher row equals a `cat2` name exactly, by code point.
- Days: 13, 20, 21, 27 and 28 April each hold the one voucher keyed on them, and their census holds the same voucher (GUID, alteration id, date). 12, 14, 19, 22, 26 and 29 April answer with no voucher; their census and vouchers answers are one and the same envelope (`b072f28f`).
- Counting: every answer also holds a header count node, `<VOUCHER>0</VOUCHER>`, with no attribute and no child, and the marks and catalogue answers hold `<COMPANY>0</COMPANY>` or `<LEDGER>0</LEDGER>` the same way. A count of nodes named `VOUCHER` reads one too many unless it sets that node aside; the code counts the rows it parses.
- The vouchers and catalogue answers carry Tally's `&#4;` character references, as received.

## What they do not show

One book, one release (7.2), one sitting. The catalogue's spelling equalled the voucher rows' spelling on this small book; another book may differ, and the code compares names exactly so that a difference keeps a row unchecked. Nothing here shows a day Tally withheld or refused, a day of many vouchers, a census in more than one AlterID span, Gold, or another release. 22 April was read (`b072f28f`) and is not committed, as no test reads it; the run's `log.txt` (`e3977650`) is not committed either.

## Files

| File | Bytes | SHA-256 (first 8) |
| --- | ---: | --- |
| `cat.request.utf16le.xml` | 2112 | `119f80c0` |
| `cat1.utf16le.xml` | 7840 | `fdc7c83e` |
| `cat2.utf16le.xml` | 9048 | `a64e215c` |
| `d12-c.request.utf16le.xml` | 1442 | `997f0bf0` |
| `d12-c.utf16le.xml` | 3026 | `b072f28f` |
| `d12-v.request.utf16le.xml` | 1746 | `d1837438` |
| `d12-v.utf16le.xml` | 3026 | `b072f28f` |
| `d13-c.request.utf16le.xml` | 1442 | `d15d482a` |
| `d13-c.utf16le.xml` | 5436 | `7d01c9c9` |
| `d13-v.request.utf16le.xml` | 1746 | `a35da164` |
| `d13-v.utf16le.xml` | 38972 | `77e8ab25` |
| `d14-c.request.utf16le.xml` | 1442 | `687cc6d9` |
| `d14-c.utf16le.xml` | 3026 | `b072f28f` |
| `d14-v.request.utf16le.xml` | 1746 | `ad95846d` |
| `d14-v.utf16le.xml` | 3026 | `b072f28f` |
| `d19-c.request.utf16le.xml` | 1442 | `2fe06877` |
| `d19-c.utf16le.xml` | 3026 | `b072f28f` |
| `d19-v.request.utf16le.xml` | 1746 | `105459be` |
| `d19-v.utf16le.xml` | 3026 | `b072f28f` |
| `d20-c.request.utf16le.xml` | 1442 | `4804c672` |
| `d20-c.utf16le.xml` | 5436 | `22d8ce8b` |
| `d20-v.request.utf16le.xml` | 1746 | `002a99ad` |
| `d20-v.utf16le.xml` | 38972 | `c09457d7` |
| `d21-c.request.utf16le.xml` | 1442 | `e4703c9b` |
| `d21-c.utf16le.xml` | 5438 | `54f6975f` |
| `d21-v.request.utf16le.xml` | 1746 | `84a7fe59` |
| `d21-v.utf16le.xml` | 44674 | `61c9eed7` |
| `d26-c.request.utf16le.xml` | 1442 | `78baa7f6` |
| `d26-c.utf16le.xml` | 3026 | `b072f28f` |
| `d26-v.request.utf16le.xml` | 1746 | `eb53fcf7` |
| `d26-v.utf16le.xml` | 3026 | `b072f28f` |
| `d27-c.request.utf16le.xml` | 1442 | `fa27e79e` |
| `d27-c.utf16le.xml` | 5440 | `bd0fbe46` |
| `d27-v.request.utf16le.xml` | 1746 | `4dc933c0` |
| `d27-v.utf16le.xml` | 39048 | `c0e2f796` |
| `d28-c.request.utf16le.xml` | 1442 | `c372707b` |
| `d28-c.utf16le.xml` | 5442 | `1a58d520` |
| `d28-v.request.utf16le.xml` | 1746 | `9c9f2380` |
| `d28-v.utf16le.xml` | 39058 | `26754f63` |
| `d29-c.request.utf16le.xml` | 1442 | `ecd9d4e8` |
| `d29-c.utf16le.xml` | 3026 | `b072f28f` |
| `d29-v.request.utf16le.xml` | 1746 | `8c3d5213` |
| `d29-v.utf16le.xml` | 3026 | `b072f28f` |
| `m2.utf16le.xml` | 3404 | `63ff95d5` |
