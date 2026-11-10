// SPDX-License-Identifier: Apache-2.0
//
// Files that name "the newest published release" must agree with each other and with the manifest
// (docs/release-process.md, release step 5 and "When a listing exists"). This module parses each such
// statement once into a record and applies the rules to the records; scripts/release-text-agreement.test.mjs
// holds the repository to them. It reads text and writes nothing, and asks nobody over the network.
//
// The statements:
//   - two marked spans, `managed:current-release` in README.md and `managed:latest-preview` in SECURITY.md,
//     each naming a tag and a date: the newest PUBLISHED build;
//   - registry/server.json's `version`: the build the registry lists, moved with the spans;
//   - README.md's "At a glance, release X", "release X installs" and "current development source is version
//     `X`": the build in SOURCE, which the version pull request moves to the manifest's version.
// The spans and the registry file may be behind the manifest (between the version pull request and the
// post-release pull request the manifest is ahead). They may not be ahead of it, and they must name one version.

const MONTHS = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
const SPAN_BODY = new RegExp(
  `^\\[\`mcp-v(\\d+)\\.(\\d+)\\.(\\d+)\`\\]\\(https://github\\.com/ComplyEaze/bridge/releases/latest\\) \\((\\d{1,2}) (${MONTHS.join("|")}) (\\d{4})\\)$`,
);

// A statement that cannot be read is a refusal, never "nothing found".
export class ReleaseTextError extends Error {
  constructor(code, message) {
    super(message);
    this.name = "ReleaseTextError";
    this.code = code;
  }
}

export function parseVersion(text, where) {
  const match = /^(\d+)\.(\d+)\.(\d+)$/.exec(text);
  if (!match) throw new ReleaseTextError("version_unreadable", `${where}: "${text}" is not a version of the form X.Y.Z`);
  return { major: Number(match[1]), minor: Number(match[2]), patch: Number(match[3]), text };
}

export function compareVersions(left, right) {
  return left.major - right.major || left.minor - right.minor || left.patch - right.patch;
}

// One span, once: its tag, version and date. Anything else is an error.
export function parseSpan(text, name, where) {
  const open = `<!-- managed:${name} -->`;
  const close = `<!-- /managed:${name} -->`;
  const opens = text.split(open).length - 1;
  const closes = text.split(close).length - 1;
  if (opens !== 1 || closes !== 1) {
    throw new ReleaseTextError("span_count", `${where}: expected exactly one managed:${name} span, found ${opens} opening and ${closes} closing marks`);
  }
  const body = text.slice(text.indexOf(open) + open.length, text.indexOf(close));
  const match = SPAN_BODY.exec(body);
  if (!match) throw new ReleaseTextError("span_form", `${where}: managed:${name} reads "${body}", not a tag link and a date such as [\`mcp-v1.2.3\`](https://github.com/ComplyEaze/bridge/releases/latest) (30 October 2026)`);
  const [, major, minor, patch, day, month, year] = match;
  const monthIndex = MONTHS.indexOf(month);
  const stamp = new Date(Date.UTC(Number(year), monthIndex, Number(day)));
  if (stamp.getUTCFullYear() !== Number(year) || stamp.getUTCMonth() !== monthIndex || stamp.getUTCDate() !== Number(day)) {
    throw new ReleaseTextError("span_date", `${where}: managed:${name} names ${day} ${month} ${year}, which is not a calendar date`);
  }
  const version = parseVersion(`${major}.${minor}.${patch}`, `${where} managed:${name}`);
  return { tag: `mcp-v${version.text}`, version, date: `${year}-${String(monthIndex + 1).padStart(2, "0")}-${String(day).padStart(2, "0")}` };
}

// Every version a sentence form names (a line break inside the sentence is allowed; every match of "release X installs" must name the manifest's version, so a historical aside in that form would fail) in a text, at least `least` times, each parsed.
function namedVersions(text, pattern, where, least, exactly) {
  const found = [...text.matchAll(pattern)].map((match) => parseVersion(match[1], where));
  if (exactly !== undefined && found.length !== exactly) {
    throw new ReleaseTextError("sentence_count", `${where}: expected ${exactly} occurrence(s), found ${found.length}`);
  }
  if (found.length < least) throw new ReleaseTextError("sentence_count", `${where}: expected at least ${least} occurrence(s), found ${found.length}`);
  return found;
}

// The problems in the statements, each a sentence naming the file, or [] when they agree. A statement that
// cannot be parsed throws ReleaseTextError.
export function problemsIn({ manifestVersion, readme, security, registryVersion }) {
  const manifest = parseVersion(manifestVersion, "packaging/mcpb/manifest.json");
  const registry = parseVersion(registryVersion, "registry/server.json");
  const readmeSpan = parseSpan(readme, "current-release", "README.md");
  const securitySpan = parseSpan(security, "latest-preview", "SECURITY.md");
  const problems = [];

  for (const [file, span] of [["README.md", readmeSpan], ["SECURITY.md", securitySpan]]) {
    if (compareVersions(span.version, manifest) > 0) problems.push(`${file} names ${span.tag} as the newest published release, which is ahead of the manifest (${manifest.text})`);
  }
  if (compareVersions(registry, manifest) > 0) problems.push(`registry/server.json lists ${registry.text}, which is ahead of the manifest (${manifest.text})`);
  if (compareVersions(readmeSpan.version, securitySpan.version) !== 0) problems.push(`README.md names ${readmeSpan.tag} and SECURITY.md names ${securitySpan.tag} as the newest published release`);
  if (compareVersions(readmeSpan.version, registry) !== 0) problems.push(`README.md names ${readmeSpan.tag} as the newest published release and registry/server.json lists ${registry.text}`);
  if (readmeSpan.date !== securitySpan.date && compareVersions(readmeSpan.version, securitySpan.version) === 0) {
    problems.push(`README.md dates ${readmeSpan.tag} ${readmeSpan.date} and SECURITY.md dates it ${securitySpan.date}`);
  }

  const glance = namedVersions(readme, /At\s+a\s+glance,\s+release\s+(\d+\.\d+\.\d+)\./g, "README.md \"At a glance, release X.\"", 1, 1);
  const installs = namedVersions(readme, /release\s+(\d+\.\d+\.\d+)\s+installs/g, "README.md \"release X installs\"", 1);
  const source = namedVersions(readme, /current\s+development\s+source\s+is\s+version\s+`(\d+\.\d+\.\d+)`/g, "README.md \"current development source is version\"", 1, 1);
  for (const [what, versions] of [["\"At a glance, release\"", glance], ["\"release X installs\"", installs], ["\"current development source is version\"", source]]) {
    for (const version of versions) {
      if (compareVersions(version, manifest) !== 0) problems.push(`README.md ${what} names ${version.text} and the manifest says ${manifest.text}`);
    }
  }
  return problems;
}
