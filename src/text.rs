// SPDX-License-Identifier: Apache-2.0

//! Generated types for text-based definitions

// https://github.com/oxidecomputer/typify/issues/245
#![allow(clippy::clone_on_copy)]

include!(concat!(env!("OUT_DIR"), "/substrait_text.rs"));

#[cfg(test)]
mod tests {
    use crate::text::simple_extensions::SimpleExtensions;
    use std::{fs, path::PathBuf};
    use walkdir::{DirEntry, WalkDir};

    #[test]
    fn deserialize_core_extensions() {
        WalkDir::new(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("substrait/extensions"))
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .filter(|extension| extension == &"yaml")
                    .is_some()
            })
            .map(DirEntry::into_path)
            .for_each(|path| {
                let file = fs::read_to_string(&path).unwrap();
                let simple_extension = serde_yaml::from_str::<SimpleExtensions>(&file);
                assert!(simple_extension.is_ok());
            });
    }
}
