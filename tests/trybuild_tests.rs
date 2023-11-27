#[test]
fn test_compile_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("trybuild_tests/test_compile_fail_one.rs");
    t.compile_fail("trybuild_tests/test_compile_fail_two.rs");
    t.compile_fail("trybuild_tests/test_compile_fail_three.rs");

    #[cfg(all(feature = "cloneable-secret", not(feature = "alloc")))]
    t.compile_fail("trybuild_tests/test_compile_fail_four.rs");

    #[cfg(all(feature = "cloneable-secret", not(feature = "alloc")))]
    t.compile_fail("trybuild_tests/test_compile_fail_five.rs");

    #[cfg(all(feature = "alloc", feature = "cloneable-secret"))]
    t.compile_fail("trybuild_tests/test_compile_fail_six.rs");

    // std env + alloc + no clone, no clone should error
    #[cfg(all(feature = "alloc", not(feature = "cloneable-secret")))]
    t.compile_fail("trybuild_tests/test_compile_fail_seven.rs");

    // no_std env + alloc + extern crate alloc::vec::Vec in main()
    #[cfg(all(feature = "alloc", not(feature = "cloneable-secret")))]
    t.compile_fail("trybuild_tests/test_compile_fail_eight.rs");

    #[cfg(feature = "cloneable-secret")]
    t.pass("trybuild_tests/test_compile_pass_one.rs");

    // no_std env + no alloc + no cloneable-secret should work
    t.pass("trybuild_tests/test_compile_pass_two.rs");
}
