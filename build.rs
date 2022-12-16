// SPDX-License-Identifier: Apache-2.0

use std::{
    env,
    error::Error,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use prost_build::Config;
use walkdir::{DirEntry, WalkDir};

const PROTO_ROOT: &str = "substrait/proto";
const TEXT_ROOT: &str = "substrait/text";

#[cfg(all(feature = "serde", feature = "pbjson"))]
compile_error!("Either feature `serde` or `pbjson` can be enabled");

/// `text` type generation
fn text(out_dir: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    use heck::ToSnakeCase;
    use schemars::schema::{RootSchema, Schema};
    use typify::{TypeSpace, TypeSpaceSettings};

    let mut out_file = File::create(out_dir.as_ref().join("substrait_text").with_extension("rs"))?;

    for schema_path in WalkDir::new(TEXT_ROOT)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .filter(|&extension| extension == "yaml") // Option::contains
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
            .map(ToString::to_string)
            .unwrap_or_else(|| {
                panic!(
                    "$id missing in schema metadata (`{}`)",
                    schema_path.display()
                )
            });
        let title = metadata
            .and_then(|metadata| metadata.title.as_ref())
            .map(|title| title.to_snake_case())
            .unwrap_or_else(|| {
                panic!(
                    "title missing in schema metadata (`{}`)",
                    schema_path.display()
                )
            });
        let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
        type_space.add_ref_types(schema.definitions)?;
        type_space.add_type(&Schema::Object(schema.schema))?;
        out_file.write_fmt(format_args!(
            r#"
#[doc = "Generated types for `{id}`"]
pub mod {title} {{
    use serde::{{Deserialize, Serialize}};
    {}
}}"#,
            type_space.to_string()
        ))?;
    }
    Ok(())
}

#[cfg(feature = "serde")]
/// Serialize deserialize implementations for proto types using `serde`
fn serde(protos: &[impl AsRef<Path>], out_dir: PathBuf) -> Result<(), Box<dyn Error>> {
    use prost_types::DescriptorProto;
    use prost_wkt_build::{FileDescriptorSet, Message};
    use std::fs;

    let descriptor_path = out_dir.join("proto_descriptor.bin");

    fn serde_default(cfg: &mut Config, dp: Vec<DescriptorProto>, path: String) {
        dp.into_iter().for_each(move |descriptor| {
            let name = descriptor.name().to_string();
            cfg.type_attribute(format!("{path}.{name}"), "#[serde(default)]");
            serde_default(cfg, descriptor.nested_type, format!("{path}.{name}"))
        });
    }

    let mut cfg = Config::new();
    cfg.file_descriptor_set_path(&descriptor_path)
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
        .compile_protos(protos, &[PROTO_ROOT])?;

    FileDescriptorSet::decode(&mut fs::read(&descriptor_path)?.as_slice())?
        .file
        .into_iter()
        .for_each(|fdp| {
            let package = fdp.package().into();
            serde_default(&mut cfg, fdp.message_type, package)
        });

    cfg.skip_protoc_run()
        .compile_protos(protos, &[PROTO_ROOT])?;

    prost_wkt_build::add_serde(
        out_dir,
        FileDescriptorSet::decode(fs::read(descriptor_path)?.as_slice())?,
    );

    Ok(())
}

#[cfg(feature = "pbjson")]
/// Serialize and deserialize implementations for proto types using `pbjson`
fn pbjson(protos: &[impl AsRef<Path>], out_dir: PathBuf) -> Result<(), Box<dyn Error>> {
    use pbjson_build::Builder;
    use std::fs;

    let descriptor_path = out_dir.join("proto_descriptor.bin");
    let mut cfg = Config::new();
    cfg.file_descriptor_set_path(&descriptor_path);
    cfg.compile_well_known_types()
        .extern_path(".google.protobuf", "::pbjson_types")
        .compile_protos(protos, &[PROTO_ROOT])?;

    Builder::new()
        .register_descriptors(&fs::read(descriptor_path)?)?
        .build(&[".substrait"])?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // for use in docker build where file changes can be wonky
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD");

    #[cfg(feature = "protoc")]
    std::env::set_var("PROTOC", protobuf_src::protoc());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    text(&out_dir)?;

    let protos = WalkDir::new(PROTO_ROOT)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .filter(|&extension| extension == "proto")
                .is_some()
        })
        .map(DirEntry::into_path)
        .inspect(|entry| {
            println!("cargo:rerun-if-changed={}", entry.display());
        })
        .collect::<Vec<_>>();

    #[cfg(feature = "pbjson")]
    pbjson(&protos, out_dir)?;

    #[cfg(feature = "serde")]
    serde(&protos, out_dir)?;

    #[cfg(not(any(feature = "serde", feature = "pbjson")))]
    Config::new().compile_protos(&protos, &[PROTO_ROOT])?;

    Ok(())
}
