# gpui-binder

`gpui-binder` is a GitHub Actions automation repository for generating a usable GPUI workspace from Zed’s upstream source and binding the external `gpui-component` crate family into that generated workspace.

The repository does not primarily contain application source code. Instead, it contains workflows that:

1. Clone a selected Zed repository and branch.
2. Keep only the crates needed to build GPUI-related packages.
3. Generate a `gpui_facade` crate that re-exports GPUI and related platform crates through one dependency.
4. Import `gpui-component` crates from an external source repository.
5. Rewrite dependency declarations so the imported crates use the local generated workspace.
6. Validate that local Cargo dependencies resolve correctly.
7. Push generated branches back to this repository.
8. Optionally generate nightly branches and commit a branch-local `README.md` after validation.

The default manual pipeline uses:

* GPUI source: `zed-industries/zed`
* GPUI branch: `main`
* Component source: `longbridge/gpui-component`
* Component branch/ref: `main`
* Intermediate branch: `generated-gpui-base`
* Final branch: `generated-gpui-binded`

The default nightly pipeline uses:

* GPUI source: `zed-industries/zed`
* GPUI branch: `main`
* Component source: `longbridge/gpui-component`
* Component branch/ref: `main`
* Intermediate nightly branch: `generated-zed-nightly`
* Final nightly branch: `generated-gpui-nightly`
* Schedule: every day at `17:42` GMT+9 / Asia-Tokyo, which is `08:42` UTC

## Usage Example

### Nightly branch

The nightly branch is generated automatically:

```text
https://github.com/gpui-binder/gpui-binder/tree/generated-gpui-nightly
```

You can depend on the moving nightly branch:

```toml
gpui = { git = "https://github.com/gpui-binder/gpui-binder", package = "gpui_facade", branch = "generated-gpui-nightly", features = ["gpui_macos", "gpui_web", "gpui_macros", "font-kit", "x11", "wayland", "runtime_shaders"] }
gpui-component = { git = "https://github.com/gpui-binder/gpui-binder", package = "gpui-component", branch = "generated-gpui-nightly" }
```

### Pinned commit

For reproducible builds, prefer the pinned commit shown in the generated nightly branch README.

The nightly workflow commits `README.md` separately after the generated source commit. The dependency `rev` should point to the generated source commit before the README commit.

```toml
gpui = { git = "https://github.com/gpui-binder/gpui-binder", package = "gpui_facade", rev = "<commit-hash>", features = ["gpui_macos", "gpui_web", "gpui_macros", "font-kit", "x11", "wayland", "runtime_shaders"] }
gpui-component = { git = "https://github.com/gpui-binder/gpui-binder", package = "gpui-component", rev = "<commit-hash>" }
```

Replace `<commit-hash>` with the generated source commit hash listed in the `generated-gpui-nightly` branch README.

Do not use the README-only commit hash as the dependency revision unless you intentionally want to pin to that commit.

## Repository layout

```text
.
├── README.md
└── .github/
    └── workflows/
        ├── fork-gpui.yaml
        ├── bind-gpui-component.yaml
        ├── check-cargo-local-dependencies.yaml
        ├── combined-workflow.yaml
        └── generated-nightly.yaml
```

## What this repo is for

This repository is a generator and integration tool for GPUI development outside the full Zed monorepo.

Zed’s GPUI code lives inside the larger `zed-industries/zed` repository. The full Zed repository contains many crates and project files that are not needed when the goal is to experiment with, package, or integrate GPUI and GPUI component crates. This repo automates the creation of a smaller generated branch containing a GPUI-focused Rust workspace.

The generated workspace is intended to include:

* Core GPUI crates from Zed.
* A generated `gpui_facade` crate.
* Imported `gpui-component` crates.
* Patched Cargo workspace configuration.
* Patched dependency references so imported components use local GPUI crates.
* A generated branch README for nightly output.

## Generated branches

The workflows create or update branches in this repository.

### `generated-gpui-base`

Created by `fork-gpui.yaml`.

This branch is generated from the selected upstream Zed source branch. It contains a reduced GPUI-focused workspace plus a generated `gpui_facade` crate.

Typical crates in this branch include:

