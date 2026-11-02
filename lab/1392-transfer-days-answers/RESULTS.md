# Results: #1392 transfer days: the ledger catalogue and three kinds of transfer day, 2 Nov 2026

**PASS. All 27 answers received in full (HTTP 200, complete XML), `m2` equals `m3`, and steps 7 and 8 are reported. Every voucher ledger name equals a catalogue name by code point, including the new `TLab Clearing`. Each case's day reads as keyed, and every neighbouring day answers with no voucher.**

- **Book:** BRIDGE TRANSFER LAB, the sheet's book, loaded alone, port 9001.
- **Before the first send:** `tally_status` read TallyPrime 7.2, Silver, `education_mode` false, that one company loaded. Build for that read: `bridge_mcp.exe` sha256 996e7f4089e64a1f47f0145374b70210cf55ae947fea33ce050d30bb1d8e8b58 (head 4cdfe448, 0.7.0), redaction off.
- **Sheet:** `lab/1392-transfer-days-sheet` at ddac9f69; 26 of 26 files equal to `SHA256.txt`.
- **Backup:** taken by Tally's own screen before any change, kept on the lab PC (28,318 B, sha256 244c95c3e2a7fe18...).

## Marks
| read | ALTVCHID | ALTMSTID |
|---|---|---|
| `m1`, before keying | 7 | 210 |
| `m2`, after keying | 12 | 211 |
| `m3`, after the reads | 12 | 211 |

`m2` plus 5 EXPECT met; `ALTMSTID` moved by 1 for the one ledger; under 8,000. `m2` and `m3` are byte-equal. [verified]

## What was keyed
One ledger, `TLab Clearing` under `Suspense A/c`, no opening balance. Then five vouchers, in order, double entry, no narration, no reference; bank allocations accepted as offered, nothing typed in them.

| # | date | type, number | By (Dr) | To (Cr) | amount | ALTERID |
|---|---|---|---|---|---|---|
| T1 | 20260413 | Contra 6 | Cash | TLab Bank A | 7000.00 | 8 |
| T2 | 20260420 | Contra 7 | Cash | TLab Bank A | 4000.00 | 9 |
| T3 | 20260421 | Contra 8 | TLab Bank B | TLab Bank A | 25000.00 | 10 |
| T4 | 20260427 | Payment 1 | TLab Clearing | TLab Bank A | 30000.00 | 11 |
| T5 | 20260428 | Receipt 1 | TLab Bank B | TLab Clearing | 30000.00 | 12 |

Each row above is read back from the day's vouchers answer. [verified]

## The days
| day | vouchers in census | vouchers in vouchers read | what it holds |
|---|---|---|---|
| 12 | 0 | 0 | neighbour |
| 13 | 1 | 1 | T1: case 2, a voucher on `TLab Bank A` and a third ledger, none on both banks |
| 14 | 0 | 0 | neighbour |
| 19 | 0 | 0 | neighbour |
| 20 | 1 | 1 | T2: case 3, makes the skip's day whole |
| 21 | 1 | 1 | T3: case 3, the A-B Contra on the day after |
| 22 | 0 | 0 | neighbour |
| 26 | 0 | 0 | neighbour |
| 27 | 1 | 1 | T4: case 4, first leg through the clearing ledger |
| 28 | 1 | 1 | T5: case 4, second leg |
| 29 | 0 | 0 | neighbour |

Census and vouchers agree on every day. [verified] `evidence-report.txt` lists every response's size, sha256, `VOUCHER` count and fields, and every `ALLLEDGERENTRIES.LIST`.

**Counting note:** every response holds one `VOUCHER` node with no attribute and no child: Tally's header count, `<VOUCHER>0</VOUCHER>`. The counts above are voucher objects; the report gives both numbers. The same is true of `COMPANY` and `LEDGER` in the marks and catalogue answers. An exact node count that does not set this aside reads one too many on every response.

## Step 8: the spelling
`cat2` names: `Cash`, `Profit & Loss A/c`, `TLab Bank A`, `TLab Bank B`, `TLab Clearing`. Every `LEDGERNAME` on a voucher row (`Cash`, `TLab Bank A`, `TLab Bank B`, `TLab Clearing`) equals a catalogue name exactly, by code point, with no trimming and no case folding. None differs. [verified]

## Deviations
1. **Screenshots are at the Accept prompt, just before each voucher was saved**, not of the saved voucher reopened. Reopening five vouchers was not worth the risk of an altered one on a sheet that forbids any. The vouchers answers above show each saved voucher.
2. The step's "any dialog you did not expect": the bank allocation screens and the Accept prompts were expected by the sheet. A `Quit?` prompt on a blank creation form and a blank voucher form, on leaving them, was answered Yes; nothing was on either form.
3. Tally prefilled the second ledger on T1 and T3 with a ledger other than the sheet's. Each was changed to the sheet's ledger before saving; the saved vouchers read as the table above.

4. **The screenshots stay on the lab PC, against the sheet**, which sends them to this branch: a taskbar can show a user name. They are kept there and listed here by sha256.

## Screenshots, kept on the lab PC
- `T1-at-accept.jpg` 40463 B sha256 7a5a9fe83099822d22da85428d451c78ebf29b4a3a75b482ec4b132ba579ac12
- `T2-at-accept.jpg` 39435 B sha256 fb03a7addb0bc9246cfa3149526334f359dca4485986779f82262017ff637acd
- `T3-at-accept.jpg` 40171 B sha256 337320564d5819792d751828aa440b20010051371676d06aefbf35ee64bada77
- `T4-at-accept.jpg` 39793 B sha256 dde7f9a8be38569029ef964932a7f07e958bb5b94c344e28a5228ca1af3e6379
- `T5-at-accept.jpg` 40119 B sha256 2cefe4bcb2e71164396caefd3baf87319af3a9d26357ea18fa6d352381f8fdbc

**Confidence:** one sitting, one synthetic book: PARTIAL for anything beyond this book. Not covered: the catalogue on other books, spellings a typed ledger cannot produce, Gold, other releases.
