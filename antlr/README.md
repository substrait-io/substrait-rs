<!-- SPDX-License-Identifier: Apache-2.0 -->

# ANTLR code generation

The Substrait type-derivation DSL is parsed with [ANTLR]. The Rust target for
ANTLR is not part of upstream ANTLR; it lives in the [antlr4rust] fork, which
pairs with the `antlr4rust` runtime crate used by this project.

Generation requires a Java runtime, so to keep an ordinary `cargo build` free of
that dependency, the **generated parser is committed to the repository** (under
`src/parse/text/simple_extensions/derivations/`). CI verifies that the committed
files are in sync with the grammar.

The grammar itself is not vendored here. `antlr.py` downloads
`grammar/SubstraitType.g4` (and the `SubstraitLexer.g4` it imports) from the
[Substrait repository][substrait] at the tag matching the exact `substrait-prost`
version pin in `Cargo.toml`, the same pin `build.rs` uses to derive the Substrait
version. Bumping that pin therefore moves the grammar too, and CI flags the
committed parser as stale until it is regenerated.

## Usage

A Java runtime and Python are provided through [pixi]:

```shell
# Regenerate the committed parser from the pinned grammar.
pixi run gen-antlr

# Verify the committed parser is up to date (used in CI).
pixi run check-codegen
```

`antlr.py` downloads the grammar and the pinned ANTLR fork jar (verifying its SHA-256), runs it
with `-Dlanguage=Rust`, prepends a license header and lint allowances, and copies
the generated `substraittypelexer.rs`, `substraittypeparser.rs` and
`substraittypelistener.rs` into the destination directory.

[ANTLR]: https://www.antlr.org/
[antlr4rust]: https://github.com/antlr4rust/antlr4
[pixi]: https://pixi.sh/
[substrait]: https://github.com/substrait-io/substrait
