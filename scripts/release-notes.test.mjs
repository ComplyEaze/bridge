import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { parseDocument } from "yaml";
import { FRAGMENT_DIRECTORY, TEMPLATE_MARKER, assembleFragments, composeBody, fallbackPage, fragmentProblems, mergeFragments, parseChangelog, parseChangelogWithProblems, pickNotes, readFragments, renderBody, renderPage, versionFromTag, withFragments } from "./release-notes.mjs";
import { loadRules, tierOf } from "./review-tier.mjs";

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
  assert.equal(composeBody({ mode: "version", body: "Body" }, "Footer.\n", "Header.\n"), "Header.\n\n---\n\nBody\n\n---\n\nFooter.\n");
  assert.equal(composeBody({ mode: "none", body: "" }, "Footer.\n", "Header.\n"), "Header.\n\n---\n\nFooter.\n");
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
  assert.match(html, /<strong>&lt;b&gt;bold&lt;\/b&gt;<\/strong> \(<a href="https:\/\/github\.com\/ComplyEaze\/bridge\/issues\/77">#77<\/a>\)/);
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
    writeFileSync(join(dir, "packaging", "mcpb", "INSTALL_AND_CHECK_RELEASE.md"), "Header.\n");
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
    assert.equal(compose(dir, "mcp-preview-0.4.0").notes, "Header.\n\n---\n\n### In plain words\n\n- Four (#12).\n\n```\n## [9.9.9] - inside a fence, not a section\n```\n\n---\n\nFooter.\n");
    const unreleased = compose(dir, "mcp-preview-0.5.0");
    assert.equal(unreleased.notes, "Header.\n\n---\n\nNext words.\n\n---\n\nFooter.\n");
    assert.match(unreleased.stdout, /::warning::.*no 0\.5\.0 section/);
    writeFileSync(join(dir, "CHANGELOG.md"), "## [0.3.0] - 2026-09-26\n\n- Three.\n");
    const none = compose(dir, "mcp-preview-0.5.0");
    assert.equal(none.notes, "Header.\n\n---\n\nFooter.\n");
    assert.match(none.stdout, /::warning::.*no section/);
    rmSync(join(dir, "CHANGELOG.md"));
    const missing = compose(dir, "mcp-preview-0.5.0");
    assert.equal(missing.notes, "Header.\n\n---\n\nFooter.\n");
    assert.match(missing.stdout, /::warning::could not read release notes/);
  });
  inTemporaryRepository((dir) => {
    copyFileSync(join(root, "scripts", "release-notes.mjs"), join(dir, "scripts", "release-notes.mjs"));
    writeFileSync(join(dir, "CHANGELOG.md"), changelog);
    rmSync(join(dir, "packaging", "mcpb", "INSTALL_AND_CHECK_RELEASE.md"));
    const headerless = compose(dir, "mcp-preview-0.5.0");
    assert.equal(headerless.notes, "Next words.\n\n---\n\nFooter.\n");
    assert.match(headerless.stdout, /::warning::could not read the release header/);
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

  const deploy = workflow(".github/workflows/deploy-install-page.yml").jobs.build;
  const render = step(deploy, "Render the changelog page");
  const upload = deploy.steps.find((candidate) => candidate.uses?.startsWith("actions/upload-pages-artifact@"));
  assert.ok(deploy.steps.indexOf(render) < deploy.steps.indexOf(upload), "the changelog page is rendered before the site is uploaded");
  assert.match(render.run, /node scripts\/release-notes\.mjs page --changelog CHANGELOG\.md --template site\/changelog\.template\.html --out site\/changelog\.html/);
  assert.match(read(".gitignore"), /^site\/changelog\.html$/m);
  assert.doesNotMatch(read(".gitignore"), /changelog\.template/);
  assert.match(read("site/index.html"), /href="\.\/changelog\.html"/);
});

function writeEntries(dir, files) {
  for (const [path, text] of Object.entries(files)) {
    mkdirSync(join(dir, FRAGMENT_DIRECTORY, path, ".."), { recursive: true });
    writeFileSync(join(dir, FRAGMENT_DIRECTORY, path), text);
  }
}

