# `vouchers` on an empty window beside a busy day: the tool's answer from one live run

Covers `vouchers-empty-window-beside-busy-day.live-answer.json`.

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `vouchers-empty-window-beside-busy-day.live-answer.json` | 1028 | `bfe73e841229dc3b8b1fb05db5807afe34fcaa74b04c28bf281fdcad9d45ecad` |

Captured, not authored: the file is the tool's answer to one `vouchers` call as the lab
recorded it, beside the call's arguments, copied here without any change (its size and hash
are in the table; how the lab wrote the file is not known beyond that, so "captured" rests
on the lab's record and the hash). It is the tool's answer, not Tally wire bytes: the answer
records only the response's hash and size (`response_sha256`, `bytes`), and the lab recorded
that the response bytes were not retained.

## What was read

The file itself shows: the call (`vouchers`, 2026-09-02 to 2026-09-03), a company number and
GUID of a company named `BRIDGE FINGERPRINT LAB`, and a read time of 2026-10-12, 15:08 UTC.
The rest is context the lab recorded when it took the file; this file does not prove it:

- The company is a synthetic one made for lab work; no client data.
- TallyPrime 7.1, licensed Silver, education mode off, the company loaded alone, on the
  Windows lab machine, with the installed Bridge build 411f7cca (the build in which this
  reason was still named `nonempty_uncorroborated`).
- A second call in the same minute, `vouchers` for 2026-09-01 to 2026-09-30, answered
  `complete` with a total of 6 vouchers, all dated 1 September (that answer is not committed
  here); so 2 and 3 September held none, and the day before the window was busy.

## What it establishes

On this book, in this build: a window with no vouchers, beside a day that has some (by the lab's record), was
answered `partial` with total 0, an empty `items` list and the window's one part served with
0 rows; the answer carries one census request and one marks request, and no cost block.
`evidence.state` repeats `partial`. It is the shape that
`an_empty_window_beside_a_busy_day_is_partial_by_the_wider_reads_rows` compares its own result
with: the same keys and value kinds at every level (an array's elements are compared with the live array's
first element; counts are not compared), and the same state, total and items values.

## What it does not establish

The reason in this file is the name that build used; the current name is
`empty_window_wider_read_nonempty`, so a test must not compare that one value. The file
says nothing about an empty window with no busy neighbour, about a window the census could
have miscounted, or about any other build or edition.
