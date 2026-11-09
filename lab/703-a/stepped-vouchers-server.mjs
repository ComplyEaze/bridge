#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0
//
// A DEV-ONLY stdio MCP server for lab row 703-A (#703 shape A). It never ships: it is not bundled by Tauri, not
// part of the `bridge` crate, and opens no socket. It reads one file, the tools.json beside it, and writes a
// timestamped event log. It sends nothing to Tally and holds no book: every row it returns is invented here.
//
// Run: node stepped-vouchers-server.mjs --tools <tools.json> [--log <file>] [--step-scale <0.01 to 1>]
//
// It serves Bridge's own text, taken from source by extract_text.py into tools.json: the server instructions,
// and the `list_companies` and `vouchers` tools with their shipped descriptions plus the build's planned stepping
// sentences. The question is whether a model, given that text and a real task, carries the server-issued read_id
// from call to call until the read is done.
//
// `list_companies` answers one invented company. `vouchers` is a stepped read: a call with no read_id mints one
// and runs step 1; each call carrying it runs the next step; every step but the last answers done false with no
// rows; the last answers done true with the rows. The plan is chosen by the window: up to 10 days is FIVE, longer
// is TWELVE. Each plan's steps take the seconds listed (some 50 and 55 s, to exercise the host's margin), times
// --step-scale (a CLI pre-screen runs at a small scale; the lab row at 1). The server holds the plan, never the
// caller. In band, as Bridge refuses: an id this process did not issue (read_unknown), a finished one
// (read_finished), one idle past READ_IDLE_SECONDS (read_expired), one sent with different other arguments
// (read_arguments_changed), and a company_guid not served (company_not_open). A cancelled step does not count, so
// the next call with that id runs it again. One call runs at a time; a second is refused in band (busy). At most
// MAX_LIVE_READS unfinished reads are held; a new one past that drops the least recently used. Every log line
// carries the pid, because a host may start a new server process between calls, and every received message is
// logged whole (initialize capabilities and any `_meta` included), up to MAX_LOGGED_MESSAGE_CHARS.

import { randomBytes } from "node:crypto";
import { appendFileSync, existsSync, readFileSync, realpathSync } from "node:fs";
import { createInterface } from "node:readline";
import { fileURLToPath } from "node:url";

export const PROTOCOL_VERSION = "2025-06-18";
export const MAX_LOGGED_MESSAGE_CHARS = 16384;
export const READ_IDLE_SECONDS = 900;
export const MAX_LIVE_READS = 4;
export const FIVE_DAYS_AT_MOST = 10;
export const PLANS = {
  five: [40, 50, 40, 55, 30],
  twelve: [40, 40, 50, 40, 40, 55, 40, 40, 50, 40, 55, 30],
};
const USAGE = "usage: node stepped-vouchers-server.mjs --tools <tools.json> [--log <file>] [--step-scale <0.01 to 1>]";

export function parseArgs(argv) {
  const args = { tools: null, log: null, stepScale: 1 };
  for (let i = 0; i < argv.length; i += 1) {
    const value = argv[i + 1];
    if (value === undefined || value.startsWith("--")) throw new Error(USAGE);
    if (argv[i] === "--tools") args.tools = value;
    else if (argv[i] === "--log") args.log = value;
    else if (argv[i] === "--step-scale") args.stepScale = Number(value);
    else throw new Error(USAGE);
    i += 1;
  }
  if (!args.tools || !Number.isFinite(args.stepScale) || args.stepScale < 0.01 || args.stepScale > 1) throw new Error(USAGE);
  return args;
}

// Bridge's serde_json keeps no insertion order, so every object it sends has its keys sorted; so do these.
export function sortKeys(value) {
  if (Array.isArray(value)) return value.map(sortKeys);
  if (value === null || typeof value !== "object") return value;
  return Object.fromEntries(Object.keys(value).sort().map((key) => [key, sortKeys(value[key])]));
}

// Strips the kit's own `planned` markers, which the model must not see.
export function loadTools(text) {
  const document = JSON.parse(text);
  const tools = document.tools.map((tool) => sortKeys(JSON.parse(JSON.stringify(tool, (key, value) => (key === "planned" ? undefined : value)))));
  if (tools.map((tool) => tool.name).join(",") !== "list_companies,vouchers") throw new Error("tools.json must hold list_companies and vouchers");
  if (typeof document.planned?.next_step !== "string") throw new Error("tools.json must hold planned.next_step");
  return { instructions: document.instructions, tools, nextStep: document.planned.next_step, sourceCommit: document.source_commit };
}

