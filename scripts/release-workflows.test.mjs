import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { parseDocument } from "yaml";

const defaultBranchGuard = "github.ref == format('refs/heads/{0}', github.event.repository.default_branch)";

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
  assertWorkflowDispatch(page, "install page workflow");
  const deploy = page.jobs.deploy;
  assert.ok(deploy, "install page deployment job must be present");
  assert.equal(deploy.if, defaultBranchGuard, "Pages deployment must be gated to the default branch");
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
function runSnapshotStep(run, releases) {
  const dir = mkdtempSync(join(tmpdir(), "bridge-snapshot-"));
  mkdirSync(join(dir, "site"));
  writeFileSync(join(dir, "fixture.json"), JSON.stringify(releases));
  const script = `gh() { [ "$1" = api ] && [ "$3" = --jq ] || exit 97; jq "$4" fixture.json; }\n${run}`;
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
  const release = (tag_name, draft) => ({ tag_name, draft, prerelease: true, published_at: "2026-09-26T11:19:29Z", body: "x", assets: [{ name: `${tag_name}.mcpb`, browser_download_url: `https://example.invalid/${tag_name}`, size: 1 }] });

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
