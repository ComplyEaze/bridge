# Pilot lab fixtures

Answers Tally gave on 2026-10-09 on the synthetic company `BRIDGE PILOT LAB` (TallyPrime 7.1
Silver, licensed mode), sent by an outside contributor from his own machine, one request at a
time, each answer written straight to a file with `curl.exe -o` and never piped. The company was
the only one loaded. He published the answers on the branch `lab/1342-capture-answers` (commit
`4bceb5d0`) with a hash table; each file here is byte-equal to the file there and its SHA-256
equals the one in that table.

The data is synthetic: the company, its GSTIN (of the synthetic family the fixtures may carry)
and its ledgers were made for this measurement. One book, one release, one day.

| Fixture | Bytes | SHA-256 | What it is | Request |
| --- | ---: | --- | --- | --- |
| `pilot-lab-tax-units-typed-request.utf8.xml` | 11,152 | `f66ed7c325edc01e6d5d627e6c3490b6f7c3edf573ddf670d936e0783cbb9e40` | the company's tax units, all fields, read after the registration was entered and before any voucher was keyed: the Default Tax Unit and one GST registration, `Rajasthan Registration`, with one dated row from 2026-04-01 (Regular, not inactive); UTF-8 with CRLF line ends, as received; Tally's `&#4;` references as received | typed by hand from the text in the request list of #1342 (`b-taxunit.xml`, 426 bytes, SHA-256 `e5bbb2ab3e16838d8cb3a39c48d0d5683d5a8d565e099783af0c240d58d30081`, sent as UTF-8): the same collection, type and `NATIVEMETHOD *` as `render_company_registration_request`, under another collection name and in UTF-8, where the code sends UTF-16. An answer to the code's own request is owed (it was asked for in the same thread). |
