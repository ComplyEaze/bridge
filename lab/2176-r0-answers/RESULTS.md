# Results: #2176 R0, which connection carries the create and the reconcile, 3 Nov 2026

**Case (a), and every EXPECT holds. The create went on a reused connection, as request 71 of connection 1 both times. The reconcile read after the lost answer went on a new connection. Two connections were never open at once.**

Build master 8520f071, run 41537873751, artifact 13185636227, zip digest equal, `bridge_mcp.exe` sha256 bb8454bc6b52fe800a61ba7faf553427ebae1948a25771a787ed0dad77ba195f, equal to the package's smoke file, before and after. `relay.py` as issued, sha256 38a2115baaf6d3a3327af4a49a1b0c76d862f04048724040e2bc75e4c783e23b. BRIDGE ABANDON LAB alone, TallyPrime 7.2 Silver, licensed. Settings as Row S. One process and one relay folder for each part.

## Steps
| step | call | answer | seconds | M after |
|---|---|---|---|---|
| before part 1 | marks, ledgers | 24 ledgers, name absent | | 231 |
| 2 | list, status, prepare `Lab Order R0`, post, Yes by mouse | `posted_verified`, `dispatched` | 18.10 | 232 |
| before part 2 | marks, ledgers | 25 ledgers, name absent | | 232 |
| 5 | list, status, prepare `Lab Order R0L`, post, Yes by mouse | `isError` true, `import_dispatch_outcome_unknown`, cause `request_deadline_exceeded`, `reconciliation_required`, resent false | 27.56 | |
| 6 | post, same `batch_id`, at once | no window, `posted_unattributed`, `dispatched`, resent false | 6.85 | 233 |
| 7 | marks, ledgers | 26 ledgers, both names present | | 233 |

Step 5's seconds include the window. The relay held the answer 19.578 s after it arrived. `out2/create.answer` reads CREATED 1, errors 0. The prepare was not refused in either part. [verified]

Step 6's watcher saw no window in 30 s, and started before the call returned. A window would have held the call until answered, and the call returned in 6.85 s. Its `reasons` read `counters_unavailable`, with every other read-back check passed. [verified]

## Connections, parsed from `relay.jsonl`
| | part 1, `out1` | part 2, `out2` |
|---|---|---|
| lines, `open` lines | 93, 1 | 103, 2 |
| connections opened | 1 | 2 |
| most open at once | 1 | 1 |
| requests | 90 | 97 |
| the create | conn 1, request 71 | conn 1, request 71 |
| Tally's connection header on it | KEEP-ALIVE | KEEP-ALIVE |
| `tally_closed` or `relay_error` lines | none | none |

Every one of the 187 requests' answers carries KEEP-ALIVE. In part 1 the create is the one request whose sha256 matches no other and whose size, 1,004 B, is unique. In part 2 it is the one line with `dropped` true, 1,008 B. [verified]

**Step 6's first read:** connection 2, request 1, an empty-bodied request. That connection was not already open: connection 1 closed at 785.531 on the relay's clock with `bridge_closed`, held 19.578 s, and connection 2 opened at 786.718, 1.187 s later, after step 5 had returned. Every one of the 26 requests after the create is on connection 2. The spans do not overlap. [verified]

Seen and not asked: connection 2's first three requests are empty-bodied, two seconds apart, before its first bodied request. [verified]

## Which it is
**(a)** create on a reused connection, reconcile on a new one, never two open.

## Other clients
No other client of Tally was stopped. Four `bridge_mcp` processes run under the desktop client as its connectors, two builds, started before this row, and they were left as they were. In a 20 s sample before part 1 none held a connection to 9001. During each part a sample of port 9001's established connections shows one owner, the relay: `evidence/owners1.txt`, `evidence/owners2.txt`. The sample ran slower than asked, at 62 and 146 passes, so a brief connection could fall between passes. The relay can only see Bridge's own process, so it does not bear on those four. [verified]

## Limits
The relay adds a little delay per hop, so this shows what can happen, not how often. It gives each of Bridge's connections one of Tally's own. One book, one sitting, one create per part.

## Left behind
BRIDGE ABANDON LAB holds `Lab Order R0` and `Lab Order R0L`, 26 ledgers, M 233, V 54600: no delete or restore was asked for. Company shut, no company loaded, one tally.exe on 9001.

**Confidence:** PARTIAL, as the limit above.
