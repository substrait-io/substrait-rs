// SPDX-License-Identifier: Apache-2.0

use prost_build::Config;
use std::{
    env,
    error::Error,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

const PROTO_ROOT: &str = "substrait/proto";
const TEXT_ROOT: &str = "substrait/text";

// Release builds should already have the substrait version file generated.
// We don't want to require git2 in those cases
#[cfg(not(feature = "git2"))]
fn substrait_version() -> Result<(), Box<dyn Error>> {
    let substrait_version_file =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("src/version.in");
    if !substrait_version_file.exists() {
        panic!("Couldn't open any pre-calculated substrait version file.  If this is a development build then please enable the git2 feature.")
    }
    Ok(())
}

/// Add Substrait version information to the build
#[cfg(feature = "git2")]
fn substrait_version() -> Result<(), Box<dyn Error>> {
    use fs;
    use git2::{DescribeFormatOptions, DescribeOptions, Repository};

    let substrait_version_file =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("src/version.in");

    // Rerun if the Substrait submodule changed (to allow setting `dirty`)
    println!(
        "cargo:rerun-if-changed={}",
        Path::new("substrait").display()
    );

    // Get the version from the submodule
    match Repository::open("substrait") {
        Ok(repo) => {
            // Rerun if the Substrait submodule HEAD changed (when there is a submodule)
            println!(
                "cargo:rerun-if-changed={}",
                Path::new(".git/modules/substrait/HEAD").display()
            );

            // Get describe output
            let mut describe_options = DescribeOptions::default();
            describe_options.describe_tags();
            let mut describe_format_options = DescribeFormatOptions::default();
            describe_format_options.always_use_long_format(true);
            describe_format_options.dirty_suffix("-dirty");
            let git_describe = repo
                .describe(&describe_options)?
                .format(Some(&describe_format_options))?;

            let mut split = git_describe.split('-');
            let git_version = split.next().unwrap_or_default();
            let git_depth = split.next().unwrap_or_default();
            let git_dirty = git_describe.ends_with("dirty");
            let git_hash = repo.head()?.peel_to_commit()?.id().to_string();
            let semver::Version {
                major,
                minor,
                patch,
                ..
            } = semver::Version::parse(git_version.trim_start_matches('v'))?;

            fs::write(
                substrait_version_file,
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

/// The amount of commits between the latest tag and this version of the
/// Substrait module used to build this crate
pub const SUBSTRAIT_GIT_DEPTH: u32 = {git_depth};

/// The dirty state of the Substrait submodule used to build this crate
pub const SUBSTRAIT_GIT_DIRTY: bool = {git_dirty};
"#
                ),
            )?;
        }
        Err(e) => {
            // If this is a package build the `substrait_version_file` should
            // exist. If it does not, it means this is probably a Git build that
            // did not clone the substrait submodule.
            if !substrait_version_file.exists() {
                panic!("Couldn't open the substrait submodule: {e}")
            }
        }
    };

    Ok(())
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

#[cfg(feature = "serde")]
/// Serialize and deserialize implementations for proto types using `pbjson`
fn serde(protos: &[impl AsRef<Path>], out_dir: PathBuf) -> Result<(), Box<dyn Error>> {
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

    substrait_version()?;

    #[cfg(feature = "protoc")]
    std::env::set_var("PROTOC", protobuf_src::protoc());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    text(out_dir.as_path())?;

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

    #[cfg(feature = "serde")]
    serde(&protos, out_dir)?;

    #[cfg(not(feature = "serde"))]
    Config::new().compile_protos(&protos, &[PROTO_ROOT])?;

    Ok(())
}
