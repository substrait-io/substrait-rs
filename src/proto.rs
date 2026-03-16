// SPDX-License-Identifier: Apache-2.0

#![allow(
    clippy::doc_overindented_list_items,
    clippy::large_enum_variant,
    clippy::needless_borrow,
    clippy::needless_borrows_for_generic_args,
    clippy::needless_lifetimes,
    rustdoc::invalid_html_tags
)]

//! Generated types for the protobuf `substrait` package.

/// Generated types for the protobuf `substrait.extensions` package
pub mod extensions {
    include!(concat!(env!("OUT_DIR"), "/substrait.extensions.rs"));

    #[cfg(feature = "serde")]
    include!(concat!(env!("OUT_DIR"), "/substrait.extensions.serde.rs"));
}

include!(concat!(env!("OUT_DIR"), "/substrait.rs"));

#[cfg(feature = "serde")]
include!(concat!(env!("OUT_DIR"), "/substrait.serde.rs"));

/// The encoded file descriptor set for the Substrait protobuf definitions.
///
/// This can be used for protobuf reflection.
#[cfg(feature = "embed-descriptor")]
pub const FILE_DESCRIPTOR_SET: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/proto_descriptor.bin"));

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    #[test]
    fn pbjson_serde() -> Result<(), Box<dyn std::error::Error>> {
        let plan = serde_json::from_str::<super::Plan>(
            r#"{
  "version": { "minorNumber": 75, "producer": "substrait-rs" },
  "extensionUrns": [
    {
      "extensionUrnAnchor": 1,
      "urn": "extension:io.substrait:functions_string"
    }
  ]
}"#,
        )?;
        assert_eq!(
            plan.version,
            Some(super::Version {
                minor_number: 75,
                producer: "substrait-rs".into(),
                ..Default::default()
            })
        );
        assert_eq!(plan.extension_urns.len(), 1);
        Ok(())
    }

    #[cfg(feature = "serde")]
    #[test]
    fn forward_compatible_unknown_fields() -> Result<(), Box<dyn std::error::Error>> {
        // Test that unknown fields are ignored for forward compatibility
        let plan = serde_json::from_str::<super::Plan>(
            r#"{
  "version": { "minorNumber": 75, "producer": "substrait-rs" },
  "unknownField": "this field doesn't exist in the proto",
  "anotherUnknownField": {"nested": "data"},
  "extensionUrns": [
    {
      "extensionUrnAnchor": 1,
      "urn": "extension:io.substrait:functions_string",
      "futureField": "should be ignored"
    }
  ]
}"#,
        )?;
        assert_eq!(
            plan.version,
            Some(super::Version {
                minor_number: 75,
                producer: "substrait-rs".into(),
                ..Default::default()
            })
        );
        assert_eq!(plan.extension_urns.len(), 1);
        Ok(())
    }

    #[cfg(feature = "embed-descriptor")]
    #[test]
    fn file_descriptor_set_is_valid() {
        use prost::Message;
        let fds = prost_types::FileDescriptorSet::decode(super::FILE_DESCRIPTOR_SET).unwrap();
        assert!(
            fds.file.iter().any(|f| f.package() == "substrait"),
            "expected substrait package in file descriptor set"
        );
    }

    #[test]
    fn dynamic_parameter_expression_roundtrip() {
        use super::expression::RexType;
        use prost::Message;

        let dp = super::DynamicParameter {
            r#type: Some(super::Type {
                kind: Some(super::r#type::Kind::I32(super::r#type::I32 {
                    type_variation_reference: 0,
                    nullability: super::r#type::Nullability::Required as i32,
                })),
            }),
            parameter_reference: 1,
        };

        let expr = super::Expression {
            rex_type: Some(RexType::DynamicParameter(dp.clone())),
        };

        let encoded = expr.encode_to_vec();
        let decoded = super::Expression::decode(encoded.as_slice()).unwrap();

        assert_eq!(expr, decoded);

        match decoded.rex_type.unwrap() {
            RexType::DynamicParameter(decoded_dp) => {
                assert_eq!(decoded_dp.parameter_reference, 1);
                assert!(decoded_dp.r#type.is_some());
            }
            other => panic!("expected DynamicParameter, got {other:?}"),
        }
    }

    #[test]
    fn dynamic_parameter_binding_roundtrip() {
        use prost::Message;

        let binding = super::DynamicParameterBinding {
            parameter_anchor: 1,
            value: Some(super::expression::Literal {
                nullable: false,
                type_variation_reference: 0,
                literal_type: Some(super::expression::literal::LiteralType::I32(42)),
            }),
        };

        let encoded = binding.encode_to_vec();
        let decoded = super::DynamicParameterBinding::decode(encoded.as_slice()).unwrap();

        assert_eq!(binding, decoded);
        assert_eq!(decoded.parameter_anchor, 1);
        assert_eq!(
            decoded.value.unwrap().literal_type,
            Some(super::expression::literal::LiteralType::I32(42))
        );
    }

    #[test]
    fn plan_with_dynamic_parameter_roundtrip() {
        use super::expression::RexType;
        use prost::Message;

        let dp_expr = super::Expression {
            rex_type: Some(RexType::DynamicParameter(super::DynamicParameter {
                r#type: Some(super::Type {
                    kind: Some(super::r#type::Kind::String(super::r#type::String {
                        type_variation_reference: 0,
                        nullability: super::r#type::Nullability::Nullable as i32,
                    })),
                }),
                parameter_reference: 0,
            })),
        };

        let plan = super::Plan {
            version: Some(super::Version {
                minor_number: 85,
                producer: "substrait-rs-test".into(),
                ..Default::default()
            }),
            relations: vec![super::PlanRel {
                rel_type: Some(super::plan_rel::RelType::Root(super::RelRoot {
                    input: Some(super::Rel {
                        rel_type: Some(super::rel::RelType::Filter(Box::new(super::FilterRel {
                            common: None,
                            input: Some(Box::new(super::Rel {
                                rel_type: Some(super::rel::RelType::Read(Box::new(
                                    super::ReadRel {
                                        common: None,
                                        base_schema: None,
                                        filter: None,
                                        best_effort_filter: None,
                                        projection: None,
                                        advanced_extension: None,
                                        read_type: None,
                                    },
                                ))),
                            })),
                            condition: Some(Box::new(dp_expr)),
                            advanced_extension: None,
                        }))),
                    }),
                    names: vec!["output".into()],
                })),
            }],
            parameter_bindings: vec![super::DynamicParameterBinding {
                parameter_anchor: 0,
                value: Some(super::expression::Literal {
                    nullable: false,
                    type_variation_reference: 0,
                    literal_type: Some(super::expression::literal::LiteralType::String(
                        "hello".into(),
                    )),
                }),
            }],
            ..Default::default()
        };

        let encoded = plan.encode_to_vec();
        let decoded = super::Plan::decode(encoded.as_slice()).unwrap();

        assert_eq!(plan, decoded);
        assert_eq!(decoded.parameter_bindings.len(), 1);
        assert_eq!(decoded.parameter_bindings[0].parameter_anchor, 0);
    }

    #[test]
    fn multiple_dynamic_parameters_in_plan() {
        use super::expression::RexType;
        use prost::Message;

        let dp0 = super::Expression {
            rex_type: Some(RexType::DynamicParameter(super::DynamicParameter {
                r#type: Some(super::Type {
                    kind: Some(super::r#type::Kind::I32(super::r#type::I32 {
                        type_variation_reference: 0,
                        nullability: super::r#type::Nullability::Required as i32,
                    })),
                }),
                parameter_reference: 0,
            })),
        };

        let dp1 = super::Expression {
            rex_type: Some(RexType::DynamicParameter(super::DynamicParameter {
                r#type: Some(super::Type {
                    kind: Some(super::r#type::Kind::Fp64(super::r#type::Fp64 {
                        type_variation_reference: 0,
                        nullability: super::r#type::Nullability::Nullable as i32,
                    })),
                }),
                parameter_reference: 1,
            })),
        };

        let plan = super::Plan {
            version: Some(super::Version {
                minor_number: 85,
                producer: "substrait-rs-test".into(),
                ..Default::default()
            }),
            relations: vec![super::PlanRel {
                rel_type: Some(super::plan_rel::RelType::Root(super::RelRoot {
                    input: Some(super::Rel {
                        rel_type: Some(super::rel::RelType::Project(Box::new(super::ProjectRel {
                            common: None,
                            input: Some(Box::new(super::Rel {
                                rel_type: Some(super::rel::RelType::Read(Box::new(
                                    super::ReadRel {
                                        common: None,
                                        base_schema: None,
                                        filter: None,
                                        best_effort_filter: None,
                                        projection: None,
                                        advanced_extension: None,
                                        read_type: None,
                                    },
                                ))),
                            })),
                            expressions: vec![dp0, dp1],
                            advanced_extension: None,
                        }))),
                    }),
                    names: vec!["col0".into(), "col1".into()],
                })),
            }],
            parameter_bindings: vec![
                super::DynamicParameterBinding {
                    parameter_anchor: 0,
                    value: Some(super::expression::Literal {
                        nullable: false,
                        type_variation_reference: 0,
                        literal_type: Some(super::expression::literal::LiteralType::I32(100)),
                    }),
                },
                super::DynamicParameterBinding {
                    parameter_anchor: 1,
                    value: Some(super::expression::Literal {
                        nullable: true,
                        type_variation_reference: 0,
                        literal_type: Some(super::expression::literal::LiteralType::Fp64(1.5)),
                    }),
                },
            ],
            ..Default::default()
        };

        let encoded = plan.encode_to_vec();
        let decoded = super::Plan::decode(encoded.as_slice()).unwrap();

        assert_eq!(plan, decoded);
        assert_eq!(decoded.parameter_bindings.len(), 2);
        assert_eq!(decoded.parameter_bindings[0].parameter_anchor, 0);
        assert_eq!(decoded.parameter_bindings[1].parameter_anchor, 1);

        // Verify the expressions in the project
        let root = match decoded.relations[0].rel_type.as_ref().unwrap() {
            super::plan_rel::RelType::Root(root) => root,
            _ => panic!("expected Root"),
        };
        let project = match root.input.as_ref().unwrap().rel_type.as_ref().unwrap() {
            super::rel::RelType::Project(p) => p,
            _ => panic!("expected Project"),
        };
        assert_eq!(project.expressions.len(), 2);
        match project.expressions[0].rex_type.as_ref().unwrap() {
            RexType::DynamicParameter(dp) => assert_eq!(dp.parameter_reference, 0),
            other => panic!("expected DynamicParameter, got {other:?}"),
        }
        match project.expressions[1].rex_type.as_ref().unwrap() {
            RexType::DynamicParameter(dp) => assert_eq!(dp.parameter_reference, 1),
            other => panic!("expected DynamicParameter, got {other:?}"),
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn dynamic_parameter_serde_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let plan = serde_json::from_str::<super::Plan>(
            r#"{
  "version": { "minorNumber": 85, "producer": "substrait-rs-test" },
  "relations": [{
    "root": {
      "input": {
        "filter": {
          "input": { "read": {} },
          "condition": {
            "dynamicParameter": {
              "type": { "i32": { "nullability": "NULLABILITY_REQUIRED" } },
              "parameterReference": 1
            }
          }
        }
      },
      "names": ["output"]
    }
  }],
  "parameterBindings": [{
    "parameterAnchor": 1,
    "value": { "i32": 42 }
  }]
}"#,
        )?;
        assert_eq!(plan.parameter_bindings.len(), 1);
        assert_eq!(plan.parameter_bindings[0].parameter_anchor, 1);

        // Re-serialize and verify
        let json = serde_json::to_string(&plan)?;
        let roundtripped = serde_json::from_str::<super::Plan>(&json)?;
        assert_eq!(plan, roundtripped);

        Ok(())
    }
}
