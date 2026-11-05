# Results: #2265 row R4 on THE CLIENT BOOK, a month's registers on a very large book, 5 Nov 2026

**sales_register did not answer the month: it ran 1,207.3 s (over P7's 15 minutes) and was refused `voucher_window_too_many_reads` (reads allowed 128, needed at least 156).** Every one of its 808 requests to Tally was answered (none failed), 119,090,014 bytes from Tally in all; every read was answered. The voucher reads in the kept tail of the request trail took 2.8 to 3.8 s each, whatever their answer size, each sent twice. `purchase_register` and `gst_by_counter_ledger` were not made. Read only: nothing written, keyed, backed up or restored; marks before and after byte-equal. One book, one sitting: PARTIAL.

Book: THE CLIENT BOOK, loaded alone by the owner, shown here only by shape. GUID `<company guid>`. TallyPrime 7.2 Silver. Tally's date Thursday 24-Sep-2026; period 1-Apr-26 to 31-Mar-27. The Gateway showed two lines, "(c) Connected for online access" and "(e) Data exceptions exist": Gateway lines, not dialogs. No login was asked.

Build: BUILD-M (187e882b), zip sha256 equal to the digest, `bridge_mcp.exe` 9bda7144f54e5ef68477b5ca30be33a17759de2e1ad756f840785c67bcab2dc0 equal to the smoke file, before and after. Over stdio on 9001, `BRIDGE_AGENT_ENABLE_WRITES` unset, one new empty data folder, one process.

## Sizes (raw reads by curl, one at a time)

SVCURRENTCOMPANY removed from the raw size reads; one company loaded. The templates are R4b's (`lab/register-large-book` at 14cf41c4); `extent` also drops Name, CompanyNumber and its GUID filter.

| read | bytes | seconds | count |
|---|---:|---:|---|
| marks m0 | 3,458 | 0.005 | COMPANY rows 1, V 1,026,162, M 20,190 |
| catalogue | 11,610,016 | 1.250 | 9,451 ledgers |
| groups | 2,758,478 | 0.369 | 1,455 groups |
| extent | 4,610 | 0.007 | BOOKSFROM 20220401; NUMLEDGERS 9,451; NUMGROUPS 1,455; NUMVOUCHERTYPES 54; LASTVOUCHERDATE 50420604 (as given; not a date in the books) |
| day census, August 2026 (31 reads) | 51,877,674 | 10.569 summed | 16,112 vouchers (393 to 861 a day; 0.24 to 0.56 s a read) |
| marks m1, after | 3,458 | 0.005 | byte-equal to m0 |

The month is the latest complete one inside the books. The census is Bridge's census shape with FETCH DATE, one day per request. The first day's answer carried tags with a namespace prefix, which the lab's parser rejected ("unbound prefix"); the parser was changed to read prefixed tags as plain names and the saved answer was re-read, not resent.

Against R4b's synthetic book (10,123 ledgers, 2,430 vouchers in its month): about the same ledger count, about 6.6 times the month's vouchers.

## The calls

| call | seconds | Tally bytes | `isError` | state | code, cause, `failed_request` | observed, rows |
|---|---:|---:|---|---|---|---|
| `list_companies` | 0.085 | 8,196 | false | complete | none | 1 company, verified_tuple |
| `sales_register`, 20260801 to 20260831 | **1,207.303 (over P7)** | 119,090,014 | true | evidence partial | `voucher_window_too_many_reads`; reads allowed 128, needed at least 156; no cause, no `failed_request` | none in the refusal |
| `purchase_register` | not made | | | | | |
| `gst_by_counter_ledger` | not made | | | | | |

**P7.** No answer 15 minutes after the send (10:20:25Z); nothing was resent and nothing more sent to Tally; on the run lead's word the call was left to answer. At 611 s `/status` answered 200 in 2.510 s, at 15 minutes in 0.003 s, at 16 minutes in 1.767 s; `tally.exe` used about 0.85 of a core (CPU 2,510 s to 2,764 s over five minutes; working set 1,271 MB) while `bridge_mcp` waited (CPU 5 s, 20 MB, one connection to 9001 open). The answer came at 1,207.3 s.

**What Bridge sent (its `agent-egress.jsonl`, kinds, bytes and times only).** `sales_register`'s line: request trail sent 808, failed 0, the last 32 kept (21 posts, 11 status reads, all answered). In that tail each voucher read is a 1,588-B request sent twice in a row, held 2.79 to 3.76 s whether its answer is 3,022 B or 17,905,816 B; company reads (2,524 B) take 3 to 13 ms and status reads under 1 ms. 808 requests in 1,207 s is about 1.5 s each on average.

Marks m2 after the call: byte-equal to m0. `bridge_mcp` stopped by the client's stop, exit code 0.

## Reading

On this book a month's `sales_register` neither answers nor fails fast: Bridge spent about 20 minutes of reads, every one answered, before refusing on its own read cap. The per-read cost of the voucher reads (about 2.8 s even for a near-empty answer) points at Tally's per-request cost on a book whose voucher mark is about a million (V 1,026,162; the voucher count itself was not read), not at answer size. Whether the cap or the per-read cost is the thing to change is for the code, not this row. [measured: the times, bytes and counts; hypothesis: the cause]

Each voucher read was sent twice in a row in the kept tail (why is for the code).

## Limits

One book, one sitting, one month, one call. `purchase_register` and `gst_by_counter_ledger` were not run. The trail keeps only the last 32 requests, so the first 776 are counted, not described. Silver, one user; the owner's book as loaded. Raw answers and the data folder stay on the lab PC and are deleted at the end of the cycle on the run lead's word.

## Left behind

THE CLIENT BOOK loaded as the owner left it; nothing written. One tally.exe on 9001. No `bridge_mcp` of this row running.

**Confidence:** PARTIAL.
