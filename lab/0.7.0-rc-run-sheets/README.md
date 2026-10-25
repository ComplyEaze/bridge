# Run sheets for the 0.7.0 release-candidate lab sitting (eight rows)

Not for merge. The build is NOT named yet: the run coordinator fills the marks below at the cut, and a row whose change is "not in" the candidate is reported "not in RC", never run on another build. Synthetic BRIDGE companies only, each NEW where a sheet says so (named in its sheet); never a client book.

Written for a lab running TallyPrime 7.2. Earlier measurements in the sheets were taken on 7.1: record the Tally version, the licence mode (licensed, or Education with day-1/2/31 boundaries) and the date at the top of every row, and report any difference from a 7.1 measurement as a finding, not a failure. A row that posts or keys a voucher on an ordinary day needs licensed mode.

Standing rules for every row: one request at a time and no retry; Gateway of Tally with no dialog before every call; every answer saved whole with its egress line and the data-folder snapshot; marks and counts taken structurally, never by substring, and a failed request reported as failed (never as "0 rows"); backups before any keying and a restore only on the run coordinator's explicit word; STOP at any dialog, any answer outside the stated shape, or any mark that moves where a row says it must not.

## Marks the run coordinator fills at the cut
RC COMMIT (full SHA of master): ______  RUN ID / ARTIFACT ID: ______  MCPB SHA256: ______  SETUP SHA256: ______  DESKTOP EXE SHA256: ______  EXE SHA256: ______  TOOLS/LIST BYTES of the built server's answer (under 225,000, and the budget test of #1798 passes): ______
RC CONTAINS (per row: "in" or "not in"): C1 #1787 __  C2 #1806 __  C3 #1805 __  C5 #1832 __  C6 #1704 __  C7 #1814 __  C8 #1733 __

## Standing checks before any row
(a) Sales stays switched off: `LIVE_QUALIFIED_VOUCHER_TYPES` does not list Sales in the RC commit, so every Sales build is refused before anything is sent. The cut re-checks it in the code.
(b) Rows C5 and C7 write to Tally (a native post and a restore); C1 and C3 rename a master and undo it; C2, C6 and C8 only read. Another lane's rows cover its own write-path changes.

## Order (about 3 hours; cut from the bottom if short)
| # | Sheet | Company | Change | Writes |
|---|---|---|---|---|
| 1 | `C2-1806-proof-of-sync-screen.md` | any BRIDGE company with a completed run | #1806 | none |
| 2 | `C6-1704-voucher-presence-numbering.md` | BRIDGE NUMBERING LAB (new) | #1704 | hand-keyed set-up only |
| 3 | `C8-1733-audit-test-run.md` | BRIDGE AUDIT LAB (new or existing synthetic) | #1733 | none |
| 4 | `C1-1699-identity-refusal-cause.md` | any BRIDGE company X, with another loaded | #1787 | rename and back |
| 5 | `C3-1805-book-changed-next-step.md` | BRIDGE SHAPE LAB or any with 3+ ledgers | #1805 | rename and back |
| 6 | `C5-1832-windows-approval-window.md` | BRIDGE APPROVAL LAB (new) | #1832 | 1 native post, 1 cancelled batch |
| 7 | `C7-1814-restore-remembered.md` | BRIDGE RESTORE LAB (new) | #1814 | 2 native posts, hand keying, a restore |

Restore each company at its row's end, before the next row.

Not run live, by design: C4 (#1799, a snapshot request that reaches its 20-second deadline is recorded as `request_deadline_exceeded`; no cheap live trigger) and the rebuild wording of #1821; both are covered by tests only and are recorded "not run live" in the release notes. The reads lane's own rows (cost-centre allocations, party mailing details, stock items and the rest) are in its own request file and are not repeated here.

`SHA256.txt` lists every file here. Check it before the sitting starts.
