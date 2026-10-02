import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import test from "node:test";

import {
  RegistryFileError,
  checkMatchesManifest,
  checkNotAhead,
  checkPublished,
  checkServerJson,
  expectedAssetUrl,
  parseServerJson,
} from "./check-registry-server-json.mjs";

const HASH_WINDOWS = "a".repeat(64);
const HASH_MAC = "b".repeat(64);

function valid(version = "1.2.3") {
  return {
    $schema: "https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json",
    name: "io.github.ComplyEaze/bridge-tally",
    description: "A synthetic description.",
    version,
    packages: [
      { registryType: "mcpb", identifier: expectedAssetUrl(version, "windows-x64"), fileSha256: HASH_WINDOWS, transport: { type: "stdio" } },
      { registryType: "mcpb", identifier: expectedAssetUrl(version, "macos-arm64"), fileSha256: HASH_MAC, transport: { type: "stdio" } },
    ],
  };
}

function codeOf(run) {
  try {
    run();
  } catch (error) {
    assert.ok(error instanceof RegistryFileError, `expected a RegistryFileError, got ${error}`);
    return error.code;
  }
  return "no error";
}

test("a consistent file is accepted and names both assets", () => {
  const checked = checkServerJson(valid());
  assert.equal(checked.version, "1.2.3");
  assert.deepEqual(checked.assets.map((asset) => asset.platform), ["windows-x64", "macos-arm64"]);
});

test("each fault in the file is refused with its own code", () => {
  const cases = [
    ["schema_url", (s) => { s.$schema = "https://example.invalid/schema.json"; }],
    ["name", (s) => { s.name = "io.github.someone-else/bridge-tally"; }],
    ["name", (s) => { s.name = "io.github.ComplyEaze/bridge-tal1y"; }],
    ["description_length", (s) => { s.description = "x".repeat(101); }],
    ["description_length", (s) => { s.description = ""; }],
    ["version_form", (s) => { s.version = "1.2"; }],
    ["package_count", (s) => { s.packages.pop(); }],
    ["package_url", (s) => { s.packages[0].identifier = expectedAssetUrl("1.2.2", "windows-x64"); }],
    ["package_type", (s) => { s.packages[1].registryType = "npm"; }],
    ["package_hash_form", (s) => { s.packages[0].fileSha256 = "A".repeat(64); }],
    ["package_hash_form", (s) => { delete s.packages[1].fileSha256; }],
    ["package_hash_repeated", (s) => { s.packages[1].fileSha256 = HASH_WINDOWS; }],
  ];
  for (const [code, mutate] of cases) {
    const server = valid();
    mutate(server);
    assert.equal(codeOf(() => checkServerJson(server)), code);
  }
  assert.equal(codeOf(() => checkServerJson(null)), "not_an_object");
  assert.equal(codeOf(() => parseServerJson("{")), "invalid_json");
});

test("the version may trail the manifest but never run ahead of it", () => {
  assert.equal(codeOf(() => checkNotAhead("1.2.3", "1.2.3")), "no error");
  assert.equal(codeOf(() => checkNotAhead("1.2.3", "1.10.0")), "no error");
  assert.equal(codeOf(() => checkNotAhead("1.10.0", "1.2.3")), "ahead_of_manifest");
  assert.equal(codeOf(() => checkNotAhead("1.2.3", "next")), "manifest_version_form");
});

test("the publish check requires the version to equal the manifest's", () => {
  assert.equal(codeOf(() => checkMatchesManifest("1.2.3", "1.2.3")), "no error");
  assert.equal(codeOf(() => checkMatchesManifest("1.2.3", "1.2.4")), "version_mismatch");
  assert.equal(codeOf(() => checkMatchesManifest("1.2.3", "1.2")), "manifest_version_form");
});

