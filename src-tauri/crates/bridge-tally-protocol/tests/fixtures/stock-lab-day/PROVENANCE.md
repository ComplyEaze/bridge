# `stock-lab-day/`: a taxed Sales item invoice, read once through `sales_register`

Eighteen files from one live `sales_register` call on one synthetic company, for a day that holds
one taxed Sales item invoice. They are here for the register tool's test:
`agent_register_server_tests.rs` replays the call from its sequence record, with the ledger masters,
groups and company listings the call read, and compares the tool's row with the answer file.

## Three facts first

1. **The tool returned the invoice as one row, `complete`, with both tax heads taken from the ledger
   masters.** The party (under Sundry Debtors) `-118.00`; the sales ledger `100.00` under
   `taxable_entries`; two entries under `tax_in_books`, heads `cgst` (raw `CGST`) and
   `sgst_utgst` (raw `SGST/UTGST`), `9.00` each. `is_invoice` is `true` and the row carries no
   `not_measured_live` field.
2. **This is the first capture in which the register tool classified an item invoice against its own
   masters.** The earlier sales-day captures hold only the voucher window, read by the purchase
   register's build.
3. **The call cost 96 requests** (54 exports and 42 status reads) on a book with 8 ledgers and one
   currency. The same tool sent 118 on a larger book with two currencies (see `note-days/`).

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`,
  `http://127.0.0.1:9001` (a lab instance).
- **Date:** 2026-10-02, 13:03 IST.
- **Book:** `BRIDGE STOCK LAB`, a **synthetic** company seeded by this project: 8 ledgers, one
  currency, stock items and one imported Sales item invoice on the day read. Its ledger names are
  labels in a test book; the book holds no account number, GSTIN, PAN, address, e-mail or phone.
  The invoice was imported by this project, so this shows how the tool classifies Tally's own bytes
  for that voucher, not how a voucher typed on Tally's screen reads.
