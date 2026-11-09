#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0
//
// A DEV-ONLY stdio MCP server with two tools, `sleep` and `stepped_read`, for measuring MCP hosts (#703): the
// longest call a host waits for, what it shows when it gives up, whether it sends
// `notifications/cancelled`, and whether `notifications/progress` resets its timer. It never ships:
// it is not bundled by Tauri, not part of the `bridge` crate, and not among Bridge's registered tools.
// It reads no file and opens no socket; it only answers on stdout and writes a timestamped event log.
//
// Run: node scripts/mcp-sleep-server.mjs [--log <file>]
// Each line of the log is one JSON event with an ISO time: every message received, every progress
// notification sent, every result sent, and every cancellation, so a measurement reads what arrived
// when instead of trusting what the host showed. Without --log the events go to stderr.
//
// The tool: `sleep` with `seconds` (0 to 900) and `progress_every` (seconds, optional). With
// `progress_every` set and a `progressToken` in the call's `_meta`, it sends
// `notifications/progress` at that interval while it sleeps (not faster than every 0.1 s, which bounds
// the log of a 900 s call to 9,000 progress lines). One call runs at a time: a second is refused in-band.
// A `notifications/cancelled` naming the call stops it and sends no result, as the protocol asks.
//
// The second tool, `stepped_read` (#703 shape A, lab row 703-A): a read that takes STEPS calls of
// STEP_SECONDS each, to measure whether a host's model carries a server-issued handle from call to call.
// The first call (no `read_id`) mints an id and returns it with `done: false`; each later call carrying
// that id runs the next step; the last returns `done: true` and the answer. No step but the last carries
// the answer. The server holds the plan, never the caller: an id this process did not issue, an id whose
// read is finished, and an id idle past READ_IDLE_SECONDS are each refused in band with their own code.
// A cancelled step does not count, so the next call with that id runs it again. At most MAX_LIVE_READS
// unfinished reads are held; a new one past that drops the least recently used. Every log line carries
// the server's pid, because a host may start a new server process between two calls.

import { randomBytes } from "node:crypto";
import { appendFileSync, existsSync, realpathSync } from "node:fs";
import { createInterface } from "node:readline";
import { fileURLToPath } from "node:url";

export const PROTOCOL_VERSION = "2025-06-18";
export const MAX_SECONDS = 900;
export const MIN_PROGRESS_EVERY = 0.1;
export const MAX_LOGGED_MESSAGE_CHARS = 8192;
export const STEP_SECONDS = 40;
export const STEPS = 5;
export const READ_IDLE_SECONDS = 900;
export const MAX_LIVE_READS = 4;
export const MAX_READ_ID_CHARS = 64;
const USAGE = "usage: node scripts/mcp-sleep-server.mjs [--log <file>]";

export const SLEEP_TOOL = {
  name: "sleep",
  description:
    "Dev-only measurement tool: waits `seconds`, optionally sending progress every `progress_every` seconds, then returns.",
  inputSchema: {
    type: "object",
    properties: {
      seconds: { type: "number", minimum: 0, maximum: MAX_SECONDS },
      progress_every: { type: "number", minimum: MIN_PROGRESS_EVERY, maximum: MAX_SECONDS },
    },
    required: ["seconds"],
    additionalProperties: false,
  },
};

export const STEPPED_READ_TOOL = {
  name: "stepped_read",
  description:
    "Dev-only measurement tool. Reads a result that takes several calls: each call works for about 40 seconds and returns. Call it first with no read_id. While the result says done is false, call it again with exactly the read_id it returned; carry on without asking the user, because each call is short. Only the last call, where done is true, carries the answer.",
  inputSchema: {
    type: "object",
    properties: { read_id: { type: "string", minLength: 1, maxLength: MAX_READ_ID_CHARS } },
    additionalProperties: false,
  },
};

export function parseArgs(argv) {
  const args = { log: null };
  for (let i = 0; i < argv.length; i += 1) {
    if (argv[i] === "--log" && i + 1 < argv.length && !argv[i + 1].startsWith("--")) args.log = argv[++i];
    else throw new Error(USAGE);
  }
  return args;
}

