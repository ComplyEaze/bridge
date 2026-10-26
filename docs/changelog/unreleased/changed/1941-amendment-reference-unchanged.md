- An amendment now refuses a reference that differs from any build of its batch, whether added, changed or removed
  (`import_amend_voucher_reference_changed`, #1941). Before, an amendment that changed only a Journal's reference
  could read `posted_verified` without having been imported, and one that removed a reference could not clear it.
  Change a reference in Tally after the amendment is imported and verified.
