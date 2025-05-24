use super::declaration::SimpleExtensionDeclarationError;
use crate::{
    parse::{proto::extensions::SimpleExtensionUri, typed::Name, Anchor, Context, Parse},
    proto,
};

/// A parsed [`ExtensionFunction`]
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionFunction {
    /// The parsed [`Anchor`] for this [`ExtensionFunction`]
    pub(crate) anchor: Anchor<Self>,
    /// The parsed [`Name`] for this [`ExtensionFunction`]
    pub(crate) name: Name<Self>,
    /// The parsed [`SimpleExtensionUri`] for this [`ExtensionFunction`]
    pub(crate) extension_uri_reference: Anchor<SimpleExtensionUri>,
}

impl ExtensionFunction {
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

impl<C: Context> Parse<C> for proto::extensions::simple_extension_declaration::ExtensionFunction {
    type Parsed = ExtensionFunction;
    type Error = SimpleExtensionDeclarationError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::extensions::simple_extension_declaration::ExtensionFunction {
            extension_uri_reference,
            function_anchor,
            name,
        } = self;

        // Construct the parsed ExtensionFunction.
        let extension_function = ExtensionFunction {
            extension_uri_reference: Anchor::new(extension_uri_reference),
            name: Name::new(name),
            anchor: Anchor::new(function_anchor),
        };

        // Add the ExtensionFunction to the given context.
        ctx.add_extension_function(&extension_function)?;

        Ok(extension_function)
    }
}

impl From<ExtensionFunction>
    for proto::extensions::simple_extension_declaration::ExtensionFunction
{
    fn from(value: ExtensionFunction) -> Self {
        let ExtensionFunction {
            anchor,
            name,
            extension_uri_reference,
        } = value;

        Self {
            extension_uri_reference: extension_uri_reference.into_inner(),
            function_anchor: anchor.into_inner(),
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
    fn parse_extension_function() -> Result<(), SimpleExtensionDeclarationError> {
        let extension_type = proto::extensions::simple_extension_declaration::ExtensionFunction {
            function_anchor: 1,
            extension_uri_reference: 2,
            name: "test_extension_function".to_string(),
        };
        let extension_type: ExtensionFunction = extension_type.parse(&mut Context::default())?;
        assert_eq!(extension_type.anchor(), Anchor::new(1));
        assert_eq!(
            extension_type.name(),
            &Name::new("test_extension_function".to_string())
        );
        assert_eq!(extension_type.uri(), Anchor::new(2));
        assert_eq!(extension_type.extension_uri_reference, Anchor::new(2));
        Ok(())
    }

    #[test]
    fn duplicate_simple_extension_function() {
        let mut ctx = Context::default();
        let extension_type = proto::extensions::simple_extension_declaration::ExtensionFunction {
            extension_uri_reference: 1,
            function_anchor: 2,
            name: "test_extension_function".to_string(),
        };
        assert!(ctx.parse(extension_type.clone()).is_ok());
        assert_eq!(
            ctx.parse(extension_type.clone()),
            Err(SimpleExtensionDeclarationError::Context(
                ContextError::DuplicateExtensionFunction(Anchor::new(2))
            ))
        );
    }

    #[test]
    fn can_get_protobuf_from_struct() {
        let extension_type = ExtensionFunction {
            extension_uri_reference: Anchor::new(1),
            anchor: Anchor::new(1),
            name: Name::new("test_extension_function".to_string()),
        };

        let protobuf_extension_function =
            proto::extensions::simple_extension_declaration::ExtensionFunction::from(
                extension_type,
            );

        assert_eq!(protobuf_extension_function.extension_uri_reference, 1);
        assert_eq!(protobuf_extension_function.function_anchor, 1);
        assert_eq!(protobuf_extension_function.name, "test_extension_function");
    }
}
