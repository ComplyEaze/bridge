# R3: RC-SETTLE on the 0.7.0 release candidate: not completed

R3 not completed: Tally's period lock needs user security (a user and password); nothing saved; book as found.

Build: candidate A, RC COMMIT 026ba163dfcbf75d515a24d0dbbf14c5245534ec, EXE SHA256 576cc0c5aab4e72c43c05799116d76a29429c595a86dd54834d841c551f50d7c (bridge-tally 0.6.1; #2150 with its system-modal settle box, and #2183, in the build). Company: BRIDGE ABANDON LAB (GUID 5f4825c4-5d89-47fa-9872-6397cca3a591), synthetic, books from 1 April 2025. TallyPrime 7.2 Silver. 30 Oct 2026, 06:47 to 06:54 UTC. Folder `<LAB_DIR>\`.

| EXPECT | result | confidence |
|---|---|---|
| 0.1 C2, one Tally on 9001, raw MARKS | PASS: ALTMSTID 226, ALTVCHID 48000 | verified |
| 0.2 BK0 | taken: 13,255,946 B, sha256 3b91686d299602ce0cdd848b7d20431b30601986cfa3ecb2be46309823570937 | verified |
| 0.3 L1, L2 not bill-wise, not cash or bank; one Currency | PASS: Lab Bulk Expense (Indirect Expenses) and Lab Bulk Income (Indirect Incomes) have no bill-by-bill field, which reads No; one currency | verified |
| 0.4 one company; the Journal type automatic / auto_retain | PASS | verified |
| 0.5 MARKS; no voucher on THE DATE | PASS: M0 226, V0 48000 (reused false); 30 Apr 2026 empty | verified |
| 2.1 lock the period in Tally | **stopped by the step's own rule**: Company > Security ("Security And User Access") offers only "Control User Access to Company Data: No"; TallyPrime's back-dated-voucher limits are set on User Roles, which need user access control on with users and a password; F11 Features has no lock. Nothing was saved | verified (the screens); partial (that no other lock control exists: TallyPrime's help, Data Security FAQ and Manage Users) |
| steps 2.2 to 7 | not run | |
| outcome (a), (b) or (c) | none: the post was not built or sent | |

The book is as found: nothing was saved, nothing was sent, and the marks read 226 / 48000 after the stop. The posting launch was stopped and the posting key removed. BK0 is kept on the lab PC; no restore was needed. Part 2: not available (the R3 NOTE). The settle dialog of #2150 stays covered by its tests only. The lab sets up no security, user or password itself: that is the owner's to do at the PC. If he does, R3 restarts from step 0 with a fresh BK0, and the restore at its end removes the security again (the run lead's word).

Notes, one line per step with UTC times: `notes.txt`. Answers whole: `answers/` and `raw/`. Captures and BK0 stay on the lab PC, listed in `SCREENSHOTS.txt` and `kept-local.txt`.
