- `stock_by_godown` returns closing stock by godown at a month's last day (part of #2017): each godown under
  Tally's root with its value and Tally's net quantity, from Tally's own Godown Summary, only when the godown
  values add up to Tally's Stock Summary total (otherwise both totals and nothing else). For an item directly
  under the root it also returns the item's value and quantity in each godown, each godown at its own cost,
  only where they add up to the item's own line. An item inside a stock group gets no value per godown, and a
  godown under another godown is folded into its parent. Godown and item names are masked under
  `mask_parties`. Measured on TallyPrime 7.2 on one synthetic book; the tool itself has not yet been run
  against a live Tally.
