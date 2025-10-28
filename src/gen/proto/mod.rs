// SPDX-License-Identifier: Apache-2.0

#[cfg(not(feature = "serde"))]
#[expect(clippy::doc_overindented_list_items, clippy::module_inception)]
mod proto {
    include!("substrait.rs");

    /// Generated types for the protobuf `substrait.extensions` package
    pub mod extensions {
        include!("substrait.extensions.rs");
    }
}

#[cfg(feature = "serde")]
#[expect(
    clippy::doc_overindented_list_items,
    clippy::module_inception,
    rustdoc::invalid_html_tags
)]
mod proto {
    include!("pbjson/substrait.rs");
    include!("pbjson/substrait.serde.rs");

    /// Generated types for the protobuf `substrait.extensions` package
    pub mod extensions {
        include!("pbjson/substrait.extensions.rs");
        include!("pbjson/substrait.extensions.serde.rs");
    }
}

#[doc(inline)]
pub use proto::*;
