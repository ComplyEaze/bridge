// SPDX-License-Identifier: Apache-2.0

import { existsSync, readFileSync, readdirSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const scriptRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const rootArgument = process.argv.indexOf("--root");
if (rootArgument !== -1 && !process.argv[rootArgument + 1]) {
  throw new Error("--root requires a repository path");
}
const repositoryRoot = rootArgument === -1 ? scriptRoot : resolve(process.argv[rootArgument + 1]);

// These are legacy, deliberately quarantined request profiles. The match is
// exact: removing one does not create capacity for another, and adding either
// hazard anywhere else fails this gate. New exceptions require reviewed edits
// to this list and a distinct request-profile decision.
const expected = new Set([
  "custom-report|src-tauri/crates/bridge-tally-protocol/src/xml_read_profiles.rs::render_company_list|Company Report",
  "custom-report|src-tauri/crates/bridge-tally-protocol/src/xml_read_profiles.rs::render_ledgers|BRIDGE Ledger Export V1",
  "custom-report|src-tauri/crates/bridge-tally-protocol/src/xml_read_profiles.rs::render_vouchers|BRIDGE Voucher Export V2",
  "custom-report|src-tauri/src/tally/tdl_engine.rs::groups_request|BRIDGE Group Export V1",
  "custom-report|src-tauri/src/tally/tdl_engine.rs::ledger_period_balances_request|BRIDGE Ledger Period Balances V1",
  "custom-report|src-tauri/src/tally/tdl_engine.rs::legacy_company_list_request|Company Report",
  "function-argument-with-space|src-tauri/crates/bridge-tally-protocol/src/xml_read_profiles.rs::render_ledgers|$$NumItems:BRIDGE Ledger Collection V1",
  "function-argument-with-space|src-tauri/crates/bridge-tally-protocol/src/xml_read_profiles.rs::render_vouchers|$$NumItems:BRIDGE Voucher Collection V1",
  "function-argument-with-space|src-tauri/src/tally/tdl_engine.rs::groups_request|$$NumItems:BRIDGE Group Collection V1",
  "function-argument-with-space|src-tauri/src/tally/tdl_engine.rs::ledger_period_balances_request|$$NumItems:BRIDGE Ledger Period Collection V1",
]);

// Method and function names whose value is money, by suffix. A money method
// named otherwise is not caught, and a FIELD with no <SET> is not scanned.
const amountMethod = /(?:Balance|Amount|Opening|Closing|Totals?|Debit|Credit|Value|Limit)$/i;

// The attributes and head of a `mod` item.
const ATTRIBUTES = String.raw`((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*`;

const actual = new Set();
const sources = ["src-tauri", "tools"].flatMap((sourceRoot) => rustFiles(resolve(repositoryRoot, sourceRoot)));
// A file reached only through a test-only module declaration is test code, by
// its #[cfg(test)] attribute rather than by its name (#837).
const quarantined = testOnlyFiles(sources);
for (const path of sources) {
  if (quarantined.has(path)) continue;
  scanRequestBuilderStrings(repositoryRoot, path, actual);
}

const unexpected = [...actual].filter((violation) => !expected.has(violation)).sort();
const missing = [...expected].filter((violation) => !actual.has(violation)).sort();
if (unexpected.length || missing.length) {
  throw new Error(
    "Tally request-builder hazard allowlist changed:\n" +
      (unexpected.length ? `unexpected:\n${unexpected.map((value) => `- ${value}`).join("\n")}\n` : "") +
      (missing.length ? `missing:\n${missing.map((value) => `- ${value}`).join("\n")}\n` : "") +
      "Use a native Collection export by default; a new exception requires a reviewed exact-set update.",
  );
}

console.log(
  `Tally request-builder hazards match the pinned set (${actual.size} violations; ` +
    `${quarantined.size} test-only file(s) skipped by their #[cfg(test)] mod declaration).`,
);

function rustFiles(directory) {
  const files = [];
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    // "tests" is skipped here (integration-test crates live entirely under a
    // top-level tests/ directory) as part of the test-quarantine strategy
    // described in scanRequestBuilderStrings() below.
    if (["target", ".git", "tests"].includes(entry.name)) continue;
    const path = resolve(directory, entry.name);
    if (entry.isDirectory()) files.push(...rustFiles(path));
    else if (entry.isFile() && entry.name.endsWith(".rs")) files.push(path);
  }
  return files;
}

