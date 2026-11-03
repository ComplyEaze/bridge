# Row 2162-S2: are an invoice's GST status flags set when the read-back reads them, straight after its import?

Question (#2162 item 2): `BP/26-27/0010` was read only after later vouchers had been keyed, so no answer shows the status flags at the moment the code reads them, straight after a post (voucher-writes reference §9.16). Does a read-back sent at once after the import's answer equal the same read-back at +5 s and at +60 s, byte for byte?

Book: BRIDGE PILOT LAB (synthetic). Grant: BK before the import, restore after it, restore check byte-identical, as S1 did. Write down the release shown on Tally's screen.

The import is the row's subject, not its setup: it is the build's own request that created `BP/26-27/0010` on 10 Oct, with the number and the REMOTEID changed and nothing else (gen.py asserts both). The seed rule does not apply, because the flags this import sets are what is measured.

No Bridge build. Raw requests, one at a time, by curl.exe, each answer written to a file, counted by an XML parser, not by a text search. Close every other Bridge process first, and send nothing else to Tally while run.py runs.

Steps:
1. Read the marks (`r02-marks.xml` of `lab/1578-postdated`). Take BK. Read the Gateway's current date and write it down. If it is before 1-Aug-2026, stop: the invoice would be post-dated, which is S1's variable, not this row's.
2. `python3 -I gen.py BP/26-27/0046` in this folder (the files here were made with it; running it again writes the same files). Send `R-0046.xml` once. Expect no VOUCHER row: the number is unused. If a row comes back, pick the next unused number, run gen.py with it, and repeat this step.
3. `python3 -I run.py <gateway URL> 0046`. It sends `I-0046.xml`, then `R-0046.xml` at once, at +5 s and at +60 s, counted from the import's answer, and writes `times-0046.txt`. If curl fails it stops. Do not resend the import; go to step 5 and report it.
4. Read the marks again.
5. Restore BK. Read the marks again, and check that answer is byte-identical to step 1's.

Record:
- the import's answer: CREATED, ALTERED, ERRORS, EXCEPTIONS, LASTVCHID.
- for each of the three read-backs: bytes, sha256, the number of VOUCHER rows, `ALTERID`, `GUID`, `ISCANCELLED`, `ISOPTIONAL`, `ISPOSTDATED`, the three `VCHGSTSTATUS` flags and `ISGSTOVERRIDDEN`, each counted as direct children of the voucher;
- whether 00s, 05s and 60s are byte-identical;
- `times-0046.txt` as written;
- the marks of steps 1, 4 and 5.

Expected: CREATED 1, ERRORS 0, EXCEPTIONS 0. Each read-back holds one row reading included Yes, uncertain No, excluded No, post-dated No, cancelled No and optional No, as 0010 read later (`pilot-lab-invoice-readback-post-dated-no`, S1). All three are byte-identical. An empty read-back means the filter did not find the voucher: stop, and report the number sent and the number on the screen. That is not a finding about the flags.

Decides: if all three are equal, the read-back straight after a post reads the flags Tally keeps, item 2 closes, and the post reads without a wait. The code's read comes later than 00s, after the marks, masters and window reads. If 00s differs from 60s, the post must wait or re-read; whether 05s equals 60s shows how long; and the flags that differed go to D66 before any code.

Code admits the trigger, line numbers at master a6c64b8b:
- No Bridge path posts this invoice: Sales is not in `LIVE_QUALIFIED_VOUCHER_TYPES` (agent_import.rs:390). That is why this row sends raw requests, as #2162 item 2 asks.
- The code's read is straight after its own POST: the post calls `verify_import_after_current_dispatch` (agent_import_post.rs:1614, agent_import.rs:2155), which runs the verification of the post just made. It calls `read_back_sales_invoice` (agent_import.rs:2380, agent_import_invoice.rs:1677) after the mark, masters and window reads, with no wait in between.
- `template-import-0010.utf16le.xml` is the fixture `pilot-lab-invoice-post-request.utf16le.xml`, sha256 b333f0dc, which `native_post_request` (agent_import_post.rs:3063) rendered. Its answer was CREATED 1 (`pilot-lab-invoice-post-answer`).
- `template-r-0010.utf16le.xml` (sha256 10f9711c) is the read-back the code at 094c3edc (`fix/2162-postdated-readback`) renders with `render_invoice_readback_request`. S1 sent it, and a test there checks it byte for byte. Its filter (agent_import_invoice_wire.rs:493) tests the window, the number and the type GUID, and 1 Aug 2026 lies inside the window.

Row: lab/2162-s2/
