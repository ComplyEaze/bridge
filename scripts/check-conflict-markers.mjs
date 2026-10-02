// SPDX-License-Identifier: Apache-2.0
//
// Fails when a tracked text file contains a merge-conflict marker at the start of a line: `<<<<<<<`,
// diff3's `|||||||`, `>>>>>>>` (each alone or followed by a space and a label) or `=======` alone. A
// resolution that keeps a marker is committed by an ordinary merge, passes every other check, and ships
// as text (it reached a changelog and the agent README once). Reads the working-tree copy of each tracked
// file, so run it on a clean checkout (CI does). A line of exactly seven `=` that is a heading underline must be reworded.
//
//   node scripts/check-conflict-markers.mjs [--root DIR]
//
// Exit 0: none. Exit 1: names each file and line. Exit 2: could not read the repository (never "none").
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

export const MARKER = /^(?:<<<<<<<|\|\|\|\|\|\|\||>>>>>>>)(?: .*)?$|^=======$/;

export function markerLines(text) {
  return text.split("\n").flatMap((line, index) => (MARKER.test(line.replace(/\r$/, "")) ? [index + 1] : []));
}

export function findMarkers(root) {
  const listed = spawnSync("git", ["-C", root, "ls-files", "-z"], { encoding: "utf8", maxBuffer: 256 * 1024 * 1024 });
  if (listed.error || listed.status !== 0) throw new Error(`git ls-files failed: ${listed.error?.message ?? listed.stderr.trim()}`);
  const found = new Set(); // a repository mid-merge lists an unmerged path once per stage
  for (const file of listed.stdout.split("\0").filter(Boolean)) {
    let data;
    try {
      data = readFileSync(resolve(root, file));
    } catch (error) {
      // Only a tracked path that is not a file here (a submodule, a dangling link) is skipped; any other read
      // failure is an error, never "nothing found".
      if (error.code === "ENOENT" || error.code === "EISDIR") continue;
      throw error;
    }
    if (data.includes(0)) continue; // binary and UTF-16 files are not text a marker could be read in
    for (const line of markerLines(data.toString("utf8"))) found.add(`${file}:${line}`);
  }
  return [...found];
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const rootIndex = process.argv.indexOf("--root");
  const root = rootIndex === -1 ? process.cwd() : resolve(process.argv[rootIndex + 1] ?? "");
  try {
    const found = findMarkers(root);
    if (found.length) {
      console.error(`Merge-conflict markers are committed:\n${found.map((entry) => `  ${entry}`).join("\n")}`);
      process.exit(1);
    }
    console.log("No merge-conflict markers in tracked text files.");
  } catch (error) {
    console.error(`could not check for conflict markers: ${error.message}`);
    process.exit(2);
  }
}
