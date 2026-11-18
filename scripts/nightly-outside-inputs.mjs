// SPDX-License-Identifier: Apache-2.0
//
// Lists, as the Markdown section of the nightly mutation run's tracking issue, the files outside the
// tax-audit crate that its mutation records depend on and that changed between the last green nightly
// run's commit and this one. The nightly run is the only check of these inputs (they select nothing at
// pull-request time), so a reader of a failure needs to see at once whether one of them moved.
//
//     node scripts/nightly-outside-inputs.mjs --base <40-hex commit or ""> --head <40-hex commit> [--reason TEXT]
//
// Three outcomes, never mixed: "None changed", a list of the files, or "Could not be computed" with the
// reason. A failure to compute (no earlier green run, a commit missing from the checkout, a git error)
// is never reported as "none changed". The command prints the section and exits 0 for all three; a bad
// argument is exit 2. Needs a checkout that holds both commits (the report job fetches full history).
import { execFileSync } from "node:child_process";
import { existsSync, realpathSync } from "node:fs";
import { fileURLToPath } from "node:url";

/** The paths the crate's records depend on that lie outside the crate (see parity/mutations.mjs, "What the guarantee does NOT cover"). */
export const OUTSIDE_INPUTS = [
  "src-tauri/crates/bridge-tally-protocol",
  "src-tauri/crates/bridge-tally-primitives",
  "src-tauri/Cargo.lock",
  "src-tauri/Cargo.toml",
  "rust-toolchain.toml",
];
export const LISTED = 40; // files named in the section; the rest are counted
const REASON_CHARS = 200;
const COMMIT = /^[0-9a-f]{40}$/;

const failed = (reason) => ({ status: "failed", files: [], reason });
const oneLine = (text) => String(text).replace(/\s+/g, " ").trim().slice(0, REASON_CHARS);

function git(args, cwd) {
  return execFileSync("git", args, { cwd, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"], maxBuffer: 64 * 1024 * 1024 });
}

/** {status: "none" | "changed" | "failed", files, reason} for the two commits. */
export function outsideInputs({ base, head, cwd = process.cwd(), reason = "" }) {
  if (!COMMIT.test(head ?? "")) return failed("this run's commit is not a full commit id");
  if (!base) return failed(oneLine(reason) || "there is no earlier green run to compare with");
  if (!COMMIT.test(base)) return failed("the earlier green run's commit is not a full commit id");
  try {
    git(["cat-file", "-e", `${base}^{commit}`], cwd);
  } catch {
    return failed("the earlier green run's commit is not in this checkout");
  }
  let listing;
  try {
    listing = git(["diff", "--name-only", "-z", "--no-renames", base, head, "--", ...OUTSIDE_INPUTS], cwd);
  } catch (error) {
    return failed(`git diff failed (${oneLine(error.stderr || error.message)})`);
  }
  const files = listing.split("\0").filter(Boolean).sort();
  return files.length ? { status: "changed", files, reason: "" } : { status: "none", files: [], reason: "" };
}

/** The Markdown section for `result`, comparing `base` with `head` (shown at 8 characters). */
export function render(result, base, head) {
  const short = (sha) => (COMMIT.test(sha ?? "") ? sha.slice(0, 8) : "unknown");
  const lines = [
    "## Outside inputs changed since the last green nightly",
    "",
    `Compared ${short(base)} (the last green nightly run) with ${short(head)} (this run), for the protocol and primitives crates (with the protocol crate's native fixtures), \`src-tauri/Cargo.lock\`, \`src-tauri/Cargo.toml\` and \`rust-toolchain.toml\`.`,
    "",
  ];
  if (result.status === "none") {
    lines.push("None changed.");
  } else if (result.status === "changed") {
    lines.push(`${result.files.length} changed:`, "");
    for (const file of result.files.slice(0, LISTED)) lines.push(`- \`${file}\``);
    if (result.files.length > LISTED) lines.push(`- and ${result.files.length - LISTED} more`);
  } else {
    lines.push(`Could not be computed: ${result.reason}. This is not the same as "none changed": the list is unknown.`);
  }
  return `${lines.join("\n")}\n`;
}

function main(argv) {
  const options = { base: "", head: "", reason: "" };
  for (let i = 0; i < argv.length; i += 2) {
    const name = argv[i]?.replace(/^--/, "");
    if (!["base", "head", "reason"].includes(name) || !argv[i]?.startsWith("--") || i + 1 >= argv.length) {
      process.stderr.write('usage: nightly-outside-inputs.mjs --base <commit or ""> --head <commit> [--reason TEXT]\n');
      return 2;
    }
    options[name] = argv[i + 1];
  }
  process.stdout.write(render(outsideInputs(options), options.base, options.head));
  return 0;
}

if (process.argv[1] && existsSync(process.argv[1]) && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url))) {
  process.exit(main(process.argv.slice(2)));
}
