# A post-dated Sales voucher read with Tally's current date before and after its date (#1342, #1664; Windows lab, 13 Oct 2026)

Synthetic data only: BRIDGE PILOT LAB (its GUID is in every marks answer), BRIDGE Sales vouchers of August
2026. TallyPrime 7.1, Silver Rental, on Windows. One run. No ComplyEaze Bridge build: every read is a raw
HTTP request by curl, one at a time; the one voucher was keyed on Tally's own screen. A backup was taken
first and restored at the end. Times in send-log.txt are UTC.

## Method

- Marks: `requests/r02-marks.xml`. Status read: `requests/w21-status-window-aug-postdated.xml`, already
  answered on the branch `lab/1342-w21-answers` as `answers/aw21-b.xml`.
- Tally's current date is set with F2 at the Gateway. It read 15-Sep-2026 at the start.
- Order: backup; marks-0; date to 10-Aug-2026; GSTR-1 figures (g0); read (a); one BRIDGE Sales invoice
  keyed, dated 25-Aug-2026, numbered BP/26-27/0045, the shape of BP/26-27/0001 (BRIDGE Walk-in, 1,000.00
  plus 25.00 and 25.00 tax, total 1,050.00), marked post-dated before saving; date to 10-Aug-2026 again;
  read (b); GSTR-1 figures (g1); Day Book for 25 Aug; date to 26-Aug-2026; read (c); GSTR-1 figures (g2);
  Day Book again; marks-1; restore; marks-2; date back to 15-Sep-2026.
- `tools/read-answers.ps1` parses the answers; its output is `derived/analysis-output.txt`.

## Result

| read | current date | bytes | finding |
|---|---|---|---|
| a | 10 Aug, voucher not yet keyed | 108,442 | byte-identical to `aw21-b.xml` of the earlier run; 32 rows |
| b | 10 Aug, before the voucher's date | 111,748 | the 32 rows unchanged, plus BP/26-27/0045 |
| c | 26 Aug, after the voucher's date | 111,748 | byte-identical to (b) |

- BP/26-27/0045 in (b) and (c): ISPOSTDATED Yes (once), VCHGSTSTATUSISINCLUDED Yes,
  VCHGSTSTATUSISUNCERTAIN No, VCHGSTSTATUSISEXCLUDED No, ISGSTOVERRIDDEN No, ALTERID 69, MASTERID 45.
- GSTR-1 screen for August (vouchers / included / not relevant / uncertain): g0 39 / 27 / 7 / 5; g1 and g2
  both 40 / 28 / 7 / 5, with 1,000.00 more taxable value than g0.
- The Day Book for 25 Aug listed BP/26-27/0045 (1,050.00) at both current dates.
- Marks (ALTVCHID / ALTMSTID): 68 / 245, then 69 / 245 with the voucher, then 68 / 245 after the restore;
  the last answer is byte-identical to the first.
- After the save the Gateway showed the current date 25-Aug-2026 (the date keyed in the voucher). After
  the restore and reload it showed 15-Sep-2026 before F2 was pressed.

## One pause, while keying

In the ledger field of the voucher, "BRIDGE CGST" was typed and Enter pressed while two ledgers matched.
Tally showed a red box: "Error / Oops! / Too many choices". The run paused with the box on screen. Three
acts were then approved one at a time: one Enter (the box stayed), one click on the row "BRIDGE CGST 2.5%"
(the box went, the row was not yet chosen), a second click on that row and Enter (taken). The SGST ledger
was then chosen by typing until one row was left. Earlier on the same line 1000 was typed into the rate
field by mistake; it was cleared before leaving the line and entered as the amount, the rate left blank
as on BP/26-27/0001. The save asked nothing. The restore asked "Accept ?" and then whether to replace the
existing company data folder; Replace was chosen.

## What it decides

On this book and this release:

- Tally's current date alone did not change the status read (a).
- A post-dated Sales voucher read ISPOSTDATED Yes and included Yes both while the current date was before
  its date and after it; the two answers are the same bytes.
- The GSTR-1 screen counted that voucher at both dates, and the Day Book of its date listed it at both.

## Not covered

A post-dated voucher whose date lies in a later return period; one that is uncertain; other voucher types;
Tally started afresh between the two dates; Gold; other releases.

## Files

`requests/`, `answers/` (every answer with its headers), `derived/analysis-output.txt`, `send-log.txt`,
`tools/read-answers.ps1`, `SHA256.txt`. Kept on the lab PC, listed by hash in `SHA256-kept-on-lab-pc.txt`:
12 screenshots, the backup, the process listing and the working log.
