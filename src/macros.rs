/// Return the type path of the callee.
///
/// Example:
/// ```
/// # use anyerror::func_path;
///
/// fn foo() -> String {
///     func_path!()
/// }
/// // `foo()` outputs: "rust_out::main:: ... ::foo";
/// ```
#[macro_export]
macro_rules! func_path {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        // "anyerror::macros::tests::{{closure}}::test_func_name::f"
        let type_name = type_name_of(f);

        // Script `::f`:
        // "anyerror::macros::tests::{{closure}}::test_func_name"
        let name_path = &type_name[..type_name.len() - 3];

        // Remove `{{closure}}`:
        // "anyerror::macros::tests::test_func_name"
        let stripped = name_path.replace("::{{closure}}", "");

        stripped
    }};
}

/// Return the name of the callee.
///
/// Example:
/// ```
/// # use anyerror::func_name;
///
/// fn foo() -> String {
///     func_name!()
/// }
/// assert_eq!("foo", foo());
/// ```
#[macro_export]
macro_rules! func_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        // "anyerror::macros::tests::{{closure}}::test_func_name::f"
        let type_name = type_name_of(f);

        // Script `::f`:
        // "anyerror::macros::tests::{{closure}}::test_func_name"
        let name_path = &type_name[..type_name.len() - 3];

        // Remove `{{closure}}`:
        // "anyerror::macros::tests::test_func_name"
        let stripped = name_path.replace("::{{closure}}", "");

        stripped.split("::").last().unwrap().to_string()
    }};
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_func_path() {
        async fn foo() -> String {
            func_path!()
        }

        let p = foo().await;
        assert_eq!("anyerror::macros::tests::test_func_path::foo", p);
        // let x = func_name!();
    }

    #[tokio::test]
    async fn test_func_name() {
        async fn bar() -> String {
            func_name!()
        }

        let p = bar().await;
        assert_eq!("bar", p);
    }
}
