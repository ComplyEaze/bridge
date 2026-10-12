# 1050-B: what identifies a natively posted voucher after a restore (Windows lab, 12 Oct 2026)

Requests: lab/1050-identity-requests e0d61b81. All five request files matched SHA256.txt. The three templates were filled with `fill.ps1`, which replaces only the placeholder (UTF-16 with BOM); `checkfill.py` confirms each filled file equals its template byte for byte apart from the placeholder. Filled hashes are in `filled/FILLED-SHA256.txt`.

Each request was sent once with curl. Every answer is HTTP 200 with curl exit 0, a complete ENVELOPE with one COLLECTION and no LINEERROR. Exit codes and printed lines are in `answers/log.txt`.

Book: new synthetic BRIDGE IDENTITY LAB (company number 100056, GUID dcac52b6-2168-43fa-be2a-95b1488c11b4).
- Books from 1-Apr-2026; no GST, no inventory, automatic numbering.
- Ledgers: Synthetic Bank Ledger (Bank Accounts), Party A (Sundry Debtors, bill-by-bill No), Lab Seed Income (Indirect Incomes).
- Seed Receipt No. 1, 1-Jul-2026, Cash Dr / Lab Seed Income Cr 1,000.00, "Seed voucher".
- Loaded alone at every step.

Build: bridge_mcp.exe 411f7cca80ca4a5fee543f4f44d14b911ce23e982169785b1762d8d50472e5a6 (installed extension). TallyPrime 7.1 Silver (`b0-tally-status.json`).

CLICK MODE: lane. The Bridge dialog's Yes was clicked with the mouse.

## Steps

| Step | What | Result |
|---|---|---|
| 1 | restore-plan.txt written; backup I0 | TDBK1800_100056.001, 25,111 B, sha256 49ed631d851d7611c7bc2aa86bd7b403928c03adfa21a6fad84eb38ccd681470, 10:13:57Z |
| 2 | N1 native post: Receipt 2026-07-10, Dr Synthetic Bank Ledger 301.00 / Cr Party A 301.00, "Lab identity probe 1" | build `n1-build.json` (batch bridge-319f2454-07af-42b4-b752-67354f1c5771); post `n1-post-1-2.json` (call 1 pending; Yes clicked 10:15:45Z; call 2 posted_verified, created 1, all other counters 0, bound, target_voucher_step 1→2); verify `n1-verify.json` posted_verified, bound. N1: number 2, GUID …-00000002, master_id 2, alter_id 2 |
| 3 | sent REMOTEID | `sent-remoteid.txt` = 5da1d4d5-fc22-424f-9955-776d9c8ee090 (agent-import-ledger.jsonl, record_type dispatch_intent, field native_remote_id) |
| 4 | restore-plan updated; backup I1 | 26,099 B, sha256 141612a5144b7fcf9fb9ce17b68b4295d1b277b3167e7228c52a78158e222568, 10:17:36Z |
| 5 | reads A (a1-*) | see the table below |
| 6 | restore I1 over BRIDGE IDENTITY LAB | screens as planned (`s6-*.jpg`): Restore Data Path = the Tally data folder, source backups\I1, the list showed BRIDGE IDENTITY LAB (100056) 12-Oct-26 3:47 pm, "Accept ?" Yes, "Company data folder (100056) already exists" Replace. Reads a2-* |
| 7 | restore I0, then key K by hand | screens as planned (`s7-restore-*.jpg`, 3:43 pm backup). K: Receipt 10-Jul-2026, To Party A 301.00 / By Synthetic Bank Ledger 301.00, "Lab identity probe 1", automatic number (Tally gave 2). Tally opened Bank Allocations for Synthetic Bank Ledger (Cheque/DD, 301.00, inst. date 10-Jul-26); accepted as shown (`s7-k-bank-allocation.jpg`, `s7-k-voucher.jpg`). Reads a3-*: b3 and b4 filled with K's GUID from a3-window (…-00000002), b5 with N1's sent REMOTEID |
| 8 | restore-plan updated; backup I2 | 26,492 B, sha256 bc591993b7d074cc0cd604fb71daedb68667a407ad2fd78aa448b72c6fd383a9, 10:25:45Z |

