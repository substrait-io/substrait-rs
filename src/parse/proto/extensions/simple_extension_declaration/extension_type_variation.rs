use crate::parse::proto::extensions::SimpleExtensionUri;
use crate::parse::typed::Name;
use crate::parse::{Anchor, Context, Parse};
use crate::proto;

use super::declaration::SimpleExtensionDeclarationError;

/// A parsed [`ExtensionTypeVariation`]
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTypeVariation {
    /// The parsed [`Anchor`] for this [`ExtensionTypeVariation`]
    pub(crate) anchor: Anchor<Self>,
    /// The parsed [`Name`] for this [`ExtensionTypeVariation`]
    pub(crate) name: Name<Self>,
    /// The parsed [`SimpleExtensionUri`] for this [`ExtensionTypeVariation`]
    pub(crate) extension_uri_reference: Anchor<SimpleExtensionUri>,
}

impl ExtensionTypeVariation {
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

impl<C: Context> Parse<C>
    for proto::extensions::simple_extension_declaration::ExtensionTypeVariation
{
    type Parsed = ExtensionTypeVariation;
    type Error = SimpleExtensionDeclarationError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::extensions::simple_extension_declaration::ExtensionTypeVariation {
            extension_uri_reference,
            type_variation_anchor,
            name,
        } = self;

        // Construct the parsed ExtensionTypeVariation.
        let extension_type_variation = ExtensionTypeVariation {
            extension_uri_reference: Anchor::new(extension_uri_reference),
            name: Name::new(name),
            anchor: Anchor::new(type_variation_anchor),
        };

        // Add the ExtensionTypeVariation to the given context.
        ctx.add_extension_type_variation(&extension_type_variation)?;

        Ok(extension_type_variation)
    }
}

impl From<ExtensionTypeVariation>
    for proto::extensions::simple_extension_declaration::ExtensionTypeVariation
{
    fn from(value: ExtensionTypeVariation) -> Self {
        let ExtensionTypeVariation {
            anchor,
            name,
            extension_uri_reference,
        } = value;

        Self {
            extension_uri_reference: extension_uri_reference.into_inner(),
            type_variation_anchor: anchor.into_inner(),
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
    fn parse_extension_type_variation() -> Result<(), SimpleExtensionDeclarationError> {
        let extension_type =
            proto::extensions::simple_extension_declaration::ExtensionTypeVariation {
                type_variation_anchor: 1,
                extension_uri_reference: 2,
                name: "test_extension_variation".to_string(),
            };
        let extension_type: ExtensionTypeVariation =
            extension_type.parse(&mut Context::default())?;
        assert_eq!(extension_type.anchor(), Anchor::new(1));
        assert_eq!(
            extension_type.name(),
            &Name::new("test_extension_variation".to_string())
        );
        assert_eq!(extension_type.uri(), Anchor::new(2));
        assert_eq!(extension_type.extension_uri_reference, Anchor::new(2));
        Ok(())
    }

    #[test]
    fn duplicate_simple_extension_variation() {
        let mut ctx = Context::default();
        let extension_type =
            proto::extensions::simple_extension_declaration::ExtensionTypeVariation {
                extension_uri_reference: 1,
                type_variation_anchor: 2,
                name: "test_extension_var".to_string(),
            };
        assert!(ctx.parse(extension_type.clone()).is_ok());
        assert_eq!(
            ctx.parse(extension_type.clone()),
            Err(SimpleExtensionDeclarationError::Context(
                ContextError::DuplicateExtensionTypeVariation(Anchor::new(2))
            ))
        );
    }

    #[test]
    fn can_get_protobuf_from_struct() {
        let extension_type = ExtensionTypeVariation {
            extension_uri_reference: Anchor::new(1),
            anchor: Anchor::new(1),
            name: Name::new("test_extension_var".to_string()),
        };

        let protobuf_extension_type_variation =
            proto::extensions::simple_extension_declaration::ExtensionTypeVariation::from(
                extension_type,
            );

        assert_eq!(protobuf_extension_type_variation.extension_uri_reference, 1);
        assert_eq!(protobuf_extension_type_variation.type_variation_anchor, 1);
        assert_eq!(protobuf_extension_type_variation.name, "test_extension_var");
    }
}
