# Provenance: 10 Oct 2026 statement tie-out files (#1215)

This folder holds two files, copied byte for byte from a Windows lab PC. Their hashes are unchanged.

| File | Original path | Bytes | SHA-256 |
|---|---|---|---|
| `bridge-21f8cfe4-67a1-44d2-8f7e-77fd6619db13.xml` (the import file) | `C:\bridge-lab\1215\imports\bridge-21f8cfe4-67a1-44d2-8f7e-77fd6619db13.xml` | 5,106 | 89b59f16215dc9d69ca4774a8e3f0e50b81e367d1b197e98a122755041866be8 |
| `bridge-21f8cfe4-67a1-44d2-8f7e-77fd6619db13.proof.json` (the batch's proof) | `C:\bridge-lab\1215\imports\bridge-21f8cfe4-67a1-44d2-8f7e-77fd6619db13.proof.json` | 4,008 | a8602f9947db91a9f9a3a0faf7d2a575a6d87550447c9f06c216a457a1aa7c30 |

`C:\bridge-lab\1215` is a ComplyEaze Bridge data folder. It was written on 10 Oct 2026 between 04:56Z and 05:09Z. No file in it lies under a user folder (`<user>`).

## Who ran it and with what
- The 10 Oct steps were run by an outside contributor, as stated in the pull request's public comment.
- Build: `bridge_mcp-rel-df7678d0.exe`, sha256 `91dd2751962c36e331f1d4952f7c5d94d197d68361c31907499c3c1933671004`. The executable is not copied.

## Checks made before copying (parsers, not text search)
- **Company:**
  - The import file's `SVCURRENTCOMPANY` is `BRIDGE TIE OUT LAB 1`.
  - The proof's `company.name` is `BRIDGE TIE OUT LAB 1`, and its `company.guid` is `ead80089-9c57-4d75-aaff-7f65bef58ee1` (company number 100045).
  - Every `company_guid` in `agent-egress.jsonl` is one of the two synthetic companies: `ead80089-9c57-4d75-aaff-7f65bef58ee1` on 7 lines and `f170ca12-abb6-4f6c-a4f2-c89e0dfa994f` (BRIDGE TIE OUT LAB 2) on 2 lines.
  - The GUIDs inside the import file are the six vouchers' remote ids, not company GUIDs.
- **Content:** the import file holds six vouchers on Union Bank / Suspense. Each matches the repository's synthetic statement `ubi-synthetic.pdf` (sha256 `6694b20b393c09b53cc7359226c69998adf90d9e731e1413fbfd310733d9098b`):

  | Date | Type | Amount | Party in narration |
  |---|---|---|---|
  | 1 Aug 2026 | Receipt | 1500.00 | NORTHWIND TRADERS |
  | 2 Aug 2026 | Payment | 250.50 | BLUE RIVER CO |
  | 3 Aug 2026 | Receipt | 750.50 | CASH DEPOSIT |
  | 4 Aug 2026 | Payment | 13000.00 | GREEN FIELD LTD |
  | 5 Aug 2026 | Receipt | 1000.00 | UNRESOLVED |
  | 6 Aug 2026 | Receipt | 500.00 | SILVER OAK MUTUAL |

  - The parser names the 5 Aug row UNRESOLVED and the cash line CASH DEPOSIT, as the fixture tests expect. The fixture's SOME THREE WORDS row parses as UNRESOLVED, and its BY CASH line becomes CASH DEPOSIT.
  - The proof's six voucher entries carry ids only (GUID, master and alter id, voucher number, transaction id). They have no dates, amounts or parties.

## Kept on the lab PC (not copied)
- **`agent-egress.jsonl`** stays on the lab PC: its `args_sha256` values hash arguments that include local paths.
  - SHA-256 `d9e5fa852ac506d188c327e1ac84814b26df2339b20e6aa419192375dcf6fbaa`, 53,715 bytes, 36 lines.
  - By record type: 18 `response_prepared` and 18 `stdio_write_completed`.
  - Lines per tool name: list_companies 4, parse_bank_statement 5, statement_tie_out 6, build_import_xml 1, verify_import 1, ledger_movement 1. The 18 `stdio_write_completed` lines carry no tool name.
- The import summary was not captured.
