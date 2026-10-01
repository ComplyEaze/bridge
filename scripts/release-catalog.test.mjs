import assert from "node:assert/strict";
import test from "node:test";

import { assetName, combineReleaseSources, isInstallablePreview, mergeReleases, releaseAssets, releaseLabel, selectRelease } from "../site/release-catalog.mjs";

const preview = {
  draft: false,
  prerelease: true,
  tag_name: "mcp-preview-0.2.0",
  assets: [
    { name: "bridge-tally-mcp-preview-0.2.0-windows-x64.mcpb", browser_download_url: "https://example.invalid/windows" },
    { name: "bridge-tally-mcp-preview-0.2.0-windows-x64.mcpb.sha256", browser_download_url: "https://example.invalid/windows.sha256" },
    { name: "bridge-tally-mcp-preview-0.2.0-macos-arm64.mcpb", browser_download_url: "https://example.invalid/macos" },
    { name: "bridge-tally-mcp-preview-0.2.0-macos-arm64.mcpb.sha256", browser_download_url: "https://example.invalid/macos.sha256" },
  ],
};

test("release catalogue selects the newest usable release, not an unrelated one", () => {
  const unrelatedNewest = { ...preview, prerelease: false, tag_name: "v9.0.0", assets: [] };
  const incompleteNewestPreview = { ...preview, tag_name: "mcp-preview-0.3.0", assets: preview.assets.slice(0, 2) };
  assert.equal(isInstallablePreview(unrelatedNewest), false);
  assert.equal(isInstallablePreview(incompleteNewestPreview), false);
  assert.equal(selectRelease([unrelatedNewest, incompleteNewestPreview, preview]), preview);
  assert.match(releaseLabel(preview), /not yet code-signed/);
});

test("a preview marked Latest (pre-release flag cleared) stays installable", () => {
  const latest = { ...preview, prerelease: false };
  assert.equal(isInstallablePreview(latest), true);
  assert.equal(selectRelease([latest]), latest);
  assert.equal(isInstallablePreview({ ...latest, draft: true }), false);
});

test("the live list and the published snapshot merge newest first, live entry winning a shared tag", () => {
  const older = { ...preview, published_at: "2026-09-16T08:33:24Z" };
  const newerTag = "mcp-preview-0.3.0";
  const newerAssets = preview.assets.map((asset) => ({ ...asset, name: asset.name.replace("0.2.0", "0.3.0") }));
  const newerLive = { ...preview, tag_name: newerTag, prerelease: false, published_at: "2026-09-26T11:19:29Z", assets: newerAssets };
  const newerSnapshot = { ...newerLive, assets: newerAssets.slice(0, 2) };

  assert.deepEqual(mergeReleases([], [older, newerSnapshot]).map((release) => release.tag_name), [newerTag, "mcp-preview-0.2.0"]);
  const merged = mergeReleases([newerLive], [older, newerSnapshot]);
  assert.equal(merged.length, 2);
  assert.equal(merged[0], newerLive);
  assert.equal(selectRelease(merged), newerLive);
  // With only the snapshot, its incomplete 0.3.0 entry is skipped and 0.2.0 is still offered.
  assert.equal(selectRelease(mergeReleases([], [older, newerSnapshot])), older);
});

test("the page's two sources: both failing is unavailable, the snapshot alone is flagged, live alone is not", () => {
  const ok = (value) => ({ status: "fulfilled", value });
  const failed = { status: "rejected", reason: new Error("release_lookup_failed:403") };
  const live = { ...preview, published_at: "2026-09-26T11:19:29Z" };

  assert.deepEqual(combineReleaseSources(failed, failed), { releases: null, snapshotOnly: false });
  assert.deepEqual(combineReleaseSources(failed, ok([preview])), { releases: [preview], snapshotOnly: true });
  assert.deepEqual(combineReleaseSources(ok([live]), failed), { releases: [live], snapshotOnly: false });
  assert.deepEqual(combineReleaseSources(ok([live]), ok([preview])), { releases: [live], snapshotOnly: false });
  // An empty but successful snapshot is not a failure of both sources.
  assert.deepEqual(combineReleaseSources(failed, ok([])), { releases: [], snapshotOnly: true });
});

test("a release without a publication date sorts last instead of breaking the order", () => {
  const dated = { ...preview, published_at: "2026-09-16T08:33:24Z" };
  const undated = { ...preview, tag_name: "mcp-preview-0.9.0", published_at: null };
  assert.deepEqual(mergeReleases([undated, dated], []).map((release) => release.tag_name), ["mcp-preview-0.2.0", "mcp-preview-0.9.0"]);
});

test("release catalogue requires immutable package and checksum names together", () => {
  assert.equal(assetName("mcp-preview-0.2.0", "windows-x64"), "bridge-tally-mcp-preview-0.2.0-windows-x64.mcpb");
  assert.deepEqual(releaseAssets(preview, "windows-x64"), {
    bundle: preview.assets[0],
    checksum: preview.assets[1],
  });
  assert.equal(releaseAssets({ ...preview, assets: preview.assets.slice(0, 2) }, "macos-arm64"), undefined);
});

test("user-facing site text does not call a release a preview or say it is for evaluation only", async () => {
  const { readFile } = await import("node:fs/promises");
  for (const file of ["index.html", "app.mjs"]) {
    const text = await readFile(new URL(`../site/${file}`, import.meta.url), "utf8");
    // The tag prefix is how a user finds the release; it is a name, not prose.
    const prose = text.replaceAll("mcp-preview", "");
    assert.doesNotMatch(prose, /\bpreviews?\b/i, `${file} calls a release a preview`);
    assert.doesNotMatch(prose, /for evaluation/i, `${file} says a release is for evaluation only`);
  }
  assert.doesNotMatch(releaseLabel({ tag_name: "mcp-preview-9.9.9" }).replace("mcp-preview", ""), /\bpreview\b|for evaluation/i);
});
