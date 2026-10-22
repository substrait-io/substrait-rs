<!--
// SPDX-License-Identifier: Apache-2.0
-->

# substrait-rs

[![substrait](https://raw.githubusercontent.com/substrait-io/substrait/main/site/docs/img/logo.svg)](https://substrait.io)

[![crates.io](https://img.shields.io/crates/v/substrait.svg)](https://crates.io/crates/substrait)
[![docs.rs](https://docs.rs/substrait/badge.svg)](https://docs.rs/substrait)

Rust crate for [Substrait](https://substrait.io/): Cross-Language Serialization for Relational Algebra.

## Documentation

- [Docs (release)](https://docs.rs/substrait)
- [Docs (main)](https://substrait-io.github.io/substrait-rs/)

## Substrait version

The specification version a release targets is set by the exact `substrait-prost` and
`substrait-extensions` requirements in [Cargo.toml](Cargo.toml) — the generated types and
the standard extension definitions come from those crates rather than being vendored here.
[`substrait::version`](src/version.rs) reports it at runtime.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how the specification relates to this repository,
the development environment, the build / format / lint / test commands, and the commit and
pull request conventions. Releases are automated with
[release-plz](https://release-plz.dev/).

## Getting Involved

To learn more, head over to [Substrait](https://substrait.io/), our parent project, and join
our [community](https://substrait.io/community/).
