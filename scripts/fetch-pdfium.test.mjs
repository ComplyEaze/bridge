// SPDX-License-Identifier: Apache-2.0
// fetch-pdfium.mjs refuses every byte it was not pinned to, and writes the notice it was pinned
// to. No network is used: archives are built here and passed with --archive, and the lock is a temporary copy.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createServer } from "node:http";
import { spawn } from "node:child_process";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, beforeEach, test } from "node:test";
import { gzipSync } from "node:zlib";

import { DOWNLOAD_ATTEMPTS, LOCK, main, noticeBytes, Refusal, RETRY_PAUSES_MS, tarFiles } from "./fetch-pdfium.mjs";

const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

function header(name, size, type = "0", { prefix = "", corrupt = false } = {}) {
  const block = Buffer.alloc(512);
  block.write(name, 0, 100, "utf8");
  block.write("0000644\0", 100, "latin1");
  block.write("0000000\0", 108, "latin1");
  block.write("0000000\0", 116, "latin1");
  block.write(`${size.toString(8).padStart(11, "0")}\0`, 124, "latin1");
  block.write(`${"0".repeat(11)}\0`, 136, "latin1"); // mtime: eleven octal zeros
  block.write(type, 156, "latin1");
  block.write("ustar\0", 257, "latin1");
  block.write("00", 263, "latin1");
  block.write(prefix, 345, 155, "utf8");
  block.fill(0x20, 148, 156);
  let sum = 0;
  for (const byte of block) sum += byte;
  block.write(`${(corrupt ? sum + 1 : sum).toString(8).padStart(6, "0")}\0 `, 148, "latin1");
  return block;
}

function entry(name, data, type = "0", options = {}) {
  const body = Buffer.from(data);
  const padding = Buffer.alloc((512 - (body.length % 512)) % 512);
  return Buffer.concat([header(name, body.length, type, options), body, padding]);
}

function paxRecord(key, value) {
  const tail = ` ${key}=${value}\n`;
  let length = tail.length + 1;
  while (`${length}${tail}`.length !== length) length += 1;
  return `${length}${tail}`;
}

function tar(entries) {
  return Buffer.concat([...entries, Buffer.alloc(1024)]);
}

function archiveOf(members) {
  return gzipSync(tar(Object.entries(members).map(([name, data]) => entry(name, data))));
}

let root;
let members;
let archive;
let lock;
const lockPath = () => join(root, "lock.json");

function writeLock() {
  writeFileSync(lockPath(), JSON.stringify(lock));
}

beforeEach(() => {
  root = mkdtempSync(join(tmpdir(), "fetch-pdfium-"));
  members = {
    LICENSE: "top licence\r\n",
    "licenses/b.txt": "b licence\n",
    "licenses/a.txt": "a licence\n",
    "lib/libpdfium.dylib": "library bytes",
    "include/fpdfview.h": "not shipped",
  };
  archive = archiveOf(members);
  lock = {
    source: "https://example.invalid/pdfium",
    release: "chromium/1",
    platforms: {
      test: { asset: "pdfium.tgz", bytes: archive.length, sha256: sha256(archive), library: "lib/libpdfium.dylib" },
    },
  };
  const pin = lock.platforms.test;
  const library = Buffer.from(members["lib/libpdfium.dylib"]);
  const files = new Map(Object.entries(members).map(([name, data]) => [name, Buffer.from(data)]));
  const notice = noticeBytes(lock, pin, files);
  Object.assign(pin, {
    library_bytes: library.length,
    library_sha256: sha256(library),
    notice_bytes: notice.length,
    notice_sha256: sha256(notice),
  });
  writeLock();
});

afterEach(() => rmSync(root, { recursive: true, force: true }));

async function runFetch(given = archive, extra = []) {
  const path = join(root, "given.tgz");
  writeFileSync(path, given);
  const output = [];
  const code = await main(["--platform", "test", "--dest", join(root, "out"), "--archive", path, ...extra], {
    lockPath: lockPath(),
    stdout: { write: (text) => output.push(text) },
  });
  return { code, output: output.join("") };
}

