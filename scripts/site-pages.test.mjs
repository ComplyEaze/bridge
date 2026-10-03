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
  assert.deepEqual(pages, ["blog-bank-statement-ledger-names.html", "blog-ten-questions.html", "blog-where-does-client-data-go.html", "blog-who-approves-a-post.html", "blog.html", "capabilities.html", "changelog.template.html", "download.html", "faq.html", "index.html", "legal.template.html", "releases.html", "security.html"]);
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
  for (const name of [...pages, "download.js", "faq.js", "releases.js", "release-source.mjs", "chrome.js", "app.js", "scene.js", "chrome.css", "home.css", "pages.css", "blog.css"]) {
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
const indexed = ["index.html", "download.html", "faq.html", "capabilities.html", "releases.html", "security.html", ...pages.filter((name) => name.startsWith("blog"))];
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

// The questions page: what it promises a reader, and that the structured copy of it cannot drift from the words.
const faqText = (html) => html.replace(/<\/?(?:p|li|ul|ol|div|tr|td|th|table|summary|h[1-6])\b[^>]*>/g, " ").replace(/<[^>]+>/g, "").replace(/&nbsp;/g, " ").replace(/&rsquo;/g, "\u2019").replace(/&lsquo;/g, "\u2018").replace(/&ldquo;/g, "\u201c").replace(/&rdquo;/g, "\u201d").replace(/&amp;/g, "&").replace(/\s+/g, " ").trim();

test("every question is a native disclosure with its own address, written as a question, with an answer", () => {
  const html = read("faq.html");
  const items = [...html.matchAll(/<details class="faq-item" id="([^"]+)"( open)?>\s*<summary>(.*?)<\/summary>\s*<div class="faq-answer">([\s\S]*?)<\/div>\s*<\/details>/g)];
  assert.ok(items.length >= 15, "the page holds the questions the readers asked");
  const ids = items.map((item) => item[1]);
  assert.equal(new Set(ids).size, ids.length, "two questions share an address");
  assert.ok(items.filter((item) => item[2]).length <= 1, "more than one question opens by default");
  for (const [, id, , question, answer] of items) {
    assert.match(id, /^[a-z][a-z0-9-]*$/);
    assert.match(faqText(question), /\?$/, `${id}: a question ends with a question mark`);
    assert.ok(faqText(answer).length > 60 && faqText(answer).split(" ").length < 260, `${id}: an answer is short but is one`);
  }
  assert.equal([...html.matchAll(/<details\b/g)].length, items.length, "a question is not in the form the check reads");
  for (const [, href] of html.matchAll(/<a href="#([^"]+)"/g)) assert.ok(html.includes(`id="${href}"`), `#${href} points at nothing`);
});

test("the structured copy of the questions says exactly what the page says, question for question", () => {
  const html = read("faq.html");
  const blocks = [...html.matchAll(/<script type="application\/ld\+json">([\s\S]*?)<\/script>/g)];
  assert.equal(blocks.length, 1);
  const data = JSON.parse(blocks[0][1]);
  assert.equal(data["@type"], "FAQPage");
  const shown = [...html.matchAll(/<summary>(.*?)<\/summary>\s*<div class="faq-answer">([\s\S]*?)<\/div>\s*<\/details>/g)].map((item) => [faqText(item[1]), faqText(item[2])]);
  const structured = data.mainEntity.map((entity) => [entity.name, entity.acceptedAnswer.text]);
  assert.deepEqual(structured, shown);
  const all = JSON.stringify(data);
  assert.doesNotMatch(all.replace("has not set a price", ""), /price|\bfree\b|[\u20b9$]/i, "a price, or a cost claim beyond the one the owner approved");
});

test("the questions page keeps the claims that tell a reader what ComplyEaze Bridge cannot do", () => {
  const text = faqText(read("faq.html"));
  // each phrase is a limit or a caution the README and the security page state; dropping one makes the page kinder than they are
  for (const needle of [
    "has not been independently audited",
    "Neither choice hides amounts",
    "No ComplyEaze Bridge tool can approve it for you, but software that controls your screen could click the window",
    "A post never alters or deletes a voucher",
    "has no tool to delete or undo a posted voucher",
    "None of these files is encrypted",
    "not yet code-signed",
    "Not run yet: the package of the latest release against a live TallyPrime",
    "book with stock items is expected to be refused",
    "Intel Macs are not supported",
    "has not set a price for ComplyEaze Bridge and does not sell licences to it",
    "through ComplyEaze Bridge, ComplyEaze does not receive it (sections 4 to 6 of the Privacy Policy)",
    "We have not decided whether to charge for anything in future",
  ]) assert.ok(text.includes(needle), `the page no longer says: ${needle}`);
});

test("the jobs table gives one of four answers per row, links each row to its answer, and every \"No\" or \"Not yet\" is also in the list of what it cannot do", () => {
  const html = read("faq.html");
  const rows = [...html.matchAll(/<tr><th scope="row"><a href="#([^"]+)">(.*?)<\/a>(?:<small>.*?<\/small>)?<\/th><td><span class="faq-verdict faq-verdict--(\w+)">([^<]+)<\/span><\/td><\/tr>/g)].map((row) => ({ link: row[1], job: faqText(row[2]), kind: row[3], word: row[4] }));
  assert.ok(rows.length >= 9);
  for (const { link, job, kind, word } of rows) {
    assert.ok(["Yes", "Partly", "No", "Not yet"].includes(word), `${job}: ${word}`);
    assert.equal(kind, word.replace(" ", "").toLowerCase(), `${job}: the style does not match the word`);
    assert.ok(html.includes(`<details class="faq-item" id="${link}"`), `${job}: #${link} is not a question`);
  }
  const cannot = faqText(html.match(/id="what-it-cannot-do"[\s\S]*?<\/details>/)[0]);
  for (const { job, word } of rows.filter((row) => row.word === "No" || row.word === "Not yet")) {
    const key = /GSTR/.test(job) ? "GSTR-2B" : /Tax-audit/.test(job) ? "tax-audit" : /sales/i.test(job) ? "sales or purchase invoices" : null;
    assert.ok(key && cannot.includes(key), `${job} (${word}) is not in "What can it not do yet?"`);
  }
  // a verdict stronger than the answer under it would teach a reader to distrust the page
  const asks = faqText(html.match(/id="what-can-i-ask"[\s\S]*?<\/details>/)[0]);
  for (const { job, word } of rows.filter((row) => /stock items|Profit/.test(row.job))) assert.equal(word, "Partly", `${job}`);
  assert.match(asks, /book with stock items is expected to be refused/);
});

test("the page says which release and day it was checked against, and the structured copy is dated no earlier", () => {
  const html = read("faq.html");
  const stamp = html.match(/Checked against release ([0-9.]+) on <time datetime="([0-9-]+)">/);
  assert.ok(stamp, "no check stamp");
  // a version bump fails here until someone has read the answers again and moved the stamp
  const version = JSON.parse(readFileSync(new URL("../packaging/mcpb/manifest.json", import.meta.url), "utf8")).version;
  assert.equal(stamp[1], version, `site/faq.html is stamped for release ${stamp[1]} but the manifest says ${version}: reread every answer on the Questions page against the new release (the README, the security page, the Terms and the release notes), then move the stamp in the hero and the dateModified in the structured data`);
  // the page can change after its last full check (one answer edited), so the structured copy may be dated later than
  // the stamp, never earlier; the stamp moves only when every answer has been read again
  const modified = JSON.parse(html.match(/<script type="application\/ld\+json">([\s\S]*?)<\/script>/)[1]).dateModified;
  assert.match(stamp[2], /^[0-9]{4}-[0-9]{2}-[0-9]{2}$/);
  assert.match(modified, /^[0-9]{4}-[0-9]{2}-[0-9]{2}$/);
  assert.ok(modified >= stamp[2], `the structured copy says the page last changed on ${modified}, before it was checked on ${stamp[2]}`);
});

test("the list above the questions tells a person asked to try it out the four things to do first", () => {
  const html = read("faq.html");
  const list = faqText(html.match(/<section class="page-section faq-before">[\s\S]*?<\/section>/)[0]);
  for (const needle of ["test company", "backup", "firewall", "Keep posting off"]) assert.ok(list.includes(needle), needle);
});

// An IndexNow key file is a root text file named for its key (8 to 128 letters, digits and dashes) and
// holding exactly that key. llms.txt and robots.txt are shorter than any key, so they are not mistaken for one.
const indexNowKey = /^[A-Za-z0-9-]{8,128}\.txt$/;
test("the site has one IndexNow key file, and it holds exactly the key it is named for", () => {
  const keyFiles = readdirSync(site).filter((file) => indexNowKey.test(file));
  assert.equal(keyFiles.length, 1, `expected one IndexNow key file, found ${keyFiles.length}`);
  assert.equal(read(keyFiles[0]), keyFiles[0].replace(/\.txt$/, ""), `${keyFiles[0]} does not hold its own key`);
});

// The liability and contact answers paraphrase the Terms of Use and the Privacy Policy. Each figure and section
// number in them, and each qualifier listed below, is pinned to the words of the clause it summarises: a pinned
// qualifier dropped from the page, a figure or section number with no pin of its own, or a clause reworded under the
// page fails here. Wording outside the pins is checked by reading, not by this test.
const legalText = (name) => readFileSync(new URL(`../docs/legal/${name}.md`, import.meta.url), "utf8");
const flat = (text) => text.replace(/\*\*/g, "").replace(/\s+/g, " ").trim();
function clause(doc, number) {
  const text = legalText(doc);
  if (/^[0-9]+$/.test(number)) {
    const section = text.match(new RegExp(`^## ${number}\\. [\\s\\S]*?(?=^## |(?![\\s\\S]))`, "m"));
    assert.ok(section, `${doc} has no section ${number}`);
    return flat(section[0]);
  }
  const paragraph = text.match(new RegExp(`^${number.replace(".", "\\.")} [\\s\\S]*?(?=^[0-9]+\\.[0-9]+ |^## |(?![\\s\\S]))`, "m"));
  assert.ok(paragraph, `${doc} has no clause ${number}`);
  return flat(paragraph[0]);
}
// [what the page says, document, clause, what the clause says]
const legalPins = {
  liability: [
    ["sections 13 and 14 of the Terms of Use are what apply", "terms", "13", "No warranty"],
    ["sections 13 and 14 of the Terms of Use are what apply", "terms", "14", "Limitation of liability"],
    ["section 5.2 of the Terms of Use adds the Apache License\u2019s own disclaimer and limit of liability", "terms", "5.2", "The Apache Licence's own disclaimer of warranty and limitation of liability (its sections 7 and 8) apply in addition to sections 13 and 14"],
    ["To the extent the law allows, we, our partners, employees and agents, and the contributors to ComplyEaze Bridge are not liable for some kinds of loss", "terms", "14.1", "To the maximum extent permitted by applicable law, none of ComplyEaze, its partners, employees and agents, or the contributors to Bridge, will be liable"],
    ["however caused, including by negligence (section 14.1)", "terms", "14.1", "however it is caused. It applies whether the claim is in contract, tort (including negligence)"],
    ["indirect, consequential or similar loss", "terms", "14.1", "any indirect, incidental, special, consequential, exemplary or punitive loss or damage"],
    ["lost profits, revenue or clients", "terms", "14.1", "any loss of profits, revenue, business, goodwill, clients"],
    ["lost or corrupted data, including your Tally books, and the cost of restoring it", "terms", "14.1", "any loss or corruption of data, including your Tally books, or the cost of restoring or re-entering it"],
    ["any tax, interest, penalty, fee or late fee imposed on you or your clients, or the cost of correcting a return, filing or books", "terms", "14.1", "any tax, interest, penalty, fee or late fee imposed on you or your Clients, or the cost of correcting a return, filing or set of books"],
    ["any claim your clients or anyone else makes against you", "terms", "14.1", "any claim by your Clients or any other third party against you"],
    ["loss caused by your AI assistant, your AI provider, Tally or other third-party software", "terms", "14.1", "any loss caused by your AI Assistant, your AI Provider, Tally or other third-party software"],
    ["To the extent the law allows, our total liability to you for all claims connected with ComplyEaze Bridge, its website, any service we provide for it or the Terms", "terms", "14.2", "To the maximum extent permitted by applicable law, our total liability to you for all claims arising out of or in connection with Bridge, the Services or these Terms"],
    ["its website, any service we provide for it", "terms", "2", "The Bridge website, and any support or other service we choose to provide for Bridge"],
    ["the total amount you paid us for ComplyEaze Bridge in the twelve months before the event giving rise to the claim (section 14.2)", "terms", "14.2", "the total amount you paid us for Bridge in the twelve months before the event giving rise to the claim"],
    ["If a court or other authority decides that limit cannot apply", "terms", "14.3", "If a court or other authority decides that the limit in section 14.2 cannot apply"],
    ["our total liability for all claims together is limited to INR 1,000, to the extent the law allows (section 14.3)", "terms", "14.3", "our total liability for all claims together is limited to INR 1,000, to the maximum extent permitted by applicable law"],
    ["Nothing in the Terms excludes or limits liability for fraud, wilful misconduct or gross negligence, or any liability the law does not let us exclude or limit (section 14.4)", "terms", "14.4", "Nothing in these Terms excludes or limits liability for fraud, wilful misconduct or gross negligence, or any liability that cannot be excluded or limited under applicable law"],
  ],
  "client-claims": [
    ["To the extent the law allows, and except as section 14.4 provides", "terms", "14.4", "Nothing in these Terms excludes or limits liability"],
    ["we are not liable for a claim your client or anyone else makes against you (Terms of Use, section 14.1)", "terms", "14.1", "any claim by your Clients or any other third party against you"],
    ["makes a claim against us, our partners, employees or agents, you cover", "terms", "15", "you will indemnify ComplyEaze and its partners, employees and agents against third-party claims"],
    ["you cover any claim, demand, loss, cost or expense, including reasonable legal fees, that a third party brings", "terms", "15", "any claim, demand, loss, cost or expense (including reasonable legal fees) brought by a third party, including your Clients"],
    ["to the extent the law allows and to the extent it arises from your breach of the Terms or the law", "terms", "15", "To the extent permitted by applicable law"],
    ["or of Tally’s or your AI provider’s terms", "terms", "15", "or of Tally's or your AI Provider's terms"],
    ["from a voucher you approved, or data you accessed or shared, without the authority section 8 requires", "terms", "15", "a voucher you approved, or data you accessed or shared, without the authority required by section 8"],
    ["without the authority section 8 requires", "terms", "8", "Having the right to access each Tally company you connect Bridge to"],
    ["your breach of a duty you owe your clients", "terms", "15", "your breach of any duty you owe your Clients"],
    ["You do not cover a claim to the extent it results from our breach of the Terms, our negligence or our wilful misconduct.", "terms", "15", "It does not apply to the extent a claim results from our breach of these Terms or our negligence or wilful misconduct."],
    ["Nor do you cover any penalty imposed on us for our own breach of the law (section 15)", "terms", "15", "It does not cover any penalty imposed on us for our own breach of the law."],
    ["your decision to approve a voucher and its accounting consequences, whoever proposed it", "terms", "9.3", "You are responsible for that decision and its accounting consequences, whoever proposed the voucher"],
    ["that responsibility does not extend to any difference between what the approval window showed and what ComplyEaze Bridge actually posted (section 9.3)", "terms", "9.3", "This does not apply to any difference between what the window showed and what Bridge actually posted"],
    ["To the extent the law allows, and except as section 14.4 provides, section 14 still limits any liability we have for such a difference", "terms", "14.2", "To the maximum extent permitted by applicable law, our total liability to you for all claims arising out of or in connection with Bridge"],
    ["sections 8, 9, 13, 14 and 15 are what apply", "terms", "8", "Your responsibilities"],
    ["sections 8, 9, 13, 14 and 15 are what apply", "terms", "9", "AI assistants, posting and approvals"],
    ["sections 8, 9, 13, 14 and 15 are what apply", "terms", "13", "No warranty"],
    ["sections 8, 9, 13, 14 and 15 are what apply", "terms", "14", "Limitation of liability"],
    ["sections 8, 9, 13, 14 and 15 are what apply", "terms", "15", "Indemnity"],
  ],
  contact: [
    ["contact@complyeaze.com (SPMS Comply Eaze Solutions LLP; its registered office address is in section 1 of the Privacy Policy", "privacy", "1", "Our registered office is at"],
    ["these routes are in section 19 of the Terms of Use", "terms", "19", "General questions and notices: contact@complyeaze.com, SPMS Comply Eaze Solutions LLP"],
    ["Before starting any proceedings over a dispute, write to contact@complyeaze.com or to our registered office; both sides then try in good faith to resolve it within 30 days", "terms", "18.2", "Before starting any proceedings, the party raising a dispute will notify the other in writing, at the contact address in section 19. Both will then try in good faith to resolve it within 30 days."],
    ["or to our registered office", "terms", "19", "at the address in section 1.1"],
    ["or to our registered office", "terms", "1.1", "Our registered office is at"],
    ["though either side can still seek urgent interim relief (section 18.2 of the Terms of Use)", "terms", "18.2", "This does not stop either party from seeking urgent interim relief."],
    ["Do not put real client data, passwords or other confidential information in issues, bug reports, logs or screenshots you share with us or post publicly (section 12.3)", "terms", "12.3", "Do not include real client data, passwords or other confidential information in issues, bug reports, logs or screenshots that you share with us or post publicly"],
    ["“Grievance” in the subject", "privacy", "16", "with \"Grievance\" in the subject"],
    ["or write by post to that address", "privacy", "16", "or by post to the address in section 1"],
    ["we acknowledge within 7 days of receiving it and answer within one month (section 16 of the Privacy Policy)", "privacy", "16", "We will acknowledge a grievance within 7 days of receiving it and give you our response within one month"],
    ["write to security@complyeaze.com or use GitHub private vulnerability reporting for the ComplyEaze Bridge repository", "terms", "19", "security@complyeaze.com, or GitHub private vulnerability reporting for the Bridge repository"],
    ["not a public issue; we aim to acknowledge within 7 days", "terms", "19", "We aim to acknowledge a report within seven days. Please do not report a vulnerability in a public issue."],
    ["Section 4.4 of the Terms of Use says we are not obliged to provide support", "terms", "4.4", "We are not obliged to provide support"],
  ],
};
// "section 14.1", "Section 5", "sections 8, 9 and 15", "sections 4 to 6", "section 16 of the Privacy Policy"
const citation = /\bsections? ([0-9]+(?:\.[0-9]+)*)((?:(?:, | and | to )[0-9]+(?:\.[0-9]+)*)*)/gi;
const figure = /\b(?:[0-9][0-9,]*[0-9]|[0-9]|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred|thousand|lakh|lakhs|crore|crores)\b/gi;
const withoutCitations = (text) => text.replace(citation, " ");
// Each citation as "document number", read fail-closed: a citation followed by " of " must name exactly the Terms of
// Use or the Privacy Policy, and a bare one is the Terms' only if its clause names neither the Privacy Policy nor the
// Apache License, whose own sections would otherwise pass as the Terms'.
function citedIn(text) {
  return [...text.matchAll(citation)].flatMap((match) => {
    const after = text.slice(match.index + match[0].length);
    let doc = "terms";
    if (after.startsWith(" of ")) {
      if (after.startsWith(" of the Privacy Policy")) doc = "privacy";
      else assert.ok(after.startsWith(" of the Terms of Use"), `"${match[0]}${after.slice(0, 24)}" names a document this check does not read`);
    } else {
      const start = Math.max(...[". ", "; ", ": "].map((mark) => text.lastIndexOf(mark, match.index)));
      const ends = [". ", "; "].map((mark) => text.indexOf(mark, match.index)).filter((at) => at >= 0);
      const around = text.slice(start + 1, ends.length ? Math.min(...ends) : text.length);
      assert.doesNotMatch(around, /Privacy Policy|Apache/i, `"${match[0]}" sits beside another document without naming its own: ${around.trim()}`);
    }
    return [match[1], ...match[2].split(/, | and | to /).filter(Boolean)].map((number) => `${doc} ${number}`);
  });
}

test("the liability and contact answers say no more and no less than the clauses they summarise", () => {
  const html = read("faq.html");
  for (const [id, pins] of Object.entries(legalPins)) {
    const found = html.match(new RegExp(`<details class="faq-item" id="${id}">[\\s\\S]*?<div class="faq-answer">([\\s\\S]*?)<\\/div>\\s*<\\/details>`));
    assert.ok(found, `the page has no question #${id}`);
    const answer = faqText(found[1]);
    // a link named for a document goes to that document
    for (const [, href, name] of found[1].matchAll(/<a href="([^"]*)">([^<]*)<\/a>/g)) {
      if (name === "Terms of Use") assert.equal(href, "./terms.html", `#${id}: a "Terms of Use" link goes to ${href}`);
      if (name === "Privacy Policy") assert.equal(href, "./privacy.html", `#${id}: a "Privacy Policy" link goes to ${href}`);
    }
    for (const [says, doc, number, reads] of pins) {
      assert.ok(answer.includes(says), `#${id} no longer says: ${says}`);
      assert.ok(clause(doc, number).includes(reads), `${doc} ${number} no longer reads: ${reads} (reread #${id} against it)`);
    }
    // every section the answer cites has a pin to that very clause or section, inside a phrase that cites it
    for (const cited of citedIn(answer)) {
      const [doc, number] = cited.split(" ");
      const own = pins.filter(([says, pinDoc, pinNumber]) => citedIn(says).includes(cited) && pinDoc === doc && (pinNumber === number || pinNumber.startsWith(`${number}.`)));
      assert.ok(own.length > 0, `#${id} cites section ${cited} without a pin to it`);
    }
    // every figure, in digits or in words, sits inside a pinned phrase
    for (const [word] of withoutCitations(answer).matchAll(figure)) {
      const pinned = pins.some(([says]) => new RegExp(`\\b${word.replace(",", "\\,")}\\b`, "i").test(withoutCitations(says)));
      assert.ok(pinned, `#${id} gives the figure "${word}" without a pin to the clause`);
    }
  }
});

