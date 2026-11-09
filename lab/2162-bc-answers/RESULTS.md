# Row 2162-BC results: the buyer and consignee block, for a registered and an unregistered customer, and each customer's mailing name

Run on 9 Nov 2026, 06:49Z to 07:00Z, on TallyPrime 7.2 Silver (tally.exe 1.1.7.2), Gateway on 9001, by the sheet at `lab/2162-bc-sheet` (e2b1ca82), in a sitting of its own on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic, in the registry). Kit: 35 of 35 files equal to `SHA256.txt` by sha256 and size. No Bridge build ran. Every read was answered (20 sends, 46 curl calls, exit 0 each). No dialog appeared after any write or at any other step. The only prompts were the backup's and the restore's, written below with the key that answered each.

## The backup and the restore

- **BK (C5), before the first write.** At 06:51Z, Data > Backup & Restore > Backup. The Backup Data Path was set by Specify Path to a new empty folder on the lab PC (absent before: `evidence/s1-bk-before.txt`, kept local). BRIDGE PILOT LAB was selected alone, then End of List.
  - Prompt "Accept ?": answered `y`. Capture `shots/s1-bk-before-accept.png` (06:51:47Z). Then the Gateway, with no dialog.
  - The file: `TDBK1800_<company number>.001`, 90,051 B, sha256 `49221c2a9bd50cace58c839183932d0e506c64998099d05f125237bf07718447`, written 06:51:48Z. It is the only file in its folder. It stays on the lab PC (`kept-local.txt`).
- **The restore (C6), after step 5, at 06:57Z to 06:59Z.** The backup's sha256 was checked again; it is the only file in its folder. The restore plan and the data folder listing (before and after) were written and stay local.
  - Company > Shut. Then Data > Backup & Restore > Restore: the Restore Data Path is the Tally data folder, and the source was set by Specify Path to the backup folder. The list showed BRIDGE PILOT LAB with its number and 9-Nov-26 (12:21 pm), as planned. Return selected it, and Return on End of List.
  - Capture `shots/s6-before-accept.png` (06:58:50Z). Prompt "Accept ?": answered `y`.
  - Prompt "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.": answered `r` (Replace).
  - Then Select Company, with no other prompt. The company was loaded alone (PILOT typed, the one row, Enter): the Gateway, current date 15-Sep-2026, no dialog (`shots/s6-after-gateway.png`).
- **s9, the check.** Marks byte-equal to s1's (3,394 B, `8a0c4268…`, V 68, M 245). `count5.py --same-row a-s1 a-s9`: equal. C byte-equal to s2's (25,010 B, `873803dd…`, 18 rows, none named BRIDGE Customer RJ).

## What was found

- **Step 1.** Gateway current date Tuesday 15-Sep-2026, which is not before 1-Aug-2026, so the row went on. Period 1-Apr-26 to 31-Mar-27. s1: one COMPANY row, the book's GUID, V 68, M 245.
- **Step 2, the book as found (EXPECT met).**
  - C and Q: 18 LEDGER rows each, 0 named BRIDGE Customer RJ.
  - N-0053 to N-0056: 0 rows each.
  - R-0010: 1 row, included Yes, uncertain No, excluded No.
  - K-walkin: 1 LEDGER row. `MAILINGNAME` absent. One `LEDMAILINGDETAILS.LIST`: APPLICABLEFROM 20260401, MAILINGNAME 'BRIDGE Walk-in', STATE Rajasthan, COUNTRY India, PINCODE and CITY empty. `STATENAME` '', `COUNTRYNAME` absent, `PARTYGSTIN` ''.
- **Step 3, the setup ledger (EXPECT met).** s3m passed count5.py's before-write check. s3 `L-seed.xml`: CREATED 1, every other counter 0. Capture `s3-after.png`: the Gateway, no dialog. s3r:
  - C: 19 rows, one BRIDGE Customer RJ, PARENT Sundry Debtors, ISBILLWISEON No.
  - Q: one registration row, 20260401, Regular, Rajasthan, `08ZZZZZ0000Z3ZO`.
  - K-customer: 1 LEDGER row. `MAILINGNAME` absent, `LEDMAILINGDETAILS.LIST` empty, `STATENAME` '', `COUNTRYNAME` absent, `PARTYGSTIN` ''.
  - Marks: V 68, M 246.
