# MCPB assembly

Fetch the pinned PDFium build, then build the host `bridge_mcp` binary and assemble an MCPB archive:

```sh
python3 scripts/fetch-pdfium.py --platform macos-arm64 --dest /tmp/mcpb-pdfium   # windows-x64 on Windows
node scripts/package-mcpb.mjs --pdfium /tmp/mcpb-pdfium
npx --yes @anthropic-ai/mcpb@2.1.2 validate packaging/mcpb/stage/manifest.json
npx --yes @anthropic-ai/mcpb@2.1.2 pack packaging/mcpb/stage src-tauri/target/release/bridge-tally.mcpb
npx --yes @anthropic-ai/mcpb@2.1.2 info src-tauri/target/release/bridge-tally.mcpb
python3 scripts/check-mcpb-bundle.py src-tauri/target/release/bridge-tally.mcpb
```

When a reviewed release binary is already available, stage that binary without
building Rust again. This is useful for archive-only verification on the same
host architecture:

```sh
node scripts/package-mcpb.mjs --binary /path/to/bridge_mcp --pdfium /tmp/mcpb-pdfium
```

Staging refuses to run without `--pdfium`: the manifest always enables imports, so every archive lists `parse_bank_statement`, and without the library every call would fail. `fetch-pdfium.py` checks the archive, the extracted library and the licence notice it writes against `packaging/pdfium/pdfium.lock.json` before anything is used.

The committed `manifest.json` is a schema-valid template; it is not an archive manifest. The command builds the host `bridge_mcp` release binary with the locked dependencies, then replaces `packaging/mcpb/stage/` with a clean host-specific stage. Its binary is staged at `bin/<target-triple>/bridge_mcp` (or `.exe` on Windows), alongside `LICENSE`, `NOTICE`, `THIRD_PARTY_LICENSES.txt`, and `THIRD_PARTY_LICENSES_RUST.txt`. The PDFium library is staged beside the binary (`bin/<target-triple>/libpdfium.dylib` or `pdfium.dll`), where `parse_bank_statement` loads it, with `THIRD_PARTY_LICENSES_PDFIUM.txt` at the root: every licence file the pinned PDFium archive ships, byte for byte.

The generated manifest uses the [official MCPB schema](https://github.com/anthropics/mcpb/blob/main/MANIFEST.md): a string entry point, an explicit launch command, environment substitutions for user settings, and an operating-system compatibility declaration. The manifest is MCPB 0.2, which adds `privacy_policies` and has no architecture compatibility field. The configuration UI asks first for **I accept the ComplyEaze Bridge Terms of Use** (off by default and deliberately not `required`, see below; the server refuses every tool until it is on, and `scripts/check-mcpb-bundle.py` refuses a manifest that drops the setting, defaults it on or marks any setting `required`), then keeps the local Tally host, numeric HTTP port (default `9000`), response redaction, and **Allow voucher posting (Journal, Payment, Receipt, Contra)**. Posting is **off by default** while the limits recorded on bridge#574 and bridge#579 remain (both issues are closed; what remains is stated on them); turning it on adds `post_import` and `acknowledge_post_review` (recording, in its own native dialog, that you reviewed a post whose ledgers changed master; it writes nothing to Tally), and every new posting still requires native approval. The manifest maps `BRIDGE_AGENT_ENABLE_IMPORT` to a constant `true`, so file preparation and bank-statement parsing are available whatever the setting; `scripts/check-mcpb-bundle.py` refuses a manifest that defaults posting on or maps imports to anything else.

No setting is marked `required` (bridge#1413). Claude Desktop builds no launch command for an extension while a required setting has no stored value: the MCPB library's `getMcpConfigForManifest` (`src/shared/config.ts`, `@anthropic-ai/mcpb` 2.1.2) checks required settings against the stored values before it fills in the defaults, and Claude Desktop 2.26454.2 on macOS carries the same code and then logs "No MCP config found for extension". An update keeps only the stored settings the new manifest still declares, so when new Terms rename the setting, the update leaves it with no value. With 0.5.0, which marked it required, that was measured on one Mac: installed over 0.4.2 and switched on, the extension was never started, and nothing on screen named the Terms. Not required, the absent value takes its default `false`, Claude Desktop starts the server, and every tool answers `terms_not_accepted`. Windows has not been checked.

Build and distribute separate archives for Windows x64 and macOS arm64. Intel macOS is not currently qualified, so do not label an arm64 archive as universal macOS support. Identify the architecture in each distributed filename and select the archive matching the client host.

The extension icon is `packaging/mcpb/icon.png`, staged at the archive root as `icon.png` and named by the manifest's `icon`; the packaging step copies it byte for byte, the stage verifier requires a PNG, and the smoke check requires the archive's copy to equal the committed file. The verifier requires a matching binary, launch command, operating-system declaration, legal resources, and the PDFium library and notice; the smoke check additionally requires both to match the SHA-256 pinned for the host platform. The official CLI validates the complete manifest and creates the archive from the host-specific `stage/` directory. The binary, archive, and staged resources are ignored local build outputs and must not be committed. Run the commands independently on each supported host. CI packages Windows and the hosted macOS runner architecture, then extracts and launches each actual archive with a temporary data directory and loopback port 9. The bounded smoke checks initialization, the default tool catalog, the local voucher schema, and its egress receipt, and parses the repository's synthetic encrypted statement through `parse_bank_statement`, which proves the bundled PDFium loads from beside the binary; it never requests Tally data. Archive and binary hashes plus payload-free verification counts are retained in `mcpb-smoke.json` alongside the archive. This establishes packaged stdio execution on the runner; desktop-client installation and live Tally interoperability remain separate checks. Use `python` instead of `python3` for the local command on Windows.

CI smoke artifacts expire after seven days. The manual preview-release workflow instead publishes immutable, explicitly unsigned GitHub prereleases with the actual archive, checksum, source-provenance record, and payload-free smoke result. It has no production-signed channel: raw MCPB archives are not a notarization-and-stapling carrier for the macOS binary, so a production channel needs a separately reviewed signed distribution design and protected credentials.
