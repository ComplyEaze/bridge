// SPDX-License-Identifier: Apache-2.0

import assert from "node:assert/strict";
import test from "node:test";

import { classifyTallyError, classifyUnstructuredTallyError } from "../src/tally-error-copy.ts";

test("a deadline explains the safe next step without exposing protocol jargon", () => {
  assert.deepEqual(
    classifyTallyError({
      code: "tally_request_deadline_exceeded",
      message: "The bounded Tally read exceeded its production deadline.",
    }),
    {
      category: "Tally is taking longer than expected",
      action: "ComplyEaze Bridge did not change data in Tally. Do not repeat the same request. Wait until Tally is responsive, then review the connection and company before making a changed request.",
    },
  );
});

test("a missing or invalid mirror key says a retry does not help and points at the credential store, not at Tally", () => {
  const expected = {
    category: "The local Tally mirror cannot be opened",
    action: "Retrying does not help. Restore this computer user's credential store (see the technical details), then open ComplyEaze Bridge again. Nothing was deleted and nothing changed in Tally.",
  };
  for (const code of ["tally_mirror_key_missing", "tally_mirror_key_invalid"]) {
    assert.deepEqual(classifyTallyError({ code, message: "The encrypted Tally mirror cannot be opened." }), expected, code);
  }
  assert.notDeepEqual(
    classifyTallyError({ code: "tally_mirror_unavailable", message: "The encrypted Tally mirror could not be opened." }),
    expected,
    "a credential-store failure keeps its own, retryable guidance",
  );
});

test("an unstructured request failure remains fail-closed without internal navigation jargon", () => {
  assert.deepEqual(
    classifyUnstructuredTallyError("Cannot read properties of undefined (reading 'invoke')"),
    {
      category: "ComplyEaze Bridge could not complete this request",
      action: "ComplyEaze Bridge cannot confirm the final state. Do not retry the same request yet; check Tally and the connection first.",
    },
  );
});

test("a plain endpoint failure gives the operator one clear next step", () => {
  assert.deepEqual(
    classifyUnstructuredTallyError("endpoint connection failed"),
    {
      category: "Check the Tally address",
      action: "Confirm the local address and Tally XML server, then start a fresh connection check.",
    },
  );
});

test("a persisted-profile failure stays local instead of directing the operator to Tally", () => {
  const guidance = classifyUnstructuredTallyError("persisted_tally_company_profiles_unavailable");

  assert.deepEqual(guidance, {
    category: "ComplyEaze Bridge's saved company profiles are unavailable",
    action: "ComplyEaze Bridge could not read its local saved company profiles. Restart ComplyEaze Bridge. If this happens again, keep Tally unchanged and ask for help with ComplyEaze Bridge's local data.",
  });
});
