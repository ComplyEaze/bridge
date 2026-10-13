# Run sheets and requests for three Windows lab items (#626, #1038, #1050), 13 Oct 2026

Not for merge. Synthetic lab companies only; no ComplyEaze Bridge build is used by any of the three.

- `LAB-COMMON.md`: the rules all three items carry (step 0, raw requests, reading an answer, backup, restore, screens and dialogs, after a STOP, Close, evidence).
- `WINDOWS-626-T.md`: which ledger a voucher lands on when ledger names differ only by a trailing line break, by case, or are reached through an alias (#626).
- `WINDOWS-1038-R.md`: what a voucher that arrives by import takes after a backup is restored, and in which order a batch is numbered (#1038).
- `WINDOWS-1050-D.md`: whether any value on the Company object changes when a backup is restored and at no other time (#1050).
- `requests/<item>/`: every request file of that item, ready to send, byte for byte (UTF-16LE with a BOM, no line break at the end; `ledgers.utf16le.xml` keeps the line breaks of the pinned request it is made from).

Where an item says "File on this branch: requests/...", that file is the request; the item gives the size and sha256 it must have, and `SHA256.txt` lists every file here. Check both before a send. A file that writes is sent once.

Order: 626-T, then 1038-R, then 1050-D, each through its Close before the next begins. 1038-R and 1050-D use the same company and are never interleaved.
