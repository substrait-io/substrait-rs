// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto](crate::proto) types.

mod version;
pub use version::{Version, VersionError};

mod plan_version;
pub use plan_version::{PlanVersion, PlanVersionError};

pub mod extensions;
