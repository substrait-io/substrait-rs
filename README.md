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
- [Docs (main)](https://substrait-io.github.io/substrait-rs/substrait/)

## Development

### Requirements

- [Rust](https://rustup.rs)
- [protoc (>=3.15)](https://github.com/protocolbuffers/protobuf/releases)

### Build & test

Clone this repository.

```shell
git clone git@github.com:substrait-io/substrait-rs.git
cd substrait-rs
```

Update submodules.

```shell
git submodule init
git submodule update
```

Build and test with Cargo.

```shell
cargo test
```