test("a pinned archive writes the library and a byte-exact notice", async () => {
  const envFile = join(root, "github-env");
  const { code, output } = await runFetch(archive, ["--github-env", envFile]);
  assert.equal(code, 0);
  const out = join(root, "out");
  assert.deepEqual(readdirSync(out).sort(), ["THIRD_PARTY_LICENSES_PDFIUM.txt", "libpdfium.dylib"]);
  assert.equal(readFileSync(join(out, "libpdfium.dylib"), "utf8"), "library bytes");
  const notice = readFileSync(join(out, "THIRD_PARTY_LICENSES_PDFIUM.txt"));
  // licence bytes are reproduced unmodified (the CRLF survives), LICENSE first, then by name
  assert.ok(notice.includes("===== LICENSE =====\ntop licence\r\n"));
  assert.ok(notice.indexOf("licenses/a.txt") < notice.indexOf("licenses/b.txt"));
  assert.ok(!notice.includes("not shipped"));
  assert.equal(output, `${join(out, "libpdfium.dylib")}\n`);
  assert.equal(readFileSync(envFile, "utf8"), `BRIDGE_PDFIUM_LIBRARY=${join(out, "libpdfium.dylib")}\n`);
});

test("an unpinned archive is refused before extraction", async () => {
  const tampered = archiveOf({ ...members, "lib/libpdfium.dylib": "other library" });
  await assert.rejects(runFetch(tampered), (error) => error instanceof Refusal && /refusing/.test(error.message));
  assert.equal(existsSync(join(root, "out")), false);
});

test("an archive whose digest differs from its pin is refused before extraction", async () => {
  lock.platforms.test.sha256 = "0".repeat(64);
  writeLock();
  await assert.rejects(runFetch(), (error) => error instanceof Refusal && error.message.startsWith("pdfium.tgz:"));
  assert.equal(existsSync(join(root, "out")), false);
});

test("a library that differs from its pin is refused", async () => {
  lock.platforms.test.library_sha256 = "0".repeat(64);
  writeLock();
  await assert.rejects(runFetch(), (error) => error instanceof Refusal && error.message.startsWith("lib/libpdfium.dylib"));
  assert.equal(existsSync(join(root, "out")), false);
});

test("a notice that differs from its pin is refused", async () => {
  lock.platforms.test.notice_sha256 = "0".repeat(64);
  writeLock();
  await assert.rejects(
    runFetch(),
    (error) => error instanceof Refusal && error.message.startsWith("THIRD_PARTY_LICENSES_PDFIUM"),
  );
  assert.equal(existsSync(join(root, "out")), false);
});

test("an unknown platform is refused", async () => {
  await assert.rejects(
    main(["--platform", "nowhere", "--dest", join(root, "out")], { lockPath: lockPath() }),
    (error) => error instanceof Refusal && /no pinned PDFium/.test(error.message),
  );
});

test("the committed lock pins every shipped platform", () => {
  const committed = JSON.parse(readFileSync(LOCK, "utf8"));
  for (const platform of ["macos-arm64", "windows-x64"]) {
    for (const key of ["sha256", "library_sha256", "notice_sha256"]) {
      assert.match(committed.platforms[platform][key], /^[0-9a-f]{64}$/, `${platform} ${key}`);
    }
  }
});

test("tar names: leading ./ is removed, a ustar prefix, a pax path and a GNU long name are read", () => {
  const long = `licenses/${"x".repeat(120)}.txt`;
  const files = tarFiles(
    tar([
      entry("./LICENSE", "a"),
      entry("pdfium.txt", "b", "0", { prefix: "licenses" }),
      entry("PaxHeader", paxRecord("path", long), "x"),
      entry("ignored-name", "c"),
      entry("././@LongLink", `licenses/${"y".repeat(110)}.txt\0`, "L"),
      entry("ignored-too", "d"),
      entry("lib/", "", "5"),
    ]),
  );
  assert.deepEqual([...files.keys()], ["LICENSE", "licenses/pdfium.txt", long, `licenses/${"y".repeat(110)}.txt`]);
  assert.equal(files.get(long).toString(), "c");
});

test("a pax size replaces the header's size", () => {
  const body = Buffer.concat([header("big", 0), Buffer.from("12345"), Buffer.alloc(507)]);
  const files = tarFiles(tar([entry("PaxHeader", paxRecord("size", "5"), "x"), body]));
  assert.equal(files.get("big").toString(), "12345");
});

test("a header that fails its checksum, a truncated archive and a malformed pax header are refused", () => {
  assert.throws(() => tarFiles(tar([entry("LICENSE", "a", "0", { corrupt: true })])), /checksum/);
  assert.throws(() => tarFiles(entry("LICENSE", "a").subarray(0, 600)), /runs past|ends inside/);
  assert.throws(() => tarFiles(Buffer.alloc(100)), /ends inside/);
  assert.throws(() => tarFiles(tar([entry("PaxHeader", "garbage", "x"), entry("LICENSE", "a")])), /pax header/);
});

