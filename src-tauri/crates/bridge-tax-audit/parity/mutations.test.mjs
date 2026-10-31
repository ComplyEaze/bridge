// SPDX-License-Identifier: Apache-2.0
// Tests of mutations.mjs: output parsing, killer-to-file mapping, selection, verdicts (with a fake
// worker in place of cargo), the merge check, the nightly report and issue loop, the worker copy and
// the results file. No cargo run; a throwaway git repository where git is needed.
//
//     node --test src-tauri/crates/bridge-tax-audit/parity/mutations.test.mjs
//
// MUTATIONS_UNDER_TEST names another copy of the runner to test in place of ./mutations.mjs.

import { execFileSync, spawn } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, beforeEach, describe, test } from "node:test";
import assert from "node:assert/strict";
import { fileURLToPath, pathToFileURL } from "node:url";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const UNDER_TEST = path.resolve(process.env.MUTATIONS_UNDER_TEST ?? path.join(HERE, "mutations.mjs"));
const mu = await import(pathToFileURL(UNDER_TEST).href);

// Every temporary file of this run lives here, removed on exit.
const SCRATCH = fs.realpathSync(fs.mkdtempSync(path.join(os.tmpdir(), "mutations-test-")));
process.on("exit", () => fs.rmSync(SCRATCH, { recursive: true, force: true }));

// A guard first on PATH: should a fake cargo be missing, the real toolchain is still never run.
const GUARD = path.join(SCRATCH, "guard");
fs.mkdirSync(GUARD);
for (const tool of ["cargo", "rustc", "rustdoc", "rustup"]) {
  fs.writeFileSync(path.join(GUARD, tool), `#!/bin/sh\necho "mutations.test.mjs: ${tool} is never run here" >&2\nexit 97\n`, { mode: 0o755 });
}
process.env.PATH = `${GUARD}${path.delimiter}${process.env.PATH}`;
const REAL_GIT = execFileSync("sh", ["-c", "command -v git"], { encoding: "utf8" }).trim();

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
const tempDir = () => fs.realpathSync(fs.mkdtempSync(path.join(SCRATCH, "t-")));

// Output shaped as `cargo test` prints it with stderr merged: a status line per target, then that
// binary's own results. "error: test failed" and "error: 2 targets failed" are not compile errors.
const OUTPUT = `   Compiling bridge-tax-audit v0.1.0 (/x/src-tauri/crates/bridge-tax-audit)
    Finished \`test\` profile [unoptimized + debuginfo] target(s) in 9.12s
     Running unittests src/lib.rs (/x/target/debug/deps/bridge_tax_audit-0a1b)

running 3 tests
test book::tests::a_voucher_reference_is_read ... FAILED
test support::tests::ok ... ok
test tests::root_level ... FAILED

failures:

---- book::tests::a_voucher_reference_is_read stdout ----
thread panicked
     Running tests/edge_books.rs (/x/target/debug/deps/edge_books-9f)

running 2 tests
test every_edge_book_matches_the_reference ... FAILED
test other ... ok
error: test failed, to rerun pass \`-p bridge-tax-audit --test edge_books\`

Caused by:
  process didn't exit successfully: \`/x/target/debug/deps/edge_books-9f\` (exit status: 101)
     Running tests/registry.rs (/x/target/debug/deps/registry-11)

running 1 test
test registry_ok ... ok
   Doc-tests bridge_tax_audit

running 1 test
test src/book.rs - book::Voucher (line 12) ... FAILED
error: 2 targets failed:
`;

// A binary that overflowed its stack: libtest names no failed test; cargo reports the signal.
const CRASH = `     Running tests/deep.rs (/x/target/debug/deps/deep-1)

running 1 test

thread 'recurse' has overflowed its stack
fatal runtime error: stack overflow
error: test failed, to rerun pass \`-p bridge-tax-audit --test deep\`

Caused by:
  process didn't exit successfully: \`/x/target/debug/deps/deep-1\` (signal: 6, SIGABRT: process abort signal)
     Running tests/fine.rs (/x/target/debug/deps/fine-2)

running 1 test
test fine ... ok
`;

function crateLayout(root) {
  const files = {
    "src/lib.rs": "pub mod book;\n#[cfg(test)]\nmod tests { #[test] fn root_level() {} }\n",
    "src/book.rs": "// book\n#[test] fn a() {}\n#[test] fn b() {}\n",
    "src/support.rs": '#[test] fn tables() { let _ = include_str!("text_tables.rs"); }\n',
    "src/text_tables.rs": "// generated\n",
    "src/read.rs": "#[cfg(test)]\nmod tests;\n",
    "src/read/tests.rs": "#[test] fn a() {}\n",
    "tests/deep.rs": "#[test] fn recurse() {}\n",
    "src/rules.rs": 'include_str!("../rules/x.toml")\n',
    "tests/edge_books.rs": "mod common;\n#[test] fn every_edge_book_matches_the_reference() {}\n",
    "tests/registry.rs": "#[test] fn registry_ok() {}\n",
    "tests/common/mod.rs": "pub fn fixtures() -> &'static str { \"tests/fixtures\" }\n",
    "tests/fixtures/golden/x.json": "{}\n",
  };
  for (const [rel, text] of Object.entries(files)) {
    const p = path.join(root, rel);
    fs.mkdirSync(path.dirname(p), { recursive: true });
    fs.writeFileSync(p, text, "utf8");
  }
  return root;
}

function mutation(i, file) {
  return { id: i, by: "author", what: i, file, from: `from-${i}`, to: `to-${i}` };
}

/** One open issue as ci.yml's step writes it into the --nightly-issues file. */
function issue(body, number = 612) {
  return `<!-- ${mu.ISSUE_MARK} ${number} -->\n${body}\n`;
}

function record(m, killers, { verdict = mu.KILLED, tree = "t" } = {}) {
  return { verdict, killers, mutation: mu.mutationHash(m), mode: "full", crate_tree: tree, commit: "c", time: "now" };
}

describe("ParseOutput", () => {
  test("test_failures_are_qualified_by_their_target", () => {
    const [failed, compileError] = mu.parseTestOutput(OUTPUT);
    assert.deepEqual(
      failed,
      [
        "src/lib.rs::book::tests::a_voucher_reference_is_read",
        "src/lib.rs::tests::root_level",
        "tests/edge_books.rs::every_edge_book_matches_the_reference",
        "doc::src/book.rs - book::Voucher (line 12)",
      ].sort(),
      "a binary that named its failures is not also recorded as crashed",
    );
    assert.equal(compileError, false, "cargo's 'error: test failed' is not a compile error");
  });

  test("test_a_binary_that_crashes_without_naming_a_failure_is_killed_by_its_target", () => {
    assert.deepEqual(mu.parseTestOutput(CRASH), [["tests/deep.rs::<crashed>"], false]);
    for (const signal of ["signal: 9, SIGKILL: kill", "signal: 15, SIGTERM: termination signal", "exit status: 101"]) {
      const killed = CRASH.replace("signal: 6, SIGABRT: process abort signal", signal);
      assert.deepEqual(mu.parseTestOutput(killed), [[], false], `killed from outside: ${signal}`);
    }
  });

  test("test_a_long_list_is_not_cut", () => {
    let out = "     Running tests/x.rs (/t)\n";
    for (let i = 0; i < 300; i++) out += `test t${String(i).padStart(3, "0")} ... FAILED\n`;
    assert.equal(mu.parseTestOutput(out)[0].length, 300);
  });

  test("test_compile_errors_are_seen", () => {
    for (const line of ["error[E0308]: mismatched types", "error: could not compile `bridge-tax-audit` (lib)"]) {
      assert.equal(mu.parseTestOutput(`   Compiling x\n${line}\n`)[1], true, line);
    }
  });
});

describe("KillerFile", () => {
  test("test_each_kind_maps_to_the_file_holding_the_test", () => {
    const crate = crateLayout(tempDir());
    const workspaceRelative = path.relative(path.dirname(path.dirname(crate)), crate).split(path.sep).join("/");
    const cases = {
      "tests/edge_books.rs::every_edge_book_matches_the_reference": "tests/edge_books.rs",
      "src/lib.rs::book::tests::a": "src/book.rs",
      "src/lib.rs::book::inner::tests::a": "src/book.rs", // nearest existing module file
      "src/lib.rs::read::tests::a": "src/read/tests.rs", // tests in their own file
      "src/lib.rs::tests::root_level": "src/lib.rs",
      "doc::src/book.rs - book::Voucher (line 12)": "src/book.rs",
      "tests/deep.rs::<crashed>": "tests/deep.rs",
      "src/lib.rs::<crashed>": null, // any test in the library binary
      "weird::x": null,
      // the file must still define the test: a rename, a #[path] remap or a macro is unknown
      "src/lib.rs::book::tests::gone": null,
      "src/lib.rs::tests::gone": null,
      "tests/renamed.rs::every_edge_book_matches_the_reference": null,
      "tests/registry.rs::gone": null,
      "tests/renamed.rs::<crashed>": null,
      "doc::src/renamed.rs - x (line 1)": null,
      // cargo prints a doc test's path relative to the workspace
      [`doc::${workspaceRelative}/src/book.rs - book::V (line 3)`]: "src/book.rs",
    };
    for (const [killer, want] of Object.entries(cases)) assert.equal(mu.killerFile(killer, crate), want, killer);
    assert.deepEqual(mu.included("src/support.rs", crate), new Set(["src/text_tables.rs"]));
    assert.deepEqual(mu.included("src/rules.rs", crate), new Set(["rules/x.toml"]));
  });

  test("test_killers_first_arguments", () => {
    assert.deepEqual(mu.killerArgs(["src/lib.rs::book::tests::a", "tests/edge_books.rs::every"]), [
      ["--lib", "--test=edge_books"],
      ["book::tests::a", "every"],
    ]);
    assert.equal(mu.killerArgs(["doc::src/book.rs - x (line 1)"]), null, "a doc test is not run alone");
    assert.equal(mu.killerArgs(["tests/deep.rs::<crashed>"]), null, "a crash names no test to run");
    assert.equal(mu.killerArgs([]), null);
  });
});

