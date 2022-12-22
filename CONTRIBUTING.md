# Contributing

All contributors and contributions are welcome! Please open an issue on GitHub if you have issues, questions or ideas.

## GitHub

### Pull requests

Substrait follows the [Conventional Commits specification](https://www.conventionalcommits.org/en/v1.0.0/) for Pull Request titles. This allows for automation of releases based on commit messages that are merged to the default branch.

The `Title` job of the [Pull Request](.github/workflow/pull-request.yml) workflow and the `Conventional Commits` job of the [Merge](.github/workflows/merge.yml) workflow check the Pull Request title and merge commit message.

### Dependabot

substrait-rs uses [Depedendabot](https://docs.github.com/en/code-security/dependabot) to update dependencies using the [dependabot.yml](.github/dependabot.yml) configuration file.

### Bors

substrait-rs uses [bors](https://bors.tech/) to merge Pull Requests to prevent breaking the default branch using the [bors.toml](.github/bors.toml) configuration file.

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

### Substrait submodule

There is a git submodule for [Substrait](https://github.com/substrait-io/substrait) that must be cloned when building from source.

```shell
git clone --recurse-submodules git@github.com:substrait-io/substrait-rs.git
```

When the Substrait version is bumped make sure to update your local submodule.

```shell
git submodule update
```

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
