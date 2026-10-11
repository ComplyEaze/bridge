// Renders the two legal Markdown documents (Privacy Policy, Terms of Use) into pages for the static
// site. Only the constructs those documents use are supported; anything else, an unresolved
// [VERIFY ...] / [PUBLISH DATE] marker, or a broken template fails with the input line number and
// writes nothing. Unlike the changelog renderer there is no fallback page: a legal page must not
// silently degrade.
import { readFileSync, writeFileSync } from "node:fs";
import { basename } from "node:path";
import { pathToFileURL } from "node:url";

export const ARTICLE_MARKER = "<!-- legal -->";
export const TITLE_MARKER = "<!-- legal-title -->";
export const CANONICAL_MARKER = "<!-- legal-canonical -->";
// The site's own address; each rendered page names itself as its canonical one (privacy.html and
// terms.html are also reachable without ".html", and the same bytes must not count as two pages).
export const SITE_ORIGIN = "https://bridge.complyeaze.com/";
const PAGE_NAME = /^[a-z0-9][a-z0-9-]*\.html$/;
export const UNRESOLVED_MARKERS = ["[VERIFY", "[PUBLISH DATE]"];

export class LegalError extends Error {
  constructor(line, reason) {
    super(reason);
    this.line = line;
    this.reason = reason;
  }
}
const fail = (line, reason) => {
  throw new LegalError(line, reason);
};

export const escapeHtml = (text) => text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;").replace(/'/g, "&#39;");

// One pass over the text: code, bold, [text](url), bare URL. Everything between tokens is escaped
// and must be free of leftover syntax, so no unsupported markup can pass as plain text.
const TOKEN = /`([^`\n]+)`|\*\*(?=\S)(.+?)(?<=\S)\*\*|\[([^\]\n]*)\]\(([^)\n]*)\)|(https?:\/\/[^\s<>"'`]+)/g;

function checkPlain(text, line) {
  if (text.includes("`")) fail(line, "unbalanced backtick or unsupported code span");
  if (text.includes("*")) fail(line, "unbalanced ** or unsupported emphasis (only **bold** is supported)");
  if (/[[\]]/.test(text)) fail(line, "square brackets that are not a [text](https://...) link (reference links and footnotes are not supported)");
  if (/(^|[^\p{L}\p{N}])_|_($|[^\p{L}\p{N}])/u.test(text)) fail(line, "underscore emphasis is not supported");
}

export function renderInline(text, line, { bold = true, links = true } = {}) {
  let html = "";
  let last = 0;
  for (const match of text.matchAll(TOKEN)) {
    let [token, code, strong, linkText, linkUrl, bare] = match;
    checkPlain(text.slice(last, match.index), line);
    html += escapeHtml(text.slice(last, match.index));
    if (code !== undefined) {
      html += `<code>${escapeHtml(code)}</code>`;
    } else if (strong !== undefined) {
      if (!bold) fail(line, "nested ** is not supported");
      html += `<strong>${renderInline(strong, line, { bold: false, links })}</strong>`;
    } else if (linkUrl !== undefined) {
      if (!links) fail(line, "a link cannot contain a link");
      if (!/^https:\/\/[^\s"<>]+$/.test(linkUrl)) fail(line, `only https links are supported: (${linkUrl})`);
      if (linkText.trim() === "") fail(line, "link text is empty");
      html += `<a href="${escapeHtml(linkUrl)}">${renderInline(linkText, line, { bold, links: false })}</a>`;
    } else {
      if (!links) fail(line, "a link cannot contain a link");
      if (!bare.startsWith("https://")) fail(line, `only https links are supported: ${bare}`);
      const url = bare.replace(/[.,;:!?)\]]+$/, "");
      token = url;
      html += `<a href="${escapeHtml(url)}">${escapeHtml(url)}</a>`;
    }
    last = match.index + token.length;
  }
  checkPlain(text.slice(last), line);
  return html + escapeHtml(text.slice(last));
}

function slugFor(raw, used) {
  const base = raw.toLowerCase().normalize("NFKD").replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "") || "section";
  let id = base;
  for (let count = 2; used.has(id); count++) id = `${base}-${count}`;
  used.add(id);
  return id;
}