describe("Select", () => {
  let s;
  beforeEach(() => {
    const crate = crateLayout(tempDir());
    const book = mutation("B1", "src/book.rs");
    const support = mutation("S1", "src/support.rs");
    const edge = mutation("E1", "src/support.rs");
    s = {
      crate,
      book,
      support,
      edge,
      muts: [book, support, edge],
      results: {
        B1: record(book, ["src/lib.rs::book::tests::a"]),
        // killed by a test that lives in src/book.rs, not in its own target file
        S1: record(support, ["src/lib.rs::book::tests::b"]),
        E1: record(edge, ["tests/edge_books.rs::every_edge_book_matches_the_reference"]),
      },
    };
  });
  const picked = (changed, results = null) => new Set(mu.select(s.muts, results ?? s.results, changed, s.crate).keys());

  test("test_nothing_changed_selects_nothing_proven", () => {
    assert.deepEqual(picked([]), new Set());
  });

  test("test_a_target_file_change_selects_its_mutations", () => {
    assert.deepEqual(picked(["src/support.rs"]), new Set(["S1", "E1"]));
  });

  test("test_a_killers_file_change_selects_the_mutation", () => {
    assert.deepEqual(picked(["tests/edge_books.rs"]), new Set(["E1"]));
    assert.deepEqual(picked(["src/book.rs"]), new Set(["B1", "S1"]));
  });

  test("test_a_file_a_killers_file_includes_selects_the_mutation", () => {
    s.results.B1 = record(s.book, ["src/lib.rs::support::tests::tables"]);
    assert.deepEqual(picked(["src/text_tables.rs"]), new Set(["B1"]));
  });

  test("test_tests_common_selects_every_integration_killer", () => {
    assert.deepEqual(picked(["tests/common/mod.rs"]), new Set(["E1"]));
  });

  test("test_any_non_source_change_selects_everything", () => {
    for (const changed of [
      "tests/fixtures/golden/x.json",
      "tests/fixtures/PROVENANCE.md",
      "parity/python_golden.py",
      "rules/x.toml",
      "Cargo.toml",
      "build.rs",
      "tests/fixtures/helper.rs",
    ]) {
      assert.deepEqual(picked([changed]), new Set(["B1", "S1", "E1"]), changed);
    }
  });

  test("test_unproven_edited_or_unmappable_mutations_are_always_selected", () => {
    const results = { ...s.results };
    delete results.B1;
    results.S1 = record(s.support, ["src/lib.rs::book::tests::b"], { verdict: mu.SURVIVED });
    s.muts[2] = { ...s.edge, to: "something else" };
    assert.deepEqual(picked([], results), new Set(["B1", "S1", "E1"]));
    s.muts[2] = s.edge;
    const crashed = {
      B1: record(s.book, ["src/lib.rs::<crashed>"]),
      S1: s.results.S1,
      E1: record(s.edge, [], { verdict: mu.TIMEOUT }),
    };
    assert.deepEqual(picked([], crashed), new Set(["B1"]), "a timeout record proves; an unknown killer file does not");
  });

  test("test_the_runner_and_its_results_are_inert", () => {
    assert.deepEqual(picked([...mu.INERT]), new Set());
  });

  test("test_the_local_parity_example_is_inert", () => {
    assert.ok(mu.INERT.includes("examples/local_parity.rs"));
    assert.deepEqual(picked(["examples/local_parity.rs"]), new Set());
    // Another example is not inert: it is not named, so its change selects everything.
    assert.notDeepEqual(picked(["examples/other.rs"]), new Set());
  });

  test("test_a_change_to_the_list_selects_only_the_entries_it_changes", () => {
    const changed = ["parity/mutations.json"];
    const added = mutation("N1", "src/book.rs");
    s.muts.push(added);
    assert.deepEqual(picked(changed), new Set(["N1"]), "an added entry has no record");
    s.muts[0] = { ...s.book, to: "edited" };
    assert.deepEqual(picked(changed), new Set(["N1", "B1"]), "an edited entry fails its hash");
    // Once the branch commits records for them, only the base's list still tells them apart.
    const base = Object.fromEntries([s.book, s.support, s.edge].map((m) => [m.id, mu.mutationHash(m)]));
    const killer = ["tests/registry.rs::registry_ok"]; // a killer whose file exists and defines it
    const proven = { ...s.results, N1: record(added, killer), B1: record(s.muts[0], killer) };
    assert.deepEqual(new Set(mu.select(s.muts, proven, changed, s.crate).keys()), new Set(), "without the base: blind");
    assert.deepEqual(new Set(mu.select(s.muts, proven, changed, s.crate, null, base).keys()), new Set(["N1", "B1"]));
    const order = s.muts.map((m) => m.id).filter((i) => i !== "S1"); // S1 removed from the list
    assert.deepEqual(mu.verify([], s.results, "t", order), ["S1: a record for a mutation no longer in the list (delete it)"]);
  });

  test("test_no_crate_source_reads_the_inert_files", () => {
    // INERT is sound only while no test can read these files; name one in a crate source and
    // this fails, so the file must leave INERT (or the test must not read it).
    const crate = path.dirname(HERE);
    const names = [...mu.INERT.map((f) => path.posix.basename(f)), "accepted-survivors.json"];
    const sources = [];
    for (const d of ["src", "tests", "examples", "benches"]) {
      const dir = path.join(crate, d);
      if (!fs.existsSync(dir)) continue;
      for (const rel of fs.readdirSync(dir, { recursive: true })) {
        const p = path.join(dir, rel);
        if (p.endsWith(".rs") && fs.statSync(p).isFile()) sources.push(p);
      }
    }
    if (fs.existsSync(path.join(crate, "build.rs"))) sources.push(path.join(crate, "build.rs"));
    assert.ok(sources.length > 20, "the crate's sources were found");
    const hits = [];
    for (const p of sources) {
      const text = fs.readFileSync(p, "utf8");
      for (const n of names) if (text.includes(n)) hits.push(`${path.relative(crate, p)}: ${n}`);
    }
    assert.deepEqual(hits, []);
  });
});

describe("Verify", () => {
  test("test_a_record_is_fresh_only_on_its_tree_for_its_definition", () => {
    const m = mutation("A", "src/a.rs");
    assert.equal(mu.fresh(m, record(m, ["tests/x.rs::t"], { tree: "T" }), "T"), true);
    assert.equal(mu.fresh(m, record(m, [], { verdict: mu.TIMEOUT, tree: "T" }), "T"), true, "a hang is killed here too");
    assert.equal(mu.fresh(m, record(m, ["tests/x.rs::t"], { tree: "T" }), "U"), false, "another tree");
    assert.equal(mu.fresh({ ...m, to: "edited" }, record(m, ["tests/x.rs::t"], { tree: "T" }), "T"), false);
    for (const verdict of [mu.SURVIVED, mu.COMPILE_ERROR, mu.BUILD_TIMEOUT, mu.FAILED_NO_TEST, mu.DID_NOT_APPLY]) {
      assert.equal(mu.fresh(m, record(m, [], { verdict, tree: "T" }), "T"), false, verdict);
    }
    assert.equal(mu.fresh(m, null, "T"), false);
  });

  test("test_a_record_for_a_retired_mutation_stops_the_merge", () => {
    const a = mutation("A", "src/a.rs");
    const gone = mutation("GONE", "src/a.rs");
    const results = { A: record(a, ["tests/x.rs::t"], { tree: "T" }), GONE: record(gone, [], { tree: "T" }) };
    assert.deepEqual(mu.verify([a], results, "T", ["A", "GONE"]), []);
    assert.deepEqual(mu.verify([a], results, "T", ["A"]), ["GONE: a record for a mutation no longer in the list (delete it)"]);
  });

  test("test_an_accepted_survivor_passes_only_for_its_listed_definition", () => {
    const m = mutation("A", "src/a.rs");
    const other = mutation("B", "src/a.rs");
    const accepted = { A: { mutation: mu.mutationHash(m), reason: "equivalent", documented: "#1" } };
    const survived = record(m, [], { verdict: mu.SURVIVED, tree: "T" });
    assert.equal(mu.passes(m, survived, accepted), true);
    assert.equal(mu.fresh(m, survived, "T", accepted), true);
    assert.equal(mu.fresh(m, survived, "U", accepted), false, "still bound to its tree");
    assert.equal(mu.passes(other, record(other, [], { verdict: mu.SURVIVED }), accepted), false, "not listed");
    const edited = { ...m, to: "edited" }; // the entry's hash is the old definition's
    assert.equal(mu.passes(edited, record(edited, [], { verdict: mu.SURVIVED }), accepted), false, "edited since listed");
    for (const verdict of [mu.COMPILE_ERROR, mu.FAILED_NO_TEST, mu.BUILD_TIMEOUT]) {
      assert.equal(mu.passes(m, record(m, [], { verdict }), accepted), false, verdict);
    }
    assert.equal(mu.select([m], { A: survived }, [], ".", accepted).size, 0, "not re-selected by itself");
    assert.equal(mu.passes(m, survived, { A: { reason: "no hash" } }), false, "an entry must name a hash");
    assert.deepEqual(mu.survivorProblems([m, other], accepted), []);
    assert.deepEqual(mu.survivorProblems([edited], accepted), ["A: accepted as a survivor for another definition"]);
    assert.deepEqual(mu.survivorProblems([other], accepted), ["A: accepted as a survivor but no longer in the list"]);
    const committed = { A: survived, B: record(other, ["tests/x.rs::t"]) };
    let [text, failed] = mu.report([m, other], committed, committed, [], accepted);
    assert.equal(failed, false, text);
    assert.ok(text.includes("## Accepted survivors (1)"), text);
    [text, failed] = mu.report([m, other], { ...committed, A: record(m, ["tests/x.rs::t"]) }, committed, [], accepted);
    assert.equal(failed, false, text);
    assert.ok(text.includes("now killed: remove the entry (warning) (1)"), text);
    [text, failed] = mu.report(
      [edited, other],
      { A: record(edited, [], { verdict: mu.SURVIVED }), B: committed.B },
      { A: record(edited, []), B: committed.B },
      [],
      accepted,
    );
    assert.equal(failed, true);
    assert.deepEqual(mu.failingIds(issue(text)), ["A"]);
    assert.ok(text.includes("Stale accepted-survivor entries (1)"), text);
    const killed = { A: record(m, ["tests/x.rs::t"]), B: committed.B };
    [text, failed] = mu.report([m, other], killed, killed, [], { GONE: { mutation: "x" } });
    assert.equal(failed, true, "a stale entry alone fails the nightly");
  });

  test("test_apply_check_wants_exactly_one_match", () => {
    const crate = tempDir();
    fs.writeFileSync(path.join(crate, "a.rs"), Buffer.from("x = 1;\ny = ₹2;\ny = ₹2;\n", "utf8"));
    const muts = [
      { ...mutation("ONE", "a.rs"), from: "x = 1;" },
      { ...mutation("TWO", "a.rs"), from: "y = ₹2;" },
      { ...mutation("NONE", "a.rs"), from: "z" },
      { ...mutation("NOFILE", "b.rs"), from: "x" },
    ];
    assert.deepEqual(mu.applyCheck(muts, crate), [
      "TWO: `from` found 2 times in a.rs",
      "NONE: `from` found 0 times in a.rs",
      "NOFILE: `from` found 0 times in b.rs",
    ]);
  });
});

/** Stands in for cargo: answers each run with the next scripted [exit code, output], and keeps the
 * arguments and the target file's bytes as each run saw them. */
class FakeWorker {
  constructor(crate, answers, file = "src/a.rs") {
    this.crate = crate;
    this.answers = [...answers];
    this.file = file;
    this.calls = [];
    this.seen = [];
    this.dir = crate;
  }

  async run(args, _timeout) {
    this.calls.push(args);
    this.seen.push(fs.readFileSync(path.join(this.crate, this.file)));
    const answer = this.answers.shift();
    if (answer instanceof Error) throw answer;
    if (answer.length === 3) await sleep(answer[2] * 1000); // [exit code, output, seconds the run takes]
    return answer.slice(0, 2);
  }
}

const BUILT = [0, "    Finished `test` profile\n"];
const PASSED = [0, "     Running tests/x.rs (/t)\ntest t ... ok\n"];
const FAILED = [101, "     Running tests/x.rs (/t)\ntest t ... FAILED\ntest u ... FAILED\n"];
const BROKEN = [101, "error[E0308]: mismatched types\n"];
const HUNG = [null, ""];

