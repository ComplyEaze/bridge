// One source for what changed: CHANGELOG.md. `notes` composes a GitHub release body from it and
// `page` renders the same file for the install site, so the two cannot drift. Neither may block a
// release or a deploy: every fallback prints a warning and still writes a usable file.
import { readFileSync, writeFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const REPOSITORY_URL = "https://github.com/ComplyEaze/bridge";
// The current `mcp-vX.Y.Z` form and the older `mcp-preview-X.Y.Z` (see scripts/release-tag-forms.test.mjs).
const previewTag = /^mcp-(?:preview-|v)([0-9]+\.[0-9]+\.[0-9]+)$/;
const sectionHeading = /^## \[(?:v(?=\d))?([^\]]+)\](?:\s+[-\u2013\u2014]\s+(\S.*))?\s*$/;
const linkDefinition = /^\[[^\]]+\]: \S+\s*$/;
const fenceLine = /^\s*(`{3,}|~{3,})(.*)$/;

export function versionFromTag(tag) {
  return previewTag.exec(tag)?.[1];
}

// A fence opens on three or more backticks or tildes and closes only on the same character, at
// least as many, with nothing after them. A backtick fence's info string cannot hold a backtick,
// so an inline ```code``` span is not a fence.
function nextFence(line, open) {
  const match = fenceLine.exec(line);
  if (open) {
    const closes = match && match[1][0] === open.char && match[1].length >= open.length && match[2].trim() === "";
    return closes ? undefined : open;
  }
  if (!match || (match[1][0] === "`" && match[2].includes("`"))) return undefined;
  return { char: match[1][0], length: match[1].length };
}

function scan(source, trackFences) {
  source = source.replace(/^\uFEFF/, "");
  const sections = [];
  const strays = [];
  let current;
  let open;
  for (const line of source.replace(/\r\n?/g, "\n").split("\n")) {
    const before = open;
    if (trackFences) open = nextFence(line, open);
    const inFence = before !== undefined || open !== undefined;
    const heading = !inFence && sectionHeading.exec(line);
    if (heading) {
      current = { label: heading[1], date: heading[2], lines: [] };
      sections.push(current);
    } else if (current && (inFence || !linkDefinition.test(line))) {
      current.lines.push(line);
    }
    if (!inFence && !heading && /^## /.test(line)) strays.push(line);
  }
  return { sections: sections.map(({ label, date, lines }) => ({ label, date, body: lines.join("\n").trim() })), strays, unterminatedFence: open !== undefined };
}

// Sections open at a level-two `## [label]` heading outside a fenced block. Link reference
// definitions (`[0.1.0]: https://...`) trail the last section and are not part of its text. A
// fence that never closes would swallow every later section, so it is read again ignoring fences
// and reported through `problems`.
export function parseChangelogWithProblems(source) {
  const first = scan(source, true);
  const result = first.unterminatedFence ? scan(source, false) : first;
  const problems = [];
  if (first.unterminatedFence) problems.push("a code fence in CHANGELOG.md is never closed; sections were read ignoring fences");
  for (const line of result.strays) problems.push(`CHANGELOG.md has a level-two heading that is not a section, so it is not published as one: ${line}`);
  return { sections: result.sections, problems };
}

export function parseChangelog(source) {
  return parseChangelogWithProblems(source).sections;
}

export function pickNotes(sections, tag) {
  const version = versionFromTag(tag);
  if (version === undefined) throw new Error(`not a release tag (mcp-v or mcp-preview-): ${tag}`);
  const own = sections.find((section) => section.label === version && section.body !== "");
  if (own) return { mode: "version", body: own.body };
  const unreleased = sections.find((section) => section.label.toLowerCase() === "unreleased" && section.body !== "");
  if (unreleased) return { mode: "unreleased", body: unreleased.body };
  return { mode: "none", body: "" };
}

export function composeBody(picked, footer) {
  const tail = footer.trim();
  return picked.body === "" ? `${tail}\n` : `${picked.body}\n\n---\n\n${tail}\n`;
}

const escapeHtml = (text) => text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");

// A single pass over the raw text: every emitted piece is escaped or built from a matched token,
// so nothing from the changelog reaches the page as markup.
const inlineToken = /`([^`\n]+)`|\*\*([^*\n]+)\*\*|\[([^\]\n]+)\]\((https:\/\/[^)\s"<>]+)\)|(?<![\w&/#])#(\d{1,6})\b/g;

function inline(text) {
  let html = "";
  let last = 0;
  for (const match of text.matchAll(inlineToken)) {
    html += escapeHtml(text.slice(last, match.index));
    const [, code, bold, linkText, url, issue] = match;
    if (code !== undefined) html += `<code>${escapeHtml(code)}</code>`;
    else if (bold !== undefined) html += `<strong>${escapeHtml(bold)}</strong>`;
    else if (url !== undefined) html += `<a href="${escapeHtml(url)}">${escapeHtml(linkText)}</a>`;
    else html += `<a href="${REPOSITORY_URL}/issues/${issue}">#${issue}</a>`;
    last = match.index + match[0].length;
  }
  return html + escapeHtml(text.slice(last));
}

