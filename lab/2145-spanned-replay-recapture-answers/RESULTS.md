# 2145-R: the replays that held only an unspanned census, recorded again with the census spanned (Part of #2145)

Sheet `lab/2145-spanned-replay-recapture` at 8a2ecc3a (`check_requests.py`: files ok 22, requests ok 17). Requests were sent by curl, one at a time, by LAB-COMMON C3's line; reads only. TallyPrime 7.2 Silver, 30 Oct 2026, 07:10 to 07:26 UTC. Tally's data path at the sitting's first Select Company visit: `<Tally data folder>`; the lab is left on it.

| part | result | confidence |
|---|---|---|
| A: BRIDGE AMEND LAB as it is now | **PASS**: marks 1829 / 245 at the start and the end. The 1 April census has 49 rows, none above 1829, with BODY digest `ce95e4cf…` (= 2145-C). The 3 April census has 4 rows, none above 1829, with BODY digest `ee381b99…` (= 2145-C). Both verification reads are recorded (49 and 4 rows) | verified |
| P: BRIDGE PILOT LAB, the register call for 4 to 5 August 2026 | **PASS**: marks 68 / 245 at the start and the end; the extent check holds (68, 245, 20260915, 20260401). The census has 3 rows (ALTERIDs 31 and 32 on 4 Aug, 35 on 5 Aug), none above 68. The window has the same 3: two Purchase, one BRIDGE Sales | verified |
| 0: backups of BRIDGE AMEND LAB | one file carries its number: a backup of every company taken 12 Oct 2026 08:00 (616,136 B, sha256 `f8ad5a71…`) | verified |
| B1: that backup, restored apart into a new folder | **stopped by the rule**: the copy's marks are 1829 / 245, not one of (1469, 222), (1676, 229), (1685, 232); nothing more was sent to it | verified (the marks); hypothesis (that every recorded state is older than 12 Oct) |

Part B kept Tally's data folder and its book untouched. The backup of the live book was taken first, then the copy was restored into `2145R\restored-B1`, and only "Accept ?" was answered. The live book read 1829 / 245 again after Select Company's data path was set back. The restored folder, its source copy and the live-book backup stay on the lab PC on the run lead's word.

On the run lead's word, no backup on the lab PC holds a recorded state, so #2199's three AMEND LAB replays take route (A) and are re-derived from Part A's answers.

Every answer is kept whole, with its headers, send log, `GET /status` record and `summarise2145r.py` output. The company numbers in the company-list and extent answers are redacted, with the answers kept UTF-16LE (`substitutions.txt`). Part 0's full listing, backups and captures stay on the lab PC, listed by sha256 (`kept-local.txt`, `SCREENSHOTS.txt`). Notes, one line per step with UTC times: `notes.txt`.
