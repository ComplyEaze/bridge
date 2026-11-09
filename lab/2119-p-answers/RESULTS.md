First line: **No row of "What it decides" matches as written. Both calls answered as EXPECT in 5.1 s and 4.4 s, with every kept request under 2 s and `failed` 0, but each trail reads `omitted` 12, and row 6 asks for 0. One deviation: each party name is field 1 of a TAB-separated line, on the recorded word of 9 Nov (see below). No stop, no skip, no dialog.**

Sheet `lab/2119-p-sheet` at `742fc289`, kit 4 of 4 OK by its `SHA256.txt`. Book BRIDGE CORPUS DENSE (synthetic), GUID `d45bc1b0...`, Tally's current date 31 March 2026, loaded alone. TallyPrime 7.2 Silver (ProductVersion 1.1.7.2). All times are UTC on 9 Nov 2026. Every number is read from its file here, and `evidence/notes.txt` has each step.

## Deviation: the party names
The sheet says a name is the file's first line with one line terminator removed, nothing else trimmed. In both files that line is the name, a TAB, and the file's bill count. Read whole, neither name passes the tool's name rule (a TAB is outside its alphabet), so both calls would have been skipped. On the recorded word of 9 Nov, the name is field 1 of the TAB-separated line, used only if all of these hold. Each held for both names (`evidence/names-check.txt`):
- the line has exactly one TAB;
- field 2 equals the sheet's count: 248 for `party.txt`, 250 for line 1 of `top3.txt`;
- field 1 passes `LedgerScopeName` unchanged: 12 characters each;
- the file's sha256 equals its row's record: `078b01c5...` on `lab/2119-l-answers`, and `ac313c65...` on `lab/2377-b-answers`.

## What it reports
| step | result |
|---|---|
| build | commit `d04f67a9` (master; `6805d78f` an ancestor), run `<CI run id>` (Bridge CI, the merge-queue run of that head, success), artifact `<CI artifact id>` `unsigned-bundle-smoke-windows-latest`, 60,123,844 B, digest equal to GitHub's. `bridge_mcp.exe` sha256 `49d9d9a1eb38891896d75272e78d876e2a9f4ef31598a09c06bcfac6b4050281` (equal to the bundle's `binary_sha256`), the same before the first call and after the last. Server 0.7.0 |
| step 1 | `tools/list`: the four tools are listed, and `outstandings`'s `detail` offers `open_bills`. `list_companies`: THE COMPANY alone, `verified_tuple`, `books_from` 20250401. M0 328, V0 29900 |
| P-top1 | answered: `observed`, `tally_ledger_outstandings`, `open_bills`, `as_of` 20260331; bills 250; On Account none; `tied`; `checked`; `ledger_match` `exact`; sum of `closing` equal to row 2377-B's kept sum. Whole time 5.121 s (written 05:56:21.001Z, read 05:56:26.123Z) |
| P-p1 | answered as P-top1; bills 248; On Account none; sum of `closing` −11,815,459.60, equal to row 2119-L's. Whole time 4.432 s (written 05:57:41.090Z, read 05:57:45.517Z) |
| largest `held_ms` | P-top1: 1,302 ms, seq 28, the first Ledger Outstandings post. P-p1: 1,013 ms, seq 30, the second. Both are among the kept entries only (seq 13 to 44) |
| M1 and V1; close | 328 and 29900, equal to M0 and V0. A's exit code 0, error stream 0 B. C10 met |
| dialog, skip or deviation | no dialog and no skip. Deviations: the names, above. One `ledger_masters` call at step 1 was sent by the lab without `company_guid` and refused by the tool before any Tally request (`company_guid_required`, 0 bytes). It was sent again with the GUID |

## The trails
Each `request_trail` reads `sent` 44, `failed` 0, `omitted` 12: the record keeps the last 32 entries, seq 13 to 44. Every kept entry is `answered`. The pairs are told **by inference from position and size**, not from request bodies. `read_evidence` was not called, because it is outside rule 1's closed set.
- **Snapshot pair: seq 24 and 26.** These are the first equal-sized posts after the extent pair (seq 20 and 22), each 1,290 B, answered with 155,514 B.
- **Ledger Outstandings pair: seq 28 and 30.** These are the next pair, each 820 B, answered with 206,974 B for top1 (the same size as row 2377-B's curl read) and 205,336 B for p1.

| seq | kind | request_bytes | P-top1 response_bytes | P-top1 held_ms | P-p1 response_bytes | P-p1 held_ms |
|---|---|---|---|---|---|---|
| 14 | post | 1980 | 138074 | 31 | 138074 | 27 |
| 16 | post | 2524 | 4012 | 5 | 4012 | 4 |
| 18 | post | 2524 | 4012 | 4 | 4012 | 5 |
| 19 | post | 2524 | 4012 | 5 | 4012 | 3 |
| 20 | post | 1100 | 3854 | 3 | 3854 | 4 |
| 22 | post | 1100 | 3854 | 3 | 3854 | 3 |
| 24 | post (snapshot) | 1290 | 155514 | 41 | 155514 | 24 |
| 26 | post (snapshot) | 1290 | 155514 | 41 | 155514 | 27 |
| 28 | post (Ledger Outstandings) | 820 | 206974 | 1302 | 205336 | 999 |
| 30 | post (Ledger Outstandings) | 820 | 206974 | 1098 | 205336 | 1013 |
| 32 | post | 1100 | 3854 | 4 | 3854 | 3 |
| 34 | post | 1100 | 3854 | 5 | 3854 | 4 |
| 36 | post | 2524 | 4012 | 6 | 4012 | 4 |
| 38 | post | 2524 | 4012 | 4 | 4012 | 6 |
| 39 | post | 2524 | 4012 | 21 | 4012 | 4 |
| 40 | post | 982 | 3406 | 10 | 3406 | 3 |
| 42 | post | 982 | 3406 | 9 | 3406 | 4 |
| 44 | post | 2524 | 4012 | 6 | 4012 | 5 |

Every status entry in between (seq 13, 15, 17, 21, 23, 25, 27, 29, 31, 33, 35, 37, 41, 43) was answered with 51 B, held 0 to 4 ms. The full tables are in `evidence/p-top1-outcome.txt` and `evidence/p-p1-outcome.txt`.

## Which row
- Rows 1 to 5 are not met: every EXPECT was met, nothing was refused or skipped, and both whole times are under 50 s.
- Row 6 is met except for one condition: "each trail has `failed` 0 and `omitted` 0". Each reads `omitted` 12, so 12 of 44 requests per call are not in the trail, and their `held_ms` is not shown.
- Rule 3's screenshots at 10 s and 30 s were not taken, because each call was answered before 10 s. The capture after each answer equals the step 0 Gateway: no dialog.

The maintainers decide whether row 6 holds with the 12 omitted entries, or whether a read of those entries is needed first.

## Limits
One sitting, one book, one date, two parties, one build. The whole time is the stdio client's, not a desktop app's.
