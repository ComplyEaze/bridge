# L0b: ten ledgers created in one import request, and the ten-ledger approval box on Windows (lab sheet, not for merge)

Part of #690. It follows L0 (`lab/690-batch-ledger-create-l0`, answers at `078db03e`), which measured three ledger
`Create`s in one request on TallyPrime 7.2: `CREATED 3`, a mark step of three, and the new ledgers at the next three
`ALTERID`s in request order. ComplyEaze Bridge's batch of ledgers is held at three until a larger request is
measured. This sheet measures ten, the owner's ceiling, in two parts:

- **Part A**, a raw capture sent with `curl`: one request of ten ledger `Create`s. ONE write.
- **Part B**, the approval box for ten ledgers on Windows: whether it is drawn whole on the lab's screen. It sends
  nothing to Tally: the box is declined. Part B needs a build whose batch limit is ten, which no branch has yet; it
  runs only when the run coordinator names such a build. Part A does not wait for it.

Synthetic BRIDGE company only: never a client book, never REGISTER LAB. About 30 minutes for Part A; 15 for Part B.

## The company

BRIDGE LEDGERBOX CI LAB, licensed TallyPrime 7.2 Silver, loaded alone, as in L0. The request files name it in
`SVCURRENTCOMPANY`; another company needs the requests rendered again, never edited by hand.

## Rules

- **Backup BK0 first.** Before step A0, back the company up into a NEW empty folder as BK0; record its name, size and
  SHA-256. Part A creates ten ledgers, and BK0 is the way back. No BK0, or a BK0 with no recorded hash = do not start.
  After step A3, the owner (or the lab, on the owner's standing word for synthetic books) restores BK0 by Tally's own
  Restore, and step A5 checks it.
- One request at a time; wait for each answer before the next. Never retry, cancel or re-send a write. Before every
  request, Tally shows its Gateway with no dialog open.
- Send each file's bytes exactly as they are, `Content-Type: text/xml; charset=utf-16`, to `http://127.0.0.1:9001`,
  and write the answer to a file with `curl --output`; never pipe it. Read each answer with an XML parser, never by
  eye or by grep. The answers are UTF-16LE without a byte-order mark and may carry `&#4;`: decode and parse for that.
  Record each answer file's size and SHA-256.
- Before sending, check each request file's SHA-256 against `SHA256.txt`; a mismatch = STOP.
- STOP and report, with nothing improvised, on: any dialog in Tally, a second company loaded, Education mode, an
  answer that does not parse, or a count unlike an EXPECT line below.
- Record: TallyPrime's release and edition, the company's GUID, BK0's hash, and a timed log of every request with its
  file name.

## Part A: ten creates in one request

A0. `marks.utf16le.xml`, `ledgers.utf16le.xml`, `groups.utf16le.xml`. EXPECT one company row (this company); record
    its `ALTMSTID` as M0. Record every ledger's GUID and `ALTERID`, and every group's. EXPECT no ledger named
    `L0b Batch 01` to `L0b Batch 10`, under any case, spacing or alias; one present = STOP (the request would alter
    it).
A1. **Request** `create-10.utf16le.xml`: ten party ledgers under Sundry Creditors, not bill-wise, no opening, no
    GSTIN, in this order: `L0b Batch 01` to `L0b Batch 10`. Record every counter Tally gives (`CREATED`, `ALTERED`,
    `DELETED`, `COMBINED`, `IGNORED`, `ERRORS`, `CANCELLED`, `EXCEPTIONS`, `LASTMID`, `LASTVCHID`) and any `LINEERROR`
    text. No EXPECT on the counters: they are what this part measures.
A2. `marks`, `ledgers`, `groups` again. Record M1 = `ALTMSTID` now. For each ledger, whether its GUID is new, its
    `ALTERID`, and whether any ledger or group that existed at A0 has another `ALTERID` now.
A3. Nothing is sent. Copy every answer file and the log into the evidence folder.
A4. BK0 is restored by Tally's own Restore.
A5. `marks`, `ledgers`. EXPECT the ledger GUIDs and `ALTERID`s of A0 exactly, and no `L0b Batch` ledger. A difference =
    STOP and report; nothing more is sent.

### What Part A decides

| Question | Read from |
| --- | --- |
| Does one request of ten new ledgers count `CREATED 10` and nothing else? | A1 |
| Does `ALTMSTID` step by exactly ten (M1 = M0 + 10)? | A0, A2 |
| Do the ten new ledgers take M0+1 to M0+10, in request order? (L0 saw the order for three; this repeats it.) | A2 |
| Is every ledger and group of A0 unchanged? | A0, A2 |
| How long did Tally take to answer the request (its time in the log), against L0's three? | A1 |

The batch limit rises to ten only if A1 shows `CREATED 10` and A2 a step of ten with the span M0+1 to M0+10. Any other
answer keeps it at three and is reported as found.

## Part B: the ten-ledger approval box on Windows

Runs only with a build whose batch limit is ten, named by the run coordinator with its program file's sha256. It
sends nothing to Tally: every box is declined.

- Settings as for the earlier ledger box row: posting on (`BRIDGE_AGENT_ENABLE_WRITES`), ledger creation on
  (`BRIDGE_AGENT_ENABLE_MASTER_WRITES`), a fresh empty data folder, typed by the owner.
- The screen as for the earlier ledger box row: 1920x1080 at 125% scaling. Record both.

B0. BK0 is not needed: nothing is written. `list_companies`: EXPECT exactly this company.
B1. `prepare_ledger_creates` with `ledgers`: ten parties on `sundry_creditors`, no GSTIN, `bill_wise` false, named
    `L0b Box 01` to `L0b Box 10`. EXPECT a `batch_id` and a preview of 27 lines: five header lines, two lines for each
    of the ten ledgers, a blank line and the warning.
B2. `post_ledger_creates` with that `batch_id`. The box opens. Without answering it, record by screenshot:
    - whether the whole box is on the screen: its title, the first line, all ten ledgers, the warning and the buttons;
    - whether any part of it is cut off by the screen's edge or the taskbar;
    - its title text and its buttons' labels.
B3. Answer **No**. EXPECT `master_approval_declined`, nothing sent (the ledger catalogue and the marks unchanged from
    B0, read through `ledger_masters` and `tally_status`), and the masters journal holding the batch with no intent.

### What Part B decides

| Question | Read from |
| --- | --- |
| Is a ten-ledger box drawn whole on the lab's screen at 125% scaling? | B2 |
| Do its title and buttons name the count ("create 10 ledgers?")? | B2 |
| Does declining it send nothing? | B3 |

A box that does not fit keeps the limit below ten, whatever Part A shows, until the ledger box is measured against the
screen as the post's box is.

## The requests

`marks.utf16le.xml`, `ledgers.utf16le.xml` and `groups.utf16le.xml` are L0's read requests, byte for byte (the same
company). `create-10.utf16le.xml` is the UTF-16LE form (with a byte-order mark) of one request rendered by
ComplyEaze Bridge's own ledger `Create` message and masters import envelope at `e2ce63cd` (whose renderers are master's), with the ten messages one after
another in one envelope; a temporary test asserted that its first message is the one a single ledger renders. A
candidate shape for ten ledgers, sent here to the lab book only.
