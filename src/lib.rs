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
}

/// Generated types for text-based definitions
pub mod text {
    include!(concat!(env!("OUT_DIR"), "/substrait_text.rs"));
}

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
