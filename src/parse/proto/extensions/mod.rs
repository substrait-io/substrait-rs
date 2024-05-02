// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::extensions] types.

mod simple_extension_declaration;
mod simple_extension_uri;

pub use simple_extension_declaration::declaration::SimpleExtensionDeclaration;
pub use simple_extension_declaration::extension_function::ExtensionFunction;
pub use simple_extension_declaration::extension_type::ExtensionType;
pub use simple_extension_declaration::extension_type_variation::ExtensionTypeVariation;
pub use simple_extension_declaration::mapping_type::MappingType;
pub use simple_extension_uri::SimpleExtensionUri;
