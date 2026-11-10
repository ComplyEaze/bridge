# Row 2464-RS: on the book with two GST registrations and `BRIDGE Sales` bound to `Rajasthan Registration`, does saving the imported, uncertain invoice again on Tally's screen, with no change, fill its sales lines' GST details and clear its GSTR-1 status?

Shape discovery: the import and the candidate read-back are row 2464-RB's files, written for a row and not rendered by the build, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2464 (0.9.0), related to #2467 (gap 1). Written 10 Nov 2026. RB's two master changes on Tally's screen, RB's import A'' sent again under a C3 exception named here, one save on Tally's screen with no change, then a restore from the row's own backup. Confidence of everything here until the row runs: unverified.

## Question
Row 2464-RB (`lab/2464-rb-answers` at afe4fbad) set up two registrations, bound `BRIDGE Sales` to `Rajasthan Registration`, and imported A'' (`BP/26-27/0105`), which names that unit. It was created and read uncertain (included `No`, uncertain `Yes`). Its two sales legs, `BRIDGE Svc 998313 5%` and `BRIDGE Svc 998314 5%`, came back with every GST and HSN/SAC field empty and no rate rows. On `BP/26-27/0010` the same legs hold `Ledger`, the ledger's name, its HSN/SAC code, `Taxable`, `Services` and five rate rows. B'', which also wrote `CMPGSTIN`, changed none of that.

