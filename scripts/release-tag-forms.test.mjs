// SPDX-License-Identifier: Apache-2.0
import assert from "node:assert/strict";
import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";

import { assertPreviewAdmission, previewVersion } from "./check-mcpb-preview-admission.mjs";
import { latestReleaseTag } from "./next-version.mjs";
import { versionFromTag } from "./release-notes.mjs";
import { writeMcpbReleaseMetadata } from "./write-mcpb-release-metadata.mjs";
import { assetName, isInstallablePreview } from "../site/release-catalog.mjs";

// Four modules on the release path (the admission gate, the release notes, the install page's
// predicate and the metadata writer) each hold their own copy of the release-tag pattern (the
// install page cannot import from scripts/). This test runs all four over the same tags, so a
// copy that drifts from the others fails here. The version proposer (next-version.mjs) reads
// tags too, with its own deliberately different pattern (no prerelease suffix, and the
// historical `v` form), so it has its own cases at the end. The current form is `mcp-vX.Y.Z`; the older
// `mcp-preview-X.Y.Z` is what the published 0.2.0 and 0.3.0 use and must keep working.
const ACCEPTED = [
  ["mcp-v0.4.0", "0.4.0"],
  ["mcp-v1.20.300", "1.20.300"],
  ["mcp-preview-0.3.0", "0.3.0"],
  ["mcp-preview-0.2.0", "0.2.0"],
];
const REJECTED = [
  "",
  "v0.4.0", // the historical MIT release form: not a release this workflow may publish
  "mcp-0.4.0",
  "mcp-v0.4",
  "mcp-v0.4.0.",
  "mcp-v",
  "mcp-vX.Y.Z",
  "mcp-w0.4.0",
  "mcp-preview-",
  "mcp-preview-0.4",
  "MCP-V0.4.0",
  " mcp-v0.4.0",
  "mcp-v0.4.0 ",
  "mcp-v0.4.0\n",
  "xmcp-v0.4.0",
  "mcp-v0.4.0/../x",
  "mcp-v0.4.0;rm",
  "mcp-v0.4.0/x",
  "mcp-v0.4.0+b",
  "mcp-v0x4x0",
  "mcp-v0.4.0_x",
  "mcp-v.4.0",
  "mcp-v0..4.0",
  "mcp-v0.4.0-",
  "mcp-v0.4.0-a_b",
  "mcp-v0.4.0-rc.1", // exactly X.Y.Z: the manifest carries no prerelease suffix
  "mcp-v0.4.0.beta",
  "mcp-v0.4.0.1",
  "mcp-v1.2.3.4",
  "mcp-preview-0.4.0-beta.1",
  "refs/tags/mcp-v0.4.0",
];

function release(tag) {
  return {
    tag_name: tag,
    draft: false,
    prerelease: false,
    published_at: "2026-10-01T00:00:00Z",
    assets: ["windows-x64", "macos-arm64"].flatMap((platform) => [
      { name: assetName(tag, platform), browser_download_url: "https://example.invalid/a" },
      { name: `${assetName(tag, platform)}.sha256`, browser_download_url: "https://example.invalid/s" },
    ]),
  };
}

test("every consumer of the release tag accepts the same tags and reads the same version", async (t) => {
  const directory = await mkdtemp(join(tmpdir(), "bridge-tag-forms-"));
  t.after(() => rm(directory, { recursive: true, force: true }));
  for (const [tag, version] of ACCEPTED) {
    assert.equal(previewVersion(tag), version, `admission version of ${tag}`);
    assert.equal(versionFromTag(tag), version, `release-notes version of ${tag}`);
    assert.equal(isInstallablePreview(release(tag)), true, `install page offers ${tag}`);
    const archive = join(directory, `bridge-tally-${tag}-windows-x64.mcpb`);
    await writeFile(archive, "fixture");
    const written = await writeMcpbReleaseMetadata({
      archivePath: archive,
      channel: "preview-unsigned",
      platform: "windows-x64",
      releaseTag: tag,
      sourceSha: "a".repeat(40),
    });
    assert.ok(written.sha256, `metadata written for ${tag}`);
    assert.doesNotThrow(
      () => assertPreviewAdmission({ releaseTag: tag, sourceRef: "refs/heads/master", defaultBranch: "master", applicationVersion: version, mcpbVersion: version }),
      `admission of ${tag}`,
    );
  }
});

test("every consumer of the release tag rejects the same tags", async (t) => {
  const directory = await mkdtemp(join(tmpdir(), "bridge-tag-forms-"));
  t.after(() => rm(directory, { recursive: true, force: true }));
  for (const tag of REJECTED) {
    assert.throws(() => previewVersion(tag), /release tag must be/, `admission rejects ${JSON.stringify(tag)}`);
    assert.equal(versionFromTag(tag), undefined, `release-notes rejects ${JSON.stringify(tag)}`);
    assert.equal(isInstallablePreview(release(tag)), false, `install page skips ${JSON.stringify(tag)}`);
    await assert.rejects(
      writeMcpbReleaseMetadata({
        archivePath: join(directory, "missing.mcpb"),
        channel: "preview-unsigned",
        platform: "windows-x64",
        releaseTag: tag,
        sourceSha: "a".repeat(40),
      }),
      /release tag must be/,
      `metadata rejects ${JSON.stringify(tag)}`,
    );
  }
});

test("the install page still offers the published releases beside a new-form one, newest first by publication", () => {
  const releases = [release("mcp-v0.4.0"), release("mcp-preview-0.3.0"), release("mcp-preview-0.2.0")];
  assert.equal(releases.every(isInstallablePreview), true);
  assert.equal(assetName("mcp-v0.4.0", "windows-x64"), "bridge-tally-mcp-v0.4.0-windows-x64.mcpb");
  assert.match(assetName("mcp-v0.4.0", "macos-arm64"), /mcp/, "the registry needs 'mcp' in the URL");
});

test("the newest tag is found across both forms and the bootstrap tag", () => {
  assert.equal(latestReleaseTag(["v0.1.0", "mcp-preview-0.2.0", "mcp-preview-0.3.0"]), "mcp-preview-0.3.0");
  assert.equal(latestReleaseTag(["v0.1.0", "mcp-preview-0.3.0", "mcp-v0.4.0"]), "mcp-v0.4.0");
  assert.equal(latestReleaseTag(["mcp-v0.4.0", "mcp-preview-0.10.0", "mcp-preview-0.9.1"]), "mcp-preview-0.10.0");
  assert.equal(latestReleaseTag(["mcp-v0.4.0", "mcp-v0.10.0", "mcp-v0.9.1"]), "mcp-v0.10.0");
  assert.equal(latestReleaseTag(["mcp-v0.4.0-rc.1", "mcp-v0.4.0.1", "other"]), null, "only exactly X.Y.Z is a release");
  assert.equal(latestReleaseTag(["refs/tags/v9.9.9", "xmcp-v9.9.9", "mcp-v9.9.9 ", "mcp-v0.4.0"]), "mcp-v0.4.0", "a tag with a junk prefix or suffix is ignored");
  assert.equal(latestReleaseTag([]), null);
});
