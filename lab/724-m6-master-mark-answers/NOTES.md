# Does saving a voucher type's numbering move the company's master mark? (#724, Windows lab, 13 Oct 2026)

Synthetic data only. Company: BRIDGE MANUAL NUMBERING LAB 2 (100059, GUID
c48f3906-7682-4828-9e87-2b660064b6f8), the Contra voucher type only. TallyPrime 7.1, Silver
Rental, not Education mode, port 9001. No connector build was used: every read is a raw HTTP
request sent by curl, one at a time, and every change was keyed on Tally's own voucher type
screen. Times are UTC unless marked IST (UTC+5:30).

## Result

Every save of the Contra voucher type screen moved the company's ALTMSTID by 2, including a
save with no field changed. ALTVCHID stayed 6 throughout. After each save Contra's own ALTERID
equals the new ALTMSTID.

| step | field changed on the screen (one per save, accepted with Ctrl+A) | ALTMSTID before | after | moved |
|---|---|---|---|---|
| u1 | none (screen opened and accepted) | 213 | 215 | yes |
| u2 | Method of Voucher Numbering: Automatic to Manual | 215 | 217 | yes |
| u3 | Method: Manual to Automatic | 217 | 219 | yes |
| u4 | Method: Automatic to Automatic (Manual Override) | 219 | 221 | yes |
| u4 back | Method: Automatic (Manual Override) to Automatic | 221 | 223 | yes |
| u5 | Method: Automatic to Multi-user Auto | 223 | 225 | yes |
| u5 back | Method: Multi-user Auto to Automatic | 225 | 227 | yes |
| u6 | Numbering behaviour on insertion/deletion: Retain Original Voucher No. to Renumber Vouchers | 227 | 229 | yes |
| u6 back | Numbering behaviour: Renumber Vouchers to Retain Original Voucher No. | 229 | 231 | yes |

What the series read shows for Contra (one VOUCHERNUMBERSERIES.LIST at every step; 24
VOUCHERTYPE elements at every step):

- u0: NUMBERINGMETHOD `Automatic`, PREVENTDUPLICATES `No`, NUMBERINGSUBMETHOD `Auto Retain`.
- u2: NUMBERINGMETHOD `Manual`, PREVENTDUPLICATES `No`. On the screen "Prevent creating
  duplicate Voucher Nos." is offered with the default No, which was left.
- u3: `Automatic` again.
- u4: NUMBERINGMETHOD is exactly `Automatic (Manual Override)`: 27 characters, plain ASCII
  (UTF-16LE hex in derived/analysis-output.txt). PREVENTDUPLICATES `No` (offered on the screen
  with the default No, left).
- u5: "Multi-user Auto" is offered on this Silver licence. The methods list reads: Automatic;
  Automatic (Manual Override); Manual; Multi-user Auto; None. NUMBERINGMETHOD is exactly
  `Multi-user Auto`, plain ASCII. PREVENTDUPLICATES `No`.
- u6: the screen's choices are "Renumber Vouchers" and "Retain Original Voucher No.". With
  Renumber Vouchers saved, the only child of Contra's series list whose text differs from u0 is
  NUMBERINGSUBMETHOD: `Auto Retain` becomes `Auto Renumber`. NUMBERINGMETHOD stays `Automatic`.
  So the setting is exported.
- End: for all 24 voucher types, each series' NUMBERINGMETHOD and PREVENTDUPLICATES equal u0's
  (compared by program over the parsed answers). Every child of Contra's series list equals
  u0's. Contra's type screen, opened to look and left without saving, shows every field as it
  was before u1.

Not equal to u0 at the end, and recorded as a fact of this company from now on:

- Contra's ALTERID is 231 (34 at u0); no other voucher type's ALTERID moved. The company's
  ALTMSTID is 231 (213 at u0).
- Contra's NUMBERINGMETHOD element directly under VOUCHERTYPE (not the one inside the series
  list) read `Default` at u0 and reads `None` in every answer from u2 on, including the end.
  The other 23 types still read `Default` there. This is why the end answer is 81,974 bytes
  and the u0 answer 81,978. On the pilot-lab company the same element reads `None` on its four
  voucher types that were created by hand and `Default` on the rest
  (answers/restart-pilot-series.xml).

## What was sent

- Marks: `requests/n2-r02-marks.xml` (1,002 bytes, sha256 4260c9effd23c5c877d03967641e939c954885c1c3d89ce330a30048b7aec6fa).
  It is `lab/1342-capture-requests/post-round/r02-marks.xml` at commit
  7fb92a528eea71c9ced5462dbf99c6ae7cabae0e (976 bytes, sha256
  81892a744009b6cf462a6f8d2fd505e650f8ad022e1462b23414cf2b8b3d2d7c, checked before the edit)
  with only the text inside SVCURRENTCOMPANY changed. UTF-16LE with the byte-order mark kept.
  ALTVCHID and ALTMSTID are read from the COMPANY row whose GUID is the company's; every marks
  answer holds exactly one COMPANY row.