Across rows 2464-GR (second sitting), 2464-NR and 2464-RB, the five imports of this request shape read included where the sales lines are filled and uncertain where they are empty. Two earlier saves on this book bear on it:
- **`BP/26-27/0021`** (#691, `lab/1342-capture-answers`, W7D, 10 Oct 2026): uncertain, with one sales line empty after its ledger's HSN/SAC was set. Opened and saved with no change, it read included, with the line filled (`pilot-lab-invoice-readback-status-uncertain.utf16le.xml` and `pilot-lab-invoice-readback-status-resaved.utf16le.xml` on master).
- **`BP/26-27/0057`** (row 2162-RS, `lab/2162-rs-answers` at 8c24fe17): the build's request, created on the one-registration book and included. Saved with no change from the Day Book, it kept its GUID and MASTERID, moved its ALTERID by one, raised no prompt, and filled `VATEXPAMOUNT` on five legs and `RATEOFINVOICETAX` on the two tax legs. Its status stayed included.

This row changes one variable from RB's s5r: A'' is saved once on Tally's screen with no change, by row 2162-RS's path. It reads R, RC and V before and after, with the legs recorded whole.

- **The set-up is RB's, key for key.** Change 1 is RB's step 3 (GN's step 3: a second registration, `Maharashtra Registration`). Change 2 is RB's step 4 (`BRIDGE Sales`' Default GST Registration set to `Rajasthan Registration`).
- **A'' is RB's `I-0105.xml`, byte-equal, sent once in this row.** Its first send was RB's, and RB's restore undid it. Re-sending it with its number and REMOTEID is a C3 exception, named below.
- **Steps 1 to 5 must reproduce RB's answers** before the save. A difference in any field stops the row before the save, since the save would then not be on RB's invoice.
- **Not asked here:** B'' or any other import, a save with any change, GSTR-1 on Tally's screen, `CMPGSTSTATE` or `CMPGSTREGISTRATIONTYPE` written by a request, and the build's own rendered request.

## The C3 exception this row needs
LAB-COMMON C3 says a file that writes is sent once in the item's life. This row sends `I-0105.xml` (sha256 `b333331a`), which row 2464-RB sent once on 10 Nov 2026 at 06:14:59Z and then restored away.
- **What is re-sent:** that file byte for byte, with its number `BP/26-27/0105` and its REMOTEID tail `000000001105`. It is sent at most once in this row, at step 5, and never again, whatever its answer.
- **Why the same file:** the question is about RB's invoice. A new number or REMOTEID would change the voucher the save acts on.
- **Why it is safe to send:** step 2's N-0105 read must return 0 rows, as RB's s2 did. If it holds a row, the row stops before any key (as RB's step 2).
- **The coordinator's yes** to this exception goes in the queue line. Without it the row is not run.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2464-GN's, 2464-NR's and 2464-RB's) applies in full, with the exception above. Where it and this sheet differ, the stricter one holds.

- **The book is pinned by GUID.** `count19.py` holds BRIDGE PILOT LAB's GUID as `BOOK`, as in RB. STOP before anything is keyed unless s1's COMPANY row is the book's.
- **Its state:** every row queued before this one ends with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245, as RB's s1 read them. Steps 2 to 5 compare each read with RB's answer of the same step. Those compares hold only from the same marks, so at s1 any other marks stop the row, before BK and before any key.
- **BK is this row's way back, and the restore always runs.**
  - BK is taken at step 1, before anything is keyed, and restored at step 8 by C6.
  - Every path from step 3 on that is not a STOP reaches step 8. That includes a compare failing, A'' answered other than created, the save not made, and the save made.
  - After a STOP (C7, C8, C9), C8 holds: nothing more is sent or keyed, and BK is restored only on the run coordinator's word.
  - After the restore, every read is compared with step 2's. Any part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. GN, NR and RB read TallyPrime with `tally.exe` 1.1.7.2, from the program's file.

**No Bridge build.** Raw requests, one at a time, through `run.py`. The two master changes and the save are made on Tally's screen, by hand. The one write is `I-0105.xml`, sent once.

**The sends** are RB's, unchanged.
- Each of these stops the run and is reported as "the request failed", never as "nothing found":
  - a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`);
  - a collection answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count19.py` or `legs.py`, which print STOP and exit non-zero).
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with `count19.py` and copy its whole output into RESULTS.md.
- Close every other Bridge process first, or record that they ran, as RB did, with none called.
- Answers that hold a GSTIN, or a run of ten or more digits outside a `VOUCHERKEY` value, stay on the lab PC under the lab's standing rules, listed by size and sha256, as RB's did. That covers U, V, RC-0010 and every V-0105.

## Files
**Byte-equal to row 2464-RB's file of the same name** (`lab/2464-rb-sheet` at 16e3593a, folder `lab/2464-rb/`; each sha256 is in that sheet's `SHA256.txt` and in this one's): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count19.py`, `M-marks.xml`, `U-taxunits.xml`, `T-types.xml`, `R-0010.xml`, `RC-0010.xml`, `V-0010.xml`, `N-0105.xml`, `R-0105.xml`, `RC-0105.xml`, `V-0105.xml`, `I-0105.xml`, `a-46bs2-T-types.xml` and `a-46bs3-T-types.xml`.
- `I-0105.xml` is A'': RB's `gen.py` wrote it, and RB's step 1 wrote it again and found it byte-equal. No file here is generated, so this sheet has no `gen.py` and no template.
- `R-0105.xml` is the build's read-back for `BP/26-27/0105`. `RC-0105.xml` is that read-back with `GSTREGISTRATION` and `CMPGSTIN` added at the end of its FETCH. `V-0105.xml` is the native read of one voucher by number. `N-0105.xml` is the build's number read.

**New in this row: `legs.py`.** It reads one R- or RC- answer and prints each leg's ledger, its seven GST and HSN/SAC fields (`GSTSOURCETYPE`, `GSTLEDGERSOURCE`, `HSNSOURCETYPE`, `HSNLEDGERSOURCE`, `GSTHSNNAME`, `GSTOVRDNTAXABILITY`, `GSTOVRDNTYPEOFSUPPLY`) and its count of rate rows, then the leg's lines whole. With `--diff OLD NEW` it prints both sets of summary lines, then a line diff leg by leg.
- It records; it never decides, and it exits 0 whatever the legs hold.
- It STOPs (exit 1) when an answer is empty, does not decode, holds no `BODY/DATA/COLLECTION`, holds other than one voucher row, or holds no ledger entry. A row opens with attributes, so the bare `<VOUCHER>` counter in the answer's `DESC/CMPINFO` block is not counted.
- Output is bounded: 1,500 lines of leg text or 400 lines of diff a call, with the count not shown (AGENTS.md P5).
- It was tried on RB's committed answers. `--diff a-s2-R-0010.xml a-s5r-R-0105.xml` printed legs 1 and 2 `DIFFER, 55 line(s)` and the other four `EQUAL` (148 lines in all). `--diff a-s5r-R-0105.xml a-s7r-R-0107.xml` printed six `EQUAL`. RC-0105 alone printed 1,171 lines, nothing cut. An import answer, a 0-row N answer and a 1-row N answer each printed STOP.

**What steps 2 to 5 compare with: RB's answers.** Before step 1, copy each one below beside this row's answers as `a-rb-<RB's name>`, checking its size and sha256 first. The committed ones come from `lab/2464-rb-answers` at afe4fbad (`answers/`). The lab-PC ones come from RB's folder on the lab PC, as its `kept-local.txt` lists them. If any is missing or its sha256 differs: STOP before step 1.

| RB's answer | Bytes | sha256 | Where |
| --- | --- | --- | --- |
| `a-s1-M-marks.xml` | 3,394 | `8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6` | committed |
| `a-s2-M-marks.xml` | 3,394 | `8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6` | committed |
| `a-s2-T-types.xml` | 306,946 | `cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed` | committed |
| `a-s2-N-0105.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` | committed |
| `a-s2-R-0010.xml` | 94,474 | `b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5` | committed |
| `a-s2-U-taxunits.xml` | 22,636 | `e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989` | lab PC |
| `a-s2-RC-0010.xml` | 94,744 | `5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399` | lab PC |
| `a-s2-V-0010.xml` | 52,784 | `4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456` | lab PC |
| `a-s3-T-types.xml` | 306,946 | `3239e41dfed99766ac1ee6da5724eea72ca206f55113d729bb0c998d56402425` | committed |
| `a-s3-U-taxunits.xml` | 33,304 | `2ec479c6d234d5d5d130a362268c042adf35a05a6993d6c09ad71071519196f1` | lab PC |
| `a-s4-T-types.xml` | 307,038 | `eef7b1cc48eecd1fbc654b94379015dd87820579331d0b8cbf3222ffe2cf5987` | committed |
| `a-s4-U-taxunits.xml` | 33,304 | `508dd8de3eb34f011a48debf7e86154657b129efc570d4ccd6a6ec5d96143621` | lab PC |
| `a-s5-I-0105.xml` | 540 | `fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea` | committed |
| `a-s5r-N-0105.xml` | 5,606 | `23929638cd0c6b653435a33a9903f8e5e15b2efb481d0a5d7f71fb5a4792a2ff` | committed |
| `a-s5r-R-0105.xml` | 87,968 | `56e3fff1e5c6b0d7594e7c863bb6e795f66eff7ae0e7e49dfe3df7e26146d075` | committed |
| `a-s5r-RC-0105.xml` | 88,208 | `5e4b0537c33d4d87ed53e37cee5162db1db7ccbe6eea41cf30d0f2c89a4f3265` | committed |
| `a-s5r-V-0105.xml` | 52,630 | `81ede927c72c679406a5197ce6a8b41a199b8ecf479fd49affb2cbe1b5d8e94a` | lab PC |
| `a-s5r-V-0010.xml` | 52,784 | `456ddfbe553487a466c405e963e4f094e50507a8fa4f2665c0be7a7a30893a0e` | lab PC |
| `a-s5r-M-marks.xml` | 3,394 | `eaa698ccbf94cb0145d6f3c656c6672f9bfa0631020026d175bc33a1df9d3aad` | committed |

`SHA256.txt` lists every file in this folder with its size. Requests are UTF-16LE with a BOM, as sent.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count19.py a-<label>-*`. A STOP from any script stops the row there.

**The prompt rule, at every step.** The only prompts answered are C5's, C6's and step 3's three named prompts (RB's), each with the key named. Step 6's save names none: row 2162-RS's save on this book raised none.
- For any other prompt, message or dialog: copy it word for word and screenshot it first.
- Then answer No, or Esc if it offers no No. Press nothing more and send nothing.
- Then leave any screen still open by Ctrl+Q (C7), restore BK by C6, compare as step 8, and close (C10).
- That prompt is this row's result.

**The wait rule.** After each Ctrl+A, and after the import's send: if 5 minutes pass with no list, no Gateway, no answer and no message:
- send nothing, press nothing, screenshot the whole window, and report to the run coordinator;
- a read sent while Tally is busy can hold its gateway.
- With the Gateway showing and no dialog, run C2's status check (`GET /status` 200, one tally.exe owning 9001) before the next read.

**The path rule.** Every screen in steps 3 and 4 is as RB recorded it (`lab/2464-rb-answers` at afe4fbad, RESULTS.md and `evidence/notes.txt`), and every screen in step 6 is as row 2162-RS recorded its step 5 (`lab/2162-rs-answers` at 8c24fe17, RESULTS.md).
- If the line or screen shown is not exactly what the step names: stop, then Esc out of a list, a report or a prompt. Never press Esc on a creation or alteration screen, voucher or master: leave it by Ctrl+Q (C7).
- Restore BK by C6, check as step 8, then Quit (C10).

1. **Load, check and back up.**
   - Check the `a-rb-` copies (Files).
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down. RB read 1-Apr-26 to 31-Mar-27 and Tuesday 15-Sep-2026.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245, and `count19.py --same-row a-rb-a-s1-M-marks.xml a-s1-M-marks.xml` passes.
     - Other marks: stop here, before BK, with nothing keyed.
   - **Take BK** by C5, to the new empty folder `C:\lab\2464-rs\backup-1`. RB's step 1 is the path. Tally may show the folder with its own capital. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `U-taxunits.xml T-types.xml N-0105.xml R-0010.xml RC-0010.xml V-0010.xml M-marks.xml`.
   - EXPECT each answer byte-equal to RB's s2 answer of the same kind (`a-rb-a-s2-*`). So: two units, `Rajasthan Registration` the one GST unit; `BRIDGE Sales` unbound; 0 rows for `BP/26-27/0105`; `BP/26-27/0010` as RB read it.
   - **If any answer differs:** compare it and record the whole output (`--unit-diff`, `--type-diff` or `--diff` against the `a-rb-` copy; `--same-row` for M; `count19.py`'s own lines for N).
     - Then stop before step 3: nothing has been sent but reads and nothing keyed but BK, so nothing is restored. Close by C10.
     - One row or more in N-0105 is such a difference. Record each row whole.
3. **Change 1: a second GST registration.** RB's step 3 (GN's step 3, items 1 to 7), key for key and prompt for prompt, with the same three named prompts answered `c`, `y` and `n`, and the same GSTIN typed (`27ZZZZZ0000Z1ZQ`, in GN's two parts, zoomed after each).
   - Before it, `s3m`: `M-marks.xml`, then `count19.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. It must pass. Then no key is pressed in Tally but this step's.
   - Then `s3`: `U-taxunits.xml T-types.xml M-marks.xml`.
     - `count19.py --unit-diff a-rb-a-s3-U-taxunits.xml a-s3-U-taxunits.xml`. EXPECT the names `Default Tax Unit`, `Maharashtra Registration` and `Rajasthan Registration` on both sides, and 0 leaves differ in each.
     - `count19.py --type-diff a-rb-a-s3-T-types.xml a-s3-T-types.xml`. EXPECT every line printed reads 0 leaves differ, and no type in one only.
     - Record both outputs whole, and ALTMSTID (RB: 246).
     - **Any leaf that differs from RB's stops the row here:** press nothing more, restore BK by C6, check as step 8, and close.
4. **Change 2: bind BRIDGE Sales to `Rajasthan Registration`.** RB's step 4, items 1 to 4, key for key: Go To, `Alter Master`, `Voucher Type`, Enter on `BRIDGE Sales`; write down every field of `Voucher Type Alteration` (EXPECT RB's, `Default GST Registration` reading `Any`); click that value; Down twice to `Rajasthan Registration` (zoomed); Enter; press no key on the `Use common voucher numbering series` list that opens; Ctrl+A (EXPECT no prompt, the List of Voucher Types); Ctrl+Q once to the Gateway.
   - Before it, `s4m`: `M-marks.xml`; record ALTMSTID against s3's.
   - Then `s4`: `T-types.xml U-taxunits.xml M-marks.xml`. Record ALTMSTID (RB: 248).
   - **STOP before the import unless both of these hold:**
     - `count19.py --type-diff a-rb-a-s4-T-types.xml a-s4-T-types.xml`: EXPECT every line printed reads 0 leaves differ, and no type in one only. (RB's own s3 to s4 compare moved `BRIDGE Sales`' `ALTERID` 212 to 248 and `DEFAULTGSTREGISTRATION` from empty to `Rajasthan Registration`, and nothing else.)
     - `count19.py --unit-diff a-rb-a-s4-U-taxunits.xml a-s4-U-taxunits.xml`: EXPECT the same three names and 0 leaves differ in each.
   - Record both outputs whole. If either shows anything else: send nothing, press nothing more, restore BK by C6, check as step 8, and close.
5. **Import A'' (the C3 exception), and the invoice before the save.**
   - `s5m`: `M-marks.xml`, then `count19.py --before-write a-s1-M-marks.xml a-s5m-M-marks.xml`. It must pass. Then no key is pressed in Tally before the send.
   - `s5`: `I-0105.xml`, sent once. Then the wait rule.
     - Record every counter and any LINEERROR, word for word. EXPECT byte-equal to RB's `a-s5-I-0105.xml` (540 B, `CREATED` 1).
     - The window after it (shot): the Gateway, no dialog. A dialog: the prompt rule.
   - `s5r`: `N-0105.xml R-0105.xml RC-0105.xml V-0105.xml V-0010.xml M-marks.xml`.
     - EXPECT each byte-equal to RB's s5r answer of the same kind. Where one is not, record `count19.py --diff` (N: `count19.py`'s lines) against the `a-rb-a-s5r-` copy, whole. A difference in the `DESC/CMPINFO` count lines alone is not a field (LAB-COMMON C4).
     - `python3 -I legs.py a-s5r-R-0105.xml` and `python3 -I legs.py a-s5r-RC-0105.xml`, and record both outputs whole.
     - Record R-0105's three status flags and `ISGSTOVERRIDDEN`; V-0105's `GSTREGISTRATION`, `CMPGSTIN`, `CMPGSTSTATE`, `CMPGSTREGISTRATIONTYPE`, `PLACEOFSUPPLY`, `STATENAME`, `ALTERID` and `MASTERID`; and ALTVCHID and ALTMSTID (RB: 69 and 248).
   - **Go on to step 6 only if** `s5` read `CREATED` 1, `ERRORS` 0, `EXCEPTIONS` 0, N-0105 holds one row, and no `--diff` against RB's s5r printed a field or a list that differs. Then the invoice before the save is RB's A'', uncertain with its sales lines empty.
   - **Otherwise** record it and go to step 8. Nothing is saved on screen.
   - `s5b`, 60 s later with no key pressed: `R-0105.xml RC-0105.xml V-0105.xml M-marks.xml`, the control. EXPECT each `count19.py --diff` against s5r's prints no field and no list. A difference is recorded whole and stops the row before the save: go to step 8.
   - Never send `I-0105.xml` again (C3).
