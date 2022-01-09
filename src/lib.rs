#![feature(backtrace)]

mod any_error_impl;

#[cfg(test)]
mod any_error_test;

pub use any_error_impl::AnyError;