- **Step 4, the four invoices.** Each passed its before-write check. Each write was answered CREATED 1, every other counter 0 except LASTVCHID, with no LINEERROR. Each capture after the write is byte-equal to the plain Gateway (`ca58f95e…`), so no dialog appeared.

  | invoice | customer | block | LASTVCHID | R: included / uncertain / excluded | N | the six in V | COUNTRYOFRESIDENCE | V direct children | marks after |
  |---|---|---|---|---|---|---|---|---|---|
  | 0053 | BRIDGE Customer RJ (Regular, `08ZZZZZ0000Z3ZO`) | with | 45 | Yes / No / No | 1 row, Sales class | as sent | India | 729 | V 69, M 246 |
  | 0054 | the same | without | 46 | Yes / No / No | 1 row, Sales class | each present and empty | India | 729 | V 70 |
  | 0055 | BRIDGE Walk-in (Unregistered/Consumer) | with | 47 | Yes / No / No | 1 row, Sales class | as sent | India | 729 | V 71 |
  | 0056 | the same | without | 48 | Yes / No / No | 1 row, Sales class | each present and empty | India | 729 | V 72 |

  As sent with the block, the six are: BASICBUYERNAME, PARTYMAILINGNAME and CONSIGNEEMAILINGNAME each the party ledger's name; CONSIGNEESTATENAME Rajasthan; CONSIGNEECOUNTRYNAME India; VATDEALERTYPE Regular.

  The two diffs of the read-backs, `0053` against `0054` and `0055` against `0056`, differ only in identity fields: ALTERID, GUID, MASTERID, REFERENCE, VOUCHERKEY, VOUCHERNUMBER and VOUCHERRETAINKEY. Legs 6 -> 6. Neither the six elements nor any status flag is among the differences. The read-back does not fetch the six; V does.
- **Step 5, GSTR-1 for August 2026, read only.** Written down word for word in `evidence/s5-screens.txt`. Against row 2162-M1's read of the same screens an hour before:

  | | 2162-M1 (before the four) | this row (after the four) |
  |---|---|---|
  | Total Vouchers | 39 | 43 |
  | Included in Return | 27 | 31 |
  | Ready for Upload | 27 | 31 |
  | Not Uploaded | (no line) | 2 |
  | Modified in Books After Upload/Export | 27 | 29 |
  | Not Relevant for This Return | 7 | 7 |
  | Uncertain Transactions | 5 | 5 |
  | B2B Invoices | blank | 2, taxable 2,001.60, tax 100.08, invoice 2,102.00 |
  | B2C (Small) - 7 | 27 (2), 13,508.70 | 29 (2), 15,510.30 |
  | of which Rajasthan - 5% | 12,808.70 | 14,810.30 |

  - B2B shows one line: `08ZZZZZ0000Z3ZO`, BRIDGE Customer RJ, Voucher Count 2, taxable 2,001.60, CGST 50.04, SGST 50.04, tax 100.08, invoice 2,102.00. That is `0053` and `0054`, the only invoices to that party, at 1,000.80 each.
  - B2C (Small) Rajasthan - 5% rose by 2,001.60 and the section's count rose from 27 to 29. That is `0055` and `0056`.
  - Uncertain Transactions holds 5, as before. Its two causes name vouchers `5` and `6` (HSN/SAC), and `BP/26-27/0002`, `0003` and `0008` (tax amount). They name none of `0053` to `0056`.
  - No screen opened names `0053` to `0056` by number: B2B lists by party, and B2C (Small) by place and rate. Their placement above is read by count and value.
  - No Enter on a voucher line, no button pressed, no prompt.
