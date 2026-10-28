# Results: #899 part 2, the spanned census on the note-day book, 28 Oct 2026. On TallyPrime 7.2, BRIDGE SHAPE LAB (ALTVCHID 112), each of the three days answered the same bytes three times over: the recorded answer of 1 Oct, the unspanned census today and the census spanned to (0, 111]. `compare899.py` exited 0 for all three days. Reads only; the marks did not move

The sheet is `lab/899-note-days-spanned-census-run-sheet` at fdef8edf5958badaf0eff2b256959b5531d77ca8.
- **Sheet checks:** `SHA256.txt` 12 of 12; `check-request-ids.py requests`: census ok 7, skipped 0, FAIL 0.
- **Company:** BRIDGE SHAPE LAB. `m_marks` returned one COMPANY row with GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346 (the GUID check was met).
- **Marks:** ALTVCHID 112, ALTMSTID 292 at the start and the end; the answer is byte-equal (3,396 B, c71056a0...).
- **Tally:** TallyPrime 7.2, Silver Rental, Licensed. No dialog appeared; one tally.exe owned 9001.

## Answers
| day | request | sent / answered (UTC) | bytes | sha256 |
|---|---|---|---|---|
| 28 Apr 2025 | d0428_c1_unspanned | 14:34:49.809 / 14:34:50.438 | 5,450 | 8bf751237defd84bdfe4dc7ae98ad9cb584be3f983694caefe0de359e1876e9d |
| 28 Apr 2025 | d0428_c2_spanned_0_111 | 14:34:50.472 / 14:34:50.719 | 5,450 | 8bf751237defd84bdfe4dc7ae98ad9cb584be3f983694caefe0de359e1876e9d |
| 29 Apr 2025 | d0429_c1_unspanned | 14:34:50.740 / 14:34:50.960 | 5,454 | 54ca29c0d72ffdd17272f6bf872e4de7740ce0a975a1d6a7f1833f0dfff26206 |
| 29 Apr 2025 | d0429_c2_spanned_0_111 | 14:34:50.976 / 14:34:51.174 | 5,454 | 54ca29c0d72ffdd17272f6bf872e4de7740ce0a975a1d6a7f1833f0dfff26206 |
| 3 Jul 2025 | d0703_c1_unspanned | 14:34:51.191 / 14:34:51.402 | 5,434 | 71c527b65edcb2aaee81561d581528ce3f620a10deaca764f6bd050fd2842973 |
| 3 Jul 2025 | d0703_c2_spanned_0_111 | 14:34:51.418 / 14:34:51.616 | 5,434 | 71c527b65edcb2aaee81561d581528ce3f620a10deaca764f6bd050fd2842973 |

## compare899.py
| day | exit | rows (GUID, ALTERID, DATE) | unspanned = recorded | spanned = unspanned | all rows ALTERID <= 111 |
|---|---|---|---|---|---|
| 28 Apr 2025 | 0 | ...-0000000a, 10, 20250428 | holds | holds | holds |
| 29 Apr 2025 | 0 | ...-0000000b, 11, 20250429 | holds | holds | holds |
| 3 Jul 2025 | 0 | ...-00000029, 41, 20250703 | holds | holds | holds |

**Confidence:** one sitting, PARTIAL until repeated. Not covered:
- a day where the span leaves a voucher out;
- the other requests of the note-day replays;
- the code's own rendering of the spanned request.

## Files
- `raw/`: every answer, whole, with its headers. The marks are saved as `-start` and `-end`.
- `cmp-d0428.txt`, `cmp-d0429.txt`, `cmp-d0703.txt`: each `compare899.py` output, whole.
- `marks.py`.
- `send-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures by sha256; they stay on the lab PC.
