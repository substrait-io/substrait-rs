// SPDX-License-Identifier: Apache-2.0

//! Generated types for the protobuf `substrait` package.
//!
//! These types are provided by the [`substrait-prost`](https://docs.rs/substrait-prost)
//! crate and re-exported here. This includes the [`extensions`] submodule and,
//! with the `embed-descriptor` feature, the `FILE_DESCRIPTOR_SET` constant.

pub use substrait_prost::*;

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
}
