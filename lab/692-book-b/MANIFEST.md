# Book B answers for #692 (kept as received, 9 October 2026)

Synthetic company BRIDGE TIEOUT B (TallyPrime 7.1 Silver, Windows 10, the only company loaded). Each file is the published 0.5.1 `bridge_mcp` JSON-RPC response to one tool call, byte for byte as it was received over stdio (#692 comment 6083129100). Tally's own XML bytes are not in these files; each answer's `evidence` block records the size and SHA-256 of Tally's response. The Profit and Loss answers carry Tally's `Less: Closing Stock` line.

| File | Tool call | Bytes | SHA-256 |
| --- | --- | --- | --- |
| `05-pl-fy.json` | profit_and_loss, full year | 10540 | `78a1796149668921a26f9089b82a7a9291a82b6f02840185f4b94d9c93d41871` |
| `06-bs-fy.json` | balance_sheet, full year | 8194 | `686c181ab1c50a8af334809bce55020f304ef89b183f637b3a17272a7ce8fcff` |
| `07-pl-h1.json` | profit_and_loss, H1 | 10540 | `a8d8b9167b7770c4e997526b47ea65c14f7c22e117a690b138b338b66b6433c5` |
| `08-bs-h1.json` | balance_sheet, H1 | 8194 | `aa34b22c3609df66b14a8ec55e3ec207417b800265a53af4ccecfd4e0e3b8e64` |
| `09-stock_summary.json` | stock_summary | 10494 | `dc91bfbce4190f2b7128d40c3273d9813f1ca413d8f69284ea672019610330b3` |
