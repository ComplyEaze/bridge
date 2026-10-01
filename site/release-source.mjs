import { combineReleaseSources, repository } from "./release-catalog.mjs";

async function fetchReleaseList(url, init) {
  const response = await fetch(url, init);
  if (!response.ok) throw new Error(`release_lookup_failed:${response.status}`);
  const list = await response.json();
  if (!Array.isArray(list)) throw new Error("release_lookup_malformed");
  return list;
}

// The live GitHub API allows 60 unauthenticated requests an hour per address, so each page also
// reads the snapshot the deploy job saves beside it (releases.json). Either source alone is
// enough; a failure of one is never shown as "no release exists". Same contract as the install
// page this site replaces.
export async function loadReleases() {
  const [live, snapshot] = await Promise.allSettled([
    fetchReleaseList(`https://api.github.com/repos/${repository}/releases?per_page=100`, {
      headers: { Accept: "application/vnd.github+json" },
    }),
    fetchReleaseList("./releases.json", { cache: "no-cache" }),
  ]);
  return combineReleaseSources(live, snapshot);
}

// "Version 0.3.0" for a tag like mcp-preview-0.3.0 or v0.1.0: the tag stays in links and file
// names, but the page never shows the word the tag carries
export function versionLabel(release) {
  return `Version ${release.tag_name.replace(/^mcp-preview-/, "").replace(/^v(?=\d)/, "")}`;
}

// The snapshot keeps only tag, flags, date and asset links, so the release page link is built
// from the tag rather than read from html_url.
export function releasePageUrl(release) {
  return `https://github.com/${repository}/releases/tag/${encodeURIComponent(release.tag_name)}`;
}
