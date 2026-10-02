# `note-days/`: a credit note, a debit note and a cancelled purchase, each read once through a register tool

Thirty-six files from three live tool calls on one synthetic company: `sales_register` for a day
that holds one Credit Note, `purchase_register` for a day that holds one Debit Note, and
`purchase_register` for a day that holds one cancelled Purchase. They are here
for the register tools' tests: `agent_register_server_tests.rs` replays each call from its sequence
record and compares the tool's row with the answer file.

## Four facts first

1. **A credit note is returned as a row with its signs reversed, as Tally sends them.** Tax is
   `-90.00` twice, the sales ledger `-1000.00`, the party `1180.00`. Nothing is netted or flipped by
   the tool. The debit note is the mirror image on the purchase side.
2. **The state-side head on this book is `state_tax` (raw `State Tax`).** On the other lab book
   read the same night it is `sgst_utgst` (raw `SGST/UTGST`). Both are recognised heads for the same
   side of the tax; which one a book carries depends on its ledger master.
3. **The same tool cost 118 requests here and 96 on the smaller book.** This book has two
   currencies defined and a voucher mark above the read planner's threshold, so each call adds a
   voucher census and base-currency reads.

4. **A cancelled Purchase is not returned as a row.** It is listed, with `cancelled: true`, under
   `purchase_vouchers_without_duties_taxes_entry`: a cancelled voucher keeps no entries, so it has
   none under Duties & Taxes. A reader of that list's count alone would take it for an untaxed
   purchase.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`,
  `http://127.0.0.1:9001` (a lab instance).
- **Date:** 2026-10-01, 23:24 IST (the credit note day), 23:24 to 23:25 IST (the debit note day) and
  23:36 IST (the cancelled purchase day).
- **Book:** `BRIDGE SHAPE LAB`, a **synthetic** company seeded by this project: 44 ledgers, 33
  groups, two currencies. Its ledger names are labels in a test book. Where a name resembles a
  real organisation (two bank-style ledger names do), it is a label, not an account of that
  organisation: the book holds no account number, GSTIN, PAN, address, e-mail or phone, and the
  `PARTYGSTIN` elements are present and empty.
