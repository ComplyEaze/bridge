#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0
// Fetch the pinned PDFium shared library, refusing any byte it did not expect.
//
// Three digests recorded in packaging/pdfium/pdfium.lock.json are checked, and a
// mismatch in any of them is a hard failure, never a warning, because the library
// is loaded into Bridge's process and shipped in the MCP bundle:
//
//   * the release archive (byte count and SHA-256), BEFORE anything is extracted;
//   * the extracted library (byte count and SHA-256);
//   * THIRD_PARTY_LICENSES_PDFIUM.txt, the notice this script writes beside it.
//
// The notice reproduces every licence file the archive ships, byte for byte, under
// a fixed header. It is generated rather than committed because the archives do
// not carry identical licence bytes: the Windows build's files use CRLF, and its
// pdfium.txt drops the `//` comment prefix the macOS build keeps.
//
// Usage:
//   node scripts/fetch-pdfium.mjs --platform macos-arm64 --dest DIR [--github-env FILE] [--cache-dir DIR] [--strict-cache]
//
// With --cache-dir, a copy of the archive kept there by an earlier run is used instead of a download, but only
// after it passes the same byte-count and SHA-256 check a download gets; a copy that fails it is deleted and the
// archive is downloaded. A downloaded archive that passed is written there. The directory is untrusted storage:
// it can make a run skip the network, not accept a byte the lock does not name, and this check is its only
// protection. A kept file of the pinned size whose bytes differ is more than damage in transit, so with
// --strict-cache the run still completes from the download but then exits 1 with an error annotation.
//
// Prints the absolute library path. With --github-env, also appends
// BRIDGE_PDFIUM_LIBRARY=<path> to that file (a GitHub Actions environment file).
// Node built-ins only, and only APIs the runners' preinstalled Node has: the
// native job runs this without actions/setup-node.

import { createHash } from "node:crypto";
import { appendFileSync, existsSync, mkdirSync, readFileSync, realpathSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parseArgs } from "node:util";
import { gunzipSync } from "node:zlib";

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "..");
export const LOCK = join(ROOT, "packaging", "pdfium", "pdfium.lock.json");
export const NOTICE = "THIRD_PARTY_LICENSES_PDFIUM.txt";

/** A refusal: printed on its own line and the process exits 1. */
export class Refusal extends Error {}

/** The licence notice for one archive: a fixed header, then each licence file (LICENSE first,
 * then licenses/ in name order) exactly as shipped. */
export function noticeBytes(lock, pin, files) {
  const header =
    "PDFium, as redistributed in the Bridge MCP bundle\n" +
    `Source: ${lock.source} release ${lock.release}, asset ${pin.asset}\n` +
    `Archive SHA-256: ${pin.sha256}\n` +
    "The licence files that asset ships follow, byte for byte.\n";
  const names = ["LICENSE", ...[...files.keys()].filter((name) => name.startsWith("licenses/")).sort(byCodePoint)];
  const parts = [Buffer.from(header, "utf8")];
  for (const name of names) {
    parts.push(Buffer.from(`\n===== ${name} =====\n`, "utf8"), files.get(name));
  }
  return Buffer.concat(parts);
}

function byCodePoint(a, b) {
  return a < b ? -1 : a > b ? 1 : 0;
}

export function verify(label, data, expectedBytes, expectedSha256) {
  const digest = createHash("sha256").update(data).digest("hex");
  if (data.length !== expectedBytes || digest !== expectedSha256) {
    throw new Refusal(
      `${label}: expected ${expectedBytes} bytes sha256 ${expectedSha256}, ` +
        `got ${data.length} bytes sha256 ${digest}; refusing it`,
    );
  }
}

/** The regular files of an uncompressed tar archive, by name with leading `.` and `/` characters
 * removed (as the Python original's `name.lstrip("./")` did); a later entry of the same name
 * replaces an earlier one. Reads POSIX ustar headers with their checksum, pax `path` and `size`
 * records, and GNU long names; refuses anything it cannot read rather than guessing. */