## Vouchers returned (counted by structure: `analyse.py`)

| Read | a1 (after N1) | a2 (after harmless restore) | a3 (after restore + re-key K) |
|---|---|---|---|
| b1 window | 2 | 2 | 2 |
| b2 candidates | 2 | 2 | 2 |
| b3 by GUID (positive control) | **1** (N1) | **1** (N1) | **1** (K) |
| b4 by own REMOTEID (= GUID) | 0 | 0 | 0 |
| b5 by sent REMOTEID | 0 | 0 | 0 |

The positive control returned exactly one voucher at every step, so the b4 and b5 zeros are real answers and not a filter failure.

[inferred] b4 returns 0 even though the VOUCHER element's REMOTEID attribute equals the GUID. The filter evidently does not match that attribute this way.

## Field comparison (`compare-n1-n1r-k.md`: every attribute and child element of the b2 voucher dated 2026-07-10, flattened)

**a1 (N1) vs a2 (N1 after the harmless restore):** 443 fields each, 0 differing.

**a1 (N1) vs a3 (K, hand re-key after restoring I0):** 443 vs 536 fields, 106 differing. Every difference comes from how K's lines were keyed:
- **Leg order:** Party A was keyed first, so the ALLLEDGERENTRIES.LIST[0] and [1] LEDGERNAME, AMOUNT, ISDEEMEDPOSITIVE, ISLASTDEEMEDPOSITIVE and VATEXPAMOUNT swap.
- **PARTYLEDGERNAME:** Synthetic Bank Ledger on N1, Party A on K.
- **Bank allocation:** K's bank leg carries a filled BANKALLOCATIONS.LIST (Cheque/DD, BANKPARTYNAME Party A, 20260710, "Available Only in Books"); N1's is empty.

Every identity and creation field is EQUAL between N1 and K:

| Field | Value |
|---|---|
| @REMOTEID | dcac52b6-…-00000002 |
| @VCHKEY | dcac52b6-…-0000b484:00000008 |
| GUID | …-00000002 |
| MASTERID | 2 |
| ALTERID | 2 |
| VOUCHERKEY | 198479028682760 |
| VOUCHERRETAINKEY | 5 |
| VOUCHERNUMBER | 2 |
| NUMBERINGSTYLE | Auto Retain |
| REUSEHOLEID | 0 |
| VOUCHERNUMBERSERIES | Default |
| ENTEREDBY | "" |
| ALTEREDBY | "" |
| AUDITENTRIES.LIST, ACCOUNTAUDITENTRIES.LIST | empty |
| OLDAUDITENTRYIDS | -1 |
| NARRATION | equal |

## Answers

- **A field a hand re-key cannot reproduce, which a harmless restore keeps:** none found among the voucher's attributes and the b2 fields.
  - The voucher key (VCHKEY and VOUCHERKEY), REMOTEID, GUID, MasterID, AlterID and the retain key are all reproduced exactly by K.
  - The harmless restore keeps everything (0 differences).
  - The only differences are content-shaped (leg order, party ledger, bank allocation). They depend on how the hand entry was keyed, so they are not identity.
- **Which field holds the sent REMOTEID:** none.
  - The b5 filter returns 0 at every step, with a passing positive control.
  - The sent value 5da1d4d5-… appears in no field of the b2 answer; the voucher's REMOTEID attribute is its own GUID.
  - [inferred] Tally does not keep the REMOTEID ComplyEaze Bridge sent.
- **Creation facts:** ENTEREDBY and ALTEREDBY are empty on both (security is off) and the audit lists are empty on both. Not measured, as the README says: an install with Tally security on, and an install without the edit log.

## Deviations
- **Seed slip:** while keying the seed, a keystroke went into the To/By selector and Tally showed its input toast "Oops! Spelling Error". It was cleared with Backspace; nothing was saved by it. This happened before step 1.
- **Reused filled requests:** the a2 reads used the same filled b3–b5 files as a1, as the README asks ("repeat step 5's five reads").
- **Identical a3 fills:** the a3 b3/b4 files were filled again with K's GUID. Since K's GUID equals N1's, they are byte-identical to a1's (same SHA-256).
