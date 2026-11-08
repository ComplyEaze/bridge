# Part W4, the export at the cut: results

First line: **W4 as EXPECT: verified.** The app's mailing post was cut at 19.7 s. The export failed with the withheld message, and no file was written. After the cut no standard master post went out until D2's admitted STANDARD. D1 to D3 and Whole ran as an Xn: the read was owed, refused four times, admitted at call 5, and 175 of 175 rows were equal. The voucher mark rose by exactly 1535, the front's own count. The book is left rebuilt: restoring BK-2074-2214 by C6 waits for the run lead's word.

Sheet `lab/2074-2214-export-cut` at `7a49b5c1`, kit 11 of 11 OK by its `SHA256.txt` (checked again at the close). Build: the merge-queue run's artifact `unsigned-bundle-smoke-windows-latest` for head `1f5dc024`, 59,872,060 B, GitHub digest `sha256:a6eca6d0c8511c8f031f4a78c9227146a04680dfd97c755a9507e451f323d0b6`. The NSIS setup was run by the owner from Explorer into a real per-user folder, and the screen grant resolved to that folder. `bridge_mcp.exe` from the same bundle, sha256 `58860204d114ebb0cdab106f35d3753b9f42d36d7bdf10b9feb0f25bac4fecd6`, equal to the bundle's `binary_sha256`, unchanged at the close. Server 0.7.0. TallyPrime 7.2 Silver. Book BRIDGE BATCH LAB, synthetic. All times are UTC on 8 Nov 2026. Every number is read from its file here, and `evidence/notes-w4.txt` has each step.

## Rebuild to the X state (verified)

| step | result |
|---|---|
| C5 | A new backup was taken before any write: one file, 57,180 B, sha256 `2f39e73e51bf194b1a0129028b87a01e60c337cb03f9d6f79f3be39bd5d2f174` (kept on the lab PC). |
| P0, S0 | marks 224 / 87; the P0 answer is byte-equal to 3 Nov's (`566be20d...`). |
| A1 | create-probe CREATED 1 (M 225); create-000 CREATED 1 (M 226). |
| M1 | create-001 to create-160, each CREATED 1; M 386; the runner halted once on a capture of the Gateway without focus, which was looked at and resumed (notes). |
| F0 | front-ledgers CREATED 2 (M 388); front-probe CREATED 500 in 16.281 s (V 87 to 587, M 388); capture with no dialog. |
| F1, F2 | n50 = floor(500 x 50 / 16.281) = 1535, n17 = 522. First indices: x1 500, x2 2035, w4 3570, x3 5105. `front-w4.xml` 1,793,044 B, sha256 `93142582...`. |
| R1a, R1b | STANDARD 120 s apart: total 175, marks 388 / 587 (the sheet's EXPECT of 11 was for before the rebuild). Master posts held 107 / 78 ms and 71 / 62 ms, so the post threshold is max(214, 2,107) = 2,107 ms. Drain record clear. |

## W4

| step | EXPECT | measured | grade |
|---|---|---|---|
| 1 | drain record clear; armed; export | `{"state":"clear"}` at 13:06:46.358Z; armed 13:06:46.383Z; export pressed at about 13:06:54Z | verified |
| 2 | the mailing post cut at about 20 s | front written at=196062.640; the app's 1784 B post (front 1) forwarded at 196062.953; `bridge_cut` at 196082.640, **19.687 s**; the app's connection closed at the cut and sent nothing after | verified |
| 2 | fails, with no file written | the message below; no file in Downloads, Documents or Desktop written after the export | verified |
| 2 | message text | opens "ComplyEaze Bridge withheld the party/ledger master:" and holds "The read that failed also asked Tally for each party's dated mailing addresses" | verified |
| 3 | after `bridge_cut`, no standard master post until D2's STANDARD | the first 1736 B post after line 539 is line 645, at 13:08:23Z, D2 call 5's; between them, only status gets and 2524 / 1094 / 966 B posts | verified |
| 4 D1 | the drain record owed | `owed`, answered 0, at 13:07:15.587Z | verified |
| 4 D2 | STANDARD until admitted | calls 1 to 4 (13:07:15Z to 13:08:07Z) refused `tally_endpoint_draining`, `retry_after_s` 10, no master post sent; call 5 admitted, total 175, rows 175 | verified |
| 4 D3 | gap; held_ms against R1's threshold | gap from D1 to the admitted read's end 68.155 s; master posts held 84 and 69 ms, under 2,107 ms | verified |
| 4 Whole | one whole read, rows equal | total 175, rows 175, equal 175 of 175 | verified |
| marks | the voucher mark up by the front alone | ALTMSTID 388 (unmoved), ALTVCHID 2122 = 587 + 1535; the front's answer: CREATED 1535, ERRORS 0, LASTVCHID 2118 | verified |
| drain | clear after | `{"state":"clear"}` after call 5, Whole and the marks | verified |

The app's message, whole: "ComplyEaze Bridge withheld the party/ledger master: The bounded Tally read exceeded its production deadline. The read that failed also asked Tally for each party's dated mailing addresses, which makes Tally's answer larger than the read without them. [tally_request_deadline_exceeded] Do not retry the unchanged export: it sends the same larger read. Try again only after something has changed, such as Tally being less busy or this company's books being smaller; if it keeps failing, report it to ComplyEaze Bridge, which cannot yet export this company's workbook without the mailing addresses."

D2's remediation, whole: "Tally was still busy with an earlier request that did not finish, so this request was not sent. Call again after retry_after_s seconds; the same request is safe to repeat, and a different read would only wait behind the same work. It clears by itself within 10 minutes of the earlier request."

The front was answered at 196132.796, 70.156 s after it was written. Each refused D2 call first sent a status get that ended `request_deadline_exceeded` at about 5,000 ms, then refused its post at 0 ms.

## Deviations

- The runner's D3 log line says "against 2026 ms": it is a constant carried over from the 2214 runner's copy. This sitting's threshold is 2,107 ms, as above; both values are well above 84 ms.
- M1's runner halted once after create-003, when Tally lost focus to the app's window; the capture was looked at (the Gateway, no dialog), added as a known capture, and the run resumed at create-004.
- The app first pointed at its own default port; it was set to 127.0.0.1:9002 in Settings, and Check Tally showed Reachable before W4.

## Not done here (unverified)

- Close 3 to 5: the restore of BK-2074-2214 by C6, MARKS after it, `ledger_masters` basic on port 9001 and C10 wait for the run lead's word. The book is left with M 388 and V 2122.
- Parts W1 to W3 and O were not part of this sitting.

## Files

- `run2/answers/`: every tool answer (company_number redacted by a JSON parser); `run2/client-log.txt`, `launch-env.json`, the server's stderr.
- `relay/relay.jsonl`, `relay/bodies/` (named by the originals' sha256), `relay/front-1.answer`; `evidence/relay-events.txt` lists its events and master posts, parsed.
- `debt/`: the drain record copies. `reads/`: the raw marks and creates reads. `answers-creates/`.
- `evidence/`: `notes-w4.txt`, `notes.txt`, the runners' outputs. `scripts/`: the helpers.
- `kept-local.txt`: what stays on the lab PC (screens, the backup, the generated requests and fronts, the downloads, process and port listings), by sha256. `substitutions.txt`: what the pushed copies changed. `SHA256.txt`: every file.
