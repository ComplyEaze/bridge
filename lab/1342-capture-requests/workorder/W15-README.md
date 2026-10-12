# W15: which objects does a GSTR-1 JSON export alter?

Synthetic company `BRIDGE PILOT LAB` only. W13 found that each GSTR-1 JSON export for upload moves the company's voucher mark (`ALTVCHID`) by 3 without changing any of the 28 August Sales vouchers. This finds the three objects. It writes what an export writes and nothing else. Take a fresh backup first and say where it is.

**STOP rule (any step).** Stop and tell us before going on if: Tally shows a dialog this list does not mention (the "potential conflicts between the master and the transaction" warning and the export options are the ones W13 saw: take the same choices as in W13 and screenshot them); an answer is not a complete envelope with a `COLLECTION` and no `LINEERROR` (the checker says FAILED READ); the mark moves by anything but 3 over the export; a script refuses its input. Never use Accept As Is. Do not alter or delete any voucher or master.

**The two scripts** (in this folder, pinned by hash in `SHA256.txt`; run them with `python3 -I`, which needs no packages):
- `make-w15-request.py BEFORE AFTER FROM TO OUTFILE` writes the request for the vouchers whose `AlterID` is above BEFORE and not above AFTER, in the date window FROM to TO (a collection read with no dates returns only the display period, so a window is always given). It prints the file's SHA-256 and size; write the printed line into your notes. Its header ID equals its collection name (checked by the script).
- `check-w15-answer.py ANSWER BEFORE AFTER` reads the answer and says IDENTIFIED (exactly AFTER minus BEFORE vouchers, with each `ALTERID` from BEFORE+1 to AFTER once), NOT IDENTIFIED (a complete answer that does not hold that set: this is not "nothing was altered", the counter moved) or FAILED READ (nothing is claimed).

## Steps
1. Send `r02-marks.xml` (earlier set) -> `aw15-marks-0.xml`. Read `ALTVCHID`: call it BEFORE.
2. Do ONE GSTR-1 JSON export of August 2026 exactly as in W13 (Export > GST Returns > Upload GSTR-1 > the X: button that writes the file rather than the S: Send button, with the same choices as W13; do not use Include Uploaded). Say what the Upload GSTR-1 screen read before and after (pending counts). Copy the JSON file out unchanged as `gstr1-aug-2026-w15.json`.
3. Send `r02-marks.xml` -> `aw15-marks-1.xml`. Read `ALTVCHID`: AFTER. STOP unless AFTER = BEFORE + 3.
4. Run `make-w15-request.py BEFORE AFTER 20260401 20270331 w15-a.xml` and send it -> `aw15-altered-a.xml`; run `check-w15-answer.py aw15-altered-a.xml BEFORE AFTER` and paste its whole output.
5. Run `make-w15-request.py BEFORE AFTER 20250401 20260331 w15-b.xml` and send it -> `aw15-altered-b.xml`; run the checker on it likewise (the objects may sit in the previous financial year's window).
6. Send `r02-marks.xml` -> `aw15-marks-2.xml` and say whether the marks equal `aw15-marks-1.xml` (the reads must not move it).
7. (Optional, only if steps 1 to 6 ran clean) a SECOND export as in step 2 (call the marks X and Y, expecting Y = X + 3 with X = AFTER), then steps 4 to 5 with X and Y -> `aw15-altered-c.xml`, `aw15-altered-d.xml`. Compare the object numbers with the first export: are the same three objects altered again?
8. Screens: the Upload GSTR-1 screen before and after each export (counts and the "Pending to Upload" lines), the GSTR-1 summary.

## What to post back
The answers, the generated request files with their printed hashes, the checker outputs verbatim, the JSON copy, screenshots and hash additions in `SHA256.txt`; for each step what happened with curl's exit code and printed line. Say whether you did anything that differs from this list.

---

*For the reader (not for the lab):* if the checker says IDENTIFIED we learn the voucher type, number, date and narration of the three objects (candidates: the GST return-summary records that the "pending to upload" counts come from). If it says NOT IDENTIFIED with 0 rows in both windows, the objects are not in the Voucher collection (or have no date in either window), and the next read changes the object class. The answer decides whether a read that treats a mark rise as "the book changed" (issue #1633) can recognise an export from its own signature (three objects of that class, no Sales voucher changed).