describe("RunOne", () => {
  let r;
  beforeEach(() => {
    const crate = tempDir();
    fs.mkdirSync(path.join(crate, "src"));
    const original = Buffer.from("let a = 1;\r\nlet b = 2;\n"); // a CR byte must survive the round trip
    fs.writeFileSync(path.join(crate, "src/a.rs"), original);
    r = { crate, original, m: { ...mutation("A", "src/a.rs"), from: "let a = 1;", to: "let a = 0;" } };
  });
  const runOne = async (answers, prior = null, full = false) => {
    const w = new FakeWorker(r.crate, answers);
    const rec = await mu.runOne(w, r.m, prior, full, 5, 5);
    assert.deepEqual(fs.readFileSync(path.join(r.crate, "src/a.rs")), r.original, "the copy is restored");
    return [w, rec];
  };

  test("test_each_verdict_of_the_full_suite", async () => {
    const cases = [
      [[BUILT, PASSED], mu.SURVIVED, []],
      [[BUILT, FAILED], mu.KILLED, ["tests/x.rs::t", "tests/x.rs::u"]],
      [[BUILT, HUNG], mu.TIMEOUT, []],
      [[BUILT, [101, ""]], mu.FAILED_NO_TEST, []],
      [[BUILT, [101, "error: could not compile `x` (doctest)\n"]], mu.COMPILE_ERROR, []],
      [[BROKEN], mu.COMPILE_ERROR, []],
      [[[101, ""]], mu.COMPILE_ERROR, []],
      [[HUNG], mu.BUILD_TIMEOUT, []],
    ];
    for (const [answers, verdict, killers] of cases) {
      const [w, rec] = await runOne(answers);
      assert.deepEqual([rec.verdict, rec.killers, rec.mode], [verdict, killers, "full"], JSON.stringify(answers));
      assert.deepEqual(w.calls[0], ["--no-run"], "the build is its own step");
      assert.ok(w.seen.every((seen) => seen.includes(Buffer.from("let a = 0;\r\n"))), "every run saw the mutation");
    }
    assert.ok(mu.PASSING.has(mu.TIMEOUT));
    assert.ok(!mu.PASSING.has(mu.BUILD_TIMEOUT));
  });

  test("test_killers_run_first_and_the_full_suite_decides_otherwise", async () => {
    const prior = record(r.m, ["tests/x.rs::t", "src/lib.rs::a::tests::b"]);
    let [w, rec] = await runOne([BUILT, FAILED], prior);
    assert.deepEqual([rec.verdict, rec.mode], [mu.KILLED, "killers-first"]);
    assert.deepEqual(w.calls, [
      ["--no-run", "--lib", "--test=x"],
      ["--lib", "--test=x", "--", "--exact", "a::tests::b", "t"],
    ]);
    [w, rec] = await runOne([BUILT, HUNG], prior);
    assert.deepEqual([rec.verdict, rec.mode], [mu.TIMEOUT, "killers-first"]);
    for (const killersAnswer of [[BUILT, PASSED], [BROKEN], [BUILT, [101, ""]]]) {
      [w, rec] = await runOne([...killersAnswer, BUILT, PASSED], prior);
      assert.deepEqual([rec.verdict, rec.mode], [mu.SURVIVED, "full"], JSON.stringify(killersAnswer));
      assert.deepEqual(w.calls.at(-1), [], "a survivor comes from the full suite");
    }
    [w, rec] = await runOne([BUILT, FAILED], prior, true);
    assert.deepEqual([rec.mode, w.calls.length], ["full", 2], "--full skips the killers");
    [w, rec] = await runOne([BUILT, FAILED], record(r.m, ["tests/x.rs::t"], { verdict: mu.SURVIVED }));
    assert.equal(rec.mode, "full", "only a killed record has killers to try");
  });

  test("test_a_stopped_run_restores_the_file_and_a_stale_mutation_does_not_apply", async () => {
    await assert.rejects(mu.runOne(new FakeWorker(r.crate, [BUILT, new mu.Stopped()]), r.m, null, false, 5, 5), mu.Stopped);
    assert.deepEqual(fs.readFileSync(path.join(r.crate, "src/a.rs")), r.original, "restored on the way out");
    for (const [body, n] of [[Buffer.concat([r.original, Buffer.from("let a = 1;\n")]), 2], [Buffer.from("nothing\n"), 0]]) {
      fs.writeFileSync(path.join(r.crate, "src/a.rs"), body);
      r.original = body;
      const [w, rec] = await runOne([]);
      assert.deepEqual([rec.verdict, rec.note, w.calls], [mu.DID_NOT_APPLY, `${n} matches`, []]);
    }
  });

  test("test_the_baseline_refuses_an_unmutated_suite_that_fails_or_leaves_no_margin", async () => {
    const name = path.basename(r.crate);
    const ok = [new FakeWorker(r.crate, [BUILT, PASSED]), new FakeWorker(r.crate, [BUILT])];
    assert.deepEqual(await mu.baseline(ok, 5, 5), []);
    assert.deepEqual(ok[1].calls, [["--no-run"]], "other workers only build");
    const broken = [new FakeWorker(r.crate, [BUILT, PASSED]), new FakeWorker(r.crate, [BROKEN])];
    assert.deepEqual(await mu.baseline(broken, 5, 5), [`${name}: compile_error`]);
    assert.deepEqual(broken[0].calls, [["--no-run"]], "no suite runs until every copy builds");
    assert.deepEqual(await mu.baseline([new FakeWorker(r.crate, [BUILT, FAILED])], 5, 5), [
      `${name}: fails unmutated: tests/x.rs::t, tests/x.rs::u`,
    ]);
    assert.deepEqual(await mu.baseline([new FakeWorker(r.crate, [BUILT, [101, ""]])], 5, 5), [`${name}: fails unmutated: exit 101`]);
    assert.deepEqual(await mu.baseline([new FakeWorker(r.crate, [BUILT, HUNG])], 5, 5), [
      `${name}: the unmutated suite exceeded --timeout 5s`,
    ]);
    const slow = [new FakeWorker(r.crate, [BUILT, [...PASSED, 0.3]])];
    assert.deepEqual(await mu.baseline(slow, 0.5, 5), ["--timeout 0.5s is under 3x the unmutated suite's 0s"]);
    assert.deepEqual(await mu.baseline([new FakeWorker(r.crate, [BUILT, [...PASSED, 0.1]])], 5, 5), []);
  });
});

// A refused baseline says why: the first compiler lines travel with the typed reason.
describe("BaselineOutput", () => {
  let crate;
  beforeEach(() => {
    crate = tempDir();
    fs.mkdirSync(path.join(crate, "src"));
    fs.writeFileSync(path.join(crate, "src/a.rs"), "x\n");
  });

  test("test_the_baseline_returns_the_compiler_lines_of_a_build_that_fails", async () => {
    const details = [];
    const broken = new FakeWorker(crate, [[101, "   Compiling x\nerror[E0308]: mismatched types\n  --> src/a.rs:1:1\n"]]);
    const problems = await mu.baseline([broken], 5, 5, details);
    assert.deepEqual(problems, [`${path.basename(crate)}: compile_error`]);
    assert.deepEqual(details.slice(0, 2), ["error[E0308]: mismatched types", "  --> src/a.rs:1:1"]);
  });

  test("test_the_lines_are_bounded", async () => {
    const details = [];
    let out = "error[E0308]: first\n";
    for (let i = 0; i < 500; i++) out += `  line ${i}\n`;
    await mu.baseline([new FakeWorker(crate, [[101, out]])], 5, 5, details);
    assert.ok(details.length <= mu.REFUSAL_LINES);
    assert.ok(Math.max(...details.map((line) => line.length)) <= mu.REFUSAL_LINE_CHARS);
  });

  test("test_a_clean_baseline_has_no_lines", async () => {
    const details = [];
    assert.deepEqual(await mu.baseline([new FakeWorker(crate, [BUILT, PASSED])], 5, 5, details), []);
    assert.deepEqual(details, []);
  });
});

const exited = (proc) =>
  proc.exitCode !== null || proc.signalCode !== null ? Promise.resolve() : new Promise((resolve) => proc.once("exit", resolve));

/** Whether `pid` is gone, waiting up to 5 s for it. */
async function gone(pid) {
  for (let i = 0; i < 50; i++) {
    try {
      process.kill(pid, 0);
    } catch (e) {
      if (e.code === "ESRCH") return true;
    }
    await sleep(100);
  }
  return false;
}

/** Whether the process group `pgid` is gone (no member left, its leader reaped), waiting up to 5 s. */
const groupGone = (pgid) => gone(-pgid);

/** The process group of a live `pid`. */
const pgidOf = (pid) => Number(execFileSync("ps", ["-o", "pgid=", "-p", String(pid)], { encoding: "utf8" }).trim());

// `killGroup` on the process groups a worker starts (its own session, as `Worker.run` does).
describe("KillGroup", () => {
  let started = [];
  const start = (...argv) => {
    const proc = spawn(argv[0], argv.slice(1), { detached: true, stdio: "ignore" });
    started.push(proc);
    return proc;
  };
  afterEach(async () => {
    for (const proc of started) {
      if (proc.exitCode === null && proc.signalCode === null) {
        proc.kill("SIGKILL");
        await exited(proc);
      }
    }
    started = [];
  });

  test("test_a_group_whose_leader_exited_unreaped_is_already_gone", async () => {
    // macOS answers kill(-pgid) with EPERM here (the crash that stopped a night run); Linux delivers it.
    // Node reaps a child only on its event loop, so a synchronous wait keeps the leader a zombie.
    const proc = start("/bin/sh", "-c", "exit 0");
    const deadline = Date.now() + 10_000;
    while (Date.now() < deadline) {
      const stat = execFileSync("ps", ["-o", "stat=", "-p", String(proc.pid)], { encoding: "utf8" }).trim();
      if (stat.startsWith("Z")) break;
      Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 10);
    }
    await mu.killGroup(proc);
    await exited(proc);
    assert.equal(proc.exitCode, 0);
  });

  test("test_a_running_group_is_ended", async () => {
    const proc = start("/bin/sleep", "30");
    await mu.killGroup(proc);
    assert.equal(proc.signalCode, "SIGTERM");
    assert.ok(await groupGone(proc.pid), "the group is gone");
  });

  test("test_eperm_from_a_running_leader_is_raised", async (t) => {
    const proc = start("/bin/sleep", "30");
    const kill = t.mock.method(process, "kill", () => {
      throw Object.assign(new Error("kill EPERM"), { code: "EPERM", errno: -1, syscall: "kill" });
    });
    try {
      await assert.rejects(mu.killGroup(proc), { code: "EPERM" });
    } finally {
      kill.mock.restore();
    }
    assert.deepEqual([proc.exitCode, proc.signalCode], [null, null], "still running");
  });
});

describe("Shard", () => {
  test("test_shards_partition_the_list_by_whole_files", () => {
    const muts = Array.from({ length: 23 }, (_, i) => mutation(`M${i}`, `src/f${i % 5}.rs`));
    const seen = [];
    for (let k = 1; k < 4; k++) {
      const part = mu.shard(muts, k, 3);
      const files = new Set(part.map((m) => m.file));
      for (let other = 1; other < 4; other++) {
        if (other !== k) assert.ok(mu.shard(muts, other, 3).every((m) => !files.has(m.file)));
      }
      seen.push(...part.map((m) => m.id));
    }
    assert.deepEqual(seen.sort(), muts.map((m) => m.id).sort());
  });
});

