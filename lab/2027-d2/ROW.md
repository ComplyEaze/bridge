# Row 2027-D2: does each admitted window read stay the same when the seven dispatch words are added?

Not for merge. Part of #2027. Lines are at master a4b4adff.

**The question.** Row 2027-D (`lab/2027-dispatch-answers` at 4e0918e7) sent the vouchers window read with `include_gst_details` (W) and the same read with seven dispatch words added (WD). WD carried each word once a row, and its answer was otherwise W's. The change for #2027 adds the seven words to every read that names the GST words. A `vouchers` call also sends them with `include_e_invoice`, and with a voucher-type filter, and nobody has sent those with the seven words. This row sends each such read with and without them. It also sends WD with the Delivery Note list named, a separate request, to see whether the window read carries that list.

**Book.** `BRIDGE EINVOICE LAB`, GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`, TallyPrime 7.2 Silver, as row 2027-D left it: restored to its s1 state, byte-identical. It holds three vouchers, all dated 3-Apr-25: Sales `1` (with e-invoice details), Sales `2`, and a Payment `1`. Its marks are `ALTMSTID` 215 and `ALTVCHID` 3. No voucher in it holds a dispatch detail or a Delivery Note now.

**Read-only.** This row keys nothing, saves nothing, takes no backup and restores nothing. Every request is a read, sent one at a time through `run.py`, which is `lab/2027-dispatch/run.py` byte-equal. No Bridge build is run.
- `run.py` sends `Content-Type: text/xml; charset=utf-16`.
- It stops at a curl failure, and at an answer that says Unknown Request.
- `count.py` counts every answer with an XML parser, and it prints only tags, counts, lengths, yes or no, and the values this sheet names.
- Record any other Bridge process. None of them may take a call.

**Output: shapes and counts only.** `RESULTS.md` carries:
- the whole output of `python3 -I count.py`;
- the release on Tally's screen;
- one line per step saying it was done as written, or what differed.

The answers and the screenshots stay on the lab PC. List each by sha256, bytes and name in `kept-local.txt`. The window answers hold the book's invented GSTINs.

## Files
UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file. Each request is for `BRIDGE EINVOICE LAB` and 3 Apr 2025 alone.
- `M-marks.xml` and `L-list.xml`: `lab/2027-dispatch`'s, byte-equal. M reads the company's marks; L lists every voucher.
- Four requests a build of the #2027 change renders (`VoucherReadShape::render`, no AlterID span), for the four `vouchers` calls that name the GST words:
  - `GD.xml`: `include_gst_details`. Byte-equal to `lab/2027-dispatch/WD.xml` (sha256 6bda8952…), the request 2027-D answered.
  - `EGD.xml`: `include_gst_details` and `include_e_invoice`.
  - `CGD.xml`: `include_gst_details` and a voucher-type filter, which makes Tally resolve each row's type in the same answer.
  - `CEGD.xml`: all three.
- `G.xml`, `EG.xml`, `CG.xml` and `CEG.xml`: each of the four with the seven words removed, which is the request master sends today for the same call. `G.xml` is `lab/2027-dispatch/W.xml`, byte-equal.
- `GDN.xml`: GD with `,INVOICEDELNOTES.*` added after the seven words, naming the list where 2027-D found the Delivery Note number in the full read (`INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE`). No build sends it. `.*` follows the window read's own `ALLLEDGERENTRIES.*`.
- `gen.py` makes the five derived files from the four, and asserts each change and GD's sha256.

## Prompts this row expects
None. Any prompt is a stop (see Stop).

## Steps (`python3 -I run.py <gateway URL> <label> <files>`)
0. **The book.** If the Gateway shows `BRIDGE EINVOICE LAB` loaded alone, change nothing. If another book is loaded with it, or it is not loaded, stop and ask before anything else is pressed. Take a screenshot of the Gateway.
1. **Check the book.** Run `run.py URL s1 M-marks.xml L-list.xml`, then `count.py`. It must show:
   - GUID is the book's: yes;
   - `ALTMSTID` 215 and `ALTVCHID` 3;
   - L with three rows, all 20250403: `1` Sales, `2` Sales, `1` Payment.

   Write down the release and the Gateway's date and period.
2. **The reads.** Run `run.py URL r1 G.xml GD.xml EG.xml EGD.xml CG.xml CGD.xml CEG.xml CEGD.xml GDN.xml`.
3. **Check the book again.** Run `run.py URL s9 M-marks.xml L-list.xml`, then `count.py`. The s9 M and L answers must be byte-identical to s1's (compare their sha256).

## Stop
Stop at once, and send nothing more, if any of these happens:
- any prompt or screen appears that this sheet does not name;
- the s1 check differs from the book above;
- `run.py` stops (a curl failure or Unknown Request);
- any answer does not parse in `count.py`;
- s9 differs from s1.

Screenshot the window, keep the answers, and ask before anything else is pressed or sent. Never press any e-Invoice or e-Way Bill action, and never save, alter or delete anything.

## Record
- The whole output of `count.py`.
- For each pair below, from count.py's last lines: the rows in each answer; each added word once in every row (yes or no); rows otherwise equal (yes or no); whole answer otherwise equal (yes or no):
  - GD against G, the 2027-D measurement repeated in this sitting;
  - EGD against EG, CGD against CG, and CEGD against CEG;
  - GDN against GD, with each row's `INVOICEDELNOTES.LIST` count and its children's tags.
- s9 against s1: byte-identical (yes or no).

## Expected
- Every request answered, 3 rows each.
- Each of the seven words once in every row of GD, EGD, CGD and CEGD, empty, since no voucher here holds a dispatch detail. Each answer otherwise equal to its pair's.
- GDN: unknown. With no Delivery Note in the book, the list may come back empty or not at all, or it may change something else.

## Decides
- **All four pairs: each word once in every row, and both "otherwise equal" yes.** Adding the seven words to every read that names the GST words is measured for the calls `vouchers` admits, on this book, with the values empty. The values themselves rest on 2027-D.
- **Any pair "no".** The words are not added to that call's read. The code either sends them only where measured or refuses the combination. That goes back to the maintainer, not decided here.
- **GDN.** Only whether the window read accepts the list and is otherwise unchanged. This book holds no Delivery Note, so the row cannot show a number landing in it. Building the Delivery Note needs a row with one keyed.

## Code admits the trigger
At master a4b4adff:
- **G:** `vouchers` with `include_gst_details`. agent_vouchers.rs:720-731 picks `VoucherReadShape::EntryWildcard` and asks for the GST words by `VoucherGstFields::following` (:731). `render_agent_named_vouchers_in_span` (agent_read_profiles.rs:174) appends `AGENT_VOUCHER_GST_WORDS` at :187.
- **EG:** the same with `include_e_invoice`. `EInvoiceFields::from_args` (agent_vouchers.rs:601) asks for the e-invoice words, appended at agent_read_profiles.rs:184 before the GST words. Nothing refuses the two together.
- **CG, CEG:** the same with a voucher-type filter. agent_vouchers.rs:724-725 picks `ClassEntryWildcard`, and agent_read_profiles.rs:196 adds `voucher_type_class_computes()`.
- **Not a combination:** `stock_lines` refuses `include_gst_details` and `include_e_invoice` (`NEEDS_LEDGER_ENTRIES`, agent_voucher_parse.rs:337, refused at :369), so its read never names these words.
- **GD, EGD, CGD, CEGD:** the #2027 change appends the seven words right after `AGENT_VOUCHER_GST_WORDS` in that same function, so each read that names the GST words names them too. The four files are its renders.
- **GDN:** a candidate. No build sends it.
