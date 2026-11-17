// @vitest-environment jsdom
// SPDX-License-Identifier: Apache-2.0

import { act } from "react";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

const mocks = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));

const actEnvironment = globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean };
actEnvironment.IS_REACT_ACT_ENVIRONMENT = true;

// #2462: a verified company whose name holds a zero-width space and a
// right-to-left override. `name` is as Tally reported it; `shown_name` is what
// the backend's one display rule gives, and the only form any screen draws.
const raw = "Synthetic\u200bTra\u202eders";
const shown = "Synthetic\\u{200b}Tra\\u{202e}ders";
const origin = "http://127.0.0.1:9000";
const company = {
  name: raw,
  shown_name: shown,
  guid: "00000000-0000-4000-8000-000000002462",
  company_number: "100001",
  books_from_yyyymmdd: "20260401",
  correlation_key: "c".repeat(64),
  identity_confidence: "observed",
};

function button(root: HTMLElement, text: string): HTMLButtonElement {
  const matched = [...root.querySelectorAll<HTMLButtonElement>("button")].find((candidate) =>
    candidate.textContent?.includes(text),
  );
  expect(matched, text).toBeDefined();
  return matched!;
}

function calls(command: string) {
  return mocks.invoke.mock.calls.filter(([name]) => name === command).map(([, argument]) => argument);
}

beforeEach(() => vi.resetModules());

afterEach(async () => {
  const root = document.getElementById("root") as (HTMLElement & {
    bridgeRoot?: { unmount(): void };
  }) | null;
  await act(async () => root?.bridgeRoot?.unmount());
  document.body.replaceChildren();
  window.localStorage.clear();
  mocks.invoke.mockReset();
});

test("a verified company is drawn by its shown name on setup and a report, and every request keeps the raw name", async () => {
  mocks.invoke.mockImplementation((command: string) => {
    if (command === "desktop_terms_status") return Promise.resolve({ terms_version: "2026-10.1", state: "accepted", accepted: true });
    if (command === "tally_persisted_company_profiles") {
      return Promise.resolve({ profiles: [], total_profiles: 0, limit: 100, truncated: false });
    }
    if (command === "probe_tally") {
      return Promise.resolve({
        connection: { reachable: true, compatible: true, product: "TallyPrime", server_text: "" },
        companies: [company], canonical_origin: origin, observed_at_unix_ms: 1,
        review_id: "review-2462", profile_sha256: "profile-2462", review_commitment_sha256: "commitment-2462",
        profile: { profile_version: 1, product: "TallyPrime", transports: { xml_http: { state: "supported", confidence: "observed" } }, features: {}, packs: {} },
      });
    }
    if (command === "save_tally_setup") {
      return Promise.resolve({
        passport_snapshot_id: "snapshot-2462",
        canonical_origin: origin,
        observed_at_unix_ms: 2,
        company: { ...company, mirror_company_id: "company-2462" },
      });
    }
    // The report's currency read never settles, so the screen stays on its
    // opening heading and no further read starts.
    if (command === "detect_tally_base_currency") return new Promise(() => {});
    return Promise.resolve(undefined);
  });
  const root = document.createElement("div");
  root.id = "root";
  document.body.append(root);
  await act(async () => { await import("../src/main.tsx"); });
  await act(async () => { button(root, "Settings").click(); });
  await act(async () => { button(root, "Check Tally").click(); });
  const companiesView = [...root.querySelectorAll<HTMLButtonElement>("nav button, aside button")]
    .find((candidate) => candidate.textContent?.trim() === "Companies");
  expect(companiesView).toBeDefined();
  await act(async () => { companiesView!.click(); });

  // The setup screen's verified picker draws the codes, never the raw name.
  const picker = root.querySelector<HTMLElement>('[aria-label="Companies found in Tally"]');
  expect(picker).not.toBeNull();
  const option = [...picker!.querySelectorAll<HTMLButtonElement>("button")];
  expect(option).toHaveLength(1);
  expect(option[0].querySelector("span")?.textContent).toBe(shown);
  expect(root.textContent).not.toContain(raw);

  // Choosing and saving it sends the name exactly as Tally reported it.
  if (option[0].getAttribute("aria-pressed") !== "true") {
    await act(async () => { option[0].click(); });
  }
  await act(async () => { button(root, "Use this company").click(); });
  expect(calls("save_tally_setup")).toEqual([
    expect.objectContaining({
      request: expect.objectContaining({
        selected_company: {
          display_name: raw,
          company_guid: company.guid,
          company_number: company.company_number,
          books_from_yyyymmdd: company.books_from_yyyymmdd,
        },
      }),
    }),
  ]);
  expect(root.querySelector(".setup-complete")?.textContent).toContain(`${shown} is ready.`);

  // The client switcher shows the code form and finds the company by it.
  await act(async () => { button(root, "Switch client").click(); });
  const search = root.querySelector<HTMLInputElement>('.client-switcher-search input');
  expect(search).not.toBeNull();
  await act(async () => {
    const setValue = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value")!.set!;
    setValue.call(search, "\\u{202e}ders");
    search!.dispatchEvent(new Event("input", { bubbles: true }));
  });
  const found = [...root.querySelectorAll<HTMLElement>('[aria-label="Available clients"] button')];
  expect(found).toHaveLength(1);
  expect(found[0].querySelector(".client-switcher-option-details > span")?.textContent).toBe(shown);
  await act(async () => { found[0].click(); });

  // A report screen draws the same codes, and its read carries the raw name.
  const overview = [...root.querySelectorAll<HTMLButtonElement>("nav button, aside button")]
    .find((candidate) => candidate.textContent?.trim() === "Overview");
  expect(overview).toBeDefined();
  await act(async () => { overview!.click(); });
  expect(root.querySelector(".outstandings-empty h2")?.textContent).toBe(`Opening ${shown}`);
  expect(calls("detect_tally_base_currency")).toEqual([
    { request: expect.objectContaining({ selected_company: expect.objectContaining({ display_name: raw, company_guid: company.guid }) }) },
  ]);
  expect(root.textContent).not.toContain(raw);
});

