# Results: #1709 first-voucher captures, rerun of steps 4 to 6 on #1867 head 796e94f1 and TallyPrime 7.2, 27 Oct 2026. All three calls complete; the day-before read forwarded

The sheet is `lab/gs2-l3/RERUN-STEPS-4-6.md` with `WINDOWS-GS2-L3.md` at 882a2aae474ed830ca16d415dfd352e7ec4b5ec4 (SHA256.txt: 3 of 3 equal). The first sitting's evidence is on `lab/1709-first-voucher-answers`.

## One variable
- The recorder at 300ea3c3: 21 files equal to its SHA256.txt, the recorder script 34,437 bytes sha256 40a9c57cf494d9f7101e5f83a1802c359b66f8d908c6999ca94e095b22cbb9a0, FIRST_DATE 20250331.
- Its self-test printed `self-test: ok (11 forwarded, 42 refused, none of the refused reached the target)` and exited 0.
- It recorded into a new folder, committed here as `recorder-rerun/`.
- The build, the company, the arguments and the steps are the first sitting's.

## Build and setting
- `bridge_mcp.exe` of #1867 head 796e94f1, sha256 bf8cdf82e10bf22f693d3c5d6d89bd074b35b2dbf6dc2cbad748ac8609916f46, before the first call and after the last. It reports 0.6.0.
- One launch over MCP stdio, 12:27:37 to 12:29:39 UTC, port 9011 (the recorder). Posting and ledger creation were off. The server's error stream stayed empty.
- TallyPrime 7.2 (Latest), Silver (the About screen).

## Steps
| step | act | result |
|---|---|---|
| R0 | no company loaded; BRIDGE FIRST VOUCHER LAB (100085) in Select Company's list; loaded alone; "No Vouchers Entered" | PASS |
| R3 | `list_companies` (000) | one company, GUID f88ff11c-6fa1-4a73-8228-ab1c8c72ad4a, the first sitting's: PASS |
| R4 | `purchase_register` 2025-04-01 to 2025-04-30 (001) | `state` complete, `total` 0, `vouchers_observed` 0, every listed section empty. The read for 20250331 to 20250501 is recorder lines 102 and 104, both `forwarded` (200) |
| R5 | the same call again (002) | complete, total 0, vouchers_observed 0 |
| R6 | the company shut and loaded again alone; the same call (003) | complete, total 0, vouchers_observed 0 |
| R7 | company shut, no company loaded; one tally.exe owns 9001; GET /status 200 | the company is kept as it is |

No answer shows a voucher in the company.

## What the recording holds for the requester
- `recorder-rerun/manifest.tsv` has 328 lines, none refused, each request and answer kept byte for byte with bytes and sha256.
- There are 12 company-marks reads (`Bridge Agent Company High Water`), four per call: R4's are lines 56 and 58 among them.
- The voucher-window reads of each call are there too: the April window, and the widened 31 March to 1 May read.
- Nothing is judged at the lab.

## Files
- `answers/000-003`: the four tool answers whole.
- `recorder-rerun/`: the recorder's folder whole.
- `notes.txt`: UTC times.
- `SCREENSHOTS.txt`: the screen captures by name, bytes and sha256; the captures stay on the lab PC.
