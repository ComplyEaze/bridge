# W12: what Accept As Is changes in a voucher (Windows lab, 12 Oct 2026)

Book: synthetic BRIDGE PILOT LAB, company number 100033, GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85. It was loaded alone throughout. TallyPrime 7.1 Silver.
- Request set: lab/1342-capture-requests @ 99e750ac. W12-README.md sha256 3ab06490…. Every request file matched SHA256.txt or SHA256-top.txt (`r02-marks.xml` 81892a74…) before it was sent; `send.sh` checks this on every send.
- CLICK MODE: lane. Every Tally Yes/No and every Accept was clicked with the mouse.
- Owner approval: Accept As Is on BP/26-27/0003 and BP/26-27/0006 only. BP/26-27/0002 is the control and was never opened. No other voucher or master was altered.
- Fresh backup before any change: `<lab>\w12\backup-pre\TDBK1800_100033.001`, 78,961 B, sha256 bc0b054810f45f88737403d16f2154654ffde7d39820f29bcdba266e65f2dcd8, written 12:07:43Z. It stays on the lab PC (`SHA256-backups.txt`).
- Screenshots: 32 JPGs, which stay on the lab PC and are listed by hash in `SHA256-screenshots.txt`. This differs from the README, which asks for screens in `workorder/`; lab rule: screenshots stay local.
- Every send was one request at a time with the README's curl line. All 24 sends returned curl exit 0 and printed `200 <bytes>`.
  - Each answer is a complete ENVELOPE with a COLLECTION element and no LINEERROR. Each of the 18 full/status answers holds exactly one VOUCHER.
  - The full send log, with timestamps, the printed line, bytes and sha256 for every send, is `send-log.txt`.

## Answers
| Step | Answer | Bytes | sha256 (first 8) |
|---|---|---|---|
| 1 | aw12-marks-0.xml | 3,394 | c89f6f43 |
| 2 | aw12-full-0002-0.xml / aw12-status-0002-0.xml | 167,584 / 64,940 | a0ad917c / 0824a192 |
| 2 | aw12-full-0003-0.xml / aw12-status-0003-0.xml | 167,584 / 64,940 | 25a8de8a / 1f420497 |
| 2 | aw12-full-0006-0.xml / aw12-status-0006-0.xml | 167,584 / 64,940 | 8665c3e9 / 371c534b |
| 5 | aw12-marks-1, aw12-full-0003-1, aw12-status-0003-1, aw12-full-0002-1 | 3,396 / 167,608 / 64,944 / 167,586 | 75417b49 / ae73f873 / e6b9d42b / b04d4946 |
| 7 | aw12-marks-2, aw12-full-0006-1, aw12-status-0006-1, aw12-full-0002-2 | 3,396 / 167,606 / 64,946 / 167,586 | c7f9a54e / 300f7508 / 70a55635 / 237a3c11 |
| 8 | aw12-marks-3, aw12-full-0006-2, aw12-status-0006-2 | 3,396 / 167,606 / 64,946 | 9941ed70 / 1019ba04 / e09ffea7 |
| 9 | aw12-marks-4, aw12-full-0003-2, aw12-status-0003-2 | 3,398 / 167,638 / 64,946 | cd149375 / e3870ff5 / da529d7b |
| 10 | aw12-marks-5, aw12-full-0003-3, aw12-status-0003-3 | 3,402 / 167,620 / 64,948 | 818854dd / bdd1d989 / ba132e62 |

Full hashes are in `SHA256-answers.txt`.

**Published copies.** The ten full answers carry the synthetic company's GSTIN in `<CMPGSTIN>`.
- In the copies on this branch it is replaced by the same-length marker `<GSTIN-SYNTH15>`, so byte counts are unchanged. No other byte differs.
- `SHA256-answers.txt` gives both hashes: the lab file's, which matches `send-log.txt`, and the published copy's.
- The status and marks answers needed no change.

