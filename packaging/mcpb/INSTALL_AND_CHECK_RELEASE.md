**Install and check.** Download the `.mcpb` file for your computer from the Assets list at the bottom of this page:
the one ending `windows-x64.mcpb` for Windows, `macos-arm64.mcpb` for a Mac with an Apple chip (M1 or newer). Follow the
[installation guide](https://github.com/ComplyEaze/bridge/blob/master/docs/agent/INSTALL.md). Before you open the
file, check it against the `.sha256` file of the same name in the same list: on Windows, in Command Prompt, type
`cd Downloads` and then run `certutil -hashfile "<file>" SHA256`; on a Mac, in Terminal, type `cd Downloads` and then
run `shasum -a 256 "<file>"` (put your downloaded file's name in place of `<file>`, brackets included, and keep the quotes: a file
downloaded twice can get a name with "(1)" in it, which needs them). The 64-character code it prints must match the code at
the start of the `.sha256` file; that shows the file arrived whole, not who made it. ComplyEaze Bridge is not yet
code-signed or notarized.
