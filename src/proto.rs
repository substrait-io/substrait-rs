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
}
