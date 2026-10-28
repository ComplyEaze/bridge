# Lab run sheet: recapture the five hand-keyed Contras with references the merge gate accepts (#1231 / #2006). SYNTHETIC COMPANY ONLY. Not for merge.

About 40 minutes. Keying on screen plus one read; ComplyEaze Bridge writes nothing.

**WHY:** the capture of five hand-keyed Contras that #2006 commits carries three 12-digit references that start with 9 (the first run's synthetic ones). The merge gate's privacy scan reads a 10-digit number starting with 6 to 9 as a phone number and any run of 11 or more digits as an identifier, so the capture's parse record, tests and notes cannot carry them, and a capture is never edited by hand. The same five Contras are keyed again with references the scan accepts, in a NEW company, so the old company (BRIDGE TRANSFER LAB, with its backup BK0 and its reconciled entries) is left exactly as it is.

**WHAT IT DECIDES:** nothing new about Tally. It produces the same shape of answer (five Contras, a `BANKALLOCATIONS.LIST` on each entry, an instrument number on C2 and C5, the transfer reference in the narration on C1 and C4) with synthetic references of a form the gate accepts, so #2006's fixtures, parse record, tests and notes can name them.

**THE REFERENCES (type them exactly; they are synthetic):**

| # | reference | scan |
|---|---|---|
| C1 | narration `UTR 111111111111` | accepted |
| C2 | instrument number `222222222222`, on BOTH entries' bank allocations | accepted |
| C4 | narration `UTR 444444444444` | accepted |
| C5 | cheque or instrument number `123456` (as before) | accepted |

Each new one is a single digit repeated twelve times (12 digits, as long as the first run's): the merge gate's privacy scan accepts exactly that family, and it is still a number, so no field refuses it. Use no other number. After typing each reference, read it back on the screen before accepting; after the capture, check in step 2 that the exact string is in the answer (below). If Tally changes what you typed, STOP, screenshot, and report; do not retype it in another form.

**RULES:**
- Create a NEW company named exactly `BRIDGE BANKREF LAB` (books from 1 Apr 2026, no inventory, no GST). Load only it. Port 9001 only. Never touch a client company, and do not open, alter, back up over or restore BRIDGE TRANSFER LAB.
- One request at a time; never abandon one; no `--max-time`. STOP on any dialog you did not expect, a second company loaded, Education mode (check with ComplyEaze Bridge's `tally_status`), or 10 minutes without an answer.
- ComplyEaze Bridge's redaction setting stays off (the default); report it.
- Screenshot every voucher's screen, including its bank-allocation screen.
- Record Tally's version, release, edition and licence (from `tally_status` and the About screen) in your report.
- Raw requests by curl are allowed for this item (the owner's decision of 26 Oct 2026). Evidence goes in `C:\bridge-lab\1231-ref\evidence\`; zip it and report its SHA256.

**STEPS**
1. **Keying (Tally screen), the same as the first run:**
   - Two ledgers under Bank Accounts: `TLab Bank A` and `TLab Bank B`, no opening balance, Tally's defaults for bank details.
   - Five Contras, each 50,000.00, typed by hand in Tally's default single-entry Contra screen (`Account` is the credited bank, `Particulars` the debited one), in this order so the voucher numbers are 1 to 5:

| # | Date | Dr | Cr | Where the reference goes |
|---|---|---|---|---|
| C1 | 3 Apr 2026 | TLab Bank B | TLab Bank A | narration only: `UTR 111111111111` |
| C2 | 3 Apr 2026 | TLab Bank B | TLab Bank A | bank allocation only, on both entries: transaction type `Same Bank Transfer` (the type the first run used), instrument number `222222222222`, instrument date 3 Apr 2026; no narration |
| C3 | 4 Apr 2026 | TLab Bank B | TLab Bank A | nowhere |
| C4 | 3 Apr 2026 | TLab Bank A | TLab Bank B | narration only: `UTR 444444444444` (reverse direction) |
| C5 | 5 Apr 2026 | TLab Bank B | TLab Bank A | no transfer reference; bank allocation with number `123456`, and the transaction types Tally offers by default as the first run had them: `Cheque` on the credited bank's entry, `Cheque/DD` on the debited one |

   Do not reconcile, alter, delete or re-save any voucher after keying; a voucher saved wrongly is not corrected: STOP, report, and the company is recreated from the start (a correction changes the marks and identifiers the capture would carry). If a screen differs from the first run's (a field missing, a type not offered), STOP, screenshot it, and report; do not improvise a different shape.
2. **Raw capture (mandatory).** Check every file in this folder against `SHA256.txt` with `Get-FileHash -Algorithm SHA256`, and run `python3 -I check-request-ids.py .` (it must exit 0 before anything is sent). `V-CONTRA.utf16le.xml` must be 1,744 bytes with SHA256 `d93946038f2ae6a40b48eb53d4e567352434533fc8b593a5aa766935ecbcc080`; it is ComplyEaze Bridge's own voucher window request with only the company name changed. Send it:
   `curl.exe -sS --data-binary "@V-CONTRA.utf16le.xml" -H "Content-Type: text/xml; charset=utf-16" -o "evidence\ref-contra.response" -D "evidence\ref-contra.headers" -w "%{http_code} %{size_download}\n" http://localhost:9001/`
   Record the exit code, the `-w` line and the ISO time in `evidence\log.txt`. A non-zero exit or a non-200 answer is a transport failure: STOP. Keep the response byte for byte (no re-encoding, no editor). Report its bytes and SHA256, and per VOUCHER: VOUCHERNUMBER, DATE, GUID, ALTERID, NARRATION, REFERENCE, and per `ALLLEDGERENTRIES.LIST` its LEDGERNAME, AMOUNT, ISDEEMEDPOSITIVE and every element under its `BANKALLOCATIONS.LIST` with its value (an empty element as empty), including the NAME identifier of each of the ten bank allocations, listed by voucher and entry. Check with an XML parser, not a text search, that `INSTRUMENTNUMBER` reads exactly `222222222222` on both entries of voucher 2 and `123456` on voucher 5, that the narrations of vouchers 1 and 4 read exactly as typed, and that vouchers 1, 3 and 4 carry no instrument number.
3. **Company identity.** From `list_companies` (or the response), report the company's GUID and company number exactly as returned: the merge gate lists the synthetic identifiers it accepts, and this one has to be added before the capture is committed.
4. **Read through ComplyEaze Bridge** (the extension as installed, or the staged build named with this sheet): `vouchers` with `company_guid`, `from` 2026-04-01, `to` 2026-04-05, `voucher_type` Contra. Save the answer verbatim (`vouchers-answer.txt`). Report per voucher `voucher_number`, `guid`, `narration`, `reference` and each amount line as returned.

**REPORT, one message:** Tally's version, release, edition and licence; the screenshots; where each reference or number sits (narration, or which bank-allocation field and its label); step 2's facts and the response's bytes and SHA256; the company GUID and number; step 4's facts; any STOP; the evidence zip SHA256.

A command that errors is reported as an error, never as "absent".
