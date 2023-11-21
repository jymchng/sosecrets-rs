#[test]
fn test_compile_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("trybuild_tests/test_compile_fail_one.rs");
    t.compile_fail("trybuild_tests/test_compile_fail_two.rs");
    t.compile_fail("trybuild_tests/test_compile_fail_three.rs");

    #[cfg(feature = "cloneable-secret")]
    t.compile_fail("trybuild_tests/test_compile_fail_four.rs");

    #[cfg(feature = "cloneable-secret")]
    t.compile_fail("trybuild_tests/test_compile_fail_five.rs");

    #[cfg(feature = "cloneable-secret")]
    t.compile_fail("trybuild_tests/test_compile_fail_six.rs");

    #[cfg(feature = "cloneable-secret")]
    t.pass("trybuild_tests/test_compile_pass_one.rs");
}