// Validates a call's arguments; returns { seconds, progressEvery } or an error string.
export function readSleepArguments(input) {
  if (!input || typeof input !== "object" || Array.isArray(input)) return "arguments must be an object";
  for (const key of Object.keys(input)) {
    if (key !== "seconds" && key !== "progress_every") return `unknown argument ${key}`;
  }
  const { seconds, progress_every: progressEvery } = input;
  if (typeof seconds !== "number" || !Number.isFinite(seconds) || seconds < 0 || seconds > MAX_SECONDS) {
    return `seconds must be a number from 0 to ${MAX_SECONDS}`;
  }
  if (progressEvery !== undefined && (typeof progressEvery !== "number" || !Number.isFinite(progressEvery) || progressEvery < MIN_PROGRESS_EVERY || progressEvery > MAX_SECONDS)) {
    return `progress_every must be a number from ${MIN_PROGRESS_EVERY} to ${MAX_SECONDS}`;
  }
  return { seconds, progressEvery: progressEvery ?? null };
}

// Validates a stepped_read call's arguments; returns { readId } (null for a first call) or an error string.
export function readSteppedArguments(input) {
  if (!input || typeof input !== "object" || Array.isArray(input)) return "arguments must be an object";
  for (const key of Object.keys(input)) {
    if (key !== "read_id") return `unknown argument ${key}`;
  }
  const { read_id: readId } = input;
  if (readId === undefined) return { readId: null };
  if (typeof readId !== "string" || readId.length < 1 || readId.length > MAX_READ_ID_CHARS) {
    return `read_id must be a string of 1 to ${MAX_READ_ID_CHARS} characters`;
  }
  return { readId };
}

// Letters only, so no id or answer in a pushed log reads as a number of any kind.
const LETTERS = "abcdefghjkmnpqrstuvwxyz";
const letters = (count) => [...randomBytes(count)].map((b) => LETTERS[b % LETTERS.length]).join("");
const mintReadId = () => `r${letters(12)}`;
const mintAnswer = () => letters(6).toUpperCase();