// Anything that starts a block other than a plain paragraph line.
const STRUCTURAL = /^(#{1,6}(\s|$)|>|\||- |[-=]+\s*$|\d+[.)] |[*+] )/;

function table(rows) {
  const cells = ({ t, n }) => {
    if (t.length < 2 || !t.trimEnd().endsWith("|")) fail(n, "a table row must start and end with |");
    return t.trim().slice(1, -1).split("|").map((cell) => cell.trim());
  };
  if (rows.length < 3) fail(rows[0].n, "a table needs a header row, a delimiter row and at least one body row");
  const head = cells(rows[0]);
  if (!cells(rows[1]).every((cell) => /^:?-+:?$/.test(cell))) fail(rows[1].n, "the second table row must be a |---|---| delimiter row");
  const row = (r, tag) => {
    const found = cells(r);
    if (found.length !== head.length) fail(r.n, `table row has ${found.length} cells but the header has ${head.length}`);
    return `<tr>${found.map((cell) => `<${tag}>${renderInline(cell, r.n)}</${tag}>`).join("")}</tr>`;
  };
  return `<table>\n<thead>\n${row(rows[0], "th")}\n</thead>\n<tbody>\n${rows.slice(2).map((r) => row(r, "td")).join("\n")}\n</tbody>\n</table>`;
}

function list(lines, start) {
  let i = start;
  const items = [];
  while (i < lines.length && lines[i].t.startsWith("- ")) {
    const item = { text: lines[i].t.slice(2).trim(), n: lines[i].n, ordered: [] };
    if (item.text === "") fail(item.n, "empty list item");
    for (i++; i < lines.length && /^ +\S/.test(lines[i].t); i++) {
      const numbered = /^ {2}\d+\. +(\S.*)$/.exec(lines[i].t);
      if (!numbered) fail(lines[i].n, /^ +[-*+] /.test(lines[i].t) ? "nested bullet lists are not supported (only a numbered list inside a bullet item)" : "a list indent other than two spaces before a numbered item is not supported");
      item.ordered.push({ text: numbered[1], n: lines[i].n });
    }
    items.push(item);
  }
  if (i < lines.length && lines[i].t.trim() !== "" && !STRUCTURAL.test(lines[i].t)) fail(lines[i].n, "text directly after a list must be separated from it by a blank line");
  const li = ({ text, n, ordered }) => {
    const nested = ordered.length ? `\n<ol>\n${ordered.map((o) => `<li>${renderInline(o.text, o.n)}</li>`).join("\n")}\n</ol>\n` : "";
    return `<li>${renderInline(text, n)}${nested}</li>`;
  };
  return { html: `<ul>\n${items.map(li).join("\n")}\n</ul>`, next: i };
}

function paragraphText(para) {
  const text = para.map(({ t }) => t.trim()).join(" ");
  try {
    return renderInline(text, para[0].n);
  } catch (error) {
    if (!(error instanceof LegalError) || para.length === 1) throw error;
    for (const { t, n } of para) renderInline(t.trim(), n); // a line that fails alone names the right line
    throw error;
  }
}

function blocks(lines, ctx) {
  const out = [];
  let i = 0;
  while (i < lines.length) {
    const { t, n } = lines[i];
    if (t.trim() === "") { i++; continue; }
    if (t.startsWith(" ")) fail(n, /^ {4}/.test(t) ? "indented code blocks are not supported" : "unsupported indentation");
    if (/^[-=]+\s*$/.test(t)) {
      if (t.trim() !== "---") fail(n, "only a --- line is supported as a rule (setext underline or other rule)");
      out.push("<hr>");
      i++;
    } else if (/^#{1,6}(\s|$)/.test(t)) {
      const heading = /^(#{1,2}) +(\S.*?)\s*$/.exec(t);
      if (!heading) fail(n, /^#{3,}/.test(t) ? "headings deeper than ## are not supported" : "malformed heading");
      if (ctx.quote) fail(n, "headings are not supported inside a block quote");
      const [, marks, raw] = heading;
      if (marks === "#") {
        ctx.titles.push({ raw, n });
        out.push(`<h1>${renderInline(raw, n)}</h1>`);
      } else {
        out.push(`<h2 id="${slugFor(raw, ctx.ids)}">${renderInline(raw, n)}</h2>`);
      }
      i++;
    } else if (t.startsWith(">")) {
      if (ctx.quote) fail(n, "nested block quotes are not supported");
      const inner = [];
      for (; i < lines.length && lines[i].t.startsWith(">"); i++) inner.push({ t: lines[i].t.replace(/^> ?/, ""), n: lines[i].n });
      out.push(`<aside class="legal-summary">\n${blocks(inner, { ...ctx, quote: true }).join("\n")}\n</aside>`);
    } else if (t.startsWith("|")) {
      const rows = [];
      for (; i < lines.length && lines[i].t.startsWith("|"); i++) rows.push(lines[i]);
      out.push(table(rows));
    } else if (t.startsWith("- ")) {
      const { html, next } = list(lines, i);
      out.push(html);
      i = next;
    } else if (/^\d+[.)] /.test(t)) {
      fail(n, "a numbered list is only supported inside a bullet item");
    } else if (/^[*+] /.test(t)) {
      fail(n, "only - bullets are supported");
    } else {
      const para = [];
      for (; i < lines.length && lines[i].t.trim() !== "" && !STRUCTURAL.test(lines[i].t); i++) {
        if (lines[i].t.startsWith(" ")) fail(lines[i].n, "unsupported indentation");
        para.push(lines[i]);
      }
      if (i < lines.length && /^[-=]+\s*$/.test(lines[i].t)) fail(lines[i].n, "setext headings are not supported (a line of only = or - directly under text)");
      out.push(`<p>${paragraphText(para)}</p>`);
    }
  }
  return out;
}

function checkLine({ t, n }) {
  if (/[\t\r]/.test(t)) fail(n, "tabs and stray carriage returns are not supported");
  if (/<[A-Za-z/!?]/.test(t)) fail(n, "raw HTML is not supported");
  if (t.includes("![")) fail(n, "images are not supported");
  if (t.includes("[^")) fail(n, "footnote syntax is not supported");
  if (/^\s*(`{3,}|~{3,})/.test(t)) fail(n, "code fences are not supported");
  if (/^\s*\[[^\]]+\]:/.test(t) || t.includes("][")) fail(n, "reference-style links are not supported");
}

// Returns the rendered article and the plain text of the document's single # heading.
export function renderDocument(source) {
  const lines = source.replace(/^﻿/, "").split(/\r?\n/).map((t, at) => ({ t, n: at + 1 }));
  for (const { t, n } of lines) {
    const marker = UNRESOLVED_MARKERS.find((m) => t.includes(m));
    if (marker) fail(n, `unresolved marker ${marker}${marker.endsWith("]") ? "" : " ...]"}; resolve it before publishing`);
  }
  lines.forEach(checkLine);
  const ctx = { titles: [], ids: new Set(), quote: false };
  const body = blocks(lines, ctx);
  if (ctx.titles.length !== 1) fail(ctx.titles[1]?.n ?? 1, `expected exactly one # heading, found ${ctx.titles.length}`);
  return { title: ctx.titles[0].raw, article: `<article class="legal">\n${body.join("\n")}\n</article>` };
}

export function applyTemplate(template, { title, article }, page) {
  for (const marker of [ARTICLE_MARKER, TITLE_MARKER, CANONICAL_MARKER]) {
    const count = template.split(marker).length - 1;
    if (count !== 1) fail(undefined, `template must contain ${marker} exactly once, found ${count}`);
  }
  if (!PAGE_NAME.test(page)) fail(undefined, `output file name ${JSON.stringify(page)} is not a page name (lowercase letters, digits, hyphens, then .html)`);
  return template
    .replace(TITLE_MARKER, () => escapeHtml(title))
    .replace(CANONICAL_MARKER, () => SITE_ORIGIN + page)
    .replace(ARTICLE_MARKER, () => article);
}

const USAGE = "usage: render-legal.mjs page --in F --template F --out F";

export function main(argv) {
  const [command, ...args] = argv;
  const options = {};
  for (let at = 0; at < args.length; at += 2) {
    const name = args[at];
    if (!["--in", "--template", "--out"].includes(name) || args[at + 1] === undefined || name in options) {
      console.error(USAGE);
      return 2;
    }
    options[name] = args[at + 1];
  }
  if (command !== "page" || Object.keys(options).length !== 3) {
    console.error(USAGE);
    return 2;
  }
  let where = options["--in"];
  try {
    const source = readFileSync(options["--in"], "utf8");
    const rendered = renderDocument(source);
    where = options["--template"];
    const html = applyTemplate(readFileSync(options["--template"], "utf8"), rendered, basename(options["--out"]));
    where = options["--out"];
    writeFileSync(options["--out"], html);
    return 0;
  } catch (error) {
    const line = error instanceof LegalError && error.line !== undefined ? `:${error.line}` : "";
    const reason = error instanceof LegalError ? error.reason : error.message;
    console.error(`render-legal: ${where}${line}: ${reason}`.replace(/\s*\n\s*/g, " "));
    return 1;
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) process.exitCode = main(process.argv.slice(2));
