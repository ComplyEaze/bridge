ComplyEaze Bridge is still being developed, and this release is not yet
code-signed or notarized, so your computer may warn you before opening it.
Compare each download with its .sha256 file.

It includes an unsigned third-party PDFium shared library
(bblanchon/pdfium-binaries, pinned by SHA-256 in
packaging/pdfium/pdfium.lock.json) beside the server binary. The asset
checksums and payload-free smoke evidence identify the exact archive and
source commit.

What the release check covers: the workflow builds and smoke-tests the
packaged stdio server on hosted Windows x64 and Apple Silicon Mac runners. That
proves the archive launches, exposes its local tool catalog, and parses a
synthetic encrypted bank statement with the bundled PDFium; it does not
establish live Tally behaviour or Claude Desktop conversational tool calls on
either host. What has and has not been run against a real TallyPrime is listed
in the README. Native Windows Tally/Claude Desktop validation remains
outstanding, and Intel Mac is not qualified.
