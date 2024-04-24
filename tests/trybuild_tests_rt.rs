#[rustversion::not(stable(1.70.0))]
#[test]
fn test_compile_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("trybuild_tests/runtime/cannot_cross_unwind_if_not_copy.rs");
    t.compile_fail("trybuild_tests/runtime/cannot_return_exposed_secret.rs");
    t.compile_fail("trybuild_tests/runtime/u0_cannot_call_expose_secret.rs");

    #[cfg(all(
        not(feature = "debug-secret"),
        not(feature = "cloneable-secret"),
        not(feature = "alloc")
    ))]
    t.compile_fail("trybuild_tests/runtime/cannot_call_debug_clone_alloc_if_not_use.rs");

    #[cfg(all(
        feature = "debug-secret",
        feature = "cloneable-secret",
        feature = "alloc"
    ))]
    t.pass("trybuild_tests/runtime/cannot_call_debug_clone_alloc_if_not_use.rs");
}
