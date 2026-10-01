# Purchase register scripted-transport fixtures

These ten files are the responses Tally sent, in order, to one live `purchase_register` call
(bridge_mcp built from the register branch at 7edab62a, writes not
compiled) on the disposable BRIDGE GST RECON LAB book, TallyPrime 7.1 Silver, on 2026-10-01,
for the single day 2025-09-03. A serial read-only relay sat between the binary and the gateway
and recorded the bytes; 102 requests were relayed, none refused. The order in which they were sent and each request's
SHA-256 are in `agent_register_server_tests.rs` (`RECORDED_ORDER`, `recorded_request_sha256`).
The body of each response is kept exactly as received (UTF-16LE without a BOM, the status probe
UTF-8), except the three company-listing responses, where every `COMPANY` row but the target
book's was removed: the untrimmed responses name every company loaded in that Tally. The
data is synthetic (SYN names); the book, the vouchers and the ledgers were created for this
work and read back through Bridge. One instance, one day, one voucher: they back the order and
shape of Bridge's reads and the refusals that follow from them, not any claim about other books.

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `native-register-e2e-extent.utf16le.xml` | 4,016 | `dc7173ec87e522f53706d381e25a43ae744ddc32229d04cd7783ae995a61a919` | the company list read (BridgeCompanyExtent); trimmed: all 31 COMPANY rows but the target book's removed (1 of 31 kept) |
| `native-register-e2e-status.txt` | 51 | `655415972de8e54d65743a548e00c2218810683fc0c4cab76cf6ea97ff1d3800` | the gateway status probe body; byte-exact |
| `native-register-e2e-book-extent.utf16le.xml` | 3,862 | `5d263c2e81a5544ef9519f36d27065aa07fdfc2c51c2ccb69b7ca8c5931fec09` | the company book extent read (BridgeCompanyBookExtentV2); trimmed: all 31 COMPANY rows but the target book's removed (1 of 31 kept) |
| `native-register-e2e-currencies.utf16le.xml` | 3,414 | `b2ee7d1d54f6ac805f888752c80287131592f6b902301de586d5cca7803ef656` | the company currencies read; byte-exact |
| `native-register-e2e-ledgers-compliance.utf16le.xml` | 86,670 | `e9bf94bd42d4ec82c09cd44d1293432ff340ee2bd24c4fee03a885e5b828c94e` | the compliance ledger listing (List of Ledgers with TAXTYPE and GSTDUTYHEAD); byte-exact |
| `native-register-e2e-ledgers-paired.utf16le.xml` | 56,672 | `a3a2387aa8a77777397ed366ccd56d3a3539a73b042fe32d87efbe2ac8d7ecc3` | the paired ledger listing (List of Ledgers, opening balances); byte-exact |
| `native-register-e2e-groups.utf16le.xml` | 54,290 | `00d5f32623e01a53199bc27758777b132122a139037f88cc0275da06f07c3b2b` | the group collection (List of Groups); byte-exact |
| `native-register-e2e-census.utf16le.xml` | 5,456 | `68cceb2bac21c2763175caea0334ee3292a9e1fd2bdb920031a831624baeb27a` | the voucher census for 2025-09-03; byte-exact |
| `native-register-e2e-window.utf16le.xml` | 64,026 | `6388f8a27853aae18d34f2d0bf7c7f05bd282d3bc5cccd8ef31494bf7a147ba1` | the class-shaped voucher window for 2025-09-03 (one Purchase voucher); byte-exact |
| `native-register-e2e-marks.utf16le.xml` | 3,412 | `275a54131269a18b717c8d9fa93b8484b5a76977808b03172f279c815467c09f` | the company marks read (Bridge Agent Company High Water); trimmed: all 31 COMPANY rows but the target book's removed (1 of 31 kept) |
