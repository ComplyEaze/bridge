# Row 2162-S4: does a Sales type's own guard decline a number already in use, and how do Tally's number match and that guard treat case and leading zeros?

Question (#2162 item 6, the bullet "two invoices that differ only by their number"; ADR 0004 checklist item 12, its first sub-item). Bridge refuses an invoice number its number read finds (`invoice_number_already_used`). Behind that read sits the Sales type's guard: the build admits a Sales type only with Manual numbering and `PREVENTDUPLICATES` Yes, and that guard has never been measured on a Sales type. On Journal it declined in-band (`CREATED=0, ALTERED=0, EXCEPTIONS=1`, voucher-writes reference line 32). One variable per sub-row, against one seeded invoice `BP/26-27/0051`:
- **a (reads only):** does the number read find the seed under `bp/26-27/0051` (case), and under `BP/26-27/051` (a leading zero dropped)?
- **b1:** a second invoice under `BP/26-27/0051`, with a new REMOTEID: does the guard decline it, in-band?
- **b2:** b1 under `bp/26-27/0051`.
- **b3:** b1 under `BP/26-27/051`.
- **c:** the same figures under a new number, `BP/26-27/0052`: does it post?

Book: BRIDGE PILOT LAB (synthetic). Run after S3b, in a sitting of its own.
- One BK before the first write and one restore after the last, with the restore checked byte-identical.
- If b2 or b3 creates a voucher (CREATED 1), restore BK, check that M's answer equals s1's, then run s3 again (the seed), and go on with the next sub-row. If b1 creates one, see Stop.
- Write down the release shown on Tally's screen.

The imports in b1 to b3 send a number the build itself would never send: its number read finds the number first and refuses (see Code admits the trigger). The rows measure the guard behind that read, as raw requests to the lab book. They are not a path a release may send.

Every invoice import is the build's own request that created `BP/26-27/0010`, with two things changed: the number (in `VOUCHERNUMBER` and `REFERENCE`) and the REMOTEID. The party (`BRIDGE Walk-in`, not bill-wise), the date (1 Aug 2026) and the figures (1,051.00) are unchanged. gen.py asserts each change. The seed `I-0051.xml` sets the test up; it is not evidence of its own shape.

No Bridge build. Raw requests, one at a time, through `run.py`.
- It sends `Content-Type: text/xml; charset=utf-16`.
- It stops at a curl failure, and at an answer that says Unknown Request. It decodes each answer as UTF-16 little-endian whether or not it starts with a byte-order mark (Tally's answers have none), and also as UTF-8.
- Count every answer with an XML parser, never a text search.
- Close every other Bridge process first.

Files (UTF-16LE with a BOM, as sent; `SHA256.txt` lists them):
- `M-marks.xml`: the company high-water read, `lab/2162-s3b-sheet:lab/2162-s3b/M-marks.xml`, byte-equal.
- `N-0051.xml`, `N-lc0051.xml`, `N-z051.xml`, `N-0052.xml`: the number read for `BP/26-27/0051`, `bp/26-27/0051`, `BP/26-27/051` and `BP/26-27/0052`, over 1 Apr 2026 to 31 Mar 2027. Each is `lab/1342-capture-requests:lab/1342-capture-requests/r10-number-control-k0.xml` with the number changed.
- `I-0051.xml` (the seed, REMOTEID ending 0783), `I-0051b.xml` (b1, 0784), `I-lc0051.xml` (b2, 0785), `I-z051.xml` (b3, 0786), `I-0052.xml` (c, 0787).
- `R-0051.xml`, `R-lc0051.xml`, `R-z051.xml`, `R-0052.xml`: the read-back, `lab/2162-s3b-sheet:lab/2162-s3b/template-r-0010.utf16le.xml` (S1's R) with the number changed.
- `gen.py` makes `N-`, `I-` and `R-` again, from three templates:
  - `template-n-0001.utf16le.xml`: r10 above, byte-equal;
  - `template-import-0010.utf16le.xml`: `lab/2162-s3b-sheet:lab/2162-s3b/template-import-0010.utf16le.xml`, byte-equal;
  - `template-r-0010.utf16le.xml`: `lab/2162-s3b-sheet:lab/2162-s3b/template-r-0010.utf16le.xml`, byte-equal.

Steps (`python3 -I run.py <gateway URL> <label> <files>`):
1. `run.py URL s1 M-marks.xml`. Take BK. Read the Gateway's current date and write it down. If it is before 1-Aug-2026, stop: the invoices would be post-dated, which is S1's variable.
2. `run.py URL s2 N-0051.xml N-lc0051.xml N-z051.xml N-0052.xml R-0051.xml R-0052.xml`. If any N holds a row or any R holds a `VOUCHER` element, a number is in use: stop and report it.
3. The seed: `run.py URL s3 I-0051.xml`, then `run.py URL s3r R-0051.xml N-0051.xml M-marks.xml`. If the import is not CREATED 1 with ERRORS 0, or R does not hold exactly one voucher, stop at step 9.
4. a: `run.py URL s4 N-lc0051.xml N-z051.xml`.
5. b1: `run.py URL s5 I-0051b.xml`, then `run.py URL s5r R-0051.xml N-0051.xml M-marks.xml`.
6. b2: `run.py URL s6 I-lc0051.xml`, then `run.py URL s6r R-0051.xml R-lc0051.xml N-0051.xml N-lc0051.xml M-marks.xml`.
7. b3: `run.py URL s7 I-z051.xml`, then `run.py URL s7r R-0051.xml R-z051.xml N-0051.xml N-z051.xml M-marks.xml`.
8. c: `run.py URL s8 I-0052.xml`, then `run.py URL s8r R-0052.xml R-0051.xml N-0052.xml M-marks.xml`.
9. Restore BK. Run `run.py URL s9 M-marks.xml`, and check that its answer is byte-identical to step 1's.

Capture Tally's window after each import (s3, s5, s6, s7, s8): the Gateway, or a dialog.

**Stop**, with no key pressed, if Tally's window shows a dialog after any import, or the gateway stops answering. Screenshot the window, send the screenshot and the answers to D66, and ask before anything else is pressed or sent. A dialog would make the guard a failure inside Tally's screen rather than one in its answer (P7), and that decides the design before any count does.

Also stop at an import whose answer is none of these:
- CREATED 1 with ERRORS 0 and EXCEPTIONS 0;
- CREATED 0 with ERRORS 1 or EXCEPTIONS 1 (a decline in the answer).

A decline in the answer is a result: record it and go on. A b-row that creates a voucher is a result too: record it, then restore and seed as above before the next sub-row. If b1 creates one, restore, check M, and stop there: b2 and b3 then measure nothing the guard does.

Record, for each answer: bytes, sha256, and the counts below. Each count says what it counts.
- **M answers:** the company's `ALTVCHID` and `ALTMSTID`, the voucher and master alteration marks.
- **Every N answer:** the number of rows with a `VOUCHERNUMBER` child, and for each row its `VOUCHERNUMBER`, `DATE`, `VOUCHERTYPENAME`, `ISCANCELLED` and `BRIDGEVCHISSALES`.
- **Every I answer:** `CREATED`, `ALTERED`, `ERRORS`, `EXCEPTIONS` and `LASTVCHID`, and the text of each `LINEERROR`.
- **Every R answer:** the number of `VOUCHER` elements, and for each voucher its `VOUCHERNUMBER`, `DATE`, `ALTERID` and the last 8 characters of its `GUID`.

Expected:
- **s2:** every N holds 0 rows and both R hold 0 `VOUCHER` elements (the numbers are unused).
- **s3:** CREATED 1, ERRORS 0. R-0051 holds 1 voucher, `BP/26-27/0051`, dated 20260801. N-0051 holds 1 row, `BRIDGEVCHISSALES` Yes. M's `ALTVCHID` is above s1's.
- **s4, s5 to s7:** not predicted; this is the row's question. Record:
  - s4: how many rows each N finds, and under which `VOUCHERNUMBER`.
  - s5 to s7: whether the import is declined in the answer or creates a voucher; and how many vouchers R-0051 and the sub-row's own R then hold.
- **s8:** CREATED 1, ERRORS 0. R-0052 holds 1 voucher, and R-0051 still holds 1.
- **s9:** the marks answer byte-identical to s1's.

An empty R or N after a creating import means the filter did not find the voucher. Report the number sent and the number on Tally's screen. That is not a finding about the guard.

Decides:
- **b1 declined in the answer:** the guard holds on a Sales type, in-band, and ADR item 12's first sub-item is measured. **b1 creates a voucher:** the guard does not hold on Sales. The number read is then the only defence, and Sales stays off until the build has another one.
- **a against b2 and b3:**
  - Where the read finds a variant (its s4 answer holds 1 row), the build refuses that variant before sending. That is safe.
  - Where the read misses a variant and the guard declines it, the build admits a number Tally then declines, in-band. The build should then fold the number the way the guard does, or refuse a number that differs from one in use only that way.
  - Where both miss and the import creates a voucher, Tally holds two invoices whose numbers differ only by case or zeros. Whether GST allows that is decided before any code.
- **c creates and R-0051 still holds 1:** two invoices that differ only by number are two vouchers in Tally, as Bridge's identity check already assumes (`two_invoices_that_differ_only_by_their_number_are_two_documents`, agent_import_number_identity_tests.rs:172).
- **A dialog anywhere:** the row stops (see Stop).

Code admits the trigger: each request's sender at file:line, and the branch that decides whether it is sent. Lines are at master 4c7b8db6.
- **N, the number read:**
  - Rendered by `render_invoice_number_request` (agent_import_invoice_wire.rs:426). Its filter is `$VoucherNumber = "{number}"` (:433), over the financial year, for vouchers of any type. Its class compute is `$$IsSales:$VoucherTypeName`.
  - Wrapped by `invoice_number_read` (agent_read_profiles.rs:673). The invoice build asks for it at agent_import_invoice.rs:2270-2272, once the type has resolved (:2266) and while the number read has no answer. It renders it at :2099-2108 (`Need::Number`).
  - Its answer is counted by `count_sales_vouchers` (agent_import_invoice_wire.rs:445-455). Any row of Sales class refuses the build as `invoice_number_already_used` (agent_import_invoice.rs:2274-2279).
  - r10 was sent live for #1342. A render of the literal at :433 with r10's company, window and number is byte-equal to it (1,682 bytes, sha256 90d188e7…). So each N here is that render with the number changed.
  - The number admitted is ASCII letters, digits, `/` and `-`, up to 16 characters (`InvoiceNumber::parse`, agent_import_invoice.rs:560-566). That makes `bp/26-27/0051` and `BP/26-27/051` both numbers a caller can supply.
- **I, the invoice:**
  - Rendered by `render_sales_invoice_xml` (agent_import_invoice.rs:2637), from agent_import.rs:5565 when `voucher_type.is_invoice()`. It is sent as the native post (`native_post_request`, agent_import_post.rs:3073, built at :945).
  - Whether a Sales post is sent at all is decided by `LIVE_QUALIFIED_VOUCHER_TYPES` (agent_import.rs:390). That list does not include Sales, so no release sends this request.
  - In b1 to b3 the build would refuse at its number read first (above). The row sends them raw to measure the guard the build relies on.
- **The guard:**
  - `resolve_voucher_type` admits a Sales type only with Manual numbering and `PREVENTDUPLICATES` Yes on its one series (agent_import_invoice_wire.rs:383-387). Its comment says the flag is not measured on Sales (:377-382).
  - The committed answer `pilot-lab/pilot-lab-numbering-series.utf16le.xml` (16 Oct 2026) shows `BRIDGE Sales`, the type the invoice names, with its Default series Manual and `PREVENTDUPLICATES` Yes. The row does not read the type again. If b1 creates a voucher, send the answers to D66 before going on: the type may have changed since, and a read of its numbering is then a row of its own.
- **R, the read-back:** rendered by `render_invoice_readback_request` (agent_import_invoice_wire.rs:484). It is sent by `read_back_sales_invoice` (agent_import_invoice.rs:1683) from agent_import.rs:2431, when `invoice_readback_due` (agent_import_verification.rs:942, called at agent_import.rs:2425) finds an invoice in the batch.
- **M, the marks:** `render_agent_company_high_water` (agent_read_profiles.rs:567), sent as `company_high_water_read` before a post (agent_import.rs:3250, `current_voucher_mark`).

Row: lab/2162-s4/