```text
crates/collections
crates/gpui
crates/gpui_facade
crates/gpui_linux
crates/gpui_macos
crates/gpui_macros
crates/gpui_platform
crates/gpui_shared_string
crates/gpui_tokio
crates/gpui_util
crates/gpui_web
crates/gpui_wgpu
crates/gpui_windows
crates/http_client
crates/http_client_tls
crates/media
crates/refineable
crates/reqwest_client
crates/scheduler
crates/sum_tree
crates/util
crates/util_macros
crates/zlog
crates/ztracing
crates/ztracing_macro
```

### `generated-gpui-binded`

Created by `bind-gpui-component.yaml`.

This branch starts from the generated GPUI base branch, imports `gpui-component`, patches Cargo dependencies, and validates the result.

Typical additional crates in this branch include:

```text
crates/gpui_component
crates/gpui_component_assets
crates/gpui_component_macros
crates/gpui_component_story
crates/gpui_component_story_web
```

### `generated-zed-nightly`

Created by `generated-nightly.yaml` through `fork-gpui.yaml`.

This is the nightly intermediate branch. It contains the reduced GPUI-focused workspace generated from upstream Zed, but it does not include the imported `gpui-component` crates.

The nightly workflow avoids updating this branch when the newly generated files are identical to the existing branch.

### `generated-gpui-nightly`

Created by `generated-nightly.yaml` through `bind-gpui-component.yaml`.

This is the nightly final branch. It contains:

* The reduced GPUI-focused workspace.
* The generated `gpui_facade` crate.
* Imported `gpui-component` crates.
* Local dependency rewrites.
* A generated `README.md` committed after successful validation.

The generated source commit comes first. The README commit comes second.

This shape is intentional:

```text
A = generated source commit
B = generated README commit
HEAD = B
```

The usage examples in the generated nightly README should point to `A`, not `B`.

## Workflows

### 1. `fork-gpui.yaml`

Workflow name:

```text
Fork branch and apply embedded gpui facade fixes
```

This workflow creates a reduced GPUI workspace from an upstream Zed repository.

It performs the first stage of the pipeline.

#### Inputs

| Input             | Required | Default                 | Description                                                                           |
| ----------------- | -------: | ----------------------- | ------------------------------------------------------------------------------------- |
| `upstream_repo`   |      Yes | `zed-industries/zed`    | Repository to copy from.                                                              |
| `upstream_branch` |      Yes | `main`                  | Branch, tag, or ref to copy from the upstream repo.                                   |
| `output_branch`   |       No | empty                   | Branch to create in this repository. If empty, an auto-generated branch name is used. |
| `commit_message`  |      Yes | `Add gpui facade crate` | Commit message used for generated changes.                                            |
| `force_push`      |      Yes | `false`                 | Whether to update the output branch if it already exists.                             |

#### What it does

The workflow:

1. Configures Git.
2. Sparse-clones the selected upstream Zed branch.
3. Excludes many files and crates that are not needed for the GPUI-focused output.
4. Preserves the files needed for Cargo metadata validation.
5. Copies the sparse checkout into a clean output directory.
6. Removes materialized excluded files and empty directories.
7. Creates `crates/gpui_facade`.
8. Patches the root `Cargo.toml` workspace.
9. Runs `cargo metadata --no-deps --format-version=1`.
10. Commits the generated output.
11. Compares the generated file tree with the existing remote branch.
12. Pushes only when files changed.

#### Avoiding unnecessary pushes

Before pushing, `fork-gpui.yaml` checks whether the remote output branch already has the same files:

```bash
git diff --quiet "origin/$OUTPUT_BRANCH" HEAD -- .
```

If the files are the same, the workflow skips the push.

If the files are different and `force_push` is enabled, the workflow uses:

```bash
git push --force-with-lease origin "HEAD:$OUTPUT_BRANCH"
```

This avoids changing `generated-zed-nightly` when upstream Zed produces the same generated files as the existing branch.

#### Sparse exclusion behavior

The workflow uses an embedded `EXCLUDE_PATTERNS` list to remove unneeded files and crates from the upstream Zed checkout.

Examples of excluded paths include:

```text
.github/**
.devcontainer/**
.vscode/**
script/**
assets/**
docs/**
nix/**
ci/**
extensions/workflows/
tooling/xtask/
crates/agent/
crates/editor/
crates/project/
crates/terminal/
crates/workspace/
crates/zed/
```

The goal is to keep enough of the Zed workspace to build and resolve GPUI-related crates while removing most editor, AI, collaboration, language, extension, terminal, and application-specific code.

