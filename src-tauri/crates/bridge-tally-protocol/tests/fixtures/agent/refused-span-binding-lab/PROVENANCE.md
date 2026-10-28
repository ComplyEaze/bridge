# A native post whose span binding was refused, as `verify_import` answered it

Covers the one file below.

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `013-s8-verify_import-b1.json` | 42671 | `93f57151a41fed962934231c9c8ec0539120bb97bb4ae94acbeb584a0d3498c8` |

Captured, not authored. The file is the whole MCP exchange of one `verify_import` call (the
arguments, the JSON-RPC response and its timing), copied with no edit from branch
`lab/1039-refusal-answers` at `7f64a199`, whose `SHA256.txt` lists the same SHA-256. The run is
#1039's row R1 of 28 October 2026 on the synthetic book BRIDGE CONCUR LAB (TallyPrime 7.2,
Silver), through the 0.6.1 cut build: a Payment saved on Tally's screen inside a 50-voucher
native post's span moved the company's voucher mark by 51 against 50 created, and the binding
of the post to its own vouchers was refused `span_step_not_created`. The answer reads
`verification_incomplete`, with all 50 vouchers `matching_content_observed`. It is the state a
content review (`acknowledge_post_review` with `doubt` set to `content_only`) is admitted on.
No file carries a GSTIN, PAN or contact detail.