test("a pax path applies to the next entry only, a pre-POSIX directory is skipped, and the last of two names wins", () => {
  const files = tarFiles(
    tar([
      entry("PaxHeader", paxRecord("path", "licenses/renamed.txt"), "x"),
      entry("first", "renamed"),
      entry("second", "kept its own name"),
      entry("licenses/", "", "\0"),
      entry("LICENSE", "old"),
      entry("LICENSE", "new"),
    ]),
  );
  assert.deepEqual([...files.keys()], ["licenses/renamed.txt", "second", "LICENSE"]);
  assert.equal(files.get("LICENSE").toString(), "new");
});

test("an unknown option is a refusal, not a stack trace", async () => {
  await assert.rejects(main(["--platfrom", "test"], { lockPath: lockPath() }), (error) => error instanceof Refusal && /usage/.test(error.message));
});

// The archive download is retried a bounded number of times, and only for failures that say nothing about the bytes.
// A scripted fetch stands in for the network; the pauses are recorded, not slept.
function scripted(...answers) {
  const calls = [];
  const fetchImpl = async (url) => {
    calls.push(url);
    const answer = answers[Math.min(calls.length, answers.length) - 1];
    if (answer instanceof Error) throw answer;
    return typeof answer === "function" ? answer() : answer;
  };
  return { calls, fetchImpl };
}

const ok = (bytes) => () => new Response(bytes, { status: 200 });

async function runDownload({ calls, fetchImpl }) {
  const pauses = [];
  const output = [];
  const logs = [];
  const code = await main(["--platform", "test", "--dest", join(root, "out")], {
    lockPath: lockPath(),
    stdout: { write: (text) => output.push(text) },
    fetchImpl,
    sleep: async (ms) => pauses.push(ms),
    log: (line) => logs.push(line),
  });
  return { code, pauses, calls, logs, output: output.join("") };
}

test("the retry budget is five attempts with four pauses of 5, 15, 45 and 90 seconds", () => {
  assert.equal(DOWNLOAD_ATTEMPTS, 5);
  assert.deepEqual(RETRY_PAUSES_MS, [5000, 15000, 45000, 90000]);
  assert.equal(RETRY_PAUSES_MS.length, DOWNLOAD_ATTEMPTS - 1);
});

test("a 502 and then the pinned archive succeeds on the second attempt, after one pause", async () => {
  const { code, pauses, calls } = await runDownload(scripted(new Response("bad gateway", { status: 502 }), ok(archive)));
  assert.equal(code, 0);
  assert.equal(calls.length, 2);
  assert.deepEqual(pauses, [5000]);
  assert.equal(readFileSync(join(root, "out", "libpdfium.dylib"), "utf8"), "library bytes");
});

test("a network error, then a 429, then the pinned archive succeeds on the third attempt, after two pauses", async () => {
  const { code, pauses, calls } = await runDownload(scripted(new TypeError("fetch failed"), new Response("slow down", { status: 429 }), ok(archive)));
  assert.equal(code, 0);
  assert.equal(calls.length, 3);
  assert.deepEqual(pauses, [5000, 15000]);
});

test("a body stream that breaks mid-download is retried", async () => {
  const broken = () => new Response(new ReadableStream({ start(controller) { controller.enqueue(archive.subarray(0, 10)); controller.error(new Error("connection reset")); } }), { status: 200 });
  const { code, calls } = await runDownload(scripted(broken, ok(archive)));
  assert.equal(code, 0);
  assert.equal(calls.length, 2);
});

test("five 503 answers are refused after exactly five attempts, naming the status", async () => {
  const run = runDownload(scripted(new Response("", { status: 503 })));
  await assert.rejects(run, (error) => error instanceof Refusal && /HTTP 503/.test(error.message) && /tried 5 times/.test(error.message));
  const probe = scripted(new Response("", { status: 503 }));
  const pauses = [];
  await main(["--platform", "test", "--dest", join(root, "out2")], { lockPath: lockPath(), fetchImpl: probe.fetchImpl, sleep: async (ms) => pauses.push(ms), log: () => {} }).catch(() => {});
  assert.equal(probe.calls.length, 5);
  assert.deepEqual(pauses, [5000, 15000, 45000, 90000]);
});

