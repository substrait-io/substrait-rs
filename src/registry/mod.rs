//! Substrait Extension Registry
//!
//! This module provides types and methods that abstract over Substrait
//! SimpleExtensions.
//!
//! ## Design Philosophy
//!
//! Internally, the types in this module are handles to the raw parsed
//! SimpleExtensions from the text module. Externally, they provide a coherent
//! interface that hides those internal details and presents methods where
//! extensions are validated on creation and then assumed valid thereafter.
//!
//! This allows for a clean API that externally follows the "parse don't
//! validate" principle, with an API that encourages users to work with
//! validated extensions without worrying about their internal structure,
//! without needing to add entirely new parse trees - the type tree can be
//! recreated on-demand.
//!
//! ## Core Types
//!
//! - [`ExtensionFile`]: Validated wrapper around SimpleExtensions + URI focusing on type definitions
//! - [`CustomType`]: Parsed and validated extension type definitions 
//! - [`Registry`]: Immutable registry for URI+name based type lookup
//!
//! Currently only type definitions are supported. Function parsing will be added in a future update.

mod context;
mod extension;
mod registry;
pub mod types;

pub use extension::{ExtensionFile, ValidationError};
pub use registry::Registry;
pub use types::{ConcreteType, CustomType};
