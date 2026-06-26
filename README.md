# gpui-binder

**gpui-binder** generates a GPUI-focused Rust workspace by combining GPUI crates from Zed with the `gpui-component` crate family behind a single facade dependency.

The repository is primarily an automation repo. Its GitHub Actions workflows generate branches that contain usable Rust workspaces for downstream GPUI and `gpui-component` experimentation.

## Why this exists

GPUI lives inside the larger [`zed-industries/zed`](https://github.com/zed-industries/zed) monorepo. Using it together with [`longbridge/gpui-component`](https://github.com/longbridge/gpui-component) outside Zed requires pulling in the right GPUI crates, patching local dependencies, and keeping feature selection consistent.

`gpui-binder` automates that integration.

It generates:

- a reduced GPUI workspace from Zed,
- a `gpui_facade` crate that re-exports GPUI-related crates through one public dependency,
- imported `gpui-component` crates,
- patched Cargo manifests that point to the local generated workspace,
- validated generated branches suitable for direct Git dependencies.

## Quick start

### Use the generated nightly branch

The main generated branch is:

```text
https://github.com/gpui-binder/gpui-binder/tree/generated-gpui-nightly
```

For quick experiments, depend on the moving branch:

```toml
gpui = { git = "https://github.com/gpui-binder/gpui-binder", package = "gpui_facade", branch = "generated-gpui-nightly", features = ["gpui_macos", "gpui_web", "gpui_macros", "font-kit", "x11", "wayland", "runtime_shaders"] }
gpui-component = { git = "https://github.com/gpui-binder/gpui-binder", package = "gpui-component", branch = "generated-gpui-nightly" }
```

For reproducible builds, prefer the pinned `rev` shown in the README on `generated-gpui-nightly`:

```toml
gpui = { git = "https://github.com/gpui-binder/gpui-binder", package = "gpui_facade", rev = "<generated-source-commit>", features = ["gpui_macos", "gpui_web", "gpui_macros", "font-kit", "x11", "wayland", "runtime_shaders"] }
gpui-component = { git = "https://github.com/gpui-binder/gpui-binder", package = "gpui-component", rev = "<generated-source-commit>" }
```

> The nightly README is committed after the generated source commit. Use the generated source commit shown in that README, not the README-only commit, when pinning dependencies.

## The facade crate

`gpui_facade` is the recommended public entry point for downstream users.

Instead of depending directly on multiple split GPUI crates, users can depend on `gpui_facade` and alias it as `gpui`:

```toml
gpui = { package = "gpui_facade", ... }
```

Then Rust code can use:

```rust
use gpui::*;
use gpui::gpui_platform::*;
```

The facade re-exports the core GPUI API and exposes selected related crates under the same namespace:

```rust
pub use gpui::*;

pub mod gpui_platform {
    pub use ::gpui_platform::*;
}
```

Optional features can also expose:

- `gpui_macros`
- `gpui_web`
- `gpui_macos`
- `gpui_linux`
- `gpui_windows`

### Benefits of `gpui_facade`

- **One public dependency** for downstream apps.
- **Consistent crate identity** between app code and `gpui-component`.
- **Fewer dependency-shape mistakes** when combining GPUI platform crates and component crates.
- **Centralized feature selection** for `wayland`, `x11`, `font-kit`, `gpui_macos`, `gpui_web`, `gpui_macros`, and related features.
- **Cleaner imports** through `gpui::gpui_platform`, `gpui::gpui_web`, and similar facade namespaces.
- **A stable generated entry point** for nightly GPUI + `gpui-component` usage.

Note: `gpui_facade` is a packaging and integration layer. It is not intended to improve compile time, runtime CPU usage, memory usage, or binary size.

## Generated branches

| Branch | Purpose |
| --- | --- |
| `generated-gpui-base` | Manual intermediate branch generated from Zed. Contains the reduced GPUI workspace and `gpui_facade`. |
| `generated-gpui-binded` | Manual final branch. Starts from `generated-gpui-base`, imports `gpui-component`, rewrites dependencies, and validates the result. |
| `generated-zed-nightly` | Nightly intermediate branch generated from upstream Zed. Does not include `gpui-component`. |
| `generated-gpui-nightly` | Nightly final branch. Contains GPUI, `gpui_facade`, imported `gpui-component` crates, local dependency rewrites, and a generated branch README. |

The nightly final branch has two commits when content changes:

```text
A = generated source commit
B = generated README commit
HEAD = B
```

Dependency examples should pin to `A`, not `B`.

## What gets generated

A successful final branch contains a reduced Rust workspace with core GPUI crates, support crates, and imported component crates.

Typical generated crates include:

```text
crates/gpui
crates/gpui_facade
crates/gpui_platform
crates/gpui_macros
crates/gpui_web
crates/gpui_macos
crates/gpui_linux
crates/gpui_windows
crates/gpui_wgpu
crates/gpui_util
crates/gpui_tokio
crates/gpui_component
crates/gpui_component_assets
crates/gpui_component_macros
crates/gpui_component_story
crates/gpui_component_story_web
```

The exact workspace may change as upstream Zed or `gpui-component` changes.

## Workflows

### `generated-nightly.yaml`

Scheduled nightly pipeline.

Default behavior:

```text
zed-industries/zed@main
longbridge/gpui-component@main
```

Outputs:

```text
generated-zed-nightly
generated-gpui-nightly
```

Schedule:

```text
08:42 UTC / 17:42 GMT+9
```

It runs:

1. `fork-gpui.yaml`
2. `bind-gpui-component.yaml`
3. `check-cargo-local-dependencies.yaml`
4. README generation for `generated-gpui-nightly`

The nightly pipeline avoids unnecessary branch movement. If generated files are unchanged, it skips pushing. For `generated-gpui-nightly`, README-only differences are ignored when deciding whether generated source files changed.

### `combined-workflow.yaml`

Manual end-to-end pipeline.

Default outputs:

```text
generated-gpui-base
generated-gpui-binded
```

Run it from GitHub:

```text
Actions → Run GPUI Pipeline → Run workflow
```

Or with GitHub CLI:

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

### `fork-gpui.yaml`

Creates a reduced GPUI workspace from an upstream Zed ref.

Main tasks:

- sparse-checks out the selected Zed source,
- removes unrelated Zed application/editor crates,
- keeps GPUI-related crates and required support crates,
- generates `crates/gpui_facade`,
- patches the root workspace manifest,
- validates with `cargo metadata --no-deps --format-version=1`,
- pushes only when generated files changed.

Run directly:

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

### `bind-gpui-component.yaml`

Imports `gpui-component` crates into a generated GPUI workspace.

Main tasks:

- checks out a generated GPUI base branch,
- imports known `gpui-component` crates,
- adds them as workspace members,
- rewrites manifests to use local workspace dependencies,
- forces `gpui-component` to depend on local `gpui_facade`,
- rewrites imports such as `gpui_platform::` to `gpui::gpui_platform::`,
- validates with `cargo metadata --no-deps`,
- pushes only when generated files changed.

Run directly:

```bash
gh workflow run bind-gpui-component.yaml \
  --repo gpui-binder/gpui-binder \
  --ref main \
  -f zed_ref=generated-gpui-base \
  -f source_repo=longbridge/gpui-component \
  -f source_ref=main \
  -f target_branch=generated-gpui-binded
```

### `check-cargo-local-dependencies.yaml`

Validates local Cargo path dependencies after generation.

It checks dependencies from:

- `[dependencies]`
- `[dev-dependencies]`
- `[build-dependencies]`
- target-specific dependency tables
- `[patch.*]`
- `[replace]`
- root `[workspace.dependencies]` entries used through `workspace = true`

It verifies that local paths stay inside the repository, point to existing directories, and contain `Cargo.toml`.

Run directly:

```bash
gh workflow run check-cargo-local-dependencies.yaml \
  --repo gpui-binder/gpui-binder \
  --ref main \
  -f ref=generated-gpui-binded \
  -f scan_roots=. \
  -f fail_on_warnings=false
```

## Local inspection

Clone the repository:

```bash
git clone https://github.com/gpui-binder/gpui-binder.git
cd gpui-binder
```

Fetch generated branches:

```bash
git fetch origin \
  generated-gpui-base \
  generated-gpui-binded \
  generated-zed-nightly \
  generated-gpui-nightly
```

Check out a generated branch:

```bash
git checkout generated-gpui-nightly
```

Validate Cargo metadata:

```bash
cargo metadata --no-deps
```

Inspect generated manifests:

```bash
cat crates/gpui_facade/Cargo.toml
cat crates/gpui_facade/src/lib.rs
cat crates/gpui_component/Cargo.toml
```

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

## Naming note

The manual final branch is currently named:

```text
generated-gpui-binded
```

The name is kept for compatibility with the current workflow defaults, even though `bound` would be the more conventional English spelling.

## Troubleshooting

### `gpui-component` was not found

The binding workflow requires a package named `gpui-component` in the selected component source repository and ref.

Check:

```text
source_repo
source_ref
```

### `cargo metadata` fails during GPUI generation

The reduced Zed workspace is probably missing a crate or dependency needed by retained GPUI crates.

Check:

- exclusion patterns in `fork-gpui.yaml`,
- root workspace members,
- root default members,
- workspace exclusions,
- upstream Zed crate or dependency changes.

### `cargo metadata` fails during component binding

The imported `gpui-component` crates may still refer to a dependency that was not imported or rewritten correctly.

Check:

- `crates/gpui_component/Cargo.toml`,
- imported `crates/gpui_component*` manifests,
- root `[workspace.dependencies]`,
- local path dependencies,
- remaining direct references to `gpui_platform`, `gpui_web`, or `gpui_macros`.

### Local dependency check fails

The checker reports exact manifest paths and dependency names.

Common causes:

- a `path = "..."` dependency points to a removed crate,
- a `workspace = true` dependency resolves to a missing local path,
- a dependency path points outside the repository,
- a dependency directory exists but does not contain `Cargo.toml`.

### Generated branch already exists

Generated branches are updated only when file contents change. When changes exist and `force_push` is enabled, workflows use:

```bash
git push --force-with-lease
```

Use a new branch name or enable `force_push` when intentionally replacing a generated branch.

### Nightly README keeps changing

The nightly workflow is designed to avoid README-only churn. If `HEAD` is already a README-only commit, the dependency revision is based on `HEAD^`, the generated source commit.

## Development notes

Most transformation logic is embedded directly in the workflow YAML files as Bash and Python scripts. This keeps the repository self-contained, but workflow edits should be made carefully.

When modifying the pipeline:

1. Update `fork-gpui.yaml` if the Zed source layout changes.
2. Update `bind-gpui-component.yaml` if `gpui-component` package names or dependencies change.
3. Update `check-cargo-local-dependencies.yaml` if validation needs to cover more Cargo features.
4. Use `combined-workflow.yaml` to test the manual end-to-end flow.
5. Use `generated-nightly.yaml` to test the scheduled nightly flow.

## License

Generated branches include code copied from upstream projects. Check each generated branch for upstream license files and package metadata.

The generated GPUI base branch includes license files from the upstream Zed/GPUI source where present.

## Summary

`gpui-binder` is a workflow-driven repository for producing a GPUI-focused Rust workspace by combining:

- GPUI crates from Zed,
- a generated `gpui_facade` crate,
- and `gpui-component` crates from an external source repository.

Main entry points:

```text
Manual:  .github/workflows/combined-workflow.yaml
Nightly: .github/workflows/generated-nightly.yaml
```

Main outputs:

```text
Manual:  generated-gpui-binded
Nightly: generated-gpui-nightly
```
