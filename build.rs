// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "pbjson")]
use std::fs;
use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

use heck::ToSnakeCase;
#[cfg(feature = "pbjson")]
use pbjson_build::Builder;
use prost_build::Config;
use schemars::schema::{RootSchema, Schema};
use typify::{TypeSpace, TypeSpaceSettings};
use walkdir::{DirEntry, WalkDir};

const PROTO_ROOT: &str = "substrait/proto";
const TEXT_ROOT: &str = "substrait/text";

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

    let mut substrait_text = File::create(
        PathBuf::from(env::var("OUT_DIR").unwrap())
            .join("substrait_text")
            .with_extension("rs"),
    )?;
    for schema_path in WalkDir::new(TEXT_ROOT)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().is_file()
                && entry
                    .path()
                    .extension()
                    .filter(|extension| extension == &"yaml")
                    .is_some()
        })
        .map(DirEntry::into_path)
        .inspect(|entry| {
            println!("cargo:rerun-if-changed={}", entry.display());
        })
    {
        let schema = serde_yaml::from_reader::<_, RootSchema>(File::open(&schema_path)?)?;
        let metadata = schema.schema.metadata.as_ref();
        let id = metadata
            .and_then(|metadata| metadata.id.as_ref())
            .expect("$id missing in schema")
            .to_string();
        let title = metadata
            .and_then(|metadata| metadata.title.as_ref())
            .map(|title| title.to_snake_case())
            .expect("title missing in schema");
        let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
        type_space.add_ref_types(schema.definitions)?;
        type_space.add_type(&Schema::Object(schema.schema))?;
        let content = type_space.to_string();
        substrait_text.write_fmt(format_args!(
            r#"
#[doc = "Generated types for `{id}`"]
pub mod {title} {{
    use serde::{{Deserialize, Serialize}};
    {content}
}}"#
        ))?;
    }

    Ok(())
}
