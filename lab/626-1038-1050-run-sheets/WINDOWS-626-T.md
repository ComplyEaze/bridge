# Windows item 626-T v1.2: which ledger a voucher lands on when names differ only by a trailing line break, by case, or are reached through an alias (13 Oct 2026; for issue #626, the four measurements of its 26 Sep comment). Plain HTTP plus Tally's own screens: NO ComplyEaze Bridge build, no Terms entry. SYNTHETIC ONLY: THE COMPANY = BRIDGE LINE BREAK LAB (number 100042, GUID 224e7f8c-a0aa-45ce-889a-ad4b7fdbe852). LAB-COMMON v3 applies in full; the STATE READS are MARKS, LEDGERS and VCH. Writes: three ledger Creates and up to four one-voucher imports by request, each file sent ONCE; one alias added and two ledgers attempted on Tally's screen. The book is restored to its start at the end. Needs the run coordinator's go-ahead naming the seven write hashes.

Why: ComplyEaze Bridge refuses a ledger that has a "twin" (a second ledger whose name is equal after folding case, spaces and line breaks). Before that refusal can narrow, four things about Tally itself must be known. Each step changes one thing and RECORDS what Tally does; where a line says RECORD, each listed outcome is a result and none is a STOP.

## Requests (each ONE line written as UTF-16LE with a BOM, no line break at the end, unless said)
- MARKS: 986 bytes, sha256 84e23f4d992dfec24383da4df9f0659ab6df51be290d602c5b4c2da102a98ce2 (UTF-8: 492 bytes, 966c6db39665f435d58c059b766da9d1d36537cbb37c498c74d59543cf968c94)
  File on this branch: requests/626-T/marks.utf16le.xml
- LEDGERS: lab/masters-m1-requests at 47aed0a8f149476b803378ef631d529a199d8b42, file R-V3.utf16le.xml (2,232 bytes, sha256 b6c6e6b4aa1c90bdc41f498e5389c6e9f9f34f65ab21ee96f1cecdc18d7d773f), with ONLY the text inside SVCURRENTCOMPANY changed to BRIDGE LINE BREAK LAB: the file sent must be 2,238 bytes, sha256 2142f3884a544810da9dcf4a61b2a26b8842066f9d125a23058117b19d4f190d (ready-made on this branch: requests/626-T/ledgers.utf16le.xml). Its list: one line per LEDGER row: the row's NAME attribute, every NAME element under it (the first is its own name, later ones are aliases), GUID, PARENT, ALTERID. Count the rows.
- VCH (every voucher of 1 to 31 Aug 2026; the 12 Oct window read with the company and dates changed): 1,786 bytes, sha256 cc06efc816c30768234bb104b1c8d806117f163f34ebac34a3f8033606060e62 (UTF-8: 892 bytes, dd412e02e584d94c6f597c524139264aa292fd221728accad7206c4e304eb309)
  File on this branch: requests/626-T/vch-aug.utf16le.xml
- Three ledger Creates (the pinned N-0a of lab/masters-m1-requests, one ledger each, without its bill-wise element, which these expense and income ledgers do not take):
  - LED1, "Lab Line Break Ledger" with NO line break, under Indirect Incomes: 992 bytes, sha256 1f32ec9ecafa9f745e1a055c45cc542b5b6f4ca7c8218c1d7b8eb745c48bd7b0 (UTF-8: 495 bytes, 806e3ec1329a1f9a0bb58ef9aba8c6d2252eac4bacd9fc4e50819e5bbc449c22)
    File on this branch: requests/626-T/led1.utf16le.xml
  - LED2, "LAB CASE PROBE" under Indirect Expenses: 966 bytes, sha256 c189d4b290c8eeb5b9c5f7d54d7e023767473a08344cc0e93ef61e9774540020 (UTF-8: 482 bytes, d6176421a9989b4f37f0224816213909ffa72e2efa2ea086044ef42f65a6adec)
    File on this branch: requests/626-T/led2.utf16le.xml
  - LED3, "lab case probe" under Indirect Incomes: 964 bytes, sha256 7823f514fe4d38d08e62ae628885a7d1a44e86d5773c5ce1110244e47613e1cd (UTF-8: 481 bytes, d76a69aa8073766f4a16dbc595927852099e9c47988647a7d71f082730236c2f)
    File on this branch: requests/626-T/led3.utf16le.xml
