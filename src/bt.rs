use std::backtrace::Backtrace;
use std::error::Error;

/// Create a new backtrace instance.
pub fn new() -> Backtrace {
    Backtrace::force_capture()
}

/// Create a new backtrace in multiline string
pub fn new_str() -> String {
    format!("{:?}", new())
}

pub fn error_backtrace_ref<'err>(e: &'err (dyn Error + 'static)) -> Option<&'err Backtrace> {
    e.request_ref::<Backtrace>()
}

pub fn error_backtrace_str(e: &(dyn Error + 'static)) -> Option<String> {
    let bt = error_backtrace_ref(e);
    bt.map(|b| format!("{:?}", b))
}
