# Captured macOS SIGTRAP control

Captured at 2026-09-08T13:41:56.277223+00:00 from a disposable `bridge_lib-cafe` process on macOS. The process was compiled from `#include <signal.h>` and `int main(void) { return raise(SIGTRAP); }` and exited with signal 5. This is a collector control, not the unexplained application-test failure.

Raw report SHA-256: `e74da8d2157556ec96304d1def818f860c3d5130207ebaff4d678e7faeed0407`. The raw report is not committed because it contains process/user metadata. This privacy-reduced IPS preserves the captured two-object format, `bug_type`, process name, exception type/signal, faulting-thread index, thread trigger flags, frame/image indexes, symbols and relative offsets, and image basenames. Other fields, including paths, identifiers, registers and absolute memory addresses, were omitted; retained stack values were not invented.

The [hosted calibration workflow](../../.github/workflows/macos-crash-capture-control.yml) reproduces the same owned control operation and verifies capture on a hosted Mac. [Its initial successful run](https://github.com/ComplyEaze/bridge/actions/runs/34237062493) corroborates the capture mechanism; this fixture originated from the local control report above. The tests add synthetic privacy markers and bounded-shape mutations to this captured structure.

Format reference: [Apple IPS crash reports](https://developer.apple.com/documentation/xcode/interpreting-the-json-format-of-a-crash-report).

## Integrity digest

The row is the SHA-256 of the file's committed bytes, which
`scripts/check-fixture-provenance.mjs` checks (#838). A digest pins the bytes as
committed and claims nothing about where they came from: the Capture column
repeats only what this note says above.

| Fixture | Bytes | SHA-256 (integrity digest) | Capture |
| --- | ---: | --- | --- |
| `macos-sigtrap-control.ips` | 1,240 | `d67c736bf71b878dca4a6b39b3a19926081d33037d0c41de7a3316fcd4b8ad61` | privacy-reduced from a captured report, as above; the raw report SHA-256 above is not of these bytes |
