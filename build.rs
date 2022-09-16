// SPDX-License-Identifier: Apache-2.0

use std::error::Error;
#[cfg(feature = "pbjson")]
use std::{env, fs, path::PathBuf};

#[cfg(feature = "pbjson")]
use pbjson_build::Builder;
use prost_build::Config;
use walkdir::{DirEntry, WalkDir};

const PROTO_ROOT: &str = "substrait/proto";

fn main() -> Result<(), Box<dyn Error>> {
    // for use in docker build where file changes can be wonky
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD");

    let protos = WalkDir::new(PROTO_ROOT)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().is_file()
                && entry
                    .path()
                    .extension()
                    .filter(|extension| extension == &"proto")
                    .is_some()
        })
        .map(DirEntry::into_path)
        .inspect(|entry| {
            println!("cargo:rerun-if-changed={}", entry.display());
        })
        .collect::<Vec<_>>();

    #[cfg(feature = "pbjson")]
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");

    let mut cfg = Config::new();
    #[cfg(feature = "pbjson")]
    cfg.file_descriptor_set_path(&descriptor_path)
        .compile_well_known_types()
        .extern_path(".google.protobuf", "::pbjson_types");

    cfg.compile_protos(&protos, &[PROTO_ROOT])?;

    #[cfg(feature = "pbjson")]
    Builder::new()
        .register_descriptors(&fs::read(&descriptor_path)?)?
        .build(&[".substrait"])?;

    Ok(())
}
