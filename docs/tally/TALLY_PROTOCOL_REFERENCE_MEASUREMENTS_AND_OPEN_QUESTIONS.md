# Tally XML gateway protocol reference — measurements and open questions

> This is a section of the canonical [Tally XML gateway protocol reference](./TALLY_PROTOCOL_REFERENCE.md). Read its confidence-marker convention and legacy-link note before relying on a finding.

---

## 10. Change detection

**VERIFIED.** Company-level `ALTVCHID` and `ALTMSTID` are monotonic high-water marks and move
in step with writes — two master creates advanced `ALTMSTID` by 2; one voucher create
advanced `ALTVCHID` by 1. Per-object `ALTERID` is exposed on vouchers and masters.

**UNVERIFIED but high value:** `AUDITENTRIES.LIST`, `OLDAUDITENTRIES.LIST` and
`ACCOUNTAUDITENTRIES.LIST` containers appear in voucher exports on the Edit Log SKU. They
were empty on un-edited demo data. If editing a voucher populates them with before/after
values, the Edit Log SKU exposes change history directly and AlterID diffing becomes
unnecessary on that SKU. **Not tested.**

### 10.1 Server-side `AlterID` filtering works — **contradicts published community guidance**

**VERIFIED.** A `<FILTERS>` predicate of the form `$AlterID > N` filters correctly at the
server. Measured on the demo company:

| Predicate | Rows |
| --- | --- |
| `$AlterID > 0` | 150 (whole book) |
| `$AlterID > 100` | 51 |
| `$AlterID > 200` | 3 |
| `$AlterID > 500` | 0 |

Semantically verified, not merely monotonic: `$AlterID > 200` returned exactly three
vouchers with `AlterID` 440, 441 and 442.

This matters because the published community position is the opposite. A TallyForum thread
on conditional export states *"Tally Prime does not respond to any filtering criteria other
than date duration"*, and the accepted workaround is to download the entire book on every
sync and diff `AlterID` client-side — described there as "network intensive". That workaround
is unnecessary, at least on this release.

**Consequence for incremental sync:** the design becomes cheap. Probe company-level
`ALTVCHID`/`ALTMSTID`; if moved, fetch only `$AlterID > checkpoint`. Three rows instead of
one hundred and fifty.

**Known gap — deletions.** A deleted object has no `AlterID` to exceed the checkpoint, so an
AlterID-filtered scan cannot see it. Deletion detection still requires either a complete
scan with absence reasoning (subject to §5.3's corroboration rule) or, on the Edit Log SKU,
the audit-entry containers above. **A cancelled voucher IS detected** — the cancel in §9.6
produced a new object at `AlterID` 443, visible to a `> 440` filter.

**UNVERIFIED:** whether other fields filter server-side (`$IsCancelled`, `$VoucherTypeName`,
`$PartyLedgerName`). Only `$Date` and `$AlterID` have been tested. Given the forum's claim
was wrong about `AlterID`, it is probably wrong more broadly — worth testing before accepting
any "Tally can't filter that" advice.

---

## 11b. Scale findings at 101,287 vouchers — the two that matter most

**VERIFIED 2026-07-29** on a 101,287-voucher production-shaped corpus.

| Request | Time | Bytes | Rows | B/row |
| --- | --- | --- | --- | --- |
| Minimal fetch, whole book | 108 s | 123.4 MB | 101,287 | 1,218 |
| **Curated, whole book** | **600 s (client gave up)** | **282.7 MB** | **78,320 of 101,287** | 3,609 |
| Curated, one month | 7.3 s | 17.7 MB | 4,891 | 3,612 |
| Curated, `$AlterID` filtered | 24.1 s | 5.7 MB | 1,583 | 3,615 |
| Wildcard, one month | 69.2 s | 105.3 MB | 4,891 | 21,532 |

### 11b.1 A truncated read is indistinguishable from a complete one — **P0**

The whole-book curated read returned **78,320 of 101,287 rows and still carried
`STATUS=1`**. `STATUS` sits in the `HEADER` at the *start* of the document, so it is emitted
long before Tally knows whether the response will complete. There is **no trailer, no row
count, and no completeness marker anywhere in the response.**

A consumer that checks `STATUS=1` and parses what arrived will silently conclude it has the
whole book while missing 22,967 vouchers. Under §3.1.7's absence-implies-deletion rule that
is mass false tombstoning.

**Required:** completeness must be established by the *client*, from something other than the
response's own claims — expected row count from a prior probe, byte-length agreement, or an
explicit second read. `STATUS=1` proves the request started, not that it finished.

**Tally never enforced a size cap** — it streamed 282 MB without complaint. The 32 MiB limit
is Bridge's own client-side rule. Tally will not refuse an oversized request on your behalf.

### 11b.2 A client timeout does not cancel server-side work — **P0 operationally**

After the client abandoned the 600 s read, **the gateway stayed completely unresponsive for a
further 523 s**, then recovered by itself with no restart. Total server-side occupancy for
that one request: roughly **19 minutes**, of which nearly nine minutes came *after* the
client had disconnected.

Throughout that window every other request failed — including `/status`. Because the gateway
serialises (§1), one abandoned expensive read blocks the entire instance.

**Measured since, for a shorter cut** (30 Oct 2026, `lab/2142-status-while-busy-answers` and
`lab/2142-drain-repeat-answers`: one synthetic book, one long report read cut by the client at
20 s, five times over two sittings; **partial**):
- `GET /status` sent while Tally builds a long read waits until the read ends (twice).
- After the cut, Tally's work ran on 8.9 s to 25.4 s. A `/status` sent about 1 s after the cut
  waited 9.3 s and 10.8 s.
- Once `/status` had answered quickly three times in a row, 2 s apart, a data read answered at
  its at-rest speed (four times).

Bridge's drain after a cut post rests on these readings (`DrainPolicy::MEASURED` in the
transport crate).

**Measured through Bridge's own transport** (30 Oct 2026, `lab/2142-drain-live-answers` at
35d4f151: one synthetic book, one sitting, two runs that agree; **partial**):
- A report read cut at the 20 s limit was refused in about 21.5 s, and the drain was then owed.
- A ledger read sent at once was refused as `tally_endpoint_draining`, `retry_after_s` 10: its
  one `/status` probe was cut at 5 s and its post was not sent.
- The next call, 10 s and 15 s later, went ahead on its first try after three answered probes
  and answered whole, and the drain cleared. The probes' two 2 s pauses made that call about
  4 s longer than at rest (5.3 s and 5.5 s, against 1.0 s to 1.8 s); its read did not wait.

Not measured: a dropped connection, a size limit or an abandoned send in place of the cut; a
second process sending meanwhile; other reads, books and editions.

**Consequences:**

1. **Abandoning a request frees your socket, not Tally.** There is no observed way to cancel
   server-side work once issued.
2. **A user-facing "cancel sync" cannot stop Tally.** Phase 3's "cancellable mid-segment"
   requirement is achievable only as "stop consuming" — the accountant's Tally remains busy
   regardless.
3. **The blast radius is the accountant's own session.** For ~19 minutes that Tally was
   unusable to the person sitting in front of it. This is the failure that generates support
   tickets and destroys trust in a sync product.

**Design rule: never issue a read you cannot afford to wait out.** Segment size must be
chosen by expected *duration*, not only by byte size — a segment that fits under 32 MiB but
takes ten minutes is still unacceptable. The one-month curated read (4,891 rows, 17.7 MB,
7.3 s) is the right order of magnitude; **~5,000 vouchers per request** is a defensible
default, well under the ~9,300 the byte cap alone would allow.

### 11b.3 Filter cost is fixed, not proportional to results

`$AlterID > 190000` matched **zero** rows and still took 22.09 s. `$IsCancelled` matched
**one** row in 22.61 s. Tally evaluates the predicate across the whole collection, so every
filtered query at 101K costs ~22 s regardless of how little it returns.

Server-side filtering saves bandwidth and parsing, **not scan time**. This makes the cheap
company-level `ALTVCHID`/`ALTMSTID` probe (§10) essential rather than an optimisation — it is
the only way to avoid paying 22 s to discover that nothing changed.

### 11b.4 Bytes-per-row is stable and predictable

1,218 minimal · 3,609–3,615 curated (identical across whole-book, one-month and filtered
reads) · 21,532 wildcard. Segment sizing can be derived from a single sample read with
confidence.

## 11c. A windowed voucher read is bounded before it is sent — **rule; the measurements below are VERIFIED, the bound's own requests are UNVERIFIED live**

> **Status 2026-09-21.** The bound's request shapes — the census, its AlterID spans and the
> AlterID-narrowed data parts — are now **VERIFIED live** (§11c.5), and the rectified bound was run
> end to end on one inventory-heavy book. The heading keeps its original wording so existing links
> resolve; §11c.4 lists what is still open.

### 11c.1 What was measured

**VERIFIED**, measured on a licensed lab gateway the week of 2026-09-14. Figures are UTF-8 bytes
unless marked; Bridge's wire is UTF-16LE (§1.2), which costs exactly twice as much.

- **Response size follows vouchers returned, not window width**, with a fixed per-request overhead
  of about 0.15 s. This repeats §11b.4 and §12a.8 on more books.
- **Bytes per voucher depend on the book and on the request shape:**

  | Book | Shape | Bytes per voucher |
  | --- | --- | --- |
  | Synthetic, accounting-only | named ledger-entry fields | ~4.7 KB |
  | Larger accounting book | named ledger-entry fields | ~6.7 KB |
  | Inventory-heavy trading book, whole year (16,367 vouchers, 587 MB) | named ledger-entry fields | **35.0 KB** mean |
  | Same book, one day (41 vouchers) | named ledger-entry fields | 28 KB |
  | Same book, one day | `ALLLEDGERENTRIES.*` entry wildcard | ~128 KB |

- **A one-day probe understated the year on that book** (28 KB against a 35.0 KB mean). A single
  small sample is not a reliable estimator on its own.
- **Half-month windows were barely inside the cap.** With named fields the heaviest half-month of
  that year reached 98.4% of Bridge's 32 MiB transport cap. It succeeded; it had no margin.
- **Month-scale entry-wildcard windows on that book returned hundreds of megabytes and took
  minutes.** A response that large can fail mid-transfer, and afterwards the gateway can stop
  accepting connections for every company for tens of minutes; a client that gives up does not stop
  Tally (§11b.2). The reproducing detail is deliberately not recorded here.

### 11c.2 Why the existing limits do not protect Tally

Bridge's response cap (`XML_RESPONSE_MAX_BYTES`, 32 MiB) and per-leg deadline (20 s) are enforced
on the response, client-side. By the time either trips, Tally has already committed to building the
whole response and will finish it regardless (§11b.1, §11b.2). They protect Bridge's memory and its
caller; they cannot protect the gateway. The only bound that protects Tally is one applied before a
request is sent — which is what §11b.2's "never issue a read you cannot afford to wait out" and
§12a.8's pre-flight count already ask for.

### 11c.3 The rule

Before a windowed voucher read, predict its response and divide the window so that no single
request is predicted over a budget well below the cap.

