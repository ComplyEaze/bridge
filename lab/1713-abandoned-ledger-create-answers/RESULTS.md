# Results: #1713 abandoned ledger create, 28 to 29 Oct 2026. On BRIDGE ABANDON LAB (TallyPrime 7.2), a ledger Create abandoned at ComplyEaze Bridge's 20-second limit while the gateway was busy was still run by Tally. In each of L2 to L5, the next ledger read was queued behind the busy work and answered with the probe ledger present, and every observation read up to 30 minutes later showed the same. No read found the probe absent, and Tally never refused a connection. By the sheet's PASS rule, every counting row passes: a request answered after the create was sent showed the create had run

The sheet is `lab/1713-abandoned-ledger-create`:
- First sitting at f86d4f1a.
- Then at 9a77082a: `read.py marks` accepts an absent ALTVCHID.
- Then at b28ec796: `read.py import` accepts a bare RESPONSE root.
- Then at 811aed8b511b3af3cc855688e315ad6e9daf0361: the order gates compare against A's first byte.

Each change followed a halt recorded in `notes.txt`. `SHA256.txt` was 18 of 18 equal at each.
- **Tally:** TallyPrime 7.2 (Latest), Silver Rental, licensed, a single currency.
- **Company:** BRIDGE ABANDON LAB, created on Tally's screen for this sheet. Number 100096, GUID 5f4825c4-5d89-47fa-9872-6397cca3a591. Books from 1 April 2025.
- **Requests:** by curl only, with no ComplyEaze Bridge build.
- **Owner's yes:** for the writes, recorded in `notes.txt` before S1.
- **Backups (all on the lab PC):**
  - BK0: 23,385 B, da352c5d....
  - BK1 (the bulk book): 13,255,159 B, cf65e098....
  - BKE: 13,255,549 B, ba508e32....
- **The book stays on the PC as BKE left it**, with the probes L0, L1, L1B and L2 to L5.

## Setup
| step | result |
|---|---|
| S0 | scripts parsed with 0 errors; dry runs equal the sheet |
| S1 | company created; MARKS: GUID as above, ALTMSTID 207, ALTVCHID absent |
| S2 | BK0 |
| S3 | two bulk ledgers, `import ok CREATED 2` (a bare RESPONSE); ALTMSTID 207 → 209 |
| S4 | `bulk.ps1 -First 1 -Last 96`: `DONE`, each file `CREATED 250` |
| M0 | BUSY 119,036,974 B, `busy ok rows 24000`, 21.786 s; plan: send bulk-097 to bulk-192, predicted 238,073,948 B |
| M0 run | `bulk.ps1 -First 97 -Last 192`: `DONE`; ALTVCHID 48000 |
| S5 | BK1 |
| M1 | BUSY 238,211,294 B, `busy ok rows 48000`, **186.645 s**: D = 187 s (40 to 300; at least 55, so L5 runs) |

Doubling the book from 24,000 to 48,000 Journals took BUSY from 21.8 s to 186.6 s.

## The rows
In each row, A is the BUSY read, B the probe create and C the ledger read. Times are UTC, from each row's `times.txt`. "A first byte" is A's start plus curl's `time_starttransfer`.

| row | A | B | C | probe in C | probe at C + 1, 5, 15, 30 min | ALTMSTID |
|---|---|---|---|---|---|---|
| L0 | none | no limit: `CREATED 1` | none | Lab Probe L0 present (Suspense A/c) | | 209 → 210 |
| L1 | no limit; first byte 20:23:22.32, end 20:23:24.43 | no limit, queued T0 + 5 s; answered 20:23:22.89, `CREATED 1` | none | Lab Probe L1 present after the row | | 211 |
| L1B | no limit; first byte 21:04:18.068, end 21:04:21.173 | no limit; answered 21:04:19.102, `CREATED 1` | T0 + 26 s; answered 21:04:19.249 | present | present, present, present, present | 212 |
| L2 | no limit; first byte 21:38:32.670, end 21:38:34.710 | **limit 20: abandoned** (exit 28, all 1,004 B uploaded) | 1 s after B gave up; answered 21:38:33.442 | **present** | present ×4 | 213 |
| L3 (repeat of L2) | no limit; first byte 22:12:30.503, end 22:12:32.378 | **abandoned** at 20 s | answered 22:12:31.020 | **present** | present ×4 | 214 |
| L4 | **abandoned** at 20 s | **abandoned** at 20 s | after both curls exited; held 163 s, answered 22:47:23.385 | **present** | present ×4 | 215 |
| L5 | **abandoned** at 20 s | sent only after A's curl exited; **abandoned** at 20 s | after B's exit; held 120 s, answered 23:20:47.893 | **present** | present ×4 | 216 |

