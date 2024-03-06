// SPDX-License-Identifier: Apache-2.0

//! [Substrait]: Cross-Language Serialization for Relational Algebra
//!
//! # Serialization and deserialization
//!
//! This crate provides generated types to serialize and deserialize Substrait
//! data.
//!
//! ## Protobuf
//!
//! Protobuf serialization and deserialization are provided via [prost] in the
//! [proto] module.
//!
//! ### Example
//!
//! #### Serialize and deserialize a plan
//! ```rust
//! # fn main() -> Result<(), prost::DecodeError> {
//! use prost::Message;
//! use substrait::proto::Plan;
//!
//! let plan = Plan::default();
//!
//! // Serialize the plan
//! let encoded = plan.encode_to_vec();
//!
//! // Deserialize the buffer to a Plan
//! let decoded = Plan::decode(encoded.as_slice())?;
//!
//! assert_eq!(plan, decoded);
//! # Ok(()) }
//! ```
//!
//! ### Serde support
//!
//! The `serde` feature generates serde implementations that match the [Protobuf JSON Mapping]
//! via [pbjson].
//!
//! ##### Example
//! ###### Deserialize a plan version using the `serde` feature
//! ```rust
//! # fn main() -> Result<(), serde_json::Error> {
//! # #[cfg(feature="serde")] {
//! use substrait::proto::Version;
//!
//! let version_json = r#"{
//!   "minorNumber": 21
//! }"#;
//!
//! let version = serde_json::from_str::<Version>(version_json)?;
//! assert_eq!(
//!   version,
//!   Version {
//!     minor_number: 21,
//!     ..Default::default()
//!   }
//! );
//! # } Ok(()) }
//! ```
//!
//! ## Text
//!
//! Substrait defines a YAML schema for extensions. Types with serialization and
//! deserialization support for these are provided via [typify] in the [text]
//! module.
//!
//! ### Example
//!
//! #### Read a simple extension
//! ```rust
//! # fn main() -> Result<(), serde_yaml::Error> {
//! use substrait::text::simple_extensions::SimpleExtensions;
//!
//! let simple_extension_yaml = r#"
//! %YAML 1.2
//! ---
//! scalar_functions:
//!   -
//!     name: "add"
//!     description: "Add two values."
//!     impls:
//!       - args:
//!          - name: x
//!            value: i8
//!          - name: y
//!            value: i8
//!         options:
//!           overflow:
//!             values: [ SILENT, SATURATE, ERROR ]
//!         return: i8
//! "#;
//!
//! let simple_extension = serde_yaml::from_str::<SimpleExtensions>(simple_extension_yaml)?;
//!
//! assert_eq!(simple_extension.scalar_functions.len(), 1);
//! assert_eq!(simple_extension.scalar_functions[0].name, "add");
//! # Ok(()) }
//! ```
//!
//! [pbjson]: https://docs.rs/pbjson
//! [Protobuf JSON Mapping]:
//!     https://developers.google.com/protocol-buffers/docs/proto3#json
//! [Substrait]: https://substrait.io
//! [typify]: https://docs.rs/typify

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/substrait-io/substrait/main/site/docs/img/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/substrait-io/substrait/main/site/docs/img/logo.svg"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod proto;
pub mod text;
pub mod version;
