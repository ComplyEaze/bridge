# Row 2162-S1: does the read-back request read ISPOSTDATED Yes on a post-dated invoice?

Question (ADR 0004, 31 Oct amendment, S1; #2162 item 1): `ISPOSTDATED` added to the read-back request comes back once and reads No on an ordinary invoice (`lab/1578-postdated`). Yes was never read on that request, because the book held no post-dated voucher. Does it read Yes, once, as a direct child of the voucher?

Book: BRIDGE PILOT LAB (synthetic). Grant: BK before the voucher, restore after, restore check byte-identical, as `lab/1342-w21c-answers` did. Write the release shown on the Tally screen into the answers: the earlier reads were on 7.1. If it is 7.2, say so; that is the row's one difference from them and it does not stop the row.

Voucher: keyed on Tally's screen, not seeded. The post-dated flag is the row's one variable, and an import that sets it was never measured, so the seed rule does not apply here: a seeded row is no evidence of its own shape.

No Bridge build. Raw requests by curl, one at a time, each answer written to a file, counted by an XML parser, not by a text search. Close every other Bridge process first.

Steps:
1. Read the marks (`r02-marks.xml` of `lab/1578-postdated`). Take BK. Read the Gateway's current date and write it down.
2. F2 at the Gateway: set the current date to 10-Aug-2026.
3. Key one BRIDGE Sales invoice dated 25-Aug-2026, the shape of BP/26-27/0001 (BRIDGE Walk-in, 1,000.00 plus 25.00 and 25.00 tax, total 1,050.00), marked post-dated before saving, as `lab/1342-w21c-answers` keyed BP/26-27/0045. Write down the number Tally gives. `BRIDGE CGST` matches two ledgers there: pick `BRIDGE CGST 2.5%` by typing until one row is left, and do not press Enter on "Too many choices".
4. If the number is not BP/26-27/0045: `python3 -I gen.py <number>` in this folder. It changes the number only.
5. Send R (the read-back with `ISPOSTDATED`) twice, then C (the read-back as the code sends it) twice. Files: `R-0045.xml`, `C-0045.xml`, or the ones step 4 made. Content type `text/xml`, the files as they are, BOM included.
6. Send `R-0010.xml` (made with `python3 -I gen.py BP/26-27/0010`) once: the control, an ordinary invoice.
7. Read the marks. Restore, read the marks again, check that answer is byte-identical to step 1's, set the current date back to step 1's.

Record: for R, C and the control: bytes, sha256, number of VOUCHER rows, number of `ISPOSTDATED` direct children of the voucher and their text, `ISCANCELLED`, `ISOPTIONAL`, the three `VCHGSTSTATUS` flags, `ISGSTOVERRIDDEN`, `ALTERID`. Whether each pair is byte-identical. Whether R with its `ISPOSTDATED` line and line break removed equals C byte for byte, as it did on 21 Oct.

Expected: R holds one row with `ISPOSTDATED` Yes once, included Yes, uncertain No, excluded No (the status read gave these on 13 Oct); the control reads No. Empty R or C: the filter did not find the voucher. Stop at step 7, and report the number sent and the number on the screen. It is not a finding about the flag.

Decides: Yes once: #2162 item 1's code can fetch the element and decide the state with `status_of_voucher`, after #1828 item 1. No, absent or twice: the read-back stays as it is, and the limit stated in `agent_import_invoice.rs:1628` stays.

Code admits the trigger, line numbers at master d214d6a0:
- No Bridge path reaches this voucher and none is needed: Sales is not in `LIVE_QUALIFIED_VOUCHER_TYPES` (agent_import.rs:390), so it is keyed. Same reason the row sends raw requests.
- C-0010's bytes are the pinned request: sha256 352dfca9 at agent_import_invoice_admission_tests.rs:890 and agent_import_post_e2e_tests.rs:7297, which is `render_invoice_readback_request` (agent_import_invoice_wire.rs:484). R is that request with `, ISPOSTDATED` after `ISGSTOVERRIDDEN`, unchanged since `lab/1578-postdated`.
- The filter at agent_import_invoice_wire.rs:493 tests date, number and type GUID only, so a post-dated voucher passes it. The window is 1 Apr 2026 to 31 Mar 2027, and 25 Aug 2026 lies inside it.
- `status_of_voucher` (agent_gst_status.rs:114) reads the flag: the arm at :126 turns ("No","No","Yes") into `GstStatus::PostDated`, the read-back does not fetch it (the limit at agent_import_invoice.rs:1628). That is the change this row licenses.
- The answer's shape was read on 13 Oct on the status window (`lab/1342-w21c-answers`), and the current date did not change it, so step 2 only repeats that keying.

Row: lab/2162-s1/