export function createServer({
  send,
  log,
  stepSeconds = STEP_SECONDS,
  steps = STEPS,
  idleSeconds = READ_IDLE_SECONDS,
  now = () => performance.now(),
  mint = mintReadId,
  answer = mintAnswer,
}) {
  const calls = new Map(); // request id (as JSON) -> { timers, started }
  const reads = new Map(); // read id -> { done, of, answer, finished, used }
  const key = (id) => JSON.stringify(id);

  function result(id, value) {
    send({ jsonrpc: "2.0", id, result: value });
  }
  function error(id, code, message) {
    send({ jsonrpc: "2.0", id, error: { code, message } });
    log({ event: "error_sent", id, code, message });
  }

  function startSleep(id, args, progressToken) {
    const started = performance.now();
    const timers = [];
    calls.set(key(id), { timers, started });
    if (args.progressEvery !== null && progressToken !== undefined) {
      let count = 0;
      const tick = setInterval(() => {
        count += 1;
        const elapsed = (performance.now() - started) / 1000;
        send({
          jsonrpc: "2.0",
          method: "notifications/progress",
          params: { progressToken, progress: count, total: Math.ceil(args.seconds / args.progressEvery), message: `slept ${elapsed.toFixed(1)} s` },
        });
        log({ event: "progress_sent", id, count, elapsed });
      }, args.progressEvery * 1000);
      timers.push(tick);
    }
    timers.push(
      setTimeout(() => {
        for (const t of timers) clearInterval(t);
        calls.delete(key(id));
        const elapsed = (performance.now() - started) / 1000;
        result(id, { content: [{ type: "text", text: `slept ${elapsed.toFixed(1)} s` }], isError: false });
        log({ event: "result_sent", id, elapsed });
      }, args.seconds * 1000),
    );
  }

  function text(id, value, isError) {
    result(id, { content: [{ type: "text", text: JSON.stringify(value) }], isError });
  }

  function refuse(id, code, readId) {
    text(id, { refused: code, read_id: readId, next: "Start the read again: call stepped_read with no read_id." }, true);
    log({ event: "step_refused", id, code, read_id: readId });
  }

  function startStep(id, readId) {
    let read;
    if (readId === null) {
      const live = [...reads].filter(([, r]) => !r.finished).sort(([, a], [, b]) => a.used - b.used);
      if (live.length >= MAX_LIVE_READS) {
        reads.delete(live[0][0]);
        log({ event: "read_evicted", read_id: live[0][0] });
      }
      readId = mint();
      read = { done: 0, of: steps, answer: answer(), finished: false, used: now() };
      reads.set(readId, read);
      log({ event: "read_started", id, read_id: readId, of: steps, answer: read.answer });
    } else {
      read = reads.get(readId);
      if (!read) return refuse(id, "read_unknown", readId);
      if (read.finished) return refuse(id, "read_finished", readId);
      if ((now() - read.used) / 1000 > idleSeconds) {
        reads.delete(readId);
        return refuse(id, "read_expired", readId);
      }
      read.used = now();
    }
    const started = performance.now();
    const timers = [];
    calls.set(key(id), { timers, started });
    timers.push(
      setTimeout(() => {
        calls.delete(key(id));
        read.done += 1;
        read.used = now();
        read.finished = read.done >= read.of;
        const elapsed = (performance.now() - started) / 1000;
        const step = { read_id: readId, step: read.done, of: read.of, done: read.finished };
        if (read.finished) step.answer = `The read is complete. Its answer is ${read.answer}.`;
        else step.next = `Call stepped_read again with read_id ${readId} to run step ${read.done + 1} of ${read.of}. No answer is given until done is true.`;
        text(id, step, false);
        log({ event: "step_result", id, read_id: readId, step: read.done, of: read.of, done: read.finished, elapsed });
      }, stepSeconds * 1000),
    );
  }

  function handle(message) {
    // A line of any length could otherwise fill the log: past the cap only its start and length are kept.
    const text = JSON.stringify(message) ?? "";
    log(text.length <= MAX_LOGGED_MESSAGE_CHARS ? { event: "received", message } : { event: "received", truncated: true, length: text.length, head: text.slice(0, 1024) });
    if (!message || typeof message !== "object" || Array.isArray(message) || message.jsonrpc !== "2.0") {
      return error(message?.id ?? null, -32600, "invalid request");
    }
    const { id, method, params } = message;
    if (method === undefined && ("result" in message || "error" in message)) return; // a response: this server sends no requests
    if (typeof method !== "string") return error(id ?? null, -32600, "invalid request");
    if (method === "notifications/cancelled") {
      const call = calls.get(key(params?.requestId));
      if (!call) return log({ event: "cancel_ignored", id: params?.requestId ?? null }); // finished, unknown or already cancelled
      for (const t of call.timers) { clearInterval(t); clearTimeout(t); }
      calls.delete(key(params.requestId));
      log({ event: "cancelled", id: params.requestId, elapsed: (performance.now() - call.started) / 1000, reason: params?.reason ?? null });
      return;
    }
    if (id === undefined) return; // other notifications (notifications/initialized and the rest) need no answer
    if (method === "initialize") {
      // The client's version is echoed: the server uses only methods every version has, and a host that
      // refused the connection would measure nothing.
      return result(id, {
        protocolVersion: typeof params?.protocolVersion === "string" ? params.protocolVersion : PROTOCOL_VERSION,
        capabilities: { tools: {} },
        serverInfo: { name: "bridge-dev-mcp-sleep", version: "0.0.0-dev" },
      });
    }
    if (method === "ping") return result(id, {});
    if (method === "tools/list") return result(id, { tools: [SLEEP_TOOL, STEPPED_READ_TOOL] });
    if (method === "tools/call" && params?.name === "stepped_read") {
      const args = readSteppedArguments(params.arguments ?? {});
      if (typeof args === "string") return error(id, -32602, args);
      if (calls.size > 0) return error(id, -32600, "a call is already running: this server runs one at a time");
      return startStep(id, args.readId);
    }
    if (method === "tools/call") {
      if (params?.name !== "sleep") return error(id, -32602, `unknown tool ${params?.name}`);
      const args = readSleepArguments(params.arguments ?? {});
      if (typeof args === "string") return error(id, -32602, args);
      if (calls.size > 0) return error(id, -32600, "a call is already running: this server runs one at a time");
      return startSleep(id, args, params?._meta?.progressToken);
    }
    return error(id, -32601, `method not found: ${method}`);
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

  return { handleLine, running: () => calls.size, liveReads: () => [...reads.values()].filter((r) => !r.finished).length };
}

function main() {
  let args;
  try {
    args = parseArgs(process.argv.slice(2));
  } catch (cause) {
    process.stderr.write(`${cause.message}\n`);
    process.exit(2);
  }
  const log = (event) => {
    const line = `${JSON.stringify({ time: new Date().toISOString(), pid: process.pid, ...event })}\n`;
    if (args.log) appendFileSync(args.log, line);
    else process.stderr.write(line);
  };
  const server = createServer({ send: (message) => process.stdout.write(`${JSON.stringify(message)}\n`), log });
  try {
    log({ event: "started", pid: process.pid });
  } catch (cause) {
    // A relative --log path resolves against the host's working directory, which is rarely this checkout.
    process.stderr.write(`cannot write the log ${args.log}: ${cause.message}\n`);
    process.exit(2);
  }
  const input = createInterface({ input: process.stdin });
  input.on("line", (line) => server.handleLine(line));
  // A host that gives up may close stdin or signal the server; either ends any running call unanswered,
  // and the log says which and how many.
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
