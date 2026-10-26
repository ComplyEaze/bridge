# Results: #1233, stock items and Stock Summary to two dates inside the year on BRIDGE TIEOUT D, 26 Oct 2026. PASS: each answer gives the closing at its own end date (22 Nos at 30 Sep; 20 Nos at 31 May), and each stock items answer and its Stock Summary give the same value (2,640.00; 2,200.00)

The request is `lab/1233-stock-mid-year-requests` at 1c0037fed7808a8406b825662fb1e8b5c5fc189a. Its `SHA256.txt` checks 8/8 OK. `j-r4` was taken from `lab/692-opening-stock-d3-requests` at 6f4f7869909bbff7081ac29f6faa3ebdc3eebfdb, where its sha256 d621a687e49f41574c7d137f9a8d586d3a8027ceefa086cbf593a77ab654fccc equals that request's `MANIFEST.sha256`.

`python -I check-request-ids.py requests` gave `census: ok 3, skipped 1, FAIL 0`. The skipped file is `k4`; checked by eye, its ID is `Stock Summary` and it defines no `COLLECTION`. `m-marks-D` is the template with TIEOUT D's GUID filled in, and its check gave `ok 1`.

Reads only; no owner act was needed. Every request was sent by curl, one at a time. The screen acts were:
- loading and shutting the company;
- opening the Profit and Loss and Stock Summary reports;
- changing the Stock Summary period with Alt+F2.

Each was one key per action. Nothing was keyed into a voucher or master, imported, backed up or restored. No dialog appeared at any point.

TallyPrime 7.2, Silver, Licensed: this is the same tally.exe (PID 29692) that the built tool read on the same day. The About screen was not opened, and opening the company raised no prompt.

## State S
The company was loaded alone. The Gateway showed:
- period 1-Apr-25 to 31-Mar-26;
- current date 15-Oct-2025;
- last entry 15-Oct-25.

Profit & Loss A/c, 1-Apr-25 to 15-Oct-25:

| Line | Amount |
|---|---|
| Opening Stock | 1,000.00 |
| Purchase Accounts | 2,600.00 |
| Sales Accounts | 2,850.00 |
| **Closing Stock** | **2,040.00** |
| Nett Profit | 1,290.00 |

Closing Stock 2,040.00 is state S, so the reads went ahead.

## The reads (all HTTP 200, each under its bound)
| Step | Request | Bytes | sha256 | Lab Gadget |
|---|---|---|---|---|
| 1 | `m-marks-D`, before | 4,886 | 84e0a5be…4104 | ALTVCHID 4, ALTMSTID 213; GUID 6a9518b2-a88c-417c-a700-40243f9d9119 |
| 2 | `k1` stock items, to 30-Sep-2025 | 4,356 | 2a85a2a3…2c4a | CLOSINGBALANCE ` 22 Nos`, CLOSINGVALUE `-2640.00` (opening ` 10 Nos`, `-1000.00`) |
| 3 | `j-r4` Stock Summary, 1-Apr to 30-Sep-2025 | 510 | 6ea369d1…16a7 | DSPCLQTY `22 Nos`, DSPCLRATE `120.00`, DSPCLAMTA `-2640.00` |
| 4 | `k3` stock items, to 31-May-2025 | 4,356 | c6414fcc…c3fc | CLOSINGBALANCE ` 20 Nos`, CLOSINGVALUE `-2200.00` (opening ` 10 Nos`, `-1000.00`) |
| 5 | `k4` Stock Summary, 1-Apr to 31-May-2025 | 510 | 64f67059…fe71 | DSPCLQTY `20 Nos`, DSPCLRATE `110.00`, DSPCLAMTA `-2200.00` |
| 6 | `m-marks-D`, after | 4,886 | 84e0a5be…4104 | byte-identical to step 1 |

`j-r4` had not been sent at S earlier in this sitting, so it was sent once here. Its answer is byte-identical (same sha256) to the 21 Oct `d1-r5` answer on this book.

The Stock Summary answers hold one item and carry no separate report-total element. The item's line is the report's whole content.

## Tally's own Stock Summary (screenshots)
| Period | Lab Gadget | Grand Total |
|---|---|---|
| 1-Apr-25 to 15-Oct-25 (as opened) | 17 Nos @ 120.00 = 2,040.00 | 17 Nos, 2,040.00 |
| 1-Apr-25 to 30-Sep-25 (Alt+F2) | 22 Nos @ 120.00 = 2,640.00 | 22 Nos, 2,640.00 |
| 1-Apr-25 to 31-May-25 (Alt+F2) | 20 Nos @ 110.00 = 2,200.00 | 20 Nos, 2,200.00 |
| 1-Apr-25 to 31-Mar-26 (Alt+F2, set back) | 17 Nos @ 120.00 = 2,040.00 | 17 Nos, 2,040.00 |

After the period was set back, the report was left. The Gateway again showed period 1-Apr-25 to 31-Mar-26, current date 15-Oct-2025 and last entry 15-Oct-25, as at the start.

## What this records
- **The stock items collection follows its own `SVTODATE`.** `k1` gives 22 Nos at 30 Sep and `k3` gives 20 Nos at 31 May. Neither gives the 17 Nos held after the 15 October sale.
- **Each stock items answer and its Stock Summary agree on the item's value**: 2,640.00 at 30 Sep and 2,200.00 at 31 May. The screen shows the same figures. On the wire, the value of stock held is written negative.
- **30 Sep, 22 Nos at 120.00 = 2,640.00**, as expected.
- **31 May, 20 Nos at 110.00 = 2,200.00.** On this book that is the average over the inwards to that date: 10 Nos opening at 100.00 and 10 Nos bought on 5 May at 120.00. The request left this value open.

## Pass / fail
PASS. Every read answered HTTP 200, each under its bound, and each was saved with its size and sha256. The marks after equal the marks before, byte for byte.

C10: Company > Shut. Then Select Company with none loaded; one tally.exe; 9001 LISTENING; /status 200, 51 B. No XML request was sent after the shut.

## In this commit
- `answers/`: every answer as received, with its headers; the C10 observation and /status answer.
- `requests/`: the two request files not in the request folder: `m-marks-D` (the filled template) and `j-r4`.
- `send-log.txt`: every read, with request sha256, send and answer times (UTC), curl output, bytes, answer sha256 and bound.
- `notes.txt`: UTC times, one line per step.
- `SCREENSHOTS.txt` (name and sha256) and `SHA256.txt`.

Not committed: the screenshots, which stay on the lab PC.

## Not measured (as the request says)
A costing method other than the default, a second item, a stock group, godowns, batches, a window across two financial years, or another book.