// Letters only, so no id, name or code in a pushed log reads as a number of any kind.
const LETTERS = "abcdefghjkmnpqrstuvwxyz";
const letters = (count) => [...randomBytes(count)].map((b) => LETTERS[b % LETTERS.length]).join("");
const mintReadId = () => `rd${letters(14)}`;
const mintCode = () => letters(5).toUpperCase();

// YYYYMMDD or YYYY-MM-DD, a real calendar day, as Bridge's schema admits; returns a UTC Date or null.
export function parseDay(value) {
  const match = typeof value === "string" && /^([0-9]{4})-?([0-9]{2})-?([0-9]{2})$/.exec(value);
  if (!match) return null;
  const day = new Date(Date.UTC(Number(match[1]), Number(match[2]) - 1, Number(match[3])));
  return day.getUTCMonth() === Number(match[2]) - 1 && day.getUTCDate() === Number(match[3]) ? day : null;
}

const iso = (day) => day.toISOString().slice(0, 10);

// Validates a vouchers call against the served schema; returns { readId, key, from, to } or a refusal code.
export function readVoucherArguments(input, schema) {
  if (!input || typeof input !== "object" || Array.isArray(input)) return "argument_invalid:arguments";
  for (const name of Object.keys(input)) if (!Object.hasOwn(schema.properties, name)) return `argument_unknown:${name}`;
  for (const name of schema.required) if (input[name] === undefined) return `argument_missing:${name}`;
  for (const [name, value] of Object.entries(input)) {
    const rule = schema.properties[name];
    const bad =
      (rule.type === "string" && (typeof value !== "string" || value.length < (rule.minLength ?? 0) || value.length > (rule.maxLength ?? Infinity) || (rule.pattern !== undefined && !new RegExp(rule.pattern, "u").test(value)))) ||
      (rule.type === "integer" && (typeof value !== "number" || !Number.isInteger(value) || value < (rule.minimum ?? -Infinity))) ||
      (rule.type === "boolean" && typeof value !== "boolean") ||
      (rule.enum !== undefined && !rule.enum.includes(value));
    if (bad) return `argument_invalid:${name}`;
  }
  const from = parseDay(input.from);
  const to = parseDay(input.to);
  if (!from) return "invalid_from";
  if (!to) return "invalid_to";
  if (from > to) return "from_after_to";
  // The read's other arguments, with a schema default standing for an argument left out, so `offset: 0` on a later
  // call and no offset on the first are the same read.
  const key = JSON.stringify(Object.keys(schema.properties).filter((name) => name !== "read_id").sort().map((name) => input[name] ?? schema.properties[name].default ?? null));
  return { readId: input.read_id ?? null, key, from, to, guid: input.company_guid };
}

