// @vitest-environment jsdom
// SPDX-License-Identifier: Apache-2.0

import React, { act } from "react";
import { createRoot } from "react-dom/client";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, expect, test, vi } from "vitest";

import { PRIVACY_ADDRESS, TERMS_ADDRESS, TermsGate, termsAcceptanceLabel } from "../src/TermsGate";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const app = <p>The Bridge app</p>;
const open = { terms_version: "2026-10.1", state: "accepted", accepted: true };
const closed = { terms_version: "2026-10.1", state: "not_accepted", accepted: false };

async function mount(props: { loadStatus: () => Promise<unknown>; accept?: (version: string) => Promise<unknown> }) {
  const host = document.createElement("div");
  document.body.append(host);
  const reactRoot = createRoot(host);
  await act(async () => reactRoot.render(<TermsGate {...props}>{app}</TermsGate>));
  return { host, unmount: () => act(async () => reactRoot.unmount()) };
}

const text = () => document.body.textContent ?? "";
const checkbox = () => document.querySelector<HTMLInputElement>('input[type="checkbox"]');
const button = (name: string) =>
  [...document.querySelectorAll("button")].find((candidate) => candidate.textContent === name) as HTMLButtonElement;

afterEach(() => document.body.replaceChildren());

test("the app is not shown until the Terms are accepted, and the control starts off", async () => {
  const accept = vi.fn(async () => open);
  await mount({ loadStatus: async () => closed, accept });
  expect(text()).not.toContain("The Bridge app");
  expect(text()).toContain("reads nothing from Tally and posts nothing until you accept");
  expect(text()).toContain(TERMS_ADDRESS);
  expect(text()).toContain(PRIVACY_ADDRESS);
  expect(checkbox()?.checked).toBe(false);
  expect(button("Continue").disabled).toBe(true);
  expect(accept).not.toHaveBeenCalled();
});

test("ticking the control and continuing records the acceptance of the version shown, then shows the app", async () => {
  const accept = vi.fn(async (_version: string) => open);
  await mount({ loadStatus: async () => ({ ...closed, terms_version: "2099-01.7" }), accept });
  expect(text()).toContain(termsAcceptanceLabel("2099-01.7"));
  await act(async () => checkbox()!.click());
  expect(button("Continue").disabled).toBe(false);
  await act(async () => button("Continue").click());
  expect(accept).toHaveBeenCalledExactlyOnceWith("2099-01.7");
  expect(text()).toContain("The Bridge app");
});

test("an acceptance already recorded shows the app at once and asks for nothing", async () => {
  const accept = vi.fn();
  await mount({ loadStatus: async () => open, accept });
  expect(text()).toContain("The Bridge app");
  expect(document.querySelector("input")).toBeNull();
  expect(accept).not.toHaveBeenCalled();
});

test("anything but a clear accepted keeps the app closed", async () => {
  for (const answer of [[], null, {}, { terms_version: "2026-10.1", state: "accepted", accepted: false }, { terms_version: "2026-10.1", state: "not_accepted", accepted: true }]) {
    await mount({ loadStatus: async () => answer });
    expect(text(), JSON.stringify(answer)).not.toContain("The Bridge app");
    document.body.replaceChildren();
  }
  await mount({ loadStatus: async () => { throw { code: "x", message: "The check failed." }; } });
  expect(text()).not.toContain("The Bridge app");
  expect(text()).toContain("The check failed.");
});

test("a failed check can be tried again", async () => {
  const loadStatus = vi.fn().mockRejectedValueOnce(new Error("offline")).mockResolvedValue(open);
  await mount({ loadStatus });
  expect(text()).not.toContain("The Bridge app");
  await act(async () => button("Try again").click());
  expect(text()).toContain("The Bridge app");
});

test("a record that cannot be written says so and keeps the app closed", async () => {
  await mount({ loadStatus: async () => ({ ...closed, state: "record_unavailable" }) });
  expect(text()).toContain("could not read or write its record");
  expect(text()).not.toContain("The Bridge app");
});

test("a refused acceptance shows the backend's reason and keeps the app closed", async () => {
  const accept = async () => {
    throw { code: "terms_version_changed", message: "The window showed a different version.", remediation: "Reload the window." };
  };
  await mount({ loadStatus: async () => closed, accept });
  await act(async () => checkbox()!.click());
  await act(async () => button("Continue").click());
  expect(text()).toContain("The window showed a different version.");
  expect(text()).not.toContain("The Bridge app");
});

test("the control's label is the extension's, with the version the build enforces", () => {
  const rust = readFileSync(join(root, "src-tauri/src/agent_terms.rs"), "utf8");
  const version = /const TERMS_VERSION: &str = "([^"]+)";/.exec(rust)?.[1];
  expect(version).toBeTruthy();
  const manifest = JSON.parse(readFileSync(join(root, "packaging/mcpb/manifest.json"), "utf8"));
  const titles = Object.entries(manifest.user_config as Record<string, { title: string }>)
    .filter(([key]) => key.startsWith("accept_terms_"))
    .map(([, entry]) => entry.title);
  expect(titles).toEqual([termsAcceptanceLabel(version!)]);
});