1. **Budget, and the one margin.** Half the transport cap, in encoded wire bytes: 16 MiB. A part is
   planned at the book's measured cost per voucher, not at an inflated one; the other half of the
   cap is the only allowance for that measurement being wrong. It is sized for the understatement
   measured in §11c.1 (a one-day probe 1.25 times below the year) with room.
   **The floor makes that margin hold.** A measured cost never replaces the default below **half** of
   it. A part planned within the budget at a figure of at least half the default holds at most
   `2 × budget / default` vouchers, so if no voucher in it is heavier than the default — which is set
   above the heaviest cost measured for the shape — the part is at most twice the budget: the cap.
   Without the floor, one light first part (bank receipts at a tenth of an inventory voucher's cost)
   would plan the next part at ten times its safe size. A light book pays in more, smaller parts.
2. **Vouchers in the window, cheapest estimate first.**
   - The company's voucher high-water mark, `ALTVCHID` (§10), bounds the whole book: every voucher
     carries an AlterID no greater than it. If the mark times the shape's conservative per-voucher
     cost fits the budget, no window of the book is predicted to exceed it, and the window is sent **undivided,
     exactly as before**. A company that has never held a voucher omits `ALTVCHID`; that is a zero.
   - Otherwise, a **census** of the window: one row per voucher carrying only `GUID`, `ALTERID` and
     `DATE` (the §12.7 witness fetch), **every census request bounded before it is sent**. A date
     range can hold every voucher the book has, so a date census is bounded only by the mark itself:
     a book whose mark fits one census is counted in **one date census of the window**. A larger
     book is counted in **AlterID spans across `(0, mark]`, each also narrowed to the window's
     dates**, and each bounded by construction because AlterIDs are distinct. The spans are produced
     one at a time, and a mark needing more than 256 of them is refused before any census is sent.
   - **A census is sized against the whole transport cap,** not the half-budget: 8,192 rows at the
     planning figure of 4 KiB a row. The half-budget exists to absorb the error in an *estimated*
     data part; a census span's row count is fixed by construction, so its only uncertainty is the
     per-row figure, measured live at 2.42–2.72 KB on earlier books (§11c.5) and at about
     3.27 KB on the wire (about 1.64 KB decoded) on a book with a mark of about 1.03M (PARTIAL,
     30-Sep-2026, bridge#899). A full census is about 22 to 27 MB of the 33.5 MB cap: about 1.25x
     headroom on the heaviest row seen. Whether a heavier row exists is UNVERIFIED.
   - The cost is a census count proportional to the **book**, not the window: a mark of 250,000 is
     31 census requests however short the window. It is a cost in elapsed time, not in gateway
     safety: every request stays bounded, so a caller that abandons the read leaves at most one
     bounded request in Tally. An earlier revision sized a date census from the
     book's average density; that size was an expectation, learned too late when it was wrong, so it
     was withdrawn. A request returning only a count per date range would avoid the span walk, but no
     such shape is qualified live.
   - A census Tally does not serve — oversized, or timed out — is not divided or retried: every
     census is already bounded, so an oversized one means the per-row figure was wrong, and a
     timed-out one may still be building on the gateway. The window is refused as unestimated.
   - A caller already holding such a witness count for the same window may supply it instead, and
     no census is sent. A count that names a day outside the window is refused.
3. **Bytes per voucher for the request shape.** Until something is measured, a **conservative
   per-shape default** above the heaviest figure measured for that shape: 96 KiB per voucher on the
   wire for named ledger-entry fields (48 KiB of UTF-8, against a 35.0 KB measured mean) and 384 KiB
   for the entry wildcard (192 KiB of UTF-8, against ~128 KB measured). It sizes only the **first
   part**, which is real data, not a discarded sample. That part's measured cost then replaces the
   default for the rest of the window; each later part heavier than the planning figure raises it,
   and a lighter one never lowers it again. Nothing is remembered between calls.
4. **Divide by date, then within a day by AlterID.** Days are packed in order into contiguous
   date ranges, each predicted within the budget; a day without vouchers joins whichever range it
   falls in. A day too heavy for one read is read alone, in AlterID spans of that day that together
   cover every AlterID up to the high-water mark. Reading every part observes the same vouchers one
   undivided read would, provided the book does not change during the read (rule 6).
5. **Refuse by name, before any further data read,** rather than send:
   - `voucher_window_part_over_budget` — a single voucher is predicted over the budget, so no
     division can help.
   - `voucher_window_too_many_reads` — the read would dispatch more than 128 data requests. The
     allowance is spent **when a request is dispatched**, not when a plan is made, so a part divided
     after Tally could not serve it — and the failed attempt itself — count against it.
     The refusal carries `reads.needed_at_least` against `reads.allowed`. A census read in several
     AlterID spans stops as soon as it has counted more vouchers than the allowance can hold at the
     default figure (the allowed requests times the vouchers one request holds at it, which every
     first plan uses), so that certain refusal costs no further census span (#945).
     The tax audit's read is the exception, because its period cannot be narrowed (#2173). It is
     refused on the count before any data part only when even the plan it would make after a first
     part measured at the floor (half the default figure, the lightest it can use) needs more than
     128 requests: 42 vouchers in the first request and 85 in each after it, at most 10,837
     entry-wildcard vouchers in the period, and fewer when its days are dense, since a request
     packs whole days. The audit's parts must tile its period by whole days, so a day no request
     can carry whole is refused before any data part as `audit_window_divided_by_alterid`: the first
     day that holds vouchers, over 42, or any later day over 85. A first day of 40 and then 127 days
     of 85 fit (10,835 vouchers in 128 requests); a book with 82 vouchers on its first day does not.
     Otherwise it reads the first part and is refused, if at all, when a measured part's re-plan
     needs too many, or divides a day. Before a part is measured it sends no more than the parts its
     admission counted. Every request keeps the same size bound; only the moment of the refusal
     moves. Its census stops at that floor bound, not the default's. Under Education the boundary
     check still covers every part of the plan made at the default figure before the first part is
     read, so such a period can be refused for an unsupported boundary instead.
   - `voucher_window_book_too_large` — the book's mark needs more than 256 census spans (a mark above
     about 2.1 million). A **product limit**: narrowing the window does not help, because the census
     walks the book's AlterIDs whatever the window.
   - `voucher_window_volume_unestimated` — a census could not be read, was refused as oversized, or
     timed out. A mark that cannot be read is refused under the mark parser's own code.
   - `window_part_boundary_unsupported_in_education` — the endpoint is in Education mode and a part
     still to be read starts or ends on a day other than the 1st, 2nd or 31st. Education serves a
     read **starting** on such a day as a well-formed empty collection, not an error (bridge#581,
     lab capture 2026-09-22); the end side is unmeasured in the voucher shapes and held to the same
     rule. The mode is read from the `EDUMODE` field of the `CompanyListV2` response that brackets
     every read, so it costs no request. The whole remaining plan is checked before each part, so a
     divided read is refused before its first part, and the runtime refuses any single read with
     such a boundary before sending it, or after it when only the closing bracket reports Education.
     An `EDUMODE` other than `No` counts as Education; a list with no `EDUMODE` keeps ordinary
     boundaries. `EDUMODE = Yes` was observed live on 2026-09-22, alongside `SILVER = Yes` and
     `GOLD = No`, so Education still reports Silver and the mode is read from `EDUMODE` alone.
6. **Every part is admitted, and so is their union.** Each row of a part must lie in the part's dates
   and AlterID span. When the window was counted, a part's vouchers must be **exactly** the ones the
   census counted for it, by AlterID and GUID; a window counted and then read whole, in one part, is
   admitted against its census the same way, not only a divided one (#985) — a matching count is not enough, because a substituted
   voucher preserves it. And GUIDs and master IDs must be unique across the union of parts, not only
   within each response: a voucher re-dated between two parts is returned by both, each valid alone.
   Either failure refuses as `voucher_window_part_not_admitted` or
   `voucher_source_identity_invalid`. A part refusal names its `cause`: `part_row_unreadable`,
   `part_row_outside_dates`, `part_row_outside_alter_id_span`, `part_row_duplicated`, or
   `part_census_mismatch`, which also carries `counts` (`returned` against `counted`) so that an
   empty part reads as "returned 0 of N".
7. **A divided read is bracketed on both marks.** `ALTVCHID` **and** `ALTMSTID` are read again after
   the last part, and either moving refuses the read as `voucher_window_changed_during_read`. Live
   (§11c.5): creating, altering, cancelling, re-dating and deleting a voucher each advance
   `ALTVCHID`, but renaming a ledger advances only `ALTMSTID` while changing that ledger's name in
   every voucher export. A read bracketed on `ALTVCHID` alone returned `complete` across a rename
   made between two of its parts. Every divided read is bracketed — including one planned whole and
   divided only after Tally could not serve it. An undivided read is one observation, as before.
8. **A corroborating second read replays exactly the first read's parts and carries its witness** —
   it never re-plans from a measurement, which could merge parts back into a request the first read
   found Tally could not serve. The witness is the marks the first read opened and closed on, and
   the census its parts were admitted against. The replay is admitted against that census and
   closes against those marks. Without them, a replay of
   AlterID-limited parts reads only up to the first read's ceilings: a voucher posted in the window
   between the two reads takes an AlterID above them, both reads miss it, and their responses match
   byte for byte. A replay of a divided read without a witness is refused as
   `voucher_window_replay_unwitnessed`. The import-verification read is not replayed at all when it
   was divided, every part was admitted against a census naming each voucher's GUID, and the marks
   read after its last part equal the marks it opened on (#1241): each part is already a paired
   read that matched the census voucher for voucher, and each create, alter, cancel, re-date and
   delete moved a mark every time it was measured (§11c.5; §11c.4 lists what was not tested), so the
   replay could only observe a later state. A window read whole, or divided with no census, is still replayed.
9. **A caller that must send the undivided request itself decides on what was measured.** The
   pre-post check inside the import dispatch lease sends the whole verification window as one
   request. Before approval it is admitted on the `verify_import` read of the same window that runs
   just before: an undivided read was that request, and a divided read's parts together (one copy of
   each response) are its size, which must be within the budget — unless Tally refused one of that
   read's requests as too large or timed out, since a timed-out request has no size to sum and the
   refused one may be the whole window itself. Otherwise the batch is refused as
   `import_post_window_not_bounded`. The book can still grow between that read and the lease; the
   transport cap is the backstop there.
10. **The transport cap remains the final safeguard.** Where a caller already divided a window after
   a deadline or an oversized response (#485, the import-verification read), it still does: by date,
   then, for a single day, by the day's counted AlterIDs.

### 11c.4 What this does not establish

- **The rectified bound was run end to end live only twice.** A build of the tree committed as
  `986c1d77` made the two calls timed below (§11c.5). Two changes made after review — a replay
  that no longer re-plans, and the measured pre-post admission — are established by simulator and
  unit regressions only.
- **Latency on a large book.** Measured end to end on the rectified bound (§11c.5), on an
  inventory-heavy book with a mark of about 250,000 (31 census spans): a one-day `vouchers` call took
  34 s (7.5 s before the rectify) and a one-month `ledger_movement` 105 s (53 s before). The MCP
  host's own timeout is not measured.
- **The import-verification read's second read is skipped for a divided, counted, bracketed read
  (#1241).** That rests on the marks moving for every change that could alter what a window
  returns, which §11c.5 measured for gateway writes and the screen actions listed there, once each;
  changes made in the Edit Log SKU, in Education, by synchronisation, or by another connector on a
  multi-user book were not tested. A change that altered a voucher's export without moving either
  mark, made between two parts, would be caught by the census (a part holds the counted
  AlterIDs and GUIDs) only if it also changed the voucher's AlterID or GUID. **Confidence: PARTIAL**:
  the request counts are from scripted doubles through the tool call; no live Tally was read, and the
  saving in seconds was not measured here (the issue (#1241) states 93.4 s to verify a 4,972-voucher
  window against 48.8 s for its build read).
  `ledger_movement` keeps its second read: it compares two ledger catalogues taken either side of
  the voucher read, and skipping the replay would leave the second catalogue outside the marks
  bracket.
- **An abandoned call is withdrawn before its next Tally operation, never during one.** The stdio
  server keeps reading input while any tool runs (#554). A `notifications/cancelled` naming the call
  stops its queued operations before the next one starts and the call is answered as withdrawn; a
  request already sent to Tally is not abandoned, because abandoning a request does not stop Tally.
  A `lab_import_*` write is never withdrawn, so its mandatory read-back runs. A closed input is not
  a withdrawal: the call runs to completion and the session ends after it. `post_import` has its own
  withdrawal (#725), before its dispatch intent only. This is from reading the code
  (`agent_protocol.rs`); no Tally was involved, and what a host sends is in section 11f.
- **`ALTVCHID` and `ALTMSTID` moving on every change** was observed only for changes made through
  the XML gateway, once each, on one release (§11c.5). Changes made in the Tally UI, the Edit Log
  SKU and Education mode were not tested.
- **The first part is one place in the window.** A book whose later vouchers are much heavier than
  its first part's is protected only by rule 3's raising and rule 1's margin. Live, three equal-count
  spans of one day on an inventory-heavy book differed 1.8× in bytes.
- **A mark that is loose as a density prior.** On the inventory-heavy book the mark was ten times the
  voucher count. It is still a correct upper bound; it only makes the whole-book shortcut rarer.
- **The empty-window corroboration reuses the first read's marks.** Its widened read (±1 day) opens
  on the marks the window's own read observed. On a book large enough to be counted it is a census of
  its own and no read of the widened window's vouchers (#1240): the census carries each voucher's day,
  which is all the control needs. A census sent in several requests is bracketed, and then a change
  anywhere in the company since the marks the first read opened on refuses it; one sent in a single
  request is not. Before #1240 a widened read that the book needed in several data parts was bracketed
  too, so a book counted in one census with a busy neighbouring day was refused on moved marks and now
  answers from the one observation; and a widened window whose data plan exceeded the read allowance was
  refused at planning, which no longer applies because no data part is sent (a census of several spans can
  still refuse mid-census when its count passes the allowance). A book that fits one request is read whole, uncounted, as before. Either way a voucher in the
  window refuses the read as `window_contradicted` (a census of several requests whose marks moved refuses as
  `voucher_window_changed_during_read` first), and otherwise the empty result is corroborated or
  reported partial by the existing empty-window control. **Confidence: PARTIAL**: one live run (26 Oct 2026,
  TallyPrime 7.2 Silver, two synthetic books; `RESULTS.md` on the branch `lab/1240-empty-window-live-answers` at
  `ce849cc6`) gave the same state, reason and items as the previous build on all five windows read. On the book
  counted in one request it sent 28 and 34 requests where the previous build sent 34 and 40, as the doubles predict.
  On the book counted in four requests it sent 76 against 76 where the widened window was empty, and 70 against 88
  (475 KB of answers against 6.08 MB) where a neighbouring day held vouchers, where the doubles predicted equal
  counts; which of the previous build's requests are saved there is not established, since the trail keeps only the
  last 32 sends. The counts that follow are from scripted doubles through the tool call: for a neighbour-empty widened window
  a census of one request replaces one paired window read and so saves six requests (a census of
  several requests adds a closing marks read, so a small widened window saves none), and with vouchers on the
  neighbouring days it saves every part those days would have taken. The saving is counted in requests;
  the bytes of a census and of the parts it replaces were not compared.
- **AlterID 0.** Every span starts above an exclusive lower bound of 0, so a voucher with AlterID 0
  could not be read by a divided day. None has been observed; a census row carrying AlterID 0 is
  refused rather than planned around.

### 11c.5 Live evidence, 2026-09-21

**VERIFIED** on a licensed TallyPrime 7.1 Silver lab (the last licensed day), one request at a time,
`/status` probed before and after each, on four synthetic companies and one inventory-heavy client
book (read-only there). The request strings were the branch's own at `cf618c00`.

| Check | Result |
| --- | --- |
| Census row | `DATE` and `ALTERID` are direct children of `VOUCHER`; 2.42–2.72 KB per row in UTF-16 on the earlier books, and about 3.27 KB on one later book (§11c.3; about forty default fields come with each row); an empty window parses to zero rows with `CMPINFO`'s `<VOUCHER>0</VOUCHER>` not counted |
| Census count equals the full read | yes, on every whole-day window tried, in every shape |
| `ALTVCHID` as a count bound | the count never exceeded the mark and no AlterID exceeded it, on every book: 67/90, 2,790/2,797, 29,900/29,900, and 23,634 against 249,948; a company with no vouchers omits `ALTVCHID` |
| AlterID spans, three shapes | three contiguous spans of one day, in the entry-wildcard, movement and import-verification shapes, on three books: disjoint, their union equal to the unspanned read and to the census, no row outside its span; every read under 1.6 s |
| Charset | the same 1,230-voucher read is exactly twice as large in UTF-16 as in UTF-8 |
| `ALTVCHID` on each change (gateway writes, synthetic company) | create +1; alter (re-post with the same client `REMOTEID`) +1; cancel (official `TAGNAME` shape; the counter that moves is `ALTERED`) +1; re-date +1; delete +2; a no-op re-post +1. A ledger rename: `ALTVCHID` +0, `ALTMSTID` +1 |
| End to end (`bridge_mcp` @ `cf618c00`, before #520) | whole-FY, one-day and one-month `vouchers`, a quarter and a month of `ledger_movement`, on three books: complete, every part under 16 MiB and 2 s. A voucher created between two parts refused the read as `voucher_window_changed_during_read`; a ledger renamed between two parts did not (fixed by rule 7) |
| Paired reads | every Tally request is sent twice, back to back (the repeated-source read), so wire traffic is about twice the data |
| End to end after the #520 rectify (census spans of 8,192; a build of the tree committed as `986c1d77`) | inventory-heavy book, mark ~250,000: one-day `vouchers` complete in 34.3 s (31 census spans, 2 data parts); one-month `ledger_movement` complete in 104.7 s (31 census spans, 5 data parts of at most 6.6 MB, and the replay closed against the first read's marks). Every request under 16 MiB and 2 s |

**A screen action whose prompt said it would update stored data, and that moved neither mark — PARTIAL, one run, TallyPrime 7.2, reported by the lab, 2026-10-25.** The bytes are on the unmerged lab branch `lab/gs1-x1-answers` (commit `4453dd7b`); the prompt, the marks and the one changed counter below match its saved answers. Go To, then GST Annual Computation, showed a prompt that it would update the GST status of 20 vouchers; the answer was Yes; `ALTVCHID` (65) and `ALTMSTID` (293) were the same after as before, and only the `CMPINFO` `VOUCHER` counter went from 0 to 40. No read of any voucher's GST status was made before and after, so that a status changed rests on the prompt alone. That counter is not a count of the book's vouchers: the committed extent answer of a book with a six-figure voucher mark carries `VOUCHER` 0 (and every other counter 0), and what the `CMPINFO` counters report is recorded below as not established; they appear to count what Tally has loaded, so 0 to 40 probably says that voucher objects were loaded, not that 40 were changed (UNVERIFIED). Not established: whether a read through the gateway can trigger the update, whether 7.1 does the same, what else the update changes, and whether a Bridge read returns a changed status.

**`ALTVCHID` on a multi-voucher import, and on screen edits — PARTIAL, 2026-09-24.** Measured on a
licensed TallyPrime 7.1 Silver lab, on a synthetic company, one run of each. The requests were lab
scripts that posted Bridge's voucher shape straight to the gateway, **not** Bridge's post path. The
captures are not committed.

| Change | Counters | `ALTVCHID` |
| --- | --- | --- |
| One import of 50 new Journals | CREATED 50 | +50 (6 → 56) |
| One import of 200 | CREATED 200 | +200 (→ 256) |
| One import of 500 | CREATED 500 | +500 (→ 756) |
| The same 50 posted again with the same `REMOTEID`s | CREATED 0, ALTERED 50 | +50 (→ 806) |
| 50, one of them naming a ledger that does not exist | CREATED 49, EXCEPTIONS 1 | +49 (→ 855) |
| 50 of mixed type: 13 Journal, 13 Payment, 12 Receipt, 12 Contra | CREATED 50 | +50 (→ 905) |
| Three edits by hand on Tally's screen to one voucher: its amount, the field the screen labels Reference, its narration | none (no import) | +1 each (3 → 4 → 5 → 6) |

In these runs the mark moved by exactly the number of vouchers created or altered, including a
partial commit, and each screen edit moved it by one. This does not establish:

- the step through Bridge's own post path, which a batch post must measure before it gates on it
  (since measured, below);
- that nothing else can move the mark within the same interval;
- a multi-user book with edits made while an import runs.

**The multi-voucher step through Bridge's own post path — VERIFIED, 2026-09-26.** One run of each,
on a licensed TallyPrime 7.1 Silver lab, on a quiet synthetic company with automatic Journal
numbering. Each was one native `post_import` of one saved batch, after one approval, through a debug
build of the batch-posting branch. The 200-voucher run raised only the batch-size cap for the
measurement. The captures are not committed.

| Batch | Counters (one POST) | `ALTVCHID` | Readback |
| --- | --- | --- | --- |
| 50 Journals | CREATED 50, all else 0 | +50, equal to CREATED | 50 verified; AlterIDs contiguous |
| 200 Journals | CREATED 200, all else 0 | +200, equal to CREATED | 200 verified; AlterIDs contiguous |

A second read a minute later gave the same verdict. A multi-user book, other voucher types and other
licence tiers are not measured.

**What screen actions move the marks — measured 2026-09-26.** One run of each, on a licensed
TallyPrime 7.1 Silver lab, on synthetic companies. Each person's action on screen was followed by
Bridge's own reads, and nothing else was changed in between.

| Action on screen | `ALTVCHID` | `ALTMSTID` | Confidence |
| --- | --- | --- | --- |
| Open a posted voucher in alteration, save with no change | +1; the voucher's AlterID moves | 0 | VERIFIED |
| Change a bill allocation's reference | +1; AlterID moves | 0 | PARTIAL |
| Add a cost-centre allocation | +1; AlterID moves | 0 | PARTIAL |
| Set a bank-reconciliation date | +1; AlterID moves | 0 | PARTIAL |
| Cancel a voucher (Alt+X) | +1; AlterID moves | 0 | VERIFIED |
| Mark a voucher optional (Ctrl+L) | +2 (its own AlterID +1; the other step unexplained) | 0 | PARTIAL |
| Delete a voucher (Alt+D) | +2 | 0 | VERIFIED |
| Enable cost centres, create one, make a ledger applicable | 0 | +3 | PARTIAL |
| Regroup a ledger; change its opening balance; create a ledger; delete it | 0 | +1 each | VERIFIED |
| Rename the base currency, then restore it (a company with no vouchers) | absent throughout | +2, then +1 | PARTIAL |

Further observations from the same runs:

- **Invisible fields.** Bridge's import-verification read returns a bill allocation's amount only,
  not its name or type; a cost-centre allocation as an empty `CATEGORYALLOCATIONS.LIST`; and no bank
  allocation. An edit to only those fields shows up as the AlterID moving, not as a field change.
- **Tally allocates bills itself.** A native import to a bill-wise party ledger read back with a bill
  allocation of the full amount already on that line.
- **Cancelled and optional vouchers read differently.** A cancelled voucher keeps its number but
  loses its ledger entries in the readback. Bridge labelled it `posted_divergent` at the time
  (bridge#758); since bridge#771 it reads as `posted_not_effective` (`voucher_cancelled`). An
  optional voucher's number changed when it was marked optional, and again, with no AlterID
  change, when a later voucher of the same type was posted. Its number is not a stable identity.
- **A voucher held open in alteration** blocked none of Bridge's reads, builds or posts, and Tally
  raised no dialog.
- **Emptied is not never-used.** A company that has never held a voucher omits `ALTVCHID`. After
  vouchers are entered and all deleted, it keeps it (+1 per create, +2 per delete).
- **Currency rename.** A Company Alteration rename of the base currency (symbol and formal name)
  changed the currency master's `ORIGINALNAME` and the company's `CURRENCYNAME` with it, both ways.
  A non-INR company given a ₹ symbol was not measured.

**A window read whole and admitted against its census (#985, #1020) — PARTIAL, 2026-10-02.**
Run by the maintainers on a licensed TallyPrime 7.1 Silver lab, through Bridge's own tools, on
synthetic lab books, one-day windows. The reads ran on builds of two #1020 heads, `5797c9ec` and
`ff50b571` (before it merged master), with the same results on both; the post ran on `ff50b571`
only. One run of each. The requests are described from each tool's own request trail (counts,
hashes and sizes); no request or response was captured into this tree.

When a census is read (rule 2 of §11c.3) is derived from the code, not measured: the planner reads
one only when a book's voucher mark times the shape's default size per voucher exceeds the 16 MiB
budget. That is
a mark of 43 or more for the `vouchers` and `voucher_presence` shape (384 KiB a voucher) and 171 or
more for the import-verification shape (96 KiB a voucher) that the read-back after a post uses.

**Deviation, 6 Oct 2026 (#1029).** The two paragraphs above, and rule 2 of §11c.3 ("sent undivided,
exactly as before"), describe the build the table below was measured on. Since #1029 `vouchers`,
`voucher_presence` and the registers read a census on a small book as well, whatever its mark (a mark
of zero is never counted), so the "no census" rows below describe the earlier build for those tools.
`ledger_movement`, the read-back after a post, `outstandings` and the widened read that corroborates an
empty window still read a small book undivided and uncounted. The 22 and 34 request counts for a small
book came from scripted doubles and were then measured on one synthetic book of eight vouchers on 7 Oct 2026
(a `vouchers` call 16 to 22 requests, `voucher_presence` 28 to 34, every request served; the six more are two
census reads, two company-extent reads and two status reads). **Confidence: PARTIAL**: one book, one run of the
day and week calls and three of the presence call, debug builds, windows of one voucher. This change does not alter a larger
book's path (from the code); a larger book's census cost was not measured here.

| Call | Book | Window label | Verdicts | Requests |
| --- | --- | --- | --- | --- |
| `vouchers`, a day with one voucher | census read | `complete`, total 1 | | 22 |
| `vouchers`, the same | no census | `partial`, `nonempty_window_unqualified`, total 1 | | 16 |
| `voucher_presence`, 2 proposals | census read | `complete` | the voucher in the book `present`; one nothing in the book resembles `absent` | 34 |
| `voucher_presence`, 2 proposals | no census | `partial`, `nonempty_window_unqualified` | the voucher in the book `present`; one sharing its party `possibly_present`, `resembles_book_voucher` | 28 |
| `voucher_presence`, 1 proposal whose party is bound and is not the book voucher's | no census | `partial`, `nonempty_window_unqualified` | `possibly_present`, `window_not_proven_complete`, no candidates | 28 |

- No `absent` was issued on a `partial` window.
- The `vouchers` call on the census book sent the same requests as a build without #1020, in the
  same order, with the same request hashes and response sizes.
- On `5797c9ec`, an empty day read `partial` on both books, as before #1020:
  `empty_uncorroborated` on the census book, and, on the other, `nonempty_uncorroborated`
  (the reason as observed; since renamed `empty_window_wider_read_nonempty`). From the code, not the run: that reason is returned when
  the read widened by a day on each side finds rows, none inside the window, so it names the
  neighbouring days, not the window, which returned no row; the name says so.

**The read-back after a post.** One synthetic Journal, posted through `build_import_xml`,
`post_import` and `verify_import`, on a lab book with one currency and a voucher mark of 1,794, so
the read-back read a census. `post_import` returned `posted_verified`: created 1 and every other
counter 0, one import sent and not resent, and the voucher mark moved by exactly 1. On a day empty
before the post, the census answer grew in size from the empty answer's, as one holding a voucher
would; that a census was read is taken from request and response sizes in the tool's own request
trail, not from captured bytes. The read was admitted against it and returned complete.
`verify_import` then returned `posted_verified`, 1 verified, none unverified and no duplicate,
reading the window again with the same request and response hashes. No `voucher_window_part_not_admitted` was returned on a book
nobody changed.

**Where a one-month window read spends its time, and what a later page costs — PARTIAL, 2026-10-02.**
One run of each, on a synthetic benchmark book (one month, 2,542 vouchers, about 82 a day), the `vouchers`
tool of a master build through a recording read-only relay, licensed TallyPrime 7.1 Silver.

- A whole-window read took 65.8 s in a release build (75.6 s in a debug build) and sent 232 requests. Tally
  held 45.5 s of it, Bridge's request spacing about 19.1 s, Bridge's own CPU 2.4 s. Of the relay's Tally seconds, the
  data parts took 36.7 s and the census 5.2 s: 55.7% and 7.9% of the 65.84 s the call took (computed from those three figures
  of the same run). In 57 relay logs of paired reads on quiet synthetic books, the two sends of a pair never differed.
- Before the page snapshot change (#1053), every later page of the same window read the whole window again:
  232 requests each. With it, in one run, a later page that named the first page's `snapshot_id` took 1.2 s and
  sent 10 requests (0.18 s of it Tally's), against 68.0 s and 232 requests for the first page, on a quiet book.
- Not measured: a change made at Tally's screen between two live pages (the screen actions that move the marks
  are in the entries above, one run each), a window larger than the 64 MiB the held windows share, another book,
  another release, the desktop screen (it holds nothing), and the effect of any change to the request spacing.

This does not establish:

- a window of several days read in one request and admitted against its census, which is the case
  `voucher_presence` is mostly asked about (ADR 0017's note of 2026-10-02);
- the refusal itself: no run produced a disagreement between a census and its read, so what a
  caller sees then, after a read or after a post, is shown only by #1020's own tests;
- how often that refusal fires on a book several people are changing;
- Education mode, or tiers and releases other than Silver 7.1;
- the voucher types' own numbering: the caller declared them `manual`;
- anything beyond one run of each, on synthetic lab books.

### 11c.6 One ledger's vouchers can be read alone, with the same spans — **VERIFIED once, 2026-10-28; TallyPrime 7.2 Silver, one synthetic book**

**Scope: licensed TallyPrime 7.2 Silver, `BRIDGE CORPUS DENSE` (about 30,000 vouchers), read only, one request at a time.** A collection of `<TYPE>Vouchers : Ledger</TYPE>` with `<CHILDOF>ledger name</CHILDOF>`, the window formula of the book-wide reads and the AlterID span of §11c.3, answered in band. Captures in `tests/fixtures/ledger-scoped-vouchers/` (`PROVENANCE.md`); lab record `lab/485-ledger-filtered-vouchers-answers` and `lab/485-ledger-census-spans-answers`.

- **The census of a day.** `FETCH` `GUID,ALTERID,DATE` of DN Sales for one day: 82 vouchers in the first span of 8,192 AlterIDs (201,862 bytes) and none in the next three (an empty collection, byte for byte the same each time). The book-wide read of that day holds the same 82 GUIDs. The lab recorded the ledger-scoped nine-field read of the same day at about 8% of the bytes of the book-wide read of the day (part D of its run).
- **A year.** DN Party 001 over 1 April 2025 to 31 March 2026: 67, 68, 69 and 44 vouchers in four spans, 248 in all, the same GUIDs as the unspanned ledger-scoped read of the year (not a book-wide read); every row's AlterID is inside the span asked for and its date inside the window.
- **The movement shape.** The movement `FETCH` of Bridge's movement read, `CHILDOF` DN Party 001 for May 2025, without a span: 21 vouchers, the same as the census rows of that month, whose ledger lines sum to the committed one-month Trial Balance's debit for the ledger exactly (pinned by a test).
- **Request shape.** The collection and the request header carry the same name (`Bridge Agent Ledger Census`, `Bridge Agent Ledger Vouchers`), as the book-wide requests do for their own, so the request-ID check holds. `CHILDOF` takes a formula in Tally's own use (`$$GroupSundryDebtors`), not a quoted literal, so no character set makes an arbitrary name inert. The builders therefore take a `LedgerScopeName` admitting only what was observed: ASCII letters, digits and single spaces; every other name is refused, not escaped.
- **The bill trail's fields (#1356).** The same ledger-scoped read with the window read's `FETCH` list as it stood before `REFERENCEDATE` was added, and no `COMPUTE` line, compared voucher by voucher with a book-wide read of the same window in the same sitting (the window read with its class `COMPUTE` lines, before `REFERENCEDATE`): on `BRIDGE BILLTRAIL LAB`, `Lab Party P1` over the year 2025-26, 7 of 7 vouchers equal in their own fields and in every `ALLLEDGERENTRIES.LIST` of the ledger, bill allocations included, among them three Receipts whose party ledger is the bank (a filter on the voucher's party ledger drops those three); on this book, DN Sales on 15 May 2025, 82 of 82. Captures in `tests/fixtures/ledger-filtered-allocations/` (`PROVENANCE.md`); lab record `lab/1356-ledger-filtered-allocations-answers`. On both, every voucher of the window carried the ledger, so the read cost about 96% of the book-wide read's bytes.
- **Not established:** a name with any other character (none was read), including a name holding a TDL operator word or that Tally could read as an expression; a name that matches no ledger (whether the answer is an empty collection or something else); whether a voucher with two lines on the ledger is listed twice; a census without a span and a movement read with one (neither was sent); the window field list with `REFERENCEDATE` (the lab read the list without it ledger-scoped for the bill trail, #1356, on two books, by requests that match the builder's byte for byte; the bullet above); a book other than this one, a release other than 7.2 Silver, a book where one ledger's vouchers are most of the book; the read under concurrent screen activity.
- **What `ledger_movement` sends from this (#485).** Only the two observed forms: the census of the ledger's vouchers, always in a span (a book that fits one census request is asked over `(0, high_water]`), and the movement read without a span. It builds the name from the catalogue's own spelling of the ledger it resolved, never from the argument, and only when the type admits it: a name holding a word a TDL formula reads as an operator (`FORMULA_WORDS`: `and or not is in contains like between starting ending with equal greater less than`, any case, as whole words) or any character outside the alphabet reads the book's window instead, because `CHILDOF` takes a formula and no such name has been read. It never sends the window field list (the bill trail's, #1356). It does not send a part with an AlterID span, so a ledger-scoped plan that needs one (a day too heavy for one read) is not sent: the book's window is planned instead. Each of these is said in the result (`window_scope`, `whole_book`, with a reason). A ledger-scoped read that finds no vouchers is not taken as proof that there are none: the book's window decides. A row with no line on the ledger refuses the read, a repeated GUID is refused by the union check, and a ledger-scoped census is a count of that ledger's vouchers only, never of the book (it earns no bracketed count). The company-wide high-water bracket still closes a divided read, so a posting on any ledger refuses it. **Not yet read end to end on a Tally**, and not compared with the book-wide read for post-dated, optional, cancelled, zero-amount or invoice-allocation vouchers, nor for a ledger that holds most of the book: a voucher that Tally leaves out of its list for the ledger would not be seen by the tool. The lab row (`ledger_movement` over the cases above on the dense book) is what establishes it.
- **What the `outstandings` party detail sends from this (#1356).** Only after the book's window was refused as needing more requests than one call may spend, the same scoped window read as `ledger_movement`, with the window field list in place of the movement fields: the census of the party's ledger, always in a span, then the planned parts, each the unspanned ledger-scoped read. Only for a ledger matched alone whose name the party's vouchers carry as stored and the type admits (no TDL operator word). The result says so in `window_scope` (`ledger_scoped`, with what was compared). A ledger not read, a read that finds no vouchers, or a plan that needs a part with an AlterID span leaves the book's refusal standing, with `window_scope` `whole_book` and the reason; a part not admitted, a row with no line on the ledger, and every other failure of the ledger read are its own refusals, code kept.

## 11d. Education refuses Bridge's report-family TDL with a blocking dialog — **VERIFIED live for `ledgers_v1`, 2026-09-22; the rest inferred**

On a TallyPrime 7.1 instance in Education mode, `ledgers_v1`'s custom report, whose `$$` function
takes a spaced identifier (IMPLEMENTATION_GUIDE I1), raised a modal **Error** dialog about its
formula and sent no response. The dialog holds the XML gateway until someone dismisses it on the
Tally screen. From the network, that looks like a busy or dead gateway (bridge#45). A ledger
`Collection` export with no formula, sent to the same instance, returned `STATUS 1` promptly, with
no dialog.

Only `ledgers_v1` was observed. Two things are **inferred**: that the other builders using the same
construct raise it too, and that the space in the argument is the cause. Those builders are every
`function-argument-with-space` entry of `scripts/check-tally-request-builder-hazards.mjs`: the
ledger canary. (`vouchers_v2`, which this list once named, was deleted 2026-10-12, Part of #1570.
The period-balance report it named: this read was removed by #1643; see #1642.)

All of them are refused **before sending** once Bridge knows the endpoint is in Education, with
`education_report_family_unsupported`. Admission is unchanged: nothing is sent, and nothing is
promoted.

- **Sync period-balance tie-out.** Reads the mode from the `CompanyListV2` identity read that
  already precedes it, or from the run's own probe. The run carries on and records
  `report_tie_out_unavailable` and `education_report_family_unsupported`. A later window's
  successful report clears the second code, as it does the other tie-out codes. (2026-10-12: this
  read was removed by #1643; see #1642.)
- **Selected-ledger and selected-voucher qualifiers.** Deleted with the rest of their runtime
  path (bridge#732; ADR 0015's 2026-09-26 amendment): no command or tool reached them.
- **The live-read tool.** Takes its mode from its configuration; it does not observe it. In
  Education, the ledger step is recorded as failed with `education_report_family_unsupported`. Its
  two voucher steps were deleted 2026-10-12 (Part of #1570), so the run ends after the ledger step.
  - **Residual:** a run configured as `Licensed` against an endpoint that is actually in Education
    still sends `ledgers_v1`. The tool's company read (`CompanyListV1`) carries no `EDUMODE`. Moving
    it to an observed mode means changing its profile sequence, which is not done here.
  - 2026-10-21: the live-read tool was removed with the rest of the `tools/` workspace (#1745);
    nothing sends `ledgers_v1` or `CompanyListV1` from it any more.
- **The native-outstandings qualification.** Accepts only an Education configuration, and its
  identity brackets read through `ledgers_v1`. It therefore refuses on load, so the tool **cannot
  run** until Phase 2 Unit A. (2026-10-21: removed with the live-read tool, #1745.)
- **`groups_request`.** Had no production caller and compiled only for tests. Deleted 2026-10-12
  together with the Bridge-schema group and voucher-type parsers that read its
  answer; core groups come from the native group collection.

Reading ledgers and vouchers in Education waits for the Collection-based profiles (Phase 2 Unit A,
`IMPROVEMENT_PLAN_2026H2.md` §8.12).

**Unmeasured:**
- whether Education accepts `$$` functions whose arguments contain no space;
- whether Education accepts a `COMPUTE` of `$GUID:Company:##SVCurrentCompany`;
- whether Education accepts `CompanyListV1`, a custom report with no `$$` function that the
  live-read tool sent before its removal (#1745). (2026-10-31: no longer asked; Bridge builds no
  such report since #1872.)

## 11e. Ledger master read sizes and part reads on two books — **PARTIAL**

**PARTIAL, 2026-09-29.** One licensed TallyPrime 7.1 Silver book of 1,989 synthetic ledgers
(`ALTMSTID` 2,197, one period, no vouchers), read by scripts, not by Bridge's read path. Every
size below is wire bytes: the responses are UTF-16LE, so the file size is the byte count Tally sent.

| Read (whole book) | Bytes | Bytes per ledger | Time |
| --- | --- | --- | --- |
| Balance-free ledger catalogue | 2,195,342 | 1,104 | 0.35 s |
| Ledger balances | 2,468,320 | 1,241 | 0.42 to 0.46 s |
| Party-ledger master | 5,718,512 | 2,875 | 0.86 to 1.00 s |
| Groups | 59,520 | not per ledger | 0.06 s |

- **Repeat reads were byte-identical.** Each paired read returned the same bytes twice, and the
  catalogue read at three different times agreed with itself.
- **Row cost depends on the row.** A ledger with party fields cost about 3.2 to 3.3 KB in the
  master read and a plain ledger about 1.9 to 2.0 KB, so 2,875 bytes per ledger is a mean for this
  synthetic mix, not a bound. The 3,750 bytes per ledger Bridge budgeted (#637; 5,000 for a read that names the mailing details, #1490) sits above it.
- **A part read of the master is cheap (the #679 go/no-go, GO).** A 20-ledger part took 0.037 to
  0.122 s, 4 to 14 percent of the whole master read, against a no-go line of 50 percent. 150
  ledgers took 0.08 s, 300 took 0.14 to 0.22 s and 907 took 0.38 s: time tracked the row count
  plus a fixed envelope of about 0.03 s. Parts split by parent group worked for a parent name
  containing `&`, a nested parent and the reserved root, and balance parts worked too.
- **The parts add up.** The union of 14 master parts by GUID equalled the catalogue's 1,989 ledgers,
  with none missing, none extra, none twice and no differing field.
- **The whole book is the cost, not the count.** The count read (the catalogue) is about 38 percent
  of the master read's bytes and 35 to 41 percent of its time on this book, which is why the read is
  admitted by a count and not by the mark alone once the mark is high enough to matter (#668).

**A second book, PARTIAL, 2026-09-29 (one licensed 7.1 Silver book of about 9.5 thousand ledgers,
about 1.35 thousand parents of which about three quarters hold one ledger, `ALTMSTID` about 20
thousand; read by scripts).** Whole reads: the
catalogue about 11.6 MB in about 1.5 s; balances about 12.4 MB in about 12 s. Balance time is not linear from parts: a
part of a few hundred rows took about 0.1 s (0.4 ms per row) and the whole took about 12 s (1.3 ms per row). A parent-group
part of the master with the compliance fields took under 0.1 s at about 20 rows, about 0.5 s at a few hundred rows and
under 0.8 s at about 1.5 thousand rows (about 4 MB); every row bound to the selected company. The 3,750-byte budget held (mean
1,900 to 2,800).

**Filters other than the parent returned an empty collection — the cause is UNVERIFIED, so do not
build a split on them (#679).** Each of these, sent on the ledger balance collection with the
parent filter's formula mechanism, returned zero rows and the same 2,994-byte body a closed or
absent company returns: `$Name >= "lo" AND $Name < "hi"` (lower-case bounds, then the exact-case
bounds of a real name range), `$$StringPart:$Name:0:1 = "X"`, `$AlterID > 0 AND $AlterID <= 2000`
and `$AlterID > 10000 AND $AlterID <= 12000`. Whether Tally does not support them on this
collection or the formula syntax was wrong is not known. Only `$Parent = "..."` is measured to
partition. Splitting by parent fits this book poorly: about three quarters of its parents hold one ledger.

**Every one of those filter forms partitions on the synthetic book, so the empty result is not "the
filter is unsupported" (PARTIAL, one synthetic book of 1,989 ledgers, no vouchers, 2026-09-29).**
Each returned exactly the expected rows, compared as sets by name: the name range, the `StringPart`
prefix and the `AlterID` span on the balance collection, on the master collection and on a custom
`TYPE Ledger` collection with no `ISMODIFY` (300, 300 and 100 rows); a single comparison (`$Name >
"lo"`, 1,787 rows) and `$AlterID > 100` (1,987 rows), the latter also through a `$$Number:` cast
with the same set; an exact-name filter (the must-match control, 1 row); and two parents joined
with `OR`, including one whose name contains `&` (360 and 400 rows, each the union of its parts).
Every request took 0.07 to 0.45 s. This is the prerequisite for splitting a read by several
parents at once (#679).

**The filter mechanism also works on the second book (PARTIAL, one real book of about 9.5 thousand
ledgers, 2026-09-29).** On the balance collection an exact-name filter for a name taken from that book's own
catalogue returned exactly 1 row (the must-match control that was missing), and `$Parent = "a" OR
$Parent = "b"` for two parents of 3 ledgers each returned 6 rows, 3 from each, every row bound to the
selected company; both took under 0.11 s. So the earlier empty results on that book were not "the
formula mechanism does not work there". Whether the name-range, `StringPart` and `AlterID` forms
match rows on it is still UNVERIFIED: they were not re-sent, and the untested explanation is that
the bounds they used matched no ledger.

**A one-day voucher census at a far-future date was answered normally with zero vouchers
(PARTIAL, one book, 2026-09-29).** On the second book a census for a far-future day, the
company header's own last-voucher date, returned a well-formed empty collection in 0.18 s, while the same request for a
day known to hold a few hundred vouchers returned all of them. It is UNVERIFIED that zero means no voucher sits on
that day: a five-digit-year date literal may not parse to that date and match nothing, and a census cannot
tell the two apart.

**Tally honours a far-future `SVTODATE` on the balance collection; it does not clamp it (PARTIAL,
one parent of about 80 ledgers on the second book, 2026-09-29).** The same parent read as at
20260401, at 20260929 and at a far-future date returned the same ledgers and the same opening balances
each time, with closing balances that differ in tens of rows (first and second date, first and third) and in
fewer than 20 (second and third). If the far-future date were dropped or replaced by the reporting
date the second and third reads would be identical, so it was applied as a later date. What that
proves is only that an as-of balance read at such a date takes in whatever is dated after the
reporting date. Whether those rows moved because of a mistyped far-future voucher or because of
legitimate later-dated ones is UNVERIFIED, as is a book whose stray voucher sits in another
parent.

**`NOT` filters, several of them listed together, reach the ledgers a parent filter cannot name
(PARTIAL, read-only: the catalogue collection on one real book of about 9.5 thousand ledgers, 2026-09-29; the master and balance collections on a synthetic book, 2026-09-30).** A parent name with
a control character cannot be written into a `$Parent = "..."` literal, so its ledgers are in no
named part. On the balance-free ledger catalogue collection, `NOT ($Parent = "A" OR ...)` formulas (each at most 200
terms), each declared as its own `SYSTEM` formula and all listed in one comma-separated `FILTERS`
element, applied together as an AND. With one, two, three and five formulas the row counts each
matched the count the catalogue predicted, every GUID was distinct, and every row was bound to the
selected company. With enough formulas to exclude every parent whose name can be written (about
1.35 thousand parents), the response was exactly the four ledgers under three parents whose names
carry a control character: 3.9 s, an 8 KB response for a 54 KB request. UNVERIFIED: a book whose
unnameable parents hold thousands of ledgers, a parent name that differs from another only in case
(whether Tally's `=` folds case here was not tested), and a real book's master and balance
collections under these formulas. On the party-ledger master (with the compliance fields) and the
balance collection of a third book, a synthetic licensed 7.1 Silver one of 4,339 ledgers under 514
parents (one request at a time), `NOT` formulas of the same form returned exactly
the expected ledgers, every GUID distinct and every row bound to the selected company (balance rows
compared by name and parent): one formula of 2 and of 4 terms, two formulas of 2 terms applied
together, one formula of 200 terms, two of 200 terms applied together, and 511 excluded parents as
three formulas of 200, 200 and 111 terms, which left 41 ledgers; the same 511 as seven formulas of
73 terms; and those 511 plus 889 parent names that are not in the book as seven formulas of 200
terms (a request of about 110 KB), which also left 41. Each answered in 0.3 s to 1.0 s with a body
of 53 KB to 3.9 MB, and a second identical request returned identical bytes. That book has no
vouchers, and seven formulas of 200 terms naming real parents were not measured. Bridge now ends a part read as
soon as any part's master comes back with a different ledger count than the catalogue holds: that
part's balance, any later part and the group read are never requested, though the master pair
itself was sent, so this cannot keep the complement's own `NOT`-filtered master off the wire. Every
part is checked against the catalogue by GUID before a row is released, and a part whose answer
passes the response limit ends the read under its own cause.

**The parent filter also works on the rates request, and `$Parent` folds letter case (PARTIAL, one synthetic licensed 7.1 Silver book of 15 ledgers under 8 parents, read only, 10 Oct 2026, #1331).** The `List of Ledgers` request that fetches the compliance fields plus `GSTDETAILS.LIST`, `RATEOFTAXCALCULATION`, `ROUNDINGMETHOD` and `ROUNDINGLIMIT` was sent unfiltered (15 rows) and with `$Parent = "Duties &amp; Taxes" OR $Parent = "Indirect Expenses" OR $Parent = "Sales Accounts" OR $Parent = "Sundry Debtors"` (11 rows). The 11 were exactly the unfiltered read's rows under those four parents, by GUID, and every filtered row had the same element paths and values as its unfiltered row, the four rate fields included (`ROUNDINGMETHOD` is absent from the same rows in both). The same filter spelled `SALES ACCOUNTS` returned the four `Sales Accounts` ledgers, each row still carrying the book's own spelling: **`$Parent` matched across letter case**, for this one parent and this one spelling. Each request answered in 0.008 to 0.012 s. The answers are in the pilot-lab fixtures (`pilot-lab-ledger-rates-scoped*`, PROVENANCE.md). Not measured: any other characters or spellings, whether the compliance master and balance collections fold the same way (the mechanism is the same), two parents of one book that differ only in case, and any book larger than 15 ledgers. Bridge therefore refuses an invoice whose ledgers' parents have a case-twin among the book's parents, before the request, and proves a scoped answer against the catalogue by GUID, name and exact parent text.

**Not established:** a book above about 9,500 ledgers, any size at which Tally stops answering,
or the cause of the empty range, prefix and span filters on the second book. The largest single latency was 1.00 s on the synthetic
book (the whole party-ledger master) and 12.0 s on the second (the whole balance read). Do not read the 3,750-byte budget or the
10,000-mark reach for the count as measured limits: they are Bridge's own choices, set above the
row cost seen here.

**Bridge's parent partition (#679) is CODE, live unproven.** A counted book that does not fit one read
is split by the parent groups the catalogue names: packed first-fit-decreasing by ledger count into
parts of at most 4,266 ledgers (3,200 when the request names the mailing details, as `ledger_masters` with `include_mailing_details` does, and the workbook does only for a book whose master-alteration mark is at most 3,200, otherwise sending the standard request; 2,000 with the GST details) and at most 200 parents, at most 12 parts (so at most 2,400 parents, refused as `parent_partition_too_many_parts` above that), and read through the
`$Parent = "a" OR $Parent = "b"` filter measured above, with the reserved root written as
`&#4; Primary`. Each part is read as its master pair then its balance pair inside one extent bracket,
and every catalogue ledger must come back exactly once, from its own part, with the catalogue's name
and parent, or the whole call is refused. The routing is by count, not by mark: any mark whose
catalogue Bridge can bound is counted, and a count over 4,266 (3,200 with the mailing details) is read in parts. The catalogue is
bounded before it is sent (mark times 1,400 bytes, at most 32,000,000, a mark of 22,857), because a
response past the transport's 32 MiB cap is cut off mid-read and the connection dropped with the
rest unread, which is an abandoned read. On one real book the largest parents hold about 2,000, 1,900
and 1,500 ledgers (rounded), all under a part's 3,200 or 4,266, so a parent too large for a part is not the
constraint there; a book with about 1,350 parents, most holding a few ledgers, is why parts are
packed by ledger count. The 200-parent limit is set from a measurement on the synthetic book of 4,339
ledgers: an `OR` of 1, 8, 50 and 200 parents answered every time with exactly the parents named (VERIFIED
for that book; 0.1 to 0.6 s). Nothing longer is measured, so 200 is the cap and a longer formula is
UNVERIFIED. The 12-part limit, Tally's cost of evaluating an `OR` over thousands of ledgers and any
limit on its length are Bridge's own choices, UNVERIFIED against Tally. Whether the catalogue's and the master's parent text
always agree byte for byte is UNVERIFIED; if they differ the read fails closed as
`parent_part_row_differs_from_catalogue`.

**The ledger census (bridge#679) counts a book whose mark is past its catalogue — PARTIAL, three books.**
A `List of Ledgers` collection filtered by `$AlterID > a AND $AlterID <= b` (one `SYSTEM` formula, its
name in `FILTERS`) returned exactly the ledgers whose AlterID lies in `(a, b]`, and the union over
slices covering `(0, mark]` equalled the whole catalogue as a set of GUIDs, with no GUID twice, on
three licensed 7.1 Silver books read on 30 Sep 2026, one request at a time: a synthetic book (mark
5,547, 4,339 ledgers, 7 slices of 800 over (0, 5,547], the last clipped at the mark), a stock-heavy book (mark 102,161, 864
ledgers, 15 slices of 8,000 over (0, 120,000], of which 14 were empty: the ledgers' AlterIDs sit in
one band, the rest of the mark is stock items and other masters), and a second stock-heavy book (mark
316,028, 2,649 ledgers, 318 slices of 1,000 over (0, 318,000] and again 80 slices of 4,000 over
(0, 320,000]). The tilings on the two stock-heavy books reached past the mark, so the numbers of
slices measured there are not what Bridge sends: it clips the last slice at the mark and sends
ceil(mark / width) of them (13 slices of 8,000 for 102,161, 317 of 1,000 and 80 of 4,000 for
316,028). Every ledger lay below its book's mark. A slice held at most its width in rows on every request. The survey made earlier that day counted one
ledger fewer than the census and the catalogue on each of the two stock-heavy books (unexplained). The GUID-only fetch (`NATIVEMETHOD` GUID and the company
GUID computed) makes a row about 846 to 892 bytes; the row still carries the ledger's name twice
(as the row's `NAME` attribute and inside `LANGUAGENAME.LIST`) whatever is fetched, so a row is
`2 x (415 + 2 x name characters)` bytes on the synthetic book, and the worst row Bridge assumes is
`2 x (415 + 12 x 128)` (name 128 characters, each escaped to six): the longest name seen is 88
characters, so 128 is UNVERIFIED (bridge#917). A slice of 4,000 then holds at most about 15.6 MB, under
the 16 MB budget and about half the transport's cap; the largest slice read was 1.7 MB. A slice past
every ledger answers a well-formed status 1 envelope with an empty collection (2,994 bytes; captured
byte for byte). An earlier measurement recorded the same size for a closed or absent company (that
body was not kept, so identical content is UNVERIFIED), so an empty slice is not evidence of a healthy
read: Bridge refuses a census that found no ledger and relies on the company
extent bracket (opening equal to closing, master mark and company GUID included) to say the company
was open. Each request cost about 0.15 s on the 316,028-mark book, whether or not the slice was empty
(the filter scan dominates): 80 slices took 11.8 s of transport time. Bridge reads each slice once,
inside the bracket, and does not pair the reads.

**The company's own ledger count (`NUMLEDGERS`, bridge#938) agrees with the census — PARTIAL, three books.**
The Company collection fetched with `Name, GUID, NUMLEDGERS` (request `BridgeCompanyLedgerCountV1`, sent once on
30 Sep 2026 on a licensed 7.1 Silver holding 31 lab companies) answered 200 with a status 1 collection of one
`COMPANY` row per loaded company, whatever `SVCURRENTCOMPANY` named, each row `NAME`, `GUID` and
`NUMLEDGERS` (a number with a leading space, as `ALTMSTID` is): present on 31 of 31 rows, in 0.09 s for
18.7 KB. The synthetic 4,339-ledger book's value, 4,339, equalled its catalogue and AlterID-census counts, and two
real stock-heavy books, about 900 and about 2,600 ledgers, read separately, also equalled their censuses
(recorded in #938 only; the tree holds the synthetic book's capture, and the counts of the two other kept rows,
6 and 123, are not compared with a census anywhere in this tree). That is three books, one of them captured here: what `NUMLEDGERS` counts for a book with deleted or hidden ledgers, and on another Tally
version, is not measured. Bridge therefore uses it only to refuse: a census that counted fewer ledgers than the
company reports is refused (`ledger_count_company_differs`), which is meant to catch a company closed and
reopened with equal marks while the census ran (by reasoning: that case was not reproduced live) and also
refuses a ledger added during the read, and (by reasoning, not reproduced: #965) one altered during it before its
slice was read, whose new AlterID is past every slice while `NUMLEDGERS` still counts it (one altered after its slice
was read was counted, and the closing extent read refuses that case instead); an equal or higher census count, or an answer with no `NUMLEDGERS`, admits and
sizes nothing, and the result of a counted read says which it was (`ledger_count_cross_check.status`).

**A company name of 150 characters is stored whole, and a request naming it is handled (bridge#917) — PARTIAL, one company, one release.**
On 1 Oct 2026, on the licensed 7.1 Silver holding 31 lab companies, the Company collection fetched with `Name, GUID,
NUMLEDGERS` (the request above, 992 bytes, answered 200 in 18,696 bytes, status 1, 31 rows) gave the lab company its owner
says they created with the longest name they could type (the owner's account) a `NAME` attribute and a `NAME` element of 150 characters, 150 UTF-8 bytes, equal, with
no XML escape. That is the stored length, not Tally's limit: it is the limit only if more was typed and Tally cut it, which
is UNVERIFIED. A census slice request naming that company in `SVCURRENTCOMPANY` (2,174 bytes, `$AlterID > 0 AND $AlterID <=
100`) answered 200, status 1, a complete envelope of 4,716 bytes in 0.06 s with the company's two ledgers, each row
carrying that company's GUID; no dialog appeared and `GET /` answered the same 200 and 51 bytes before and after (VERIFIED
for that one request shape; other shapes with that name are not measured). A census row carries the company's GUID and
not its name, so a long company name changes the size of the request, not of a row.

**A census row carries the ledger's name twice, or three times for a reserved ledger — PARTIAL, two rows.**
On the same answer the row of the ledger `Cash` (a 4-character name, `RESERVEDNAME` empty) was 396 characters, a fixed part of
388 plus two copies of the name, against 391 in the pinned capture (Bridge's constant of 415 keeps its margin over both), and
the row of `Profit & Loss A/c` (a built-in ledger, 17 characters, 21 once escaped, `RESERVEDNAME` equal to the name) carried the name
three times: its `NAME` attribute, its `RESERVEDNAME` attribute and its `NAME` element, 451 characters, the same 388 fixed plus three
copies of the escaped name. If a ledger a user names never has a `RESERVEDNAME` (not measured), `2 x (415 + 12 x 128)` (two
copies, six characters each) bounds a user-named ledger's row. Not measured: whether a built-in ledger a user renames to a long
name keeps its `RESERVEDNAME` and so carries three copies (such a row could pass that bound: three copies of a 128-character name
at six characters each is about 2,300 characters before the fixed part, against the 27 characters of margin between the pinned 415 and the measured
388), and what an alias adds. The longest ledger and group names Tally accepts, and whether its limit counts characters or bytes, are not
measured (bridge#917: they need writes to a synthetic company).

**Not established** for the census: that AlterIDs are distinct across a book's ledgers (a slice's row
bound rests on it: two ledgers sharing an AlterID both fall in one slice, and only a slice pushed past
its span by them is refused, so this can cost a spurious refusal, not a missed ledger), how many names a ledger's row carries when it has aliases (each alias would add to the row
past the two copies bounded here), a book whose ledgers' AlterIDs exceed its master mark (the mark is
assumed to bound them; a census then undercounts, and Bridge compares the count with the ledgers the
read returns and refuses a difference), a mark above 400,000, a slice of names longer than 88
characters, and how a busy or edited book answers a slice (the bracket, not the slice, is what
detects an edit). Tally's behaviour on a slice that would exceed the 32 MiB response cap is not
measured and is not to be: Bridge sizes a slice by its width before sending it.

## 11f. How long Claude Desktop waits on one tool call — **PARTIAL**

**PARTIAL: one run per case, one app build, no Tally involved. Not a Tally observation.** This section records how a host treats a long tool call, because the size of a read (§11c) is bounded by it as well as by Tally's own time. No request was sent to Tally.

- **Build, system and date:** Claude Desktop (the chat app) on macOS, 2026-10-02. The build number, 2.19675.0, is the app bundle's version; the probe's own log records neither the build nor the system. A throw-away extension installed in the app, a stdio server with one tool that waits a given number of seconds and logs every message it receives with its time. It sent no progress notifications in the runs below.
- **Silent calls of 20, 55, 75 and 130 seconds each were answered by the server without a cancel**, one run each (the server's own log; the log has no host-side receipt).
- **A silent call of 250 seconds was cancelled 240 seconds after it arrived:** the server received a `notifications/cancelled` naming that call with the reason `SdkError: Request timed out`. So the limit seen in this Claude Desktop build is 240 seconds per call. The 60 seconds in #703 and #485 was seen through the Claude Code client, a host this run did not test.
- **The host offered no `progressToken` on any of the six calls** (five in the first run, one in the second; each call's `_meta` was empty), so the server could not send a progress notification tied to a call. Six calls do not show that it never does; whether progress would extend the limit is **not answered**.
- **The host started three copies of the probe within the same second,** each initialised by a different client (one named `claude-ai`, two with names derived from the extension's); in the first run all five calls reached the `claude-ai` one, and in the second the one call did.
- **After the cancel, in the first run, no further message reached that server process,** and its log has no exit record; a later call in the same chat left no record in the server's log. A second run of the same 250-second silent call, with a probe that also logged signals, exit and a 10-second heartbeat, was cancelled the same way, 239.96 seconds after the call arrived, with the same reason. The server process that received it kept writing heartbeats for at least 74 seconds after the cancel, to the end of the log, with no signal, no exit and no closed input. So on this build the host cancelled the call and did not stop or disconnect the server within that time: one run, PARTIAL. Of the other two copies started at install, one logged an exit at once and one kept writing heartbeats with no call.
- **What Bridge does on a cancel (from reading the code, not from this host; the probe server was not Bridge):** a `notifications/cancelled` naming a read in flight withdraws it before its next Tally operation, and a post not yet dispatched is withdrawn; a read already sent is not abandoned, and a `lab_import_*` write, or a post past its dispatch intent, runs to completion (`agent_protocol.rs`). A cancelled post that has written no intent still runs its operation in flight and any already admitted in full; the code's own comment puts that at about 11 to 13 minutes at worst at the default deadlines, far past a 240-second host limit. On a host timeout like the one seen, the server was not stopped within a minute, so Bridge's own cancel handling applies and a post past its dispatch intent completes while the chat shows an error: a person who sees one should check with `verify_import` before trying again. Whether a host ever stops the Bridge process after a timeout, which would abandon a request in flight, was not measured beyond that one run.
- **Not measured:** other app builds, Windows, Claude Code (its limits are separate and configurable), the effect of progress, a call between 130 and 240 seconds, the host's behaviour more than a minute after a cancel.

## 11a. Scale measurements — 11,287-voucher corpus

**VERIFIED 2026-07-29** on a generated production-shaped corpus: 25 customers and 15
suppliers with valid-format GSTINs across six state codes, Sales/Purchase/Payment/Receipt
mix, 9%+9% CGST/SGST splits, invoice-referencing narrations, spread across every
Education-legal date from 2024-04 to 2026-03.

### Read performance

| Request | Elapsed | Bytes | Rows | B/row |
| --- | --- | --- | --- | --- |
| Minimal fetch (`DATE,ALTERID`), whole book | 5.44 s | 13.7 MB | 11,287 | 1,214 |
| **Curated fetch, whole book** | 15.88 s | **40.6 MB** | 11,287 | **3,598** |
| Curated fetch, one-month window | 0.72 s | 1.94 MB | 538 | 3,605 |
| Curated + `$AlterID` filter (no matches) | 1.99 s | 1.5 KB | 0 | — |
| Wildcard fetch, one-month window | 2.94 s | 11.6 MB | 538 | 21,532 |

**Key results:**

1. **The 32 MiB cap binds at ~9,300 vouchers** with a curated fetch. A whole-book read of
   11,287 vouchers produced **40.6 MB — already over the cap**. Segmentation is not an
   optimisation, it is a correctness requirement at any realistic client size.
2. **Bytes-per-row is highly stable** — 3,598 whole-book versus 3,605 for a single month.
   Segment sizing can be predicted reliably from a sample read.
3. **Wildcard costs 6.0×** curated (21,532 vs 3,598 B/row), confirming the earlier 6.3×
   measurement on a smaller sample.
4. **Server-side filtering is not free.** A filter matching zero rows still took 1.99 s —
   Tally evaluates the predicate across the whole collection. Filtering saves bandwidth and
   parsing, not scan time.
5. Curated read throughput is roughly **710 rows/second**.

### Write performance and degradation

10,000 vouchers imported in 624 s, zero errors, batches of 250. **Throughput degraded
monotonically as the book grew:**

| Progress | Rate |
| --- | --- |
| 250 | 21/s |
| 2,750 | 21/s |
| 5,250 | 19/s |
| 7,750 | 17/s |
| 10,000 | 16/s |

Roughly a 25% drop across the first 10K rows. **Bulk import cost is superlinear in existing
book size**, which matters for any initial-load or migration story: a 100K-voucher onboarding
will not take ten times a 10K load.

## 11. Measurements

| Quantity | Value | Basis |
| --- | --- | --- |
| Curated voucher payload | **3,142 B/voucher** | 6 vouchers, curated FETCH |
| Wildcard voucher payload | **19,658 B/voucher** | same window, `ALLLEDGERENTRIES.*` |
| Response cap | 32 MiB for every read (the 40 MiB exception for the wildcard outstandings request was deleted with that request, October 2026) | Bridge-side limits; guide §2.5b |
| Segmentation ceiling | **Profile-specific; derived at runtime** | ~10,600 applies only to the older curated shape, not wildcard outstandings |
| Independent corroboration | 10,000 vouchers/batch | `tally-database-loader` caps here to avoid hangs |
| Whole demo book, wildcard | 2.94 MB / 150 vouchers | wide filter |
| Typical collection latency | 30–90 ms | demo company |

Segment size must be derived from **observed** per-voucher cost at runtime — inventory lines
and long narrations run heavier than this demo data.

---

## 12. Verification method — how to test this gateway

Learned the hard way; every one of these produced a wrong conclusion at least once.

1. **Write responses to a file, then inspect the file.** Never pipe `curl` through
   `head`/`tail` — SIGPIPE truncates the capture mid-element and the truncated file looks
   like a complete short response.
2. **Never count rows with a bare substring grep.** `<VOUCHER` also matches `<VOUCHERNUMBER>`
   and `<VOUCHERTYPENAME>`; `<GROUP` matches the `CMPINFO` header. Match the opening tag with
   its trailing space, or parse.
3. **Diff whole files, not fragments.** A narrow `grep -A1` window missed a 13-byte
   difference that reversed a conclusion.
4. **Check the returned span, not just the row count.** A plausible row count can come from
   entirely the wrong period.
5. **Repeat before believing anything anomalous.** Determinism distinguishes a protocol rule
   from a transient.
6. **Change one variable at a time.** A multi-variable change established that a combination
   works while leaving the actual cause unknown.

---

### 12.7 Empty-date witness profile qualification

**VERIFIED 2026-08-02** under owner-supervised dispatch against the reference
corpus. `VoucherEmptyPartitionWitnessV1` was healthy before and after every
request and produced no dialog or hang.

| Purpose | Window | Observation |
| --- | --- | --- |
| Non-empty control | `20240401..20240501` | 20 vouchers dated `20240401..20240402` |
| Empty primary | `20240502..20240601` | no rows |
| Shifted cover A | `20240501..20240531` | no rows |
| Shifted cover B | `20240531..20240601` | no rows |

The non-empty control proves the profile returns rows when rows exist; the two
date-shifted covers corroborate that the primary partition is genuinely empty.
This qualifies the runtime to use the profile only as the nearest non-empty
control and the bounded shifted cover for a positive-high-water empty primary
partition. It does not qualify a wider date window, change the universal
31-day cap, establish a compatibility claim, or establish a sizing rule.

**Parser trap.** An empty witness response also contains `CMPINFO` with bare
`<VOUCHER>0</VOUCHER>`. Counting `VOUCHER` over the whole document sees one
element and reverses the verdict. This was observed three times in the
qualification session. Count only `VOUCHER` start elements inside `<DATA>` and
deserialize through `BODY.DATA.COLLECTION`; the implementation does both.

---

## 12a. Bill-wise semantics, the native reports, and volume — live measurement 2026-08-02

**VERIFIED 2026-08-02** against TallyPrime Edit Log 7.0 EDU on port 9000, using
`Bridge Ageing Lab` — a company created for this session, every shape placed deliberately —
and `Aarav Trading Company Demo` for scale. All data synthetic.

Several entries here **correct or qualify earlier sections**; each says which.

### 12a.1 Built-in named reports work — this qualifies §2.2's blanket AVOID

§2.2 tests a **custom** `REPORT`/`FORM`/`PART` and concludes `<TYPE>Data</TYPE>` should be
avoided. That conclusion does not extend to Tally's **own built-in reports addressed by name**:

```xml
<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST>
<TYPE>Data</TYPE><ID>Bills Receivable</ID></HEADER>
<BODY><DESC><STATICVARIABLES>
  <SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>
  <SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>
  <SVFROMDATE TYPE="Date">{from}</SVFROMDATE><SVTODATE TYPE="Date">{to}</SVTODATE>
</STATICVARIABLES></DESC></BODY></ENVELOPE>
```

Accepted on the first attempt. `Bills Payable` works identically. Per-bill rows:

```
<BILLFIXED><BILLDATE/><BILLREF/><BILLPARTY/></BILLFIXED> <BILLCL/> <BILLDUE/> <BILLOVERDUE/>
```

Four properties that matter more than the speed:

1. **It was fast on the observed book.** 101,603 vouchers returned 22 rows in **0.10 s /
   5 KB**. This single measurement does not establish an asymptotic bound: voucher count,
   open-bill count, cache state, and release were not varied independently. A voucher scan over
   the same book was much more expensive, but capacity and deadline planning must treat that as
   book- and release-specific evidence.
2. **`STATUS` is emitted only on failure.** A successful response contains **no `STATUS` tag
   at all**, so you cannot assert `STATUS=1` as §2.1 allows.
   **But absence of `STATUS` does not prove success.** An unrecognised report `ID` returns a
   bare `<RESPONSE>Unknown Request, cannot be processed</RESPONSE>`, which also carries no
   `STATUS` — a parser treating "no `STATUS`" as success would read it as a report with zero
   bills and silently publish no exposure. **Require the expected report structure, and reject
   `RESPONSE` or `LINEERROR` independently of `STATUS`.**
3. **It fails closed on a company that is not loaded**, with
   `LINEERROR: Could not set 'SVCurrentCompany' to '<name>'` — see 12a.6.
4. **It carries no GUID anywhere**, so the response cannot be identity-bound the way a
   voucher or ledger collection can. Bracket every candidate native read with GUID-verified
   extent probes before and after it, and reject drift. Bracketing detects some changes; it does
   not create an atomic source cut.

An unrecognised report `ID` returns a clean `<RESPONSE>Unknown Request, cannot be
processed</RESPONSE>` — **not** the modal-dialog hang of §1.2. The hazards are different
failure paths and should not be conflated.

**They scope by balance SIGN, not by party group.** On the demo book, each report included
parties from both Sundry *Creditors* and Sundry *Debtors*. That is correct accounting — a
supplier advance is a debit balance — but a screen labelled "receivables" that renders this
list is showing every debit-balance bill, including advances paid out. The measurement records
rows, not a complete distinct-ledger census.

### 12a.2 Which date each allocation kind ages from

Measured against Tally's own *Ledger Outstandings* screen and both native reports, not
inferred from the data model.

| Kind | Ages from | Note |
| --- | --- | --- |
| `New Ref` | `BILLDATE` | which on an opening equals the voucher date |
| `Agst Ref` re-opening a settled bill | **the original bill's `BILLDATE`** | not the voucher that reused it |
| `Advance` | `BILLDATE` | always its own voucher date; see 12a.4 |
| `On Account` | **not aged at all** | shown at the *report* date, blank overdue |

The second row was contested in the codebase. Constructed explicitly — a bill opened at
3,000, settled to exactly zero, then reused by a later `Agst Ref` for 1,500 — Tally reports:

```
1-Jun-26  ZBR  3,000.00 Dr opening  1,500.00 Cr pending  Due on 1-Jun-26  Overdue 60
```

Aged from **1-Jun-26**, the original bill, though the reusing voucher is dated 1-Jul-26.
Confirmed three ways: on screen, in `Bills Receivable`, and in `Bills Payable`.

The mechanism is 12a.4: an `Agst Ref` **carries** the original bill's `BILLDATE`, so ageing
from `BILLDATE` reaches the true opening without special handling.

### 12a.3 Tally offers two ageing methods, and the anchor differs

`F6: Ageing Method` offers *Ageing by Bill Date* and *Ageing by Due Date*, and the choice
changes the buckets. On a bill dated 1-May-26 with a 30-day credit period, due 31-May-26:

| as-of | method | days | bucket |
| --- | --- | --- | --- |
| 2-Jul-26 | by Due Date | 32 | 30 to 60 |
| 2-Jul-26 | by Bill Date | 62 | 60 to 90 |

Both are correct. Neither is "the" answer, so **any tool computing ageing must state which
basis it used.**

Licensed-gateway capture/read-back evidence on **2026-08-23** establishes these accepted units:

| entered value | returned `BILLCREDITPERIOD` |
| --- | --- |
| `2 Months` | `2 Months` |
| `3 Weeks` | `3 Weeks` |
| `1 Day` | `1 Days` |

The same licensed TallyPrime 7.1 import/read-back on **2026-08-23** measured the
day-unit persistence ceiling and confirms that it is not a shared ceiling for
other units:

| entered value | returned `BILLCREDITPERIOD` |
| --- | --- |
| `365 Days` | `365 Days` |
| `3650 Days` | `3650 Days` |
| `9999 Days` | `9999 Days` |
| `10000 Days` | empty |
| `100 Months` | `100 Months` |
| `1000 Months` | `1000 Months` |

Tally silently discards a day count above `9999` to an **empty** credit period.
That is not neutral: due-date ageing then uses the bill date as the due date,
which can move the bill to a different ageing bucket. The parser therefore
rejects day counts above the measured `9999` ceiling, but does not invent a
matching weeks or months ceiling.

The voucher parser accepts explicit `N Day(s)`, `N Week(s)`, and `N Month(s)` forms only. It
does not guess a day count for an unknown unit: the shared parser rejects it, and the
agent `vouchers` read carries the text as `unrecognised` instead (so `10000 Days`, above the
ceiling, is carried, not refused). Bridge does not compute a due date from a credit period: the
voucher scan that did was removed. The same licensed-gateway evidence measured Tally's own due dates:

| bill date and period | due date |
| --- | --- |
| 15-Jan-26 + 4 Weeks | 12-Feb-26 |
| 30-Jan-26 + 1 Month | 28-Feb-26 |
| 31-Jan-26 + 1 Month | 28-Feb-26 |
| 31-Mar-26 + 2 Months | 31-May-26 |

2026 is not a leap year; the 29-Feb case was not observed in this capture.

**`BILLOVERDUE` in the native report is not usable as an ageing oracle.** Its as-of is
Tally's own period date, not the caller's, and it does **not** follow the `F6` setting:
toggling the UI to *Ageing by Bill Date* left the exported `BILLOVERDUE` unchanged at the
due-date value. Recompute ageing from `BILLDATE`/`BILLDUE` against an explicit as-of on
every path.

**A due date past 2099 prints its year in full, and an as-of report listed no later-dated bill — PARTIAL, 2026-10-02 (bridge#1091).**
One read each of the Bills Receivable and Bills Payable reports of a synthetic company (BRIDGE PROBE B SANDBOX, licensed
TallyPrime 7.1 Silver) as of 20260331, from the book's start 20250401: 1,477 open receivable bills and 21 payable ones.

- The bill dated 1-Aug-25 whose credit period was about 83 years (the printed date is exactly 1000 months after it: an inference from
  the arithmetic, the report does not state the period) printed `BILLDUE` as `1-Dec-2108`, the year in full. Due dates in 2026, 2033, 2035
  and 2052 in the same report printed two digits. Bridge's parser accepted two digits only, so the whole read refused with no cause. It now
  reads a four-digit year of 2100 or later in a **due date** exactly as written, inside the same window; a four-digit year in a bill date,
  or below 2100, still refuses. Observed once: whether every year from 2100 prints so, and on other releases, is not established.
- Six rows, the ones with a due date after the as-of date, carried an empty `BILLOVERDUE` element.
- The report's latest bill date was the as-of date. The issue records later probe vouchers dated after the financial year on this book
  (not re-read here), so on this book the as-of Bills report did not list a later-dated bill. One book; it was inferred before.
- Fixtures: `native-outstandings-probe-b-bills-receivable` and `native-outstandings-probe-b-bills-payable`, byte-exact, with provenance.
- **The whole call then completed on this book — PARTIAL, one run, 2026-10-03.** A release build of `outstandings` from master 7b994e65 (later
  master commits are not in it; it holds the four-digit due-date change), for this company, as of 20260331, direction `both`, default limits,
  through a recording read-only relay on the same licensed 7.1 Silver gateway. No write was made: of the call's 48 requests, all 26 POSTs were Export requests and the other
  22 were status GETs; with the status read before and the one after (each one export POST and one GET) that is 28 POSTs, all Export, and 24 GETs over all 52. State `complete`, evidence `complete`, 4.0 s wall. It sent 48 requests (8 company-extent, 8 book-extent, 22
  status GETs, 2 currency, and 2 each of Bills Receivable, Bills Payable, List of Ledgers and List of Groups), plus 2 status reads before and 2
  after: 52 in all, none refused. Sizes: Bills Receivable 674,070 bytes and Bills Payable 9,528 (each fetched twice, the same size both times),
  List of Ledgers 33,850, List of Groups 54,280; about 1.7 MB relayed in all, the slowest request 0.6 s (Bills Receivable). The result carried
  1,498 open bills (1,477 receivable and 21 payable, counted in this run's own Bills answers) and 15 unallocated parties, in a 181,044-byte MCP
  result, under the 200,000-byte cap. The bills page is a truncated view of a complete read: `truncated` true with `state` `complete`, 500
  bills shown (`open_bills_shown` 500, `next_offset` 500), in the order `all_open_bill_rows` gives (days past the due date, largest first; then party; then reference), so the oldest-due bills come first and bills not yet due, which have no age, come last. The gross billed total equalled receivable plus payable
  and the sum of the five ageing buckets, and the `unaged` bucket equalled, to the paisa, the six receivable bills whose `BILLOVERDUE` was empty
  (the not-yet-due ones, the 1-Dec-2108 bill among them): checked by script, amounts are not quoted here.
- What that shows, and what it does not: the four-digit due date was parsed without a refusal and the bill is inside the totals (deduced from the
  sums above, not seen as a row). The 2108 bill, like the 2033, 2035 and 2052 ones, was not on the returned page, because every due date on it was in
  2025 (the first 500 by that order). The 2108 bill is not yet due, so it has no age and sits in the tail of bills not yet due, in party then
  reference order, among the last rows: on the last page (offset 1000), not the second (inferred from the order above and the six
  not-yet-due bills in the saved Bills answers; only the first page was read). How a 2108 due date shows in a
  live response was not seen. The overdue crosscheck has no field in the result; the code gives a `partial` reason for every outcome except an
  honoured one, so `complete` is the evidence it was honoured. No further page was read; one run, one book, `both` only.

### 12a.4 The import path rewrites what you send — extends §9

§9.2 already records that `ERRORS=0` does not mean success. Eight further rewrites, each
measured by writing a distinguishable value and reading it back. **Every one reported
`CREATED=1, ERRORS=0, EXCEPTIONS=0`.**

| # | Behaviour |
| --- | --- |
| 1 | `BILLDATE` on a bill-**opening** allocation is overwritten with the voucher date; a differing supplied value is discarded |
| 2 | `BILLDATE` on an `Agst Ref` is **inherited** from the settled bill — so it legitimately differs from the voucher date |
| 3 | The allocation **kind** is rewritten when the reference name already exists: `Advance` naming an existing ref is stored as `Agst Ref` |
| 4 | `On Account` names are **stripped** — it is structurally unnamed |
| 5 | An allocation on a ledger with `ISBILLWISEON=No` is **silently discarded**; the entry stores with no allocations. Treat `No` as a mandatory fail-closed preflight for any bill-wise write |
| 6 | `VOUCHERNUMBER` is **overridden** with Tally's own per-type sequence — **under automatic numbering only.** §9.8 establishes that Manual + `PREVENTDUPLICATES=Yes` preserves it verbatim. The company measured here used the default, so this observation does not generalise; the numbering method decides it |
| 7 | On the automatically numbered voucher type measured here, `ACTION="Alter"` carrying the target `GUID` returns `CREATED:1, ALTERED:0` — it creates a duplicate rather than editing. This is not a numbering-method-independent result; see §9.8 |
| 8 | The company-level `Enable Bill-wise entry` gate does **not** apply to XML import — see 12a.5 |

Consequence for §9 generally: **a write is only verified by reading it back.** Counters
describe the operation, not the result.

One error message is actively misleading. A voucher dated on a day Education forbids returns:

```
CREATED=0  ERRORS=0  EXCEPTIONS=1
LINEERROR: Voucher date is missing for: 'Sales' voucher BAD-1.
```

The date was present and well-formed — it was illegal for the mode. Note `ERRORS=0`; the
failure lives in `EXCEPTIONS`.

### 12a.5 Configuration is not a diagnostic, in either direction

`F11 → Enable Bill-wise entry` set to **No** on a company holding bill-wise data, confirmed
on screen, changed **nothing** observable over XML: per-ledger `ISBILLWISEON` stayed `Yes`,
existing allocations survived, both native reports returned identical rows, and a **new**
allocation imported afterwards was still accepted and retained.

So the company flag gates Tally's own voucher-entry screen, not the API or the data. It is
also **not fetchable over XML**, so a client could not read it even if it meant something.

And the per-ledger flag is not a sufficient **positive** diagnostic. `ISBILLWISEON=Yes` does
not prove that a ledger has bill references, but `No` remains a mandatory fail-closed signal
before a bill-wise XML write. On the 101,603-voucher book:

```
party ledgers (Sundry Debtors / Creditors) : 60
  ISBILLWISEON = Yes : 60
  ISBILLWISEON = No  :  0
  absolute closing balance                 : 208,232,027.79
```

Every ledger correctly configured — yet only **7 of 60** carry any bill reference at all.

### 12a.6 The unallocated remainder, and how to see it

Because a voucher can post to a bill-wise ledger without allocating to a reference, a book
can be fully configured and almost entirely unallocated. On the demo book the residual is
**≈ ₹2.83 crore, about 98.4% of debtor balance** — and it is **invisible to both native
reports**, which return no error.

The residual is recoverable:

```
On Account = ledger CLOSINGBALANCE − Σ BILLCL
```

Verified to the paisa on every bill-carrying party — **at a current as-of only.** This is a
cross-request candidate calculation, not an atomic source cut: a voucher can change between
the report and ledger reads. Bracket the reads with unchanged content/high-water evidence and
retain the non-atomic qualification; no cross-request combination becomes `Verified` solely
because its arithmetic agrees.

**CORRECTION — 2026-08-24.** §7 now establishes that `CLOSINGBALANCE` is as-of scoped, so
this identity can be evaluated at a historical as-of when the ledger snapshot and `BILLCL`
report use the same validated `SVFROMDATE`/`SVTODATE` window. Mixing windows would still
misclassify later activity as `On Account` and silently overstate the residual. The ledger
read costs **0.08 s / 36 KB** for 88 ledgers. A candidate as-of view needs at least both
sign-scoped native reports plus the ledger read, and separate identity bracketing; it must not
be described as full atomic coverage.

**Any outstandings figure that omits this is silently incomplete.** Depending on the residual's
sign, it can understate receivables, overstate them, or hide a payable position. On the observed
debtor-heavy book a native-report-only answer would present **1.6%** of the position as complete.

### 12a.7 The `Company` collection ignores `SVCURRENTCOMPANY` — qualifies §9.11

§9.11's table records a *non-existent* company name returning 0 rows on a `Ledger`
collection. A `Company` collection behaves differently: it **enumerates every loaded
company regardless of `SVCURRENTCOMPANY`**.

So requesting an existing-but-**unloaded** company returns the loaded company's rows, with
`STATUS=1`, in under 100 ms. Reading "row 1" binds to the wrong book. This is the default
state after a Tally restart, which reopens only the last-active company.

The native report path does **not** share this — it fails closed with
`Could not set 'SVCurrentCompany'`.

**No working XML company-load path was observed.** `<TALLYREQUEST>Load Company</TALLYREQUEST>`
returns `STATUS=0` with empty `DATA`; an `SVLOADCOMPANY` variable returns `STATUS=1` with
`<COMPANY>0</COMPANY>`. Those two attempts establish only that these shapes do not load a
company. Until a working supported path is observed, a client must detect the condition and
instruct the operator to open the company on the server by hand.

**What a `Company` collection fetch returned for a company's own details (2026-10-01; licensed TallyPrime 7.1 Silver, two synthetic companies, four runs).** These were raw lab requests, not ones Bridge sends: the GUID-filtered `Company` collection of §12a.13 (one formula, `$GUID = "<the company's GUID>"`, which returns exactly that company's row) with only its `FETCH` list changed, to `Name, GUID, CompanyNumber, StartingFrom, BooksFrom, LastVoucherDate, CurrencyName, StateName, PinCode, IsGstOn, GstRegistrationType, GstRegistrationNumber` and, in a second request, the same plus `Address.List`. The requests and responses were kept and are not committed to this repository. Each answer was in band with one `COMPANY` row.

- **Returned, with their text forms (VERIFIED on both companies).** `NAME`, `GUID` and `CURRENCYNAME` as `TYPE="String"`; `STARTINGFROM`, `BOOKSFROM` and `LASTVOUCHERDATE` as `TYPE="Date"`, written `yyyymmdd`; `ISGSTON` as `TYPE="Logical"` (`Yes`); `COMPANYNUMBER` as `TYPE="Number"` with a leading space. The row also carries `NAME` and an empty `RESERVEDNAME` as attributes. The elements do not arrive in the order of the `FETCH` list.
- **An unset state or PIN code is an absent element, not an empty one (VERIFIED on one company, before and after).** On one company the first answer held no `STATENAME` and no `PINCODE` element. After a state and a PIN code were entered on Tally's own screen, the same request bytes returned both, each as `TYPE="String"`. This is the collection behaviour §8.1 records for ledgers: a collection `FETCH` emits only values that are set.
- **No registration type or number on the row, even where a registration exists (VERIFIED for these two fetch names on one company, one run).** On a company whose GST registration screen shows an active regular registration with a GSTIN (seen on Tally's screen by the book's owner, not by the request), the answer held `ISGSTON` `Yes` and no `GSTREGISTRATIONTYPE` and no `GSTREGISTRATIONNUMBER` element, and no GSTIN-shaped text anywhere. So under these names the `Company` row does not carry the registration. Where a company's GSTIN can be read from was not measured.
- **The address list returned nothing (PARTIAL: one sample).** Adding `Address.List` to the `FETCH` left the answer byte-identical on one company and returned no `ADDRESS.LIST` on either company. Whether an address line was set on either company at the time is not confirmed, so this does not yet tell an unset address from a name the collection does not return.
- **Not measured:** a company with an address known to be set; any other name for the registration or the address; several registrations on one company; an Object export of the company with the same names; Education and Gold; a client book.

### 12a.8 Payload scales with voucher count; measure request time separately

The wildcard voucher fetch averaged **~21.7 KB per voucher on this corpus**, stable across a
360× range.

**Treat that as a mean, not an upper bound.** Per-voucher cost varies with content — §11 already
records this — so a book with nested inventory lines or long narrations will exceed it. A segment
projected under a transport cap using this constant can still breach it after dispatch. Sizing
must use **observed** bytes for the profile in hand, with headroom, and split adaptively when a
projection proves wrong.

| AlterID range | vouchers | payload | time | bytes/voucher |
| --- | --- | --- | --- | --- |
| 0..500 | 9 | 0.17 MiB | 1.30 s | 19,482 |
| 0..2000 | 78 | 1.59 MiB | 1.77 s | 21,390 |
| 0..8000 | 369 | 7.63 MiB | 6.72 s | 21,685 |
| 0..20000 | 948 | 19.63 MiB | 9.92 s | 21,713 |
| 2-day window, unbounded | 3,264 | 67.67 MiB | 43.67 s | 21,740 |

**A date window is not a volume bound.** A 2-day window on a dense book returned 67.67 MiB
in 43.67 s; a 31-day window returned 101.5 MiB in 81.3 s. Both exceed every stated response
limit, and both are ordinary requests.

**A pre-flight count reduced response payload on the observed windows.** A minimal `FETCH`
over those windows returned the observed voucher count with about **1/17th** the wildcard
response payload. Its elapsed-time reduction was measured separately and is not a general
cost estimate:

| window | minimal `FETCH` | wildcard | payload / elapsed ratio |
| --- | --- | --- | --- |
| 2-day | 3.98 MB / 2.61 s | 67.67 MiB / 43.67 s | 17× / 17× |
| 31-day | 5.96 MB / 5.74 s | 101.5 MiB / 81.3 s | 17× / 14× |

Incidentally, `FETCH ALTERID` and `FETCH GUID, ALTERID, DATE` return **byte-identical**
responses — Tally emits a fixed minimal envelope regardless of which narrow fields are
named. The wildcard is a **17.8× amplifier** on that baseline.

The count response can itself exceed a response limit on a dense book, or be incomplete while
appearing successful. Bound and completeness-check the count probe before using it; otherwise
recursively partition that probe too. Even then, observed bytes per voucher are a planning
estimate, not a bound. A pre-flight projection does not replace the closed streaming transport
boundary: it must fail closed when encoded response bytes exceed the cap. Planning stays upstream
to avoid dispatching likely-oversize requests; the transport cap remains the final safeguard.

**Elapsed time does not follow the same model, and subdivision does not reduce it.** The cost
has a large fixed component: Tally scans the whole collection regardless of how many rows match.
The 9-row request above took **1.30 s**, not the ~117 ms a purely linear model predicts, and
§11b.3 records a **zero-row** filter taking ~22 s on the large book. So time is closer to
`fixed_scan(book) + linear(rows)`, and **every subdivision pays the fixed cost again** — halving
a partition can increase total wall-clock even as it reduces peak payload.

Deadline planning must therefore model a book-dependent fixed scan cost separately from
serialization, and subdivision must be justified by payload, not by time.

### 12a.9 A ledger GUID survived an observed UI rename — coverage must compare GUID to name

`LedgerOpeningCoverageV1` was captured immediately before and after renaming one ledger
through the Tally UI. The response contained the same six ledger GUIDs on both sides, while
exactly one GUID's `Name` changed. Therefore a count-only comparison, and a comparison of
GUID membership alone, are both blind to this master-data change.

Bridge must compare the full GUID-to-name map and return a partial result when it changes
during a scan. This catches the observed rename without treating a stable GUID as evidence
that the ledger master itself stayed unchanged.

This establishes only a GUI rename on the observed TallyPrime Edit Log EDU profile. XML
rename behaviour, other releases, and other configurations remain unverified (a gateway rename by `Alter` on Gold is measured in §9.4).

**VERIFIED 2026-09-11; one synthetic company on TallyPrime 7.1 licensed Silver
(`education_mode: false`), read through `StandardLedgerCatalogV1`.** Two of the axes named above
are now observed. One ledger was renamed in the UI and then restored; the ledger's GUID and master
slot were unchanged, the ledger count held, and no other ledger's GUID moved — a rename in place,
not a delete-plus-create. So the finding holds across two SKUs and two read profiles. The
before/after responses are retained as
`fixtures/agent/native-ledger-catalogue{,-renamed}.utf16le.xml` and drive the binding revalidation
regression test.

Three further facts fell out of that capture. They are marked separately because two of them are
generalisations from a single company, and the rule behind them is not established:

- **PARTIAL.** A ledger GUID appears to be **company-scoped with a master suffix** —
  `<company GUID>-000000d0`. The composition was observed on every ledger of the one company
  captured, so the *shape* is verified there; that it holds across other companies, releases and
  SKUs is inferred, not tested. If it does hold, a ledger GUID is meaningful only within its
  company and the company GUID is recoverable from it — and since company GUIDs are not unique
  across split companies (§9.11b), a ledger GUID inherits that ambiguity.
- **VERIFIED 2026-09-11; the nine ledgers in the captured company.** Ledger rows carry
  `RESERVEDNAME` and use it **the same way group rows do** — populated for a reserved master, empty
  for one that is not. Eight of the nine are empty; `Profit & Loss A/c` carries
  `RESERVEDNAME="Profit &amp; Loss A/c"`.

  This is the three-state convention already implemented in
  `TallyNamedMaster::reserved_name`: a non-empty value is a trustworthy reserved identity, an
  **empty** value is Tally's own explicit signal that the row is *not* a reserved master, and an
  absent attribute is no signal at all. A parser must therefore key on neither the attribute's
  presence nor an assumption of emptiness.

  Note that "reserved" is narrower than "auto-created": `Cash` is created by Tally and its
  `RESERVEDNAME` is empty, so Tally itself declines to treat it as reserved. Do not infer
  reservation from a ledger having appeared without the operator creating it.

  An earlier revision of this section claimed ledgers emit `RESERVEDNAME` *always empty*. That was
  wrong, and contradicted by the very fixtures it cited.
- **UNVERIFIED — the `CMPINFO` counters, and a claim withdrawn.** An earlier revision of this
  section said the response was "identical except `<CMPINFO><LEDGER>` advancing 60 → 62", read that
  as a monotonic per-master-type alteration counter, and marked it PARTIAL. **Both the figure and
  the interpretation are withdrawn.** The committed fixtures show neither. Diffing them gives six
  changed lines, and five of them are `CMPINFO`:

      <LEDGER>0</LEDGER>               ->  <LEDGER>61</LEDGER>
      <CURRENCY>0</CURRENCY>           ->  <CURRENCY>14</CURRENCY>
      <TAXUNIT>0</TAXUNIT>             ->  <TAXUNIT>20</TAXUNIT>
      <VOUCHERNUMBERSERIES>0</...>     ->  <VOUCHERNUMBERSERIES>20</...>
      <VOUCHER>0</VOUCHER>             ->  <VOUCHER>19</VOUCHER>

  (the sixth is the renamed ledger itself). A counter that is **zero across every master type** in
  one capture and populated in the next is not an alteration count incremented by the rename, and
  `61` does not correspond to the nine ledgers the collection returns. What `CMPINFO` reports here,
  and why one capture reports all zeros, is **not established** — do not build on it.

  **The actionable consequence survives, and is strengthened.** A "restore and prove nothing
  changed" check must compare master **identity**, never a response hash or byte equality: fields
  outside the master set differ between two captures of the same company for reasons this reference
  cannot yet explain. That is a stronger reason to compare identity than the withdrawn one.

**UNVERIFIED — XML-driven rename (a gateway rename by `Alter` on Gold is measured in §9.4).** Neither capture used one; both renames were performed in the
UI. Deletion was not exercised at all. Per P6, neither may be built upon.

### 12a.10 An opening bill keeps a date before `BOOKSFROM` — **VERIFIED 2026-09-22; single captured book**

**Scope:** TallyPrime 7.1 Silver, licensed, one synthetic book (`BRIDGE CORPUS FOREX`), the
production Bills Receivable request for 20250401 to 20250930. Committed as
`bills_receivable_forex_live` (`LEDGER_CURRENCY_CAPTURE_PROVENANCE.md`). The book's `BOOKSFROM`,
20250401, was reported by the capturing session; it is not in the committed bytes.

**Measured:** two opening bills, `FX-OPEN-1` and `INR-OPEN-1`, report `BILLDATE` and `BILLDUE` as
`31-Mar-25`, the day before `BOOKSFROM`, with `BILLOVERDUE` `183` (20250331 to the as-of date).
`BOOKSFROM` is therefore not a lower bound on a Bills row's dates, and a parser that treats it as
one refuses the whole read (bridge#612).

**Not measured:**
- an opening bill dated more than a day before `BOOKSFROM`, or one with a credit period;
- whether a due date can precede its bill date;
- whether an as-of Bills report can list a bill dated after its as-of date. The parser assumes it
  cannot: such a bill refuses the whole read, unless it is dated fifty years or more after
  `BOOKSFROM`, when it is read into the previous century.

How Bridge resolves the two-digit years is a design choice, documented at
`native_outstandings/date.rs`.

### 12a.11 Balance Sheet and Profit and Loss by name on licensed 7.1

**VERIFIED 2026-09-26, licensed TallyPrime 7.1 Silver** (`education_mode=false`). Two synthetic companies: one with 3 vouchers, one with 29,900. One run of each request. **Confidence: PARTIAL.** This extends §12a.1, measured on 7.0 EDU for the Bills reports, to the two financial statements.

Requests used §12a.1's shape with `<ID>Balance Sheet</ID>` and `<ID>Profit and Loss</ID>`. Responses:
- **Success carries no `HEADER` or `STATUS`**, as in §12a.1.
- **Balance Sheet.** Top-level `BSNAME` / `BSAMT` pairs. The line name is in `BSNAME/DSPACCNAME/DSPDISPNAME`; amounts are in `BSAMT/BSMAINAMT` and `BSAMT/BSSUBAMT`. The default export gave five top-level lines: four primary groups and the `Profit & Loss A/c` line.
- **Profit and Loss.** Top-level `DSPACCNAME` / `PLAMT` pairs. The amount is in `PLAMT/BSMAINAMT` (a heading such as `Cost of Sales :`) or `PLAMT/PLSUBAMT` (a group such as `Purchase Accounts`).
- **Amounts are plain signed decimals.** There is no digit grouping, symbol or `Dr`/`Cr` suffix, and a debit is negative, as in the trial balance. Examples: `-4250.00`; `222962422.38` and `-222962422.38` on the larger book.
- **An empty window gives empty amount elements**, not `0.00`: a month with no activity returned all five Balance Sheet amounts empty. An empty amount is not zero (§7).
  - **A zero-valued line is empty in both columns, not `0.00` (PARTIAL: four synthetic lab books, one financial year each, 2 Oct 2026, read through the installed 0.4.1 Bridge).** Every zero line (a capital account, loans, current liabilities, a direct-expenses group with one ledger and two empty amounts) came back empty in both columns; no line was present as `0` in both. No client book was read, and the case of a present `0` in both columns has not been observed. Bridge treats that shape as a zero amount and compares it with the derived line (#1067); an empty pair is compared the same way.
- **The figures tie to the native trial balance (§5.6)** for the small book's full year. Current liabilities equalled the sundry creditors' TB closing. The P&L line equalled the P&L ledger's TB closing (`0.00`) plus the year's result. `Purchase Accounts` equalled the purchase ledger's TB closing.
- **A part-year window (PARTIAL: one book, one month).** On the 29,900-voucher book, for 2025-05-01 to 2025-05-31, the statements were read beside the native TB and group tree in the same two minutes. Every line tied.
  - The book's one P&L ledger (sales) had a TB covering the window only. Its `TBALOPENING` was `0.00`, and its `TBALCLOSING` equalled the month's movement, `113726661.73`. That is the P&L's `Sales Accounts` line.
  - The `Profit & Loss A/c` ledger's `TBALOPENING` was `109235760.65`. That is the negation of the current-asset ledgers' combined opening. No ledger other than those current-asset ledgers and the `Profit & Loss A/c` ledger opened non-zero. So the year's earlier result is carried in that ledger rather than in the sales ledger's opening (inferred from the balancing, not from a voucher read).
  - The Balance Sheet's `Profit & Loss A/c` line, `222962422.38`, equalled that ledger's TB closing plus the sales ledger's TB closing.
  - The one-month TB (123 ledgers) was 150 KB, the group tree 54 KB and the one-month P&L 368 B.
- **Cost:** 0.14–0.72 s and 0.7–1.8 KB per request on the small book; **0.23 s and 1.8 KB for a one-month Balance Sheet on the 29,900-voucher book**. No full year was requested on the larger book.

`<ID>Stock Summary</ID>` returned an empty `<ENVELOPE/>` on a company not known to hold inventory. An empty envelope cannot tell "no items" from "not rendered", so it is not read as zero. **Not measured:**
- closing stock's line in either statement (the Stock Summary on a book with inventory is §12a.13);
- the explode flag;
- a foreign-currency book;
- Education and Gold.

---

### 12a.12 Master collections on licensed 7.1

**Measured 2026-09-30 on licensed TallyPrime 7.1 Silver** (`education_mode=false`). The fixtures come from two synthetic companies. Two stock-heavy client books were read only for the figures marked "by role", and nothing from them is committed. One run of each request. **Confidence: PARTIAL**

- **Four kinds read as one collection each.** Voucher types, godowns, units and stock groups can each be read as one `TYPE=Collection` export with `<COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE>` (the group snapshot's compute).
  - Every row of every kind carried the computed GUID, equal to the company's.
  - A collection with rows came back as one `COLLECTION` element.
  - **A kind with no master:** on a synthetic book without inventory, units and stock groups returned `STATUS` 1 with a present, empty `COLLECTION` element (its `ISMSTDEPTYPE` and `MSTDEPTYPE` attributes, no rows). It was not absent and not an empty envelope. Godowns returned one row, the company's default location; whether a company can answer with no godown is unmeasured, so a zero-row godown answer is admitted like the other inventory kinds (voucher types are refused, because every company has predefined ones).
  - Bridge refuses a zero-row `voucher_types` answer (`masters_voucher_types_empty`), because every company has predefined voucher types. That is a rule Bridge applies, not a measurement: every captured voucher-type answer had rows.
  - Fixtures and exact requests: `tests/fixtures/MASTERS_CAPTURE_PROVENANCE.md`.
- **Numbering method.** `NUMBERINGMETHOD` is a direct child of each voucher-type row when fetched on `List of VoucherTypes`.
  - `NUMBERINGMETHOD` alone adds about 130 bytes per row: on a client book, by role, the same request without it was about 38 KB for 24 types, and about 41 KB with it.
  - Bridge's request adds four things to `List of VoucherTypes`: `ISACTIVE`, `ISOPTIONAL`, `NUMBERINGMETHOD` and the `BRIDGECOMPANYGUID` compute. In the committed synthetic capture they add about 500 bytes per row (UTF-16, a row counted from its opening to its closing tag, line ends and indentation included), so its rows average about 1,700 bytes against about 1,200 in production's `List of VoucherTypes` capture, roughly 40% larger. Apart from those four tags the rows carry the same tags as production's, so the row population is the same.
  - Values seen: `Default`, `Automatic` and `Manual` on a synthetic book (24/1/1), and `Automatic` and `Manual` on a client book.
  - What `Default` means for numbering is unmeasured, and a value outside these is unmeasured. Bridge reports the value raw rather than refusing it.
  - `PREVENTDUPLICATES` was not returned by this shape.
- **The company's voucher-type count is not the row count.** The `Company` collection's `NUMVOUCHERTYPES` did not equal the voucher-type rows returned: 35 against 26 on a synthetic book, and 33 against 24 on a client book. On that client book it equalled `NUMVOUCHERNUMBERSERIES` (33), so it may count number series. That is an inference, unmeasured.
- **Names and aliases.** Voucher-type, godown and stock-group rows carry `LANGUAGENAME.LIST`, so aliases add names without a fixed limit. The synthetic book's unit rows did not carry it.
  - A whole read can only be sized by assuming a longest name and an alias count.
  - Bridge checks each row's length against that assumption after the read, and refuses a row beyond it.
- **Every captured row had its own AlterID at or under the book's mark.** On both synthetic books the AlterIDs were distinct and the largest sat under `ALTMSTID`: 269 against 289 on one, and 100 against 242 on the other (the one godown). For godowns, units and stock groups, Bridge sizes a read from this premise before sending it, and refuses a response that breaks it. Voucher types have no size check before the read and are checked after it (rows, AlterIDs and response size), and groups are not checked at all.
- **Master marks run far past the counts.** The two client books had master-alteration marks (`ALTMSTID`) of about 100,000 and 300,000 (rounded, by role), against a few thousand ledgers. A size estimate from the mark refuses both, and any book whose mark is above about 1,150 (the admission limits for godowns, units and stock groups are 1,152, 1,168 and 1,160) refuses those three kinds. How common such marks are across live books is unmeasured; both client books measured refuse.
- **Not measured:**
  - a numbering value other than the three seen;
  - what `Default` numbering does on import;
  - a book whose masters were copied from another company (whether row GUIDs keep the source prefix);
  - how a deletion moves `ALTMSTID`;
  - Education and Gold.

### 12a.13 Stock items and the Stock Summary by name on licensed 7.1

**VERIFIED 2026-09-30, licensed TallyPrime 7.1 Silver** (`education_mode=false`). The fixtures come from one synthetic company with integrated inventory (11 items, 3 stock groups). The tie in the second bullet was also measured on one client book, by role, and nothing from it is committed. One run of each request. **Confidence: PARTIAL.** This extends §12a.11, where `Stock Summary` returned an empty envelope on a book not known to hold inventory, to a book with inventory.

- **Stock items as one collection.** The `AuditStockItemsV1` request (`TYPE=Collection`, `StockItem`, `FETCH` of `NAME, GUID, PARENT, BASEUNITS, OPENINGBALANCE, OPENINGVALUE, CLOSINGBALANCE, CLOSINGVALUE`, over a financial year) returned one `COLLECTION` with one `STOCKITEM` row per item. Fixtures and exact requests: `tests/fixtures/STOCK_CAPTURE_PROVENANCE.md`.
  - The row GUID carried the company's GUID as its prefix. There is no computed `BRIDGECOMPANYGUID` on this request, so the GUID prefix is the row's only binding to its company.
  - A quantity is `<number> <unit>` (`100 Box`, `400.000 Kgs`), sent with a leading space; a negative one has no space (`-50.000 Kgs`). A value is a plain signed decimal with no grouping or symbol. **Both signs appeared among items holding a positive quantity** (`100 Box` with `2500.00`, `400.000 Kgs` with `-1000.00`). Bridge keeps the sign as sent. By the sign measurement below, `-1000.00` is stock held and `2500.00` is a value Tally's screen shows as negative; how this book's values came to be positive is not recorded.
  - An empty quantity or value element means nothing was sent, and is not zero: three items carried an empty closing quantity and an empty closing value, and one item carried `-50.000 Kgs` with an empty closing value. An opening value of `0.00` was sent as a value.
  - `LANGUAGENAME.LIST` (aliases) was present on all eleven rows, each with one name. The largest row was 654 UTF-16 units with its line ends (641 without the carriage returns), names included.
- **The Stock Summary by name.** `<ID>Stock Summary</ID>` in §12a.1's shape (`TYPE=Data`, the statements' envelope) returned no `HEADER` or `STATUS`, and one `DSPACCNAME` / `DSPSTKINFO` pair per top-level stock group (three here).
  - A pair's amount is `DSPSTKINFO/DSPSTKCL/DSPCLAMTA`. `DSPCLQTY` and `DSPCLRATE` sat beside it, empty. The amounts were `18750.00`, `14500.00` and `-30249.99`.
  - **The report's closing amounts summed to the stock items' `CLOSINGVALUE` sum exactly (`3000.01`).** The same equality held on a client book, by role, at the same period end. It is measured at one financial year end only: 31 March 2026, the period 20250401 to 20260331. Other years' 31 March share that request shape but not the measurement. (Since 2026-10-29 `stock_summary` admits a month's last day: the period-ends bullet below.)
  - An empty `<ENVELOPE/>` (§12a.11) is not a total of zero: it is an answer that cannot be told from a report Tally did not render, and is not compared.
- **The company's inventory flags.** The `Company` collection with its `FETCH` extended by `ISINTEGRATED, ISINVENTORYON, ISBATCHWISEON` (and seven `NUM*` counts) and one single-term filter, `$GUID = "<the company's GUID>"`, returned exactly that company's row (§12a.7: without the filter every loaded company is returned). The three flags were `Yes`. `NUMSTOCKITEMS`, `NUMGODOWNS` and `NUMUNITS` equalled the rows of the stock items, godowns and units captures; `NUMVOUCHERTYPES` did not equal the voucher-type rows (§12a.12).
- **The sign of a stock value (2026-10-01; VERIFIED on one synthetic company, licensed TallyPrime 7.1 Silver, against Tally's own Stock Summary screen as read by the book's owner).** A negative value on the wire is a debit: stock held.
  - A stock item imported with `OPENINGVALUE` `-50.00` (5 units at 10.00) read back with a closing value of `-50.00`, and Tally's screen showed it as 5 units at 10.00, value 50.00.
  - A stock item imported with `OPENINGVALUE` `50.00` read back as `50.00`, and the screen showed `(-)50.00`.
  - The report total read as `5584.56` while the screen's grand total showed `(-)5,584.56`: the same magnitude, the opposite presentation.
  - A purchase whose goods line was sent as a debit (`-300.00`) moved its item's value from `500.00` to `200.00`.
  - So an importer that sends a positive opening value creates what Tally shows as negative stock value, and the Stock Summary report nets signed values exactly as the item collection does.
  - `stock_summary` keeps the wire sign, as the Trial Balance does (a debit is negative), and says so. Not measured: other releases, a book with several currencies, and a value that arises only from vouchers with no opening.
- **The stock item count after a delete (2026-10-01; PARTIAL: one sample, licensed TallyPrime 7.1 Silver).** On one synthetic company, after one stock item with no vouchers was deleted on Tally's own screen, `NUMSTOCKITEMS` fell from 11 to 10 and equalled the stock item rows read (10); the master alteration mark rose by one. The count was read with the flags request's own bytes, and the rows through a build of master. `stock_summary` refuses a read whose rows differ from this count in either direction (`stock_summary_item_count_differs`): a count that disagrees is not counting the list that was read. An equal count is a cross-check, not proof of a complete list.
- **A company with inventory on and no stock item (2026-10-01; PARTIAL: one synthetic company, licensed TallyPrime 7.1 Silver, captured through `bridge_mcp`).** `NUMSTOCKITEMS` was written `0`, with no leading space (a non-zero count has one, ` 11`). The stock items collection came back with `STATUS` 1 and a present, empty `COLLECTION`. The Stock Summary came back as `<ENVELOPE></ENVELOPE>`. `stock_summary` answers `no_stock_items` only when all three agree. The same run re-took the three requests on the first company through `bridge_mcp`: the answers were byte-identical to the captures of 2026-09-30.
- **The Stock Summary with the explode flag (2026-10-01; PARTIAL: one synthetic company of 11 and then 12 stock items, licensed TallyPrime 7.1 Silver, two runs).** This was a raw lab request, not one Bridge sends: the Stock Summary request above plus one static variable, `<EXPLODEFLAG>Yes</EXPLODEFLAG>`. A control read with the product's own bytes was taken beside it, and the items were read through `stock_summary` in the same session.
  - The answer stayed in band and was **flat**: every line is a top-level sibling in document order, and nothing is nested. Each top-level line of the plain report was expanded by **one level**: a stock group was followed by its items, and a stock item that sits directly under the root was followed by one `SSBATCHNAME` (`SSBATCH`, `SSGODOWN`) and a `DSPSTKINFO` repeating its figures.
  - An item line carries `DSPCLQTY` (`<number> <unit>`), `DSPCLRATE` and `DSPCLAMTA`. **For each of the nine items that held stock, the quantity text and the amount equalled the item collection's `CLOSINGBALANCE` and `CLOSINGVALUE`**, across two units, a three-decimal quantity and a negative amount.
  - A group line carries an amount and no quantity (its items had two units). In the plain report, an item directly under the root is a top-level line with its quantity and rate.
  - Limits, which is why the exploded report is not used for a quantity (the plain report is, below):
    - the two items with an empty quantity and value had **no line**, so the report cannot vouch for an item it omits;
    - only one level is expanded. In a second run, after a stock group was created inside the first and one item holding stock was put in it, the nested group appeared as one line under its parent (an amount, no quantity) and **the item inside it had no line**;
    - the answer is flat, so a group line and an item line differ only by position and by the empty quantity (a nested group's line sits among its parent's items), and nothing but its name ties a line to an item;
    - the amount on an item line is already covered by the total; the quantity is the only new information.
  - Candidate ways to reach items below the first level, all **unmeasured** and none sent: the same request scoped to one stock group at a time, repeated for each nested group (which needs the report variable that scopes the Stock Summary to a group, not yet identified from Tally's own documentation); a report variable that expands every level, if Tally has one; and another built-in report that lists stock items directly. Until one is measured, quantities are not checked on a book with nested stock groups (since 2026-10-27 an item inside a group carries its item row's own quantity, uncompared: the nested-groups bullet below).
- **A quantity is returned where the plain Stock Summary agrees (build note 2026-10-07 on two real captures; run through the tool on a live Tally 2026-10-09: VERIFIED on one new synthetic book of two items directly under the root, equal to Tally's own Stock Summary on every point; PARTIAL beyond that book).** `stock_summary` returns an item's closing quantity only where exactly one line of the plain report carries the item's name and shows the same quantity, the same unit and the same amount as the item row (two empty amounts agree). A line is tied to an item by those four things and by the item's own parent being the root, because the report does not say whether a line is a stock group or an item. An item whose own parent is not the root (a stock group) is taken to have no line of its own, because in the captures the report listed only what sits directly under the root (none holds a group and an item under the root side by side), so none is returned for it (a line of the same name is read as the group's); an item with no parent sent gets none either, and the exploded report is not used. Every other item says why no quantity is returned (`none_sent`, `unread`, `inside_stock_group`, `report_has_no_line`, `report_name_not_unique`, `report_differs`). The two captures are the item rows and the plain report after a sale of an item that held none: both showed `-5 Nos` with no value for it, and `18 Nos` with `-186.00` for the other item. The live run: an outside contributor, on the maintainers' Windows laptop (TallyPrime 7.1 Silver, licensed), a new synthetic company `BRIDGE STOCK QTY LAB` built to the list of the captured book (books from 2025-04-01, inventory on and integrated, batches off, no godowns, no GST, unit `Nos`, items `Lab Item NEG` and `Lab Item POS` directly under the root, the same vouchers; Tally warned on the sale of `NEG` and saved it), the only company loaded, one `stock_summary` for 2026-03-31 through `bridge_mcp` built at this pull request's earlier head `5ef86b972`, driven over stdio, nothing else on the port. Tally's own Stock Summary for 1-Apr-25 to 31-Mar-26: `NEG` (-)5 Nos with no rate or value, `POS` 18 Nos, rate 10.33, value 186.00, total 13 Nos, 186.00. The tool: `NEG` `agreed`, -5 `Nos`, value null; `POS` `agreed`, 18 `Nos`, value -186.00 (the wire sign); `checks.closing_quantity` `checked_per_item`; two items returned; `item_count_cross_check` 2 rows, Tally count 2, `matched`; `tie_out` `matched`, total -186.00. The answer was 12,194 bytes (sha256 `d85f8a0a1d38ac317c55e3a152a375f55a0abb7dd67d2d13bcc194977c3f7ae1`); the evidence: complete, 39,242 bytes. Not measured: a book whose stock sits in groups, a group that carries an item's name, a compound unit, other dates, other releases, macOS. Superseded for items inside a stock group by the 2026-10-27 bullet below: such an item now carries its row's own quantity, uncompared, and `checks.closing_quantity` is `checked_where_agreed`.
- **Items inside nested stock groups: the item list's quantity at every depth equals Tally's expanded Stock Summary (2026-10-27; VERIFIED on one synthetic book, TallyPrime 7.2 Silver, one sitting; PARTIAL beyond that book).** A new company `BRIDGE NESTED STOCK LAB` was keyed on Tally's screens: units `Nos` (0 decimals) and `Kgs` (3 decimals), three stock groups nested three deep (`Lab Group One` under the root, `Lab Group Two` in it, `Lab Group Three` in that), eight items (one directly under the root beside the first group, the rest at depths one to three in both units, one with no opening and one with a negative opening quantity) and two Stock Journals, each moving 5 `Nos` from an item at depth three to one at depth two: one dated inside the period, one after 31 March 2026. The product's own three requests (the flags, `AuditStockItemsV1` and the plain report, byte-equal to the builders for this company) were sent by curl and the tool was run once. Fixtures and exact requests: `tests/fixtures/STOCK_CAPTURE_PROVENANCE.md` (`*_nested_lab_*`).
  - The item collection returned all eight rows, every depth, with the parents as keyed. `NUMSTOCKITEMS` was 8: Tally's count includes items two and three levels down.
  - The plain report had two lines: the top-level group (an amount, no quantity) and the item under the root (quantity, rate, amount). No line carried an item inside a group. The item under the root agreed with its row, so a group and an item under the root side by side do not disturb the agreement above.
  - On Tally's Stock Summary screen, with `Format of Report` set to Detailed and `Expand all levels in Detailed format` set to Yes (each set back afterwards), transcribed twice a minute apart: every item with a quantity showed the same number and unit as its row's `CLOSINGBALANCE` (`2.500 Kgs` against ` 2.500 Kgs`, `(-)0.750 Kgs` against `-0.750 Kgs`), and the same value with the wire sign reversed. The item with no opening had an empty quantity and value in its row and no line on the screen. Each group's line equalled the sum of its items' values at every depth below it.
  - The two items moved by the Stock Journals showed their 31 March quantity in the row and on the screen, not the quantity after the later voucher. 31 March 2026 was also the end of the company's own period, and the later voucher lay outside both, so this capture does not tell a collection that follows `SVTODATE` from one clipped to the company's period. The mid-year reads on `BRIDGE TIEOUT D` (2026-10-26) do tell them apart, for one item under the root: its row and the plain report each followed `SVTODATE` (30 September and 31 May 2025).
  - `stock_summary` therefore returns an item inside a stock group with its own row's quantity, under the state `inside_stock_group` and the keys `item_list_amount` and `item_list_unit`, never as `agreed`: nothing in the read compares it. A parent that reads `Primary` without the root marker, or carries a marker anywhere else, is `parent_not_resolved`, with no quantity.
  - Not measured: another book or release (7.1 included), more than three levels, a group named like an item, godowns, batches, a compound unit, an item inside a group with a quantity and an empty or zero value, and any `as_of` other than 31 March 2026 on a book with groups (31 August and 30 September 2025 were measured since: the period-ends bullet below).
- **Period ends inside the year, and a voucher dated on the end date (2026-10-26 and 2026-10-29; PARTIAL: two synthetic books, one sitting each, TallyPrime 7.2 Silver).** Each read was the product's own item collection and plain Stock Summary request with only the period changed, sent by curl. Fixtures: `tests/fixtures/STOCK_CAPTURE_PROVENANCE.md` (`*_tieout_d_to_*`, `*_nested_lab_to_*`).
  - `BRIDGE TIEOUT D`, one item under the root, to 30 September and 31 May 2025: the row and the report line agreed, at each date, with each other and with Tally's screen, and showed the quantity at that date rather than the one after a later sale.
  - The nested book, to 31 August and 30 September 2025: all eight rows at both dates, every nested quantity equal to Tally's Stock Summary screen with every level expanded, read twice. The two items the Stock Journal of 15 September moved showed their quantity before it at 31 August and after it at 30 September; every other item was the same at both dates; each report's amounts summed to its date's closing values. The company's own current period on the Gateway was 1-Apr-26 to 31-Mar-27 throughout, so these reads of the previous year followed their own `SVFROMDATE`/`SVTODATE`, not the company's period: this answers, for this book, what the 31 March capture above could not.
  - `BRIDGE TIEOUT D` to 15 October 2025, the date of its one sale that month: the row, the report line and Tally's screen (twice) all showed the quantity after the sale, and Tally's Stock Item Vouchers for 1 to 15 October (twice) showed that sale, dated 15 October, as the only movement. The end date is inclusive in both sources, on this evidence: one sitting and one Sales voucher (lab branch `lab/1233-tieout-d-end-date-answers` at `1c9669fd7`; not committed as fixtures).
  - `stock_summary` therefore admits an `as_of` that is a month's last day (the shape of every period end measured), and refuses any other date as `stock_summary_as_of_not_measured`. The period still starts on 1 April of the year containing `as_of`, or the book's start if later; the company's own financial-year start is not read.
  - Not measured: February's end, a month end with a voucher on it (the end-date read was on 15 October), several vouchers on the end date, a Stock Journal or a purchase dated on it, TallyPrime 7.1 at a date other than 31 March, a year after the company's current period, a book that starts mid-year, a company whose financial year does not start on 1 April, and any other book.
- **A sales goods line on a batch-wise company needs a batch allocation (2026-10-01; VERIFIED on one pair, one synthetic company, licensed TallyPrime 7.1 Silver).** Two imports of one sales item invoice that differed only in whether the goods line carried a `BATCHALLOCATIONS.LIST`: the one without was refused in band, with no message; the one with it was created. The allocation named Tally's default godown and default batch, and that addressed stock which the exploded Stock Summary had shown with an empty batch name. Both import requests and Tally's two responses were kept; they are not committed to this repository, and the import shape with a batch allocation is not yet recorded in any part of this reference. Not measured: a purchase line or any other voucher type without an allocation, and a company that is not batch-wise.
- **A taxed sales item invoice is stored as sent (2026-10-01; VERIFIED on one voucher, one synthetic company, licensed TallyPrime 7.1 Silver).** One sales item invoice imported with two credit entries on ledgers under Duties & Taxes (masters created by import with `TAXTYPE` `GST` and the duty heads `CGST` and `SGST/UTGST`, §8.3, with no change to the company) was created, and read back with both tax entries at the amounts imported: Tally did not recompute or drop them. The import request and Tally's response were kept and are not committed to this repository. The voucher carried no GSTIN, place of supply, HSN or rate. In a fetch of `ALLLEDGERENTRIES.*` it read back as four ledger entries (the party, the sales ledger with the goods line nested under it, and the two tax ledgers); in a fetch of `ALLINVENTORYENTRIES.*` the party and the two tax ledgers arrive as ledger entries beside the goods line, and the sales ledger only as the goods line's accounting allocation. The company's GST state was not controlled: a raw lab read of its Company row later the same day returned `ISGSTON` `Yes` and no registration type or number element, and what registration it holds was not established. Not measured: an inter-state line, a tax Tally computes itself, a company whose GST flag reads `No`, and a company whose registration is known.
- **A stock value changed sign after an item's first outward line (2026-10-01; PARTIAL: one item, the same company; unexplained).** The item imported with a positive opening value in the sign measurement above (`50.00` for 5 units at 10.00, which Tally's screen shows as negative) read back, after one sale of 2 units, with a closing value of `-30.00`: the remaining quantity (3) times the opening rate, with the ordinary sign of stock held. Another item's value on the same book moved linearly under a purchase. So on a book whose openings were imported with a positive value, the sign of an item's value can change as it trades; nothing here says by what rule.
- **A sale of an item that held none created a stock line of a negative quantity, with no value (2026-10-07; PARTIAL: one voucher, one item, one synthetic company, one sitting with each request answered twice by identical bytes, licensed TallyPrime 7.1 Silver).** One sales item invoice dated 8 July 2025 (5 Nos at 25.00 per Nos, no batch and no godown) was imported as the only voucher of its import, for an item whose closing balance read empty before it. The company was not batch-wise and not GST-enabled (`ISBATCHWISEON` and `ISGSTON` `No`, read just before the sale), and eight accounting vouchers with no stock lines were imported between the two reads. The item collection (the `AuditStockItemsV1` request, 20250401 to 20260331) and the Stock Summary by name (§12a.13, the plain report) were read before and after, from one Tally, in one sitting. The captured responses are not committed to this repository.
  - Before the sale, the item's row carried an empty `CLOSINGBALANCE` and an empty `CLOSINGVALUE`, and the Stock Summary had no line for it.
  - After the sale, the row carried `CLOSINGBALANCE` `-5 Nos` (no leading space, as above) and still an empty `CLOSINGVALUE`. The Stock Summary then had a line for the item: `DSPCLQTY` `-5 Nos` and an empty `DSPCLRATE` and `DSPCLAMTA`. The other item's row and line were unchanged (` 18 Nos` and `-186.00` in the row). So Tally kept the stock line of the sale, as a quantity below zero, and sent no value for it; an empty value is nothing sent, not zero (above).
  - The row and the line agreed on the negative quantity. The exploded report's agreement with the rows (above) was recorded on a raw lab request, not on this plain report, and is not stated for a quantity below zero.
  - `stock_summary` withholds every quantity, so its answer for this book shows the item with `closing.value` `null`, one `empty_closing_value_count` and no trace of the `-5 Nos`. The quantity is on the wire and is not returned. The voucher read returns amounts only, so it does not show the stock line either.
  - Not measured: whether the value stays empty after a later voucher or an opening is set; how Tally's screen presents the line (no screen read was taken); an item in a godown or batch; and other releases.
- **Cost, by role, on a stock-heavy client book (several thousand items; figures rounded).** The Stock Summary report took about 1.5 s (about 150 KB, a few hundred top-level lines). One span of 2,000 items took about 1 s (about 3 MB). Bridge does not read stock items on such a book: its master-alteration mark is far above the admitted size (§12a.12).
- **`NUMSTOCKITEMS` against the item rows through a run of operations (2026-10-02; PARTIAL: one synthetic company of 12 to 55 items, once each; licensed TallyPrime 7.1 Silver).** After each of eight imports the Company collection's `NUMSTOCKITEMS` and the `AuditStockItemsV1` rows were read and compared by parsing: the count equalled the rows (and no name repeated) at the baseline and after every step. Each import's counters were as expected and every other counter was 0.
  - Three new items (CREATED 3): both rose by 3. Two imports of 20 items (CREATED 20 each): both rose by 20 each.
  - A Create on an existing item name: CREATED 0 / ALTERED 1, no new row, the count unchanged, and **none of the fields read for the item changed** (opening and closing quantity and value, parent, name entries): a different opening quantity and value in the request were not applied. For a stock item, one item, once. For a ledger, §9.4g records (one run, PARTIAL) that a `Create` on an existing name with a changed parent, bill-wise flag and opening balance replaced all three, also answering `CREATED=0, ALTERED=1`: the two master types gave different results here, and neither result is generalised to the other.
  - A rename by an Alter that carries only the language-name list: accepted (ALTERED 1): the item's name in the rows changed from the old to the new, one name added and one removed, and the count was unchanged.
  - One second language name (an alias) added to one item the same way: accepted, and the item's row then carried **two name entries** (`LANGUAGENAME.LIST/NAME.LIST/NAME` in the row, one before), while neither `NUMSTOCKITEMS` nor the row count changed: **this alias was not counted as an item.** The "each with one name" observation above is a property of that capture, not a rule.
  - An Alter with only a `PARENT` (ALTERED 1): accepted, and only the parent changed; the item's quantities and values were unchanged.
  - An inventory purchase of five units of an existing item (one voucher, CREATED 1): the item's closing quantity and value moved by the voucher, and the count and rows were unchanged. The voucher's batch name was accepted on an item whose batch tracking was not examined; whether it was kept is not checked.
  - **Not measured:** an item created by hand on Tally's screen, a delete of an item that has vouchers, removing an alias, a merge, a reload of the company, a book of thousands of items, a count after a crash or an interrupted import, and other Tally releases than licensed 7.1 Silver. One synthetic company is not evidence about a client book.
- **Not measured:**
  - a book whose inventory is not integrated with the accounts (what the report and the items then show);
  - an `SVTODATE` other than 31 March 2026 (another year's 31 March, a day 1 or 2, any other date);
  - the date a stock item's `OPENINGBALANCE` and `OPENINGVALUE` are as at: the captured book's `BOOKSFROM` (20250401) equals the request's `SVFROMDATE`, so this capture cannot tell an opening at the period start from one at the books' beginning. `stock_summary` reads and validates both and returns neither;
  - a quantity in a compound unit, or one with a unit that has a space;
  - the stock item count after a delete of an item with vouchers, after a merged or altered item, and on other releases;
  - the report with a godown or batch split;
  - the explode flag on a larger book and with a compound unit, and any way of expanding more than one level;
  - a foreign-currency book;
  - Education and Gold.

---

### 12a.14 One party's bill trail, tied to the native balance

**PARTIAL: observed by read-back on one seeded synthetic book** (TallyPrime Silver 7.1, `BRIDGE OUTSTANDINGS LAB`,
seeded by this project; seeded data is not evidence about a client book). Each row below was read
from Tally's own Bills Receivable and Bills Payable reports and from the vouchers' bill allocations
at one as-of date, and the `outstandings` party detail (#945) ties the two. The captures committed
with that change are the voucher window, the two bills reports, and the responses of two whole
`outstandings` calls with a party detail (`native-outstandings-detail-*`, with an ordered record of
each call's 66 requests): an `unadjusted` detail of the on-account debtor, tied (on-account sum and
residual both 3,000), and a `bill_trail` of one named bill, tied. Every response body is kept byte
for byte (UTF-16LE with no byte-order mark, as received; the status read as received) except three
listings, the company list, the book extent and the company marks, which answer for every loaded
company and are trimmed to this company's row, with the untrimmed size and hash in their records.
The HTTP heads are described in the sequence records, not kept. The unreferenced-opening row rests
on a read-back through ComplyEaze Bridge that is not captured as a fixture:

| What was measured | Result |
| --- | --- |
| The same bill reference on two parties | two rows in one Bills Receivable report, so a bill is identified by its party as well as its reference |
| A reference reused by the same party a year later | stored as an `Agst Ref` carrying the original bill's date (see 12a.2), so within one party a reference is one continuing bill |
| A journal that moves a bill from one party to another | the voucher's own party field names the first (debit) ledger, while the `Agst Ref` sits on another ledger's entry, so allocations belong to the party of the entry they sit on, not of the voucher's party field |
| An on-account receipt and an on-account payment on one party | they net into one unallocated figure |
| A note or an on-account entry with no reference | it appears only in the party's unallocated residual (closing balance less its named bills), never as a bill |
| A referenced opening balance | it is a bill, with no voucher; an unreferenced opening is residual only |
| An advance or a pending note with a reference | listed as a named bill in the report of the opposite direction, with no type label (12a.1) |

**Not shown:** any client book; a multi-currency book; cost centres; whether a `PARTYLEDGERNAME`
term in the request would be lossless for the journal above (it was not tried, which is why the
party detail reads the whole window and filters by each entry's own ledger); the cost of that
whole-window read on a large book.

**Captures of 9 October 2026 (capture 4 of #1356): VERIFIED on one synthetic party of one book.** TallyPrime Silver 7.1
(licensed), Windows 10, the synthetic company `BRIDGE BILLTRAIL LAB` (books from 1 April 2025, bill-wise on, inventory off,
the only company loaded), through the published 0.5.1 `outstandings` tool with `party` and `detail`. Every result was
compared, bill for bill, with Tally's Ledger Outstandings (Pending Bills) screen for the same date; the screen was compared
by screenshot, and the screenshots are kept, not committed. The ledgers and five vouchers were created through the lab
import tool and read back; the Credit Note and the post-dated receipt were keyed by hand in Tally's screens. The answers
are kept unchanged with these sizes and hashes:

| As of | Detail | State | Bytes | SHA-256 |
| --- | --- | --- | --- | --- |
| 31-Jul-2025 | `bill_trail` | `bills_listed`, 4 vouchers read | 8,096 | `86e89c3bd7318035b7240dec759e12ccc32d22969f138f27fc0f5aa47bb6673e` |
| 31-Jul-2025 | `unadjusted` | `residual_not_explained_by_vouchers` | 6,412 | `4bf23b2c00d5dbaa81232241362deaae2237c2d9efdc90cef179c38252f432b8` |
| 31-Mar-2026 | `bill_trail` | `bills_listed`, 7 vouchers read | 9,608 | `da006110f4722b4923e7fad910457c8b7bbc0286d0c8c0bedef980045d762800` |
| 31-Mar-2026 | `unadjusted` | `residual_not_explained_by_vouchers` | 7,494 | `9b4a3291acdc5d7dfdb258bf2919ef9a080944953d9bbe19e27477f7cb00d6d6` |
| 28-Feb-2026 | `bill_trail` | `bills_listed`, 6 vouchers read | 9,196 | `fea009bb902258c552e686594c5b53354f481bac9d87a667315689e4bbc2d0b2` |
| 28-Feb-2026 | `unadjusted` | `residual_not_explained_by_vouchers` | 7,494 | `8eb81d86757ec1e816fe7a0df10d5c544a7dfb045c513bfefa1042bc44230ca3` |
| 31-Mar-2026, after a hand-keyed opening bill | `unadjusted` | `residual_not_explained_by_vouchers`, 7 vouchers read | 7,794 | `106e92ded3552b74e42fd712414d3894dc680a34a5b70ec39e18dd3815e17060` |

| What was measured | Result |
| --- | --- |
| An `as_of` earlier than the last voucher | the window ends at the `as_of` (4 vouchers read of 7 at 31-Jul-2025, 6 at 28-Feb-2026); bills and balances equal the screen for that date. VERIFIED |
| An advance later adjusted by an invoice | the bill trail lists both allocations as made and marks the bill closed; the unadjusted detail still lists the advance as an `advance` row at the amount allocated (4,000), with `native_balance` null and `native_rows` 0 in all four unadjusted reads at 31-Jul-2025, 28-Feb-2026, 31-Mar-2026 and 31-Mar-2026 after the opening bill; the screen does not list it. VERIFIED, as "no netting is built on a later adjustment". Whether a closed advance should be hidden is a choice not yet taken; it needs a second book showing a later invoice adjusting an advance |
| A post-dated receipt dated before the `as_of` | listed as an `Agst Ref` allocation on its invoice at 31-Mar-2026 (3,000 left) and counted by the screen too. VERIFIED for this case |
| The same post-dated receipt, with an `as_of` before its date (28-Feb-2026) | left out of the tool's window and of the screen (6 vouchers, the invoice stays at 5,000). VERIFIED. The screenshot for 28-Feb was taken after the opening bill below was added, so its On Account line (4,600) differs from the unadjusted residual read before it (-14,600) by exactly that bill's 10,000; the 28-Feb reads were not repeated after the change |
| A note with a reference and a note without one | the first is a named bill of the opposite sign (`pending_note_with_reference`); the second is `on_account` and its 400 is in `on_account_sum`. VERIFIED |
| The unadjusted `residual` | equals the screen's On Account line on both dates (-15,000 at 31-Jul-2025, -14,600 at 31-Mar-2026). VERIFIED |
| An opening bill with a reference, keyed by hand (10,000 Dr; Tally rejected 1-Apr-2025 as its date and kept 31-Mar-2025) | the unadjusted detail does not list it as a row; its `difference` falls from -15,000 to -5,000 and `difference_equals_opening_balance` becomes false; the `residual` (-4,600) equals the screen's On Account line; `totals.receivable` (scope `open_bills_only`) rises from 15,000 to 25,000, which includes the opening bill. VERIFIED on this book |
| `totals` with scope `open_bills_only` before the opening bill | receivable 25,000 at 31-Jul-2025 and 15,000 at 31-Mar-2026, the open debit bills only; the screen's totals (37,000 and 26,000 Dr) also include On Account and the credit bills. Recorded as observed |

The lab import tool did not create the opening bills it was given (Tally showed no bill and the whole 15,000 as On Account);
its read-back checked only the opening balance. That is a defect of the lab tool, not of the shipped reads (#1500; the lab
tool now refuses a ledger's opening bills before any request, #1508). **Not measured:** `bill_trail` with an opening bill; an
opening bill with a credit sign; more than one opening bill; narrowing by `reference`; the three captures of #1356 that are
still open (a party term against an unfiltered control, a per-ledger or per-bill native report, the cost on a large book).

**What the tie-out does not prove.** The detail reads the company's two change marks before the
bills and again after the voucher window and refuses when they moved (`voucher_window_changed_during_read`), so a
voucher posted between the reads is a refusal and not a broken tie; a change that moves neither mark is not seen (one
letter-case-only ledger rename moved neither, in one book), and allocations that net to zero can still tie. An unadjusted `tied`
means the on-account sum equals the residual, not that the residual's composition is proven. One
unadjusted tie-out is captured whole (the on-account debtor above); the other residuals in the
tests are typed from the seeded vouchers, not captured ledger rows. A ledger the snapshot lists no residual for is reported as
having no residual row, never as a residual of zero, and an empty voucher read is reported as
such, never corroborated. One captured party shows a later invoice adjusting an advance (above), and the
unadjusted detail lists each advance and note at its amount as allocated and puts Tally's own open
balance for its reference beside it, rather than netting allocations on a rule shown by one party only.

---

### 12a.15 The ledger catalogue can carry each ledger's `ISBILLWISEON`

**PARTIAL: VERIFIED on five synthetic licensed 7.1 Silver books (`BRIDGE OUTSTANDINGS LAB`, `BRIDGE GST RECON LAB`,
`BRIDGE AMEND LAB`, `BRIDGE SHAPE LAB`, `WR2 Unicode Lab`), reads only, 6 Oct 2026; not measured on a large book.** The request is
`StandardLedgerCatalogV2`: the `StandardLedgerCatalogV1` request (`List of Ledgers`, NATIVEMETHOD `Name`, `GUID`,
`Parent`, the two company computes) plus one more line, `<NATIVEMETHOD>IsBillWiseOn</NATIVEMETHOD>`, rendered by
Bridge's own profile code and sent over the gateway by a reads-only sitting. What was observed:

- Every ledger row carried exactly one `<ISBILLWISEON TYPE="Logical">` element, `Yes` or `No`: 17 of 17 and 43 of 43
  rows on the first two books, 19 of 19, 44 of 44 and 9 of 9 on the other three.
- The rows (GUID, name, parent) were the V1 answer's rows, and the flags equalled, by name, the flags of the same book's
  outstandings ledger snapshot (10 `Yes` of 17 and 10 of 43).
- Ledgers under groups where bill-wise tracking is not offered (cash, bank, duties, income, expense, purchase, sales and
  the root) answered `No`, never an empty element: 6 of 17 and 26 of 43 rows.
- Four ledgers whose names end in a carriage return and line feed (three on the amend book, one on the shape book) came
  back with the name as an escaped attribute and a flag: `No` for the three on the amend book, `Yes` for the one on
  the shape book (pinned by a protocol test).
- The same request sent twice answered with byte-identical bodies on the first two books. The answer was about 8 to 9
  percent larger than V1's and took under 0.4 s.
- A flag in a form other than `Yes` or `No` was not seen; Bridge's parser accepts only those two and refuses a row with
  no flag, so a V1 answer is never read as "no ledger keeps bills".

Not shown: a book above a few thousand ledgers (size and time), or a ledger whose flag reads `No` yet holds bills
(company-wide bill-wise settings, §12a.5).

### 12a.16 Cash Flow and its sibling built-in reports by name on licensed 7.1

**PARTIAL: VERIFIED 2026-10-06, licensed TallyPrime 7.1 Silver** (`education_mode=false`), attended, one request at a time. Two synthetic companies, one run each (one repeated request answered byte for byte the same). Nothing here is evidence about a client book. This extends §12a.1 and §12a.11 to six more report names. Every request is §12a.1's envelope with `<ID>` set to the name; every answer was HTTP 200 with `/status` healthy after it, and no dialog appeared at the Tally screen. Fixtures: the `builtin_*` files and `BUILTIN_REPORTS_CAPTURE_PROVENANCE.md`.

- **Cash Flow.** No `HEADER` or `STATUS`. One `DSPPERIOD` (a month name, **with no year**) then one `DSPACCINFO` per month, holding `DSPDRAMT/DSPDRAMTA`, `DSPCRAMT/DSPCRAMTA` and `DSPCLAMT/DSPCLAMTA`. A full year gave 12 rows, 2025-04-01 to 2025-06-30 gave 3 (Tally accepted a day-30 end and answered with those three months, unlike the day 1, 2 and 31 rule of Education mode; whether it computed the figures to the 30th was not separated from month rounding) and one month gave 1. **A month with no activity is a row with every amount empty, not a missing row**, and an empty amount is not zero (§7). A debit is negative, as in the trial balance. The closing figure is the month's own: May's equalled May's debit alone. The credit column was empty in every row of this book, so the sign and meaning of a credit, and a closing figure with both columns present, were unmeasured on this book (the 2026-10-07 captures below measure them).
  - **The tie.** On the book (one cash ledger and one bank ledger, receipts only), the sum of the months' debits equalled the debit totals of the ledgers under Cash-in-Hand and Bank Accounts read by `trial_balance` for the same window, to the paisa (the trial balance responses were not kept, so that side is a recorded reading, not committed bytes), for the year (17,975,981.22), April to June (7,263,013.22) and June (5,472,348.20). **Not measured on that book:** an outflow, a contra, a Bank OD A/c ledger, a month with both (the 2026-10-07 books below measured a Bank OD ledger and months with both columns, and showed a credit column; a contra and the direction of a credit stayed unverified), a window that is not whole months, a several-currency book, optional or post-dated vouchers, a large book.
  - **Three more books, 2026-10-07 (licensed 7.1 Silver, synthetic books, one request each; fixtures `builtin_cash_flow_*` and the provenance file).** VERIFIED on `BRIDGE AMEND LAB` and `BRIDGE SHAPE LAB`: with credits present, **a row's closing is its debit plus its credit** (a debit negative) on every row with an amount: a month of credit only (empty debit), a month with both, a positive closing. **The net total (the sum of the closings) ties to the trial balance's debit plus credit over the ledgers under Cash-in-Hand and Bank Accounts** on AMEND LAB (VERIFIED, one window); on SHAPE LAB it ties only with the ledger under Bank OD A/c added (VERIFIED, below). The trial balance figures are those `trial_balance` returned for the same window at the time (recorded readings; the responses are kept privately, not committed), and the tests type the rows from them. **Bank OD:** `HDFC CC` (group `Bank OD A/c`, a debit of 2,600.00 in the year and in April 2025, no credit) is **counted in Tally's Cash Flow**: without it the two sides differ by exactly its net. VERIFIED for that one ledger, one book, debit only, on whole windows that contain its movement; a Bank OD credit, a Bank OCC A/c ledger and a ledger inside a subgroup of Bank OD A/c were not measured. PARTIAL: the debit column and the credit column each differed from the debit and credit totals of the ledgers Tally counts as cash and bank (on SHAPE LAB including the Bank OD ledger) by one common amount in size (the ledgers' columns larger in both) (AMEND LAB 17,000.00, SHAPE LAB 37,600.00) while the net tied; without the Bank OD ledger the SHAPE LAB columns differ by 35,000.00 and 37,600.00; that a transfer between money ledgers (a contra) is counted in the ledgers' columns and left out of Tally's is the leading reading and is UNVERIFIED. PARTIAL: `BRIDGE CORPUS DENSE` (about 30,000 vouchers) answered the year as 12 rows with every amount empty in 0.1 s, as predicted for a book whose earlier trial balance showed no cash or bank movement; this answer alone cannot tell that from a report that did not compute, and the timing is a no-activity lower bound, not a cost for a large book. The tie rule that a net of two empties compares nothing, and two zero nets are not a tie, rests on these answers and is built in `reports::cash_flow`.
- **Funds Flow** gave the same month-row shape. On the same book Dr is the working capital at the start of the month, Cr at its end, and `CL` = Cr - Dr is the month's change; June's three figures equalled the trial balance's opening, closing and movement of Sundry Debtors, Bank Accounts and Cash-in-Hand. Which groups Tally counts as working capital is not measured.
- **Sales Register** gave the same month-row shape with Dr empty, Cr the month's sales and `CL` the running total. The year's total (the last month's `CL`) equalled the sales ledger's credit in the trial balance; the Sales Accounts group total was 21.00 higher, the ledger that three journals posted to, which the register leaves out.
- **Ratio Analysis** gave 23 `RATIONAME` / `RATIOVALUE` pairs. Money values are **rendered text**: Indian digit grouping and a Dr or Cr suffix (`6,31,22,592.83 Dr`); ratios are text too (`0.00 : 1`, `99.88 %`, `260.75 days`); several values are present and empty. The exact-decimal parser refuses them. A current ratio of `0.00 : 1` was a division by zero (no current liabilities) rather than a zero ratio.
- **Negative Stock** (a book with 11 stock items) listed 5 in the Stock Summary item shape (`DSPACCNAME`, `DSPSTKINFO/DSPSTKCL` with `DSPCLQTY`, `DSPCLRATE`, `DSPCLAMTA`): four items of positive quantity whose closing value was a credit (§12a.13's sign rule) and one of quantity `-50.000 Kgs`. The three items with a debit closing value and the three with no quantity or value were not listed. So the listed set was exactly the items whose closing quantity is below zero or whose closing value is a credit (one book).
  - **Three more answers (VERIFIED 2026-10-21, licensed TallyPrime 7.1 Silver, synthetic books, one request each, HTTP 200; fixtures `builtin_negative_stock_negstock_lab_2_*` and `builtin_negative_stock_tieout_c_*`, and the provenance file).** On a book built for the purpose (inventory on and integrated, unit Nos, a sale of 5 Nos of an item on 10 May, a purchase of 2 Nos of it on 20 June, no opening stock), the whole year listed that one item as one row: closing quantity `-3 Nos`, rate `120.00`, amount `360.00` (positive). Tally's plain Stock Summary for the same window printed the same item as `-3 Nos`, `120.00`, `360.00` (read by the lab, not committed), and printed a second item that was not listed, `10 Nos` at `100.00` with the amount `-1000.00` (a debit, stock held).
  - **The window's end decides the quantity (VERIFIED, one book).** The same request ended 31 May listed the same item at `-5 Nos`, the quantity as at 31 May, with `DSPCLRATE` and `DSPCLAMTA` **present and empty**: an empty rate or value is not zero (§7), and the parser keeps it `Empty`. Whether a window starting after 1 April is measured the same way was not tried.
  - **A book with no short item answers an empty envelope (VERIFIED, one book).** On a synthetic book with stock items and no item below zero, the whole year answered exactly `<ENVELOPE></ENVELOPE>` (46 bytes, the same bytes as an empty Negative Ledgers answer): no rows and no list element. It cannot be told from a report that was not rendered (§12a.11), so the parser refuses it as `EmptyEnvelope` and the `negative_stock` tool reports it as `not_established`, never as "no item is negative".
  - **Not measured:** a batch-wise or multi-godown book, a window crossing a financial year, an item with a positive quantity and a credit value beside a short one on a second book, the cost on a large book, other releases. A company of a similar name exists on the same instance, so a read is bound by the verified identity, never by name alone.
- **Negative Ledgers.** **PARTIAL:** the row shape and the sign rule are VERIFIED on one book; two more books agree with the credit half of the rule (each listed one credit closing), and a negative debit was seen on the first book alone; the rule by which Tally lists a ledger is UNVERIFIED.
  - **An empty answer is inconclusive.** On a book with no ledger whose balance is on the side opposite its group's usual side, the report answered an empty `<ENVELOPE>`. An empty envelope cannot be told from a report that was not rendered (§12a.11). It was recognised as a report (an unknown name is refused, below).
  - **Row shape (VERIFIED 2026-10-09, licensed TallyPrime 7.1 Silver, one synthetic book, a whole financial year, one request at a time).** The success has no `HEADER` or `STATUS`. Directly under the root, each listed ledger is one `DSPACCNAME` that holds the ledger's name in its `DSPDISPNAME`, followed by one `DSPACCINFO` that holds `DSPCLDRAMT`/`DSPCLDRAMTA` and `DSPCLCRAMT`/`DSPCLCRAMTA`. Two ledgers were listed, as a 1,018-byte answer to a 772-byte request.
  - **Sign rule (VERIFIED on that book as to the committed answer; the match with Tally's screen and the trial balance is reported, not kept).** A debit closing is a **negative** number in the debit column and a credit closing a **positive** number in the credit column, the other column empty. A creditor listed with a debit closing read `-2500.50` in `DSPCLDRAMTA`; a debtor listed with a credit closing read `10000.00` in `DSPCLCRAMTA`. The contributor reported that both entries matched Tally's own Negative Ledgers screen and the trial balance of the same window (the contributor's comment of 9 October on the issue for these reports); no screen capture and no trial-balance answer of that book is kept on the capture branch, so that match is reported, not kept. A second request answered the same bytes (reported, not kept). A ledger under Suspense A/c with a credit balance was **not** listed, on the screen and in the answer alike.
  - **Where the bytes are.** The fixtures are the `builtin_negative_ledgers_bridge_lab_win_fy_*` files, described in `BUILTIN_REPORTS_CAPTURE_PROVENANCE.md`; the capture is on the branch `lab/1358-report-captures`, commit `fae62c700fe5ee6d8f1fbbeb7b360d46a08f1a71`.
  - **Two more books (PARTIAL, 2026-10-13, the same release, read by the maintainers on the Windows lab computer; not committed in this tree).** The same report for a whole financial year was read on two further synthetic books, each beside Tally's trial balance of the same year. Each listed one ledger (a credit balance on an expense ledger, a credit balance on a bank ledger), in the same row shape. Each amount equalled that ledger's closing balance in the trial balance, and no other ledger of either trial balance closed on the side opposite its group. The evidence, including each trial-balance answer, is on the branch `lab/issue-reads-2026-10-13`, commit `9c75e2e26f5fb2f5d83c1c0d2193006fcfb4ef8a`, folder `1358/`. One of the two books holds CGST and SGST ledgers (group Duties & Taxes) with credit closings, none listed, and, as the notes of the same commit for another read of it record (folder `693/`), one optional voucher. Neither book's settings, stock items or currencies were read.
  - **A fourth book, eleven ledgers (VERIFIED 2026-10-21 as to the shape, committed bytes `builtin_negative_ledgers_shape_lab_fy_live`; licensed TallyPrime 7.1 Silver, one synthetic book, a whole financial year, one request).** The answer listed 11 ledgers in the same row shape, each with exactly one column filled: a credit closing on a bank ledger and on three debtors (positive `DSPCLCRAMTA`), and debit closings on a capital ledger, a bank overdraft ledger, a loan ledger and four Duties & Taxes ledgers (negative `DSPCLDRAMTA`). So a list of many entries, both sides in one answer and several groups are now seen. The book's trial balance and Tally's screen were not read in that sitting, so neither the side-opposite-the-group reading nor the completeness of the list is checked on it (UNVERIFIED).
  - **Not established (UNVERIFIED).** The rule by which Tally lists a ledger: on three books each listed ledger's balance sat on the side opposite its group, and the Suspense ledger of the first book was not listed, so "opposite to its group" is three data points and not a rule. Not measured: an empty answer told apart from an unrendered report, a list of more than eleven ledgers, the trial balance of the eleven-ledger book, a group other than the ones above, a window that is not a whole financial year, what GST, stock, several currencies, or optional or post-dated vouchers do to the list (one of the two further books holds GST ledgers and one optional voucher, recorded in the branch's notes; nothing was read with and without them, and neither book's settings were read), the cost of the report on a large book, another release.
- **An unknown report name** answered `STATUS` 0 in a `HEADER` and a `LINEERROR` "Could not find Report '<name>'!" in `BODY` / `DATA`, with no dialog and the gateway healthy afterwards. §12a.1 measured a bare `RESPONSE` "Unknown Request" on another build, so the text and the shape of the refusal differ by build; classify by structure (`STATUS` 0 with a `LINEERROR`, or a bare `RESPONSE`), not by text.

### 12a.17 Cost centres and categories are returned whatever the company's setting reads; an imported allocation is stored although it reads No

**PARTIAL: VERIFIED 2026-10-07, licensed TallyPrime 7.1 Silver** (`education_mode=false`), attended, one request at a time. Two synthetic books (`BRIDGE SHAPE LAB`, `BRIDGE CORPUS FOREX`), one run each; the owner read the Cost Centres setting on screen (F11): No on both. A third synthetic book with the setting at Yes was captured the same day in a separate sitting through the same requests (below; that sitting's own status read recorded TallyPrime 7.1, licence Silver, Education mode off, and the status answer is not committed). Nothing here is evidence about a client book. Committed captures: the `masters_cost_*` files and `MASTERS_CAPTURE_PROVENANCE.md`. Readings marked "recorded" below have no committed bytes.

- **The setting is not "no centres".** `CostCentre` on `BRIDGE SHAPE LAB` (setting No) still returned two centres (`Assembly`, `Trading`; `PARENT` the reserved root, `CATEGORY` `Business Line`, `GUID`, `MASTERID`, `ALTERID`, and the computed `BRIDGECOMPANYGUID`) and `CostCategory` two categories (`Business Line`: `ALLOCATEREVENUE` Yes, `ALLOCATENONREVENUE` No, `AFFECTSSTOCK` No; `Primary Cost Category`: Yes, Yes, No). `BRIDGE CORPUS FOREX`, with none defined, answered `STATUS` 1 and one present, empty `COLLECTION` carrying `MSTDEPTYPE` 32. The Company collection (a `$GUID` filter) answered `ISCOSTCENTRESON` No, equal to the screen, on both books (recorded). So a No setting plus an empty list does not say whether the feature is off or no centre is defined, and a No setting alone must not refuse a cost-centre read. The `COLLECTION` element carried `MSTDEPTYPE` 32 on every cost-centre answer and 16 on every cost-category answer; Bridge requires it, so an empty answer that was not resolved to the type is not read as "none".
- **The same shapes with the setting at Yes (a third synthetic book, captured in a separate sitting; its `ISCOSTCENTRESON` read Yes, equal to the owner's screen, as reported at capture).** PARTIAL (committed bytes, one book, captured in a separate sitting, whose own status read recorded TallyPrime 7.1, licence Silver, Education mode off; that status answer is not committed): `CostCentre` returned three centres: two at the top level (`PARENT` the reserved root) and one whose `PARENT` is the name of another centre (a child), every `CATEGORY` the default `Primary Cost Category`, each row with a `LANGUAGENAME.LIST` that holds only the centre's own name (no alias beyond it was seen); `CostCategory` returned the one predefined `Primary Cost Category` (`ALLOCATEREVENUE` Yes, `ALLOCATENONREVENUE` Yes, `AFFECTSSTOCK` No). On these two books the answers carry the same field set and markers as the No book's (apart from `LANGUAGEID` 1033 inside each cost centre's `LANGUAGENAME.LIST` on the Yes book), with one book each. The fixtures are scrubbed (company GUID prefix and the lab names replaced; structure untouched). Not measured: centres nested more than one level, a centre with an alias beyond its own name, a screen-keyed allocation. The predefined `Primary Cost Category` was present in the two books whose categories were captured (one at No, one at Yes); the categories of the book with no centre defined were not captured, so "it always exists" is UNVERIFIED and is only the reason an empty category list is refused.
- **An imported allocation is stored although the setting is No (recorded).** The one-day `vouchers` read of the Journal `SHAPELAB-C9-costcentre` (2025-06-05, written by an import) returned, in Tally's raw voucher export and under one `ALLLEDGERENTRIES.LIST`, `CATEGORYALLOCATIONS.LIST` with `CATEGORY` `Business Line`, `ISDEEMEDPOSITIVE` Yes and two `COSTCENTREALLOCATIONS.LIST` (`NAME` `Trading`, `AMOUNT` -600.00; `NAME` `Assembly`, `AMOUNT` -400.00; empty `ACTUALQTY` and `BILLEDQTY`; `PAYHEADSORTORDER` and `EMPLOYEESORTORDER` 0), adding up to -1000.00, the amount of the ledger entry they sit under. Bridge's own `vouchers` output does not carry these allocations. How a screen-keyed allocation looks was not captured.
- **The Company collection answers the F11 flags (recorded)** `ISACCOUNTINGON, ISINVENTORYON, ISINTEGRATED, ISBILLWISEON, ISALLBILLWISEON, ISCOSTCENTRESON, ISBATCHWISEON, ISPAYROLLON, ISTDSON, ISJOBCOSTINGON, ISTCSON, ISGSTON, ISGSTCLASSIFON, ISEDITLOGON, ISCOSTTRACKINGON, ISISOCURRENCYAPPLICABLE, ISTRACKVOUCHERSON` with `CompanyNumber`, `ALTVCHID`, `ALTMSTID`, `LASTVOUCHERDATE` and `CURRENCYNAME` written as the symbol. Only `ISCOSTCENTRESON` was compared with a screen on those books (the other two settings later, below). `ISISOCURRENCYAPPLICABLE` read No on both of the maintainers' books although `BRIDGE SHAPE LAB` has currency masters: it is not the multi-currency signal.
- **Bank allocations stayed empty (recorded):** every `BANKALLOCATIONS.LIST` container of the books read (2 on `BRIDGE AMEND LAB`, 94 on `BRIDGE SHAPE LAB` in a June window) was empty, so a reconciled entry's wire form is still unseen.
- **Not measured:** a screen-keyed allocation, a reconciled bank entry, a larger book, any release other than this one.

### 12a.18 The Company collection's settings and currency symbol

**PARTIAL: the shape VERIFIED 2026-10-07 on two committed captures; what each setting means is not, except three (compared with the screen 2026-10-09).** Licensed TallyPrime 7.1 Silver, synthetic lab books, attended, one request at a time. Committed captures: the `company_features_*` files and `COMPANY_FEATURES_CAPTURE_PROVENANCE.md`. Nothing here is evidence about a client book.

- **The request and the answer (verified, two committed captures).** A `Company` collection filtered to one GUID (`$GUID = "<GUID>"`; a Company collection ignores `SVCURRENTCOMPANY` and returns every loaded company, §12a.7) fetching `Name, GUID, CompanyNumber, BooksFrom, LastVoucherDate, ALTVCHID, ALTMSTID, CURRENCYNAME` and 17 flags answers one `COMPANY` row under `ENVELOPE/BODY/DATA/COLLECTION`. Each flag is `<IS...ON TYPE="Logical">Yes</...>` or `No`; `CURRENCYNAME` is the currency's symbol, not an ISO code. The `CMPINFO` block carries a bare `<COMPANY>0</COMPANY>` counter, which is not a row. The request ComplyEaze Bridge sends is byte-equal to the captured one.
- **What differs across books (verified from the bytes of the two committed books; the third book's reading is recorded, not committed).** Only three of the 17 flags read differently on different books: `ISCOSTCENTRESON` (No, No, Yes), `ISGSTON` (Yes, Yes, No) and `ISBATCHWISEON` (Yes, No, No). Four read Yes on every book (accounting, inventory, integrated, bill-wise) and ten read No on every book. A flag that has only ever shown one value proves that its element exists, not that it follows its setting.
- **Against the screen (VERIFIED 2026-10-09 for the three settings the tool returns).** `ISCOSTCENTRESON` equalled the F11 screen on the first three books (§12a.17). Then the built tool was run once on each of three new synthetic books (`BRIDGE FEATURES ONE`, `TWO`, `THREE`, books from 2026-04-01, no vouchers, each the only company loaded) by an outside contributor on the maintainers' Windows laptop, TallyPrime 7.1 Silver, with the F11 screen taken before each read and again after it with nothing changed between: cost centres No/Yes/No, GST Yes/No/Yes, batch-wise No/No/Yes on the screen, and the tool's `no/yes/no`, `yes/no/yes`, `no/no/yes` equal on all nine values; each setting read Yes on at least one book and No on another. Each answer's `company` equalled the loaded company in name, GUID, company number and books-from date. The 14 other flags were not compared (the tool does not return them). One book (ONE) had batches off though meant to have them on; its setting was left as the screen showed it.
- **A setting read two ways in one sitting (recorded, PARTIAL).** On the third book `ISGSTON` read Yes and, about three and a half minutes later, No, and No in every later read of that sitting; that fits a setting changed on screen during the lab setup and does not show it.
- **The currency symbol.** `CURRENCYNAME` carried the rupee sign on all three books. A user can set any symbol, so it is not mapped to a currency code. `ISISOCURRENCYAPPLICABLE` read No on every book, including one that has currency masters: it is not the multi-currency signal (§12a.17).
- **What ComplyEaze Bridge does with it.** `company_features` returns the cost-centre, GST and batch-wise settings, each as yes, no or not_reported (an element Tally did not send is never read as no; an empty or odd one refuses the read), each saying what it rests on, and the symbol. The row is bound to the verified company by its GUID, name, company number and books-from date. Nothing it returns changes what another tool reads or refuses; `stock_summary` reads its own inventory, integrated and batch-wise flags from its own request. All three settings equalled the screen on three synthetic books, so each carries `compared_with_tally_screen_on_synthetic_books`.
- **Not measured:** the 14 flags the tool does not return against the screen; a setting changed inside a sitting (the GST reading above is one such case, unexplained); books with vouchers; macOS; a setting changed one at a time; Education mode; any release other than 7.1; a book with companies that share a GUID; a client book.

### 12a.19 A company's GST registration is a tax unit, with dated rows

**PARTIAL: VERIFIED 2026-10-09, licensed TallyPrime 7.1 Silver**, one synthetic company (`BRIDGE PILOT LAB`) with one GST registration, read by an outside contributor. The `TaxUnit` answer described in the second bullet was read after the registration was entered and before any voucher was keyed, to the code's collection typed by hand from the text in #1342's request list and sent as UTF-8, where ComplyEaze Bridge sends UTF-16 (that answer is not kept in this tree; `pilot-lab/PROVENANCE.md` says why). Each other read is placed in the lab sitting by the bullet that uses it.

- **The company's own row holds no registration (VERIFIED, two captures).** A `Company` collection with `NATIVEMETHOD *`, read before and after the registration was entered, differed only in the `CMPINFO` counter `TAXUNIT` (0, then 1); `GSTREGISTRATIONNUMBER` and `GSTREGISTRATIONTYPE` were empty both times. GST was already on (`ISGSTON` Yes) in the first capture, so a company with GST on and no registration was read, but not one with GST off. This agrees with §12a.7's field-listed read.
- **A `TaxUnit` collection returns the registration (VERIFIED).** With `NATIVEMETHOD *` it returned two rows: `Default Tax Unit` (attribute `TAXTYPE=""`, `ISPRIMARYUNIT` Yes, no registration) and the registration, `Rajasthan Registration` (attributes `TAXTYPE="GST"` and `TAXREGISTRATION` holding the GSTIN; `USEDFOR` GST; `GSTREGNUMBER` the GSTIN). Each row's `GUID` was the company's GUID followed by `-` and a suffix (PARTIAL: two rows of one company). The registration's history is a list of dated rows, `GSTREGISTRATIONDETAILS.LIST`, each with `FROMDATE`, `STATE`, `PLACEOFSUPPLY`, `REGISTRATIONTYPE` (`Regular`) and `ISINACTIVE` (`No`). The unit-level `GSTREGISTRATIONTYPE`, `STATENAME` and dates are empty on the registered unit.
- **The `CMPINFO` counter `TAXUNIT` is not a count of units.** It read 1 beside two rows in this answer, and 14, then 22, in reads of the same book after vouchers were keyed; the two tax-unit answers read after ten vouchers were keyed (`pilot-lab-tax-units` and `pilot-lab-tax-units-fields`) read 0 beside the same two rows. What moved it is not known.
- **What ComplyEaze Bridge does (ADR 0004, slice 1).** The Sales build reads the tax units and refuses an invoice unless exactly one GST unit is in force on the invoice date: the unit's latest dated row on or before that date is not inactive, says `Regular`, and names a state that the unit's GSTIN, valid and equal to `TAXREGISTRATION`, carries. A unit with an earlier GSTIN recorded (`GSTOLDREGNUMBER`, empty here; that a change fills it is assumed from its name) is refused, because the GSTIN is held once per unit and is not dated. A row whose GUID is not the company's refuses the answer, and an answer with no unit at all is a failed read. The registration's state replaces the company's `STATENAME` as the supplier's state.
- **The code's own request, answered (VERIFIED, 9 Oct 2026, after ten vouchers were keyed and before the invoice was imported).** Sent in UTF-16 as ComplyEaze Bridge sends it, the request returned the same two rows and the same registration, and the build admitted an invoice on it. A narrower request that fetches only `GUID`, `TAXTYPE`, `TAXREGISTRATION`, `USEDFOR`, `GSTREGNUMBER`, `GSTOLDREGNUMBER` and the registration's dated rows returned the same values for those fields. Both answers are in the `pilot-lab` fixtures.
- **Not measured:** a company with GST off (the company's GST flag is not read), or with no registration; how Tally writes an inactive or composition registration, or a changed GSTIN; two loaded companies (whether `TaxUnit`, like `Company`, ignores `SVCURRENTCOMPANY`); several registrations; another company's units; a voucher type bound to a tax unit (`TAXUNITNAME`, `Any` on the lab's Sales type); Gold, Education, other releases; a real book (not a lab book).

### 12a.20 A ledger's input tax credit flag on the wire is the opposite of the screen's setting (PARTIAL)

**PARTIAL: one ledger (under Purchase Accounts), one dated entry, one release.** Measured on 15 October 2026 on TallyPrime 7.1 in one synthetic company (`BRIDGE SHAPE LAB`) by a keying sitting of `lab/itc-ledger-write-run-sheet` (curl only, no ComplyEaze Bridge build; one backup, one ledger created, two alterations, each followed by a read). The element is `GSTINELIGIBLEITC`, inside a ledger's `GSTDETAILS.LIST` entry (read as the committed capture `ledger_gst_details_live` was read: the ledger collection with its GST details).

- **A keyed Yes reads as the element absent; a keyed No reads as `Yes` (PARTIAL).** A new ledger under Purchase Accounts was given one GST history row dated 20250401 (taxability `Taxable`, source `Specify Details Here`, IGST 18). With the screen setting `Eligible for Input Tax Credit` keyed Yes, the ledger read back with no `GSTINELIGIBLEITC` in the entry (answer of 105,844 bytes, sha256 `b4b2641bc7ae81b88c23afa50775b51fe09746fc433d46b3d5ff5c7d2a9ffa72`). Changing only that setting to No, the next read carried `GSTINELIGIBLEITC` `Yes` in the same entry and nothing else moved (105,970 bytes, sha256 `5d531467b4c5714bfcf04b961f0dda2138c32297b3f424e55db954699baf61e7`). The screen label was exactly `Eligible for Input Tax Credit` in both photographs. The book's master mark rose by one for each change (290, 291, 292). So on this ledger the element is an ineligibility flag: Yes on the wire means the screen says No. Nothing here says the rule holds for any other ledger.
- **The same reading was seen earlier on a stock item (PARTIAL: one item, two rows).** A change that adds stock items to `masters` (§12a.22) reads the element the same way and names it `ineligible_itc` too.
- **A Yes elsewhere says nothing either way (VERIFIED in the committed capture `ledger_gst_details_live`, not read against the screen).** The three Sales Accounts ledgers of that capture carry `GSTINELIGIBLEITC` `Yes` on their keyed entries (source `Specify Details Here`); the one Purchase Accounts ledger carries none; a ledger entry whose source is `As per Company/Group` (Tally's own default entry) carries `Yes` with no keyed No behind it. None of these was compared with its screen, so reading the Sales ledgers as ineligible, or a default entry's Yes as a choice, is not supported.
- **What ComplyEaze Bridge does.** `ledger_masters` with `include_gst_details` sends the element as `ineligible_itc` (true for Yes, false for No, null when absent, never defaulted) and says in its text and in the result's `limitations` that true is not eligible. 0.6.0 sent it as `itc_eligible`, which read the opposite way (#1741).
- **Not measured:** any other ledger or group (a Sales ledger, a duty ledger, a party ledger), a second dated entry on one ledger, an entry keyed new beside one Tally made itself, what an absent element means for the tax (eligible, or deferring to the company or group), the screen of Tally's own default entry, Gold, other releases.

### 12a.21 Tally's own Profit and Loss carries closing stock as a line of its own on an integrated inventory book

**PARTIAL: VERIFIED 2026-10-21 on one synthetic book, licensed TallyPrime 7.1 Silver**, read-only, the company loaded alone (captures `statement_*_stock_book_*`, provenance in `STATEMENT_CAPTURE_PROVENANCE.md`; lab branch `lab/692-tieout-b-answers` at `20ef79c4`). The book has inventory integrated with accounts (`ISINTEGRATED` and `ISINVENTORYON` both `Yes`), three stock items, one purchase of 3,000 and one sale of 1,400, freight of 500, and no opening stock.

- **The Profit and Loss export carries the figure as a leaf (verified).** In order: `Sales Accounts` main `1400.00`; `Cost of Sales :` main `-900.00`; `Opening Stock` empty; `Add: Purchase Accounts` sub `-3000.00`; `Less: Closing Stock` sub `-2100.00` (the fifth `PLAMT`, sub column, main column empty); `Indirect Expenses` main `-500.00`. There is no Gross Profit leaf: 500 is `1400.00` plus the heading `-900.00`. Closing stock has the sign of the purchase line (a debit is negative), and the heading is the purchase line less it: `-900.00` is `-3000.00` less `-2100.00`. On a book without stock the purchase line is named `Purchase Accounts`.
- **The Balance Sheet export has no stock leaf (verified).** `Capital Account` `50000.00`, `Current Liabilities` `3000.00`, `Profit & Loss A/c` empty, `Current Assets` `-53000.00`: the stock sits inside Current Assets. Derived from the Trial Balance of the same window, Current Assets are `-50900.00`, short by exactly the closing stock, and the derived Profit & Loss A/c line is `-2100.00` (a loss) where Tally's is empty.
- **The Trial Balance carries no closing stock (verified).** Its Current Assets are the bank, the debtor and cash only. The Trial Balance and group answers carry raw `U+0003` characters (82, in `PARENTSTRUCTURE`) and `&#4;` character references (30, and one in the Trial Balance); the crate's parsers accept both.
- **What Bridge does with it.** `profit_and_loss` and `balance_sheet` both read Tally's Profit and Loss. Only when that figure alone closes Current Assets, the carried Profit & Loss A/c line and the `Cost of Sales :` heading, with the company's flags both `Yes`, `Opening Stock` shown empty, no Stock-in-Hand ledger carrying an amount and the derived Balance Sheet lines summing to zero (a Trial Balance that does not balance would leave a `Difference in opening balances` out of this book's export), are the results established with it, and it is returned as `closing_stock`, a line naming Tally as its source. Anything else keeps today's refusal.
- **Two books with opening stock, read on 21 October 2026 (PARTIAL: read-only, synthetic, licensed 7.1 Silver; answers kept on the lab branches `lab/692-tieout-c-answers`, `lab/692-opening-stock-c-answers` and `lab/692-opening-stock-d-answers`, which are not merged into master; the Profit and Loss answers of the second are committed as `statement_profit_and_loss_opening_stock_book_*`).** The first (`BRIDGE TIEOUT C`) holds an item with opening stock 10 at 100, purchases at 120 and 140 and a sale of 8, costing shown as average: its Profit and Loss carried `Opening Stock` `-1000.00` and `Less: Closing Stock` `-2640.00`, equal to the Stock Summary (first-in first-out would give 2,800 by arithmetic; not read). Its ledger collection balances (the ledger closings sum to 0) and carries no stock, so the ledgers' Current Assets fall short of Tally's by the closing stock alone (2,640), while the Profit and Loss and its carried line fall short by closing minus opening (1,640): no single figure closes both. Its Balance Sheet export has five lines and **no `Difference in opening balances` leaf**: the year's answer is committed on `lab/692-tieout-c-answers`, and the July to September and two-year answers (same five lines, same bytes as the year's) are on `lab/tonight-read-answers-2026-10-21`; that Tally's screen shows such a difference of 1,000.00 is the lab's report in that branch's manifest, with screenshots kept on the lab computer and not committed (UNVERIFIED here). The export does not balance by the opening stock. The second book (`BRIDGE TIEOUT D`) is the first plus a capital ledger of 1,000.00 Cr, so that the screen shows no difference line and the sheet balances: in all four windows read (year, July to September, year after a further sale, October to March) the ledgers' Current Assets fell short by exactly the closing stock, the nett result and the carried line by exactly closing minus opening, and the ledger closings summed to exactly minus the window's opening stock. A non-empty `Opening Stock` line declines the figure here; a rule that used both lines would need its own ruling and a book that is not hand-keyed from one item (UNVERIFIED on a real book). The Balance Sheet export is the position at the window's end, so it is the same bytes for any window that ends on the same day.
- **A Balance Sheet export that does not balance (verified once; PARTIAL: BRIDGE TIEOUT D, TallyPrime 7.2 Silver, 27 Oct 2026, read-only; captures `statement_*_opening_difference_*`, provenance in `STATEMENT_CAPTURE_PROVENANCE.md`; lab branch `lab/692-opening-stock-d3-answers` at `56f293fd7`).** After one stock item's opening value was raised by 100.00 (experiment X5), Tally's Balance Sheet screen shows a line `Difference in opening balances` of 100.00 and totals of 4,946.67 on both sides. The export has no line for it, under that name or another: `Capital Account` `1000.00`, `Loans (Liability)` empty, `Current Liabilities` `2600.00`, `Profit & Loss A/c` `1246.67`, `Current Assets` `-4946.67`, which add to `-100.00` (liabilities 4,846.67, assets 4,946.67). The five other states of the same sitting (the unchanged start and four other changes) balance. Bridge sums the export's main amounts and, when they differ, reports both totals and establishes nothing (#2063). Not measured: a book without stock whose ledger openings do not agree, another release.
- **Not measured:** opening stock as an accepted case, a costing method, a book that is not integrated, a window spanning two financial years or a part year after earlier movement, more than one item, purchase rate or godown, Direct Expenses or Direct Incomes beside stock, negative stock, a renamed Purchase Accounts group, another language, any release other than this one.

### 12a.22 Stock items carry their own dated HSN and GST lists in a masters read; whether a group's or the company's setting reaches them is not read

**PARTIAL: VERIFIED 2026-10-14 on three committed captures of one synthetic book, licensed TallyPrime 7.1 Silver on Windows** (`education_mode=false`), attended, one request at a time, the sitting restored from a backup at its end. Nothing here is evidence about a client book or another release. Fixtures `masters_stock_items_shape_lab_*_live.utf16le.xml` and their request, with sizes and hashes, in `crates/bridge-tally-protocol/tests/fixtures/MASTERS_CAPTURE_PROVENANCE.md`. Counts were taken by parsing the XML, not by searching the text.

- **The request (verified).** A `StockItem` collection (`ISMODIFY="No"`, no dates) fetching `NAME, PARENT, GUID, MASTERID, ALTERID, HSNDETAILS.LIST, GSTDETAILS.LIST`, with the `BRIDGECOMPANYGUID` compute the other masters collections send. The answer's `COLLECTION` carries `ISMSTDEPTYPE="Yes"` and `MSTDEPTYPE="512"`; every row carried the computed company GUID. Bridge requires the 512 as it requires the types of cost centres and categories, so that an empty collection that was not resolved to the type is not read as "no stock items". That requirement is a choice, not a measurement: no zero-row stock-item answer was captured.
- **List shapes (verified).** Each dated entry is one list element directly under the row, in the order every `GSTDETAILS.LIST`, then every `HSNDETAILS.LIST`, then `LANGUAGENAME.LIST`. An `HSNDETAILS.LIST` entry holds `APPLICABLEFROM`, `HSNCODE`, `HSN` (a description) and `SRCOFHSNDETAILS`. A `GSTDETAILS.LIST` entry is the shape of a ledger's (`ledger_gst_details_live`): `APPLICABLEFROM`, `TAXABILITY`, `SRCOFGSTDETAILS`, `GSTINELIGIBLEITC`, then `STATEWISEDETAILS.LIST` with `STATENAME`, four or five `RATEDETAILS.LIST` (CGST, SGST/UTGST, IGST and Cess, and State Cess on some state rows only) and a `GSTSLABRATES.LIST`. An item with no entry carries one whitespace-only element of each list, not an absent list.
- **Values (verified).** `GSTRATE` is text with a leading space (` 9`, ` 18`, ` 2.50`) and is absent on State Cess and, for a Cess head, the valuation type is `Not Applicable`. `STATENAME` (`Any`) and the Cess valuation type arrive with the reserved character `&#4;`, which Bridge's sanitiser writes as U+FFFD `#4;`. The HSN description is filled by Tally from its own list (`Disodium Carbonate, Dense` for 28362010, `Other` for 28362090).
- **Tally's own rows on the first save (verified).** Saving a new item added one HSN and one GST entry of Tally's own, source `As per Company/Stock Group`, with no code and no rate, dated 1 April 2025 (the book's first day; **PARTIAL: one book, the date on any other book is not measured**, so recognise the entry by its source, not by that date). The GST entry carried `GSTINELIGIBLEITC` Yes and no `GSTRATE` on any head. Keying the first HSN or GST row edited that row; no own row was added by a later save in this sitting. Such an entry says the item defers to its group or the company.
- **A second book shows Tally's own entries on all three of its items (VERIFIED 2026-10-21, committed capture `masters_stock_items_tieout_b_live`; synthetic book, licensed 7.1 Silver).** Each item carried one HSN and one GST entry of Tally's own, dated 1 April 2025, source `As per Company/Stock Group`, no code, no taxability and no rate on any head, `GSTINELIGIBLEITC` Yes, a state row `Any` and an empty slab-rate list element (a placeholder). It confirms the first-save shape on another book; the date and the Yes are still not compared with a screen on it.
- **Two dated entries (verified, one item).** After two HSN rows (from 1 April 2025 and from 22 September 2025) and two GST rows (IGST 18, then IGST 5) were keyed, the answer carried both of each. `GSTINELIGIBLEITC` was Yes on the first GST entry and absent from the second. On the screen `Eligible for Input Tax Credit` read No on the first row and Yes on the second. The element's name says ineligible, and both rows fit it as an ineligibility flag: Yes where the screen says not eligible, omitted where it says eligible. **PARTIAL: one item, two rows, screen against wire.** The screen readings are in `lab/hsn-masters-answers/NOTES.md` ("Other observations") and `lab/hsn-masters-answers/check-lines.txt` (steps 9 and 10), commit `46dc0a98246ac399e294cadb7c4b43fe64e1506d` of this repository; the answers themselves are the committed fixtures. **The two points do not separate the readings.** The first GST entry was Tally's own first-save row, edited; the second was keyed new, so "Yes means ineligible" and "the flag follows where the row came from" fit both rows alike; and the flag on the unedited own row (Yes in the first-save capture) was never compared with the screen. What an absent element means for the tax is not known. Bridge returns the value as sent under the key `ineligible_itc` for a stock item (`true`, `false` or `null`, never defaulted), not as an eligibility, except where Tally sent a flag on an entry whose source is `As per Company/Stock Group` (the entry Tally adds on a first save; a person can also choose that source), where it sends `null` with `ineligible_itc_withheld: tally_default_entry`; a flag Tally did not send stays `null` with no marker, and an entry with any other source is sent as it is; `ledger_masters` names the same element the same way (§12a.20).
- **A code keyed on a group was not on the group's four other items (PARTIAL: one book, one release).** After HSN 2836 was keyed on the stock group `Raw Chemicals`, the item answer 36 seconds later still carried the empty HSN placeholder on each of the group's four other items (the fifth, `Soda Ash Light`, kept its own code) (`lab/seeding-answers/answers/ans-s06b.xml` (the group after the code), `lab/seeding-answers/answers/ans-s05c.xml` (the items 36 seconds later) and `lab/seeding-answers/send-log.txt`, commit `8a0c9e55ce249c93c6b35add00e5aa220c7676c6` of this repository). An empty item-level list is therefore not "the item has no HSN or GST", and Bridge does not read the group's or the company's setting. An item that defers to a coded group was not observed (the one item carrying an `As per Company/Stock Group` entry was created on the restored book, where no group was coded), so whether such an item inherits the group's code is not known: the `masters` rows say so (`inherited_from_group_or_company: not_read`).
- **Size (verified for the capture, assumed beyond it).** In UTF-16 units from the opening to the closing tag, line ends kept: 607 to 635 for an item with empty lists, 1,936 for the item with one GST entry, 2,238 for the first-save item, 4,373 for the item with two entries in each list (the GST entries 1,627 and 1,569, the HSN entries 287 and 267). Bridge refuses a row past 16,344 units (10,200 plus the name allowances), about seven further GST entries of the measured size; a non-`Any` state row is read and is refused only through that bound. The same row size admits master marks up to 489, so the kind is refused before the request on a book above that mark, and a limit of 489 refuses, by arithmetic, both client books measured (marks of about 100,000 and 300,000; stock items were not tried on them).
- **What Bridge does with it.** The `stock_items` kind of `masters` returns each item's two lists as observations (entries sorted by date, `unreadable` with a defect, or `not_observed`), choosing no code and no rate for a date. A Tally `ERROR` or `LINEERROR` at any depth of a list, markup inside a value and a cut-off list refuse the read; an undated or invalid-dated entry, a repeated field, two different entries on one date, a rate that is not a plain decimal, a credit flag other than Yes or No and a repeated state or duty-head row fail that item's list only.
- **Not measured:** a state row other than `Any`, slab rates, a GST rate keyed on a stock group or on the company, an HSN code keyed on the company, an item that defers to a coded group (whether it inherits the code), a nested group, another release, a larger book, a zero-row stock-item answer, a client book.

### 12a.23 Stock Query by item name gives an item's closing quantity per godown and batch

**PARTIAL: VERIFIED 2026-10-28 on one synthetic book, TallyPrime 7.2**, one request at a time, reads only, the marks unchanged before and after. Fixtures and their sizes and hashes: `crates/bridge-tally-protocol/tests/fixtures/stock-query/PROVENANCE.md`. Counts were taken by parsing the XML, not by searching the text.

- **The request (verified).** The built-in report envelope the Stock Summary is sent in (§12a.1), with `ID` `Stock Query` and one more static variable after the period's, `STOCKITEMNAME`, the item's name escaped once as XML text. Without it, or with a name Tally does not know, the answer is the 46-byte empty envelope (`<ENVELOPE></ENVELOPE>`); both are fixtures. With `SVSTOCKITEM` in its place, or with an unknown variable, the answer was the same envelope at the first sitting (`lab/2018-stock-query-with-item-answers` at `263cc72c`); those captures are not among the fixtures.
- **The answer (verified).** A flat `ENVELOPE`. First, zero or more last-purchase groups of six elements in this order: `STQPURCDATE`, `STQPURCPARTY`, `STQPURCVCHQTY`, `STQPURCVCHRATE`, `STQPURCVCHDISC`, `STQPURCVCHAMOUNT`, or last-sale groups: `STQSALESDATE`, `STQSALESVCHNO` (which holds the party), `STQSALESVCHQTY`, `STQSALESVCHRATE`, `STQSALESVCHDISC`, `STQSALESVCHAMOUNT`. No capture holds both kinds, so their order relative to each other is not measured, and Bridge reads them in either order. Then one `STQGODOWNNAME`, `STQBATCHNAME`, `STQGODOWNQTY` triple per godown and batch holding stock at the period end.
  - One item carried five purchase groups (`r-09`, five purchases of 100 Kgs). They do not account for its movement in the year, from 300 Kgs opening to 1300 Kgs closing in the stock-item fixture below, so whether Tally caps their number, or the rest came from other vouchers, is not measured.
- **The lines (PARTIAL: six items' answers are fixtures; the others are lab captures named here).**
  - They follow the period end: at the opening day only the opening quantities came, and no group. Fixtures: `r-12` against `r-4` (Hydrochloric Acid 33pc) and `r-14` against `r-03` (Cleaning Kit A).
  - At the year end, the lines of the six fixture answers that hold lines (`r-01`, `r-02`, `r-03`, `r-4`, `r-09`, `r-11`) sum to the same items' `CLOSINGBALANCE` in `stock_items_shape_lab_fy_live` (§12a.13): 100 Box, 400 Kgs, 12 Nos, 816.667 Kgs, 1300 Kgs and -50 Kgs. That fixture was taken on 7.1 at another sitting of the same book; the collection read again on 7.2 at the year end equalled it in every leaf value (`lab/2018-stock-query-names-units-dates-answers` at `86ea5211`).
  - At the sitting (`lab/2018-stock-query-items-periods-answers` at `a5b369bf`), eight of the book's eleven items answered lines, and each one's lines equalled the godown and batch details on Tally's own Stock Query screen for that item and summed to the closing that screen showed. Two of the eight answers (HDPE Drum 50L, Sulphuric Acid 98pc) are not fixtures. The other three items answered the empty envelope (below).
  - At 1 April and 1 May 2025, five items' lines equalled the stock-item collection's `CLOSINGBALANCE` (§12a.13) at the same date, except that at 1 April Zero Stock Item answered the empty envelope beside a closing of `0.000 Kgs` (`86ea5211`). None of those answers is among the fixtures.
  - Stock not kept in a named batch is the batch `Primary Batch`.
  - A quantity is `<number> <unit>` in the item's base unit, with the item's own decimals (`150.000 Kgs`, `25 Nos`, `100 Box`), signed as stock is (`-50.000 Kgs`, `-13 Nos`).
  - An item with no stock at the period end answered the same empty envelope as an unknown name (`a5b369bf`): Zero Stock Item at the opening day, and three items at the year end (Cleaning Kit B, Descaler Concentrate, Label Roll). Tally's Stock Query screen showed 0 for the first three; Label Roll's screen was not opened. The stock-item collection's closing for the three year-end items is empty, not zero, in the fixture above and at all three dates of `86ea5211`. None of these empty answers is among the fixtures, whose empty answers are the unknown name and the bare request. So the empty answer is not yet tied to zero stock by any fixture.
- **No value.** No line carries a value, rate, expiry or manufacturing date; the only amounts are the groups' voucher amounts.
- **Names (verified, a later sitting, captures not yet fixtures).** Names with `&`, `(`, `)`, `'`, `"`, `%`, `/`, `,` and `-` were selected when escaped once, and the raw and entity forms of `'` and `"` answered alike (`lab/2018-stock-query-names-units-dates-answers` at `86ea5211`). An item with an alternate unit is written in its base unit only (`lab/2018-alternate-unit-row-answers` at `88616807`). Bridge's builder admits letters, digits and single spaces only, until those captures are fixtures.
- **What Bridge does with it.** `native_stock_query` renders the request for one item and reads the answer into its lines, skipping the groups. Among the shapes no capture holds, it refuses an element in another case or with an attribute, a comment or declaration, a kind of group coming back after the other, a godown and batch read twice, a name or quantity holding the sanitiser's marker (§1.1(d)), a character in the closed set `unsafe_display_character` refuses (every control character, `U+0000` to `U+001F` and `U+007F` to `U+009F`, and `U+061C`, `U+200B`, `U+200E`, `U+200F`, `U+202A` to `U+202E`, `U+2060`, `U+2066` to `U+206F` and `U+FEFF`; it admits `U+200C` and `U+200D`, which Indic spelling needs), or padded at either end, a group with no line after it, and lines in more than one unit. A padded quantity is refused rather than trimmed as the stock read's grammar trims it, because no captured Stock Query answer pads one. It reads others that no capture holds, without refusing them: an empty group element, both kinds of group once each, and a zero quantity. No read sends it yet.
- **Not measured:** another release; a book that is not batch-wise; a period starting after the books' first day (every request sent `SVFROMDATE` 1 April 2025, the books' first day, and `render_stock_query_request` accepts any period, so whether the lines are then the closing at the period end or the movement within the period is open); among the fixtures, a period end other than the year end and the opening day (1 May 2025 is lab only); more than five purchase groups; a full stop or leading, trailing or doubled spaces in a name; a value for a batch; a godown under another godown (both captured godowns sit directly under the root), so whether a parent's line includes its children's; Tally's failure answer to a Stock Query, none captured (a failure element below the envelope's first level is read as malformed, not as Tally's failure); a godown name holding a tab (the parser refuses it as a control character, and no capture holds one, so whether Tally ever writes one is open); and which company or item an answer is for, since it echoes neither.

### 12a.24 The Godown Summary gives godown totals, and the exploded Stock Summary values a root item per godown at that godown's own cost

**PARTIAL: VERIFIED 2026-10-28 and 2026-10-30 on one synthetic book, licensed TallyPrime 7.2 Silver on Windows** (`education_mode=false`), attended, one request at a time, two sittings. Every request was the committed Stock Summary request (§12a.13) with only the report ID, the period end or the one variable `<EXPLODEFLAG>Yes</EXPLODEFLAG>` changed. Fixtures and exact requests: `tests/fixtures/BUILTIN_REPORTS_CAPTURE_PROVENANCE.md` (`builtin_godown_summary_*`, `builtin_godown_closing_*`).

- **The Godown Summary by name (verified).** `<ID>Godown Summary</ID>`, the title of Tally's own godown-wise screen, answered in band with the Stock Summary's grammar: one `DSPACCNAME` / `DSPSTKINFO` pair per godown directly under the root, and no line for a stock group or an item.
  - A godown's `DSPCLQTY` is the net of every item in it that shares a unit, never one item's closing: `316.667 Kgs` was one item's `366.667 Kgs` and another's `-50.000 Kgs`. It was empty for a godown holding items in several units; the cause of an empty quantity is not measured, because the Stock Summary's line for a group whose items were all in `Kgs` was empty too.
  - The godown amounts summed exactly to the plain Stock Summary's total for the same period at every date read: `3000.01` on the unchanged book, and `2300.01` and `2292.01` at 31 August 2025 and 31 March 2026 after the keying below. The screen showed the same lines with the sign reversed, as the Stock Summary's does.
  - A godown keyed under `Main Location` had **no line of its own**: `Main Location`'s amount held the child's. Tally's Godown Summary screen in its Detailed format, its drill-down into `Main Location`, and the report with the explode flag each showed the child as its own line under its parent; the plain (Condensed) screen did not.
  - With the explode flag, the answer stayed flat and each godown was followed by its stock groups, its items directly under the root and its child godowns, one level, all in the same pair grammar. Nothing but a name tells a group, an item and a child godown apart there, so this shape is not read. These answers (`answers/b-04` and `b-10`, and `a-03` before the keying) are kept on the lab branch only, not committed.
  - Drilling into a godown or a group on the screen kept the title `Godown Summary`.
- **The Stock Summary with the explode flag on 7.2 (verified).** It answered as on 7.1 (§12a.13): flat, one level expanded. After two items were keyed directly under the root, each was followed by one `SSBATCHNAME` (`SSBATCH` empty, `SSGODOWN` the godown) and a `DSPSTKINFO` per godown holding stock, the child godown by its own name. An item inside a stock group had no such line in the answer, although Tally's Stock Summary screen, Detailed with every level expanded, showed such an item's godown and batch lines with their values. Before the keying, with every item in a group, no `SSBATCHNAME` appeared at all (`answers/a-02` and `a-07`, kept on the lab branch only).
- **Each godown is valued at its own cost (verified, one item).** The split item was keyed with 30 Nos at 10.00 in `Main Location` and 20 Nos at 12.50 in the child godown, then 8 Nos were moved to `Factory Floor` by a Stock Journal dated 15 September 2025 at 11.00. At 31 March 2026 the exploded report gave `Main Location` 22 Nos at 10.00, `-220.00`: not the 245.52 that the item's 31 March rate of 11.16 would give, nor the 242.00 at its 11.00 before the transfer; `Factory Floor` 8 Nos at 11.00, `-88.00`; and the child godown 20 Nos at 12.50, `-250.00`. These add up to the item's own line, 50 Nos at 11.16, `-558.00`, which equalled the item's value in Tally's Stock Summary screen and in `stock_summary`. On the voucher screen Tally offered the source godown's own cost, 10.00, as the transfer's outward rate.
- **Stock Query (§12a.23) on the same items** gave the same quantities per godown, with no value.
- **What Bridge does with it.** `stock_by_godown` reads the plain Godown Summary for godown totals and returns them only when they tie to the plain Stock Summary; it reads a value per item per godown only from the exploded Stock Summary's `SSGODOWN` lines, only for an item whose godown values and quantities add up to its own line, and only where the plain report shows that line with the same value. The exploded Godown Summary is never read.
- **Not measured:** a batch-wise item under the root (every `SSBATCH` was empty), a value per godown for an item inside a stock group over the gateway, a costing method other than the average cost these items used, a compound unit, a godown two levels down, a godown with no stock, an item whose godown values do not add up, another book or release, and TallyPrime 7.1 for the Godown Summary.

## 13. Open questions

| Question | Why it matters |
| --- | --- |
| Does editing a voucher populate `AUDITENTRIES.LIST`? | Could replace AlterID diffing entirely on the Edit Log SKU |
| Is there a syntax for two-level `FETCH`? | Decides the 6.3× GST payload penalty |
| Why does `ClosingBalance` render empty at some dates? | Blocks trusting any balance field |
| Which report attribute makes rendering work? | Only matters if custom reports are retained |
| Behaviour on standard TallyPrime and Tally.ERP 9 | Every finding here is single-SKU |
| Behaviour with third-party TDL installed | The demo instance has none; client machines will. Could alter the report surface §12a.1 depends on |
| Is the crash observed on 2026-08-02 volume-driven or UI-driven? | It did not reproduce on an identical repeat; a UI keypress during generation is at least as likely. Recorded privately |
| What is the correct key for `ACTION="Alter"` on a voucher? | On automatically numbered types observed, `GUID`, `REMOTEID`, `MASTERID`, and the `REMOTEID`/`MASTERID` combination have each created duplicates (§9.7, §12a.4). Manual + `PREVENTDUPLICATES=Yes` instead rejects the failed Alter (§9.8); other request shapes and licensed-SKU behavior remain untested |
| Does the `On Account` residual identity hold on a bill-dominated book? | Verified on a book that is 98% unallocated; the opposite composition is the case where a false zero would look like success |
| What do the `CMPINFO` counters report, and why can one capture return all zeros? | Two catalogue captures of the same company minutes apart returned `LEDGER` 0 then 61, with `CURRENCY`, `TAXUNIT`, `VOUCHERNUMBERSERIES` and `VOUCHER` likewise 0 then populated. Until this is explained, no two responses can be compared by hash or byte equality (§12a.9) |

---

## 14. Changelog

| Date | Change |
| --- | --- |
| 2026-07-29 | Created. All VERIFIED entries established this day against TallyPrime Edit Log 7.0 EDU. |
| 2026-07-30 | Recorded the outstandings-only wildcard exception, curated bill-type corruption, and the contextual polarity finding. |
| 2026-08-02 | Added §12a from a live measurement session: built-in named reports (qualifying §2.2), per-kind ageing semantics, the two ageing methods, eight import rewrites (extending §9), configuration as a non-diagnostic, the unallocated remainder and its recovery, the `Company` collection ignoring `SVCURRENTCOMPANY` (qualifying §9.11), and a linear volume model with a cheap pre-flight count. |
| 2026-08-22 | Updated §5.3 with the observed Education `{1,2,31}` boundary rule and the limited TallyPrime Silver arbitrary-day observations; this settles #115 item 1 for the recorded profile. |
| 2026-08-28 | Added §8.1's read-only ledger-master field-presence observation and explicit public-fixture privacy boundary. |
| 2026-09-10 | Added §9.13's Payment/Receipt/Contra import shapes from a licensed 7.1 Gold bank-statement import, and §8.2b's `RESERVEDNAME` group-identity rule that its cash/bank gate is built on. |
| 2026-09-11 | Extended §12a.9 to TallyPrime 7.1 licensed Silver and the `StandardLedgerCatalogV1` profile from a live rename/restore capture (VERIFIED), and recorded three structural facts with separate markers: ledger `RESERVEDNAME` follows the same reserved/not-reserved convention as groups, one of nine populated (VERIFIED), company-scoped ledger GUIDs (PARTIAL — verified on all nine rows of one company). XML-driven rename and deletion remain UNVERIFIED. A later revision the same day withdrew a `CMPINFO` alteration-counter claim that the committed fixtures did not support. |
| 2026-09-11 | Narrowed §9.13's company-guard paragraph to match §9.11d: which *kind* of mismatched `SVCURRENTCOMPANY` posts silently is UNVERIFIED, so the classification by name shape was withdrawn, and the pre-write check was corrected from the GUID alone to the whole §9.11b identity tuple. |
| 2026-09-18 | Added §8.2c: `REFERENCE`/`ISPOSTDATED`/`ISINVOICE`/`PARTYGSTIN` added to the voucher `FETCH` list and captured on licensed TallyPrime 7.1 Silver (`BRIDGE SHAPE LAB`, 67 vouchers, twelve windows). `ISINVOICE` never carries `TYPE="Logical"`, unlike the other three; `PARTYGSTIN` round-trips but was empty on every observed row (population UNVERIFIED). |
| 2026-09-18 | Added §1.1(d): the rule `mark_forbidden_numeric_references` applies before parsing, now public in `bridge-tally-protocol`, including that a `&#` with no `;` in its window no longer ends the rewrite. |
| 2026-09-18 | Added §11c: the pre-flight volume bound for windowed voucher reads, with the bytes-per-voucher measurements behind it (a whole-year 35.0 KB mean on an inventory-heavy book, understated by a one-day probe; half-month windows at 98.4% of the cap). The measurements are VERIFIED; the date-first census and the AlterID-narrowed parts the rule sends are UNVERIFIED live. |
| 2026-09-21 | §11c after the bridge#520 rectify: every census is bounded before it is sent (one date census when the mark fits one, otherwise AlterID spans of 8,192 sized against the whole cap; a mark needing more than 256 spans is refused as `voucher_window_book_too_large`); parts are admitted against the census and their union; the read allowance is spent at dispatch; a divided read is bracketed on `ALTVCHID` and `ALTMSTID`; a replay carries and closes against the first read's witness; the pre-post check refuses a window the bound would divide. Added §11c.5, the first live evidence (licensed 7.1 Silver lab): census and span shapes, `ALTVCHID` as a count bound and on every voucher change, a ledger rename moving only `ALTMSTID`, and end-to-end timings. |
| 2026-09-23 | Added §12a.10: two opening bills in a licensed TallyPrime 7.1 Bills Receivable capture are dated and due the day before the book's `BOOKSFROM`, so `BOOKSFROM` does not bound a Bills row's dates (bridge#612). Earlier dates, a due date before its bill, and bills dated after the as-of date are not measured. |
| 2026-09-25 | §11c.5: added the `ALTVCHID` step on multi-voucher gateway imports and on screen edits (PARTIAL: one run each, lab scripts, not Bridge's post path), the basis of `post_import`'s reported `target_voucher_step`. |
| 2026-09-25 | §9.13: a scoped correction recording two licensed TallyPrime 7.1 Gold field runs of Bridge-built Payment, Receipt and Contra files sent over the gateway by a script, with `verify_import` returning `posted_verified` (VERIFIED on one book; PARTIAL on a second, where larger reads failed, bridge#485). §3.1 and §5.3's import-verification note now cite it. Native `post_import` on Gold was still not observed as of that date (see the 2026-09-28 row). |
| 2026-09-26 | §9.14: "an upsert omitting `REFERENCE` kept the stored value" moves from PARTIAL to VERIFIED for a gateway-written `REFERENCE` on licensed 7.1 Silver, on a second independent run through Bridge's own amendment file (bridge#239). A `REFERENCE` typed in Tally's screens remains unmeasured. |
| 2026-09-26 | §6.3: a custom-report FIELD without `<TYPE>Amount</TYPE>` returned money as a display string (sign dropped, digits grouped); with it, signed. One variable, licensed 7.1 Silver, PARTIAL |
| 2026-09-26 | §9.4e: fold-equal ledgers (a trailing CR LF, and case) coexist, and an import binds the exact name in both creation orders; the fold-only case is open. Licensed 7.1 Silver, PARTIAL |
| 2026-09-26 | §11c.5: the multi-voucher step through Bridge's own post path (50 and 200, one run each) is VERIFIED; and what screen actions move `ALTVCHID` and `ALTMSTID`, with the fields the import-verification read cannot see. |
| 2026-09-26 | §12a.11: Balance Sheet and Profit and Loss by name on licensed 7.1: structure, plain signed amounts, empty not zero, a trial-balance tie, cost. PARTIAL |
| 2026-09-28 | Licensed TallyPrime 7.1 Gold, one client book, one run each, VERIFIED for what was read back (one session, not repeated on a second book; the delete is one voucher and PARTIAL): §9.14 an upsert (same `REMOTEID`, `ACTION="Create"`) alters Payment, Receipt and Contra in place (about 300 vouchers, full read-back), and `native_remote_ids` is in batch voucher order (9 of 9 captured batches; a tenth was not captured); §9.4 a rename by `Alter` changes only the name, `ALTERID` and the company counter (about 120 ledgers, 2 groups); §9.12b a delete by the creation `REMOTEID` of a directly imported voucher (one voucher, PARTIAL); §9.4e a CR LF in a master name sent as `&#13;&#10;` (6 vouchers). |
| 2026-09-29 | §11e: ledger catalogue, balance, party-master and group read sizes, and the master part-read go/no-go (GO), on one 1,989-ledger synthetic licensed 7.1 Silver book with no vouchers. PARTIAL (bridge#668, bridge#679) |
| 2026-09-30 | §11c.3: a full census span is about 22 to 27 MB, not 22 MB: the census row measured about 3.27 KB on the wire on a book with a mark of about 1.03M. PARTIAL (bridge#899) |
| 2026-09-30 | §11e: `NOT` filters, several listed together, on the party-ledger master and balance collections of a 4,339-ledger synthetic licensed 7.1 Silver book with no vouchers: exact expected rows for up to 511 excluded parents as up to seven formulas, and for a seven-formula request of about 110 KB whose extra names were fictitious. PARTIAL (bridge#679) |
| 2026-09-30 | §11e: the `$AlterID > a AND $AlterID <= b` filter on `List of Ledgers` (GUID-only fetch) on three licensed 7.1 Silver books, marks 5,547, 102,161 and 316,028: slices hold at most their width, their union equals the catalogue, an empty slice is a well-formed 2,994-byte answer. PARTIAL (bridge#679) |
| 2026-09-30 | §11e: the Company collection's `NUMLEDGERS` (request `BridgeCompanyLedgerCountV1`): present on all 31 loaded lab companies, equal to the ledger catalogue and census counts on a 4,339-ledger synthetic book, and to the census counts of two real books read by another lane. PARTIAL (bridge#938) |
| 2026-10-01 | §12a.13: a company with inventory on and no stock item, captured through `bridge_mcp` (count `0`, an empty collection, an empty Stock Summary envelope), and the 2026-09-30 stock captures reproduced byte for byte through `bridge_mcp`. PARTIAL, one company each |
| 2026-10-01 | §12a.13: the sign of a stock value: a negative wire value is stock held and shows as positive on Tally's Stock Summary screen; a positive one shows as `(-)`. VERIFIED on one synthetic licensed 7.1 Silver company against Tally's own screen |
| 2026-10-01 | §12a.13: the Company collection's `NUMSTOCKITEMS` after one stock item was deleted on Tally's own screen: it fell by one and equalled the rows read, on one synthetic licensed 7.1 Silver company. PARTIAL, one sample |
| 2026-10-01 | §12a.13: a taxed sales item invoice (two entries on duty ledgers with GST heads) was created and read back with its tax amounts as imported, and where its ledgers and goods line arrive in each of two fetches. VERIFIED, one voucher, licensed 7.1 Silver, one synthetic company |
| 2026-10-01 | §12a.13: on a batch-wise company a sales goods line with no batch allocation was refused in band and the same line with one was created (VERIFIED, one pair); and an item's stock value changed sign after its first outward line (PARTIAL, unexplained, one item). Licensed 7.1 Silver, one synthetic company |
| 2026-10-01 | §12a.13: the Stock Summary with the explode flag, a raw lab request: a flat answer expanded by one level, whose item lines' quantity and amount equalled the item collection's for the nine items holding stock on one synthetic licensed 7.1 Silver company; items holding nothing, and an item under a stock group inside a stock group, had no line. PARTIAL, one book, two runs |
| 2026-10-01 | §11e: a 150-character company name stored whole (limit not shown) and handled in a census slice request with no dialog, and a census row's name copies (two, three for a built-in ledger), on the licensed 7.1 Silver lab book set. PARTIAL, one company, one release (bridge#917, part) |
| 2026-10-01 | §12a.14: what a party's bill trail and unadjusted detail were measured against on one seeded synthetic licensed 7.1 Silver book (same reference on two parties, a reused reference, a two-party journal, on-account netting); the whole-window read and the entry-ledger filter follow from it. |
| 2026-10-01 | §12a.7: what a GUID-filtered `Company` collection fetch returned for a company's own details on two synthetic licensed 7.1 Silver companies: the dates, currency, GST flag and company number with their text forms; an unset state or PIN code as an absent element (one company, before and after it was set); no registration type or number on the row of a company that has a registration; nothing for the address list (one sample, the address not confirmed set). VERIFIED for the names returned; PARTIAL for the address |
| 2026-10-02 | §11f: how long Claude Desktop (chat app, macOS, bundle version 2.19675.0) waited on one tool call: silent calls of 20, 55, 75 and 130 s were answered, a 250 s call was cancelled at 240 s with `Request timed out` in two runs and the server was not stopped within a minute of the cancel; no `progressToken` was offered on six calls, so the effect of progress is unanswered; no Tally request. PARTIAL, one run per case |
| 2026-10-02 | §12a.13: `NUMSTOCKITEMS` against the item rows after create, create-on-existing, rename, alias, move to another group, an inventory purchase and two bulk imports of 20 items on one synthetic licensed 7.1 Silver company: equal at every point; this alias was not counted as an item and added a name entry to the row; a Create on an existing item changed no field of those read; a parent-only Alter changed, of the fields read, only the parent. For a ledger, §9.4g records (one run, PARTIAL) that a Create on an existing name with a changed parent, bill-wise flag and opening balance replaced all three, also answering `CREATED=0, ALTERED=1`: the two master types gave different results here, and neither result is generalised to the other. PARTIAL, one item each, once; not measured: screen-created items, delete with vouchers, merge, a reload. |
| 2026-10-03 | §12a.3: a whole `outstandings` call (release build from master 7b994e65) on the book of the 1-Dec-2108 due date: state `complete`, 52 requests with none refused, 4.0 s, 1,498 open bills in a page of 500 and a 181,044-byte result. PARTIAL, one run, one book; not measured: the 2108 bill's row in a live response (not on the page: the page runs oldest-due first and the bill is not yet due), the later pages, other books or directions. |
| 2026-10-06 | §12a.15: the ledger catalogue request with one more native method (`IsBillWiseOn`) returned exactly one `Yes` or `No` `ISBILLWISEON` on every ledger of five synthetic licensed 7.1 Silver books, equal, on the two books where it was compared, to the outstandings snapshot's flags, with the cash, bank and duty ledgers answering `No` and four CR LF ledger names answering a flag (three `No`, one `Yes`). VERIFIED for those books (the captures of four are committed; the GST RECON LAB answer is not); no large book |
| 2026-10-09 | §12a.14: capture 4 of #1356 on one synthetic licensed 7.1 Silver party (`BRIDGE BILLTRAIL LAB`, the published 0.5.1 tool): an `as_of` earlier than the last voucher, a post-dated receipt before and after its date, an advance later adjusted by an invoice (listed as allocated in the unadjusted detail, closed in the bill trail), a note with and without a reference, and a hand-keyed opening bill, each compared with Tally's Pending Bills screen. VERIFIED for this party; what was not measured is listed in the section |
| 2026-10-14 | §12a.22: the stock items of a `masters` read with their dated HSN and GST lists, on three committed captures of one synthetic licensed 7.1 Silver book (baseline, item as first saved, item with two dated entries in each list): list shapes, the reserved character, Tally's own rows on a first save, the credit element present on one entry only, a stock group's code not on the group's four other items (an item deferring to a coded group not observed), row sizes. PARTIAL, one book, one release |
| 2026-10-21 | §12a.21: the Profit and Loss of an integrated inventory book with `Less: Closing Stock` as a `PLSUBAMT` leaf, the Balance Sheet without a stock leaf, no Gross Profit leaf, on four committed captures of one synthetic licensed 7.1 Silver book; a second book with opening stock reported by the lab, not committed. |
| 2026-10-26 | §11c.4: the empty-window corroboration by census marked PARTIAL after one live run on TallyPrime 7.2 Silver (`lab/1240-empty-window-live-answers`, `ce849cc6`): the same answers, the request counts as predicted except one window that sent fewer. |
| 2026-10-27 | §12a.13: items inside stock groups nested three deep on one synthetic TallyPrime 7.2 Silver book: the item collection lists every depth, `NUMSTOCKITEMS` counts them, the plain report has a line only for what sits under the root, and each nested row's quantity equals Tally's Stock Summary screen with every level expanded. VERIFIED on that book |
| 2026-10-28 | §12a.23: Stock Query by item name on one synthetic TallyPrime 7.2 Silver book: the request, the answer's groups and godown, batch and quantity lines, which summed to each item's closing; the empty answer for an unknown name. PARTIAL |
| 2026-10-29 | §12a.13: period ends inside the year on two synthetic TallyPrime 7.2 Silver books (30 September and 31 May; 31 August and 30 September, nested groups included), and a voucher dated on the end date counted by both sources. PARTIAL |
| 2026-10-30 | §12a.23: separated the claims its fixtures carry from those resting on lab captures that are not fixtures (`SVSTOCKITEM`, an unknown variable, an item with no stock), and listed the shapes the parser refuses. |
| 2026-10-31 | §12a.23: the lines' bullet cut to six fixture items tied to the stock-item fixture, with every lab-only claim given its branch; the year-end closings were Tally's Stock Query screens, not the Stock Summary; three items, not two, answered empty at the year end; the shapes the parser still reads; a later period start and four other gaps added to Not measured. PARTIAL |
| 2026-10-31 | §12a.23: the parser now refuses a hidden or control character in a name or quantity, a group with no line after it, and lines in more than one unit. |
| 2026-10-31 | §12a.23: named the closed set of characters the parser refuses in a name or quantity, and added a godown name holding a tab to Not measured. |