export function createServer({ send, log, tools, stepScale = 1, idleSeconds = READ_IDLE_SECONDS, now = () => performance.now(), mint = mintReadId, code = mintCode }) {
  const company = { name: "Lab Stepped Traders", guid: `${letters(8)}-${letters(4)}-${letters(4)}-${letters(4)}-${letters(12)}`, books_from: "2026-04-01" };
  const vouchersSchema = tools.tools[1].inputSchema;
  const calls = new Map(); // request id (as JSON) -> { timer, started }
  const reads = new Map(); // read id -> { plan, seconds, done, key, code, finished, used, from, to }
  const id = (value) => JSON.stringify(value);
  log({ event: "company", guid: company.guid, source_commit: tools.sourceCommit, step_scale: stepScale });

  const companyJson = () => ({ name: company.name, guid: company.guid, company_number: "lab", books_from: company.books_from, identity_state: "verified_tuple", missing_field: null });
  function result(requestId, value) {
    send({ jsonrpc: "2.0", id: requestId, result: value });
  }
  function answer(requestId, unsorted) {
    const payload = sortKeys(unsorted);
    result(requestId, { content: [{ type: "text", text: JSON.stringify(payload) }], structuredContent: payload, isError: false });
  }
  function refuse(requestId, tool, code, readId = null) {
    result(requestId, {
      content: [{ type: "text", text: `${tool}: refused\n${code}` }],
      structuredContent: { error: { code, message: "ComplyEaze Bridge refused this operation." } },
      isError: true,
    });
    log({ event: "refused", id: requestId, tool, code, read_id: readId });
  }
  function error(requestId, code, message) {
    send({ jsonrpc: "2.0", id: requestId, error: { code, message } });
    log({ event: "error_sent", id: requestId, code, message });
  }

  function rows(read) {
    const party = `Lab Party ${read.code}`;
    const day = (offset) => iso(new Date(Math.min(read.from.getTime() + offset * 86400000, read.to.getTime())));
    return [
      { date: day(0), voucher_type: "Sales", number: "LAB-A", party, narration: "Lab sale", entries: [{ ledger: party, amount: "-1250.00" }, { ledger: "Lab Sales", amount: "1250.00" }] },
      { date: day(1), voucher_type: "Receipt", number: "LAB-B", party, narration: "Lab receipt", entries: [{ ledger: "Lab Bank", amount: "-1250.00" }, { ledger: party, amount: "1250.00" }] },
      { date: day(2), voucher_type: "Payment", number: "LAB-C", party: "Lab Rent", narration: "Lab rent", entries: [{ ledger: "Lab Rent", amount: "-400.00" }, { ledger: "Lab Bank", amount: "400.00" }] },
    ];
  }

  function vouchersCall(requestId, args) {
    const parsed = readVoucherArguments(args, vouchersSchema);
    if (typeof parsed === "string") return refuse(requestId, "vouchers", parsed);
    if (parsed.guid !== company.guid) return refuse(requestId, "vouchers", "company_not_open", parsed.readId);
    let readId = parsed.readId;
    let read;
    if (readId === null) {
      const live = [...reads].filter(([, r]) => !r.finished).sort(([, a], [, b]) => a.used - b.used);
      if (live.length >= MAX_LIVE_READS) {
        reads.delete(live[0][0]);
        log({ event: "read_evicted", read_id: live[0][0] });
      }
      const span = Math.round((parsed.to - parsed.from) / 86400000) + 1;
      const plan = span <= FIVE_DAYS_AT_MOST ? "five" : "twelve";
      readId = mint();
      read = { plan, seconds: PLANS[plan], done: 0, key: parsed.key, code: code(), finished: false, used: now(), from: parsed.from, to: parsed.to };
      reads.set(readId, read);
      log({ event: "read_started", id: requestId, read_id: readId, plan, of: read.seconds.length, span_days: span, code: read.code });
    } else {
      read = reads.get(readId);
      if (!read) return refuse(requestId, "vouchers", "read_unknown", readId);
      if (read.finished) return refuse(requestId, "vouchers", "read_finished", readId);
      if ((now() - read.used) / 1000 > idleSeconds) {
        reads.delete(readId);
        return refuse(requestId, "vouchers", "read_expired", readId);
      }
      if (parsed.key !== read.key) return refuse(requestId, "vouchers", "read_arguments_changed", readId);
      read.used = now();
    }
    const seconds = read.seconds[read.done] * stepScale;
    const started = performance.now();
    log({ event: "step_started", id: requestId, read_id: readId, step: read.done + 1, of: read.seconds.length, seconds });
    const timer = setTimeout(() => {
      calls.delete(id(requestId));
      read.done += 1;
      read.used = now();
      read.finished = read.done >= read.seconds.length;
      const elapsed = (performance.now() - started) / 1000;
      const window = { from: iso(read.from), to: iso(read.to) };
      const progress = { read_id: readId, step: read.done, steps: read.seconds.length, done: read.finished };
      const payload = read.finished
        ? { company: companyJson(), result: { state: "observed", ...window, ...progress, vouchers: rows(read), offset: 0, next_offset: null, truncated: false } }
        : { company: companyJson(), result: { state: "in_progress", ...window, ...progress, next_step: tools.nextStep } };
      answer(requestId, payload);
      log({ event: "step_result", id: requestId, read_id: readId, step: read.done, of: read.seconds.length, done: read.finished, elapsed });
    }, seconds * 1000);
    calls.set(id(requestId), { timer, started });
  }

  function handle(message) {
    const text = JSON.stringify(message) ?? "";
    log(text.length <= MAX_LOGGED_MESSAGE_CHARS ? { event: "received", message } : { event: "received", truncated: true, length: text.length, head: text.slice(0, 1024) });
    if (!message || typeof message !== "object" || Array.isArray(message) || message.jsonrpc !== "2.0") return error(message?.id ?? null, -32600, "invalid request");
    const { id: requestId, method, params } = message;
    if (method === undefined && ("result" in message || "error" in message)) return; // a response: this server sends no requests
    if (typeof method !== "string") return error(requestId ?? null, -32600, "invalid request");
    if (method === "notifications/cancelled") {
      const call = calls.get(id(params?.requestId));
      if (!call) return log({ event: "cancel_ignored", id: params?.requestId ?? null });
      clearTimeout(call.timer);
      calls.delete(id(params.requestId));
      log({ event: "cancelled", id: params.requestId, elapsed: (performance.now() - call.started) / 1000, reason: params?.reason ?? null });
      return;
    }
    if (requestId === undefined) return; // other notifications need no answer
    if (method === "initialize") {
      return result(requestId, {
        protocolVersion: typeof params?.protocolVersion === "string" ? params.protocolVersion : PROTOCOL_VERSION,
        capabilities: { tools: {} },
        serverInfo: { name: "bridge-lab-703a", version: "0.0.0-dev" },
        instructions: tools.instructions,
      });
    }
    if (method === "ping") return result(requestId, {});
    if (method === "tools/list") return result(requestId, { tools: tools.tools });
    if (method !== "tools/call") return error(requestId, -32601, `method not found: ${method}`);
    const name = params?.name;
    if (name !== "list_companies" && name !== "vouchers") return error(requestId, -32602, `unknown tool ${name}`);
    if (calls.size > 0) return refuse(requestId, name, "busy");
    if (name === "list_companies") {
      const args = params.arguments ?? {};
      if (typeof args !== "object" || Array.isArray(args) || Object.keys(args).length > 0) return refuse(requestId, name, "argument_unknown");
      answer(requestId, { companies: [companyJson()], open_count: 1 });
      return log({ event: "companies_sent", id: requestId });
    }
    return vouchersCall(requestId, params.arguments ?? {});
  }

  function handleLine(line) {
    if (!line.trim()) return;
    let message;
    try {
      message = JSON.parse(line);
    } catch {
      log({ event: "parse_error", length: line.length });
      return send({ jsonrpc: "2.0", id: null, error: { code: -32700, message: "parse error" } });
    }
    handle(message);
  }

  return { handleLine, company, running: () => calls.size, liveReads: () => [...reads.values()].filter((r) => !r.finished).length };
}

