import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { parseDocument } from "yaml";
import { TEMPLATE_MARKER, composeBody, fallbackPage, parseChangelog, parseChangelogWithProblems, pickNotes, renderBody, renderPage, versionFromTag } from "./release-notes.mjs";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const read = (path) => readFileSync(join(root, path), "utf8");
const workflow = (path) => parseDocument(read(path)).toJS();
const step = (job, name) => {
  const found = job.steps.find((candidate) => candidate.name === name);
  assert.ok(found, `missing ${name} step`);
  return found;
};

const changelog = [
  "# Changelog",
  "",
  "Intro text that belongs to no section.",
  "",
  "## [Unreleased]",
  "",
  "Next words.",
  "",
  "## [0.4.0] - 2026-10-01",
  "",
  "### In plain words",
  "",
  "- Four (#12).",
  "",
  "```",
  "## [9.9.9] - inside a fence, not a section",
  "```",
  "",
  "## [0.3.0] - 2026-09-26",
  "",
  "- Three.",
  "",
  "[Unreleased]: https://example.invalid/compare",
  "[0.3.0]: https://example.invalid/tag",
  "",
].join("\n");

test("release tags map to changelog versions", () => {
  assert.equal(versionFromTag("mcp-preview-0.3.0"), "0.3.0");
  assert.equal(versionFromTag("mcp-v0.4.0"), "0.4.0");
  assert.equal(versionFromTag("mcp-preview-1.2.3-rc.1"), undefined, "a release is exactly X.Y.Z");
  assert.equal(versionFromTag("v0.1.0"), undefined);
  assert.equal(versionFromTag("mcp-preview-0.3"), undefined);
});

test("the changelog parser keeps fenced text, drops link definitions and reads dates", () => {
  const sections = parseChangelog(changelog);
  assert.deepEqual(sections.map((section) => section.label), ["Unreleased", "0.4.0", "0.3.0"]);
  assert.equal(sections[1].date, "2026-10-01");
  assert.match(sections[1].body, /inside a fence, not a section/);
  assert.equal(sections[2].body, "- Three.");
  assert.doesNotMatch(sections[2].body, /example\.invalid/);
  assert.deepEqual(parseChangelog(changelog.replace(/\n/g, "\r\n")).map((section) => section.label), ["Unreleased", "0.4.0", "0.3.0"]);
});

test("a byte-order mark before the first heading does not hide that section", () => {
  const { sections, problems } = parseChangelogWithProblems("\uFEFF## [0.3.0] - 2026-01-01\n\n- Fixed a thing.\n");
  assert.deepEqual(sections.map(({ label }) => label), ["0.3.0"]);
  assert.deepEqual(problems, []);
});

test("fences follow their own marker, and an unclosed fence cannot hide later sections", () => {
  const labels = (source) => parseChangelog(source).map((section) => section.label);
  const tail = "\n## [0.3.0] - 2026-09-26\n\n- Three.\n";
  assert.deepEqual(labels("## [0.4.0] - 2026-10-01\n\n```\n~~~\n## [9.9.9]\n```\n" + tail), ["0.4.0", "0.3.0"]);
  assert.deepEqual(labels("## [0.4.0] - 2026-10-01\n\n```foo``` starts this line.\n" + tail), ["0.4.0", "0.3.0"]);
  assert.deepEqual(parseChangelogWithProblems("## [0.4.0] - 2026-10-01\n\n```foo``` starts this line.\n" + tail).problems, []);
  assert.deepEqual(labels("## [0.4.0] - 2026-10-01\n\n````\n```\n## [9.9.9]\n````\n" + tail), ["0.4.0", "0.3.0"]);
  const open = parseChangelogWithProblems("## [0.4.0] - 2026-10-01\n\n```\nnever closed\n" + tail);
  assert.deepEqual(open.sections.map((section) => section.label), ["0.4.0", "0.3.0"]);
  assert.match(open.problems.join("\n"), /never closed/);
  assert.deepEqual(parseChangelogWithProblems(changelog).problems, []);
});

