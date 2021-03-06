#![forbid(unsafe_code)]

/// Some fancy docs.
///
/// ## Example
///
/// ```rust
/// ::my_crate::module::foo();
/// ```
// Assuming `cargo test --doc --features integration-tests` is run:
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
mod module {
    pub fn foo() {}
}
