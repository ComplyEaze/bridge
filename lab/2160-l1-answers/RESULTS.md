# Results: #2160-L1, the first read of voucher types after a load, 2 Nov 2026

**Reproduced by raw request, three times of three, and the first difference is named from bytes. The first `List of VoucherTypes` read after a company load omits `<LANGUAGEID TYPE="Number">0</LANGUAGEID>` from `LANGUAGENAME.LIST` on two rows of 26, `Shape Purchase` and `Shape Sales`. The second read has it, and every read after that matches the second. Row order is the same. No other value differs. Nothing in the book moved.**

Book BRIDGE SHAPE LAB, the book of the recorded refusals, TallyPrime 7.2 Silver, loaded alone. Reads only; nothing was keyed or written. Requests rendered from master 559eff14c's format strings, not captured from Bridge: [partial] for that link. Each pair as `fetch_native_report_paired` sends it: POST, `GET /status`, POST, `GET /status`. Each POST was its own `curl.exe` process, so a new connection. Bridge's own pair may reuse one; that was not compared.

| step | what came first | first read | second read | pair |
|---|---|---|---|---|
| a | Tally restarted, company loaded | 48,456 B, `1410f760` | 48,648 B, `2220d9ab` | **differ** |
| b | step a's pair, no reload | 48,648 B, `2220d9ab` | 48,648 B, `2220d9ab` | equal |
| c | company shut and reloaded, same Tally | 48,456 B, `1410f760` | 48,648 B, `2220d9ab` | **differ** |
| d | Tally restarted, loaded, one ledger read | 48,456 B, `1410f760` | 48,648 B, `2220d9ab` | **differ** |

Full sha256 in `evidence/log.txt`. Marks read after each step: V 112, M 292. [verified]

## The difference
First differing byte: offset 44,874, the first read's `</LANGUAGENAME.LIST>` where the second has the `LANGUAGEID` line. A line diff finds two insertions and nothing else, 96 B each, 192 B in all:

```
      <LANGUAGEID TYPE="Number">0</LANGUAGEID>
```

- Before `</LANGUAGENAME.LIST>` of `Shape Purchase` (MASTERID 266).
- Before `</LANGUAGENAME.LIST>` of `Shape Sales` (MASTERID 218).

The other 24 rows carry the line on both reads. By structure: rows 26 and 26, order equal, same set; 737 values and 741, the four extra being those two elements and their `TYPE` attributes. [verified]

## What the steps settle
- A Tally restart is not needed: a reload alone brings it back (c).
- Ledger read first does not cure it (d). The ledger read here is `render_native_ledger_export_request`, for 1 April 2026 to 31 March 2027. `masters` has no ledger kind, so this is my reading of the row's "masters list of ledgers".
- A voucher-types read cures it until the next load (b).
- The two rows are the only two of the 26 with an empty `RESERVEDNAME`: the book's own types, MASTERID 218 and 266, under Sales and Purchase. The other 24 are Tally's, each with a `RESERVEDNAME` and MASTERID 88 or below. That matches one to one on this book. That it is the cause is a [hypothesis]: no other book was read, and why Tally does this is not known.

The recorded refusals fit, from the notes of those sittings: each followed a company load with no voucher-types read since. `lab/1709-settle-row-2-answers`: loaded 11:49 before launch 1, restored and loaded 12:08 before launch 2. That is a reading of notes, not a repeat through Bridge: this row did not run Bridge.

## Reach, from the code, not run
`require_stable` compares bytes (`runtime_masters.rs:223`), and `first != second` in `fetch_native_report_paired_with_evidence` refuses on this difference. The numbering read in `runtime_masters.rs` near line 437 calls the same `read_native_rows` with `VoucherTypes`. So it would meet the same difference on the first read after a load, on a book with such rows. Severity is for the issue; this row did not run a build or a post.

`LANGUAGEID` is not among the request's FETCH fields; it arrives inside `LANGUAGENAME.LIST`. `native_masters.rs` does not name it. Whether the two answers parse to equal rows through that code was not run.

## Left behind
Tally restarted twice, by its own Quit at Select Company with no company loaded, and started again outside the package; each start loaded the demo company by its ini, which was shut before the book was loaded. No licence or other prompt. Company shut, no company loaded, one tally.exe on 9001, `GET /status` 200. Its PID is new. The Gateway shows `(e) Data exceptions exist` on this book and on the demo; it was not looked into, and no read depended on it.

**Confidence:** one book, one Tally, one sitting, three repeats with identical bytes: PARTIAL for the cause; verified for the named difference on this book.
