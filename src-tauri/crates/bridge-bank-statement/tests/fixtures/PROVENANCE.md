# Bank-statement fixture provenance

No client data. Every fixture here is synthetic.

## Generated PDFs

Written by `scripts/generate-bank-statement-fixtures.py`, deterministically;
CI runs it with `--check`, so these bytes are that script's output and nothing
else. Every name, account digit, reference and amount is invented. What is
carried over from real statements is only the template geometry the SBI and
HDFC profiles were calibrated on, which is already public in
`scripts/bank_statement_import.py`, and, for ICICI, column positions and
shapes measured on two real statements (no value from either).

| Fixture | Bytes | SHA-256 |
| --- | ---: | --- |
| `hdfc-synthetic.pdf` | 4,810 | `3eb43c5376ca6588e492747ab64d1494c7f59a380f9ac3ad0d41a073adacfd0f` |
| `sbi-owner-password-only.pdf` | 3,545 | `64bb53bf392ec9cc52aede9085390b29b7cac0ba7908d678ba933551414d14c0` |
| `hdfc-rotated.pdf` | 2,921 | `655d670aa5513a6386d723611c61db19555196ea034e85e611846a8f6f904d21` |
| `ubi-synthetic.pdf` | 3,413 | `6694b20b393c09b53cc7359226c69998adf90d9e731e1413fbfd310733d9098b` |
| `icici-synthetic.pdf` | 5,632 | `d10c64f1a65ca7e4794ea368611334992a624bb8cc94db00df556d4a2373d097` |
| `scan-image-only.pdf` | 1,481 | `ac3f526b32c6e7883e7cd665503f88af96b4c4117e07400ff0c4b5c66382bb6a` |
| `print-to-pdf-vector-glyphs.pdf` | 21,643 | `08a3722a9e27975eb8547e0d11973f760fab7a2993554ccf8392923f7e54ade9` |
| `scan-with-visible-stamp.pdf` | 1,256 | `ae4bc131732ad8808db287097ddaf363bbc1add12ba510e51f2413a66cbf5008` |
| `hdfc-logo-and-watermark.pdf` | 3,260 | `d82ea459eb763263af46482c5adea1250e08866ba63778d08680e59f054332c3` |

- `hdfc-synthetic.pdf` — three pages, user password `synthetic-user-4321`. A
  12-digit UPI reference and an ACH reference broken mid-token at the 240pt
  narration edge, a wrap that ends at a real space, a transaction naming the
  bank that must not be read as the footer, a row printed below the footer,
  `STATEMENT SUMMARY`, and a third page that must not be read. The phone line
  also ends in the account digits; only the account-number line binds.
