# 2074-L: mailing details on a book with many master alterations and few ledgers, on #2188's build (Part of #2074)

Sheet `lab/2074-mailing-by-count` at 37b717c5. BUILD-2188: #2188's head 8f7c7adfbad294f33c97566edde7a08a4bd7a3d5, Bridge CI run 40954366305 (success), artifact 12933752645 (digest sha256:b575f2cf…, equal), exe sha256 dd819eaf7744a80a5cb2931cef0a6087f409cd47d27c34c5461627a4896f4b0e. Company: BRIDGE BATCH LAB (GUID a30f23c3-4819-4334-96ab-ffa6fba70976), synthetic; BK0 57,180 B, sha256 294c5dbd80946c55bd25ae5b6d3518863e4d4727e6c5b754ea5aaa5a6a1c31fb, restored at the close (marks back to 224 / 87, no probe left). TallyPrime 7.2 Silver. 30 Oct 2026, 08:01 to 09:53 UTC. One book, one release: partial.

## Measurements (`ledger_masters`, compliance with mailing; evidence bytes of the whole read)

| read | when | evidence bytes | duration | marks M / V |
|---|---|---:|---:|---|
| Rpre | before the probe (c0 = 11 ledgers) | 219,890 | 1,415 ms | 224 / 87 |
| R0 | the probe created, no entry | 226,770 | 1,332 ms | 225 / 87 |
| Ra | after the first dated entry | 228,402 | 1,329 ms | 226 / 87 |
| R1 | after ten dated entries (ten saves) | 245,026 | 1,518 ms | 235 / 87 |
| R1b | probe two, five entries in one save | 260,562 | 1,342 ms | 236 / 87 |
| A5a read | after 10 alterations | 260,570 | 1,372 ms | 246 / 87 |
| R2 | after the push, M past 4,266 | 295,086 | 1,273 ms | 4,400 / 87 |

## Results
- **A4, the further entry's cost: 923.6 bytes, against the 950 allowed.** (R1 − Ra) / 18, nine further entries each counted twice. **The allowance held on this book, by about 26 bytes.** This is a finding for #2188's bound: the margin is thin for addresses of two short lines.
- A4, the first entry: (Ra − R0) / 2 = 816 bytes. The upper bound on an average ledger's cost, R0 / (2 × 12) = 9,448.75 bytes, is over 5,000, so it is **not conclusive** (it is a bound, not a measure).
- **A4b, the undercount: real.** One creating save of "Lab Mark Probe Two" with five dated entries moved the master mark by **s = 1**, while the party holds five entries. **The bound's further-entry term does not see 4 of them.** Probe two's growth, (R1b − R1) / 2 = 7,768 bytes, against (R0 − Rpre) / 2 + 4 × 923.6 = 7,134 bytes. This is a finding for #2188's bound.
- **L3, marks and time.**
  - The mark moved by exactly one per screen save: ten saves in A3, then one for probe two's single save.
  - It moved by one per alteration: 4,164 alterations, 236 to 4,400, with the voucher mark 87 throughout.
  - Every read answered whole in about 1.3 to 1.5 s.
  - The push took 1,265 s for its last 4,154 alterations, with no halt.
- **A5a.** After ten partial Alters (`NAMEONPAN`), both probes' dated entries are unchanged: the partial Alter keeps the mailing history.
- **A6.** At M 4,400 with 13 ledgers, R2 read whole in one part, both probes and every ledger unchanged. R2 − R1b = 34,524 bytes against the sheet's estimate of about 28,600 (2 × 13 × 1,100): the count path ran, its catalogue about 1,328 bytes a ledger.
- Every probe entry read back exactly as keyed (date, mailing name, both address lines, PIN, state, country, oldest first, `other_content_skipped` false), and every other ledger's mailing details were unchanged in every read.

## Not run
- **Part W (W1, W1b), the workbook: not run.** The install landed in the client's virtualized folder; skipped by the owner, 30 Oct. In detail: #2188's installer (`Bridge_0.6.1_x64-setup.exe`, sha256 f93bca72…) was run silently from the lab's shell on the owner's typed OK. It installed into the desktop client's virtualized copy of the per-user folder. The screen grant for that app was declined, and the owner then chose to skip Part W. **L1 is therefore not decided by this row.**
- **Part L: not run, by the run lead.** Its L, about 16,787 marks, is about four more hours of alterations, and the refusal path is covered by tests.

## How the entries were made
On Tally's own screen: Alter > Ledger > More Details > Show More > "Mailing Details (History)", one save per entry for the first probe, and all five rows before one save for probe two. The marks push used the sheet's named exception: `alter-a` and `alter-b` alternately, each checked before (MARKS) and after (counters). The 4,164 alteration answers are aggregated: they differ only in Tally's CMPINFO session counts, so one is kept, with the last, and each alteration is one line of `evidence/A5-log.tsv` with its answer's sha256.

Not measured (the sheet's list): mailing entries added by request, many populated parties, the census path past 22,857, the master body's own bytes, other releases, Gold.