test("an unreleased entry is one bullet that cites the number in its file name", () => {
  assert.deepEqual(fragmentProblems(12, "- One (#12).\n"), []);
  assert.deepEqual(fragmentProblems(12, "- One\n  wrapped\n  - inner, part of #12\n"), []);
  const bad = {
    "no final line feed": "- One (#12).",
    "a blank line at the end": "- One (#12).\n\n",
    "carriage returns": "- One (#12).\r\n",
    "not a bullet": "One (#12).\n",
    "an empty bullet": "- \n",
    "a second bullet at the left edge": "- One (#12).\n- Two.\n",
    "a heading": "- One (#12).\n## [9.9.9]\n",
    "a blank line inside": "- One (#12).\n\n  Two.\n",
    "a code fence": "- One (#12).\n  ```\n  x\n  ```\n",
    "a tilde fence": "- One (#12).\n  ~~~\n  x\n  ~~~\n",
    "a heading in the bullet": "- ## One (#12)\n",
    "a heading inside": "- One (#12).\n  ## Two\n",
    "a setext underline": "- One (#12)\n  ---\n",
    "an empty bullet that cites the number": "- \n  see #12\n",
    "another number": "- One (#123).\n",
    "no number": "- One.\n",
  };
  for (const [what, text] of Object.entries(bad)) assert.notDeepEqual(fragmentProblems(12, text), [], what);
});

