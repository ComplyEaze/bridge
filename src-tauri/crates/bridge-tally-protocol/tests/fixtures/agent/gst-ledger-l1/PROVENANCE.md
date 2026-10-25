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