6. **The save, on Tally's screen, with no change.** Row 2162-RS's step 5, with this row's voucher. No request is sent during this step.
   1. At the Gateway (shot): Go To, by clicking the `G: Go To` label. Type `Day Book`. EXPECT Day Book offered under Common Reports and under Voucher Reports. Enter takes the first. EXPECT the Day Book `For 15-Sep-26` (shot).
   2. Alt+F2: `Change Period`. From `1-8-2026`, Enter. To `1-8-2026`, Enter. F2 is not pressed. EXPECT the Day Book `For 1-Aug-26` (shot), 12 lines, each 1-Aug-26, BRIDGE Walk-in, BRIDGE Sales: row 2162-RS's twelve with `BP/26-27/0105` (1,051.00) where its `BP/26-27/0057` was. That is BP/26-27/0001 to 0009, 0011, 0010 and 0105. Write the lines down as text.
      - Any other list: the path rule.
   3. Down once a line until `BP/26-27/0105` is highlighted (2162-RS: Down 11 times). Zoom and check its number reads `BP/26-27/0105` (shot).
   4. Enter. EXPECT `Accounting Voucher Alteration (Secondary)`: BRIDGE PILOT LAB; voucher type BRIDGE Sales; No. BP/26-27/0105; 1-Aug-26, Saturday; Party A/c name BRIDGE Walk-in; Particulars BRIDGE Svc 998313 5% 600.40, BRIDGE Svc 998314 5% 400.40, BRIDGE CGST 2.5% 25.02, BRIDGE SGST 2.5% 25.02, BRIDGE Round Off 0.16; total 1,051.00 (shot).
      - Write down every field the screen shows, as text, word for word, with any GST registration, tax or HSN/SAC line it shows. With two registrations the screen may show a field row 2162-RS's did not. Such a field is recorded and not touched.
      - If the screen is not that voucher, or a list or a sub-screen is open: the path rule (Ctrl+Q, never Esc).
   5. **Nothing is changed.** No field is clicked or typed in. Ctrl+A once. EXPECT no prompt and no message, as row 2162-RS's. Any prompt or message: the prompt rule.
   6. EXPECT the Day Book `For 1-Aug-26`, the same 12 lines, `0105` highlighted (shot).
   7. Esc once on the Day Book: the Gateway (shot).
