# The master mark of one synthetic book after each of 21 ledger Creates

Covers the 24 wire files below.

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `ledger-index-after-m1-shapes.utf16le.xml` | 26750 | `847eefb5779cd19e909ec4d592b8165c62ba0ecb8fcdba0262e9d9b61aed0b7b` |
| `ledger-index-after-probe-create.utf16le.xml` | 28064 | `8bc9b48b0c7ca0654b71f9db6e834b151a44c593b3868b0aac52f9e2571bdbdc` |
| `marks-00-baseline.utf16le.xml` | 3308 | `1d1d0d2a9c6c43e749624621578fd63ce4b884d5b430690555ce739718e5eea1` |
| `marks-01-n-0a.utf16le.xml` | 3308 | `4614d0f31e8651eaf1de59b9b72f1f945ba62403d560edaec69ea02a11252e0f` |
| `marks-02-n-0b.utf16le.xml` | 3308 | `ea7b5d57faeab124496342624a26a9be9e4151a80509ffe12fb48b941f97d083` |
| `marks-03-n-a.utf16le.xml` | 3308 | `ea7b5d57faeab124496342624a26a9be9e4151a80509ffe12fb48b941f97d083` |
| `marks-04-n-b.utf16le.xml` | 3308 | `e0ebd57615ea52e748c74a3f662bcb07b4c243bd8247da66fa9478a1e26c16f6` |
| `marks-05-n-c.utf16le.xml` | 3310 | `caaaccf44d2a36f1a1865ed1ebbeaf076f8937658330f9e1a22625898f7b1fe9` |
| `marks-06-n-d.utf16le.xml` | 3310 | `0e81363596c168fec435cf897adb9bbdaed8135d1799222fde000d1e93fec9ed` |
| `marks-07-n-f.utf16le.xml` | 3310 | `35ad1bf55e9909e92fd7f65971ed4244bd00689d0619d294fe39478f25723a34` |
| `marks-08-n-e.utf16le.xml` | 3310 | `adad350a4fda5d435c6b2c401042c135c848f7b40715e517b5f41b92977448a7` |
| `marks-09-m1-party.utf16le.xml` | 3308 | `47f0cc36aa37492c07ae536af2107ec2a6c852144c5febc774a871ab1151bcbb` |
| `marks-10-m1-susp.utf16le.xml` | 3308 | `01282f84d0b5a55ef5376ba41bd43efc5d0e1895f7173e28e85e8b8910058ee7` |
| `marks-11-m1-duty-cgst.utf16le.xml` | 3308 | `aac88a06614c5832aa7ea12be7ead9c39c4ff0c941f3ecd3e9f72978a42c9e51` |
| `marks-12-m1-duty-igst.utf16le.xml` | 3308 | `95853e8ce37c245bcd59093c4fd7eeea4085d73b52feba686935efccef963785` |
| `marks-13-m1-duty-state.utf16le.xml` | 3308 | `1bab57c9927dea5c6235fe0e55347b391095cb266ae2b381bb0930af1caef931` |
| `marks-14-m1-duty-sgstutgst.utf16le.xml` | 3308 | `207c8928d1baabc5963232245ed03834c5209a777c708ef9426f3470a1a2fe26` |
| `marks-15-m1-duty-ut.utf16le.xml` | 3308 | `ac4f475ec4fa5816d26c2c7cb322134dbbe41aafa8ca6c80c27a8da697216e3d` |
| `marks-16-m1-duty-cess.utf16le.xml` | 3308 | `2a199d020a365fd8d16362df47e46b61c56e39afab97a4b7d25504d703805b42` |
| `marks-17-q-0.utf16le.xml` | 3308 | `9891a16b28019a643ecf0e8fc67c6f89b741e26f98f52cfcda3e03f8aa78cd95` |
| `marks-18-q-r.utf16le.xml` | 3310 | `bb7fa3fd79cf35a15256ce4414d69063fccb1c235e2cb66cbfbdbd7f074aa78d` |
| `marks-19-q-a.utf16le.xml` | 3310 | `b5fd82d94d3cfa6aaf68de162b0426fc9e603451748231723d4cb0a7e96119c6` |
| `marks-20-q-c0.utf16le.xml` | 3310 | `c9ff9ecb12002d7a9ef11554d461634576763cadd242ea6d0967252e5f0a4595` |
| `marks-21-q-c.utf16le.xml` | 3310 | `10f6c8127f5373ebf3a88e8691230e15368993c66a18ddf735047ce66e3a10b9` |

