import { combineReleaseSources, releaseAssets, releaseLabel, repository, selectRelease } from "./release-catalog.mjs";

const status = document.querySelector("#release-status");
const channelNote = document.querySelector("#channel-note");
const port = document.querySelector("#tally-port");
const portGuidance = document.querySelector("#port-guidance");
let releases = [];

function setDownload(option, asset) {
  if (!asset) {
    option.removeAttribute("href");
    option.setAttribute("aria-disabled", "true");
    option.querySelector("small").textContent = "No matching release asset is published yet.";
    return;
  }
  option.href = asset.bundle.browser_download_url;
  option.removeAttribute("aria-disabled");
  option.querySelector("small").textContent = `Download ${asset.bundle.name}; verify ${asset.checksum.name}.`;
}

function renderRelease() {
  const release = selectRelease(releases);
  if (!release) {
    status.textContent = "No installable release is published yet.";
    channelNote.textContent = "See all releases for release notes and availability.";
    document.querySelectorAll(".download-option").forEach((option) => setDownload(option));
    return;
  }
  status.textContent = releaseLabel(release);
  channelNote.textContent = "Not yet code-signed; your computer may warn you before opening it. Compare the checksum with the .sha256 file on the release.";
  document.querySelectorAll(".download-option").forEach((option) => {
    setDownload(option, releaseAssets(release, option.dataset.platform));
  });
}

async function fetchReleaseList(url, init) {
  const response = await fetch(url, init);
  if (!response.ok) throw new Error(`release_lookup_failed:${response.status}`);
  const list = await response.json();
  if (!Array.isArray(list)) throw new Error("release_lookup_malformed");
  return list;
}

// The live GitHub API allows 60 unauthenticated requests an hour per address, so the page also
// reads the snapshot the deploy job saved beside it. Either source alone is enough to offer a
// download; a failure of one is never shown as "no release exists".
async function loadReleases() {
  const [live, snapshot] = await Promise.allSettled([
    fetchReleaseList(`https://api.github.com/repos/${repository}/releases?per_page=100`, {
      headers: { Accept: "application/vnd.github+json" },
    }),
    fetchReleaseList("./releases.json", { cache: "no-cache" }),
  ]);
  const combined = combineReleaseSources(live, snapshot);
  if (!combined.releases) {
    status.textContent = "Release details could not be loaded. Use All releases to choose a download.";
    channelNote.textContent = "The download list is unavailable until the GitHub release service responds.";
    return;
  }
  releases = combined.releases;
  renderRelease();
  if (combined.snapshotOnly && selectRelease(releases)) {
    channelNote.textContent = "Showing the release list saved when this page was published; a newer release may be listed under All releases. Compare the checksum with the .sha256 file before opening it.";
  }
}

port.addEventListener("input", () => {
  const value = Number(port.value);
  const valid = Number.isInteger(value) && value >= 1 && value <= 65535;
  port.setAttribute("aria-invalid", String(!valid));
  portGuidance.innerHTML = valid
    ? `When installing, enter <strong>${value}</strong> as Your Tally port in Claude Desktop. This page does not save it or change Tally.`
    : "Enter the local Tally HTTP gateway port, from 1 to 65535.";
});

loadReleases();
