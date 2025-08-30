// crates/rust-bootstrap/tests/cargo_integration/test_init_global_context.rs

use rust_bootstrap::cargo_integration::init_global_context;

#[test]
fn test_init_global_context() {
    let gctx = init_global_context::init_global_context();
    assert!(gctx.is_ok(), "Failed to initialize GlobalContext: {:?}", gctx.err());
}