#### Generated `gpui_facade` crate

The workflow creates:

```text
crates/gpui_facade/Cargo.toml
crates/gpui_facade/README.md
crates/gpui_facade/src/lib.rs
```

The facade crate re-exports GPUI and selected related crates so downstream crates can depend on one facade package.

Example dependency style:

```toml
gpui = { package = "gpui_facade", workspace = true }
```

Example Rust usage:

```rust
use gpui::*;

let app = gpui::gpui_platform::application();
```

The generated crate re-exports:

```rust
pub use gpui::*;

pub mod gpui_platform {
    pub use ::gpui_platform::*;
}
```

It can also optionally re-export:

* `gpui_macros`
* `gpui_web`
* `gpui_macos`
* `gpui_linux`
* `gpui_windows`

#### Facade features

The generated facade crate defines features that forward to the underlying GPUI crates.

Default features:

```toml
default = ["font-kit", "wayland", "x11", "windows-manifest"]
```

Other supported features include:

```text
test-support
bench
inspector
leak-detection
wayland
x11
screen-capture
windows-manifest
input-latency-histogram
profiler
font-kit
runtime_shaders
gpui_macros
gpui_web
gpui_web_multithreaded
gpui_macos
gpui_linux
gpui_windows
```

#### Workspace patching

The workflow modifies the root `Cargo.toml` workspace by:

* Adding `crates/gpui_facade` to `[workspace].members`.
* Removing excluded or missing crates from `[workspace].members`.
* Removing invalid entries from `[workspace].default-members`.
* Adding excluded/missing crates to `[workspace].exclude`.
* Ensuring `crates/gpui_facade` is not excluded.

#### Validation

Before pushing the branch, the workflow runs:

```bash
cargo metadata --no-deps --format-version=1
```

It fails if:

* The root `Cargo.toml` is missing.
* `crates/gpui_facade/Cargo.toml` was not generated.
* `cargo metadata` fails.
* `gpui_facade` does not appear in workspace metadata.
* There are no generated changes to commit.

---

### 2. `bind-gpui-component.yaml`

Workflow name:

```text
Import gpui-component
```

This workflow imports `gpui-component` crates into a generated GPUI workspace.

It performs the second stage of the pipeline.

#### Inputs

| Input           | Required | Default                     | Description                                                                            |
| --------------- | -------: | --------------------------- | -------------------------------------------------------------------------------------- |
| `zed_ref`       |      Yes | `main`                      | Branch, tag, or SHA of this repository’s generated Zed/GPUI branch to use as the base. |
| `source_repo`   |      Yes | `longbridge/gpui-component` | Repository to import component crates from.                                            |
| `source_ref`    |      Yes | `main`                      | Branch, tag, or SHA from the component source repository.                              |
| `target_branch` |      Yes | `generated`                 | Branch to create or update with the imported component crates.                         |

#### What it does

The workflow:

1. Checks out the selected GPUI base branch.
2. Creates or resets the target branch.
3. Checks out the `gpui-component` source repository into `_gpui_component_source`.
4. Detects known component crates by reading package names from `Cargo.toml` files.
5. Copies detected crates into the local `crates/` directory.
6. Adds imported crates to the root workspace.
7. Adds imported crates to `[workspace.dependencies]`.
8. Expands source workspace dependencies into local path dependencies where needed.
9. Removes nested workspace metadata from imported crate manifests.
10. Converts imported crate dependencies to workspace dependencies.
11. Forces `gpui-component` to depend on the local `gpui_facade`.
12. Applies source-level style/dependency rewrites.
13. Runs `cargo metadata --no-deps`.
14. Commits the generated output.
15. Compares the generated file tree with the existing remote branch.
16. Pushes only when generated files changed.

#### Avoiding unnecessary pushes

Before pushing, `bind-gpui-component.yaml` checks whether the existing target branch already has the same generated files.

For `generated-gpui-nightly`, the existing branch may already contain a separate README commit. Therefore the comparison intentionally excludes `README.md`:

```bash
git diff --quiet "origin/$TARGET_BRANCH" HEAD -- . ':(exclude)README.md'
```

If the generated files are the same, the workflow skips the push.

If the generated files are different, the workflow updates the target branch with:

```bash
git push --force-with-lease origin "HEAD:$TARGET_BRANCH"
```

