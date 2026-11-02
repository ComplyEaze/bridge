# Results: #683 tool read: `vouchers` on TallyPrime 7.2 composites through #2276, 2 Nov 2026

**PASS. State `complete`, 11 rows, 0 withheld. The evidence is complete; `withheld_vouchers` is absent from the answer. Every composite entry carries a rupee `amount` and a `foreign` with currency `$`, amount and rate; one rate is 86.4125; every voucher nets to zero.**

BRIDGE FOREXGROUP LAB (synthetic), GUID 1bb48be5-baba-45b2-90d8-7593244915a9, TallyPrime 7.2 Silver, licensed, loaded alone, as left at its last backup, restored nothing. Build #2276 at f40e9ecf, run 41455726573, artifact 13148731586, zip digest equal, `bridge_mcp.exe` sha256 b66972138b124cb1cfa04bf124562f557c1c0857c0f370862d34385b072ab6ab, equal to the package's smoke file; `initialize` 0.7.0. Posting off.

**The call:** `vouchers` with `company_guid`, `from` 2025-04-01, `to` 2025-04-01, no other argument. 2.85 s. The raw answer is `answers/003-vouchers.json`, with only `company_number` redacted.

| EXPECT | found | |
|---|---|---|
| 11 rows | `total` 11, `items` 11, one served part of 11 rows | met |
| none withheld | no `withheld_vouchers` key; nothing else marks a voucher held | met |
| not partial | evidence `complete`, result state `complete`, `reason` null, `truncated` false | met |
| composite entries | 16 entries carry `foreign`, each with currency `$`, amount and rate, and each entry a rupee `amount`; 6 plain entries, no `foreign`; 0 failed the check | met |
| one rate 86.4125 | rate 86 on 4, 86.4125 on 2, 87 on 4, 88 on 6 | met |
| each voucher nets to zero | all 11 net 0.00, summed from the entries' `amount` | met |
| the evidence includes the base read | see below | met |

## The base read
The answer has no field naming it. Two sources show it. [verified, by these two]
- **It was sent.** The call's request trail in `egress_log` (`answers/004-egress.json`) shows, after the voucher pair, three pairs of posts answered at 3,796 B, 4,000 B and 3,434 B, all `answered`, 44 sends, 0 failed. Those are the byte sizes of this book's captured currency masters and company base currency fixtures.
- **Its evidence joins.** At f40e9ecf, `agent_vouchers.rs:1292` makes the read and `:1297` passes its evidence to `accumulate_evidence`. Read from code, not a separate measure.

## Other checks
`tally_status`: 7.2, Silver, `education_mode` false, one company loaded. Raw marks before and after the call are byte-equal: 200, 3,404 B (`evidence/m0.response`, `m1.response`). No key was pressed while reading; the company was loaded by filter and shut after.

Six entries with no `foreign` belong to Journal 1, Sales 3 and Receipt 2, the book's plain vouchers.

**Confidence:** one read, one book, one date, one foreign currency, 7.2 Silver: PARTIAL. Other readers, Gold, other releases and a rounding case are not covered. The rupee amounts are as Tally holds them for this synthetic book.
