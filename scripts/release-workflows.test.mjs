import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { parseDocument } from "yaml";

const defaultBranchGuard = "github.ref == format('refs/heads/{0}', github.event.repository.default_branch)";
// The install page also redeploys after a successful preview publication, from the default branch only.
const installPageGuard = `${defaultBranchGuard} && (github.event_name == 'workflow_dispatch' || (github.event.workflow_run.conclusion == 'success' && github.event.workflow_run.event == 'workflow_dispatch' && github.event.workflow_run.head_repository.full_name == github.repository && github.event.workflow_run.head_branch == github.event.repository.default_branch))`;

async function workflow(path) {
  const source = await readFile(new URL(path, import.meta.url), "utf8");
  const document = parseDocument(source);
  assert.equal(document.errors.length, 0, `${path} must be valid YAML`);
  return document.toJS();
}

function assertWorkflowDispatch(workflow, label) {
  assert.ok(Object.hasOwn(workflow, "on"), `${label} must declare a trigger`);
  assert.deepEqual(Object.keys(workflow.on), ["workflow_dispatch"], `${label} must be manual only`);
}

function assertUnconditional(node, label) {
  assert.equal(node.if, undefined, `${label} must not be conditional`);
  assert.equal(node["continue-on-error"], undefined, `${label} must not continue after failure`);
}

function step(job, name) {
  const found = job.steps.find((candidate) => candidate.name === name);
  assert.ok(found, `missing ${name} step`);
  return found;
}

