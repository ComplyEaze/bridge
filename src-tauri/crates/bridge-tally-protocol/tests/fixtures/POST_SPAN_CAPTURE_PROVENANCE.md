# `post-span-*`: provenance

One raw gateway import of 10 untagged Payment, Receipt and Contra vouchers. Around it are the
company high-water reads, the read of the import's own AlterID span, and two reads filtered by
`$GUID`. It is the fixture for binding a posted voucher to its Tally GUID by its position inside
its own POST's AlterID span, with no narration tag.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, `http://127.0.0.1:9001`, TallyPrime 7.1,
  education mode off. Only synthetic companies were loaded (`BRIDGE AMEND LAB`, `BRIDGE SHAPE LAB`,
  `BRIDGE STOCK LAB`); no client or client-derived book.
- **Date:** 2026-10-02, 13:15:36 to 13:16:19 IST.
- **The import:** `post-span-import.xml`, sent once as BOM-prefixed UTF-16LE.
  - **Shape:** Bridge's native voucher request shape (`render_import_envelope` plus `render_voucher_xml`), with the narration carrying **no** `[BRIDGE:…]` tag.
  - **Fidelity check:** the same renderer, given the tag back, reproduced the committed Bridge-sent `wa1-payment-import.xml` byte for byte (1,037 bytes).
  - **Vouchers:** ten, dated 2026-07-10, between the synthetic ledgers `Test Bank`, `Cash`, `Test Party`, `Test Expense A` and `Test Expense B`. Vouchers 2 and 3 are identical in every element except the REMOTEID.
  - **Not Bridge's post path:** it was not sent through `post_import`; a lab script sent it under a recorded grant naming this file's sha256.
- **The reads,** each sent once, in this order:
  1. the company high-water read (`render_agent_company_high_water`) **before**;
  2. the import;
  3. the high-water read **after**;
  4. the verification read narrowed to `$AlterID > 1795 AND $AlterID <= 1805`
     (`render_import_verification_in_span` with an `AlterIdSpan`);
  5. the same read without the span (whole day; its response was **byte-identical** to read 4,
     sha256 `d83f822c…1f8f`, so it is not kept separately);
  6. the whole-day read with ` AND $GUID = "<voucher 2's GUID>"` appended to the formula;
  7. the same with a well-formed GUID no voucher carries (`<company GUID>-00fffff0`).
- **Encoding:** the responses are **BOM-less UTF-16LE**, exactly as received. The import request
  is the UTF-8 file as rendered; the wire form adds a BOM and re-encodes it as UTF-16LE.

| file | bytes | sha256 |
|---|---|---|
| `agent/post-span-import.xml` | 6984 | `eefb12173b86f5b4fb41999151309509ffad774dc1c08d00e1283e8bc325d181` |
| `agent/post-span-import-response.utf16le.xml` | 546 | `de1c549bb6c62e8569e1df57a11e1d29cabf603f2e0b8d75f8bd910ef89ee440` |
| `agent/post-span-company-high-water-before.utf16le.xml` | 4326 | `9310573bc5ba997919d5c7a203d85cf82742ed01462b01f7b2e5c58be1ea920f` |
| `agent/post-span-company-high-water-after.utf16le.xml` | 4336 | `3491c3bf30c08ac5c90ca5843c2a2ec4d21245281585eff3b0152f47c9cd8203` |
| `agent/post-span-alterid-span-read.utf16le.xml` | 56668 | `d83f822c558d4090925b6ef808da84a6320cc9e439c0a7bae535fa08fa0c1f8f` |
| `agent/post-span-guid-present.utf16le.xml` | 7972 | `a1f1857839d5ebd0837e9c941667495bf5266eaf270d5f547ca8e89b13160b4f` |
| `agent/post-span-guid-absent.utf16le.xml` | 3032 | `59140a59a2d8f027bfa0bc821bd401698b0facc2ef252e3cc1d90ffcbaa97917` |

## What it establishes

- **Counters:** `CREATED 10`; every other counter 0; no `LINEERROR`; all seven counters present; `LASTVCHID 1733`.
- **The mark:** the target company's `ALTVCHID` went from 1795 to 1805, a step of exactly 10. `ALTMSTID` stayed at 234. The other two companies' marks did not move.
- **The span:** it holds exactly the ten vouchers.
  - ALTERIDs are 1796 to 1805 in request order, the identical pair at 1797 and 1798.
  - MASTERIDs are 1724 to 1733 in request order; the last equals `LASTVCHID`.
  - Type, date, `EFFECTIVEDATE`, signed entries and the untagged narration read back as sent.
