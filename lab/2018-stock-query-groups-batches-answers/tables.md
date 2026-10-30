# Parser tables (#2018 L1-L3). Item names appear here, in the raw answers and in the tool answers only.

## Stock Query answers
| request | item, to | bytes | byte-identical to 28 Oct | purchase groups (dates) | sales groups (dates) | godown, batch, qty triples |
|---|---|---|---|---|---|---|
| q-r-09-stock-query-soda-ash-light | Soda Ash Light, 20260331 | 3152 | yes | 5 (15-Apr-25, 14-Apr-25, 13-Apr-25, 12-Apr-25, 11-Apr-25) | 0 () | 'Main Location', 'Primary Batch', '1350.000 Kgs'; 'Factory Floor', 'Primary Batch', '-50.000 Kgs' |
| q-r-03-stock-query-cleaning-kit-a | Cleaning Kit A, 20260331 | 1086 | yes | 0 () | 1 (23-Apr-25) | 'Main Location', 'Primary Batch', '25 Nos'; 'Main Location', 'K-2504-A', '-13 Nos' |
| q-r-02-stock-query-caustic-soda-flakes | Caustic Soda Flakes, 20260331 | 1094 | yes | 1 (21-Apr-25) | 0 () | 'Main Location', 'Primary Batch', '200.000 Kgs'; 'Main Location', 'B-2504-A', '200.000 Kgs' |
| q-r-4-stock-query-stockitemname | Hydrochloric Acid 33pc, 20260331 | 1354 | yes | 1 (24-Apr-25) | 0 () | 'Main Location', 'Primary Batch', '150.000 Kgs'; 'Main Location', 'H-2504-A', '300.000 Kgs'; 'Factory Floor', 'H-2504-B', '366.667 Kgs' |
| q-r-12-stock-query-hcl-to-20250401 | Hydrochloric Acid 33pc, 20250401 | 320 | yes | 0 () | 0 () | 'Main Location', 'Primary Batch', '150.000 Kgs' |
| q-r-14-stock-query-cleaning-kit-a-to-20250401 | Cleaning Kit A, 20250401 | 310 | yes | 0 () | 0 () | 'Main Location', 'Primary Batch', '25 Nos' |
| q-r-13-stock-query-zero-stock-item-to-20250401 | Zero Stock Item, 20250401 | 46 | yes | 0 () | 0 () | empty answer |
| q-r-04-stock-query-cleaning-kit-b | Cleaning Kit B, 20260331 | 46 | yes | 0 () | 0 () | empty answer |

## The stock-item collection: CLOSINGBALANCE as sent
| item | to | CLOSINGBALANCE |
|---|---|---|
| Caustic Soda Flakes | 20250401 | 200.000 Kgs |
| Caustic Soda Flakes | 20260331 | 400.000 Kgs |
| Cleaning Kit A | 20250401 | 25 Nos |
| Cleaning Kit A | 20260331 | 12 Nos |
| Cleaning Kit B | 20250401 | empty |
| Cleaning Kit B | 20260331 | empty |
| Hydrochloric Acid 33pc | 20250401 | 150.000 Kgs |
| Hydrochloric Acid 33pc | 20260331 | 816.667 Kgs |
| Soda Ash Light | 20250401 | 500.000 Kgs |
| Soda Ash Light | 20260331 | 1300.000 Kgs |
| Zero Stock Item | 20250401 | 0.000 Kgs |
| Zero Stock Item | 20260331 | -50.000 Kgs |

## The year's vouchers (one page; total 67, returned 67, state complete, reason None, withheld None)
| date | type | number | cancelled | optional | goods lines (stock_item, actual_quantity, batches as godown/batch/qty) |
|---|---|---|---|---|---|
| 20250401 | Purchase | 10 | False | False | 'Soda Ash Light' '100.000 Kgs' 'Main Location'/'Primary Batch'/'100.000 Kgs' |
| 20250401 | Purchase | 11 | False | False | 'Soda Ash Light' '100.000 Kgs' 'Main Location'/'Primary Batch'/'100.000 Kgs' |
| 20250403 | Purchase | 12 | False | False | 'Soda Ash Light' '100.000 Kgs' 'Main Location'/'Primary Batch'/'100.000 Kgs' |
| 20250404 | Purchase | 13 | False | False | 'Soda Ash Light' '100.000 Kgs' 'Main Location'/'Primary Batch'/'100.000 Kgs' |
| 20250405 | Purchase | 14 | False | False | 'Soda Ash Light' '100.000 Kgs' 'Main Location'/'Primary Batch'/'100.000 Kgs' |
| 20250411 | Purchase | 15 | False | False | 'Soda Ash Light' '100.000 Kgs' 'Main Location'/'Primary Batch'/'100.000 Kgs' |
| 20250412 | Purchase | 16 | False | False | 'Soda Ash Light' '100.000 Kgs' 'Main Location'/'Primary Batch'/'100.000 Kgs' |
| 20250413 | Purchase | 17 | False | False | 'Soda Ash Light' '100.000 Kgs' 'Main Location'/'Primary Batch'/'100.000 Kgs' |
| 20250414 | Purchase | 18 | False | False | 'Soda Ash Light' '100.000 Kgs' 'Main Location'/'Primary Batch'/'100.000 Kgs' |
| 20250415 | Purchase | 19 | False | False | 'Soda Ash Light' '100.000 Kgs' 'Main Location'/'Primary Batch'/'100.000 Kgs' |
| 20250420 | Sales | 2 | False | False | 'Zero Stock Item' '50.000 Kgs' 'Main Location'/'Primary Batch'/'50.000 Kgs' |
| 20250421 | Purchase | 20 | False | False | 'Caustic Soda Flakes' '200.000 Kgs' 'Main Location'/'B-2504-A'/'200.000 Kgs' |
| 20250423 | Sales | 3 | False | False | 'Cleaning Kit A' '13 Nos' 'Main Location'/'K-2504-A'/'13 Nos' |
| 20250424 | Purchase | 21 | False | False | 'Hydrochloric Acid 33pc' '666.667 Kgs' 'Main Location'/'H-2504-A'/'300.000 Kgs', 'Factory Floor'/'H-2504-B'/'366.667 Kgs' |
| 20250703 | Purchase | 9 | True | False |  |
| 20250709 | Sales | 1 | False | False |  |

