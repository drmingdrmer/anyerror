use std::error::Error;
use std::fmt;
use std::io::ErrorKind;

use anyhow::Context;

use crate::backtrace_str;
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

    let ae = AnyError::new(&fmt_err);

    let want_str = "core::fmt::Error: an error occurred when formatting an argument";
    assert_eq!(want_str, ae.to_string());
    assert!(ae.source().is_none());

    // chained errors

    let err1 = anyhow::anyhow!("err1");
    let err2 = Err::<(), anyhow::Error>(err1).context("err2");

    let ae = AnyError::from_dyn(err2.unwrap_err().as_ref(), None);

    assert_eq!("err2; source: err1", ae.to_string());
    let src = ae.source().unwrap();
    assert_eq!("err1", src.to_string());

    // build AnyError from AnyError

    let ae2 = AnyError::new(&ae);
    assert_eq!("err2; source: err1", ae.to_string());

    // test serde (de)serialization
    let ser = serde_json::to_string(&ae2)?;
    let de: AnyError = serde_json::from_str(&ser)?;
    assert_eq!("err2; source: err1", de.to_string());

    // test rkyv (de)serialization
    #[cfg(feature = "rkyv")]
    {
        use rkyv::Deserialize;

        let ser = rkyv::to_bytes::<_, 256>(&ae2)?;
        let archived = rkyv::check_archived_root::<AnyError>(&ser[..]).expect("rkyv deserialization failed");
        let de: AnyError = archived.deserialize(&mut rkyv::Infallible)?;
        assert_eq!(de, ae2);
    }

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
fn test_any_error_from_error_ref() -> anyhow::Result<()> {
    let io_err = std::io::Error::new(ErrorKind::AddrInUse, "foo");
    let ae = AnyError::from(&io_err);

    let want_str = r#"std::io::error::Error: foo"#;
    assert_eq!(want_str, ae.to_string());
    assert!(ae.source().is_none());
    assert!(ae.backtrace().is_none());

    Ok(())
}

#[cfg(feature = "backtrace")]
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

    // debug output with backtrace

    let debug_str = format!("{:?}", with_bt);

    assert!(debug_str.starts_with(want_str));
    assert!(debug_str.contains("test_any_error_backtrace"));

    Ok(())
}

#[cfg(not(feature = "backtrace"))]
#[test]
fn test_any_error_no_backtrace() -> anyhow::Result<()> {
    // build from std Error

    let fmt_err = fmt::Error {};

    let ae = AnyError::new(&fmt_err);

    let want_str = "core::fmt::Error: an error occurred when formatting an argument";
    assert_eq!(want_str, ae.to_string());
    assert!(ae.source().is_none());
    assert!(ae.backtrace().is_none());

    // chained errors

    let err1 = anyhow::anyhow!("err1");
    let err2 = Err::<(), anyhow::Error>(err1).context("err2");

    let ae = AnyError::from_dyn(err2.unwrap_err().as_ref(), None);

    assert_eq!("err2; source: err1", ae.to_string());
    let src = ae.source().unwrap();
    assert_eq!("err1", src.to_string());
    assert!(ae.backtrace().is_none());

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

#[test]
fn test_any_error_context_and_source() -> anyhow::Result<()> {
    let err1 = anyhow::anyhow!("err1");
    let err2 = Err::<(), anyhow::Error>(err1).context("err2");

    let ae = AnyError::from_dyn(err2.unwrap_err().as_ref(), None).add_context(|| "context_2");

    let want_str = "err2 while: context_2; source: err1";
    assert_eq!(want_str, ae.to_string());

    Ok(())
}

#[cfg(feature = "anyhow")]
#[cfg(not(feature = "backtrace"))]
#[test]
fn test_from_anyhow() -> anyhow::Result<()> {
    let err1 = anyhow::anyhow!("err1");
    let err2 = Err::<(), anyhow::Error>(err1).context("err2");

    let ae = AnyError::from(err2.unwrap_err());

    assert_eq!("err2; source: err1", ae.to_string());
    let src = ae.source().unwrap();
    assert_eq!("err1", src.to_string());
    assert!(ae.backtrace().is_none());

    Ok(())
}

#[cfg(feature = "anyhow")]
#[cfg(feature = "backtrace")]
#[test]
fn test_from_anyhow_with_backtrace() -> anyhow::Result<()> {
    let err1 = anyhow::anyhow!("err1");
    let err2 = Err::<(), anyhow::Error>(err1).context("err2");

    let ae = AnyError::from(err2.unwrap_err());

    assert_eq!("err2; source: err1", ae.to_string());
    let src = ae.source().unwrap();
    assert_eq!("err1", src.to_string());
    assert!(!ae.backtrace().unwrap().is_empty());

    Ok(())
}

#[cfg(feature = "backtrace")]
#[test]
fn test_backtrace() -> anyhow::Result<()> {
    let got = backtrace_str().expect("no none");
    assert!(got.contains("test_backtrace"));
    Ok(())
}

#[cfg(not(feature = "backtrace"))]
#[test]
fn test_backtrace() -> anyhow::Result<()> {
    let got = backtrace_str();
    assert!(got.is_none());
    Ok(())
}