- **After the look.** s5m: `--same-row` against s4dr's marks, equal (V 72, M 246). s5r: each read-back against its step 4 answer, `--diff` shows `legs: 6 -> 6` only (EXPECT met).
  - By bytes, s5r's `0053`, `0054` and `0055` read-backs differ from their step 4 answers only in the header's `DESC/CMPINFO` counts (for `0053`: LEDGER 12 -> 45, CURRENCY 1 -> 4, TAXUNIT 1 -> 4, VOUCHERNUMBERSERIES 1 -> 4, VOUCHER 2 -> 8). s5r's counts equal those in s4dr's `0056` read-back, which s5r's `0056` equals byte for byte. The voucher itself is element-equal in each.

## Read against the sheet's Decides

- **Status and placement, with and without the block, for each customer: the same.**
  - Registered: `0053` (with) and `0054` (without) both read included Yes, uncertain No, excluded No. Both are counted in the one B2B line for `08ZZZZZ0000Z3ZO` (Voucher Count 2).
  - Unregistered: `0055` and `0056` both read the same flags. Both are counted under B2C (Small), Rajasthan - 5%.
  - Neither customer's invoices are under Uncertain Transactions.
  - So on this book the block does not change Tally's GST view. PARTIAL: one book, one invoice of each kind, and a placement read by count and value rather than by voucher number.
- **What V shows without the block:** Tally does not fill the six. In `0054` and `0056` each of the six is present and empty, and no value comes from the ledger or the company. An invoice posted without them reads differently from a keyed one: that is how the rehearsal's posts read.
- **K, the mailing name:** neither ledger holds a `MAILINGNAME` element. BRIDGE Walk-in's one `LEDMAILINGDETAILS.LIST` holds MAILINGNAME 'BRIDGE Walk-in', its own name. BRIDGE Customer RJ's is empty. Neither holds a mailing name other than its name, so the ledger name is what Tally itself holds for these two. Partial: two synthetic ledgers.
- **s5m and s5r:** unchanged, so the look did not change the book.
- **A dialog anywhere:** none.

## Captures

Screenshots stay on the lab PC, listed by name, size and sha256 in `SCREENSHOTS.txt`. The capture after each write (`s3-after`, `s4a-after` to `s4d-after`) is 46,676 B, `ca58f95e95d24e9f…`, the same bytes as `s1-gateway.png`: the plain Gateway with no dialog.

## Every answer, by send

Times are UTC, from `evidence/sends-utc.txt`. run.py's log of each send (`log-<label>.txt`) is in local time.

