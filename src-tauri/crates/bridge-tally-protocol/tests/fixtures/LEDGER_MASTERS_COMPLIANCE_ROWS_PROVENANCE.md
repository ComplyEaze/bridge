# `ledger_masters_compliance_*_before_gst_details` — provenance

Two goldens of the `compliance` object that `ledger_masters` returns for `fields=compliance`, as it was built before `gst_details` existed (#1518). They are **not Tally captures**: they are the output of code, produced by the tree before that field was added, on captured inputs. No byte was written or edited by hand.

## Golden 1: `ledger_masters_compliance_rows_before_gst_details.json` (through the tool)

- **Produced by:** the tree at `7dd498fee424d95e8f536b728a1a4f6fb9779091`, the parent of #1518 (`90bcaa67`), in a detached checkout. A throwaway test, not committed, called the tool exactly as `compliance_rows_are_byte_for_byte_the_ones_read_before_gst_details` does (`fields=compliance`, the company GUID of the test, plans `compliance_plans(masters(), balances())`), took `row["compliance"]` of each item in order, and wrote `serde_json::to_string` of that array, with no trailing newline, to the file.
- **Inputs (captured, byte-exact, unchanged by this change):** `agent/native-party-masters.utf16le.xml` (sha256 `2b4440d91fe74a308d46e35419a622790fa9faae223fcea1f72e22a4cec91709`, documented by its own sidecar), `agent/native-party-balances.utf16le.xml` (`cb4c4e1bba8c13416e0acc6e1c85185998eb81fff736dac1b3e9f049fc859525`), `agent/native-party-groups.utf16le.xml` (`64e2886ff2a155ca70a209718be3de1117b0501b48cabe2032d18bfaa1af33e6`) and `agent/native-licensed-release-companies.utf16le.xml` (`998280fac5b6b40b23ee3879dcb35780bc439083a4d1a3a3fb9a64231adbaa50`), plus the extent and currency fixtures `compliance_plans` names.
- **What it holds:** 9 rows. Each is the master fields as serialised then. Every value is null, empty, `absent` or `not_observed` (the inputs hold no PAN, bank, contact or GST registration value for these ledgers), so no party name is marked. No row has a `gst_details` key.
- **What it establishes:** that the default compliance output of the tool is byte for byte what it was before #1518, for these 9 ledgers.
- **Not established:** a ledger with a party name, a registration history or any GST detail observed, other releases, the opt-in GST details read. The byte test therefore pins the shape and the missing key, not the content of populated values.
- **Regenerating:** do not, except by the same steps on the same commit; a regenerated file from a later tree would encode that tree and defend it.

| file | bytes | sha256 | content |
|---|---|---|---|
| `agent/ledger_masters_compliance_rows_before_gst_details.json` | 2692 | `db2205855c92fff9c25fa27d0105d7dcb547a877ef8de30258c0215d5a4ba664` | the 9 `compliance` objects, as an array, before #1518 (derived output, not a capture) |

## Golden 2: `ledger_masters_compliance_objects_registrations_before_gst_details.json` (populated, not through the tool)

- **Produced by:** the same tree, `7dd498fee424d95e8f536b728a1a4f6fb9779091`, in a detached checkout. A throwaway test, not committed, parsed the capture below with `parse_native_party_ledger_master_records_with_evidence` and the company GUID the committed test uses, built each record's object with the expression that tree's row used (`mark_compliance_party_names(serde_json::to_value(fields))`), and wrote `serde_json::to_string` of the array, with no trailing newline. The committed test builds the same array with `compliance_json(fields)`.
- **Why not through the tool:** the capture's ledgers carry a company GUID other than the one the tool test harness pins, and editing a capture's bytes would make it a hand-written fixture. So this golden pins the helper against the old row expression on the old parse, not a tool call. Golden 1 is the tool call.
- **Input (captured, byte-exact, unchanged by this change):** `agent/native-party-masters-gst-registrations.utf16le.xml` (sha256 `d95b63f4e0e215d8c26cf65febabdf58efc04fa351f6179a6c3ba581e6a86bc7`).
- **What it holds:** 6 objects. All have `tax_type` `Others`, `gst_duty_head` `not_tax_ledger` and a `gst_registrations` observation of `entries`; two of them hold 2 dated registration entries each. No object has a `gst_details` key. As in golden 1, a key whose value is `null` or an observation object is absent for `name_on_pan`, `bank_account_holder_name` and `bank_details`: `mark_party_field` removes the key and puts it back only for a string. That is existing behaviour; the goldens pin it and the helper keeps it.
- **Not established:** a marked party name (the capture holds no PAN, bank holder or bank detail), a ledger with GST rate details, other releases, the opt-in GST details read.

| file | bytes | sha256 | content |
|---|---|---|---|
| `agent/ledger_masters_compliance_objects_registrations_before_gst_details.json` | 2382 | `8b858764800e27aea30bb3aeee704dabd259be4933ef7fc56242cbcdb5516c56` | the 6 `compliance` objects, as an array, before #1518 (derived output, not a capture) |