- **The GUID filter:** `$GUID` returns exactly one voucher for a present GUID (voucher 2, not its twin) and an empty collection for an absent one.

## What it does not establish

- Bridge's own post path.
- Journals in this untagged shape: see `post-span-journal-*` below.
- A concurrent writer, or another Gold client process.
- Whether a GUID survives a later edit.
- Batches other than 10.
- A `$GUID` filter without a date clause.
- Any other release or licence tier.

# `post-span-journal-*`: provenance

One raw gateway import of 5 untagged Journal vouchers, with the company high-water reads around
it and the read of its own AlterID span. It extends the binding fixture above to Journals.

## Provenance

- **Host / gateway:** as above; only synthetic companies were loaded (`BRIDGE AMEND LAB`,
  `BRIDGE CORPUS DENSE`, `BRIDGE SHAPE LAB`, `BRIDGE STOCK LAB`).
- **Date:** 2026-10-02, 16:45:00 to 16:47:46 IST; the import at 16:46:08.
- **The import:** `post-span-journal-import.xml`, sent once as BOM-prefixed UTF-16LE, under a recorded grant naming
  its sha256. Bridge's untagged Journal shape (no `EFFECTIVEDATE`, no `PARTYLEDGERNAME`, the caller's entry order):
  the test `five_captured_journals_are_bridges_own_request_and_bind_in_request_order` renders the same bytes with
  `render_native_vouchers_xml`. Five Journals dated 2026-07-11, an empty day before the import: a two-leg accrual,
  an identical pair (vouchers 2 and 3), a reversal and a three-leg split, between `Test Party`, `Test Expense A` and
  `Test Expense B`. Not sent through `post_import`.
- **The reads,** each sent once, in this order: the whole day before the import (0 vouchers); the company list
  (only the four synthetic companies); the high-water read **before**; the import; the high-water read **after**;
  the verification read narrowed to `$AlterID > 1805 AND $AlterID <= 1810`; the whole day (its response was
  **byte-identical** to the span read, sha256 `740767430ad7fd5b28971033eddbb7f758a24cd03d584efdee461f5f93a8d119`, so it is not kept separately); the high-water read again
  (identical to the read after).
- **Encoding:** as above.

| file | bytes | sha256 |
|---|---|---|
| `agent/post-span-journal-import.xml` | 3422 | `69c8aa0225f67d402072b1b9015f2517a08b6a57bc741c1543737b2517a0ca4b` |
| `agent/post-span-journal-import-response.utf16le.xml` | 544 | `da3d1e4659fcf6f5ce2473c018b596df188ff6ba0d6176b3b79fc2ac3b3841ba` |
| `agent/post-span-journal-company-high-water-before.utf16le.xml` | 4812 | `eb4ed4308c42a6c85af837b4fd28d4a1f84a67d116efb4b58cdf5ed25aa395c5` |
| `agent/post-span-journal-company-high-water-after.utf16le.xml` | 4812 | `b53fcde92bb6c1d74c02fbfcd2e81f3771ba57c5bc4b985cde5c440849205f7d` |
| `agent/post-span-journal-alterid-span-read.utf16le.xml` | 33936 | `740767430ad7fd5b28971033eddbb7f758a24cd03d584efdee461f5f93a8d119` |

## What it establishes

- **Counters:** `CREATED 5`; every other counter 0; no `LINEERROR`; `LASTVCHID 1738`.
- **The mark:** `ALTVCHID` went from 1805 to 1810, a step of exactly 5; `ALTMSTID` stayed at 234; the other three
  companies' marks did not move.
- **The span:** exactly the five Journals, ALTERIDs 1806 to 1810 and MASTERIDs 1734 to 1738 in request order (the
  identical pair at 1807 and 1808), the last MASTERID equal to `LASTVCHID`; narration and signed entries as sent,
  the three-leg split included. Tally filled `EFFECTIVEDATE`, which was not sent, with the date.

## What it does not establish

As above, except that Journals in this untagged shape are now measured, once.
