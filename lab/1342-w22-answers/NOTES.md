# Does Tally keep the narration of a keyed Sales invoice byte for byte (#1342, Windows lab, 13 Oct 2026)

Synthetic data only: BRIDGE PILOT LAB (its GUID is in every marks answer). TallyPrime 7.1, Silver Rental,
on Windows. One run. No ComplyEaze Bridge build and no import: three Sales invoices were keyed on Tally's
own voucher screen, each with one narration, and read back raw by curl, one request at a time, none sent
twice. A backup was taken first and restored at the end. Times in send-log.txt are UTC.

## Method

- Two request files, six sends: the marks read before the keying, after it and after the restore; the
  one-day voucher read for 20 Sep 2026 at the same three points. Each answer was kept with its response
  headers. Each send was `curl.exe -sS -w "%{http_code} %{size_download}" --data-binary @<request>
  -H "Content-Type: text/xml; charset=utf-16" -D <headers file> http://127.0.0.1:9001 -o <answer file>`.
- The invoices: voucher type BRIDGE Sales, 20 Sep 2026, BP/26-27/0050 to 0052, the shape of BP/26-27/0001
  (party BRIDGE Walk-in; BRIDGE Svc 998313 5% 1,000.00; BRIDGE CGST 2.5% 25.00; BRIDGE SGST 2.5% 25.00;
  total 1,050.00), each saved with the plain save.
- Each narration was typed key by key, every space with the space key, and the field was read on screen
  before the save.

## Result

`tables/w22-narrations.txt` holds each text between two marks, with its length in characters.

| invoice | typed | the answer held | after XML decoding |
|---|---|---|---|
| 0050 | `Probe one & two <three> "four" 'five' 6% [seven]` (48) | the same, with `&amp;` `&lt;` `&gt;` `&quot;` `&apos;` for the five characters (78) | equal to the typed text (48) |
| 0051 | `Probe`, two spaces, `two`, two spaces, `spaces, and a trailing space`, one space (41) | both runs of two spaces; no space after the last word (40) | the typed text without its trailing space (40) |
| 0052 | `Probe three [BRIDGE:probe-0052]` (31) | the same (31) | equal to the typed text (31) |

- Before the keying the day held no voucher (3,022 B). After it: exactly three vouchers in 24,068 B,
  MASTERID 45 to 47, ALTERID 69 to 71, each with BRIDGE Walk-in -1050.00 and the lines 1000.00, 25.00, 25.00.
- Marks (ALTVCHID / ALTMSTID): 68 / 245 before, 71 / 245 after the keying, 68 / 245 after the restore.
  After the restore the marks answer (3,394 B) and the day's voucher answer are byte-identical to the first.
- 0051 on screen before the save: wider gaps after `Probe` and after `two`, and the cursor one space
  after the last word. Whether the trailing space was dropped by the field, by the save or by the export
  was not distinguished.

## Current date and prompts

Tally's current date at the Gateway read 15 Sep 2026 at the start. After the saves it read 20 Sep 2026,
the vouchers' date, moved by Tally. After the restore it read 15 Sep 2026 again before any key. The date
key at the Gateway was never pressed.

The backup asked "Accept ?"; the restore asked "Accept ?" and then whether to replace the company data
folder (Replace); each was answered once. No save raised a dialog. No list raised "Too many choices" and
nothing was typed into a rate field.

## One slip and two small differences

- On 0052 the desktop tool that pressed the keys delivered the 19-character suffix by a paste instead of
  key by key, and the field read `Probe three Lab Screen Break`. Nothing had been saved. The 16 wrong
  characters were removed with Backspace, the suffix was typed by key in three pieces, the field read
  `Probe three [BRIDGE:probe-0052]` and was then saved. Screenshots of both states are kept.
- The three invoices were keyed one after another on the voucher screen; the screen was left once, after
  the third save, with no prompt.
- On the restore screen the source path showed one capital letter where a small one had been typed. It
  was the same folder: the backup file read through that spelling had the size and hash recorded when it
  was taken.

## What it decides

On this book and this release, for a narration typed on the Sales invoice screen: the characters
`& < > " ' % [ ] :` and a tag-shaped suffix came back unchanged after XML decoding, repeated spaces inside
the text were kept, and one trailing space did not come back.

## Not covered

A narration written by an import or by ComplyEaze Bridge's own post; leading spaces; a multi-line or
longer narration; other voucher types; other releases; Gold.

## Files

`requests/` (2), `answers/` (six, each with its headers), `tables/`, `derived/` (the answers parsed by
`tools/read-answers.ps1`), `send-log.txt`, `SHA256.txt`. Kept on the lab PC, by hash in
`SHA256-kept-on-lab-pc.txt`: screenshots, the backup, process and port listings, the working log.
