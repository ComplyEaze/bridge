# Results: #1392 whole-day read on BRIDGE TRANSFER LAB, 3 to 6 April 2026 (TallyPrime 7.2), 26 Oct 2026. PASS: fourteen answers in full, marks 5 / 210 before and after, each day's two censuses byte-identical, census and vouchers agree per day

The sheet is `lab/1392-whole-day-read` at e6215ca0a85304a4c73fb9bcdb66ddb63089a2f8. Its 13 request files and README were checked against `SHA256.txt`: 15/15 OK. Every request was sent with the sheet's command, one at a time; `answers/log.txt` holds each send's UTC time, curl exit, `-w` line and the answer's sha256.

**Release and edition:** `tally_status` (`answers/tally_status.json`) reports `product` TallyPrime, `release` 7.2, `license_tier` silver and `education_mode` false. `loaded_companies` is exactly BRIDGE TRANSFER LAB (100051, GUID d6e4c24f-8c7a-4797-93ec-e44bdff60bc9). The build for it was w1-2306d86d, `bridge_mcp.exe` sha256 92a899a2..., checked before the start. It ran over stdio with host 127.0.0.1, port 9001, a new empty data folder and terms accepted; posting, import and redaction were unset. The book was loaded alone, with no question or prompt.

**Marks (step 2 and step 4):** `m1` and `m2` each hold one COMPANY row with **ALTVCHID " 5"** and **ALTMSTID " 210"**, as sent (a leading space). They are byte-identical (sha256 c412f89e...).

**Step 5:** for every day, `dDD-c1` and `dDD-c2` are byte-identical.

**Step 6, counted with an XML parser** (nodes named exactly `VOUCHER` under `ENVELOPE/BODY/DATA/COLLECTION`). Every voucher GUID begins with the book's GUID; it is shown below by its suffix.

| response | bytes | sha256 | VOUCHER | vouchers (GUID suffix, ALTERID, DATE, type) |
|---|---|---|---|---|
| d03-c1 | 10,240 | d0164be8d50a8171295d0d5c69680e466bc03b34bbc4c0d4d9b682267a5dce02 | 3 | -00000001, 1, 20260403, Contra; -00000002, 2, 20260403, Contra; -00000004, 4, 20260403, Contra |
| d03-c2 | 10,240 | = d03-c1 | 3 | the same |
| d03-v | 128,174 | 59caf01d40348413e3a4599556c565927fe424839ce1db3a0ba96334529d4355 | 3 | the same three |
| d04-c1 | 5,430 | a8aea6d104260c6e0b595d0686e4fe1f4232eff34eff9e479de5a7ac3f3c634e | 1 | -00000003, 3, 20260404, Contra |
| d04-c2 | 5,430 | = d04-c1 | 1 | the same |
| d04-v | 44,666 | 6712759b59c693ffa227b34b4cf0420fba2fd3ac65136f57101afbd2c6a4ab5a | 1 | the same |
| d05-c1 | 5,432 | c6338e1967a18dcc055ee6819b29863ee695e3510cb96c6d7904b3042ebc2821 | 1 | -00000005, 5, 20260405, Contra |
| d05-c2 | 5,432 | = d05-c1 | 1 | the same |
| d05-v | 44,764 | 7c4ed54061e6ea0286d5cad19d2a0d04196607793120429753d6fe2d3c72a983 | 1 | the same |
| d06-c1 | 3,022 | 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 | 0 | (COLLECTION present, no VOUCHER) |
| d06-c2 | 3,022 | = d06-c1 | 0 | |
| d06-v | 3,022 | = d06-c1 | 0 | the vouchers answer for 6 April is byte-identical to its census answers |
| m1, m2 | 3,398 each | c412f89ed1fcf9b70d4476d57f7fdea988e3311c23effad29431547953933129 | 0 | one COMPANY row |

Ledger entries in the vouchers answers (`ALLLEDGERENTRIES.LIST`: LEDGERNAME, AMOUNT):

| voucher | first entry | second entry |
|---|---|---|
| -00000001 (3 Apr) | TLab Bank A 50000.00 | TLab Bank B -50000.00 |
| -00000002 (3 Apr) | TLab Bank A 50000.00 | TLab Bank B -50000.00 |
| -00000004 (3 Apr) | TLab Bank B 50000.00 | TLab Bank A -50000.00 |
| -00000003 (4 Apr) | TLab Bank A 50000.00 | TLab Bank B -50000.00 |
| -00000005 (5 Apr) | TLab Bank A 50000.00 | TLab Bank B -50000.00 |

The four days together hold the five Contras, each on its own date. These are the same GUIDs, AlterIDs, dates and bank allocations as the committed five-day answer `lab/transfer-1392-requests` `answers-1392-h/h-contra.response`, counted the same way. The 6 April census lists none.

**Step 7, `time_total` per send (seconds):**

| day | c1 | c2 | v |
|---|---|---|---|
| 03 | 0.037 | 0.035 | 0.076 |
| 04 | 0.032 | 0.031 | 0.049 |
| 05 | 0.032 | 0.034 | 0.048 |
| 06 | 0.006 | 0.006 | 0.004 |

The marks reads took 0.005 (m1) and 0.008 (m2). A day of this book costs three requests and at most about 0.15 s in all.

**Close.** After these reads, #899's four reads of the same book were sent on the same load (`lab/899-spanned-census-answers`). The book was then shut by the Company menu > Shut, leaving no company loaded. Nothing was written. No dialog appeared, and no stop rule was met.

The evidence folder was zipped on the lab PC (sha256 50fe956172301760f8bae084184e40ee517196e8e54f7ce65f92e000d4628476); the files are under `answers/` unedited.
