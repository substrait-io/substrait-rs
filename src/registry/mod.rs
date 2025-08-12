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
//! This "validate once, assume valid" approach allows for:
//! - **Type safety**: Invalid extensions are caught at construction time
//! - **Performance**: No repeated validation during registry operations
//! - **Clean APIs**: Methods can focus on logic rather than error handling
//! - **Reliability**: Once constructed, registry operations won't fail due to
//!   malformed data
//!
//! ## Core Types
//!
//! - [`ExtensionFile`]: Validated wrapper around a SimpleExtensions + URI
//! - [`ConcreteType`]: Fully-specified types for function arguments and return
//!       values
//! - [`TypeSignature`]: Pattern matching for function signatures
//! - [`GlobalRegistry`]: Immutable registry for URI+name based function lookup

mod extension;
mod registry;
pub mod types;

pub use extension::{
    ExtensionFile, ValidationError,
    ScalarFunctionRef, AggregateFunctionRef, WindowFunctionRef,
    ScalarFunctionImplRef, AggregateFunctionImplRef, WindowFunctionImplRef,
};
pub use registry::GlobalRegistry;