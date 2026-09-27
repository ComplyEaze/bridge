export const repository = "lamemustafa/bridge";
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
  return [...byTag.values()].sort((a, b) => Date.parse(b.published_at ?? 0) - Date.parse(a.published_at ?? 0));
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
