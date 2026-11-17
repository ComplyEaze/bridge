// SPDX-License-Identifier: Apache-2.0
//
// Holds the files that name the newest published release to scripts/release-text-agreement.mjs. The real
// files must agree; each rule is shown to fail on a copy of the real text changed in exactly one place.
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { once } from "node:events";
import { readFileSync } from "node:fs";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { checkPublished, exitStatus, parseSpan, parseVersion, problemsIn, ReleaseTextError, run } from "./release-text-agreement.mjs";

const read = (path) => readFileSync(new URL(`../${path}`, import.meta.url), "utf8");
const real = {
  manifestVersion: JSON.parse(read("packaging/mcpb/manifest.json")).version,
  readme: read("README.md"),
  security: read("SECURITY.md"),
  registryVersion: JSON.parse(read("registry/server.json")).version,
};

// The tag the real spans name, found by parsing them, so the mutations below follow the repository forward.
const published = parseSpan(real.readme, "current-release", "README.md").version.text;
const next = (version) => version.replace(/\.(\d+)\.(\d+)$/, (_all, minor) => `.${Number(minor) + 1}.0`);
const published2 = next(published);

const replaceOnce = (text, from, to) => {
  assert.equal(text.split(from).length - 1, 1, `the mutation target "${from}" must occur exactly once`);
  return text.replace(from, to);
};
// Move the three sentences about the build in source, as the version pull request does.
const moveSource = (readme, from, to) => {
  let out = replaceOnce(readme, `At a glance, release ${from}.`, `At a glance, release ${to}.`);
  assert.ok(out.includes(`release ${from} installs`), "the README has no 'release X installs' sentence");
  out = out.replaceAll(`release ${from} installs`, `release ${to} installs`);
  return replaceOnce(out, `current development source is version \`${from}\``, `current development source is version \`${to}\``);
};
const moveSpan = (text, name, to) => {
  const open = `<!-- managed:${name} -->`;
  const start = text.indexOf(open) + open.length;
  const end = text.indexOf(`<!-- /managed:${name} -->`);
  return text.slice(0, start) + text.slice(start, end).replaceAll(`mcp-v${published}`, `mcp-v${to}`) + text.slice(end);
};

test("the real files agree on the newest published release and on the build in source", () => {
  assert.deepEqual(problemsIn(real), []);
});

test("the version pull request's state passes: the manifest and the README's source sentences ahead, the spans and the registry file still on the published build", () => {
  const state = { ...real, manifestVersion: published2, readme: moveSource(real.readme, real.manifestVersion, published2) };
  assert.deepEqual(problemsIn(state), []);
});

test("the post-release pull request's state passes: the spans and the registry file moved to the new build together", () => {
  const state = {
    manifestVersion: published2,
    readme: moveSpan(moveSource(real.readme, real.manifestVersion, published2), "current-release", published2),
    security: moveSpan(real.security, "latest-preview", published2),
    registryVersion: published2,
  };
  assert.deepEqual(problemsIn(state), []);
});

test("a span, or the registry file, ahead of the manifest is a problem", () => {
  assert.match(problemsIn({ ...real, readme: moveSpan(real.readme, "current-release", published2) }).join("\n"), /README\.md names mcp-v.* ahead of the manifest/);
  assert.match(problemsIn({ ...real, security: moveSpan(real.security, "latest-preview", published2) }).join("\n"), /SECURITY\.md names mcp-v.* ahead of the manifest/);
  assert.match(problemsIn({ ...real, registryVersion: published2 }).join("\n"), /registry\/server\.json lists .* ahead of the manifest/);
});

test("a half-done post-release pull request is a problem: the spans, or the registry file, left behind", () => {
  const state = { ...real, manifestVersion: published2, readme: moveSource(real.readme, real.manifestVersion, published2) };
  const spanOnly = problemsIn({ ...state, readme: moveSpan(state.readme, "current-release", published2) });
  assert.ok(spanOnly.some((line) => line.includes("README.md names mcp-v") && line.includes("SECURITY.md names")), spanOnly.join("\n"));
  const registryBehind = problemsIn({ ...state, readme: moveSpan(state.readme, "current-release", published2), security: moveSpan(real.security, "latest-preview", published2) });
  assert.ok(registryBehind.some((line) => line.includes("registry/server.json lists")), registryBehind.join("\n"));
});