function scanRequestBuilderStrings(repositoryRoot, path, violations) {
  const file = relative(repositoryRoot, path).replaceAll("\\", "/");
  // Defense in depth: rustFiles() already prunes any directory literally
  // named "tests", but re-check the relative path so a future traversal
  // change can't silently start pulling integration-test files back in.
  if (file.split("/").includes("tests")) return;

  const source = readFileSync(path, "utf8");
  // Scan all three Rust string forms a request builder could use:
  // hash-delimited raw strings (r#"..."#), zero-hash raw strings (r"..."),
  // and ordinary escaped strings ("..."). Comments (//, /* */, nested) are
  // skipped so commented-out hazards don't trip the gate.
  //
  // Test-quarantine strategy: strings are skipped when they fall inside a
  // #[cfg(test)] *module* body (specifically `#[cfg(test)] mod name { ... }`,
  // tracked via brace-depth), inside a file under a tests/ directory
  // (integration tests), or in a file reached only through a test-only
  // `mod name;` declaration (testOnlyFiles(), by its #[cfg] attribute and never
  // by the file's name: #837). This is deliberate, not incidental: unit tests such
  // as `exact_report_collection_is_shared_by_count_and_rows`, which live
  // inside `#[cfg(test)] mod tests { ... }`, assert against string literals
  // containing `<REPORT NAME="...">` or `$$NumItems:... With Spaces` as
  // *expectations*, not as a dispatched request. Those are not request
  // builders and must not be scanned.
  //
  // Deliberately narrower than "skip anything under #[cfg(test)]": a bare
  // #[cfg(test)] fn (not inside a mod) is still scanned, because this
  // repository pins exactly that shape as a real hazard --
  // `legacy_company_list_request` in tdl_engine.rs is a top-level
  // `#[cfg(test)] fn` whose body *is* the rendered request, kept only to
  // assert byte-parity with the production renderer. Excluding all
  // #[cfg(test)] items would silently drop it from the pinned set.
  //
  // What this still cannot catch:
  //  - A hazard assembled at runtime via string concatenation across
  //    multiple literals (no single literal contains the full pattern). A
  //    format placeholder inside an unquoted `$$Fn:` argument or a
  //    `<REPORT NAME="...">` is reported as `unresolved-argument` (#837),
  //    since the literal cannot show what the request sends there.
  //  - A hazard placed in a bare #[cfg(test)] fn/const/impl (not a `mod`)
  //    that is purely a test fixture, not a request builder -- it will
  //    still be scanned and, if it happens to contain hazard-shaped text,
  //    flagged. That is a false-positive risk, not a missed-hazard one.
  //  - A hazard built from a `const`/`static` marked #[cfg(test)] without a
  //    brace-delimited body (e.g. `#[cfg(test)] const X: &str = "...";`) --
  //    scanned the same way, same false-positive-only risk.
  //  - A zero-hash raw string (r"...") can never itself contain a `"`
  //    character (Rust raw strings have no escape mechanism at all), so it
  //    can carry a function-argument-with-space hazard but never the
  //    quote-bearing custom-report `<REPORT NAME="...">` hazard -- that is a
  //    fact about Rust's grammar, not a gap in this scanner.
  for (const literal of scanStrings(source)) {
    if (literal.insideTest) continue;
    const identifier = enclosingFunction(source, literal.start);
    for (const match of literal.value.matchAll(/\$\$[A-Za-z_][A-Za-z0-9_]*:/g)) {
      const afterColon = match.index + match[0].length;
      // A TDL function argument that opens with a double quote is a quoted
      // span: the real argument is whatever sits between that quote and the
      // next one, and only whitespace *inside* the quotes is a hazard.
      // Text after the closing quote (e.g. the rest of a larger XML-escaped
      // expression like `AND $Date &lt;= $$Date:"..."`) is not part of this
      // argument at all, so it must not be swallowed into the check --
      // that was the false-positive source this quote-aware branch fixes.
      // An unquoted argument keeps the pre-existing behaviour exactly: it
      // runs to the next real `<` or newline, and any whitespace anywhere
      // in that span is a hazard.
      if (literal.value[afterColon] === '"') {
        const closeQuote = literal.value.indexOf('"', afterColon + 1);
        if (closeQuote !== -1) {
          const inner = literal.value.slice(afterColon + 1, closeQuote);
          if (/\s/.test(inner)) {
            const expression = `${match[0]}${literal.value.slice(afterColon, closeQuote + 1)}`.trim();
            violations.add(`function-argument-with-space|${file}::${identifier}|${expression}`);
          }
          continue;
        }
        // No closing quote found -- fall through to the unquoted scan below
        // so a malformed literal is still checked rather than silently
        // skipped.
      }
      const rest = literal.value.slice(afterColon);
      const unquoted = /^([^<\r\n]*)/.exec(rest)[1];
      // A format placeholder in an unquoted argument is filled at run time, so
      // the literal cannot show whether the argument the request sends holds
      // a space (#837): it is reported until a reviewed exact-set entry pins it.
      if (hasPlaceholder(unquoted)) {
        violations.add(`unresolved-argument|${file}::${identifier}|${`${match[0]}${unquoted}`.trim()}`);
      }
      if (/\s/.test(unquoted)) {
        const expression = `${match[0]}${unquoted}`.trim();
        violations.add(`function-argument-with-space|${file}::${identifier}|${expression}`);
      }
    }
    for (const match of literal.value.matchAll(/<REPORT\s+NAME="([^"]+)"/g)) {
      // A report named by a placeholder is a custom report whose name the
      // literal does not show (#837).
      if (hasPlaceholder(match[1])) {
        violations.add(`unresolved-argument|${file}::${identifier}|<REPORT NAME="${match[1]}">`);
      } else {
        violations.add(`custom-report|${file}::${identifier}|${match[1]}`);
      }
    }
    // A report FIELD that SETs an amount-valued method without declaring
    // <TYPE>Amount</TYPE> returns Tally's display text, with the sign dropped
    // and digits grouped (protocol reference §6.3). The pinned set for this
    // kind is empty: every amount FIELD must carry its TYPE.
    for (const field of literal.value.matchAll(/<FIELD\b([^>]*)>([\s\S]*?)<\/FIELD>/gi)) {
      const sets = [...field[2].matchAll(/<SET>([\s\S]*?)<\/SET>/gi)].map((set) => set[1]);
      // Every method and function in every SET counts, so an amount in a
      // compound expression (`$Quantity + $OpeningBalance`), inside a `$$`
      // function's argument (`$$Abs:$ClosingBalance`) or at the end of a
      // sub-object path (`$LedgerEntries[1].Amount`, read as `Amount`, with
      // one level of brackets inside an index) is caught too.
      const index = /\[(?:[^[\]]|\[[^[\]]*\])*\]/g;
      const methods = sets.flatMap((set) =>
        [...set.matchAll(/\$\$?[A-Za-z_][A-Za-z0-9_]*(?:\[(?:[^[\]]|\[[^[\]]*\])*\]|\.[A-Za-z_][A-Za-z0-9_]*)*/g)].map(
          (reference) => reference[0].replace(index, "").split(".").pop().replace(/^\$+/, ""),
        ),
      );
      // A formula reference (`@Name`, `@@Name`) hides what it evaluates, so a
      // SET holding one fails closed unless the FIELD declares a TYPE.
      const opaque = sets.some((set) => /@@?[A-Za-z_]/.test(set));
      // A TYPE counts only outside every SET and comment, so TYPE-shaped text
      // inside an expression or a comment cannot pass for the declaration.
      const declarations = field[2].replace(/<SET>[\s\S]*?<\/SET>|<!--[\s\S]*?-->/gi, "");
      const typed = /<TYPE>[\s\S]*?<\/TYPE>/i.test(declarations);
      const amountTyped = /<TYPE>\s*Amount\s*<\/TYPE>/i.test(declarations);
      const needsAmountType = methods.some((method) => amountMethod.test(method));
      if (needsAmountType ? amountTyped : !opaque || typed) continue;
      const name = /NAME="([^"]*)"/.exec(field[1])?.[1] ?? "<unnamed>";
      violations.add(`amount-field-without-type|${file}::${identifier}|${name}`);
    }
  }
}