test("every page names the publisher with the statement of limited liability, registration number and registered office the Privacy Policy gives", () => {
  // The footer repeats the name, the statement of limited liability, the registration number and the registered
  // office from section 1 of the Privacy Policy, so the two cannot drift apart.
  const section1 = flat(legalText("privacy").match(/^## 1\. [\s\S]*?(?=^## )/m)[0]);
  const llpin = section1.match(/LLP identification number ([A-Z]{3}-[0-9]{4})/);
  const office = section1.match(/Our registered office is at ([^.]+(?:\.[^.]+)*?, India)\./);
  assert.ok(llpin && office, "section 1 of the Privacy Policy no longer gives the LLP identification number and registered office");
  const registered = "a limited liability partnership registered with limited liability under the Limited Liability Partnership Act, 2008";
  assert.ok(section1.includes(`SPMS Comply Eaze Solutions LLP, ${registered}, LLP identification number`), "section 1 of the Privacy Policy no longer states that the LLP is registered with limited liability");
  const line = `Published by SPMS Comply Eaze Solutions LLP, ${registered}, LLP identification number ${llpin[1]}. Registered office: ${office[1]}.`;
  for (const page of pages) assert.ok(faqText(read(page)).includes(line), `${page}: the footer does not carry: ${line}`);
});

test("the Atom feed is complete, and has one entry for each blog post, pointing at the post's own address", () => {
  const origin = "https://bridge.complyeaze.com";
  const rfc3339 = "\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}Z";
  const time = (text) => {
    const when = new Date(text);
    assert.ok(!Number.isNaN(when.getTime()) && when.toISOString().replace(".000", "") === text, `${text} is not a real time`);
    return when.getTime();
  };
  const posts = pages.filter((name) => name.startsWith("blog-"));
  assert.ok(posts.length >= 1, "the blog has at least one post");
  const feed = read("feed.xml");
  const entries = [...feed.matchAll(/<entry>([\s\S]*?)<\/entry>/g)].map((match) => match[1]);
  assert.equal(entries.length, posts.length, "the feed has one entry per post page");
  const own = feed.replace(/<entry>[\s\S]*?<\/entry>/g, "");
  assert.match(own, /^<\?xml version="1\.0" encoding="utf-8"\?>\n<feed xmlns="http:\/\/www\.w3\.org\/2005\/Atom">/, "the feed has no Atom namespace");
  assert.ok(own.includes(`<link rel="self" type="application/atom+xml" href="${origin}/feed.xml" />`), "the feed's self link is not its own address");
  assert.ok(own.includes(`<id>${origin}/blog.html</id>`), "the feed has no id");
  for (const tag of ["title", "author"]) assert.match(own, new RegExp(`<${tag}>[\\s\\S]+?</${tag}>`), `the feed has no ${tag}`);
  const [, feedUpdated] = own.match(new RegExp(`<updated>(${rfc3339})</updated>`)) ?? [];
  assert.ok(feedUpdated, "the feed itself has no RFC 3339 update time");
  const newest = Math.max(...entries.map((entry) => time(entry.match(new RegExp(`<updated>(${rfc3339})</updated>`))?.[1] ?? "")));
  assert.ok(time(feedUpdated) >= newest, "the feed says it was updated before its newest entry");
  for (const post of posts) {
    const address = `${origin}/${post}`;
    const entry = entries.find((text) => new RegExp(`<id>${address.replaceAll(".", "\\.")}</id>`).test(text));
    assert.ok(entry, `${post} has no feed entry with its address as the id`);
    for (const tag of ["title", "summary"]) assert.match(entry, new RegExp(`<${tag}>[^<]+</${tag}>`), `${post}: the feed entry has no ${tag}`);
    for (const tag of ["published", "updated"]) assert.match(entry, new RegExp(`<${tag}>${rfc3339}</${tag}>`), `${post}: the feed entry has no ${tag} time`);
    assert.ok(entry.includes(`<link rel="alternate" type="text/html" href="${address}" />`), `${post}: the feed entry does not link to the page`);
  }
  // the feed carries words too, so it keeps to the same wording rules as a page
  for (const banned of [/\bfree\b/i, /\boffline\b/i, /nothing leaves/i, /\bpreviews?\b/i, /[™®]|&trade;|&reg;/]) assert.doesNotMatch(feed, banned, `feed.xml says ${banned}`);
  assert.doesNotMatch(feed, /(?<!ComplyEaze )\bBridge\b/, "feed.xml names the product as a bare Bridge");
});
