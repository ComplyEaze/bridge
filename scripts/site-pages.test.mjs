import assert from "node:assert/strict";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import test from "node:test";

const site = new URL("../site/", import.meta.url);
const read = (name) => readFileSync(new URL(name, site), "utf8");
// changelog.html is written by the deploy from this template; the template is the tracked page.
const pages = readdirSync(site).filter((name) => name.endsWith(".html") && name !== "changelog.html").sort();
const generated = new Set(["changelog.html", "releases.json"]);

function region(html, open, close) {
  const start = html.indexOf(open);
  const end = html.indexOf(close, start);
  assert.ok(start >= 0 && end > start, `missing ${open} … ${close}`);
  return html.slice(start, end + close.length);
}

// Text a visitor can read or hear: comments, scripts and styles removed, tags dropped, and the
// descriptions, labels and tooltips carried in attributes kept.
function visibleText(html) {
  const body = html.replace(/<!--[\s\S]*?-->/g, " ").replace(/<(script|style)\b[\s\S]*?<\/\1>/g, " ");
  const spoken = [...body.matchAll(/\b(?:content|aria-label|alt|title)="([^"]*)"/g)].map((match) => match[1]);
  return `${body.replace(/<[^>]+>/g, " ")} ${spoken.join(" ")}`;
}

test("the site has the pages this test expects, so none is checked by accident or skipped", () => {
  assert.deepEqual(pages, ["changelog.template.html", "download.html", "index.html", "releases.html"]);
});

test("every page carries the same header and footer, apart from which link is the current page", () => {
  const chrome = (name) => {
    const html = read(name).replaceAll(' aria-current="page"', "");
    return region(html, '<header class="ce-header"', "</header>") + region(html, '<footer class="ce-footer"', "</footer>");
  };
  const home = chrome("index.html");
  for (const name of pages) assert.equal(chrome(name), home, `${name} has its own header or footer`);
});

test("every page sets the stored colour world before its stylesheet, and loads the shared stylesheet and script", () => {
  for (const name of pages) {
    const html = read(name);
    const theme = html.indexOf('localStorage.getItem("ce-theme")');
    const css = html.indexOf('<link rel="stylesheet" href="chrome.css" />');
    assert.ok(theme > 0 && css > theme, `${name}: the theme line must come before chrome.css`);
    assert.match(html, /try\{if\(localStorage\.getItem\("ce-theme"\)[^<]*\}catch\(e\)\{\}<\/script>/, `${name}: storage access must be inside try/catch`);
    assert.match(html, /<html lang="en">/, `${name}: the page as written is the cobalt world, with no theme attribute`);
    assert.match(html, /<script src="chrome\.js" defer><\/script>/, name);
  }
});

test("every local link and file reference on a page points at a file the site ships or the deploy writes", () => {
  for (const name of pages) {
    const html = read(name).replace(/<!--[\s\S]*?-->/g, " ");
    for (const [, ref] of html.matchAll(/\b(?:href|src)="([^"]+)"/g)) {
      if (/^(https:|#|data:)/.test(ref)) continue;
      assert.doesNotMatch(ref, /^(http:|\/\/|\.\.\/|\/)/, `${name}: ${ref} leaves the site folder`);
      const path = ref.replace(/^\.\//, "").replace(/[?#].*$/, "") || "index.html";
      assert.ok(generated.has(path) || existsSync(new URL(path, site)), `${name}: ${ref} does not exist`);
    }
  }
});

test("nothing on the site points at the roadmap or the comparison, which ship separately", () => {
  for (const name of readdirSync(site, { recursive: true })) {
    if (!/\.(html|js|mjs|css)$/.test(name) || name.startsWith("vendor/") || generated.has(name)) continue;
    assert.doesNotMatch(read(name), /roadmap|compare\.html|\/compare\b|comparison/i, `${name} mentions the roadmap or the comparison`);
  }
});

test("page text keeps to the wording rules", () => {
  for (const name of [...pages, "download.js", "releases.js"]) {
    const raw = read(name);
    const text = name.endsWith(".html") ? visibleText(raw) : raw;
    for (const banned of [/\bfree\b/i, /\boffline\b/i, /nothing leaves/i, /\bpreviews?\b/i, /[™®]|&trade;|&reg;/]) {
      assert.doesNotMatch(text.replaceAll("mcp-preview", ""), banned, `${name} says ${banned}`);
    }
  }
});

test("the product is never a bare Bridge, except inside the replica of the released build's own dialog", () => {
  for (const name of pages) {
    let html = read(name);
    if (name === "index.html") {
      const dialog = region(html, '<div class="dialog-replica">', '<div class="dialog-actions">');
      html = html.replace(dialog, " ");
    }
    const bare = visibleText(html).match(/(?<!ComplyEaze(?:\s|&nbsp;))\bBridge\b/g);
    assert.equal(bare, null, `${name} names the product as a bare Bridge`);
  }
});
