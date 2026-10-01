import { selectRelease, releaseAssets } from "./release-catalog.mjs";
import { loadReleases, versionLabel } from "./release-source.mjs";

var PLATFORMS = {
  "windows-x64": {
    name: "Windows&nbsp;x64",
    label: "Windows",
    note: "Windows x64.", // A1 facts F2; "Windows 10 or 11" is not a verified fact
  },
  "macos-arm64": {
    name: "Mac (Apple&nbsp;Silicon)",
    label: "Mac",
    note: "Apple&nbsp;Silicon only. Intel Macs are not qualified for this release.", // A1 facts F2: "not qualified"
  },
};

function detectPlatform() {
  var ua = navigator.userAgent || "";
  var platform = navigator.platform || "";
  if (/Mac/i.test(platform) || /Macintosh/i.test(ua)) return "macos-arm64";
  if (/Win/i.test(platform) || /Windows/i.test(ua)) return "windows-x64";
  return null;
}

function el(tag, className, html) {
  var node = document.createElement(tag);
  if (className) node.className = className;
  if (html !== undefined) node.innerHTML = html;
  return node;
}

function platformCard(release, platformKey, kind) {
  var info = PLATFORMS[platformKey];
  var assets = releaseAssets(release, platformKey);
  var card = el("div", "dl-card dl-card--" + kind);

  // No "Recommended:" prefix (critique #33): the browser's Mac/Windows guess
  // cannot tell Apple Silicon from Intel, so a confident label on the wrong
  // build would be worse than none. The primary card's border and shadow
  // (.dl-card--primary) are the only emphasis.
  var heading = el("h3", null, info.name);
  card.appendChild(heading);

  if (!assets) {
    card.appendChild(el("p", "dl-note", "No build for this platform in the latest release."));
    return card;
  }

  var button = el(
    "a",
    "btn " + (kind === "primary" ? "btn-primary" : "btn-secondary"),
    "Download for " + info.label
  );
  button.href = assets.bundle.browser_download_url;
  card.appendChild(button);

  var meta = el("p", "dl-meta");
  // the tag alone, not releaseLabel(): the site never labels a release with the word the catalog adds (L1 legal pages v1.0)
  var tag = el("span", "ver", versionLabel(release));
  var sep = document.createTextNode(" · ");
  var checksumLink = el("a", null, ".sha256 checksum");
  checksumLink.href = assets.checksum.browser_download_url;
  meta.appendChild(tag);
  meta.appendChild(sep);
  meta.appendChild(checksumLink);
  card.appendChild(meta);

  card.appendChild(el("p", "dl-note", info.note));
  // beside every download, and only this (L1 legal pages v1.0)
  card.appendChild(el("p", "dl-note", "Not yet code-signed; your computer may warn you."));
  return card;
}

async function main() {
  var loading = document.getElementById("dlLoading");
  var errorState = document.getElementById("dlError");
  var ready = document.getElementById("dlReady");

  try {
    var combined = await loadReleases();
    if (!combined.releases) throw new Error("neither the GitHub API nor the saved release list answered");
    var release = selectRelease(combined.releases);
    if (!release) {
      // an answer that lists no installable build is not a network failure
      errorState.querySelector("p").textContent = "No installable build is published yet.";
      loading.hidden = true;
      errorState.hidden = false;
      return;
    }

    var detected = detectPlatform();
    var order = detected === "macos-arm64" ? ["macos-arm64", "windows-x64"] : ["windows-x64", "macos-arm64"];

    if (!detected) {
      ready.appendChild(
        el(
          "p",
          "dl-note dl-note--unknown",
          "We could not detect your platform from the browser. Choose the build you need below."
        )
      );
    }

    ready.appendChild(platformCard(release, order[0], "primary"));
    ready.appendChild(platformCard(release, order[1], "secondary"));
    if (combined.snapshotOnly) {
      ready.appendChild(
        el(
          "p",
          "dl-note",
          "Showing the release list saved when this page was published; a newer release may be listed under Releases."
        )
      );
    }

    loading.hidden = true;
    ready.hidden = false;
  } catch (err) {
    loading.hidden = true;
    errorState.hidden = false;
  }
}

main();
