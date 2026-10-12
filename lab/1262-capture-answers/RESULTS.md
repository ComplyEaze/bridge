# #1262 capture K0-K5: Windows lab, 12 Oct 2026

TallyPrime 7.1 Silver, the regular TallyPrime on port 9001; TallyPrime Edit Log not running. No ComplyEaze Bridge build was used: every read is curl.exe with the four request files from akshit-khandelwal47/bridge@135b1b33 lab/1262-capture-requests/. All four were checked against that folder's SHA256.txt before each send (q1 84e00770, q2 d49ec338, q3 f09848fb, q4 5f457502; UTF-16LE with BOM). One request at a time. Every answer: HTTP 200, no LINEERROR or exception, and no dialog in Tally's window. Status, size and SHA-256 per answer are in log.txt; the answers come back as UTF-16LE without a BOM.

## What was done (in order; times UTC)
1. K0, the pre-existing BRIDGE SPELL LAB (Load 100049, history unknown), loaded alone (shots/K0-before-first-request-gateway.jpg).
   - q1-q4 → K0/K0-*.xml, at 08:00.
   - Its ledger list: Cash, Bridge Spell Party, Profit & Loss A/c (shots/K0-ledger-list.jpg). Its vouchers of 1-Apr-2026: Receipt 1, Bridge Spell Party, 100.00 (shots/K0-daybook-0401.jpg).
   - K0 answers: attribute 'Bridge Spell Party' = stored name. The voucher's LEDGERNAME/PARTYLEDGERNAME is 'Bridge Spell Party'.
   - This is a pre-existing company with unknown history and is not part of the K comparisons.
2. Tally backup of 100049: TDBK1800_100049.001 (25,435 B, sha256 39e9b802faa8c62891bc7fcaa944c6c840f660439b674a1fd6c90029d03c5f04, 08:03). Then:
   - Company name altered to BRIDGE SPELL LAB PRE (shots/pre-renamed-gateway.jpg). Nothing deleted.
   - Shut, and kept closed for the rest of the item.
3. Step 1: BRIDGE SPELL LAB created afresh as company 100052.
   - Books from 1-Apr-2026, India; GST No, inventory No; other features at Tally's defaults (bill-wise entry Yes at company level).
   - Tally backup right after creation: TDBK1800_100052.001 (22,961 B, sha256 53675e8dd6c846a23fb27ddf4afaf7b2f4a8b059328733d11425a73d461ee2fa, 08:06).
   - Then ledger 'Bridge Spell Party' under Sundry Debtors: no opening balance, bill-by-bill No.
   - Then Receipt 1 dated 1-Apr-2026: Cash Dr 100.00 / Bridge Spell Party Cr 100.00, no narration, automatic number 1.
4. K1, A alone, at 08:09.
5. Step 2: BRIDGE SPELL TWIN created as company 100053 while A stayed loaded.
   - Books from 1-Apr-2026; features as A (GST No, inventory No) — the README names only books-from.
   - Tally backup right after creation: TDBK1800_100053.001 (22,962 B, sha256 f2371572365229981a1eb9cdb86c31a06ac88271a8a5d1421e6cfb1e2575e128, 08:11).
   - Then ledger 'BRIDGE SPELL PARTY' (keyed in capitals) under Sundry Debtors: no opening balance, bill-by-bill No (as A; the README does not say). No vouchers.
6. K2, A and B loaded (B current), at 08:12.
7. K3: Tally quit fully (no tally.exe process; 9001 refused), then started.
   - It auto-opened 'Aarav Trading Company Demo' (a synthetic demo book, "(e) Data exceptions exist") (shots/K3-after-start-gateway.jpg); it was shut.
   - Then B loaded first, A second. 9001 answered "TallyPrime Server is Running" before q1. Reads at 08:15.
8. K4: the same restart (Aarav auto-opened and shut), then A first, B second. Reads at 08:17.
9. K5: the same restart (Aarav auto-opened and shut), then A alone. Reads at 08:19.

## Results for the 'Bridge Spell Party' ledger (company BRIDGE SPELL LAB 100052, by structure)
| checkpoint | loaded (order) | q1 row NAME attr | q1 stored name (LANGUAGENAME.LIST/NAME.LIST first NAME) | q2 LEDGERNAME / PARTYLEDGERNAME | q3 LEDGERNAME / PARTYLEDGERNAME | q4 attr vs stored | failing state? |
|---|---|---|---|---|---|---|---|
| K1 | A | Bridge Spell Party | Bridge Spell Party | Bridge Spell Party / Bridge Spell Party | same | equal | no |
| K2 | A, then B created | Bridge Spell Party | Bridge Spell Party | Bridge Spell Party / Bridge Spell Party | same | equal | no |
| K3 | B first, then A | **BRIDGE SPELL PARTY** | Bridge Spell Party | **BRIDGE SPELL PARTY / BRIDGE SPELL PARTY** | same | attr differs | no: attr differs, but the voucher carries the attribute spelling, not the stored name |
| K4 | A first, then B | Bridge Spell Party | Bridge Spell Party | Bridge Spell Party / Bridge Spell Party | same | equal | no |
| K5 | A alone | Bridge Spell Party | Bridge Spell Party | Bridge Spell Party / Bridge Spell Party | same | equal | no |

## Comparisons as the README frames them
- K1 against K2 (the second company created and loaded): no change. The four answers are byte-identical (same SHA-256 per file).
- K3 against K4 (load order only): the ONLY differences are in A's answers:
  - the ledger's NAME attribute in q1 and q4;
  - the voucher's LEDGERNAME and PARTYLEDGERNAME in q2 and q3.
  - With B loaded first (K3) they carry B's spelling 'BRIDGE SPELL PARTY'; with A first (K4) A's own 'Bridge Spell Party'. The stored name (NAME.LIST) is 'Bridge Spell Party' in both.
- K5 against K1 (a restart only): same names. The bytes differ only in the CMPINFO counts (COMPANY, LEDGER, CURRENCY, VOUCHERNUMBERSERIES, VOUCHER: 1/4/2/1/1 at K1, 0 at K5, after a restart). K5 is byte-identical to K4.
- [inferred] The spelling Tally shows for a name shared, in another case, by two loaded companies is the first-loaded company's. It reaches both the ledger row attribute and the voucher's ledger names alike. So the README's failing state (attribute differs AND the voucher carries the stored name) was NOT produced by any checkpoint. Per the README, nothing else was tried.

## Unexpected
- Right after step 2, Tally's own Chart of Accounts > Ledgers screen in TWIN listed TWIN's new ledger as 'Bridge Spell Party', though it was keyed and shown in capitals on the creation screen (shots/step2-B-ledger-creation-caps.jpg, step2-B-ledger-list-shows-mixed-case.jpg). A was loaded first at the time. Not read through XML; TWIN was never queried, since every request names BRIDGE SPELL LAB.
- Each restart auto-opened Aarav Trading Company Demo, which was shut before loading.

## Files
K0/ (4 answers, kept apart), answers/ (K1..K5 × q1..q4 = 20), log.txt (one line per send: times, status, bytes, sha256), notes.txt, shots/ (screenshots), SHA256.txt. Backups stay on the lab PC: names and hashes are above.
