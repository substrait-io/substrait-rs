use crate::parse::proto::extensions::SimpleExtensionUri;
use crate::parse::typed::Name;
use crate::parse::{Anchor, Context, Parse};
use crate::proto;

use super::declaration::SimpleExtensionDeclarationError;

/// A parsed [`ExtensionType`]
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionType {
    /// The parsed [`Anchor`] for this [`ExtensionType`]
    pub(crate) anchor: Anchor<Self>,
    /// The parsed [`Name`] for this [`ExtensionType`]
    pub(crate) name: Name<Self>,
    /// The parsed [`SimpleExtensionUri`] for this [`ExtensionType`]
    pub(crate) extension_uri_reference: Anchor<SimpleExtensionUri>,
}

impl ExtensionType {
    /// Returns the name of this [ExtensionType]
    ///
    /// See [proto::extensions::simple_extension_declaration::ExtensionType::name].
    pub fn name(&self) -> &Name<Self> {
        &self.name
    }

    /// Returns the anchor value of this [ExtensionType].
    ///
    /// See [proto::extensions::simple_extension_declaration::ExtensionType].
    pub fn anchor(&self) -> Anchor<Self> {
        self.anchor
    }

    /// Returns the Extension URI Reference of this [ExtensionType].
    ///
    /// See [proto::extensions::simple_extension_declaration::ExtensionType::extension_uri_reference].
    pub fn uri(&self) -> Anchor<SimpleExtensionUri> {
        self.extension_uri_reference
    }
}

impl<C: Context> Parse<C> for proto::extensions::simple_extension_declaration::ExtensionType {
    type Parsed = ExtensionType;
    type Error = SimpleExtensionDeclarationError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::extensions::simple_extension_declaration::ExtensionType {
            extension_uri_reference,
            type_anchor,
            name,
        } = self;

        // Construct the parsed ExtensionType.
        let extension_type = ExtensionType {
            extension_uri_reference: Anchor::new(extension_uri_reference),
            name: Name::new(name),
            anchor: Anchor::new(type_anchor),
        };

        // Add the ExtensionType to the given context.
        ctx.add_extension_type(&extension_type)?;

        Ok(extension_type)
    }
}

impl From<ExtensionType> for proto::extensions::simple_extension_declaration::ExtensionType {
    fn from(value: ExtensionType) -> Self {
        let ExtensionType {
            anchor,
            name,
            extension_uri_reference,
        } = value;

        Self {
            extension_uri_reference: extension_uri_reference.into_inner(),
            type_anchor: anchor.into_inner(),
            name: name.into_inner(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{
        context::{tests::Context, ContextError},
        Context as _,
    };

    #[test]
    fn parse_extension_type() -> Result<(), SimpleExtensionDeclarationError> {
        let extension_type = proto::extensions::simple_extension_declaration::ExtensionType {
            type_anchor: 1,
            extension_uri_reference: 2,
            name: "test_extension_type".to_string(),
        };
        let extension_type: ExtensionType = extension_type.parse(&mut Context::default())?;
        assert_eq!(extension_type.anchor(), Anchor::new(1));
        assert_eq!(
            extension_type.name(),
            &Name::new("test_extension_type".to_string())
        );
        assert_eq!(extension_type.uri(), Anchor::new(2));
        assert_eq!(extension_type.extension_uri_reference, Anchor::new(2));
        Ok(())
    }

    #[test]
    fn duplicate_simple_extension() {
        let mut ctx = Context::default();
        let extension_type = proto::extensions::simple_extension_declaration::ExtensionType {
            extension_uri_reference: 1,
            type_anchor: 2,
            name: "test_extension_type".to_string(),
        };
        assert!(ctx.parse(extension_type.clone()).is_ok());
        assert_eq!(
            ctx.parse(extension_type.clone()),
            Err(SimpleExtensionDeclarationError::Context(
                ContextError::DuplicateExtensionType(Anchor::new(2))
            ))
        );
    }

    #[test]
    fn can_get_protobuf_from_struct() {
        let extension_type = ExtensionType {
            extension_uri_reference: Anchor::new(1),
            anchor: Anchor::new(1),
            name: Name::new("test_extension_type".to_string()),
        };

        let protobuf_extension_type =
            proto::extensions::simple_extension_declaration::ExtensionType::from(extension_type);

        assert_eq!(protobuf_extension_type.extension_uri_reference, 1);
        assert_eq!(protobuf_extension_type.type_anchor, 1);
        assert_eq!(protobuf_extension_type.name, "test_extension_type");
    }
}
