// @vitest-environment jsdom
// SPDX-License-Identifier: Apache-2.0

import React, { act } from "react";
import { createRoot } from "react-dom/client";
import { afterEach, expect, test, vi } from "vitest";

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const mocks = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));

import { AllClientsScreen } from "../src/AllClientsScreen";

afterEach(() => {
  document.body.replaceChildren();
  mocks.invoke.mockReset();
});

// #2462: each row and its group label draw the company's shown name. The
// sweep sends, and the row opens, the company by the name Tally reported.
test("a client row draws the shown name, and the sweep and the row keep the raw one", async () => {
  const raw = "Synthetic\u200bTra\u202eders";
  const shown = "Synthetic\\u{200b}Tra\\u{202e}ders";
  const company = {
    name: raw,
    guid: "00000000-0000-4000-8000-000000002462",
    company_number: "100001",
    books_from_yyyymmdd: "20260401",
    canonical_origin: "http://127.0.0.1:9000",
  };
  mocks.invoke.mockImplementation((command: string) => {
    if (command === "load_client_group_labels") return Promise.resolve({ labels: {}, degradation_reason: null });
    if (command === "fetch_tally_outstandings_all_companies") {
      return Promise.resolve([{
        company: raw,
        company_shown_name: shown,
        company_guid: company.guid,
        company_number: company.company_number,
        books_from_yyyymmdd: company.books_from_yyyymmdd,
        canonical_origin: company.canonical_origin,
        result: {
          state: "complete",
          report: {
            receivable_total: "100.00",
            payable_total: "0",
            ageing: { days_0_30: "100.00", days_31_60: "0", days_61_90: "0", days_90_plus: "0" },
            open_receivable_bill_count: 1,
            top_parties: [],
          },
        },
      }]);
    }
    return Promise.resolve(null);
  });
  const opened: unknown[] = [];
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);
  await act(async () => root.render(
    <AllClientsScreen
      config={{ host: "127.0.0.1", port: 9000 }}
      companies={[company]}
      onOpenCompany={(chosen) => { opened.push(chosen); }}
      liveReadNavigationLocked={false}
      asOf="2026-09-08"
      onTallyReadActivityChange={() => {}}
    />,
  ));
  const read = [...host.querySelectorAll<HTMLButtonElement>("button")].find((button) => button.textContent?.includes("Read all clients"));
  expect(read).toBeDefined();
  await act(async () => { read!.click(); });

  const sweeps = mocks.invoke.mock.calls.filter(([command]) => command === "fetch_tally_outstandings_all_companies");
  expect(sweeps).toHaveLength(1);
  expect(JSON.stringify(sweeps[0][1])).toContain(JSON.stringify(raw));
  expect(host.querySelector(".clients-name strong")?.textContent).toBe(shown);
  const groupLabel = host.querySelector<HTMLElement>(".client-group-label-grid label");
  expect(groupLabel?.querySelector("span")?.textContent).toBe(shown);
  expect(groupLabel?.querySelector("input")?.getAttribute("aria-label")).toBe(`Group label for ${shown}`);
  expect(host.textContent).not.toContain(raw);

  await act(async () => { host.querySelector<HTMLButtonElement>("button.clients-row")?.click(); });
  expect(opened).toEqual([company]);
  root.unmount();
});