function assertReleaseWorkflow(release) {
  assertWorkflowDispatch(release, "preview release workflow");
  const admission = release.jobs["release-admission"];
  const packageJob = release.jobs.package;
  const publish = release.jobs["publish-preview"];
  assert.ok(admission && packageJob && publish, "release jobs must be present");
  assertUnconditional(admission, "release admission job");
  assertUnconditional(packageJob, "package job");
  assertUnconditional(publish, "publish job");
  assert.equal(packageJob.needs, "release-admission", "package must wait for release admission");
  assert.equal(publish.permissions.contents, "write", "only publication receives contents write permission");
  // Build attestations. The package jobs run dependency build scripts, so they stay read-only. Only
  // the attest job, which downloads the two archives and runs no repository code, may mint the
  // identity token and write attestations; publication may only read them.
  assert.deepEqual(release.permissions, { contents: "read" }, "the workflow default stays read-only");
  assert.deepEqual(packageJob.permissions, { contents: "read" }, "the jobs that build hold no token-minting permission");
  const attestJob = release.jobs.attest;
  assert.ok(attestJob, "an attest job must be present");
  assertUnconditional(attestJob, "attest job");
  assert.equal(attestJob.needs, "package", "attestation waits for both package archives");
  assert.deepEqual(
    attestJob.permissions,
    { contents: "read", "id-token": "write", attestations: "write" },
    "the attest job alone may create attestations",
  );
  assert.deepEqual(publish.needs, ["package", "attest"], "publication waits for the archives and their attestations");
  assert.deepEqual(publish.permissions, { contents: "write", attestations: "read" }, "publication may only read attestations");
  assert.equal(admission.permissions, undefined, "admission keeps the default read-only permissions");
  assert.equal(packageJob.steps.some((candidate) => candidate.uses?.startsWith("actions/attest@")), false, "no attestation is made where dependency build scripts run");
  // The attest job runs no repository code: no checkout, no install, no build, only pinned actions and one script.
  assert.deepEqual(
    attestJob.steps.map((candidate) => candidate.uses?.replace(/@[0-9a-f]{40}.*/, "@sha") ?? candidate.name),
    ["actions/download-artifact@sha", "Check each archive against the digest its package job recorded", "actions/attest@sha"],
    "the attest job downloads, checks digests and attests, and does nothing else",
  );
  const [download, check, attest] = attestJob.steps;
  for (const candidate of attestJob.steps) {
    assertUnconditional(candidate, "attest job step");
    assert.equal(candidate["continue-on-error"], undefined, "no archive is published without its attestation");
  }
  assert.match(download.uses, /^actions\/download-artifact@[0-9a-f]{40}$/);
  assert.deepEqual(download.with, { pattern: "mcpb-preview-*", path: "release-assets", "merge-multiple": true });
  assert.deepEqual(Object.keys(check).sort(), ["env", "name", "run", "shell"], "the digest check sets nothing else");
  assert.deepEqual(check.env, { RELEASE_TAG: "${{ inputs.release_tag }}" });
  // The digest check is pinned line by line: an early `exit 0`, a single platform, or a digest
  // taken from the wrong place would each leave the job green and the check switched off.
  assert.deepEqual(
    check.run.trim().split("\n").map((line) => line.trim()),
    [
      "set -euo pipefail",
      "for platform in windows-x64 macos-arm64; do",
      'asset="release-assets/bridge-tally-${RELEASE_TAG}-${platform}.mcpb"',
      'test -f "$asset"',
      `actual="$(sha256sum "$asset" | cut -d' ' -f1)"`,
      `listed="$(cut -d' ' -f1 "$asset.sha256")"`,
      'recorded="$(jq -r .sha256 "$asset.provenance.json")"',
      'source_sha="$(jq -r .source_sha "$asset.provenance.json")"',
      'if [[ "$actual" != "$listed" || "$actual" != "$recorded" ]]; then',
      'echo "digest mismatch for $asset: file $actual, .sha256 $listed, provenance $recorded"',
      "exit 1",
      "fi",
      'if [[ "$source_sha" != "$GITHUB_SHA" ]]; then',
      'echo "provenance names commit $source_sha, not this run\'s $GITHUB_SHA"',
      "exit 1",
      "fi",
      "done",
    ],
  );
  assert.deepEqual(
    Object.keys(attestJob).sort(),
    ["name", "needs", "permissions", "runs-on", "steps", "timeout-minutes"],
    "the attest job sets no env, container, strategy, defaults or services",
  );
  assert.equal(attestJob["runs-on"], "ubuntu-latest", "the attest job runs on a GitHub-hosted runner");
  assert.match(attest.uses, /^actions\/attest@[0-9a-f]{40}$/, "the attestation action is pinned by full commit SHA");
  assert.deepEqual(Object.keys(attest).sort(), ["name", "uses", "with"], "the attestation step sets nothing else");
  assert.deepEqual(Object.keys(attest.with), ["subject-path"], "no other attestation input is set");
  assert.deepEqual(
    attest.with["subject-path"].trim().split("\n"),
    [
      "release-assets/bridge-tally-${{ inputs.release_tag }}-windows-x64.mcpb",
      "release-assets/bridge-tally-${{ inputs.release_tag }}-macos-arm64.mcpb",
    ],
    "the attested files are exactly the two archives that publication verifies",
  );
  const verify = step(publish, "Verify each archive against its build attestation");
  assertUnconditional(verify, "attestation verification step");
  assert.equal(verify["continue-on-error"], undefined);
  // The whole script is pinned line by line: a trailing `|| true`, a commented-out command, a
  // third platform or a different asset name would each turn the gate off or point it elsewhere.
  assert.deepEqual(
    verify.run.trim().split("\n").map((line) => line.trim()),
    [
      "set -euo pipefail",
      "for platform in windows-x64 macos-arm64; do",
      'asset="release-assets/bridge-tally-${RELEASE_TAG}-${platform}.mcpb"',
      'gh attestation verify "$asset" --repo "$REPOSITORY" --signer-workflow "$REPOSITORY/.github/workflows/release-mcpb-preview.yml" --source-digest "$SOURCE_SHA" --source-ref "refs/heads/$DEFAULT_BRANCH" --deny-self-hosted-runners',
      "done",
    ],
  );
  assert.deepEqual(Object.keys(verify).sort(), ["env", "name", "run", "shell"], "the verification step sets nothing else");
  assert.equal(verify.shell, "bash");
  assert.deepEqual(verify.env, {
    GH_TOKEN: "${{ github.token }}",
    RELEASE_TAG: "${{ inputs.release_tag }}",
    REPOSITORY: "${{ github.repository }}",
    SOURCE_SHA: "${{ github.sha }}",
    DEFAULT_BRANCH: "${{ github.event.repository.default_branch }}",
  });
  const publishNames = publish.steps.map((candidate) => candidate.name ?? candidate.uses);
  assert.ok(
    publishNames.indexOf(verify.name) < publishNames.indexOf("Create the immutable GitHub preview release"),
    "archives are verified against their attestations before the release is created",
  );

  const admissionStep = step(admission, "Require reviewed source and a matching immutable preview version");
  assertUnconditional(admissionStep, "release admission step");
  assert.equal(admissionStep.env.SOURCE_REF, "${{ github.ref }}");
  assert.equal(admissionStep.env.DEFAULT_BRANCH, "${{ github.event.repository.default_branch }}");
  assert.match(admissionStep.run, /node scripts\/check-mcpb-preview-admission\.mjs/);

  const windowsSetup = step(packageJob, "Set up Windows native prerequisites");
  assert.equal(windowsSetup.if, "runner.os == 'Windows'");
  assert.equal(windowsSetup["continue-on-error"], undefined, "Windows prerequisites must not continue after failure");
  assert.equal(windowsSetup.uses, "./.github/actions/setup-windows-native");
  const cargoBuild = step(packageJob, "Build the host Bridge MCP binary");
  assertUnconditional(cargoBuild, "Cargo build step");
  assert.match(cargoBuild.run, /cargo build --locked --release --manifest-path src-tauri\/Cargo\.toml --bin bridge_mcp/);
  assert.ok(
    packageJob.steps.indexOf(windowsSetup) < packageJob.steps.indexOf(cargoBuild),
    "Windows prerequisites must run before Cargo",
  );

  const releaseStep = step(publish, "Create the immutable GitHub preview release");
  assertUnconditional(releaseStep, "immutable release step");
  assert.equal(releaseStep.env.GH_REPO, "${{ github.repository }}");
  assert.match(releaseStep.run, /gh release create/);
  assert.match(releaseStep.run, /refusing to replace existing release assets/);
  assert.match(releaseStep.run, /for other in "mcp-preview-\$version" "mcp-v\$version"; do/);
  assert.match(releaseStep.run, /found="\$\(git ls-remote --tags origin "refs\/tags\/\$other"\)"/);
  assert.match(releaseStep.run, /released="\$\(gh release list --limit 1000 --json tagName --jq '\.\[\]\.tagName'\)"/);
  assert.match(releaseStep.run, /refusing to publish: version \$version already has the tag \$other/);
  assert.doesNotMatch(releaseStep.run, /gh release view/, "a release lookup that cannot tell 'not found' from a failure is not used");
  assert.match(releaseStep.run, /git ls-remote --tags origin "refs\/tags\/\$RELEASE_TAG" "refs\/tags\/\$RELEASE_TAG\^\{\}"/);
  assert.match(releaseStep.run, /\$\{peeled_sha:-\$tag_sha\}/);
  assert.match(releaseStep.run, /could not verify whether \$RELEASE_TAG already exists; refusing to publish/);
  assert.match(releaseStep.run, /existing tag \$RELEASE_TAG does not identify \$SOURCE_SHA/);
  assert.match(releaseStep.run, /gh api --method POST "repos\/\$GH_REPO\/git\/refs"/);
  assert.match(releaseStep.run, /--verify-tag/);
  assert.doesNotMatch(releaseStep.run, /--target/);
  assert.match(releaseStep.run, /--prerelease/);
}