test("a 404 is refused on the first sight, without a retry or a pause", async () => {
  const probe = scripted(new Response("", { status: 404 }));
  const pauses = [];
  await assert.rejects(
    main(["--platform", "test", "--dest", join(root, "out")], { lockPath: lockPath(), fetchImpl: probe.fetchImpl, sleep: async (ms) => pauses.push(ms), log: () => {} }),
    (error) => error instanceof Refusal && /HTTP 404/.test(error.message) && !/tried/.test(error.message),
  );
  assert.equal(probe.calls.length, 1);
  assert.deepEqual(pauses, []);
});

test("a download whose bytes do not match the pin is refused at once and never retried", async () => {
  const tampered = Buffer.from(archive);
  tampered[tampered.length - 1] ^= 0xff;
  const probe = scripted(ok(tampered), ok(archive));
  const pauses = [];
  await assert.rejects(
    main(["--platform", "test", "--dest", join(root, "out")], { lockPath: lockPath(), fetchImpl: probe.fetchImpl, sleep: async (ms) => pauses.push(ms), log: () => {} }),
    (error) => error instanceof Refusal && /refusing/.test(error.message),
  );
  assert.equal(probe.calls.length, 1, "a mismatch must not be retried until a good answer arrives");
  assert.deepEqual(pauses, []);
  assert.equal(existsSync(join(root, "out")), false, "nothing is written for a refused archive");
});

test("a retry that succeeds still verifies the library and the notice (a good archive with a wrong library pin is refused)", async () => {
  lock.platforms.test.library_sha256 = "0".repeat(64);
  writeLock();
  const probe = scripted(new Response("", { status: 502 }), ok(archive));
  await assert.rejects(
    main(["--platform", "test", "--dest", join(root, "out")], { lockPath: lockPath(), fetchImpl: probe.fetchImpl, sleep: async () => {}, log: () => {} }),
    (error) => error instanceof Refusal && /libpdfium/.test(error.message),
  );
  assert.equal(probe.calls.length, 2);
});

test("each retry writes one line saying why, so a flaky host leaves a trace in the job log", async () => {
  const { logs } = await runDownload(scripted(new Response("", { status: 502 }), new TypeError("fetch failed"), ok(archive)));
  assert.equal(logs.length, 2);
  assert.match(logs[0], /HTTP 502.*attempt 1 of 5/);
  assert.match(logs[1], /fetch failed.*attempt 2 of 5/);
});

for (const status of [408, 500]) {
  test(`HTTP ${status} is retried`, async () => {
    const { code, calls } = await runDownload(scripted(new Response("", { status }), ok(archive)));
    assert.equal(code, 0);
    assert.equal(calls.length, 2);
  });
}

async function refusedAtOnce(response, pattern) {
  const probe = scripted(response, ok(archive));
  const pauses = [];
  await assert.rejects(
    main(["--platform", "test", "--dest", join(root, "out")], { lockPath: lockPath(), fetchImpl: probe.fetchImpl, sleep: async (ms) => pauses.push(ms), log: () => {} }),
    (error) => error instanceof Refusal && pattern.test(error.message) && !/tried/.test(error.message),
  );
  assert.equal(probe.calls.length, 1, "refused on the first sight, with no second request");
  assert.deepEqual(pauses, []);
}

test("a clean answer that is shorter than the pin is refused at once, not retried", async () => {
  await refusedAtOnce(() => new Response(archive.subarray(0, archive.length - 5), { status: 200 }), /refusing/);
});

test("a clean answer longer than the pin is refused at once, not retried", async () => {
  await refusedAtOnce(() => new Response(Buffer.concat([archive, Buffer.from("extra bytes")]), { status: 200 }), /refusing/);
});

test("a 204 with no body is a malformed answer, refused at once, not retried", async () => {
  await refusedAtOnce(() => new Response(null, { status: 204 }), /no body/);
});

test("the body of a failed answer is cancelled before the next attempt, so no socket is held open", async () => {
  let cancelled = 0;
  const failing = () => new Response(new ReadableStream({ pull(controller) { controller.enqueue(new Uint8Array(4)); }, cancel() { cancelled += 1; } }), { status: 502 });
  const { code } = await runDownload(scripted(failing, failing, ok(archive)));
  assert.equal(code, 0);
  assert.equal(cancelled, 2);
});

// --cache-dir: an archive kept by an earlier run saves the download, never the check.
async function runCached(probe, extra = []) {
  const logs = [];
  const code = await main(["--platform", "test", "--dest", join(root, "out"), "--cache-dir", join(root, "cache"), ...extra], {
    lockPath: lockPath(),
    stdout: { write: () => {} },
    fetchImpl: probe.fetchImpl,
    sleep: async () => {},
    log: (line) => logs.push(line),
  });
  return { code, logs };
}

