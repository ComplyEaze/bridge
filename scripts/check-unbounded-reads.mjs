// SPDX-License-Identifier: Apache-2.0
//
// "Missing bound/cap on unbounded output or resource" was 49 of the 880
// findings this coverage is built from. The single most common concrete shape
// of that in a Rust codebase is `Read::read_to_end`/`Read::read_to_string`
// called on a reader that can hand back an attacker- or environment-sized
// amount of data (stdin, a subprocess pipe, a network response, a file whose
// size this process does not control) with nothing capping how much of it
// gets pulled into memory.
//
// This repo already has the fix pattern in wide use — `reader.take(N +
// 1).read_to_end(&mut buf)`, reading one byte past a declared limit so an
// over-limit input is detectable rather than silently truncated (see
// src-tauri/src/agent_bank_statement.rs, agent_desktop_journal.rs, source_draft/files.rs,
// tools/bridge-tally-qualification). This gate checks that every such call
// site actually uses it, so a new call site that forgets the `.take(...)` is
// caught mechanically instead of depending on a reviewer noticing.
//
// Two things a naive `grep` gets wrong, both handled below:
//
//   1. `quick_xml::Reader::read_to_end(end: QName)` is a same-named, wholly
//      different method — "skip forward to this closing tag", bounded by the
//      document's own structure, not a byte sink. `std::io::Read`'s two
//      methods both take `&mut <buffer>` as their argument; quick_xml's does
//      not. Requiring the argument to start with `&mut` is what tells them
//      apart, and bridge-tally-protocol/src/lib.rs has six of exactly this
//      quick_xml call that a bare substring match would misreport.
//   2. A test fixture is allowed to read a small, test-authored buffer
//      unbounded — the hazard is untrusted or environment-sized input, and a
//      `#[test]` function's input is neither. Calls inside a `#[cfg(test)]`
//      module or a `#[test]`/`#[tokio::test]`-attributed function are
//      excluded, tracked by indentation rather than brace-counting: this
//      codebase is rustfmt-clean (see rustfmt.toml and the reported
//      `cargo fmt --check` count), so indentation reliably marks scope, and
//      unlike brace-counting it cannot be thrown off by a `{`/`}` inside a
//      string or XML/JSON literal — of which this codebase's parser tests
//      have many.
//
// Both call forms are read (#837): the method form, `reader.read_to_end(&mut
// buf)`, and the fully qualified form, `std::io::Read::read_to_end(&mut
// reader, &mut buf)` (or `AsyncReadExt::`, and `read_to_string` likewise). The
// scan reads each call in the whole source with comments and string literals
// blanked, never line by line, so formatting cannot split a call from its
// argument and a commented-out call is not counted.
//
// A call is bounded only when the reader it reads IS a `take(...)`: the method
// form's receiver ends in `.take(...)`, `Read::take(...)` or
// `AsyncReadExt::take(...)`, or the qualified form's first argument does. A
// `take` anywhere else, in the same statement or a neighbouring one, does not
// count. Residual, stated rather than hidden: a reader capped on an earlier
// statement and bound to a variable is reported; ALLOWED_UNBOUNDED below is
// the reviewed escape hatch for that.

import { readFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { relative, resolve } from "node:path";

const scriptRoot = fileURLToPath(new URL("../", import.meta.url));
const rootArgument = process.argv.indexOf("--root");
if (rootArgument !== -1 && !process.argv[rootArgument + 1]) {
  throw new Error("--root requires a repository path");
}
const repositoryRoot = rootArgument === -1 ? scriptRoot : resolve(process.argv[rootArgument + 1]);

const SOURCE_ROOTS = ["src-tauri/src", "src-tauri/crates", "tools"];
// The method form; its argument must start `&mut` (see 1. above).
const METHOD_READ = /\.\s*read_to_(?:end|string)\s*\(/g;
// The qualified form; quick_xml's `Reader::read_to_end` is not `Read::`.
const QUALIFIED_READ = /\b(?:Read|AsyncReadExt)\s*::\s*read_to_(?:end|string)\s*\(/g;
// What a capped reader's expression ends in, just before its argument list.
const TAKE_CALLEE = /(?:\.\s*take|\b(?:Read|AsyncReadExt)\s*::\s*take)\s*$/;
const TEST_ATTRIBUTE = /^#\[(?:test|tokio::test|async_std::test|wasm_bindgen_test)\]\s*$/;
const CFG_TEST = /^#\[cfg\(test\)\]\s*$/;
const FN_LINE = /^(\s*)(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s/;
const MOD_LINE = /^(\s*)(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*\{?\s*$/;

// Reviewed, named exceptions — never a bare count. Each entry documents why
// the direct-chain heuristic above cannot see that this call site is already
// bounded, so the exception is legible on its own without re-deriving it.
const ALLOWED_UNBOUNDED = new Set([
  // Zip entries read from a template XLSX/PDF bundled into the binary at
  // build time (via `include_bytes!` / the packaged app resources), not from
  // an untrusted or attacker-sized source. Tracked as a known gap rather than
  // silently accepted: docs/proposed-ci-gates.md's REPORTING entry for this
  // gate lists these as the concrete class-1 findings this scan actually
  // surfaced.
  "src-tauri/src/reports/outstandings_working_paper_xlsx.rs",
  "src-tauri/src/reports/trial_balance_xlsx.rs",
  "src-tauri/src/reports/party_statement_pdf.rs",
]);

function listSourceFiles() {
  const files = [];
  for (const sourceRoot of SOURCE_ROOTS) {
    const result = execFileSync(
      "git",
      ["ls-files", "-z", "--", sourceRoot],
      { cwd: repositoryRoot, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 },
    );
    for (const path of result.split("\0")) {
      if (path && path.endsWith(".rs")) files.push(path);
    }
  }
  return files;
}

function isExcludedByPath(path) {
  return path.includes("/tests/") || /_tests?\.rs$/.test(path);
}

// Returns, for each line index, whether that line is inside a `#[cfg(test)]`
// module or a `#[test]`-family-attributed function — tracked by indentation,
// not braces (see file banner for why).
function testScopeMask(lines) {
  const inside = new Array(lines.length).fill(false);
  const stack = []; // { indent }
  let pendingCfgTest = false;
  let pendingTestAttribute = false;

  for (let index = 0; index < lines.length; index += 1) {
    const line = lines[index];
    const trimmed = line.trim();

    // Pop any scope whose owning line we have now dedented past or to.
    if (trimmed.length) {
      const indent = line.length - line.trimStart().length;
      while (stack.length && indent <= stack[stack.length - 1].indent) stack.pop();
    }

    inside[index] = stack.length > 0;

    if (!trimmed.length) continue; // Blank lines do not reset a pending attribute.

    if (CFG_TEST.test(trimmed)) {
      pendingCfgTest = true;
      continue;
    }
    if (TEST_ATTRIBUTE.test(trimmed)) {
      pendingTestAttribute = true;
      continue;
    }

    const indent = line.length - line.trimStart().length;
    if (pendingCfgTest) {
      const mod = MOD_LINE.exec(line);
      if (mod) stack.push({ indent });
      pendingCfgTest = false;
    } else if (pendingTestAttribute) {
      const fn = FN_LINE.exec(line);
      if (fn) stack.push({ indent });
      pendingTestAttribute = false;
    } else {
      // Any other attribute line (`#[derive(...)]`, `#[allow(...)]`, ...)
      // between the marker and its target is tolerated by simply not
      // clearing pending* here — but only for attribute lines, so a real
      // statement in between correctly drops a stale pending marker.
      if (!/^#\[/.test(trimmed)) {
        pendingCfgTest = false;
        pendingTestAttribute = false;
      }
    }
  }
  return inside;
}

// `text` with every comment and string or character literal blanked to spaces,
// newlines kept, so offsets and line numbers still match the source.
function maskRust(text) {
  const out = text.split("");
  const blank = (from, to) => {
    for (let index = from; index < to; index += 1) if (out[index] !== "\n") out[index] = " ";
  };
  const isIdent = (char) => /[A-Za-z0-9_]/.test(char ?? "");
  let index = 0;
  while (index < text.length) {
    const rest = text.slice(index, index + 2);
    if (rest === "//") {
      const end = text.indexOf("\n", index);
      const stop = end === -1 ? text.length : end;
      blank(index, stop);
      index = stop;
    } else if (rest === "/*") {
      let depth = 0;
      let cursor = index;
      while (cursor < text.length) {
        if (text.startsWith("/*", cursor)) {
          depth += 1;
          cursor += 2;
        } else if (text.startsWith("*/", cursor)) {
          depth -= 1;
          cursor += 2;
          if (depth === 0) break;
        } else cursor += 1;
      }
      blank(index, cursor);
      index = cursor;
    } else if (!isIdent(text[index - 1]) && /^b?r#*"/.test(text.slice(index, index + 260))) {
      const opening = /^b?r(#*)"/.exec(text.slice(index, index + 260));
      const closing = `"${opening[1]}`;
      const end = text.indexOf(closing, index + opening[0].length);
      const stop = end === -1 ? text.length : end + closing.length;
      blank(index, stop);
      index = stop;
    } else if (text[index] === '"') {
      let cursor = index + 1;
      while (cursor < text.length && text[cursor] !== '"') cursor += text[cursor] === "\\" ? 2 : 1;
      blank(index, cursor + 1);
      index = cursor + 1;
    } else if (text[index] === "'") {
      // A character literal, never a lifetime (`'a` has no closing quote).
      const literal = /^'(?:[^'\\\n]|\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]{1,6}\}|.))'/.exec(
        text.slice(index, index + 14),
      );
      const stop = index + (literal ? literal[0].length : 1);
      if (literal) blank(index, stop);
      index = stop;
    } else index += 1;
  }
  return out.join("");
}

// The index of the parenthesis that closes the one opened at `open`, or -1.
function closingParen(masked, open) {
  let depth = 0;
  for (let index = open; index < masked.length; index += 1) {
    if (masked[index] === "(") depth += 1;
    else if (masked[index] === ")" && (depth -= 1) === 0) return index;
  }
  return -1;
}

// Whether `expression` ends in a call to `take`: its last `(...)` is a
// `take`'s argument list.
function endsInTake(expression) {
  const trimmed = expression.trimEnd();
  if (!trimmed.endsWith(")")) return false;
  let depth = 0;
  for (let index = trimmed.length - 1; index >= 0; index -= 1) {
    if (trimmed[index] === ")") depth += 1;
    else if (trimmed[index] === "(" && (depth -= 1) === 0) {
      return TAKE_CALLEE.test(trimmed.slice(0, index));
    }
  }
  return false;
}

// Each read call in `masked`: its offset and whether its own reader is a take.
function readCalls(masked) {
  const calls = [];
  for (const match of masked.matchAll(METHOD_READ)) {
    const open = match.index + match[0].length - 1;
    if (!/^\s*&\s*mut\b/.test(masked.slice(open + 1, open + 64))) continue;
    calls.push({ offset: match.index, bounded: endsInTake(masked.slice(0, match.index)) });
  }
  for (const match of masked.matchAll(QUALIFIED_READ)) {
    const open = match.index + match[0].length - 1;
    const close = closingParen(masked, open);
    const argumentsText = masked.slice(open + 1, close === -1 ? masked.length : close);
    let depth = 0;
    let comma = argumentsText.length;
    for (let index = 0; index < argumentsText.length; index += 1) {
      const char = argumentsText[index];
      if ("([{".includes(char)) depth += 1;
      else if (")]}".includes(char)) depth -= 1;
      else if (char === "," && depth === 0) {
        comma = index;
        break;
      }
    }
    calls.push({ offset: match.index, bounded: endsInTake(argumentsText.slice(0, comma)) });
  }
  return calls.sort((left, right) => left.offset - right.offset);
}

const failures = [];
let scanned = 0;
let boundedCount = 0;

for (const path of listSourceFiles()) {
  if (isExcludedByPath(path)) continue;
  const absolute = resolve(repositoryRoot, path);
  const text = readFileSync(absolute, "utf8");
  const lines = text.split("\n");
  const inTestScope = testScopeMask(lines);

  for (const call of readCalls(maskRust(text))) {
    const index = text.slice(0, call.offset).split("\n").length - 1;
    scanned += 1;
    if (inTestScope[index]) continue;
    if (call.bounded) {
      boundedCount += 1;
      continue;
    }
    if (ALLOWED_UNBOUNDED.has(path)) continue;
    failures.push(`${path}:${index + 1}: ${lines[index].trim()}`);
  }
}

if (failures.length) {
  throw new Error(
    `${failures.length} unbounded Read::read_to_end/read_to_string call(s) found ` +
      `(${scanned} scanned, ${boundedCount} bounded) — each reads an unbounded amount of ` +
      "external data into memory through a reader that is not a `take(N)`. Wrap the " +
      "reader itself in `.take(limit + 1)` (see " +
      "src-tauri/src/agent_bank_statement.rs for the pattern — the `+ 1` lets an " +
      "over-limit input be detected rather than silently truncated), or add a " +
      "reviewed entry to ALLOWED_UNBOUNDED in this script with the reason:\n" +
      failures.map((line) => `  - ${line}`).join("\n"),
  );
}

console.log(
  `Read bound coverage holds: ${scanned} read_to_end/read_to_string call site(s) scanned, ` +
    `${boundedCount} bounded, 0 unbounded (${ALLOWED_UNBOUNDED.size} reviewed exception(s)).`,
);
