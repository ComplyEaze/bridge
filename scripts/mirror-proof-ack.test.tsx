// @vitest-environment jsdom
import React, { act } from "react";
import { createRoot } from "react-dom/client";
import { expect, test, vi } from "vitest";
import { MirrorProofScreen } from "../src/MirrorProofScreen";

type Props = React.ComponentProps<typeof MirrorProofScreen>;
type SnapshotJob = NonNullable<Props["snapshotJob"]>;

function run(run_id: string, phase: SnapshotJob["phase"], requires_resume = false): SnapshotJob {
  return {
    run_id,
    phase,
    requires_resume,
    resume_available: requires_resume,
    mirror_company_id: "company",
    pack_id: "core_accounting",
    requested_from_yyyymmdd: null,
    requested_to_yyyymmdd: null,
    active_window_id: null,
    completed_windows: 0,
    total_windows: 1,
    verification: null,
    proof_id: null,
    proof_sha256: null,
    gap_codes: [],
    warning_codes: [],
    failure_code: null,
  };
}

function props(
  refreshRecentSnapshots: Props["refreshRecentSnapshots"],
  snapshotOutcomeUnknownRunId: string | null = null,
  snapshotSelectionVersion = { current: 1 },
) {
  const clear = vi.fn();
  const result: Props = {
    config: { host: "127.0.0.1", port: 9001 },
    status: null,
    passport: null,
    tallyAction: null,
    selectedCompanyRecord: undefined,
    selectedCompanyLive: false,
    savedCompanyPicker: null,
    companyError: null,
    fixtureControls: null,
    voucherFrom: "",
    setVoucherFrom: vi.fn(),
    voucherTo: "",
    setVoucherTo: vi.fn(),
    syncEvidence: null,
    syncEvidenceError: null,
    refreshSyncEvidence: async () => {},
    latestProof: undefined,
    mirrorTruthState: "unknown",
    snapshotJob: null,
    setInspectedJob: vi.fn(),
    snapshotSelectionVersion,
    snapshotActive: false,
    snapshotError: null,
    snapshotStartOutcomeUnknown: true,
    snapshotOutcomeUnknownRunId,
    liveReadActionsLocked: false,
    setSnapshotStartOutcomeUnknown: clear,
    setSnapshotOutcomeUnknownRunId: vi.fn(),
    startCoreSnapshot: async () => {},
    cancelCoreSnapshot: async () => {},
    resumeCoreSnapshot: async () => {},
    selectedRecentSnapshotRuns: [],
    refreshRecentSnapshots,
    inspectedJob: null,
    activeGapCodes: [],
    activeWarningCodes: [],
    mirrorExplorer: null,
    mirrorExplorerError: null,
    loadMirrorExplorerPage: async () => {},
    proofPreview: null,
    proofPreviewSelection: null,
    previewRedactedProof: async () => {},
    runtimeSessions: [],
    runtimeError: null,
    refreshRuntime: async () => {},
    cancelTallyRequest: async () => {},
  };
  return { clear, result };
}

async function renderAcknowledgment(input: Props) {
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);
  await act(async () => root.render(<MirrorProofScreen {...input} />));
  const button = [...host.querySelectorAll("button")].find((candidate) =>
    candidate.textContent?.includes("Refresh and confirm"),
  );
  expect(button).toBeDefined();
  return {
    button: button!,
    cleanup: async () => {
      await act(async () => root.unmount());
      host.remove();
    },
  };
}

async function acknowledge(
  runs: SnapshotJob[],
  knownRunId: string | null = null,
  epoch = { current: 1 },
) {
  const fixture = props(async () => runs, knownRunId, epoch);
  const screen = await renderAcknowledgment(fixture.result);
  await act(async () => screen.button.click());
  await screen.cleanup();
  return fixture.clear;
}

test("acknowledgment keeps the posting block for active, missing, and failed refreshes", async () => {
  expect(await acknowledge([run("old", "completed"), run("new", "extract")])).not.toHaveBeenCalled();
  expect(await acknowledge([run("old", "completed")], "missing")).not.toHaveBeenCalled();

  const failed = props(async () => null);
  const screen = await renderAcknowledgment(failed.result);
  await act(async () => screen.button.click());
  expect(failed.clear).not.toHaveBeenCalled();
  await screen.cleanup();
});

test("a stale response retains the block and re-enables acknowledgment", async () => {
  let resolveRuns: (runs: SnapshotJob[]) => void;
  const refresh = new Promise<SnapshotJob[]>((resolve) => {
    resolveRuns = resolve;
  });
  const epoch = { current: 1 };
  const fixture = props(async () => refresh, null, epoch);
  const screen = await renderAcknowledgment(fixture.result);

  act(() => screen.button.click());
  expect(screen.button.disabled).toBe(true);
  epoch.current = 2;
  await act(async () => resolveRuns!([]));

  expect(fixture.clear).not.toHaveBeenCalled();
  expect(screen.button.disabled).toBe(false);
  await screen.cleanup();
});

