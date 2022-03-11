/// Create a new backtrace instance.
pub fn new() -> backtrace::Backtrace {
    backtrace::Backtrace::new()
}

/// Create a new backtrace in multiline string
pub fn new_str() -> String {
    format!("{:?}", new())
}
