# 2214 mailing cut and drain: on #2204's build, an oversize mailing read is refused at its head with no drain owed, and the standard read after it answers whole at once (Part of #2214)

Sheet `lab/2214-mailing-cut-drain` at 3ba77c56. Build: #2204 at 651cdfa6, artifact 12963818564 (the zip the lab holds; digest equal, no new download). `bridge_mcp.exe` sha256 2c41a51e…, the same at P0 and at Close. Company: BRIDGE BATCH LAB (synthetic). It started at 224 / 87 with 11 ledgers; 201 parties were created with 40,005 dated mailing entries; BK-2214 was restored at Close, back to 224 / 87 with 11 ledgers. TallyPrime 7.2 Silver. 30 Oct 2026, 12:47 to 13:10 UTC. One sitting, so the result is PARTIAL.

## Q1: the ladder
The `ledger_masters` master read is a pair of posts. Its request is 1,736 B in STANDARD and 1,784 B in MAILING (see the deviations below).

| rung | parties | entries | MARKS | S_n master held (ms) | M_n outcome | M_n master held (ms) | M_n master answer (B) | M_n call (s) |
|---|---:|---:|---|---|---|---|---:|---:|
| K1 | 0 | 5 | 225 / 87 | (R1: 23/23, 17/17) | answered | 17 / 15 | 34,206 | 1.91 |
| L1 | 5 | 1,005 | 230 / 87 | 19 / 17 | answered | 101 / 100 | 974,466 | 2.00 |
| L2 | 10 | 2,005 | 235 / 87 | 34 / 25 | answered | 256 / 218 | 1,914,728 | 3.09 |
| L3 | 20 | 4,005 | 245 / 87 | 21 / 24 | answered | 349 / 375 | 3,795,248 | 3.38 |
| L4 | 40 | 8,005 | 265 / 87 | 38 / 34 | answered | 753 / 677 | 7,556,288 | 5.17 |
| L5 | 80 | 16,005 | 305 / 87 | 56 / 65 | answered (2 pages) | 1,667 / 1,470 | 15,078,368 | 10.50 |
| L6 | 160 | 32,005 | 385 / 87 | 78 / 66 | answered (3 pages) | 4,165 / 3,325 | 30,122,530 | 19.20 |
| L7 | 200 | 40,005 | 425 / 87 | 94 / 83 | **`response_size_limit_exceeded`** | 3,448 (first post) | over 33,554,432 | 5.14 |

- **No rung was cut at the 20 s limit.** The master answer grows by about 941 B per entry, at about 0.13 ms per entry.
- **The 32 MiB cap came first, at L7.** Its whole answer is 37,644,610 B (D5), and Tally sends it in about 5.5 s. At this answer size a 20 s cut would need about 145,000 entries, well past the cap.
- **K1, the import shape:** the five entries of "Lab Mail Load 000" read back exactly as sent, oldest first, with `other_content_skipped` false. Every other ledger was unchanged.
- **Creates:** all 201 answered `CREATED` 1. Each master mark moved by exactly one, and the voucher mark held at 87.

## Q2 and Q3: the drain after the refusal, and the read after it
| step | call | s | code | master post | drain record after | next STANDARD |
|---|---|---:|---|---|---|---|
| D1 | M7 | 5.14 | `party_ledger_master_read_failed`, no cause | `response_size_limit_exceeded`, 3,448 ms | `{"state":"clear"}` | sent 0.55 s later, answered at once in 2.33 s |
| D4 | M7 again, 120 s later | 6.50 | the same | `response_size_limit_exceeded`, 4,888 ms | clear | sent 0.58 s later, answered at once in 2.44 s |
| C2 | M7 a third time, 120 s after C1 | 6.97 | the same | `response_size_limit_exceeded`, 5,412 ms | clear | sent 0.58 s later, answered at once in 2.41 s |

- **No drain is owed after an oversize at this size.** Tally declares the length at the head, and #2204 records no debt, as the sheet foresaw for this case. No call was refused as `tally_endpoint_draining`.
- **The standard read after it is whole every time:**
  - `total` 212, and every row equal to S7's as parsed JSON;
  - snapshot marks 425 / 87, then raw MARKS 425 / 87;
  - master posts 80 to 91 ms, against R1's post threshold of 2,017 ms;
  - the gap from D1's end to D3's end was 3.6 s, both times.
- Reading: the sheet's first row ("D3 answers within 3 minutes, whole, marks unchanged, in both runs") holds trivially. With no drain to wait out, #2214's retry can follow at once at this size. **A cut at the 20 s limit, and the drain after one, were not reached on this book**, so the drain after a cut mailing read stays unmeasured.

## Q4: past 32 MiB
- **D5, the whole answer by curl:** exit 0, 200, 37,644,610 B, equal to `CONTENT-LENGTH`. `time_starttransfer` 5.163 s, `time_total` 5.513 s. The body holds 212 LEDGER rows.
  - The STANDARD after it went 4.7 s late, because the lab's helper parsed the body first. It answered at once, whole, with the record clear.
  - `time_total` less 20 s is negative: at this rung Tally finishes well inside the limit.
- **C1, curl with `--max-filesize 33554432`:** exit 63, 0 bytes kept. The head arrived at 4.927 s and declared `CONTENT-LENGTH: 37644610`, so the oversize was known at the head. curl ended at 4.968 s.
  - The STANDARD sent 0.4 s later answered at once in 2.38 s, whole against S7, with no debt. Tally was not still busy once curl dropped the answer.
- **C2, ComplyEaze Bridge's own read:** `response_size_limit_exceeded` within the limit, about 5 s into the master post. #2214's `ResponseTooLarge` path is live at this size.
- Reading: the sheet's rows "C1: `CONTENT-LENGTH` declared and over the cap" and "C2: `response_size_limit_exceeded`" both hold.

## At rest and thresholds
- R1, STANDARD twice, 120 s apart: 1.907 s and 1.969 s, `total` 11, 224 / 87.
  - The threshold is 3.969 s.
  - The post threshold is 2,017 ms, from R1b's slower master post (17 ms).
- R2, MAILING: 1.812 s. Its master post took 1,784 B, answered 27,698 B, and held 20 / 17 ms.
- The drain record read `{"state":"clear"}` at every copy (33 copies under `debt/`).

## Deviations
- **The master posts.** The sheet takes the two posts with the largest request bytes. On this build those are a 2,524 B company read, the same in both reads. The master pair is the post whose request changes with `include_mailing_details` (1,736 B / 1,784 B). The lab's helper picks it by that size from after Part R; Part R's figures are read from the full trails.
- **The runner's capture check.** After create-001, a capture byte-equal to the looked-at reference capture (create-000: the Gateway, no dialog) let the runner go on, and any other capture would have stopped it. Every capture was byte-equal.
- **D5's following STANDARD** went 4.7 s after curl ended, not at once, because the helper parsed the body first.
- **The first server start** did not launch: the client's path had been mangled by the shell, and nothing ran. The files were rewritten and checked before the real start.
- **P0** shut BRIDGE CORPUS DENSE, which the previous check had left loaded, before loading THE COMPANY.

Not covered (the sheet's list): a book whose entries are spread thinly over many parties; longer addresses; a second process sending during a drain; a connection dropped after sending began; other releases; Gold. Also not reached: a mailing read cut at the 20 s limit, and the drain after one.
