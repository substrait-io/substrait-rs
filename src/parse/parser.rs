// SPDX-License-Identifier: Apache-2.0

//! A parser implementation.

use crate::text;

use super::{
    extensions::simple_extension_uri::{
        SimpleExtensionAnchor, SimpleExtensionURI, SimpleExtensionURIError,
    },
    Context, ContextError,
};
use std::collections::hash_map::Entry;
use url::Url;

/// A parser implementation.
#[derive(Default)]
pub struct Parser {
    extensions_uris: std::collections::HashMap<SimpleExtensionAnchor, SimpleExtensionURI>,
    // or use Either<url, SimpleExtensions> extensions: HashMap<u32,
    // crate::text::simple_extensions::SimpleExtensions>,
}

impl Context for Parser {
    fn add_simple_extension_uri(
        &mut self,
        simple_extension_uri: &SimpleExtensionURI,
    ) -> Result<(), ContextError> {
        // Insert the extension in the map, returning an error if the anchor was
        // already added.
        match self.extensions_uris.entry(simple_extension_uri.anchor()) {
            Entry::Occupied(other) => Err(ContextError::SimpleExtensionURI(
                SimpleExtensionURIError::DuplicateAnchor {
                    added: Box::new(simple_extension_uri.clone()),
                    existing: Box::new(other.get().uri().clone()),
                },
            )),
            Entry::Vacant(entry) => {
                entry.insert(simple_extension_uri.clone());
                Ok(())
            }
        }
    }

    #[tracing::instrument(skip(self))]
    fn simple_extension(
        &mut self,
        anchor: SimpleExtensionAnchor,
    ) -> Result<text::simple_extensions::SimpleExtensions, ContextError> {
        self.extensions_uris
            .get(&anchor)
            .ok_or(ContextError::SimpleExtensionURI(
                SimpleExtensionURIError::Undefined(anchor),
            ))
            .and_then(|simple_extension_uri| {
                reqwest::blocking::get(simple_extension_uri.uri().clone())
                    .and_then(reqwest::blocking::Response::text)
                    .map_err(|e| {
                        ContextError::SimpleExtensionURI(SimpleExtensionURIError::Resolve {
                            simple_extension_uri: Box::new(simple_extension_uri.clone()),
                            reason: e.to_string(),
                        })
                    })
            })
            .and_then(|simple_extension| {
                serde_yaml::from_str::<text::simple_extensions::SimpleExtensions>(&simple_extension)
                    .map_err(ContextError::SimpleExtension)
            })
    }
}
