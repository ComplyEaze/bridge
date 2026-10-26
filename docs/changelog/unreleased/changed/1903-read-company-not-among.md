- A read for a company that is not among those open in Tally, refused as `company_identity_not_found`, now carries
  a next step: call `list_companies`, use an ID it returns, and ask the user to open the company in Tally if it is
  not listed. The refusal itself is unchanged. Checked on a test double, not a live Tally (#1903).
