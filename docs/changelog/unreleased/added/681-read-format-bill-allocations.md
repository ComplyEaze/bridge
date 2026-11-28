- The tax-audit read format is now version 1.2 (part of #681). A vouchers part may say, with
  `bill_allocations: true`, that its request asked for each ledger entry's bill allocations. The
  tax-audit engine then reads each allocation's bill type, the bill it names and its amount, and
  tells a read that did not ask apart from vouchers that have none. A row it cannot read, such as
  one with an amount and no bill type, leaves that read's allocations unreadable and refuses
  nothing else. No tax-audit result uses the allocations yet, and ComplyEaze Bridge's own audit
  read does not yet mark its vouchers parts.
