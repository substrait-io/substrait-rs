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
GRAMMAR = ROOT / "substrait" / "grammar" / "SubstraitType.g4"
DEST = ROOT / "src" / "parse" / "text" / "simple_extensions" / "derivations"

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


def generate(out_dir):
    """Run ANTLR into out_dir, returning the generated (header-prepended) files.

    All `.g4` files next to the grammar are made available so that grammar
    imports (e.g. `import SubstraitLexer;`) resolve.
    """
    for g4 in GRAMMAR.parent.glob("*.g4"):
        shutil.copyfile(g4, out_dir / g4.name)
    subprocess.run(
        ["java", "-jar", str(JAR), "-Dlanguage=Rust", GRAMMAR.name],
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