test("acknowledgment clears only after no active run, including a detached known resume", async () => {
  expect(await acknowledge([run("old", "completed")])).toHaveBeenCalledWith(false);
  expect(await acknowledge([run("resume", "extract", true)], "resume")).toHaveBeenCalledWith(false);
});

test("inspecting a terminal run leaves the active worker state untouched", async () => {
  const active = run("active", "extract");
  const terminal = run("terminal", "completed");
  const fixture = props(async () => [active, terminal]);
  fixture.result.snapshotJob = active;
  fixture.result.snapshotActive = true;
  fixture.result.selectedRecentSnapshotRuns = [active, terminal];
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);

  await act(async () => root.render(<MirrorProofScreen {...fixture.result} />));
  const inspect = [...host.querySelectorAll("button")].filter((candidate) => candidate.textContent === "Inspect").at(-1);
  expect(inspect).toBeDefined();
  await act(async () => inspect!.click());

  expect(fixture.result.setInspectedJob).toHaveBeenCalledWith(terminal);
  expect(fixture.result.snapshotJob).toBe(active);
  expect(fixture.result.snapshotSelectionVersion.current).toBe(1);
  await act(async () => root.unmount());
  host.remove();
});

test("an inspected interrupted run can resume only while no worker is active", async () => {
  const interrupted = run("interrupted", "extract", true);
  const fixture = props(async () => [interrupted]);
  fixture.result.inspectedJob = interrupted;
  fixture.result.snapshotStartOutcomeUnknown = false;
  fixture.result.resumeCoreSnapshot = vi.fn(async () => {});
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);
  for (const active of [true, false]) {
    fixture.result.snapshotActive = active;
    await act(async () => root.render(<MirrorProofScreen {...fixture.result} />));
    const resume = [...host.querySelectorAll("button")].find((button) => button.textContent?.includes("Resume interrupted run"));
    expect(resume).toBeDefined();
    expect(resume!.disabled).toBe(active);
    await act(async () => resume!.click());
    expect(fixture.result.resumeCoreSnapshot).toHaveBeenCalledTimes(active ? 0 : 1);
  }
  expect(fixture.result.resumeCoreSnapshot).toHaveBeenCalledWith("interrupted");
  await act(async () => root.unmount());
  host.remove();
});

test("a run whose commit the mirror refused says so and offers no resume", async () => {
  const refused: SnapshotJob = {
    ...run("refused", "commit_pending", true),
    resume_available: false,
    failure_code: "snapshot_commit_refused",
  };
  const interrupted = run("interrupted", "commit_pending", true);
  const fixture = props(async () => [refused, interrupted]);
  fixture.result.snapshotJob = refused;
  fixture.result.inspectedJob = refused;
  fixture.result.selectedRecentSnapshotRuns = [refused, interrupted];
  fixture.result.snapshotStartOutcomeUnknown = false;
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);
  await act(async () => root.render(<MirrorProofScreen {...fixture.result} />));
  const spans = [...host.querySelectorAll("span")].map((span) => span.textContent);
  expect(spans).toContain("Failure: The local mirror refused to commit this run");
  expect(spans).toContain("Commit refused: a resume cannot succeed. Start a new read.");
  expect(spans).not.toContain("Detached legacy state: inspect only");
  expect([...host.querySelectorAll("button")].some((button) => button.textContent?.includes("Resume interrupted run"))).toBe(false);
  const cells = [...host.querySelectorAll("tbody tr")].map((row) => row.querySelectorAll("td")[4]?.textContent);
  expect(cells).toEqual(["Commit refused", "Resume available"]);
  await act(async () => root.unmount());
  host.remove();
});

test("a failed run's Failure line shows the Gap Map title, and an unlisted code its own words", async () => {
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);
  for (const [code, line] of [
    ["company_altmstid_missing", "Failure: Tally did not report its master-change marker"],
    ["some_unlisted_code", "Failure: Some unlisted code"],
  ]) {
    const failed = { ...run("failed", "failed"), failure_code: code };
    const fixture = props(async () => [failed]);
    fixture.result.snapshotJob = failed;
    fixture.result.snapshotStartOutcomeUnknown = false;
    await act(async () => root.render(<MirrorProofScreen {...fixture.result} />));
    const spans = [...host.querySelectorAll("span")].map((span) => span.textContent);
    expect(spans).toContain(line);
  }
  await act(async () => root.unmount());
  host.remove();
});

