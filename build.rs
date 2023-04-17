// SPDX-License-Identifier: Apache-2.0

use std::{
    env,
    error::Error,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use gix::bstr::ByteSlice;
use prost_build::Config;
use walkdir::{DirEntry, WalkDir};

#[cfg(feature = "extensions")]
const EXTENSIONS_ROOT: &str = "substrait/extensions";
const PROTO_ROOT: &str = "substrait/proto";
const TEXT_ROOT: &str = "substrait/text";

#[cfg(all(feature = "serde", feature = "pbjson"))]
compile_error!("Either feature `serde` or `pbjson` can be enabled");

/// Add Substrait version information to the build
fn substrait_version() -> Result<semver::Version, Box<dyn Error>> {
    use gix::commit::describe::SelectRef;
    use std::process::Command;

    let substrait_version_in_file =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("src/version.in");
    let substrait_version_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("src/version");

    // Or if the Substrait submodule HEAD changed
    println!(
        "cargo:rerun-if-changed={}",
        Path::new(".git/modules/substrait/HEAD").display()
    );
    // Or if the Substrait submodule changed (to allow setting `dirty`)
    println!(
        "cargo:rerun-if-changed={}",
        Path::new("substrait").display()
    );

    // Get the version from the submodule
    match gix::open("substrait") {
        Ok(repo) => {
            let mut format = repo
                .head_commit()?
                .describe()
                .names(SelectRef::AllTags)
                .try_resolve()?
                .expect("substrait submodule tags missing")
                .format()?;
            format.long(true);

            // TODO(mbrobbel): replace with something from `git-repository`
            let git_dirty = !Command::new("git")
                .current_dir("substrait")
                .arg("status")
                .arg("--porcelain")
                .output()?
                .stdout
                .is_empty();
            let git_depth = format.depth;
            let git_hash = format.id.to_hex_with_len(40).to_string();
            // TODO(mbrobbel): use `git-repository` dirty state support
            let git_describe = format!("{format}{}", if git_dirty { "-dirty" } else { "" });
            let version = semver::Version::parse(
                format
                    .name
                    .as_ref()
                    .expect("substrait submodule tag missing")
                    .to_str()?
                    .trim_start_matches('v'),
            )?;

            let &semver::Version {
                major,
                minor,
                patch,
                ..
            } = &version;

            fs::write(
                substrait_version_in_file,
                format!(
                    r#"// SPDX-License-Identifier: Apache-2.0

// Note that this file is auto-generated and auto-synced using `build.rs`. It is
// included in `version.rs`.

/// The major version of Substrait used to build this crate
pub const SUBSTRAIT_MAJOR_VERSION: u32 = {major};

/// The minor version of Substrait used to build this crate
pub const SUBSTRAIT_MINOR_VERSION: u32 = {minor};

/// The patch version of Substrait used to build this crate
pub const SUBSTRAIT_PATCH_VERSION: u32 = {patch};

/// The Git SHA (lower hex) of Substrait used to build this crate
pub const SUBSTRAIT_GIT_SHA: &str = "{git_hash}";

/// The `git describe` output of the Substrait submodule used to build this
/// crate
pub const SUBSTRAIT_GIT_DESCRIBE: &str = "{git_describe}";

/// The amount of commits between the latest tag and the version of the
/// Substrait submodule used to build this crate
pub const SUBSTRAIT_GIT_DEPTH: u32 = {git_depth};

/// The dirty state of the Substrait submodule used to build this crate
pub const SUBSTRAIT_GIT_DIRTY: bool = {git_dirty};
"#
                ),
            )?;

            // Also write the version to a file
            fs::write(substrait_version_file, version.to_string())?;

            Ok(version)
        }
        Err(e) => {
            // If this is a package build the `substrait_version_file` should
            // exist. If it does not, it means this is probably a Git build that
            // did not clone the substrait submodule.
            if !substrait_version_file.exists() {
                panic!("Couldn't open the substrait submodule: {e}")
            }

            // File exists we should get the version and return it.
            Ok(semver::Version::parse(&fs::read_to_string(
                substrait_version_file,
            )?)?)
        }
    }
}

/// `text` type generation
fn text(out_dir: &Path) -> Result<(), Box<dyn Error>> {
    use heck::ToSnakeCase;
    use schemars::schema::{RootSchema, Schema};
    use typify::{TypeSpace, TypeSpaceSettings};

    let mut out_file = File::create(out_dir.join("substrait_text").with_extension("rs"))?;

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
            prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream())?),
        ))?;
    }
    Ok(())
}

#[cfg(feature = "extensions")]
/// Add Substrait core extensions
fn extensions(version: semver::Version) -> Result<(), Box<dyn Error>> {
    use std::collections::HashMap;

    let substrait_extensions_file =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("src/extensions.in");

    let mut output = String::from(
        r#"// SPDX-License-Identifier: Apache-2.0

// Note that this file is auto-generated and auto-synced using `build.rs`. It is
// included in `extensions.rs`.
"#,
    );
    let mut map = HashMap::<String, String>::default();
    for extension in WalkDir::new(EXTENSIONS_ROOT)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .filter(|&extension| extension == "yaml")
                .is_some()
        })
        .map(DirEntry::into_path)
        .inspect(|entry| {
            println!("cargo:rerun-if-changed={}", entry.display());
        })
    {
        let name = extension.file_stem().unwrap_or_default().to_string_lossy();
        let url = format!(
            "https://github.com/substrait-io/substrait/raw/v{}/extensions/{}",
            version,
            extension.file_name().unwrap_or_default().to_string_lossy()
        );
        let var_name = name.to_uppercase();
        output.push_str(&format!(
            r#"
/// Included source of [`{name}`]({url}).
pub const {var_name}: &str = include_str!("../{}");
"#,
            extension.display()
        ));
        map.insert(url, var_name);
    }
    // Add static lookup map.
    output.push_str(
        r#"
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Map with Substrait core extensions. Maps URIs to included extension source strings.
pub static EXTENSIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();"#,
    );
    for (url, var_name) in map {
        output.push_str(&format!(
            r#"
    map.insert("{url}", {var_name});"#,
        ));
    }
    output.push_str(
        r#"
    map
});"#,
    );

    // Write the file.
    fs::write(substrait_extensions_file, output)?;

    Ok(())
}

#[cfg(feature = "serde")]
/// Serialize deserialize implementations for proto types using `serde`
fn serde(protos: &[impl AsRef<Path>], out_dir: PathBuf) -> Result<(), Box<dyn Error>> {
    use prost_types::DescriptorProto;
    use prost_wkt_build::{FileDescriptorSet, Message};

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

    let _version = substrait_version()?;

    #[cfg(feature = "protoc")]
    std::env::set_var("PROTOC", protobuf_src::protoc());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    text(out_dir.as_path())?;

    #[cfg(feature = "extensions")]
    extensions(_version)?;

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
