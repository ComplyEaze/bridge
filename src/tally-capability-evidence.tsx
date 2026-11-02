import React from "react";
import { coreReadRefusalTitle } from "./core-read-refusals";
import { formatIdentifier } from "./display-format";

type CapabilityEvidence = {
  state: "supported" | "unsupported" | "unknown" | "not_configured";
  confidence: "documented" | "observed" | "inferred" | "unknown";
  safe_reason_code?: string;
};

export type CapabilityProfile = {
  profile_version: number;
  product: string;
  release?: string;
  mode?: string;
  transports: Record<string, CapabilityEvidence>;
  features: Record<string, CapabilityEvidence>;
  packs: Record<string, CapabilityEvidence>;
};

export const PACK_LABELS: Record<string, string> = {
  core_accounting: "Core accounting",
  india_tax: "India tax",
  bills_and_payments: "Bills and payments",
  inventory: "Inventory",
};

const CAPABILITY_REASON_LABELS: Record<string, string> = {
  xml_export_shape_unrecognized: "The endpoint answered, but not as a Tally company list. Check that this port is TallyPrime's, then probe again.",
  configuration_not_observed: "ComplyEaze Bridge did not inspect this optional transport's configuration.",
  company_identity_invalid: "The company result contained an invalid or unsafe identity field.",
  company_identity_ambiguous: "Two or more returned companies shared the same complete observed identity.",
  company_identity_display_scope_ambiguous: "Two same-GUID books differ only by name casing or surrounding whitespace, so Tally cannot safely scope the selected book. Rename one book, then probe again.",
  standard_ledger_identity_profile_observed: "A strict, scoped standard ledger collection observed one local company identity. It does not establish completeness, sync eligibility, or write support.",
  scoped_standard_identity_observed: "A strict, scoped local company identity was observed. Responder authenticity and accounting completeness remain unestablished.",
  practical_limit_not_measured: "No live workload has established a practical response limit for this endpoint.",
  selected_read_probe_not_run: "This selected read was not run by the connection probe.",
  write_probe_not_run: "No write probe was run. ComplyEaze Bridge never infers write support from read access.",
  verified_snapshot_not_run: "No profile-scoped capability run has established this pack's declared contract.",
};

function formatCapabilityState(state: CapabilityEvidence["state"]): string {
  switch (state) {
    case "supported":
      return "Supported";
    case "unsupported":
      return "Unsupported";
    case "not_configured":
      return "Not configured";
    default:
      return "Unknown";
  }
}

function formatConfidence(confidence: CapabilityEvidence["confidence"]): string {
  switch (confidence) {
    case "documented":
      return "Documented evidence";
    case "observed":
      return "Observed by this probe";
    case "inferred":
      return "Inferred, not directly observed";
    default:
      return "Evidence confidence unknown";
  }
}

function formatCapabilityReason(reason?: string): string {
  if (!reason) {
    return "No reason code was returned.";
  }

  const refusal = coreReadRefusalTitle(reason);
  return CAPABILITY_REASON_LABELS[reason] || (refusal ? `The core read stopped: ${refusal}.` : `Reason: ${formatIdentifier(reason)}.`);
}

function CapabilityBadge({ evidence }: { evidence?: CapabilityEvidence }) {
  if (!evidence) {
    return <span className="capability-badge state-unobserved">Not observed</span>;
  }

  return (
    <span className={`capability-badge state-${evidence.state}`}>
      {formatCapabilityState(evidence.state)}
    </span>
  );
}

export function CapabilityRows({
  capabilities,
  labels,
}: {
  capabilities?: Record<string, CapabilityEvidence>;
  labels: Record<string, string>;
}) {
  const keys = Array.from(new Set([...Object.keys(labels), ...Object.keys(capabilities || {})]));

  return (
    <div className="capability-list">
      {keys.map((key) => {
        const evidence = capabilities?.[key];
        return (
          <div className="capability-row" key={key}>
            <div>
              <strong>{labels[key] || formatIdentifier(key)}</strong>
              <span>
                {evidence
                  ? `${formatConfidence(evidence.confidence)}. ${formatCapabilityReason(evidence.safe_reason_code)}`
                  : "This endpoint has not been probed in the current configuration."}
              </span>
            </div>
            <CapabilityBadge evidence={evidence} />
          </div>
        );
      })}
    </div>
  );
}