| send | sent (UTC) | answer | bytes | sha256 |
|---|---|---|---|---|
| s1 | 06:50:17.313 | `a-s1-M-marks.xml` | 3,394 | `8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6` |
| s2 | 06:52:00.824 | `a-s2-C-catalogue.xml` | 25,010 | `873803dd0ab3eab2e0a4dc315afb109f40e11fe3b81cf96fc915c04a8df97a6c` |
| s2 | 06:52:00.824 | `a-s2-K-walkin.xml` | 35,798 | `3a7b22c5032563b50d1abc44fa8d8d3aa2a46b7d36115eefa99de733f2dc2bea` |
| s2 | 06:52:00.824 | `a-s2-N-0053.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` |
| s2 | 06:52:00.824 | `a-s2-N-0054.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` |
| s2 | 06:52:00.824 | `a-s2-N-0055.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` |
| s2 | 06:52:00.824 | `a-s2-N-0056.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` |
| s2 | 06:52:00.824 | `a-s2-Q-rates.xml` | 82,392 | `fe04e3fbce168fe349e2c459511edf775ca3e96307dffaa9e37dbe4661018cc6` |
| s2 | 06:52:00.824 | `a-s2-R-0010.xml` | 94,474 | `b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5` |
| s3m | 06:52:17.254 | `a-s3m-M-marks.xml` | 3,394 | `8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6` |
| s3 | 06:52:18.336 | `a-s3-L-seed.xml` | 538 | `1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010` |
| s3r | 06:52:51.261 | `a-s3r-C-catalogue.xml` | 26,224 | `34b72655ca124816ba52b63bf749d059b2db400238a05d6cfbea759453578e81` |
| s3r | 06:52:51.261 | `a-s3r-K-customer.xml` | 34,730 | `68e27d7e72ca5ed2cc55f3e9b902052e6e8aa629e79e41bfd449f426a29561a2` |
| s3r | 06:52:51.261 | `a-s3r-M-marks.xml` | 3,394 | `49bdfb3f21e1db5f7d0c4991efa5683bbb0d9cb180ad8092b961024713ac66a7` |
| s3r | 06:52:51.261 | `a-s3r-Q-rates.xml` | 85,190 | `bb7e2e1ba48c3768332741febccbef6f9483d515e22d188e4487b331d123a100` |
| s4am | 06:53:02.378 | `a-s4am-M-marks.xml` | 3,394 | `49bdfb3f21e1db5f7d0c4991efa5683bbb0d9cb180ad8092b961024713ac66a7` |
| s4a | 06:53:03.361 | `a-s4a-I-0053.xml` | 540 | `fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea` |
| s4ar | 06:53:16.101 | `a-s4ar-M-marks.xml` | 3,396 | `17c5bde53bf5e459f4ae36fce8ee62e8bfe5eb636f47f1970faff44c230a59ed` |
| s4ar | 06:53:16.101 | `a-s4ar-N-0053.xml` | 5,844 | `eb7294bbd01fe69dce719d97fc7a56f15faf8ed31235a3aa92a57fe30ab02ca7` |
| s4ar | 06:53:16.101 | `a-s4ar-R-0053.xml` | 94,496 | `7478c1acf02ed9c8574eee97f0b013dcb99c5410397c014a36cf15438c01badb` |
| s4ar | 06:53:16.101 | `a-s4ar-V-0053.xml` | 52,862 | `34fa689913cceb3575658d2ed08634ebe3cd457cfe4dcd665224ef09d5960bd1` |
| s4bm | 06:53:24.620 | `a-s4bm-M-marks.xml` | 3,396 | `17c5bde53bf5e459f4ae36fce8ee62e8bfe5eb636f47f1970faff44c230a59ed` |
| s4b | 06:53:25.638 | `a-s4b-I-0054.xml` | 540 | `32e7a29b201c773547aa47bc95bd3eda8af83aeb6e35090303b9b27bcd34eec5` |
| s4br | 06:53:29.547 | `a-s4br-M-marks.xml` | 3,396 | `d55cf90ef81a5c355be9e694f8fae317da2e1701fa95feb6f4cc446cb4a7553e` |
| s4br | 06:53:29.547 | `a-s4br-N-0054.xml` | 5,844 | `690ef4fe94ad15a85d061b0e82d73f75c11498182264f9f38f0c3bfea4c5322c` |
| s4br | 06:53:29.547 | `a-s4br-R-0054.xml` | 94,496 | `bb92b78c15ae070b2972f66fa3667ae01660d2928ddef38253078e14ff280557` |
| s4br | 06:53:29.547 | `a-s4br-V-0054.xml` | 52,486 | `97fe7bce260374dc12f3dc67b682cdb70e806115327cf7cf5004590351d2ea04` |
| s4cm | 06:53:45.197 | `a-s4cm-M-marks.xml` | 3,396 | `d55cf90ef81a5c355be9e694f8fae317da2e1701fa95feb6f4cc446cb4a7553e` |
| s4c | 06:53:46.279 | `a-s4c-I-0055.xml` | 540 | `f8384a874e1d8a6be92c2969f94e2887fb700768f6e111b6b3f649cd13ffdc7e` |
| s4cr | 06:53:50.252 | `a-s4cr-M-marks.xml` | 3,396 | `4ac186ec14f33c8066791cc711c6b86445781db3758c5abc3d982280fa4b5b7f` |
| s4cr | 06:53:50.252 | `a-s4cr-N-0055.xml` | 5,844 | `372527c988f78b66dfd3a062b68885051fb523be34932df5f94af31469ee05a8` |
| s4cr | 06:53:50.252 | `a-s4cr-R-0055.xml` | 94,478 | `cacf726fb82fa763a28a84d87ab5a2d102495a4369ff27693dae5cbd83e37102` |
| s4cr | 06:53:50.252 | `a-s4cr-V-0055.xml` | 52,788 | `ccaf748308234b90fbddd2fa93ece97f9bf3fddbc30051bc9cb6b868110f4325` |
| s4dm | 06:53:51.950 | `a-s4dm-M-marks.xml` | 3,396 | `4ac186ec14f33c8066791cc711c6b86445781db3758c5abc3d982280fa4b5b7f` |
| s4d | 06:53:53.102 | `a-s4d-I-0056.xml` | 540 | `e6540f6534095f5608f3c59f802c1f40a3ae13529a7e9041847b460f03c32fec` |
| s4dr | 06:53:57.046 | `a-s4dr-M-marks.xml` | 3,396 | `419f6b56d7e3f2add5ccb53dede6f88457ffc8a20399d3d60a4786a0d48e062e` |
| s4dr | 06:53:57.046 | `a-s4dr-N-0056.xml` | 5,844 | `579039081619cebc2be657e05401fe67bc66f619872286dcb18018b950838fb2` |
| s4dr | 06:53:57.046 | `a-s4dr-R-0056.xml` | 94,478 | `607f3dd30879454c7307d7b6edbb8ea80d74bce591ac0f3f89464c8a1d004e22` |
| s4dr | 06:53:57.046 | `a-s4dr-V-0056.xml` | 52,436 | `90cf42cdf46833c562de9c0df1a92cd89f95fcca746e55c73ba120d41018e0c3` |
| s5m | 06:57:20.096 | `a-s5m-M-marks.xml` | 3,396 | `419f6b56d7e3f2add5ccb53dede6f88457ffc8a20399d3d60a4786a0d48e062e` |
| s5r | 06:57:21.396 | `a-s5r-R-0053.xml` | 94,496 | `78792bb9156b969e953925b5f2a8ddee4d715fb274ebb5f270d32fcbf52c878e` |
| s5r | 06:57:21.396 | `a-s5r-R-0054.xml` | 94,496 | `16553e1b40119e5dd039c3abb610ab729d8e501e766a83e46d157a4b62c3055e` |
| s5r | 06:57:21.396 | `a-s5r-R-0055.xml` | 94,478 | `f0fa1c1bfc14a4327286ddc687a815badf2932a8e2df904b0607cccb79a88855` |
| s5r | 06:57:21.396 | `a-s5r-R-0056.xml` | 94,478 | `607f3dd30879454c7307d7b6edbb8ea80d74bce591ac0f3f89464c8a1d004e22` |
| s9 | 06:59:33.389 | `a-s9-C-catalogue.xml` | 25,010 | `873803dd0ab3eab2e0a4dc315afb109f40e11fe3b81cf96fc915c04a8df97a6c` |
| s9 | 06:59:33.389 | `a-s9-M-marks.xml` | 3,394 | `8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6` |