## L1
Purchase goods lines for Soda Ash Light (not cancelled, not optional): 10 (20250401, 20250401, 20250403, 20250404, 20250405, 20250411, 20250412, 20250413, 20250414, 20250415); q-r-09 purchase groups: 5 (15-Apr-25, 14-Apr-25, 13-Apr-25, 12-Apr-25, 11-Apr-25)

## L2
| item | (godown, batch) pairs in the goods lines, signed sum | opening lines (1 Apr 2025) | year-end lines (31 Mar 2026) | opening + sum = year end, pair by pair | year-end lines in no voucher and no opening |
|---|---|---|---|---|---|
| Soda Ash Light | 1: ('Main Location', 'Primary Batch'): 1000.000 | not read | ('Main Location', 'Primary Batch'): 1350.000; ('Factory Floor', 'Primary Batch'): -50.000 | - | [('Factory Floor', 'Primary Batch')] |
| Cleaning Kit A | 1: ('Main Location', 'K-2504-A'): -13 | ('Main Location', 'Primary Batch'): 25 | ('Main Location', 'Primary Batch'): 25; ('Main Location', 'K-2504-A'): -13 | ('Main Location', 'K-2504-A'): 0 + -13 = -13 vs -13 equal; ('Main Location', 'Primary Batch'): 25 + 0 = 25 vs 25 equal | none |
| Caustic Soda Flakes | 1: ('Main Location', 'B-2504-A'): 200.000 | not read | ('Main Location', 'Primary Batch'): 200.000; ('Main Location', 'B-2504-A'): 200.000 | - | [('Main Location', 'Primary Batch')] |
| Hydrochloric Acid 33pc | 2: ('Factory Floor', 'H-2504-B'): 366.667; ('Main Location', 'H-2504-A'): 300.000 | ('Main Location', 'Primary Batch'): 150.000 | ('Main Location', 'Primary Batch'): 150.000; ('Main Location', 'H-2504-A'): 300.000; ('Factory Floor', 'H-2504-B'): 366.667 | ('Factory Floor', 'H-2504-B'): 0 + 366.667 = 366.667 vs 366.667 equal; ('Main Location', 'H-2504-A'): 0 + 300.000 = 300.000 vs 300.000 equal; ('Main Location', 'Primary Batch'): 150.000 + 0 = 150.000 vs 150.000 equal | none |

## L3
| item | date | Stock Query | CLOSINGBALANCE | reading |
|---|---|---|---|---|
| Soda Ash Light | 20260331 | Main Location/Primary Batch/1350.000 Kgs; Factory Floor/Primary Batch/-50.000 Kgs | 1300.000 Kgs | lines sum 1300.000 vs closing 1300.000: equal |
| Cleaning Kit A | 20260331 | Main Location/Primary Batch/25 Nos; Main Location/K-2504-A/-13 Nos | 12 Nos | lines sum 12 vs closing 12: equal |
| Caustic Soda Flakes | 20260331 | Main Location/Primary Batch/200.000 Kgs; Main Location/B-2504-A/200.000 Kgs | 400.000 Kgs | lines sum 400.000 vs closing 400.000: equal |
| Hydrochloric Acid 33pc | 20260331 | Main Location/Primary Batch/150.000 Kgs; Main Location/H-2504-A/300.000 Kgs; Factory Floor/H-2504-B/366.667 Kgs | 816.667 Kgs | lines sum 816.667 vs closing 816.667: equal |
| Hydrochloric Acid 33pc | 20250401 | Main Location/Primary Batch/150.000 Kgs | 150.000 Kgs | lines sum 150.000 vs closing 150.000: equal |
| Cleaning Kit A | 20250401 | Main Location/Primary Batch/25 Nos | 25 Nos | lines sum 25 vs closing 25: equal |
| Zero Stock Item | 20250401 | empty | 0.000 Kgs | zero-stock case |
| Cleaning Kit B | 20260331 | empty | empty | no stock movement (closing empty) |
