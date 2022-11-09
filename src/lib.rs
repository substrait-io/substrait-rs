// SPDX-License-Identifier: Apache-2.0

/// Generated types for the protobuf `substrait` package
#[allow(clippy::all)]
pub mod protobuf {

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

#[cfg(test)]
mod tests {
    use crate::protobuf::expression::{literal::LiteralType, Literal};
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
        let plan: crate::protobuf::Plan = serde_json::from_str(
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
