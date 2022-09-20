// SPDX-License-Identifier: Apache-2.0

/// Generated types for the protobuf `substrait` package
#[allow(clippy::all)]
pub mod protobuf {

    /// Generated types for the protobuf `substrait.extensions` package
    pub mod extensions {
        include!(concat!(env!("OUT_DIR"), "/substrait.extensions.rs"));
    }

    include!(concat!(env!("OUT_DIR"), "/substrait.rs"));
}

#[cfg(test)]
mod tests {
    use crate::protobuf::expression::{literal::LiteralType, Literal};

    #[test]
    fn literal() {
        let _ = Literal {
            nullable: true,
            literal_type: Some(LiteralType::I32(123)),
            type_variation_reference: 0,
        };
    }
}
