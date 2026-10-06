# Outstandings party-detail request sequences — integrity digests

`agent/native-outstandings-detail-sequence-unadjusted.json` and
`agent/native-outstandings-detail-sequence-bill-trail.json` are their own
provenance records. Each JSON object carries its `source`, `observed_date`,
`transformation` and `provenance_limit`, and no other file shares its stem.
They were added in `cbfb1bb8` (#981).

A file cannot hold its own hash, so their integrity digests are kept here. Each
row is the SHA-256 of the file's committed bytes, which
`scripts/check-fixture-provenance.mjs` checks (#838). A digest pins the bytes as
committed and claims nothing about where they came from: what each file
establishes is what its own fields say.

| Fixture | Bytes | SHA-256 (integrity digest) | Capture |
| --- | ---: | --- | --- |
| `agent/native-outstandings-detail-sequence-unadjusted.json` | 27,538 | `9695b8107905bb4483ef8c82ad0ba9927ac52fb3ddc830d1b3897de806a16817` | as the record's own fields say |
| `agent/native-outstandings-detail-sequence-bill-trail.json` | 27,780 | `07a1512d5af5ca7457a0d6664906ef2fedf20d1501f764c5912e635f3e91c610` | as the record's own fields say |
