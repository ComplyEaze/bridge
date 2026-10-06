// SPDX-License-Identifier: Apache-2.0

import { spawnSync } from "node:child_process";
import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../", import.meta.url));
const modes = new Set(process.argv.slice(2));
const checkFrontend = modes.size === 0 || modes.has("--frontend");
const checkRust = modes.size === 0 || modes.has("--rust");
const firstPartyRustPackages = new Set([
  "bridge",
  "bridge-bank-statement",
  "bridge-tally-core",
  "bridge-tally-primitives",
  "bridge-tally-protocol",
  "bridge-tally-transport",
  "bridge-tax-audit",
  "tally-protocol-simulator",
]);

if ([...modes].some((mode) => !["--frontend", "--rust"].includes(mode))) {
  throw new Error("Usage: check-dependency-inventory.mjs [--frontend] [--rust]");
}

// Says why a command failed: a runner flake (a network error from cargo) and a real drift look the same
// without the command's own error, so the last lines of stderr are kept, bounded.
const commandFailure = (label, result) => {
  const cause = result.error ? `could not run: ${result.error.message}` : `${result.signal ? `signal ${result.signal}` : `exit status ${result.status}`}`;
  const tail = String(result.stderr ?? "").trimEnd().split(/\r?\n/).slice(-20).join("\n").slice(-2000);
  return `${label} inventory command failed (${cause})${tail ? `:\n${tail}` : ""}`;
};

// `cargo tree` downloads index entries, and a runner's network fails now and then (4 and 5 Oct 2026:
// "curl failed ... Error in the HTTP2 framing layer"). Only such a network failure is retried, twice;
// any other failure (a real drift, a lockfile problem) fails at once.
const networkFailure = /download of .+ failed|curl failed|HTTP2 framing|HTTP\/2 stream|timed out|Timeout was reached|Connection (?:was )?reset|resolve host|Failed to connect|Couldn't connect|SSL connect error|Empty reply|Recv failure|Send failure|transfer closed|spurious network error|got 50[234]/i;
const retryDelayMs = Number(process.env.INVENTORY_RETRY_DELAY_MS ?? 3000);
const spawnCommand = (command, args, label) => {
  for (let attempt = 1; ; attempt += 1) {
    const result = spawnSync(command, args, {
      cwd: root,
      encoding: "utf8",
      maxBuffer: 64 * 1024 * 1024,
      windowsHide: true,
    });
    if (!result.error && result.status === 0) return result;
    if (attempt === 3 || result.error || !networkFailure.test(String(result.stderr ?? ""))) {
      throw new Error(commandFailure(label, result));
    }
    const cause = String(result.stderr).split(/\r?\n/).find((line) => networkFailure.test(line))?.trim().slice(0, 200);
    console.error(`${label} inventory command hit a network failure (attempt ${attempt} of 3${cause ? `: ${cause}` : ""}); retrying`);
    if (retryDelayMs > 0) Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, retryDelayMs * attempt);
  }
};

const runJson = (command, args, label) => {
  const result = spawnCommand(command, args, label);
  try {
    return JSON.parse(result.stdout);
  } catch {
    throw new Error(`${label} inventory command returned invalid JSON`);
  }
};

const runText = (command, args, label) => spawnCommand(command, args, label).stdout;

const componentPattern = /(@?[A-Za-z0-9_.+-]+(?:\/[A-Za-z0-9_.+-]+)?) (\d+\.\d+\.\d+(?:[+-][^,\s]+)?)/g;
const reportComponents = (report, endMarker) => {
  const components = new Set();
  const inventory = endMarker ? report.split(endMarker, 1)[0] : report;
  for (const line of inventory.split(/\r?\n/)) {
    for (const match of line.matchAll(componentPattern)) {
      components.add(`${match[1]} ${match[2]}`);
    }
  }
  return components;
};

const compare = (label, expected, reported, { allowStale = false } = {}) => {
  const missing = [...expected].filter((component) => !reported.has(component)).sort();
  const stale = [...reported].filter((component) => !expected.has(component)).sort();
  if (missing.length || (!allowStale && stale.length)) {
    const details = [
      missing.length ? `missing: ${missing.join(", ")}` : "",
      stale.length ? `stale: ${stale.join(", ")}` : "",
    ].filter(Boolean).join("; ");
    throw new Error(`${label} third-party inventory drift (${details})`);
  }
  return stale;
};

if (checkFrontend) {
  const packageManager = process.env.npm_execpath;
  if (!packageManager) {
    throw new Error("Run the frontend inventory through the pinned pnpm script");
  }
  const licenses = runJson(
    process.execPath,
    [packageManager, "licenses", "list", "--prod", "--json"],
    "frontend",
  );
  const expected = new Set();
  for (const packages of Object.values(licenses)) {
    for (const dependency of packages) {
      for (const version of dependency.versions) {
        expected.add(`${dependency.name} ${version}`);
      }
    }
  }
  const report = await readFile(new URL("../THIRD_PARTY_LICENSES.txt", import.meta.url), "utf8");
  compare("frontend", expected, reportComponents(report));
  console.log(`Frontend license inventory matches ${expected.size} locked components.`);
}

if (checkRust) {
  const cargo = process.platform === "win32" ? "cargo.exe" : "cargo";
  const targets = [
    "x86_64-pc-windows-msvc",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
  ];
  const expected = new Set();
  for (const target of targets) {
    const tree = runText(
      cargo,
      [
        "tree", "--locked",
        "--manifest-path", "src-tauri/Cargo.toml",
        "--target", target,
        "--edges", "normal,build",
        "--prefix", "none",
        "--format", "{p}",
      ],
      `Rust ${target}`,
    );
    for (const line of tree.split(/\r?\n/)) {
      const match = line.match(/^([A-Za-z0-9_.+-]+) v(\d+\.\d+\.\d+(?:[+-][^\s]+)?)/);
      if (match && !firstPartyRustPackages.has(match[1])) {
        expected.add(`${match[1]} ${match[2]}`);
      }
    }
  }
  const report = await readFile(
    new URL("../THIRD_PARTY_LICENSES_RUST.txt", import.meta.url),
    "utf8",
  );
  const overincluded = compare(
    "Rust",
    expected,
    reportComponents(report, "License texts and notices"),
    { allowStale: true },
  );
  console.log(`Rust license inventory matches ${expected.size} locked components.`);
  if (overincluded.length) {
    console.warn(
      `Rust notice conservatively includes ${overincluded.length} additional ` +
        `multi-target components: ${overincluded.join(", ")}`,
    );
  }
}
