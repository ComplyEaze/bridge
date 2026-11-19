# 2214 export cut on the current desktop app: answers

Sheet `lab/2214-cut-sheet` at `a127d83b3f8f3b5cda7ce2a0887bb09c187f3479` (folder `lab/2214-export-cut-current-app/`), run 2026-11-19 on TallyPrime Silver Release 7.2 (tally.exe ProductVersion 1.1.7.2, Licensed) against the synthetic book BRIDGE BATCH LAB (GUID `a30f23c3-4819-4334-96ab-ffa6fba70976`), behind a backup taken at S0 and restored at Close.

**First line:** every EXPECT was met except one wording point in W0: the app sent its mailing master read twice (both answered), where W0's EXPECT says one. W4 met all 12 of its EXPECT lines; it is not VOID.

## Build

| item | read at the lab |
|---|---|
| installed app, `bridge.exe` | sha256 `8c1a640d4fb8a8e2586683beb90bd421776bae07908bf03f351906230e53a8ed`, version 0.7.0, at P0 and Close |
| installed app, `bridge_mcp.exe` | sha256 `7086f1d0d7430e7dc1fefd6f69c0d4a14d717a2e06477b32935a1d3aa4fee1b8`, version 0.7.0, at P0 and Close |
| server exe used for every tool read | the bundle's `bridge_mcp.exe`, the same sha256 |
| bundle zip | 60,535,877 bytes, sha256 `4f6e91b082d5fde228aded36713e65dcb0f65f36a7fb7f13bb03730aa8a72734`, equal to the GitHub digest |
| kit | 21 of 21 files equal to the sheet's `SHA256.txt`, at P0 and Close |

## Steps

| step | result | grade |
|---|---|---|
| P0 | THE COMPANY alone loaded; one tally.exe owns 9001; `/status` 200; marks 224 / 87 | verified |
| S0 | backup into a new empty folder: one file, 57,180 bytes, sha256 `7cec76828a020ee8496ad4d842ae163a794e34723f575c3ab8fa20a9ada592cb`; marks after it 224 / 87 | verified |
| B5, proxy, server | no Bridge process of ours before the start; the proxy listens on 9002; server 0.7.0 with writes off; `list_companies`: THE COMPANY alone, `verified_tuple` | verified |
| R1 | two standard reads 120 s apart: `total` 11 each, marks 224 / 87, drain record `clear`. Slower master post 24 ms; this row's threshold is 2,048 ms | verified |
| F0 | ledgers front: `CREATED` 2, marks 226 / 87. Probe of 500 Journals in 8.015 s: `CREATED` 500, every other counter 0, no `LINEERROR`, marks 226 / 587. No dialog after either | verified |
| F1, F2 | n50 = floor(25,000 / 8.015) = 3,119, past the cap: 3,000. `front-w4.xml` 3,502,700 bytes, sha256 `ffe8cc8eff1c872474e1e6079d99a28712a8463e6f704038d691a0704e7d7c60`, indices 500 to 3499 | verified |
| W0 | endpoint 127.0.0.1:9002, Reachable, THE COMPANY alone offered and chosen. Export pressed with As of 19-11-2026: workbook `party-ledger-master-bridge-batch-lab-20260831-4.xlsx`, 10,495 bytes, sha256 `c05afd0ea3e4e30dfc0d04dc2dbe49b477aaae2330095600384c909dbc5e8948`, the only new file. Cut log: two `mailing master` requests (req 85 and 87), both `answered`; no front and no cut | partial: two mailing master requests where the EXPECT says one |
| W4 | 12 of 12, below | verified |
| Close | server and proxy stopped; app left running; hashes equal to P0; backup restored; marks 224 / 87 with the answer byte-equal to P0's; a basic `ledger_masters` on 9001 gives 11 ledgers; THE COMPANY shut, no company loaded, `/status` 200 | verified |

On opening Overview, before the press, the app read Tally and showed: "Tally did not confirm this as-of date", saying the totals were withheld because Tally returned no bill references while the ledger still carried a balance. It is recorded, not graded.

