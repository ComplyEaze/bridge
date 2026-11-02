# Results: #2243-C1, the tax-audit company request with two more fields, 2 Nov 2026

**Pass. All three elements come back on all three books, present and direct children of `COMPANY`. A shows `ISINVENTORYON` Yes and `NUMSTOCKITEMS` 0. B's count, 55, equals the 55 stock item rows read in the same sitting. No element was absent or empty.**

Request: the bytes of `AuditCompanyObjectV1` as `render_company_object` renders them on master 3876be074, with `<FETCH>ISINVENTORYON</FETCH><FETCH>NUMSTOCKITEMS</FETCH>` after `<FETCH>ISINTEGRATED</FETCH>`, and nothing else changed. UTF-16LE with a BOM, as Bridge sends. The render was built from the code's strings, not captured from Bridge. Curl, one request at a time, one book loaded alone each time. TallyPrime 7.2 Silver. Reads only; no book was seeded.

| step | book | HTTP | answer bytes = CONTENT-LENGTH | `COMPANY` rows |
|---|---|---|---|---|
| A | BRIDGE EMPTY BOOK | 200 | 4,432, equal | 1 |
| B | BRIDGE STOCK LAB | 200 | 4,430, equal | 1 |
| C | BRIDGE CASHDAY LAB | 200 | 4,434, equal | 1 |
| A again | BRIDGE EMPTY BOOK, shut and loaded again | 200 | 4,432, equal | 1 |

Rows counted at `/ENVELOPE/BODY/DATA/TALLYMESSAGE/COMPANY` by parsing. `STATUS` 1 each time. The `CMPINFO` block above them has a `COMPANY` line of its own, reading 0; it is not a row. [verified]

## The elements, verbatim
| step | `ISINTEGRATED` | `ISINVENTORYON` | `NUMSTOCKITEMS` |
|---|---|---|---|
| A | `<ISINTEGRATED TYPE="Logical">Yes</ISINTEGRATED>` | `<ISINVENTORYON TYPE="Logical">Yes</ISINVENTORYON>` | `<NUMSTOCKITEMS TYPE="Number">0</NUMSTOCKITEMS>` |
| B | `<ISINTEGRATED TYPE="Logical">Yes</ISINTEGRATED>` | `<ISINVENTORYON TYPE="Logical">Yes</ISINVENTORYON>` | `<NUMSTOCKITEMS TYPE="Number"> 55</NUMSTOCKITEMS>` |
| C | `<ISINTEGRATED TYPE="Logical">No</ISINTEGRATED>` | `<ISINVENTORYON TYPE="Logical">No</ISINVENTORYON>` | `<NUMSTOCKITEMS TYPE="Number">0</NUMSTOCKITEMS>` |

- All present on all three, none empty, none absent. [verified]
- `NUMSTOCKITEMS` has a leading space when above zero (` 55`) and none at zero, as the reference records for this count. A reader must trim it.
- C is BRIDGE CASHDAY LAB, chosen because it has inventory off. It turned out to be both: inventory off and not integrated. So this row does not separate those two, and it has no book with inventory on and not integrated.

## B against the stock item rows
`Bridge Audit Stock Items` (`AUDIT_STOCK_ITEM_FETCH`) for 1 April 2025 to 31 March 2026, in the same load: 55 rows on B, 0 on A, 0 on C. Each equals that book's count. [verified]

## Repeat and marks
Step A's second read, after B and C and a fresh load: the request, the company answer, the marks and the stock item read are each byte-equal to the first, company answer sha256 1e8f8a969cc6df0d4184a53cc1eb3c91efde62431e7d4052f10b9d880e15c0f2. [verified]

Marks before and after each book are the same bytes: A V empty, M 207; B V 4, M 292; C V 11, M 214. [verified]

## Also in the answer
Each `COMPANY` carries nine children: `BOOKSFROM`, `NAME`, `GUID`, `ISINVENTORYON`, `ISINTEGRATED`, `ISDEEMEDPOSITIVE`, `CANDELETE`, `NUMSTOCKITEMS`, `MASTERID`. `ISDEEMEDPOSITIVE`, `CANDELETE` and `MASTERID` were not asked for, the same three the reference records Tally adding to this list. The order is the same on all three books.

## Left behind
Nothing written. Company shut, no company loaded, one tally.exe on 9001.

**Confidence:** three books, one Tally, one sitting, TallyPrime 7.2 Silver only: PARTIAL beyond these.