7. **The invoice after the save.**
   - `s6`: `N-0105.xml R-0105.xml RC-0105.xml V-0105.xml V-0010.xml M-marks.xml`. Then, recording every output whole:
     - **The status:** R-0105's three status flags and `ISGSTOVERRIDDEN`.
     - **The lines:** `python3 -I legs.py --diff a-s5b-R-0105.xml a-s6-R-0105.xml` and `python3 -I legs.py --diff a-s5b-RC-0105.xml a-s6-RC-0105.xml`, then `python3 -I legs.py a-s6-R-0105.xml`.
     - **The lines against `BP/26-27/0010`'s:** `python3 -I legs.py --diff a-s2-R-0010.xml a-s6-R-0105.xml`.
     - **The fields:** `count19.py --diff a-s5b-R-0105.xml a-s6-R-0105.xml`, `--diff a-s5b-RC-0105.xml a-s6-RC-0105.xml` and `--diff a-s5b-V-0105.xml a-s6-V-0105.xml`. Record V-0105's six registration and state fields, `ALTERID` and `MASTERID`, as step 5 did.
     - **The registration in RC:** RC-0105's `GSTREGISTRATION` (and any attribute) and `CMPGSTIN`.
     - **Nothing else:** N-0105 holds one row (no second voucher numbered `0105`). `count19.py --diff a-s2-V-0010.xml a-s6-V-0010.xml`: record whether `0010` moved. ALTVCHID and ALTMSTID against s5b's.
   - `s6b`, 60 s later with no key pressed: `R-0105.xml RC-0105.xml V-0105.xml M-marks.xml`. EXPECT each `count19.py --diff` against s6's prints no field and no list. Record whole.