## count5.py, whole

Every `count5.py` output of the row, in send order, as written to `evidence/`. A VOUCHERKEY value is shown here as `<15-digit voucher key>`; the value itself is in the answers.

### `evidence/count-s1.txt`

```
== a-s1-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  COMPANY rows: 1
```

### `evidence/count-s2.txt`

```
== a-s2-C-catalogue.xml: 25010 bytes
  STATUS '1'
  LEDGER rows: 18; named 'BRIDGE Customer RJ': 0
== a-s2-K-walkin.xml: 35798 bytes
  STATUS '1'
  LEDGER rows: 1
   NAME 'BRIDGE Walk-in' MAILINGNAME <absent>
   LEDMAILINGDETAILS.LIST: APPLICABLEFROM '20260401'; PINCODE ''; MAILINGNAME 'BRIDGE Walk-in'; STATE 'Rajasthan'; CITY ''; COUNTRY 'India'
   STATENAME ''
   COUNTRYNAME <absent>
   PARTYGSTIN ''
== a-s2-N-0053.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0054.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0055.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0056.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-Q-rates.xml: 82392 bytes
  STATUS '1'
  LEDGER rows: 18; named 'BRIDGE Customer RJ': 0
== a-s2-R-0010.xml: 94474 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0010'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   ALTERID '17'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
```

