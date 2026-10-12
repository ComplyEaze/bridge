# Lab run sheet: where a hand-keyed Contra keeps a transfer reference (#1392 evidence). SYNTHETIC COMPANY ONLY. Not for merge.

About 40 minutes. Keying on screen plus reads; ComplyEaze Bridge writes nothing.

**WHAT IT DECIDES:**
- Where TallyPrime stores a transfer reference keyed by hand: the narration, or a bank allocation field (which one, under which transaction type).
- What Tally sends for each voucher with ComplyEaze Bridge's own voucher request (the raw capture, which keeps `BANKALLOCATIONS.LIST`), and what ComplyEaze Bridge's `vouchers` answer shows. ComplyEaze Bridge's parser keeps only the ledger, amount and side of each entry, so a bank-allocation reference is expected to be absent from the answer and present only in the capture.
- Whether two Contras between the same two bank ledgers with the same amount and date can be told apart by anything but the reference.

The result decides whether a later #1392 slice (pairing a skipped statement line with a hand-keyed Contra) can be built.

**RULES:**
- Use a NEW company, "BRIDGE TRANSFER LAB" (books from 1 Apr 2026, no inventory, no GST). Load only it. Port 9001 only. Never touch a client company.
- The references below are synthetic. Type them exactly; never use a real one.
- One request at a time; never abandon a request; STOP on any Tally dialog you did not expect, a second company loaded, Education mode (check with ComplyEaze Bridge's `tally_status`), or 10 minutes without an answer.
- ComplyEaze Bridge's redaction setting must be off (the default), so narrations are returned; report it.
- Screenshot every voucher's screen, including its bank-allocation screen when one opens.
- Evidence goes in `C:\bridge-lab\1392-h\evidence\`; zip it and report its SHA256.

**STEPS**
1. **Keying (Tally screen):**
   - Two ledgers under Bank Accounts: `TLab Bank A` and `TLab Bank B`, no opening balance. Accept Tally's defaults for bank details, and screenshot each ledger's creation screen, including which bank fields it offers.
   - Four Contras, each ₹50,000, typed by hand:

| # | Date | Dr | Cr | Where the reference goes |
|---|---|---|---|---|
| C1 | 3 Apr 2026 | TLab Bank B | TLab Bank A | narration only: `UTR 900000000001` |
| C2 | 3 Apr 2026 | TLab Bank B | TLab Bank A | bank allocation only (if Tally opens one; pick an electronic-transfer type and put `900000000002` in the field Tally offers for a reference or UTR; report the field's label) |
| C3 | 4 Apr 2026 | TLab Bank B | TLab Bank A | nowhere |
| C4 | 3 Apr 2026 | TLab Bank A | TLab Bank B | narration only: `UTR 900000000004` (reverse direction) |

   If Tally opens no bank allocation for C2, say so; type C2 with the reference nowhere, and report it as "no allocation offered".
2. **Raw capture (mandatory):** send `V-CONTRA.utf16le.xml` from this folder. Check `Get-FileHash -Algorithm SHA256` against SHA256.txt first. It is ComplyEaze Bridge's own voucher request for this company and window. Send it with:
   `curl.exe -sS --data-binary "@V-CONTRA.utf16le.xml" -H "Content-Type: text/xml; charset=utf-16" -o "evidence\h-contra.response" -D "evidence\h-contra.headers" -w "%{http_code} %{size_download}\n" http://localhost:9001/`
   No `--max-time`. Record curl's exit code and the `-w` line in `evidence\log.txt` with an ISO time. A non-zero exit or a non-200 answer is a transport failure: STOP. Decode it by its charset, parse it as XML, and report per VOUCHER: VOUCHERNUMBER, DATE, GUID, ALTERID, NARRATION, REFERENCE, and every ALLLEDGERENTRIES.LIST's LEDGERNAME, AMOUNT and ISDEEMEDPOSITIVE, with every element under its BANKALLOCATIONS.LIST, by name and value. Count by parsing, never by text search.
3. **Read through ComplyEaze Bridge** (Claude Desktop, the extension as installed): call `list_companies` for the company's GUID, then `vouchers` with `company_guid`, `from` = 2026-04-01, `to` = 2026-04-05 and `voucher_type` = Contra. Save the answer verbatim (`vouchers-answer.txt`). Report per voucher: `voucher_number`, `guid`, `narration`, `reference`, and each amount line as returned (sign and side as given; do not convert). The same-day ₹50,000 Contras are told apart by `voucher_number` and `guid`.

**REPORT, one message:**
- the two ledgers' bank-detail fields as offered;
- for C1-C4: the screenshots, and where the reference sits;
- step 2's capture facts per voucher (above), the response's bytes and SHA256;
- step 3's answer facts per voucher;
- any STOP;
- the evidence zip SHA256.

A command that errors is reported as an error, never as "absent".
