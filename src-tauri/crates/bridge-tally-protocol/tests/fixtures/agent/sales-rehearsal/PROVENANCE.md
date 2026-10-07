# GST Sales rehearsal fixtures

These seven files are answers Tally gave on 2026-10-07 during one rehearsal of the GST Sales
invoice core on the disposable BRIDGE GST RECON LAB book (TallyPrime 7.1 Silver, licensed mode).
Each request was sent by itself, straight to the gateway, by a small export script that refuses
anything but an Export request and saves the decoded answer; the two posts between them were made
by `bridge_mcp` built from this pull request's commit `5905768c5` with one line added, Sales in
the qualified list (a local build, never pushed). Each file is the answer's body as Tally sent it, but for one substitution named under the table (UTF-16LE without a
BOM): the saved text was encoded again and its SHA-256 compared with the one the script printed for
the bytes on the wire, for two of the day's answers (both equal; which two was not recorded); for these seven the byte counts
equal the counts the script reported, and the decoded text of each equals the text the script saved,
but for the substituted token in two of them. Nothing was trimmed: each answer is of this one company only.

The data is synthetic. The book is the disposable lab company other fixture sets here were captured
from, so the answers also carry its earlier synthetic vouchers and types; the customers
`TG Buyer Regular RJ` and `TG Buyer Unregistered RJ`, their invented GSTIN, the type `Sales Manual`
and the invoices numbered `TG/25-26/...` were made for this rehearsal. Besides the two raw U+0005
characters named in the table, the answers carry Tally's numeric references to control characters
(`&#4;`), as received. One book, one release, one day:
the files back the parsers and the order of the invoice's reads on answers Tally really gave, not
any claim about another release, licence tier or book.

Each request's text is the one `agent_import_invoice_wire.rs` renders (the script that filled the
request files proves the text equal to the source's before it writes one); the SHA-256 of each
request file (UTF-8, as sent to the script) is in the last column.

| Fixture | Bytes | SHA-256 | What it is | Request SHA-256 |
| --- | ---: | --- | --- | --- |
| `sales-rehearsal-voucher-types.utf16le.xml` | 272,124 | `d0777cca7aab46685a485b626c78b3330ad1243fc84370e8870ae188511a1959` | every voucher type of the book, all fields (the invoice build's voucher-type request); 25 types, among them the user type `Sales Manual` (parent Sales, one series, Manual, duplicates prevented) keyed by hand for the rehearsal; Tally wrote two raw U+0005 characters inside that type's class name, kept as received | `d6e7d30737d55609ec39eb0e7b31676ce1c15f6ea39b27d30ac110f50ec67259` |
| `sales-rehearsal-number-known.utf16le.xml` | 5,600 | `846aeb69976ad2c687e12d5561986acde4f9931b350e1e7a9b6685106faa5536` | the duplicate-number request for `TG/25-26/001`, financial year 2025-26: one row, the hand-keyed invoice, Sales class Yes | `83e5beb9bd634a4a81662bb8da2c696b0f9b7019de4f03b59da2a6aca031a012` |
| `sales-rehearsal-number-absent.utf16le.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` | the same request for `TG/25-26/999`, a number no voucher carries: STATUS 1 and a COLLECTION with no row | `d5b5923c2c1f7094b426519a86ae36394d735c9828ba2fe4a01ce89fccf0c571` |
| `sales-rehearsal-number-shared.utf16le.xml` | 8,132 | `ac4992ac615d854ffb5092f5b173504b94aa715b25bf666be663d79892ca46d5` | the same request for `12`: two rows, a Purchase (Sales class No) and a Sales voucher (Sales class Yes) | `1b4102bd571d6ef328a1e3767ead3724a39a1cee31b7f2e08e950ffc34b12a6a` |
| `sales-rehearsal-readback-posted-registered.utf16le.xml` | 77,512 | `cd3b769439ede5a2b3758cbee9aa50da5f05cc0dceddee2b6b48c4eaaa266eb4` | the invoice read-back request for `TG/25-26/002`, the invoice ComplyEaze Bridge posted to the registered, bill-wise customer (five legs, a New Ref, a credit round off) | `fce86c686271f0266b7a04b79360064d190c5deb8c89179451a44810a65ede22` |
| `sales-rehearsal-readback-keyed-registered.utf16le.xml` | 64,178 | `75638ffd0c433c70fa1a190db3c015b3f06f46c995a89416438a5bfb13fe126c` | the same request for `TG/25-26/001`, an invoice keyed by hand in Tally to the same customer: it carries no REFERENCE and no REFERENCEDATE | `6cabebc27fccd8fd526d1aeda9968f159b15c14426f67a1d52f4b10d05ac2835` |
| `sales-rehearsal-readback-posted-unregistered.utf16le.xml` | 62,770 | `a5b4f4ce7364b48372e9f2e77b6e51fbe4b00e02b5a84da9de7f8c74f282b248` | the same request for `TG/25-26/003`, the invoice ComplyEaze Bridge posted to the unregistered customer (four legs, no bill allocation) | `435850e45e83378f389676da02d9d45359e06aef87906451d0dfc4f02f1532b2` |

One substitution was made, in the two read-backs of the registered customer's invoices. That
customer's invented GSTIN had the shape of one that could be issued, so it is replaced, once in each
of the two files, by a token of the same length that cannot be issued (`08ZZZZZ0000Z1ZQ`). Those two
files differ from Tally's bytes by that token only, and their SHA-256 in the table is of the committed
file. The other five are as received.

Two notes for a reader checking the table. The voucher-types request was written the day before the
others, with that day's request files, and its hash is of that file. In that answer the keyed type's
`LASTNUMBER` reads `TG/25-26/013`: three invoices were keyed by hand in the type before the rehearsal,
numbered 001, 011 and 013, and the rehearsal then posted 002 and 003.
