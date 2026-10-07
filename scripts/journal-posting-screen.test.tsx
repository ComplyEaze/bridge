// @vitest-environment jsdom

import { readFileSync } from "node:fs";
import React, { act } from "react";
import { createRoot } from "react-dom/client";
import { afterEach, expect, test, vi } from "vitest";

const mocks = vi.hoisted(() => ({ invoke: vi.fn() }));

vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));

import { JournalPostingScreen } from "../src/JournalPostingScreen";

const review = {
  batchId: "bridge-00000000-0000-4000-8000-000000000001",
  sha256: "a".repeat(64),
  company: {
    name: "Synthetic Accounts",
    guid: "00000000-0000-4000-8000-000000000002",
    companyNumber: "100001",
    booksFrom: "20260401",
  },
  builtAt: "2026-09-07T00:00:00Z",
  dispatched: false,
  responseRecorded: false,
  details: {
    date: "20260901",
    reference: "REF-1",
    narration: "Synthetic test only",
    entries: [
      { ledger: "Expense", side: "Dr" as const, amount: "12.50" },
      { ledger: "Cash", side: "Cr" as const, amount: "12.50" },
    ],
    totalDebit: "12.50",
    totalCredit: "12.50",
  },
};

const config = { host: "127.0.0.1", port: 9001 };

function button(host: HTMLElement, label: string) {
  const match = [...host.querySelectorAll<HTMLButtonElement>("button")].find((item) => item.textContent?.includes(label));
  if (!match) throw new Error(`Missing button: ${label}`);
  return match;
}

afterEach(() => {
  document.body.replaceChildren();
  mocks.invoke.mockReset();
});

test("renders the review flow and keeps safe recovery actions after each backend result", async () => {
  mocks.invoke.mockResolvedValueOnce(review);
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);

  await act(async () => {
    root.render(<JournalPostingScreen config={config} />);
  });
  await act(async () => {
    button(host, "Choose Journal file").click();
  });
  expect(host.textContent).toContain("Synthetic Accounts");
  expect(host.textContent).toContain("2026-09-01");
  expect(host.textContent).toContain("Synthetic test only");
  expect(host.textContent).toContain("Expense");
  expect(host.textContent).toContain("Total debit");
  expect(host.textContent).toContain("Total credit");

  mocks.invoke.mockResolvedValueOnce({
    batchId: review.batchId,
    result: {
      result: {
        dispatch: { state: "not_dispatched" },
        attempt_recorded: false,
        error: { message: "No posting attempt was recorded." },
      },
    },
  });
  await act(async () => {
    button(host, "Post Journal").click();
  });
  expect(button(host, "Post Journal")).toBeTruthy();
  expect(host.textContent).toContain("No posting attempt was recorded.");

  mocks.invoke.mockRejectedValueOnce({ message: "The post result is uncertain." });
  await act(async () => {
    button(host, "Post Journal").click();
  });
  expect(button(host, "Reconcile original batch")).toBeTruthy();

  mocks.invoke.mockResolvedValueOnce({
    batchId: review.batchId,
    result: {
      result: {
        dispatch: { state: "previous_attempt_reconciled", resent: false },
      },
    },
  });
  await act(async () => {
    button(host, "Reconcile original batch").click();
  });
  expect(host.textContent).toContain("ComplyEaze Bridge confirmed the original Journal and its saved batch.");
  expect(button(host, "Choose another file")).toBeTruthy();
  expect([...host.querySelectorAll("button")].some((item) => item.textContent?.includes("Post Journal"))).toBe(false);

  mocks.invoke.mockResolvedValueOnce(null);
  await act(async () => {
    button(host, "Choose another file").click();
  });
  expect(host.textContent).toContain("ComplyEaze Bridge confirmed the original Journal and its saved batch.");
  expect([...host.querySelectorAll("button")].some((item) => item.textContent?.includes("Post Journal"))).toBe(false);

  root.unmount();
});

