# Results: #2145, the census spanned to (0, mark] on four synthetic books, 30 Oct 2026 (UTC). **Every request answered 200 with bytes equal to CONTENT-LENGTH, every `GET /status` after it answered 200, no dialog, and every part's marks were the same at its end as at its start. PILOT LAB (1 window), GST LEDGER LAB (14 windows) and STOCK LAB (1 window): every verdict that ran holds, and every spanned answer is byte-equal to its control. AMEND LAB: the 17 June window (M 1790) holds on all four verdicts. The 1 April window recorded at M 1469 does not hold A or B, as the sheet expects. The 1 April window recorded at M 1685 and the 3 April window (M 1676) do not hold A or B, where the sheet expected them to. The book has moved past both records: its voucher mark now reads 1829 (the highest M is 1790). The 3 April control's 4 rows all carry ALTERIDs above 1676, so its spanned answer is empty, and verdict D does not hold for it either.**

## By window (`compare2145.py`; each output whole in `evidence/compare-<window>.txt`)
| part | window | M | control rows above M | spanned = control (bytes) | A | B | C | D |
|---|---|---|---|---|---|---|---|---|
| A | 20260403 (`a-wa1`) | 1676 | 4 | no (the spanned answer holds no row) | does not hold | does not hold | holds | **does not hold** (the spanned answer is empty) |
| A | 20260401 (`a-d3batch`) | 1469 | 3 | no | does not hold (expected) | does not hold (expected) | holds | holds |
| A | 20260401 (`a-d3cancel`) | 1685 | 0 | yes | **does not hold** | **does not hold** | holds | holds |
| A | 20260617 (`a-l1`) | 1790 | 0 | yes | holds | holds | holds | holds |
| P | 20260401 to 20270331 (`p-audit`) | 68 | 0 | yes | holds | holds | holds | holds |
| G | 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 16, 17 Nov 2025 (14 windows) | 64 | 0 each | yes, each | holds, each | holds, each | holds, each | holds, each |
| S | 20250421 (`s-0421`) | 3 | 0 | yes | skipped (no recorded answer) | skipped | holds | holds: one row, ALTERID 3, dated 20250421 |

- **Expected and seen:** A's 1 April window at M 1469 does not hold A or B, as the sheet expects (3 of the control's rows lie above 1469).
- **Not expected:** A's 1 April window at M 1685 and its 3 April window at M 1676 do not hold A. The control's BODY digest differs from the recorded one, so each window is not as it was when recorded. The book's voucher mark is now 1829, above every recorded M. On 1 April no row lies above 1685, and the spanned answer equals the control byte for byte. On 3 April all 4 rows lie above 1676: those vouchers were altered after the record.
- By the sheet these replays are then decided on the issue: re-recorded from the book as it is now, or kept as replays of the unspanned census. Nothing was re-sent and nothing was edited in the books.

## Marks (start = end in every part)
| part | THE COMPANY (GUID) | ALTVCHID | ALTMSTID | the part's highest M |
|---|---|---|---|---|
| A | BRIDGE AMEND LAB (17a10910-773c-42c6-bd66-7bba9a392536) | 1829 | 245 | 1790 |
| P | BRIDGE PILOT LAB (6b43e498-430c-4d5c-bfef-d32e2ab93c85) | 68 | 245 | 68 |
| G | BRIDGE GST LEDGER LAB (aae372c5-a894-491c-8843-5532e43d2d2f) | 71 | 228 | 64 |
| S | BRIDGE STOCK LAB (1edb9b05-8d35-4c0c-9959-dce655730463) | 4 | 292 | 3 |

Each ALTVCHID is at or above the part's highest M, so every part went on.

## Checks
- **Sheet:** `lab/2145-spanned-census-captures` at 6a968f0175bb7f0a4d7860d6d5e2068b5db3860b; `check_requests.py requests`: `files: ok 48, FAIL 0`, `census: ok 20, FAIL 0` (`evidence/check_requests.txt`). Each request's sha256 was checked again before its send.
- **Sends** (`evidence/send-log.txt`): 48 in all, every one curl exit 0, status 200, bytes equal to CONTENT-LENGTH, within its bound (16,384 B for marks, 1,000,000 B for a census; the largest answer was 126,938 B), each followed by `GET /status` 200 in 0.04 to 0.11 s. The answers took 0.006 to 0.185 s.
- **Each part:** step 0 with no company loaded, then THE COMPANY loaded alone by its name filter (the Gateway shows it alone); step 0 again (one tally.exe owns 9001, `/status` 200); C10 after the part (Company > Shut; none loaded; one tally.exe; `/status` 200; no XML after the shut).
- **Tally:** TallyPrime 7.2 Silver, licensed (the release and licence were read on the About screen earlier in the night; the title band shows Silver).

## Deviations
1. **Part A's marks read was sent twice before the census:** the first answer (`a-marks`) was read by the lab helper with a reader that took every COMPANY element, CMPINFO's empty one included, and stopped. By the sheet's structure that answer held one COMPANY row with THE COMPANY's GUID (1829 / 245). The reader was fixed to take only the children of the COLLECTION, and part A was run again from its marks read (`a-marks-2`, byte-equal to `a-marks`); nothing else had been sent.
2. The release and licence were not read again on the About screen for this item (it was read at 19:16 UTC in the same Tally process, PID 13252).

**Confidence:** one sitting: PARTIAL.

**Who acted.** The lab, on the owner's decision of 26 Oct 2026 for shape-discovery requests by curl.
