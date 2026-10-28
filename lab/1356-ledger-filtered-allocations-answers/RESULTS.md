# Results: #1356 ledger-filtered allocations, 28 Oct 2026. On TallyPrime 7.2 the `Vouchers : Ledger` collection, given the whole-window read's field list, returns each voucher's entries and bill allocations equal to the whole-window read, voucher for voucher: 7 of 7 for Lab Party P1 on BRIDGE BILLTRAIL LAB, 82 of 82 for DN Sales on BRIDGE CORPUS DENSE. Both new shapes answered in-band with no dialog. Reads only; the marks did not move

The sheet is `lab/1356-ledger-filtered-allocations-run-sheet` at fd02e22f5282209018acef6326d8a2b07df83ee2.
- **Sheet checks:** `SHA256.txt` 13 of 13; `check-request-ids.py requests`: census ok 8, skipped 0, FAIL 0.
- **Tally:** TallyPrime 7.2, Silver Rental, Licensed. One tally.exe (13992) owned port 9001 throughout; `GET /status` returned 200 before and after each new shape.

## Answers
| request | sent / answered (UTC) | time | bytes | rows (each with GUID, ALTERID, MASTERID) | sha256 |
|---|---|---|---|---|---|
| t01_marks (start) | 13:56:33.714 / 13:56:35.553 | | 3,400 | ALTVCHID 7, ALTMSTID 212 | 0fe9f835... |
| t02_vouchers_fy (truth) | 13:56:56.840 / 13:56:57.542 | 0.70 s | 249,288 | 7 VOUCHER | 80c78964... |
| t03_ledger_vouchers_p1_fy (control) | 13:56:57.574 / 13:56:57.835 | 0.26 s | 21,904 | 7 VOUCHER, GUID set = t02 | 9a90cab5... |
| t04_ledger_vouchers_p1_fy_entries (new) | 13:57:06.812 / 13:57:07.470 | 0.66 s | 239,294 | 7 VOUCHER, GUID set = t02 | c5e1413c... |
| t01_marks (end) | 13:57:32.420 / 13:57:32.934 | | 3,400 | 7 / 212, byte-equal to the start | 0fe9f835... |
| d01_marks (start) | 13:58:50.020 / 13:58:50.628 | | 3,406 | ALTVCHID 29900, ALTMSTID 328 | fcc9086d... |
| d02_vouchers_20250515 (truth) | 13:58:58.519 / 13:58:59.872 | 1.35 s | 2,900,742 | 82 VOUCHER | 9d72ea3c... |
| d03_ledger_vouchers_dn_sales_20250515 (control) | 13:58:59.976 / 13:59:00.305 | 0.33 s | 226,626 | 82 VOUCHER, GUID set = d02 | e400170a... |
| d05_ledger_vouchers_dn_sales_20250515_entries (new) | 13:59:09.640 / 13:59:10.783 | 1.14 s | 2,785,026 | 82 VOUCHER, GUID set = d02 | 1b7b2173... |
| d01_marks (end) | 13:59:32.438 / 13:59:32.8 | | 3,406 | 29900 / 328, byte-equal to the start | fcc9086d... |

The times are measured from send to answer by the lab's send line, so they include its own overhead. Every answer was under its bound and parses.

## compare1356.py
| run | exit | result |
|---|---|---|
| "Lab Party P1" 7 t02 t04 | 0 | all 7 equal: the voucher fields and each Lab Party P1 `ALLLEDGERENTRIES.LIST` subtree, bill allocations included (for example New Ref CN-1 600.00 with On Account 400.00, and Agst Ref INV-2 2000.00) |
| "Lab Party P1" 7 t02 t03 | 1 | all 7 DIFFERENT: the control carries no entries (GUID set equal) |
| "DN Sales" 82 d02 d05 | 0 | all 82 equal |
| "DN Sales" 82 d02 d03 | 1 | all 82 DIFFERENT: no entries (GUID set equal) |

## Cost
| book | truth | identity-only | full-field ledger-filtered |
|---|---|---|---|
| BILLTRAIL (7 vouchers, the year) | t02: 249,288 B, 0.70 s | t03: 21,904 B, 0.26 s | t04: 239,294 B, 0.66 s |
| CORPUS DENSE (82 DN Sales vouchers, one day; the day holds only these) | d02: 2,900,742 B, 1.35 s | d03: 226,626 B, 0.33 s | d05: 2,785,026 B, 1.14 s |

On both books every voucher in the window carries the ledger. The full-field candidate therefore returns about the truth's bytes (96 %), less only the truth's COMPUTE fields. Its saving on a book where the ledger touches few vouchers is not measured here.

**Confidence:** one sitting, PARTIAL until repeated. The sheet's "Not covered" list stands: CHILDOF with other characters or a missing ledger, optional and cancelled vouchers, the opening bill, COMPUTE fields, a party ledger on the dense book, and any build's own read.

## Files
- `raw/`: every answer, whole, with its headers. The marks are saved as `-start` and `-end`; `status-*` holds the `GET /status` bodies.
- `cmp-*.txt`: each `compare1356.py` output, whole.
- `rows.txt` and `rows.py`; `marks.py`.
- `send-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures by sha256; they stay on the lab PC.
