# Results: 690 batch ledger rows (#2181, Part of #690): the ten-ledger box declined, then three ledgers created in one approved request, 2 Nov 2026

**PASS for Part B. L1 passed on every EXPECT that can be measured; one part of L5 could not be measured, recorded below. The ten-ledger box is wholly on the lab's screen at 125 percent, and declining it sent nothing. Three ledgers in one approved request read `posted_verified`, with the mark up by three. The same call again opened no box and sent nothing. The book was restored.**

BRIDGE LEDGERBOX CI LAB (synthetic), TallyPrime 7.2 Silver, licensed, loaded alone. Screen 1920x1080 at 125 percent (DPI 120), work area 1920x1001. Sheet `lab/690-batch-ledger-rows` at c8ab4506; `SHA256.txt` equal. Build: #2181 head 70dced80, run 41436265786, artifact 13140761067, zip digest equal; `bridge_mcp.exe` sha256 af74d0ced3f5f4056f1d9848ccca5f6fd30b54454459e7696c2d6c71203f7973, equal to the package's smoke file and before and after; `initialize` 0.7.0; 38 tools. The box was answered by mouse at its button, never by key. Times are UTC.

## Step 0
`list_companies`: the company alone, `verified_tuple`. M0 208, three ledgers (Cash, LBox Suspense, Profit & Loss A/c). No `L0b Box` or `L1 Creditor` row. [verified]

## Part B: the ten-ledger box, declined
| step | EXPECT | found | |
|---|---|---|---|
| B1 | a `batch_id`, a preview of 27 lines | `ledger_count` 10, 27 lines: five header, two per ledger, a blank line, the warning | met |
| B2 | the box's title and whether it is whole | title `ComplyEaze Bridge — create 10 ledgers?`; rect 696,171 to 1242,935 in physical pixels, inside the 1001-high work area; the title bar, answer line, all ten ledgers, the warning and the buttons Yes, No and Cancel all on screen, none cut; topmost, foreground, No the default | whole |
| B3 | `master_approval_declined`, `attempt_recorded` false, `not_dispatched` | all three; 20.42 s, the box seen to the click 18.4 s | met |
| B4 | M = M0, same rows | M 208, same rows | met |

So `MAX_LEDGERS_PER_BATCH` at 10 fits at this screen size. [verified: one screen size]

## L1: three ledgers, Yes, then the same call again
| step | EXPECT | found | |
|---|---|---|---|
| L0 | M0' | 208; BK0 taken first (23,613 B, sha256 in `kept-local.txt`) | met |
| L1 | a `batch_id`, a preview naming the company and the three ledgers | both | met |
| L2 | M = M0' | 208 | met |
| L3 | the box whole; Yes; `posted_verified`, attempt recorded, dispatched, not resent, created 3, others 0 | title `ComplyEaze Bridge — create 3 ledgers?`, rect 686,304 to 1252,802, whole. All as expected. 19.14 s; no `pending` | met |
| L5 | M1 = M0' + 3 | 211 | met |
| L5 | three rows under Sundry Creditors | all three, parent Sundry Creditors | met |
| L5 | each GUID begins with the company's GUID | L3's own `verification.guids` lists three, each so | met |
| L5 | every other row unchanged in GUID and ALTERID; the three ALTERIDs | **not measurable**, see below | - |
| L6 | no box; `posted_verified`, dispatched, not resent, `journal.response_recorded` true, counters created 3, verdict `posted_verified`; under a second | all; 0.227 s; no box in 7 s of watching | met |
| L7 | M = M1 | 211 | met |
| L7 | no entry for L6 beyond its own `response_prepared` line | its `response_prepared` line with an empty request trail: nothing sent. One `stdio_write_completed` line follows, as it does after every call; read here as the delivery of that answer, not a second entry | met |
| L8 | M = M0' and the same rows as L0 | M 208; the answer equal to L0's apart from timestamps, hashes of the timed read and the snapshot id | met |

## Recorded as asked
- **L5, not measurable.** `ledger_masters` in both its forms returns no GUID and no ALTERID per row, and the sheet bars raw reads, so none was added. The evidence in their place is L3's `verification`: `guids` as above, and the checks `guid_new`, `pre_image_unchanged` and `target_mark_moved`, each `passed`. That is the build's own statement, not an independent read. [partial]
- **`target_mark_step_one`** reads `passed` on a step of 3. At #2181's head it passes when before + count equals after (`agent_master_verdict.rs:756`), so 208 + 3 = 211 is a correct pass; only the name is narrow. The code was read by the run lead. [verified for this reading]

## Restore
On the run lead's word: the company shut, BK0's hash checked and the folder holding that one file, Tally's Restore from BK0's folder, this company only; the screen's date and time were the backup's; prompts `Accept ?` and `already exists`, answered Y and R. Loaded alone, then L8 above. L9: exe hash equal; company shut; Select Company, none loaded; one tally.exe owns 9001; `GET /status` 200.

## Deviations
1. **The branch** is `lab/690-batch-ledger-answers`, on the run lead's word; the sheet names `lab/690-batch-ledger-rows-answers`.
2. **The box was read from a capture and the window's rectangles**, with no key and no screen-tool call while it was open, as the sheet's rule 7 asks.
3. **The About screen was not opened:** release and edition come from `tally_status` and the title band.
4. **One more read than the sheet lists:** `ledger_masters` with `fields` compliance after L5, to look for ALTERIDs. It moved nothing.

**Confidence:** one screen size, one book, one sitting, 7.2 Silver: PARTIAL. Not measured: a box at 1366x768 at 150 percent, a person's own click, ten ledgers written through the build, macOS, Gold, Education.
