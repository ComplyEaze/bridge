// SPDX-License-Identifier: Apache-2.0

import { spawnSync } from "node:child_process";
import { readFileSync, readdirSync, realpathSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join, relative } from "node:path";
import {
  FIXTURE_ROOTS,
  classifyEntry,
  unregisteredFixtureDirectories,
} from "./fixture-roots.mjs";

const repositoryRoot = fileURLToPath(new URL("../", import.meta.url));
// Found by name or by content (fixture-roots.mjs, #838).
const unexpectedFixtureDirectories = unregisteredFixtureDirectories(
  repositoryRoot,
  gitIgnoredPaths,
);
if (unexpectedFixtureDirectories.length) {
  throw new Error(
    "unexpected fixture directories are not covered by byte-integrity policy:\n" +
      unexpectedFixtureDirectories.map((directory) => `- ${directory}`).join("\n") +
      "\nRegister each directory in FIXTURE_ROOTS (scripts/fixture-roots.mjs) and " +
      ".gitattributes before adding fixtures.",
  );
}
const fixtures = FIXTURE_ROOTS
  .flatMap((directory) => {
    const paths = walkFiles(join(repositoryRoot, directory));
    if (!paths.length) throw new Error(`fixture tree is empty: ${directory}`);
    return paths.map((path) => relative(repositoryRoot, path).replaceAll("\\", "/"));
  })
  .sort();

if (!fixtures.length) {
  throw new Error("fixture trees are empty");
}

// Paths go on stdin: every fixture as an argument is past Windows' command-line limit (#1471).
const attributes = runGit(["check-attr", "-z", "--stdin", "text"], { input: `${fixtures.join("\0")}\0` });
const attributeRecords = attributes.stdout.toString("utf8").split("\0");
// One record per path read, so input Git split differently cannot pass as "every fixture checked".
if (attributeRecords.length - 1 !== fixtures.length * 3) {
  throw new Error(
    `git check-attr returned ${(attributeRecords.length - 1) / 3} record(s) for ${fixtures.length} fixture(s)`,
  );
}
const attributeFailures = [];
for (let index = 0; index < attributeRecords.length - 1; index += 3) {
  const [path, attribute, value] = attributeRecords.slice(index, index + 3);
  if (attribute !== "text" || value !== "unset") {
    attributeFailures.push(`${path}: ${attribute ?? "missing"}=${value ?? "missing"}`);
  }
}
if (attributeFailures.length) {
  throw new Error(
    `fixtures must opt out of text normalization:\n${attributeFailures.join("\n")}`,
  );
}

// Which fixtures already have a blob at HEAD is established by an explicit,
// batched `git ls-tree` lookup — never by interpreting a `git show` failure.
// That keeps "this path has no HEAD blob yet" (fine, it's a brand-new
// fixture) distinct from "git show failed for some other reason"
// (permissions, a corrupt object, git missing, a broken pipe, ...), which
// must fail the gate loudly instead of being read as "new file, skip".
const trackedFixtures = gitTrackedFixturePaths(fixtures);

const byteFailures = [];
let newFixtureCount = 0;
for (const fixture of fixtures) {
  if (!trackedFixtures.has(fixture)) {
    // A new fixture has no blob until its first commit. Attribute coverage still
    // protects its bytes during that window; comparison starts once evidence is
    // committed. Existing fixtures always have a HEAD blob and remain strict.
    newFixtureCount += 1;
    continue;
  }
  // In CI the checkout materialises this file from the same HEAD blob, so this
  // comparison cannot independently establish captured-byte provenance there.
  // In CI that is the checked .gitattributes rule, and the committed digest
  // every fixture's provenance row declares (check-fixture-provenance.mjs,
  // #838); this comparison still catches local worktree conversion or mutation.
  //
  // `trackedFixtures` already proved this path has a HEAD blob, so any
  // failure here is a real problem, not a "new file" — runGit throws instead
  // of swallowing the failure.
  const committed = runGit(["show", "--no-textconv", `HEAD:${fixture}`]);
  const worktree = readFileSync(join(repositoryRoot, fixture));
  if (!worktree.equals(committed.stdout)) {
    byteFailures.push(
      `${fixture}: worktree=${worktree.length} bytes, HEAD=${committed.stdout.length} bytes`,
    );
  }
}
if (byteFailures.length) {
  throw new Error(
    `protocol fixture bytes differ from committed evidence:\n${byteFailures.join("\n")}`,
  );
}

console.log(
  `Fixture byte integrity is sealed (${fixtures.length} files, ${newFixtureCount} new).`,
);

// Walks a fixture directory for files, following symlinked subdirectories
// (and symlinked files) instead of silently dropping them. `visited` guards
// against symlink cycles by tracking canonical (realpath'd) directories.
function walkFiles(directory, visited = new Set()) {
  const canonical = realpathSync(directory);
  if (visited.has(canonical)) return [];
  visited.add(canonical);
  const paths = [];
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    const kind = classifyEntry(path, entry);
    if (kind === "directory") paths.push(...walkFiles(path, visited));
    else if (kind === "file") paths.push(path);
  }
  return paths;
}

function gitIgnoredPaths(paths) {
  if (!paths.length) return new Set();
  const result = runGit(["check-ignore", "-z", "--stdin"], {
    allowFailure: true,
    input: `${paths.join("\0")}\0`,
  });
  if (result.status !== 0 && result.status !== 1) {
    throw new Error(`git check-ignore failed with status ${result.status}`);
  }
  return new Set(
    result.stdout
      .toString("utf8")
      .split("\0")
      .filter(Boolean)
      .map((path) => path.replaceAll("\\", "/")),
  );
}

// Returns the subset of `paths` that have a blob at HEAD, via a single
// batched `git ls-tree` lookup. A path missing from the result is simply
// absent from the HEAD tree (a brand-new fixture); a failure of the
// `ls-tree` invocation itself (bad HEAD, corrupt repo, git missing, ...)
// throws via runGit's default (non-allowFailure) behaviour instead of being
// mistaken for "every path is new".
//
// The lookup names the fixture roots, not each path: `ls-tree` has no --stdin,
// and every fixture as an argument is past Windows' command-line limit (#1471).
// Every path is under a root, so the subset is the same.
function gitTrackedFixturePaths(paths) {
  if (!paths.length) return new Set();
  const wanted = new Set(paths);
  const result = runGit(["ls-tree", "-r", "-z", "--name-only", "HEAD", "--", ...FIXTURE_ROOTS]);
  return new Set(
    result.stdout
      .toString("utf8")
      .split("\0")
      .filter(Boolean)
      .map((path) => path.replaceAll("\\", "/"))
      .filter((path) => wanted.has(path)),
  );
}

function runGit(args, { allowFailure = false, input } = {}) {
  const result = spawnSync("git", args, {
    cwd: repositoryRoot,
    input,
    maxBuffer: 64 * 1024 * 1024,
    windowsHide: true,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    const detail = result.error?.message ?? result.stderr?.toString("utf8") ?? "unknown error";
    throw new Error(`git ${args.join(" ")} failed: ${detail}`);
  }
  return result;
}
