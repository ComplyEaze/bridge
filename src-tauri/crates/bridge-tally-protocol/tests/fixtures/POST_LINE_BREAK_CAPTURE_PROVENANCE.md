# `post-line-break-ledger-readback`: provenance

The read-back of one Journal posted through `post_import` whose debit ledger's stored name ends in
one CR LF (bridge#626). It is the fixture for reading such a name back from Tally with its break.

## Provenance

- **Host / gateway:** TallyPrime 7.1 **Silver (licensed)**, Windows 10 22H2,
  `http://127.0.0.1:9001`, education mode off. Only the synthetic company `BRIDGE LINE BREAK LAB`
  (company 100042) was loaded; no client or client-derived book.
- **Date:** 2026-10-10, about 09:51 to 09:53 IST (attempt 3 of the Go on pull request #1386).
- **The post:** a release build of `bridge_mcp` from `7e9c7c5d7d092093c0f92e7a5eeb189f35e23b79`
  (SHA-256 `49e7a636364365ef6197ad0a31a26340f6c04e35431f9190facd9eeb0129b7ed`) posted one Journal
  dated 2026-08-03, `Lab Line Break Ledger` + CR LF debit and `Lab Plain Ledger` credit:
  `dispatch.state` `posted_verified`, `created` 1 and every other counter 0,
  `post_span_binding.state` `bound`.
- **The read:** step 6, sent once by `curl.exe` after the post: the one-day voucher read that
  ComplyEaze Bridge's verification makes, for 2026-08-03 (request `r1386-readback.xml`, 1,832 bytes,
  SHA-256 `8d5529ce48983112022ff6f366e12640a87d9d2862f1291705782b64f1be2327`). HTTP 200,
  `CONTENT-TYPE: text/xml; charset=utf-16`.
- **Encoding:** **BOM-less UTF-16LE**, exactly as received.
- **Source of the bytes:** `lab/1386-capture-answers/a6-readback.xml` in the contributor's commit
  `269c404a815ae46dfb9bbaed782d07196ca2b344` (fork `akshit-khandelwal47/bridge`, branch
  `lab/1386-capture-answers`), whose `SHA256.txt` lists the same size and hash. Reported on #1386,
  comment 6093748767.

| file | bytes | sha256 |
|---|---|---|
| `agent/post-line-break-ledger-readback.utf16le.xml` | 7966 | `5da83659340d7f9251684f7fc882c354b64fef56dd7f605db8ed26f254eec81b` |

## What it establishes

- Tally writes the stored name's trailing CR LF back as the text `&#13;&#10;` inside
  `LEDGERNAME`; the other ledger's name has no break.
- The read holds the posted Journal and its two ledger entries.

## What it does not establish

- A read-back with the break as the two raw characters CR and LF: not observed. The test reads
  that spelling too, as a variant of this capture with that one value changed.
- A break anywhere but at the end of a name, or any break other than one CR LF.
- How the approval or review dialog looks on macOS.
- Any other release or licence tier.