test("reports a busy Journal action until the native post result is rendered", async () => {
  const busyStates: boolean[] = [];
  let resolvePost!: (value: unknown) => void;
  const pendingPost = new Promise((resolve) => {
    resolvePost = resolve;
  });
  mocks.invoke.mockResolvedValueOnce(review);
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);

  await act(async () => {
    root.render(<JournalPostingScreen config={config} onBusyChange={(busy) => busyStates.push(busy)} />);
  });
  await act(async () => {
    button(host, "Choose Journal file").click();
  });

  mocks.invoke.mockReturnValueOnce(pendingPost);
  await act(async () => {
    button(host, "Post Journal").click();
    await Promise.resolve();
  });
  expect(busyStates.at(-1)).toBe(true);
  expect(button(host, "Review approval dialog").disabled).toBe(true);

  await act(async () => {
    resolvePost({
      batchId: review.batchId,
      result: { result: { dispatch: { state: "posted_verified", resent: false } } },
    });
    await pendingPost;
  });
  expect(busyStates.at(-1)).toBe(false);
  expect(host.textContent).toContain("ComplyEaze Bridge confirmed the original Journal and its saved batch.");
  expect([...host.querySelectorAll("button")].some((item) => item.textContent?.includes("Post Journal"))).toBe(false);
  root.unmount();
});

test("retains an attempted Journal and response evidence through failed reconciliation", async () => {
  mocks.invoke.mockResolvedValueOnce(review).mockResolvedValueOnce({
    batchId: review.batchId,
    result: {
      result: {
        dispatch: { state: "reconciliation_required", resent: false },
        attempt_recorded: true,
        dispatch_response: {
          request_sha256: "a".repeat(64),
          response_sha256: "b".repeat(64),
          bytes: 538,
          outcome: {
            application_status: "success",
            counters: {
              created: 1,
              altered: 0,
              deleted: 0,
              ignored: 0,
              errors: 0,
              cancelled: 0,
              exceptions: 0,
              line_error_count: 0,
            },
            exceptions_were_reported: true,
          },
        },
        error: { code: "import_ledger_append_failed", message: "Reconcile the original batch." },
      },
    },
  });
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);

  await act(async () => {
    root.render(<JournalPostingScreen config={config} />);
  });
  await act(async () => {
    button(host, "Choose Journal file").click();
  });
  await act(async () => {
    button(host, "Post Journal").click();
  });

  const details = host.querySelector("details");
  expect(details?.textContent).toContain("Tally's response is retained for reconciliation; ComplyEaze Bridge only confirms posting after a matching Journal readback.");
  expect(details?.textContent).toContain("success");
  expect(details?.textContent).toContain("Created 1");
  expect(details?.textContent).toContain("Line errors 0");
  expect(details?.textContent).toContain("a".repeat(64));
  expect(host.textContent).toContain("Reconcile original batch");
  for (const failure of [
    new Error("Reconciliation connection lost"),
    { code: "journal_review_refused", tally_state_may_have_changed: false, message: "Saved history unavailable" },
  ]) {
    let rejectReconcile!: (error: unknown) => void;
    const pendingReconcile = new Promise((_, reject) => { rejectReconcile = reject; });
    mocks.invoke.mockReturnValueOnce(pendingReconcile);
    await act(async () => { button(host, "Reconcile original batch").click(); });
    expect(button(host, "Reconciling original batch").disabled).toBe(true);
    expect(details?.textContent).toContain("b".repeat(64));
    await act(async () => { rejectReconcile(failure); });
    expect(button(host, "Reconcile original batch").disabled).toBe(false);
    expect(host.querySelectorAll("button")).toHaveLength(1);
    expect(host.textContent).not.toContain("Post Journal");
    expect(host.textContent).not.toContain("Choose another file");
    expect(details?.textContent).toContain("b".repeat(64));
    expect(details?.textContent).toContain("Created 1");
  }
  expect(mocks.invoke.mock.calls.map(([command]) => command)).toEqual([
    "desktop_pick_journal_for_review", "desktop_post_reviewed_journal",
    "desktop_reconcile_reviewed_journal", "desktop_reconcile_reviewed_journal",
  ]);
  mocks.invoke.mockResolvedValueOnce({
    batchId: review.batchId,
    result: { result: { dispatch: { state: "previous_attempt_reconciled", resent: false } } },
  });
  await act(async () => { button(host, "Reconcile original batch").click(); });
  expect(host.textContent).toContain("ComplyEaze Bridge confirmed the original Journal and its saved batch.");
  expect(button(host, "Choose another file")).toBeTruthy();
  expect(host.textContent).not.toContain("Post Journal");
  root.unmount();
});

