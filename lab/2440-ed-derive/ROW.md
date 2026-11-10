# Row 2440-ED-D: a derived copy of row 2440-ED's s1, for #2440's fixture

Part of #2162 (#2440, the e-invoice read-back).

**File work only.** No request is sent, Tally is not started, and no book is opened. The row runs one script on a file already on the lab PC.

## Why

Row 2440-ED (`lab/2440-ed-answers` at 7b115b27) kept its e-invoice day read, s1, on the lab PC: it holds 2 GSTIN-shaped values, the book's invented customer GSTIN, outside the synthetic identifier family the branch gate admits (`kept-local.txt`). #2440's parser reads no GSTIN, so a copy with those values replaced serves as its fixture. The copy is DERIVED, not captured: every byte other than the replaced values is as Tally answered it, and the report says where each value was.

## Input

- `answers/a-s1-E1-einvoice-day.xml` in row 2440-ED's folder on the lab PC: 127,966 B, sha256 `f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0` (`kept-local.txt`, `evidence/answers-sha.txt`).
- s2 is byte-equal to s1 and is not used.

## Run

From this folder, with the lab PC's `python` (3.10; there is no `python3` command):

```
python -I derive-gstin.py <row 2440-ED folder>/answers/a-s1-E1-einvoice-day.xml f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0 2 a-s1-E1-einvoice-day.derived.xml derive-report.txt
```

`derive-gstin.py` replaces each GSTIN-shaped value (`count-ed.py`'s pattern) by a family member of the same length: the value's own two state digits, `ZZZZZ0000Z`, an entity character numbering the distinct value in order of first appearance, `Z`, and the computed GSTIN check character. Equal values get equal replacements. It never prints or writes an original value. It refuses, and writes nothing, if:
- the input's sha256 is not the one above;
- the input does not hold exactly 2 such values;
- an output file already exists;
- after the replacement, the size differs, a value outside the family remains, a code unit outside the replaced values differs, or expat does not read the same elements in the same order.

## Expect

- Exit 0, and the report's lines printed: input and derived sizes (both 127,966 B) and sha256s, `values replaced: 2`, the differing code units (at most 24: the state digits and the fourteenth character are unchanged; all inside the replaced values), the element count, and one line per value with its code-unit offset, byte offset and replacement.
- **Stop** on any line beginning `REFUSED:`. Copy that line into RESULTS.md and push nothing else from this row.

## Push

To `lab/2440-ed-answers`, in a folder `derived/` beside `answers/`, after the branch gate passes on them:
- `a-s1-E1-einvoice-day.derived.xml` (binary, kept byte for byte: the folder's `.gitattributes` is `* -text`);
- `derive-report.txt`;
- this row's `RESULTS.md`: the command as run, its whole output, the time, and the sha256 of `derive-gstin.py` as run (it must match `SHA256.txt`).

The original s1 stays on the lab PC, as before.
