# #1452 empty-window live run: BRIDGE BATCH LAB (100050), 12 Oct 2026

Build: the installed bridge_mcp.exe, sha256 411f7cca…e5a6 (0.5.1/#1516), same as the rehearsal, no reinstall. Redaction none.
tally_status: 07:09:55Z before W1 (not saved), and 07:24:17Z after the run (evidence/tally-status-0724.json). TallyPrime 7.1 Silver, Education off, only 100050 loaded.
Book: last entry 21-Aug-26 (Gateway). Before the traps, the Day Book from 22-Aug-26 to 31-Mar-27 was empty (w2-distance-daybook-0822-0331.jpg).
Day Book toggles: F12 has no cancelled/optional toggle (daybook-f12-config.jpg: Show Narrations No, Condensed, Name Only, Default sort, Stripe No). Cancelled rows show "(cancelled)" and optional rows show "(Optional) n". Post-dated rows carry no marker in the Day Book; the voucher screen shows "Status Post-Dated" (t3-voucher-status-postdated.jpg).
The results carry no `read_cost` block. Below, "census reads" is `window.census.requests`. The census count itself is not in the result. The trail rows that are counted by size are: 1436 B = census post, 1740 B = window read post, each sent twice (pairs). The 1436/1740 = census/window labels are [inferred] from sizes and order.

| # | window | oracle (Day Book) | state | reason | rows | census reads | widened read in trail (20-23 style)? | outcome of the widened read | mark |
|---|---|---|---|---|---|---|---|---|---|
| W1 | 21-22 Jul 2026 (next to 20 Jul with 10 S4 vouchers) | 0 | partial | nonempty_uncorroborated | 0 | 1 | yes: a second 1436 B pair answered 27,178 B and a 1740 B pair 305,742 B (seq 24/26, 30/32) | found rows (the 20 Jul vouchers) | verified |
| W2 | 10-12 Nov 2026 (81 days after the last voucher, 21 Aug) | 0 | partial | **empty_uncorroborated** | 0 | 1 | yes: the second 1436 B pair answered 3,022 B and the 1740 B pair 3,022 B (seq 24/26, 30/32), then a 976 B pair answered 3,394 B (seq 36/38) | empty; the result is still partial | verified |
| W3 | 19-20 Aug 2026 (J 2 on 18 Aug, LSB/0001 on 21 Aug) | 0 | partial | nonempty_uncorroborated | 0 | 1 | yes: the second 1436 B pair answered 7,876 B and the 1740 B pair 63,462 B (seq 24/26, 30/32) | found rows (the neighbours) | verified |
| T1 | 14-16 Sep 2026, only voucher J 3 on 15 Sep, CANCELLED | 1 "(cancelled)" | complete | null | 1 (cancelled true, optional false, post_dated false, amounts []) | 1 | no (one 1436 B pair answered 5,422 B and one 1740 B pair answered 6,364 B; 22 requests) | n/a | verified |
| T2 | 14-16 Oct 2026, only voucher J 5 on 15 Oct, OPTIONAL | 1 "(Optional) 5" | complete | null | 1 (optional true, cancelled false, post_dated false, 2 amounts of 500) | 1 | no (1436 B → 5,440 B; 1740 B → 33,344 B; 22 requests) | n/a | verified |
| T3 | 14-16 Dec 2026, only voucher J 4 on 15 Dec, POST-DATED (Ctrl+T) | 1 (no marker in the Day Book; the voucher screen shows Post-Dated) | complete | null | 1 (post_dated true, optional false, cancelled false, 2 amounts of 500) | 1 | no (1436 B → 5,442 B; 1740 B → 33,346 B; 22 requests) | n/a | verified |

Notes
- Every empty window (W1-W3) was `partial`, even W2, whose widened read also came back empty: an empty window was never `complete` here. W2's reason is `empty_uncorroborated` (W1 and W3 are `nonempty_uncorroborated`), and only W2 sent the extra 976 B pair.
- The census answers for the empty windows were 3,022 B each, the same as the window-read answer for an empty window. The trap windows' census answers were 5,422-5,442 B. [inferred: the census answer grows with the rows it counts. The count itself is not exposed, so "census = 0" is not measured directly.]
- In every trap the census counted the cancelled, optional and post-dated vouchers alike: one row each, `complete`, no widened read.
- Traps T1-T3 were keyed by hand (Alt+2 duplicate of J 2 of 18 Aug, F2 date. T2 used Ctrl+L Optional, T3 used Ctrl+T Post-Dated, and T1 was saved and then Alt+X cancelled), after backup backup-pre/TDBK1800_100050.001 (56,610 B, sha256 7817870b189e188bb31863322fb623125a70a341420b7a5d72bbac5635b30b85, 07:18Z).
- Step 7: all three trap vouchers were deleted (Alt+D, Yes) at about 07:23Z. The Day Book from 22-Aug-26 to 31-Mar-27 is empty again (traps-removed-daybook-0822-0331.jpg). The backup was not restored; the voucher AlterID mark has moved on (82 after the cancel, higher after the deletions).
- Snapshot marks seen in the trap reads: master_alter_id 224, voucher_alter_id 82.
- Trails: each tN/wN-trail.json is the egress_log response_prepared record with fields_prepared omitted (trail-note.txt).