test("headings with an en or em dash or a v prefix are sections; other level-two headings are reported", () => {
  const parsed = parseChangelogWithProblems("## [0.4.0] \u2014 2026-10-01\n\n- Four.\n\n## [v0.3.0] \u2013 2026-09-26\n\n- Three.\n\n## Notes\n\nx\n");
  assert.deepEqual(parsed.sections.map((section) => [section.label, section.date]), [["0.4.0", "2026-10-01"], ["0.3.0", "2026-09-26"]]);
  assert.match(parsed.problems.join("\n"), /as one: ## Notes/);
  assert.equal(pickNotes(parsed.sections, "mcp-preview-0.3.0").mode, "version");
});

test("notes fall back from the version section to a non-empty Unreleased to nothing", () => {
  const sections = parseChangelog(changelog);
  assert.deepEqual(pickNotes(sections, "mcp-preview-0.4.0"), { mode: "version", body: sections[1].body });
  assert.deepEqual(pickNotes(sections, "mcp-preview-0.5.0"), { mode: "unreleased", body: "Next words." });
  const emptyUnreleased = parseChangelog("## [Unreleased]\n\n## [0.3.0] - 2026-09-26\n\n- Three.\n");
  assert.deepEqual(pickNotes(emptyUnreleased, "mcp-preview-0.5.0"), { mode: "none", body: "" });
  const emptyOwn = parseChangelog("## [0.5.0] - 2026-10-01\n\n## [Unreleased]\n\nNext words.\n");
  assert.equal(pickNotes(emptyOwn, "mcp-preview-0.5.0").mode, "unreleased");
  assert.throws(() => pickNotes(sections, "v0.1.0"), /not a release tag/);
  assert.equal(composeBody({ mode: "none", body: "" }, "Footer.\n"), "Footer.\n");
  assert.equal(composeBody({ mode: "version", body: "Body" }, "Footer.\n"), "Body\n\n---\n\nFooter.\n");
});

test("the page renderer escapes everything and links only https URLs and issue numbers", () => {
  const html = renderBody([
    "<script>alert(1)</script> a & b \"q\"",
    "",
    "- `<img src=x onerror=alert(1)>` and **<b>bold</b>** (#77)",
    "- [ok](https://example.invalid/a?x=1&y=2) [bad](javascript:alert(1)) [worse](https://x.invalid/\"onmouseover=\"y)",
  ].join("\n"));
  assert.doesNotMatch(html, /<script|<img|<b>/);
  assert.match(html, /&lt;script&gt;alert\(1\)&lt;\/script&gt; a &amp; b &quot;q&quot;/);
  assert.match(html, /<code>&lt;img src=x onerror=alert\(1\)&gt;<\/code>/);
  assert.match(html, /<strong>&lt;b&gt;bold&lt;\/b&gt;<\/strong> \(<a href="https:\/\/github\.com\/lamemustafa\/bridge\/issues\/77">#77<\/a>\)/);
  assert.match(html, /<a href="https:\/\/example\.invalid\/a\?x=1&amp;y=2">ok<\/a>/);
  assert.doesNotMatch(html, /href="javascript/);
  assert.doesNotMatch(html, /href="https:\/\/x\.invalid\/"/);
  const hrefs = [...html.matchAll(/href="([^"]*)"/g)].map((match) => match[1]);
  assert.ok(hrefs.every((href) => href.startsWith("https://")), hrefs.join(" "));
});

test("nested and wrapped list items render as balanced lists", () => {
  const html = renderBody(["- one", "  wrapped", "  - inner a", "  - inner b", "- two", "", "After."].join("\n"));
  assert.equal(html, "<ul>\n<li>one wrapped\n<ul>\n<li>inner a\n</li>\n<li>inner b\n</li>\n</ul></li>\n<li>two\n</li></ul>\n<p>After.</p>");
  const depth = [...html.matchAll(/<(\/?)(ul|li)>/g)].reduce((open, match) => open + (match[1] ? -1 : 1), 0);
  assert.equal(depth, 0, "every list tag must be closed");
});

test("the real CHANGELOG.md parses into sections that render and publish", () => {
  const source = read("CHANGELOG.md");
  const { sections, problems } = parseChangelogWithProblems(source);
  assert.deepEqual(problems, []);
  const labels = sections.map((section) => section.label);
  assert.equal(new Set(labels).size, labels.length, "section labels must be unique");
  assert.equal(labels[0], "Unreleased");
  for (const section of sections) assert.notEqual(section.body, "", `${section.label} must not be empty`);
  const published = pickNotes(sections, "mcp-preview-0.3.0");
  assert.equal(published.mode, "version");
  assert.match(published.body, /^### In plain words: `mcp-preview-0\.3\.0`/);
  const page = renderPage(sections);
  assert.doesNotMatch(page, /<script|javascript:|\]\(/);
  assert.match(page, /<h2>0\.3\.0, <time datetime="2026-09-26">2026-09-26<\/time> <a class="anchor" href="#v0-3-0" aria-label="Link to 0\.3\.0">#<\/a><\/h2>/);
  const hrefs = [...page.matchAll(/href="([^"]*)"/g)].map((match) => match[1]);
  assert.ok(hrefs.every((href) => /^(https:\/\/|\.\/|#v?[a-z0-9-]+$)/.test(href)), hrefs.find((href) => !/^(https:\/\/|\.\/|#v?[a-z0-9-]+$)/.test(href)));
  assert.match(fallbackPage(), /releases/);
});

test("the page markup is the contract the site templates rely on", () => {
  const html = renderPage(parseChangelog(changelog));
  const ids = [...html.matchAll(/<section id="([^"]+)"/g)].map((match) => match[1]);
  assert.deepEqual(ids, ["unreleased", "v0-4-0", "v0-3-0"]);
  assert.match(html, /<nav class="changelog-index" aria-label="Versions"><a href="#unreleased">Unreleased<\/a><a href="#v0-4-0">0\.4\.0<\/a><a href="#v0-3-0">0\.3\.0<\/a><\/nav>\n      <article class="changelog">\n        <h1>What changed<\/h1>/);
  assert.equal([...html.matchAll(/data-latest="true"/g)].length, 1);
  assert.match(html, /<section id="v0-4-0" data-version="0\.4\.0" data-latest="true">/);
  assert.match(html, /<section id="unreleased" data-version="unreleased">\n\s+<h2>In source, not yet published <a class="anchor" href="#unreleased" aria-label="Link to Unreleased">#<\/a><\/h2>/);
  const odd = renderPage(parseChangelog("## [1.0.0] - soon <b>\n\n- One.\n\n## [1.0.0]\n\n- Again.\n\n## [1.0.0-rc.1]\n\n- Rc.\n"));
  assert.deepEqual([...odd.matchAll(/<section id="([^"]+)"/g)].map((match) => match[1]), ["v1-0-0", "v1-0-0-2", "v1-0-0-rc-1"]);
  assert.doesNotMatch(odd, /<time|<b>/);
  assert.match(odd, /1\.0\.0, soon &lt;b&gt;/);
});

test("a template wraps the generated article only when it holds the marker exactly once", () => {
  const sections = parseChangelog("## [1.0.0] - 2026-10-01\n\n- Costs $& and $1 (#5).\n");
  const template = `<html><body class="mine">\n${TEMPLATE_MARKER}\n</body></html>\n`;
  const wrapped = renderPage(sections, template);
  assert.match(wrapped, /^<html><body class="mine">\n      <nav class="changelog-index"/);
  assert.match(wrapped, /Costs \$&amp; and \$1/);
  assert.doesNotMatch(wrapped, /<!-- changelog -->/);
  assert.equal(wrapped.split("<html>").length, 2);
  for (const bad of ["<html></html>", `${TEMPLATE_MARKER}${TEMPLATE_MARKER}`, undefined]) {
    assert.match(renderPage(sections, bad), /^<!doctype html>/, String(bad));
  }
  assert.match(fallbackPage(template), /^<html><body class="mine">[\s\S]*could not be prepared/);
});

function generate(dir, templateText) {
  writeFileSync(join(dir, "CHANGELOG.md"), "## [1.0.0] - 2026-10-01\n\n- One.\n");
  const args = ["scripts/release-notes.mjs", "page", "--changelog", join(dir, "CHANGELOG.md"), "--out", join(dir, "out.html")];
  if (templateText !== null) {
    if (templateText === undefined) rmSync(join(dir, "t.html"), { force: true });
    else writeFileSync(join(dir, "t.html"), templateText);
    args.push("--template", join(dir, "t.html"));
  }
  const result = spawnSync("node", args, { cwd: root, encoding: "utf8" });
  return { result, html: readFileSync(join(dir, "out.html"), "utf8") };
}

test("the page command uses the template, and a missing or malformed one never fails the deploy", () => {
  inTemporaryRepository((dir) => {
    const good = generate(dir, `<p>mine</p>${TEMPLATE_MARKER}`);
    assert.equal(good.result.status, 0);
    assert.match(good.html, /^<p>mine<\/p>      <nav class="changelog-index"/);
    assert.equal(good.result.stdout.includes("::warning::"), false);
    const missing = generate(dir, undefined);
    assert.equal(missing.result.status, 0);
    assert.match(missing.html, /^<!doctype html>/);
    assert.equal(missing.result.stdout.includes("::warning::"), false, "no template yet is normal, not a warning");
    const malformed = generate(dir, "<p>no marker</p>");
    assert.equal(malformed.result.status, 0);
    assert.match(malformed.html, /^<!doctype html>/);
    assert.match(malformed.result.stdout, /::warning::.*exactly once/);
    const none = generate(dir, null);
    assert.equal(none.result.status, 0);
    assert.match(none.html, /^<!doctype html>/);
  });
});

const publish = () => workflow(".github/workflows/release-mcpb-preview.yml").jobs["publish-preview"];

function inTemporaryRepository(callback) {
  const dir = mkdtempSync(join(tmpdir(), "bridge-notes-"));
  try {
    mkdirSync(join(dir, "scripts"));
    mkdirSync(join(dir, "packaging", "mcpb"), { recursive: true });
    mkdirSync(join(dir, "tmp"));
    writeFileSync(join(dir, "packaging", "mcpb", "UNSIGNED_PREVIEW_RELEASE.md"), "Footer.\n");
    return callback(dir);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

function runStep(dir, run, env, prelude = "") {
  const result = spawnSync("bash", ["-eo", "pipefail", "-c", `${prelude}\n${run}`], { cwd: dir, encoding: "utf8", env: { ...process.env, RUNNER_TEMP: join(dir, "tmp"), ...env } });
  return result;
}

test("the compose step publishes the version section, then Unreleased, then the footer alone, and never fails", () => {
  const { run } = step(publish(), "Compose the release notes from CHANGELOG.md");
  const compose = (dir, tag) => {
    const result = runStep(dir, run, { RELEASE_TAG: tag });
    assert.equal(result.status, 0, result.stderr + result.stdout);
    return { ...result, notes: readFileSync(join(dir, "tmp", "release-notes.md"), "utf8") };
  };
  inTemporaryRepository((dir) => {
    copyFileSync(join(root, "scripts", "release-notes.mjs"), join(dir, "scripts", "release-notes.mjs"));
    writeFileSync(join(dir, "CHANGELOG.md"), changelog);
    assert.equal(compose(dir, "mcp-preview-0.4.0").notes, "### In plain words\n\n- Four (#12).\n\n```\n## [9.9.9] - inside a fence, not a section\n```\n\n---\n\nFooter.\n");
    const unreleased = compose(dir, "mcp-preview-0.5.0");
    assert.equal(unreleased.notes, "Next words.\n\n---\n\nFooter.\n");
    assert.match(unreleased.stdout, /::warning::.*no 0\.5\.0 section/);
    writeFileSync(join(dir, "CHANGELOG.md"), "## [0.3.0] - 2026-09-26\n\n- Three.\n");
    const none = compose(dir, "mcp-preview-0.5.0");
    assert.equal(none.notes, "Footer.\n");
    assert.match(none.stdout, /::warning::.*no section/);
    rmSync(join(dir, "CHANGELOG.md"));
    const missing = compose(dir, "mcp-preview-0.5.0");
    assert.equal(missing.notes, "Footer.\n");
    assert.match(missing.stdout, /::warning::could not read release notes/);
  });
  inTemporaryRepository((dir) => {
    const crashed = compose(dir, "mcp-preview-0.5.0");
    assert.equal(crashed.notes, "Footer.\n");
    assert.match(crashed.stdout, /::warning::could not compose release notes/);
  });
});

test("the change-list step appends GitHub's list, and a failure leaves the release notes and the run intact", () => {
  const { run } = step(publish(), "Append GitHub's generated change list");
  assert.equal(run.includes("set -e"), false, "a failed append must not fail the publish");
  const stub = (generate, edit) => `gh() {
  case "$*" in
    "api --method POST repos/example/bridge/releases/generate-notes -f tag_name=mcp-preview-0.4.0 --jq .body") ${generate} ;;
    "release edit mcp-preview-0.4.0 --notes-file "*) ${edit} ;;
    *) echo "unexpected gh $*" >&2; return 97 ;;
  esac
}`;
  const env = { RELEASE_TAG: "mcp-preview-0.4.0", GH_REPO: "example/bridge" };
  const attempt = (generate, edit, written = true) => inTemporaryRepository((dir) => {
    if (written) writeFileSync(join(dir, "tmp", "release-notes.md"), "Written.\n");
    const result = runStep(dir, run, env, stub(generate, edit));
    const readOrUndefined = (path) => { try { return readFileSync(join(dir, path), "utf8"); } catch { return undefined; } };
    return { result, combined: readOrUndefined("tmp/release-notes-combined.md"), edited: readOrUndefined("edited-notes.md") };
  });

  const ok = attempt("printf '## What'\"'\"'s Changed\\n* item'", `cp "$5" edited-notes.md`);
  assert.equal(ok.result.status, 0, ok.result.stderr);
  assert.match(ok.result.stdout, /appended GitHub's generated change list/);
  assert.equal(ok.combined, "Written.\n\n---\n\n## What's Changed\n* item\n");
  assert.equal(ok.edited, ok.combined, "the edit call receives the combined file");

  for (const [label, generate, edit, written] of [
    ["generation fails", "return 1", "true"],
    ["generation is empty", "printf ''", "true"],
    ["the edit fails", "printf 'list'", "return 1"],
    ["the written notes are gone", "printf 'list'", `cp "$5" edited-notes.md`, false],
  ]) {
    const failed = attempt(generate, edit, written);
    assert.equal(failed.edited === undefined || label === "the edit fails", true, `${label}: the release notes must not be replaced`);
    assert.equal(failed.result.status, 0, `${label}: ${failed.result.stderr}`);
    assert.match(failed.result.stdout, /::warning::could not append/, label);
  }
});

test("the publish and deploy workflows keep the notes wiring in order", () => {
  const job = publish();
  const names = job.steps.map((candidate) => candidate.name);
  const create = step(job, "Create the immutable GitHub preview release");
  const compose = step(job, "Compose the release notes from CHANGELOG.md");
  const append = step(job, "Append GitHub's generated change list");
  assert.ok(names.indexOf(compose.name) < names.indexOf(create.name), "notes are composed before the release is created");
  assert.ok(names.indexOf(append.name) > names.indexOf(create.name), "the change list is appended after the release exists");
  assert.match(create.run, /--notes-file "\$RUNNER_TEMP\/release-notes\.md"/);
  assert.doesNotMatch(create.run, /UNSIGNED_PREVIEW_RELEASE|--generate-notes/);
  for (const optional of [compose, append]) {
    assert.equal(optional["continue-on-error"], undefined);
    assert.equal(optional.if, undefined);
  }

  const deploy = workflow(".github/workflows/deploy-install-page.yml").jobs.deploy;
  const render = step(deploy, "Render the changelog page");
  const upload = deploy.steps.find((candidate) => candidate.uses?.startsWith("actions/upload-pages-artifact@"));
  assert.ok(deploy.steps.indexOf(render) < deploy.steps.indexOf(upload), "the changelog page is rendered before the site is uploaded");
  assert.match(render.run, /node scripts\/release-notes\.mjs page --changelog CHANGELOG\.md --template site\/changelog\.template\.html --out site\/changelog\.html/);
  assert.match(read(".gitignore"), /^site\/changelog\.html$/m);
  assert.doesNotMatch(read(".gitignore"), /changelog\.template/);
  assert.match(read("site/index.html"), /href="\.\/changelog\.html"/);
});