- **Sender:** `bridge_mcp`, a debug build made locally from commit
  `5235fd1f2bb48829d531ba15453f909ab086f84d` (the sales register's branch), binary sha256
  `aa167457f0ccf05cba6aa8a45fd569a617d7408d6c90209ce02d413293e5e158`. The same binary served both
  tools.
- **Relay:** a serial read-only relay sat between the binary and the gateway and kept the exact
  request and response bytes of every exchange. It forwards only exports and refuses anything else.
  No request was refused in either call.
- **The calls:** `sales_register` with `from` and `to` `20250429`; `purchase_register` with `from`
  and `to` `20250428`; `purchase_register` with `from` and `to` `20250703`. One voucher on each day. `tally_status` was read before and after each call
  and the loaded companies were the same.
- **Requests per call: 118** (66 exports and 52 status reads), the same in all three calls.
  Exports by request id: `BridgeCompanyExtent` 20, `BridgeCompanyBookExtentV2` 16, `BridgeCompanyCurrencies` 8, `BridgeCompanyBaseCurrency` 4, `List of Ledgers` 8, `List of Groups` 4, `Bridge Agent Voucher Census` 2, `Bridge Agent Vouchers` 2, `Bridge Agent Company High Water` 2.
  On the other lab book (8 ledgers, one currency, three vouchers) the same tool sent 96 (54 exports
  and 42 status reads): no census and no base-currency read, and half the currency reads.

## What each file is

- **`*_request.utf16le.xml` and `*_response.utf16le.xml` are wire bytes.** A request is UTF-16LE
  with a byte-order mark, exactly as Bridge sent it. A response is the HTTP body, BOM-less
  UTF-16LE, exactly as received, with the one exception in the next section. The HTTP head is not
  part of any file: every export was answered `HTTP/1.1 200 OK` with `Unicode: Yes`,
  `CONTENT-TYPE: text/xml; charset=utf-16`, `RESPSTATUS: 1` and a `CONTENT-LENGTH` equal to the
  body's length.
- **One file per distinct request.** Bridge sends most requests more than once in a call (paired
  reads and brackets). Every answer to the same request bytes was byte-identical within a call.
- **`shape_lab_*` files are shared by all three calls.** Nine requests were sent byte for byte in
  every call and answered byte for byte the same; they are kept once. The census and the window differ
  by day and are kept per day.
- **`*_answer.json` is a tool answer, not a wire capture.** It is the structured content the tool
  returned for the call, written with one-space indentation. Its `evidence` hashes are the tool's
  own, over everything it read.
- **`*_sequence.json` is a record, not a wire capture.** It lists every request of the call in
  order with its method, request id, hashes and the fixture that holds the response. The status
  reads' 51-byte bodies are described there and not kept as files.

## What was trimmed, and why

Four requests ask about every loaded company, and three lab companies were loaded. Each of those
four responses is trimmed **to this company's row** so that no other company's name, GUID, dates
or marks is in a public file. One rule, where a row is a `<COMPANY NAME="...">...</COMPANY>`
element: keep the bytes before the first row, the row of `BRIDGE SHAPE LAB`, and the bytes after
the last row; drop the other two rows and the separators between rows. Every kept byte is as
captured. The `CMPINFO` counters above the rows are Tally's own and are kept as received. A test
double replaying one of these files answers with a one-company list, not the three-company list
the gateway returned. The untrimmed responses are not committed.

The untrimmed responses, for the record (the fixtures' own sizes and hashes are in the table of files below):

- the response now held as `shape_lab_base_currency_response.utf16le.xml` was 4382 bytes as received, sha256 `f48ba7e7fdd383a2d10035286647e6b2746e5993ca1687cd7c1057fb59debc68`, with three company rows;
- the response now held as `shape_lab_book_extent_response.utf16le.xml` was 5652 bytes as received, sha256 `29a1dd8493e6026ff2cd7d85584a139f422ad2fc0ff1602ecacf456b00539695`, with three company rows;
- the response now held as `shape_lab_company_list_response.utf16le.xml` was 6140 bytes as received, sha256 `f0873d4fb61ac9465a234c1e38ae42e875ea846685cd0cba65e963b81b09846a`, with three company rows;
- the response now held as `shape_lab_company_marks_response.utf16le.xml` was 4326 bytes as received, sha256 `3298da50c42daeef5950a299c9921db808518b973aa6985906e16aa960a382da`, with three company rows.

In the sequence records, `response_body_sha256` is of the body as received; for these four the
fixture's own hash differs and is the one in the table below.

## Files

| file | bytes | sha256 | content |
|---|---|---|---|
| `cancelled_purchase_day_answer.json` | 2060 | `ad9f8de7f6582219565893d607983792fc6037732168b9e2552dbe63eac6ed5e` | TOOL ANSWER, not a wire capture: the structured content the tool returned |
| `cancelled_purchase_day_sequence.json` | 36582 | `e1275debf242f64b1e50a7ad6d6d704cd5119708a2d867d609d7d493124bf045` | record, not a wire capture: every request of the call in order, with hashes |
| `cancelled_purchase_day_voucher_census_request.utf16le.xml` | 1436 | `27a22111f294ee36efc81c0f7c69683408c596cf94938d47d311792511b7ae54` | request for the voucher census for the one day (`GUID, ALTERID, DATE`); wire bytes, UTF-16LE with a byte-order mark |
| `cancelled_purchase_day_voucher_census_response.utf16le.xml` | 5434 | `71c527b65edcb2aaee81561d581528ce3f620a10deaca764f6bd050fd2842973` | response: the voucher census for the one day (`GUID, ALTERID, DATE`); wire bytes |
| `cancelled_purchase_day_voucher_window_request.utf16le.xml` | 3088 | `a216c2be79685a0e935f62257ca550cae3d5081280c90347bf4d41076baae000` | request for the class-shaped voucher window for the one day; wire bytes, UTF-16LE with a byte-order mark |
| `cancelled_purchase_day_voucher_window_response.utf16le.xml` | 7734 | `5a275b35313efec202141229c43e867e51647cfb740b70c9d7edbf9a3d679bed` | response: the class-shaped voucher window for the one day; wire bytes |
| `credit_note_day_answer.json` | 3990 | `de90fdd4ff98f862d23dfc462d6e090500899ffd4bbb219d61f1986337d08b00` | TOOL ANSWER, not a wire capture: the structured content the tool returned |
| `credit_note_day_sequence.json` | 36550 | `37f390396eb4cbce6e4ec5f07e0020f008d45788afac39e69ac6ac4882d7769b` | record, not a wire capture: every request of the call in order, with hashes |
| `credit_note_day_voucher_census_request.utf16le.xml` | 1436 | `5793fcabdb94f48c005c77cd31d5dac4c548f372e5161f761b40774d7ccfed01` | request for the voucher census for the one day (`GUID, ALTERID, DATE`); wire bytes, UTF-16LE with a byte-order mark |
| `credit_note_day_voucher_census_response.utf16le.xml` | 5454 | `54ca29c0d72ffdd17272f6bf872e4de7740ce0a975a1d6a7f1833f0dfff26206` | response: the voucher census for the one day (`GUID, ALTERID, DATE`); wire bytes |
| `credit_note_day_voucher_window_request.utf16le.xml` | 3088 | `650c6b70316d222bd7c2614f211513732fd33b72dace5113bd765ac339c18892` | request for the class-shaped voucher window for the one day; wire bytes, UTF-16LE with a byte-order mark |
| `credit_note_day_voucher_window_response.utf16le.xml` | 64928 | `32713e59cce8e093af4378911fee2f838c94832a178686c3077ba60c6712fd66` | response: the class-shaped voucher window for the one day; wire bytes |
| `debit_note_day_answer.json` | 3298 | `79470ffef59e096329c95b9c1f5a84a075e9e335ceb406e7c16363fe38c88ef0` | TOOL ANSWER, not a wire capture: the structured content the tool returned |
| `debit_note_day_sequence.json` | 36552 | `c6669649b1873ea2d0d1bfdeb14ef5342684572cae6cd5fab9463c34988e98e3` | record, not a wire capture: every request of the call in order, with hashes |
| `debit_note_day_voucher_census_request.utf16le.xml` | 1436 | `433a524c8e76343dbb991e7fcd270be1d56825e73a1b9473f6acebe6b831e804` | request for the voucher census for the one day (`GUID, ALTERID, DATE`); wire bytes, UTF-16LE with a byte-order mark |
| `debit_note_day_voucher_census_response.utf16le.xml` | 5450 | `8bf751237defd84bdfe4dc7ae98ad9cb584be3f983694caefe0de359e1876e9d` | response: the voucher census for the one day (`GUID, ALTERID, DATE`); wire bytes |
| `debit_note_day_voucher_window_request.utf16le.xml` | 3088 | `f5d28faaecf79036f77c861a0750f0c80d52b6a8f87b7b30521dd66f12c50624` | request for the class-shaped voucher window for the one day; wire bytes, UTF-16LE with a byte-order mark |
| `debit_note_day_voucher_window_response.utf16le.xml` | 65002 | `54df13ea9ad1fb20948b46da2507a38562bd78fa699f21d5cdd3ba4db1e41508` | response: the class-shaped voucher window for the one day; wire bytes |
| `shape_lab_base_currency_request.utf16le.xml` | 956 | `fcaafea1508d3295ab61563ce43e586abe267551187b231432fabbfc5d5b979c` | request for the base-currency read (a `Company` collection fetching `NAME, GUID, CURRENCYNAME`); wire bytes, UTF-16LE with a byte-order mark |
| `shape_lab_base_currency_response.utf16le.xml` | 3414 | `dcba63be69e63616c194174180e7cef6337547f693d474334b366bd26b631c6e` | response: the base-currency read (a `Company` collection fetching `NAME, GUID, CURRENCYNAME`); wire bytes, trimmed to one company's row |
| `shape_lab_book_extent_request.utf16le.xml` | 1094 | `63aa3324c7b7fa133fa4d6df3bd25f0c46ac5bad97a0922181e64fed7f836866` | request for the book-extent listing; wire bytes, UTF-16LE with a byte-order mark |
| `shape_lab_book_extent_response.utf16le.xml` | 3838 | `2b299b3c9538993402bb04b030c4de666565cee095ed1ad975e3d38edfad1765` | response: the book-extent listing; wire bytes, trimmed to one company's row |
| `shape_lab_company_list_request.utf16le.xml` | 2524 | `9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8` | request for the company list; wire bytes, UTF-16LE with a byte-order mark |
| `shape_lab_company_list_response.utf16le.xml` | 4000 | `2ec27b087b4212056a2a8e1cd31badd77bd071284cbba96375451c904ec1de5a` | response: the company list; wire bytes, trimmed to one company's row |
| `shape_lab_company_marks_request.utf16le.xml` | 976 | `f556551d09682fd94d41b40bcf539d5d34bf06260d6da4bb44ad7023c598fcb1` | request for the company marks (master and voucher alteration marks); wire bytes, UTF-16LE with a byte-order mark |
| `shape_lab_company_marks_response.utf16le.xml` | 3396 | `a0f6ee08a536bd36aa2b6b3ed93299265c2141eae52e2a6e819cf4246861f3c4` | response: the company marks (master and voucher alteration marks); wire bytes, trimmed to one company's row |
| `shape_lab_currencies_1_request.utf16le.xml` | 966 | `e7807e21d1a179b44848879a9431e3547beb70f9d2ebae67e75cf80d3d40b659` | request for the currency masters, `FETCH` of `NAME, MAILINGNAME, DECIMALPLACES`; wire bytes, UTF-16LE with a byte-order mark |
| `shape_lab_currencies_1_response.utf16le.xml` | 3816 | `d793abca6e9d2cefe882ea656b514d5d0e963e30ce5af114430b7199edd273c8` | response: the currency masters, `FETCH` of `NAME, MAILINGNAME, DECIMALPLACES`; wire bytes |
| `shape_lab_currencies_2_request.utf16le.xml` | 994 | `806e65cb2aa25220448a351900b062bd6066eb741020ddd8bc7d938180f0a879` | request for the currency masters, the same plus `ORIGINALNAME`; wire bytes, UTF-16LE with a byte-order mark |
| `shape_lab_currencies_2_response.utf16le.xml` | 4024 | `2e72259c8a144799b04487f1346b0a6253c8b2506c6bc133a38f8e542df50fec` | response: the currency masters, the same plus `ORIGINALNAME`; wire bytes |
| `shape_lab_groups_request.utf16le.xml` | 1066 | `1a635c8b200ede795af39ad1abfa9ce3041c1341c561975bc623470d9db2dd0a` | request for the group listing; wire bytes, UTF-16LE with a byte-order mark |
| `shape_lab_groups_response.utf16le.xml` | 62904 | `c635ac45139e6321c3f00a645e9913d177f68176ee385f86f4b82b03dfc670e7` | response: the group listing; wire bytes |
| `shape_lab_ledgers_1_request.utf16le.xml` | 1736 | `61c9b17f265e3f1e9dc9d6997488dc41e65ae49a2850233e16df523abd041373` | request for the ledger listing with the compliance fields, the tax type and the GST duty head; wire bytes, UTF-16LE with a byte-order mark |
| `shape_lab_ledgers_1_response.utf16le.xml` | 85746 | `dd085b77a3fd05694ccd130bf65eff6558e89e5cc784f5b4ce607e09f0457c03` | response: the ledger listing with the compliance fields, the tax type and the GST duty head; wire bytes |
| `shape_lab_ledgers_2_request.utf16le.xml` | 1284 | `72ea0f6d407252333dad3bfb6cc2d569147833d4df690306de558253c3e72c2c` | request for the ledger listing with balances, the bill-wise flag and the currency; wire bytes, UTF-16LE with a byte-order mark |
| `shape_lab_ledgers_2_response.utf16le.xml` | 57668 | `f9b437b881baafd592f61b263b747c0f661e5ddcffdc0dbeb07b1f5323af75e0` | response: the ledger listing with balances, the bill-wise flag and the currency; wire bytes |

## What the captures show

- **The credit note (2025-04-29).** One voucher, type and class `Credit Note`, not an invoice-mode
  voucher, with an on-account bill allocation on the party. The tool returned it as one item with
  `status` `complete`: the party (under Sundry Debtors) `1180.00`; the sales ledger `-1000.00`
  under `taxable_entries`; two entries under `tax_in_books`, heads `cgst` and `state_tax`,
  `-90.00` each. The item carries `not_measured_live: ["credit_note"]` (the build that made the capture marked
  every credit note; a voucher-view credit note is no longer marked).
- **The debit note (2025-04-28).** One voucher, type and class `Debit Note`, party under Sundry
  Creditors `-1180.00`; the purchase ledger `1000.00`; tax `90.00` twice, heads `cgst` and
  `state_tax`; `status` `complete`.
- **Signs.** They are Tally's own: a debit is negative. A caller that sums a tax head across a
  window gets sales net of credit notes only because it adds signed amounts.

- **The cancelled purchase (2025-07-03).** One voucher, `Purchase` no. 9. The tool answered
  `complete` with no item and `vouchers_observed: 1`, and listed the voucher's identity (date,
  number, type, class, GUID, `cancelled: true`, `optional: false`, `post_dated: false`) under
  `purchase_vouchers_without_duties_taxes_entry`, total 1. No amount of the cancelled voucher is
  returned anywhere in the answer.

## Not settled

- One credit note, one debit note and one cancelled purchase, one run each, one synthetic book: this is not evidence about
  a client's book.
- Not shown: a credit note in invoice mode or with goods lines; an inter-state note (`igst`); a
  note against a bill reference; a cancelled or optional note; a cancelled sale; an optional voucher of either register's classes; a note whose duty ledger has no
  head or an unrecognised one; several vouchers in one window; paging.
- Why this book's state-side ledger carries `State Tax` and the other book's `SGST/UTGST` was not
  investigated: each is what its ledger master holds.
- The request counts are from two books, one run per day; how the count grows with a book's size
  beyond these two is not measured.