test("a probe whose core read stopped names the refusal by its title in Pack readiness", async () => {
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);
  const fixture = props(async () => []);
  fixture.result.snapshotStartOutcomeUnknown = false;
  fixture.result.passport = {
    profile_version: 1,
    product: "TallyPrime",
    transports: {},
    features: {},
    packs: {
      core_accounting: { state: "unknown", confidence: "observed", safe_reason_code: "company_altmstid_missing" },
      india_tax: { state: "unknown", confidence: "observed", safe_reason_code: "some_unlisted_code" },
    },
  };
  await act(async () => root.render(<MirrorProofScreen {...fixture.result} />));
  const text = host.textContent ?? "";
  expect(text).toContain("Observed by this probe. The core read stopped: Tally did not report its master-change marker.");
  expect(text).toContain("Observed by this probe. Reason: Some unlisted code.");
  expect(text).not.toContain("Company altmstid missing");
  await act(async () => root.unmount());
  host.remove();
});

test("the proof screen shows no rejected-row count and names the rejected-records gap", async () => {
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);
  const fixture = props(async () => []);
  fixture.result.snapshotStartOutcomeUnknown = false;
  fixture.result.syncEvidence = {
    latest_proofs: [{
      integrity_state: "entry_hash_valid",
      run_id: "run",
      selection_token: "token",
      proof_sha256: "0".repeat(64),
      pack_id: "core_accounting",
      outcome: "failed",
      verification_state: "unverified",
      started_at_unix_ms: 1,
      completed_at_unix_ms: 2,
      accepted_records: 7,
      rejected_records: 0,
      provenance_unavailable_records: 0,
      gap_codes: ["rejected_snapshot_records"],
      warning_codes: [],
    }],
    latest_reconciliation_mismatches: [],
    incremental: {
      execution_enabled: false,
      affirmative_exact_capability_receipts: 0,
      establishment_receipts: 0,
      active_checkpoint_heads: 0,
      state: "execution_not_enabled",
      fallback_warning_code: "none",
    },
    core_accounting_freshness: { state: "never_verified", checkpoint_present: false, proof_present: true },
  };
  fixture.result.latestProof = fixture.result.syncEvidence.latest_proofs[0];
  fixture.result.activeGapCodes = ["rejected_snapshot_records"];
  await act(async () => root.render(<MirrorProofScreen {...fixture.result} />));
  const text = host.textContent ?? "";
  // The count is always zero (only accepted rows enter the mirror), so it is not shown.
  expect(text).not.toMatch(/rejected rows/i);
  expect(text).not.toContain("Accepted / rejected");
  expect(text).not.toContain("7 / 0");
  expect([...host.querySelectorAll("th")].map((cell) => cell.textContent)).toContain("Accepted returned rows");
  const cells = [...host.querySelectorAll("td")].map((cell) => cell.textContent);
  expect(cells).toContain("7");
  expect(cells).toContain("Source records were rejected");
  expect(text).toContain("Do not rely on the mirror for this run's records until the cause of the rejection is understood");
  expect(text).toMatch(/accepted canonical rows and \d+ canonical rows/);
  await act(async () => root.unmount());
  host.remove();
});

test("the proof ledger's Gaps column names each gap by its Gap Map title", async () => {
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);
  const fixture = props(async () => []);
  fixture.result.snapshotStartOutcomeUnknown = false;
  fixture.result.syncEvidence = {
    latest_proofs: [{
      integrity_state: "entry_hash_valid",
      run_id: "run",
      selection_token: "token",
      proof_sha256: "0".repeat(64),
      pack_id: "core_accounting",
      outcome: "failed",
      verification_state: "unverified",
      started_at_unix_ms: 1,
      completed_at_unix_ms: 2,
      accepted_records: 0,
      rejected_records: 0,
      provenance_unavailable_records: 0,
      gap_codes: ["company_book_extent_drifted", "some_unlisted_code"],
      warning_codes: [],
    }],
    latest_reconciliation_mismatches: [],
    incremental: {
      execution_enabled: false,
      affirmative_exact_capability_receipts: 0,
      establishment_receipts: 0,
      active_checkpoint_heads: 0,
      state: "execution_not_enabled",
      fallback_warning_code: "none",
    },
    core_accounting_freshness: { state: "never_verified", checkpoint_present: false, proof_present: true },
  };
  fixture.result.latestProof = fixture.result.syncEvidence.latest_proofs[0];
  await act(async () => root.render(<MirrorProofScreen {...fixture.result} />));
  const cells = [...host.querySelectorAll("td")].map((cell) => cell.textContent);
  expect(cells).toContain("Tally gave two different answers to the same read, Some unlisted code");
  await act(async () => root.unmount());
  host.remove();
});