describe("Report", () => {
  let muts;
  let committed;
  let killed;
  beforeEach(() => {
    muts = [mutation("A", "src/a.rs"), mutation("B", "src/b.rs"), mutation("C", "src/c.rs")];
    committed = Object.fromEntries(muts.map((m) => [m.id, record(m, ["tests/x.rs::t", "tests/x.rs::u"])]));
    killed = Object.fromEntries(muts.map((m) => [m.id, record(m, ["tests/x.rs::t", "tests/x.rs::u"])]));
  });

  test("test_all_killed_with_fresh_committed_records_passes", () => {
    const [text, failed] = mu.report(muts, killed, committed);
    assert.equal(failed, false, text);
    assert.equal(mu.failingIds(issue(text)), null, "a passing report names no failing ids");
  });

  test("test_each_failure_kind_fails_the_run_and_names_its_ids", () => {
    const merged = { A: record(muts[0], [], { verdict: mu.SURVIVED }), C: killed.C }; // B not run
    let [text, failed] = mu.report(muts, merged, committed);
    assert.equal(failed, true);
    assert.ok(text.includes("Not killed (1)"), text);
    assert.ok(text.includes("Not run (1)"), text);
    assert.deepEqual(mu.failingIds(issue(text)), ["A", "B"]);
    const stale = { ...committed, A: record({ ...muts[0], to: "edited" }, ["tests/x.rs::t"]) };
    stale.gone = record(mutation("gone", "src/z.rs"), []);
    delete stale.B;
    [text, failed] = mu.report(muts, killed, stale);
    assert.equal(failed, true);
    assert.ok(text.includes("Stale committed records (3)"), text);
    assert.deepEqual(mu.failingIds(issue(text)), ["A", "B"]);
    // An unreadable shard leaves what it held unknown: every id must be re-proved. (An earlier
    // version pinned an EMPTY line here, which let every crate change through.)
    [text, failed] = mu.report(muts, killed, committed, ["shard-3.json: JSONDecodeError"]);
    assert.equal(failed, true);
    assert.ok(text.includes("Shard results not read (1)"), text);
    assert.deepEqual(mu.failingIds(issue(text)), ["A", "B", "C"]);
    assert.ok(mu.splitlines(text)[2].includes(mu.FAILING_MARK), "the line comes first, before any capped section");
    const many = Array.from({ length: mu.REPORT_ROWS + 50 }, (_, i) => mutation(`M${String(i).padStart(3, "0")}`, "src/a.rs"));
    [text, failed] = mu.report(many, {}, {});
    assert.ok(text.includes("- `M099`"));
    assert.ok(!text.includes("- `M100`"));
    assert.ok(text.includes("... and 50 more"));
    assert.equal(mu.failingIds(issue(text)).length, mu.REPORT_ROWS + 50, "the line is never capped");
  });

  test("test_the_ids_of_a_shard_that_refused_are_listed_apart_from_ids_nobody_ran", () => {
    const both = [mu.shard(muts, 1, 2).map((m) => m.id), mu.shard(muts, 2, 2).map((m) => m.id)];
    const refused = {
      "2/2": { shard: "2/2", problems: ["w0: compile_error"], output: ["error[E0463]: can't find crate for core"] },
    };
    let merged = Object.fromEntries(both[0].map((i) => [i, killed[i]]));
    let [text, failed] = mu.report(muts, merged, committed, [], null, refused);
    assert.equal(failed, true);
    assert.ok(text.includes("Shards that refused to run (1)"), text);
    assert.ok(text.includes("`2/2`: w0: compile_error"), text);
    assert.ok(text.includes("error[E0463]: can't find crate for core"), text);
    assert.ok(text.includes(`Not run, their shard refused (${both[1].length})`), text);
    assert.ok(!text.includes("## Not run ("), "nothing is missing beyond the refused shard's ids");
    // The ids still go on the failing line, so a crate change must re-prove them.
    const order = muts.map((m) => m.id);
    assert.deepEqual(mu.failingIds(issue(text)), [...both[1]].sort((a, b) => order.indexOf(a) - order.indexOf(b)));
    // An id outside every refused shard that is missing stays "Not run".
    merged = Object.fromEntries(both[0].slice(1).map((i) => [i, killed[i]]));
    [text] = mu.report(muts, merged, committed, [], null, refused);
    assert.ok(text.includes("## Not run (1)"), text);
    assert.ok(text.includes(`Not run, their shard refused (${both[1].length})`), text);
  });

  test("test_a_refusal_with_no_ids_still_fails_the_run", () => {
    const refused = { "9/9": { shard: "9/9", problems: ["w0: build_timeout"], output: [] } };
    const [text, failed] = mu.report(muts, killed, committed, [], null, refused);
    assert.equal(failed, true, text);
    assert.ok(text.includes("`9/9`: w0: build_timeout"), text);
  });

  test("test_warnings_do_not_fail_the_run", () => {
    const merged = { ...killed, A: record(muts[0], ["tests/x.rs::t"]), B: record(muts[1], [], { verdict: mu.TIMEOUT }) };
    const [text, failed] = mu.report(muts, merged, committed);
    assert.equal(failed, false, text);
    assert.ok(text.includes("fewer than 2 tests (warning) (1)"), text);
    assert.ok(text.includes("a hang, not a failing test (warning) (1)"), text);
  });
});

function sh(repo, ...args) {
  return execFileSync("git", args, { cwd: repo, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] });
}

/** Run `fn` with `env` set in this process (a value of undefined unsets), restored afterwards. */
async function withEnv(env, fn) {
  const saved = { ...process.env };
  for (const [k, v] of Object.entries(env)) {
    if (v === undefined) delete process.env[k];
    else process.env[k] = v;
  }
  try {
    return await fn();
  } finally {
    for (const k of Object.keys(process.env)) if (!(k in saved)) delete process.env[k];
    Object.assign(process.env, saved);
  }
}

/** PATH (and `env`) set so `cargo` runs `script`, restored afterwards. */
function fakeCargo(tmp, script, env, fn) {
  const binDir = path.join(tmp, "bin");
  fs.mkdirSync(binDir, { recursive: true });
  fs.writeFileSync(path.join(binDir, "cargo"), script, { mode: 0o755 });
  fs.chmodSync(path.join(binDir, "cargo"), 0o755);
  return withEnv({ ...env, PATH: `${binDir}${path.delimiter}${process.env.PATH}` }, fn);
}

/** PATH set so `git` runs `script` (which may exec the real git as "$REAL_GIT"), restored afterwards. */
function fakeGit(tmp, script, env, fn) {
  const binDir = path.join(tmp, "gitbin");
  fs.mkdirSync(binDir, { recursive: true });
  fs.writeFileSync(path.join(binDir, "git"), script, { mode: 0o755 });
  fs.chmodSync(path.join(binDir, "git"), 0o755);
  return withEnv({ ...env, REAL_GIT, PATH: `${binDir}${path.delimiter}${process.env.PATH}` }, fn);
}

const FAKE_CARGO = `#!/bin/sh
# A stand-in for cargo: builds instantly; the suite fails when the copy holds B1's mutation.
case " $* " in *" --no-run "*)
  if [ -n "$FAKE_BUILD_FAIL" ]; then echo "error[E0463]: can't find crate for core"; exit 101; fi
  echo "    Finished"; exit 0;; esac
echo "     Running tests/registry.rs (/x)"
if [ -n "$FAKE_FAIL" ] || grep -q koob src/book.rs; then echo "test registry_ok ... FAILED"; exit 101; fi
echo "test registry_ok ... ok"
`;
const HANG_ALWAYS = `#!/bin/sh
# A stand-in for cargo: builds instantly, then every test run hangs (a child in its group).
case " $* " in *" --no-run "*) echo "    Finished"; exit 0;; esac
sleep 30 & echo $! > "$PIDFILE"; wait
`;
const HANG_ON_B1 = `#!/bin/sh
# A stand-in for cargo: builds and passes instantly, but hangs (a child in its group) on B1's mutation.
case " $* " in *" --no-run "*) echo "    Finished"; exit 0;; esac
if grep -q koob src/book.rs; then sleep 30 & echo $! > "$PIDFILE"; wait; fi
echo "     Running tests/registry.rs (/x)"
echo "test registry_ok ... ok"
`;
const SLOW_CARGO = `#!/bin/sh
# A stand-in for cargo that runs a long child in its own process group and records its pid.
sleep 30 &
echo $! > "$PIDFILE"
wait
`;
// FAKE_CARGO, except that the run which kills B1 also deletes src/read.rs from the copy, so the next
// mutation (R1) crashes the runner on reading its target: a crash after a verdict was made.
const CRASH_AFTER_B1 = FAKE_CARGO.replace("then echo \"test registry_ok ... FAILED\";", "then rm -f src/read.rs; echo \"test registry_ok ... FAILED\";");
// A git whose `cat-file` fails, as a lost disk would fail reading the committed files.
const GIT_WITHOUT_CAT_FILE = `#!/bin/sh
if [ "$1" = "cat-file" ]; then cat >/dev/null; echo "disk gone" >&2; exit 1; fi
exec "$REAL_GIT" "$@"
`;
// A git that logs every command line to $GIT_LOG.
const GIT_LOGGED = `#!/bin/sh
printf '%s\\n' "$*" >> "$GIT_LOG"
exec "$REAL_GIT" "$@"
`;

const CRATE = "src-tauri/crates/bridge-tax-audit";