Captured, not authored. Each file is the exact body Tally returned (UTF-16LE without a
byte-order mark), copied with no edit from branch `lab/masters-m1-answers` at `e783e7cb`, whose
`SHA256-evidence.txt` lists the same SHA-256 for the lab file and its published copy. The
`marks-NN-*` files are the company-marks read (`R-MARKS`): one `COMPANY` row with its `GUID` and
`ALTMSTID`, and no `ALTVCHID` (the company holds no voucher). `marks-02-n-0b` and `marks-03-n-a`
are byte-identical: the write between them was refused and moved nothing. The two
`ledger-index-*` files are the V3 ledger catalogue read (`R-V3`), the same request as
`../masters-lab.PROVENANCE.md`'s ledger-index answers; each carries Tally's own `&#4;` reference.
No file carries a GSTIN.

## What was read

- **Date and host:** 12 Oct 2026, about 14:16 to 15:02 UTC; TallyPrime 7.1, licensed Silver,
  education mode off; port 9001, one request at a time, sent with `curl` from a prepared request
  list, each once. Tally restarted between the first list (writes 1 to 8) and the second (9 to 16).
- **Book:** the synthetic company `BRIDGE MASTERS LAB` (GUID
  `f107e523-3d6b-4516-bd24-5b1a14498fa6`), keyed on Tally's own screens and loaded alone. No
  client data.
- **`marks-00-baseline`:** before any write (`ALTMSTID` 216).
- **`marks-NN-<step>`:** read straight after write NN, each a ledger `Create` (a gateway import of
  one `LEDGER` with `ACTION="Create"`), in the order sent:

| NN | Step | The `Create` and what Tally did | `ALTMSTID` after |
| ---: | --- | --- | ---: |
| 01 | N-0a | `MLab Fresh Party`, a new name: created | 217 |
| 02 | N-0b | `MLab Exact Target`, an existing ledger's exact name: that ledger altered | 218 |
| 03 | N-a | `MLab Alias Name`, another ledger's alias: refused, `Duplicate Entry!` | 218 |
| 04 | N-b | `MLAB CASE TARGET`: the existing ledger altered | 219 |
| 05 | N-c | `MLab  Space Target` (two spaces): the existing ledger altered | 220 |
| 06 | N-d | `MLab Dash Target`: `MLab Dash-Target` altered | 221 |
| 07 | N-f | `MLab Slash Target`: `MLab Slash/Target` altered | 222 |
| 08 | N-e | `MLab Group Target`, a group's name: a new ledger created | 223 |
| 09 | M1-PARTY | `MLab M1 Party`, Sundry Debtors, with a GSTIN: created | 224 |
| 10 | M1-SUSP | `MLab M1 Suspense`, Suspense A/c: created | 225 |
| 11 | M1-DUTY-CGST | `MLab M1 Duty CGST`, head CGST: created | 226 |
| 12 | M1-DUTY-IGST | `MLab M1 Duty IGST`, head IGST: created | 227 |
| 13 | M1-DUTY-STATE | `MLab M1 Duty State Tax`, head State Tax: created | 228 |
| 14 | M1-DUTY-SGSTUTGST | `MLab M1 Duty SGST UTGST`, head SGST/UTGST: created | 229 |
| 15 | M1-DUTY-UT | `MLab M1 Duty UT Tax`, head UT Tax: created | 230 |
| 16 | M1-DUTY-CESS | `MLab M1 Duty Cess`, head Cess: created | 231 |
| 17 | Q-0 | `MLab Exact Target` again (a control): altered | 232 |
| 18 | Q-r | `MLab Alias Target` with a `REMOTEID` no master holds: altered | 233 |
| 19 | Q-a | `MLab FreshParty`: `MLab Fresh Party` altered | 234 |
| 20 | Q-c0 | `MLab Probe Pvt. Ltd.`, a new name: created | 235 |
| 21 | Q-c | `MLab Probe Pvt Ltd`: `MLab Probe Pvt. Ltd.` altered | 236 |

- **`ledger-index-after-m1-shapes`:** read after write 16 (`ALTMSTID` 231), 18 ledgers.
- **`ledger-index-after-probe-create`:** read after write 20 (`ALTMSTID` 235), 19 ledgers.
- The ledger-index reads after writes 8 and 21 are `../masters-lab-ledger-index-after-name-cases.utf16le.xml`
  and `../masters-lab-ledger-index-after-fold-cases.utf16le.xml`.

## What it establishes

On this book, release and edition: each of the 20 writes that changed a ledger moved `ALTMSTID`
by exactly one, and the ledger it wrote (found by its `GUID`) then had an `ALTERID` equal to the
new `ALTMSTID`, for a create and an alteration alike; the refused write moved nothing. The test
`the_measured_marks_series_has_each_written_ledger_at_its_writes_mark` in
`src-tauri/src/agent_master_verdict_tests.rs` checks each of these from the files with ComplyEaze
Bridge's own parsers. Protocol reference §9.4j.

**Not established:** an answer lost on its way back; a create Tally ran after the client stopped
waiting; a master keyed on Tally's screen; a `Create` that changes nothing; groups and other
master types; two clients writing at once; Gold, Education and other releases.
