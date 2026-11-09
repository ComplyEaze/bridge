# Row 2162-M1 results: the month reads on the pilot and e-invoice books, and Tally's GSTR-1 placement of `BP/26-27/0010`

Run on 9 Nov 2026, 06:40Z to 06:49Z, on TallyPrime 7.2 Silver (tally.exe 1.1.7.2), Gateway on 9001, by the sheet at `lab/2162-m1-sheet` (eaf2a4fc). Kit: 34 of 34 files equal to `SHA256.txt` by sha256 and size. No Bridge build ran. Nothing was written: no import, no backup, no restore. Every read was answered (54 sends, curl exit 0 each). No dialog or prompt appeared at any step.

- **Part A, `BRIDGE PILOT LAB`:** the Gateway's current date 15-Sep-2026, period 1-Apr-26 to 31-Mar-27. Marks a1 = aend = bend: V 68, M 245, byte-equal (3,394 B). a2 read 41 rows, 117,992 B, as the sheet expected. Every month pair is byte-equal. August holds 40 rows and September 1. The other ten months are 0 rows, all with the same bytes. The twelve months' rows equal a2's element for element, in order.
- **Part B, the same book, GSTR-1 for August 2026, read only:** `BP/26-27/0010` is counted on one line under B2C (Small) Invoices - 7, in the register Rajasthan (5%): taxable 1,000.80, CGST 25.02, SGST 25.02, tax 50.04, invoice 1,051.00. The HSN Summary shows one line for 998313 and one for 998314. Each line pools every August invoice with that code. The screen has one list (B2C Supplies) and no B2B list.
- **Part C, `BRIDGE EINVOICE LAB`:** the Gateway's current date 3-Apr-2025, period 1-Apr-25 to 31-Mar-26. Marks c0 = cend: V 3, M 215, byte-equal (3,398 B). April holds 3 rows: Sales `1` with an IRN of length 64, Sales `2` with the IRN empty, and the Payment. The other eleven months are 0 rows, with the same bytes as part A's 0-row months. Every pair is byte-equal.

## One line per send

Times are UTC, from `evidence/sends-utc.txt`. run.py's own log for each send (`log-<label>.txt`) is in local time. The rows and counts are from `evidence/count-<label>.txt`. V and M are ALTVCHID and ALTMSTID. The year and month answers are 0-row empty collections unless rows are shown.