function assertInstallPageWorkflow(page) {
  assert.deepEqual(Object.keys(page.on), ["workflow_dispatch", "workflow_run"], "install page workflow must be manual or follow a preview publication");
  assert.deepEqual(page.on.workflow_run, { workflows: ["Publish MCPB preview release"], types: ["completed"] });
  const deploy = page.jobs.deploy;
  assert.ok(deploy, "install page deployment job must be present");
  assert.equal(deploy.if, installPageGuard, "Pages deployment must be gated to the default branch and to a successful publication");
  assert.equal(deploy["continue-on-error"], undefined, "Pages deployment must not continue after failure");
  const upload = deploy.steps.find((candidate) => candidate.uses?.startsWith("actions/upload-pages-artifact@"));
  const publish = deploy.steps.find((candidate) => candidate.uses?.startsWith("actions/deploy-pages@"));
  assert.ok(upload && publish, "Pages deployment must upload and deploy the site artifact");
  assertUnconditional(upload, "Pages upload step");
  assertUnconditional(publish, "Pages deployment step");
  assert.equal(upload.with.path, "site");
  const snapshot = deploy.steps.find((candidate) => candidate.name === "Snapshot releases for the install page");
  assert.ok(snapshot, "the install page must ship a release snapshot for when the GitHub API refuses it");
  assertUnconditional(snapshot, "release snapshot step");
  assert.ok(deploy.steps.indexOf(snapshot) < deploy.steps.indexOf(upload), "the snapshot must be written before the site is uploaded");
  assert.match(snapshot.run, /set -euo pipefail/);
}