// Single forward pass over the source that recognises (and skips) line and
// block comments, char literals, and #[cfg(test)] *module* bodies, while
// collecting every raw (r#"..."#, r"...") and ordinary ("...", escape-aware)
// string literal found in "production" code.
function scanStrings(source) {
  const strings = [];
  const n = source.length;
  const cfgTestAttribute = /^#\[\s*cfg\s*\(\s*test\s*\)\s*\]/;
  const charLiteral = /^'(?:\\(?:['"\\nrt0]|x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]{1,6}\})|[^'\\\n])'/;
  // Only a #[cfg(test)] attribute immediately (modulo whitespace and other
  // attributes) followed by `mod name {` opens a quarantined test region --
  // see the "Deliberately narrower" note in scanRequestBuilderStrings().
  const cfgTestModuleAhead = /^(?:(?:pub(?:\([^)]*\))?\s+)?mod\s+[A-Za-z_][A-Za-z0-9_]*\s*\{)/;

  function nextItemIsTestModule(position) {
    let cursor = position;
    for (;;) {
      while (cursor < n && /\s/.test(source[cursor])) cursor += 1;
      if (source[cursor] === "#" && source[cursor + 1] === "[") {
        let depth = 0;
        let j = cursor + 1;
        while (j < n) {
          if (source[j] === "[") depth += 1;
          else if (source[j] === "]") {
            depth -= 1;
            j += 1;
            if (depth === 0) break;
            continue;
          }
          j += 1;
        }
        cursor = j;
        continue;
      }
      break;
    }
    return cfgTestModuleAhead.test(source.slice(cursor, cursor + 200));
  }

  let i = 0;
  let braceDepth = 0;
  let pendingCfgTest = false;
  const testStack = []; // brace depths at which a #[cfg(test)] item body opened

  while (i < n) {
    const two = source.slice(i, i + 2);

    if (two === "//") {
      const end = source.indexOf("\n", i);
      i = end === -1 ? n : end;
      continue;
    }

    if (two === "/*") {
      let depth = 1;
      i += 2;
      while (i < n && depth > 0) {
        const pair = source.slice(i, i + 2);
        if (pair === "/*") {
          depth += 1;
          i += 2;
        } else if (pair === "*/") {
          depth -= 1;
          i += 2;
        } else {
          i += 1;
        }
      }
      continue;
    }

    const cfgMatch = cfgTestAttribute.exec(source.slice(i, i + 64));
    if (cfgMatch) {
      const afterAttribute = i + cfgMatch[0].length;
      if (nextItemIsTestModule(afterAttribute)) pendingCfgTest = true;
      i = afterAttribute;
      continue;
    }

    const ch = source[i];

    if (ch === "'") {
      const charMatch = charLiteral.exec(source.slice(i, i + 10));
      if (charMatch) {
        i += charMatch[0].length;
        continue;
      }
      // Not a char literal (e.g. a lifetime like 'a) -- fall through as a
      // plain character so lifetimes never trip the raw/ordinary parsers.
      i += 1;
      continue;
    }

    if (ch === "r" && (source[i + 1] === '"' || source[i + 1] === "#")) {
      let cursor = i + 1;
      let hashCount = 0;
      while (source[cursor] === "#") {
        hashCount += 1;
        cursor += 1;
      }
      if (source[cursor] === '"') {
        const hashes = "#".repeat(hashCount);
        const endMarker = `"${hashes}`;
        const valueStart = cursor + 1;
        const end = source.indexOf(endMarker, valueStart);
        if (end === -1) throw new Error(`unterminated Rust raw string at offset ${i}`);
        strings.push({
          start: i,
          value: source.slice(valueStart, end),
          insideTest: testStack.length > 0,
        });
        i = end + endMarker.length;
        continue;
      }
      // Looked like a raw-string prefix but wasn't (e.g. an identifier
      // starting with "r"); treat the "r" as an ordinary character.
    }

    if (ch === '"') {
      let cursor = i + 1;
      let value = "";
      let terminated = false;
      while (cursor < n) {
        const c = source[cursor];
        if (c === "\\" && cursor + 1 < n) {
          // Escape-aware: \" does not end the string, and \\ consumes only
          // the escaped backslash, so a following quote (as in \\") is a
          // real, unescaped terminator.
          const next = source[cursor + 1];
          if (next === '"') value += '"';
          else if (next === "\\") value += "\\";
          else value += next;
          cursor += 2;
          continue;
        }
        if (c === '"') {
          cursor += 1;
          terminated = true;
          break;
        }
        value += c;
        cursor += 1;
      }
      if (!terminated) throw new Error(`unterminated Rust string literal at offset ${i}`);
      strings.push({ start: i, value, insideTest: testStack.length > 0 });
      i = cursor;
      continue;
    }

    if (ch === "{") {
      braceDepth += 1;
      if (pendingCfgTest) {
        testStack.push(braceDepth);
        pendingCfgTest = false;
      }
      i += 1;
      continue;
    }

    if (ch === "}") {
      if (testStack.length && testStack[testStack.length - 1] === braceDepth) {
        testStack.pop();
      }
      braceDepth = Math.max(0, braceDepth - 1);
      i += 1;
      continue;
    }

    if (ch === ";" || ch === ",") {
      // The pending #[cfg(test)] attribute applied to an item with no
      // brace-delimited body (a `use`/`const`/struct field/...); there is
      // nothing to push onto testStack, so just stop tracking it rather
      // than letting it leak onto an unrelated later brace.
      pendingCfgTest = false;
      i += 1;
      continue;
    }

    i += 1;
  }

  return strings;
}

