# Row 2027-DN2: do the e-invoice, voucher-type and combined window reads carry the Delivery Note list as the plain read does?

Not for merge. Part of #2027. Code lines are at 9a415553, the head of the #2027 Delivery Note change.

**The question.** Row 2027-DN (`lab/2027-dn-answers` at 81c551da) keyed two Sales invoices, one with one Delivery Note and one with two. It sent the plain window read with the Delivery Note list named (GDN). Each note came back as one `INVOICEDELNOTES.LIST` entry, holding its date in `BASICSHIPPINGDATE` and its number in `BASICSHIPDELIVERYNOTE`, in the order keyed. Every other row carried one empty list, and the answer was otherwise GD's. The change for #2027 names the list in every read that names the GST words. Three of those reads have been sent with the seven dispatch words only, never with the list (`lab/2027-d2-answers`):
- the read with `include_e_invoice` (EGD);
- the read with a voucher-type filter (CGD);
- the read with both (CEGD).

This row keys the same two invoices again. It sends each of the three reads with the list and without it, and answers one thing: whether each read carries the notes as GDN did and is otherwise unchanged.

**Book.** `BRIDGE EINVOICE LAB`, GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`, TallyPrime 7.2 Silver, as row 2027-DN left it ("The book now", `lab/2027-dn-answers` RESULTS.md):
- restored to its state before that sitting;
- three vouchers, all dated 3-Apr-25: Sales `1` (with e-invoice details), Sales `2`, and a Payment `1`;
- `ALTVCHID` 3 and `ALTMSTID` 215;
- loaded alone at the Gateway, one tally.exe on 9001.

No voucher in it holds a dispatch detail or a Delivery Note.

**No Bridge build.** Every request is raw, sent one at a time through `run.py`, which is `lab/2027-d2/run.py` byte-equal.
- `run.py` sends `Content-Type: text/xml; charset=utf-16`.
- It stops at a curl failure, and at an answer that says Unknown Request.
- `count.py` counts every answer with an XML parser, and it prints only tags, counts, lengths, yes or no, and the values this sheet names.
- Record any other Bridge process. None of them may take a call.

**Output: shapes and counts only.** `RESULTS.md` carries:
- the whole output of `python3 -I count.py --company-number N` after s9;
- the release on Tally's screen;
- one line per step saying it was done as written, or what differed.

N is the book's company number, the number in its data folder's name. It goes on the command line only. It is never written into `RESULTS.md`, `kept-local.txt`, a file name or a commit.

The answers, the screenshots and the backup stay on the lab PC. List each by sha256, bytes and name in `kept-local.txt`. The window reads and the screens hold the book's invented GSTINs, and the e-invoice reads hold Sales `1`'s invented IRN.

## Files
UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file. Each read is for `BRIDGE EINVOICE LAB`, and each window read is for 3 Apr 2025 alone, the day both new vouchers are dated.
- `M-marks.xml` and `L-list.xml`: `lab/2027-d2`'s, byte-equal. M reads the company's marks; L lists every voucher.
- `EGD.xml`, `CGD.xml` and `CEGD.xml`: `lab/2027-d2`'s, byte-equal (sha256 e182bb24…, 05b10dca… and 1b72a961…). Each is the request master renders today for `vouchers` with `include_gst_details` and, in turn, `include_e_invoice`, a voucher-type filter, or both. They carry the seven dispatch words and no list.
- `EGDN.xml`, `CGDN.xml` and `CEGDN.xml`: each of the three with `,INVOICEDELNOTES.*` added after the seven dispatch words and nothing else changed. Each is the request the #2027 Delivery Note change renders for the same call (see Code admits the trigger).
- `run.py`: `lab/2027-d2/run.py`, byte-equal.
- `gen.py` makes no request. It asserts each earlier file by its sha256, and that each `…N.xml` is its pair with only the list added. Run it before step 0; any failure is a stop.
- `count.py`: `lab/2027-dn/count.py`, with its last part comparing the three pairs below in place of GDN against GD. See Record.

## Prompts this row expects
These, and no others, each answered as written:
1. **"Accept ?"** on the Backup screen (step 1): y.
2. **"Accept ?"** on saving each Sales voucher (steps 2 and 3): y.
3. **"Do you want to generate e-Invoice?"** after each save: n.
4. **A blank Sales voucher** if Tally opens one after that: Escape. Then **"Quit ?"**: y. Nothing is saved.
5. **"Accept ?"** at the restore (step 5): y.
6. **"Company data folder (<company number>) already exists"** at the restore: Replace (r).

The Delivery Note line asks for a date in the voucher itself (2027-D). That is a field, not a prompt. Steps 2 and 3 say what goes in it. Any other prompt is a stop (see Stop).

## Steps (`python3 -I run.py <gateway URL> <label> <files>`)
0. **The book.** Run `python3 -I gen.py`; it must print `checked: run.py M-marks.xml L-list.xml EGD.xml EGDN.xml CGD.xml CGDN.xml CEGD.xml CEGDN.xml`. Then:
   - If the Gateway shows `BRIDGE EINVOICE LAB` loaded alone, change nothing.
   - If another book is loaded with it, or it is not loaded, stop and ask before anything else is pressed.

   Take a screenshot of the Gateway.
1. **Check the book.** Run `run.py URL s1 M-marks.xml`, then `count.py --company-number N`; then `run.py URL s1 L-list.xml`, then `count.py --company-number N`. It must show:
   - GUID is the book's: yes;
   - `ALTMSTID` 215 and `ALTVCHID` 3;
   - L with three rows, all 20250403: `1` Sales, `2` Sales, `1` Payment;
   - the content checks pass on both answers.

   Then take a BK on Tally's Backup screen: one company, to a new empty folder, "Accept ?" y. Record its file's bytes and sha256 in `kept-local.txt`. Write down the release and the Gateway's date and period.
2. **Sales A, one Delivery Note.** Key one Sales voucher in the invoice mode Tally opens, exactly as row 2027-DN's step 2:
   - dated 3-Apr-25, party `BRIDGE IRN Customer`;
   - `BRIDGE IRN Sales` 100.00 and `BRIDGE IRN IGST` 18.00 typed;
   - narration blank;
   - `Provide GST/e-Invoice details` No. Press nothing under e-Invoice.

   In the dispatch section ("Dispatch Details"), fill only these:
   - `Delivery Note No(s)`: `DN-81`;
   - its `Date`: change what Tally offers to 1-Apr-25;
   - the second Delivery Note line that follows: leave it empty and move on.

   Leave every other dispatch field blank. Accept what Tally offers for Order details, Party Details and the rest.

   Screenshot the dispatch section before saving. Then answer prompts 2, 3 and 4. Record the voucher number Tally gave it (expected `3`).
3. **Sales B, two Delivery Notes.** The same as step 2, with these Delivery Note lines:
   - line 1: `DN-82`, date 2-Apr-25;
   - line 2: `DN-83`, date 1-Apr-25;
   - line 3: leave it empty and move on.

   Screenshot the dispatch section before saving, then answer prompts 2, 3 and 4. Record the voucher number (expected `4`).
4. **The reads.** One request per `run.py` call, label `r1`, with `count.py --company-number N` after each, and each check passing before the next is sent. In this order:
   1. `L-list.xml`
   2. `EGD.xml`
   3. `EGDN.xml`
   4. `CGD.xml`
   5. `CGDN.xml`
   6. `CEGD.xml`
   7. `CEGDN.xml`
   8. `M-marks.xml`
5. **Undo.**
   - Shut the company, then restore the step 1 BK: Data > Backup & Restore > Restore, Specify Path, the BK folder. Answer prompts 5 and 6.
   - Load the book alone.
   - Run `run.py URL s9 M-marks.xml`, then `count.py --company-number N`; then `run.py URL s9 L-list.xml`, then `count.py --company-number N`.
   - The s9 M and L answers must be byte-identical to s1's (compare their sha256).

   This restore undoes the sitting's two writes. Nothing else in the book changes.

## Stop
**Stop with no key pressed** if any of these happens:
- a prompt or screen appears that this sheet does not name;
- the gateway stops answering;
- a screen offers an action that would reach the portal.

Screenshot the window, keep the answers, and ask before anything else is pressed or sent. Never press any of these: `Get IRN Info`, `Manually Update IRN Info`, `F10: Cancel IRN`, `Open e-Invoice Portal`, `Set GST Status`, `Fetch Details Using GSTIN/UIN`, e-Way Bill actions, or any send action. Save nothing beyond the two Sales vouchers.

**Content checks, for every answer.** `count.py` prints, for each answer, a `content checks:` line, and at the end `all answers parse and pass the content checks`. Each answer must hold:
- **10 or more digits in a row:** only inside `VOUCHERKEY`, where Tally writes a 15-digit run. A run anywhere else is a stop, and so is a run the parser cannot place in an element.
- **GSTIN shapes:** only the lab's invented all-Z family, `\d{2}ZZZZZ0000Z[A-Z\d]Z[A-Z\d]`. Any other is a stop.
- **PAN shapes:** only `ZZZZZ0000Z`. Any other is a stop.
- **The company number N:** 0 hits as a whole number, in the answer read as UTF-16LE and in its bytes read as UTF-8. Any hit is a stop.

A content stop means: send nothing more, keep the answer on the lab PC only, and go to step 5. Report the counts `count.py` printed, never the value.

Also stop in these cases:
- **`gen.py`, step 0 or step 1:** any check fails, or the book is not in Tally's list. Nothing has been keyed: report and end.
- **Step 2 or 3:** the screen shows no Delivery Note field. Escape without saving, accept the quit, record it, and go on. If neither voucher was saved, go to step 5.
- **`run.py` stops, or an answer does not parse:** do not resend. Go to step 5.
- **Step 5:** the checks fail. Key nothing more and report.

An answer that differs from Expected is a result: record it and go on.

## Record
`count.py` prints all of the following.
- **M:** whether the GUID is the book's, and `ALTVCHID` and `ALTMSTID`.
- **L:** each row's number, date and type.
- **EGD, EGDN, CGD, CGDN, CEGD and CEGDN,** for each row:
  - its number, its type, its count of direct children and of all elements, and its count of `INVOICEDELNOTES.LIST`;
  - for each of `DN-81`, `DN-82`, `DN-83`, `20250401` and `20250402`: how many elements hold it, and at which paths;
  - for each `INVOICEDELNOTES.LIST`, in order: its children and elements, and each descendant's tag, `TYPE` attribute and value. Values this sheet does not name print as their length.
- **EGDN against EGD, CGDN against CGD, and CEGDN against CEGD:**
  - the rows in each;
  - whether every row of the read with the list holds it, and how many lists each holds;
  - rows otherwise equal: each row's other children, in order, with attributes, text and descendants;
  - whole answer otherwise equal, once the lists are removed;
  - how many elements of the read without the list hold a keyed number.
- **Every answer:** its bytes and its content checks.
- **By hand:** the voucher numbers Tally gave in steps 2 and 3, and what the dispatch screen showed, from the screenshots.

## Expected
- **s1:** GUID yes, `ALTMSTID` 215, `ALTVCHID` 3. L holds three rows.
- **r1:**
  - L holds five rows; the new ones are Sales `3` and `4`, dated 20250403. M's `ALTVCHID` is above s1's.
  - Each of the six window reads holds five rows.
  - From 2027-DN, in each of EGDN, CGDN and CEGDN: Sales `3` holds one `INVOICEDELNOTES.LIST` entry, `BASICSHIPPINGDATE` 20250401 then `BASICSHIPDELIVERYNOTE` DN-81. Sales `4` holds two, DN-82 / 20250402 then DN-83 / 20250401. Every other row holds one entry with no children. Each pair is otherwise equal, both per row and as a whole answer.
  - EGD, CGD and CEGD hold none of the three numbers.
- **s9:** M and L byte-identical to s1's.

## Decides
- **All three pairs as Expected:** the list is measured on every read that names the GST words, on this book. The #2027 Delivery Note change keeps the list on all four reads, and its partial mark narrows to one book and one release.
- **Any pair not as Expected** (a number missing, the entries laid out another way, or an "otherwise equal" no): stop and report, with no code. The change does not merge as it stands. Whether the list is named only on the reads where it was measured, or the combination is refused, goes back to the maintainer.

## Code admits the trigger
At 9a415553:
- **The three reads with the list:** `vouchers` with `include_gst_details`, and with `include_e_invoice` (EGDN), a voucher-type filter (CGDN), or both (CEGDN).
  - `EInvoiceFields::from_args` (agent_vouchers.rs:601) asks for the e-invoice words. A voucher-type filter picks `VoucherReadShape::ClassEntryWildcard` (agent_vouchers.rs:725). The GST words are asked by `VoucherGstFields::following` (:731).
  - `render_agent_named_vouchers_in_span` (agent_read_profiles.rs:180) appends `AGENT_VOUCHER_E_INVOICE_WORDS` at :190, `AGENT_VOUCHER_GST_WORDS` at :193 and `AGENT_VOUCHER_DISPATCH_WORDS` at :194. The class shape adds `voucher_type_class_computes()` at :203.
  - `AGENT_VOUCHER_DISPATCH_WORDS` (:63) now ends with `,INVOICEDELNOTES.*`. Nothing refuses the GST words with the e-invoice words or a voucher-type filter.
  - `only_a_read_that_asks_names_the_e_invoice_and_voucher_gst_details` (agent_voucher_window_tests.rs:9108) asserts, for the plain and class shapes, that each such read is the plain request with exactly those words added, the list included. So each `…N.xml` is its pair with the list added, and `gen.py` asserts that on the bytes.
- **EGD, CGD and CEGD:** the requests master (ae9a1bb5) renders for the same calls, and 2027-D2 sent. After the change, no build sends them; here they are the reads without the list.
- **M:** `render_agent_company_high_water` (agent_read_profiles.rs:574).
- **L:** a lab read for shape, which the build does not send.
- **The two vouchers:** keyed by hand on Tally's screen. No Bridge request writes them.

**Fixture to commit afterwards:** `a-r1-CEGDN.xml`, if its content checks pass, by the manifest route as before.
