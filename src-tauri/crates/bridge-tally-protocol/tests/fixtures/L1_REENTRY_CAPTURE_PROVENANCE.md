# `l1-reentry-*` — provenance

One `verify_import` of a 50-voucher batch that Bridge had posted and verified 50 of 50, captured
byte for byte at the wire after a person cancelled one of its vouchers in Tally's own screen and
then entered an identical voucher by hand without Bridge's marker. It is the fixture for bridge#806:
a cancelled batch voucher whose effective re-entry the verification proof does not report. The
code for #806 is not part of this change; the capture comes first.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, `http://127.0.0.1:9001`, TallyPrime 7.1,
  education mode off. Only synthetic companies were loaded (the company listings in the extent
  and high-water answers name ten, every one starting `BRIDGE `); no client or client-derived
  book. The batch was posted to `BRIDGE AMEND LAB`.
- **Date:** 2026-09-28: the batch was built at about 11:35 IST and the capture taken at about 11:47.
- **The post:** a debug `bridge_mcp` built from master `79cc54ef` (sha256
  `6e7a6bfe0ce23ab610497c1a1a650d90f2d3596369e6188c559c76224edfc1db`) posted one saved batch of 50
  Journals (1.00 to 50.00, dated 2026-06-17) after one native approval. Tally answered `CREATED 50`
  with every other counter 0, and all 50 read back verified. The company's voucher mark was 1,738
  before the post (`alter_id_delta.before`).
- **What a person did at the screen, between the post and the capture** (as reported on #806):
  1. cancelled the batch's 50.00 Journal, voucher 352, which carries Bridge's narration marker;
  2. entered a new Journal with the same date, ledgers and amount, narration `manual re-entry`
     and no marker. Tally numbered it 353.
- **The capture:** the same binary ran one `verify_import` of that batch through a pass-through
  proxy that wrote every request and response verbatim and forwarded each to completion: 30
  exchanges, all status 200, 5.1 s from the first request to the last. The four distinct POST bodies were sent 11, 2, 2 and 4
  times; every repeat of a request received the same response. The proof's own record of the
  company-extent request and response hashes equals the first of these.
- **Local state, not wire bytes:** `l1-reentry-journal.jsonl` is the batch's journal as written
  (eight lines; its `endpoint_origin` is the proxy's loopback address) and `l1-reentry-import.xml`
  the saved import file, whose sha256 is the journal's `batch_sha256`.
- **Encoding:** the four responses are **BOM-less UTF-16LE**, exactly as received. Nothing is
  derived or edited.

| file | bytes | sha256 |
|---|---|---|
| `agent/l1-reentry-company-extent.utf16le.xml` | 13740 | `8c21e3b2154357b7b5a051178d9c17d3ac15aa84b99bab975f2aad6172b52b4c` |
| `agent/l1-reentry-company-high-water.utf16le.xml` | 7378 | `fb14725f8f9e2b59bc516b4539cd1f09776219529d5ab6d3b0b774e5380d3d47` |
| `agent/l1-reentry-voucher-census.utf16le.xml` | 126960 | `a46f499f37692b910c070c80d95b01d136082f15bb2ec35bec3814561af966d6` |
| `agent/l1-reentry-import-verification.utf16le.xml` | 257094 | `f4e2743372adc8e044672bcb3e2b9614f148f6ab2cfad84d4ad466cf7287ecfa` |
| `agent/l1-reentry-journal.jsonl` | 18034 | `43410a22735001b5d0f585899a097020da9cd5ef22629c36d0e80e510e9a87da` |
| `agent/l1-reentry-import.xml` | 31152 | `0c37efaca462e617b0c430a8a442fe24f651e14307abe15a8574a2fea3780cfd` |

The four distinct request bodies, by sha256 (the requests themselves are not committed):

| request | sha256 |
|---|---|
| company extent | `9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8` |
| company high-water | `0930288f6eb531926d018cc4762288084831b8fad16a554e48fafbd694e245c2` |
| voucher census | `e58e009cb88cb483f89723370fcf89fb0de3aff7db29d1bccb41757826c3dc86` |
| import verification | `0fe49c89693f59aa011a488f86c2beb97b8f24a87cf32c2aa448e4c51f205418` |

## What it establishes (read from the bytes)

- The import-verification read holds **51 vouchers**, each with a distinct REMOTEID: 50 carry
  Bridge's `[BRIDGE:…]` narration marker and one does not. The census holds the same 51 REMOTEIDs.
  AlterIDs run from 1,739 to 1,790.
- **Voucher 352** (marked, number 352, dated 2026-06-17): `ISCANCELLED` Yes, ALTERID 1,789,
  MASTERID 1,719, its marker still in the narration, an empty `ALLLEDGERENTRIES.LIST` and an
  empty `PERSISTEDVIEW`. Cancelling dropped its entries, as in `d3-cancelled-*`.
- **Voucher 353** (unmarked, narration `manual re-entry`, Journal, dated 2026-06-17, effective
  the same day): `ISCANCELLED` No, `ISOPTIONAL` No, ALTERID 1,790, MASTERID 1,720, both above the
  pre-import mark of 1,738. Its entries are `Test Expense B` −50.00 with `ISDEEMEDPOSITIVE` Yes and
  `Cash` 50.00 with `ISDEEMEDPOSITIVE` No: the ledgers, amounts and signs the build wrote for
  voucher 352.
- The proof Bridge produced from these bytes (the saved proof of that run) counts
  `posted_verified` 49 and `posted_not_effective` 1, with `verification_status` `verification_incomplete` and both `duplicates` and
  `unrelated_duplicates_in_window` empty; the proof names no voucher 353.

## Limits

One run, one book, one release (TallyPrime 7.1 Silver), for a cancel and a re-entry made in
Tally's own screens. The fingerprint of voucher 353 has not been computed by Bridge's own
function; the first test written from this capture does that. Whether a re-entry of a voucher
other than a Journal reads back the same way is not measured. The narrations carry the batch's
lab label, as in `d3-batch-*`.
