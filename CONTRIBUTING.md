# Contributing

All contributors and contributions are welcome! Please open an issue on GitHub if you have issues, questions or ideas.

1. [The specification is the source of truth](#the-specification-is-the-source-of-truth)
2. [GitHub](#github)
   - [Pull requests](#pull-requests)
   - [Breaking changes](#breaking-changes)
   - [Releases](#releases)
3. [Governance](#governance)
4. [Community](#community)
5. [License](#license)
6. [Development](#development)

## The specification is the source of truth

substrait-rs is an implementation of the [Substrait specification](https://substrait.io/); it does not define Substrait semantics. Review behavioral changes against the spec — the spec text and the `.proto` comments in [`substrait-io/substrait`](https://github.com/substrait-io/substrait) for the version this tree targets.

That version is not vendored here. It is pinned by the exact `substrait-prost` and `substrait-extensions` requirements in [Cargo.toml](Cargo.toml), which is also where the generated types and the standard extension definitions come from; see [Packaged Substrait crates](#packaged-substrait-crates). [build.rs](build.rs) derives the version constants from that pin, and [`substrait::version`](src/version.rs) reports it at runtime.

Where the spec is genuinely unclear, don't settle it here. Survey the ecosystem for an existing consensus first. The closest comparison is the sibling language bindings listed under [Active Libraries](https://substrait.io/community/active_libraries/) — `substrait-go`, `substrait-java` and `substrait-python` solve the same modeling problem at the same layer, so how they represent a construct is directly relevant; that page also separates active bindings from inactive ones, and an inactive binding's choice is weaker evidence. For questions about runtime semantics rather than modeling, the engines under [Powered by Substrait](https://substrait.io/community/powered_by/) (Acero, DataFusion, DuckDB, Gluten, Velox) are the better reference.

If they agree, follow that de facto consensus and say so in the PR. If they disagree, or none of them cover the case, raise a clarification issue in [`substrait-io/substrait`](https://github.com/substrait-io/substrait/issues) or bring it to the [community](https://substrait.io/community/) channels rather than encoding a guess — and record the open question in the PR so the assumption stays reviewable.

## GitHub

### Pull requests

Substrait follows the [Conventional Commits specification](https://www.conventionalcommits.org/en/v1.0.0/) for commit messages. This allows for automation of releases based on commit messages that are merged to the default branch.

The `Conventional Commits` job of the [Pull Request](.github/workflows/pull-request.yml) workflow check the Pull Request title and body and the resulting merge commit message.

Pull requests are squash-merged, and squash is the only merge method enabled: the **PR title becomes the commit subject and the PR description becomes the commit body**, verbatim. Your local commit messages are discarded, so the description is the artifact to get right — and a trailer you did not type, such as a `Signed-off-by:` added by `git commit -s`, reaches history when the description is filled from a commit message (`gh pr create --fill`, or pasting) rather than written.

From that message release-plz publishes the **subject** of every commit — it is the line that appears in [CHANGELOG.md](CHANGELOG.md) and in the GitHub release notes — and, for a breaking change, the footer described [below](#breaking-changes). The rest of the description is not published. So write a title that stands on its own, and leave out of the description anything the diff and the CI checks already show:

- **Lists of files touched** — they are in the diff.
- **Claims that CI-verified things pass** — "tests pass", "clippy clean". If they didn't, the checks would be red.
- **Process notes that are already implicit** — "opened as draft pending review".

Do include the rationale, and for spec-tracking changes the spec version (e.g. `spec v0.87.0`). The description is where the reasoning is preserved for anyone reading `git log` later, even though it is not part of the release notes.

### Breaking changes

Mark a breaking change twice: with `!` after the type and scope in the title (`feat!:` or `feat(scope)!:`), and with a `BREAKING CHANGE:` footer in the description. The `!` drives the version bump; the footer is what describes the break, so say what breaks and what consumers should do instead. Note that this project is pre-1.0, so a breaking change produces a **minor** bump rather than a major one, matching the [Substrait versioning policy](https://substrait.io/spec/versioning/).

The footer is the one part of the description that is published: the `[changelog]` body in [release-plz.toml](release-plz.toml) renders it into a `⚠ Breaking changes` section of [CHANGELOG.md](CHANGELOG.md) and the release notes. That makes its exact extent worth knowing.

Keep the footer **last, with nothing after it** — below the rationale and below any `Closes #NNN` line. The conventional-commits parser ends the note only at another `Key: value` footer or an issue reference. A trailing line that is neither — prose, or a bare tool-attribution line — is absorbed into the note and published verbatim, as if it were part of the migration instructions. Trailers like `Signed-off-by:` and `Co-authored-by:` do terminate it, so they are safe there, but a line without the `Key: value` shape is not. If the description has to carry such a line, put it _above_ the footer.

Writing the footer carefully matters because the subject line cannot carry the detail. v0.64.0 is the cautionary case: the footer of [#507](https://github.com/substrait-io/substrait-rs/pull/507) enumerates every affected `proto`, `text` and `parse` item, but the release notes for that version are a single summary line, because the changelog template rendered only subjects at the time. Putting the footer last also means the squash-merge message can be trimmed to just the subject and the footer in a single cut.

### Releases

Releases are published automatically with the [Release](.github/workflows/release.yml) workflow. The workflow is triggered for every commit to the `main` branch. [release-plz](https://release-plz.dev/), configured in [release-plz.toml](release-plz.toml), is used to bump the version, update [CHANGELOG.md](CHANGELOG.md), and create and publish the new release.

Because `release_always` is disabled, a commit to `main` does not publish directly. release-plz instead maintains a release pull request carrying the version bump and the generated changelog entries; merging that PR is what triggers the publish to [crates.io](https://crates.io/crates/substrait). `cargo-semver-checks` runs as part of the same workflow to catch breaking API changes that the commit types did not declare.

### Dependabot

substrait-rs uses [Depedendabot](https://docs.github.com/en/code-security/dependabot) to update dependencies using the [dependabot.yml](.github/dependabot.yml) configuration file.

### Prettier

substrait-rs uses [Prettier](https://prettier.io/) to format non-Rust source files. The `Formatting` job in the [Check](.github/workflows/check.yml) workflow checks this. To format your files locally (requires [Node.js](https://nodejs.org/en/)):

```shell
npm install prettier prettier-plugin-toml --save-dev --save-exact
npx prettier --write --no-config .
```

## Governance

Please refer to the [Substrait Governance](https://substrait.io/governance/) page.

## Community

Please refer to the [Substrait Community](https://substrait.io/community/) page.

## License

All contributions should be licensed under [Apache License, Version 2.0](LICENSE).
All source files must have a valid SPDX license header. The `SPDX License Header` job in the [Check](.github/workflows/check.yml) workflow checks this.

Substrait requires all contributors to sign the [Contributor License Agreement (CLA)](https://cla-assistant.io/substrait-io/substrait). There is a GitHub app installed to help new contributors sign it.

## Development

Formatting, lints and tests are checked in the [Test](.github/workflows/test.yml) workflow. Note that it checks each feature in isolation as well as `--all-features`, so anything feature-gated has to compile on its own.

### Requirements

- [Rust](https://rustup.rs)
- [protoc (>=3.15)](https://github.com/protocolbuffers/protobuf/releases)

In environments where no `protoc` is available the `protoc` feature can be enabled to build `protoc` from source:

```shell
cargo build --features protoc
```

### Packaged Substrait crates

The generated Substrait types come from the [`substrait-prost`](https://crates.io/crates/substrait-prost) and [`substrait-extensions`](https://crates.io/crates/substrait-extensions) crates, which are versioned to track the Substrait spec tag. They are pinned to an exact version in [Cargo.toml](Cargo.toml) and must always be bumped in lockstep (they share the same version). The Substrait version reported by [`substrait::version`](src/version.rs) is derived from the `substrait-prost` pin in [build.rs](build.rs).

#### Substrait version bumps and semver

Substrait version bumps (raising the `substrait-prost`/`substrait-extensions` pin) are handled automatically by Dependabot (see [dependabot.yml](.github/dependabot.yml)), which groups both crates into a single PR. Because a spec bump can change generated types, treat these as potentially breaking: mark the commit with a `!` (e.g. `feat(deps,substrait)!:`), which causes release-plz to do a minor version bump (major post-1.0).

If you believe a bump is non-breaking, you can omit the `!` marker. `cargo-semver-checks` runs as part of the release workflow and may catch breaking API changes, but it does not cover all possible semver violations. When in doubt, mark the change as breaking.

### Docs

#### Rustdoc

The crate documentation is built with [rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html):

```shell
cargo doc
```

Or to enable automatic feature information (requires a nightly toolchain):

```shell
cargo +nightly rustdoc -- --cfg docsrs
```

Or use [cargo-doc-rs](https://crates.io/crates/cargo-docs-rs):

```
cargo +nightly docs-rs
```

### Formatting

All Rust code is formatted using [rustfmt](https://github.com/rust-lang/rustfmt):

```shell
cargo fmt
```

### Linting

All Rust code passes [Clippy](https://github.com/rust-lang/rust-clippy) lints without warnings:

```shell
cargo clippy -- -Dwarnings
```

### Tests

To run tests and [documentation tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html):

```shell
cargo test
```