export function tarFiles(tar) {
  const files = new Map();
  let offset = 0;
  let pending = {};
  while (true) {
    if (offset + 512 > tar.length) throw new Refusal("tar archive ends inside a header");
    const header = tar.subarray(offset, offset + 512);
    if (header.every((byte) => byte === 0)) return files;
    checkChecksum(header, offset);
    const type = String.fromCharCode(header[156] || 0x30); // NUL is a regular file, as "0"
    const size = pending.size ?? octal(header.subarray(124, 136), offset);
    const dataStart = offset + 512;
    const dataEnd = dataStart + size;
    if (dataEnd > tar.length) throw new Refusal(`tar entry at ${offset} runs past the archive`);
    const data = tar.subarray(dataStart, dataEnd);
    offset = dataStart + Math.ceil(size / 512) * 512;
    if (type === "x") {
      pending = { ...pending, ...paxRecords(data, dataStart) };
      continue;
    }
    if (type === "g" || type === "K") continue;
    if (type === "L") {
      pending = { ...pending, path: nulTerminated(data) };
      continue;
    }
    const rawName = pending.path ?? ustarName(header);
    pending = {};
    // A pre-POSIX header marks a directory only by a trailing slash on a regular-file type.
    if ((type === "0" || type === "7") && !rawName.endsWith("/")) {
      files.set(rawName.replace(/^[./]+/, ""), Buffer.from(data));
    }
  }
}

function checkChecksum(header, offset) {
  let sum = 0;
  for (let index = 0; index < 512; index += 1) {
    sum += index >= 148 && index < 156 ? 0x20 : header[index];
  }
  if (octal(header.subarray(148, 156), offset) !== sum) {
    throw new Refusal(`tar header at ${offset} fails its checksum`);
  }
}

function octal(field, offset) {
  if (field[0] & 0x80) {
    let value = 0;
    for (let index = 1; index < field.length; index += 1) value = value * 256 + field[index];
    if (!Number.isSafeInteger(value)) throw new Refusal(`tar size at ${offset} is too large`);
    return value;
  }
  const text = nulTerminated(field).trim();
  if (!/^[0-7]*$/.test(text)) throw new Refusal(`tar header at ${offset} has a malformed number`);
  return text === "" ? 0 : parseInt(text, 8);
}

function nulTerminated(bytes) {
  const end = bytes.indexOf(0);
  return bytes.subarray(0, end === -1 ? bytes.length : end).toString("utf8");
}

function ustarName(header) {
  const name = nulTerminated(header.subarray(0, 100));
  const magic = header.subarray(257, 262).toString("latin1");
  const prefix = magic === "ustar" ? nulTerminated(header.subarray(345, 500)) : "";
  return prefix ? `${prefix}/${name}` : name;
}

function paxRecords(data, offset) {
  const records = {};
  let index = 0;
  while (index < data.length) {
    const space = data.indexOf(0x20, index);
    if (space === -1) throw new Refusal(`pax header at ${offset} is malformed`);
    const digits = data.subarray(index, space).toString("latin1");
    const length = /^[0-9]+$/.test(digits) ? Number(digits) : NaN;
    if (!Number.isInteger(length) || length <= space - index || index + length > data.length) {
      throw new Refusal(`pax header at ${offset} is malformed`);
    }
    const record = data.subarray(space + 1, index + length - 1).toString("utf8");
    const equals = record.indexOf("=");
    if (equals === -1) throw new Refusal(`pax header at ${offset} is malformed`);
    const key = record.slice(0, equals);
    const value = record.slice(equals + 1);
    if (key === "path") records.path = value;
    if (key === "size") {
      if (!/^[0-9]+$/.test(value)) throw new Refusal(`pax size at ${offset} is malformed`);
      records.size = Number(value);
    }
    index += length;
  }
  return records;
}

// Fetching the archive is retried a bounded number of times when the failure is the transport's or the
// server's and says nothing about the bytes: the connection failed or timed out, the body stream broke, or the
// host answered 408, 429 or a 5xx (a GitHub release download answered 502 on 23 October 2026 and failed a CI
// job). Nothing else is retried: any other HTTP status, a malformed answer, and above all a digest mismatch are
// refusals on the first sight, because verify() runs on the bytes AFTER the download returns and is not inside
// this loop, so a retry can never be used to fetch until the pinned digest happens to match.
//
// The pauses are longer than a blip because the outages seen were: on 8 and 9 Nov 2026 four Windows native
// jobs failed here with a 503 or 504 on all three attempts, whose pauses added up to 4 s, and a rerun a couple of
// minutes after one of them failed the same way. Five attempts with pauses of 5, 15, 45 and 90 s wait up to 155 s
// in all; a passing job spends nothing, a failing one spends that long longer (up to about 12 minutes if every
// attempt also runs to its 120 s limit).
export const RETRY_PAUSES_MS = [5_000, 15_000, 45_000, 90_000]; // before the second, third, fourth and fifth attempt
export const DOWNLOAD_ATTEMPTS = RETRY_PAUSES_MS.length + 1;

