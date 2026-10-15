# Reads only: the ledgers of BRIDGE SHAPE LAB with their dated GST details, as Tally sends them

Synthetic company `BRIDGE SHAPE LAB` only, loaded alone. **Nothing is written, keyed, accepted, created or deleted: three Export requests and three status reads, and no key is pressed in Tally.** The answers are the raw evidence for #1741: the wire value of `GSTINELIGIBLEITC` (Yes / No / absent) on every ledger that holds a GST details entry, to be set beside the setting `Eligible for Input Tax Credit` that Tally's own screens already show for those ledgers.

Files: `requests/` (3), `check.py`, `check-request-ids.py`, `.gitattributes`, `SHA256.txt`. Check the files against `SHA256.txt` (PowerShell: `Get-FileHash -Algorithm SHA256 FILE`), then `python -I check-request-ids.py requests` (EXPECT `census: ok 3, skipped 0, FAIL 0`). The three requests are UTF-16LE with a byte-order mark and no final line break, sent as they are; each answer is saved raw with `-o`, never opened in an editor that rewrites it.

**Stop and report** if: Tally shows any dialog; `check.py` exits 3 or 2; an answer is not a complete envelope, carries a `LINEERROR` or has header `STATUS` 0; an answer is above its cap (company list 8,000 B, book extent 6,000 B, ledger read 100,000 B); an answer has not finished after five minutes (do nothing: do not cancel it, do not send it again, do not touch Tally); a status read is not HTTP 200 `TallyPrime Server is Running`; a company other than `BRIDGE SHAPE LAB` is loaded.

**How to send.** One at a time: `curl.exe -s -S -X POST -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests\FILE" -o ans-NAME.xml -w "%{http_code} %{size_download}" http://127.0.0.1:9001`, then `curl.exe -s -S -o status-NAME.txt -w "%{http_code}" http://127.0.0.1:9001/status`. Note curl's printed line and exit code for each.

## Steps
1. `r01-company-list.xml` -> `ans-r01.xml`, then the status read. `python -I check.py a01 ans-r01.xml`. EXPECT PASS (the only company is `BRIDGE SHAPE LAB`, Education mode No).
2. `r02-book-extent.xml` -> `ans-r02.xml`, then the status read. `python -I check.py a02 ans-r02.xml`. EXPECT PASS. Write down `ALTVCHID`, `ALTMSTID`, `COMPANYNUMBER`.
3. `r03-ledger-gst-details.xml` -> `ans-r03.xml`, then the status read. Size cap 100,000 B. `python -I check.py base ans-r03.xml --company ans-r02.xml`. EXPECT PASS: every ledger row carries this book's GUID. The printed lines list each ledger that holds a GST details entry: its parent, applicable-from, taxability, source, the wire state of `GSTINELIGIBLEITC` as Yes / No / absent, and its rate rows. If the check says there is no ledger row, report it (the request is sent without a date range; whether it answers that way on this book has not been seen).
4. Record the table below from the printed lines and from what Tally's own screens already show for those ledgers (from the lab's existing records; no screen is opened for this and nothing is keyed). A ledger whose screen setting is not known is listed with `not known`: it is not looked at here.

| ledger | parent | details source | wire GSTINELIGIBLEITC (Yes / No / absent) | screen `Eligible for Input Tax Credit` (Yes / No / not known) |
|---|---|---|---|---|

5. If no ledger of this book holds a GST details entry whose screen setting is known, say so: the pairing then needs the ledger keyed on the screen, which is the ITC row of the combined release-candidate sheet (a created ledger, its setting keyed Yes and then No, a read after each, a backup first and a restore at the end). Its reads are these same three requests.

## Hand back
Every `ans-*` and `status-*` file with bytes and sha256, every `check.py` line, the curl lines (code, size, exit code), and the table. The book is synthetic, so the answers are handed back whole.
