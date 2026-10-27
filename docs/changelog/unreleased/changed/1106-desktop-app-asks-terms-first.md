- The desktop app now shows a window asking the person to accept the Terms of Use, before it shows anything else and
  before it reads from or sends anything to Tally. Until the box for the current Terms version is ticked and accepted,
  every read and every send from the app is refused. The acceptance is a line in a small file in the app's own data
  folder (the Terms version, the time and that it came from the desktop app); if that file cannot be read or written the
  window stays closed and names the file. A new Terms version asks again. Reads of data already kept on the machine and
  saved drafts are not gated. The extension and the MCP server keep their own acceptance, unchanged. Tested with unit,
  component and browser tests; the packaged app and the acceptance window were not run (part of #1106).
