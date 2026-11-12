# Import answers of a post that landed in part (bridge#1108, rows P2 and P3)

Covers `partial-landing-one-refused-of-three.utf16le.xml` and
`partial-landing-twins-one-refused-of-three.utf16le.xml`.

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `partial-landing-one-refused-of-three.utf16le.xml` | 726 | `7ba9450729a82eb7a20667baf382f62ae93da2d11332157c87d25f4b64b62b4b` |
| `partial-landing-twins-one-refused-of-three.utf16le.xml` | 726 | `609cb603488a334e90a28b52919327d8062c51177af02d4e5e7d49b3ad5ca850` |

Captured, not authored: each file is the body of Tally's answer to one import, copied without
any change. They are `sitting-2/answers/proxy-import-2-body.xml` and
`sitting-2/answers/proxy-import-3-body.xml`, with the same hashes in `SHA256.txt`, of the lab
evidence branch `lab/1108-a2-partial-landing-answers` at commit
`ebf70249` (queue item 84, TallyPrime 7.2, one synthetic lab book). The lab's `RESULTS.md`
names the rows: P2 (`E-P2`, `E-PL`, `E-P2m`) and P3 (`E-P3`, `E-T`). Neither file holds a
company GUID or number, so no substitution was made.

## What was read

- P2: three vouchers sent, the middle one refused. The answer reports `CREATED 2`,
  `EXCEPTIONS 1`, every other counter `0` and `LASTVCHID 3`, with one `LINEERROR`. The lab
  read the company's voucher mark up by 2 and two vouchers added.
- P3: the same three-voucher post with the two vouchers Tally created identical twins. The
  answer reports the same counters, with `LASTVCHID 5`.

## What it establishes

On this book, in this build: an import of three vouchers of which one is refused is answered
with the counters above, every counter reported. The answer does not say which voucher was
refused. The `LINEERROR` names a ledger of the lab book and is not read for a cause (§9.2).

## What it does not establish

Which of the three vouchers Tally refused: the lab read it from the refused voucher's content
and from the two vouchers added, on a book with no pre-existing equal voucher. The fixtures
are used for the counters only. No test here reads a voucher as refused from them.
