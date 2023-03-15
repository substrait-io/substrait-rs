// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::RelRoot].

use super::{
    relation::{Relation, RelationError},
    Context, Parse,
};
use crate::proto;
use thiserror::Error;

/// A root relation.
#[derive(Debug, PartialEq)]
pub struct RootRelation {
    /// The relation.
    relation: Relation,

    /// The names for the output fields (depth-first).
    schema: Vec<String>, // todo
}

/// Parse errors for [proto::RelRoot].
#[derive(Debug, Error, PartialEq)]
pub enum RootRelationError {
    /// Relation error.
    #[error(transparent)]
    Relation(#[from] RelationError),

    /// A root relation must have an input relation.
    #[error("a root relation must have an input relation")]
    MissingRelation,

    /// The names length should match the number of types in the relation.
    #[error("the length of names must match the count of fields in the relation, expected {expected} but got {actual}")]
    InvalidNamesCount {
        /// The expected length.
        expected: usize,
        /// The actual length.
        actual: usize,
    },
}

impl From<RootRelation> for proto::RelRoot {
    fn from(value: RootRelation) -> Self {
        let RootRelation { relation, schema } = value;

        proto::RelRoot {
            input: Some(relation.into()),
            names: schema,
        }
    }
}

impl<C: Context> Parse<C> for proto::RelRoot {
    type Parsed = RootRelation;
    type Error = RootRelationError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::RelRoot {
            input,
            names: schema,
        } = self;

        let relation = input
            .map(|relation| relation.parse(ctx))
            .transpose()?
            .ok_or(RootRelationError::MissingRelation)?;

        // if relation.schema().len() != schema.len() {
        //     return Err(RootRelationError::InvalidNamesCount {
        //         expected: relation.schema().len(),
        //         actual: schema.len(),
        //     });
        // }

        Ok(RootRelation { relation, schema })
    }
}
