import { isInstallablePreview, releaseAssets, selectRelease } from "./release-catalog.mjs";
import { loadReleases, releasePageUrl, versionLabel } from "./release-source.mjs";

var PLATFORM_NAMES = {
  "windows-x64": "Windows\u00a0x64",
  "macos-arm64": "Apple\u00a0Silicon Mac",
};

function el(tag, className, text) {
  var node = document.createElement(tag);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

// d MMM yyyy everywhere on the site: en-IN's "short" month can
// render "Sept", so it is corrected to the 3-letter form after formatting.
function formatDate(iso) {
  if (!iso) return null;
  var d = new Date(iso);
  if (isNaN(d.getTime())) return null;
  return d
    .toLocaleDateString("en-IN", { year: "numeric", month: "short", day: "numeric" })
    .replace("Sept", "Sep");
}

function assetRow(release, platformKey) {
  var assets = releaseAssets(release, platformKey);
  var row = el("div", "rel-asset");
  var name = el("span", "rel-asset__name", PLATFORM_NAMES[platformKey]);
  row.appendChild(name);
  if (!assets) {
    row.appendChild(el("span", "rel-asset__missing", "not built for this release"));
    return row;
  }
  var link = el("a", null, "Download");
  link.href = assets.bundle.browser_download_url;
  link.setAttribute("aria-label", "Download " + versionLabel(release) + " for " + PLATFORM_NAMES[platformKey]);
  var checksum = el("a", "rel-asset__sha", ".sha256");
  checksum.href = assets.checksum.browser_download_url;
  checksum.setAttribute("aria-label", ".sha256 checksum of " + versionLabel(release) + " for " + PLATFORM_NAMES[platformKey]);
  row.appendChild(link);
  row.appendChild(checksum);
  return row;
}

function releaseCard(release, isLatest) {
  var card = el("article", "rel-card" + (isLatest ? " rel-card--latest" : ""));
  var head = el("div", "rel-card__head");
  var h2 = el("h2");
  h2.appendChild(el("span", "version", versionLabel(release)));
  head.appendChild(h2);

  if (isLatest) head.appendChild(el("span", "rel-tag rel-tag--latest", "Latest"));
  if (isInstallablePreview(release)) {
    head.appendChild(el("span", "rel-tag", "Not yet code-signed"));
  } else if (release.draft) {
    head.appendChild(el("span", "rel-tag rel-tag--muted", "Draft"));
  } else if (release.prerelease) {
    head.appendChild(el("span", "rel-tag rel-tag--muted", "Pre-release"));
  }
  card.appendChild(head);

  var date = formatDate(release.published_at);
  if (date) card.appendChild(el("p", "rel-date", date));

  // A single primary button on the newest card only: it is
  // one clear action, not a second copy of the platform-detection on
  // download.html.
  if (isLatest && isInstallablePreview(release)) {
    var primaryCta = el("p", "rel-primary-cta");
    var ctaLink = el("a", "btn btn-primary", "Get the latest build");
    ctaLink.href = "download.html";
    primaryCta.appendChild(ctaLink);
    card.appendChild(primaryCta);
  }

  var hasWindows = !!releaseAssets(release, "windows-x64");
  var hasMac = !!releaseAssets(release, "macos-arm64");
  if (!isLatest) {
    // An earlier build is history, not an offer: its settings and checks differ from the latest
    // (0.2.0 had posting on by default), so it links to its notes and carries no download.
    card.appendChild(el("p", "rel-one-line", hasWindows || hasMac ? "Superseded by the latest release." : "Source only. No installer for this release."));
  } else if (!hasWindows && !hasMac) {
    // Collapse two "not built for this release" rows into one honest line
    // instead of repeating the same note per platform.
    card.appendChild(el("p", "rel-one-line", "Source only. No installer for this release."));
  } else {
    var assets = el("div", "rel-assets");
    assets.appendChild(assetRow(release, "windows-x64"));
    assets.appendChild(assetRow(release, "macos-arm64"));
    card.appendChild(assets);
  }

  var notesLink = el("a", "rel-notes", "Full release notes on GitHub");
  notesLink.href = releasePageUrl(release);
  notesLink.setAttribute("aria-label", "Full release notes for " + versionLabel(release) + " on GitHub");
  notesLink.target = "_blank";
  notesLink.rel = "noopener noreferrer";
  card.appendChild(notesLink);

  return card;
}

async function main() {
  var loading = document.getElementById("relLoading");
  var errorState = document.getElementById("relError");
  var list = document.getElementById("relList");

  try {
    var combined = await loadReleases();
    if (!combined.releases) throw new Error("neither the GitHub API nor the saved release list answered");
    var releases = combined.releases;
    if (!releases.length) throw new Error("no releases returned");

    if (combined.snapshotOnly) {
      list.appendChild(
        el("p", "rel-state", "GitHub did not answer, so this is the list saved when this page was published. A newer release may exist on GitHub.")
      );
    }
    // "Latest" is the build the download page offers, by the same predicate,
    // not merely the newest entry (which could be a source-only tag)
    var latest = selectRelease(releases);
    releases.forEach(function (release) {
      list.appendChild(releaseCard(release, release === latest));
    });

    loading.hidden = true;
    list.hidden = false;
  } catch (err) {
    loading.hidden = true;
    errorState.hidden = false;
  }
}

main();
