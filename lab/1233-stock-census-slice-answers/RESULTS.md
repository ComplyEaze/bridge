# Results: #1233, a stock-item AlterID census in three slices on BRIDGE STOCK LAB, 29 Oct 2026. **The three slices covering every AlterID up to the mark returned the company's 55 stock items exactly once across them (0, 0 and 55 rows), as many as Tally's `NUMSTOCKITEMS`; each slice stayed within its width; a slice past every AlterID returned a present, empty collection, byte-equal to the two empty slices; every row carried the company's GUID in `BRIDGECOMPANYGUID`. The stock-group census slice returned the same 3 groups as the `masters` collection. The k1 comparison was not made: Tally's current date on this book is 21 Apr 2025.**

## The step-1 gate, and the stop
The first gate read (`m-a`) gave ALTMSTID 292, ALTVCHID 4, NUMSTOCKITEMS 55, ISINVENTORYON Yes, BOOKSFROM 20250401 (LASTVOUCHERDATE 20250421): all as gated. The current date recorded at step 0 is **Monday 21 Apr 2025**, not on or after 31 March 2026, so the gate failed and nothing more was sent (stop; the book was shut). On the run lead's word the row then ran **without `k1`** (the date affects only `k1`, which is then not the request `stock_summary` would send for 31 March 2026). The book was loaded again, step 0 repeated, the gate read sent again (`m-a2`, byte-equal to `m-a`), then the rest. The Tally date was not touched.

## Reads (send log in `evidence/send-log.txt`)
| read | request | rows | bytes = CONTENT-LENGTH | time_total | answer sha256 |
|---|---|---|---|---|---|
| m-a / m-a2 | m-marks-S | 1 company | 4894 | 0.010 / 0.013 s | 6b68c769a11de31494248b096ad683cd669eb44613b3d22df33d1f353cd2efe1 (both) |
| c1 | (0, 100] | 0 | 2998 | 0.030 s | cf34fa0a514109d2691ae8ccd750fd83338927b5153541d55f810b0530658a1a |
| c2 | (100, 200] | 0 | 2998 | 0.019 s | cf34fa0a... (equal to c1) |
| c3 | (200, 300] | 55 | 30262 | 0.026 s | db77a9065fea6813f888ab2e9ca27475ea18286e31aaa275a58a21bf30b6d374 |
| c4 | (400000, 400010] | 0 | 2998 | 0.019 s | cf34fa0a... (equal to c1) |
| g1 | stock groups (0, 300] | 3 | 4496 | 0.019 s | b3204c318fe38b1379541843ca30ce643bee59a3439d8af11c8259749425d167 |
| c1r | (0, 100] again | 0 | 2998 | 0.018 s | cf34fa0a... (byte-identical to c1) |
| k1 | **not sent** (see the gate) | - | - | - | - |
| k2 | stock-group masters | 3 | 6296 | 0.024 s | 31a5b8979f1bed4e52698183ceebf8760763e0c41c33c56a1f3b6e3a718b12a6 |
| m-b | m-marks-S | 1 company | 4894 | 0.010 s | 6b68c769... (equal to m-a): ALTMSTID 292, ALTVCHID 4 |

Every answer: curl exit 0, status 200, bytes equal to CONTENT-LENGTH, within its read bound; a capture of Tally's whole window after each (no dialog) and `GET /status` 200.

## By parser (`evidence/parse-records.txt`, `tables/`)
- **Every census answer** (c1 to c4, g1, c1r): `DATA/COLLECTION` present, `HEADER/STATUS` 1. Stock-item collections carry `ISMSTDEPTYPE="Yes" MSTDEPTYPE="512"`, stock-group collections `MSTDEPTYPE="128"`.
- **Rows:** c1 0, c2 0, c3 55 (each at most 100); c1 + c2 + c3 = **55 = NUMSTOCKITEMS**, 55 distinct GUIDs, none twice.
- **GUIDs:** every GUID begins with THE COMPANY's GUID; every `BRIDGECOMPANYGUID` equals it; no GUID differs from its trimmed self.
- **Row shape** (census rows, stock items and stock groups alike): child elements `GUID` and `BRIDGECOMPANYGUID` only; attributes `NAME` and `RESERVEDNAME`. Every row has the same set.
- **c4** (past every AlterID): a present, empty collection; c1 and c2, also empty, are **byte-equal** to it.
- **c1r:** the same rows as c1 (none), byte-identical.
- **g1 (stock groups):** 3 rows, at most 300; every GUID prefixed by THE COMPANY's GUID (the stock groups carry the prefix, as stock items do); no GUID twice. **k2:** 3 rows; the GUID set **equals** g1's. (k2's rows also carry `ALTERID`, `MASTERID`, `PARENT` and `LANGUAGENAME.LIST`.)
- **k1:** not measured (not sent).
- **Marks:** m-b equals m-a (292 / 4).

## Checks
- **Sheet:** `lab/1233-stock-census-slice` at bcee293e2534034f1f6e8f9af884547e0ae86d29; `SHA256.txt` 12 of 12 equal; `check-request-ids.py requests`: `census: ok 8, skipped 0, FAIL 0`. Each request's sha256 checked again before its send.
- **Step 0** (twice, no company loaded): one tally.exe owns 9001; `/status` 200. **C10** (twice): shut from the Company menu; none loaded; one tally.exe owns 9001; `/status` 200; no XML after the shut.
- **The Gateway:** Current Period 1-Apr-25 to 31-Mar-26; current date Monday 21-Apr-2025; last entry 21-Apr-25.

## Deviations
1. **k1 not sent,** on the run lead's word after the gate's stop: the current date is 21 Apr 2025. The k1 GUID-set comparison is not measured.
2. **A second sitting of the gate:** the row ran the gate read twice (`m-a`, then `m-a2` after the stop and reload); both byte-equal.

**PASS (the sheet ran, without k1).** **Confidence:** one sitting, one small synthetic book: PARTIAL. Not measured: k1; Bridge's sliced read; a book with a mark over 874; a mark above 300; more items than a slice's width.