- Four one-voucher imports (the builder's own Payment shape, as the first voucher of the committed fixture post-span-import.xml; no [BRIDGE:...] tag, no number, a new REMOTEID): each a Payment, Dr the name under test / Cr Cash.
  - IMP0, the CONTROL: 4 Aug 2026, 610.00, Dr "Lab Plain Ledger" (an exact, twin-less name): 1,998 bytes, sha256 dc5fa7616937490ad711ccd7cb155d4d8fed76a564df72760bdf77450afb15cd (UTF-8: 998 bytes, b49c917b969041960e3326d4990ca581fd374b3139998bf06b6f9d1aa70c67e9)
    File on this branch: requests/626-T/imp0.utf16le.xml
  - IMP1, 5 Aug 2026, 611.00, Dr "LAB LINE BREAK LEDGER" (capitals, no line break: equal to neither twin, folds to both): 2,030 bytes, sha256 7021089bf312291f60754eb7848923e8b78e1eded917358cfd5df3566eaf30d1 (UTF-8: 1,014 bytes, a2a7e2d4e30964549af2b351bd59b5193c2afac8384894f543d9cf0c482b0169)
    File on this branch: requests/626-T/imp1.utf16le.xml
  - IMP2, 6 Aug 2026, 612.00, Dr "lab plain ledger" (small letters: equal to neither case twin): 2,004 bytes, sha256 935164512c3de84699da5a1c19fff4f8560ed904f71970e910b8c2690ddfc918 (UTF-8: 1,001 bytes, 5e0bf46c8cda3488465e371cd1b67965a28fe6c897aa679ac471a30f85eafd28)
    File on this branch: requests/626-T/imp2.utf16le.xml
  - IMP3, 7 Aug 2026, 613.00, Dr "Lab Plain Alias": 1,990 bytes, sha256 0767ac62d654bf414cfc362992cf16f9bbeb487233e41fa197830e8aeb42b508 (UTF-8: 994 bytes, b5979c444637e0ee755f902159ed65228da92aa0cb380d32937b2edea0abbc76)
    File on this branch: requests/626-T/imp3.utf16le.xml

## Rules for this item
- SIZE BOUNDS: a MARKS answer 20,000 bytes; a LEDGERS answer 500,000 bytes; a VCH answer 400,000 bytes (about 32 KB per voucher, six at most). Above its bound = STOP before it is used.
- MARKS are RECORDED at every step, never required: the STOPs of this item are on the LEDGERS list and the VCH table.
- WHICH LEDGER A VOUCHER DEBITS is read so: take the debit entry's LEDGERNAME from VCH exactly as it reads (line break included) and find the LEDGER rows of the LEDGERS list read in the same step whose NAME attribute equals it exactly. Exactly one such row = that ledger (give its GUID). None, or more than one = write "not told apart by these reads"; do not guess from the spelling.
- AN IMPORT THAT ANSWERS WITH AN ERROR is a result only if CREATED is 0, ERRORS is 1 and its LINEERROR names the debit name under test; then VCH must show no voucher of that date. Any other error (one naming Cash, the type or the date) = STOP.
- ON A LEDGER SCREEN (t6, t8, t10): a screenshot must show the title "Ledger Creation" or "Ledger Alteration" before any Ctrl+Q; Ctrl+Q is never pressed at the Gateway. After an accept in Ledger Creation Tally shows a fresh blank creation form: screenshot it, leave it with Ctrl+Q, and screenshot the Gateway before any request. A message at the Name field (for example a duplicate-name message) IS the result of that step: screenshot it, copy its text exactly, press no key, report and wait for the run coordinator's word on leaving the screen.

## Steps (LAB-COMMON start and step 0 first; every step ends with MARKS, LEDGERS and its list, VCH and its table)
- EXPECT AT EVERY STEP (a miss is a STOP): the three reads parse; no ledger and no voucher of the t0 lists has gone; no ledger appears that the step did not make; the August vouchers are the t0 ones plus this item's imports, each with the date, amount and narration of its file.
- t0 (as found): MARKS. LEDGERS: EXPECT the list to hold "Lab Line Break Ledger" + line break (21 characters, 23 with the break), "Lab Plain Ledger" and "Cash"; no other ledger whose name, in small letters and without spaces or line breaks, equals that of "lab line break ledger", "lab plain ledger", "lab case probe", "lab screen break" or "lab plain alias"; no alias on "Lab Plain Ledger". Record every ledger with its GUID and parent. VCH: EXPECT 1 August voucher (a Journal of 3 Aug 2026, 500.00); record it. Any miss = STOP. BACKUP T0 to C:\bridge-lab\626-T\backups\T0\ (C5).
- t1 (control): send IMP0. EXPECT CREATED 1 (C4); VCH shows a 4 Aug Payment, 610.00, debiting "Lab Plain Ledger" (one row by the rule above), crediting Cash. Anything else = STOP: the import shape or Cash is not as assumed, and no probe is sent.
- Part A, twins that differ by a trailing line break:
  - t2: send LED1. EXPECT [26 Sep, one run on another book]: CREATED 1, ALTERED 0; LEDGERS shows one MORE row, "Lab Line Break Ledger" without a break under Indirect Incomes, with its own GUID; the row with the break unchanged. If instead ALTERED 1 (no twin made): record which row changed and how, skip t3, go on to Part B.
  - t3: send IMP1. RECORD the answer in full: CREATED 1 and which ledger the 5 Aug Payment debits (the rule above), or the error result with its LINEERROR. LEDGERS: EXPECT the same row count as after t2.
- Part B, case twins by request:
  - t4: send LED2. EXPECT CREATED 1; one more row, "LAB CASE PROBE".
  - t5: send LED3. EXPECT [12 Oct, the other direction, on another book]: ALTERED 1, CREATED 0, the same GUID as t4's, its parent now Indirect Incomes; record the row's NAME attribute and its NAME elements as they read (the attribute may keep the old spelling for a while: not a STOP). If instead CREATED 1 (a case twin by request): record both rows. No import is aimed at this pair.
- Part D (before Part C on purpose), an alias:
  - t6: Alter, Ledger, "Lab Plain Ledger": add the alias "Lab Plain Alias" in the alias field under the name; change nothing else; accept (Ctrl+A); back at the Gateway. LEDGERS: EXPECT that row's GUID unchanged and a second NAME "Lab Plain Alias" on it. If the screen offers no alias field: record that, leave with Ctrl+Q, and IMP3 is NOT sent ("t7 not run").
  - t7: send IMP3. RECORD: CREATED 1 and the 7 Aug Payment debits "Lab Plain Ledger" (the alias reached its ledger; the rule above), or the error result.
- Part C, Tally's own screens (one attempt each):
  - t8: Create, Ledger. Name "LAB PLAIN LEDGER" (capitals), Under Indirect Incomes. RECORD what the screen does: a message at the Name field (then the screen rule above), or it lets the ledger be accepted: accept it (Ctrl+A), and LEDGERS shows whether there is now a second row beside "Lab Plain Ledger".
  - t9 (only if t8 left TWO rows): send IMP2. RECORD: which of the two the 6 Aug Payment debits (the rule above; "not told apart" is a result), or the error result. Otherwise IMP2 is NOT sent: write "t9 not run: no case twin on screen".
  - t10: put the 16 characters "Lab Screen Break" followed by CR LF on the Windows clipboard and check the clipboard holds 18 characters. Create, Ledger; with the cursor in the empty Name field press Ctrl+V ONCE. Screenshot. RECORD what the field shows and where the cursor is. If nothing arrived: write "Ctrl+V pasted nothing", leave with Ctrl+Q. If the name is in the field and a ledger can be accepted from there: Under Indirect Incomes, accept, and LEDGERS says whether its stored name ends in a line break (18 characters against 16). If the screen has gone somewhere this item does not describe: press nothing more, screenshot, report.
- End (put the book back): restore T0 (C6). MARKS: RECORD both marks beside t0's. LEDGERS: the t0 list, row for row (NAME elements, GUID, parent). VCH: the t0 table. Else STOP: "the book is not as found". Then Close (C10).
- SAFE PLACES TO STOP: after any step's three reads. The End restore is owed before any other item uses this company. Report after t3 and at the end, without waiting in between, or at a STOP.

## Stop rules (beyond LAB-COMMON)
A ledger or voucher of the t0 lists missing or changed where the step does not say so; a ledger appearing that no step made; a Create or an import answering anything other than the outcomes its step lists; a screen that asks something the step does not name.
WHAT THE BOOK HOLDS IF THE ITEM STOPS (say which in the report): the t0 book plus every ledger, alias and voucher of the steps finished so far; list them from the last LEDGERS list and VCH table kept. A STOP with a ledger screen open says which screen and what is typed in it. The way back is T0, on the run coordinator's word, then Close. If T0 cannot be restored: STOP; the only earlier backups of this book are the two of 10 Oct (TDBK1800_100042.001, before the seed, and TDBK1800_100042_1.001, before the 3 Aug Journal), neither of which is the book as found: read their folder, bytes and sha256 out of the 10 Oct run's records in the report, and restore neither without the run coordinator's word.

## What it decides
- t3: whether a name that only FOLDS to two ledgers can land on either of them. ComplyEaze Bridge never sends such a name; the answer says whether one typed elsewhere could.
- t5, t8, t9: whether twins that differ by case alone can exist, by request (small letters after capitals; capitals after mixed case was measured on 12 Oct) or on the screen, and if on the screen, where a voucher goes.
- t10: whether Tally's own screen can store a name that ends in a line break, or only an import can.
- t6, t7: whether a voucher that names an alias lands on the aliased ledger; ComplyEaze Bridge's catalogue does not read aliases today.
Not covered: both creation orders for each pair; an alias that only folds equal to another ledger's name; a ledger balance read by GUID (it would settle "not told apart"); Gold and Education; other releases; the same through ComplyEaze Bridge's own build and post.
End state: BRIDGE LINE BREAK LAB as found; one new backup T0 on the lab PC; Tally on 9001 with no company loaded.

Evidence branch: lab/626-twin-answers.
