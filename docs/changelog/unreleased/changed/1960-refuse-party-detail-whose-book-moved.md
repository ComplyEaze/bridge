- The `outstandings` party detail (#1960) now reads the company's two change marks before the statement of open bills and again after
  the voucher window, and refuses (`voucher_window_changed_during_read`, the same refusal and next step as every read
  that brackets the book) when either moved; the voucher window no longer reads the marks a third time. Before, the statement, the ledger catalogue and the vouchers were tied by the
  party's spelling with nothing checking that the book was the same between them, so a ledger renamed in Tally between two of
  those reads could answer as a finding (for example no bill for the party). It adds one marks read net (two, and the window reads none of its own), and any voucher or
  master edit in Tally during the call refuses it; run the same request again once. A change that moves neither mark is not
  seen. Measured: a rename moves the master mark and a voucher edit moves the voucher mark (TallyPrime 7.1 Silver, 21
  Sep 2026), but one rename that changed only a ledger's letter case moved neither mark (one book), so that kind of
  rename is still not caught; this read has not been run against Tally.
