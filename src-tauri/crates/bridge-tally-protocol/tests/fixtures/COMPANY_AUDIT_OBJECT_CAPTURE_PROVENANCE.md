# `company_audit_object_batch_lab_*` — provenance

A live capture of the tally-read v1 company part: one single-object `Company` export (`audit_company_object_v1`, a `FETCHLIST` of `GUID`, `NAME`, `BOOKSFROM`, `ISINTEGRATED`) on one synthetic lab company, with the request that was sent. `admit_audit_company_part` admits the answer (protocol reference §9.11e).

## Provenance

- **Host / gateway:** TallyPrime 7.1 (`PRODMAJORREL`/`PRODMINORREL` in the answer), a lab instance. Its licence and edition are not recorded in this capture.
- **Source:** lab evidence commit `2af546bce1f7` on branch `lab/4c-audit-company-read`. The request bytes were sent with curl, not Bridge's transport.
- **Date:** 2026-10-12. One request.
- **Book:** `BRIDGE BATCH LAB`, a synthetic lab company. The company name and GUID are synthetic.
- **Request:** the exact bytes tabled below, UTF-16LE with a BOM. A test asserts that `ReadOnlyProfile::AuditCompanyObjectV1`, rendered for this company and encoded by `encode_tally_xml_request_utf16le`, produces them (`the_request_is_byte_for_byte_the_one_sent_live` in `audit_company_part_tests.rs`).
- **Encoding:** the response is **BOM-less UTF-16LE**, exactly as received, answered with HTTP 200. `.gitattributes` marks this tree `-text`.
- **What the capture shows:**
  - `HEADER/STATUS` is `1`, and only CRLF follows `</ENVELOPE>`.
  - Exactly one `DATA/TALLYMESSAGE`, holding one `COMPANY` child. Its `NAME` attribute is `BRIDGE BATCH LAB`.
  - `GUID` (`a30f23c3-4819-4334-96ab-ffa6fba70976`) and `BOOKSFROM` (`20260401`) occur once each in the whole document.
  - The only other `COMPANY` is `DESC/CMPINFO/COMPANY`, which reads `2` and has no children.
  - The `COMPANY` has seven children: `BOOKSFROM`, `NAME`, `GUID`, `ISINTEGRATED` (`No`), `ISDEEMEDPOSITIVE` (empty), `CANDELETE` (`No`) and `MASTERID` (`29`). The answer has 56 distinct tags; a `FETCH *` answer from a different book had 565 (§9.11a), and no same-book `FETCH *` was sent.
- **Not established:** that the answer is byte-stable across two sends (the `CMPINFO` counters could differ); Bridge's own transport and a live admission call; an `ID` that does not resolve; a name with `&`, quotes or non-ASCII text; a book that is a year-split sibling; a book changing during the read; other releases, editions and hosts.

| file | bytes | sha256 | content |
|---|---|---|---|
| `company_audit_object_batch_lab_request.utf16le.xml` | 896 | `55b145be1d140f86b0f312f41d055ef7697cf9daa41c897ad4f384cac6a2803f` | the request (UTF-16LE with a BOM) |
| `company_audit_object_batch_lab_live.utf16le.xml` | 4216 | `960d98d4908eb72b512ceddc912b37e4e1c55efe243b13ac2da7f73de4fe94fe` | its answer: STATUS 1, one `COMPANY` with seven children |