/** A failure worth another try; once the attempts are spent it is reported as the refusal it is. */
class Transient extends Refusal {}

const pause = (ms) => new Promise((done) => setTimeout(done, ms));

async function downloadOnce(url, limit, fetchImpl) {
  const response = await fetchImpl(url, { signal: AbortSignal.timeout(120_000), redirect: "follow" }).catch((error) => {
    throw new Transient(`${url}: ${error.message}; refusing it`);
  });
  if (!response.ok) {
    await response.body?.cancel().catch(() => {}); // release the socket before another attempt
    const message = `${url}: HTTP ${response.status}; refusing it`;
    throw response.status === 408 || response.status === 429 || response.status >= 500 ? new Transient(message) : new Refusal(message);
  }
  // An answer with no body (a 204, say) is malformed, not transient: refused on the first sight.
  if (!response.body) throw new Refusal(`${url}: HTTP ${response.status} with no body; refusing it`);
  // Read at most one byte past the pin, so an oversized answer is refused by its count, not read whole.
  // The 120-second signal bounds one whole attempt, body included, not each read; a retry makes a new one.
  const chunks = [];
  let total = 0;
  try {
    const reader = response.body.getReader();
    while (total < limit) {
      const { done, value } = await reader.read();
      if (done) break;
      chunks.push(value);
      total += value.length;
    }
    await reader.cancel().catch(() => {});
  } catch (error) {
    throw new Transient(`${url}: ${error.message}; refusing it`);
  }
  return Buffer.concat(chunks.map((chunk) => Buffer.from(chunk))).subarray(0, limit);
}

async function download(url, limit, { fetchImpl = fetch, sleep = pause, log = (line) => process.stderr.write(`${line}\n`) } = {}) {
  let last;
  for (let attempt = 1; attempt <= DOWNLOAD_ATTEMPTS; attempt += 1) {
    try {
      return await downloadOnce(url, limit, fetchImpl);
    } catch (error) {
      if (!(error instanceof Transient)) throw error;
      last = error;
      if (attempt < DOWNLOAD_ATTEMPTS) {
        log(`retrying after: ${error.message} (attempt ${attempt} of ${DOWNLOAD_ATTEMPTS})`);
        await sleep(RETRY_PAUSES_MS[attempt - 1]);
      }
    }
  }
  throw new Refusal(`${last.message} (tried ${DOWNLOAD_ATTEMPTS} times)`);
}

/** The archive kept in `cacheDir` by an earlier run: `data` is the verified bytes, or undefined when there is none
 * or it is not the pinned one (in which case it is deleted, so that the next run does not meet it again).
 * `altered` is true when the file had the pinned size but not the pinned digest. */
function cachedArchive(cacheDir, pin, log) {
  const file = join(cacheDir, pin.asset);
  if (!existsSync(file)) {
    log("pdfium cache: miss");
    return { data: undefined, altered: false };
  }
  let altered = false;
  try {
    if (!statSync(file).isFile() || statSync(file).size !== pin.bytes) throw new Refusal(`${file}: not a file of ${pin.bytes} bytes; refusing it`);
    const data = readFileSync(file);
    try {
      verify(pin.asset, data, pin.bytes, pin.sha256);
    } catch (error) {
      altered = true;
      throw error;
    }
    log("pdfium cache: hit");
    return { data, altered };
  } catch (error) {
    log(`::warning::pdfium cache: bad (${error.message}); downloading it`);
    try {
      rmSync(file, { force: true, recursive: true });
    } catch (removal) {
      log(`could not delete it either (${removal.message}); the download will replace it if it can`);
    }
    return { data: undefined, altered };
  }
}

/** Keep a verified archive for the next run; false when it could not be kept. A cache that cannot be written is a
 * missed saving, not a failure, except under --strict-cache, whose writer must not save a directory holding no archive. */
