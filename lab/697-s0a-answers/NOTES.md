# 697-S0A answers: the audit-read request shapes against a real Tally (#697, first slice), 15 Oct 2026

Not for merge. Synthetic company BRIDGE PILOT LAB only, loaded alone. Read only: eight request files, ten
sends, nothing written, no Tally setting changed. Item description and requests: branch
`lab/697-s0a-run-sheet` at 6cab0068 (WINDOWS-697-S0A.md; SHA256.txt sha256 99108e37...; every file matched
before the first send). TallyPrime 7.1 Silver, Education mode No, one tally.exe owning the port. No build
was running during the item. Confidence: one run, PARTIAL.

Every send is the item's curl command verbatim, one at a time, with no time or size limit; the answers are
kept byte for byte (UTF-16LE as Tally sent them; `.gitattributes` is `* -text`) for the replay through the
reader.

## Results

| step | request | HTTP | bytes | seconds | count recorded |
|---|---|---|---|---|---|
| 0b | r01 company list | 200 | 4,000 | 0.014 | 1 company |
| 1 | r02 marks | 200 | 3,394 | 0.012 | 1 COMPANY row |
| 2 | s03 company Object | 200 | 4,208 | 0.019 | 1 COMPANY under TALLYMESSAGE |
| 3 | s04 groups | 200 | 55,954 | 0.038 | 29 GROUP |
| 4 | s05 ledgers | 200 | 26,106 | 0.049 | 18 LEDGER |
| 5 | s06 trial balance | 200 | 24,498 | 0.056 | 18 LEDGER |
| 6 | s07 voucher types | 200 | 37,956 | 0.021 | 28 VOUCHERTYPE |
| 7 | s08 vouchers of the period, entry wildcard | 200 | 2,568,786 | 0.774 | 41 VOUCHER |
| 8 | r02 marks again | 200 | 3,394 | 0.012 | 1 COMPANY row |
| 9 | r01 company list again | 200 | 4,000 | 0.013 | 1 company |

Every answer: HTTP 200, STATUS 1, no LINEERROR, parsed without error, under 16 MiB; every POST under 20 s.

- 0b: exactly one company, BRIDGE PILOT LAB; Education mode No; Silver Yes; books from 2026-04-01 (on or
  before 2026-04-01). GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85.
- 1: one COMPANY row with that GUID; ALTVCHID (M) 68, ALTMSTID 245; M within 1 to 170.
- 2: exactly one `/ENVELOPE/BODY/DATA/TALLYMESSAGE/COMPANY`, NAME BRIDGE PILOT LAB, with GUID equal to 0b's
  and BOOKSFROM present, element text exactly `20260401` (equal to 0b's books-from). ISINTEGRATED is
  present, text `Yes`.
- 3: 29 groups, every one with a PARENT child; the top-level groups' parent reads ` Primary` after the
  parse snippet removes the control character, as the item predicts.
- 4: 18 ledgers, every one with a PARENT child.
- 5: 18 ledger rows in the trial balance.
- 6: 28 voucher types, every one with a NAME attribute and a PARENT child.
- 7: 41 VOUCHER rows, every one with a GUID child (none skipped); 41 is at most M (68). Not envelope only.
- 8: ALTVCHID 68 and ALTMSTID 245, unchanged; the answer is byte-identical to step 1's.
- 9: the same single company and GUID; the answer is byte-identical to step 0b's.

## Files

- requests/: the eight request files, byte for byte.
- answers/: every answer (`<step>.xml`) and its headers (`<step>.headers.txt`).
- send-log.txt: one line per send.
- SHA256.txt: sha256 and size of every other file in this folder.
- Kept on the lab PC and not on this branch: one screenshot of step 0a (the Gateway with the company loaded
  alone), 44,176 bytes, sha256 126eb0c7120905659ffe1e8e156218439e417b6bb23adcd667992fc79864570e.

## Not established

The counted and divided read of a book above the single-request size (the census requests and the parts
planned from it); Education mode's outward rounding; ComplyEaze Bridge's own HTTP headers (curl's User-Agent
and Accept differ; the body is the same); the identity check Bridge makes before and after each of these
requests, and the marks read its voucher window makes just before the vouchers; timings under load; any
client book.
