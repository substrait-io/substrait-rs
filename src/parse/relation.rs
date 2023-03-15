// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::Rel].

use super::{Context, Parse};
use crate::proto;
use thiserror::Error;

/// A relation.
#[derive(Debug, PartialEq)]
pub enum Relation {
    /// A read relation.
    Read(ReadRelation),
}

/// Read relation.
#[derive(Debug, PartialEq)]
pub struct ReadRelation {}

impl From<ReadRelation> for proto::ReadRel {
    fn from(_read_relation: ReadRelation) -> Self {
        todo!()
    }
}

/// Parse error for [proto::ReadRel].
#[derive(Debug, Error, PartialEq)]
pub enum ReadRelationError {}

/// Parse error for [proto::Rel].
#[derive(Debug, Error, PartialEq)]
pub enum RelationError {
    /// A relation must have a relation type.
    #[error("a relation must have a relation type")]
    MissingType,

    /// Read relation error.
    #[error(transparent)]
    Read(#[from] ReadRelationError),
}

impl From<Relation> for proto::Rel {
    fn from(relation: Relation) -> Self {
        proto::Rel {
            rel_type: Some(relation.into()),
        }
    }
}

impl<C: Context> Parse<C> for proto::Rel {
    type Parsed = Relation;
    type Error = RelationError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        // A relation must have a type.
        self.rel_type
            .map(|rel_type| rel_type.parse(ctx))
            .transpose()?
            .ok_or(RelationError::MissingType)
    }
}

impl From<Relation> for proto::rel::RelType {
    fn from(relation: Relation) -> Self {
        match relation {
            Relation::Read(read_relation) => {
                proto::rel::RelType::Read(Box::new(read_relation.into()))
            }
        }
    }
}

impl<C: Context> Parse<C> for proto::rel::RelType {
    type Parsed = Relation;
    type Error = RelationError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        match self {
            proto::rel::RelType::Read(read_relation) => read_relation
                .parse(ctx)
                .map(Relation::Read)
                .map_err(RelationError::Read),
            _ => todo!(),
        }
    }
}

impl<C: Context> Parse<C> for proto::ReadRel {
    type Parsed = ReadRelation;
    type Error = ReadRelationError;

    fn parse(self, _ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::ReadRel {
            common: _,
            base_schema: _,
            filter: _,
            best_effort_filter: _,
            projection: _,
            advanced_extension: _,
            read_type: _,
        } = self;
        todo!()
    }
}
