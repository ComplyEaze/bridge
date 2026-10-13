# Security policy

## Supported versions

| Version | Security status |
| --- | --- |
| `master` | Development branch; a fix, if made, lands here first |
| <!-- managed:latest-preview -->[`mcp-v0.5.1`](https://github.com/ComplyEaze/bridge/releases/latest) (9 October 2026)<!-- /managed:latest-preview -->, the latest published release | Receives security fixes, as a new release, if we make one |
| Earlier releases | Not supported; install the latest release |

ComplyEaze Bridge is published as extension archives (`.mcpb`) on GitHub
Releases. The latest published release, named in the table above, carries a
Windows archive and a macOS archive. Each archive has a SHA-256 checksum, a
provenance record naming the source commit it was built from, and a build
attestation that the release workflow checks before publishing. The
archives are not code-signed or notarized. An attestation says which workflow
run and commit produced a file; it is not a code signature, no client checks it
yet, and it does not show the code is safe. The desktop application has no
published installer, and CI bundle artifacts are smoke evidence that must not be
presented as a signed production release.

ComplyEaze Bridge is still being developed. If we fix a security issue, the fix
will be in a new release; we are not obliged to make one.

## Published advisories

- [GHSA-vm5g-r3p7-wxx7](https://github.com/ComplyEaze/bridge/security/advisories/GHSA-vm5g-r3p7-wxx7):
  the bank statement tool opened file paths that name a network location on
  Windows. It affects 0.3.0 and 0.4.0 and is fixed in 0.4.1.

## Reporting a vulnerability

Do not open a public issue for a suspected vulnerability or credential leak.
Report it privately, either through
[GitHub private vulnerability reporting](https://github.com/ComplyEaze/bridge/security/advisories/new)
or by email to <security@complyeaze.com>. If the advisory channel is
temporarily unavailable, use the email address; do not disclose the
vulnerability in a public issue.

Private security reporting supersedes the public Bug/Rectify issue rule in
[AGENTS.md](./AGENTS.md). A sanitized public issue or PR may be opened only
after maintainers determine disclosure is safe. Reporters should receive an
initial acknowledgement through the advisory within seven days; remediation
and disclosure timing depends on impact and coordinated-fix availability.

Do not include exploit details, credentials, customer data, certificate private
key material, certificate dumps, token PINs, API keys, or access tokens in a
public issue, pull request, discussion, screenshot, fixture, or log.

## Privacy and diagnostic data

Repository content and shared diagnostics must use synthetic data. Remove or
replace:

- personal names, email addresses, phone numbers, and account identifiers
- company, tax, ledger, voucher, financial, and document data
- certificate subject, issuer, serial number, fingerprint, and private key data
- PINs, tokens, secrets, session identifiers, and authentication headers
- local usernames, home directories, and absolute checkout paths

Preserve only the smallest redacted excerpt needed to reproduce a problem.
Treat certificate metadata and hardware-token details as sensitive even when
they are not private key material.

## Security review scope

Changes to credentials, endpoints, Tally data, documents, or persistence
require review of:

- secret lifetime and in-memory handling
- error, tracing, and subprocess output
- file-system and path boundaries
- endpoint scheme, host, and redirect validation
- migration compatibility and rollback behavior
- Windows and macOS differences

Security-sensitive changes require the security review and impact notes defined
in [AGENTS.md](./AGENTS.md) and [review-checklist.md](./review-checklist.md).
