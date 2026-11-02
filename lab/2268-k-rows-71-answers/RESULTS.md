# Results: #2268, the K rows on the Edit Log 7.1 copy, 2 Nov 2026

**No K row ran. The copy will not open this book without migrating it, and a migration was not accepted. No request reached the book on 7.1, so this says nothing yet about the order rule on 7.1.**

## What ran
- **The program:** TallyPrime Edit Log, listed installed as Series A 7.1.0 Build 29327, file version 1.1.7.1, run from a copy of the program on D:, on port 9011, with its own settings file. Its title band reads Edit Log, SILVER. It is the Edit Log variant, not plain 7.1. The installed program and its settings were not touched: that file's sha256 was equal before and after.
- **The book:** a file copy of BRIDGE ABANDON LAB (synthetic) from 7.2's data folder, taken with no company loaded in 7.2, into the copy's own new data folder; 21 files, 152 MB, byte-equal to the source. The copy's settings named it to load at start, so no key was needed. A second copy was kept first, as the backup, because the copy's own Backup screen cannot be driven in this session.
- **Sheet:** `lab/1713-settle-order` at 869bca9e, read from origin; `ORDER-ROWS.md` unchanged.

## What the copy did
Started, `GET /status` on 9011 answered 200, and at 30 s it showed **Migrate Company Data: the selected company will be migrated**, naming the book and its number. The screen says TallyPrime Edit Log will always keep a log of all activities. Its choices are Configure and Migrate; Backup Company Data before Migration read Yes, to TallyDrive. [verified]

No key was pressed. The screen was left unanswered, and the process was ended by its own id. 7.2 kept its process and port and answered 200 throughout. [verified]

## After
- The copy's 7 data files of type `.TSF`, Tally's session state, differ from the backup, three of them new. No `.1800` data file differs. The source book in 7.2's folder is byte-equal to the backup. [verified]
- The copy's settings no longer name a company to load.

## What this answers
- [verified] A book made on plain TallyPrime 7.2 does not open on this Edit Log 7.1 program without a migration. That is a stop for this row as worded.
- [not established] Whether a migrated book would behave as an unmigrated one for the K rows, whether plain 7.1 opens it, and whether a book made natively on Edit Log 7.1 could carry the K rows.

**Confidence:** one start, one book. It does not measure the order rule, on Edit Log 7.1 or on plain 7.1.

## Deviation
The sheet's backup is by Tally's Backup screen (C5). Here it is a file-level copy of the book, listed in `backup-manifest.txt`, as that screen could not be driven on the copy.
