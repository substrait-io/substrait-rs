use url::Url;

use super::{CustomType, SimpleExtensions, SimpleExtensionsError};
use crate::parse::{Context, Parse};
use crate::text;

/// A parsed and validated [text::simple_extensions::SimpleExtensions].
#[derive(Debug)]
pub struct ExtensionFile {
    /// The URI this extension was loaded from
    pub uri: Url,
    /// The extension data containing types and eventually functions
    extension: SimpleExtensions,
}

impl ExtensionFile {
    /// Create a new, empty SimpleExtensions
    pub fn empty(uri: Url) -> Self {
        Self {
            uri,
            extension: SimpleExtensions::default(),
        }
    }

    /// Create a validated SimpleExtensions from raw data and URI
    pub fn create(
        uri: Url,
        extensions: text::simple_extensions::SimpleExtensions,
    ) -> Result<Self, SimpleExtensionsError> {
        // Parse all types (may contain unresolved Extension(String) references)
        let mut extension = SimpleExtensions::default();

        for type_item in &extensions.types {
            let custom_type = type_item.clone().parse(&mut extension)?;
            // Add the parsed type to the context so later types can reference it
            extension.add_type(&custom_type);
        }

        // TODO: Validate that all Extension(String) references in structure
        // fields exist Walk through all CustomType.structure fields and check
        // that Extension(String) references can be resolved to actual types in
        // the registry.

        Ok(Self { uri, extension })
    }

    /// Get a type by name
    pub fn get_type(&self, name: &str) -> Option<&CustomType> {
        self.extension.get_type(name)
    }

    /// Get an iterator over all types in this extension
    pub fn types(&self) -> impl Iterator<Item = &CustomType> {
        self.extension.types()
    }

    /// Get a reference to the underlying SimpleExtension
    pub fn extension(&self) -> &SimpleExtensions {
        &self.extension
    }
}

impl<C: Context> Parse<C> for text::simple_extensions::SimpleExtensions {
    type Parsed = ExtensionFile;
    type Error = SimpleExtensionsError;

    fn parse(self, _ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        // For parsing without URI context, create a dummy URI
        let dummy_uri = Url::parse("file:///unknown").unwrap();
        ExtensionFile::create(dummy_uri, self)
    }
}

impl From<ExtensionFile> for text::simple_extensions::SimpleExtensions {
    fn from(_value: ExtensionFile) -> Self {
        // TODO: Implement conversion back to text representation
        unimplemented!("Conversion from parsed SimpleExtensions back to text representation not yet implemented")
    }
}
