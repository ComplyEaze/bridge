# `opening-stock-tieout-e/`: opening stock that Tally carries from a prior year (#692)

Forty-nine answers from the five runs of 5 Nov 2026 (R1 to R5) on one synthetic company, read for
#692's option B. They are here for `tests/opening_stock_tieout_e.rs`, which reproduces the run's
verdicts through this crate's own parsers, and for the parse records in
`tests/captured_parse_record.rs`.

## Provenance

- **Host / gateway:** TallyPrime 7.2 Silver, one `tally.exe` owning port 9001 (a lab instance).
- **Date:** 2026-11-05.
- **Book:** `BRIDGE TIEOUT E`, a **synthetic** company created on Tally's screen for this run, with
  books from 1 Apr 2024, inventory integrated with accounts, no batches and no GST. Its masters and
  seven vouchers were keyed on screen with invented values. Its company identifier (it begins `ffa928f6`) is
  in `scripts/merge-gate-synthetic-uuids.json`. The book holds no account number, GSTIN, PAN,
  address, e-mail or phone.
- **Requests:** the lab sheet's own, one per file kind. The ledger read is the request
  `render_native_trial_balance_request` writes for the window. The Balance Sheet, Profit and Loss,
  Stock Summary and negative-stock reads are built-in reports by name over the window; the groups
  read is `List of Groups` with `RESERVEDNAME` in its fetch list.
- **Windows:** w1 is 2024-25, w2 is 2025-26, w3 is Apr to Sep 2025, w4 is Oct 2025 to Mar 2026,
  and w5 is 1 Apr 2024 to 31 Mar 2026. R1 read w1 to w4 on the book's backed-up state, R5 read w5
  straight after. Each of R2, R3 and R4 changed one thing, read, and restored the backup: R2 the
  costing method of one item, R3 a sale of an item with no purchase (negative stock), and R4 one
  direct expense ledger and a payment to it. Each `rN-after-w2-pl` is the Profit and Loss read
  after that restore, and equals R1's `r1-w2-pl` byte for byte.
- **Bytes:** each answer is stored as received: UTF-16LE without a byte-order mark, served as
  `text/xml; charset=utf-16`, with its size equal to the response's `CONTENT-LENGTH`. Each equals its
  line in `scripts/merge-gate-lab-captures.json` and in the lab's answers record. No answer was
  edited.

## What the tests establish, and what they do not

