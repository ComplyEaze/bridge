# #2176 N2 row note (3 Nov 2026). Not for merge.

Question: with another client's long import in front, does a read sent after a create that was abandoned on a warmed connection still see that create?
Why this row is next: R0 (`lab/2176-r0-answers` cd414007) showed Bridge's own pattern is K1: the create on a connection that had already served requests, the reconcile read on a new one. K1 ran with a read in front. N1 ran an import in front on new connections only. Nobody has run K1 with an import in front. One variable changes from N1: B's connection.

Why the import is another client's: Bridge's wire lock lets one send through at a time on a port (`tally/runtime.rs:2518`, `:3841`, master 8520f071), so one Bridge process cannot put its own create behind its own import. Another client of the same Tally can.

Head: no build. Curl-level row, `order.py` and the runner, as N1. Owner's standing word allows writes by curl in the lab for this.
Book: BRIDGE ABANDON LAB alone, GUID 5f4825c4-5d89-47fa-9872-6397cca3a591, TallyPrime 7.2 Silver, licensed. Nothing else touching Tally. Two ledgers are written, each name used once: `Lab Order N2a`, `Lab Order N2b`. VOID repeats once as `<row>-2`.

Kit, all from `lab/2176-n1-answers` at ea73dcaa, with `lab/1713-settle-order` at 869bca9e:
- `order.py` sha256 858c7acf7b4c4634bfc1e1fc4d74cb51604611dae9f80fcc680361afb1bad6ec, unchanged. Check it first.
- `scripts/rows_n1.py` sha256 0820f3eb0b05f063ba961de85ef634966ed110178e5fb5f015cbdd477e4a6c76, copied to `rows_n2.py`, with one line added under `VARIANT`:
  `VARIANT['N2'] = ('K1', 'new', 20)`
  `plan_for` keys on `row[:2]`, so `N2a` and `N2b` take it. That gives `conns` K1 warmed with `marks.xml`, B on K1 at T0+5, limit 20; A fresh at T0+1, the seeded file; C fresh at T0+26.
- `scripts/watch.ps1` and `scripts/verdict.py`, N1's copies, unchanged. Presence comes from `verdict.py` (`orderread.py`). The live VERDICT line of the runner is not the verdict (N1 RESULTS, third local change).
- Seeds: `setup2.py`'s builder, 2,700 Journals each, same `gen.py`, count unchanged, REMOTEIDs past N1's: `('N2a', 107800), ('N2b', 110500)`. Before use, check the book holds no Journal in those ranges. Do not change the count; a D under the floor is a VOID, not a reason to resize.

Steps: as N1, in order N2a, N2b.
1. Check sha256s. `/status` 200, MARKS (one COMPANY row), LEDGERS with the name absent. The runner does this per row.
2. `python -I rows_n2.py N2a`, then after its +30 read, `python -I rows_n2.py N2b`.
3. `python -I verdict.py N2a`, `python -I verdict.py N2b`.

What counts, N1's list plus one:
- `events.jsonl` holds `warmed` for K1 and `reused` for B, conn K1. Without `reused` (B `stale`, or on a new connection), the row is VOID: it measured N1 again.
- B `written`, B end `abandoned`.
- C `written` before A's `first_byte`, C's `first_byte` after A's `first_byte`, C `answered`.
- D at least 55 s.
- A answered `CREATED 2700`, by parsing the counters, not by search.

Outcomes, as the sheet: PASS-present (present in C and at +1, +5, +15, +30). PASS-dropped (absent in C and in all four, master mark unmoved from before B). FAIL (absent in C and present in any later read, C answered before A's first byte, a probe ledger listed twice, a ledger no row created). Anything else: NOTE, send it, do not choose.
The mark moves by exactly one in a PASS-present row.

Stop at once and send the file: any FAIL, a dialog in Tally, a refused read still refused after 10 minutes, the watcher firing with a real change. A false alarm of the watcher is a VOID with its capture attached.

Decides:
| result | then |
|---|---|
| N2a and N2b both count and pass | K1 is measured with a read and with an import in front. #2176's text change goes in: `NEXT_ABSENT` and the verdict's note state the measured basis and its coverage. Only the optional restart row stays open |
| any FAIL | `confirmed_absent` tightens in the change that settles #1713. No text change |
| VOID twice, or NOTE | Stop. Send the file. No conclusion |

Limits: two repeats on one book, Silver 7.2, ledger creates only. The warmed connection here served one read and sat about 5 s idle; Bridge's had served about 70 and its idle time was not reported. A warmed connection is not known to differ by how many requests it served, and this row does not test that.

Keep: `runs/N2*/main/`, `events.jsonl`, observation answers, `evidence/times-final.txt`, the seed sha256s, MARKS before and after. RESULTS graded verified, partial or unverified. Answers branch `lab/2176-n2-answers`. Say in RESULTS what the book holds after the row, and take a backup by Tally's screen before N2a as N1 did. No restore.
