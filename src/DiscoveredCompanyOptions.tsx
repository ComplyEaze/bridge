// SPDX-License-Identifier: Apache-2.0

import React from "react";
import { Building2 } from "lucide-react";

/**
 * A company Tally listed, before it is verified. `name` is exactly as Tally
 * reported it and is what verification sends back. `shown_name` is the name
 * drawn here, by the agent's own rule (#2449): each control or invisible
 * format character written as its `\u{..}` code, the joiners kept in a name
 * that can be sent. Two limits follow from that rule: a name of joiners alone
 * draws as nothing, and a name that spells a code out as text draws like one
 * holding the character.
 */
export type UntrustedCompanyCandidate = {
  name: string;
  shown_name: string;
  name_refusal?: "company_name_blank" | "company_name_control_character" | "company_name_length_not_established";
};

const REFUSAL_TEXT: Record<NonNullable<UntrustedCompanyCandidate["name_refusal"]>, string> = {
  company_name_blank: "This company's name is blank, so Bridge cannot read it.",
  company_name_control_character: "This company's name holds a control character, so Bridge cannot read it.",
  company_name_length_not_established: "This company's name is longer than Bridge can read.",
};

export function DiscoveredCompanyOptions({
  companies,
  disabled,
  checking,
  onChoose,
  label = "Companies to verify",
  actionText = "Use this company",
}: {
  companies: UntrustedCompanyCandidate[];
  disabled: boolean;
  checking: boolean;
  onChoose: (name: string) => void;
  label?: string;
  actionText?: string;
}) {
  return (
    <div className="company-options" role="list" aria-label={label}>
      {companies.map((company, index) => {
        const refusal = company.name_refusal ? REFUSAL_TEXT[company.name_refusal] : null;
        return (
          <button
            className="company-option"
            type="button"
            key={`${company.name}-${index}`}
            onClick={() => onChoose(company.name)}
            disabled={disabled || refusal !== null}
            data-name-refusal={company.name_refusal}
          >
            <Building2 size={20} />
            <span>{company.shown_name}</span>
            <small>{refusal ?? (checking ? "Checking…" : actionText)}</small>
          </button>
        );
      })}
    </div>
  );
}
