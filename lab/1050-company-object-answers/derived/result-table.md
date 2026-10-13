# 1050-D result: paths of the COMPANY object that are in any set

Reads: 1 as found; 2 after 5 minutes untouched; 3 after shut and reload; 4 after a backup; 5 after a restore of that same backup;
6 after a restore of an older backup (one voucher fewer); 7 after the first backup restored again.

| path | read 1 | read 2 | read 3 | read 4 | read 5 | read 6 | read 7 | sets |
|---|---|---|---|---|---|---|---|---|
| COMPANY/AUDITEDUPTO[0] | '20260710' | '20260710' | '20260710' | '20260710' | '20260710' | '20260701' | '20260710' | FOLLOWS-CONTENT |

Paths per read: read 1: 414; read 2: 414; read 3: 414; read 4: 414; read 5: 414; read 6: 414; read 7: 414.
Set sizes: NOISE 0; RELOAD 0; BACKUP 0; RESTORE-SAME 0; FOLLOWS-CONTENT 1; EVERY-RESTORE 0.

A. Paths that changed at the restore of the same content and not by themselves, on a reload or at a backup (RESTORE-SAME): none among 414 paths
B. Paths that differ between two restores of the same backup and not by themselves or on a reload (EVERY-RESTORE): none among 414 paths

Reads 1, 2, 3, 4, 5 and 7 are byte-identical answers (32,986 bytes each). Read 6 differs from them in the one path above.
The shape used (the one whose control passed at the first read) returned no ALTVCHID and no ALTMSTID element on the COMPANY row.
A path here is every attribute and every leaf element of the COMPANY row, empty ones included (414 in all, 199 with text).
Withheld paths: none (the only path in the table holds a date).
