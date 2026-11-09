# Ledger aliases: the three ledger lists of a book whose ledgers carry aliases (1688 lab row)

These 4 files are answers Tally sent, on 30 Oct 2026, to ComplyEaze Bridge's three ledger-list requests: the standard ledger catalogue (V1), the import ledger catalogue (V2) and the ledger name index (V3). They were read on the synthetic book BRIDGE ALIAS LAB, TallyPrime 7.2 Silver, during the lab row of the run sheet `lab/1688-ledger-aliases` at 33a02077. Its answers are on the branch `lab/1688-ledger-aliases-answers` at aa493681 (not for merge).

The requests were the sheet's own copies of Bridge's renderings, with the company name set: `requests/ledger-catalogue.utf16le.xml`, `requests/ledger-catalogue-v2.utf16le.xml` and `requests/ledger-name-index.utf16le.xml` on the sheet's branch. Each was sent by `curl`, one at a time. The bodies are kept exactly as received (UTF-16LE without a BOM).

The book was keyed for the lab. It held five ledgers: Cash, Profit & Loss A/c, Lab Other Ledger, Lab Plain Ledger and Lab Sales.
- **On Tally's ledger alteration screen,** these aliases were then added:
  - `Lab Plain Alias` on Lab Plain Ledger;
  - `Lab Other First` and `Lab Other Second` on Lab Other Ledger;
  - `Lab Café Sales` on Lab Sales.
- **By import,** two ledgers were created under Sundry Debtors, each with one alias in its language list:
  - Lab Alias Shape, alias `Lab Shape Alias`;
  - Lab Alias Script, alias `लैब खाता`.

The data is synthetic. The book was restored from its backup when the row ended.

One sitting, one book, one release: these files back the parse of a ledger's aliases from Tally's own answers, not any claim about other books. Their `CMPINFO` lines are not counts of what was read: the `LEDGER` line reads 5 in the answers taken after the imports, which hold 7 rows.

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `k3-cv2.utf16le.xml` | 9,346 | `fd9314ed146070992a87122d9bddf96a1fa00700497a6f7ae1f8c1b6116cb395` | the import ledger catalogue (V2) after the screen aliases: 5 ledgers, four aliases (the sheet's step K3, `answers/K3-cv2.xml`); byte-exact |
| `i1-cat.utf16le.xml` | 11,158 | `6e0fdfbb72b43e59ebc38ae490c70033826aa18ed27de81233c6d582d8f828a6` | the standard ledger catalogue (V1) after the two imports: 7 ledgers, six aliases (step I1, `answers/I1-reads-cat.xml`); byte-exact |
| `i1-cv2.utf16le.xml` | 11,908 | `38bec50a2716de56368a93619218f9fb6a0795b3c87d1a03a4944a4e74083759` | the import ledger catalogue (V2) at the same point (`answers/I1-reads-cv2.xml`); byte-exact |
| `i1-idx.utf16le.xml` | 12,520 | `d577b6afe35b085142d080502c1140faa06651b7c4bfe8ae361686011b239637` | the ledger name index (V3) at the same point (`answers/I1-reads-idx.xml`); byte-exact |