test("publishing needs a final release whose recorded digests and .sha256 files equal the file's hashes, and an unreachable read is not a match", async () => {
  const checked = checkServerJson(valid());
  const apiUrl = "https://api.github.com/repos/ComplyEaze/bridge/releases/tags/mcp-v1.2.3";
  const names = { "windows-x64": HASH_WINDOWS, "macos-arm64": HASH_MAC };
  const release = (overrides = {}) => JSON.stringify({
    draft: false,
    prerelease: false,
    assets: Object.entries(names).map(([platform, hash]) => ({
      name: `bridge-tally-mcp-v1.2.3-${platform}.mcpb`,
      digest: `sha256:${hash}`,
    })),
    ...overrides,
  });
  const files = new Map([
    [apiUrl, release()],
    [`${expectedAssetUrl("1.2.3", "windows-x64")}.sha256`, `${HASH_WINDOWS}  bridge-tally-mcp-v1.2.3-windows-x64.mcpb\n`],
    [`${expectedAssetUrl("1.2.3", "macos-arm64")}.sha256`, `${HASH_MAC}  bridge-tally-mcp-v1.2.3-macos-arm64.mcpb\n`],
  ]);
  const read = async (url) => {
    if (!files.has(url)) throw new Error("HTTP 404");
    return files.get(url);
  };
  const rejected = async (run) => {
    try {
      await run();
    } catch (error) {
      assert.ok(error instanceof RegistryFileError);
      return error.code;
    }
    return "no error";
  };
  const macSidecar = `${expectedAssetUrl("1.2.3", "macos-arm64")}.sha256`;
  const winSidecar = `${expectedAssetUrl("1.2.3", "windows-x64")}.sha256`;

  assert.equal(await rejected(() => checkPublished(checked, "mcp-v1.2.3", read)), "no error");
  assert.equal(await rejected(() => checkPublished(checked, "mcp-v1.2.4", read)), "tag_mismatch");

  files.set(apiUrl, release({ prerelease: true }));
  assert.equal(await rejected(() => checkPublished(checked, "mcp-v1.2.3", read)), "release_not_final");
  files.set(apiUrl, release({ draft: true }));
  assert.equal(await rejected(() => checkPublished(checked, "mcp-v1.2.3", read)), "release_not_final");
  files.set(apiUrl, "not json");
  assert.equal(await rejected(() => checkPublished(checked, "mcp-v1.2.3", read)), "release_unavailable");
  files.delete(apiUrl);
  assert.equal(await rejected(() => checkPublished(checked, "mcp-v1.2.3", read)), "release_unavailable");

  files.set(apiUrl, release({ assets: [{ name: "bridge-tally-mcp-v1.2.3-windows-x64.mcpb", digest: `sha256:${HASH_WINDOWS}` }] }));
  assert.equal(await rejected(() => checkPublished(checked, "mcp-v1.2.3", read)), "digest_mismatch");
  names["macos-arm64"] = "c".repeat(64);
  files.set(apiUrl, release());
  assert.equal(await rejected(() => checkPublished(checked, "mcp-v1.2.3", read)), "digest_mismatch");
  names["macos-arm64"] = HASH_MAC;
  files.set(apiUrl, release());

  files.set(macSidecar, `${"c".repeat(64)}  x\n`);
  assert.equal(await rejected(() => checkPublished(checked, "mcp-v1.2.3", read)), "hash_mismatch");
  files.set(macSidecar, "not a hash\n");
  assert.equal(await rejected(() => checkPublished(checked, "mcp-v1.2.3", read)), "hash_file_form");
  files.delete(winSidecar);
  assert.equal(await rejected(() => checkPublished(checked, "mcp-v1.2.3", read)), "hash_file_unavailable");
});

test("the file in the repository is consistent and not ahead of the manifest", async () => {
  const root = resolve(import.meta.dirname, "..");
  const server = parseServerJson(await readFile(resolve(root, "registry", "server.json"), "utf8"));
  const manifest = JSON.parse(await readFile(resolve(root, "packaging", "mcpb", "manifest.json"), "utf8"));
  const checked = checkServerJson(server);
  assert.equal(codeOf(() => checkNotAhead(checked.version, manifest.version)), "no error");
});
