# GST per counter ledger: scripted-transport fixtures (GS1-L1 run-3)

These 36 files are responses Tally sent to `gst_by_counter_ledger` on the synthetic book BRIDGE GST LEDGER LAB, TallyPrime 7.2
Silver, on 26 Oct 2026: the 14 calls of the lab item's step 6, one per voucher day of November 2025, made by `bridge_mcp` built
from the head of pull request 1867 at 796e94f1 (posting off). A serial read-only recording relay sat between the program and the
gateway and recorded every request and answer; 1,657 requests were relayed and none refused. Every voucher and master was typed on
Tally's own screen by a person, one at a time; nothing was imported. Each call sent 102 requests in the order
`agent_gst_by_ledger_server_tests.rs` pins (`RECORDED_ORDER`), with the request fingerprints it pins; every answer other than the
census (n) and the window (w) was byte-equal across the 14 calls, so those are kept once, from the first call. The bodies are kept
exactly as received (UTF-16LE without a BOM; the status probe UTF-8); the company-listing answers held only the lab company's row,
so nothing was trimmed. The book's month call (step 7, 64 vouchers) passed live but its answers (about 4.1 MB) are not kept here.
The data is synthetic (SYN and GS1 names, no GSTIN). One instance, 14 vouchers, one release: they back the shapes and refusals
these vouchers exercise, not any claim about other books.

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `s6-03nov-L-1.utf16le.xml` | 37,742 | `4050fe96d18f996da1c109a3c76212e7d045f817cb41f609bc99e31f726b5172` | the compliance ledger listing (List of Ledgers with TAXTYPE and GSTDUTYHEAD); recording row 27; byte-exact |
| `s6-03nov-P-1.utf16le.xml` | 25,434 | `c4372ae1536d68d652d1efb5dc3de5b20827969bb89e912be20b67859d45ed96` | the paired ledger listing (List of Ledgers, opening balances); recording row 31; byte-exact |
| `s6-03nov-b-1.utf16le.xml` | 3,866 | `4271d60f71fe0d878446296cf3135d58d5a6c00814ddff2fc10056c6258927da` | the company book extent read (BridgeCompanyBookExtentV2); one COMPANY row; recording row 7; byte-exact |
| `s6-03nov-c-1.utf16le.xml` | 3,410 | `2f7f19fe7eaa1f8b98d734dcb598fbf12e04c8c3edc4e394c0b0b1c9e6cdedca` | the company currencies read; recording row 11; byte-exact |
| `s6-03nov-e-1.utf16le.xml` | 4,030 | `c29b8dccbc1a449db4273ed666e8e77a8a0753877b08126da254870c46cd9189` | the company list read (BridgeCompanyExtent); one COMPANY row, the lab company's (no other company was loaded); recording row 2; byte-exact |
| `s6-03nov-g-1.utf16le.xml` | 54,290 | `e65f72894a59defc8bb91a3fe64d3e37d72fe9365aac67eadacd16bbb47dbdfa` | the group collection (List of Groups); recording row 35; byte-exact |
| `s6-03nov-m-1.utf16le.xml` | 3,414 | `5518a0661c7812c67324e4abad94a4315c3487e5c89bf06e537039588a111619` | the company marks read (Bridge Agent Company High Water); one COMPANY row; recording row 59; byte-exact |
| `s6-03nov-n-1.utf16le.xml` | 5,448 | `f0d39e7fa67588da76d66a40363a6ec72e216d92948858e09570f19e53f58113` | the voucher census for 03 Nov 2025 (V1, a Purchase to an Indirect Expenses ledger); recording row 47; byte-exact |
| `s6-03nov-s-1.txt` | 51 | `655415972de8e54d65743a548e00c2218810683fc0c4cab76cf6ea97ff1d3800` | the gateway status probe body (UTF-8); recording row 3; byte-exact |
| `s6-03nov-w-1.utf16le.xml` | 65,516 | `1511d735ee77a3e7e68ba81e5e3a2fe3e9fd58eb1d83507b6b832e6747fbe0ff` | the class-shaped voucher window for 03 Nov 2025 (V1, a Purchase to an Indirect Expenses ledger); recording row 53; byte-exact |
| `s6-04nov-n-1.utf16le.xml` | 5,444 | `d0f5d50a525a1735b342427ac4cfe98dd6f88509350bc24d0b800a0167d5e214` | the voucher census for 04 Nov 2025 (V4, a Journal: an expense with input IGST); recording row 149; byte-exact |
| `s6-04nov-w-1.utf16le.xml` | 52,068 | `63619a18f92c43d65312968864ff1507991eeaaee5875bc53ed964e1b34e03d3` | the class-shaped voucher window for 04 Nov 2025 (V4, a Journal: an expense with input IGST); recording row 155; byte-exact |
| `s6-05nov-n-1.utf16le.xml` | 5,448 | `ff6393088fdbd1c670251c2b1da6d15dce93bff0ecbfd3483270c8ae86f2013c` | the voucher census for 05 Nov 2025 (V5, a Purchase to a Fixed Assets ledger); recording row 251; byte-exact |
| `s6-05nov-w-1.utf16le.xml` | 65,528 | `c7cfd470123ef68de8d44b36e777acbcc2fcea0fb22acb0ea6c055b5b8008fb9` | the class-shaped voucher window for 05 Nov 2025 (V5, a Purchase to a Fixed Assets ledger); recording row 257; byte-exact |
| `s6-06nov-n-1.utf16le.xml` | 5,448 | `4f669c66dda0031f567447088619496d6f5d9be02e234d450b08bf7cc54f018b` | the voucher census for 06 Nov 2025 (V6, a Purchase to a Direct Expenses ledger); recording row 353; byte-exact |
| `s6-06nov-w-1.utf16le.xml` | 65,528 | `0533a439ccb3c6576f546fe611d3a9a62e975bd71d9315c17ec63ad2ce29a140` | the class-shaped voucher window for 06 Nov 2025 (V6, a Purchase to a Direct Expenses ledger); recording row 359; byte-exact |
| `s6-07nov-n-1.utf16le.xml` | 5,450 | `4587e4d498c77fa5a563fe899c6583b1d3037f8ddf77582eee50f68775eea7a0` | the voucher census for 07 Nov 2025 (V7, a Purchase to two expense ledgers); recording row 455; byte-exact |
| `s6-07nov-w-1.utf16le.xml` | 81,492 | `227f09bd2893282421e481d79c48504c96871a94d348e59ea5e71f983820e941` | the class-shaped voucher window for 07 Nov 2025 (V7, a Purchase to two expense ledgers); recording row 461; byte-exact |
| `s6-08nov-n-1.utf16le.xml` | 5,444 | `4395c211e7ebe55f3e754846634be1ace1299a8fd499948df0a6734aa1e0c01a` | the voucher census for 08 Nov 2025 (V4b, a Journal with a TDS ledger typed Others); recording row 557; byte-exact |
| `s6-08nov-w-1.utf16le.xml` | 65,552 | `fc2bf699f246a1b3a5fb6e1562963bf0c16debdec4e0ae763fa5296742f26ddb` | the class-shaped voucher window for 08 Nov 2025 (V4b, a Journal with a TDS ledger typed Others); recording row 563; byte-exact |
| `s6-10nov-n-1.utf16le.xml` | 5,456 | `700d208be69301e70a8d5a71b68d82fd8edcb00592c7343b677e5716d02dd67b` | the voucher census for 10 Nov 2025 (V8, a Debit Note); recording row 659; byte-exact |
| `s6-10nov-w-1.utf16le.xml` | 65,500 | `f0130239483a6fef8fcf0d04ea85c6b01477e7e4ca7d627933382db1a5d19883` | the class-shaped voucher window for 10 Nov 2025 (V8, a Debit Note); recording row 665; byte-exact |
| `s6-11nov-n-1.utf16le.xml` | 5,444 | `946bee1424766c64aa5df4a7fe4259ac04f207c90d803139785b4c6972905d27` | the voucher census for 11 Nov 2025 (V9, a set-off Journal); recording row 761; byte-exact |
| `s6-11nov-w-1.utf16le.xml` | 34,710 | `f4aaecb8bb430634db21a0fce26a2f2d36f8a9a71b30449638cf4fe786cd6a20` | the class-shaped voucher window for 11 Nov 2025 (V9, a set-off Journal); recording row 767; byte-exact |
| `s6-12nov-n-1.utf16le.xml` | 5,446 | `f96c8f32d34379e19d6fb4ef587a8aa8ce703a09eb148207360598e5679a9497` | the voucher census for 12 Nov 2025 (V10, a Journal reversing input tax with no supplier); recording row 863; byte-exact |
| `s6-12nov-w-1.utf16le.xml` | 50,640 | `5dd45fe460107e3303828fba3d4d14b9dd5cf3c380de9f9c36a6451deaa06913` | the class-shaped voucher window for 12 Nov 2025 (V10, a Journal reversing input tax with no supplier); recording row 869; byte-exact |
| `s6-13nov-n-1.utf16le.xml` | 5,448 | `81db84c63f6a675b562e5407d9618c3012a0d6b3564a3752049b85da0e805f4a` | the voucher census for 13 Nov 2025 (V11, a tax Payment); recording row 965; byte-exact |
| `s6-13nov-w-1.utf16le.xml` | 48,190 | `fd4362fec21fbea1f747819964d0666091820bcf551583f23b8d7bb88819536e` | the class-shaped voucher window for 13 Nov 2025 (V11, a tax Payment); recording row 971; byte-exact |
| `s6-14nov-n-1.utf16le.xml` | 5,444 | `fb6df7804b4bbee2680309ce36e2bb691fe348baa0a5cd19e21617dddfbd4cac` | the voucher census for 14 Nov 2025 (V2, an item invoice); recording row 1067; byte-exact |
| `s6-14nov-w-1.utf16le.xml` | 78,626 | `bd1a3c39bf3b51c94b40feee969a9936d1ca971a19941e851845e9cce6e188e4` | the class-shaped voucher window for 14 Nov 2025 (V2, an item invoice); recording row 1073; byte-exact |
| `s6-15nov-n-1.utf16le.xml` | 5,448 | `a75c6fffb5390c3788cf5bf4f07c5cfe79434141ae9c663588222258cfd44c81` | the voucher census for 15 Nov 2025 (V13, a tax Payment that also pays interest); recording row 1169; byte-exact |
| `s6-15nov-w-1.utf16le.xml` | 64,146 | `e8eed3b9558a34fb713cf0675fe44b7d32604416f404c56c1de69d0a10722af8` | the class-shaped voucher window for 15 Nov 2025 (V13, a tax Payment that also pays interest); recording row 1175; byte-exact |
| `s6-16nov-n-1.utf16le.xml` | 5,464 | `8c1a770145779c10d5c103d22c366c661fbc99621abe5702e3669f248b5e6e1f` | the voucher census for 16 Nov 2025 (V14, a Credit Note with output tax); recording row 1271; byte-exact |
| `s6-16nov-w-1.utf16le.xml` | 65,552 | `f196f25421ea9ab593a58ad4d78050c5380d7ece3a207b7113344f115646b272` | the class-shaped voucher window for 16 Nov 2025 (V14, a Credit Note with output tax); recording row 1277; byte-exact |
| `s6-17nov-n-1.utf16le.xml` | 5,444 | `599435920156c7bba90ef2c4fe075932c597278188b1811e8d1340bcddbcf074` | the voucher census for 17 Nov 2025 (V3, an item invoice with freight); recording row 1373; byte-exact |
| `s6-17nov-w-1.utf16le.xml` | 94,594 | `2e72097ac027adfac2dd804f270542085f547af082a8cf2af3c844c9b6cc3a97` | the class-shaped voucher window for 17 Nov 2025 (V3, an item invoice with freight); recording row 1379; byte-exact |

