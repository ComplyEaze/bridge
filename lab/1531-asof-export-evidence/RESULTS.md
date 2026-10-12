# #1531 as-of export: live run on Windows (synthetic BRIDGE BATCH LAB), 12 Oct 2026

Build: the desktop app from Bridge CI run 38192614653 (head 00babcf7b0e12db11436b90d537e769817a9080a), artifact unsigned-bundle-smoke-windows-latest (id 11723395045, 51,886,385 B).
- Installer used: nsis/Bridge_0.5.1_x64-setup.exe, 16,028,263 B, sha256 a7bd9fed667945cde07b5575566a61e50ad713a8aeed993fa12293814f1ba194. It matched the pinned hash before running. The msi also matched (12ad8069845bcf6105cba3d5ecb4a2ef8975f34ecaa6c5e6199030d19edbc12b) but was not used.
- Before: no ComplyEaze Bridge desktop app was installed. Its data folder from earlier installs existed (tally-mirror-v1.db, July; WAL 9 Oct).
- What the installer did: /S was ignored and the wizard ran: Welcome, Apache-2.0 licence (accepted with the owner's yes), per-user folder <userprofile>\AppData\Local\Bridge, Install, Finish. Desktop shortcut unticked; "Run Bridge" left ticked.
- Result: an HKCU uninstall entry "Bridge" 0.5.1, publisher complyeaze. bridge.exe sha256 66ec1c544280331ad7bf65481613fa3c968cae24059158f366011e935b62343d; bridge_mcp.exe 9db5345ee771a72b38badc629dfa1511f0f885c5742be5a5d2d976cc5d110b7f.
- The app opened a console window next to its UI. It stayed empty.
Tally: TallyPrime 7.1 Silver, Education off, port 9001. BRIDGE BATCH LAB (100050) loaded alone; Edit Log not running.
Book, read before the steps:
- Books from 20260401 (app header, extent read).
- Last voucher date 20260821 (Tally Gateway "date of last entry", and the app's own master read SVTODATE).
- Vouchers in 2026-07 (61) and 2026-08 (17), from a Bridge MCP month summary (book-extent-note.txt).
Request log: the app was pointed at a logging proxy (tools/logproxy.py: 127.0.0.1:9011 → 127.0.0.1:9001, one request at a time, every request and response saved). The app's port was set to 9011 for the run and back to 9001 afterwards. stepN-index.tsv columns: seq, start, end, method, path, request bytes, request sha256, status, response bytes, response sha256. stepN-request-structures.txt groups the requests by identical body (ID, collection, SVFROMDATE/SVTODATE). Distinct request bodies are in requests/.
Every step started from the Overview screen; the date was typed into As of and then the "Ledger master + group subtotal trace" button was clicked, with no Refresh.

| step | As of typed | app message | workbook (Balances as of) | requests (all HTTP 200) | dates sent |
|---|---|---|---|---|---|
| 1 default | left at the screen's default 12-10-2026 (today), not touched | "Saved party-ledger-master-bridge-batch-lab-20260831.xlsx to Downloads" (no further note) | 20260831. No line about a moved date; Read period 20260401 to 20260831 | 41: 18 status GET, 7 BridgeCompanyExtent, 8 BridgeCompanyBookExtentV2, 2 BridgeCompanyCurrencies, 2 List of Ledgers (master), 2 List of Ledgers (balance), 2 List of Groups | master SVTODATE 20260821; balance SVFROMDATE 20260401, SVTODATE 20260831 |
| 2 after last voucher | 30-09-2026 (month end) | Saved …-20261001.xlsx, plus: "The date chosen for this export, 20260930, is not a date Tally accepts as the end of a balance period. The balances end at the next date it accepts, 20261001 (the 1st, 2nd or 31st of a month), and include the vouchers dated 20261001." | 20261001, with the same sentence on the Ledger master sheet (row 3) and in Read period | 41, same structure as step 1 | master 20260821; balance 20260401 to 20261001 |
| 3 before last voucher, not a boundary | 15-07-2026 | Saved …-20260731.xlsx, plus the moved-date sentence (20260715 → 20260731) and: "Vouchers dated after 20260731 (the book's last voucher date is 20260821) are not included in these balances. Review them in Tally before relying on this workbook. The balances end at the first month boundary (the 1st, 2nd or 31st of a month) on or after the date chosen for this export, 20260715." | 20260731; both dates stated in row 3 and Read period | 41, same structure | master 20260821; balance 20260401 to 20260731 |
| 3b moved-to day carries a voucher | 30-06-2026 | Saved …-20260701.xlsx, plus the moved-date sentence (20260630 → 20260701) and the "vouchers after 20260701 … last voucher date is 20260821" note | 20260701 | 41, same structure | master 20260821; balance 20260401 to 20260701 |
| 4 before books-from | 31-03-2026 | "ComplyEaze Bridge withheld the party/ledger master: the As of date is before this company's books begin. No ledger master was requested. [as_of_precedes_books_from] Choose an As of date on or after the start of the books, then export again." No file saved | none | 22: 9 status GET, 5 BridgeCompanyExtent, 6 BridgeCompanyBookExtentV2, 2 BridgeCompanyCurrencies. NO List of Ledgers, NO List of Groups | none |

Step 3b (is the moved-to day inclusive?): YES, inclusive [verified].
- The book's only voucher up to 15 Jul is Receipt 1 of 20260701: Cash Dr 1,000.00 / Lab Seed Income Cr 1,000.00 (step3b-bridge-mcp-vouchers-0630-0702.json). No voucher is dated 30 Jun, and no Journal was created.
- The workbook at 20260701 shows Lab Seed Income closing 1000.00 (Cr in Tally's sign) and Cash -1000.00 (Dr). Profit & Loss A/c is 1000.00; every other ledger is 0.00.
- Tally's own Ledger Vouchers for Lab Seed Income, 1-Apr-26 to 1-Jul-26, shows Receipt 1 of 1-Jul-26, closing balance 1,000.00 Cr (screenshot kept on the lab PC). They agree.

Other observations
- Step 1: the default (12-10-2026 untouched) ended at 20260831, the month boundary after the last voucher, with no note in the workbook or on the screen. Step 2's typed 30-09-2026 instead moved forward to 20261001 and was not pulled back to the last voucher. [inferred] The default keeps the pre-change path (balance end from the book's last voucher), and a typed date takes the new one. Whether step 1 equals master's own output was NOT compared: master was not run on this book. Unverified.
- An SVTODATE later than the last voucher (the unverified clamp case) answered normally. The balance read with SVTODATE 20260831 (step 1), 20261001 (step 2) and 20261012 (the Overview auto-load before step 1, log-setup) returned byte-identical responses: 16,518 B, sha256 980fdc4efcaf856d8f57a9ff24e86b441c00e579b009a96f39f159963dfb86be. There is no voucher after 20260821 to tell the dates apart, so this shows Tally accepts those dates. It does not show which day the balances end on.
- 30 Sep, a calendar month end, was treated as not a boundary (the app says the 1st, 2nd or 31st).
- Each source was read twice (pairs in the log) and the workbook says the paired bytes agreed.
- Lab Sales closing reads "Not established" in steps 1-3 (empty CLOSINGBALANCE), and Lab Capital and Suspense also read "Not established" at 20260731. They are in the workbook's exclusion list.
- The Overview showed "Tally did not confirm this as-of date … withheld the totals" on the auto-load. That is not part of the export.