| send | file | sent (UTC) | bytes | sha256 | rows | Sales-class Yes / No | types | dates |
|---|---|---|---|---|---|---|---|---|
| a1 | `A-M-marks.xml` | 06:41:51.807 | 3,394 | `8a0c42682c958e48…` | marks: 1 COMPANY row, the book's GUID yes, V 68, M 245 |  |  |  |
| a2 | `A-Y0-year.xml` | 06:41:52.658 | 117,992 | `f8ccb3ca4631c5af…` | 41 | Yes 35; No 6 | BRIDGE Sales 33; Receipt 4; Purchase 2; BRIDGE Sales Two 1; BRIDGE Sales Z1 1 | 20260801 to 20260915 |
| a01a | `A-Y0-2026-04.xml` | 06:42:02.238 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a01b | `A-Y0-2026-04.xml` | 06:42:03.157 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a02a | `A-Y0-2026-05.xml` | 06:42:04.103 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a02b | `A-Y0-2026-05.xml` | 06:42:04.979 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a03a | `A-Y0-2026-06.xml` | 06:42:06.122 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a03b | `A-Y0-2026-06.xml` | 06:42:06.997 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a04a | `A-Y0-2026-07.xml` | 06:42:08.024 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a04b | `A-Y0-2026-07.xml` | 06:42:10.393 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a05a | `A-Y0-2026-08.xml` | 06:42:11.649 | 115,168 | `f67cf46f57e7f4ae…` | 40 | Yes 34; No 6 | BRIDGE Sales 32; Receipt 4; Purchase 2; BRIDGE Sales Two 1; BRIDGE Sales Z1 1 | 20260801 to 20260822 |
| a05b | `A-Y0-2026-08.xml` | 06:42:13.085 | 115,168 | `f67cf46f57e7f4ae…` | 40 | Yes 34; No 6 | BRIDGE Sales 32; Receipt 4; Purchase 2; BRIDGE Sales Two 1; BRIDGE Sales Z1 1 | 20260801 to 20260822 |
| a06a | `A-Y0-2026-09.xml` | 06:42:14.634 | 5,850 | `97e447efb4d0670b…` | 1 | Yes 1 | BRIDGE Sales 1 | 20260915 to 20260915 |
| a06b | `A-Y0-2026-09.xml` | 06:42:15.795 | 5,850 | `97e447efb4d0670b…` | 1 | Yes 1 | BRIDGE Sales 1 | 20260915 to 20260915 |
| a07a | `A-Y0-2026-10.xml` | 06:42:16.994 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a07b | `A-Y0-2026-10.xml` | 06:42:18.123 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a08a | `A-Y0-2026-11.xml` | 06:42:19.174 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a08b | `A-Y0-2026-11.xml` | 06:42:20.230 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a09a | `A-Y0-2026-12.xml` | 06:42:21.259 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a09b | `A-Y0-2026-12.xml` | 06:42:22.319 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a10a | `A-Y0-2027-01.xml` | 06:42:23.412 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a10b | `A-Y0-2027-01.xml` | 06:42:24.447 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a11a | `A-Y0-2027-02.xml` | 06:42:25.481 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a11b | `A-Y0-2027-02.xml` | 06:42:26.499 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a12a | `A-Y0-2027-03.xml` | 06:42:27.468 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| a12b | `A-Y0-2027-03.xml` | 06:42:28.477 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| aend | `A-M-marks.xml` | 06:42:36.740 | 3,394 | `8a0c42682c958e48…` | marks: 1 COMPANY row, the book's GUID yes, V 68, M 245 |  |  |  |
| bend | `A-M-marks.xml` | 06:46:41.706 | 3,394 | `8a0c42682c958e48…` | marks: 1 COMPANY row, the book's GUID yes, V 68, M 245 |  |  |  |
| c0 | `C-M-marks.xml` | 06:48:01.474 | 3,398 | `41aad235aec1eeb8…` | marks: 1 COMPANY row, the book's GUID yes, V 3, M 215 |  |  |  |
| c01a | `C-Y0-2025-04.xml` | 06:48:02.620 | 11,502 | `57ef938df392c231…` | 3 | Yes 2; No 1 | Sales 2; Payment 1 | 20250403 to 20250403 |
| c01b | `C-Y0-2025-04.xml` | 06:48:04.103 | 11,502 | `57ef938df392c231…` | 3 | Yes 2; No 1 | Sales 2; Payment 1 | 20250403 to 20250403 |
| c02a | `C-Y0-2025-05.xml` | 06:48:05.353 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c02b | `C-Y0-2025-05.xml` | 06:48:06.425 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c03a | `C-Y0-2025-06.xml` | 06:48:07.562 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c03b | `C-Y0-2025-06.xml` | 06:48:08.619 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c04a | `C-Y0-2025-07.xml` | 06:48:09.714 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c04b | `C-Y0-2025-07.xml` | 06:48:10.794 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c05a | `C-Y0-2025-08.xml` | 06:48:11.899 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c05b | `C-Y0-2025-08.xml` | 06:48:12.936 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c06a | `C-Y0-2025-09.xml` | 06:48:14.045 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c06b | `C-Y0-2025-09.xml` | 06:48:15.270 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c07a | `C-Y0-2025-10.xml` | 06:48:16.471 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c07b | `C-Y0-2025-10.xml` | 06:48:17.525 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c08a | `C-Y0-2025-11.xml` | 06:48:18.661 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c08b | `C-Y0-2025-11.xml` | 06:48:19.641 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c09a | `C-Y0-2025-12.xml` | 06:48:20.745 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c09b | `C-Y0-2025-12.xml` | 06:48:21.824 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c10a | `C-Y0-2026-01.xml` | 06:48:22.908 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c10b | `C-Y0-2026-01.xml` | 06:48:23.974 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c11a | `C-Y0-2026-02.xml` | 06:48:25.037 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c11b | `C-Y0-2026-02.xml` | 06:48:26.119 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c12a | `C-Y0-2026-03.xml` | 06:48:27.347 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| c12b | `C-Y0-2026-03.xml` | 06:48:29.038 | 3,022 | `947fbf369d0fcf74…` | 0 |  |  |  |
| cend | `C-M-marks.xml` | 06:48:30.324 | 3,398 | `41aad235aec1eeb8…` | marks: 1 COMPANY row, the book's GUID yes, V 3, M 215 |  |  |  |

## The comparisons

From `count.py --compare A` (`evidence/compare-A.txt`) and `--compare C` (`evidence/compare-C.txt`), whole in those files.

| | part A, pilot (1 Apr 2026 to 31 Mar 2027) | part C, e-invoice (1 Apr 2025 to 31 Mar 2026) |
|---|---|---|
| opening against closing marks (GUID, ALTVCHID, ALTMSTID) | the same (a1 = aend; bend = aend too) | the same (c0 = cend) |
| each month's second send, element for element | equal, 12 of 12 | equal, 12 of 12 |
| rows dated outside their month | 0 | 0 |
| months answered | 12 of 12, distinct 12 | 12 of 12, distinct 12 |
| rows over the twelve first sends | 41 (August 40, September 1) | 3 (April) |
| distinct sha256 of the 0-row month answers | 1 (`947fbf36…`, 3,022 B) | 1, the same `947fbf36…` as part A |
| the months' rows against a2's | 41 and 41; equal element for element in order: yes; as a multiset: yes | (not asked) |

On both books, every pair of sends of one month was byte-equal, not only element-equal: each pair has one sha256 in the table above.

Sales-class rows (count.py): August 34, with 34 distinct folded numbers and no row sharing a folded number with another. September 1 (`BP/26-27/0020`, 15 Sep 2026). The year 35, with 35 distinct folded numbers. In e-invoice April, Sales `1` has an IRN of length 64 and Sales `2` has an IRN of length 0. In every pilot row, IRN, IRNACKNO and IRNACKDATE each occur once and are empty.