**Header note.** In every answer after step 2, the `<DESC><CMPINFO>` counters change: LEDGER, CURRENCY, TAXUNIT, VOUCHERNUMBERSERIES and VOUCHER rise at each later read, for example LEDGER 0 → 14 → 28 → 42.
- This happens identically in the control's answers, and it accounts for their +2 B.
- It is header only; it is not a voucher field.
- Voucher comparisons below are structural: every leaf of `COLLECTION/VOUCHER`, 2,304 fields (`vcmp.py`).

## Before (steps 1–3)
- All three vouchers: BRIDGE Sales, 1-Aug-2026, party BRIDGE Walk-in, sales ledger BRIDGE Svc 998313 5% (HSN 998313 from the ledger), narration empty, VCHGSTSTATUSISUNCERTAIN Yes.
  - ALTERIDs: 0002 = 11, 0003 = 15, 0006 = 7.
- Company marks-0: ALTVCHID 45, ALTMSTID 243.

**GSTR-1, 1-Aug-26 to 31-Aug-26 (summary):**
- Total vouchers 35: included 22, not relevant 6, uncertain 7.
- "Check Vouchers Having Potential Conflicts with Masters: Yes".
- B2C (Small) 22 (2): taxable 8,505.70, CGST 258.17, SGST 258.17, tax 516.34.

**Uncertain list, with reason texts.** Every uncertain August voucher shows exactly one reason; none is listed under two.
- Section "Transactions with Incomplete/Mismatch in Information", 7 vouchers in total.
- Outward Supplies, Invalid or Missing Information: **"HSN/SAC is invalid, mismatched, or not specified"**, 2 vouchers.
  - Both are 7-Aug-26 BRIDGE Sales No. 5 (ledger BRIDGE Svc INH 5%) and No. 6 (BRIDGE Svc INH2 5%), each 100.00 + 5.00 tax. These are the W11 vouchers and were not touched.
- Mismatch in Information Between Masters and Transactions: **"Mismatch between Expected Tax Amount and Modified Tax Amount in transaction"**, 5 vouchers. The screen is "GSTR-1 - Resolution of Uncertain Transactions":

| Voucher | Taxable | Expected tax | Modified tax | Difference | CGST exp/mod | SGST exp/mod |
|---|---|---|---|---|---|---|
| BP/26-27/0002 | 1,001.00 | 50.06 | 50.05 | 0.01 | 25.03 / 25.03 | 25.03 / 25.02 |
| BP/26-27/0003 | 1,001.00 | 50.06 | 50.05 | 0.01 | 25.03 / 25.02 | 25.03 / 25.03 |
| BP/26-27/0005 | 1,001.00 | 50.06 | 50.04 | 0.02 | 25.03 / 25.02 | 25.03 / 25.02 |
| BP/26-27/0006 | 1,000.00 | 50.00 | 50.02 | (-)0.02 | 25.00 / 25.01 | 25.00 / 25.01 |
| BP/26-27/0008 | 1,001.00 | 50.06 | 50.05 | 0.01 | 25.03 / 25.03 | 25.03 / 25.02 |

So 0003 and 0006 both show the expected tax-mismatch reason.

**Selecting several at once (look only):**
- Both voucher lists show a "Space: Select" footer key. This suggests Tally offers multi-select for Accept As Is.
- It was not used, so whether Accept As Is applies to a multi-selection is unmeasured.
- Buttons offered:
  - Tax-mismatch list: "J: Accept As Is" and "S: Set GST Status".
  - HSN list: "V: Accept As Is (Format)" and "S: Set GST Status".

