#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Generate (or verify) the Rust bindings for the Substrait type-derivation
grammar.

The Rust target for ANTLR is not part of upstream ANTLR; it lives in the
antlr4rust fork (https://github.com/antlr4rust/antlr4) and pairs with the
`antlr4rust` runtime crate. This script downloads the matching fork jar, runs it
with `-Dlanguage=Rust`, and copies the generated lexer/parser/listener into the
crate. The generated files are committed so that an ordinary `cargo build` does
not require a Java runtime.

Usage (via pixi, which provides Java and Python):
    pixi run gen-antlr        # regenerate the committed parser
    pixi run check-codegen    # verify the committed parser is up to date
"""

import filecmp
import hashlib
import re
import shutil
import subprocess
import sys
import tempfile
import urllib.request
from pathlib import Path

# The forked ANTLR jar that supports the Rust target.
ANTLR_URL = "https://github.com/antlr4rust/antlr4/releases/download/v0.5.0/antlr4-4.13.3-SNAPSHOT-complete.jar"
ANTLR_SHA256 = "8227a91d45625aec824e31cc7792ecb8b22b973da71f2396e81542b88d897d8f"

# Paths, relative to the repository root (the parent of this script's directory).
ROOT = Path(__file__).resolve().parent.parent
JAR = ROOT / "antlr" / "antlr.jar"
DEST = ROOT / "src" / "parse" / "text" / "simple_extensions" / "derivations"

# The grammar is fetched from the Substrait repository at the spec tag matching
# the exact `substrait-prost` pin in Cargo.toml (the same pin build.rs uses to
# derive the Substrait version), so bumping that pin is what moves the grammar.
SUBSTRAIT_DEP = "substrait-prost"
GRAMMAR_URL = "https://raw.githubusercontent.com/substrait-io/substrait/v{version}/grammar/{name}"
# The entry grammar and the grammars it imports.
GRAMMAR = "SubstraitType.g4"
GRAMMAR_FILES = [GRAMMAR, "SubstraitLexer.g4"]

# Only these generated files are kept and compiled (ANTLR also emits a base
# listener and `.tokens`/`.interp` files that we do not need).
GENERATED = ["substraittypelexer.rs", "substraittypeparser.rs", "substraittypelistener.rs"]

# Prepended to every generated file: the SPDX header required by CI, plus lint
# allowances (the generated code does not satisfy our `#![deny(missing_docs)]`).
HEADER = (
    "// SPDX-License-Identifier: Apache-2.0\n"
    "#![allow(clippy::all)]\n"
    "#![allow(dead_code)]\n"
    "#![allow(missing_docs)]\n"
    "#![cfg_attr(rustfmt, rustfmt_skip)]\n"
)


def acquire_jar():
    """Download the ANTLR jar if needed and verify its hash."""
    if not JAR.is_file():
        print(f"Downloading {ANTLR_URL}...")
        urllib.request.urlretrieve(ANTLR_URL, JAR)
    digest = hashlib.sha256(JAR.read_bytes()).hexdigest()
    if digest != ANTLR_SHA256:
        sys.exit(f"jar hash mismatch: expected {ANTLR_SHA256}, got {digest}")


def substrait_version():
    """Read the Substrait spec version from the exact `substrait-prost` pin."""
    manifest = (ROOT / "Cargo.toml").read_text()
    match = re.search(rf'^{re.escape(SUBSTRAIT_DEP)}\s*=\s*"=?([^"]+)"', manifest, re.MULTILINE)
    if not match:
        sys.exit(f"could not find the `{SUBSTRAIT_DEP}` version pin in Cargo.toml")
    return match.group(1)


def fetch_grammar(out_dir):
    """Download the grammar files for the pinned spec version into out_dir."""
    version = substrait_version()
    for name in GRAMMAR_FILES:
        url = GRAMMAR_URL.format(version=version, name=name)
        print(f"Downloading {url}...")
        urllib.request.urlretrieve(url, out_dir / name)


def generate(out_dir):
    """Run ANTLR into out_dir, returning the generated (header-prepended) files.

    The grammar and the grammars it imports (e.g. `import SubstraitLexer;`) are
    placed next to each other so that imports resolve.
    """
    fetch_grammar(out_dir)
    subprocess.run(
        ["java", "-jar", str(JAR), "-Dlanguage=Rust", GRAMMAR],
        cwd=out_dir,
        check=True,
    )
    for name in GENERATED:
        f = out_dir / name
        if not f.is_file():
            sys.exit(f"ANTLR did not generate {name}")
        f.write_text(HEADER + f.read_text())


def main(check):
    acquire_jar()
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        generate(tmp)
        for name in GENERATED:
            new, committed = tmp / name, DEST / name
            if check:
                if not committed.is_file() or not filecmp.cmp(new, committed, shallow=False):
                    sys.exit(f"{committed} is out of date; run `pixi run gen-antlr`")
            else:
                DEST.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(new, committed)
    print("up to date" if check else "regenerated")


if __name__ == "__main__":
    main(check="--ci-check" in sys.argv[1:])
