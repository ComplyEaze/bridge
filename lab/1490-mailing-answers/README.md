# #1490 party-master captures: raw requests and answers (synthetic probe data)

Book: 'Bridge Lab Win 0.5.1' (company number 100031), loaded alone in TallyPrime 7.1 Silver on port 9001, read-only.
Every file here is the exact bytes sent or received (copied, never re-encoded). The ledgers are synthetic probe data.

| capture | sent (UTC, 12 Oct 2026) | request | request sha256 | answer | answer sha256 | answer bytes |
|---|---|---|---|---|---|---|
| 4b: party-master FETCH as the app sends it | 17:10:01.290 -> 17:10:01.659 | 4b/request-4b.xml (1,744 B, FF FE + UTF-16LE) | c5c018f82d3f503277f35611c52e1d8bb080a3e635f1e30beadf5c057ae2d1df | 4b/response-4b.xml | 3c0d8c5b74d575fc192c5c8607b33463d692b49d75a0147e1c2223cb0d9d1915 | 19,094 |
| 4c: the same FETCH plus LEDMAILINGDETAILS.LIST | 17:32:26.873 -> 17:32:27.051 | 4c/request-4c.xml (1,792 B, FF FE + UTF-16LE) | 1bf17a26c209253cb85a1725037cbf6000df4c6bc5ba77bfdd41ff65ab1ae8fa | 4c/response-4c.xml | ceaa6d789d1ce9a41c7bcf233e5d26cbe383dcd261e57fb804935d8845cc75d5 | 20,896 |

Each request was sent once with `Content-Type: text/xml; charset=utf-16`; HTTP 200 both times; GET /status before and after
answered "TallyPrime Server is Running". Response headers are in each folder's response-headers.txt.

What the answers show (parsed, not text-searched): 8 ledger rows each. In 4b the probe ledger 'Lab Party Duplicate Probe' has an
empty LEDADDRESS.LIST, empty LEDPINCODE and LEDGSTPINCODE, no STATENAME and no PARTYGSTIN; its GSTIN appears only in two
LEDGSTREGDETAILS.LIST entries (APPLICABLEFROM 20260401 and 20260701). In 4c everything 4b returned is identical, and each row
also carries one LEDMAILINGDETAILS.LIST; the probe's holds ADDRESS.LIST (two ADDRESS lines), APPLICABLEFROM 20260401, PINCODE,
MAILINGNAME, STATE and COUNTRY, matching the ledger's mailing details on screen. No PAN, bank, IFSC, email or phone element
occurs in either answer; the only GSTIN values are two synthetic-family GSTINs of the probe ledger.