// The runner's git-facing paths, run through `main` against a throwaway repository laid out as
// this one is.
describe("GitRepo", () => {
  let g;
  beforeEach(() => {
    const t = tempDir();
    const repo = path.join(t, "repo");
    const root = path.join(repo, CRATE);
    crateLayout(root);
    g = {
      t,
      repo,
      root,
      lay: mu.layout(root),
      muts: [
        { ...mutation("B1", "src/book.rs"), from: "// book", to: "// koob" },
        { ...mutation("R1", "src/read.rs"), from: "mod tests;", to: "mod tset;" },
      ],
    };
    write("parity/mutations.json", JSON.stringify(g.muts));
    fs.writeFileSync(path.join(repo, "rust-toolchain.toml"), '[toolchain]\nchannel = "1.96.0"\n');
    fs.writeFileSync(path.join(repo, "src-tauri/Cargo.toml"), "[workspace]\n");
    sh(repo, "init", "-q", "-b", "master");
    sh(repo, "-c", "user.name=t", "-c", "user.email=t@example.invalid", "add", "-A");
    commit("base");
  });
  afterEach(() => fs.rmSync(g.t, { recursive: true, force: true }));

  const write = (rel, text) => {
    const p = path.join(g.root, rel);
    fs.mkdirSync(path.dirname(p), { recursive: true });
    fs.writeFileSync(p, text, "utf8");
  };
  const commit = (message) => {
    sh(g.repo, "add", "-A");
    sh(g.repo, "-c", "user.name=t", "-c", "user.email=t@example.invalid", "commit", "-q", "-m", message);
  };
  const main = async (...argv) => {
    const out = [];
    const err = [];
    const { log, error } = console;
    console.log = (...a) => out.push(a.join(" ") + "\n");
    console.error = (...a) => err.push(a.join(" ") + "\n");
    let rc;
    try {
      rc = await mu.main(argv, g.lay);
    } finally {
      console.log = log;
      console.error = error;
    }
    return [rc, out.join("") + err.join("")];
  };
  const order = () => g.muts.map((m) => m.id);
  const readJson = (p) => JSON.parse(fs.readFileSync(p, "utf8"));
  /** Commit killed records for `ids` made on the current tree, as a run would. */
  const prove = (...ids) => {
    const tree = mu.crateTree("HEAD", g.repo, CRATE);
    const results = mu.loadResults(g.lay.results);
    for (const m of g.muts) if (ids.includes(m.id)) results[m.id] = record(m, ["tests/registry.rs::registry_ok"], { tree });
    fs.writeFileSync(g.lay.results, mu.renderResults(results, order()));
    commit("prove " + ids.join(" "));
    assert.equal(mu.crateTree("HEAD", g.repo, CRATE), tree, "the results file is not in the tree");
  };

  test("test_an_entry_the_runner_cannot_judge_is_refused_at_load", async () => {
    for (const [bad, why] of [
      [{ file: "src/./book.rs" }, "file 'src/./book.rs' is not a tracked crate path"],
      [{ file: "./src/book.rs" }, "file './src/book.rs' is not a tracked crate path"],
      [{ file: "src/gone.rs" }, "file 'src/gone.rs' is not a tracked crate path"],
      [{ id: "B 1" }, "'B 1': an id may hold only"],
      [{ id: "B1\n" }, "an id may hold only"],
    ]) {
      write("parity/mutations.json", JSON.stringify([{ ...g.muts[0], ...bad }, g.muts[1]]));
      commit(`a bad entry: ${JSON.stringify(bad)}`);
      const [rc, out] = await main("--verify");
      assert.equal(rc, 2, JSON.stringify(bad));
      assert.ok(out.includes(why), out);
    }
    write("parity/mutations.json", JSON.stringify(g.muts));
    commit("the good list");
    assert.deepEqual(mu.listProblems(g.muts, mu.trackedFiles(g.repo, CRATE)), []);
  });

  test("test_a_list_change_is_judged_against_the_base_list", async () => {
    prove("B1", "R1");
    sh(g.repo, "branch", "base");
    let results = mu.loadResults(g.lay.results);
    const n1 = { ...mutation("N1", "src/book.rs"), from: "fn a()", to: "fn a_()" };
    g.muts.push(n1);
    write("parity/mutations.json", JSON.stringify(g.muts));
    results.N1 = record(n1, ["tests/registry.rs::registry_ok"], { tree: "made-on-another-tree" });
    fs.writeFileSync(g.lay.results, mu.renderResults(results, order()));
    commit("add N1 with a record from another tree");
    let [rc, out] = await main("--verify", "--changed-since", "base");
    assert.equal(rc, 1, out);
    assert.ok(out.includes("N1: new or edited since the base"), out);
    assert.ok(!out.includes("B1:"), "only the entry the change adds");
    prove("N1");
    assert.equal((await main("--verify", "--changed-since", "base"))[0], 0);
    // Removing a proven mutation and re-adding it under a new id with the same definition (a
    // rename) carries its record over: the new id is new since the base, so it is selected.
    g.muts = g.muts.map((m) => (m.id === "R1" ? { ...m, id: "R1b" } : m));
    write("parity/mutations.json", JSON.stringify(g.muts));
    results = mu.loadResults(g.lay.results);
    results.R1b = results.R1;
    delete results.R1;
    fs.writeFileSync(g.lay.results, mu.renderResults(results, order()));
    commit("rename R1");
    [rc, out] = await main("--verify", "--changed-since", "base");
    assert.equal(rc, 1, out);
    assert.ok(out.includes("R1b: new or edited since the base"), out);
  });

  test("test_changed_since_diffs_from_the_merge_base", () => {
    sh(g.repo, "checkout", "-q", "-b", "pr");
    write("src/book.rs", "// book, edited\n");
    commit("pr edit");
    sh(g.repo, "checkout", "-q", "-b", "fixture-only");
    write("tests/fixtures/श्री ₹.json", "{}\n"); // git quotes such a path unless -z
    commit("a non-ASCII fixture");
    assert.ok(mu.changedSince("pr", g.repo, CRATE).includes("tests/fixtures/श्री ₹.json"));
    sh(g.repo, "checkout", "-q", "pr");
    sh(g.repo, "checkout", "-q", "master");
    write("src/read.rs", "#[cfg(test)]\nmod tests; // master moved\n");
    commit("master edit");
    sh(g.repo, "checkout", "-q", "pr");
    assert.deepEqual(mu.changedSince("master", g.repo, CRATE), ["src/book.rs"]);
    sh(g.repo, "mv", `tests/../${CRATE}/tests/registry.rs`, `${CRATE}/tests/reg.rs`);
    commit("rename a killer's file");
    assert.deepEqual(
      mu.changedSince("master", g.repo, CRATE),
      ["src/book.rs", "tests/reg.rs", "tests/registry.rs"],
      "a rename lists its old name too",
    );
  });

  test("test_the_nightly_issue_loop_closes", async () => {
    prove("B1", "R1");
    const issueFile = path.join(path.dirname(g.root), "issues.txt");
    const [text, failed] = mu.report(
      g.muts,
      { B1: record(g.muts[0], [], { verdict: mu.SURVIVED }), R1: mu.loadResults(g.lay.results).R1 },
      mu.loadResults(g.lay.results),
    );
    assert.equal(failed, true);
    const head7 = `<!-- ${mu.ISSUE_MARK} 7 -->`; // as ci.yml writes them
    const head8 = `<!-- ${mu.ISSUE_MARK} 8 -->`;
    fs.writeFileSync(issueFile, `${head7}\n${text}\nRun: https://example.invalid/run\n${head8}\n<!-- ${mu.FAILING_MARK} R1 -->\n`); // a second open issue
    sh(g.repo, "branch", "base");
    // A crate change that selects nothing by itself ...
    write("src/support.rs", "// unrelated\n");
    commit("unrelated crate change");
    assert.equal((await main("--verify", "--changed-since", "base"))[0], 0);
    // ... is refused while the issue is open, because it does not re-prove B1 ...
    let [rc, out] = await main("--verify", "--changed-since", "base", "--nightly-issues", issueFile);
    assert.equal(rc, 1, out);
    assert.ok(out.includes("B1: failing on the nightly run  [NOT PROVEN ON THIS TREE]"), out);
    assert.ok(out.includes("R1: failing on the nightly run  [NOT PROVEN ON THIS TREE]"), out);
    // ... while a change outside the crate is not held by it, B1 still unproven ...
    fs.writeFileSync(path.join(g.repo, "README.md"), "outside the crate\n");
    commit("outside the crate");
    [rc, out] = await main("--verify", "--changed-since", "HEAD~1", "--nightly-issues", issueFile);
    assert.deepEqual([rc, out.includes("B1:")], [0, false], out);
    // ... re-proving only one of the failing ids is not enough ...
    prove("B1");
    [rc, out] = await main("--verify", "--changed-since", "base", "--nightly-issues", issueFile);
    assert.equal(rc, 1, out);
    // ... and the fix that re-proves them all on its own tree passes; an issue without the line refuses.
    prove("R1");
    [rc, out] = await main("--verify", "--changed-since", "base", "--nightly-issues", issueFile);
    assert.equal(rc, 0, out);
    for (const body of [
      `${head7}\nsomeone rewrote the body\n`,
      // one issue lost its line while another kept one: the other must not hide it
      `${head7}\nsomeone rewrote the body\n${head8}\n<!-- ${mu.FAILING_MARK} R1 -->\n`,
      `${head8}\n<!-- ${mu.FAILING_MARK} R1 -->\n${head7}\n`,
    ]) {
      fs.writeFileSync(issueFile, body);
      [rc, out] = await main("--verify", "--changed-since", "base", "--nightly-issues", issueFile);
      assert.equal(rc, 1, body);
      assert.ok(out.includes("no failing-ids line"), out);
    }
    assert.deepEqual(mu.failingIds(""), [], "no open issue holds nothing");
  });

  test("test_failing_ids_reads_the_header_ci_yml_writes", () => {
    const ci = fs.readFileSync(path.resolve(HERE, "../../../..", ".github/workflows/ci.yml"), "utf8");
    assert.ok(ci.includes(`"<!-- ${mu.ISSUE_MARK} \\(.number) -->`), "ci.yml writes the header failingIds reads");
    const line = `<!-- ${mu.FAILING_MARK} A1 -->`;
    assert.deepEqual(mu.failingIds(issue(line, 612) + issue(line.replace("A1", "B2"), 1613)), ["A1", "B2"]);
    assert.equal(mu.failingIds(issue(line, 612) + issue("rewritten", 1613)), null, "multi-digit numbers");
    assert.equal(mu.failingIds(`${line}\nno header at all\n`), null, "a drifted header refuses");
  });

  test("test_verify_fails_on_an_unproven_selection_a_stale_mutation_or_a_retired_record", async () => {
    prove("B1", "R1");
    sh(g.repo, "branch", "base");
    write("src/book.rs", "// book, edited\n");
    commit("edit");
    let [rc, out] = await main("--verify", "--changed-since", "base");
    assert.equal(rc, 1, out);
    prove("B1");
    assert.equal((await main("--verify", "--changed-since", "base"))[0], 0);
    write("src/book.rs", "// nothing to mutate\n");
    commit("the mutation's text is gone");
    prove("B1");
    [rc, out] = await main("--verify", "--changed-since", "base");
    assert.equal(rc, 1, out);
    assert.ok(out.includes("STALE B1: `from` found 0 times"), out);
    write("src/book.rs", "// book\n");
    g.muts = g.muts.slice(1);
    write("parity/mutations.json", JSON.stringify(g.muts));
    commit("retire B1");
    [rc, out] = await main("--verify", "--changed-since", "HEAD");
    assert.equal(rc, 1, out);
    assert.ok(out.includes("B1: a record for a mutation no longer in the list"), out);
    // An accepted-survivor entry for a retired mutation fails the merge check too.
    const delResults = mu.loadResults(g.lay.results);
    delete delResults.B1;
    fs.writeFileSync(g.lay.results, mu.renderResults(delResults, ["R1"]));
    fs.writeFileSync(path.join(g.root, "parity/accepted-survivors.json"), JSON.stringify({ B1: { mutation: "x" } }));
    commit("drop the record; list B1 as a survivor");
    [rc, out] = await main("--verify", "--changed-since", "HEAD");
    assert.equal(rc, 1, out);
    assert.ok(out.includes("B1: accepted as a survivor but no longer in the list"), out);
  });

  /** [exit, which of `ids` the output names] of `--check-anchors`: judged by id, never by message text. */
  const anchors = async (ids, ...argv) => {
    const [rc, out] = await main("--check-anchors", ...argv);
    const esc = (s) => s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    return [rc, new Set(ids.filter((i) => new RegExp(`(?<![\\w.-])${esc(i)}(?![\\w.-])`).test(out)))];
  };

  test("test_check_anchors_exits_2_on_an_anchor_not_exactly_once_in_heads_bytes", async () => {
    const work = path.join(path.dirname(g.root), "mutants");
    write("tests/deep.rs", "#[test] fn recurse() {}\n#[test] fn again() {}\n");
    const twice = { ...mutation("D2", "tests/deep.rs"), from: "#[test] fn", to: "#[ignore] fn" };
    const never = { ...mutation("Z0", "src/support.rs"), from: "not in the file", to: "x" };
    const ids = ["B1", "D2", "Z0"]; // B1 is the clean control
    const stale = JSON.stringify([g.muts[0], twice, never]);
    const clean = JSON.stringify([g.muts[0]]);
    write("parity/mutations.json", stale);
    commit("an anchor twice in a tests/ file, one nowhere in a src/ file");
    assert.deepEqual(await anchors(ids, "--workdir", work), [2, new Set(["D2", "Z0"])]);
    // The working tree differs from HEAD in the list and in a file; HEAD's answer stands ...
    write("parity/mutations.json", clean);
    write("tests/deep.rs", "#[test] fn recurse() {}\n");
    assert.deepEqual(await anchors(ids, "--workdir", work), [2, new Set(["D2", "Z0"])]);
    sh(g.repo, "checkout", "-q", "--", ".");
    write("parity/mutations.json", clean);
    commit("only the clean control");
    assert.deepEqual(await anchors(ids, "--workdir", work), [0, new Set()]);
    // ... both ways: a clean HEAD is not made stale by its working tree.
    write("parity/mutations.json", stale);
    write("src/book.rs", "// nothing to mutate\n");
    assert.deepEqual(await anchors(ids, "--workdir", work), [0, new Set()]);
    sh(g.repo, "checkout", "-q", "--", ".");
    assert.ok(!fs.existsSync(work), "no worker copy is made");
    assert.equal(sh(g.repo, "status", "--porcelain"), "");
  });

  test("test_check_anchors_names_at_most_report_rows_ids", async () => {
    const many = Array.from({ length: mu.REPORT_ROWS + 1 }, (_, i) => ({ ...mutation(`S${i}`, "src/support.rs"), from: `absent ${i};` }));
    write("parity/mutations.json", JSON.stringify(many));
    commit("more stale anchors than a report shows");
    const [rc, named] = await anchors(many.map((m) => m.id));
    assert.deepEqual([rc, named.size], [2, mu.REPORT_ROWS]);
  });

  test("test_check_anchors_refuses_a_head_list_the_runner_cannot_judge", async () => {
    for (const bad of [[g.muts[0], g.muts[0]], [{ ...g.muts[0], id: "B 1" }]]) {
      // every anchor is clean
      write("parity/mutations.json", JSON.stringify(bad));
      commit(`a list the runner cannot judge: ${JSON.stringify(bad)}`);
      write("parity/mutations.json", JSON.stringify(g.muts)); // a good list, not committed
      assert.equal((await anchors(["B1"]))[0], 2, JSON.stringify(bad));
      sh(g.repo, "checkout", "-q", "--", ".");
    }
  });

  test("test_check_anchors_with_ids_or_another_mode_or_option_refuses", async () => {
    const out = path.join(path.dirname(g.root), "out.json");
    for (const extra of [
      ["B1"],
      ["--verify"],
      ["--list"],
      ["--merge", out],
      ["--changed-since", "HEAD"],
      ["--results", out],
      ["--nightly-issues", out],
      ["--report", out],
      ["--full"],
      ["--shard", "1/2"],
    ]) {
      assert.equal((await anchors([], ...extra))[0], 2, JSON.stringify(extra)); // every anchor is clean: 2 is the refusal
    }
    assert.ok(!fs.existsSync(out), "a refusal writes nothing");
    assert.equal((await anchors([]))[0], 0, "the same list alone passes");
  });

  test("test_a_run_refuses_a_dirty_tree_and_a_held_workdir_before_any_build", async () => {
    const work = path.join(path.dirname(g.root), "mutants");
    prove("B1", "R1");
    fs.writeFileSync(path.join(g.repo, "rust-toolchain.toml"), '[toolchain]\nchannel = "stable"\n');
    let [rc, out] = await main("B1", "--workdir", work);
    assert.equal(rc, 2, out);
    assert.ok(out.includes("M rust-toolchain.toml"), out);
    sh(g.repo, "checkout", "-q", "--", "rust-toolchain.toml");
    fs.writeFileSync(g.lay.results, "{\n}\n"); // an uncommitted results file does not stop a run
    const porcelain = sh(g.repo, "status", "--porcelain", "--untracked-files=no");
    assert.ok(porcelain.includes("mutation-results.json"), porcelain);
    assert.deepEqual(mu.dirty(porcelain, g.lay.resultsRel), []);
    const held = mu.lockWorkdir(work);
    try {
      [rc, out] = await main("B1", "--workdir", work);
      assert.equal(rc, 2, out);
      assert.ok(out.includes("another run holds"), out);
    } finally {
      held.release();
    }
  });

  test("a held workdir whose holder is gone is taken over (the pid-file claim)", async () => {
    // Python's flock dies with its holder; the pid-file claim must do the same by reading the pid.
    const outside = g.t;
    const work = path.join(outside, "mutants");
    const lock = path.join(work, ".lock");
    const live = spawn("/bin/sleep", ["30"], { stdio: "ignore" });
    const dead = spawn("/bin/sh", ["-c", "exit 0"], { stdio: "ignore" });
    await exited(dead);
    try {
      fs.mkdirSync(work, { recursive: true });
      fs.writeFileSync(lock, `${live.pid}\n`);
      assert.equal(mu.lockWorkdir(work), null, "a live holder refuses");
      assert.equal(fs.readFileSync(lock, "utf8"), `${live.pid}\n`, "and keeps its claim");
      const [rc, out] = await main("B1", "--workdir", work);
      assert.equal(rc, 2, out);
      assert.ok(out.includes("another run holds"), out);
      fs.writeFileSync(lock, `${dead.pid}\n`);
      const claim = mu.lockWorkdir(work);
      assert.notEqual(claim, null, "a dead holder's claim is taken over");
      assert.equal(fs.readFileSync(lock, "utf8"), `${process.pid}\n`);
      claim.release();
      assert.ok(!fs.existsSync(lock), "a released claim leaves no file");
      fs.writeFileSync(lock, `${dead.pid}\n`);
      await fakeCargo(outside, FAKE_CARGO, {}, async () => {
        const [rc2, out2] = await main("B1", "--workdir", work, "--results", path.join(outside, "r.json"));
        assert.equal(rc2, 0, out2);
      });
      assert.ok(!fs.existsSync(lock), "the run took the claim over and released it");
    } finally {
      live.kill("SIGKILL");
      await exited(live);
    }
  });

  // The takeover gate (.lock.takeover) names its owner's pid in a file inside it, and is stale when that run is gone, not
  // when it is old: a run stalled inside the gate (suspended, or asleep with its machine) keeps it.
  const oldGate = (work, owner) => {
    const gate = path.join(work, ".lock.takeover");
    fs.mkdirSync(gate, { recursive: true });
    if (owner !== null) fs.writeFileSync(path.join(gate, "owner"), `${owner}\n`);
    const minuteAgo = new Date(Date.now() - 60_000);
    fs.utimesSync(gate, minuteAgo, minuteAgo);
    return gate;
  };

  test("a takeover gate whose owner is alive keeps it however long the owner has been stalled", async () => {
    // A real run, stopped (as Ctrl-Z or a sleeping machine would) while it holds the gate: the child suspends itself
    // as it is about to remove the dead holder's claim, which only happens inside the gate.
    const work = path.join(g.t, "stalled-gate");
    fs.mkdirSync(work, { recursive: true });
    const dead = spawn("/bin/sh", ["-c", "exit 0"], { stdio: "ignore" });
    await exited(dead);
    fs.writeFileSync(path.join(work, ".lock"), `${dead.pid}\n`);
    const child = path.join(g.t, "stalls-in-the-gate.mjs");
    fs.writeFileSync(
      child,
      `import fs from "node:fs";
const remove = fs.rmSync;
fs.rmSync = (target, ...rest) => {
  if (String(target).endsWith("/.lock")) process.kill(process.pid, "SIGSTOP");
  return remove(target, ...rest);
};
const mu = await import(${JSON.stringify(pathToFileURL(UNDER_TEST).href)});
mu.lockWorkdir(process.argv[2]);
`,
    );
    const stalled = spawn(process.execPath, [child, work], { stdio: "ignore" });
    const gate = path.join(work, ".lock.takeover");
    try {
      for (let waited = 0; !fs.existsSync(gate); waited += 20) {
        assert.ok(waited < 10_000, "the child reached the gate");
        await new Promise((resolve) => setTimeout(resolve, 20));
      }
      await new Promise((resolve) => setTimeout(resolve, 11_000)); // past the old 10 s limit, while it stays stopped
      assert.equal(mu.lockWorkdir(work), null, "the stalled owner's gate is not taken from it");
      assert.ok(fs.existsSync(gate), "and it is still there");
      assert.equal(fs.readFileSync(path.join(work, ".lock"), "utf8"), `${dead.pid}\n`, "so the dead claim was not touched");
    } finally {
      stalled.kill("SIGKILL");
      await exited(stalled);
      fs.rmSync(gate, { recursive: true, force: true });
    }
  });

  test("a takeover gate whose owner is gone is removed at once, however young it is", async () => {
    const work = path.join(g.t, "dead-owner-gate");
    fs.mkdirSync(work, { recursive: true });
    const dead = spawn("/bin/sh", ["-c", "exit 0"], { stdio: "ignore" });
    await exited(dead);
    fs.writeFileSync(path.join(work, ".lock"), `${dead.pid}\n`);
    const gate = path.join(work, ".lock.takeover");
    fs.mkdirSync(gate);
    fs.writeFileSync(path.join(gate, "owner"), `${dead.pid}\n`); // made just now by a run that has since died
    const claim = mu.lockWorkdir(work);
    assert.notEqual(claim, null, "the dead owner's gate and the dead claim are both taken over");
    assert.equal(fs.readFileSync(path.join(work, ".lock"), "utf8"), `${process.pid}\n`);
    claim.release();
    assert.ok(!fs.existsSync(gate), "no gate left behind");
  });

  test("a takeover gate that never named an owner is taken over once it is old", async () => {
    const work = path.join(g.t, "ownerless-gate");
    fs.mkdirSync(work, { recursive: true });
    const dead = spawn("/bin/sh", ["-c", "exit 0"], { stdio: "ignore" });
    await exited(dead);
    fs.writeFileSync(path.join(work, ".lock"), `${dead.pid}\n`);
    oldGate(work, null); // its maker died between making the directory and naming itself
    const claim = mu.lockWorkdir(work);
    assert.notEqual(claim, null);
    claim.release();
  });

  test("a run whose still-unnamed gate was taken as abandoned starts over instead of failing", async () => {
    // Suspended between making the gate and naming itself for longer than the age limit, a run finds its gate gone: it
    // must not throw out of lockWorkdir, nor remove a gate that is no longer its own.
    const work = path.join(g.t, "gate-taken-before-naming");
    fs.mkdirSync(work, { recursive: true });
    const dead = spawn("/bin/sh", ["-c", "exit 0"], { stdio: "ignore" });
    await exited(dead);
    fs.writeFileSync(path.join(work, ".lock"), `${dead.pid}\n`);
    const write = fs.writeFileSync;
    let taken = false;
    fs.writeFileSync = (target, ...rest) => {
      if (!taken && String(target).endsWith("/.lock.takeover/owner")) {
        taken = true;
        fs.rmSync(path.dirname(String(target)), { recursive: true, force: true }); // another run removed the unnamed gate
      }
      return write(target, ...rest);
    };
    try {
      const claim = mu.lockWorkdir(work);
      assert.ok(taken, "the gate was taken before its owner named itself");
      assert.notEqual(claim, null, "the run started over and took the workdir");
      claim.release();
      assert.ok(!fs.existsSync(path.join(work, ".lock.takeover")), "no gate left behind");
    } finally {
      fs.writeFileSync = write;
    }
  });

  test("concurrent claimants never hold the workdir together, though they start from a dead holder's claim", async () => {
    // The claim must be exclusive as flock was: made whole (a reader never sees it without its pid), and a
    // takeover of a dead holder's claim never removes a claim another run has just made.
    const work = path.join(g.t, "contended");
    const inside = path.join(work, "inside");
    fs.mkdirSync(work, { recursive: true });
    const dead = spawn("/bin/sh", ["-c", "exit 0"], { stdio: "ignore" });
    await exited(dead);
    const child = path.join(g.t, "claimant.mjs");
    fs.writeFileSync(
      child,
      `import fs from "node:fs";
const mu = await import(${JSON.stringify(pathToFileURL(UNDER_TEST).href)});
const [work, inside] = process.argv.slice(2);
let held = 0, overlaps = 0, refused = 0;
for (let i = 0; i < 150; i++) {
  const claim = mu.lockWorkdir(work);
  if (claim === null) { refused++; continue; }
  held++;
  try { fs.mkdirSync(inside); } catch (e) { if (e.code === "EEXIST") overlaps++; else throw e; }
  const until = Date.now() + 2;
  while (Date.now() < until) {}
  fs.rmdirSync(inside, { recursive: false });
  claim.release();
}
console.log(JSON.stringify({ held, overlaps, refused }));
`,
    );
    for (let round = 0; round < 3; round++) {
      fs.writeFileSync(path.join(work, ".lock"), `${dead.pid}\n`); // each round starts from a dead holder's claim
      const runs = Array.from({ length: 4 }, () => {
        const proc = spawn(process.execPath, [child, work, inside], { stdio: ["ignore", "pipe", "inherit"] });
        let text = "";
        proc.stdout.on("data", (chunk) => (text += chunk));
        return exited(proc).then(() => JSON.parse(text));
      });
      const results = await Promise.all(runs);
      assert.deepEqual(
        results.map((r) => r.overlaps),
        [0, 0, 0, 0],
        `two runs held the workdir at once: ${JSON.stringify(results)}`,
      );
      assert.ok(results.reduce((n, r) => n + r.held, 0) > 0, "somebody held it");
      assert.ok(!fs.existsSync(path.join(work, ".lock")), "every claim was released");
      assert.ok(!fs.existsSync(path.join(work, ".lock.takeover")), "no takeover gate left behind");
    }
  });

  test("test_the_copy_holds_the_committed_bytes_and_only_them", () => {
    fs.writeFileSync(path.join(g.root, "src/book.rs"), Buffer.from("// book\r\n"));
    commit("a CR byte");
    fs.writeFileSync(path.join(g.root, "src/book.rs"), Buffer.from("// uncommitted\n"));
    const files = mu.committedFiles("HEAD", g.repo);
    assert.deepEqual(files[`${CRATE}/src/book.rs`], Buffer.from("// book\r\n"));
    assert.ok("rust-toolchain.toml" in files);
    const w = new mu.Worker(0, path.join(path.dirname(g.root), "mutants"), g.lay);
    w.sync(files);
    const kept = path.join(w.tree, "Cargo.toml");
    fs.utimesSync(kept, 1, 1);
    fs.writeFileSync(path.join(w.tree, "stale.rs"), "old"); // e.g. a test file since deleted
    w.sync(files);
    assert.deepEqual(fs.readFileSync(path.join(w.dir, CRATE, "src/book.rs")), Buffer.from("// book\r\n"));
    assert.ok(!fs.existsSync(path.join(w.tree, "stale.rs")));
    assert.equal(fs.statSync(kept).mtimeMs, 1000, "an unchanged file is not re-written");
    fs.writeFileSync(path.join(g.root, "src/book.rs"), Buffer.from("// book, weakened\n"));
    commit("a changed file");
    w.sync(mu.committedFiles("HEAD", g.repo));
    assert.deepEqual(
      fs.readFileSync(path.join(w.dir, CRATE, "src/book.rs")),
      Buffer.from("// book, weakened\n"),
      "a copy file whose bytes differ from HEAD is re-written",
    );
    fs.symlinkSync("book.rs", path.join(g.root, "src/link.rs"));
    commit("a tracked symlink");
    assert.throws(() => mu.committedFiles("HEAD", g.repo), mu.Refusal);
  });

  test("test_a_run_through_main_proves_on_the_committed_tree_and_refuses_a_failing_baseline", async () => {
    const outside = g.t; // beside the repository, not in it
    const work = path.join(outside, "mutants");
    const results = path.join(outside, "r.json");
    const head = sh(g.repo, "rev-parse", "HEAD").trim();
    const tree = mu.crateTree("HEAD", g.repo, CRATE);
    // In place of patching crate_tree and committed_files: a git that logs what it was asked.
    const gitLog = path.join(outside, "git.log");
    let [rc, out] = await fakeGit(outside, GIT_LOGGED, { GIT_LOG: gitLog }, () =>
      fakeCargo(outside, FAKE_CARGO, {}, () => main("B1", "R1", "--workdir", work, "--results", results)),
    );
    const asked = mu.splitlines(fs.readFileSync(gitLog, "utf8"));
    assert.deepEqual(asked.filter((l) => l.startsWith("rev-parse")), ["rev-parse HEAD"], "HEAD is read once");
    assert.deepEqual(
      asked.filter((l) => l.startsWith("ls-tree") && !l.includes("--name-only")),
      [`ls-tree -r ${head} -- ${CRATE}`, `ls-tree -r -z ${head} -- src-tauri rust-toolchain.toml`],
      "the tree and the files both use that commit",
    );
    await fakeCargo(outside, FAKE_CARGO, {}, async () => {
      assert.equal(rc, 1, out); // R1 survives: the fake suite ignores it
      const recs = readJson(results);
      assert.deepEqual(
        Object.fromEntries(Object.entries(recs).map(([k, v]) => [k, [v.verdict, v.killers, v.mode, v.crate_tree, v.commit]])),
        {
          B1: [mu.KILLED, ["tests/registry.rs::registry_ok"], "full", tree, head],
          R1: [mu.SURVIVED, [], "full", tree, head],
        },
      );
      [rc, out] = await main("B1", "--workdir", work, "--results", results);
      assert.deepEqual([rc, readJson(results).B1.mode], [0, "killers-first"], out);
      await withEnv({ FAKE_FAIL: "1" }, async () => {
        [rc, out] = await main("B1", "--workdir", work, "--results", path.join(outside, "r2.json"));
      });
      assert.equal(rc, 2, out);
      assert.ok(out.includes("the unmutated copy does not pass"), out);
      assert.ok(!fs.existsSync(path.join(outside, "r2.json")));
      g.muts.push({ ...mutation("X1", "src/book.rs"), from: "not in the file" });
      write("parity/mutations.json", JSON.stringify(g.muts));
      commit("a stale mutation");
      [rc, out] = await main("B1", "--workdir", work, "--results", results);
      assert.equal(rc, 1, "B1 is killed, but a stale mutation in the list fails the run");
      assert.ok(out.includes("STALE X1"), out);
    });
    assert.equal(sh(g.repo, "status", "--porcelain"), "", "the working tree is never touched");
  });

  test("test_killers_from_supplies_killer_lists_only_and_a_missing_file_or_full_refuses", async () => {
    const outside = g.t;
    const work = path.join(outside, "mutants");
    const first = path.join(outside, "first.json");
    const proof = path.join(outside, "proof.json");
    await fakeCargo(outside, FAKE_CARGO, {}, async () => {
      // A full run gives B1 a killed record with its killer list, as the committed records hold.
      let [rc, out] = await main("B1", "--full", "--workdir", work, "--results", first);
      assert.equal(rc, 0, out);
      const killers = readJson(first);
      assert.deepEqual([killers.B1.verdict, killers.B1.mode], [mu.KILLED, "full"]);
      const before = fs.readFileSync(first);
      // A proof run into an empty results file runs B1's killers first, from those records only.
      [rc, out] = await main("B1", "--killers-from", first, "--workdir", work, "--results", proof);
      assert.equal(rc, 0, out);
      const made = readJson(proof);
      assert.deepEqual(Object.keys(made), ["B1"], "only what this run measured is written");
      assert.deepEqual([made.B1.verdict, made.B1.mode], [mu.KILLED, "killers-first"]);
      assert.deepEqual(fs.readFileSync(first), before, "the killers file is read, never written");
      // Without killer lists it is a full run, which is what proof mode must never quietly become.
      [rc, out] = await main("B1", "--workdir", work, "--results", path.join(outside, "none.json"));
      assert.equal(readJson(path.join(outside, "none.json")).B1.mode, "full", out);
      [rc, out] = await main("B1", "--killers-from", path.join(outside, "absent.json"), "--workdir", work, "--results", proof);
      assert.equal(rc, 2, out);
      assert.ok(out.includes("is not a file"), out);
      [rc, out] = await main("B1", "--full", "--killers-from", first, "--workdir", work, "--results", proof);
      assert.equal(rc, 2, out);
      assert.ok(out.includes("--killers-from is for a run of mutations without --full"), out);
    });
  });

  test("test_a_shard_that_cannot_build_writes_a_typed_refusal_and_prints_the_compiler_lines", async () => {
    const outside = g.t;
    const work = path.join(outside, "mutants");
    const results = path.join(outside, "shard-1.json");
    let [rc, out] = await fakeCargo(outside, FAKE_CARGO, { FAKE_BUILD_FAIL: "1" }, () =>
      main("--full", "--shard", "1/1", "--workdir", work, "--results", results),
    );
    assert.equal(rc, 2, out);
    assert.ok(out.includes("error[E0463]: can't find crate for core"), out);
    const doc = readJson(results);
    assert.deepEqual(Object.keys(doc), ["refused"]);
    assert.equal(doc.refused.shard, "1/1");
    const problem = doc.refused.problems[0];
    assert.equal(problem.slice(problem.indexOf(": ") + 2), "compile_error");
    assert.ok(doc.refused.output.includes("error[E0463]: can't find crate for core"));
    // Not a shard run: no results file, as before.
    [rc, out] = await fakeCargo(outside, FAKE_CARGO, { FAKE_BUILD_FAIL: "1" }, () =>
      main("B1", "--workdir", work, "--results", path.join(outside, "plain.json")),
    );
    assert.equal(rc, 2, out);
    assert.ok(!fs.existsSync(path.join(outside, "plain.json")));
  });

  // Python exits 1 on an uncaught exception, which the shard step reads as "a mutation not killed"
  // and leaves green. Any crash is exit 2, and before the first mutation it is a typed refusal like
  // a baseline failure.
  test("test_a_shard_that_crashes_before_any_mutation_writes_a_typed_refusal_and_exits_2", async () => {
    const outside = g.t;
    const results = path.join(outside, "shard-1.json");
    // In place of a committed_files that raises: a git that cannot read the committed files.
    const [rc, out] = await fakeGit(outside, GIT_WITHOUT_CAT_FILE, {}, () =>
      fakeCargo(outside, FAKE_CARGO, {}, () =>
        main("--full", "--shard", "1/1", "--workdir", path.join(outside, "mutants"), "--results", results),
      ),
    );
    assert.equal(rc, 2, out);
    const doc = readJson(results);
    assert.deepEqual(Object.keys(doc), ["refused"]);
    assert.equal(doc.refused.shard, "1/1");
    assert.deepEqual(doc.refused.problems, ["crashed: Error"]);
    assert.ok(doc.refused.output[0].startsWith("Error: Command failed: git cat-file --batch"), JSON.stringify(doc));
    assert.ok(doc.refused.output[0].includes("disk gone"), JSON.stringify(doc));
    assert.ok(out.includes("disk gone"), out);
  });

  test("test_a_crash_after_a_mutation_has_run_keeps_its_records_and_exits_2", async () => {
    const outside = g.t;
    const results = path.join(outside, "shard-1.json");
    // In place of a run_one that raises on its second call: B1's killing run deletes R1's target.
    const [rc, out] = await fakeCargo(outside, CRASH_AFTER_B1, {}, () =>
      main("B1", "R1", "--full", "--shard", "1/1", "--workdir", path.join(outside, "mutants"), "--results", results),
    );
    assert.equal(rc, 2, out);
    const doc = readJson(results);
    assert.ok(!("refused" in doc), "records already made are never replaced by a refusal");
    assert.deepEqual(Object.keys(doc).sort(), ["B1"]);
  });

  test("test_a_crash_whose_refusal_cannot_be_written_still_exits_2", async () => {
    const outside = g.t;
    const unwritable = path.join(outside, "shard-1.json");
    fs.mkdirSync(unwritable); // replacing a directory with the refusal file fails
    const [rc, out] = await fakeGit(outside, GIT_WITHOUT_CAT_FILE, {}, () =>
      fakeCargo(outside, FAKE_CARGO, {}, () =>
        main("--full", "--shard", "1/1", "--workdir", path.join(outside, "mutants"), "--results", unwritable),
      ),
    );
    assert.equal(rc, 2, out);
    assert.ok(out.includes("disk gone"), out);
  });

  test("test_a_crash_in_a_run_that_is_not_a_shard_exits_2_and_writes_no_file", async () => {
    const outside = g.t;
    const [rc, out] = await fakeGit(outside, GIT_WITHOUT_CAT_FILE, {}, () =>
      fakeCargo(outside, FAKE_CARGO, {}, () =>
        main("B1", "--workdir", path.join(outside, "mutants"), "--results", path.join(outside, "r.json")),
      ),
    );
    assert.equal(rc, 2, out);
    assert.ok(!fs.existsSync(path.join(outside, "r.json")));
  });

  test("test_a_shard_run_without_results_never_overwrites_the_committed_results_file", async () => {
    const outside = g.t;
    const bytes = () => (fs.existsSync(g.lay.results) ? fs.readFileSync(g.lay.results) : null);
    const before = bytes();
    const [rc, out] = await fakeCargo(outside, FAKE_CARGO, { FAKE_BUILD_FAIL: "1" }, () =>
      main("--full", "--shard", "1/1", "--workdir", path.join(outside, "mutants")),
    );
    assert.equal(rc, 2, out);
    assert.deepEqual(bytes(), before);
  });

  test("test_a_corrupt_refusal_document_is_not_read_and_does_not_stop_the_merge", async () => {
    const parent = path.dirname(g.root);
    const good = path.join(parent, "shard-2.json");
    fs.writeFileSync(good, JSON.stringify(Object.fromEntries(g.muts.map((m) => [m.id, record(m, ["tests/registry.rs::registry_ok"])]))));
    const corrupt = [
      '"refused"', // a string, not a document
      JSON.stringify({ refused: null }),
      JSON.stringify({ refused: "x" }),
      JSON.stringify({ refused: { problems: [], output: [] } }), // no shard
      JSON.stringify({ refused: { shard: "abc", problems: [], output: [] } }),
      JSON.stringify({ refused: { shard: "0/0", problems: [], output: [] } }),
      JSON.stringify({ refused: { shard: "3/2", problems: [], output: [] } }),
      JSON.stringify({ refused: { shard: "1/2", problems: "w0", output: [] } }),
      JSON.stringify({ refused: { shard: "1/2", problems: [], output: [7] } }),
    ];
    for (const [i, text] of corrupt.entries()) {
      const bad = path.join(parent, `bad-${i}.json`);
      fs.writeFileSync(bad, text);
      const [rc, out] = await main("--merge", bad, good, "--results", path.join(parent, "all.json"), "--report", path.join(parent, "r.md"));
      const report = fs.readFileSync(path.join(parent, "r.md"), "utf8");
      assert.equal(rc, 1, `${text}: ${out}`);
      assert.ok(report.includes("Shard results not read (1)"), text);
      assert.ok(!report.includes("Shards that refused to run"), text);
    }
  });

  test("test_a_shard_file_that_is_json_but_not_records_is_not_read_and_does_not_stop_the_merge", async () => {
    // Valid JSON that is not an object of records (#1041): each is "not read", the run fails
    // naming every id, and the good shard's records still merge. Read after the good shard, so
    // a shape that merged would overwrite its records rather than hide behind them.
    const parent = path.dirname(g.root);
    const good = path.join(parent, "shard-1.json");
    const records = Object.fromEntries(g.muts.map((m) => [m.id, record(m, ["tests/registry.rs::registry_ok"])]));
    fs.writeFileSync(good, JSON.stringify(records));
    prove("B1", "R1");
    for (const text of ["null", "5", '""', "[]", "[1, 2]", JSON.stringify({ B1: "text" })]) {
      const bad = path.join(parent, "shard-2.json");
      fs.writeFileSync(bad, text);
      for (const written of [path.join(parent, "all.json"), path.join(parent, "r.md")]) fs.rmSync(written, { force: true }); // never read a previous shape's
      const [rc, out] = await main("--merge", good, bad, "--results", path.join(parent, "all.json"), "--report", path.join(parent, "r.md"));
      assert.equal(rc, 1, `shard=${text}: ${out}`);
      const report = fs.readFileSync(path.join(parent, "r.md"), "utf8");
      assert.ok(report.includes("Shard results not read (1)"), `shard=${text}`);
      assert.deepEqual(mu.failingIds(issue(report)), ["B1", "R1"], `shard=${text}`);
      assert.deepEqual(readJson(path.join(parent, "all.json")), records, `shard=${text}`);
    }
  });

  test("test_merge_reads_a_refusal_apart_from_results_and_never_writes_it_into_them", async () => {
    const parent = path.dirname(g.root);
    const refused = path.join(parent, "shard-1.json");
    fs.writeFileSync(refused, JSON.stringify({ refused: { shard: "1/2", problems: ["w0: compile_error"], output: ["error[E0463]: x"] } }));
    const good = path.join(parent, "shard-2.json");
    const ids2 = mu.shard(g.muts, 2, 2).map((m) => m.id);
    fs.writeFileSync(
      good,
      JSON.stringify(Object.fromEntries(g.muts.filter((m) => ids2.includes(m.id)).map((m) => [m.id, record(m, ["tests/registry.rs::registry_ok"])]))),
    );
    prove("B1", "R1");
    const [rc, out] = await main("--merge", refused, good, "--results", path.join(parent, "all.json"), "--report", path.join(parent, "r.md"));
    assert.equal(rc, 1, out);
    const report = fs.readFileSync(path.join(parent, "r.md"), "utf8");
    assert.ok(report.includes("Shards that refused to run (1)"), report);
    assert.ok(!report.includes("Shard results not read"), report);
    assert.ok(!("refused" in readJson(path.join(parent, "all.json"))));
  });

  test("test_an_accepted_survivor_passes_every_path_through_main", async () => {
    const outside = g.t;
    const work = path.join(outside, "mutants");
    const results = path.join(outside, "r.json");
    const r1 = g.muts.find((m) => m.id === "R1"); // the fake suite never kills R1
    fs.writeFileSync(g.lay.survivors, JSON.stringify({ R1: { mutation: mu.mutationHash(r1), reason: "test" } }));
    commit("accept R1 as a survivor");
    let [rc, out] = await fakeCargo(outside, FAKE_CARGO, {}, () => main("B1", "R1", "--workdir", work, "--results", results));
    assert.equal(rc, 0, out); // the run's own judgement
    assert.equal(readJson(results).R1.verdict, mu.SURVIVED);
    [rc, out] = await main("R1", "--verify", "--results", results);
    assert.equal(rc, 0, out); // the merge check
    assert.ok(out.includes("1 selected, 1 proven"), out);
    assert.ok(!out.includes("NOT PROVEN"), out);
    fs.writeFileSync(path.join(g.root, "src/support.rs"), "// unrelated\n");
    commit("an unrelated source change");
    [rc, out] = await main("--list", "--changed-since", "HEAD~1", "--results", results);
    assert.ok(!out.includes("R1:"), "selection does not re-pick an accepted survivor by itself");
    fs.writeFileSync(g.lay.results, fs.readFileSync(results, "utf8"));
    commit("commit the records");
    [rc, out] = await main("--merge", results, "--results", path.join(outside, "all.json"));
    assert.equal(rc, 0, out); // the nightly's judgement
    assert.ok(out.includes("## Accepted survivors (1)"), out);
  });

  const sigtermRun = async (script) => {
    const outside = g.t;
    const runner = path.join(g.root, "parity/mutations.mjs"); // the runner committed into the throwaway repo
    fs.copyFileSync(UNDER_TEST, runner);
    commit("the runner");
    const pidfile = path.join(outside, "pid");
    const work = path.join(outside, "mutants");
    const results = path.join(outside, "r.json");
    let start;
    let code;
    let output = "";
    let group;
    await fakeCargo(outside, script, { PIDFILE: pidfile }, async () => {
      const proc = spawn(process.execPath, [runner, "B1", "--workdir", work, "--results", results], {
        stdio: ["ignore", "pipe", "pipe"],
        env: process.env,
      });
      proc.stdout.on("data", (d) => (output += d));
      proc.stderr.on("data", (d) => (output += d));
      const done = new Promise((resolve) => proc.once("close", (c, s) => resolve(c ?? s)));
      try {
        for (let i = 0; i < 200; i++) {
          if (fs.existsSync(pidfile) && fs.readFileSync(pidfile, "utf8").trim()) break;
          await sleep(50);
        }
        assert.ok(fs.existsSync(pidfile), "the mutated suite started");
        group = pgidOf(Number(fs.readFileSync(pidfile, "utf8")));
        start = performance.now();
        proc.kill("SIGTERM"); // this test's own child, by its PID
        code = await Promise.race([done, sleep(20_000).then(() => "no exit in 20 s")]);
      } finally {
        if (proc.exitCode === null && proc.signalCode === null) {
          proc.kill("SIGKILL");
          await done;
        }
      }
    });
    assert.equal(code, 130, output);
    assert.ok(performance.now() - start < 15_000, "the stop did not wait for the hung suite");
    assert.ok(await gone(Number(fs.readFileSync(pidfile, "utf8"))), "the hung suite's child is gone");
    assert.ok(await groupGone(group), "the hung suite's process group is gone");
    assert.ok(!fs.readFileSync(path.join(work, "w0", CRATE, "src/book.rs")).includes("koob"), "the copy is restored");
    assert.ok(!fs.existsSync(results), "no verdict for the stopped mutation");
  };

  test("test_sigterm_stops_a_run_ends_its_cargo_and_restores_the_copy", async () => {
    await sigtermRun(HANG_ON_B1);
  });

  test("test_sigterm_during_the_baseline_ends_its_cargo", async () => {
    await sigtermRun(HANG_ALWAYS);
  });

  test("test_merge_reports_an_unreadable_shard_and_never_writes_the_committed_file", async () => {
    const parent = path.dirname(g.root);
    const shardFile = path.join(parent, "shard-1.json");
    fs.writeFileSync(shardFile, '{\n  "B1": {"verdict": "kil'); // truncated
    let [rc, out] = await main("--merge", shardFile, path.join(parent, "missing.json"));
    assert.equal(rc, 2, out);
    assert.ok(out.includes("refusing: --merge writes --results"), out);
    [rc, out] = await main("--merge", shardFile, path.join(parent, "missing.json"), "--results", path.join(parent, "all.json"), "--report", path.join(parent, "r.md"));
    assert.equal(rc, 1, out);
    const report = fs.readFileSync(path.join(parent, "r.md"), "utf8");
    assert.ok(report.includes("Shard results not read (2)"), report);
    assert.deepEqual(mu.failingIds(issue(report)), ["B1", "R1"]);
    assert.ok(!fs.existsSync(g.lay.results));
    // Every shard killed everything, but the committed file lacks those records: still a failure.
    const good = path.join(parent, "shard-2.json");
    fs.writeFileSync(good, JSON.stringify(Object.fromEntries(g.muts.map((m) => [m.id, record(m, ["tests/registry.rs::registry_ok"])]))));
    [rc, out] = await main("--merge", good, "--results", path.join(parent, "all.json"));
    assert.equal(rc, 1, out);
    assert.ok(out.includes("B1: no committed record"), out);
    prove("B1", "R1");
    assert.equal((await main("--merge", good, "--results", path.join(parent, "all.json")))[0], 0);
  });
});

