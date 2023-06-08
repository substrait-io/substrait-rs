// SPDX-License-Identifier: Apache-2.0

//! A parser implementation.

use super::{
    extensions::{
        simple_extension_declaration::{ExtensionFunction, ExtensionType, ExtensionTypeVariation},
        simple_extension_uri::SimpleExtensionURI,
    },
    simple_extensions::{SimpleExtensionFunction, SimpleExtensions},
    Anchor, Context, ContextError,
};
use crate::{
    extensions::EXTENSIONS,
    text::{
        self,
        simple_extensions::{SimpleExtensionsTypeVariationsItem, SimpleExtensionsTypesItem},
    },
};
use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
};
use url::Url;

/// A resolver for simple extensions.
pub trait SimpleExtensionURIResolver: Default {
    /// The error type of this resolver.
    type Error: std::error::Error;

    /// Attempt to resolve the given URI to the source text of a [SimpleExtensions].
    fn resolve(&self, uri: &Url) -> Result<String, Self::Error>;
}

/// A parser implementation.
#[derive(Default)]
pub struct Parser<R: SimpleExtensionURIResolver = UreqResolver> {
    extensions_uris: HashMap<Anchor<SimpleExtensionURI>, (SimpleExtensionURI, SimpleExtensions)>,
    extension_types: HashMap<Anchor<ExtensionType>, (ExtensionType, SimpleExtensionsTypesItem)>,
    extension_type_variations: HashMap<
        Anchor<ExtensionTypeVariation>,
        (ExtensionTypeVariation, SimpleExtensionsTypeVariationsItem),
    >,
    extension_functions:
        HashMap<Anchor<ExtensionFunction>, (ExtensionFunction, SimpleExtensionFunction)>,
    resolver: R,
}

#[cfg(feature = "parse-ureq")]
/// A resolver that uses [ureq] to resolve simple extensions.
pub struct UreqResolver {
    agent: ureq::Agent,
}

impl Default for UreqResolver {
    fn default() -> Self {
        Self {
            agent: ureq::Agent::new(),
        }
    }
}

impl SimpleExtensionURIResolver for UreqResolver {
    type Error = ureq::Error;

    fn resolve(&self, uri: &Url) -> Result<String, Self::Error> {
        Ok(self.agent.get(uri.as_str()).call()?.into_string()?)
    }
}

#[cfg(feature = "parse-reqwest")]
#[derive(Default)]
/// A resolver that uses [reqwest] to resolve simple extensions.
pub struct ReqwestResolver {
    client: reqwest::blocking::Client,
}

#[cfg(feature = "parse-reqwest")]
impl SimpleExtensionURIResolver for ReqwestResolver {
    type Error = reqwest::Error;

    fn resolve(&self, uri: &Url) -> Result<String, reqwest::Error> {
        self.client
            .get(uri.clone())
            .send()
            .and_then(reqwest::blocking::Response::text)
    }
}

impl Context for Parser {
    fn add_simple_extension_uri(
        &mut self,
        simple_extension_uri: &SimpleExtensionURI,
    ) -> Result<&SimpleExtensions, ContextError> {
        // Attempt to resolve the simple extension using the included extensions, else use the resolver.
        let simple_extension_str = match EXTENSIONS.get(simple_extension_uri.uri().as_str()) {
            Some(&path) => Cow::from(path),
            None => self
                .resolver
                .resolve(simple_extension_uri.uri())
                .map(Cow::from)
                .map_err(|e| ContextError::FailedSimpleExtensionResolve(e.into()))?,
        };

        // Deserialize.
        let simple_extensions: text::simple_extensions::SimpleExtensions =
            serde_yaml::from_str::<text::simple_extensions::SimpleExtensions>(
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

    fn simple_extensions(&self, anchor: Anchor<SimpleExtensionURI>) -> &SimpleExtensions {
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
        extension_type_anchor: Anchor<ExtensionType>,
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
        extension_type_variation_anchor: Anchor<ExtensionTypeVariation>,
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
                        let name = extension_function.name().as_ref();
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
        extension_function_anchor: Anchor<ExtensionFunction>,
    ) -> &SimpleExtensionFunction {
        &self
            .extension_functions
            .get(&extension_function_anchor)
            .unwrap()
            .1
    }
}