function blocks(body) {
  const out = [];
  let paragraph = [];
  let item;
  const flush = () => {
    if (paragraph.length) out.push({ type: "p", text: paragraph.join(" ") });
    paragraph = [];
    item = undefined;
  };
  let open;
  for (const line of body.split("\n")) {
    const before = open;
    open = nextFence(line, open);
    if (before !== undefined || open !== undefined) continue;
    if (line.trim() === "") {
      flush();
      continue;
    }
    const heading = /^(#{3,6}) (.+)$/.exec(line);
    const bullet = /^(\s*)[-*] (.+)$/.exec(line);
    if (heading) {
      flush();
      out.push({ type: "h", level: heading[1].length, text: heading[2] });
    } else if (bullet) {
      flush();
      item = { type: "li", depth: Math.min(Math.floor(bullet[1].length / 2), 3), text: bullet[2] };
      out.push(item);
    } else if (item) {
      item.text += ` ${line.trim()}`;
    } else {
      paragraph.push(line.trim());
    }
  }
  flush();
  return out;
}

export function renderBody(body) {
  const html = [];
  let depth = -1;
  for (const block of blocks(body)) {
    if (block.type !== "li") {
      while (depth >= 0) html.push("</li></ul>"), depth -= 1;
    }
    if (block.type === "h") html.push(`<h${block.level}>${inline(block.text)}</h${block.level}>`);
    else if (block.type === "p") html.push(`<p>${inline(block.text)}</p>`);
    else {
      const target = Math.min(block.depth, depth + 1);
      if (target > depth) {
        html.push("<ul>");
        depth += 1;
      } else {
        html.push("</li>");
        while (depth > target) html.push("</ul></li>"), depth -= 1;
      }
      html.push(`<li>${inline(block.text)}`);
    }
  }
  while (depth >= 0) html.push("</li></ul>"), depth -= 1;
  return html.join("\n");
}

const pageHead = `<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>What changed in ComplyEaze Bridge</title>
    <link rel="stylesheet" href="./styles.css" />
  </head>
  <body>
    <main class="page">
      <header class="masthead">
        <a class="wordmark" href="./">ComplyEaze Bridge</a>
        <nav class="masthead-links"><a class="quiet-link" href="./">Install</a><a class="quiet-link" href="${REPOSITORY_URL}/releases">All releases</a></nav>
      </header>
`;
const pageTail = `    </main>
  </body>
</html>
`;

export const TEMPLATE_MARKER = "<!-- changelog -->";

const isUnreleased = (label) => label.toLowerCase() === "unreleased";

// One anchor per section, unique even if the changelog repeats a label: "0.3.0" is "v0-3-0".
function sectionIds(sections) {
  const seen = new Map();
  return sections.map(({ label }) => {
    const base = isUnreleased(label) ? "unreleased" : `v${label.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "")}`;
    const count = (seen.get(base) ?? 0) + 1;
    seen.set(base, count);
    return count === 1 ? base : `${base}-${count}`;
  });
}

// The generated part of the page: a version index, then one section per changelog entry. A
// template supplies everything around it, so restyling never touches generated output.
export function renderArticle(sections) {
  const ids = sectionIds(sections);
  const latest = sections.findIndex(({ label }) => !isUnreleased(label));
  const index = sections.map(({ label }, at) => `<a href="#${ids[at]}">${escapeHtml(label)}</a>`).join("");
  const parts = sections.map(({ label, date, body }, at) => {
    const unreleased = isUnreleased(label);
    const when = !date ? "" : /^\d{4}-\d{2}-\d{2}$/.test(date) ? `, <time datetime="${date}">${date}</time>` : `, ${escapeHtml(date)}`;
    const heading = unreleased ? "In source, not yet published" : `${escapeHtml(label)}${when}`;
    const anchor = `<a class="anchor" href="#${ids[at]}" aria-label="Link to ${escapeHtml(label)}">#</a>`;
    const attributes = `id="${ids[at]}" data-version="${escapeHtml(unreleased ? "unreleased" : label)}"${at === latest ? ' data-latest="true"' : ""}`;
    return `        <section ${attributes}>\n          <h2>${heading} ${anchor}</h2>\n${renderBody(body)}\n        </section>\n`;
  });
  return `      <nav class="changelog-index" aria-label="Versions">${index}</nav>\n      <article class="changelog">\n        <h1>What changed</h1>\n${parts.join("")}      </article>\n`;
}

export function fallbackArticle() {
  return `      <article class="changelog">\n        <h1>What changed</h1>\n        <p>The change list could not be prepared for this page. Read <a href="${REPOSITORY_URL}/releases">the releases on GitHub</a>.</p>\n      </article>\n`;
}

// A template is used only when it holds the marker exactly once; otherwise the built-in page is.
export function usableTemplate(template) {
  return template !== undefined && template.split(TEMPLATE_MARKER).length === 2;
}

export function renderPage(sections, template) {
  return wrap(renderArticle(sections), template);
}

export function fallbackPage(template) {
  return wrap(fallbackArticle(), template);
}

function wrap(article, template) {
  return usableTemplate(template) ? template.replace(TEMPLATE_MARKER, () => article) : pageHead + article + pageTail;
}

function warn(message) {
  console.log(`::warning::${message}`);
}

function argument(args, name) {
  const at = args.indexOf(name);
  if (at < 0 || args[at + 1] === undefined) throw new Error(`missing ${name}`);
  return args[at + 1];
}

export function main(argv) {
  const [command, ...args] = argv;
  if (command === "notes") {
    const tag = argument(args, "--tag");
    const footer = readFileSync(argument(args, "--footer"), "utf8");
    let picked = { mode: "none", body: "" };
    try {
      const { sections, problems } = parseChangelogWithProblems(readFileSync(argument(args, "--changelog"), "utf8"));
      for (const problem of problems) warn(problem);
      picked = pickNotes(sections, tag);
    } catch (error) {
      warn(`could not read release notes from CHANGELOG.md (${error.message})`);
    }
    if (picked.mode === "none") warn(`CHANGELOG.md has no section for ${tag}; the release carries the standard text and GitHub's change list only`);
    if (picked.mode === "unreleased") warn(`CHANGELOG.md has no ${versionFromTag(tag)} section; the release carries its [Unreleased] text, which may describe changes this build does not have. Check the release body`);
    writeFileSync(argument(args, "--out"), composeBody(picked, footer));
    console.log(picked.mode);
    return 0;
  }
  if (command === "page") {
    let template;
    const templatePath = args.includes("--template") ? argument(args, "--template") : undefined;
    if (templatePath !== undefined) {
      try {
        template = readFileSync(templatePath, "utf8");
        if (!usableTemplate(template)) {
          warn(`${templatePath} must contain ${TEMPLATE_MARKER} exactly once; using the built-in page`);
          template = undefined;
        }
      } catch (error) {
        if (error.code !== "ENOENT") warn(`could not read ${templatePath} (${error.message}); using the built-in page`);
      }
    }
    let html;
    try {
      const { sections, problems } = parseChangelogWithProblems(readFileSync(argument(args, "--changelog"), "utf8"));
      for (const problem of problems) warn(problem);
      html = renderPage(sections, template);
    } catch (error) {
      warn(`could not render CHANGELOG.md for the site (${error.message}); the page links to the releases instead`);
      html = fallbackPage(template);
    }
    writeFileSync(argument(args, "--out"), html);
    return 0;
  }
  console.error("usage: release-notes.mjs notes --tag T --changelog F --footer F --out F | page --changelog F [--template F] --out F");
  return 2;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) process.exitCode = main(process.argv.slice(2));
