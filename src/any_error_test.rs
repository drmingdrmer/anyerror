use std::error::Error;
use std::fmt;

use anyhow::Context;

use crate::AddContext;
use crate::AnyError;

#[test]
fn test_any_error_eq() -> anyhow::Result<()> {
    let fmt_err = fmt::Error {};

    let ae1 = AnyError::new(&fmt_err);

    let ae2 = AnyError::new(&fmt_err);

    // derive(Eq)
    let _ = ae1 == ae2;

    Ok(())
}

#[test]
fn test_any_error() -> anyhow::Result<()> {
    // build from std Error

    let fmt_err = fmt::Error {};

    let ae = AnyError::new(&fmt_err).with_backtrace();

    let want_str = "core::fmt::Error: an error occurred when formatting an argument";
    assert_eq!(want_str, ae.to_string());
    assert!(ae.source().is_none());
    assert!(!ae.backtrace().unwrap().is_empty());

    // debug output with backtrace

    let debug_str = format!("{:?}", ae);

    assert!(debug_str.starts_with(want_str));
    assert!(debug_str.contains("test_any_error"));

    // chained errors

    let err1 = anyhow::anyhow!("err1");
    let err2 = Err::<(), anyhow::Error>(err1).context("err2");

    let ae = AnyError::from_dyn(err2.unwrap_err().as_ref(), None).with_backtrace();

    assert_eq!("err2 source: err1", ae.to_string());
    let src = ae.source().unwrap();
    assert_eq!("err1", src.to_string());
    assert!(!ae.backtrace().unwrap().is_empty());

    // build AnyError from AnyError

    let ae2 = AnyError::new(&ae);
    assert_eq!("err2 source: err1", ae.to_string());

    let ser = serde_json::to_string(&ae2)?;
    let de: AnyError = serde_json::from_str(&ser)?;

    assert_eq!("err2 source: err1", de.to_string());

    Ok(())
}

#[test]
fn test_any_error_error() -> anyhow::Result<()> {
    let ae = AnyError::error(123);

    let want_str = "123";
    assert_eq!(want_str, ae.to_string());
    assert!(ae.source().is_none());
    assert!(ae.backtrace().is_none());

    Ok(())
}

#[test]
fn test_any_error_backtrace() -> anyhow::Result<()> {
    let ae = AnyError::error(123);

    let want_str = "123";
    assert_eq!(want_str, ae.to_string());
    assert!(ae.source().is_none());
    assert!(ae.backtrace().is_none());

    let with_bt = ae.with_backtrace();
    assert!(with_bt.backtrace().is_some());

    let ae2 = AnyError::new(&with_bt);

    assert!(ae2.backtrace().is_some());

    Ok(())
}

#[test]
fn test_any_error_context() -> anyhow::Result<()> {
    let ae = AnyError::error(123).add_context(|| "foo");

    let want_str = "123 while: foo";
    assert_eq!(want_str, ae.to_string());

    let res: Result<i32, AnyError> = Ok(3);
    assert_eq!(Ok(3), res.add_context(|| "foo"));

    let res: Result<i32, AnyError> = Err(ae);
    assert_eq!(
        "123 while: foo, while: bar",
        res.add_context(|| "bar").unwrap_err().to_string()
    );

    Ok(())
}

#[cfg(feature = "anyhow")]
#[test]
fn test_from_anyhow() -> anyhow::Result<()> {
    let err1 = anyhow::anyhow!("err1");
    let err2 = Err::<(), anyhow::Error>(err1).context("err2");

    let ae = AnyError::from(err2.unwrap_err());

    assert_eq!("err2 source: err1", ae.to_string());
    let src = ae.source().unwrap();
    assert_eq!("err1", src.to_string());
    assert!(!ae.backtrace().unwrap().is_empty());

    Ok(())
}
