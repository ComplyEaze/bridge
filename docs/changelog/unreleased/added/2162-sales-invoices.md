- ComplyEaze Bridge can now prepare and post a GST Sales invoice (#2162). With posting on, `build_import_xml` takes a
  Sales voucher with its `invoice` object and invoice number, for a supply within the company's own state, one invoice
  per batch. Before the build it reads the invoice's financial year a month at a time and refuses a number already in
  use there, ignoring case and leading zeros. An invoice is posted only through `post_import`, after you approve it in
  its own window, and is read back afterwards. Never import an invoice's file through Tally's Import menu. The
  description of the setting **Allow voucher posting, one voucher or a batch of up to 50** now names Sales invoices.
  **Downgrade:** once a Sales invoice is saved, do not install a release older than `mcp-v0.7.0` on this computer: it
  can refuse this computer's import journal, and then every import and verification stops there, Journal, Payment,
  Receipt and Contra included. To recover, install this release or a later one again. Do not edit the journal by hand.
