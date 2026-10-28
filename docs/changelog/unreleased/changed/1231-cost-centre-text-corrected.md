- The text about cost-centre allocations in voucher reads that 0.6.1 shipped is corrected in three places. The vouchers tool no longer says
  that each category restates the entry's full amount and that summing rows across categories double-counts: that was not measured (only one
  category has been captured), so it now says the rows are not checked against the entry and not to add rows across categories without
  checking them. The privacy page no longer says that an allocation in a shape the read does not recognise is not sent and not flagged: it
  is flagged (`cost_centre_allocation_shape_unrecognised`), and the tool text, the guide and the protocol reference now list all eleven
  reasons an entry's list can be unreadable. On an item invoice the sales or purchase ledger's entry carries the goods line: an allocation list
  beside the goods line is read like any other and one inside it is flagged; no captured item invoice carries allocations. No request and no
  answer changed (part of #1231).