test("entries are read in the order they are listed, and a bad file is reported and left out", () => {
  inTemporaryRepository((dir) => {
    assert.deepEqual(readFragments(join(dir, FRAGMENT_DIRECTORY)), { fragments: [], problems: [] }, "no folder is no entries");
    writeEntries(dir, {
      "changed/30-beta.md": "- Beta (#30).\n",
      "changed/9-zeta.md": "- Zeta (#9).\n",
      "added/5-new-tool.md": "- New tool (part of #5).\n  Second line.\n",
      "changed/30-alpha.md": "- Alpha (#30).\n",
      "removed/7-old.md": "- Old (#7).\n",
      "changed/notes.md": "- Not named by number (#1).\n",
      "changed/31-bad.md": "- Wrong number (#32).\n",
      "fixed/8-other.md": "- A kind with no folder (#8).\n",
      "9-root.md": "- In the root folder (#9).\n",
    });
    const read = readFragments(join(dir, FRAGMENT_DIRECTORY));
    assert.deepEqual(read.fragments.map(({ kind, number, slug }) => `${kind}/${number}-${slug}`), ["added/5-new-tool", "changed/9-zeta", "changed/30-alpha", "changed/30-beta", "removed/7-old"]);
    assert.equal(read.problems.length, 4);
    assert.match(read.problems.join("\n"), /unreleased\/"fixed" is not one of added, changed, removed/);
    assert.match(read.problems.join("\n"), /unreleased\/"9-root\.md" is not one of/);
    assert.match(read.problems.join("\n"), /changed\/"notes\.md" is not named/);
    assert.match(read.problems.join("\n"), /changed\/31-bad\.md is not listed: its text must cite #31/);
    assert.equal(assembleFragments(read.fragments), "**Added**\n\n- New tool (part of #5).\n  Second line.\n\n**Changed**\n\n- Zeta (#9).\n- Alpha (#30).\n- Beta (#30).\n\n**Removed**\n\n- Old (#7).");
    assert.equal(assembleFragments([]), "");
  });
});

test("entries are listed after the text of [Unreleased], which is made when the file has none", () => {
  const fragments = [{ kind: "added", number: 5, slug: "x", text: "- New (#5)." }];
  const withText = withFragments(parseChangelog(changelog), fragments);
  assert.equal(withText[0].body, "Next words.\n\n**Added**\n\n- New (#5).");
  assert.equal(withText[1].label, "0.4.0");
  assert.equal(withText[1].body, parseChangelog(changelog)[1].body, "a released section is left as it is");
  const without = withFragments(parseChangelog("## [0.4.0] - 2026-10-01\n\n- Four.\n"), fragments);
  assert.deepEqual(without.map((section) => section.label), ["Unreleased", "0.4.0"]);
  assert.equal(without[0].body, "**Added**\n\n- New (#5).");
  const sections = parseChangelog(changelog);
  assert.equal(withFragments(sections, []), sections, "no entries leave the sections as they are");
});

test("the page and the notes list the entries, and a bad entry warns without failing; unreleased fails on it", () => {
  inTemporaryRepository((dir) => {
    writeFileSync(join(dir, "CHANGELOG.md"), "## [Unreleased]\n\nNext words.\n\n## [1.0.0] - 2026-10-01\n\n- One.\n");
    writeEntries(dir, { "added/5-new-tool.md": "- New tool (#5).\n", "changed/6-bad.md": "No bullet (#6).\n" });
    const run = (...args) => spawnSync("node", ["scripts/release-notes.mjs", ...args], { cwd: root, encoding: "utf8" });
    const page = run("page", "--changelog", join(dir, "CHANGELOG.md"), "--out", join(dir, "out.html"));
    assert.equal(page.status, 0);
    assert.match(page.stdout, /::warning::.*changed\/6-bad\.md is not listed/);
    const html = readFileSync(join(dir, "out.html"), "utf8");
    assert.match(html, /<li>New tool \(<a href="[^"]*issues\/5">#5<\/a>\)\./);
    assert.doesNotMatch(html, /No bullet/);
    const notes = run("notes", "--tag", "mcp-v1.1.0", "--changelog", join(dir, "CHANGELOG.md"), "--footer", join(dir, "packaging", "mcpb", "UNSIGNED_PREVIEW_RELEASE.md"), "--out", join(dir, "notes.md"));
    assert.equal(notes.status, 0);
    assert.match(readFileSync(join(dir, "notes.md"), "utf8"), /Next words\.\n\n\*\*Added\*\*\n\n- New tool \(#5\)\./);
    const refused = run("unreleased", "--changelog", join(dir, "CHANGELOG.md"));
    assert.equal(refused.status, 1);
    assert.match(refused.stderr, /changed\/6-bad\.md is not listed/);
    const missing = run("unreleased", "--changelog", join(dir, "nowhere", "CHANGELOG.md"));
    assert.equal(missing.status, 1);
    assert.match(missing.stderr, /does not exist/);
    // Windows cannot name a file with a line feed in it, so there the case cannot be built.
    if (process.platform !== "win32") {
      writeEntries(dir, { "changed/7-odd\nname.md": "- Odd (#7).\n" });
      const odd = run("page", "--changelog", join(dir, "CHANGELOG.md"), "--out", join(dir, "odd.html"));
      assert.equal(odd.status, 0);
      assert.equal(odd.stdout.split("\n").filter((line) => line !== "" && !line.startsWith("::warning::")).length, 0, "a file name cannot start another workflow command");
      rmSync(join(dir, FRAGMENT_DIRECTORY, "changed", "7-odd\nname.md"));
    }
    rmSync(join(dir, FRAGMENT_DIRECTORY, "changed"), { recursive: true });
    const printed = run("unreleased", "--changelog", join(dir, "CHANGELOG.md"));
    assert.equal(printed.status, 0);
    assert.equal(printed.stdout, "**Added**\n\n- New tool (#5).\n");
  });
});

test("every unreleased entry in the repository is well formed and in the one review tier", () => {
  const entries = readFragments(join(root, FRAGMENT_DIRECTORY));
  assert.deepEqual(entries.problems, []);
  const rules = loadRules(JSON.parse(read("scripts/review-tier-paths.json")));
  for (const { kind, number, slug } of entries.fragments) {
    const path = `${FRAGMENT_DIRECTORY}/${kind}/${number}-${slug}.md`;
    assert.equal(tierOf([path], rules).tier, "one", `${path} must not be named with privacy, security, redact or egress, which the review tiers send to the full tier`);
  }
});

test("entries join the groups the text already has, so no heading appears twice", () => {
  const entries = [
    { kind: "added", number: 5, slug: "a", text: "- New (#5)." },
    { kind: "changed", number: 6, slug: "b", text: "- Moved (#6).\n  More." },
    { kind: "removed", number: 7, slug: "c", text: "- Gone (#7)." },
  ];
  const body = ["Intro.", "", "**Added**", "", "- Old added.", "", "**Changed**", "", "- Old changed.", "  wrapped", "", "Closing words.", ""].join("\n").trim();
  const merged = mergeFragments(body, entries);
  assert.equal(merged, ["Intro.", "", "**Added**", "", "- Old added.", "- New (#5).", "", "**Changed**", "", "- Old changed.", "  wrapped", "", "Closing words.", "- Moved (#6).", "  More.", "", "**Removed**", "", "- Gone (#7)."].join("\n"));
  for (const heading of ["Added", "Changed", "Removed"]) assert.equal(merged.split(`**${heading}**`).length, 2, heading);
  assert.equal(mergeFragments("", entries.slice(0, 1)), "**Added**\n\n- New (#5).");
  assert.equal(mergeFragments("Words.", []), "Words.");
});

test("a dotfile is ignored, and a link or a folder is reported and not listed", () => {
  inTemporaryRepository((dir) => {
    writeEntries(dir, { "added/5-real.md": "- Real (#5).\n", "added/.DS_Store": "x", "changed/.hidden": "x" });
    mkdirSync(join(dir, FRAGMENT_DIRECTORY, "added", "6-folder.md"));
    symlinkSync(join(dir, FRAGMENT_DIRECTORY, "added", "5-real.md"), join(dir, FRAGMENT_DIRECTORY, "added", "7-link.md"));
    const read = readFragments(join(dir, FRAGMENT_DIRECTORY));
    assert.deepEqual(read.fragments.map(({ number }) => number), [5]);
    assert.equal(read.problems.length, 2);
    assert.match(read.problems.join("\n"), /added\/6-folder\.md is not a plain file/);
    assert.match(read.problems.join("\n"), /added\/7-link\.md is not a plain file/);
  });
});

test("an entry still present when a version is built is listed in its notes, with a warning that names it", () => {
  inTemporaryRepository((dir) => {
    writeFileSync(join(dir, "CHANGELOG.md"), "## [Unreleased]\n\nNext words.\n\n## [1.0.0] - 2026-10-01\n\n**Added**\n\n- One.\n");
    writeEntries(dir, { "added/5-late.md": "- Late (#5).\n", "changed/6-later.md": "- Later (#6).\n" });
    const result = spawnSync("node", ["scripts/release-notes.mjs", "notes", "--tag", "mcp-v1.0.0", "--changelog", join(dir, "CHANGELOG.md"), "--footer", join(dir, "packaging", "mcpb", "UNSIGNED_PREVIEW_RELEASE.md"), "--out", join(dir, "notes.md")], { cwd: root, encoding: "utf8" });
    assert.equal(result.status, 0);
    assert.match(result.stdout, /::warning::2 unreleased entries are still in docs\/changelog\/unreleased and were listed in this release's notes \(added\/5-late, changed\/6-later\)/);
    const notes = readFileSync(join(dir, "notes.md"), "utf8");
    assert.match(notes, /\*\*Added\*\*\n\n- One\.\n\n\*\*Also in this build\*\*\n\n\*\*Added\*\*\n\n- Late \(#5\)\.\n\n\*\*Changed\*\*\n\n- Later \(#6\)\./);
  });
});

test("no entry in the repository is misplaced, or already written into a released section", () => {
  const listed = spawnSync("git", ["ls-files", "-z", "docs/changelog"], { cwd: root, encoding: "utf8" });
  assert.equal(listed.status, 0, listed.stderr);
  const misplaced = listed.stdout.split("\0").filter(Boolean).filter((path) => !path.startsWith(`${FRAGMENT_DIRECTORY}/`));
  assert.deepEqual(misplaced, []);
  const released = parseChangelog(read("CHANGELOG.md")).filter((section) => section.label.toLowerCase() !== "unreleased");
  for (const { kind, number, slug, text } of readFragments(join(root, FRAGMENT_DIRECTORY)).fragments) {
    const section = released.find((candidate) => candidate.body.includes(text));
    assert.equal(section, undefined, `${kind}/${number}-${slug} is already in the ${section?.label} section: remove its file`);
  }
});