describe("StopPath", () => {
  test("test_a_stop_or_a_timeout_ends_the_cargo_group_and_a_stop_gives_no_verdict", async () => {
    const tmp = tempDir();
    const pidfile = path.join(tmp, "pid");
    const w = new mu.Worker(0, path.join(tmp, "mutants"));
    fs.mkdirSync(w.crate, { recursive: true });
    await fakeCargo(tmp, SLOW_CARGO, { PIDFILE: pidfile }, async () => {
      try {
        mu.state.stop = true;
        await assert.rejects(w.run([], 5), mu.Stopped);
        assert.ok(!fs.existsSync(pidfile), "a stopped run starts nothing");
        mu.state.stop = false;
        let group;
        const stopOnceStarted = async () => {
          // a loaded machine can start the fake cargo late
          for (let i = 0; i < 200; i++) {
            if (fs.existsSync(pidfile) && fs.readFileSync(pidfile, "utf8").trim()) break;
            await sleep(50);
          }
          group = [...mu.state.active][0]?.pid;
          mu.state.stop = true;
          await mu.killActive();
        };
        const start = performance.now();
        const stopper = stopOnceStarted();
        await assert.rejects(w.run([], 60), mu.Stopped);
        await stopper;
        assert.ok(performance.now() - start < 15_000, "the stop ended the run");
        assert.ok(await gone(Number(fs.readFileSync(pidfile, "utf8"))), "the child in cargo's group is gone");
        assert.ok(group !== undefined && (await groupGone(group)), "cargo's group is gone");
        mu.state.stop = false;
        fs.unlinkSync(pidfile);
        const [rc] = await w.run([], 1);
        assert.equal(rc, null, "a timeout");
        assert.ok(await gone(Number(fs.readFileSync(pidfile, "utf8"))));
        assert.equal(mu.state.active.size, 0);
      } finally {
        mu.state.stop = false;
      }
    });
    fs.unlinkSync(pidfile);
    // KILL_GRACE is a constant here, so the ignored SIGTERM is put to killGroup with a short grace
    // directly, on a group started as Worker.run starts one.
    const script = path.join(tmp, "ignores-term");
    fs.writeFileSync(script, "#!/bin/sh\ntrap '' TERM\n" + SLOW_CARGO.split("\n").slice(2).join("\n"), { mode: 0o755 });
    const proc = spawn(script, [], { detached: true, stdio: "ignore", env: { ...process.env, PIDFILE: pidfile } });
    try {
      for (let i = 0; i < 200; i++) {
        if (fs.existsSync(pidfile) && fs.readFileSync(pidfile, "utf8").trim()) break;
        await sleep(50);
      }
      const start = performance.now();
      await mu.killGroup(proc, 0.5);
      assert.ok(performance.now() - start < 15_000, "SIGKILL follows an ignored SIGTERM");
      assert.ok(await gone(Number(fs.readFileSync(pidfile, "utf8"))), "the group that ignored SIGTERM is gone");
      assert.ok(await groupGone(proc.pid));
    } finally {
      if (proc.exitCode === null && proc.signalCode === null) proc.kill("SIGKILL");
    }
  });
});

