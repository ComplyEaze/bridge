# Row 2119-B: one party's Ledger Outstandings answer against the Bills Payable report and the ledger's closing balance, on BRIDGE BILLTRAIL LAB: results

**First line: every EXPECT met; no stop. The "What it decides" row met is the first: the design note (d78b8e71) stands.**
- At step 6, `debit_vs_receivable` matched 3 of 3 and `credit_vs_payable` matched 2 of 2. Both sides were all `equal`, with none on one side only.
- Both ties are equal at 31 March 2026 (step 6) and at 31 July 2025 (step 9).
- The sign rule, read from `credit_vs_payable`: the answer's credit bills carry the same sign as their Bills Payable rows. (verified)

Sheet `lab/2119-b/ROW.md` at a37763ba; SHA256.txt 4 of 4 equal. Reads only. Every send and stamp is in `notes.txt` (UTC), and every C3 line is in `evidence/send-log.txt`.

| step | result |
|---|---|
| Tally's release; the current date; V and M at steps 1 and 10 | TallyPrime 7.2 Silver. Current Date Monday, 1-Sep-2025, unchanged. Step 1 V 7, M 212; step 10 V 7, M 212. (verified) |
| 1 marks | 3,400 B, 0.009889 / 0.011017 s, sha256 0fe9f835a74656c10a6a0d75331a445a29e6d960be0a40d2a6c300a433b8342c: **byte-equal to 27 Oct's**. (verified) |
| 2 receivable-0331 | 1,378 B, 2.890353 / 2.894875 s, sha256 eb146596703f506fa988d2bfb39dd3dd24d0526f7728c2863112263cd591dd0e. `report ok bytes 1378 rows 3 sum -25000.00; the party's rows 3 sum -25000.00`. (verified) |
| 3 payable-0331 | 922 B, 0.629573 / 0.630596 s, sha256 58d9bfeee4bf8e21d21d724657f8cd5534cc36db2d5dc59b02752df0dad02993. `report ok bytes 922 rows 2 sum 3600.00; the party's rows 2 sum 3600.00`. (verified) |
| 4 ledgers-0331 | 10,378 B, 0.046753 / 0.048861 s, sha256 e111b013092484701c172917728a4e3dfdd90be6f5acab10070396ca4b490cf9. `ledgers ok bytes 10378 rows 6; the party's CLOSINGBALANCE -26000.00`. (verified) |
| 5 lo-0331 | 5,878 B, 1.331891 / 1.333032 s, sha256 063b3c7684b4fe416a0749c114e3b454fe59d71eb6e9bd0e01c9db25bb86de5b: **byte-equal to 27 Oct's**. `lo ok bytes 5878 rows 6 sum -26000.00 voucher_lines 9`. (verified) |
| 6 tie0331 | the whole output is below. (verified) |
| 7 ledgers-0731 | 10,378 B, 0.030829 / 0.033143 s, sha256 829ee265067cdc7d3b18c20fb276ac4d8d5e565a370bfbc13fe1c8f84328e419. `ledgers ok bytes 10378 rows 6; the party's CLOSINGBALANCE -37000.00`. (verified) |
| 8 lo-0731 | 3,926 B, 0.104002 / 0.105698 s, sha256 f505ba966c106ecd6674bffb390187e6a1e8c52ffc33d173ed87bbdd3ec88091: **byte-equal to 27 Oct's r05**. `lo ok bytes 3926 rows 5 sum -37000.00 voucher_lines 5`. (verified) |
| 9 tie0731 | the whole output is below. (verified) |
| 10 marks | 3,400 B, 0.007357 / 0.008852 s, sha256 0fe9f835…: byte-equal to step 1's. (verified) |
| screenshots | Tally's window was captured after every send; each capture is byte-equal to the Gateway capture. No dialog, and no change on Tally's screen. (verified) |
| 11 close | Shut by the top menu's Company, then SHut. The list named BRIDGE BILLTRAIL LAB alone and was answered by one click. Select Company then showed no company loaded. One tally.exe owns 9001; GET /status 200. (verified) |

**Step 6, whole (`evidence/s6-tie0331.txt`):**
```
party rows: receivable 3 (sum -25000.00), payable 2 (sum 3600.00); lo debit 3 (sum -25000.00), credit 2 (sum 3600.00), on_account 1 (sum -4600.00)
ledgers rows 6; the party: CLOSINGBALANCE -26000.00 ISBILLWISEON Yes
debit_vs_receivable matched 3 equal 3 opposite_sign 0 differ 0 only_report 0 only_lo 0 repeated_keys 0
credit_vs_payable matched 2 equal 2 opposite_sign 0 differ 0 only_report 0 only_lo 0 repeated_keys 0
tie closing_vs_lo_total equal (-26000.00)
tie residual_vs_on_account equal (-4600.00)
```

**Step 9, whole (`evidence/s9-tie0731.txt`):**
```
lo debit 3 (sum -35000.00), credit 1 (sum 3000.00), on_account 1 (sum -5000.00)
ledgers rows 6; the party: CLOSINGBALANCE -37000.00
tie closing_vs_lo_total equal (-37000.00)
tie residual_vs_on_account equal (-5000.00)
```

The `tie.py` outputs of steps 1 to 5, 7, 8 and 10 are in `evidence/s*-tie-*.txt`, whole. The sheet's inference (§12a.14) holds on this book. Step 3's answer holds exactly two rows for the party, with BILLREF `ADV-1` (dated 20-May-25) and `CN-1` (dated 1-Sep-25), and `credit_vs_payable` matched both. (verified)

The four desktop-client connector servers stayed running: no call was made to them from this sitting, and their calls were not measured.