test("a verified download is kept, and the next run uses it without a request", async () => {
  const first = scripted(ok(archive));
  const firstRun = await runCached(first);
  assert.equal(firstRun.code, 0);
  assert.ok(firstRun.logs.includes("pdfium cache: miss"));
  assert.equal(first.calls.length, 1);
  assert.deepEqual(readFileSync(join(root, "cache", "pdfium.tgz")), archive);
  assert.deepEqual(readdirSync(join(root, "cache")), ["pdfium.tgz"], "no partial file is left behind");
  const second = scripted(new Error("the network must not be used"));
  const { code, logs } = await runCached(second);
  assert.equal(code, 0);
  assert.equal(second.calls.length, 0);
  assert.ok(logs.some((line) => /pdfium cache: hit/.test(line)));
  assert.equal(readFileSync(join(root, "out", "libpdfium.dylib"), "utf8"), "library bytes");
});

test("a cached archive with one changed byte is discarded and downloaded again", async () => {
  mkdirSync(join(root, "cache"));
  const bad = Buffer.from(archive);
  bad[bad.length - 1] ^= 0xff;
  writeFileSync(join(root, "cache", "pdfium.tgz"), bad);
  const probe = scripted(ok(archive));
  const { code, logs } = await runCached(probe);
  assert.equal(code, 0);
  assert.equal(probe.calls.length, 1);
  assert.ok(logs.some((line) => /pdfium cache: bad/.test(line)));
  assert.deepEqual(readFileSync(join(root, "cache", "pdfium.tgz")), archive, "the good download replaced the bad copy");
});

for (const [label, make] of [
  ["a short file", () => writeFileSync(join(root, "cache", "pdfium.tgz"), archive.subarray(0, 10))],
  ["an empty file", () => writeFileSync(join(root, "cache", "pdfium.tgz"), "")],
  ["a longer file", () => writeFileSync(join(root, "cache", "pdfium.tgz"), Buffer.concat([archive, Buffer.from("x")]))],
  ["a directory", () => mkdirSync(join(root, "cache", "pdfium.tgz"))],
]) {
  test(`${label} in the cache is not used`, async () => {
    mkdirSync(join(root, "cache"));
    make();
    const probe = scripted(ok(archive));
    assert.equal((await runCached(probe)).code, 0);
    assert.equal(probe.calls.length, 1);
    assert.deepEqual(readFileSync(join(root, "cache", "pdfium.tgz")), archive);
  });
}

test("a download that does not match the pin is refused and is not kept", async () => {
  const tampered = Buffer.from(archive);
  tampered[tampered.length - 1] ^= 0xff;
  await assert.rejects(runCached(scripted(ok(tampered))), (error) => error instanceof Refusal && /refusing/.test(error.message));
  assert.equal(existsSync(join(root, "cache")), false, "nothing is kept for a refused download");
});

test("a cache directory that cannot be written costs the saving, not the run", async () => {
  writeFileSync(join(root, "cache"), "a file where the directory should be");
  const { code, logs } = await runCached(scripted(ok(archive)));
  assert.equal(code, 0);
  assert.ok(logs.some((line) => /could not keep pdfium\.tgz/.test(line)));
});

test("a local --archive is not read from or written to the cache", async () => {
  const path = join(root, "given.tgz");
  writeFileSync(path, archive);
  const probe = scripted(new Error("the network must not be used"));
  assert.equal((await runCached(probe, ["--archive", path])).code, 0);
  assert.equal(existsSync(join(root, "cache")), false);
});

test("a pinned asset name that is a path is refused when a cache directory is given", async () => {
  lock.platforms.test.asset = "../escape.tgz";
  writeLock();
  await assert.rejects(runCached(scripted(ok(archive))), (error) => error instanceof Refusal && /not a plain file name/.test(error.message));
  assert.equal(existsSync(join(root, "escape.tgz")), false);
});

