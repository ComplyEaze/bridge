# `import_post_partial_landing_*`: provenance

Three posts sent through Bridge's own `post_import`, each request and answer kept as wire bytes by
a relaying proxy. Each post held one Journal that Tally refused because it debited a ledger under
Stock-in-Hand. The first post held only that voucher. The second and third each held two more
Journals, which Tally created. These are the first captured answers of a batch that landed in part
through Bridge's own post path (bridge#1108).

## Provenance

- **Host / gateway:** TallyPrime single-user edition, release 7.2, reached through a proxy that
  relayed each request unchanged and kept each import's request and answer whole.
- **Bridge:** a master build reporting `bridge-tally` 0.7.0 (not the 0.7.0 release), with an
  empty data folder. Each post was approved in the approval box.
- **Date:** 2026-11-11, one sitting, each post sent once.
- **Company:** `BRIDGE STOCK CREATE LAB`, a synthetic company with accounts integrated with
  inventory. No client or client-derived book was loaded.
- **The posts:**
  - **p1:** one Journal debiting `A1 Closing Stock` (under Stock-in-Hand).
  - **p2:** three Journals in this order: one between synthetic expense and capital ledgers, the
    Stock-in-Hand Journal, then one more expense Journal of a different amount. The narrations
    differ.
  - **p3:** three Journals in this order: an expense Journal, the Stock-in-Hand Journal, then a
    voucher identical to the first in every element except its `REMOTEID`.
- **Requests:** as Bridge rendered and sent them. The `Import Data` envelope is pinned to the
  company by name. Each voucher carries a `REMOTEID` and no voucher number. They are
  **BOM-prefixed UTF-16LE** on the wire.
- **Answers:** **BOM-less UTF-16LE**, exactly as received. Each body's length equals its
  `CONTENT-LENGTH`. The HTTP headers are not kept here. `.gitattributes` does not normalise this
  tree.
- **Source:** the lab answers branch `lab/1108-a2-partial-landing-answers` (`ebf70249`), files
  `sitting-2/proxy/import-1` to `import-3`, `.request` and `.answer`, with the HTTP head removed.
  The bytes are otherwise unchanged.

| file | bytes | sha256 |
|---|---|---|
| `import_post_partial_landing_p1_request.utf16le.xml` | 1846 | `2d9d896ef4b6b957740fa48e627e47a6f041f574aa3b216ad0ca40ec4eb084f4` |
| `import_post_partial_landing_p1_live.utf16le.xml` | 726 | `2dc624f4f110c2e32d1d38245fb2e4169058b6e8f6c50ff62e0f8d1d8e730728` |
| `import_post_partial_landing_p2_request.utf16le.xml` | 4150 | `0423accc5c75b9bded37508f4750729213d3ff2c8445dc7247ad169306183253` |
| `import_post_partial_landing_p2_live.utf16le.xml` | 726 | `7ba9450729a82eb7a20667baf382f62ae93da2d11332157c87d25f4b64b62b4b` |
| `import_post_partial_landing_p3_request.utf16le.xml` | 4146 | `d673c2dcc5cc30331f4df4304a0f6aab5fa9cc99770d1e7a4522312a2547621d` |
| `import_post_partial_landing_p3_live.utf16le.xml` | 726 | `609cb603488a334e90a28b52919327d8062c51177af02d4e5e7d49b3ad5ca850` |

## What it establishes

- **p1:** `CREATED 0`, `EXCEPTIONS 1`, `LASTVCHID 0`.
- **p2:** `CREATED 2`, `EXCEPTIONS 1`, `LASTVCHID 3`.
- **p3:** `CREATED 2`, `EXCEPTIONS 1`, `LASTVCHID 5`.
- **In all three answers:**
  - `ALTERED`, `DELETED`, `IGNORED`, `ERRORS`, `CANCELLED`, `COMBINED` and `LASTMID` are 0, and all
    seven counters are present.
  - There is one `LINEERROR`, naming the Stock-in-Hand ledger, not a voucher.
- **What the lab's reads around the posts showed** (they are not committed here):
  - **Marks:** the voucher mark stepped by `CREATED` each time and the master mark did not move.
  - **p2:** the two created vouchers took AlterID and MasterID 2 and 3, in request order, and no
    row was found for the refused one.
  - **p3:** both identical vouchers were created.
- **Counting:** `EXCEPTIONS` counted the refused voucher for this second cause, as it did for a
  missing ledger (protocol reference §9.2).

## What it does not establish

- **Which voucher was refused:** no answer names it (no index, no `REMOTEID`, no number).
- **Bridge's own reads:** the span, mark and verification reads of these posts were not kept as
  bytes, so no read-back is committed.
- **Not measured:**
  - a refused voucher first or last in a batch;
  - two refusals through `post_import`;
  - a refusal of one of two identical vouchers;
  - another release, a multi-user edition, or a voucher type other than Journal.
- **Repeatability:** each post ran once.
