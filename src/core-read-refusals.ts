// Titles for the core read's refusals, shared by the Gap Map and the capability probe's
// reason line, which receives the same codes when a probe's core read stops.
export const CORE_READ_REFUSAL_TITLES = {
  company_book_extent_drifted: "Tally gave two different answers to the same read",
  company_altmstid_missing: "Tally did not report its master-change marker",
  company_identity_mismatch: "Tally did not report the selected company",
} as const;

export function coreReadRefusalTitle(code: string): string | undefined {
  return Object.prototype.hasOwnProperty.call(CORE_READ_REFUSAL_TITLES, code)
    ? CORE_READ_REFUSAL_TITLES[code as keyof typeof CORE_READ_REFUSAL_TITLES]
    : undefined;
}
