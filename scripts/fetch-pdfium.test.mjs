// SPDX-License-Identifier: Apache-2.0
// fetch-pdfium.mjs refuses every byte it was not pinned to, and writes the notice it was pinned
// to. Offline: archives are built here and passed with --archive, and the lock is a temporary copy.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, test } from "node:test";
import { gzipSync } from "node:zlib";

import { LOCK, main, noticeBytes, Refusal, tarFiles } from "./fetch-pdfium.mjs";

const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

function header(name, size, type = "0", { prefix = "", corrupt = false } = {}) {
  const block = Buffer.alloc(512);
  block.write(name, 0, 100, "utf8");
  block.write("0000644\0", 100, "latin1");
  block.write("0000000\0", 108, "latin1");
  block.write("0000000\0", 116, "latin1");
  block.write(`${size.toString(8).padStart(11, "0")}\0`, 124, "latin1");
  block.write("00000000000\0", 136, "latin1");
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
