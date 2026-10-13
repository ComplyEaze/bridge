# The uncertain-Sales read (status, party, amount) around one Accept As Is (#1342, Windows lab, 13 Oct 2026)

Synthetic data only: BRIDGE PILOT LAB (its GUID is in every marks answer), BRIDGE Sales vouchers of August
2026. TallyPrime 7.1, Silver Rental, on Windows. One run. No ComplyEaze Bridge build: every read is a raw
HTTP request by curl, one at a time; the one accept was made on Tally's own GSTR-1 screen. A backup was
taken before the accept and restored after it. Times in send-log.txt are UTC.

## Method

- Marks: `requests/r02-marks.xml`.
- Narrow status read: `requests/w21-status-window-aug-postdated.xml`, already answered on the branch
  `lab/1342-w21-answers` as `answers/aw21-b.xml`.
- Wide status read: `requests/w21d-status-window-aug-wide-postdated.xml`, the narrow one with
  PARTYLEDGERNAME and AMOUNT added.
- Order: marks-0; narrow (a); wide (b); three invoices looked at in display mode; backup; GSTR-1 figures;
  Accept As Is on BP/26-27/0003 alone; wide (c); marks-1; restore; marks-2.
- `tools/read-answers.ps1` parses the answers; its output is `derived/analysis-output.txt`. Nothing is
  counted by text search.

## Result

- (a) 108,442 bytes, byte-identical to `aw21-b.xml` of the earlier run: 32 voucher rows.
- (b) 242,632 bytes: the same 32 vouchers with the same flags, each with PARTYLEDGERNAME and AMOUNT once
  as direct children. AMOUNT is the invoice total, negative: BP/26-27/0001 -1050.00, 0003 -1051.04, 0016
  -210.42, where the screen showed 1,050.00, 1,051.04 and 210.42, party BRIDGE Walk-in on each. The
  cancelled BP/26-27/0041 reads an empty party and amount; the optional BP/26-27/0042 reads its party and
  -1050.00.
- (c) 242,636 bytes: differs from (b) only in BP/26-27/0003: ALTERID 50 to 69, VCHGSTSTATUSISINCLUDED No
  to Yes, VCHGSTSTATUSISUNCERTAIN Yes to No, ISGSTOVERRIDDEN No to Yes, AMOUNT unchanged. No other
  voucher's XML differs.
- Marks (ALTVCHID / ALTMSTID): 68 / 245 at the start, 69 / 245 after the accept, 68 / 245 after the
  restore; the last answer is byte-identical to the first.
- GSTR-1 screen for August, before the accept: 39 vouchers, 27 included, 7 not relevant, 5 uncertain;
  after it: 39, 28, 7, 4.

## The accept

The uncertain list held 5: 2 under "HSN/SAC is invalid, mismatched, or not specified" and 3 under
"Mismatch between Expected Tax Amount and Modified Tax Amount in transaction" (BP/26-27/0002, 0003, 0008).
With only the row of 0003 selected, Accept As Is raised one question: "This exception will be accepted
and included in Returns. Do you want to accept as is? Yes or No". It was answered Yes once. Afterwards the
list showed 0002 and 0008; the screen offered nothing to undo the accept, and no undo was tried. The
restore asked "Accept ?" and then whether to replace the existing company data folder; Replace was chosen.

## One pause

Read (b) came in at 242,632 bytes against a written expectation of about 450 to 480 KB. The run paused
there; the expectation was a wrong figure and was corrected to about 225 to 250 KB (the size of the
earlier wide answers), and the run went on.

## What it decides

On this book and this release:

- The wide read returns the party and the signed invoice total of each Sales voucher beside the status
  flags, and adding the two fields changes no flag.
- One Accept As Is on one voucher changed that voucher alone: not uncertain, included, overridden, a higher
  ALTERID, and ALTVCHID up by one; ALTMSTID did not move.
- A cancelled voucher reads an empty party and amount here; an optional one does not.

## Not covered

An item invoice, a registered buyer, a credit note or another Sales-class type, a foreign-currency voucher,
the amount of a post-dated voucher, an accept of several vouchers at once, Gold, other releases.

## Files

`requests/`, `answers/` (every answer with its headers), `derived/analysis-output.txt`, `send-log.txt`,
`tools/read-answers.ps1`, `SHA256.txt`. Kept on the lab PC, listed by hash in `SHA256-kept-on-lab-pc.txt`:
12 screenshots, the backup, the process listings and the working log.
