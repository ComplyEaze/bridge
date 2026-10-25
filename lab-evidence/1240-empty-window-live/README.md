# Lab run sheet: the empty-window check by census on a live Tally (read-only)

Written 25 Oct 2026. It measures one change, the pull request #1897 (head `15c887091f2cf07f6fb0aed52f24b6e4da975d56`, branch `empty-window-census-only`). The same change was merged earlier as #1452 and reverted for want of a live run. Until this sheet has been run, the change is checked on test doubles only.

**What changes.** When `vouchers` (also `voucher_presence`, `ledger_movement` and the registers, which share the code) finds a window empty on a book that is counted, it looks again at the window widened by a day on each side. Before, that second look counted the widened window and then read its vouchers; now it only counts it, because the count carries each voucher's day and that is all the check uses. The answer should be the same, with fewer requests. A book whose count takes more than one request (a voucher mark above 8,192) has its count bracketed by a second read of the company's marks; a book counted in one request is not.

**What this run decides.** (1) Does the answer stay the same, on a real Tally, for the windows the 12 October run used (`lab/1452-empty-window-evidence`, TallyPrime 7.1)? (2) Are the requests really fewer? (3) Does a book counted in four requests (`BRIDGE CORPUS DENSE`, voucher mark about 29,900) give the same answer, and does the bracket hold when nothing else touches the book?

Read-only. No key, no import, no write, no backup or restore, no setting changed. Synthetic BRIDGE companies only. Posting in the built tool stays OFF.

## Rules

