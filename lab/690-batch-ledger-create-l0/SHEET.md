# L0: three ledgers created in one import request, and one name colliding in a batch (lab sheet, not for merge)

Part of #690. A raw capture on TallyPrime 7.2, sent with `curl`, not through ComplyEaze Bridge's tools. It decides
whether ComplyEaze Bridge may build one approval for several ledger creates: what Tally counts, how far the
company's master mark (`ALTMSTID`) steps, and which `ALTERID` each new ledger takes, when one request carries
three ledger `Create`s. Nothing about it is known for masters: the protocol reference's §9.4i lists "more than one
ledger in one request" as not established, and §9.15 measured the step and the order for vouchers only.

Synthetic BRIDGE company only: never a client book, never REGISTER LAB. TWO writes in all (requests A and B),
each sent once. Every other request is a read. About 30 minutes.

## The company

BRIDGE LEDGERBOX CI LAB, licensed TallyPrime 7.2 Silver, loaded alone. The request files below name it in
`SVCURRENTCOMPANY`; another company needs the requests rendered again (see "The requests"), never edited by hand.

## Rules

- **Backup BK0 first.** Before step 0, back the company up into a NEW empty folder as BK0; record its name, size
  and SHA-256. Request B alters an existing ledger, and an altered ledger's earlier values cannot be put back by
  any request: BK0 is the only way back. No BK0, or a BK0 with no recorded hash = do not start. After step 5, the
  owner restores BK0 by Tally's own Restore (step 6), and step 7 checks it.
- One request at a time; wait for each answer before the next. Never retry, cancel or re-send a write. Before every
  request, Tally shows its Gateway with no dialog open.
- Send each file's bytes exactly as they are, `Content-Type: text/xml; charset=utf-16`, to `http://127.0.0.1:9001`,
  and write the answer to a file with `curl --output`; never pipe it. Read each answer with an XML parser, never by
  eye or by grep. Record each answer file's size and SHA-256.
- Before sending, check each request file's SHA-256 against `SHA256.txt`; a mismatch = STOP.
- STOP and report, with nothing improvised, on: any dialog in Tally, a second company loaded, Education mode, an
  answer that does not parse, or a count unlike an EXPECT line below.
- Record: TallyPrime's release and edition, the company's GUID, the backup's hash, and a timed log of every request
  with its file name.

## Steps

0. `marks.utf16le.xml`, `ledgers.utf16le.xml`, `groups.utf16le.xml`. EXPECT one company row (this company);
   record its `ALTMSTID` as M0. Record every ledger's GUID and `ALTERID`, and every group's. EXPECT no ledger named
   `L0 Batch Alpha`, `L0 Batch Bravo`, `L0 Batch Charlie`, `L0 Batch Delta` or `L0 Batch Echo`, under any case,
   spacing or alias; one present = STOP (the request would alter it).
1. **Request A** `create-a.utf16le.xml`: three party ledgers under Sundry Creditors, not bill-wise, no opening, no
   GSTIN, in this order: `L0 Batch Alpha`, `L0 Batch Bravo`, `L0 Batch Charlie`. Record every counter Tally gives
   (`CREATED`, `ALTERED`, `DELETED`, `IGNORED`, `ERRORS`, `CANCELLED`, `EXCEPTIONS`, `LASTMID`, `LASTVCHID`) and any
   `LINEERROR` text. No EXPECT on the counters: they are what this row measures.
2. `marks`, `ledgers`, `groups` again. Record M1 = `ALTMSTID` now. For each ledger, whether its GUID is new, its
   `ALTERID`, and whether any ledger or group that existed at step 0 has another `ALTERID` now.
3. **Request B** `create-b.utf16le.xml`: three ledgers of the same shape, in this order: `L0 Batch Delta` (new),
   `L0 Batch Bravo` (the ledger request A made, every field sent as request A sent it), `L0 Batch Echo` (new).
   Record every counter and any `LINEERROR`.
4. `marks`, `ledgers`, `groups` again. Record M2, and the same three facts as step 2, now including whether
   `L0 Batch Bravo` kept its GUID and whether its `ALTERID` moved.
5. Nothing is sent. Copy every answer file and the log into the evidence folder.
6. The owner restores BK0 by Tally's own Restore.
7. `marks`, `ledgers`. EXPECT the ledger GUIDs and `ALTERID`s of step 0 exactly, and no `L0 Batch` ledger. A
   difference = STOP and report; nothing more is sent.

## What the result decides

Recorded by the coordinator from the answers, each with its confidence:

| Question | Read from |
| --- | --- |
| Does one request of three new ledgers count `CREATED 3` and nothing else? | step 1 |
| Does `ALTMSTID` step by exactly 3 (M1 = M0 + 3)? | steps 0, 2 |
| Do the three new ledgers take `ALTERID`s M0+1, M0+2, M0+3, in request order? | step 2 |
| Is every ledger and group of step 0 unchanged? | steps 0, 2 |
| With one name already a ledger, does Tally create the other two and alter that one (`CREATED 2, ALTERED 1`), or refuse the request? | step 3 |
| Does the altered ledger keep its GUID, and does an alteration that changes no field move its `ALTERID` and the mark? | steps 2, 4 |
| Does the mark step by 3 for request B? | steps 2, 4 |
| On 7.2, does each master the requests write take the mark's next value? (The check that a single create's step is exactly one rests on this; the reference has it for 7.1 only, §9.4j.) | steps 0, 2, 4 |

ComplyEaze Bridge builds one approval for several ledgers only if request A shows `CREATED 3`, a step of 3 and
the span M0+1 to M0+3. Any other answer changes the design before any code.

## The requests

Each request file is the UTF-16LE form (with a byte-order mark) of a request rendered by ComplyEaze Bridge's own
code at master `5b2a352d`: the reads by the renderers its ledger tools send inside the queue (the company marks,
the V3 ledger catalogue and the group snapshot), and the two writes by its ledger `Create` message and its masters
import envelope, with the three messages of each request one after another in one envelope. A candidate shape for
several ledgers, sent here to the lab book only.
