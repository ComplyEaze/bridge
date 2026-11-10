# Row 2440-ED: what does the e-invoice read-back's day read answer, on the e-invoice book and on a dense day?

Not for merge. Part of #2162 (item 9's e-invoice details), for #2440. Written 11 Nov 2026. Lines below are at #2440's head 09adf38c (`fix/2162-e-invoice-details`); this branch starts at master 5bf79364. Confidence of everything here until the row runs: unverified.

## Question
After a post, #2440 reads an invoice's e-invoice details back with one request: the `vouchers` window read with the three e-invoice words, over the invoice's one date (`invoice_e_invoice_read`, `src-tauri/src/agent_read_profiles.rs:706`, sent by `read_back_e_invoice`, `src-tauri/src/agent_import_invoice.rs:1810`). It finds the invoice in that answer by the GUID the invoice read-back returned, and compares `IRN`, `IRNACKNO` and `IRNACKDATE` with what was sent. Two things about that request are not measured, and #2440 lists both under "Owed before this can leave draft":
- **Its answer has not been captured.** #2440's read-back tests parse a captured answer to another request: the class-shape window read that also names the GST words (`einvoice-lab-dispatch`, row 2027-D2). The answer §8.2j's row received is not committed.
- **Its size on a dense day is not known.** The read returns every voucher of the day with its entries, bounded only by the book-wide voucher mark the invoice build already checks. The transport refuses an answer over 32 MiB (`XML_RESPONSE_MAX_BYTES`, `src-tauri/crates/bridge-tally-transport/src/lib.rs:94`) and gives up after 20 s (`DEFAULT_REQUEST_TIMEOUT`, :95). A read that fails, or an answer the parse refuses, leaves an invoice that posted correctly reported as divergent.

**This row sends that request, unchanged, to two books,** each read twice, because every admitted read is sent as a pair. It writes nothing.