This avoids changing `generated-gpui-nightly` when neither upstream Zed nor `gpui-component` changed.

#### Detected source crates

The workflow looks for these package names in the component source repository:

| Source package             | Target path                       |
| -------------------------- | --------------------------------- |
| `gpui-component`           | `crates/gpui_component`           |
| `gpui-component-assets`    | `crates/gpui_component_assets`    |
| `gpui-component-macros`    | `crates/gpui_component_macros`    |
| `gpui-component-story`     | `crates/gpui_component_story`     |
| `gpui-component-story-web` | `crates/gpui_component_story_web` |

The workflow requires at least `gpui-component` to be found.

#### Workspace changes

The workflow patches the root `Cargo.toml` to add imported crates as workspace members.

Example generated members:

```toml
members = [
    "crates/gpui_component",
    "crates/gpui_component_assets",
    "crates/gpui_component_macros",
    "crates/gpui_component_story",
    "crates/gpui_component_story_web",
]
```

It also adds workspace dependencies for the imported packages:

```toml
[workspace.dependencies]
gpui-component = { path = "crates/gpui_component" }
gpui-component-assets = { path = "crates/gpui_component_assets" }
gpui-component-macros = { path = "crates/gpui_component_macros" }
gpui-component-story = { path = "crates/gpui_component_story" }
gpui-component-story-web = { path = "crates/gpui_component_story_web" }
```

#### Dependency rewriting

The workflow rewrites imported crate manifests so they work inside the generated local workspace.

It keeps some dependencies as workspace dependencies:

```text
gpui
gpui_platform
gpui_macros
gpui_web
story
```

It rewrites many other source workspace dependencies into explicit local path dependencies when possible.

It also rewrites `github.com/zed-industries/...` dependencies to local paths when the matching package exists in the generated workspace.

#### Forcing `gpui-component` to use `gpui_facade`

The workflow explicitly patches:

```text
crates/gpui_component/Cargo.toml
```

It removes existing direct `gpui` dependency declarations and inserts:

```toml
gpui = { package = "gpui_facade", path = "../gpui_facade", features = ["gpui_macos", "gpui_web", "gpui_macros", "font-kit", "x11", "wayland", "runtime_shaders"] }
```

This makes `gpui-component` use the generated local facade rather than depending directly on the split GPUI crates.

#### Source code rewrites

The workflow applies text rewrites across imported component crates:

```text
gpui_platform:: -> gpui::gpui_platform::
gpui_web::      -> gpui::gpui_web::
gpui_macros::   -> gpui::gpui_macros::
```

This matches the facade-based import style.

#### Manifest cleanup

The workflow removes direct dependencies on split GPUI crates from imported manifests:

```text
gpui_platform
gpui_web
gpui_macros
```

It also simplifies inspector feature declarations such as:

```toml
inspector = ["gpui_macros/inspector", "gpui/inspector"]
```

to:

```toml
inspector = ["gpui/inspector"]
```

#### Validation

The workflow runs:

```bash
cargo metadata --no-deps
```

It then commits:

```text
Import gpui-component into workspace
```

and pushes the target branch only when the generated files differ from the existing target branch.

---

### 3. `check-cargo-local-dependencies.yaml`

Workflow name:

```text
Check Cargo Local Dependencies
```

This workflow validates local Cargo path dependencies.

It is useful after generating the final branch because the pipeline removes many crates and rewrites many dependency paths. This check ensures local dependencies still point to valid directories containing `Cargo.toml`.

#### Inputs

| Input              | Required | Default | Description                                                            |
| ------------------ | -------: | ------- | ---------------------------------------------------------------------- |
| `ref`              |      Yes | `main`  | Branch, tag, or commit SHA to check.                                   |
| `scan_roots`       |       No | `.`     | Newline-separated directories to scan. Use `.` to scan the whole repo. |
| `fail_on_warnings` |       No | `false` | Whether warnings should fail the workflow.                             |

#### What it checks

The workflow scans for `Cargo.toml` files and checks local dependency edges from:

* `[dependencies]`
* `[dev-dependencies]`
* `[build-dependencies]`
* target-specific dependency tables
* `[patch.*]`
* `[replace]`
* root `[workspace.dependencies]` entries used through `workspace = true`

It checks that local path dependencies:

1. Stay inside the repository.
2. Point to an existing directory.
3. Point to a directory containing `Cargo.toml`.