describe("ResultsFile", () => {
  test("test_one_record_per_line_in_list_order_and_valid_json", () => {
    const records = { B: { verdict: "killed", killers: ["x"] }, A: { verdict: "survived", killers: [] } };
    const text = mu.renderResults(records, ["A", "B"]);
    assert.deepEqual(JSON.parse(text), records);
    const lines = mu.splitlines(text);
    assert.deepEqual([lines[0], lines.at(-1)], ["{", "}"]);
    assert.ok(lines[1].trimStart().startsWith('"A"') && lines[2].trimStart().startsWith('"B"'));
    assert.equal(mu.renderResults({}, []), "{\n}\n");
    const t = tempDir();
    const file = path.join(t, "r.json");
    mu.writeAtomic(file, text);
    assert.deepEqual([fs.readFileSync(file, "utf8"), fs.readdirSync(t).sort()], [text, ["r.json"]]);
  });

  test("test_a_mutations_hash_covers_file_from_and_to_only", () => {
    const m = mutation("X", "src/a.rs");
    assert.equal(mu.mutationHash(m), mu.mutationHash({ ...m, what: "reworded", by: "reviewer" }));
    for (const key of ["file", "from", "to"]) {
      assert.notEqual(mu.mutationHash(m), mu.mutationHash({ ...m, [key]: "changed" }), key);
    }
  });
});