function main() {
  let args;
  let tools;
  try {
    args = parseArgs(process.argv.slice(2));
    tools = loadTools(readFileSync(args.tools, "utf-8"));
  } catch (cause) {
    process.stderr.write(`${cause.message}\n`);
    process.exit(2);
  }
  const log = (event) => {
    const line = `${JSON.stringify({ time: new Date().toISOString(), pid: process.pid, ...event })}\n`;
    if (args.log) appendFileSync(args.log, line);
    else process.stderr.write(line);
  };
  try {
    log({ event: "started", pid: process.pid });
  } catch (cause) {
    process.stderr.write(`cannot write the log ${args.log}: ${cause.message}\n`);
    process.exit(2);
  }
  const server = createServer({ send: (message) => process.stdout.write(`${JSON.stringify(message)}\n`), log, tools, stepScale: args.stepScale });
  const input = createInterface({ input: process.stdin });
  input.on("line", (line) => server.handleLine(line));
  input.on("close", () => {
    log({ event: "stdin_closed", running: server.running() });
    process.exit(0);
  });
  for (const signal of ["SIGTERM", "SIGINT", "SIGHUP"]) {
    process.on(signal, () => {
      log({ event: "signal", signal, running: server.running() });
      process.exit(0);
    });
  }
  process.stdout.on("error", (cause) => {
    log({ event: "stdout_error", code: cause.code ?? null, running: server.running() });
    process.exit(0);
  });
}

if (process.argv[1] && existsSync(process.argv[1]) && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url))) {
  main();
}
