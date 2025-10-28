// SPDX-License-Identifier: Apache-2.0

#[expect(clippy::derivable_impls, clippy::clone_on_copy)]
mod substrait;

#[doc(inline)]
pub use substrait::*;