for (const [code, expected] of [
  ["import_approval_timed_out", "The approval dialog expired before ComplyEaze Bridge could post this Journal."],
  ["import_approval_declined", "The Journal was not posted because approval was declined."],
] as const) {
  test(`explains ${code} while keeping Post Journal available`, async () => {
    mocks.invoke.mockResolvedValueOnce(review).mockResolvedValueOnce({
      batchId: review.batchId,
      result: {
        result: {
          dispatch: { state: "not_dispatched" },
          attempt_recorded: false,
          error: { code, message: "No posting attempt was recorded." },
        },
      },
    });
    const host = document.createElement("div");
    document.body.append(host);
    const root = createRoot(host);

    await act(async () => {
      root.render(<JournalPostingScreen config={config} />);
    });
    await act(async () => {
      button(host, "Choose Journal file").click();
    });
    await act(async () => {
      button(host, "Post Journal").click();
    });

    expect(host.textContent).toContain(expected);
    expect(host.textContent).toContain(code);
    expect(button(host, "Post Journal")).toBeTruthy();
    root.unmount();
  });
}

test("pre-dispatch admission refusal with explicit no-attempt evidence returns to file review without suggesting a resend", async () => {
  mocks.invoke.mockResolvedValueOnce(review);
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);
  await act(async () => { root.render(<JournalPostingScreen config={config} />); });
  await act(async () => { button(host, "Choose Journal file").click(); });
  mocks.invoke.mockResolvedValueOnce({
    result: { result: {
      dispatch: { state: "admission_refused", resent: false },
      attempt_recorded: false,
      error: { code: "import_batch_not_found", message: "This request stopped before approval or posting." },
    } },
  });
  await act(async () => { button(host, "Post Journal").click(); });
  expect(host.textContent).toContain("This request stopped before approval or posting.");
  expect(button(host, "Choose another file").disabled).toBe(false);
  expect([...host.querySelectorAll("button")].some((item) => /Post Journal|Reconcile original/.test(item.textContent ?? ""))).toBe(false);
  root.unmount();
});

test("admission refusal with unavailable history keeps the original Journal for reconciliation", async () => {
  mocks.invoke.mockResolvedValueOnce(review).mockResolvedValueOnce({
    result: { result: {
      dispatch: { state: "admission_refused", resent: false },
      attempt_recorded: null,
      error: { code: "import_ledger_unavailable", message: "Saved history is unavailable." },
    } },
  });
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);
  await act(async () => { root.render(<JournalPostingScreen config={config} />); });
  await act(async () => { button(host, "Choose Journal file").click(); });
  await act(async () => { button(host, "Post Journal").click(); });

  expect(host.textContent).toContain("The original batch needs reconciliation.");
  expect(button(host, "Reconcile original batch")).toBeTruthy();
  expect([...host.querySelectorAll("button")].some((item) => item.textContent?.includes("Choose another file"))).toBe(false);
  root.unmount();
});

test.each([
  { code: "journal_review_refused", tally_state_may_have_changed: false, retryable: true },
  { code: "journal_review_refused", tally_state_may_have_changed: true, retryable: false },
  { code: "journal_review_refused", tally_state_may_have_changed: "false", retryable: false },
  { code: "unknown_failure", tally_state_may_have_changed: false, retryable: false },
])("post command failure preserves the typed dispatch boundary: $code / $tally_state_may_have_changed", async ({ retryable, ...failure }) => {
  mocks.invoke.mockResolvedValueOnce(review);
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);
  await act(async () => { root.render(<JournalPostingScreen config={config} />); });
  await act(async () => { button(host, "Choose Journal file").click(); });
  mocks.invoke.mockRejectedValueOnce({ ...failure, message: "The saved data directory is unavailable." });
  await act(async () => { button(host, "Post Journal").click(); });
  expect(host.textContent).toContain("The saved data directory is unavailable.");
  const labels = [...host.querySelectorAll("button")].map((item) => item.textContent);
  expect(labels.some((label) => label?.includes("Post Journal"))).toBe(retryable);
  expect(labels.some((label) => label?.includes("Choose another file"))).toBe(retryable);
  expect(labels.some((label) => label?.includes("Reconcile original batch"))).toBe(!retryable);
  root.unmount();
});