## The census spanned to the mark (#2145)

Since #2145 each call sends its census spanned to the voucher mark it read, `(0, 64]`. The replay pins those requests:
each day's recorded census request with ` AND $AlterID &gt; 0 AND $AlterID &lt;= 64` added to its census formula and nothing
else changed, which the code renders byte for byte (`requests/g-DDnov_c2_spanned_0_64.request.utf16le.xml` on the run sheet
`lab/2145-spanned-census-captures` at `6a968f01`). The lab sent each day's recorded request and its spanned one to the same
book on 30 Oct 2026 (`lab/2145-spanned-census-captures-answers` at `993d96bc`). The comparison removes the
answers' one `CMPINFO` element (the session's object counts); with it removed, both answers equal the recorded census answer
of that day, so the replay keeps the recorded answer as the answer to the spanned request. Confidence: VERIFIED for these 14
days at the mark 64, one sitting.

| Day | Spanned request (sha256, pinned) | Spanned answer (sha256) |
| --- | --- | --- |
| 2025-11-03 | `7bc489839fbf652322e85e4202ecfafb791106d40e650d5d1113a39a8e31cf03` | `29ec24f48bba2de7cbe1ac27a1f14cfb65b40175f1ccd4e812edb84c932dd8a1` |
| 2025-11-04 | `e2e992df45dd2f162a260a072d70040f6604fd754b5502bcda03c84d2d5fc2cd` | `3e099a0a87bf2ae5c7dcd0ee7fe04f4d49860d52896510ac1ceb14ab41f8e8f5` |
| 2025-11-05 | `45edf463087e037063e0dc0a6889d683a8c3d20d9dba6b8831c3c696e927ca10` | `13262ceed37314a18165eb17e524964885cef7f5ccc8be8d48d88643656b521a` |
| 2025-11-06 | `a4b070197b4d1914ddf75665ac3af332510684a97d7dec7fb15169f6953c3714` | `8a1a3c34a08669c42c8fbbf8e7ef86ffb20597e6430cd0eae072fb0a3670307a` |
| 2025-11-07 | `e42684ba34516fc9c72808973ac3a35a2bc77ae07f9191fee692cda2ab369fd2` | `ccb35449de63d592025022748a4b1574d86b30db9b1a8c129264d8861e9f557e` |
| 2025-11-08 | `8a8d29571f09bf75ad788fad9ff84d7c4b5f068591f4b46709648068960bb099` | `39253dce9f705dbe7f8092329017cd7579d9ed81bd615e7315f50e85b460a750` |
| 2025-11-10 | `f411cf5dd1c5f39cfe5b6354405190745170b99f5e2eb48e67ceafaf42c05bc4` | `ff63c8c46c8a820aa7a6958b39d7d80180d57cb3222f9fa1023952b6fa480186` |
| 2025-11-11 | `7eed0576ff4a9fb8ce2db9451915c627e5b93d90cfa2475b0577f74c118f42e1` | `f2c907950db7d4636a0871f9812102ad4d1126580f123f3b08db8bd9d125fd70` |
| 2025-11-12 | `310a0b6378da3a909da18d404f776fb16cdfef787df5c006859c979d1000d5ad` | `2f9708d251d305d46a4144074e39612453be22b47c7daf203be1889aaec24102` |
| 2025-11-13 | `2f5176e406b6ad0d226e8e4ad7629e497c21b0890c1ab4f70bc4fdc043e41218` | `3d27f06f6cf4620a44edca9c4a9eb3e5b10a3ded1b699ebc8be3ca1feb4ea34c` |
| 2025-11-14 | `c6d5a5e68a792b799db65e4ce3dcd81f549952b14e6b343f80e733a7e5e5e5b6` | `72ab4be79713cbf00aa9acdca585a4de756cd87eae6451748b21425b6d8d49fe` |
| 2025-11-15 | `0071e6a2e7fa25369bd8c06238a6e04e8b41b98c53544dc1112a9400fd0c798b` | `34868121421178ca2c3509dd953438ffa8de42fa369f8debdc2abdfce0c448e6` |
| 2025-11-16 | `ba02b3c9ae17f9aa5fc010a3afe3ae8b7d96fdc89782a26fd672ca78b6f5bd4f` | `3df8927e9ca74d05eea7251a14e467301fdf77189afc174f9d0dda3ed86ec816` |
| 2025-11-17 | `78e29ff774397bd4de0bc08072078db92d6512d851c13ee81697b37061a85233` | `d300197d42690382da3b7fbd19698cd23deb8c7a7a26b01e621411ee6e45f1ed` |
