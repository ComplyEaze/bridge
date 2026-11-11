// SPDX-License-Identifier: Apache-2.0
//
// Holds the files that name the newest published release to scripts/release-text-agreement.mjs. The real
// files must agree; each rule is shown to fail on a copy of the real text changed in exactly one place.
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import { parseSpan, parseVersion, problemsIn, ReleaseTextError } from "./release-text-agreement.mjs";

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