function enclosingFunction(source, position) {
  const prefix = source.slice(0, position);
  const functions = [...prefix.matchAll(/(?:pub(?:\([^)]*\))?\s+)?fn\s+([A-Za-z0-9_]+)/g)];
  return functions.at(-1)?.[1] ?? "<module>";
}

// Whether `attributes` hold a `#[cfg(...)]` whose predicate holds only under
// test: `test`, an `all(...)` with any such argument, or an `any(...)` whose
// every argument is one. `any(test, feature = "...")` is not test-only.
function testOnlyCfg(attributes) {
  const args = (inner) => {
    const parts = [];
    let depth = 0;
    let start = 0;
    for (let k = 0; k < inner.length; k += 1) {
      if (inner[k] === "(") depth += 1;
      else if (inner[k] === ")") depth -= 1;
      else if (inner[k] === "," && depth === 0) {
        parts.push(inner.slice(start, k));
        start = k + 1;
      }
    }
    parts.push(inner.slice(start));
    return parts.map((part) => part.trim()).filter(Boolean);
  };
  const implies = (predicate) => {
    if (predicate === "test") return true;
    const call = /^(all|any)\s*\(([\s\S]*)\)$/.exec(predicate);
    if (!call) return false;
    const inner = args(call[2]);
    return call[1] === "all" ? inner.some(implies) : inner.length > 0 && inner.every(implies);
  };
  for (const match of attributes.matchAll(/#\[\s*cfg\s*\(/g)) {
    let depth = 1;
    let k = match.index + match[0].length;
    const start = k;
    while (k < attributes.length && depth > 0) {
      if (attributes[k] === "(") depth += 1;
      else if (attributes[k] === ")") depth -= 1;
      k += 1;
    }
    if (implies(attributes.slice(start, k - 1).trim())) return true;
  }
  return false;
}

// Whether `text` holds a Rust format placeholder (`{}`, `{name}`, `{0:?}`). An
// escaped brace (`{{`, `}}`) is a literal brace, not a placeholder.
function hasPlaceholder(text) {
  return /\{[^{}]*\}/.test(text.replaceAll("{{", "").replaceAll("}}", ""));
}

// `source` with every comment and string or character literal blanked to
// spaces (quotes and newlines kept), so offsets still match the source.
function blankLiterals(source) {
  const out = source.split("");
  const blank = (from, to) => {
    for (let k = from; k < to; k += 1) if (out[k] !== "\n") out[k] = " ";
  };
  const charLiteral = /^'(?:\\(?:['"\\nrt0]|x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]{1,6}\})|[^'\\\n])'/;
  let i = 0;
  while (i < source.length) {
    const two = source.slice(i, i + 2);
    if (two === "//") {
      const end = source.indexOf("\n", i);
      const stop = end === -1 ? source.length : end;
      blank(i, stop);
      i = stop;
    } else if (two === "/*") {
      let depth = 0;
      let j = i;
      while (j < source.length) {
        if (source.startsWith("/*", j)) {
          depth += 1;
          j += 2;
        } else if (source.startsWith("*/", j)) {
          depth -= 1;
          j += 2;
          if (depth === 0) break;
        } else j += 1;
      }
      blank(i, j);
      i = j;
    } else if (source[i] === "r" && /^r#*"/.test(source.slice(i, i + 260)) && !/[A-Za-z0-9_]/.test(source[i - 1] ?? "")) {
      const hashes = /^r(#*)"/.exec(source.slice(i, i + 260))[1];
      const valueStart = i + hashes.length + 2;
      const end = source.indexOf(`"${hashes}`, valueStart);
      const stop = end === -1 ? source.length : end;
      blank(valueStart, stop);
      i = stop + 1 + hashes.length;
    } else if (source[i] === '"') {
      let j = i + 1;
      while (j < source.length && source[j] !== '"') j += source[j] === "\\" ? 2 : 1;
      blank(i + 1, j);
      i = j + 1;
    } else if (source[i] === "'") {
      const literal = charLiteral.exec(source.slice(i, i + 12));
      if (literal) blank(i, i + literal[0].length);
      i += literal ? literal[0].length : 1;
    } else i += 1;
  }
  return out.join("");
}

// The span of the brace-delimited body that opens at `open` in `masked`.
function bodyEnd(masked, open) {
  let depth = 0;
  for (let k = open; k < masked.length; k += 1) {
    if (masked[k] === "{") depth += 1;
    else if (masked[k] === "}" && (depth -= 1) === 0) return k;
  }
  return masked.length;
}

// Every `mod name;` declaration of a file: its name, its `#[path]`, the inline
// modules it sits in (outermost first), and whether it is test-only, by a
// #[cfg(test)] attribute of its own or of an inline module around it.
function moduleDeclarations(source) {
  const masked = blankLiterals(source);
  const inline = [...masked.matchAll(new RegExp(`${ATTRIBUTES}\\{`, "g"))].map((m) => {
    const open = m.index + m[0].length - 1;
    return { name: m[2], start: open, end: bodyEnd(masked, open), test: testOnlyCfg(m[1]) };
  });
  return [...masked.matchAll(new RegExp(`${ATTRIBUTES};`, "g"))].map((m) => {
    const around = inline.filter((block) => block.start < m.index && m.index < block.end);
    const path = /#\[\s*path\s*=\s*"([^"]*)"\s*\]/.exec(source.slice(m.index, m.index + m[1].length))?.[1];
    return {
      name: m[2],
      path,
      chain: around.map((block) => block.name),
      test: testOnlyCfg(m[1]) || around.some((block) => block.test),
    };
  });
}

// The file a declaration in `file` names, by Rust's module path rules, or
// undefined when none exists.
function moduleFile(file, declaration) {
  const modRs = ["mod.rs", "lib.rs", "main.rs"].includes(basename(file));
  const own = modRs ? dirname(file) : join(dirname(file), basename(file, ".rs"));
  if (declaration.path !== undefined) {
    const base = declaration.chain.length ? join(own, ...declaration.chain) : dirname(file);
    const target = resolve(base, declaration.path);
    return existsSync(target) ? target : undefined;
  }
  const base = join(own, ...declaration.chain);
  return [join(base, `${declaration.name}.rs`), join(base, declaration.name, "mod.rs")].find(existsSync);
}

// Every file reached only through a test-only module declaration, and every
// file such a file declares in turn. A file that production code also reaches
// through an ordinary `mod` is scanned, whatever else declares it (#837): a
// gate that fails closed never lets a shared file go unscanned.
function testOnlyFiles(files) {
  const declared = new Map(files.map((file) => [file, moduleDeclarations(readFileSync(file, "utf8"))]));
  const quarantined = new Set();
  const pending = [];
  for (const [file, declarations] of declared) {
    for (const declaration of declarations.filter((d) => d.test)) {
      const target = moduleFile(file, declaration);
      if (target && !quarantined.has(target)) {
        quarantined.add(target);
        pending.push(target);
      }
    }
  }
  while (pending.length) {
    const file = pending.pop();
    const declarations = declared.get(file) ?? moduleDeclarations(readFileSync(file, "utf8"));
    for (const declaration of declarations) {
      const target = moduleFile(file, declaration);
      if (target && !quarantined.has(target)) {
        quarantined.add(target);
        pending.push(target);
      }
    }
  }
  const production = files.filter((file) => !quarantined.has(file));
  while (production.length) {
    const file = production.pop();
    const declarations = declared.get(file) ?? moduleDeclarations(readFileSync(file, "utf8"));
    for (const declaration of declarations.filter((d) => !d.test)) {
      const target = moduleFile(file, declaration);
      if (target && quarantined.delete(target)) production.push(target);
    }
  }
  return quarantined;
}
