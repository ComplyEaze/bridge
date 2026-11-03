# Results: #2164 spanned census of ten transfer days, 3 Nov 2026

**PASS.** All 12 answers came in full (HTTP 200, complete XML). `m1` equals `m2`, byte for byte: ALTVCHID 12, ALTMSTID 211. Steps 5 and 6 are reported below. Each day's voucher objects equal 2 Nov's: the same GUID, ALTERID, date, type and number. No day's sha256 equals 2 Nov's. In every one the only difference is four header count lines in CMPINFO, which are not rows. [verified]

Sheet: `lab/1392-spanned-census-sheet` at 862cbd39, all 14 files equal to `SHA256.txt` by `Get-FileHash` (`evidence/sheet-hashes.txt`). Book: BRIDGE TRANSFER LAB (synthetic), loaded alone on port 9001. The Gateway's current date was 28-Apr-2026 and was not changed. Nothing was keyed, altered or re-saved. [verified]

## Before the first send
`tally_status` (ComplyEaze Bridge, master a6c64b8b, `bridge_mcp.exe` sha256 0345e74beff3c44ff874fdb45d31306dfb8d7eb694ab0f8db66537ad6857b1bd, writes off, redaction off) read:
- TallyPrime 7.2, licence tier silver, `education_mode` false;
- one loaded company, BRIDGE TRANSFER LAB, GUID d6e4c24f-8c7a-4797-93ec-e44bdff60bc9.

The server was stopped before the first raw send (`run/client-log.txt`). [verified]

## Sends
Each was sent with the sheet's curl line exactly, one at a time, with no `--max-time`. Exit code, `-w` line and ISO time are in `evidence/log.txt`. All exit 0, all 200. [verified]

| name | bytes | sha256 | voucher objects | equals 2 Nov |
|---|---:|---|---|---|
| m1 | 3,400 | 174d6672… | ALTVCHID 12, ALTMSTID 211 | |
| d12-c2 | 3,022 | 947fbf36… | 0 | no: header counts only |
| d13-c2 | 5,432 | 084d21a0… | 1: …-00000006, ALTERID 8, 20260413, Contra 6 | no: header counts only |
| d14-c2 | 3,022 | 947fbf36… | 0 | no: header counts only |
| d19-c2 | 3,022 | 947fbf36… | 0 | no: header counts only |
| d20-c2 | 5,432 | a556f23d… | 1: …-00000007, ALTERID 9, 20260420, Contra 7 | no: header counts only |
| d21-c2 | 5,434 | 68de428e… | 1: …-00000008, ALTERID 10, 20260421, Contra 8 | no: header counts only |
| d26-c2 | 3,022 | 947fbf36… | 0 | no: header counts only |
| d27-c2 | 5,436 | c73299e5… | 1: …-00000009, ALTERID 11, 20260427, Payment 1 | no: header counts only |
| d28-c2 | 5,438 | 2a9f0aa0… | 1: …-0000000a, ALTERID 12, 20260428, Receipt 1 | no: header counts only |
| d29-c2 | 3,022 | 947fbf36… | 0 | no: header counts only |
| m2 | 3,400 | 174d6672… | equal to m1 | |

Every voucher GUID begins with the company's GUID. Full sha256 values and GUIDs are in `evidence/count.txt`. [verified]

## Step 5, by structure
Each answer was decoded as UTF-16LE, stripped only of the characters XML forbids, and parsed. Voucher objects are `VOUCHER` nodes that carry attributes or children. The header's bare `<VOUCHER>0</VOUCHER>` count is not one. The script is `scripts/count.py`. [verified]

## Step 6, against 2 Nov
- No answer's sha256 equals its `earlier-answers.sha256` line.
- **Byte offset:** each differs first at byte offset 348, and each is 4 bytes shorter than 2 Nov's.
- **Header lines:** decoded line by line, the only lines that differ are four in CMPINFO: `LEDGER` 18 → 0, `CURRENCY` 5 → 0, `VOUCHERNUMBERSERIES` 5 → 0, `VOUCHER` 10 → 0.
- **Voucher objects:** each day's objects are equal to 2 Nov's: 0 on the five empty days, and the same one voucher on the five others.

[verified]

- The 2 Nov answers were read from `lab/1392-transfer-days-answers` at 5fb43569. Each was checked against `earlier-answers.sha256` first: all ten equal.
- **Why the counters changed.** These counters move with what Tally's session has loaded; rows 2162-S1 and #2210 saw the same four. Here the reads ran straight after a fresh load with no other read in between, which would explain zeros. [hypothesis]

## Evidence
`evidence/` holds the 12 responses and headers, `log.txt`, `count.txt` and the sheet hash check, all unedited. The responses hold no company number. The zip of that folder is kept on the lab PC: `evidence-1392-spanned.zip`, sha256 fb4d49a441be65efecc0a87be0cc35a25258974d504b6bcde3f011778697afc8. The `tally_status` answer is in `run/answers/` with `company_number` replaced by a JSON parser; the client log and launch file have the lab folder replaced.

## The book now
BRIDGE TRANSFER LAB as before: marks 12 / 211, unchanged. It was loaded alone and is shut at the row's end.

**Confidence:** one book, one sitting, 7.2: PARTIAL.
