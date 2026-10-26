# Results: #2027 Place of Supply, State and registration type in the vouchers read, on BRIDGE NOTES LAB (TallyPrime 7.2), 27 Oct 2026. PASS: the candidate's answer is today's answer plus `PLACEOFSUPPLY` Maharashtra, `STATENAME` Maharashtra and `GSTREGISTRATIONTYPE` Regular on each of the four vouchers

The sheet is `lab/2027-place-of-supply` at cdc6f770d512d9eb8f20469d5200402fe26f8c4d. Preflight:
- its 6 files were checked against `SHA256.txt`: 6/6 OK;
- `python -I check-request-ids.py .` gave `census: ok 3, skipped 0, FAIL 0` (exit 0).

Every request was sent with the sheet's command, one at a time (`answers/log.txt`).

**Release and edition:** `tally_status` (`answers/tally_status.json`, build w1-2306d86d 92a899a2, new empty data folder): TallyPrime **7.2**, `license_tier` silver, `education_mode` false. `loaded_companies` was exactly BRIDGE NOTES LAB (100084, GUID d0b6b9e8-9f2e-4863-b0f1-feac353fd9b0), loaded alone with no question or prompt.

| response | bytes | sha256 | result |
|---|---|---|---|
| m1 | 3,392 | ab86af9a821b3e3290ca9fd71548975ed7d2acc7ad5ca956c08a167c81d89a7c | one COMPANY row, **ALTVCHID " 4"**, **ALTMSTID " 215"** (= W3's close) |
| v1 (today's request) | 251,770 | d9ec588503a26e9c74d6b906c4e58ed85eec84a98b42f4933dc0320bdc09374d | 4 VOUCHER |
| v2 (the candidate) | 253,282 | b24aa6d57e06b79cb3d03ae39365f9f35b9c5629292ee40d6273a9bb5eab566d | 4 VOUCHER |
| m2 | 3,392 | = m1 | 4 / 215 |

Every answer was HTTP 200 and within its bound (marks 8,192, vouchers 2,000,000). Marks are equal before and after.

**Step 6**, counted with an XML parser (nodes named exactly `VOUCHER` under `ENVELOPE/BODY/DATA/COLLECTION`; `compare-output.txt`, from `compare.py` on the lab PC):

| type | number | date | PLACEOFSUPPLY | STATENAME | GSTREGISTRATIONTYPE |
|---|---|---|---|---|---|
| Sales | 1 | 20250601 | present, `TYPE="String"`, Maharashtra | present, `TYPE="String"`, Maharashtra | present, `TYPE="String"`, Regular |
| Purchase | 1 | 20250601 | Maharashtra | Maharashtra | Regular |
| Credit Note | 1 | 20250602 | Maharashtra | Maharashtra | Regular |
| Debit Note | 1 | 20250602 | Maharashtra | Maharashtra | Regular |

Each element is a direct child of its `VOUCHER`, with the attribute `TYPE="String"`; none is empty. They match the Party Details recorded on W3's screens.

**Step 7:** the three elements were removed from `v2` with the parser (12 elements, three on each voucher). `v1` and `v2` were then serialised the same way, and they are **equal**. Nothing else differs. The 1,512-byte difference in size is those twelve elements.

**Close.** The book was shut by the Company menu > Shut, leaving no company loaded. Nothing was written. No dialog appeared, and no stop rule was met. The answers carry the synthetic GSTINs typed in W3. The evidence was zipped on the lab PC (sha256 dabaf364fb91b42e4eea644268554e5338ad5f63a3f96639267722deeb994cc0).
