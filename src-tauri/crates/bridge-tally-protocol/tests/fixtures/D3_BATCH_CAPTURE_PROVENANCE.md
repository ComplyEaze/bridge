# `d3-batch-*` — provenance

One `verify_import` of a 50-voucher batch that Bridge posted through its own post path, captured
byte for byte at the wire, with the journal and saved import file that post left behind. It is the
fixture for reviewing a doubted batch end to end (`acknowledge_post_review` over 50 vouchers).

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, `http://127.0.0.1:9001`, TallyPrime 7.1,
  education mode off. Only synthetic companies were loaded; no client or client-derived book.
- **Date:** 2026-09-26.
- **The post (slice D3):** a debug `bridge_mcp` built from the batch-posting branch at `c4cd4e08`, with
  posting and batch posting switched on, posted one saved batch of 50 Journals (1.00 to 50.00,
  Dr `Test Expense B` / Cr `Cash`, dated 2026-04-01) to a synthetic company after one native
  approval. Tally answered `CREATED 50` and nothing else; the company's voucher mark moved from
  1,419 to 1,469; all 50 read back verified.
- **The capture:** the same binary ran one `verify_import` of that batch through a pass-through
  proxy that wrote every request and response verbatim and forwarded each to completion. The four
  distinct request kinds were checked byte for byte against Bridge's own: the company extents and
  import-verification reads against the hashes Bridge recorded, and the company high-water and
  voucher census reads against the same requests emitted from Bridge's builders in a test. Every
  repeat of a request received the same response.
- **Local state, not wire bytes:** `d3-batch-journal.jsonl` is the post's journal and
  `d3-batch-import.xml` the saved import file, both as written. The test rewrites the journal's
  `endpoint_origin` to the simulator's address, since a dispatched batch verifies only on the
  origin it recorded. The step doubt the test reviews is written by the test itself; the live post
  had none.
- **Encoding:** the four responses are **BOM-less UTF-16LE**, exactly as received.

| file | bytes | sha256 |
|---|---|---|
| `agent/d3-batch-company-extent.utf16le.xml` | 7246 | `b9fe8d5febd4ae22eae2d11b408ec311fcec3a21410829d67cd37f9ebace907c` |
| `agent/d3-batch-company-high-water.utf16le.xml` | 4818 | `fda1afb616fed9adf8a3408f63bcd04094bb5bab16c275c882ccfbafda77996c` |
| `agent/d3-batch-voucher-census.utf16le.xml` | 124420 | `eb90841da8d2237f43fee668b5219a3bb287e064d85ff27b3536c4ac003166ae` |
| `agent/d3-batch-import-verification.utf16le.xml` | 253666 | `e2f34ff30b486869aa8560742a1be5ba4abb4251dd0850f9f4eb0e254321ce0f` |
| `agent/d3-batch-journal.jsonl` | 17410 | `922b2c13173ce23e39a13ed067bda5f87098ebc719e6f42dddefce3719289b54` |
| `agent/d3-batch-import.xml` | 31052 | `1b7ff4ddc901fcab949218b78005ad7bd8870ac5cd34f00465f1bb71712dcec5` |

## The census spanned to the mark (#2145)

Since #2145 the call sends this capture's census spanned to the voucher mark it read, `(0, 1469]`: the recorded census
request with ` AND $AlterID &gt; 0 AND $AlterID &lt;= 1469` added to its formula and nothing else changed, sha256
`3ae647206cdff8225eaaa74582c87ffb8a14a3cf6f7eb37df220987f3499f6d5` (`requests/a-d3batch_c2_spanned_0_1469.request.utf16le.xml`
on the run sheet `lab/2145-spanned-census-captures` at `6a968f01`). This capture holds no answer to that request, and an
answer captured for the unspanned census may not stand in for it (#899). The replay therefore still pins the recorded
request, and fails until the capture is re-recorded spanned in 2145-R.

The lab sent the recorded request and the spanned one to the same book on 30 Oct 2026 (`lab/2145-spanned-census-captures-answers`
at `993d96bc`), after the book had moved on: its voucher mark read 1829. Three of the capture's vouchers now carry ALTERIDs
above 1469 (1677, 1680 and 1685), as the sheet expected, so the spanned answer
(`08bdf64948cf5ab9bc78fd9bcfd17925276d54fbd5b430d5eb059681b3bc4f22`, 114,704 bytes) leaves them out, and neither answer stands
for this capture.

## What it establishes

- A 50-voucher readback of Bridge's own batch, as Tally returns it to the import-verification read:
  AlterIDs 1,420 to 1,469 in batch order, each voucher carrying its batch marker.
- The order of one `verify_import` on a book this size: a status probe and paired company-extent
  reads around the high-water, census and import-verification reads, 30 exchanges in all.