- **L1:** it halted on its first gate, which compared curl ends. B was answered about 0.48 s after A's first byte, while A's 238 MB body streamed for 2 s more. It is kept as measured: the gates were then revised to compare against A's first byte, and L1 was not run again. `read.py busy` was run on L1's A by hand afterwards: `busy ok rows 48000`.
- **L1B:** `ORDER ok: A first byte < B end < C end`, with `NOTE: B ended during A's stream`. B and C were answered about 0.9 s after A's first byte, 2 ms apart.
- **L2 and L3:** C was answered after A's first byte and during A's stream (`NOTE`, which counts). No `FINDING` in any row.
- **Refusals:** no C was refused (no curl exit 7, 52 or 56 in any row). Each was accepted and held until Tally reached it, including after an abandoned read (L4, L5).
- **ALTMSTID:** rose by exactly 1 per row, at its create. ALTVCHID stayed 48000 from M0's run to the end.
- **A's answer:** in L1B to L3 it was 238,211,254 B, 40 B fewer than M1's. Every complete BUSY answer read `busy ok rows 48000`. Their sha256 differ from run to run (`BUSY-ANSWERS.txt`).
- **Screens:** no dialog at any time. Tally's window was captured every 30 s while each script ran.

**PASS (decided before the sitting):** in each of L2 to L5, C was answered after A's first byte (L2, L3) or after both curls (L4, L5), and every observation read showed what C showed (present). **FAIL:** not met. So a request answered after an abandoned create was sent showed the create as run, in all four rows.

**Confidence:** L2 and L3 are a repeated pair. L1B, L4 and L5 are single observations, PARTIAL until repeated.

**Not covered:**
- a create sent on a connection Bridge reused (curl opened a new connection for every send; every answer's `Connection` header reads KEEP-ALIVE);
- a busy gateway caused by anything other than a read;
- Gold;
- a Tally restart during a row;
- an abandoned create of another master type.

## Files
- `requests/`: the sheet's 11 requests, as sent.
- `evidence/hand/` and `evidence/resume/`: requests sent by hand and their answers with headers; `sends.txt`.
- `evidence/L0`, `L1`, `L1B`, `L2` to `L5`: each row's `times.txt` and answers, headers and curl error files.
  - A's answers are listed in `BUSY-ANSWERS.txt`.
  - L4's and L5's A were abandoned before any byte arrived and left no file.
- `evidence/bulk/`: `bulk-answers.txt` and every `.sent`, header and `.curl.err` file. The 192 answers are listed in `BULK-ANSWERS-SHA256.txt`.
- `evidence/m0/`, `evidence/m1/`: the BUSY headers.
- `evidence/starts.txt` and `s0/` (the dry runs).
- `notes.txt` (UTC).
- **Copies, not the bytes the scripts printed.** The branch holds copies of `times.txt`, `bulk-answers.txt`, `starts.txt`, the dry runs and `notes.txt`, each with three substitutions: the lab folder path written `<lab folder>`, the sheet's capital halt word written `HALT`, and its name for who decides written `run owner`.
- **netstat files** on the branch are reduced to their `:9001` lines.
- **Originals:** `LOGS-ORIGINALS-SHA256.txt` lists the originals of both, which stay on the lab PC.
- **`SCREENSHOTS.txt`:** the captures by sha256. They, the backups and the BUSY and bulk answers stay on the lab PC.
