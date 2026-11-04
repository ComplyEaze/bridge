# Row 2162-S6: the year's number read on BRIDGE CORPUS DENSE, as built (Y0) and with `$$IsSales` as a filter term (Y1): results

**First line: every EXPECT met, no dialog, no stop.**
- Every Y1 answer is byte-equal to its window's Y0 answer, and count6 finds it element-for-element equal to Y0's Sales-class rows (day, month and year). The filter serves.
- Every voucher this book holds in the year is Sales class: 29,900 of 29,900. So this book does not measure the cut.
- The year's answer is 83,742,646 B, above the transport's 32 MiB. Y0-year took 44.06 s and Y1-year 13.06 s (twice), both above its 20 s.
- The control read N1 (24.81 s) equals Y0-year's row numbered `1`. (verified)

Sheet `lab/2162-s6/ROW.md` at 49fa4c17; SHA256.txt 14 of 14 equal. Reads only, with no Bridge build. Every send is in `run/log-*.txt` and `notes.txt` (UTC). count6.py's whole output after each step is in `evidence/count6-after-*.txt`.

## Each send

| send | file | rows | bytes | sha256 | curl first byte / total (s) | wall (s) |
|---|---|---:|---:|---|---:|---:|
| m1 | M-marks | – | 3,406 | fcc9086d… | 0.010501 / 0.011781 | 0.34 |
| s1 | Y0-day | 82 | 231,386 | 9534412b… | 0.686588 / 0.688809 | 0.77 |
| s2 | Y1-day | 82 | 231,386 | 9534412b… (= s1) | 0.182039 / 0.183996 | 0.26 |
| s3 | Y0-month | 2,430 | 6,792,322 | 9262740a… | 8.308709 / 8.376538 | 8.65 |
| s4 | Y1-month | 2,430 | 6,792,322 | 9262740a… (= s3) | 3.036744 / 3.082395 | 3.40 |
| s5 | N1-year | 1 | 5,804 | 0f848678… | 24.807076 / 24.809086 | 24.90 |
| s6 | Y0-year | 29,900 | 83,742,646 | 1db925db… | 43.829520 / 44.063520 | 45.15 |
| s7 | Y1-year | 29,900 | 83,742,646 | 1db925db… (= s6) | 12.775252 / 13.062260 | 14.00 |
| s8 | Y1-year again | 29,900 | 83,742,646 | 1db925db… (= s7) | 12.793392 / 13.062791 | 14.03 |
| m2 | M-marks | – | 3,406 | fcc9086d… (= m1) | 0.004499 / 0.004922 | 0.01 |

Whole sha256 values are in `evidence/count6-after-m2.txt`. (verified)

**Row bytes:**
- day: total 227,376; rest 4,010; per row 2,766 to 2,774, mean 2,773.
- year: total 83,380,820; rest 361,826; per row 2,766 to 2,792, mean 2,789.

**Classes and counts:** in every Y answer, BRIDGEVCHISSALES is Yes for every row, VOUCHERTYPENAME is Sales for every row and ISCANCELLED is No for every row. Dates run 20250401 (day), 20250401 to 20250430 (30 days, month) and 20250401 to 20260331 (365 days, year). Folded duplicates: 0 in each. (verified)

**count6.py's comparison lines:**
- s2 against s1's Sales-class rows: 82 and 82, element for element equal: yes.
- s4 against s3's: 2,430 and 2,430, equal: yes.
- s7 against s6's: 29,900 and 29,900, equal: yes.
- s8 against s6's: equal: yes.
- s7 against s8: the same rows: yes.
- s5 N1-year against s6's rows numbered `1`: 1 and 1, equal: yes. (verified)

## Ladder decisions

- s6 was sent: s3's curl total was 8.376538 s, × 12 = 100.5 s, at most 300 s.
- s7 was sent: s4's curl total was 3.082395 s, × 12 = 37.0 s, at most 300 s.
- s8 is s7 again, s7 being the last year read sent. (verified)

## The rest of the Record

- **m1 and m2:** 3,406 B each, sha256 fcc9086dfa112813c447d23e98eb4500c9a1badbe983d3f8d0c0ceff11dc8430, the book's GUID, V 29,900, M 328, byte-equal (and equal to 1029-T's m1). (verified)
- **The folder:** 20 files shut before, 23 loaded, 20 shut after. (verified)
- **Tally:** TallyPrime 7.2 Silver. Tally's Current Date Tuesday 31-Mar-2026; the PC's date 4-Nov-2026. (verified)
- **The P7 guard (s2):**
  - One send, watched.
  - Tally's window was captured as soon as the answer was written and showed the Gateway with no dialog.
  - The answer is a collection of rows.
- **Tally's window after the other sends:** the Gateway each time, no dialog. After s6, s7 and s8 the capture differs only in focus (Tally was not the foreground window). (verified)
- **The four desktop-client connector servers** were left running and untouched, on the run lead's word. This session made no ComplyEaze Bridge connector call from step 0 to the close.
  - The desktop client's default Bridge data folder's egress log read 1,776,853 B and 868 lines (last modified 2 Nov) before m1 and again after the last send: unchanged.
  - The two servers run from a lab build keep their data folders out of sight of the lab without touching their processes, so those were not read. (verified)
- **Close:** Shut Company by Tally's own menu (Alt+K, Down ×4 to "SHut", Enter, Enter on BRIDGE CORPUS DENSE). No company loaded; one tally.exe owns 9001. (verified)

## Kept on the lab PC

- The three year answers (s6, s7 and s8), each 83,742,646 B, above the sheet's 10 MB, are listed in `kept-local.txt` by name, size and sha256 until the run lead says.
- Every other captured answer is pushed whole, unless `kept-local.txt` names it with a reason.
- Screens are listed in `SCREENSHOTS.txt`. (verified)
