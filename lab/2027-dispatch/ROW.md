# Row 2027-D: does the vouchers window read carry a voucher's dispatch details when they are named?

Not for merge. Part of #2027. Lines are at master a4b4adff.

**The question.** R2 of `lab/2162-irn-read-answers` (9a20049b) keyed dispatch details on one Credit Note and found each value in one element of a whole-voucher read. No one has sent the vouchers window read with those elements named. This row sends it, once with and once without them, against one Sales invoice and one Credit Note that carry the details. It also finds where a Sales invoice keeps them, and a Delivery Note number if the Sales screen offers one.

**Book.** `BRIDGE EINVOICE LAB`, GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`, TallyPrime 7.2 Silver, the book of `lab/2162-irn-read`. That row restored it to its state before it keyed anything, then shut it when `BRIDGE FOREXGROUP LAB` was loaded for the `683-build` row. It holds three vouchers, all dated 3-Apr-25: Sales `1` (with e-invoice details), Sales `2`, and a Payment `1`. Its marks are `ALTMSTID` 215 and `ALTVCHID` 3.

**No Bridge build.** Every request is raw, sent one at a time through `run.py`, which is `lab/2162-irn-read/run.py` byte-equal.
- `run.py` sends `Content-Type: text/xml; charset=utf-16`.
- It stops at a curl failure, and at an answer that says Unknown Request.
- `count.py` counts every answer with an XML parser, and it prints only tags, counts, lengths, yes or no, and the values this sheet names.
- Record any other Bridge process. None of them may take a call.

**Output: shapes and counts only.** `RESULTS.md` carries:
- the whole output of `python3 -I count.py`;
- the release on Tally's screen;
- one line per step saying it was done as written, or what differed.

The answers, the screenshots and the backup stay on the lab PC. List each by sha256, bytes and name in `kept-local.txt`. The full reads and the screens hold the book's invented GSTINs.

## Files
UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file.
- `M-marks.xml` and `L-list.xml`: `lab/2162-irn-read`'s, byte-equal. M reads the company's marks; L lists every voucher.
- `W.xml`: the vouchers window read the build sends with `include_gst_details` (`AGENT_VOUCHER_FETCH` then `AGENT_VOUCHER_GST_WORDS`, agent_read_profiles.rs:47-59), for 3 Apr 2025 on this book. It is `lab/2027-voucher-gst-fields-fetch`'s candidate request with only its company and day changed. That sheet's request was answered on 30 Oct, and on 3 Nov a build sent the same words.
- `WD.xml`: W with seven words added after the six: `BASICSHIPDOCUMENTNO, BASICSHIPPEDBY, BASICFINALDESTINATION, EICHECKPOST, BILLOFLADINGNO, BILLOFLADINGDATE, BASICSHIPVESSELNO`. These are the elements R2 found holding the note's values. Nothing else changes.
- `F-sales.xml` and `F-note.xml`: 1378's `reqs/full1.xml` (every native method, no fetch list), filtered to the vouchers of type Sales and of type Credit Note. F-note is `lab/2162-irn-read/F-note.xml`, byte-equal.
- `gen.py` makes W, WD and both F files again from `template-a07.utf16le.xml` and `template-full-1.utf16le.xml`, and asserts each change:
  - `template-a07.utf16le.xml` is `lab/2027-voucher-gst-fields-fetch` at 7be9ae07, `templates/a07-vouchers-shape-20250424.xml`, byte-equal.
  - `template-full-1.utf16le.xml` is 1378's `reqs/full1.xml`, byte-equal.

## Prompts this row expects
These, and no others:
- **"Accept ?"** when the backup is taken (step 1): y.
- **"Accept ?"** on saving each voucher (steps 2 and 3): y.
- **"Do you want to generate e-Invoice?"** after each save: n. If Tally then opens a blank voucher of the same type, leave it with Escape and accept the quit, saving nothing.
- **"Accept ?"** at the restore, then **"Company data folder (…) already exists"**: Replace (r) (step 5).

Any other prompt is a stop (see Stop).

## Steps (`python3 -I run.py <gateway URL> <label> <files>`)
0. **Load the book.** Shut `BRIDGE FOREXGROUP LAB` with no change to it. Then load `BRIDGE EINVOICE LAB` alone: Select Company, the book by its name filter, then Enter. Take a screenshot of the Gateway.
1. **Check the book.** Run `run.py URL s1 M-marks.xml L-list.xml`, then `count.py`. It must show:
   - GUID is the book's: yes;
   - `ALTMSTID` 215 and `ALTVCHID` 3;
   - L with three rows, all 20250403: `1` Sales, `2` Sales, `1` Payment.

   Then take a BK on Tally's Backup screen: one company, to a new empty folder, "Accept ?" y. Record its file's bytes and sha256 in `kept-local.txt`. Write down the release and the Gateway's date and period.
2. **The Sales invoice.** Key one Sales voucher in the invoice mode Tally opens, as for invoice B:
   - dated 3-Apr-25, party `BRIDGE IRN Customer`;
   - `BRIDGE IRN Sales` 100.00 and `BRIDGE IRN IGST` 18.00 typed;
   - narration blank. Leave `Provide GST/e-Invoice details` No, and press nothing under e-Invoice.

   In the dispatch section (Tally may title it "Dispatch Details"), fill each field Tally shows, by its label:

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
   - A dispatch label the table does not name (for example `Tracking No(s)`): leave it blank and record its name.
   - Order details, Party Details and the rest: accept what Tally offers.

   Screenshot the dispatch section before saving. Save: "Accept ?" y, then "Do you want to generate e-Invoice?" n. Record the voucher number Tally gave it.
3. **The Credit Note.** Key it as R2 of `lab/2162-irn-read` did, using that row's RESULTS.md, "The Credit Note as keyed":
   - dated 3-Apr-25, party `BRIDGE IRN Customer`;
   - Original Invoice No. `2`, date 3-4-2025;
   - `BRIDGE IRN Sales` 100.00 and `BRIDGE IRN IGST` 18.00;
   - `Provide GST/e-Invoice details` set to No;
   - the same table of dispatch values as step 2, in the "Receipt Details" sub-screen. Delivery Note was not offered there in R2; if it is now, key `DN-71`.

   Screenshot the dispatch section before saving. Save: "Accept ?" y, then "Do you want to generate e-Invoice?" n.
4. **The reads.** Run `run.py URL r1 L-list.xml W.xml WD.xml F-sales.xml F-note.xml M-marks.xml`, then `count.py`.
5. **Undo.** Shut the company, then restore the step 1 BK: Data > Backup & Restore > Restore, Specify Path, the BK folder. "Accept ?" y, then "already exists": Replace (r). Load the book alone. Run `run.py URL s9 M-marks.xml L-list.xml` and `count.py`.
   - M's values must equal s1's: the GUID, `ALTVCHID` and `ALTMSTID`.
   - L must hold s1's three rows.

   This restore undoes the sitting's two writes. Nothing else in the book changes.

## Stop
**Stop with no key pressed** if any of these happen:
- a prompt appears that this sheet does not name;
- the gateway stops answering;
- a screen offers an action that would reach the portal.

Screenshot the window, keep the answers, and ask before anything else is pressed or sent. Never press any of these: `Get IRN Info`, `Manually Update IRN Info`, `F10: Cancel IRN`, `Open e-Invoice Portal`, `Set GST Status`, `Fetch Details Using GSTIN/UIN`, e-Way Bill actions, or any send action.

Also stop in these cases:
- **Step 0 or 1:** any check fails, or the book is not in Tally's list. Nothing has been keyed: report and end.
- **Step 2 or 3:** the voucher's screens show no dispatch section. Escape without saving, accept the quit, record it, and go on to the next step. If neither voucher was saved, go to step 5.
- **`run.py` stops:** do not resend. Go to step 5.
- **Step 5:** the checks fail. Key nothing more and report.

An answer that differs from Expected is a result: record it and go on.

## Record
`count.py` prints all of the following.
- **M:** whether the GUID is the book's, and `ALTVCHID` and `ALTMSTID`.
- **L:** each row's number, date and type.
- **W and WD:** for each row, its number, type and count of direct children, and each of the seven dispatch elements with its count, `TYPE` attribute and value.
- **WD against W,** the three things this row measures:
  1. Each of the seven appears once in every row ("each dispatch word once in every row").
  2. Each row is otherwise equal to W's: every other child, in order, with its attributes, text and descendants ("rows otherwise equal").
  3. The whole answer is equal to W's once the seven are removed ("whole answer otherwise equal").
- **F-sales and F-note,** for each voucher:
  - its number, its type, and its count of direct children and of all elements;
  - for each keyed value, how many elements hold it, and at which paths;
  - every non-empty direct child whose name contains SHIP, DISPATCH, DESTINATION, CARRIER, LADING, VEHICLE, DELIVERY, TRANSPORT or CHECKPOST, and every such `.LIST` child with its element count.
- **Every answer:** its bytes and the count of GSTIN shapes in it.

## Expected
- **s1:** GUID yes, `ALTMSTID` 215, `ALTVCHID` 3. L holds three rows.
- **r1:**
  - L holds five rows, the two new ones a Sales and a Credit Note dated 20250403. M's `ALTVCHID` is above s1's.
  - W and WD hold five rows each.
  - R2 predicts, for the Credit Note's row in WD:
    - `BASICSHIPDOCUMENTNO` `DD-72`, `BASICSHIPPEDBY` `ROADWAY`, `BASICFINALDESTINATION` `Mysuru`;
    - `EICHECKPOST` `CARRIER-73`;
    - `BILLOFLADINGNO` `LR-74`, with `BILLOFLADINGDATE` 20250403;
    - `BASICSHIPVESSELNO` `KA-75`.

    R2 read these in a whole-voucher read, not in this request.
  - Where the Sales invoice keeps its values, and a Delivery Note number, is not predicted; that is this row's second question.
  - F-sales holds three vouchers of type Sales, and F-note one of type Credit Note.
- **s9:** M's values equal s1's, and L holds s1's three rows.

## Decides
- **Each word once in every row, and both "otherwise equal" yes:** the window read can carry the seven words with no change to what it already answers. Building them into `vouchers` behind its own opt-in argument is then measured, for the rows this book holds.
- **The Sales values land in the same seven elements as the note's:** one mapping serves both. If any lands elsewhere, record it; that field waits.
- **A Delivery Note number lands in exactly one element or list:** that is its home, and a follow-up row sends it in the window read. If it lands in none, or the field is not offered, record that.
- **A word is absent or repeated, or "otherwise equal" is no:** the code waits, and the answer decides it.

## Code admits the trigger
Lines are at master a4b4adff.
- **W:** the vouchers window read with `include_gst_details`. The list is `AGENT_VOUCHER_FETCH` (agent_read_profiles.rs:47), then `AGENT_VOUCHER_GST_WORDS` (:58), appended at :187 when `VoucherGstFields::following` (agent_voucher_parse.rs:399) asks for them. One day, one company, as the build sends it.
- **WD:** a candidate shape, W with seven fixed words. No value chooses them, and no release sends it. Under AGENTS.md P7, a new shape is first sent only to a lab book; this row is that sending.
- **M:** `render_agent_company_high_water` (agent_read_profiles.rs:567), as #1378 sent it.
- **L and F:** lab reads for shape, which the build does not send.
- **The two vouchers:** keyed by hand on Tally's screen. No Bridge request writes them.

Row: lab/2027-dispatch/
