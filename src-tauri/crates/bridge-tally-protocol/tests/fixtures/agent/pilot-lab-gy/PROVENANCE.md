# Pilot lab row 2162-GY fixtures

Two answers Tally gave to the company object read (`TYPE` Object, `SUBTYPE` Company, every field) on the
synthetic company `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`), TallyPrime Silver
(`tally.exe` file version 1.1.7.2), loaded alone. Row 2162-GY ran on 2026-11-09 (UTC) in a sitting of its own,
with raw reads only, one at a time. Each answer was written straight to a file. The book was backed up
before the change and restored from that backup afterwards. The data is synthetic.

- `gy-s2-company-object.utf16le.xml`: step s2, before the change. `ISGSTON` reads `Yes`.
- `gy-s4-company-object.utf16le.xml`: step s4, after the company's GST setting (`Enable Goods and Services
  Tax (GST)` on Tally's company features screen) was set to No and saved through Tally's own message.
  `ISGSTON` reads `No`.

The answers are on `lab/2162-gy-answers` at 3677b783, as
`answers/a-s2-O-object.number-redacted.xml` and `answers/a-s4-O-object.number-redacted.xml`. Each file
here is byte-equal to the file there, UTF-16LE with no byte-order mark.

| Fixture | Bytes | SHA-256 | Published as | What it is |
| --- | ---: | --- | --- | --- |
| `gy-s2-company-object.utf16le.xml` | 85,654 | `b7f824cbd2562d03fca635ce76c86be18da149ae33e36630ffd07eeb8cc5b8c1` | `answers/a-s2-O-object.number-redacted.xml` | the company object before the change, `ISGSTON` `Yes` |
| `gy-s4-company-object.utf16le.xml` | 84,454 | `59355fd5f32fa405b4aedff181d29baef7d929f205b29c8832521dc309a9447a` | `answers/a-s4-O-object.number-redacted.xml` | the company object with GST turned off, `ISGSTON` `No` |

They are copies, not the bytes Tally sent. Following `evidence/o-redaction.txt` on that branch, every
occurrence of the company number is replaced by the same number of capital `Z` characters, in the file's
own encoding. Each file has one such replacement, in `COMPANYNUMBER`, and no other byte is changed, so
each size equals the original's. The originals stay on the lab machine.

What the row measured, and what it did not, is in `RESULTS.md` on the same branch. It is one run: partial.
