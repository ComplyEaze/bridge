import assert from "node:assert/strict";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import test from "node:test";

const site = new URL("../site/", import.meta.url);
const read = (name) => readFileSync(new URL(name, site), "utf8");
// The deploy writes these from the tracked templates; a template is the page this test reads.
const generated = new Set(["changelog.html", "privacy.html", "releases.json", "terms.html"]);
const pages = readdirSync(site).filter((name) => name.endsWith(".html") && !generated.has(name)).sort();

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
  assert.deepEqual(pages, ["changelog.template.html", "download.html", "index.html", "legal.template.html", "releases.html"]);
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

// The only outside addresses a page may name: this project on GitHub (links), and nothing loaded from elsewhere.
const outsideLinks = /^https:\/\/github\.com\/ComplyEaze\/bridge(?:[/#?]|$)/;

test("every reference on a page is a file the site ships or the deploy writes, or a link to this project on GitHub", () => {
  for (const name of pages) {
    const html = read(name).replace(/<!--[\s\S]*?-->/g, " ");
    for (const [tag, attribute, , ref] of html.matchAll(/<[a-z][^>]*?\b(href|src)=(["'])(.*?)\2/g)) {
      if (ref.startsWith("#")) continue;
      if (/^[a-z][a-z0-9+.-]*:/i.test(ref) || ref.startsWith("//")) {
        // only an <a> may leave the site, and only for this project's GitHub pages: no outside script, style, font or image
        assert.ok(tag.startsWith("<a ") && attribute === "href" && outsideLinks.test(ref), `${name}: ${ref} is an outside reference`);
        continue;
      }
      assert.doesNotMatch(ref, /^(\.\.\/|\/)/, `${name}: ${ref} leaves the site folder`);
      const path = ref.replace(/^\.\//, "").replace(/[?#].*$/, "") || "index.html";
      assert.ok(generated.has(path) || existsSync(new URL(path, site)), `${name}: ${ref} does not exist`);
    }
  }
});

test("stylesheets and scripts name only files the site ships, and no outside address but GitHub's", () => {
  for (const name of readdirSync(site).filter((file) => /\.(css|js|mjs)$/.test(file))) {
    const text = read(name);
    for (const [, ref] of text.matchAll(/url\(\s*["']?([^"')]+)["']?\s*\)/g)) {
      if (ref.startsWith("data:")) continue;
      assert.ok(!/^[a-z]+:|^\/\//i.test(ref) && existsSync(new URL(ref, site)), `${name}: url(${ref}) is missing or outside the site`);
    }
    for (const [, ref] of text.matchAll(/\b(?:from|import)\s*\(?\s*["'](\.[^"']+)["']/g)) {
      assert.ok(existsSync(new URL(ref, site)), `${name}: imports ${ref}, which does not exist`);
    }
    for (const [ref] of text.matchAll(/https?:\/\/[^\s"'`)<>]+/g)) {
      assert.match(ref, /^https:\/\/(?:api\.)?github\.com\/|^http:\/\/www\.w3\.org\/|^http:\/\/127\.0\.0\.1:9000$/, `${name}: ${ref} is an outside address`);
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
  // scripts and stylesheets draw words too (the 3D pages, the header switch, CSS content)
  for (const name of [...pages, "download.js", "releases.js", "release-source.mjs", "chrome.js", "app.js", "scene.js", "chrome.css", "home.css", "pages.css"]) {
    const raw = read(name);
    // in a script or stylesheet the words are in the code, not in its comments
    const text = name.endsWith(".html") ? visibleText(raw) : raw.replace(/\/\*[\s\S]*?\*\//g, " ").replace(/(^|[^:"'`])\/\/.*$/gm, "$1");
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

test("no file that ships carries an internal working label", () => {
  // Comments say what the code does and why. Names of work lanes, review rounds, design directions, briefs, fix
  // numbers, working dates and files that are not in the site stay out of public files.
  const labels = [
    /\bLane\b/, /\bcritics?\b/i, /\bcritique\b/i, /\bW[0-9]\b/, /\bfix(?:es)? [0-9]/i, /\b[XYLAD][0-9]\b/, /builder[A-Z]/, /\bitem [0-9]/i,
    /motion spec/i, /_lh\//, /\bdirection [A-Z]\b/i, /\bbrief\b/i, /\baddendum\b/i, /\bA\/B\b/, /\bowner/i, /\blead decision\b/i,
    /\bround[- ]?[0-9]/i, /\bv[0-9]m?\b(?!\.[0-9]|[0-9])/, /\bv[0-9]\//, /\bpages\//, /\bstyle\.css\b/, /\b[0-9]{1,2} (?:Sep|Oct)\b(?! 20)/,
    /privacy policy, section/i, /\bPrivacy and Terms\b/, /\b(?:TBT|perf) pass\b/i, /\bfinal (?:pass|review)\b/i,
  ];
  const files = readdirSync(site, { recursive: true }).filter((name) => /\.(html|css|js|mjs|svg|md)$/.test(name) && !name.endsWith(".min.js") && !generated.has(name));
  assert.ok(files.includes("scene.js") && files.includes("chrome.js") && files.includes("home.css") && files.includes("brand/lockup.svg"), "the check reads the scripts, stylesheets and artwork");
  for (const name of files) {
    // path data (d="M2.21 7 L4.99 7 …") is drawing commands, not words
    const lines = read(name).replace(/\bd="[^"]*"/g, "").split("\n");
    for (const label of labels) {
      const hit = lines.find((line) => label.test(line));
      assert.equal(hit, undefined, `${name} matches ${label}: ${hit}`);
    }
  }
});
