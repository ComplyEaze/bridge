// SPDX-License-Identifier: Apache-2.0
// Every pull request is planned against a milestone, and the queue is armed only for one that has it.
// Failing here, at the pull request, keeps a missing milestone from being found when the queue is armed.
// The two milestones that hold unscheduled work are not a plan, so a pull request in one of them fails too.
import { existsSync, realpathSync } from "node:fs";
import { fileURLToPath } from "node:url";

const UNSCHEDULED = ["Backlog (unscheduled)", "Planned after 1.0"];

export function milestoneProblem(title) {
  if (typeof title !== "string" || title.trim() === "") return "A pull request needs a milestone; it has none.";
  if (UNSCHEDULED.includes(title.trim())) {
    return `A pull request needs a scheduled milestone; "${title.trim()}" holds unscheduled work. Choose the release it is planned for.`;
  }
  return null;
}

if (process.argv[1] && existsSync(process.argv[1]) && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url))) {
  const title = JSON.parse(process.env.PR_MILESTONE ?? "");
  if (title !== null && typeof title !== "string") {
    throw new Error("PR_MILESTONE must be a JSON string or null");
  }
  const problem = milestoneProblem(title);
  if (problem) {
    console.error(problem);
    process.exit(1);
  }
  console.log("milestone ok");
}