- **Sender:** `bridge_mcp`, a debug build made locally from commit
  `8c674d5e5f53bafe8829c2695ffe49fe2face11f` (the sales register's branch), binary sha256
  `35072848f4c0a7d86a35cdc8720ba9b473abca25d17031ea6ce22d4461458da6`.
- **Relay:** a serial read-only relay sat between the binary and the gateway and kept the exact
  request and response bytes of every exchange. It forwards only exports and refuses anything else.
  No request was refused. `tally_status` was read before and after the call and the loaded
  companies were the same.
- **The call:** `sales_register` with `from` and `to` `20250421`. One voucher on that day.
- **Requests: 96** (54 exports and 42 status reads). Exports by request id:
  `BridgeCompanyExtent` 18, `BridgeCompanyBookExtentV2` 16, `BridgeCompanyCurrencies` 4,
  `List of Ledgers` 8, `List of Groups` 4, `Bridge Agent Vouchers` 2,
  `Bridge Agent Company High Water` 2. There is no voucher census and no base-currency read: the
  book is below the read planner's threshold and has one currency.

## What each file is

- **`*_request.utf16le.xml` and `*_response.utf16le.xml` are wire bytes.** A request is UTF-16LE
  with a byte-order mark, exactly as Bridge sent it. A response is the HTTP body, BOM-less
  UTF-16LE, exactly as received, with the one exception in the next section. The HTTP head is not
  part of any file.
- **One file per distinct request.** Bridge sends most requests more than once in a call (paired
  reads and brackets). Every answer to the same request bytes was byte-identical within the call.
- **`stock_lab_taxed_day_answer.json` is a tool answer, not a wire capture.** It is the structured
  content the tool returned, written with one-space indentation. Its `evidence` hashes are the
  tool's own, over everything it read.
- **`stock_lab_taxed_day_sequence.json` is a record, not a wire capture.** It lists every request
  of the call in order with its method, request id, hashes and the fixture that holds the response.
  The status reads' bodies are described there and not kept as files.

## What was trimmed, and why

Three requests ask about every loaded company, and three lab companies were loaded. Each of those
three responses is trimmed **to this company's row** so that no other company's name, GUID, dates
or marks is in a public file. One rule, where a row is a `<COMPANY NAME="...">...</COMPANY>`
element: keep the bytes before the first row, the row of `BRIDGE STOCK LAB`, and the bytes after
the last row; drop the other two rows and the separators between rows. Every kept byte is as
captured. A test double replaying one of these files answers with a one-company list, not the
three-company list the gateway returned. The untrimmed responses are not committed.

The untrimmed responses, for the record (the fixtures' own sizes and hashes are in the table of files below):

- the response now held as `stock_lab_taxed_day_book_extent_response.utf16le.xml` was 5652 bytes as received, sha256 `fbbef30973a9220045033d125bde323613328715d543002215568ae9028358c1`, with three company rows;
- the response now held as `stock_lab_taxed_day_company_list_response.utf16le.xml` was 6140 bytes as received, sha256 `f0873d4fb61ac9465a234c1e38ae42e875ea846685cd0cba65e963b81b09846a`, with three company rows;
- the response now held as `stock_lab_taxed_day_company_marks_response.utf16le.xml` was 4326 bytes as received, sha256 `512ff1552725e16b1374501f92f57fba2cbef1034d61373573bff8b9851fd09e`, with three company rows;

In the sequence record, `response_body_sha256` is of the body as received; for these three the
fixture's own hash differs and is the one in the table below.

## Files

| file | bytes | sha256 | content |
|---|---|---|---|
| `stock_lab_taxed_day_answer.json` | 4752 | `6cda7a8a86ed55a58e280202fe6bc2c8d32612b86dc1284be826cfb3aa2d933e` | TOOL ANSWER, not a wire capture: the structured content the tool returned |
| `stock_lab_taxed_day_book_extent_request.utf16le.xml` | 1094 | `bc4bf86cdc654026343bfe819e4de86688b77eac2e2d5677453560601e10f439` | request for the book-extent listing; wire bytes, UTF-16LE with a byte-order mark |
| `stock_lab_taxed_day_book_extent_response.utf16le.xml` | 3834 | `da4fa71c9cfaf799eaa47580c94e5fcc5bebeacbc0bfeb66cef25be738b5ec1c` | response: the book-extent listing; wire bytes, trimmed to one company's row |
| `stock_lab_taxed_day_company_list_request.utf16le.xml` | 2524 | `9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8` | request for the company list; wire bytes, UTF-16LE with a byte-order mark |
| `stock_lab_taxed_day_company_list_response.utf16le.xml` | 4000 | `b724b342a59981629a22bf6d6d6802863150af802728a293ea603a9060510c11` | response: the company list; wire bytes, trimmed to one company's row |
| `stock_lab_taxed_day_company_marks_request.utf16le.xml` | 976 | `a98987c65a39652c9a0703af5cb613490473267b7170b0aeb6ca7a36f2c15848` | request for the company marks (master and voucher alteration marks); wire bytes, UTF-16LE with a byte-order mark |
| `stock_lab_taxed_day_company_marks_response.utf16le.xml` | 3392 | `8a08c7b94fc6364fa212055ee5469509ea98d37361484ab7f6ae9f1d874d3dec` | response: the company marks (master and voucher alteration marks); wire bytes, trimmed to one company's row |
| `stock_lab_taxed_day_currencies_request.utf16le.xml` | 966 | `339d8214cdbd210fba8d3bf1206f20b64b1489cd7c2287102196903d486f72d2` | request for the currency masters; wire bytes, UTF-16LE with a byte-order mark |
| `stock_lab_taxed_day_currencies_response.utf16le.xml` | 3400 | `52ae8659722ae041c69cedec495bfcf83eac1c5e326f1c7fddbdf9fd150bd0c5` | response: the currency masters; wire bytes |
| `stock_lab_taxed_day_groups_request.utf16le.xml` | 1066 | `d5d16a41d14e67cabb484083281470b4cfd89fb10c551a3e18ff02a57acb62a3` | request for the group listing; wire bytes, UTF-16LE with a byte-order mark |
| `stock_lab_taxed_day_groups_response.utf16le.xml` | 54280 | `03c3624408d000da234e54aacf61c1339dd0ee5867d119e48246dd153b1dd8e8` | response: the group listing; wire bytes |
| `stock_lab_taxed_day_ledgers_1_request.utf16le.xml` | 1736 | `93cc734025f89f1c4959503fd16ff3fcea515c3340fec2b9fcd226cf1551d282` | request for the ledger listing with the compliance fields, the tax type and the GST duty head; wire bytes, UTF-16LE with a byte-order mark |
| `stock_lab_taxed_day_ledgers_1_response.utf16le.xml` | 18126 | `d4dbd6231650b4e81744b13a48ff854edb6eb0c03ad061ee1e8ba28bd479ea66` | response: the ledger listing with the compliance fields, the tax type and the GST duty head; wire bytes |
| `stock_lab_taxed_day_ledgers_2_request.utf16le.xml` | 1284 | `6a3e975a469cafe5acfc664c4fe3c882ca45105f977560e5890a569a63928aec` | request for the ledger listing with balances, the bill-wise flag and the currency; wire bytes, UTF-16LE with a byte-order mark |
| `stock_lab_taxed_day_ledgers_2_response.utf16le.xml` | 12870 | `cb9d0b0ef667e89b3dbeba22b61a2938d989563e16888d1ecc262b2a2039624c` | response: the ledger listing with balances, the bill-wise flag and the currency; wire bytes |
| `stock_lab_taxed_day_sequence.json` | 30296 | `9604a8caac04a86d8007e9c5a094d875765546f8c6648ec31858207c7dac78c5` | record, not a wire capture: every request of the call in order, with hashes |
| `stock_lab_taxed_day_voucher_window_request.utf16le.xml` | 3088 | `32bc080b2715f9f570e65189b66e225889877fb4f056f387e9fc0979ac2a2632` | request for the class-shaped voucher window for the one day; wire bytes, UTF-16LE with a byte-order mark |
| `stock_lab_taxed_day_voucher_window_response.utf16le.xml` | 76864 | `04d28380632caa5c02aeceda49793690e82ee21418f9a4f7b7a2b6cd6d7a3488` | response: the class-shaped voucher window for the one day; wire bytes |

## Not settled

- One taxed item invoice, one run, one synthetic book, imported by this project: this is not
  evidence about a client's book or about a sale typed on Tally's screen.
- The untaxed Sales item invoice of the other day on this book was read once live by an earlier
  build; those bytes are not committed here and nothing in this directory shows that read.
- Not shown: an inter-state (`igst`) line, a company with a registration, a tax Tally computes
  itself, an invoice with several goods lines, a duty ledger with no head or an unrecognised one,
  more than one voucher in a window, paging.
