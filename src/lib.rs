#![cfg_attr(feature = "backtrace", feature(backtrace))]

mod any_error_impl;
#[cfg(feature = "backtrace")]
mod bt;
mod context;

#[cfg(test)]
mod any_error_test;

pub use any_error_impl::AnyError;
pub use context::AddContext;
