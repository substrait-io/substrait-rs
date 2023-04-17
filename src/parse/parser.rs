// SPDX-License-Identifier: Apache-2.0

//! A parser implementation.

use super::{
    extensions::{
        simple_extension_declaration::{
            ExtensionFunction, ExtensionFunctionAnchor, ExtensionType, ExtensionTypeAnchor,
            ExtensionTypeVariation, ExtensionTypeVariationAnchor,
        },
        simple_extension_uri::{SimpleExtensionAnchor, SimpleExtensionURI},
    },
    simple_extensions::{SimpleExtensionFunction, SimpleExtensions},
    Context, ContextError,
};
use crate::{
    extensions::EXTENSIONS,
    text::{
        self,
        simple_extensions::{SimpleExtensionsTypeVariationsItem, SimpleExtensionsTypesItem},
    },
};
use reqwest::blocking::{self, Response};
use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
};

/// A parser implementation.
#[derive(Default)]
pub struct Parser {
    extensions_uris: HashMap<SimpleExtensionAnchor, (SimpleExtensionURI, SimpleExtensions)>,
    extension_types: HashMap<ExtensionTypeAnchor, (ExtensionType, SimpleExtensionsTypesItem)>,
    extension_type_variations: HashMap<
        ExtensionTypeVariationAnchor,
        (ExtensionTypeVariation, SimpleExtensionsTypeVariationsItem),
    >,
    extension_functions:
        HashMap<ExtensionFunctionAnchor, (ExtensionFunction, SimpleExtensionFunction)>,
}

impl Context for Parser {
    fn add_simple_extension_uri(
        &mut self,
        simple_extension_uri: &SimpleExtensionURI,
    ) -> Result<&SimpleExtensions, ContextError> {
        // Resolve the simple extension.
        let simple_extension_str = match EXTENSIONS.get(simple_extension_uri.uri().as_str()) {
            Some(&path) => Cow::from(path),
            None => blocking::get(simple_extension_uri.uri().clone())
                .and_then(Response::text)
                .map(Cow::from)
                .map_err(ContextError::FailedSimpleExtensionResolve)?,
        };

        // Deserialize.
        let simple_extensions = serde_yaml::from_str::<text::simple_extensions::SimpleExtensions>(
            &simple_extension_str,
        )
        .map_err(ContextError::InvalidSimpleExtensions)?;

        // Parse.
        let simple_extensions = self.parse(simple_extensions)?;

        // Insert in map, error on duplicate anchor or duplicate (uri).
        if self
            .extensions_uris
            .values()
            .any(|(extension_uri, _)| extension_uri.uri() == simple_extension_uri.uri())
        {
            // todo(mbrobbel): is this an error?
            Err(ContextError::DuplicateSimpleExtensionDefinition)?;
        }
        match self.extensions_uris.entry(simple_extension_uri.anchor()) {
            Entry::Occupied(_) => Err(ContextError::DuplicateSimpleExtension(
                simple_extension_uri.anchor(),
            )),
            Entry::Vacant(entry) => {
                // Insert in the map and return.
                Ok(&entry
                    .insert((simple_extension_uri.clone(), simple_extensions))
                    .1)
            }
        }
    }

    fn simple_extensions(&self, anchor: SimpleExtensionAnchor) -> &SimpleExtensions {
        &self.extensions_uris.get(&anchor).unwrap().1
    }

    fn add_extension_type(
        &mut self,
        _extension_type: &ExtensionType,
    ) -> Result<&SimpleExtensionsTypesItem, ContextError> {
        todo!("add_extension_type")
    }

    fn extension_type(
        &self,
        extension_type_anchor: ExtensionTypeAnchor,
    ) -> &SimpleExtensionsTypesItem {
        &self.extension_types.get(&extension_type_anchor).unwrap().1
    }

    fn add_extension_type_variation(
        &mut self,
        _extension_type_variation: &ExtensionTypeVariation,
    ) -> Result<&SimpleExtensionsTypeVariationsItem, ContextError> {
        todo!("add_extension_type_variation")
    }

    fn extension_type_variation(
        &self,
        extension_type_variation_anchor: ExtensionTypeVariationAnchor,
    ) -> &SimpleExtensionsTypeVariationsItem {
        &self
            .extension_type_variations
            .get(&extension_type_variation_anchor)
            .unwrap()
            .1
    }

    fn add_extension_function(
        &mut self,
        extension_function: &ExtensionFunction,
    ) -> Result<&SimpleExtensionFunction, ContextError> {
        match self
            .extension_functions
            .entry(extension_function.function_anchor())
        {
            Entry::Occupied(_) => Err(ContextError::DuplicateExtensionFunction(
                extension_function.function_anchor(),
            )),
            Entry::Vacant(entry) => {
                // todo(mbrobbel): make sure there are no others with that name when searching, otherwise require compound name
                match self
                    .extensions_uris
                    .get(&extension_function.simple_extension_anchor())
                {
                    // filter then next, error if next is some
                    Some((_, simple_extensions)) => {
                        // Find function with name
                        let name = extension_function.name().name();
                        let simple_extension_function = simple_extensions
                            .aggregate_functions()
                            .iter()
                            .find(|aggregate_function| aggregate_function.name == name)
                            .map(SimpleExtensionFunction::from)
                            .or_else(|| {
                                simple_extensions
                                    .scalar_functions()
                                    .iter()
                                    .find(|scalar_function| scalar_function.name == name)
                                    .map(SimpleExtensionFunction::from)
                            })
                            .or_else(|| {
                                simple_extensions
                                    .window_functions()
                                    .iter()
                                    .find(|window_function| window_function.name == name)
                                    .map(SimpleExtensionFunction::from)
                            })
                            .ok_or(ContextError::UndefinedExtensionFunctionDefinition(
                                extension_function.name().clone(),
                            ))?;

                        Ok(&entry
                            .insert((extension_function.clone(), simple_extension_function))
                            .1)
                    }
                    None => Err(ContextError::UndefinedSimpleExtension(
                        extension_function.simple_extension_anchor(),
                    )),
                }
            }
        }
    }

    fn extension_function(
        &self,
        extension_function_anchor: ExtensionFunctionAnchor,
    ) -> &SimpleExtensionFunction {
        &self
            .extension_functions
            .get(&extension_function_anchor)
            .unwrap()
            .1
    }
}