- On every one of the nine reads with ledgers, the Opening Stock and Less: Closing Stock lines are
  each present once, and both are debits or zero. The Balance Sheet export balances. Identity 1
  (Tally's Current Assets less the ledgers' Current Assets equals the closing stock line),
  identity 2 (the carried Profit & Loss line's gap from the ledgers equals opening less closing)
  and identity 3 (the ledgers' closings sum to the negated opening stock line) all hold.
- Relation 4 (the Cost of Sales heading equals opening plus purchases plus the direct expense
  lines' sub amounts, less closing) holds on every read but R1 w4, where the heading has no amount.
  Only R4's read has a direct expense line.
- It is one synthetic book with a simple structure, on one release. It does not establish the
  identities for a book with several direct expenses, godowns, batches or GST, nor for another release.
  ComplyEaze Bridge's statements still refuse a book with opening stock; these fixtures do not
  change that.

| file | bytes | sha256 | content |
|---|---|---|---|
| `r1-groups.utf16le.xml` | 54286 | `229b04263df16d8438ba21af7797e2445d460ffd555c51cc2b5a35f974736095` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w1-bs.utf16le.xml` | 1852 | `f606fadec64ea694a39c847df7431dcb3ea43bc4491ce65652263cdf93bf1b1f` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w1-ledgers.utf16le.xml` | 13526 | `07e9f385637f9e2cf50dd0faf799207b2ec5de22a6b1da19ccb8cd9ddcec8869` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w1-negative-stock.utf16le.xml` | 46 | `8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w1-pl.utf16le.xml` | 1922 | `c5f34dcb0b4135af46b72f01372f063a3906db30405ae271d92696d92273efa8` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w1-stock-summary.utf16le.xml` | 966 | `5ca75217dddff3e8bcf0f5a8fc50c993a28d768ab126d5d6941d83a8a634bb3c` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w2-bs.utf16le.xml` | 1854 | `f188c7ba890f76637559ee66d5a27e9017de588d8a108e70f91f728e6b224237` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w2-ledgers.utf16le.xml` | 13548 | `c5bd41fd0816b3dd75d3f003ee4d7900a1eba27722dd91b786034807a6fa4286` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w2-negative-stock.utf16le.xml` | 46 | `8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w2-pl.utf16le.xml` | 1938 | `2beec565946297d03602d970a6e40632de6d62b9e7e6c95646fafc8f3ea72d93` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w2-stock-summary.utf16le.xml` | 962 | `d981168c6310cb3e3db42f90f54e7ecb6d3037e1974e463d7b1b9f69ab539b6a` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w3-bs.utf16le.xml` | 1854 | `d8cba5d13e1f23f6b58f54ef4bf0dea24535c73637c7e62ee34d9a3ad3543b24` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w3-ledgers.utf16le.xml` | 13508 | `2a2878575ad318b157e3b335ffb1ae304d9e505e65a980e94503d0ba9eb9778b` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w3-negative-stock.utf16le.xml` | 46 | `8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w3-pl.utf16le.xml` | 1924 | `191f80cb0d8fb94db56f8069a4616749875f86ad9f5aad7a743634a1bd9d3051` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w3-stock-summary.utf16le.xml` | 962 | `d981168c6310cb3e3db42f90f54e7ecb6d3037e1974e463d7b1b9f69ab539b6a` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w4-bs.utf16le.xml` | 1854 | `f188c7ba890f76637559ee66d5a27e9017de588d8a108e70f91f728e6b224237` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w4-ledgers.utf16le.xml` | 13462 | `666daa320f1c4a3502ccca68f731d095cb09fc1a343b0325abda2cd602a81ca0` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w4-negative-stock.utf16le.xml` | 46 | `8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w4-pl.utf16le.xml` | 1892 | `b565da7c53341cd86eb558a5f262dc0f925e1110ebdfa77f3c93a360e0d9202e` | answer as received, UTF-16LE without a byte-order mark |
| `r1-w4-stock-summary.utf16le.xml` | 962 | `d981168c6310cb3e3db42f90f54e7ecb6d3037e1974e463d7b1b9f69ab539b6a` | answer as received, UTF-16LE without a byte-order mark |
| `r2-after-w2-pl.utf16le.xml` | 1938 | `2beec565946297d03602d970a6e40632de6d62b9e7e6c95646fafc8f3ea72d93` | answer as received, UTF-16LE without a byte-order mark |
| `r2-w1-bs.utf16le.xml` | 1852 | `48377d79b6be4844ec038702f5040857240910f6e3a2b1b6ce8fe5e955313999` | answer as received, UTF-16LE without a byte-order mark |
| `r2-w1-ledgers.utf16le.xml` | 13526 | `8437286da7ab2e27ff2b40677bc96b87bd5b9eb3ec52da36c05e0e6ff5c1bd16` | answer as received, UTF-16LE without a byte-order mark |
| `r2-w1-negative-stock.utf16le.xml` | 46 | `8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3` | answer as received, UTF-16LE without a byte-order mark |
| `r2-w1-pl.utf16le.xml` | 1922 | `feebe9af203f43d4841faf31458d73babd3db5e78ba753370cb2924cdbf22ab8` | answer as received, UTF-16LE without a byte-order mark |
| `r2-w1-stock-summary.utf16le.xml` | 966 | `96936b3d493d661a918f35d39511256da79889efa2d5e4775ce177a0ea17eb14` | answer as received, UTF-16LE without a byte-order mark |
| `r2-w2-bs.utf16le.xml` | 1854 | `ff4ccf92074ba2a08e1c5dbd8502dc9f61c5c2fd7e4121912115770aa9b6d616` | answer as received, UTF-16LE without a byte-order mark |
| `r2-w2-ledgers.utf16le.xml` | 13548 | `f08f2ae27f89f29fb9df84f94a1184d6b4db3000c663da490c0a2d4bea0b75d8` | answer as received, UTF-16LE without a byte-order mark |
| `r2-w2-negative-stock.utf16le.xml` | 46 | `8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3` | answer as received, UTF-16LE without a byte-order mark |
| `r2-w2-pl.utf16le.xml` | 1938 | `41998a838d31f8ed8ed9c72279a8d2cbece0a7237e7cc3e56e17c0c8937c5e19` | answer as received, UTF-16LE without a byte-order mark |
| `r2-w2-stock-summary.utf16le.xml` | 962 | `fedd896aaf2201e9071dfca8701341f10322e89022aa09485cf24af7d55fa9f8` | answer as received, UTF-16LE without a byte-order mark |
| `r3-after-w2-pl.utf16le.xml` | 1938 | `2beec565946297d03602d970a6e40632de6d62b9e7e6c95646fafc8f3ea72d93` | answer as received, UTF-16LE without a byte-order mark |
| `r3-w2-bs.utf16le.xml` | 1854 | `8ede498a8f71dc42ff3c136849499f13137d5f9d4db38435253b07e81b11af5d` | answer as received, UTF-16LE without a byte-order mark |
| `r3-w2-ledgers.utf16le.xml` | 13542 | `6bca6a373285605d1a74a470742b23b9c3038b9f085e053b09bdebb7ff525933` | answer as received, UTF-16LE without a byte-order mark |
| `r3-w2-negative-stock.utf16le.xml` | 482 | `6c318484ea3a0375d6a6ec11e5c2ee9c5b89a99e1b637d5a4abd49336e7aa2d1` | answer as received, UTF-16LE without a byte-order mark |
| `r3-w2-pl.utf16le.xml` | 1938 | `70f9d3761df6d64cfcd792017131ba3727c9c1075e8f5980d3979c9aaf4b984c` | answer as received, UTF-16LE without a byte-order mark |
| `r3-w2-stock-summary.utf16le.xml` | 1394 | `d3aa11ff3dd19c66dd25d5af60bc5c4e89f7340a2b074d4f858f29c7176cb5c6` | answer as received, UTF-16LE without a byte-order mark |
| `r4-after-w2-pl.utf16le.xml` | 1938 | `2beec565946297d03602d970a6e40632de6d62b9e7e6c95646fafc8f3ea72d93` | answer as received, UTF-16LE without a byte-order mark |
| `r4-w2-bs.utf16le.xml` | 1854 | `be5246c94ac60f6a0dda42714ec78527c8e5358dc612fec04977c98c00d67e8e` | answer as received, UTF-16LE without a byte-order mark |
| `r4-w2-ledgers.utf16le.xml` | 14724 | `d59c98c7437eab71908fdc4b0031af34da4398ac47695cb58b4c46e72fb6e3d3` | answer as received, UTF-16LE without a byte-order mark |
| `r4-w2-negative-stock.utf16le.xml` | 46 | `8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3` | answer as received, UTF-16LE without a byte-order mark |
| `r4-w2-pl.utf16le.xml` | 2248 | `fef509e5d976c192d23bb1fa68fad2f0179dcdf5e6a70f0be7e957a9cd30f5da` | answer as received, UTF-16LE without a byte-order mark |
| `r4-w2-stock-summary.utf16le.xml` | 962 | `d981168c6310cb3e3db42f90f54e7ecb6d3037e1974e463d7b1b9f69ab539b6a` | answer as received, UTF-16LE without a byte-order mark |
| `r5-w5-bs.utf16le.xml` | 1854 | `f188c7ba890f76637559ee66d5a27e9017de588d8a108e70f91f728e6b224237` | answer as received, UTF-16LE without a byte-order mark |
| `r5-w5-ledgers.utf16le.xml` | 13532 | `b2d0519cc1d25b5eac8a0ddc15318d58818fa9aaba97e4bf3ac81a87785f533f` | answer as received, UTF-16LE without a byte-order mark |
| `r5-w5-negative-stock.utf16le.xml` | 46 | `8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3` | answer as received, UTF-16LE without a byte-order mark |
| `r5-w5-pl.utf16le.xml` | 1924 | `ef0cef45cfc1f0476cc210420a1b54b340382e3a2ff65bb5cb14323827aac8fd` | answer as received, UTF-16LE without a byte-order mark |
| `r5-w5-stock-summary.utf16le.xml` | 962 | `d981168c6310cb3e3db42f90f54e7ecb6d3037e1974e463d7b1b9f69ab539b6a` | answer as received, UTF-16LE without a byte-order mark |
