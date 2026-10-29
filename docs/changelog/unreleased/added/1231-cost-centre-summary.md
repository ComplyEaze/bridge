- `vouchers` takes `summarise_by: cost_centre` to sum a window's cost-centre allocations into one bucket per cost
  centre under each cost category, with each category's own totals (`category_totals`: a category allocates the whole
  entry, so categories overlap) and the entries that carry no allocation (`entries_unallocated`). An entry whose
  allocations cannot be read, or whose allocations under one category do not add up to the entry, refuses the summary
  (`summary_cost_centre_unreadable`, `summary_cost_centre_allocation_mismatch`). The summary does not read the company's
  Cost Centres setting: allocations were read on a book with it off, so an empty summary means nothing was allocated in
  the window. Measured on the saved raw answers of one synthetic book with Cost Centres on (TallyPrime 7.2): a Payment
  allocated to two centres of one category and a Journal entry allocated in full under two categories; no request
  changed (#1231).
