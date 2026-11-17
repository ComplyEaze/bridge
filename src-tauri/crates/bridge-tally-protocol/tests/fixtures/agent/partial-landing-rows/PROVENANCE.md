# Rows R0, R1 and R2: captured partial landings

These files are the requests and answers around three posts of the same three Journals. The book was `BRIDGE STOCK CREATE LAB`, one synthetic company with accounts integrated with inventory. The release was TallyPrime 7.2, single-user edition. The rows ran on 12 Nov 2026, each row once. Each post went through `post_import` and was approved once at its box. A relaying proxy kept every request and answer whole.

The middle voucher of R0 held a ledger under Stock-in-Hand, which Tally refused. R1 sent that voucher first and R2 last. Each post landed in part. Each import answer reports `CREATED` 2 and `EXCEPTIONS` 1 and names no voucher.

## Source and transformation

The source is the lab answers branch `lab/1108-r-rows-answers` at `1300c41e`: files `proxy/import-1.request` to `import-3.request`, `proxy/x-171`, `x-174`, `x-176`, `x-190`, `x-407`, `x-410`, `x-412` and `x-426`. The R2 files `x-643`, `x-646`, `x-648` and `x-662` are byte-equal to R1's `x-407`, `x-410`, `x-412` and `x-426`, so one file stands for both rows.

The requests are BOM-prefixed UTF-16LE. The answers are BOM-less UTF-16LE.

The pushed lab copies replace the company GUID by a placeholder. The GUID was put back, as UTF-16LE, in every place the placeholder stands (1 place in each marks answer, 6 in each verification-after answer, none in the other files), and the HTTP head was removed from every file, requests and answers alike. No other byte differs from the capture. For the six answers that held a placeholder the lab recorded the original size and sha256, and the restored bytes with their HTTP head reproduce them. The other seven files are the pushed copies, which the lab's hash list covers, less their HTTP head.

| file | bytes | sha256 | lab file |
| --- | ---: | --- | --- |
| `partial-landing-r0-import.request.utf16le.xml` | 4150 | `32f4c7123f38717c8448bde4aff9ddcf6db5d31d938a78f7da3e5fe6776f130d` | `proxy/import-1.request` |
| `partial-landing-r1-import.request.utf16le.xml` | 4150 | `d62213bbdc46ffa0da4f0f84e62e550727c6e149a5e4ddcb7906bb37f9174aab` | `proxy/import-2.request` |
| `partial-landing-r2-import.request.utf16le.xml` | 4150 | `256c51bf658ef77c067392114e59bc9965c8b95bc9771cba320f3cfa5502da55` | `proxy/import-3.request` |
| `partial-landing-company-marks.request.utf16le.xml` | 990 | `5802b3feca930e569de996e5582299ad34f2515140b233a7140290f8f874552b` | `proxy/x-174.request` |
| `partial-landing-import-verification.request.utf16le.xml` | 1836 | `cb1aec85a3af229365533fe47fe76db17497e5a7b2d775ee6594e78c0ff88b02` | `proxy/x-171.request` |
| `partial-landing-r0-marks-before.utf16le.xml` | 3406 | `19929a5e192943063e0793ce7e12c05fa9a44137ee4ffce7d82b291b4ab028f8` | `proxy/x-174.answer` |
| `partial-landing-r0-marks-after.utf16le.xml` | 3408 | `18eb10949d94db888f72e411eaf38671281a52c71eb64e18d06787d2f6c25251` | `proxy/x-176.answer` |
| `partial-landing-r0-verification-before.utf16le.xml` | 3022 | `9fca7a6deee0d5d5be97fe52df71f8bd8ccc7725398837ada16d6cdfef2291d1` | `proxy/x-171.answer` |
| `partial-landing-r0-verification-after.utf16le.xml` | 12860 | `04b53681f7dda3e93e7559b8348499aeabf4157fb0a1344587c0c733214f832a` | `proxy/x-190.answer` |
| `partial-landing-r1-r2-marks-before.utf16le.xml` | 3406 | `1f9575cae1075f16478db3343297d5b1fffa03bb18cca977c8e0661c72c07cac` | `proxy/x-410.answer` |
| `partial-landing-r1-r2-marks-after.utf16le.xml` | 3406 | `2aafd95a36011f22f7a34e53765c3645e15f0b140c4a46ea33e1a49a0e835b35` | `proxy/x-412.answer` |
| `partial-landing-r1-r2-verification-before.utf16le.xml` | 3022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` | `proxy/x-407.answer` |
| `partial-landing-r1-r2-verification-after.utf16le.xml` | 12858 | `2aa3190ab0b5da72b2f8f4fe3e6cb958eef4674435e53dc984a92b5c213c7b49` | `proxy/x-426.answer` |

## What each file is

- `partial-landing-r0-import.request.utf16le.xml`: the import request as Bridge sent it, with the three Journals in this order: R0 first, stock leg, third. It names the company by name; it carries no GUID.
- `partial-landing-r1-import.request.utf16le.xml`: the import request as Bridge sent it, with the three Journals in this order: R1 stock leg, first, third. It names the company by name; it carries no GUID.
- `partial-landing-r2-import.request.utf16le.xml`: the import request as Bridge sent it, with the three Journals in this order: R2 first, third, stock leg. It names the company by name; it carries no GUID.
- `partial-landing-company-marks.request.utf16le.xml`: the company high-water read (collection `Bridge Agent Company High Water`).
- `partial-landing-import-verification.request.utf16le.xml`: the verification read of the batch's date (collection `Bridge Agent Import Verification`, 14 April 2025 to the same day).
- `partial-landing-r0-marks-before.utf16le.xml` / `partial-landing-r0-marks-after.utf16le.xml`, `partial-landing-r1-r2-marks-before.utf16le.xml` / `partial-landing-r1-r2-marks-after.utf16le.xml`: the company-marks answers read just before and just after the import.
- `partial-landing-r0-verification-before.utf16le.xml` / `partial-landing-r0-verification-after.utf16le.xml`, `partial-landing-r1-r2-verification-before.utf16le.xml` / `partial-landing-r1-r2-verification-after.utf16le.xml`: the verification answers read before the import (no voucher on that date) and after it (the two created vouchers only).

## What they establish

Partial: one run per row, one book.

- The voucher mark (`ALTVCHID`) steps from 1 to 3. The master mark (`ALTMSTID`) stays 213.
- The verification read after the post holds exactly two rows: voucher numbers 2 and 3, narrations "Row 1108-P2 first" and "Row 1108-P2 third". It holds none for the refused voucher, whichever position it was sent in.
- The three import answers are the committed `import_post_partial_landing_p2_live` bytes, already kept as `import_post_partial_landing_p2_live.utf16le.xml`; they are not added again.

## What they do not establish

- That this book held no other voucher on that date.
- Another release, another fault kind, a book not integrated with inventory, or a multi-user edition.
- Two refusals in one post, or a refusal of one of two identical vouchers.
- What the `<VOUCHER>` count in `CMPINFO` counts. It reads 1 then 3 in R0 and 0 then 2 in R1 and R2. The lab's report does not explain it; the lab's own voucher reads show the same three vouchers in every row.