// Runs the snapshot step's own shell with `gh` replaced by a function that applies the step's
// real --jq filter to fixture JSON, so the draft filter and the empty guard are exercised, not matched.
// Each release is its own page, as `gh api --paginate` would print them, so the step must collect
// every page into one list.
function runSnapshotStep(run, releases) {
  const dir = mkdtempSync(join(tmpdir(), "bridge-snapshot-"));
  mkdirSync(join(dir, "site"));
  copyFileSync(new URL("../site/release-catalog.mjs", import.meta.url), join(dir, "site", "release-catalog.mjs"));
  const pages = releases.length ? releases.map((release) => [release]) : [[]];
  pages.forEach((page, index) => writeFileSync(join(dir, `page-${String(index).padStart(3, "0")}.json`), JSON.stringify(page)));
  const script = `gh() { [ "$1" = api ] && [ "$2" = --paginate ] && [ "$4" = --jq ] || exit 97; for page in page-*.json; do jq -c "$5" "$page"; done; }\n${run}`;
  const result = spawnSync("bash", ["-c", script], { cwd: dir, encoding: "utf8", env: { ...process.env, REPOSITORY: "example/bridge" } });
  const written = existsSync(join(dir, "site", "releases.json")) ? readFileSync(join(dir, "site", "releases.json"), "utf8") : undefined;
  rmSync(dir, { recursive: true, force: true });
  return { status: result.status, stderr: result.stderr, written };
}

test("the install page snapshot step drops drafts and refuses a list with no mcp-preview release", async (t) => {
  if (spawnSync("jq", ["--version"]).status !== 0) {
    t.skip("jq is not installed on this host; the deploy runner has it");
    return;
  }
  const page = await workflow("../.github/workflows/deploy-install-page.yml");
  const { run } = page.jobs.deploy.steps.find((candidate) => candidate.name === "Snapshot releases for the install page");
  const files = (tag) => ["windows-x64", "macos-arm64"].flatMap((platform) => [`bridge-tally-${tag}-${platform}.mcpb`, `bridge-tally-${tag}-${platform}.mcpb.sha256`]);
  const release = (tag_name, draft, names = files(tag_name)) => ({ tag_name, draft, prerelease: true, published_at: "2026-09-26T11:19:29Z", body: "x", assets: names.map((name) => ({ name, browser_download_url: `https://example.invalid/${name}`, size: 1 })) });

  const ok = runSnapshotStep(run, [release("mcp-preview-0.4.0", true), release("mcp-preview-0.3.0", false), release("v0.1.0", false)]);
  assert.equal(ok.status, 0, ok.stderr);
  const snapshot = JSON.parse(ok.written);
  assert.deepEqual(snapshot.map((entry) => entry.tag_name), ["mcp-preview-0.3.0", "v0.1.0"]);
  assert.deepEqual(Object.keys(snapshot[0]).sort(), ["assets", "draft", "prerelease", "published_at", "tag_name"]);
  assert.deepEqual(Object.keys(snapshot[0].assets[0]).sort(), ["browser_download_url", "name"]);

  const onlyDraftPreview = runSnapshotStep(run, [release("mcp-preview-0.4.0", true), release("v0.1.0", false)]);
  assert.equal(onlyDraftPreview.status, 1);
  assert.match(onlyDraftPreview.stderr, /refusing to deploy/);
  assert.equal(runSnapshotStep(run, []).status, 1);
  // A preview missing one checksum would deploy a page with no download, so it is refused too.
  const incomplete = runSnapshotStep(run, [release("mcp-preview-0.3.0", false, files("mcp-preview-0.3.0").slice(0, 3))]);
  assert.equal(incomplete.status, 1);
  assert.match(incomplete.stderr, /no installable release/);
});

