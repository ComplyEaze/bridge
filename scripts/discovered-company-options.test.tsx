// @vitest-environment jsdom
// SPDX-License-Identifier: Apache-2.0

import React, { act } from "react";
import { createRoot } from "react-dom/client";
import { afterEach, expect, test, vi } from "vitest";
import { DiscoveredCompanyOptions, type UntrustedCompanyCandidate } from "../src/DiscoveredCompanyOptions";

const actEnvironment = globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean };
actEnvironment.IS_REACT_ACT_ENVIRONMENT = true;

afterEach(() => document.body.replaceChildren());

async function render(companies: UntrustedCompanyCandidate[], onChoose: (name: string) => void) {
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);
  await act(async () => root.render(
    <DiscoveredCompanyOptions companies={companies} disabled={false} checking={false} onChoose={onChoose} />,
  ));
  return { buttons: [...host.querySelectorAll<HTMLButtonElement>("button")], unmount: () => act(async () => root.unmount()) };
}

// #2449: the picker draws each company by the name the backend shows and marks
// the one whose name cannot be sent; it sends back only the name as Tally gave it.
test("a company whose name cannot be sent is marked and not offered, next to an ordinary one", async () => {
  const marked = "Synthetic\u0007Traders";
  const onChoose = vi.fn();
  const { buttons, unmount } = await render([
    { name: marked, shown_name: "Synthetic\\u{0007}Traders", name_refusal: "company_name_control_character" },
    { name: "Synthetic Traders", shown_name: "Synthetic Traders" },
  ], onChoose);

  expect(buttons).toHaveLength(2);
  expect(buttons[0].disabled).toBe(true);
  expect(buttons[0].dataset.nameRefusal).toBe("company_name_control_character");
  expect(buttons[0].textContent).toContain("Synthetic\\u{0007}Traders");
  expect(buttons[0].textContent).toContain("holds a control character");
  expect(buttons[0].textContent).not.toContain(marked);
  expect(buttons[1].disabled).toBe(false);
  expect(buttons[1].dataset.nameRefusal).toBeUndefined();

  await act(async () => buttons[0].click());
  expect(onChoose).not.toHaveBeenCalled();
  await act(async () => buttons[1].click());
  expect(onChoose).toHaveBeenCalledWith("Synthetic Traders");
  await unmount();
});

test("the shown name is drawn and the raw name is what verification is sent", async () => {
  const raw = "Synthetic\u200bTra\u202eders";
  const onChoose = vi.fn();
  const { buttons, unmount } = await render([
    { name: raw, shown_name: "Synthetic\\u{200b}Tra\\u{202e}ders" },
    { name: "\u0915\u094d\u200d\u0937", shown_name: "\u0915\u094d\u200d\u0937" },
  ], onChoose);

  expect(buttons[0].querySelector("span")?.textContent).toBe("Synthetic\\u{200b}Tra\\u{202e}ders");
  expect(buttons[1].querySelector("span")?.textContent).toBe("\u0915\u094d\u200d\u0937");
  await act(async () => buttons[0].click());
  expect(onChoose).toHaveBeenCalledWith(raw);
  await unmount();
});

test("each refusal code has its own reason", async () => {
  const codes = ["company_name_blank", "company_name_control_character", "company_name_length_not_established"] as const;
  const { buttons, unmount } = await render(
    codes.map((code) => ({ name: code, shown_name: code, name_refusal: code })),
    vi.fn(),
  );
  const reasons = buttons.map((button) => button.querySelector("small")?.textContent);
  expect(new Set(reasons).size).toBe(codes.length);
  expect(buttons.every((button) => button.disabled)).toBe(true);
  await unmount();
});
