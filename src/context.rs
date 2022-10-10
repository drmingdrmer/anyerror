use std::fmt::Display;

use crate::AnyError;

/// The trait to add error context info to a `Result<T, E>`
pub trait AddContext<T, Err, CtxErr> {
    fn add_context<D, F>(self, f: F) -> Result<T, CtxErr>
    where
        D: Display,
        F: FnOnce() -> D;
}

/// When adding context info, convert an AnyError to another AnyError.
impl<T> AddContext<T, AnyError, AnyError> for Result<T, AnyError> {
    fn add_context<D: Display, F>(self, f: F) -> Result<T, AnyError>
    where
        D: Display,
        F: FnOnce() -> D,
    {
        self.map_err(|e| e.add_context(f))
    }
}
