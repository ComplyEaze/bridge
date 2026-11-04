# Row 2027-DN: where does a keyed Delivery Note number land in the vouchers window read?

Not for merge. Part of #2027. Code lines are at 4e94ac9a, the head of the #2027 dispatch change.

**The question.** Row 2027-D (`lab/2027-dispatch-answers`) keyed `DN-71` on a Sales invoice and found it only in a full read, at `INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE`. Tally asked a date for it, and a second, empty Delivery Note line followed. Row 2027-D2 (`lab/2027-d2-answers`) sent the window read with `,INVOICEDELNOTES.*` added (GDN). Each row came back with one empty list and was otherwise equal, but the book held no Delivery Note then. This row keys two Sales invoices, one with one Delivery Note and one with two, and sends GD and GDN against them. It answers three things:
- where the number lands in GDN;
- which element holds its date;
- how two numbers are listed.

**Book.** `BRIDGE EINVOICE LAB`, GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`, TallyPrime 7.2 Silver, as row 2027-D2 left it ("The book now", `lab/2027-d2-answers` RESULTS.md):
- unchanged since 2027-D's restore;
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

The answers, the screenshots and the backup stay on the lab PC. List each by sha256, bytes and name in `kept-local.txt`. The window and full reads, and the screens, hold the book's invented GSTINs.

## Files
UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file. Each read is for `BRIDGE EINVOICE LAB`. GD, GDN and F are for 3 Apr 2025 alone, the day both new vouchers are dated.
- `M-marks.xml` and `L-list.xml`: `lab/2027-d2`'s, byte-equal. M reads the company's marks; L lists every voucher.
- `GD.xml`: `lab/2027-d2/GD.xml`, byte-equal (sha256 6bda8952…). It is the request the #2027 dispatch change renders for `vouchers` with `include_gst_details`.
- `GDN.xml`: `lab/2027-d2/GDN.xml`, byte-equal (sha256 d99554fc…). It is GD with `,INVOICEDELNOTES.*` added after the seven dispatch words. No build sends it.
- `F-sales.xml`: `lab/2027-dispatch/F-sales.xml`, byte-equal (sha256 ac7f0913…). It is the full read (every native method, no fetch list), filtered to vouchers of type Sales.
- `run.py`: `lab/2027-d2/run.py`, byte-equal.
- `gen.py` makes no request. It asserts each file above by its sha256, and that GDN is GD with only the list added. Run it before step 0; any failure is a stop.
- `count.py`: see Record.

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
0. **The book.** Run `python3 -I gen.py`; it must print `checked: run.py M-marks.xml L-list.xml GD.xml GDN.xml F-sales.xml`. Then:
   - If the Gateway shows `BRIDGE EINVOICE LAB` loaded alone, change nothing.
   - If another book is loaded with it, or it is not loaded, stop and ask before anything else is pressed.

   Take a screenshot of the Gateway.
1. **Check the book.** Run `run.py URL s1 M-marks.xml L-list.xml`, then `count.py --company-number N`. It must show:
   - GUID is the book's: yes;
   - `ALTMSTID` 215 and `ALTVCHID` 3;
   - L with three rows, all 20250403: `1` Sales, `2` Sales, `1` Payment;
   - the content checks pass on both answers.

   Then take a BK on Tally's Backup screen: one company, to a new empty folder, "Accept ?" y. Record its file's bytes and sha256 in `kept-local.txt`. Write down the release and the Gateway's date and period.
2. **Sales A, one Delivery Note.** Key one Sales voucher in the invoice mode Tally opens, as row 2027-D's step 2:
   - dated 3-Apr-25, party `BRIDGE IRN Customer`;
   - `BRIDGE IRN Sales` 100.00 and `BRIDGE IRN IGST` 18.00 typed;
   - narration blank;
   - `Provide GST/e-Invoice details` No. Press nothing under e-Invoice.

   In the dispatch section ("Dispatch Details" in 2027-D), fill only these:
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
4. **The reads.** Run `run.py URL r1 L-list.xml GD.xml GDN.xml F-sales.xml M-marks.xml`, then `count.py --company-number N`.
5. **Undo.**
   - Shut the company, then restore the step 1 BK: Data > Backup & Restore > Restore, Specify Path, the BK folder. Answer prompts 5 and 6.
   - Load the book alone.
   - Run `run.py URL s9 M-marks.xml L-list.xml` and `count.py --company-number N`.
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
- **GD, GDN and F-sales,** for each row:
  - its number, its type, its count of direct children and of all elements, and its count of `INVOICEDELNOTES.LIST`;
  - for each of `DN-81`, `DN-82`, `DN-83`, `20250401` and `20250402`: how many elements hold it, and at which paths;
  - for each `INVOICEDELNOTES.LIST`, in order: its children and elements, and each descendant's tag, `TYPE` attribute and value. Values this sheet does not name print as their length.
- **GDN against GD:**
  - the rows in each;
  - whether every GDN row holds the list, and how many lists each holds (Sales B may hold more than one);
  - rows otherwise equal: each row's other children, in order, with attributes, text and descendants;
  - whole answer otherwise equal, once the lists are removed;
  - how many elements of GD hold a keyed number.
- **Every answer:** its bytes and its content checks.
- **By hand:** the voucher numbers Tally gave in steps 2 and 3, and what the dispatch screen showed, from the screenshots.

## Expected
- **s1:** GUID yes, `ALTMSTID` 215, `ALTVCHID` 3. L holds three rows.
- **r1:**
  - L holds five rows; the new ones are Sales `3` and `4`, dated 20250403. M's `ALTVCHID` is above s1's.
  - GD and GDN hold five rows each.
  - From 2027-D: `DN-81`, `DN-82` and `DN-83` each in `INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE` in F-sales. The element that holds each date is not known.
  - In GDN: unknown, which is this row's question. GD holds none of the three numbers.
- **s9:** M and L byte-identical to s1's.

## Decides
- **Each number in one element of one list entry, with its date in a named sibling, in GDN, for both vouchers:** the parse reads a list of `{number, date}` from `INVOICEDELNOTES.LIST`. Sales B gives the shape of two entries.
- **The numbers in GDN but not in GD, and both "otherwise equal" yes:** the window read must name the list. GDN's words go into the build, after the seven dispatch words.
- **A number in F-sales but not in GDN, or a list split across entries some other way, or an "otherwise equal" no:** stop and report, with no code. The answer goes back to the maintainer.
- **The date, if no sibling holds it:** the parse reads the number only, and the date is recorded as not reported.

## Code admits the trigger
At 4e94ac9a:
- **GD:** `vouchers` with `include_gst_details`. agent_vouchers.rs asks for the GST words by `VoucherGstFields::following` (:731). `render_agent_named_vouchers_in_span` (agent_read_profiles.rs:179) appends `AGENT_VOUCHER_GST_WORDS` (:58) at :192 and `AGENT_VOUCHER_DISPATCH_WORDS` (:63) at :193. GD is that render for this book and day, as 2027-D2 sent it.
- **GDN:** a candidate shape, GD with one fixed word. No value chooses it, and no release sends it. Under AGENTS.md P7, a new shape is first sent only to a lab book. 2027-D2 sent it to an empty list; this row sends it to a keyed one.
- **M:** `render_agent_company_high_water` (agent_read_profiles.rs:573).
- **L and F-sales:** lab reads for shape, which the build does not send.
- **The two vouchers:** keyed by hand on Tally's screen. No Bridge request writes them.

**Fixture to commit afterwards:** `a-r1-GDN.xml`, if its content checks pass, by the manifest route as before.

Row: lab/2027-dn/
