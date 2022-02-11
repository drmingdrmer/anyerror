#![feature(backtrace)]

mod any_error_impl;
mod context;

#[cfg(test)]
mod any_error_test;

pub use any_error_impl::AnyError;
pub use context::AddContext;
