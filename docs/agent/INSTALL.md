# Install ComplyEaze Bridge for Claude Desktop

Use the [Download page](https://bridge.complyeaze.com/download.html). It chooses
the current GitHub Release asset for your operating system and gives the same
setup steps without developer configuration. This guide is the fallback when that
page is unavailable.

## If a package is published

Check the project's [GitHub Releases](https://github.com/ComplyEaze/bridge/releases)
for a compatible `.mcpb`. If no release asset is listed, use the [source MCP
setup](./README.md) instead; the steps below apply only after a package is
published. The packages target Windows x64 and Apple Silicon Mac (ARM64); Intel
Mac and other platforms are not qualified. Package availability
is not a host-validation claim; read each release's notes for its current
runtime gaps.

Releases are published on GitHub, the newest listed first and normally marked
Latest. ComplyEaze
Bridge is still being developed, so a release may contain errors: try it on
test data first and keep current backups. It is not yet code-signed or
notarized. Each archive has a same-named `.sha256` file and
a small provenance record on its release so an organization can identify the
downloaded bytes and source commit.

## At a glance, release 0.5.1

- **You can ask for** outstanding receivables and payables with ageing, the trial balance, the vouchers in a date range (found by number, reference, narration or amount, and added up by ledger, month, voucher type, group or primary group; checked on one synthetic book, not a large one), Tally's own Cash Flow for whole months (shown only when its net total ties to the trial balance; not a cash flow statement under AS 3; tried on two synthetic books), and one ledger's movement.
- **Partly:** profit and loss and the balance sheet (a book with stock items is expected to be refused); closing stock value at 31 March, values only, on small books, and only the year to 31 March 2026 has been checked; and a file of Payment, Receipt and Contra vouchers from a password-protected SBI, HDFC or Union Bank of India PDF statement, which you check and import in Tally yourself. It creates no ledger, it cannot build onto a Cash-in-Hand ledger, and it has not yet been run on a real bank statement.
- **Not in this release:** GST returns or GSTR-2B matching, making sales or purchase invoices or GST entries, tax-audit flags, and Tally's Funds Flow, Ratio Analysis and negative-balance reports.

"You can ask" means it answered on the books and builds we ran it on; we have not yet run the published 0.5.1 file against TallyPrime. Very large books can fail or take longer than the assistant waits, and a book with several currencies is read only in part. Check any figure you rely on against Tally. The [Questions page](https://bridge.complyeaze.com/faq.html#what-can-i-ask) has the full table and its limits.

## Before you install

TallyPrime's HTTP gateway is off by default, and ComplyEaze Bridge cannot reach
Tally until it is on. In Tally's own connectivity / client-server configuration
settings, set Tally to act as a server (**"acts as Both"** in Tally's own
words) and note its HTTP gateway port — `9000` by default, but configurable.
To confirm the gateway is actually listening, open
`http://localhost:9000/status` (substitute your port) in a browser: a running
gateway answers with a short Tally XML response, and a browser that cannot
connect means the gateway is still off — **unless Tally is running in a Windows
virtual machine on a Mac**, in which case run this check inside that VM, or
only once the local forwarding in step 2 below is working. A Mac browser that
cannot connect may mean that forwarding is missing rather than that the gateway
is off. If instead it hangs without answering, Tally may simply be busy behind
another request — wait and retry rather than changing the setting. Do this
before step 3 below, so the port you enter in Bridge matches a gateway that is
actually on.

## Install and configure

1. In Claude Desktop, use **Settings → Extensions → Advanced settings →
   Install Extension…** and choose the `.mcpb` file. Opening the file directly
   may also work on your computer.
2. Keep **Tally host** as `localhost`. Bridge accepts only a local loopback
   endpoint. On a Mac, Tally must already be available there through a local
   Windows VM or organization-approved local forwarding. A separate PC or a
   LAN-only Tally cannot be reached by entering its network address.
3. Set **Tally port** to the HTTP gateway port you turned on and confirmed
   above (see *Before you install*). It defaults to `9000`, but only if
   Tally's gateway is configured for that port. This is not a Tally licence
   port. Changing it changes only where Bridge calls Tally, not Tally's own
   HTTP setting.
4. Read the Terms of Use linked in the extension settings, then tick **I
   accept the ComplyEaze Bridge Terms of Use**. Until you do, Bridge does
   not read from Tally, and from release 0.5.1 every Bridge tool answers that
   the Terms of Use are not accepted and names this setting (the tools the assistant called
   gave that answer on one Mac with a test package and on one Windows computer with the build our checks made for this release). (In 0.5.0, after an update
   from 0.4.2, Claude Desktop may not start Bridge: on a Mac with an error that
   does not mention the Terms, on one Windows computer with no error; bridge#1413.)
5. Save the extension settings, then quit Claude Desktop completely and reopen
   it. (On that Mac Claude Desktop
   started Bridge as soon as the settings were saved and a new chat got an answer
   from it, but the assistant first called a leftover second entry for the same extension,
   which failed with the same error; after Claude Desktop was quit and reopened there
   was one connector and no error.) In a new chat, use **Connectors** to confirm Bridge is connected.

Voucher file preparation and bank-statement parsing are available by default;
they write nothing to Tally. **Voucher posting is off by default** while four
known limits remain. Tally aims an import at a company by its name and cannot bind it to a company's GUID. Bridge's last request before the post checks that exactly one loaded company has the target's GUID and name, and that no other loaded company has the same name ignoring case and spacing; otherwise it refuses the post (bridge#607). A company renamed to, or loaded under, the target's name (or one differing only in case or spacing) in the moment after that check could still receive the voucher, if it has the voucher's ledgers. Bridge may flag afterwards that the loaded companies changed, but cannot always say where the voucher went, and cannot prevent it (accepted residual, bridge#574). A ledger renamed and replaced in that same moment means the post can land in the replacement ledger. Bridge marks the result as needing reconciliation when it sees that the ledger now resolves to a different master; a change that leaves the company's master mark unmoved, or is reverted before that check, is not seen, and a regroup in that moment is not detected (bridge#623). And Bridge has no tool to delete or undo a voucher it has posted, so a wrong post must be corrected by hand in Tally. It records the REMOTEID each post sends, but no delete tool exists yet (bridge#579, bridge#582). And the approval window covers only ComplyEaze Bridge: another Tally connector in the same Claude Desktop that can change entries can do so without it.
Turning on **Allow voucher posting (Journal, Payment, Receipt, Contra)** in the
extension settings adds posting; every new posting still requires your approval in a separate Bridge
dialog. Leave it off unless you accept those risks. If you installed an earlier
version, check the setting: an earlier default may still be saved as on.

Native posting through the extension accepts one Journal, Payment, Receipt or
Contra per approval, with existing ledgers and no supplied voucher number. A
command-line installation can also post a saved batch of 2 to 50 such vouchers
after one approval of a summary (per-ledger totals, not each voucher's date or
narration), when `BRIDGE_AGENT_ENABLE_BATCH_POST` is on together with posting.
That setting is off by default. From the next build the extension offers it as
its own setting, "Allow posting a batch of up to 50 vouchers under one approval",
also off by default and used only with posting on (bridge#712, bridge#1090).
Batch posting through Bridge has run live on a synthetic company on licensed
TallyPrime Silver, 200 Journals in one import with a test build whose cap was
raised, and verified; it is not proven on a multi-user book (bridge#725). A Payment, Receipt or Contra is
refused if any of its ledgers, or their groups, moved since the file was built
so that a bank or cash leg no longer classifies as it did. Tally assigns the number. Bridge uses a private request identity
for the native attempt; the selected XML file stays unchanged. Do not manually
import a file and then post it through Bridge: if the original Journal was edited,
Bridge may be unable to recognize that earlier business event.

While posting, pause other imports and ledger changes in the selected company
and leave Tally's product/licence mode unchanged. Bridge's checks do not lock
out changes made directly in Tally or by other software.

Stop Bridge and every client running its connector before upgrading, then restart
them with the newer version. Release 0.4.2 installed as a second extension beside
an older release instead of replacing it (its author line changed, and Claude
Desktop includes the author in an extension's identity; seen on a Mac, not tried
on Windows). The package our checks built for release 0.5.0, installed on one Mac
over 0.4.2, replaced it as an update: one extension at version 0.5.0, the Tally
port, Response redaction and the posting setting as they were (a posting
setting that was on stayed on), the old Terms setting gone and no value for the
new one (version 2026-10.1). With the published 0.5.0 file, on one Mac, the Tally
port and Response redaction were kept and the Terms setting was dropped the same
way (bridge#1413). On one Windows computer, the published 0.5.0 file over 0.4.2 was not started either: the chat showed no Bridge tool and no error. So after updating, tick the Terms setting, check the posting setting, then
quit and reopen Claude Desktop. Before the box is ticked, Claude Desktop may not
start 0.5.0 at all; from 0.5.1 it starts Bridge, and every Bridge tool answers
that the Terms of Use are not accepted (the tools the assistant called gave that answer on one Mac with a test package and on one Windows computer with the build our checks made for this release; bridge#1413). Installing over a release older than 0.4.2 was not
tried: remove such a release first. If Settings,
Extensions then lists two ComplyEaze Bridge entries (as happened when 0.4.2 was
installed beside an older release), remove the older entry before you use
either, and do not delete the data folder, which both versions use. Then enter the Tally port, the posting setting (posting starts
off), the Terms setting and Response redaction (it starts at none; set it again
if you had shortened or masked names), and quit and reopen Claude Desktop. Dispatch coordination uses the operating system's
local app-data folder on Windows and account home on macOS, independently of
launcher environment variables. Older processes may use a different coordination path.
Keep the recovery data when upgrading. New posting attempts add a native request
commitment to the journal; older connector builds cannot read that new record.
Use this version or a newer compatible build to reconcile it rather than removing
the journal to downgrade. Older receipts may lack evidence that every result
counter was actually reported. After upgrading, Bridge keeps those receipts but
cannot confirm a clean response from them, even when the Journal matches in
Tally. Preserve the original history for investigation; do not repost the Journal
to replace its receipt.

Bridge only accepts loopback Tally endpoints. Do not open a port to the
internet or use a remote host to make this work.

## Data and updates

Bridge runs locally. When Claude uses a Bridge tool, the selected Tally result
is sent to the AI provider used for that conversation, so the conversation is
not wholly local. Choose the package's redaction setting when it suits the
workflow.

Private MCPB downloads do not update automatically. To upgrade, keep the data
folder, install the newer release from the Extensions screen in Claude Desktop's
settings, then confirm its version. On one Mac the package our checks built for 0.5.0
(not the published file) replaced 0.4.2 and kept its Tally port, Response redaction and posting setting
(see above): there, tick the new Terms setting and check the posting setting,
then quit and reopen Claude Desktop. Installing over a release older than 0.4.2
was not tried: remove it first. If
Settings, Extensions lists two ComplyEaze Bridge entries instead (as happened
when 0.4.2 was installed beside an older release), remove
the older entry before you use either, enter your settings in the new one, tick
the new Terms setting, and quit and reopen Claude Desktop. Use the same screen to uninstall. Neither action changes Tally's
HTTP gateway configuration.