## Books
Two books in turn, each loaded alone, in one sitting. `LAB-COMMON.md` in this folder (v3, byte-equal to row 2162-IRN9's) applies in full to each. Where it and this sheet differ, the stricter one holds. Write down the release on Tally's screen at each load.

1. **`BRIDGE EINVOICE LAB`** (synthetic), GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`.
   - **Its state,** from row 2162-IRN9 (`lab/2162-irn-import-answers` at 9cb3ebd0), whose restore left it so: three vouchers, all dated 3-Apr-25 — Sales `1` (invoice A, with the e-invoice details keyed by hand), Sales `2` (B, without them) and a Payment `1`. Marks V 3, M 215. TallyPrime 7.2, single-user edition.
   - So the day read for 3-Apr-25 holds an invoice with the three details and one without them. Invoice A was keyed, not imported: an imported invoice with details was restored away by row 2162-IRN9.
2. **`BRIDGE CORPUS DENSE`** (synthetic), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, `books_from` 20250401.
   - **Its state,** from row 2162-S6 (`lab/2162-s6-answers`): V 29,900, M 328; every voucher of the year is Sales class; 82 vouchers on 1-Apr-25 (s1 there). Its data folder holds 20 files while the company is shut and 23 while it is loaded.
   - This book holds no e-invoice details. Its day measures the read's size and time, not the details.

Not a client book, and nothing seeded from client data. A book not on the PC, another GUID, or Education mode: stop.

**Read only.** No import, no backup, no restore, and no key pressed in Tally beyond loading and shutting each company. If either book's marks differ from the state above at its first marks read, record them and go on: nothing here writes.

## Kit
No Bridge build. Raw requests, one at a time, through `timed.py`. Close every other Bridge process first.

| file | what |
|---|---|
| `E1-einvoice-day.xml` | #2440's day read for `BRIDGE EINVOICE LAB`, 3-Apr-25 |
| `E2-dense-day.xml` | the same read for `BRIDGE CORPUS DENSE`, 1-Apr-25 |
| `M-einv.xml` | the marks read for `BRIDGE EINVOICE LAB`: a byte copy of row 2162-IRN9's `M-marks.xml` |
| `M-dense.xml` | the marks read for `BRIDGE CORPUS DENSE`: a byte copy of row 2162-S6's `M-marks.xml` |
| `timed.py` | row 2162-S6's, byte-equal: each send timed, no time limit |
| `count-ed.py` | row 2162-S6's `count6.py`, reshaped for two books and these reads (below) |

Requests are UTF-16LE with a byte-order mark, as sent. `SHA256.txt` lists every file with its sha256 and size: check it first.

**The two day reads are the build's own request.** Each is `invoice_e_invoice_read` rendered on #2440's head 09adf38c, by a scratch test that called it with the company and date above and wrote what it returned, encoded as UTF-16LE with a BOM. The scratch test was not committed. `E1` and `E2` differ only in the company and the date. The window is that one day, as both `SVFROMDATE`/`SVTODATE` and the `BridgeAgentWindow` formula, and the fetch list is `DATE,VOUCHERNUMBER,VOUCHERTYPENAME,PARTYLEDGERNAME,NARRATION,GUID,ALTERID,MASTERID,ISCANCELLED,ISOPTIONAL,ISPOSTDATED,ALLLEDGERENTRIES.*,REFERENCE,ISINVOICE,PARTYGSTIN,REFERENCEDATE,IRN,IRNACKNO,IRNACKDATE`. **`E1` is byte-equal to the request row 1378 sent through a build** to this book on 2 Nov 2026 (§8.2j's row): `lab/1378-read-answers/relay/req-e0702cbb980e35f7.bin`, 1,794 B, sha256 `e0702cbb980e35f77e84cf48085824ef7d500d2f674e62d4c93d8d7257e95a71`, which its relay saved as sent. That relay kept Tally's answer only as a size and a hash in `relay/relay.jsonl` (127,968 B, sha256 `babba160abf45255d1666c9528385c765dbca5dc5ab630ad62b81878279a0a3b`, four sends), not as a file. That is why the answer is not committed. It fetches `PARTYGSTIN`, so s1 and s2 hold the e-invoice book's invented customer GSTIN (row 2162-IRN9 kept that GSTIN on the lab PC).

**timed.py** is row 2162-S6's: `python3 -I timed.py <gateway URL> <label> <files>`. It sends `Content-Type: text/xml; charset=utf-16` by `curl.exe -sS --fail-with-body` and gives curl no time limit, because a read abandoned by its client keeps Tally busy for every company. It refuses an answer file that exists already, writes `<answer>.pending` before each send and removes it only after the send's log line, and stops at a curl failure, an HTTP code other than 200, and an answer that says Unknown Request.

**count-ed.py** (`python3 -I count-ed.py`) reads every answer with expat, never a text search, and its output is bounded. For each answer:
- bytes and sha256, and how many GSTIN-shaped values it holds (counted, never printed);
- **M:** whether the GUID is that book's, ALTVCHID and ALTMSTID;
- **E:** the rows (a `VOUCHER` under `COLLECTION` with a `VOUCHERNUMBER`), their total bytes, the rest of the answer, the per-row minimum, maximum and mean; the counts of each type name and of `ISCANCELLED`; the first and last date; how many rows hold each e-invoice tag once and how many hold it not empty; how many rows have a GUID beginning with the book's GUID; the ledger entry lists per row; and each row (all of them, up to 5; otherwise the first 3) with its number, type, each e-invoice tag's count and length, `IRNACKNO` and `IRNACKDATE`, and its GUID after the book's;
- whether two sends of one request hold the same rows, element for element;
- each send's timing from the logs;
- an answer still marked `.pending` as no answer, never as 0 rows.

When the sheet was written, count-ed.py was run on the committed class-shape answer of `BRIDGE EINVOICE LAB` (`einvoice-lab-dispatch/window-read-class-e-invoice-dispatch.utf16le.xml`, row 2027-D2), renamed to this sheet's names, and on row 2162-IRN9's s1 marks answer. It found the three vouchers of 3-Apr-25, Sales `1` holding all three details (`IRN` of 64 characters, `IRNACKNO` 112233, `IRNACKDATE` 20250403) and the other two holding them empty, each GUID beginning with the book's, and the marks V 3, M 215.

## Steps
One at a time. Run `count-ed.py` after each step and keep its whole output. **Record the wall clock at each send** beside timed.py's own lines.

0. **Check the kit.** Check `SHA256.txt`.
1. **The e-invoice book.** Load `BRIDGE EINVOICE LAB` alone, and step 0 of `LAB-COMMON.md` (C2) with `M-einv.xml` as its marks read.
   - **m1.** `timed.py URL m1 M-einv.xml`. EXPECT the book's GUID, V 3, M 215. Other marks: record them and go on.
   - **s1.** `timed.py URL s1 E1-einvoice-day.xml`: **one send, watched.** Screenshot Tally's window as soon as the answer is written. Any dialog, or a window other than the Gateway: STOP (below). An answer that is not a collection of rows, or an Unknown Request: record it whole and stop; the read does not serve.
   - **s2.** `timed.py URL s2 E1-einvoice-day.xml`, the paired read's second half.
   - **m2.** `timed.py URL m2 M-einv.xml`. EXPECT the answer byte-equal to m1's.
   - **Close it** by C10: Shut Company by Tally's own menu, with the path and keys written down.
2. **The dense book.** Count its data folder while it is shut: EXPECT 20. Load `BRIDGE CORPUS DENSE` alone, and step 0 (C2) with `M-dense.xml` as its marks read. Count the folder again: EXPECT 23.
   - **m3.** `timed.py URL m3 M-dense.xml`. EXPECT the book's GUID, V 29,900, M 328. Other marks: record them and go on.
   - **s3.** `timed.py URL s3 E2-dense-day.xml`.
   - **s4.** `timed.py URL s4 E2-dense-day.xml`, the second half of the pair.
   - **m4.** `timed.py URL m4 M-dense.xml`. EXPECT the answer byte-equal to m3's.
   - **Close it** by C10. Count the folder: EXPECT 20. Screenshot: no company loaded. One tally.exe owns 9001.

## Stop rules
- **Any dialog or prompt** on Tally's screen, at any step: press no key, screenshot, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.
- **No answer in 15 minutes:** press nothing and kill nothing, curl included. Screenshot Tally's window and ask the run coordinator. A read given up by its client keeps the gateway busy.
- **The gateway stops answering**, a curl failure, an HTTP code other than 200, or an Unknown Request: timed.py stops itself. Record it and ask; resend nothing.
- **Education mode**, or a book is not on the PC.
- **m2 not byte-equal to m1, or m4 to m3:** record both, report, and send nothing more.

## Record
For each send:
- the answer's bytes and sha256, its GSTIN-shaped count, its rows and its row bytes (total, rest, minimum, maximum, mean);
- **its seconds:** curl's first-byte and total, and the wall clock between start and end;
- the type-name counts, the date range, the e-invoice tag counts and the GUID count;
- for s1 and s2, each row as count-ed.py prints it;
- count-ed.py's comparison lines (s1 against s2, s3 against s4).

And: the marks (bytes, sha256, V, M, byte-equal or not), the folder counts, the release at each load, Tally's window after s1, and anything that differed from EXPECT.

## Expected
Estimates from earlier rows, not predictions.
- **s1 and s2:** three rows dated 20250403. Sales `1` with `IRN` of 64 characters, `IRNACKNO` 112233 and `IRNACKDATE` 20250403; Sales `2` and the Payment with the three present and empty. Each GUID begins with the book's. Row 1378's answer to this request was 127,968 B, sha256 `babba160…`. s1 equal to it says the day is as 1378 read it. A difference is recorded, not a stop: rows 2162-IRN and 2162-IRN9 wrote to this book since then and restored it.
- **s3 and s4:** 82 rows dated 20250401, all Sales. With their entries a row is larger than the 2.8 KB a number read's row was (row 2162-S6). Size and time not predicted; the transport's limits are 32 MiB and 20 s.

## Decides
- **#2440's read-back test answer.** s1 is committed as a file this time, where 1378's relay kept only its hash. If s1 holds the three vouchers with Sales `1`'s details as above, s1 is the captured answer to #2440's own request. It replaces the derived class-shape answer in #2440's read-back tests, through a fixture with its provenance, in #2440 itself. s1 against s2 says whether the two halves of the pair agree.
- **The dense day.** s3's and s4's bytes and seconds, against the transport's 32 MiB and 20 s, each counted twice for the pair. #2440's text on the unbounded read then states those figures and whether this book's day fits. Whether #2440's parse accepts s3 is judged afterwards, in a test on #2440's branch, not in the lab.
- **Not decided here:** the composed path, an import carrying `e_invoice` followed by this read-back on the built executable. That row waits for #2440's merge-up onto #2434.
- **A dialog anywhere:** the row stops.

## Code admits the trigger (09adf38c)
- **E1 and E2:** `invoice_e_invoice_read` (`agent_read_profiles.rs:706`) calls `render_agent_named_vouchers_in_span` (:180) with the date as both ends of the window, no AlterID span, `NamedVoucherFields::E_INVOICE_ONLY` (the plain voucher fetch list with the three e-invoice words added), and the plain list rather than the class-shape one (`classes` false). #2440 sends it from `read_back_e_invoice` (`agent_import_invoice.rs:1810`, the send at :1823) through `post_read`, paired; `e_invoice::read_rows` (`agent_import_invoice_e_invoice.rs:172`) parses the answer, and `readback_differences` (:138) finds the invoice by GUID. The same window read with `include_e_invoice` is what a `vouchers` call sends for a window it reads whole (§8.2j).
- **M:** `render_agent_company_high_water`, the marks request of rows 2162-IRN9 and 2162-S6, byte-equal to their files.
- **No write is possible:** no request here is an import, and no Bridge build runs.

## Keep
On `lab/2440-ed-answers`, with `SHA256.txt`:
- every answer of 10 MB or less whole, with each log, unless it holds a GSTIN-shaped value the branch gate refuses: such an answer stays on the lab PC, listed in `kept-local.txt` by name, size and sha256;
- for each larger answer, its name, size and sha256 in `kept-local.txt`; the file stays on the lab PC until the coordinator says;
- count-ed.py's whole output after each step;
- a RESULTS.md with one line per send (rows, bytes, seconds) first;
- a notes file with UTC times, one line per step.

Screens stay on the lab PC, listed by name, size and sha256. Send the sha to the run coordinator before any push.

Row: lab/2440-ed/
