# One ledger created through ComplyEaze Bridge on a live book, after one No and one Yes in its approval dialog, and read back exactly (Windows lab, 14 Oct 2026)

Synthetic data only: BRIDGE MASTERS LAB (company number 100060; its GUID is in every marks answer), the book
of the masters runs. TallyPrime 7.1, Silver Rental, on Windows. One run. The build is the `bridge_mcp`
program of pull request 1650 at 13603dba (sha256 568028b8...296e), run from a desktop config entry with its
own data folder, with ledger creation allowed. The tool answers are in `transcripts/` as written down by
the operator (the program keeps only hashes and counts of them); the program's own two records were
copied after every call into `records/`. The raw reads (marks, ledgers, groups, party) were sent by curl,
one at a time. A backup was taken before the first call and another at the end. Times are UTC.

## Result

| step | call | answer |
|---|---|---|
| 2 | prepare, two names the book already holds | `master_name_taken` twice, no batch |
| 3 | prepare "MLab Live Suspense", kind suspense | a batch; parent Suspense A/c; a nine-line preview |
| 4 | post | the approval dialog opened BEHIND other windows and closed by itself after 120 s unanswered; the next post answered `master_approval_timed_out` in 3 ms with no dialog; the post after that opened a new dialog, No was pressed, and the next call answered `master_approval_declined`; nothing sent |
| 5 | post | the dialog (title "ComplyEaze Bridge - create one ledger?", Yes / No / Cancel, No the default, its text line for line the preview); Yes pressed; the next call sent the create: `posted_verified`, created 1, every other counter 0, all ten checks passed |
| 7 | post again, same batch | `posted_verified` from the record, nothing sent (0 bytes), no verification object, no dialog |
| 8 | prepare the same ledger again | `master_name_taken`, no batch |

- The raw reads after the create: the ledgers are those before plus exactly ONE new GUID; it is the GUID
  the tool and its record name; name "MLab Live Suspense", parent "Suspense A/c", bill-wise No, ALTERID one
  above the company's master mark before. Every ledger that existed before kept its GUID, ALTERID, parent
  and name; every group row is unchanged.
- The new ledger's party row: tax type Others, no duty head and no party GSTIN element, opening balance
  0.00, an empty registration list, empty address and pincode fields.
- The company's master mark (ALTMSTID) went up by exactly one at the create and did not move again.
- The program's journal record held four lines after the create (batch, dispatch intent, dispatch
  response, verdict) and did not change at steps 7 and 8; its egress log grew by two lines per call.

## Observations

- The approval dialog can open behind other windows and stay unseen; it then closes after 120 s.
- After a timed-out approval, the next post on the batch answered from the record without a dialog;
  only the call after that opened a new one.
- Two copies of this program ran at once, both started by the desktop app; this session's tool list held
  each ledger tool once, and the egress log grew only by this run's calls. Which client the second copy
  served was not measured.

## Masking

The party answers `answers/ml1-s1-party.xml` and `answers/ml1-s6-party.xml` hold the book's synthetic
registration numbers; each GSTIN value is replaced by the same-length mask ZZZZZZZZZZZZZZZ (the files keep
their sizes). The full files are listed by hash in `SHA256-kept-on-lab-pc.txt`. `derived/analysis-output.txt`
leaves the two party answers out.

## Not covered

More than one ledger; a ledger kind other than suspense; a party ledger with GST fields; a press by a
person; Gold; other releases.

## Files

`answers/` (each raw read with its headers), `transcripts/`, `records/`, `tables/`, `step0/`, `derived/`,
`tools/`, `send-log.txt`, `SHA256.txt`. Kept on the lab PC, by hash in `SHA256-kept-on-lab-pc.txt`:
screenshots, the working log and summary, the full party answers. The two backups also stay on the lab PC:
before the first call TDBK1800_100060.001, 25,779 bytes, sha256
7965bbe3e34c747e52e4a638a2934548ce5983554ce0389b90e99b44f0fb34ec; at the end the same name, 25,871 bytes,
sha256 8aac8ad4b61c3830369068bf44d8d787e5aa2762154db065ab58314e6db148b0.
