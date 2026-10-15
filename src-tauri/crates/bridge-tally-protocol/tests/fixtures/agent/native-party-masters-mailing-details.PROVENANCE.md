# Party-master read that names the dated mailing details

Covers `native-party-masters-mailing-details.request.utf16le.xml`,
`native-party-masters-mailing-details.utf16le.xml` and
`native-party-masters-without-mailing-details.utf16le.xml`.

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `native-party-masters-mailing-details.request.utf16le.xml` | 1792 | `1bf17a26c209253cb85a1725037cbf6000df4c6bc5ba77bfdd41ff65ab1ae8fa` |
| `native-party-masters-mailing-details.utf16le.xml` | 20896 | `ceaa6d789d1ce9a41c7bcf233e5d26cbe383dcd261e57fb804935d8845cc75d5` |
| `native-party-masters-without-mailing-details.utf16le.xml` | 19094 | `3c0d8c5b74d575fc192c5c8607b33463d692b49d75a0147e1c2223cb0d9d1915` |

Captured from a synthetic company on TallyPrime 7.1 licensed Silver on 12 October 2026,
one request each, HTTP 200, raw bytes unchanged. The request is the `List of Ledgers`
collection that `render_party_ledger_master_request_with_mailing_details` renders (company `Bridge Lab Win 0.5.1`,
`SVFROMDATE` 20260401, `SVTODATE` 20260831), UTF-16LE with a byte-order mark; its `FETCH`
ends `LEDGSTREGDETAILS.LIST, LEDMAILINGDETAILS.LIST`. The answer is UTF-16LE with no
byte-order mark and holds eight ledgers.

## What it establishes

On this one book and release: every ledger row carries exactly one `LEDMAILINGDETAILS.LIST`.
For the party `Lab Party Duplicate Probe` it holds an `ADDRESS.LIST` of two `ADDRESS` lines,
`APPLICABLEFROM` 20260401, `PINCODE` 560001, `MAILINGNAME`, `STATE` Karnataka and `COUNTRY`
India. For the other seven ledgers (two more parties, `Synthetic Debtor` and `Synthetic Creditor`, and
five that are not parties) it is a
placeholder holding only whitespace. In the same answer, for the same party, the flat
`LEDADDRESS.LIST`, `LEDPINCODE` and `LEDGSTPINCODE` come back empty and `STATENAME` is absent.
The third file is the answer, from the same company, to the request sent 22 minutes earlier
that was identical except that it did not name `LEDMAILINGDETAILS.LIST` (1744 bytes, SHA-256
`c5c018f82d3f503277f35611c52e1d8bb080a3e635f1e30beadf5c057ae2d1df`; the request itself is not
committed: it is the 1792-byte request above without `, LEDMAILINGDETAILS.LIST`): HTTP 200, the same
eight ledgers, every other element the same, and the list absent from every row.

## What it does not establish

A second mailing entry with a later date, a party with two mailing rows, another release,
Gold, or any party whose address is entered another way. It says nothing of the output of
the `ledger_masters` tool or the workbook over this answer, nor of the size of a larger
book's answer.
