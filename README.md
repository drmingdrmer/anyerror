# AnyError

`AnyError` is a serializable casing for `Error`.

`AnyError` can be employed to transform other `Error` types into a serializable Error for transmission, preserving most of the essential information.

```rust
let err = fmt::Error {};
let e = AnyError::new(&err)
            .add_context(|| "running test")
            .add_context(|| "developing new feature");
println!("{:#}", e);
```

The above code will print error description with context:

```text
core::fmt::Error: an error occurred when formatting an argument
    while: running test
    while: developing new feature
```