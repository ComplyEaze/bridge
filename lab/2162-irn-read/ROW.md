# Row 2162-IRN: does the number read carry an invoice's e-invoice details, and where does Tally keep a note's dispatch details?

Two rows, one sitting, on one book.
- **R1, reads only (#2162):** the build's number read with `IRN`, `IRNACKNO` and `IRNACKDATE` added to its fetch list, against one invoice with the details and one without, with the plain read as a control.
- **R2, one write (#2027):** dispatch details filled on one Credit Note by hand, then that note read in full.

**Book.** `BRIDGE EINVOICE LAB`, GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`: the book of `lab/1378-irn-capture-answers` (183ae58c). It was made new for #1378 on 2 Nov 2026, with masters from one import of invented values and finished by hand. `lab/1378-read-answers` (95b0a2e5) then left it holding three vouchers, all dated 3-Apr-25:
- invoice A, Sales 1, with the e-invoice details;
- invoice B, Sales 2, without them;
- a Payment, numbered 1.

Its marks were then V 3, M 215. TallyPrime 7.2 Silver, as in #1378. Load it alone.

**No Bridge build.** Every request is raw, sent one at a time through `run.py`, which is `lab/2162-s4-sheet:lab/2162-s4/run.py` byte-equal.
- `run.py` sends `Content-Type: text/xml; charset=utf-16`.
- It stops at a curl failure, and at an answer that says Unknown Request.
- `count.py` counts every answer with an XML parser, and it prints only tags, counts, lengths, yes or no, and the values this sheet names.
- Record any other Bridge process. None of them may take a call.

**Output: shapes and counts only.** `RESULTS.md` carries:
- the whole output of `python3 -I count.py`;
- the `log-*.txt` files;
- the release on Tally's screen;
- one line per step saying it was done as written, or what differed.

The answers and screenshots stay on the lab PC. List each by sha256, bytes and name in `kept-local.txt`. The full read and the screens hold the book's invented GSTINs.

## Files
UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file.
- `M-marks.xml`, `L-list.xml` and `V-named.xml` are `lab/1378-irn-capture-answers` `reqs/marks.xml`, `reqs/vlist.xml` and `reqs/named.xml`, byte-equal:
  - M reads the company's marks;
  - L lists every voucher;
  - V is the vouchers window read with the three elements named.
- `N-1.xml`, `N-2.xml`: the number read for numbers `1` and `2`, over 1 Apr 2025 to 31 Mar 2026, as the build renders it.
- `NI-1.xml`, `NI-2.xml`: N with `IRN, IRNACKNO, IRNACKDATE` added at the end of its `FETCH`. Nothing else changes.
- `F-note.xml`: `reqs/full1.xml` (every native method, no fetch list) with its filter changed to `$VoucherTypeName = "Credit Note"`.
- `gen.py` makes N, NI and F again from `template-n-0001.utf16le.xml` (`lab/2162-s4-sheet:lab/2162-s4/`, byte-equal) and `template-full-1.utf16le.xml` (1378's `reqs/full1.xml`, byte-equal). It asserts each change.

## Steps (`python3 -I run.py <gateway URL> <label> <files>`)
1. **Check the book.** `run.py URL s1 M-marks.xml L-list.xml V-named.xml`, then `count.py`. It must show:
   - GUID is the book's: yes;
   - `ALTMSTID` 215, with `ALTVCHID` recorded;
   - L with three rows, all 20250403: `1` Sales, `2` Sales, and `1` of another type;
   - in V-named:
     - Sales `1` with `IRN` of length 64, `IRNACKNO` `112233` and `IRNACKDATE` 20250403;
     - Sales `2` and the Payment `1` with all three present and empty;
     - 3 rows in all.

   Then take BK on Tally's screen. Write down the release and the Gateway's date and period.
2. **No set-up.** Step 1 found invoice A (Sales 1, with the details) and invoice B (Sales 2, without them), so nothing is keyed.
3. **R1.** Run `run.py URL r1 N-1.xml NI-1.xml N-2.xml NI-2.xml V-named.xml M-marks.xml`. Then open A in Display from the Day Book, with no Alter. Screenshot the e-Invoice details if that screen shows them, and if it does not, say so.
4. **R2, the one write.** Key one Credit Note in invoice mode, as Tally offers it:
   - dated 3-Apr-25, party `BRIDGE IRN Customer`;
   - `BRIDGE IRN Sales` 100.00 and `BRIDGE IRN IGST` 18.00 typed;
   - against invoice 2 of 3-Apr-25 where Tally asks for the original invoice, with reason `01-Sales Return` where it asks for one.

   In the dispatch section, fill each field Tally shows, by its label:

   | label | value |
   |---|---|
   | Delivery Note No(s) | `DN-71` (date, if asked, 3-Apr-25) |
   | Dispatch Doc No. | `DD-72` |
   | Dispatched through | `ROADWAY` |
   | Destination | `Mysuru` |
   | Carrier Name/Agent | `CARRIER-73` |
   | Bill of Lading/LR-RR No. | `LR-74`, date 3-Apr-25 |
   | Motor Vehicle No. | `KA-75` |

   - A label in the table that the screen does not show: record it as not offered.
   - A dispatch label the table does not name: leave it blank and record its name.
   - Leave `Provide e-Invoice details` No.

   Screenshot the dispatch section before saving. On save, "Do you want to generate e-Invoice?": press n.

   Then `run.py URL r2 L-list.xml F-note.xml M-marks.xml`.
5. **Undo.** Close the company, then restore the BK taken at step 1 by #1378's path: Data > Backup & Restore > Restore, the BK folder, Accept, Replace. Load the book alone. Then run `run.py URL s9 M-marks.xml L-list.xml` and `count.py`.
   - M's values must equal s1's: the GUID, `ALTVCHID` and `ALTMSTID`. Values, not bytes: #1378's marks answers before and after its restore held the same three values with different sha256.
   - L must hold the same three rows as s1.

   This restore is the undo of R2's note, the sitting's one write. Nothing else in the book changes.

## Stop
**Stop with no key pressed** if any of these happen:
- a dialog appears that this sheet does not name;
- the gateway stops answering;
- a screen offers an action that would reach the portal.

Screenshot the window, keep the answers, and ask before anything else is pressed or sent. Never press any of these: `Get IRN Info`, `Manually Update IRN Info`, `F10: Cancel IRN`, `Open e-Invoice Portal`, `Set GST Status`, `Fetch Details Using GSTIN/UIN`, or any send action.

Also stop in these cases:
- **Step 1:** any check fails, or the book is not in Tally's list. Nothing has been keyed: report and end.
- **`run.py` stops:** do not resend. Go to step 5.
- **Step 3:** M's `ALTVCHID` or `ALTMSTID` differ from s1's, so a read changed the book. Go to step 5 and report.
- **Step 4:**
  - The note's screens show no dispatch section. Escape without saving, accept the quit, record it, and go to step 5.
  - F-note holds other than one voucher. Record it and go to step 5.
- **Step 5:** the checks fail. Key nothing more and report.

An N whose rows differ from Expected is a result: record it and go on.

## Record
`count.py` prints all of the following.
- **M:** whether the GUID is the book's, and `ALTVCHID` and `ALTMSTID`.
- **L:** each row's number, date and type.
- **N, NI, V:** for each row, each of these tags with its count and `TYPE` attribute:
  - `DATE`, `VOUCHERNUMBER`, `VOUCHERTYPENAME`, `ISCANCELLED`, `BRIDGEVCHISSALES`: each value;
  - `IRN`: its length;
  - `IRNACKNO`, `IRNACKDATE`: each value.
- **Each NI against N and V,** the three things this row measures:
  1. Each of the three elements appears once in every row ("each IRN tag once in every row").
  2. Its values equal V's for the same number ("equal on"), and the values keyed in step 2.
  3. The answer is otherwise equal to the plain read's ("otherwise equal": every other child of every row, in order, with its attributes and text).
- **F:**
  - the voucher's number, its type, and its count of direct children and of all elements;
  - for each value keyed in step 4, how many elements hold it, and at which paths;
  - every non-empty direct child whose name contains SHIP, DISPATCH, DESTINATION, CARRIER, LADING, VEHICLE, DELIVERY or TRANSPORT.
- **Every answer:** its bytes and the count of GSTIN shapes in it.

## Expected
- **s1:** GUID yes, `ALTMSTID` 215. L holds three rows. V-named holds 3 rows: A with its details, and B and the Payment with the three elements present and empty.
- **r1:**
  - N-1 holds 2 rows: Sales `1` with `BRIDGEVCHISSALES` Yes, and the Payment `1` with No. The number read matches a number in any type.
  - N-2 holds 1 row, Sales `2`, with Yes.
  - For NI-1's Sales row, #1378's V answer predicts, but does not prove:
    - `IRN` once, `TYPE` String, length 64;
    - `IRNACKNO` once, String, `112233`;
    - `IRNACKDATE` once, `TYPE` Date, 20250403.
  - NI-2 has all three once and empty.
  - Both NI are equal on the three tags with V, and otherwise equal to N.
  - V predicts that NI-1's Payment row carries all three, present and empty. If it does not, "each IRN tag once in every row" prints no: read the per-row lines.
  - M's two marks are the same as in s1.
- **r2:** L holds four rows, the new one a Credit Note dated 20250403. F holds one voucher of type Credit Note. Where each keyed value lands is not predicted; that is this row's question. M's `ALTVCHID` is above r1's.
- **s9:** M's values equal s1's, and L holds s1's three rows.

## Decides
- **R1:**
  - **All three hold on both Sales rows.** The number read can carry the e-invoice details with no change to what it already answers. Adding the three tags to `render_invoice_number_request` is then measured.
  - **A tag is absent or repeated, or "otherwise equal" is no.** The code change waits, and the answer decides it.
- **R2:**
  - **A value lands in exactly one element.** That element is the field's home, and #2027's dispatch row takes its name.
  - **It lands in none.** The native read does not carry that field, and it needs a read of its own.
  - **It lands in several.** Record them all, and nothing is decided on that field.

## Code admits the trigger
Lines are at master ec44e957.
- **N, the number read:**
  - Rendered by `render_invoice_number_request` (agent_import_invoice_wire.rs:426, literal at :433) and wrapped by `invoice_number_read` (agent_read_profiles.rs:673).
  - The invoice build asks for it as `Need::Number` (agent_import_invoice.rs:2113-2118), once the Sales type has resolved and the number read has no answer (:2283-2286).
  - `count_sales_vouchers` (agent_import_invoice_wire.rs:445) counts the answer, and any Sales row refuses the build as `invoice_number_already_used` (:2287-2293).
  - `InvoiceNumber::parse` (agent_import_invoice.rs:560) admits `1` and `2`.
  - N-1 and N-2 are byte-equal to the literal at :433, rendered with this book's name, the year 1 Apr 2025 to 31 Mar 2026, and those numbers. gen.py builds them from r10, a render of the same literal sent live for #1342.
- **NI:**
  - It is a candidate shape: N with three tags added to its fixed fetch list. No value chooses them.
  - No release sends it. Under AGENTS.md P7, a new shape is first sent only to a lab book, and this row is that sending.
- **M:** `render_agent_company_high_water` (agent_read_profiles.rs:567), as #1378 sent it.
- **L, V and F:** lab reads for shape, which the build does not send. V is the vouchers window read with the three tags named, which #1378 sent.
- **The invoices and the note:** keyed by hand on Tally's screen. No Bridge request writes them.

Row: lab/2162-irn-read/