### `evidence/count-s3m.txt`

```
== a-s3m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  COMPANY rows: 1
```

### `evidence/before-write-s3.txt`

```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### `evidence/count-s3.txt`

```
== a-s3-L-seed.xml: 538 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
```

### `evidence/count-s3r.txt`

```
== a-s3r-C-catalogue.xml: 26224 bytes
  STATUS '1'
  LEDGER rows: 19; named 'BRIDGE Customer RJ': 1
   PARENT 'Sundry Debtors'; ISBILLWISEON 'No'
== a-s3r-K-customer.xml: 34730 bytes
  STATUS '1'
  LEDGER rows: 1
   NAME 'BRIDGE Customer RJ' MAILINGNAME <absent>
   LEDMAILINGDETAILS.LIST: empty
   STATENAME ''
   COUNTRYNAME <absent>
   PARTYGSTIN ''
== a-s3r-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  COMPANY rows: 1
== a-s3r-Q-rates.xml: 85190 bytes
  STATUS '1'
  LEDGER rows: 19; named 'BRIDGE Customer RJ': 1
   PARENT 'Sundry Debtors'; PARTYGSTIN <absent>; STATENAME <absent>
   LEDGSTREGDETAILS.LIST: APPLICABLEFROM '20260401'; GSTREGISTRATIONTYPE 'Regular'; PLACEOFSUPPLY 'Rajasthan'; GSTIN '08ZZZZZ0000Z3ZO'
   LEDGSTREGDETAILS.LIST elements: 1 of which non-empty: 1
```

### `evidence/count-s4am.txt`

```
== a-s4am-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  COMPANY rows: 1
```

### `evidence/before-write-s4a.txt`

```
== a-s4am-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### `evidence/count-s4a.txt`

```
== a-s4a-I-0053.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '45'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
```

### `evidence/count-s4ar.txt`

```
== a-s4ar-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  COMPANY rows: 1
== a-s4ar-N-0053.xml: 5844 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0053' BRIDGEVCHISSALES 'Yes'
== a-s4ar-R-0053.xml: 94496 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0053'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0053'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Customer RJ'
   PARTYGSTIN '08ZZZZZ0000Z3ZO'
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Regular'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   ALTERID '69'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Customer RJ' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
== a-s4ar-V-0053.xml: 52862 bytes
  STATUS '1'
  VOUCHER rows: 1
   VOUCHERNUMBER 'BP/26-27/0053'
   VOUCHERTYPENAME 'BRIDGE Sales'
   PARTYLEDGERNAME 'BRIDGE Customer RJ'
   BASICBUYERNAME 'BRIDGE Customer RJ'
   PARTYMAILINGNAME 'BRIDGE Customer RJ'
   CONSIGNEEMAILINGNAME 'BRIDGE Customer RJ'
   CONSIGNEESTATENAME 'Rajasthan'
   CONSIGNEECOUNTRYNAME 'India'
   VATDEALERTYPE 'Regular'
   COUNTRYOFRESIDENCE 'India'
   PARTYGSTIN '08ZZZZZ0000Z3ZO'
   GSTREGISTRATIONTYPE 'Regular'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   direct children: 729
```

### `evidence/count-s4bm.txt`

```
== a-s4bm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  COMPANY rows: 1
```

### `evidence/before-write-s4b.txt`

