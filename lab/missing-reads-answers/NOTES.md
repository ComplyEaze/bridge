# Raw answers for reads not captured before: allocations, batches, HSN and GST details, tax units, currencies (Windows lab, 14 Oct 2026)

Synthetic data only: BRIDGE SHAPE LAB, then BRIDGE CORPUS FOREX, each loaded alone. TallyPrime 7.1,
Silver Rental, on Windows. Read only: 17 Collection exports from 14 request files, each step once,
nothing keyed, imported or altered. Each send was `curl.exe -s -S -X POST -H "Content-Type: text/xml;
charset=utf-16" --data-binary @<request> -o <answer> -D <headers> -w "%{http_code} %{size_download}"
http://127.0.0.1:9001`, no client time limit, then `GET /status`. Every answer: exit 0, HTTP 200,
whole, under its size cap, in under a second; every status 200. Times in send-log.txt are UTC.

## Answers

| answer | read | bytes | holds |
|---|---|---|---|
| a01, b01 | company list | 4,000; 4,012 | the one loaded company, Education mode No |
| a02 = a12 | book extent, first book | 3,838 | ALTVCHID 112, ALTMSTID 289; byte-identical |
| a04, a05 | census and vouchers, 5 Jun 2025 | 5,438; 35,860 | 1 Journal |
| a06, a07 | census and vouchers, 24 Apr 2025 | 7,844; 110,156 | a Payment and a Purchase |
| a08 | the same day, inventory entries | 45,562 | see below |
| a09, a10 | stock items, stock groups | 21,606; 6,730 | 11 items, 3 groups |
| a11, b04 | tax units | 22,036 each | 2 rows on each book |
| a13, b03 | currencies, all native methods | 7,658; 8,818 | 2 rows on each book |
| b02 = b05 | book extent, second book | 3,848 | ALTVCHID 19, ALTMSTID 216; byte-identical |

- Cost centres (a05): on one ledger entry, `ALLLEDGERENTRIES.LIST/CATEGORYALLOCATIONS.LIST` (CATEGORY
  Business Line) holds two `COSTCENTREALLOCATIONS.LIST` rows: Trading -600.00, Assembly -400.00.
- Batches: the ledger-entry read (a07) already carries both, under
  `ALLLEDGERENTRIES.LIST/INVENTORYALLOCATIONS.LIST/BATCHALLOCATIONS.LIST`: H-2504-A at Main Location
  (300.000 Kgs, -9000.00) and H-2504-B at Factory Floor (366.667 Kgs, -11000.00).
- Inventory entries (a08, a shape not sent before): the Purchase returns one `ALLINVENTORYENTRIES.LIST`
  (666.667 Kgs, -20000.00) with the same two batch rows and one `ACCOUNTINGALLOCATIONS.LIST`, and three
  `LEDGERENTRIES.LIST`; the Payment returns one empty inventory list.
- HSN and GST details: every item and group row returns one `HSNDETAILS.LIST` and one
  `GSTDETAILS.LIST`. No row of the first book holds an HSN code. One item holds GST detail content
  (Taxable; CGST 9, SGST/UTGST 9, IGST 18).
- Tax units: neither book answered empty. Each has `Default Tax Unit` and one whose name reads
  ` Registration` with a leading space (USEDFOR GST, Regular from 20250401). No registration number.
- Currency: `DAILYSTDRATES.LIST`, `DAILYSELLINGRATES.LIST` and `DAILYBUYINGRATES.LIST` are returned.
  On the first book (USD and the rupee sign) all three are empty. On the second, the `$` master's
  selling list holds four rows of DATE and TRANSACTEDRATE (84, 85, 86, 88); the other two are empty.

`derived/analysis-output.txt` holds every row, parsed by `tools/read-answers.ps1`.

## Three things that differed

- Both books show `(e)` beside the company name at the Gateway and the line `* (e) Data exceptions
  exist`. It asks nothing, was not opened, and was unchanged at the end of each part.
- The checker failed once; the answer did not. At a13 the command `python -I check.py a13 ans-a13.xml
  --company ans-a02.xml` ended with exit 1 and a traceback (directories shortened here):
  `File ".../check.py", line 202, in main` /
  `File ".../encodings/cp1252.py", line 19, in encode` /
  `UnicodeEncodeError: 'charmap' codec can't encode character '\u20b9' in position 52`.
  Run once more on the kept file as
  `python -I -X utf8 check.py a13 ans-a13.xml --company ans-a02.xml`: exit 0, PASS, 2 currencies.
  Nothing was sent again; the later checks were run with `-X utf8`. `check-lines.txt` has every line.
- Tally's current date is each book's own: Monday, 20 Apr 2026 on the first, Tuesday, 15 Sep 2026 on
  the second (each its date of last entry). The plan expected one value for both. No date was
  changed, and the second book's requests carry no date.

Tally's window, captured before every request, always showed the Gateway. One tally.exe owned port 9001 before and after; no company was left loaded.

## Not covered

HSN codes on stock masters (the first book has none); TDS, TCS and a GST classification master (they need
a write first); standard and buying rates; other releases; Gold.

## Files

`requests/` (14), `answers/` (17, each with its headers and its status body), `check-lines.txt`
(the one traceback shortened as its first line says), `derived/`, `tools/`, `send-log.txt`, `SHA256.txt`.
`check.py` as run is that of commit 4dc25809dc56, sha256
408daf4f0a8c11ffd88934c387ea64a28d4802bb27de6af8faf4205655cb89c2; commit e3b1a70bc83b later made its
output ASCII.
Kept on the lab PC, by hash in `SHA256-kept-on-lab-pc.txt`: screenshots, listings, the working log.
