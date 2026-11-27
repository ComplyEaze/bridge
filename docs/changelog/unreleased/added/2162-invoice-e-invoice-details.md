- A Sales invoice can now carry the e-invoice details the portal has already registered for it (#2162): the
  `invoice` object's optional `e_invoice` takes the IRN, acknowledgement number and acknowledgement date, which go
  into the invoice as given and are shown in its approval window. After the post, ComplyEaze Bridge reads them back
  from Tally by the invoice's GUID, and an invoice whose details are missing or differ is reported as not verified.
  ComplyEaze Bridge generates none of them and sends nothing to the portal.
  **Downgrade:** once an invoice with e-invoice details is saved, do not install a release older than this one on this
  computer: it can refuse this computer's import journal, and then every import and verification stops there. To
  recover, install this release or a later one again. Do not edit the journal by hand.
