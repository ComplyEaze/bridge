export const repository = "ComplyEaze/bridge";
const platforms = ["windows-x64", "macos-arm64"];

// A preview is identified by its tag and its complete asset set, not by GitHub's pre-release
// flag: clearing that flag is what lets a preview be marked "Latest", and it must stay
// installable from this page when that happens.
export function isInstallablePreview(release) {
  return !release.draft
    && /^mcp-preview-[0-9]+\.[0-9]+\.[0-9]+([-.][0-9A-Za-z]+)*$/.test(release.tag_name)
    && platforms.every((platform) => releaseAssets(release, platform));
}

export function selectRelease(releases) {
  return releases.find(isInstallablePreview);
}

// Joins the live GitHub list with the snapshot published beside this page, newest first. A tag
// present in both keeps the live entry, since a release can gain assets after the snapshot.
export function mergeReleases(live, snapshot) {
  const byTag = new Map();
  for (const release of [...live, ...snapshot]) {
    if (!byTag.has(release.tag_name)) byTag.set(release.tag_name, release);
  }
  const published = (release) => Date.parse(release.published_at ?? "1970-01-01T00:00:00Z") || 0;
  return [...byTag.values()].sort((a, b) => published(b) - published(a));
}

// Takes the two Promise.allSettled results the page gets and says what it can show: the merged
// list, and whether it came only from the snapshot. `releases` is null when neither source answered.
export function combineReleaseSources(live, snapshot) {
  if (live.status === "rejected" && snapshot.status === "rejected") {
    return { releases: null, snapshotOnly: false };
  }
  return {
    releases: mergeReleases(
      live.status === "fulfilled" ? live.value : [],
      snapshot.status === "fulfilled" ? snapshot.value : [],
    ),
    snapshotOnly: live.status === "rejected",
  };
}

export function assetName(tag, platform) {
  return `bridge-tally-${tag}-${platform}.mcpb`;
}

export function releaseAssets(release, platform) {
  const name = assetName(release.tag_name, platform);
  const bundle = release.assets.find((asset) => asset.name === name);
  const checksum = release.assets.find((asset) => asset.name === `${name}.sha256`);
  if (!bundle || !checksum) return undefined;
  return { bundle, checksum };
}

export function releaseLabel(release) {
  return `${release.tag_name} (unsigned preview)`;
}
