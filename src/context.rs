use std::fmt::Display;

use crate::AnyError;

/// The trait to add error context info to a `Result<T, AnyError>`
pub trait AddContext<T> {
    fn add_context<D, F>(self, f: F) -> Result<T, AnyError>
    where
        D: Display,
        F: FnOnce() -> D;
}

impl<T> AddContext<T> for Result<T, AnyError> {
    fn add_context<D: Display, F>(self, f: F) -> Result<T, AnyError>
    where
        D: Display,
        F: FnOnce() -> D,
    {
        self.map_err(|e| e.add_context(f))
    }
}
