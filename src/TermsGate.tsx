// SPDX-License-Identifier: Apache-2.0

import React from "react";
import { invoke } from "@tauri-apps/api/core";
import { formatCommandErrorMessage } from "./tally-command-error";

// The desktop app's Terms-of-Use acceptance (bridge#1106). The backend refuses every read of Tally
// and every post until the user has accepted the version this build carries; this window is where
// the user does, before anything else is shown. The version named on the control comes from the
// backend, so it is always the one the build enforces.

export type TermsStatus = {
  terms_version: string;
  state: "accepted" | "not_accepted" | "record_unavailable";
  accepted: boolean;
};

export const TERMS_ADDRESS = "https://bridge.complyeaze.com/terms.html";
export const PRIVACY_ADDRESS = "https://bridge.complyeaze.com/privacy.html";

export function termsAcceptanceLabel(version: string): string {
  return `I accept the ComplyEaze Bridge Terms of Use (version ${version})`;
}

function isTermsStatus(value: unknown): value is TermsStatus {
  const status = value as Partial<TermsStatus> | null;
  return (
    typeof status === "object" &&
    status !== null &&
    typeof status.terms_version === "string" &&
    (status.state === "accepted" || status.state === "not_accepted" || status.state === "record_unavailable") &&
    typeof status.accepted === "boolean"
  );
}

type Props = {
  children: React.ReactNode;
  loadStatus?: () => Promise<unknown>;
  accept?: (version: string) => Promise<unknown>;
};

const defaultLoadStatus = () => invoke("desktop_terms_status");
const defaultAccept = (version: string) => invoke("desktop_accept_terms", { version });

/// Shows the app only once the Terms of Use are accepted. Anything the backend answers that is not
/// a clear "accepted" keeps the app closed.
export function TermsGate({ children, loadStatus = defaultLoadStatus, accept = defaultAccept }: Props) {
  const [status, setStatus] = React.useState<TermsStatus | null>(null);
  const [problem, setProblem] = React.useState<string | null>(null);
  const [ticked, setTicked] = React.useState(false);
  const [busy, setBusy] = React.useState(false);

  const load = React.useCallback(async () => {
    setProblem(null);
    try {
      const value = await loadStatus();
      if (!isTermsStatus(value)) throw new Error("Bridge did not say whether the Terms of Use are accepted.");
      setStatus(value);
    } catch (cause) {
      setStatus(null);
      setProblem(formatCommandErrorMessage(cause, "Bridge could not check whether the Terms of Use are accepted."));
    }
  }, [loadStatus]);

  React.useEffect(() => {
    void load();
  }, [load]);

  if (status?.accepted === true && status.state === "accepted") return <>{children}</>;

  const submit = async () => {
    if (!status || !ticked || busy) return;
    setBusy(true);
    setProblem(null);
    try {
      const value = await accept(status.terms_version);
      if (!isTermsStatus(value)) throw new Error("Bridge did not confirm the acceptance.");
      setStatus(value);
    } catch (cause) {
      setProblem(formatCommandErrorMessage(cause, "Bridge could not record the acceptance."));
    } finally {
      setBusy(false);
    }
  };

  return (
    <main className="terms-gate" aria-labelledby="terms-gate-title">
      <h1 id="terms-gate-title">Terms of Use</h1>
      {status === null && problem === null && <p role="status">Checking the Terms of Use…</p>}
      {status === null && problem !== null && (
        <>
          <p role="alert">{problem}</p>
          <button type="button" onClick={() => void load()}>Try again</button>
        </>
      )}
      {status !== null && (
        <>
          <p>
            ComplyEaze Bridge reads nothing from Tally and posts nothing until you accept the Terms of Use. Read the
            Terms of Use (version {status.terms_version}) at <span className="terms-address">{TERMS_ADDRESS}</span> and
            the Privacy Policy at <span className="terms-address">{PRIVACY_ADDRESS}</span>. If you do not own the
            practice, the owner should read and agree first.
          </p>
          {status.state === "record_unavailable" && (
            <p role="alert">
              Bridge could not read or write its record of your acceptance. Check that its data folder (com.complyeaze.bridge, in your
              Application Support or AppData folder) can be written, then try again. If that fails too, its
              terms-acceptance.jsonl holds a line Bridge did not write: move that file aside and try again.
            </p>
          )}
          <label>
            <input type="checkbox" checked={ticked} onChange={(event) => setTicked(event.target.checked)} />{" "}
            {termsAcceptanceLabel(status.terms_version)}
          </label>
          <button type="button" disabled={!ticked || busy} onClick={() => void submit()}>
            Continue
          </button>
          {problem !== null && <p role="alert">{problem}</p>}
        </>
      )}
    </main>
  );
}
