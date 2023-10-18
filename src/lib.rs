#![cfg_attr(feature = "backtrace", feature(error_generic_member_access))]
#![allow(clippy::bool_assert_comparison, clippy::type_complexity)]

mod any_error_impl;
#[cfg(feature = "backtrace")]
mod bt;
mod context;
mod macros;

#[cfg(test)]
mod any_error_test;

pub use any_error_impl::backtrace_str;
pub use any_error_impl::AnyError;
pub use context::AddContext;
