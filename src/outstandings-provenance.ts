// SPDX-License-Identifier: Apache-2.0

export type OutstandingsReadProvenance = {
  read_strategy: "native_bills";
  open_receivable_bill_count: number;
};

// Strategy comes from the Rust result.
export function readProvenance(report: OutstandingsReadProvenance) {
  const bills = report.open_receivable_bill_count.toLocaleString("en-IN");
  return `${bills} open receivable ${report.open_receivable_bill_count === 1 ? "bill" : "bills"} read from Tally`;
}