```
== a-s4bm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### `evidence/count-s4b.txt`

```
== a-s4b-I-0054.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '46'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
```

### `evidence/count-s4br.txt`

```
== a-s4br-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '246'
  COMPANY rows: 1
== a-s4br-N-0054.xml: 5844 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0054' BRIDGEVCHISSALES 'Yes'
== a-s4br-R-0054.xml: 94496 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0054'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0054'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Customer RJ'
   PARTYGSTIN '08ZZZZZ0000Z3ZO'
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Regular'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   ALTERID '70'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Customer RJ' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
== a-s4br-V-0054.xml: 52486 bytes
  STATUS '1'
  VOUCHER rows: 1
   VOUCHERNUMBER 'BP/26-27/0054'
   VOUCHERTYPENAME 'BRIDGE Sales'
   PARTYLEDGERNAME 'BRIDGE Customer RJ'
   BASICBUYERNAME ''
   PARTYMAILINGNAME ''
   CONSIGNEEMAILINGNAME ''
   CONSIGNEESTATENAME ''
   CONSIGNEECOUNTRYNAME ''
   VATDEALERTYPE ''
   COUNTRYOFRESIDENCE 'India'
   PARTYGSTIN '08ZZZZZ0000Z3ZO'
   GSTREGISTRATIONTYPE 'Regular'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   direct children: 729
```

### `evidence/count-s4cm.txt`

```
== a-s4cm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '246'
  COMPANY rows: 1
```

### `evidence/before-write-s4c.txt`

```
== a-s4cm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### `evidence/count-s4c.txt`

```
== a-s4c-I-0055.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '47'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
```

### `evidence/count-s4cr.txt`

```
== a-s4cr-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '246'
  COMPANY rows: 1
== a-s4cr-N-0055.xml: 5844 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0055' BRIDGEVCHISSALES 'Yes'
== a-s4cr-R-0055.xml: 94478 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0055'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0055'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   ALTERID '71'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
== a-s4cr-V-0055.xml: 52788 bytes
  STATUS '1'
  VOUCHER rows: 1
   VOUCHERNUMBER 'BP/26-27/0055'
   VOUCHERTYPENAME 'BRIDGE Sales'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   BASICBUYERNAME 'BRIDGE Walk-in'
   PARTYMAILINGNAME 'BRIDGE Walk-in'
   CONSIGNEEMAILINGNAME 'BRIDGE Walk-in'
   CONSIGNEESTATENAME 'Rajasthan'
   CONSIGNEECOUNTRYNAME 'India'
   VATDEALERTYPE 'Regular'
   COUNTRYOFRESIDENCE 'India'
   PARTYGSTIN ''
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   direct children: 729
```

### `evidence/count-s4dm.txt`

```
== a-s4dm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '246'
  COMPANY rows: 1
```

### `evidence/before-write-s4d.txt`

```
== a-s4dm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### `evidence/count-s4d.txt`

```
== a-s4d-I-0056.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '48'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
```

### `evidence/count-s4dr.txt`

```
== a-s4dr-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '246'
  COMPANY rows: 1
== a-s4dr-N-0056.xml: 5844 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0056' BRIDGEVCHISSALES 'Yes'
== a-s4dr-R-0056.xml: 94478 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0056'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0056'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030'
   ALTERID '72'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
== a-s4dr-V-0056.xml: 52436 bytes
  STATUS '1'
  VOUCHER rows: 1
   VOUCHERNUMBER 'BP/26-27/0056'
   VOUCHERTYPENAME 'BRIDGE Sales'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   BASICBUYERNAME ''
   PARTYMAILINGNAME ''
   CONSIGNEEMAILINGNAME ''
   CONSIGNEESTATENAME ''
   CONSIGNEECOUNTRYNAME ''
   VATDEALERTYPE ''
   COUNTRYOFRESIDENCE 'India'
   PARTYGSTIN ''
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   direct children: 729
```

### `evidence/diff-s4-0053-0054.txt`

```
== --diff a-s4ar-R-0053.xml a-s4br-R-0054.xml
   ALTERID: '69' -> '70'
   GUID: '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d' -> '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   MASTERID: '45' -> '46'
   REFERENCE: 'BP/26-27/0053' -> 'BP/26-27/0054'
   VOUCHERKEY: '<15-digit voucher key>' -> '<15-digit voucher key>'
   VOUCHERNUMBER: 'BP/26-27/0053' -> 'BP/26-27/0054'
   VOUCHERRETAINKEY: '129' -> '133'
   legs: 6 -> 6
