# #1542's voucher-type numbering read, live as one tool call, matches every known answer on two numbering-lab books (#724, Windows lab, 14 Oct 2026)

Synthetic data only: BRIDGE MANUAL NUMBERING LAB 1 and BRIDGE MANUAL NUMBERING LAB 2 (their GUIDs are in
the answers), each loaded alone. TallyPrime 7.1, Silver Rental, on Windows. Read only: nothing was written
to Tally. One run. The build is #1542's bundle at bcb3aec0 (bridge_mcp.exe sha256 249f16e1...2915 at the
start and the end; bridge-tally.mcpb 3c92fd9b...4558), run from a desktop config entry with its own data
folder; posting and import were not set. The sibling answers of 724-M5 are on branch
`lab/724-m4-m5-numbering-answers` at 3684c58c. Times are UTC.

## Result

`masters` with kind `voucher_types` was not refused on either book; each returned 24 rows, state
observed. `series_numbering` per row (observation / method / duplicates):

| book | Payment | Journal | Receipt | Contra | the other 20 types |
|---|---|---|---|---|---|
| LAB 1 | observed / manual / prevented | observed / manual / prevented | observed / automatic / not_prevented | observed / automatic / not_prevented | observed / automatic / not_prevented |
| LAB 2 | observed / manual / not_prevented | observed / automatic / not_prevented | observed / automatic / not_prevented | observed / automatic / not_prevented | observed / automatic / not_prevented |

- Every known answer from the earlier screen checks holds: LAB 1 Payment and Journal Manual with
  duplicates prevented, Receipt and Contra Automatic; LAB 2 Payment Manual with duplicates not prevented;
  every other type Automatic (defaults, and LAB 2's Journal as checked on screen in 724-M5).
- The type-level `numbering_method` read `{"unrecognised": "None"}` on LAB 1's Payment and Journal and on
  LAB 2's Payment, Contra and Receipt (the types that were opened on screen before), and `default` on all
  others; as the tool says, it is not the method.
- The snapshot marks were master 215 / voucher 3 on LAB 1 and 235 / 12 on LAB 2.
- The build's egress log grew by exactly two lines per call (12 after 6 calls); no other data folder
  changed.

## What it decides

The series-level read can carry the rule that refuses a numberless voucher unless its type reads
Automatic: on these two books it gave every known answer, as one tool call, unrefused.

## Not covered

More than one series; Automatic (Manual Override); a type created new; Gold; other releases; the final
#1542 binary after its master merge.

## Files

`results/` (every tool answer in full, as transcribed: status, company list and the voucher-type read of
each book), `results/n1-m4-rows.txt` and `n2-m4-rows.txt` (one line per row), `step0/`, `answers/`,
`SHA256.txt`. Kept on the lab PC, by hash in `SHA256-kept-on-lab-pc.txt`: screenshots, the working log,
the summary and the per-call count file.