test("an active or unresolved snapshot blocks posting from an already-open review", async () => {
  mocks.invoke.mockResolvedValueOnce(review);
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);
  await act(async () => { root.render(<JournalPostingScreen config={config} />); });
  await act(async () => { button(host, "Choose Journal file").click(); });
  expect(button(host, "Post Journal").disabled).toBe(false);
  await act(async () => { root.render(<JournalPostingScreen config={config} postingBlocked />); });
  expect(button(host, "Post Journal").disabled).toBe(true);
  expect(host.textContent).toContain("Finish or reconcile the snapshot before posting this Journal.");
  await act(async () => { button(host, "Post Journal").click(); });
  expect(mocks.invoke).toHaveBeenCalledTimes(1);
  await act(async () => { root.render(<JournalPostingScreen config={config} postingBlocked={false} />); });
  expect(button(host, "Post Journal").disabled).toBe(false);
  mocks.invoke.mockResolvedValueOnce({
    batchId: review.batchId,
    result: { result: { dispatch: { state: "posted_verified", resent: false } } },
  });
  await act(async () => { button(host, "Post Journal").click(); });
  expect(mocks.invoke.mock.calls[1][0]).toBe("desktop_post_reviewed_journal");
  expect(host.textContent).toContain("ComplyEaze Bridge confirmed the original Journal and its saved batch.");
  root.unmount();
});


test("does not admit Journal actions before native lifecycle protection is ready", async () => {
  const host = document.createElement("div");
  document.body.append(host);
  const root = createRoot(host);
  await act(async () => { root.render(<JournalPostingScreen config={config} lifecycleAdmissionReady={false} lifecycleAdmissionError="ComplyEaze Bridge could not install native close protection." />); });
  expect(host.textContent).toContain("Native close protection unavailable");
  expect(button(host, "Choose Journal file").disabled).toBe(true);
  await act(async () => { button(host, "Choose Journal file").click(); });
  expect(mocks.invoke).not.toHaveBeenCalled();
  root.unmount();
});

test("marks an entry sent as On Account and shows its note only when sent (#1234)", async () => {
  const note = "On Account: a bill-wise ledger when this batch was built. Its entries carry no bill allocation.";
  const cells = async (details: typeof review.details & { onAccountNote?: string }) => {
    mocks.invoke.mockResolvedValueOnce({ ...review, details });
    const host = document.createElement("div");
    document.body.append(host);
    const root = createRoot(host);
    await act(async () => { root.render(<JournalPostingScreen config={config} />); });
    await act(async () => { button(host, "Choose Journal file").click(); });
    const shown = {
      marks: [...host.querySelectorAll("tbody td:first-child")].map((cell) => cell.querySelector("strong")?.textContent ?? null),
      note: host.textContent?.includes(note) ?? false,
    };
    root.unmount();
    return shown;
  };
  expect(await cells(review.details)).toEqual({ marks: [null, null], note: false });
  const [expense, cash] = review.details.entries;
  expect(await cells({ ...review.details, entries: [{ ...expense, onAccount: true }, { ...cash, onAccount: false }], onAccountNote: note }))
    .toEqual({ marks: ["On Account", null], note: true });
  // The details of a marked review as the backend serialises them: the text its Rust test
  // (review_marks_each_entry_on_a_ledger_approved_on_account) compares with its own output.
  const rust = readFileSync("src-tauri/src/agent_desktop_journal_tests.rs", "utf8");
  const pinned = /const MARKED_REVIEW_DETAILS_SENT: &str =\s*r#"(.+)"#;/.exec(rust);
  expect(pinned, "MARKED_REVIEW_DETAILS_SENT was not found in the Rust test file").not.toBeNull();
  const sent = JSON.parse(pinned![1]);
  expect(await cells(sent)).toEqual({ marks: ["On Account", null], note: true });
});