## Part B: Tally's GSTR-1 screens

Every screen is written down word for word in `evidence/b-screens.txt`, with its capture named. In short:

- **Steps 5 and 6.** Go To `GSTR-1` opened directly on GST Registration `08ZZZZZ0000Z1ZQ`, with no list of registrations. The period was set by Alt+F2 to 1-8-2026 .. 31-8-2026. Summary:

  | | |
  |---|---|
  | Total Vouchers | 39 |
  | Included in Return | 27 |
  | Ready for Upload | 27 |
  | Modified in Books After Upload/Export | 27 |
  | Not Relevant for This Return | 7 |
  | Uncertain Transactions (Corrections needed) | 5 |

  B2B Invoices: blank. B2C (Small) Invoices - 7: 27 (2), taxable 13,508.70, CGST 383.26, SGST 383.24, tax 766.50. Total: 13,508.70 / 383.26 / 383.24 / 766.50.
- **Step 7, B2C (Small) Invoices - 7.** The screen names no vouchers. It shows one more level, by place and rate:

  | | taxable | CGST | SGST | tax |
  |---|---|---|---|---|
  | Rajasthan - 5% | 12,808.70 | 320.26 | 320.24 | 640.50 |
  | Rajasthan - 18% | 700.00 | 63.00 | 63.00 | 126.00 |

  Enter on Rajasthan - 5% opened the register "Vouchers of : Rajasthan (5%)": 27 vouchers, total taxable 12,808.70, invoice 14,275.36. The line that counts `BP/26-27/0010` is one line:

  | Date | Particulars | Vch Type | Vch No. | Taxable | CGST | SGST | Tax | Invoice |
  |---|---|---|---|---|---|---|---|---|
  | 1-Aug-26 | BRIDGE Walk-in | BRIDGE Sales | BP/26-27/0010 | 1,000.80 | 25.02 | 25.02 | 50.04 | 1,051.00 |

  No voucher line was entered. Esc twice returned to the summary.
- **Step 8, HSN Summary - 12 (B2B - B2C Supplies).** One list, headed B2C Supplies. There is no B2B heading or line, so no second list was opened. Nine lines, each with HSN/SAC, UQC, total, rate, taxable value and tax; the Quantity column is empty on every line. The two codes of `0010`:

  | HSN/SAC | total | rate | taxable | CGST | SGST | tax |
  |---|---|---|---|---|---|---|
  | 998313 | 11,137.40 | 5 % | 10,607.00 | 265.21 | 265.19 | 530.40 |
  | 998314 | 1,366.70 | 5 % | 1,301.60 | 32.55 | 32.55 | 65.10 |

  Sub Total and Total: 14,275.20 / 13,508.70 / 383.26 / 383.24 / 766.50. Each line pools every August B2C invoice that uses its code, so the screen does not show `0010`'s own share of either line.
- **Steps 9 and 10.** Esc one screen at a time returned to the Gateway, current date 15-Sep-2026 as at step 0. bend was byte-equal to aend. The company was shut by Company > Shut.

## Read against the sheet's Decides

- **The replay fixtures (A, C).** Each pair of sends was equal and the months tile the year on both books. By the sheet, the first send of each month is the build's month fixture for that book. Where those answers are is under "Kept on the lab PC" below.
- **The months against the year (A).** The twelve months' rows equal a2's, element for element and in order. On this book, the month reads see what the year read sees.
- **The 0-row answer (A, C).** Every 0-row month on each book has the same bytes, and the two books share that answer (`947fbf36…`, 3,022 B). One fixture serves every 0-row month on both books.
- **The HSN split (B, ADR row S6).** The screen shows two lines, one per heading code: 998313 and 998314 are on separate lines, each at 5%. Each line's value is pooled over every August invoice with that code, so the screen alone cannot show that `0010`'s 998313 ledger lands only in the 998313 line, and its 998314 ledger only in the 998314 line. Partial: B2C Supplies list only, no B2B list. It is one invoice and a screen read, with the values pooled across invoices rather than given per invoice.
- **The B2C placement (B, ADR row S14).** `0010` is counted under B2C (Small) Invoices - 7, place of supply Rajasthan at 5%, on one line with its whole taxable value 1,000.80. Partial: a screen read.
- **A dialog anywhere:** none.

## Captures and kept on the lab PC

The captures stay on the lab PC. They are listed by name, size and sha256 in `SCREENSHOTS.txt`.

Every answer is here whole, with its log: the marks, the year, and every month. The seven answers that hold rows each carry a voucher's 15-digit VOUCHERKEY:

- a2;
- a05a and a05b (August);
- a06a and a06b (September);
- c01a and c01b (e-invoice April).

On the run lead's word of 9 Nov they go whole: master already holds captured fixtures with such keys, and both book GUIDs are in the synthetic-uuid registry. The stage script checked each of the seven for the pilot book's company number, and for any run of 10 or more digits outside a VOUCHERKEY value or a hash, and found neither.

Kept on the lab PC, by size and sha256 in `kept-local.txt`: the process and port listings of step 0, and the stage script.
