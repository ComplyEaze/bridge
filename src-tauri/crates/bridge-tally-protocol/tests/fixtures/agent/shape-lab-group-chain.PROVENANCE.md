# The ledger catalogue and the group snapshot of one synthetic book, read live (#1230)

Covers the four wire files and the trial-balance parents below.

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `shape-lab-standard-ledger-catalogue.request.utf16le.xml` | 1974 | `827f126cefc90cdaf4e15f41208bb5361458a87c3d8facfe3075bc6285c3dc28` |
| `shape-lab-standard-ledger-catalogue.utf16le.xml` | 51698 | `01699bbcd6b2ad7c0accefb5179b0436d845c5d41b1299d9734bf9096cab59e0` |
| `shape-lab-group-snapshot.request.utf16le.xml` | 1066 | `1a635c8b200ede795af39ad1abfa9ce3041c1341c561975bc623470d9db2dd0a` |
| `shape-lab-group-snapshot.utf16le.xml` | 62904 | `c635ac45139e6321c3f00a645e9913d177f68176ee385f86f4b82b03dfc670e7` |
| `shape-lab-fy.trial-balance-ledgers.json` | 6119 | `a93889938d458a53b9beaa29082f1bacd6785b14025514ed50c557d68c6b4635` |

Captured, not authored. The four `.xml` files are the exact bytes of two requests as the
shipped read code sent them and of the two answers Tally returned, body only (the request
files carry a byte-order mark; the answers are UTF-16LE without one, as Tally sends them).
The JSON file is the `ledger`, `parent`, `debit` and `credit` of each row of the trial balance that
the tool returned for the same year, keys sorted, no other edit.

## What was read

- **Date and host:** 2026-10-06, 18:09 to 18:13 IST; TallyPrime 7.1, licensed Silver,
  education mode off; port 9001 through a recording relay, one request at a time, read-only.
- **Book:** the synthetic company `BRIDGE SHAPE LAB` (44 ledgers, 33 groups, voucher mark
  111, master mark 289). No client data.
- **Build:** a debug build of `bridge_mcp` from origin/master 4c30f3f9f (not in a published
  build).
- **The catalogue:** the `List of Ledgers` collection with `NAME`, `GUID` and `PARENT` (the
  read a `vouchers` call with `ledger` makes; it was sent four times in that call, two paired
  reads, with identical answers). **The groups:** the `List of Groups` snapshot with `NAME`,
  `PARENT`, `GUID`, `MASTERID`, `ALTERID` and `RESERVEDNAME` (the `masters` read of kind
  `groups`; sent twice, paired, with identical answers).

## What it establishes

On this book: a ledger's immediate group is the `PARENT` of its catalogue row; user-created
groups answer an empty `RESERVEDNAME` and sit two and three levels deep (Trade Debtors - Local
under Sundry Debtors under Current Assets; Power and Fuel under Factory Overheads under
Indirect Expenses); one ledger (`Profit & Loss A/c`) has the reserved account root as its
parent; the trial balance's own `parent` column agrees with the catalogue's `PARENT` for
every ledger.

The group snapshot here is a re-capture of the book whose group snapshot is already committed as
`native-shape-lab-groups.utf16le.xml`: the two differ only in 15 header lines (the `CMPINFO`
counts, which are not about this book's groups). It is kept so that the request bytes, this answer
and the catalogue come from one run.

## What it does not establish

One synthetic book, one release, one build, one run. No renamed predefined group, no group
whose name repeats, no cycle, no ledger whose parent is missing from the group snapshot, and
nothing about a large book (the group snapshot here is about 63 KB; a large book's size is not
measured). The raw responses of the whole run are retained privately by the maintainer and are
not part of this repository.