test("a saved company is drawn by its shown name on Companies and in its local evidence", async () => {
  const saved = {
    ...company,
    canonical_endpoint: origin,
    guid_observed: true,
    mirror_company_id: "company-2462",
  };
  const run = {
    run_id: "run-2462", phase: "completed", requires_resume: false,
    resume_available: false, mirror_company_id: saved.mirror_company_id,
    pack_id: "core_accounting", requested_from_yyyymmdd: "20260401",
    requested_to_yyyymmdd: "20260401", active_window_id: null,
    completed_windows: 1, total_windows: 1, verification: null,
    proof_id: null, proof_sha256: null, gap_codes: [], warning_codes: [], failure_code: null,
  };
  mocks.invoke.mockImplementation((command: string) => {
    if (command === "desktop_terms_status") return Promise.resolve({ terms_version: "2026-10.1", state: "accepted", accepted: true });
    if (command === "tally_persisted_company_profiles") {
      return Promise.resolve({ profiles: [saved], total_profiles: 1, limit: 100, truncated: false });
    }
    if (command === "tally_recent_snapshot_runs") return Promise.resolve([run]);
    if (command === "tally_snapshot_status") return Promise.resolve({ ...run });
    if (command === "tally_write_fixture_enrollment_status") {
      return Promise.resolve({ fixture_state: "not_enrolled", candidate_gate: "not_enrolled", write_capability: "unknown" });
    }
    return Promise.resolve([]);
  });
  const root = document.createElement("div");
  root.id = "root";
  document.body.append(root);
  await act(async () => { await import("../src/main.tsx"); });

  // Choosing the saved client from the switcher opens it on Companies, since
  // no endpoint has been checked in this session.
  await act(async () => { button(root, "Switch client").click(); });
  const options = [...root.querySelectorAll<HTMLElement>('[aria-label="Available clients"] button')];
  expect(options).toHaveLength(1);
  await act(async () => { options[0].click(); });
  const details = [...root.querySelectorAll<HTMLElement>(".company-connection-details > div")]
    .find((row) => row.querySelector("dt")?.textContent === "Selected company");
  expect(details?.querySelector("dd")?.textContent).toBe(shown);
  const evidenceDescription = [...root.querySelectorAll<HTMLElement>("p.panel-description")]
    .filter((paragraph) => paragraph.textContent?.startsWith("Inspect "));
  expect(evidenceDescription).toHaveLength(1);
  expect(evidenceDescription[0].textContent)
    .toBe(`Inspect ${shown}’s local proof ledger, durable runs, and redacted proof exports without contacting Tally.`);
  expect(root.textContent).not.toContain(raw);

  // Its local evidence (a drawer outside the app root) names it by the same
  // form, in the recent runs and in the saved-companies list.
  await act(async () => { button(root, "Open local evidence").click(); });
  const drawer = document.body.querySelector<HTMLElement>(".evidence-drawer-content");
  expect(drawer).not.toBeNull();
  expect(drawer!.querySelector('[aria-label="Recent durable Core Accounting runs"] caption')?.textContent)
    .toBe(`Showing up to 10 of 1 loaded runs for ${shown}`);
  await act(async () => { button(drawer!, "Change saved company").click(); });
  const savedList = drawer!.querySelector<HTMLElement>('[aria-label="Saved companies"]');
  expect(savedList).not.toBeNull();
  expect([...savedList!.querySelectorAll("button > span")].map((span) => span.textContent)).toEqual([shown]);
  expect(document.body.textContent).not.toContain(raw);
});
