// SPDX-License-Identifier: Apache-2.0

/// Generated types for the protobuf `substrait` package
#[allow(clippy::all)]
pub mod proto {

    /// Generated types for the protobuf `substrait.extensions` package
    pub mod extensions {
        include!(concat!(env!("OUT_DIR"), "/substrait.extensions.rs"));

        #[cfg(feature = "pbjson")]
        include!(concat!(env!("OUT_DIR"), "/substrait.extensions.serde.rs"));
    }

    include!(concat!(env!("OUT_DIR"), "/substrait.rs"));

    #[cfg(feature = "pbjson")]
    include!(concat!(env!("OUT_DIR"), "/substrait.serde.rs"));

    #[cfg(test)]
    mod tests {
        use crate::proto::expression::{literal::LiteralType, Literal};
        #[cfg(feature = "pbjson")]
        use std::error::Error;

        #[test]
        fn literal() {
            let _ = Literal {
                nullable: true,
                literal_type: Some(LiteralType::I32(123)),
                type_variation_reference: 0,
            };
        }

        #[cfg(feature = "pbjson")]
        #[test]
        fn pbjson_serde() -> Result<(), Box<dyn Error>> {
            let plan: crate::proto::Plan = serde_json::from_str(
                r#"{
                "relations": [
                    {
                        "root": {
                           "names": ["substrait"]
                        }
                    },
                    {
                        "rel": {
                            "filter": {
                                "condition": {
                                    "literal": {
                                        "nullable": true,
                                        "boolean": true
                                    }
                                }
                            }
                        }
                    }
                ]
            }"#,
            )?;
            assert_eq!(plan.relations.len(), 2);
            Ok(())
        }
    }
}

/// Generated types for text-based definitions
pub mod text {
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
                .filter(|entry| {
                    entry.file_type().is_file()
                        && entry
                            .path()
                            .extension()
                            .filter(|extension| extension == &"yaml")
                            .is_some()
                })
                .map(DirEntry::into_path)
                .for_each(|path| {
                    let file = fs::read_to_string(path).unwrap();
                    assert!(serde_yaml::from_str::<SimpleExtensions>(&file).is_ok());
                });
        }
    }
}
