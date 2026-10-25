# 698 GST ledgers read (#698): a read of a synthetic book with Duties & Taxes lines, for a parity check. READ-ONLY.

Not for merge. `LAB-COMMON.md` in this folder (v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256
5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6) applies in full. THE COMPANY is `BRIDGE SHAPE LAB`, GUID
3a6bd6e1-b835-4bff-89dd-8a6af138c346 (record its company number at step 0). Start with LAB-COMMON step 0 (C2); send each request as C3 says; end with C10.
Written 25 Oct 2026. Request files: requests/ (r01 company list, r02 marks, s03 company object, s04 groups, s05 ledgers,
s06 trial balance, s07 voucher types, s08 census, s09 vouchers; UTF-16LE with a BOM) and SHA256.txt in this folder. They are the
697-S0A requests with only the company name and the window changed, and each was checked byte for byte against the request the
audit read itself sends for BRIDGE SHAPE LAB, 1 Apr-30 Jun 2025 (a test run on master's reader, 25 Oct): all nine equal.

WHY: the port of gst_ledgers_3b is checked against the frozen reference on synthetic edge books, which only exercise what their author
wrote. Before merge it must also agree with the reference on a read Tally itself produced (P1). BRIDGE SHAPE LAB is a synthetic
FY 2025-26 book with GST ledgers under Duties & Taxes; its read is that evidence. The parity run itself is done on the requester's
machine from the saved answers; nothing is written to Tally.

COMPANY: BRIDGE SHAPE LAB, loaded alone. TallyPrime 7.2, licensed, Education mode No (else STOP).
WINDOW: 1-Apr-2025 to 30-Jun-2025 (the first quarter: small enough for one sitting, long enough to hold Sales, Purchases and journals
with GST lines; if the census shows no voucher with a Duties & Taxes line, STOP and report, the read proves nothing for this test).

## Steps (one request at a time, curl as for 697-S0A; every answer saved as received with headers, bytes and sha256)
1. The company list. Record SHAPE LAB's GUID; it must equal the one the request files carry, else STOP.
2. The marks (voucher and master alteration marks).
3. The census for the window. STOP before step 4 if it counts more than 400 vouchers (the read would be too large for one request;
   report the count).
4. The company object, groups, ledgers, the trial balance for the window, the voucher types.
5. The marks again, the vouchers of the window (one request), the marks again, the company list again.
EXPECT (recorded, not predicted): every answer HTTP 200, STATUS 1, no LINEERROR; the vouchers part holds exactly the census's
vouchers; the three marks answers are identical (if not, the book changed during the read: report and repeat once).

## Evidence
Branch lab/698-gst-ledgers-read-answers with SHA256.txt and MANIFEST (request, company, window, release), as LAB-COMMON C11; SHA to the
requester before pushing. Afterwards (requester's machine): the read is built from the answers, gst_ledgers_3b is run by the reference and by Rust
with the same engagement (tax, payable and RCM ledgers named from the book's own Duties & Taxes ledgers), and the comparison is
recorded as pass/fail with the crate tree and the reference pin.
