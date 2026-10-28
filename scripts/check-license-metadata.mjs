// SPDX-License-Identifier: Apache-2.0

import { readFile } from "node:fs/promises";

const expectedLicense = "Apache-2.0";
const read = (path) => readFile(new URL(`../${path}`, import.meta.url), "utf8");

const [packageText, cargo, tauriText, license, notice, readme, frontendNotices, rustNotices, mcpbText, cargoLock] =
  await Promise.all([
    read("package.json"),
    read("src-tauri/Cargo.toml"),
    read("src-tauri/tauri.conf.json"),
    read("LICENSE"),
    read("NOTICE"),
    read("README.md"),
    read("THIRD_PARTY_LICENSES.txt"),
    read("THIRD_PARTY_LICENSES_RUST.txt"),
    read("packaging/mcpb/manifest.json"),
    read("src-tauri/Cargo.lock"),
  ]);

const packageJson = JSON.parse(packageText);
const tauri = JSON.parse(tauriText);
const failures = [];
const cargoVersion = cargo.match(/^version = "([^"]+)"$/m)?.[1];

if (packageJson.license !== expectedLicense) failures.push("package.json license");
if (!/^license = "Apache-2\.0"$/m.test(cargo)) failures.push("Cargo.toml license");
if (tauri.bundle?.license !== expectedLicense) failures.push("Tauri bundle license");
if (tauri.bundle?.licenseFile !== "../LICENSE") failures.push("Tauri licenseFile");
// The release admission check reads package.json and the MCPB manifest, and
// the bundle smoke check compares the server's CARGO_PKG_VERSION with the
// manifest; scripts/next-version.mjs writes all five together.
const lockVersion = cargoLock.match(/\[\[package\]\]\nname = "bridge"\nversion = "([^"]+)"\n/)?.[1];
const mcpbVersion = JSON.parse(mcpbText).version;
if (!cargoVersion || [cargoVersion, tauri.version, mcpbVersion, lockVersion].some((version) => version !== packageJson.version)) {
  failures.push("manifest version consistency");
}
if (packageJson.version === "0.1.0") failures.push("MIT v0.1.0 version boundary");

const resources = tauri.bundle?.resources ?? {};
for (const [source, destination] of Object.entries({
  "../LICENSE": "LICENSE",
  "../NOTICE": "NOTICE",
  "../THIRD_PARTY_LICENSES.txt": "THIRD_PARTY_LICENSES.txt",
  "../THIRD_PARTY_LICENSES_RUST.txt": "THIRD_PARTY_LICENSES_RUST.txt",
})) {
  if (resources[source] !== destination) failures.push(`Tauri resource ${source}`);
}

if (!license.includes("Apache License") || !license.includes("Grant of Patent License")) {
  failures.push("complete Apache-2.0 LICENSE");
}
if (
  !notice.includes("SQLCipher") ||
  !notice.includes("Copyright (c) 2008-2020 Zetetic LLC") ||
  !notice.includes("LOSS OF USE, DATA, OR PROFITS")
) {
  failures.push("complete bundled SQLCipher NOTICE attribution");
}
// Every build for a target other than macOS compiles OpenSSL from source through SQLCipher (#2049). NOTICE states its
// version, licence and the holders its files name, in one section, so each openssl-src release in the lockfile must be
// read against that section. Every lockfile entry is checked, not the first. The section is read alone: the Apache phrase
// and the holder list are looked for inside it, and the holder list must still be there (a floor of ten names; it holds
// thirty today).
const opensslSrcLocked = [...cargoLock.matchAll(/\[\[package\]\]\nname = "openssl-src"\nversion = "([^"]+)"\n/g)].map((match) => match[1]);
const opensslSection = notice.match(/^OpenSSL\nCopyright 1995-\d{4} The OpenSSL Project Authors\n[\s\S]*?(?=\n-{20,}\n|(?![\s\S]))/m)?.[0] ?? "";
const opensslHolders = (opensslSection.match(/^ {2}\S/gm) ?? []).length;
const opensslProse = opensslSection.replace(/\s+/g, " ");
if (
  opensslSrcLocked.length === 0 ||
  !opensslSrcLocked.every((version) => opensslProse.includes(`OpenSSL ${version.split("+")[1]} from source`)) ||
  !opensslProse.includes("Apache License, Version 2.0") ||
  opensslHolders < 10
) {
  failures.push("bundled OpenSSL NOTICE attribution");
}
if (!readme.includes("Apache License, Version 2.0")) failures.push("README license declaration");
if (!frontendNotices.includes("lucide-react") || !/\breact \d+\.\d+\.\d+/.test(frontendNotices)) {
  failures.push("frontend third-party notices");
}
if (!rustNotices.includes("Mozilla Public License 2.0")) {
  failures.push("native third-party report");
}
// OpenSSL (Apache-2.0) is built from the openssl-src crate, whose own licence is the wrapper's "MIT OR Apache-2.0".
// src-tauri/about.toml clarifies the entry, but cargo-about only warns when the clarification no longer applies and then
// prints the wrapper's licence, so the committed report is checked for every locked version (#2049).
if (opensslSrcLocked.length === 0 || !opensslSrcLocked.every((version) => rustNotices.includes(`\nopenssl-src ${version}\nLicense: Apache-2.0\n`))) {
  failures.push("native third-party report licence for openssl-src");
}

const portableFiles = [license, notice, readme, frontendNotices, rustNotices];
const localPath = /(?:[A-Za-z]:\\(?:Users|dev)\\|\/Users\/[^/]+\/)/i;
if (portableFiles.some((content) => localPath.test(content))) failures.push("local absolute path");

if (failures.length) {
  throw new Error(`License validation failed: ${failures.join(", ")}`);
}

console.log("Apache-2.0 metadata, notices, and bundle resources are consistent.");