**Observed STOP (not in the README's list).** To cover "every uncertain voucher of the book", I changed the GSTR-1 view period (F2) to 1-4-2026..31-3-2027. Tally then showed:
> "GST-related details were altered in the masters/Return. Hence, opening the report will update the GST Status of 1 vouchers. Do you want to open the report?" Yes or No.
- Screenshot: s3-STOP-gst-status-update-dialog.jpg, sha256 0c9a5d944442171a8e5b0666598d98476ac739557dee02b2eae08cfa18043e32.
- I stopped and reported. On the orchestrator's decision I clicked **No**: Tally closed the report and returned to the Gateway; nothing was opened or recomputed.
- **The full-year uncertain list was NOT READ.** Which voucher outside August would have been updated is not known.
- The period was kept at August for the remaining steps.

## Accept As Is on 0003 (steps 4–6)
**Step 4, how it was done:**
- I reopened GSTR-1 (August, same figures), then Uncertain Transactions, then the tax-mismatch reason.
- I highlighted the single row BP/26-27/0003, without Space, and clicked the right-panel button **"J: Accept As Is"** with the mouse.
- Tally asked: **"This exception will be accepted and included in Returns. Do you want to accept as is?"** Yes or No. I clicked Yes.
- No other dialog appeared. No voucher screen opened: Tally did not offer "alter, then accept", it accepted from the list.
- 0003 left the list; 4 vouchers remained.

**Edit Log (step 4):** opened via Go To, "Edit Log Transaction Summary" (Voucher Type-wise Summary, 1-Aug-26 to 15-Sep-26, all version periods).
- The default view ("Altered & Deleted Vouchers for Any Activity Except Migration") shows **no rows**.
- F5 ("All Vouchers for Any Activity") also shows **no rows**: altered, non-altered and total are all blank.
- So the Edit Log reports nothing for this company at all, including nothing for the Accept As Is.
- The voucher's own Edit Log was not opened, because that needs the voucher screen.

**Step 5 reads:**
- marks-1: ALTVCHID 45 → **46**; ALTMSTID 243 unchanged.
- 0003, -0 vs -1: **exactly 5 of 2,304 fields differ.**
  - ALTERID 15 → **46**
  - ISGSTOVERRIDDEN No → **Yes**
  - ISGSTOVRDNCRC 0 → **1669218922**
  - VCHGSTSTATUSISINCLUDED No → **Yes**
  - VCHGSTSTATUSISUNCERTAIN Yes → **No**
  - Amounts, ledgers, HSN source fields and narration are unchanged. status-0003-1 agrees.
- Control 0002 (full-0002-1): **0 of 2,304 differ** (ALTERID 11, still uncertain).

**Step 6 screens:**
- Uncertain 6 (HSN 2, mismatch 4); included 23.
- B2C (Small) 23 (2): 9,506.70, CGST 283.19, SGST 283.20 (shown in *italic*), tax 566.39.
- The increase is 0003's *modified* tax (25.02 + 25.03), not its expected tax.

## Accept As Is on 0006 (step 7)
- Same path: highlighted only BP/26-27/0006, clicked "J: Accept As Is", got the same dialog text, clicked Yes. No other dialog appeared.
- marks-2: ALTVCHID 46 → **47**.
- 0006, -0 vs -1: **exactly 5 differ**, the same fields as for 0003.
  - ALTERID 7 → 47, ISGSTOVERRIDDEN Yes, ISGSTOVRDNCRC 0 → **77975254**, INCLUDED Yes, UNCERTAIN No.
- Control 0002 (full-0002-2): **0 differ**.
- Screens: uncertain 5 (HSN 2, mismatch 3); included 24.
  - B2C (Small) 24 (2): 10,506.70, CGST 308.20, SGST 308.21 (italic), tax 616.41. The increase is 0006's modified tax, 25.01 + 25.01.
- The Edit Log was not re-opened for 0006; step 4 showed it holds nothing.

## Re-save 0006 unchanged (step 8)
- Path: Day Book (1-Aug-26), Enter on BP/26-27/0006, which opened "Accounting Voucher Alteration (Secondary)". I changed nothing and clicked "A: Accept". No dialog appeared.
- marks-3: ALTVCHID 47 → **48**.
- 0006, -1 vs -2: **2 differ.**
  - ALTERID 47 → 48.
  - BASICBASEPARTYNAME 'Bridge Walk-in' → 'BRIDGE Walk-in': the stored party text took the ledger's case.
- **The acceptance survived**: ISGSTOVERRIDDEN Yes, ISGSTOVRDNCRC 77975254 (unchanged), INCLUDED Yes, UNCERTAIN No.
- Screens unchanged from step 7.

## Narration-only edit of 0003 (step 9)
- Path: Day Book, then 0003 alteration. I clicked the narration area; focus landed on the total (1,051.05). I pressed Enter with the value unchanged, which moved to Narration. I typed `W12` and clicked "A: Accept". No dialog appeared.
- marks-4: ALTVCHID 48 → **49**.
- 0003, -1 vs -2: **2 differ**: ALTERID 46 → 49 and NARRATION '' → 'W12'.
- **The acceptance survived**: ISGSTOVERRIDDEN Yes, ISGSTOVRDNCRC 1669218922 (unchanged), INCLUDED Yes.
- 0003's BASICBASEPARTYNAME stayed 'Bridge Walk-in'. Unlike 0006 in step 8, its case did not change; the cause is not established.
- Screens unchanged: included 24, uncertain 5.

## One-paisa edit of 0003 (step 10)
- Path: 0003 alteration. I clicked the state-tax amount (BRIDGE SGST 2.5%, 25.03), typed 25.02 and pressed Enter; the total became 1,051.04. I clicked "A: Accept". No dialog appeared.
- marks-5: ALTVCHID 49 → **50**.
- 0003, -2 vs -3: **11 differ.**
  - Party amount -1051.05 → -1051.04, in ALLLEDGERENTRIES and LEDGERENTRIES.
  - SGST AMOUNT and VATEXPAMOUNT 25.03 → 25.02, in both lists.
  - ALTERID 49 → 50.
  - **ISGSTOVERRIDDEN Yes → No**
  - **ISGSTOVRDNCRC 1669218922 → 0**
  - **INCLUDED Yes → No**
  - **UNCERTAIN No → Yes**
- **Editing a figure voided the acceptance.**
- Screens:
  - Uncertain 6; included 23.
  - B2C (Small) 23 (2): 9,505.70, CGST 283.18, SGST 283.18, tax 566.36.
  - 0003 is back in the tax-mismatch list with expected 50.06, modified 50.04, difference 0.02.
  - 0006 stays accepted.

## What this shows
- Accept As Is (from GSTR-1, "J: Accept As Is") changes exactly five voucher fields:
  - ALTERID, which takes the next company voucher alter id;
  - ISGSTOVERRIDDEN, set to Yes;
  - ISGSTOVRDNCRC, a non-zero checksum that differs per voucher;
  - VCHGSTSTATUSISINCLUDED, set to Yes;
  - VCHGSTSTATUSISUNCERTAIN, set to No.
- Amounts are untouched, and the control moved in no field.
- The acceptance survives an unchanged re-save and a narration-only edit, with the checksum unchanged in both cases. An amount edit clears it: the flag goes to No and the checksum to 0, and the voucher becomes uncertain again.
- [inferred] The checksum covers the amounts or tax lines but not the narration.
- GSTR-1 counts an accepted voucher at its modified (stored) tax.

## Not measured, and differences from the README
- The full-year uncertain list was not read (observed STOP above; No clicked on the orchestrator's decision).
- Multi-select ("Space: Select") was seen but not used.
- Step 4's "alter, then accept": Tally accepted directly from the list. No alteration screen opened during Accept As Is.
- The voucher-level Edit Log was not opened. The company Edit Log Transaction Summary was empty in both views.
- Screenshots and the backup stay on the lab PC, listed by hash, not in `workorder/`.
- The full answers are published with the GSTIN replaced by a same-length marker (see Answers).
- The 0006 party-name case change on re-save is recorded, not explained.
