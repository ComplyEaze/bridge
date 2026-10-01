import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { LegalError, applyTemplate, renderDocument } from "./render-legal.mjs";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const script = join(root, "scripts", "render-legal.mjs");
const template = readFileSync(join(root, "site", "legal.template.html"), "utf8");
// The real documents live in docs/legal/ of the repository; LEGAL_DOCS_DIR points the tests elsewhere.
const docsDir = process.env.LEGAL_DOCS_DIR ?? join(root, "docs", "legal");
const realDoc = (name) => (existsSync(join(docsDir, name)) ? readFileSync(join(docsDir, name), "utf8") : undefined);

const wrapped = (md) => renderDocument(`# T\n\n${md}\n`).article;
const inner = (md) => wrapped(md).replace(/^<article class="legal">\n<h1>T<\/h1>\n/, "").replace(/\n<\/article>$/, "");
// The fixture body starts on line 3, after the "# T" heading and a blank line.
const refused = (source, line, reason) => {
  assert.throws(() => renderDocument(source), (error) => error instanceof LegalError && error.line === line && reason.test(error.reason), `${JSON.stringify(source)} should be refused at line ${line} (${reason})`);
};
const refusedBody = (md, line, reason) => refused(`# T\n\n${md}\n`, line, reason);
const count = (html, re) => (html.match(re) ?? []).length;
const resolved = (source) => source.replace(/\[VERIFY[^\]]*\]/g, "").replaceAll("[PUBLISH DATE]", "1 January 2027");
const structure = (html) => ({ h1: count(html, /<h1>/g), h2: count(html, /<h2 /g), table: count(html, /<table>/g), aside: count(html, /<aside /g), ol: count(html, /<ol>/g) });
const expectedStructure = (source) => ({
  h1: 1,
  h2: count(source, /^## /gm),
  table: count(source, /^\| *:?-+/gm),
  aside: source.split("\n").filter((line, at, all) => line.startsWith(">") && !(at > 0 && all[at - 1].startsWith(">"))).length,
  ol: count(source, /^ {2}1\. /gm),
});
const noRawMarkdown = (html) => {
  assert.ok(!html.includes("**"), "bold markers survived");
  assert.ok(!html.includes("|---"), "table delimiter survived");
  assert.ok(!/^(&gt;|>) /m.test(html), "quote marker survived");
  assert.ok(!html.includes("`"), "backticks survived");
  assert.ok(!/<p>#/.test(html) && !/^- /m.test(html), "heading or bullet marker survived");
};

test("the documents hold one placeholder, the effective date, and are refused until it is set", () => {
  for (const name of ["privacy.md", "terms.md"]) {
    const source = realDoc(name);
    assert.notEqual(source, undefined, `${name} is missing`);
    assert.ok(!source.includes("[VERIFY"), `${name} still holds a [VERIFY marker`);
    assert.equal(source.split("[PUBLISH DATE]").length - 1, 1, `${name} holds exactly one [PUBLISH DATE], on its Effective line`);
    assert.match(source, /^\*\*Effective:\*\* \[PUBLISH DATE\]$/m);
    // as they stand they must not publish: the deploy stops here until the release day is written in
    assert.throws(() => renderDocument(source), (error) => error instanceof LegalError && /unresolved marker \[PUBLISH DATE\]/.test(error.reason));
    // and with the date written in, the same text renders
    assert.doesNotThrow(() => renderDocument(source.replace("[PUBLISH DATE]", "1 January 2027")));
  }
});

test("terms.md renders once its markers are resolved in a test copy", { skip: realDoc("terms.md") === undefined }, () => {
  const source = resolved(realDoc("terms.md"));
  const { title, article } = renderDocument(source);
  assert.equal(title, "ComplyEaze Bridge Terms of Use");
  assert.deepEqual(structure(article), expectedStructure(source));
  assert.deepEqual(structure(article), { h1: 1, h2: 20, table: 1, aside: 1, ol: 0 });
  noRawMarkdown(article);
});

// privacy.md (after its authors flattened a ### heading and a nested bullet list, 30 Sep) renders with the
// allowlist unchanged; the refusal of both constructs is covered by the construct fixtures below.
test("privacy.md renders once its markers are resolved in a test copy", { skip: realDoc("privacy.md") === undefined }, () => {
  const source = resolved(realDoc("privacy.md"));
  const { title, article } = renderDocument(source);
  assert.equal(title, "ComplyEaze Bridge Privacy Policy");
  assert.deepEqual(structure(article), expectedStructure(source));
  const s = structure(article);
  assert.deepEqual([s.h1, s.table, s.aside, s.ol], [1, 4, 1, 1]);
  noRawMarkdown(article);
});

test("headings: one h1, h2 with stable unique ids", () => {
  assert.equal(inner("## 1. Who we are & more"), '<h2 id="1-who-we-are-more">1. Who we are &amp; more</h2>');
  assert.equal(inner("## Same\n\n## Same\n\n## Same"), '<h2 id="same">Same</h2>\n<h2 id="same-2">Same</h2>\n<h2 id="same-3">Same</h2>');
  assert.equal(inner("## — ₹"), '<h2 id="section">— ₹</h2>');
});

test("paragraphs join their lines with a space and lettered items stay paragraphs", () => {
  assert.equal(inner("one\ntwo\n  "), "<p>one two</p>");
  assert.equal(inner("a. First\n\nb. Second\n\n1.1 Clause"), "<p>a. First</p>\n<p>b. Second</p>\n<p>1.1 Clause</p>");
});

test("inline: bold, code, links, bare URLs, unicode and escaping", () => {
  assert.equal(inner("**bold** and `c&d`"), "<p><strong>bold</strong> and <code>c&amp;d</code></p>");
  assert.equal(inner("See [the site](https://a.example/x) or https://b.example/y."), '<p>See <a href="https://a.example/x">the site</a> or <a href="https://b.example/y">https://b.example/y</a>.</p>');
  assert.equal(inner("(https://b.example/y), then more"), '<p>(<a href="https://b.example/y">https://b.example/y</a>), then more</p>');
  assert.equal(inner("**[a](https://x.example)** `_x_` snake_case"), '<p><strong><a href="https://x.example">a</a></strong> <code>_x_</code> snake_case</p>');
  assert.equal(inner("¹²³⁴ § ₹ — ↔ →"), "<p>¹²³⁴ § ₹ — ↔ →</p>");
});

test("escaping covers <, &, quotes and apostrophes in text, code, link text and link targets", () => {
  assert.equal(inner(`a < b & "c" 'd'`), "<p>a &lt; b &amp; &quot;c&quot; &#39;d&#39;</p>");
  assert.equal(inner("`x & <- 1 < 2` ok"), "<p><code>x &amp; &lt;- 1 &lt; 2</code> ok</p>");
  assert.equal(inner(`[a & "b" < c](https://x.example/?a=1&b=2)`), '<p><a href="https://x.example/?a=1&amp;b=2">a &amp; &quot;b&quot; &lt; c</a></p>');
});

test("rules, tables, summary boxes and lists", () => {
  assert.equal(inner("a\n\n---\n\nb"), "<p>a</p>\n<hr>\n<p>b</p>");
  assert.equal(
    inner("| Term | Meaning |\n| :--- | ---: |\n| **A** | x & y |\n| B | |"),
    "<table>\n<thead>\n<tr><th>Term</th><th>Meaning</th></tr>\n</thead>\n<tbody>\n<tr><td><strong>A</strong></td><td>x &amp; y</td></tr>\n<tr><td>B</td><td></td></tr>\n</tbody>\n</table>",
  );
  assert.equal(
    inner("> **Short**\n>\n> - **a** b\n> - c\n>\n> Tail"),
    '<aside class="legal-summary">\n<p><strong>Short</strong></p>\n<ul>\n<li><strong>a</strong> b</li>\n<li>c</li>\n</ul>\n<p>Tail</p>\n</aside>',
  );
  assert.equal(inner("- one\n- two\n  1. x\n  2. **y**\n- three"), "<ul>\n<li>one</li>\n<li>two\n<ol>\n<li>x</li>\n<li><strong>y</strong></li>\n</ol>\n</li>\n<li>three</li>\n</ul>");
});

const failures = [
  ["raw HTML div", "<div>x</div>", 3, /raw HTML/],
  ["raw HTML br inside text", "a\nb <br> c", 4, /raw HTML/],
  ["tag-like text inside code", "`<div>`", 3, /raw HTML/],
  ["HTML comment", "<!-- c -->", 3, /raw HTML/],
  ["image", "![alt](https://x.example/i.png)", 3, /images/],
  ["setext heading with =", "Title\n=====", 4, /setext/],
  ["setext heading with -", "para\nTitle\n---", 5, /setext/],
  ["other rule", "***", 3, /unbalanced \*\*|only a --- line|emphasis|only - bullets/],
  ["rule of four dashes", "----", 3, /only a ---/],
  ["heading level three", "### Deep", 3, /deeper than ##/],
  ["heading level six", "###### Deeper", 3, /deeper than ##/],
  ["underscore emphasis", "some _x_ here", 3, /underscore/],
  ["double underscore emphasis", "some __x__ here", 3, /underscore/],
  ["single-star emphasis", "some *x* here", 3, /emphasis/],
  ["triple-star", "some ***x*** here", 3, /emphasis|\*\*/],
  ["backtick fence", "```\ncode\n```", 3, /code fences/],
  ["tilde fence", "~~~\ncode\n~~~", 3, /code fences/],
  ["indented code", "    code", 3, /indented code/],
  ["footnote", "text[^1]", 3, /footnote/],
  ["reference link", "[x][y]", 3, /reference-style/],
  ["reference definition", "[x]: https://x.example", 3, /reference-style/],
  ["bare brackets", "see [x] here", 3, /square brackets/],
  ["http link", "[a](http://x.example)", 3, /only https links/],
  ["javascript link", "[a](javascript:alert(1))", 3, /only https links/],
  ["relative link", "[a](/x)", 3, /only https links/],
  ["bare http URL", "go to http://x.example now", 3, /only https links/],
  ["tab", "a\tb", 3, /tabs/],
  ["unbalanced bold", "a **b c", 3, /unbalanced \*\*/],
  ["unbalanced backtick", "a `b c", 3, /backtick/],
  ["failure inside a multi-line paragraph names its line", "fine\nbad **line\nfine", 4, /unbalanced \*\*/],
  ["three-space list indent", "- a\n   1. x", 4, /list indent/],
  ["four-space list indent", "- a\n    1. x", 4, /list indent/],
  ["nested bullet", "- a\n  - b", 4, /nested bullet/],
  ["indented continuation of a bullet", "- a\n  more", 4, /list indent/],
  ["text glued to a list", "- a\nplain", 4, /blank line/],
  ["top-level numbered list", "1. one\n2. two", 3, /numbered list/],
  ["star bullet", "* one", 3, /only - bullets/],
  ["nested block quote", "> a\n> > b", 4, /nested block quotes/],
  ["heading in a block quote", "> ## h", 3, /headings/],
  ["table row with too many cells", "| A | B |\n| - | - |\n| 1 | 2 | 3 |", 5, /3 cells but the header has 2/],
  ["table row with too few cells", "| A | B |\n| - | - |\n| 1 |", 5, /1 cells but the header has 2/],
  ["table without delimiter row", "| A | B |\n| 1 | 2 |\n| 3 | 4 |", 4, /delimiter/],
  ["table without body", "| A | B |\n| - | - |", 3, /at least one body row/],
  ["table row without closing pipe", "| A | B |\n| - | - |\n| 1 | 2", 5, /start and end with \|/],
  ["marker VERIFY", "keep [VERIFY x] here", 3, /unresolved marker \[VERIFY/],
  ["marker PUBLISH DATE", "Effective: [PUBLISH DATE]", 3, /unresolved marker \[PUBLISH DATE\]/],
];
for (const [name, body, line, reason] of failures) test(`refused: ${name}`, () => refusedBody(body, line, reason));

test("refused: a document needs exactly one # heading", () => {
  refused("no heading\n", 1, /found 0/);
  refused("# One\n\n# Two\n", 3, /found 2/);
  refused("", 1, /found 0/);
});

test("template markers must each appear exactly once", () => {
  const rendered = { title: "A & <B>", article: "<p>x</p>" };
  const page = applyTemplate(template, rendered);
  assert.match(page, /<title>A &amp; &lt;B&gt;<\/title>/);
  assert.ok(page.includes("<p>x</p>") && !page.includes("<!-- legal"));
  const bad = (t, pattern) => assert.throws(() => applyTemplate(t, rendered), (error) => error instanceof LegalError && pattern.test(error.reason));
  bad(template.replace("<!-- legal -->", ""), /<!-- legal --> exactly once, found 0/);
  bad(`${template}<!-- legal -->`, /<!-- legal --> exactly once, found 2/);
  bad(template.replace("<!-- legal-title -->", ""), /<!-- legal-title --> exactly once, found 0/);
  bad(`${template}<!-- legal-title -->`, /<!-- legal-title --> exactly once, found 2/);
  assert.equal(applyTemplate("<!-- legal-title --><!-- legal -->", { title: "t", article: "$&$1" }), "t$&$1");
});

function cli(files, args) {
  const dir = mkdtempSync(join(tmpdir(), "render-legal-"));
  try {
    for (const [name, text] of Object.entries(files)) writeFileSync(join(dir, name), text);
    const run = spawnSync(process.execPath, [script, ...args(dir)], { encoding: "utf8" });
    return { ...run, out: existsSync(join(dir, "out.html")) ? readFileSync(join(dir, "out.html"), "utf8") : undefined };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}
const page = (dir) => ["page", "--in", join(dir, "in.md"), "--template", join(dir, "t.html"), "--out", join(dir, "out.html")];

test("cli: success writes the page and exits 0", () => {
  const run = cli({ "in.md": "# Privacy & Terms\n\nHello.\n", "t.html": template }, page);
  assert.equal(run.status, 0, run.stderr);
  assert.equal(run.stderr, "");
  assert.match(run.out, /<title>Privacy &amp; Terms<\/title>/);
  assert.match(run.out, /<article class="legal">\n<h1>Privacy &amp; Terms<\/h1>\n<p>Hello\.<\/p>\n<\/article>/);
});

test("cli: a failure writes nothing, names the line on one stderr line and exits non-zero", () => {
  for (const [source, needle] of [["# T\n\n<div>\n", /in\.md:3: raw HTML/], ["# T\n\nEffective: [PUBLISH DATE]\n", /in\.md:3: unresolved marker \[PUBLISH DATE\]/]]) {
    const run = cli({ "in.md": source, "t.html": template }, page);
    assert.equal(run.status, 1);
    assert.equal(run.out, undefined);
    assert.match(run.stderr, needle);
    assert.equal(run.stderr.trimEnd().split("\n").length, 1);
  }
});

test("cli: template errors and missing files write nothing", () => {
  const cases = [
    [template.replace("<!-- legal -->", ""), /t\.html: template must contain <!-- legal --> exactly once/],
    [`${template}<!-- legal -->`, /t\.html: template must contain <!-- legal --> exactly once, found 2/],
    [template.replace("<!-- legal-title -->", ""), /t\.html: template must contain <!-- legal-title --> exactly once/],
  ];
  for (const [t, needle] of cases) {
    const run = cli({ "in.md": "# T\n\nx\n", "t.html": t }, page);
    assert.notEqual(run.status, 0);
    assert.equal(run.out, undefined);
    assert.match(run.stderr, needle);
  }
  const missing = cli({ "t.html": template }, page);
  assert.equal(missing.status, 1);
  assert.equal(missing.out, undefined);
  assert.match(missing.stderr, /^render-legal: .*in\.md: ENOENT/);
});

test("cli: a failure leaves an existing output file untouched; bad usage exits 2", () => {
  const run = cli({ "in.md": "# T\n\n<b>\n", "t.html": template, "out.html": "previous" }, page);
  assert.equal(run.status, 1);
  assert.equal(run.out, "previous");
  for (const args of [[], ["page"], ["page", "--in", "a"], ["page", "--in", "a", "--in", "b", "--template", "c", "--out", "d"], ["notes", "--in", "a", "--template", "b", "--out", "c"], ["page", "--in", "a", "--template", "b", "--out", "c", "--extra", "d"]]) {
    const usage = spawnSync(process.execPath, [script, ...args], { encoding: "utf8" });
    assert.equal(usage.status, 2, args.join(" "));
    assert.match(usage.stderr, /usage: render-legal\.mjs page --in F --template F --out F/);
  }
});
