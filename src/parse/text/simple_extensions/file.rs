use super::{CustomType, SimpleExtensions, SimpleExtensionsError};
use crate::parse::Parse;
use crate::parse::text::simple_extensions::extensions::TypeContext;
use crate::text::simple_extensions::SimpleExtensions as RawExtensions;
use crate::urn::Urn;
use std::io::Read;

/// A parsed and validated [RawExtensions].
///
/// `ExtensionFile` owns the canonical URN for a simple extension file along with the parsed
/// [`SimpleExtensions`](super::SimpleExtensions) data. Keeping the URN here (instead of on the inner
/// type map) lets us thread it through I/O, registries, and conversions without duplicating state.
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
        let extension = SimpleExtensions::default();
        Self { urn, extension }
    }

    /// Create a validated SimpleExtensions from raw data
    pub fn create(extensions: RawExtensions) -> Result<Self, SimpleExtensionsError> {
        // Parse all types (may contain unresolved Extension(String) references)
        let mut ctx = TypeContext::default();
        let (urn, extension) = Parse::parse(extensions, &mut ctx)?;

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

    /// Convert the parsed extension file back into the raw text representation by value.
    pub fn into_raw(self) -> RawExtensions {
        let ExtensionFile { urn, extension } = self;
        RawExtensions::from((urn, extension))
    }

    /// Convert the parsed extension file back into the raw text representation by reference.
    pub fn to_raw(&self) -> RawExtensions {
        RawExtensions::from((self.urn.clone(), self.extension.clone()))
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
    use super::*;
    use crate::parse::text::simple_extensions::types::ParameterConstraint as RawParameterType;

    const YAML_PARAM_TEST: &str = r#"
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

    #[test]
    fn yaml_round_trip_integer_param_bounds() {
        let ext = ExtensionFile::read_from_str(YAML_PARAM_TEST).expect("parse ok");
        assert_eq!(ext.urn().to_string(), "extension:example.com:param_test");

        let ty = ext.get_type("ParamTest").expect("type exists");
        match &ty.parameters[..] {
            [param] => match &param.param_type {
                RawParameterType::Integer {
                    min: actual_min,
                    max: actual_max,
                } => {
                    assert_eq!(actual_min, &Some(1));
                    assert_eq!(actual_max, &Some(10));
                }
                other => panic!("unexpected param type: {other:?}"),
            },
            other => panic!("unexpected parameters: {other:?}"),
        }

        let back = ext.to_raw();
        assert_eq!(back.urn, "extension:example.com:param_test");
        let item = back
            .types
            .into_iter()
            .find(|t| t.name == "ParamTest")
            .expect("round-tripped type present");
        let param = item.parameters.unwrap().0.into_iter().next().unwrap();
        assert_eq!(param.name.as_deref(), Some("K"));
        assert!(matches!(
            param.type_,
            crate::text::simple_extensions::TypeParamDefsItemType::Integer
        ));
        assert_eq!(param.min, Some(1.0));
        assert_eq!(param.max, Some(10.0));
    }
}