`LAB-COMMON v3` (as sent for the 626, 1038 and 1050 items) applies in full except C5 and C6 (nothing is backed up or restored). In particular: C1 (the company named is the only one loaded; Education mode anywhere is a STOP), C2 (step 0 before the first send and again after any pause), C4 (decode and parse; an answer that does not parse is a STOP), C7 (a dialog on Tally's screen: no key, no request, report), C8, C9, C10.

C3 holds for the marks reads, which are raw requests sent with curl as C3 says. The `vouchers`, `list_companies`, `tally_status` and `egress_log` calls are sent through the built tool instead.

Added for this sheet:
- One call at a time, from one sender whose client has no timeout (the bundle's `bridge_mcp` over stdin, one file at a time, as the 697 sheet does), never cancelled, retried or re-sent while in flight. A call with no answer after 10 minutes is left running, not re-sent: STOP and report. Time every call with a wall clock (seconds).
- Every answer is bounded: an answer above 2,000,000 bytes is a STOP before it is used (keep the file). A call that fails is reported as failed, never as an empty answer or "no vouchers".
- The Gateway of Tally shows no dialog before each call. Screen steps (the Day Book readings) are done by the person at the lab computer, one at a time, back at the Gateway of Tally before the next call.
- `STOP` on a different release than the one the lab reports in step 0, on a company other than the two named, and on any refusal this sheet's table does not list: report the whole answer first.
- Both builds are told apart by their sha256, recorded before the first call; a fresh empty `BRIDGE_AGENT_DATA_DIR` for every file; `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`; the Terms value of the lab's standing setup; redaction off.

## Builds (one variable: the build)

- **P**, the change: the Windows bundle built from head `15c887091f2cf07f6fb0aed52f24b6e4da975d56` of #1897. The requester gives its CI run, artifact and digest; verify the digest before unpacking.
- **M**, the control: a Windows bundle of master without this change (the release-candidate build is one if it does not hold #1897; the requester names it and its digest). The control matters: the reason an empty window beside a voucher carries was renamed after the 12 October build (it said `nonempty_uncorroborated`; master says `empty_window_wider_read_nonempty`), so the 12 October table is a guide, not a control.

Run every window with P first, then with M, on the same book state. Both only read.

## Books

1. `BRIDGE BATCH LAB` (company number 100050), the book of the 12 October run. Loaded ALONE.
2. `BRIDGE CORPUS DENSE`, loaded ALONE (Shut the first before loading the second; a person at the lab computer does this and says so).

If any other company is loaded: STOP, no call.

## Steps

Step 0 (C2 in full) for the first book, then:

**Part B, BATCH LAB** (a count of one request)
1. `a-list.jsonl` with build P: EXPECT `list_companies` one company, number 100050. Record the GUID and `tally_status` (release, edition, Education off).
2. Marks read `M-B0` (the company-marks request `requests/c1-marks-TEMPLATE` of `lab/72-reads-rd21`, placeholders only; keep the whole answer, including its CMPINFO block). Record ALTVCHID and ALTMSTID. On 12 October the book's last entry was dated 21 Aug 2026 and the reads saw a voucher mark of 82 and a master mark of 224 while three keyed test vouchers stood in it; they were deleted afterwards and the marks moved on. A different mark now is recorded, not a STOP.
3. Oracle, by the person at the lab computer on Tally's own Day Book, one window at a time, each with a screenshot: 21 to 22 Jul 2026, then 10 to 12 Nov 2026, then 19 to 20 Aug 2026, each shown with the day before and the day after. Write down, for each window and each neighbouring day, the number of vouchers. EXPECT (the 12 October book): W1, 20 Jul holds 10 vouchers, 21 and 22 Jul none; W2, no voucher near; W3, J 2 on 18 Aug and LSB/0001 on 21 Aug, none on 19 or 20 Aug. If the book differs from this, the oracle you wrote down decides what to expect; say so.
4. `b-batch-lab.TEMPLATE.jsonl` (`<GUID>` filled) with build P, once. It sends three `vouchers` reads (W1, W2, W3), each followed by `egress_log` (the last two records). Keep stdout and stderr byte for byte.
5. Marks read `M-B1`; it must equal `M-B0`.
6. The same file with build M (a fresh data directory), once. Marks read `M-B2`, equal.

**Part C, DENSE** (a count of four requests; the heavy part)
7. Shut BATCH LAB, load `BRIDGE CORPUS DENSE` alone; step 0 again. `a-list.jsonl` with build P: record the GUID. Marks read `M-C0`: EXPECT ALTVCHID about 29,900 (a mark that differs by more than a few hundred is recorded and reported before step 8).
8. Dates, by the person on Tally's Day Book for the whole financial year: the date of the book's LAST voucher, `L`, and whether a voucher stands on every day up to it (82 a day is expected). Screenshot. From `L` build the two windows: **D1** = `L`+1 to `L`+2 (both days empty, the day before holds vouchers); **D2** = `L`+30 to `L`+32 if it lies in the same financial year (else: no D2, say why). A window that is not empty on the Day Book is not a window of this sheet: STOP and report.
9. `c-d1-dense.TEMPLATE.jsonl` (`<GUID>`, `<D1_FROM>`, `<D1_TO>` filled, dates as YYYY-MM-DD) with build P, once, timed. If its answer is a refusal or takes more than 10 minutes, STOP before D2. Marks read `M-C1`, equal to `M-C0`.
10. `c-d2-dense.TEMPLATE.jsonl` (filled) with build P, once, timed. Marks read `M-C2`.
11. The same D1 file with build M, once, timed; marks read `M-C3`. (M is sent after P, on purpose: the lighter read goes first on the heavy book. Skip this step, and say so, if the requester has not named build M.)
12. C10: shut the company, no company loaded, current date as at step 0.

## What to record for every call

The tool's whole result (state, reason, `items`, `total`, `window` with its census, marks and parts, `evidence`), the `egress_log` record of that call with its `request_trail` (sent, failed, omitted, and each request's kind, size and outcome), the wall-clock seconds, and the sha256 of the file sent. For each call: (a) the count of data posts (those not sized like the identity bracket or the census or marks reads; read the sizes off M's trail first) and (b) the count of census posts.

## EXPECT (a prediction, not a result; every line UNVERIFIED until run)

| call | state and reason, build P and build M | requests sent (`request_trail.sent`) |
|---|---|---|
| W1, 21-22 Jul | partial, `empty_window_wider_read_nonempty`, `items` empty, `total` 0 | P 28; M 34 (12 October, old build: 34) |
| W2, 10-12 Nov | partial, `empty_uncorroborated` | P 34; M 40 (12 October: 40) |
| W3, 19-20 Aug | partial, `empty_window_wider_read_nonempty` | P 28; M 34 (12 October: 34) |
| D1, `L`+1 to `L`+2 | partial, `empty_window_wider_read_nonempty` | P fewer than M; P sends no data read of the widened window |
| D2, `L`+30 to `L`+32 | partial, `empty_uncorroborated` | P fewer than or equal to M |

The totals 28 and 34 are what test doubles give for P, and 34 and 40 are what the 12 October run recorded for the old build, so P is expected to send 6 requests fewer on a book counted in one request. In P's trail the data post of the widened window (1740 bytes on BATCH LAB) is absent: two such posts (one pair) instead of four. On DENSE the closing marks read of a divided count is present in P, and the marks read before and after are equal. A refusal `voucher_window_changed_during_read` in P on a book nothing else touched is a FINDING.

## Not measured by this sheet

`voucher_presence`, `ledger_movement` and the registers by name (they share the code); a window that holds a voucher (the check refuses it as `window_contradicted`; it cannot be produced without writing); Education mode; Gold; a client book; a count that moves during the read (it needs a hand edit mid-read, a write).

## Report

The `.out` and `.err` files, the marks answers with their CMPINFO blocks, the Day Book readings, the screenshots (kept on the lab PC, listed by name, size and sha256), the builds' digests, the Tally release and edition, wall-clock times, and a notes file with UTC times and one line per step. Evidence on a new branch `lab/1240-empty-window-live-answers` with `SHA256.txt`; send the SHA to the requester before pushing. Confidence: one run on one release, PARTIAL.
