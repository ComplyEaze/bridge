// SPDX-License-Identifier: Apache-2.0
//
// Tests for scripts/mcp-sleep-server.mjs, the dev-only measurement server of #703 step 0. Each test
// starts the real server over stdio and reads its answers line by line, and its event log as a file.

import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test, { after } from "node:test";
import { fileURLToPath } from "node:url";

import { createServer, MAX_LOGGED_MESSAGE_CHARS, MAX_SECONDS, MIN_PROGRESS_EVERY, parseArgs, readSleepArguments, SLEEP_TOOL } from "./mcp-sleep-server.mjs";

const SERVER = fileURLToPath(new URL("./mcp-sleep-server.mjs", import.meta.url));
const scratch = mkdtempSync(join(tmpdir(), "mcp-sleep-"));
const children = new Set();
after(() => {
  for (const child of children) child.kill(); // a test that failed before stop() leaves its server running
  rmSync(scratch, { recursive: true, force: true });
});
let logCount = 0;

// Starts the server; `next()` resolves with the next message it writes, or rejects after `ms`.
function start() {
  const log = join(scratch, `log-${(logCount += 1)}.jsonl`);
  const child = spawn(process.execPath, [SERVER, "--log", log], { stdio: ["pipe", "pipe", "pipe"] });
  children.add(child);
  child.on("exit", () => children.delete(child));
  const queue = [];
  const waiting = [];
  let buffer = "";
  child.stdout.setEncoding("utf8");
  child.stdout.on("data", (chunk) => {
    buffer += chunk;
    let at;
    while ((at = buffer.indexOf("\n")) >= 0) {
      const message = JSON.parse(buffer.slice(0, at));
      buffer = buffer.slice(at + 1);
      const waiter = waiting.shift();
      if (waiter) waiter.resolve(message);
      else queue.push(message);
    }
  });
  return {
    send: (message) => child.stdin.write(`${typeof message === "string" ? message : JSON.stringify(message)}\n`),
    next: (ms = 3000) =>
      queue.length
        ? Promise.resolve(queue.shift())
        : new Promise((resolve, reject) => {
            const timer = setTimeout(() => reject(new Error(`no message within ${ms} ms`)), ms);
            waiting.push({ resolve: (m) => { clearTimeout(timer); resolve(m); } });
          }),
    events: () => readFileSync(log, "utf8").trim().split("\n").map((line) => JSON.parse(line)),
    stop: () => new Promise((resolve) => { child.on("exit", resolve); child.stdin.end(); }),
  };
}

async function initialized() {
  const server = start();
  server.send({ jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "test", version: "0" } } });
  const init = await server.next();
  assert.equal(init.id, 1);
  assert.equal(init.result.protocolVersion, "2025-06-18");
  assert.equal(init.result.serverInfo.name, "bridge-dev-mcp-sleep");
  server.send({ jsonrpc: "2.0", method: "notifications/initialized" });
  return server;
}

test("the handshake answers, lists exactly the one sleep tool, and answers ping", async () => {
  const server = await initialized();
  server.send({ jsonrpc: "2.0", id: 2, method: "tools/list" });
  const list = await server.next();
  assert.deepEqual(list.result.tools, [SLEEP_TOOL]);
  // the bounds are pinned here, not only compared with the exported tool
  assert.deepEqual(list.result.tools[0].inputSchema.properties, {
    seconds: { type: "number", minimum: 0, maximum: 900 },
    progress_every: { type: "number", minimum: 0.1, maximum: 900 },
  });
  server.send({ jsonrpc: "2.0", id: 3, method: "ping" });
  assert.deepEqual((await server.next()).result, {});
  await server.stop();
});

test("a sleep returns after the time asked for, and the log records the call and the result", async () => {
  const server = await initialized();
  const sent = Date.now();
  // progress_every without a progressToken: the host asked for no progress, so none is sent
  server.send({ jsonrpc: "2.0", id: 7, method: "tools/call", params: { name: "sleep", arguments: { seconds: 0.4, progress_every: 0.1 } } });
  const answer = await server.next();
  const waited = Date.now() - sent;
  assert.equal(answer.id, 7);
  assert.equal(answer.result.isError, false);
  assert.match(answer.result.content[0].text, /^slept \d+\.\d s$/);
  assert.ok(waited >= 380, `answered after ${waited} ms`);
  await server.stop();
  const events = server.events();
  const call = events.find((e) => e.event === "received" && e.message.id === 7);
  assert.deepEqual(call.message.params.arguments, { seconds: 0.4, progress_every: 0.1 });
  const sentResult = events.find((e) => e.event === "result_sent");
  assert.equal(sentResult.id, 7);
  assert.ok(sentResult.elapsed >= 0.38, `logged elapsed ${sentResult.elapsed}`);
  assert.equal(events.filter((e) => e.event === "progress_sent").length, 0, "no progress without a token");
});

test("progress is sent at the interval only when the call carries a progress token", async () => {
  const server = await initialized();
  server.send({ jsonrpc: "2.0", id: 9, method: "tools/call", params: { name: "sleep", arguments: { seconds: 1.2, progress_every: 0.3 }, _meta: { progressToken: "tok-9" } } });
  const messages = [];
  for (;;) {
    const m = await server.next();
    messages.push(m);
    if (m.id === 9) break;
  }
  const progress = messages.filter((m) => m.method === "notifications/progress");
  assert.ok(progress.length >= 2, `${progress.length} progress notifications`);
  assert.ok(progress.every((m) => m.params.progressToken === "tok-9" && m.params.total === 4));
  assert.deepEqual(progress.map((m) => m.params.progress), progress.map((_, i) => i + 1));
  await server.stop();
});

