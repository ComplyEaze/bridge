# Results: #2017, value per godown for an item inside a stock group, 2 Nov 2026

**No route found by raw request. The report gives a group's value per godown, one level down, and stops there: an item inside a group has no line. Every candidate tried either answers byte for byte what the plain exploded report answers, or is the same report. On Tally's own screen the value is there: drilled Godown Summary shows the nested item in each godown, and those lines add up. So the figure exists in Tally; this row found no way to ask for it.**

No read errored, no dialog appeared during a read, and marks across the reads are the same bytes. Book BRIDGE NESTED STOCK LAB, TallyPrime 7.2 Silver, licensed, loaded alone, 1 April to 31 August 2025. Each request is the committed request with the company and date for this book and one change, listed below; one request at a time, `GET /status` 200 after every one.

## The book needed one change, then restored
It held one godown, `Main Location`, so no nested item sat in two. On the row's fallback, after a backup: godown `Lab Second`, and a Stock Journal of 10-Apr-25 moving 5 Nos of `Lab Depth1 Nut`, under `Lab Group One`, from `Main Location`. Marks V 2 and M 219 before, V 3 and M 220 after.

Afterwards the backup was restored, and the marks, the godown list and the control read are each byte-equal to the opening reads. Period and date are back at 1-Apr-26 to 31-Mar-27, 15-Apr-2026.

## The candidates
| request | one change | answer | |
|---|---|---|---|
| `s1-control` | none | 2,776 B | group's items have no godown lines, as measured; `Lab Root Bolt`, under the root, has one |
| `q1` | ID `Godown Summary`, a screen title | 2,294 B | each godown one level down: `Lab Group One`, no item in it |
| `q2` | ID `Godown Monthly Summary`, a Go To list name | 1,814 B | months, no godown, group or item |
| `q3` | ID of a report that does not exist | 444 B | `LINEERROR` "Could not find Report", `STATUS` 0 |
| `q4` | ID `Stock Group Summary`, the drilled title | 2,776 B | **equal to the control** |
| `q5` | `SVSTOCKGROUP` Lab Group One | 2,776 B | **equal to the control** |
| `q6` | `SVGODOWN` Lab Second | 2,776 B | **equal to the control** |
| `q7` | `EXPANDALL` Yes | 2,776 B | **equal to the control** |
| `q8` | `q1` with `GODOWNNAME` Lab Second | 2,294 B | **equal to `q1`** |
| `q9` | `q1` with `SVGODOWN` Lab Second | 2,294 B | **equal to `q1`** |
| `q10` | `q1` with `SVSTOCKGROUP` Lab Group One | 2,294 B | **equal to `q1`** |
| `q11` | `GODOWNNAME` Lab Second | 2,776 B | **equal to the control** |

`q8` to `q10` carry two changes against the exploded Stock Summary: the ID and the variable. [verified]

**Where the names came from.** Titles and list names from Tally's screens. Screens show labels, not variable names, so the variables are strings in `tally.exe` itself, read from the file: `SVGodown`, `SVStockGroup`, `ExpandAll`, `GODOWNNAME`. A string there shows the name exists; it does not show that it is a report variable, or how it is spelled in a request. A variable these reports ignore answers the same as one that does not exist, so these results do not separate "wrong variable" from "wrong spelling or shape". `STOCKGROUPNAME` was not sent: `STOCKITEMNAME` is the measured Stock Query variable, but the file holds that spelling for a group only inside `ParentStockGroupName`, so it would have been a guess. So the "no route" is for these names, not for every name. [partial]

Not tried, on the row's limits: a custom collection or TDL, the Stock Item-wise view (`F5` on the drilled screen), and any request with two variables at once. The configuration's `Format of Report` and `Expand all levels in Detailed format` have no variable name on screen; `EXPANDALL` is the only attempt at the second.

## Screen titles
| where | title | subtitle |
|---|---|---|
| Go To, Godown Summary | `Select Godown`, prompt `Name of Godown` | |
| Godown Summary, Primary | `Godown Summary` | |
| drilled into a godown | `Godown Summary` | the godown |
| drilled into a group | `Godown Summary` | `Godown : <name>`, the group |
| Stock Summary | `Stock Summary` | |
| drilled into a group | `Stock Group Summary` | the group |
| Change View | `Change View`, `Monthly Summary` | |
| F12 | `Configuration` | |

The title stays `Godown Summary` at every depth, so the screen gives no other report name to send.

## The figures, 31 August 2025
From the answers, per godown and group (`q1`), against the Stock Summary (`s1-control`):
- `Lab Group One`: `Lab Second` 10.00 + `Main Location` 385.00 = 395.00, the group's line.
- `Main Location`: group 385.00 + `Lab Root Bolt` 50.00 = 435.00, the godown's line.
- `Lab Second`: group 10.00 = the godown's line. Total 445.00. [verified]

From the screen only, item by godown, as no answer holds it:

| item | Lab Second | Main Location | item's line |
|---|---|---|---|
| Lab Depth1 Nut | 5 Nos, 10.00 | 15 Nos, 30.00 | 20 Nos, 40.00 |
| Lab Depth1 Powder | | 2.500 Kgs, 250.00 | 2.500 Kgs, 250.00 |

Under `Main Location`, `Lab Group One`: `Lab Group Two` 105.00 + 30.00 + 250.00 = 385.00. Every answer's line agrees with its screen, `evidence/screen-lines.txt`. The two drilled item screens were read twice, the second time from the saved capture. [verified]

**A subtraction, not a read.** Since `stock_by_batch` gives the quantity and a group's value by godown is available, an item alone in its group could be priced by arithmetic. This book's group holds several items, so it would not close the gap here. That is an observation, not a proposal.

## Slips
- At the godown create, Tally asked "Main Location exists by default for the Company. Do you want to alter Main Location or create a new Godown?". I typed the name without reading it, and `a` answered Alter Existing. Nothing was accepted, I left by Ctrl+Q, and the godown and marks read byte-equal. Then C, Create New.
- The first voucher date was refused: "Date cannot be below the current period (1-Apr-26)". The period was changed for the session with Alt+F2; the restored book reads as before.
- One F11 view of Company Features, left unchanged. It shows no multiple-godowns switch on this release.

**Confidence:** one book, one nested item in two godowns, one Tally, one sitting: PARTIAL.
