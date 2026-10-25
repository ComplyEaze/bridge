# C2 (#1806): does the Proof of Sync screen show no rejected-row count?
WHY: the screen reported "0 rejected rows" and an "Accepted / rejected" column that could only read zero. It now shows accepted rows only. Rendered-test covered; this is a look at the real desktop app.
COMPANY: any BRIDGE synthetic company with a completed snapshot run on the release-candidate desktop app. Writes nothing to Tally.
STEPS:
1. Open Proof of Sync after a completed run. Screenshot the summary line of the latest attempt and the proof table (header and rows).
2. Read the summary line aloud into the record, word for word.
EXPECT: the summary has no "rejected rows" clause and reads "... N provenance-backed accepted canonical rows and M canonical rows with an explicit provenance-unavailable gap ..."; the table header reads "Accepted returned rows" and each row shows one number.
PASS = both hold. FAIL = any rejected count, or "N / 0" in a cell.
NOT RUN LIVE: the gap text "Source records were rejected ..." (a run that rejects source records cannot be produced on demand); record "not run live".
STOP RULES: none beyond the standing ones.
