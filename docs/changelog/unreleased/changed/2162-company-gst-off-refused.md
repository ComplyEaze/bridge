- A Sales invoice build now refuses a company whose GST is turned off in Tally (`invoice_company_gst_off`), and
  one whose answer does not say whether GST is on (`invoice_company_gst_unstated`), before anything is written; the
  re-read before a post refuses it the same way (#2162).
