# The ledger name index and the group snapshot of one synthetic book, before and after ledger Creates on near names

Covers the six wire files below.

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `masters-lab-ledger-index.request.utf16le.xml` | 2232 | `b6c6e6b4aa1c90bdc41f498e5389c6e9f9f34f65ab21ee96f1cecdc18d7d773f` |
| `masters-lab-ledger-index-baseline.utf16le.xml` | 13610 | `268040ba3927e31222aefead1e49dbef744bf60417918362c18a9e08d1e5c6eb` |
| `masters-lab-ledger-index-after-name-cases.utf16le.xml` | 16256 | `f875beee3ec7f3f1b94c0da7af075d682d560668c38454996b774ba6774dd853` |
| `masters-lab-ledger-index-after-fold-cases.utf16le.xml` | 28054 | `6ee39a99c4855fb3944391b7fc50a3ff8fc37a5a25bb6c398a0decb3a72736a8` |
| `masters-lab-groups.request.utf16le.xml` | 1070 | `d79f32c5c0519ce7dacb46839f47f15a97ef9f11180121e2b3fa57e6c9d347b0` |
| `masters-lab-groups-baseline.utf16le.xml` | 56068 | `7131aa4566ea5d630fa4506af5165319d261abe087e3a1b614b62080d2c4e3aa` |

Captured, not authored. The two request files are the exact bytes sent (with a byte-order
mark); `tests/ledger_name_index.rs` checks that they equal what ComplyEaze Bridge renders for
this company (the V3 ledger catalogue and the group snapshot). The four answers are the exact
bodies Tally returned (UTF-16LE without a byte-order mark), with no edit: they carry Tally's own
`&#4;` references (one in each ledger-index answer, 30 in the group answer), the group answer also
88 raw U+0003 characters that Tally writes unescaped, and no GSTIN.

## What was read

- **Date and host:** 12 Oct 2026, about 14:17 to 14:27 UTC; TallyPrime 7.1, licensed Silver,
  education mode off; port 9001, one request at a time, sent with `curl` from a prepared
  request list, each once.
- **Book:** the synthetic company `BRIDGE MASTERS LAB` (GUID
  `f107e523-3d6b-4516-bd24-5b1a14498fa6`), keyed on Tally's own screens and loaded alone. No
  client data.
- **Baseline answers:** the first ledger-index read and the first group read, before any write
  (8 ledgers; master mark 216).
- **After the name cases:** the ledger-index read after eight ledger Creates, each sent once,
  each with parent Sundry Creditors and bill-wise Yes, against ledgers keyed with parent Sundry
  Debtors and bill-wise No (master mark 223, 10 ledgers):
  - a new name: created;
  - an existing ledger's exact name: that ledger altered;
  - an existing ledger's alias: refused, `Duplicate Entry!`, nothing changed;
  - an existing name in capitals, with a doubled space, with a space for its `-`, and with a
    space for its `/`: each altered the existing ledger (same GUID, parent and bill-wise
    overwritten); its `NAME.LIST` now holds the incoming spelling while the row's `NAME`
    attribute keeps the old one;
  - a group's name: a new ledger was created and the group was unchanged.

- **After the fold cases:** a later session the same day (about 15:00 to 15:02 UTC), after
  Tally had restarted and eight more ledgers had been created under other names. Its ledger-index
  read after five more Creates, each sent once (master mark 236, 19 ledgers):
  - an existing ledger's exact name (a control): that ledger altered;
  - an existing ledger's exact name carrying a `REMOTEID` no master holds: that ledger altered;
  - `MLab FreshParty` against `MLab Fresh Party`: the existing ledger altered;
  - `MLab Probe Pvt. Ltd.`, new: created (a control);
  - `MLab Probe Pvt Ltd` against it: the existing ledger altered.

  Its rows for the ledgers the first session renamed carry the new spelling in the `NAME` attribute too;
  the two ledgers renamed in this session carry the old one. Three reads of that session's
  evidence labelled as taken before its first write were taken after its second (the request
  list worded it so); this file does not depend on them.

## What it establishes

On this book and release: the V3 answer carries an `ALTERID` on every ledger row; aliases follow
the stored name in the one `NAME.LIST`, in the order keyed; and a ledger Create Tally resolves to
an existing ledger by a case, doubled-space, `-` or `/` variant, a removed space or removed full
stops rewrites that ledger's stored name and leaves the row's `NAME` attribute at the old spelling
until Tally restarts; and a `REMOTEID` no master holds does not stop that match.

**Not established:** whether the `REMOTEID` was stored (the read does not fetch it); other
symbols than those named; the same for groups; other releases
and editions; a book with more than one language list.
