<!--
// SPDX-License-Identifier: Apache-2.0
-->

# AGENTS.md

Entry point for AI agents working in the `substrait-rs` repository. Read the shared,
human-facing docs first, then keep the notes below in mind.

## Start here

- **[`README.md`](README.md)** — what the crate is, what sets the Substrait version it
  targets, and where the rendered documentation lives.
- **[`CONTRIBUTING.md`](CONTRIBUTING.md)** — how the spec relates to this repo, the packaged
  Substrait crates and the semver policy for spec bumps, the build / test / format / lint
  command mechanics, and the commit and pull request conventions.

This repo _implements_ the Substrait spec; it does not define it. Read
[the specification is the source of truth](CONTRIBUTING.md#the-specification-is-the-source-of-truth)
before changing behavior. The failure mode to avoid is filling a gap in the spec with
something plausible and then describing it as spec-defined. When you cannot find the spec's
answer, say so explicitly instead of picking one silently: check the sibling bindings listed
at [Active Libraries](https://substrait.io/community/active_libraries/) for an existing
consensus, and surface what is still unresolved in the PR.

The generated `proto` and `text` types and the standard extension definitions are **not**
vendored here — they are re-exported from the
[`substrait-prost`](https://crates.io/crates/substrait-prost) and
[`substrait-extensions`](https://crates.io/crates/substrait-extensions) crates, pinned to an
exact version in [`Cargo.toml`](Cargo.toml) and always bumped in lockstep.
[`build.rs`](build.rs) parses that pin to generate the [`substrait::version`](src/version.rs)
constants, so the pin is what ties this tree to a spec version. Changing it means changing
the spec this crate targets, not fixing something in this repo; Dependabot's `substrait`
group owns those bumps.

## Conventions & workflow

- **The PR title is the changelog entry.** release-plz builds
  [`CHANGELOG.md`](CHANGELOG.md) and the GitHub release body from the commit summary, so the
  title is what consumers read. The rest of the description is not published — it reaches the
  release notes only through a `BREAKING CHANGE:` footer.
- **The PR description _is_ the commit body.** This repository squash-merges and takes the
  commit message from the PR title and description; local commit messages are discarded, and
  no other merge method is enabled. So the description is the artifact to get right, and
  reviewing your own commits is not a substitute for reviewing it. Do not populate it with
  `gh pr create --fill` or by pasting a commit message — that is how a `Signed-off-by:` added
  by `git commit -s`, or a `Co-authored-by:` added by agent tooling, ends up in the body
  without anyone typing it.
- **Keep descriptions high-signal.** Follow
  [`CONTRIBUTING.md`](CONTRIBUTING.md#pull-requests) rather than
  [`.github/pull_request_template.md`](.github/pull_request_template.md), which a PR opened
  with an explicitly supplied body never shows you. Beyond forming a valid conventional
  commit, leave out the noise agents tend to add:
  - **Lists of files touched** — they're in the diff.
  - **Claims that CI-verified things pass** — e.g. "tests pass", "clippy clean". If they
    didn't, the checks would be red.
  - **Process notes that are already implicit** — e.g. "opened as draft pending review".

  Do include the rationale, and for spec-tracking changes the spec version (e.g.
  `spec v0.87.0`).

- **A `BREAKING CHANGE:` footer goes last, with nothing after it**, and the title gets a `!`
  (`feat!:` or `feat(scope)!:`). The footer text is published verbatim, and the parser ends
  the note only at another `Key: value` footer or an issue reference — so a trailing line
  that is neither, such as a bare tool-attribution line, is absorbed into the note and
  published as if it were migration instructions. If the description has to carry such a
  line, put it _above_ the footer. See
  [`CONTRIBUTING.md`](CONTRIBUTING.md#breaking-changes).
- **No GitHub issue or PR references in Rust code** (comments or rustdoc) — they belong in
  commit messages and PR descriptions. `Closes #NNN` in the description is fine; in the code,
  describe the behavior and the spec version (e.g. `spec v0.87.0`) instead.

## Checks that fail for reasons a local `cargo test` won't show

- **New files carry an SPDX header** (`// SPDX-License-Identifier: Apache-2.0`, wrapped in the
  comment syntax of the file's language — see the top of [`README.md`](README.md) for the
  Markdown form), checked by the `SPDX License Header` job in
  [`check.yml`](.github/workflows/check.yml). Every Rust source, workflow and dotfile has one.
  A few existing files do not — `LICENSE`, the release-plz-generated `CHANGELOG.md`,
  `Cargo.lock`, and `CONTRIBUTING.md` — so copy whatever a neighbouring file of the same kind
  does rather than assuming either way.
  [`.github/pull_request_template.md`](.github/pull_request_template.md) deliberately has
  none: its contents are prefilled into every PR description, and a header there would be
  pasted into commit bodies.
- **Non-Rust files are Prettier-formatted**, including Markdown and YAML. Install and run it
  as [`CONTRIBUTING.md`](CONTRIBUTING.md#prettier) describes — the `prettier-plugin-toml`
  dependency is what makes TOML formatting match CI, so `npx prettier` without it will not
  reproduce the `Formatting` job. [`.prettierignore`](.prettierignore) exempts `CHANGELOG.md`,
  which release-plz generates.
- **A dependency change must land with a regenerated [`Cargo.lock`](Cargo.lock).** The
  `Cargo.lock Sync` job runs `cargo update --workspace` and fails on any resulting diff, so a
  stale lock file fails CI on its own rather than as part of your change.
- **Each feature is checked in isolation, not just via `--all-features`.**
  [`test.yml`](.github/workflows/test.yml) runs
  `cargo check --all-targets --no-default-features --features protoc,<feature>` for
  `extensions`, `semver`, `serde`, `parse` and `embed-descriptor` individually, plus
  `embed-descriptor,serde` together because that combination compiles differently. (`protoc`
  is present in every one of those invocations because CI needs it, not because the feature
  under test requires it.) Anything feature-gated has to compile on its own, and
  `#[cfg(feature = ...)]` gates are easy to get wrong in a way that `--all-features` hides.
- **The MSRV is pinned to 1.88** (`rust-version` in [`Cargo.toml`](Cargo.toml)) and checked
  by its own job, so newer-toolchain idioms fail even when they build locally.
- **Rustdoc is built with `-Dwarnings`** on nightly by
  [`docs.yml`](.github/workflows/docs.yml), so a broken intra-doc link fails CI. The examples
  in [`src/lib.rs`](src/lib.rs) are compiled and run as doc tests.

Before pushing:

```shell
cargo fmt
cargo clippy --all-targets --all-features -- -Dwarnings
cargo test --all-features
cargo test --doc --all-features
```

Building requires [`protoc`](https://github.com/protocolbuffers/protobuf/releases) (>=3.15);
where it is unavailable, the `protoc` feature builds it from source.