// Runs the summary step's own shell in a shallow clone, as the deploy job's checkout is, with `gh`
// replaced by a function that answers the two Deployments API reads the step makes by applying the
// step's real --jq filters to fixtures. `deployments` is newest first: each has the commit it
// deployed ("self" is the commit being deployed, "first" an earlier one, "unknown" one that cannot
// be fetched) and its statuses, newest first.
function runSiteSummaryStep(run, { deployments, changeSite }) {
  const dir = mkdtempSync(join(tmpdir(), "bridge-site-summary-"));
  const env = { PATH: process.env.PATH, HOME: dir, GIT_CONFIG_GLOBAL: "/dev/null", GIT_CONFIG_NOSYSTEM: "1", GIT_AUTHOR_NAME: "t", GIT_AUTHOR_EMAIL: "t@example.invalid", GIT_COMMITTER_NAME: "t", GIT_COMMITTER_EMAIL: "t@example.invalid" };
  const git = (cwd, ...args) => {
    const result = spawnSync("git", args, { cwd, encoding: "utf8", env });
    assert.equal(result.status, 0, `git ${args.join(" ")}: ${result.stderr}`);
    return result.stdout.trim();
  };
  const origin = join(dir, "origin");
  const seed = join(dir, "seed");
  mkdirSync(seed);
  git(seed, "init", "-q", "-b", "master");
  mkdirSync(join(seed, "site"));
  writeFileSync(join(seed, "site", "index.html"), "<p>one</p>\n");
  git(seed, "add", ".");
  git(seed, "commit", "-q", "-m", "first");
  const first = git(seed, "rev-parse", "HEAD");
  writeFileSync(join(seed, "README.md"), "outside site\n");
  if (changeSite) writeFileSync(join(seed, "site", "index.html"), changeSite);
  git(seed, "add", ".");
  git(seed, "commit", "-q", "-m", "second");
  const second = git(seed, "rev-parse", "HEAD");
  git(dir, "clone", "-q", "--bare", seed, origin);
  git(origin, "config", "uploadpack.allowAnySHA1InWant", "true");
  const work = join(dir, "work");
  git(dir, "clone", "-q", "--depth=1", `file://${origin}`, work);
  const shas = { self: second, first, unknown: "0123456789abcdef0123456789abcdef01234567" };
  writeFileSync(join(dir, "deployments.json"), JSON.stringify(deployments.map((deployment, index) => ({ id: 900 + index, sha: shas[deployment.sha] }))));
  deployments.forEach((deployment, index) => writeFileSync(join(dir, `statuses-${900 + index}.json`), JSON.stringify(deployment.states.map((state) => ({ state })))));
  deployments.forEach((deployment, index) => deployment.failsAfterPrinting && writeFileSync(join(dir, `statuses-${900 + index}.fail`), ""));
  writeFileSync(join(dir, "summary.md"), "");
  // Each item is its own page, as `gh api --paginate` prints them with a per-page --jq, so a step
  // that does not collect every page, or does not pass --paginate, fails here.
  const script = `gh() { [ "$1" = api ] && [ "$2" = --paginate ] && [ "$4" = --jq ] || exit 97; case "$3" in
    repos/example/bridge/deployments\\?environment=github-pages\\&per_page=100) jq -c '.[] | [.]' "${dir}/deployments.json" | while IFS= read -r page; do printf '%s' "$page" | jq -r "$5"; done ;;
    repos/example/bridge/deployments/*/statuses\\?per_page=100) id="\${3#*/deployments/}"; id="\${id%%/*}"; jq -c '.[] | [.]' "${dir}/statuses-\${id}.json" | { while IFS= read -r page; do printf '%s' "$page" | jq -r "$5" || exit 141; sleep 0.01; done; [ ! -e "${dir}/statuses-\${id}.fail" ] || exit 1; } ;;
    *) exit 97 ;;
  esac; }\n${run}`;
  const result = spawnSync("bash", ["-c", script], { cwd: work, encoding: "utf8", env: { ...env, REPOSITORY: "example/bridge", RUNNER_TEMP: dir, GITHUB_SHA: second, GITHUB_STEP_SUMMARY: join(dir, "summary.md") } });
  const summary = readFileSync(join(dir, "summary.md"), "utf8");
  rmSync(dir, { recursive: true, force: true });
  return { status: result.status, stderr: result.stderr, summary };
}

