#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0
// Checks registry/server.json, the file published to the MCP registry for a release.
//   (no flag)         the file is consistent with itself and not ahead of the manifest version;
//   --match-manifest  its version equals packaging/mcpb/manifest.json's;
//   --published TAG   as --match-manifest, TAG is mcp-v<version>, and each fileSha256 equals the
//                     .sha256 file of the release asset it names (read from GitHub).
// The version cannot be required equal on every pull request: the version pull request bumps the
// manifest before the release exists, and the hashes are known only after the release is built.
// Equality and the hashes are therefore checked when the publish workflow runs.
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

export class RegistryFileError extends Error {
  constructor(code, detail = "") {
    super(detail ? `${code}: ${detail}` : code);
    this.name = "RegistryFileError";
    this.code = code;
  }
}

const SCHEMA = "https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json";
const NAME = /^io\.github\.ComplyEaze\/[A-Za-z0-9][A-Za-z0-9._-]*$/;
const SEMVER = /^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$/;
const SHA256 = /^[0-9a-f]{64}$/;
const PLATFORMS = ["windows-x64", "macos-arm64"];
const DESCRIPTION_LIMIT = 100;

export function releaseTag(version) {
  return `mcp-v${version}`;
}

export function expectedAssetUrl(version, platform) {
  return `https://github.com/ComplyEaze/bridge/releases/download/${releaseTag(version)}/bridge-tally-mcp-v${version}-${platform}.mcpb`;
}

export function parseServerJson(text) {
  try {
    return JSON.parse(text);
  } catch {
    throw new RegistryFileError("invalid_json");
  }
}

function compareVersions(a, b) {
  const left = SEMVER.exec(a);
  const right = SEMVER.exec(b);
  for (let i = 1; i <= 3; i += 1) {
    const difference = Number(left[i]) - Number(right[i]);
    if (difference !== 0) return difference;
  }
  return 0;
}

// Returns the version and the packages it names; throws a RegistryFileError for the first fault.
export function checkServerJson(server) {
  if (server === null || typeof server !== "object" || Array.isArray(server)) {
    throw new RegistryFileError("not_an_object");
  }
  if (server.$schema !== SCHEMA) throw new RegistryFileError("schema_url");
  if (typeof server.name !== "string" || !NAME.test(server.name)) throw new RegistryFileError("name");
  if (typeof server.description !== "string" || server.description.length === 0 || server.description.length > DESCRIPTION_LIMIT) {
    throw new RegistryFileError("description_length");
  }
  if (typeof server.version !== "string" || !SEMVER.test(server.version)) throw new RegistryFileError("version_form");
  const packages = server.packages;
  if (!Array.isArray(packages) || packages.length !== PLATFORMS.length) throw new RegistryFileError("package_count");
  const assets = PLATFORMS.map((platform) => {
    const url = expectedAssetUrl(server.version, platform);
    const entry = packages.find((candidate) => candidate?.identifier === url);
    if (!entry) throw new RegistryFileError("package_url", platform);
    if (entry.registryType !== "mcpb") throw new RegistryFileError("package_type", platform);
    if (typeof entry.fileSha256 !== "string" || !SHA256.test(entry.fileSha256)) throw new RegistryFileError("package_hash_form", platform);
    return { platform, url, sha256: entry.fileSha256 };
  });
  if (new Set(assets.map((asset) => asset.sha256)).size !== assets.length) throw new RegistryFileError("package_hash_repeated");
  return { version: server.version, assets };
}

export function checkNotAhead(version, manifestVersion) {
  if (!SEMVER.test(manifestVersion)) throw new RegistryFileError("manifest_version_form");
  if (compareVersions(version, manifestVersion) > 0) throw new RegistryFileError("ahead_of_manifest");
}

export function checkMatchesManifest(version, manifestVersion) {
  if (!SEMVER.test(manifestVersion)) throw new RegistryFileError("manifest_version_form");
  if (version !== manifestVersion) throw new RegistryFileError("version_mismatch");
}

// fetchText(url) resolves to the response body of an HTTP 200, and rejects on any other outcome,
// so that a failed request is never read as an empty or matching answer.
export async function checkPublished(checked, tag, fetchText) {
  if (tag !== releaseTag(checked.version)) throw new RegistryFileError("tag_mismatch");
  for (const asset of checked.assets) {
    let body;
    try {
      body = await fetchText(`${asset.url}.sha256`);
    } catch {
      throw new RegistryFileError("hash_file_unavailable", asset.platform);
    }
    const listed = String(body).trim().split(/\s+/)[0];
    if (!SHA256.test(listed)) throw new RegistryFileError("hash_file_form", asset.platform);
    if (listed !== asset.sha256) throw new RegistryFileError("hash_mismatch", asset.platform);
  }
}

async function fetchOk(url) {
  const response = await fetch(url, { redirect: "follow" });
  if (response.status !== 200) throw new Error(`HTTP ${response.status}`);
  return response.text();
}

async function main() {
  const root = resolve(import.meta.dirname, "..");
  const args = process.argv.slice(2);
  const published = args.indexOf("--published");
  const tag = published >= 0 ? args[published + 1] : undefined;
  if (published >= 0 && !tag) throw new RegistryFileError("usage", "--published needs a tag");
  const known = new Set(["--match-manifest", "--published", tag]);
  for (const arg of args) if (!known.has(arg)) throw new RegistryFileError("usage", `unknown argument ${arg}`);
  const server = parseServerJson(await readFile(resolve(root, "registry", "server.json"), "utf8"));
  const manifest = JSON.parse(await readFile(resolve(root, "packaging", "mcpb", "manifest.json"), "utf8"));
  const checked = checkServerJson(server);
  if (published >= 0 || args.includes("--match-manifest")) checkMatchesManifest(checked.version, manifest.version);
  else checkNotAhead(checked.version, manifest.version);
  if (published >= 0) await checkPublished(checked, tag, fetchOk);
  console.log(`registry/server.json ok (version ${checked.version}${published >= 0 ? `, hashes equal the ${tag} release` : ""})`);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  try {
    await main();
  } catch (error) {
    console.error(error instanceof RegistryFileError ? `registry/server.json refused: ${error.message}` : error);
    process.exit(1);
  }
}