#### Direct path dependency example

```toml
some-crate = { path = "../some-crate" }
```

The checker verifies that `../some-crate/Cargo.toml` exists and is inside the repository.

#### Workspace dependency example

```toml
[workspace.dependencies]
some-crate = { path = "crates/some-crate" }

[dependencies]
some-crate.workspace = true
```

The checker resolves `some-crate.workspace = true` through root `[workspace.dependencies]` and validates that the local path exists.

#### Warnings

The checker emits warnings when:

* A scan root does not exist.
* Root `Cargo.toml` is missing.
* A dependency uses `workspace = true`, but root `[workspace.dependencies]` does not define it.
* A local package with the same name exists, but root `[workspace.dependencies]` does not map it as a local path.

By default, warnings do not fail the workflow.

Set `fail_on_warnings` to `true` to make warnings fail the workflow.

#### Skipped directories

The checker skips common generated, hidden, and tool directories, including:

```text
.git
.github
.idea
.vscode
.zed
target
node_modules
vendor
.cargo
.cache
.next
dist
build
out
```

#### Output

The workflow prints:

* Repository root.
* Whether warnings fail the check.
* Scan roots.
* Number of discovered `Cargo.toml` files.
* Number of discovered package names.
* Number of checked local dependency edges.
* List of discovered manifests.
* Warnings.
* Errors.

Successful output ends with:

```text
PASS: all discovered local Cargo dependency paths exist.
```

---

### 4. `combined-workflow.yaml`

Workflow name:

```text
Run GPUI Pipeline
```

This is the main manual orchestration workflow. It runs the other three workflows in sequence.

Use this workflow when you want to generate a complete final branch from upstream Zed plus `gpui-component`.

#### Inputs

| Input                | Required | Default                     | Description                                                     |
| -------------------- | -------: | --------------------------- | --------------------------------------------------------------- |
| `upstream_repo`      |      Yes | `zed-industries/zed`        | Source repository for GPUI.                                     |
| `upstream_branch`    |      Yes | `main`                      | Branch from the GPUI source repository.                         |
| `source_repo`        |      Yes | `longbridge/gpui-component` | Source repository for `gpui-component`.                         |
| `source_ref`         |      Yes | `main`                      | Branch, tag, or SHA from the component source repository.       |
| `fork_output_branch` |      Yes | `generated-gpui-base`       | Intermediate branch created by `fork-gpui.yaml`.                |
| `bind_target_branch` |      Yes | `generated-gpui-binded`     | Final branch created by `bind-gpui-component.yaml`.             |
| `force_push`         |      Yes | `true`                      | Whether to update generated branches if they already exist.     |

#### Required permissions

The workflow requires:

```yaml
permissions:
  contents: write
  actions: write
```

It uses the GitHub CLI to dispatch and watch the other workflows.

#### Execution order

The workflow runs:

```text
1. fork-gpui.yaml
2. bind-gpui-component.yaml
3. check-cargo-local-dependencies.yaml
```

#### What it dispatches

First, it runs `fork-gpui.yaml`:

```bash
gh workflow run fork-gpui.yaml \
  -f upstream_repo="$upstream_repo" \
  -f upstream_branch="$upstream_branch" \
  -f output_branch="$fork_output_branch" \
  -f commit_message="Add gpui facade crate" \
  -f force_push="$force_push"
```

Then, it runs `bind-gpui-component.yaml`:

```bash
gh workflow run bind-gpui-component.yaml \
  -f zed_ref="$fork_output_branch" \
  -f source_repo="$source_repo" \
  -f source_ref="$source_ref" \
  -f target_branch="$bind_target_branch"
```

Finally, it runs `check-cargo-local-dependencies.yaml`:

```bash
gh workflow run check-cargo-local-dependencies.yaml \
  -f ref="$bind_target_branch"
```

The workflow waits for each dispatched workflow run to appear and then watches it with:

```bash
gh run watch "$run_id" --exit-status
```

If any stage fails, the pipeline fails.

---

### 5. `generated-nightly.yaml`

Workflow name:

```text
Generate GPUI Nightly
```

This is the scheduled nightly orchestration workflow.

It runs the same three-stage pipeline as the manual combined workflow, but with fixed nightly branch defaults:

```text
fork-gpui.yaml                    -> generated-zed-nightly
bind-gpui-component.yaml          -> generated-gpui-nightly
check-cargo-local-dependencies    -> generated-gpui-nightly
```

After all three workflows succeed, it checks out `generated-gpui-nightly`, generates `README.md`, and commits it separately.

#### Schedule

The workflow runs every day at:

```text
17:42 GMT+9 / Asia-Tokyo
08:42 UTC
```

GitHub Actions cron uses UTC, so the workflow uses:

```yaml
schedule:
  - cron: "42 8 * * *"
```

It also supports manual dispatch through `workflow_dispatch`.

#### Nightly inputs

| Input                | Required | Default                       | Description                                               |
| -------------------- | -------: | ----------------------------- | --------------------------------------------------------- |
| `upstream_repo`      |      Yes | `zed-industries/zed`          | Source repository for GPUI.                               |
| `upstream_branch`    |      Yes | `main`                        | Branch from the GPUI source repository.                   |
| `source_repo`        |      Yes | `longbridge/gpui-component`   | Source repository for `gpui-component`.                   |
| `source_ref`         |      Yes | `main`                        | Branch, tag, or SHA from the component source repository. |
| `fork_output_branch` |      Yes | `generated-zed-nightly`       | Intermediate nightly branch.                              |
| `bind_target_branch` |      Yes | `generated-gpui-nightly`      | Final nightly branch.                                     |
| `force_push`         |      Yes | `true`                        | Whether to update generated branches if files changed.    |
| `fail_on_warnings`   |      Yes | `false`                       | Whether dependency-check warnings fail the nightly run.   |

#### README generation

The nightly workflow creates a branch-local `README.md` only after:

```text
fork-gpui.yaml succeeds
bind-gpui-component.yaml succeeds
check-cargo-local-dependencies.yaml succeeds
```

The generated README contains usage examples like:

```toml
gpui = { git = "https://github.com/gpui-binder/gpui-binder", package = "gpui_facade", rev = "<commit-hash>", features = ["gpui_macos", "gpui_web", "gpui_macros", "font-kit", "x11", "wayland", "runtime_shaders"] }
gpui-component = { git = "https://github.com/gpui-binder/gpui-binder", package = "gpui-component", rev = "<commit-hash>" }
```

`<commit-hash>` is the generated source commit before the README commit.

If the current branch `HEAD` is already a README-only commit, the workflow uses `HEAD^` as the generated source revision. This prevents README churn on nights where no generated files changed.

#### Nightly no-change behavior

The nightly pipeline avoids unnecessary branch movement.

| Zed source | gpui-component source | `generated-zed-nightly` | `generated-gpui-nightly` | README |
| ---------- | --------------------- | ----------------------- | ------------------------ | ------ |
| No change  | No change             | No push                 | No push                  | No new commit |
| Changed    | No change             | Push changed files      | Push changed final tree  | New README commit |
| No change  | Changed               | No push                 | Push changed final tree  | New README commit |
| Changed    | Changed               | Push changed files      | Push changed final tree  | New README commit |

For `generated-zed-nightly`, the comparison includes all files:

```bash
git diff --quiet "origin/$OUTPUT_BRANCH" HEAD -- .
```

For `generated-gpui-nightly`, the comparison excludes `README.md` because it is committed separately:

```bash
git diff --quiet "origin/$TARGET_BRANCH" HEAD -- . ':(exclude)README.md'
```

This means a README-only difference does not trigger regeneration of the final nightly branch.

## How to use

### Recommended: run the full manual pipeline

Open the repository on GitHub and run:

```text
Actions → Run GPUI Pipeline → Run workflow
```

Use the defaults for a normal manual generation:

```text
upstream_repo: zed-industries/zed
upstream_branch: main
source_repo: longbridge/gpui-component
source_ref: main
fork_output_branch: generated-gpui-base
bind_target_branch: generated-gpui-binded
force_push: true
```

After the workflow completes, inspect the final generated branch:

```text
generated-gpui-binded
```

### Recommended: use the nightly branch

The nightly branch is updated automatically when generated files change.

```text
https://github.com/gpui-binder/gpui-binder/tree/generated-gpui-nightly
```

For reproducibility, use the `rev = "<commit-hash>"` shown in that branch’s README rather than the moving branch name.

### Run the nightly workflow manually