test("the site summary step is informational, permitted to read deployments, and shows what would go live", async (t) => {
  if (spawnSync("jq", ["--version"]).status !== 0) {
    t.skip("jq is not installed on this host; the deploy runner has it");
    return;
  }
  const page = await workflow("../.github/workflows/deploy-install-page.yml");
  assert.equal(page.permissions.deployments, "read");
  const summary = step(page.jobs.deploy, "Summarize site changes since the last deploy");
  assert.equal(summary["continue-on-error"], true, "a failed summary must not stop the deploy");
  assert.equal(typeof summary["timeout-minutes"], "number", "a hung summary must time out inside the step, where continue-on-error applies");
  assert.ok(summary["timeout-minutes"] <= 5);
  assert.equal(summary.if, undefined);
  const names = page.jobs.deploy.steps.map((candidate) => candidate.name ?? candidate.uses);
  assert.ok(names.indexOf(summary.name) < names.findIndex((name) => name?.startsWith("actions/deploy-pages@")), "the summary is written before the deploy");

  // This job's own deployment already exists and lists first; a failed one never went live.
  const own = { sha: "self", states: ["queued", "waiting"] };
  const failed = { sha: "unknown", states: ["failure", "queued"] };
  const live = { sha: "first", states: ["success", "in_progress", "queued"] };

  const changed = runSiteSummaryStep(summary.run, { deployments: [own, failed, live], changeSite: "<p>two</p>\n```\n" });
  assert.equal(changed.status, 0, changed.stderr);
  assert.match(changed.summary, /site\/index\.html/);
  assert.match(changed.summary, /^-<p>one<\/p>$/m);
  assert.match(changed.summary, /^\+<p>two<\/p>$/m);
  assert.doesNotMatch(changed.summary, /outside site|README/, "only site/ is compared, in the file list and the diff");
  assert.match(changed.summary, /^~~~~diff\n[\s\S]*^\+```\n[\s\S]*^~~~~$/m, "a diff line with a code fence stays inside the summary's fence");

  // An earlier successful deployment of this same commit (a re-run) is not "the last deploy": the
  // comparison goes on to the first different commit rather than reporting that nothing changed.
  const rerun = runSiteSummaryStep(summary.run, { deployments: [own, { sha: "self", states: ["success"] }, live], changeSite: "<p>two</p>\n" });
  assert.equal(rerun.status, 0, rerun.stderr);
  assert.match(rerun.summary, /^\+<p>two<\/p>$/m, "a successful deployment of this commit is skipped");

  // Only a deployment whose NEWEST status is success went live. An older success under a newer
  // failure or inactive status (superseded or torn down) is not the last deploy.
  const superseded = { sha: "unknown", states: ["inactive", "success"] };
  const notLive = runSiteSummaryStep(summary.run, { deployments: [own, superseded, { sha: "unknown", states: ["failure", "success"] }, live], changeSite: "<p>two</p>\n" });
  assert.equal(notLive.status, 0, notLive.stderr);
  assert.match(notLive.summary, /^\+<p>two<\/p>$/m, "an older success under a newer status is skipped, not compared");

  // A `gh` that dies after printing must fail the step (pipefail), which only warns, and must not
  // read as "nothing to compare" or compare against an older commit.
  const dies = runSiteSummaryStep(summary.run, { deployments: [own, { sha: "unknown", states: ["success"], failsAfterPrinting: true }, live], changeSite: "<p>two</p>\n" });
  assert.notEqual(dies.status, 0, "a failed statuses read stops the step; continue-on-error turns that into a warning");
  assert.equal(dies.summary, "", "nothing is written from a partial read");

  const unchanged = runSiteSummaryStep(summary.run, { deployments: [own, live] });
  assert.equal(unchanged.status, 0, unchanged.stderr);
  assert.match(unchanged.summary, /Nothing under `site\/` differs from /);
  assert.doesNotMatch(unchanged.summary, /~~~~diff/);

  for (const deployments of [[], [own], [own, failed]]) {
    const none = runSiteSummaryStep(summary.run, { deployments });
    assert.equal(none.status, 0, none.stderr);
    assert.match(none.summary, /No earlier successful deployment of another commit was found/);
  }

  // The last successful deployment can sit far down a long history of failed ones; a first page of
  // any size would miss it and report nothing to compare.
  const buried = runSiteSummaryStep(summary.run, { deployments: [own, ...Array.from({ length: 120 }, () => failed), live], changeSite: "<p>two</p>\n" });
  assert.equal(buried.status, 0, buried.stderr);
  assert.match(buried.summary, /^\+<p>two<\/p>$/m, "a successful deployment past the first 100 is still found");

  const unfetchable = runSiteSummaryStep(summary.run, { deployments: [own, { sha: "unknown", states: ["success"] }] });
  assert.equal(unfetchable.status, 0, unfetchable.stderr);
  assert.match(unfetchable.summary, /could not be fetched, so nothing was compared/);

  const long = runSiteSummaryStep(summary.run, { deployments: [own, live], changeSite: Array.from({ length: 400 }, (_, i) => `<p>line ${i}</p>`).join("\n") + "\n" });
  assert.equal(long.status, 0, long.stderr);
  assert.match(long.summary, /The diff is cut at 300 lines of 400 characters/);
  assert.doesNotMatch(long.summary, /line 399/);

  const wide = runSiteSummaryStep(summary.run, { deployments: [own, live], changeSite: `<p>${"x".repeat(5000)}</p>\n` });
  assert.equal(wide.status, 0, wide.stderr);
  assert.ok(wide.summary.length < 3000, `a 5,000-character line is cut, not copied whole (${wide.summary.length})`);
});

test("publication workflows enforce their parsed trigger, dependency, branch, and platform controls", async () => {
  assertReleaseWorkflow(await workflow("../.github/workflows/release-mcpb-preview.yml"));
  assertInstallPageWorkflow(await workflow("../.github/workflows/deploy-install-page.yml"));
});

test("publication workflow checks reject disabled or misplaced controls", async () => {
  const release = await workflow("../.github/workflows/release-mcpb-preview.yml");
  const disabledAdmission = structuredClone(release);
  disabledAdmission.jobs["release-admission"].steps.find(
    (candidate) => candidate.name === "Require reviewed source and a matching immutable preview version",
  ).if = false;
  assert.throws(() => assertReleaseWorkflow(disabledAdmission), /must not be conditional/);

  const attestContinues = structuredClone(release);
  attestContinues.jobs.attest.steps[2]["continue-on-error"] = true;
  assert.throws(() => assertReleaseWorkflow(attestContinues), /attest job step must not continue after failure/);

  const attestUnpinned = structuredClone(release);
  attestUnpinned.jobs.attest.steps[2].uses = "actions/attest@v4";
  assert.throws(() => assertReleaseWorkflow(attestUnpinned), /the attest job downloads, checks digests and attests/);

  const packageMintsTokens = structuredClone(release);
  packageMintsTokens.jobs.package.permissions["id-token"] = "write";
  assert.throws(() => assertReleaseWorkflow(packageMintsTokens), /jobs that build hold no token-minting permission/);

  const publishMintsTokens = structuredClone(release);
  publishMintsTokens.jobs["publish-preview"].permissions["id-token"] = "write";
  assert.throws(() => assertReleaseWorkflow(publishMintsTokens), /publication may only read attestations/);

  const attestChecksOut = structuredClone(release);
  attestChecksOut.jobs.attest.steps.unshift({ uses: "actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1" });
  assert.throws(() => assertReleaseWorkflow(attestChecksOut), /the attest job downloads, checks digests and attests/);

  const verifyNeverFails = structuredClone(release);
  step(verifyNeverFails.jobs["publish-preview"], "Verify each archive against its build attestation").run += " || true\n";
  assert.throws(() => assertReleaseWorkflow(verifyNeverFails), assert.AssertionError);

  const admissionContinues = structuredClone(release);
  admissionContinues.jobs["release-admission"]["continue-on-error"] = true;
  assert.throws(() => assertReleaseWorkflow(admissionContinues), /must not continue after failure/);

  const misplacedAdmission = structuredClone(release);
  const admissionSteps = misplacedAdmission.jobs["release-admission"].steps;
  const [admissionStep] = admissionSteps.splice(admissionSteps.findIndex(
    (candidate) => candidate.name === "Require reviewed source and a matching immutable preview version",
  ), 1);
  misplacedAdmission.jobs.package.steps.push(admissionStep);
  assert.throws(() => assertReleaseWorkflow(misplacedAdmission), /missing Require reviewed source/);

  const alwaysPackage = structuredClone(release);
  alwaysPackage.jobs.package.if = "${{ always() }}";
  assert.throws(() => assertReleaseWorkflow(alwaysPackage), /package job must not be conditional/);

  const page = await workflow("../.github/workflows/deploy-install-page.yml");
  const disabledPage = structuredClone(page);
  disabledPage.jobs.deploy.if = false;
  assert.throws(() => assertInstallPageWorkflow(disabledPage), /default branch/);

  // A redeploy after a failed or cancelled publication would snapshot a release that is not there.
  const anyConclusion = structuredClone(page);
  anyConclusion.jobs.deploy.if = anyConclusion.jobs.deploy.if.replace("github.event.workflow_run.conclusion == 'success' && ", "");
  assert.notEqual(anyConclusion.jobs.deploy.if, page.jobs.deploy.if);
  assert.throws(() => assertInstallPageWorkflow(anyConclusion), /successful publication/);

  // workflow_run matches by workflow name, so a pull request's same-named workflow or a fork's run must not qualify.
  for (const condition of [
    "github.event.workflow_run.event == 'workflow_dispatch' && ",
    "github.event.workflow_run.head_repository.full_name == github.repository && ",
  ]) {
    const relaxed = structuredClone(page);
    relaxed.jobs.deploy.if = relaxed.jobs.deploy.if.replace(condition, "");
    assert.notEqual(relaxed.jobs.deploy.if, page.jobs.deploy.if, condition);
    assert.throws(() => assertInstallPageWorkflow(relaxed), /successful publication/, condition);
  }
});

test("the MCPB smoke binds initialize serverInfo.version to the archived manifest", async () => {
  const smoke = await readFile(new URL("./check-mcpb-bundle.py", import.meta.url), "utf8");
  assert.match(smoke, /server_version_mismatch/);
  assert.match(smoke, /validate_server_version\(replies\[0\], manifest\)/);
});

test("unsigned preview notes state the host-validation scope and remaining gaps", async () => {
  const notes = await readFile(new URL("../packaging/mcpb/UNSIGNED_PREVIEW_RELEASE.md", import.meta.url), "utf8");
  assert.match(notes, /hosted Windows x64 and Apple Silicon Mac runners/);
  assert.match(notes, /does not establish\nlive Tally behaviour or Claude Desktop conversational tool calls/);
  assert.match(notes, /Native Windows Tally\/Claude Desktop validation remains outstanding/);
  assert.match(notes, /Intel Mac is not qualified/);
  // The attestation line is scoped: where and by what a file was built, never a signature or a safety claim.
  assert.match(notes, /gh attestation verify <file>\.mcpb --repo ComplyEaze\/bridge --signer-workflow\s+ComplyEaze\/bridge\/\.github\/workflows\/release-mcpb-preview\.yml --source-ref\s+refs\/heads\/master --deny-self-hosted-runners/);
  assert.match(notes, /It is not a code\s+signature and does not show the code is safe\./);
});

// The two existence checks of the publish step run against stubs, so "the lookup failed" is shown
// to refuse rather than to read as "not released".
test("the publish step refuses on a failed lookup and on a version already released under the other tag form", async () => {
  const release = await workflow("../.github/workflows/release-mcpb-preview.yml");
  const run = step(release.jobs["publish-preview"], "Create the immutable GitHub preview release").run;
  const start = run.indexOf('released="$(gh release list');
  const end = run.indexOf("resolve_preview_tag_commit() {");
  assert.ok(start > 0 && end > start, "the existence checks are found");
  const checks = run.slice(start, end);
  const attempt = (tag, { gh, git }) => {
    const script = `set -euo pipefail\nRELEASE_TAG=${tag}\ngh(){ ${gh}; }\ngit(){ ${git}; }\n${checks}\necho PASSED\n`;
    const result = spawnSync("bash", ["-c", script], { encoding: "utf8" });
    return { status: result.status, out: result.stdout + result.stderr };
  };
  const ok = { gh: "printf 'mcp-preview-0.2.0\\nmcp-preview-0.3.0\\n'", git: "true" };
  assert.match(attempt("mcp-v0.4.0", ok).out, /PASSED/);
  assert.notEqual(attempt("mcp-v0.4.0", { ...ok, gh: "return 1" }).status, 0, "a failed release listing refuses");
  assert.doesNotMatch(attempt("mcp-v0.4.0", { ...ok, gh: "return 1" }).out, /PASSED/);
  assert.notEqual(attempt("mcp-v0.4.0", { ...ok, git: "return 128" }).status, 0, "a failed tag lookup refuses");
  assert.doesNotMatch(attempt("mcp-v0.4.0", { ...ok, git: "return 128" }).out, /PASSED/);
  assert.match(attempt("mcp-preview-0.3.0", ok).out, /refusing to replace existing release assets/);
  assert.match(attempt("mcp-v0.3.0", ok).out, /refusing to publish: version 0\.3\.0 already has the tag mcp-preview-0\.3\.0/);
  assert.match(attempt("mcp-v0.4.0", { ...ok, git: "printf 'abc\\trefs/tags/mcp-preview-0.4.0\\n'" }).out, /already has the tag mcp-preview-0\.4\.0/);
  // A tag already created for THIS release (the publish step creates it if absent, and checks it
  // points at the source commit later) is not "the other form".
  const ownTagExists = { ...ok, git: "case \"$*\" in *refs/tags/mcp-v0.4.0) printf 'abc\\trefs/tags/mcp-v0.4.0\\n';; esac" };
  assert.match(attempt("mcp-v0.4.0", ownTagExists).out, /PASSED/);
});