test("with --strict-cache a kept file of the pinned size and a wrong digest ends the run in failure, after replacing it", async () => {
  mkdirSync(join(root, "cache"));
  const bad = Buffer.from(archive);
  bad[bad.length - 1] ^= 0xff;
  writeFileSync(join(root, "cache", "pdfium.tgz"), bad);
  const probe = scripted(ok(archive));
  const { code, logs } = await runCached(probe, ["--strict-cache"]);
  assert.equal(code, 1);
  assert.ok(logs.some((line) => /^::error::.*pinned size but not the pinned digest/.test(line)));
  assert.equal(probe.calls.length, 1);
  assert.equal(readFileSync(join(root, "out", "libpdfium.dylib"), "utf8"), "library bytes", "the library is still written from the verified download");
  assert.deepEqual(readFileSync(join(root, "cache", "pdfium.tgz")), archive);
});

test("with --strict-cache a short or empty kept file is only a warning", async () => {
  mkdirSync(join(root, "cache"));
  writeFileSync(join(root, "cache", "pdfium.tgz"), archive.subarray(0, 10));
  const { code, logs } = await runCached(scripted(ok(archive)), ["--strict-cache"]);
  assert.equal(code, 0);
  assert.ok(!logs.some((line) => line.startsWith("::error::")));
});

test("with --strict-cache a good kept file and a plain miss both pass", async () => {
  assert.equal((await runCached(scripted(ok(archive)), ["--strict-cache"])).code, 0);
  assert.equal((await runCached(scripted(new Error("the network must not be used")), ["--strict-cache"])).code, 0);
});

test("with --strict-cache a kept copy that cannot be replaced by a saved one ends the run in failure", async () => {
  writeFileSync(join(root, "cache"), "a file where the directory should be");
  const { code, logs } = await runCached(scripted(ok(archive)), ["--strict-cache"]);
  assert.equal(code, 1);
  assert.ok(logs.some((line) => /^::error::.*could not be kept/.test(line)));
});

// The exit status of the command line is what a workflow step sees, and main()'s return value only reaches it if the
// entry point passes it on. These run the script as a process, from a copy placed beside a temporary lock.
async function runCommandLine(cacheFile) {
  const here = dirname(fileURLToPath(import.meta.url));
  mkdirSync(join(root, "scripts"), { recursive: true });
  mkdirSync(join(root, "packaging", "pdfium"), { recursive: true });
  copyFileSync(join(here, "fetch-pdfium.mjs"), join(root, "scripts", "fetch-pdfium.mjs"));
  const server = createServer((request, response) => { response.writeHead(200); response.end(archive); });
  await new Promise((done) => server.listen(0, "127.0.0.1", done));
  try {
    const source = `http://127.0.0.1:${server.address().port}`;
    // The notice names the source and release, so its pin is recomputed for the local server's.
    const local = { ...lock, source, release: "r", platforms: { test: { ...lock.platforms.test } } };
    const files = new Map(Object.entries(members).map(([name, data]) => [name, Buffer.from(data)]));
    const notice = noticeBytes(local, local.platforms.test, files);
    Object.assign(local.platforms.test, { notice_bytes: notice.length, notice_sha256: sha256(notice) });
    writeFileSync(join(root, "packaging", "pdfium", "pdfium.lock.json"), JSON.stringify(local));
    mkdirSync(join(root, "cache"), { recursive: true });
    if (cacheFile) writeFileSync(join(root, "cache", "pdfium.tgz"), cacheFile);
    const flags = ["--platform", "test", "--dest", join(root, "out"), "--cache-dir", join(root, "cache"), "--strict-cache"];
    // Asynchronous, because the server above answers on this process's event loop.
    return await new Promise((done) => {
      const child = spawn(process.execPath, [join(root, "scripts", "fetch-pdfium.mjs"), ...flags]);
      let stdout = "";
      let stderr = "";
      child.stdout.on("data", (chunk) => { stdout += chunk; });
      child.stderr.on("data", (chunk) => { stderr += chunk; });
      child.on("close", (status) => done({ status, stdout, stderr }));
    });
  } finally {
    server.close();
  }
}

test("the command line exits 1 under --strict-cache for a kept copy of the pinned size and another digest", async () => {
  const bad = Buffer.from(archive);
  bad[bad.length - 1] ^= 0xff;
  const child = await runCommandLine(bad);
  assert.equal(child.status, 1, child.stdout + child.stderr);
  assert.match(child.stderr, /::error::.*pinned size but not the pinned digest/);
  assert.equal(readFileSync(join(root, "out", "libpdfium.dylib"), "utf8"), "library bytes");
});

test("the command line exits 0 under --strict-cache for a good kept copy", async () => {
  const child = await runCommandLine(archive);
  assert.equal(child.status, 0, child.stdout + child.stderr);
  assert.match(child.stderr, /pdfium cache: hit/);
});
