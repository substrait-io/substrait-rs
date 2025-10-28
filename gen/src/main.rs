// SPDX-License-Identifier: Apache-2.0

use prost_build::Config;
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};
use walkdir::{DirEntry, WalkDir};

const SUBMODULE_ROOT: &str = "../substrait";
const EXTENSIONS_ROOT: &str = "../substrait/extensions";
const PROTO_ROOT: &str = "../substrait/proto";
const TEXT_ROOT: &str = "../substrait/text";
const GEN_ROOT: &str = "../src/gen";

/// Add Substrait version information to the build
fn substrait_version(out_dir: &Path) -> Result<semver::Version, Box<dyn Error>> {
    let version_in_file = out_dir.join("version.rs");
    let substrait_version_file = out_dir.join("version");

    // Check if there is a submodule. This file is not included in the packaged crate.
    if Path::new(SUBMODULE_ROOT).join(".git").exists() {
        // Get the version of the submodule by directly calling `git describe`.
        let git_describe = String::from_utf8(
            Command::new("git")
                .current_dir(SUBMODULE_ROOT)
                .arg("describe")
                .arg("--tags")
                .arg("--long")
                .arg("--dirty=-dirty")
                .arg("--abbrev=40")
                .output()?
                .stdout,
        )?;

        // Extract the parts.
        let mut split = git_describe.split('-');
        let git_version = split.next().unwrap_or_default();
        let git_depth = split.next().unwrap_or_default();
        let git_hash = split.next().unwrap_or_default().trim_end();
        let git_dirty = git_describe.ends_with("dirty");
        let version = semver::Version::parse(git_version.trim_start_matches('v'))?;

        let &semver::Version {
            major,
            minor,
            patch,
            ..
        } = &version;

        fs::write(
            version_in_file,
            format!(
                r#"// SPDX-License-Identifier: Apache-2.0

// Note that this file is auto-generated.

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

/// A constant with the Substrait version as name, to trigger semver bumps when
/// the Substrait version changes.
pub const SUBSTRAIT_{major}_{minor}: () = ();
"#
            ),
        )?;

        // Also write the version to a file
        fs::write(substrait_version_file, version.to_string())?;

        Ok(version)
    } else {
        // If we don't have a version file yet we fail the build.
        if !version_in_file.exists() {
            panic!(
                "Couldn't find the substrait submodule. Please clone the submodule: `git submodule update --init`."
            )
        }

        // File exists we should get the version and return it.
        Ok(semver::Version::parse(&fs::read_to_string(
            substrait_version_file,
        )?)?)
    }
}

/// `text` type generation
fn text(out_dir: &Path) -> Result<(), Box<dyn Error>> {
    use heck::ToSnakeCase;
    use schemars::schema::{RootSchema, Schema};
    use typify::{TypeSpace, TypeSpaceSettings};
    let mut out_file = File::create(out_dir.join("text").join("substrait").with_extension("rs"))?;

    for schema_path in WalkDir::new(TEXT_ROOT)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file() || entry.file_type().is_symlink())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .filter(|&extension| extension == "yaml") // Option::contains
                .is_some()
        })
        .map(DirEntry::into_path)
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
    {}
}}"#,
            prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream())?),
        ))?;
    }
    Ok(())
}

/// Add Substrait core extensions
fn extensions(version: semver::Version, out_dir: &Path) -> Result<(), Box<dyn Error>> {
    let substrait_extensions_file = out_dir.join("extensions").join("substrait.rs");

    let mut output = String::from(
        r#"// SPDX-License-Identifier: Apache-2.0
// Note that this file is auto-generated and auto-synced using `substrait-gen`.
"#,
    );
    let mut map = Vec::default();
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
const {var_name}: &str = include_str!("../../{}");
"#,
            extension.display()
        ));
        let urn = format!(
            "extension:io.substrait:{}",
            extension
                .with_extension("")
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
        );
        map.push((urn, var_name));
    }
    // Add static lookup map.
    output.push_str(
        r#"
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;
use crate::text::simple_extensions::SimpleExtensions;
use crate::urn::Urn;

/// Map with Substrait core extensions. Maps Urns to included extensions.
pub static EXTENSIONS: LazyLock<HashMap<Urn, SimpleExtensions>> = LazyLock::new(|| {
    let mut map = HashMap::new();"#,
    );

    for (urn, var_name) in map {
        output.push_str(&format!(
            r#"
    map.insert(Urn::from_str("{urn}").expect("valid urn"), serde_yaml::from_str({var_name}).expect("a valid core extension"));"#
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

/// Serialize and deserialize implementations for proto types using `pbjson`
fn serde(protos: &[impl AsRef<Path>], out_dir: &Path) -> Result<(), Box<dyn Error>> {
    use pbjson_build::Builder;

    let descriptor_path = out_dir
        .join("proto")
        .join("pbjson")
        .join("proto_descriptor.bin");
    let mut cfg = Config::new();
    cfg.out_dir(out_dir.join("proto").join("pbjson"));
    cfg.file_descriptor_set_path(&descriptor_path);
    cfg.compile_well_known_types()
        .extern_path(".google.protobuf", "::pbjson_types")
        .compile_protos(protos, &[PROTO_ROOT])?;

    Builder::new()
        .register_descriptors(&fs::read(descriptor_path)?)?
        .out_dir(out_dir.join("proto").join("pbjson"))
        .build(&[".substrait"])?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = Path::new(GEN_ROOT);
    fs::create_dir_all(out_dir)?;

    let version = substrait_version(out_dir)?;
    text(out_dir)?;
    extensions(version, out_dir)?;

    let protos = WalkDir::new(PROTO_ROOT)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file() || entry.file_type().is_symlink())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .filter(|&extension| extension == "proto")
                .is_some()
        })
        .map(DirEntry::into_path)
        .collect::<Vec<_>>();

    Config::new()
        .out_dir(out_dir.join("proto"))
        .compile_protos(&protos, &[PROTO_ROOT])?;

    serde(&protos, out_dir)
}