function keepArchive(cacheDir, pin, archive, log) {
  try {
    mkdirSync(cacheDir, { recursive: true });
    const temporary = join(cacheDir, `${pin.asset}.${process.pid}.part`);
    writeFileSync(temporary, archive);
    renameSync(temporary, join(cacheDir, pin.asset));
    return true;
  } catch (error) {
    log(`could not keep ${pin.asset} for the next run: ${error.message}`);
    return false;
  }
}

export async function main(argv = process.argv.slice(2), { lockPath = LOCK, stdout = process.stdout, fetchImpl = fetch, sleep = pause, log } = {}) {
  let values;
  try {
    ({ values } = parseArgs({
    args: argv,
    options: {
      platform: { type: "string" },
      dest: { type: "string" },
      "github-env": { type: "string" },
      archive: { type: "string" }, // a local copy, for tests
      "cache-dir": { type: "string" },
      "strict-cache": { type: "boolean" },
    },
    strict: true,
    }));
  } catch (error) {
    throw new Refusal(`${error.message}; usage: fetch-pdfium.mjs --platform P --dest DIR [--github-env FILE] [--cache-dir DIR] [--strict-cache]`);
  }
  if (!values.platform || !values.dest) throw new Refusal("usage: fetch-pdfium.mjs --platform P --dest DIR [--github-env FILE] [--cache-dir DIR] [--strict-cache]");
  const lock = JSON.parse(readFileSync(lockPath, "utf8"));
  const pin = Object.hasOwn(lock.platforms, values.platform) ? lock.platforms[values.platform] : undefined;
  if (pin === undefined) throw new Refusal(`no pinned PDFium for platform '${values.platform}'`);
  if (values["cache-dir"] && basename(pin.asset) !== pin.asset) throw new Refusal(`the pinned asset name '${pin.asset}' is not a plain file name; refusing to use a cache directory with it`);
  const say = log ?? ((line) => process.stderr.write(`${line}\n`));
  const cacheDir = values.archive || !values["cache-dir"] ? undefined : resolve(values["cache-dir"]);
  let archive;
  let fromCache = false;
  let altered = false;
  if (values.archive) archive = readFileSync(values.archive);
  else if (cacheDir) {
    ({ data: archive, altered } = cachedArchive(cacheDir, pin, say));
    fromCache = archive !== undefined;
  }
  archive ??= await download(`${lock.source}/releases/download/${lock.release}/${pin.asset}`, pin.bytes + 1, { fetchImpl, sleep, log });
  verify(pin.asset, archive, pin.bytes, pin.sha256);
  const kept = cacheDir && !fromCache ? keepArchive(cacheDir, pin, archive, say) : true;

  const files = new Map();
  for (const [name, data] of tarFiles(gunzipSync(archive))) {
    if (name === pin.library || name === "LICENSE" || name.startsWith("licenses/")) files.set(name, data);
  }
  if (!files.has(pin.library) || !files.has("LICENSE")) {
    throw new Refusal(`${pin.asset} does not contain ${pin.library} and LICENSE`);
  }
  verify(pin.library, files.get(pin.library), pin.library_bytes, pin.library_sha256);
  const notice = noticeBytes(lock, pin, files);
  verify(NOTICE, notice, pin.notice_bytes, pin.notice_sha256);

  const dest = resolve(values.dest);
  const library = join(dest, basename(pin.library));
  mkdirSync(dest, { recursive: true });
  writeFileSync(library, files.get(pin.library));
  writeFileSync(join(dest, NOTICE), notice);
  stdout.write(`${library}\n`);
  if (values["github-env"]) appendFileSync(values["github-env"], `BRIDGE_PDFIUM_LIBRARY=${library}\n`, "utf8");
  if (!kept && values["strict-cache"]) {
    say(`::error::the verified ${pin.asset} could not be kept in ${cacheDir}, so there is nothing to save`);
    return 1;
  }
  if (altered && values["strict-cache"]) {
    say(`::error::the kept ${pin.asset} had the pinned size but not the pinned digest; it was replaced from the download. Delete the cache entry (gh cache delete) or bump the key suffix`);
    return 1;
  }
  return 0;
}

if (process.argv[1] && existsSync(process.argv[1]) && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url))) {
  main().then(
    (code) => {
      process.exitCode = code;
    },
    (error) => {
      process.stderr.write(`${error instanceof Refusal ? error.message : error.stack}\n`);
      process.exitCode = 1;
    },
  );
}