8. **Restore** BK by C6, with `restore-plan.txt` written first. RB's step 8 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace).
   - Then `s8`: `M-marks.xml U-taxunits.xml T-types.xml N-0105.xml R-0010.xml RC-0010.xml V-0010.xml`, and compare whole:
     - `count19.py --same-row a-s1-M-marks.xml a-s8-M-marks.xml`: EXPECT it passes.
     - `count19.py --unit-diff a-s2-U-taxunits.xml a-s8-U-taxunits.xml`: EXPECT the two units and 0 leaves differ in each.
     - `count19.py --type-diff a-s2-T-types.xml a-s8-T-types.xml`: EXPECT every line printed reads 0 leaves differ.
     - N-0105: 0 rows.
     - `count19.py --diff a-s2-R-0010.xml a-s8-R-0010.xml` and `--diff a-s2-RC-0010.xml a-s8-RC-0010.xml`: EXPECT only `lists compared` and `legs: 6 -> 6`.
     - Record each answer's sha256 against s2's. EXPECT each equal (RB's s8 was).
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
9. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule, step 3 and C6 name, or if the gateway stops answering.
- Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.
- Say in the report what the book then holds:
  - a stop between steps 3 and 8 leaves the second registration;
  - from step 4, also the binding;
  - from step 5, also A'';
  - from step 6, also whatever the save did.

## Record
- Each answer's bytes and sha256, and `count19.py`'s and `legs.py`'s lines, whole.
- The `--unit-diff`, `--type-diff`, `--diff`, `--before-write` and `--same-row` outputs, whole.
- The screenshots of steps 3, 4 and 6, and after the import's send, listed by name, bytes and sha256.
- As text and word for word: every key pressed, and every prompt, message and busy screen, with the key that answered it and its time (UTC).
- Every field of `GST Registration Creation`, `Voucher Type Alteration` and `Accounting Voucher Alteration (Secondary)` as Tally showed it, as text, and the Day Book's lines.
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **s2 to s5r:** byte-equal to RB's answers of the same step, as the Book says.
- **The save:** no prompt, the invoice's GUID and MASTERID kept, its ALTERID 69 to 70, ALTVCHID up by one, and `VATEXPAMOUNT` and `RATEOFINVOICETAX` filled on the legs, as row 2162-RS's save moved them on the one-registration book. Not measured on this book.
- **The status and the sales lines:** not estimated. `BP/26-27/0021` was filled and included by a save, but on the one-registration book and after a ledger change. On this book, with two registrations, Tally did not fill the lines at import even with the unit named, and whether a save does is the question.
- **s8:** equal to s2 and s1.

## Decides
Each is PARTIAL: one save, one invoice, one sitting, one book, one release.

- **After the save, R-0105 reads included `Yes`, uncertain `No`, excluded `No`, and both sales lines hold their GST and HSN/SAC fields and rate rows as `BP/26-27/0010`'s do:** on this book a save on screen, with no change, fills the lines and clears the status. The status and the lines move together again, and the import alone leaves both. It does not say which causes which.
  - For the build: an invoice it imported here would be put right by the user saving it once in Tally, and the read-back after that save would read included. Whether an import can carry what the save fills is the next variable, and the request shape for it is unmeasured.
- **The lines are filled, but R-0105 still reads uncertain:** filled lines are not enough to clear the status on this book. Record which of V-0105's registration fields moved, `CMPGSTSTATE` and `CMPGSTREGISTRATIONTYPE` above all, and the coordinator decides the next variable.
- **R-0105 reads included, but the lines are still empty:** the status does not need the lines filled. The five imports' correlation then does not hold as a cause. Record whole; the coordinator decides.
- **Neither the lines nor the status moved** (with the ALTERID moved): a save on screen does not work these out again on this book. The cause is elsewhere in the two-registration set-up. Record whole; the coordinator decides.
- **The ALTERID did not move:** the save did not take. Record the screens; the coordinator decides.
- **The save raised a prompt or a message:** under the prompt rule it was answered No and nothing was saved. Its words are this row's result.
- **The registration fields:** record any of V-0105's `GSTREGISTRATION`, `CMPGSTIN`, `CMPGSTSTATE` or `CMPGSTREGISTRATIONTYPE` the save filled or changed, and whether RC-0105 still equals V-0105 on the two it fetches. That is the candidate read-back on a saved two-registration invoice.
- **V-0010 moved:** the save reached another invoice. Record which field, and report before the restore.

**What these answers are for.** They may inform #2464's design, the build's message for an invoice that reads uncertain, and a candidate builder. They are never a regression fixture for the build's output, because the build renders neither the import nor the save.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build refuses this book before any invoice request.** With two GST units in force, `parse_company_registration` (`src-tauri/src/agent_import_invoice_wire.rs:800`) takes its ambiguous arm (`:804`) and refuses as `invoice_company_registration_ambiguous`, bound or not. So the build sends no import here. This row measures what a candidate builder for this book would rely on, before it is built.
- **The build writes each line as a ledger, a side and an amount.** `render_sales_invoice_xml` (`src-tauri/src/agent_import_invoice.rs:2664`) writes each leg as `LEDGERNAME`, `ISDEEMEDPOSITIVE`, `AMOUNT` and a bill allocation (`:2686`), and no GST or HSN/SAC detail. So the lines' GST details are what Tally fills, at import or at a save. A'' writes its lines the same way.
- **The build's read-back judges both.** `invoice_readback_differences` (`src-tauri/src/agent_import_invoice.rs:1396`) names a sales line with no HSN/SAC as `invoice_line_hsn_unresolved` (`:1503`), and `gst_status_differences` (`:1649`) names `VCHGSTSTATUSISUNCERTAIN` Yes as `gst_status_uncertain`. RB's A'' gave both. Its FETCH (`src-tauri/src/agent_import_invoice_wire.rs:514`) is `R-0105.xml`'s shape, and `RC-` adds `GSTREGISTRATION` and `CMPGSTIN`.

## Later rows
- **If the save fills the lines and clears the status:** the import with the lines' GST details written in the request, one detail at a time, on this book. That row's request is a new shape and starts as a candidate.
- **If the save fills the lines and not the status:** B'' with `CMPGSTSTATE` and `CMPGSTREGISTRATIONTYPE` added, as `BP/26-27/0010` stores them.
- **Whatever it answers:** GSTR-1 on Tally's screen for the saved invoice; an unbound type on this book.

## Not measured
- B'', or any import other than A''.
- A save with any change, a save from another screen, or a second save.
- GSTR-1 on Tally's screen.
- `CMPGSTSTATE`, `CMPGSTREGISTRATIONTYPE` or a line's GST details written by a request.
- An invoice naming a unit other than its type's binding, an unbound type on this book, and an inter-state invoice.
- An invoice built, posted or saved through Bridge.
- Any other book or release.