```bash
gh workflow run generated-nightly.yaml \
  --repo gpui-binder/gpui-binder \
  --ref main \
  -f upstream_repo=zed-industries/zed \
  -f upstream_branch=main \
  -f source_repo=longbridge/gpui-component \
  -f source_ref=main \
  -f fork_output_branch=generated-zed-nightly \
  -f bind_target_branch=generated-gpui-nightly \
  -f force_push=true \
  -f fail_on_warnings=false
```

Watch the run:

```bash
gh run watch --repo gpui-binder/gpui-binder
```

### Run with GitHub CLI

```bash
gh workflow run combined-workflow.yaml \
  --repo gpui-binder/gpui-binder \
  --ref main \
  -f upstream_repo=zed-industries/zed \
  -f upstream_branch=main \
  -f source_repo=longbridge/gpui-component \
  -f source_ref=main \
  -f fork_output_branch=generated-gpui-base \
  -f bind_target_branch=generated-gpui-binded \
  -f force_push=true
```

Watch the run:

```bash
gh run watch --repo gpui-binder/gpui-binder
```

### Run only the GPUI base generation

```bash
gh workflow run fork-gpui.yaml \
  --repo gpui-binder/gpui-binder \
  --ref main \
  -f upstream_repo=zed-industries/zed \
  -f upstream_branch=main \
  -f output_branch=generated-gpui-base \
  -f commit_message="Add gpui facade crate" \
  -f force_push=true
```

### Run only the component binding step

This assumes the base branch already exists.

```bash
gh workflow run bind-gpui-component.yaml \
  --repo gpui-binder/gpui-binder \
  --ref main \
  -f zed_ref=generated-gpui-base \
  -f source_repo=longbridge/gpui-component \
  -f source_ref=main \
  -f target_branch=generated-gpui-binded
```

### Run only the local dependency check

```bash
gh workflow run check-cargo-local-dependencies.yaml \
  --repo gpui-binder/gpui-binder \
  --ref main \
  -f ref=generated-gpui-binded \
  -f scan_roots=. \
  -f fail_on_warnings=false
```

## Local inspection

Clone this repository:

```bash
git clone https://github.com/gpui-binder/gpui-binder.git
cd gpui-binder
```

Fetch generated branches:

```bash
git fetch origin generated-gpui-base generated-gpui-binded generated-zed-nightly generated-gpui-nightly
```

Check out the final manual generated branch:

```bash
git checkout generated-gpui-binded
```

Or check out the final nightly generated branch:

```bash
git checkout generated-gpui-nightly
```

Run Cargo metadata locally:

```bash
cargo metadata --no-deps
```

Inspect the generated facade crate:

```bash
cat crates/gpui_facade/Cargo.toml
cat crates/gpui_facade/src/lib.rs
```

Inspect the imported component crate:

```bash
cat crates/gpui_component/Cargo.toml
```

## Expected final result

After a successful full pipeline run, the final generated branch should contain a reduced Rust workspace with:

```text
Cargo.toml
Cargo.lock
crates/gpui
crates/gpui_facade
crates/gpui_component
crates/gpui_component_assets
crates/gpui_component_macros
crates/gpui_component_story
crates/gpui_component_story_web
```

It should also contain the GPUI platform/support crates needed by the workspace, such as:

```text
crates/gpui_platform
crates/gpui_macros
crates/gpui_web
crates/gpui_macos
crates/gpui_linux
crates/gpui_windows
crates/gpui_wgpu
crates/gpui_util
crates/gpui_tokio
```

On `generated-gpui-nightly`, it should also contain:

```text
README.md
```

The README commit should be separate from the generated source commit.

## Why `gpui_facade` exists

`gpui-component` expects access to several GPUI-related crates, such as:

```text
gpui
gpui_platform
gpui_web
gpui_macros
```

The generated `gpui_facade` crate provides a single facade package that:

* Re-exports `gpui`.
* Exposes `gpui_platform` as `gpui::gpui_platform`.
* Optionally exposes `gpui_web` as `gpui::gpui_web`.
* Optionally exposes `gpui_macros` as `gpui::gpui_macros`.
* Optionally exposes platform crates for macOS, Linux, FreeBSD, Windows, and WebAssembly.

This lets imported component code use a unified namespace:

```rust
use gpui::*;

let platform = gpui::gpui_platform::application();
```

instead of depending directly on multiple split crates.

## Notes on naming

