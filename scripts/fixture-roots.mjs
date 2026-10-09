// SPDX-License-Identifier: Apache-2.0
//
// The one list of fixture roots, and how a fixture directory is found (#838).
// check-fixture-byte-integrity.mjs and check-fixture-provenance.mjs both read
// FIXTURE_ROOTS from here and carry no copy of it, so the two gates cannot
// cover different directories.
//
// A directory is a fixture directory when either holds:
//   - its name is `fixture` or `fixtures`: the original rule, kept as a floor
//     so nothing it found drops out, a symlinked alias of a directory already
//     walked included;
//   - it holds a provenance note (`PROVENANCE.md`, `<stem>.PROVENANCE.md` or
//     `<NAME>_PROVENANCE.md`), or a JSON provenance record: an object with a
//     `source` string sharing its stem with another file there, the same
//     definition check-fixture-provenance.mjs reads. A `source` JSON with no
//     companion (a lock file naming its download URL) is not a record.
// Whether Git normalises a file's text is not a signal: measured on master
// `6136a818`, every tracked file outside these roots with `text` unset was an
// image (16 PNGs, one `.icns`, one `.ico`), and a fixture directory without
// `-text` is caught by the two signals above.

import { readFileSync, readdirSync, realpathSync, statSync } from "node:fs";
import { basename, join } from "node:path";

export const FIXTURE_ROOTS = [
  "src-tauri/crates/bridge-bank-statement/tests/fixtures",
  "src-tauri/crates/bridge-tax-audit/tests/fixtures",
  "src-tauri/crates/bridge-tally-protocol/tests/fixtures",
  "src-tauri/crates/tally-protocol-simulator/fixtures",
  "docs/tally/compatibility/fixtures",
  "scripts/fixtures",
  "scripts/testdata",
  "tools/bridge-tally-compatibility/tests/fixtures",
];

// Roots whose provenance rows keep basename matching: a bare name there may
// document a file of that name anywhere under the root. The one exemption
// keeps today's matching until its rows are converted, and is removed then.
export const BASENAME_ROWS_EXEMPT = new Set(["src-tauri/crates/bridge-tax-audit/tests/fixtures"]);

const PROVENANCE_NOTE = /(^|\.|_)PROVENANCE\.md$/;

// Why `directory` (holding `files`) is a fixture directory, or null.
export function fixtureSignal(directory, files) {
  const name = basename(directory);
  if (name === "fixture" || name === "fixtures") return "named fixture(s)";
  if (files.some((file) => PROVENANCE_NOTE.test(file))) return "holds a provenance note";
  for (const file of files) {
    if (!file.endsWith(".json")) continue;
    const stem = file.slice(0, -".json".length);
    if (!files.some((other) => other !== file && other.startsWith(`${stem}.`))) continue;
    // A read failure throws: only unparseable JSON is "not a record", so a
    // file that could not be read never passes as "nothing found".
    const text = readFileSync(join(directory, file), "utf8");
    let parsed;
    try {
      parsed = JSON.parse(text);
    } catch {
      continue;
    }
    if (parsed && typeof parsed === "object" && !Array.isArray(parsed) &&
        typeof parsed.source === "string" && parsed.source.trim()) {
      return `holds a provenance record (${file})`;
    }
  }
  return null;
}

// Every non-ignored fixture directory under `repositoryRoot` that no root
// covers, with the signal that found it. `ignored(paths)` returns the subset
// Git ignores; .git, node_modules and target are never entered.
export function unregisteredFixtureDirectories(repositoryRoot, ignored) {
  const covered = (path) =>
    FIXTURE_ROOTS.some((root) => path === root || path.startsWith(`${root}/`));
  const found = [];
  for (const { relative, files } of discoverDirectories(repositoryRoot, ignored)) {
    if (covered(relative)) continue;
    const signal = fixtureSignal(join(repositoryRoot, relative), files);
    if (signal) found.push(`${relative} (${signal})`);
  }
  return found.sort();
}

// Classifies a directory entry as "directory", "file", or "other", resolving
// one level of symlink indirection so a symlinked fixture directory (or
// fixture file) is never silently treated as absent. A symlink that cannot be
// resolved (broken target, permission denied, ...) fails loudly rather than
// being swallowed as "not present".
export function classifyEntry(path, entry) {
  if (entry.isDirectory()) return "directory";
  if (entry.isFile()) return "file";
  if (entry.isSymbolicLink()) {
    let stat;
    try {
      stat = statSync(path);
    } catch (error) {
      throw new Error(`unable to resolve symlink while walking the repository tree: ${path}: ${error.message}`);
    }
    if (stat.isDirectory()) return "directory";
    if (stat.isFile()) return "file";
    return "other";
  }
  return "other";
}

// Discovers every non-ignored directory under `root` (skipping .git,
// node_modules, and target unconditionally), with the names of the files it
// holds. Ignored trees such as .pnpm-store, coverage, and dist are filtered
// out level-by-level, BEFORE their contents are ever read — so an unreadable
// or huge directory inside an ignored tree cannot fail (or slow down) this
// walk. Symlinked directories are followed, with a realpath-based cycle guard
// that stops only the descent: a directory whose real path was already walked
// (a symlinked alias) is still recorded, with its files, so its own name and
// content are tested like any other.
function discoverDirectories(root, ignored) {
  const excludedDirectoryNames = new Set([".git", "node_modules", "target"]);
  const discovered = [];
  const visited = new Set();
  let level = [{ absolute: root, relative: "" }];
  while (level.length) {
    const candidates = [];
    for (const current of level) {
      const canonical = realpathSync(current.absolute);
      const descend = !visited.has(canonical);
      visited.add(canonical);
      const files = [];
      for (const entry of readdirSync(current.absolute, { withFileTypes: true })) {
        if (excludedDirectoryNames.has(entry.name)) continue;
        const entryAbsolute = join(current.absolute, entry.name);
        const kind = classifyEntry(entryAbsolute, entry);
        if (kind === "file") files.push(entry.name);
        if (kind !== "directory" || !descend) continue;
        const entryRelative = current.relative ? `${current.relative}/${entry.name}` : entry.name;
        candidates.push({ absolute: entryAbsolute, relative: entryRelative });
      }
      if (current.relative) discovered.push({ relative: current.relative, files });
    }
    if (!candidates.length) break;
    const ignoredPaths = ignored(candidates.map((candidate) => candidate.relative));
    level = candidates.filter((candidate) => !ignoredPaths.has(candidate.relative));
  }
  return discovered;
}
