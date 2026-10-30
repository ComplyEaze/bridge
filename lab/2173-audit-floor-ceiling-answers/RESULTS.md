# 2173-L: the tax audit's read over 5,376 vouchers, before and after #2191: B1 completes the 5,960-voucher year in 75 parts (Part of #2173)

Sheet `lab/2173-audit-floor-ceiling` at affa6436. TallyPrime 7.2 Silver, licensed. 30 Oct 2026, 13:22 to 13:50 UTC. One sitting, so the result is PARTIAL.

**Builds.** No new download was made. Both bundles were downloaded earlier on 30 Oct by the lab's setup session, on the owner's own typed OK in its chat, and re-hashed against the sheet before use:
- **BUILD-A,** candidate A (artifact 12928034390): `bridge_mcp.exe` sha256 576cc0c5…. This equals the release-candidate sitting's recorded hash.
- **BUILD-B,** #2191 at d01550869a4d… (still the PR's head), run 40969427288, artifact 12940610401: zip 57,279,587 B, sha256 ed9306cc… (= the sheet's digest); `bridge_mcp.exe` sha256 bf5a2106….

Each exe's hash was the same before and after its step.

## Phase A: the new book
- **BRIDGE AUDIT FLOOR LAB** (synthetic), created on Tally's screen with books from 1 April 2025. Its GUID is shown here as `<company guid>` until it is registered.
- **Ledgers:** one import of the two ledgers created 2; `ALTMSTID` went from 207 to 209.
- **Backup:** BK0, 23,701 B (sha256 e507d4af…), taken after the ledgers and before the first voucher. The marks read after it was byte-equal to the one before.
- **Vouchers:** thirteen imports, each sent once. Each `CREATED` count equals `plan.tsv` (440, then 480 × 11, then 240), with every other counter 0 and no `LINEERROR`.
  - `ALTVCHID` rose by each import's count: 440, 920, … 5,720, 5,960. **M1 = 5,960.**
  - Every capture after an import showed the Gateway with no dialog.
- The company is kept, shut, for later re-reads. No restore is planned.

## Phase B: the reads
All through a recording relay that passes reads only: one session, one call per step.

| step | build | company, period | answer | s | census | data parts | marks | over |
|---|---|---|---|---:|---|---:|---:|---|
| B0 | A | AUDIT FLOOR, year | `voucher_window_too_many_reads` | 16.3 | 1 request, no span, 5,960 rows, 9.3 s | 0 | 2 | none |
| **B1** | **B** | **AUDIT FLOOR, year** | **`completed`**, book checks all passed | **77.0** | 1 request, 5,960 rows, 1.5 s | **75** | **3** | none |
| B2-pre | B | CORPUS DENSE, 1 Apr 2025 | `vouchers`: 82 rows, one page | 7.1 | (the tool's own reads) | 4 | 4 | none |
| B2 | B | CORPUS DENSE, Apr–Jun 2025 | `audit_window_divided_by_alterid` | 21.2 | 4 spans: 7,432 + 0 + 0 + 0 rows | 0 | 2 | none |
| B3 | B | CORPUS DENSE, year | `voucher_window_too_many_reads` | 38.2 | 2 spans: 8,192 + 8,192 rows (14.9 s, 12.8 s), then stopped | 0 | 2 | none |

**Every step matches the sheet's EXPECT lines.**

- **B1, the step #2191's merge waits on, passes.**
  - **Result:** `state` `completed`. Book checks ID-1, MAP-0, MAP-1, POP-0, POP-1, POP-2, POP-3 and POP-5 were evaluated, and none failed.
  - **The parts:** 75 data parts. The first part holds 40 rows (1 April), most parts 80, 5,960 rows in all. The largest part was 2,431,506 B, under 16,777,216, and the slowest took 0.250 s. No part was divided by AlterID: every data part's span is empty.
  - **Weight and time:** the voucher weight on the wire averaged 30,391 B a row. The whole call took 77 s, under 240 s.
- **B0, before #2191:** the same year was refused before any data part, after one unspanned census of 5,960 rows.
- **B2:** the dense quarter was refused up front, because its first day holds 82 vouchers (B2-pre), more than 42.
  - The census finished all four spans, and only census requests were sent.
  - All 7,432 rows fell in the first span: the AlterIDs follow the dates.
- **B3:** the dense year was refused after two spans of 8,192 rows each, with no data part sent.
  - Neither census span ran past 20 s (14.9 s and 12.8 s), so there was no deadline.
- After every step: the relay's own status returned 200 and `GET /status` returned 200 within 0.1 s. Every session exited 0 with an empty error stream.

## Deviations
- **Relay stop, B0.** The B0 relay was launched by the lab's runner from a background shell, and it inherited that shell's "ignore Ctrl+C". Ctrl+C did not stop it.
  - Its last request was answered and its manifest complete (35 rows = 35 requests). It was stopped by Ctrl+Break to its console.
  - From B1 on, each relay was started from PowerShell with `Start-Process cmd.exe` as the sheet words it, and each stopped on Ctrl+C.
  - The idle `cmd /k` consoles were left open.
- **A2's capture** was taken before the next marks read, but looked at only after it, because a helper lost the file name. It shows the Gateway with no dialog.
- **The new company's GUID** must go into the merge gate's synthetic registry before this evidence is merged anywhere. Here it is replaced by `<company guid>`, and each replacement is listed in `substitutions.txt`.

Not measured (the sheet's list): an inventory-heavy book; a period between 10,837 vouchers and the dense year; a first voucher day of 43 to 85 vouchers; Gold and Education mode; the read through a desktop chat's own host limit.
