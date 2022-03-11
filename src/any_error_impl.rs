use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

/// AnyError is a serializable wrapper `Error`.
///
/// It is can be used to convert other `Error` into a serializable Error for transmission,
/// with most necessary info kept.
///
/// ```rust
/// # use anyerror::AnyError;
/// # use std::fmt;
/// let e = AnyError::new(&fmt::Error{}).add_context(|| "example");
/// assert_eq!("core::fmt::Error: an error occurred when formatting an argument while: example", e.to_string());
/// ```
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct AnyError {
    typ: Option<String>,

    msg: String,

    source: Option<Box<AnyError>>,

    /// context provides additional info about the context when a error happened.
    context: Vec<String>,

    backtrace: Option<String>,
}

impl Display for AnyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if let Some(t) = &self.typ {
            write!(f, "{}: ", t)?;
        }

        write!(f, "{}", self.msg)?;

        if let Some(ref s) = self.source {
            write!(f, " source: {}", s)?;
        }

        for (i, ctx) in self.context.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, " while: {}", ctx)?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for AnyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        <Self as Display>::fmt(self, f)?;

        if let Some(ref b) = self.backtrace {
            write!(f, "\nbacktrace:\n{}", b)?;
        }
        Ok(())
    }
}

impl std::error::Error for AnyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(x) => Some(x.as_ref()),
            None => None,
        }
    }
}

#[cfg(feature = "anyhow")]
impl From<anyhow::Error> for AnyError {
    fn from(a: anyhow::Error) -> Self {
        // NOTE: this does not work: anyhow::Error does not impl std::Error;
        //       backtrace is lost after converting it to &dyn Error with stable rust.
        // AnyError::from_dyn(a.as_ref(), None)

        let source = a.source().map(|x| Box::new(AnyError::from_dyn(x, None)));
        #[cfg(feature = "backtrace")]
        let bt = Some({
            let bt = a.backtrace();
            format!("{:?}", bt)
        });

        #[cfg(not(feature = "backtrace"))]
        let bt = None;

        Self {
            typ: None,
            msg: a.to_string(),
            source,
            context: vec![],
            backtrace: bt,
        }
    }
}

impl AnyError {
    /// Create an ad-hoc error with a string message
    pub fn error(msg: impl ToString) -> Self {
        Self {
            typ: None,
            msg: msg.to_string(),
            source: None,
            context: vec![],
            backtrace: None,
        }
    }

    /// Convert some `Error` to AnyError.
    ///
    /// - If there is a `source()` in the input error, it is also converted to AnyError, recursively.
    /// - A new backtrace will be built if there is not.
    pub fn new<E>(e: &E) -> Self
    where E: Error + 'static {
        let q: &(dyn Error + 'static) = e;

        let x = q.downcast_ref::<AnyError>();
        let typ = match x {
            Some(ae) => ae.typ.clone(),
            None => Some(std::any::type_name::<E>().to_string()),
        };

        Self::from_dyn(e, typ)
    }

    pub fn from_dyn(e: &(dyn Error + 'static), typ: Option<String>) -> Self {
        let x = e.downcast_ref::<AnyError>();

        return match x {
            Some(ae) => ae.clone(),
            None => {
                #[cfg(feature = "backtrace")]
                let bt = e.backtrace().map(|b| format!("{:?}", b));

                #[cfg(not(feature = "backtrace"))]
                let bt = None;

                let source = e.source().map(|x| Box::new(AnyError::from_dyn(x, None)));

                Self {
                    typ,
                    msg: e.to_string(),
                    source,
                    context: vec![],
                    backtrace: bt,
                }
            }
        };
    }

    #[cfg(feature = "backtrace")]
    #[must_use]
    pub fn with_backtrace(mut self) -> Self {
        if self.backtrace.is_some() {
            return self;
        }

        self.backtrace = Some(crate::bt::new_str());
        self
    }

    #[must_use]
    pub fn add_context<D: Display, F: FnOnce() -> D>(mut self, ctx: F) -> Self {
        self.context.push(format!("{}", ctx()));
        self
    }

    pub fn get_type(&self) -> Option<&str> {
        self.typ.as_ref().map(|x| x as _)
    }

    pub fn backtrace(&self) -> Option<&str> {
        self.backtrace.as_ref().map(|x| x as _)
    }
}

/// Generate backtrace in string if feature `backtrace` is enabled.
/// Otherwise it returns None.
pub fn backtrace_str() -> Option<String> {
    #[cfg(feature = "backtrace")]
    return Some(crate::bt::new_str());

    #[cfg(not(feature = "backtrace"))]
    return None;
}
