# The month-window tie-out against a live bank ledger: the balance read at a date is the opening of that day (#1628, Windows lab, 14 Oct 2026)

Synthetic data only: a new company BRIDGE TIE OUT LAB 3 (books from 1 Apr 2026), with a bank ledger
Union Bank (opening 10,000.00 Dr), a Suspense ledger and an income ledger, and one seed Receipt dated
1 Jun 2026 that does not touch the bank. TallyPrime 7.1, Silver Rental, on Windows. One run. The build is
#1628's bundle at 3ad41ae6 (bridge_mcp.exe sha256 1a2b150e...a5a3 at the start and the end), run from a
desktop config entry with its own data folder; posting was not set. The statement is a synthetic
three-month PDF written by the item's generator from the repository's own fixture code at master 4ebb9ea9
(3,927 bytes, sha256 506e05eb...4fc9, checked before every call). Every ComplyEaze Bridge answer is in
`results/`; the two imports were made by hand on Tally's own Import screen. Times are UTC.

## Result

Gaps are the book less the statement, in the statement's sign.

| step | call | result |
|---|---|---|
| S1 | parse, whole statement | 8 rows, 8 vouchers, no cash question, 8 to suspense |
| S2 | parse Jul / Aug / Sep / 2-31 Aug / Jun-Jul | 3 / 3 / 2 / 2 / 3 rows |
| S3 | tie-out, 2-31 Aug and Jun-Jul, before build | all three figures not established (`window_not_whole_calendar_months`, `window_beyond_statement_rows`); `window` null |
| S4 | tie-out, Jul, before build | opening, closing, change tied 0.00 |
| S5 | build Jul; import by hand; verify | 3 vouchers, 3 suspense lines; posted_verified 3 |
| S6 | tie-out, Jul, after post | tied 0.00, 0.00, 0.00: outcome A |
| S7 | build Aug; import by hand; verify | posted_verified 3; file hash differs from July's |
| S8 | tie-out, Aug, after post | tied 0.00 x3 |
| S9 | tie-out, whole statement, after post | opening tied; closing differs 800.00; change differs 800.00 |
| S10 | tie-out, Aug again, before build | opening tied; closing and change `file_already_partly_posted` |
| S11 | tie-out, Sep, before build | tied 0.00 x3 |
| S12 | ledger_movement Union Bank, 1 Jul-30 Sep | opening -10000.00, closing -11800 (numerically as expected) |
| S13 | Receipt dated 10 Oct 2026 keyed by hand; S8, S9, S11 again | the same figures |

- S6 decides: the bank balance read at a date is the opening of that day (outcome A). S8 extends it to a
  mid-year month start with bank vouchers on both sides; S13 to the case where the company's last voucher
  is after the window (the request hashes of S13's reads all differ from the first reads).
- S3's two tie-outs sent 4 requests each; every other tie-out (nine) sent 70, none failed. So no ledger
  was read for the two windows with no figure, as far as the trail shows.
- Each of the two hand imports showed only "Import completed successfully." on Tally's screen.
- Before each import: the company list showed only this company, the screen showed it, and the file's
  sha256 equalled the build's.
- The egress log of this build's data folder grew by exactly two lines per call (52 after 26 calls); no
  other data folder changed.

## Notes

- Tally's own import form asked to back up the company before importing, to the root of drive C; that
  option was set to No for this run and stays No on the lab PC (it is remembered by the form).
- Before the company was created, a click on Tally's company list loaded a different, unrelated sample
  company; nothing was keyed in it and nothing was sent to the port; it was shut a minute later and only
  its four access and state files changed.

## Not covered

Other banks' layouts; Gold; a statement ending at a financial year end; a voucher dated inside the
window posted during the read; ComplyEaze Bridge's own post.

## Files

`results/` (every tool answer, as transcribed), `records/` (the build's whole egress log at the end,
`agent-egress.jsonl`, 52 lines, holding every tie-out's line with its request trail; four of those lines
also as single files; the two import files), `step0/`, `answers/` (the status reads), `SHA256.txt`. In
`results/build-jul.json` and `results/build-aug.json` the lab PC's folder in `path` reads `<data folder>`;
the full files (4,528 bytes each) are listed by hash in `SHA256-kept-on-lab-pc.txt`, with the
screenshots, the working log, the summary and the per-call count file.
