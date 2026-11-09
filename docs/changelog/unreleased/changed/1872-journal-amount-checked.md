- The import journal (`agent-import-ledger.jsonl`) is now read with each entry's amount as a checked
  amount, as its dates are (part of #1872). A batch line whose amount is not a positive figure with
  exactly two decimals (a sign, a zero, a missing decimal, text) refuses the whole journal
  (`import_ledger_invalid`): building, posting, verifying and reconciling every batch stop, for every
  company on the computer, until the line is put right, and `local_data_report` shows the journal as
  not read and names the line under `journal.line`. The refusal now tells the person not to edit or delete
  the file and to send the journal and that report to support. The current code writes only valid
  amounts, so only a line edited by hand or damaged is expected to hold one, and the stored bytes do not
  change. A bank statement file whose voucher holds such an amount is refused by `statement_tie_out`
  as `proposals_file_invalid` (before, the tie-out returned its figures and the build refused the file later).