- `sbi-owner-password-only.pdf` — two pages. Its user password is one the
  operator never has; only the owner password `synthetic-owner-7788` is
  supplied. `pdftotext -upw synthetic-owner-7788` rejects it ("Incorrect
  password") and `-opw` opens it, measured with poppler 26.04.0 on 2026-09-16:
  the case the reference tried both flags for. The date is stacked over the
  year, the column header repeats on page 2 while page 1's last row is still
  open, and a 12-digit reference wraps mid-token.
- `hdfc-rotated.pdf` — one page with `/Rotate 90`, refused before parsing.
- `ubi-synthetic.pdf` — two pages, user password `synthetic-user-7788`, in the
  Union Bank of India row shape: one line per row, the amount and balance glued
  to `(Cr)`/`(Dr)`, an overdrawn `(Dr)` balance, a repeated column header, a
  `Page N of M` footer on every page, and no printed totals. The header block
  also prints a masked account line and a CIF ID; only `Account Number` binds.
  The row *shapes* were described from one real statement without its values;
  the x positions are invented, since the profile reads rows as text.
- `icici-synthetic.pdf` — two pages, empty user password and an owner password nobody is given, as a
  downloaded ICICI statement opens: 14 rows, newest first, in the ICICI Bank layout. It stands for what
  two real statements (a current account and a cash credit account) showed, measured by a layout probe
  through PDFium: the transaction date at x 15-65 and the value date at 87-137, narrations from x 149.5
  wrapping at 359.5, withdrawals ending at 543, deposits at 662 and the balance (a glued `Cr`/`Dr`) at 787,
  the column header on page 1 only, a masked account number (`123XXXXXXXX456`) printed three times above
  the table, amounts on their own line 6 and 11 points below the date (0, about 6 and about 11.5 on the real files), a `<date> <time> <word> Page N of M`
  footer and a disclaimer line on every page, and no opening or closing balance or totals. Every word,
  amount, account digit and reference is invented, including each narration's wording; the shapes are
  the observed ones (a UPI row whose VPA is broken at the wrap edge and one cut before its bank word, a
  NEFT name holding a hyphen, an RTGS name wrapped at a space, an IMPS row, charges, a loan recovery, a
  cash deposit, a cheque number in the cheque column). **Limits:** Courier is wider than the bank's font, so
  two narrations run past the 50 characters the bank allows, to exercise the wrap rule; nothing here was
  captured from a real file; the balance crosses from `Cr` to `Dr` on one row, which the cash credit account
  did not show (its balances were all `Dr`, the current account's all `Cr`); the real files are the owner's
  and are not in this repository.
- `scan-image-only.pdf` — two pages, user password `synthetic-user-4321`; each
  page is one 8x8 grey image scaled to the page and no text object. `pdftotext -bbox`
  reports no words (measured 2026-10-07).
- `print-to-pdf-vector-glyphs.pdf` — one page carrying the HDFC page 1 lines,
  every character drawn as a filled rectangle, as a printer driver converts
  glyphs to outlines; no text object. `pdftotext -bbox` reports no words (measured
  2026-10-07). The rectangles are not letter shapes: the fixture stands for
  "no extractable text", not for a legible scan.
- `scan-with-visible-stamp.pdf` — the same image under one line of real text,
  "SCANNED WITH A PHONE APP", which has no digit: the shape of a scanner app's
  stamp on a scan. `pdftotext -bbox` reports 5 words (measured 2026-10-07).
- `hdfc-logo-and-watermark.pdf` — the same lines as real text, with a 40pt
  image as a logo and a light rotated rectangle as a watermark.
  `pdftotext -bbox` reports 72 words, as for page 1 of `hdfc-synthetic.pdf`
  (measured 2026-10-07).

**Limits.** None of these four was made by a scanner or a printer driver: they
are shaped like what those tools write, not captured from them.

Encryption is RC4 128-bit (standard security handler revision 3);
real statements commonly use AES, which is not exercised here. The text is the
non-embedded base-14 Courier font; embedded and proportional fonts are not.

## Poppler captures of the generated PDFs

`pdftotext -bbox-layout` output of the two PDFs above, byte-exact, from poppler
26.04.0 (Homebrew) on macOS arm64, 2026-09-16:

```
pdftotext -bbox-layout -upw synthetic-user-4321 hdfc-synthetic.pdf hdfc-synthetic.pdftotext.xml
pdftotext -bbox-layout -opw synthetic-owner-7788 sbi-owner-password-only.pdf sbi-owner-password-only.pdftotext.xml
```

| Fixture | Bytes | SHA-256 |
| --- | ---: | --- |
| `hdfc-synthetic.pdftotext.xml` | 31,469 | `dc31411303e974361de3895258a7f1e80f26914331906d49b6c87aa6ddc2c448` |
| `sbi-owner-password-only.pdftotext.xml` | 23,420 | `cfe5a7144facde8d3bd5e4ff82b9f2ca634bbc1815543ee76b84c20b13aa2965` |

They are the reference the PDFium path is compared against: `pdf_tests.rs`
requires PDFium's words to produce exactly the rows these produce, and pins the
digests the Python reference (`scripts/bank_statement_import.py`) computes over
them. They were captured from the first generation of the PDFs; adding
`/Rotate 0` to the page dictionaries later changed the PDF bytes but not the
text, and re-running the commands above reproduces these files byte for byte.
