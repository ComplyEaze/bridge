# `ledger_masters_compliance_rows_before_gst_details` — provenance

The `compliance` object of every row that the `ledger_masters` tool returns for `fields=compliance`, as the tool returned it before `gst_details` existed. It is **not a Tally capture**: it is the output of code, produced by the tree before that field was added, on captured inputs. No byte was written or edited by hand.

## Provenance

- **Produced by:** the tree at `7dd498fee424d95e8f536b728a1a4f6fb9779091`, the parent of #1518 (`90bcaa67`), in a detached checkout. A throwaway test, not committed, called the tool exactly as `compliance_rows_are_byte_for_byte_the_ones_read_before_gst_details` does (`fields=compliance`, the company GUID of the test, plans `compliance_plans(masters(), balances())`), took `row["compliance"]` of each item in order, and wrote `serde_json::to_string` of that array, with no trailing newline, to the file.
- **Inputs (captured, byte-exact, unchanged by this change):** `agent/native-party-masters.utf16le.xml` (sha256 `2b4440d91fe74a308d46e35419a622790fa9faae223fcea1f72e22a4cec91709`, documented by its own sidecar), `agent/native-party-balances.utf16le.xml` (`cb4c4e1bba8c13416e0acc6e1c85185998eb81fff736dac1b3e9f049fc859525`), `agent/native-party-groups.utf16le.xml` (`64e2886ff2a155ca70a209718be3de1117b0501b48cabe2032d18bfaa1af33e6`) and `agent/native-licensed-release-companies.utf16le.xml` (`998280fac5b6b40b23ee3879dcb35780bc439083a4d1a3a3fb9a64231adbaa50`), plus the extent and currency fixtures `compliance_plans` names.
- **What it holds:** 9 rows. Each is the master fields as serialised then. Every value is null, empty, `absent` or `not_observed` (the inputs hold no PAN, bank, contact or GST registration value for these ledgers), so no party name is marked. No row has a `gst_details` key.
- **What it establishes:** that the default compliance output of the tool is byte for byte what it was before #1518, for these 9 ledgers.
- **Not established:** a ledger with a party name, a registration history or any GST detail observed, other releases, the opt-in GST details read. The byte test therefore pins the shape and the missing key, not the content of populated values.
- **Regenerating:** do not, except by the same steps on the same commit; a regenerated file from a later tree would encode that tree and defend it.

| file | bytes | sha256 | content |
|---|---|---|---|
| `agent/ledger_masters_compliance_rows_before_gst_details.json` | 2692 | `db2205855c92fff9c25fa27d0105d7dcb547a877ef8de30258c0215d5a4ba664` | the 9 `compliance` objects, as an array, before #1518 (derived output, not a capture) |