test("two spans that name one version with two dates is a problem", () => {
  const security = replaceOnce(real.security, /\((\d{1,2}) (\w+) (\d{4})\)<!-- \/managed:latest-preview -->/.exec(real.security)[0], "(1 January 2026)<!-- /managed:latest-preview -->");
  assert.match(problemsIn({ ...real, security }).join("\n"), /dates mcp-v.* 20\d\d-\d\d-\d\d and SECURITY\.md dates it 2026-01-01/);
});

test("the README's sentences about the build in source, left at the old version when the manifest moves, are problems", () => {
  const behind = problemsIn({ ...real, manifestVersion: published2 });
  for (const what of ['"At a glance, release"', '"release X installs"', '"current development source is version"']) {
    assert.ok(behind.some((line) => line.includes(what)), `${what} was not reported: ${behind.join(" | ")}`);
  }
});

test("a statement that cannot be read is refused with its own code, not read as absent", () => {
  const code = (fn) => {
    try {
      fn();
    } catch (error) {
      assert.ok(error instanceof ReleaseTextError, String(error));
      return error.code;
    }
    return "no error";
  };
  assert.equal(code(() => problemsIn({ ...real, readme: real.readme.replace("<!-- managed:current-release -->", "") })), "span_count");
  assert.equal(code(() => problemsIn({ ...real, readme: `${real.readme}\n<!-- managed:current-release -->x<!-- /managed:current-release -->\n` })), "span_count");
  assert.equal(code(() => problemsIn({ ...real, security: real.security.replace("`mcp-v", "`v") })), "span_form");
  assert.equal(code(() => problemsIn({ ...real, security: real.security.replace(/\(\d{1,2} \w+ \d{4}\)<!-- \/managed/, "(31 February 2026)<!-- /managed") })), "span_date");
  assert.equal(code(() => problemsIn({ ...real, registryVersion: "latest" })), "version_unreadable");
  assert.equal(code(() => problemsIn({ ...real, manifestVersion: "0.7" })), "version_unreadable");
  assert.equal(code(() => problemsIn({ ...real, readme: real.readme.replace(/At a glance, release \d+\.\d+\.\d+\./, "At a glance.") })), "sentence_count");
  assert.equal(code(() => problemsIn({ ...real, readme: `${real.readme}\n**At a glance, release ${real.manifestVersion}.**\n` })), "sentence_count");
  assert.equal(code(() => problemsIn({ ...real, readme: real.readme.replaceAll("installs", "offers") })), "sentence_count");
  assert.equal(code(() => problemsIn({ ...real, readme: real.readme.replace(/current development source is version `[^`]+`/, "current source") })), "sentence_count");
});

test("versions compare by number, not by text", () => {
  assert.equal(parseVersion("0.10.0", "x").minor, 10);
  // 0.9.0 is published and 0.10.0 is in source: the spans are behind, which is allowed. Compared as text, 0.9.0 would be ahead.
  const state = {
    manifestVersion: "0.10.0",
    readme: moveSpan(moveSource(real.readme, real.manifestVersion, "0.10.0"), "current-release", "0.9.0"),
    security: moveSpan(real.security, "latest-preview", "0.9.0"),
    registryVersion: "0.9.0",
  };
  assert.deepEqual(problemsIn(state), []);
});

const TAG = "mcp-v0.5.0";
const PUBLISHED_AT = "2026-10-08T02:15:10Z";
const PUBLISHED_DATE = "8 October 2026";
const RELEASES_URL = "https://api.github.com/repos/ComplyEaze/bridge/releases?per_page=100";
const githubRelease = (tag = TAG, published_at = PUBLISHED_AT, draft = false, prerelease = false) => ({ tag_name: tag, published_at, draft, prerelease });
const managedSpan = (name, tag, date) => `<!-- managed:${name} -->[\`${tag}\`](https://github.com/ComplyEaze/bridge/releases/latest) (${date})<!-- /managed:${name} -->`;

function publishedText({ readmeTag = TAG, securityTag = TAG, registryVersion = "0.5.0", readmeDate = PUBLISHED_DATE, securityDate = PUBLISHED_DATE } = {}) {
  return {
    readme: managedSpan("current-release", readmeTag, readmeDate),
    security: managedSpan("latest-preview", securityTag, securityDate),
    registryVersion,
  };
}

async function publishedRoot(t) {
  const root = await mkdtemp(resolve(tmpdir(), "release-text-published-"));
  t.after(() => rm(root, { recursive: true, force: true }));
  await mkdir(resolve(root, "registry"), { recursive: true });
  await writeFile(resolve(root, "README.md"), publishedText().readme);
  await writeFile(resolve(root, "SECURITY.md"), publishedText().security);
  await writeFile(resolve(root, "registry", "server.json"), JSON.stringify({ version: "0.5.0" }));
  return root;
}

function releasesAnswer(releases) {
  return async (url) => {
    assert.equal(url, RELEASES_URL);
    return JSON.stringify(releases);
  };
}

async function refusal(action) {
  try {
    await action();
  } catch (error) {
    assert.ok(error instanceof ReleaseTextError, `expected a ReleaseTextError, got ${error}`);
    return error;
  }
  assert.fail("the command agreed");
}

const commandRefusal = (args, options) => refusal(() => run(args, options));

test("the hand command uses the publication calendar date in India time", async () => {
  const rows = [
    ["mcp-v0.4.0", "2026-10-01T18:55:38Z", "2 October 2026", "2026-10-02"],
    [TAG, PUBLISHED_AT, PUBLISHED_DATE, "2026-10-08"],
    // The two sides of midnight in India: 18:29:59Z is 23:59:59 there, and one second later is the next day.
    ["mcp-v0.4.1", "2026-10-01T18:29:59Z", "1 October 2026", "2026-10-01"],
    ["mcp-v0.4.2", "2026-10-01T18:30:00Z", "2 October 2026", "2026-10-02"],
  ];
  for (const [tag, publishedAt, date, isoDate] of rows) {
    const text = publishedText({ readmeTag: tag, securityTag: tag, registryVersion: tag.slice("mcp-v".length), readmeDate: date, securityDate: date });
    const release = await checkPublished(text, tag, releasesAnswer([githubRelease(tag, publishedAt)]));
    assert.equal(release.tag, tag);
    assert.equal(release.date, isoDate);
  }
});

test("drafts do not count and publication time, not list position, decides the newest release", async () => {
  const releases = [
    githubRelease("mcp-v0.4.0", "2026-10-01T18:55:38Z"),
    githubRelease(),
    githubRelease("mcp-v0.6.0", "2026-10-09T00:00:00Z", true),
    githubRelease("mcp-v0.3.0", "2026-09-01T00:00:00Z"),
  ];
  assert.equal((await checkPublished(publishedText(), TAG, releasesAnswer(releases))).tag, TAG);
});

test("each published-release rule reports its own disagreement", async () => {
  const newer = githubRelease("mcp-v0.6.0", "2026-10-09T00:00:00Z");
  const cases = [
    [{}, [githubRelease(), newer], "GitHub's newest non-draft release is mcp-v0.6.0, not mcp-v0.5.0."],
    [{}, [githubRelease(TAG, PUBLISHED_AT, false, true)], "GitHub still marks mcp-v0.5.0 as a pre-release."],
    [{ readmeTag: "mcp-v0.5.1" }, [githubRelease()], "README.md names mcp-v0.5.1, not mcp-v0.5.0."],
    [{ securityTag: "mcp-v0.5.1" }, [githubRelease()], "SECURITY.md names mcp-v0.5.1, not mcp-v0.5.0."],
    [{ registryVersion: "0.5.1" }, [githubRelease()], "registry/server.json names mcp-v0.5.1, not mcp-v0.5.0."],
    [{ readmeDate: "9 October 2026" }, [githubRelease()], "README.md dates mcp-v0.5.0 2026-10-09, but its GitHub publication date is 2026-10-08 in India time."],
    [{ securityDate: "9 October 2026" }, [githubRelease()], "SECURITY.md dates mcp-v0.5.0 2026-10-09, but its GitHub publication date is 2026-10-08 in India time."],
  ];
  for (const [changes, releases, expected] of cases) {
    const error = await refusal(() => checkPublished(publishedText(changes), TAG, releasesAnswer(releases)));
    assert.equal(error.code, "release_disagrees");
    assert.deepEqual(error.message.split("\n"), [expected]);
  }
});

test("one refusal names every disagreement", async () => {
  const text = publishedText({ readmeTag: "mcp-v0.5.1", securityTag: "mcp-v0.5.2", registryVersion: "0.5.3", readmeDate: "9 October 2026", securityDate: "10 October 2026" });
  const releases = [githubRelease(), githubRelease("mcp-v0.6.0", "2026-10-11T00:00:00Z")];
  const error = await refusal(() => checkPublished(text, TAG, releasesAnswer(releases)));
  assert.equal(error.code, "release_disagrees");
  assert.equal(error.message.split("\n").length, 6);
});

test("an empty or all-draft answer disagrees because it has no newest published release", async () => {
  for (const releases of [[], [githubRelease("mcp-v0.6.0", null, true)]]) {
    const error = await refusal(() => checkPublished(publishedText(), TAG, releasesAnswer(releases)));
    assert.equal(error.code, "release_disagrees");
  }
});

test("request and answer failures have typed codes and never become agreement", async () => {
  const missingTime = { tag_name: TAG, draft: false, prerelease: false };
  const badDraft = { tag_name: TAG, published_at: PUBLISHED_AT, draft: "false", prerelease: false };
  const cases = [
    ["github_unavailable", async () => { throw new Error("connection failed"); }],
    ["github_unavailable", async () => { throw new Error("HTTP 503"); }],
    ["github_unreadable", async () => "not JSON"],
    ["github_unreadable", async () => JSON.stringify({ releases: [] })],
    ["github_unreadable", releasesAnswer([null])],
    ["github_unreadable", releasesAnswer([[]])],
    ["github_unreadable", releasesAnswer([githubRelease("")])],
    ["github_unreadable", releasesAnswer([badDraft])],
    ["github_unreadable", releasesAnswer([githubRelease(TAG, PUBLISHED_AT, false, "false")])],
    ["github_unreadable", releasesAnswer([missingTime])],
    ["github_unreadable", releasesAnswer([githubRelease(TAG, "2026-10-08")])],
    ["github_unreadable", releasesAnswer([githubRelease(TAG, "not a time")])],
  ];
  for (const [code, fetchText] of cases) {
    const error = await refusal(() => checkPublished(publishedText(), TAG, fetchText));
    assert.equal(error.code, code);
  }
});

test("a missing local file is a typed unreadable-input error", async (t) => {
  const never = async () => { throw new Error("no network in this test"); };
  for (const path of ["README.md", "SECURITY.md", "registry/server.json"]) {
    const root = await publishedRoot(t);
    await rm(resolve(root, path));
    const error = await commandRefusal(["--published", TAG], { root, fetchText: never });
    assert.equal(error.code, "local_file_unreadable");
    assert.ok(error.message.startsWith(`${path} cannot be read:`), error.message);
  }
});

test("registry JSON that cannot be parsed is a typed unreadable-input error", async (t) => {
  const root = await publishedRoot(t);
  await writeFile(resolve(root, "registry", "server.json"), "not JSON");
  const error = await commandRefusal(["--published", TAG], { root, fetchText: async () => { throw new Error("no network in this test"); } });
  assert.equal(error.code, "registry_unreadable");
});

test("the command agrees from the files on disk and prints the tag and its publication date", async (t) => {
  const root = await publishedRoot(t);
  const line = await run(["--published", TAG], { root, fetchText: releasesAnswer([githubRelease()]) });
  assert.equal(line, `README.md, SECURITY.md and registry/server.json agree with ${TAG} (published 2026-10-08 in India time)`);
});

test("the hand command rejects usage mistakes before asking GitHub", async () => {
  const never = async () => { throw new Error("no network in this test"); };
  for (const args of [[], ["--published"], ["--published", "--unknown"], ["--unknown"], ["--published", TAG, "extra"]]) {
    const error = await commandRefusal(args, { fetchText: never });
    assert.equal(error.code, "usage");
  }
});

test("every error code maps to disagreement or could-not-determine", () => {
  const codes = [
    "release_disagrees",
    "usage",
    "version_unreadable",
    "span_count",
    "span_form",
    "span_date",
    "sentence_count",
    "github_unavailable",
    "github_unreadable",
    "registry_unreadable",
    "local_file_unreadable",
  ];
  for (const code of codes) {
    assert.equal(exitStatus(new ReleaseTextError(code, "detail")), code === "release_disagrees" ? 1 : 2, code);
  }
  assert.equal(exitStatus(new Error("unexpected")), 2);
});

test("a child process reports a usage code and exit status without asking GitHub", async () => {
  const script = fileURLToPath(new URL("./release-text-agreement.mjs", import.meta.url));
  const child = spawn(process.execPath, [script, "--published"], { stdio: ["ignore", "ignore", "pipe"] });
  let stderr = "";
  child.stderr.setEncoding("utf8");
  child.stderr.on("data", (chunk) => { stderr += chunk; });
  const [status] = await once(child, "close");
  assert.equal(status, 2);
  assert.equal(stderr.split("\n")[0], "usage");
});