- Series: `requests/n2-series-vt-request.xml` (1,116 bytes, sha256 caf423642547b7673624ddc9a5a400a10bae208fbb99fdaa643aec0e0dbc4c28),
  the same file at every step. It is `lab/724-series-capture/series-vt-request.xml` at commit
  b986cf2fa9d3d363f4130cd0defe57a5a388c826 (1,106 bytes, sha256
  70f1d9cefbfd00bda76fcdad35dc24707ba76d11a921e2ef63d8be317fc61934, checked before the edit)
  with only the text inside SVCURRENTCOMPANY changed.
- Every answer: body and headers saved, curl exit 0, HTTP 200, bytes downloaded equal to the
  CONTENT-LENGTH header. One line per request in `send-log.txt`. GET /status exists on this
  Tally and answers the same 51 bytes as GET /; it was sent before the first request and after
  the last.
- Answers are read with an XML parser (`tools/read-answers.ps1`); its output is
  `derived/analysis-output.txt`. Nothing is counted by text search.

## Order of work

1. 06:39:21 identity: the company was chosen from Select Company after typing its name until
   one row remained; the marks read then gave one COMPANY row with the company's GUID,
   ALTVCHID 6, ALTMSTID 213.
2. u0: marks (6 / 213) and series (81,978 bytes). Tally's Statistics screen for 1-Apr-26 to
   22-Jul-26 (the date of the last entry) shows Contra 0, Journal 3, Payment 2, Receipt 1,
   total 6.
3. Backup before any change, by Tally's own Backup screen to a local folder:
   TDBK1800_100059.001, 32,154 bytes, sha256
   f680bbf85da0f6ef6caafb2af3ef7135e972643db659f45f53516a278bc89b05, written 06:42:53. The
   screen's "Backup Data Path" opened on TallyDrive (cloud storage); it was changed to
   "Specify Path" and a local folder before accepting. A marks read after the backup: still
   6 / 213 (byte-identical answer).
4. u1 to u6 as in the table, 06:44 to 06:51. For each save: Gateway, Alter, Voucher Type,
   Contra; the one field changed; a screenshot; Ctrl+A; then the reads. No save raised a
   dialog. Where a method change brings up a dependent field, that field's offered default was
   left and is recorded above.
5. End, 06:51 to 06:52: series read and comparison; Contra's screen opened to look. There the
   first Esc, pressed in the Name field, blanked that field on the screen; the second Esc
   raised "Quit ? Yes or No", answered Yes. Nothing was saved: the marks read after it is
   byte-identical to the read after the last save (6 / 231), and the list still shows Contra.
6. The company was shut. Tally was left running with no company loaded.

Two marks reads are extra to the steps: the one after the backup, and the one after the look
at the end. Both use the marks request above.

## A stop before u0, and the restart (11:57 to 12:07 IST)

Told as it happened, because the reads above were taken after it.

- 06:27:14 GET /status 200. 06:27:20 a marks read of the pilot-lab company (BRIDGE PILOT LAB,
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85), which was the only company loaded: ALTVCHID 68,
  ALTMSTID 245 (answers/restart-pilot-marks-before.xml).
- A second TallyPrime process was then started by mistake. Shortly afterwards the first
  process, which held port 9001, was no longer running. Nothing was sent to end it and the
  cause is not established. One GET / at 06:28:41 failed to connect (curl exit 7) and is
  recorded as a failed connection. Work stopped there; for this item only the two requests in
  the bullet above had been sent.
- The second process was closed with Tally's own quit. 06:34:46 Tally was started once:
  exactly one process, owning 9001; GET /status 200.
- 06:36:24 the pilot-lab company was loaded alone and the same marks request sent: the answer
  is byte-identical to the 06:27:20 answer (sha256
  8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6; 68 / 245). 06:36:36 a
  series read of it: 93,546 bytes, 28 voucher types (answers/restart-pilot-series.xml). It was
  then shut and its closed data folder copied by file copy (20 files, 878,080 bytes; hash list
  kept on the lab PC).
- On the Gateway, "(e) (Schedule Backup)" beside the company name and "(e) Data exceptions
  exist" at the foot were seen on the pilot-lab company before and after the restart; neither
  was opened. On BRIDGE MANUAL NUMBERING LAB 2 the Gateway showed "(Schedule Backup)" without
  the "(e)" and no line at the foot. From 06:38 the item ran on the restarted Tally, one
  process throughout.

## Not covered

A Gold licence; a voucher type that holds vouchers, for u4 and u5; the Journal, Payment and
Receipt types; the same answers read through the connector.

## Files

- `requests/` the three requests as sent. `answers/` every answer with its headers.
  `send-log.txt` one line per request. `derived/analysis-output.txt` the parsed tables;
  `derived/m6r-u6-contra-children-diff.txt` the one differing child at u6.
  `tools/read-answers.ps1` the reader. `SHA256.txt` this folder's files.
- Kept on the lab PC, listed by hash in `SHA256-kept-on-lab-pc.txt`: 18 screenshots, the
  backup, the hash list of the folder copy, and the working log.