```

### `evidence/diff-s4-0055-0056.txt`

```
== --diff a-s4cr-R-0055.xml a-s4dr-R-0056.xml
   ALTERID: '71' -> '72'
   GUID: '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f' -> '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030'
   MASTERID: '47' -> '48'
   REFERENCE: 'BP/26-27/0055' -> 'BP/26-27/0056'
   VOUCHERKEY: '<15-digit voucher key>' -> '<15-digit voucher key>'
   VOUCHERNUMBER: 'BP/26-27/0055' -> 'BP/26-27/0056'
   VOUCHERRETAINKEY: '137' -> '141'
   legs: 6 -> 6
```

### `evidence/count-s5m.txt`

```
== a-s5m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '246'
  COMPANY rows: 1
```

### `evidence/same-row-s5m.txt`

```
== a-s5m-M-marks.xml against a-s4dr-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'246'")
  equal
```

### `evidence/count-s5r.txt`

```
== a-s5r-R-0053.xml: 94496 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0053'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0053'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Customer RJ'
   PARTYGSTIN '08ZZZZZ0000Z3ZO'
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Regular'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   ALTERID '69'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Customer RJ' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
== a-s5r-R-0054.xml: 94496 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0054'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0054'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Customer RJ'
   PARTYGSTIN '08ZZZZZ0000Z3ZO'
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Regular'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   ALTERID '70'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Customer RJ' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
== a-s5r-R-0055.xml: 94478 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0055'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0055'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   ALTERID '71'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
== a-s5r-R-0056.xml: 94478 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0056'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0056'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030'
   ALTERID '72'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
```

### `evidence/diff-s5r.txt`

```
== --diff a-s4ar-R-0053.xml a-s5r-R-0053.xml
   legs: 6 -> 6
== --diff a-s4br-R-0054.xml a-s5r-R-0054.xml
   legs: 6 -> 6
== --diff a-s4cr-R-0055.xml a-s5r-R-0055.xml
   legs: 6 -> 6
== --diff a-s4dr-R-0056.xml a-s5r-R-0056.xml
   legs: 6 -> 6
```

### `evidence/count-s9.txt`

```
== a-s9-C-catalogue.xml: 25010 bytes
  STATUS '1'
  LEDGER rows: 18; named 'BRIDGE Customer RJ': 0
== a-s9-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  COMPANY rows: 1
```

### `evidence/same-row-s9.txt`

```
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

## What is here and what stayed on the lab PC

Answers are here whole, with their logs, under the rule the run lead set for row 2162-M1 on 9 Nov:
- an answer may carry a voucher's 15-digit VOUCHERKEY value;
- it may not carry the company number, or any other run of 10 or more digits outside a hash.

The four V reads (`a-s4ar-V-0053` to `a-s4dr-V-0056`) also carry, in `REMOTEGUID`, the all-digit remote id the invoice was sent with. That id is the same value as in the kit's `I-<n>.xml` on the sheet branch. By that rule they stay on the lab PC, by size and sha256, until the run lead's word.

`kept-local.txt` lists everything kept on the lab PC, with the reason:
- the backup;
- the restore plan;
- the folder listings;
- the process and port listings;
- any answer kept by the rule above;
- the one-write helper `w.sh`;
- the stage script.

In the pushed copies of the lab's own text, the voucher keys in count5.py's two `--diff` lines are shown as `<15-digit voucher key>`. `substitutions.txt` records each such change.