In W0 every post and status in the cut log comes in pairs, as in the R1 reads' trails; the two mailing requests are one such pair, sent 0.015 s apart with the same bytes.

## W4, each line graded from the file named

| # | EXPECT | read | grade |
|---|---|---|---|
| 1 | arm, then `front_send` (its sha256 equal to F2's), `front_written`, `forwarded` for the mailing request with `front` 1, `bridge_cut` | armed 13:12:41.711Z; proxy lines 658 to 661 in that order; `sent_sha256` `ffe8cc8e...`, equal | verified |
| 2 | that request a `mailing master`, `CUT`, 15 to 25 s after forwarding | req 126, `CUT`, 19.672 s | verified |
| 3 | nothing forwarded on the cut's own connection after it | "on the cut's own connection after it: 0" | verified |
| 4 | the export failed with the sentence and `[tally_request_deadline_exceeded]` | `evidence/w4-message.txt`, quoted whole below | verified |
| 5 | no file written | no new file in the three folders; counts and date, time and size columns equal; no write time after the arm | verified |
| 6 | no master post before the admitted STANDARD | 8 master posts after the cut, all `standard master answered` on the server's connection at 13:14:42Z and 13:14:44Z, after the admitted call was sent (13:14:38.923Z) | verified |
| 7 | D1 drain record `owed` | `owed` at 13:13:08.647Z | verified |
| 8 | D2 calls `tally_endpoint_draining` with `retry_after_s`, no master post; admitted within 20 calls and 3 minutes | calls 1 to 6 refused, `retry_after_s` 10 each, master posts none; admitted at call 7, 93.903 s after D1 | verified |
| 9 | admitted read's master posts under 2,048 ms; `total` 13; Whole equal to it | 10 and 9 ms; `total` 13; Whole 13 rows, equal 13 of 13 with the snapshot left out | verified |
| 10 | front answer `CREATED` = n50, others 0, no `LINEERROR` | `CREATED` 3000, others 0, no `LINEERROR` | verified |
| 11 | marks 226 and 587 + n50 | 226 / 3587 | verified |
| 12 | drain record `clear` after Whole and marks | `clear` | verified |

The message, whole (596 bytes with its line end, sha256 `d284fb56896853a9e0fb8fa35c462709a28bc801b0b77e0e7afbab19df7852e4`), read from the window's text:

> ComplyEaze Bridge withheld the party/ledger master: The bounded Tally read exceeded its production deadline. The read that failed also asked Tally for each party's dated mailing addresses, which makes Tally's answer larger than the read without them. [tally_request_deadline_exceeded] Do not retry the unchanged export: it sends the same larger read. Try again only after something has changed, such as Tally being less busy or this company's books being smaller; if it keeps failing, report it to ComplyEaze Bridge, which cannot yet export this company's workbook without the mailing addresses.

Each refused D2 call's trail holds one status probe that hit `request_deadline_exceeded` after about 5 s (the server cut those connections, lines 664 to 680 of the cut log) and a post refused at 0 ms; call 6's probe was answered while the record still read `owed`, and call 7 was admitted.

## Decides

Per the sheet's table, the first row applies, with W0's wording point recorded: on the current app the export fails closed on a cut with its sentence, writes no file, and sends no master post before the drain clears. `Refuse` stays as it is; this sheet moves nothing.

## Notes on the record

- `w4.py` prints "against 2026 ms", a constant carried from an older row; each held time was compared with this row's 2,048 ms by hand.
- The four folder listings (W0 and W4, before and after) name the lab PC's own files in Downloads, Documents and Desktop. They stay on the lab PC by size and sha256 (`kept-local.txt`); `evidence/listings-compare.txt` gives the counts and the differences.
- The workbook stays on the lab PC; its name, bytes and sha256 are above.
- Screens and captures stay on the lab PC, listed in `kept-local.txt`.
- After the restore, THE COMPANY's data folder lacks seven side files it held before, as after this book's earlier restores; they return when the book is loaded.
