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
  assert.equal(publish.needs, "package", "publication must wait for both package archives");
  assert.equal(publish.permissions.contents, "write", "only publication receives contents write permission");

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
