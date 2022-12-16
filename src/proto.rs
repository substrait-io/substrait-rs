// SPDX-License-Identifier: Apache-2.0

//! Generated types for the protobuf `substrait` package

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
    #[cfg(feature = "pbjson")]
    #[test]
    fn pbjson_serde() -> Result<(), Box<dyn std::error::Error>> {
        let plan = serde_json::from_str::<super::Plan>(
            r#"{
  "version": { "minorNumber": 19, "producer": "substrait-rs" },
  "extensionUris": [
    {
      "extensionUriAnchor": 1,
      "uri": "https://github.com/substrait-io/substrait/blob/main/extensions/functions_string.yaml"
    }
  ]
}"#,
        )?;
        assert_eq!(
            plan.version,
            Some(super::Version {
                minor_number: 19,
                producer: "substrait-rs".into(),
                ..Default::default()
            })
        );
        assert_eq!(plan.extension_uris.len(), 1);
        Ok(())
    }
}
