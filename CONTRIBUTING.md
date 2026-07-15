# Contributing

All contributors and contributions are welcome! Please open an issue on GitHub if you have issues, questions or ideas.

## GitHub

### Pull requests

Substrait follows the [Conventional Commits specification](https://www.conventionalcommits.org/en/v1.0.0/) for commit messages. This allows for automation of releases based on commit messages that are merged to the default branch.

The `Conventional Commits` job of the [Pull Request](.github/workflows/pull-request.yml) workflow check the Pull Request title and body and the resulting merge commit message.

### Releases

Releases are published automatically with the [Release](./github/workflows/release.yml) workflow. The workflow is triggered for every commit to the `main` branch. [`cargo-smart-release`](https://github.com/Byron/gitoxide/tree/main/cargo-smart-release) is used to bump the version, create and publish the new release.

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
All source files must have a valid SPDX license header. The `SPDX License Header` job in the [Check](.github/workflow/check.yml) workflow checks this.

Substrait requires all contributors to sign the [Contributor License Agreement (CLA)](https://cla-assistant.io/substrait-io/substrait). There is a GitHub app installed to help new contributors sign it.

## Development

### Requirements

- [Rust](https://rustup.rs)
- [protoc (>=3.15)](https://github.com/protocolbuffers/protobuf/releases)

In environments where no `protoc` is available the `protoc` feature can be enabled to build `protoc` from source:

```shell
cargo build --features protoc
```

### Packaged Substrait crates

The generated Substrait types come from the packaged [`substrait-prost`](https://crates.io/crates/substrait-prost) and [`substrait-extensions`](https://crates.io/crates/substrait-extensions) crates, which are versioned to track the Substrait spec tag. They are pinned to an exact version in [Cargo.toml](Cargo.toml) and must always be bumped in lockstep (they share the same version). The Substrait version reported by [`substrait::version`](src/version.rs) is derived from the `substrait-prost` pin in [build.rs](build.rs), so no git submodule is needed to build from source.

#### Substrait version bumps and semver

Substrait version bumps (raising the `substrait-prost`/`substrait-extensions` pin) are handled automatically by Dependabot (see [dependabot.yml](.github/dependabot.yml)), which groups both crates into a single PR. Because a spec bump can change generated types, treat these as potentially breaking: mark the commit with a `!` (e.g. `feat(deps,substrait)!:`), which causes release-plz to do a minor version bump (major post-1.0).

If you believe a bump is non-breaking, you can omit the `!` marker. `cargo-semver-checks` runs as part of the release workflow and may catch breaking API changes, but it does not cover all possible semver violations. When in doubt, mark the change as breaking.

Formatting, lints and tests are checked in the [Test](.github/workflows/test.yml) workflow.

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
