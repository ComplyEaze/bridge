// SPDX-License-Identifier: Apache-2.0
// A release proposal reads each merged pull request's type label. Failing here, at the pull
// request, keeps a missing label from blocking a release later.
import { pathToFileURL } from "node:url";

export const TYPE_LABELS = ["type:feature", "type:bug", "type:rectify", "type:chore"];

export function typeLabelProblem(labels) {
  const types = labels.filter((name) => name.startsWith("type:"));
  if (types.length === 1 && TYPE_LABELS.includes(types[0])) return null;
  const found = types.length ? `it has ${JSON.stringify(types)}` : "it has none";
  return `A pull request needs exactly one of ${TYPE_LABELS.join(", ")}; ${found}.`;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const labels = JSON.parse(process.env.PR_LABELS ?? "");
  if (!Array.isArray(labels) || labels.some((name) => typeof name !== "string")) {
    throw new Error("PR_LABELS must be a JSON array of label names");
  }
  const problem = typeLabelProblem(labels);
  if (problem) {
    console.error(problem);
    process.exit(1);
  }
  console.log("type label ok");
}