test("a cancellation stops the sleep and no result is ever sent", async () => {
  const server = await initialized();
  server.send({ jsonrpc: "2.0", id: 11, method: "tools/call", params: { name: "sleep", arguments: { seconds: 2, progress_every: 0.2 }, _meta: { progressToken: 11 } } });
  await server.next(); // the first progress notification: the call is running
  server.send({ jsonrpc: "2.0", method: "notifications/cancelled", params: { requestId: 11, reason: "test" } });
  await assert.rejects(async () => {
    for (;;) {
      const m = await server.next(2500);
      assert.notEqual(m.id, 11, "a cancelled call must not answer");
    }
  }, /no message within/);
  await server.stop();
  const cancelled = server.events().filter((e) => e.event === "cancelled");
  assert.equal(cancelled.length, 1);
  assert.equal(cancelled[0].id, 11);
  assert.equal(cancelled[0].reason, "test");
});

test("a cancellation after the result, or for an unknown id, is logged as ignored", async () => {
  const server = await initialized();
  server.send({ jsonrpc: "2.0", id: 12, method: "tools/call", params: { name: "sleep", arguments: { seconds: 0 } } });
  assert.equal((await server.next()).id, 12);
  server.send({ jsonrpc: "2.0", method: "notifications/cancelled", params: { requestId: 12 } });
  server.send({ jsonrpc: "2.0", method: "notifications/cancelled", params: { requestId: "nobody" } });
  server.send({ jsonrpc: "2.0", id: 13, method: "ping" });
  assert.equal((await server.next()).id, 13);
  await server.stop();
  const ignored = server.events().filter((e) => e.event === "cancel_ignored").map((e) => e.id);
  assert.deepEqual(ignored, [12, "nobody"]);
  assert.equal(server.events().filter((e) => e.event === "cancelled").length, 0);
});

test("bad input is refused in-band and the server keeps answering", async () => {
  const server = await initialized();
  server.send("{not json");
  assert.equal((await server.next()).error.code, -32700);
  server.send({ jsonrpc: "2.0", id: 20, method: "tools/call", params: { name: "sleep", arguments: { seconds: MAX_SECONDS + 1 } } });
  assert.equal((await server.next()).error.code, -32602);
  server.send({ jsonrpc: "2.0", id: 21, method: "tools/call", params: { name: "other", arguments: {} } });
  assert.equal((await server.next()).error.code, -32602);
  server.send({ jsonrpc: "2.0", id: 22, method: "resources/list" });
  assert.equal((await server.next()).error.code, -32601);
  server.send({ id: 24, method: "ping" }); // no jsonrpc member
  assert.deepEqual((await server.next()).error, { code: -32600, message: "invalid request" });
  server.send("[1]");
  assert.equal((await server.next()).error.code, -32600);
  server.send({ jsonrpc: "2.0", id: 25, result: {} }); // a response: this server sends no requests, so it is not answered
  server.send({ jsonrpc: "2.0", id: 23, method: "ping" });
  assert.deepEqual((await server.next()).result, {});
  await server.stop();
});

test("argument and option checks", () => {
  assert.deepEqual(readSleepArguments({ seconds: 5 }), { seconds: 5, progressEvery: null });
  assert.deepEqual(readSleepArguments({ seconds: 5, progress_every: 1 }), { seconds: 5, progressEvery: 1 });
  assert.equal(MIN_PROGRESS_EVERY, 0.1);
  assert.deepEqual(readSleepArguments({ seconds: 5, progress_every: MIN_PROGRESS_EVERY }), { seconds: 5, progressEvery: 0.1 });
  for (const bad of [null, [], {}, { seconds: -1 }, { seconds: 1, progress_every: 0.05 }, { seconds: 1, progress_every: 901 }, { seconds: "5" }, { seconds: Infinity }, { seconds: 1, progress_every: 0 }, { seconds: 1, extra: 1 }]) {
    assert.equal(typeof readSleepArguments(bad), "string", JSON.stringify(bad));
  }
  assert.deepEqual(parseArgs([]), { log: null });
  assert.deepEqual(parseArgs(["--log", "x.jsonl"]), { log: "x.jsonl" });
  for (const bad of [["--bogus"], ["--log"], ["--log", "--x"]]) assert.throws(() => parseArgs(bad), /usage:/);
});

test("a second call while one runs is refused in-band, and the first still answers", async () => {
  const server = await initialized();
  server.send({ jsonrpc: "2.0", id: 30, method: "tools/call", params: { name: "sleep", arguments: { seconds: 0.5 } } });
  server.send({ jsonrpc: "2.0", id: 31, method: "tools/call", params: { name: "sleep", arguments: { seconds: 0 } } });
  const refused = await server.next();
  assert.equal(refused.id, 31);
  assert.equal(refused.error.code, -32600);
  const answered = await server.next();
  assert.equal(answered.id, 30);
  assert.equal(answered.result.isError, false);
  await server.stop();
});

test("a received line past the cap is logged by its length and start only", () => {
  const events = [];
  const server = createServer({ send: () => {}, log: (event) => events.push(event) });
  server.handleLine(JSON.stringify({ jsonrpc: "2.0", id: 1, method: "ping" }));
  server.handleLine(JSON.stringify({ jsonrpc: "2.0", id: 2, method: "ping", params: { pad: "x".repeat(MAX_LOGGED_MESSAGE_CHARS) } }));
  const [small, large] = events.filter((e) => e.event === "received");
  assert.equal(small.message.id, 1);
  assert.equal(large.truncated, true);
  assert.ok(large.length > MAX_LOGGED_MESSAGE_CHARS);
  assert.equal(large.head.length, 1024);
  assert.equal("message" in large, false);
});
