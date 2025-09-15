use super::{CustomType, SimpleExtensions, SimpleExtensionsError};
use crate::parse::Parse;
use crate::parse::text::simple_extensions::extensions::TypeContext;
use crate::text::simple_extensions::SimpleExtensions as RawExtensions;
use crate::urn::Urn;
use std::io::Read;

/// A parsed and validated [RawExtensions].
#[derive(Debug)]
pub struct ExtensionFile {
    /// The URN this extension was loaded from
    pub urn: Urn,
    /// The extension data containing types and eventually functions
    extension: SimpleExtensions,
}

impl ExtensionFile {
    /// Create a new, empty SimpleExtensions
    pub fn empty(urn: Urn) -> Self {
        let extension = SimpleExtensions::new(urn.clone());
        Self { urn, extension }
    }

    /// Create a validated SimpleExtensions from raw data
    pub fn create(extensions: RawExtensions) -> Result<Self, SimpleExtensionsError> {
        // Parse all types (may contain unresolved Extension(String) references)
        let mut ctx = TypeContext::default();
        let extension = Parse::parse(extensions, &mut ctx)?;
        let urn = extension.urn().clone();

        // TODO: Use ctx.known/ctx.linked to validate unresolved references and cross-file links.

        Ok(Self { urn, extension })
    }

    /// Get a type by name
    pub fn get_type(&self, name: &str) -> Option<&CustomType> {
        self.extension.get_type(name)
    }

    /// Get an iterator over all types in this extension
    pub fn types(&self) -> impl Iterator<Item = &CustomType> {
        self.extension.types()
    }

    /// Returns the URN for this extension file.
    pub fn urn(&self) -> &Urn {
        &self.urn
    }

    /// Get a reference to the underlying SimpleExtension
    pub fn extension(&self) -> &SimpleExtensions {
        &self.extension
    }

    /// Read an extension file from a reader.
    /// - `reader`: any `Read` instance with the YAML content
    ///
    /// Returns a parsed and validated `ExtensionFile` or an error.
    pub fn read<R: Read>(reader: R) -> Result<Self, SimpleExtensionsError> {
        let raw: RawExtensions = serde_yaml::from_reader(reader)?;
        Self::create(raw)
    }

    /// Read an extension file from a string slice.
    pub fn read_from_str<S: AsRef<str>>(s: S) -> Result<Self, SimpleExtensionsError> {
        let raw: RawExtensions = serde_yaml::from_str(s.as_ref())?;
        Self::create(raw)
    }
}

// Parsing and conversion implementations are defined on `SimpleExtensions` in `extensions.rs`.

#[cfg(test)]
mod tests {
    use crate::parse::text::simple_extensions::types::ParameterConstraint as RawParameterType;
    use crate::text::simple_extensions::SimpleExtensions as RawExtensions;

    use super::*;

    #[test]
    fn yaml_round_trip_integer_param_bounds() {
        // A minimal YAML extension file with a single type that has integer bounds on a parameter
        let yaml = r#"
%YAML 1.2
---
urn: extension:example.com:param_test
types:
  - name: "ParamTest"
    parameters:
      - name: "K"
        type: integer
        min: 1
        max: 10
"#;

        let ext = ExtensionFile::read_from_str(yaml).expect("parse ok");
        assert_eq!(ext.urn().to_string(), "extension:example.com:param_test");

        // Validate parsed parameter bounds
        let ty = ext.get_type("ParamTest").expect("type exists");
        assert_eq!(ty.parameters.len(), 1);
        match &ty.parameters[0].param_type {
            RawParameterType::Integer { min, max } => {
                assert_eq!(min, &Some(1));
                assert_eq!(max, &Some(10));
            }
            other => panic!("unexpected param type: {other:?}"),
        }

        // Convert back to text::simple_extensions and assert bounds are preserved as f64
        let back: RawExtensions = ext.extension.into();
        let item = back
            .types
            .into_iter()
            .find(|t| t.name == "ParamTest")
            .expect("round-tripped type present");
        let param_defs = item.parameters.expect("params present");
        assert_eq!(param_defs.0.len(), 1);
        let p = &param_defs.0[0];
        assert_eq!(p.name.as_deref(), Some("K"));
        assert!(matches!(
            p.type_,
            crate::text::simple_extensions::TypeParamDefsItemType::Integer
        ));
        assert_eq!(p.min, Some(1.0));
        assert_eq!(p.max, Some(10.0));
        assert_eq!(back.urn, "extension:example.com:param_test");
    }
}
