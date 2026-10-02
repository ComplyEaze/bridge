import assert from "node:assert/strict";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import test from "node:test";

const site = new URL("../site/", import.meta.url);
const read = (name) => readFileSync(new URL(name, site), "utf8");
// The deploy writes these from the tracked templates; a template is the page this test reads.
const siteOrigin = "https://bridge.complyeaze.com/";
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
      // a page may name its own address as its canonical one, and no other address on this site's origin
      if (tag.startsWith('<link rel="canonical"')) {
        assert.equal(ref, siteOrigin + (name === "index.html" ? "" : name), `${name}: its canonical address is not its own`);
        continue;
      }
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
  const files = readdirSync(site, { recursive: true }).filter((name) => /\.(html|css|js|mjs|svg|md|txt|xml)$/.test(name) && !name.endsWith(".min.js") && !generated.has(name));
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

// What a search engine or an AI assistant reads about the site without running its script.
const indexed = ["index.html", "download.html", "releases.html"];
const metaContent = (html, property) => html.match(new RegExp(`<meta property="${property}" content="([^"]*)" />`))?.[1];

test("each indexed page names its own address, and its social card repeats the page's own title and description", () => {
  for (const name of indexed) {
    const html = read(name);
    const url = siteOrigin + (name === "index.html" ? "" : name);
    assert.equal(html.match(/<link rel="canonical" href="([^"]*)" \/>/g)?.length, 1, `${name}: exactly one canonical link`);
    assert.match(html, new RegExp(`<link rel="canonical" href="${url.replaceAll(".", "\\.")}" />`), name);
    assert.equal(metaContent(html, "og:url"), url, name);
    assert.equal(metaContent(html, "og:title"), html.match(/<title>(.*?)<\/title>/)[1], name);
    assert.equal(metaContent(html, "og:description"), html.match(/<meta name="description" content="(.*?)" \/>/)[1], name);
    assert.equal(metaContent(html, "og:site_name"), "ComplyEaze Bridge", name);
    // no image: a card image has to be a raster file made by a designer, not drawn here
    assert.doesNotMatch(html, /og:image|twitter:image/i, `${name}: a card image needs a shipped raster file first`);
  }
});

test("the structured data on the home page is valid JSON and states no price, no version and no unproven claim", () => {
  const html = read("index.html");
  const blocks = [...html.matchAll(/<script type="application\/ld\+json">([\s\S]*?)<\/script>/g)];
  assert.equal(blocks.length, 1);
  const data = JSON.parse(blocks[0][1]);
  assert.equal(data["@type"], "SoftwareApplication");
  assert.equal(data.name, "ComplyEaze Bridge");
  assert.equal(data.url, siteOrigin);
  const text = JSON.stringify(data);
  const keys = (value) => (value && typeof value === "object" ? Object.entries(value).flatMap(([key, inner]) => [key, ...keys(inner)]) : []);
  for (const key of keys(data)) assert.doesNotMatch(key, /^(?:offers?|price\w*|softwareVersion|downloadUrl|aggregateRating|review|isAccessibleForFree)$/i, `${key} would go stale or imply something unproven`);
  assert.doesNotMatch(text, /price|\bfree\b|\bpaid\b|[₹$]/i, "a price in the words");
  for (const banned of [/\boffline\b/i, /nothing leaves/i, /\bpreviews?\b/i, /[™®]/]) assert.doesNotMatch(text, banned);
  assert.equal(text.match(/(?<!ComplyEaze )\bBridge\b/g), null, "a bare Bridge");
  assert.match(data.description, /not yet code-signed/);
  assert.match(data.description, /off by default in a new install/);
});

test("the sitemap lists every page that is meant to be found and nothing the site does not ship or generate", () => {
  const listed = [...read("sitemap.xml").matchAll(/<loc>([^<]*)<\/loc>/g)].map((match) => match[1]);
  assert.equal(new Set(listed).size, listed.length, "a page is listed twice");
  for (const url of listed) {
    assert.ok(url.startsWith(siteOrigin), url);
    const file = url.slice(siteOrigin.length) || "index.html";
    assert.ok(generated.has(file) || existsSync(new URL(file, site)), `${url} is not a page the site ships or the deploy writes`);
  }
  for (const name of indexed) assert.ok(listed.includes(siteOrigin + (name === "index.html" ? "" : name)), `${name} is not in the sitemap`);
  assert.match(read("robots.txt"), /^Sitemap: https:\/\/bridge\.complyeaze\.com\/sitemap\.xml$/m);
  assert.doesNotMatch(read("robots.txt"), /^Disallow: \/\s*$/m, "a blanket disallow would hide the site");
});

test("llms.txt is a plain description that keeps to the wording rules and links only to this project", () => {
  const text = read("llms.txt");
  assert.match(text, /^# ComplyEaze Bridge\n\n> /);
  for (const banned of [/\bfree\b/i, /\boffline\b/i, /nothing leaves/i, /\bpreviews?\b/i, /[™®]|&trade;|&reg;/]) assert.doesNotMatch(text, banned);
  assert.equal(text.match(/(?<!ComplyEaze )\bBridge\b/g), null, "a bare Bridge");
  assert.match(text, /not yet code-signed/);
  assert.match(text, /amounts are always sent/);
  const links = [...text.matchAll(/\]\((https?:[^)\s]+)\)/g)].map((match) => match[1]);
  assert.ok(links.length >= 8);
  for (const link of links) assert.match(link, /^https:\/\/(?:github\.com\/ComplyEaze\/bridge(?:\/|$)|bridge\.complyeaze\.com\/)/, link);
  for (const link of links.filter((link) => link.startsWith(siteOrigin))) {
    const file = link.slice(siteOrigin.length);
    assert.ok(generated.has(file) || existsSync(new URL(file, site)), `${link} is not a page the site ships or the deploy writes`);
  }
  for (const link of links.filter((link) => link.includes("/blob/master/"))) {
    const file = link.split("/blob/master/")[1];
    assert.ok(existsSync(new URL(`../${file}`, site)), `${link} names a file the repository does not have`);
  }
});