The default final manual branch is named:

```text
generated-gpui-binded
```

This name is used by the current workflow defaults. Although “bound” would be the more conventional English spelling, the repository currently uses `binded` in the workflow input defaults.

The nightly final branch is named:

```text
generated-gpui-nightly
```

The nightly intermediate branch is named:

```text
generated-zed-nightly
```

## Troubleshooting

### `gpui-component` was not found

The binding workflow requires a package named:

```text
gpui-component
```

If the source repository or branch does not contain a matching `Cargo.toml`, the workflow fails and prints the discovered Cargo manifests.

Check:

```text
source_repo
source_ref
```

### `cargo metadata` fails during `fork-gpui.yaml`

This usually means the reduced Zed workspace is missing a crate or dependency needed by the retained GPUI crates.

Check:

* `EXCLUDE_PATTERNS` in `fork-gpui.yaml`.
* Root `Cargo.toml` workspace members.
* Root `Cargo.toml` default members.
* Root `Cargo.toml` workspace exclusions.
* Whether upstream Zed changed crate names or dependencies.

### `cargo metadata` fails during `bind-gpui-component.yaml`

This usually means the imported `gpui-component` crates still refer to a dependency that was not imported or was not rewritten correctly.

Check:

* `crates/gpui_component/Cargo.toml`
* Imported crate manifests under `crates/gpui_component*`
* Root `[workspace.dependencies]`
* Local path dependencies
* Remaining direct references to `gpui_platform`, `gpui_web`, or `gpui_macros`

### Local dependency check fails

The local dependency checker reports exact manifest paths and dependency names.

Common causes:

* A `path = "..."`
  dependency points to a removed crate.
* A `workspace = true` dependency resolves to a missing local path.
* A path points outside the repository.
* A dependency directory exists but does not contain `Cargo.toml`.

Run the checker on the final branch:

```bash
gh workflow run check-cargo-local-dependencies.yaml \
  --repo gpui-binder/gpui-binder \
  --ref main \
  -f ref=generated-gpui-binded
```

### Generated branch already exists

The workflows now compare file trees before pushing generated output.

For `generated-zed-nightly`, if the generated files are identical to the existing remote branch, the push is skipped.

For `generated-gpui-nightly`, if the generated files are identical to the existing remote branch except for `README.md`, the push is skipped.

If generated files differ and `force_push` is enabled, the workflows use:

```bash
git push --force-with-lease
```

rather than plain `git push --force`.

Use a new branch name or enable `force_push` when you intentionally want to replace an existing generated branch with changed generated files.

### Nightly README keeps changing

This should not happen with the revised nightly workflow.

The workflow detects when the current `HEAD` is already a README-only commit:

```text
HEAD = README commit
HEAD^ = generated source commit
```

In that case, it uses `HEAD^` as the dependency revision instead of `HEAD`.

This prevents the README from changing only because the previous run already added a README commit.

## Development notes

The workflows are intentionally self-contained. Most transformation logic is embedded directly inside the YAML files as Bash and Python scripts.

This makes the repo easy to run from GitHub Actions without requiring a separate tool crate or script package, but it also means workflow files are long and should be edited carefully.

When modifying the pipeline:

1. Update `fork-gpui.yaml` if the Zed source layout changes.
2. Update `bind-gpui-component.yaml` if `gpui-component` package names or dependencies change.
3. Update `check-cargo-local-dependencies.yaml` if dependency validation needs to cover more Cargo features.
4. Use `combined-workflow.yaml` to test the full end-to-end flow.
5. Use `generated-nightly.yaml` to test the scheduled nightly flow.

## License

This repository’s generated branches include code copied from upstream projects. Check the generated branch contents for upstream license files and package metadata.

The generated GPUI base branch includes license files from the upstream Zed/GPUI source, such as Apache and GPL license files where present.

## Summary

`gpui-binder` is a workflow-driven repository for producing a GPUI-focused Rust workspace by combining:

* GPUI crates from Zed,
* a generated `gpui_facade` crate,
* and `gpui-component` crates from an external source repository.

The main manual entry point is:

```text
.github/workflows/combined-workflow.yaml
```

The main scheduled nightly entry point is:

```text
.github/workflows/generated-nightly.yaml
```

The main manual output is:

```text
generated-gpui-binded
```

The main nightly output is:

```text
generated-gpui-nightly
```
