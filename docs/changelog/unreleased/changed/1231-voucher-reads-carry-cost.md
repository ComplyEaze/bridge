- Voucher reads now carry cost-centre allocations: an entry that has them gets `cost_centre_allocations` (`category`,
  `cost_centre`, `amount`; names trimmed, the amount as sent; both names are shortened under `mask_parties`, in the
  vouchers read and the two registers). A list that cannot be read whole (a centre with no name or
  no amount, an amount that is not a decimal, a centre or a category named twice, an empty allocation container, an allocation list in a shape not recognised) does not refuse the read: the entry carries
  `cost_centre_allocations_unreadable` with a reason and no partial list. No request changed; the allocations were
  already in the answers and were dropped (part of #1231).
