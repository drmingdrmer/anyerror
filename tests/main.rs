use anyerror::func_name;
use anyerror::func_path;

#[test]
fn test_func_path() {
    fn foo() -> String {
        func_path!()
    }

    assert_eq!("main::test_func_path::foo", foo());
}

#[tokio::test]
async fn test_func_name() {
    async fn foo() -> String {
        func_name!()
    }

    assert_eq!("foo", foo().await);
}
